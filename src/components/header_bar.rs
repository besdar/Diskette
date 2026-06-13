use super::icon_button;
use crate::i18n::text;
use crate::settings::{APP_ID, APP_NAME};
use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

pub(crate) struct HeaderBarParts {
    pub(crate) header: adw::HeaderBar,
    pub(crate) main_navigation: gtk::StackSwitcher,
    pub(crate) refresh_button: gtk::Button,
    pub(crate) docs_button: gtk::Button,
    pub(crate) settings_button: gtk::Button,
    pub(crate) status_indicator: gtk::Box,
    pub(crate) status_label: gtk::Label,
}

pub(crate) fn build_header_bar(stack: &gtk::Stack) -> HeaderBarParts {
    let header = adw::HeaderBar::new();
    let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let icon = gtk::Image::from_icon_name(APP_ID);
    icon.set_pixel_size(24);
    let title = gtk::Label::new(Some(APP_NAME));
    title.add_css_class("heading");

    let switcher = gtk::StackSwitcher::new();
    switcher.set_stack(Some(stack));
    title_box.append(&icon);
    title_box.append(&title);
    title_box.append(&switcher);
    header.set_title_widget(Some(&title_box));

    let refresh_button = icon_button("view-refresh-symbolic", text("refresh_status"));
    let docs_button = icon_button(
        "help-browser-symbolic",
        text("open_yandex_disk_cli_documentation"),
    );
    let settings_button = icon_button("preferences-system-symbolic", text("settings"));

    let status_indicator = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    status_indicator.set_size_request(10, 10);
    status_indicator.set_valign(gtk::Align::Center);
    status_indicator.add_css_class("diskette-status-dot");
    status_indicator.add_css_class("diskette-status-idle");

    let status_label = gtk::Label::new(Some(text("status_not_checked")));
    status_label.add_css_class("dim-label");

    let status_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    status_box.set_valign(gtk::Align::Center);
    status_box.add_css_class("diskette-header-status");
    status_box.append(&status_indicator);
    status_box.append(&status_label);

    header.pack_start(&refresh_button);
    header.pack_start(&status_box);
    header.pack_end(&settings_button);
    header.pack_end(&docs_button);

    HeaderBarParts {
        header,
        main_navigation: switcher,
        refresh_button,
        docs_button,
        settings_button,
        status_indicator,
        status_label,
    }
}
