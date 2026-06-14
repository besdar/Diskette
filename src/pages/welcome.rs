use crate::components::page_box;
use crate::i18n::text;
use crate::settings::INSTALL_DOCS_URL;
use crate::utils::open_uri;
use gtk::prelude::*;
use gtk4 as gtk;

pub(crate) fn build_welcome_page(stack: &gtk::Stack) -> gtk::Box {
    let page = page_box();
    page.set_valign(gtk::Align::Center);

    let group = gtk::Box::new(gtk::Orientation::Vertical, 16);
    group.add_css_class("diskette-welcome");
    group.set_halign(gtk::Align::Center);

    let icon = gtk::Image::from_icon_name("app.diskette.Diskette");
    icon.set_pixel_size(96);
    icon.add_css_class("diskette-welcome-icon");
    group.append(&icon);

    let title = gtk::Label::new(Some(text("welcome_title")));
    title.add_css_class("title-1");
    title.set_xalign(0.0);
    group.append(&title);

    let description = gtk::Label::new(Some(text("welcome_body")));
    description.add_css_class("diskette-muted");
    description.set_wrap(true);
    description.set_xalign(0.0);
    group.append(&description);

    let steps = gtk::Box::new(gtk::Orientation::Vertical, 8);
    steps.add_css_class("diskette-step-list");
    steps.append(&step_row("1", text("onboarding_step_auth")));
    steps.append(&step_row("2", text("onboarding_step_folder")));
    steps.append(&step_row("3", text("onboarding_step_sync")));
    group.append(&steps);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::Start);
    let get_started = gtk::Button::with_label(text("get_started"));
    let learn_more = gtk::Button::with_label(text("learn_more"));
    get_started.add_css_class("suggested-action");
    buttons.append(&get_started);
    buttons.append(&learn_more);
    group.append(&buttons);

    {
        let stack = stack.clone();
        get_started.connect_clicked(move |_| {
            stack.set_visible_child_name("settings");
        });
    }

    learn_more.connect_clicked(move |_| {
        open_uri(INSTALL_DOCS_URL);
    });

    page.append(&group);
    page
}

fn step_row(number: &str, label: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    row.set_hexpand(true);

    let badge = gtk::Label::new(Some(number));
    badge.set_width_chars(2);
    badge.add_css_class("diskette-step-badge");
    row.append(&badge);

    let text = gtk::Label::new(Some(label));
    text.set_xalign(0.0);
    text.set_wrap(true);
    row.append(&text);

    row
}
