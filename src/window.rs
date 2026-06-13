use crate::actions::{EventPumpUi, attach_event_pump};
use crate::components::yandex_disk::{OptionControls, SetupControls, submit_request};
use crate::components::{
    HeaderBarParts, append_output, build_header_bar, build_output_group, build_output_view,
    load_css,
};
use crate::i18n::text;
use crate::models::yandex_disk::{DiskRequest, SetupConfig, UiEvent};
use crate::pages::{
    advanced::build_advanced_page,
    overview::{OverviewLabels, build_overview_page},
    setup::build_setup_page,
    sharing::build_sharing_page,
    welcome::build_welcome_page,
};
use crate::settings::{APP_NAME, COMMAND_DOCS_URL};
use crate::utils::open_uri;
use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;
use std::sync::mpsc;

struct DashboardLabels {
    status_title: gtk::Label,
    status_detail: gtk::Label,
    activity_title: gtk::Label,
    activity_detail: gtk::Label,
}

pub(crate) fn build(app: &adw::Application) {
    load_css();

    let (sender, receiver) = mpsc::channel::<UiEvent>();
    let loaded_config = SetupConfig::load();
    let auth_ready = loaded_config.auth_file.exists();
    let option_controls = OptionControls::new(&loaded_config);
    let setup_controls = SetupControls::new(&loaded_config);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .default_width(960)
        .default_height(720)
        .build();

    let toolbar_view = adw::ToolbarView::new();
    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .vexpand(true)
        .build();
    let header = build_header_bar(&stack);
    header.main_navigation.set_visible(auth_ready);
    header.refresh_button.set_visible(auth_ready);
    toolbar_view.add_top_bar(&header.header);

    let labels = build_dashboard_labels();
    let (output_view, output_buffer) = build_output_view();

    let overview = build_overview_page(
        &sender,
        &output_buffer,
        &option_controls,
        &setup_controls,
        &OverviewLabels {
            status_title: &labels.status_title,
            status_detail: &labels.status_detail,
            activity_title: &labels.activity_title,
            activity_detail: &labels.activity_detail,
        },
    );
    let welcome = build_welcome_page(&stack);
    let setup_form = build_setup_page(&window, &sender, &output_buffer, &setup_controls);
    let settings = build_settings_page(&setup_form, &stack, &header.main_navigation);
    let (sharing, publish_link_entry) =
        build_sharing_page(&window, &sender, &output_buffer, &option_controls);
    let advanced = build_advanced_page(&sender, &output_buffer, &option_controls);

    stack.add_named(&welcome, Some("welcome"));
    stack.add_titled(&overview, Some("overview"), text("overview"));
    stack.add_titled(&sharing, Some("sharing"), text("sharing"));
    stack.add_titled(&advanced, Some("advanced"), text("advanced"));
    stack.add_named(&settings, Some("settings"));
    stack.set_visible_child_name(if auth_ready { "overview" } else { "welcome" });

    let output_group = build_output_group(&output_view);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.append(&stack);
    content.append(&output_group);
    toolbar_view.set_content(Some(&content));
    window.set_content(Some(&toolbar_view));

    connect_header_actions(&header, &stack, &sender, &output_buffer, &option_controls);

    let startup_output_buffer = output_buffer.clone();
    attach_event_pump(
        receiver,
        sender.clone(),
        EventPumpUi {
            output_buffer,
            status_title: labels.status_title,
            status_detail: labels.status_detail,
            header_status_indicator: header.status_indicator,
            header_status_label: header.status_label,
            activity_title: labels.activity_title,
            activity_detail: labels.activity_detail,
            publish_link_entry,
            stack,
            main_navigation: header.main_navigation,
            refresh_button: header.refresh_button,
        },
    );

    run_startup_check(
        auth_ready,
        &sender,
        &startup_output_buffer,
        &option_controls,
    );

    window.present();
}

fn build_dashboard_labels() -> DashboardLabels {
    let status_title = gtk::Label::new(Some(text("status_not_checked")));
    status_title.add_css_class("diskette-title");
    status_title.set_xalign(0.0);

    let status_detail = gtk::Label::new(Some(text("run_status_to_inspect_daemon")));
    status_detail.add_css_class("diskette-muted");
    status_detail.set_wrap(true);
    status_detail.set_xalign(0.0);

    let activity_title = gtk::Label::new(Some(text("no_recent_activity")));
    activity_title.add_css_class("diskette-title");
    activity_title.set_xalign(0.0);

    let activity_detail = gtk::Label::new(Some(text("activity_waiting_for_commands")));
    activity_detail.add_css_class("diskette-muted");
    activity_detail.set_wrap(true);
    activity_detail.set_xalign(0.0);

    DashboardLabels {
        status_title,
        status_detail,
        activity_title,
        activity_detail,
    }
}

fn connect_header_actions(
    header: &HeaderBarParts,
    stack: &gtk::Stack,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
) {
    {
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        let option_controls = option_controls.clone();
        header.refresh_button.connect_clicked(move |_| {
            submit_request(
                &sender,
                &output_buffer,
                DiskRequest::Status(option_controls.read()),
            );
        });
    }

    header.docs_button.connect_clicked(move |_| {
        open_uri(COMMAND_DOCS_URL);
    });

    {
        let stack = stack.clone();
        header.settings_button.connect_clicked(move |_| {
            stack.set_visible_child_name("settings");
        });
    }
}

fn build_settings_page(
    setup_form: &gtk::Box,
    stack: &gtk::Stack,
    main_navigation: &gtk::StackSwitcher,
) -> gtk::Box {
    let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
    page.append(setup_form);

    let footer = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    footer.set_halign(gtk::Align::End);
    footer.set_margin_start(30);
    footer.set_margin_end(30);
    footer.set_margin_bottom(18);

    let done_button = gtk::Button::with_label(text("done"));
    done_button.add_css_class("suggested-action");
    footer.append(&done_button);
    page.append(&footer);

    {
        let stack = stack.clone();
        let main_navigation = main_navigation.clone();
        done_button.connect_clicked(move |_| {
            let page_name = if main_navigation.is_visible() {
                "overview"
            } else {
                "welcome"
            };
            stack.set_visible_child_name(page_name);
        });
    }

    page
}

fn run_startup_check(
    auth_ready: bool,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
) {
    if auth_ready {
        submit_request(
            sender,
            output_buffer,
            DiskRequest::Status(option_controls.read()),
        );
    } else {
        append_output(output_buffer, &format!("{}\n", text("run_get_token_first")));
    }
}
