use crate::components::yandex_disk::{OptionControls, SetupControls, connect_command_button};
use crate::components::{append_output, page_box, section};
use crate::i18n::text;
use crate::models::yandex_disk::{DiskRequest, UiEvent};
use crate::settings::INSTALL_DOCS_URL;
use crate::utils::{open_uri, open_uri_result};
use gtk::gio;
use gtk::prelude::*;
use gtk4 as gtk;
use std::fs;
use std::sync::mpsc;

pub(crate) struct OverviewLabels<'a> {
    pub(crate) status_title: &'a gtk::Label,
    pub(crate) status_detail: &'a gtk::Label,
    pub(crate) activity_title: &'a gtk::Label,
    pub(crate) activity_detail: &'a gtk::Label,
}

struct OverviewButtons {
    status: gtk::Button,
    start: gtk::Button,
    sync: gtk::Button,
    stop: gtk::Button,
    foreground_daemon: gtk::Button,
    open_folder: gtk::Button,
    install_docs: gtk::Button,
}

pub(crate) fn build_overview_page(
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
    setup_controls: &SetupControls,
    labels: &OverviewLabels<'_>,
) -> gtk::Box {
    let page = page_box();
    let (status_group, buttons) = build_status_group(labels.status_title, labels.status_detail);

    page.append(&status_group);
    page.append(&build_storage_group());
    page.append(&build_activity_group(
        labels.activity_title,
        labels.activity_detail,
    ));
    page.append(&build_cli_coverage_group());

    connect_daemon_buttons(&buttons, sender, output_buffer, option_controls);
    connect_utility_buttons(&buttons, setup_controls, output_buffer);

    page
}

fn build_status_group(
    status_title: &gtk::Label,
    status_detail: &gtk::Label,
) -> (gtk::Box, OverviewButtons) {
    let group = section(text("sync_status"));
    let status_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    status_box.add_css_class("diskette-card");
    status_box.add_css_class("diskette-status");
    status_box.append(status_title);
    status_box.append(status_detail);
    group.append(&status_box);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::Start);
    let status_button = gtk::Button::with_label(text("status"));
    let start_button = gtk::Button::with_label(text("resume_sync"));
    let sync_button = gtk::Button::with_label(text("force_sync"));
    let stop_button = gtk::Button::with_label(text("pause_sync"));
    let no_daemon_button = gtk::Button::with_label(text("launch_foreground_daemon"));
    start_button.add_css_class("suggested-action");
    stop_button.add_css_class("destructive-action");
    buttons.append(&status_button);
    buttons.append(&start_button);
    buttons.append(&sync_button);
    buttons.append(&stop_button);
    buttons.append(&no_daemon_button);
    group.append(&buttons);

    let utility_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    utility_buttons.set_halign(gtk::Align::Start);
    let open_folder = gtk::Button::with_label(text("open_sync_folder"));
    let install_docs = gtk::Button::with_label(text("install_cli"));
    utility_buttons.append(&open_folder);
    utility_buttons.append(&install_docs);
    group.append(&utility_buttons);

    (
        group,
        OverviewButtons {
            status: status_button,
            start: start_button,
            sync: sync_button,
            stop: stop_button,
            foreground_daemon: no_daemon_button,
            open_folder,
            install_docs,
        },
    )
}

fn build_storage_group() -> gtk::Box {
    let group = section(text("storage"));
    let card = gtk::Box::new(gtk::Orientation::Vertical, 8);
    card.add_css_class("diskette-card");

    let title = gtk::Label::new(Some(text("storage_waiting_for_status")));
    title.set_xalign(0.0);
    title.add_css_class("diskette-title");

    let detail = gtk::Label::new(Some(text("storage_status_hint")));
    detail.set_xalign(0.0);
    detail.set_wrap(true);
    detail.add_css_class("diskette-muted");

    let bar = gtk::ProgressBar::new();
    bar.set_show_text(true);
    bar.set_text(Some(text("not_available_yet")));

    card.append(&title);
    card.append(&bar);
    card.append(&detail);
    group.append(&card);
    group
}

fn build_activity_group(activity_title: &gtk::Label, activity_detail: &gtk::Label) -> gtk::Box {
    let group = section(text("recent_activity"));
    let card = gtk::Box::new(gtk::Orientation::Vertical, 6);
    card.add_css_class("diskette-card");
    card.append(activity_title);
    card.append(activity_detail);
    group.append(&card);
    group
}

fn build_cli_coverage_group() -> gtk::Box {
    let group = section(text("cli_coverage"));
    let info = gtk::Label::new(Some(text("cli_coverage_description")));
    info.set_wrap(true);
    info.set_xalign(0.0);
    group.append(&info);
    group
}

fn connect_daemon_buttons(
    buttons: &OverviewButtons,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
) {
    connect_command_button(
        &buttons.status,
        sender,
        output_buffer,
        option_controls,
        DiskRequest::Status,
    );
    connect_command_button(
        &buttons.start,
        sender,
        output_buffer,
        option_controls,
        DiskRequest::Start,
    );
    connect_command_button(
        &buttons.sync,
        sender,
        output_buffer,
        option_controls,
        DiskRequest::Sync,
    );
    connect_command_button(
        &buttons.stop,
        sender,
        output_buffer,
        option_controls,
        DiskRequest::Stop,
    );
    connect_command_button(
        &buttons.foreground_daemon,
        sender,
        output_buffer,
        option_controls,
        DiskRequest::ForegroundDaemon,
    );
}

fn connect_utility_buttons(
    buttons: &OverviewButtons,
    setup_controls: &SetupControls,
    output_buffer: &gtk::TextBuffer,
) {
    {
        let setup_controls = setup_controls.clone();
        let output_buffer = output_buffer.clone();
        buttons.open_folder.connect_clicked(move |_| {
            let path = setup_controls.sync_dir_or_default();
            if let Err(error) = fs::create_dir_all(&path) {
                append_output(
                    &output_buffer,
                    &format!("{} {error}\n", text("failed_to_open_sync_folder")),
                );
                return;
            }
            let uri = gio::File::for_path(path).uri();
            if let Err(error) = open_uri_result(&uri) {
                append_output(
                    &output_buffer,
                    &format!("{} {error}\n", text("failed_to_open_sync_folder")),
                );
            }
        });
    }

    buttons.install_docs.connect_clicked(move |_| {
        open_uri(INSTALL_DOCS_URL);
    });
}
