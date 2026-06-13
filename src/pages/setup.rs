use crate::components::yandex_disk::SetupControls;
use crate::components::{
    append_output, field_row, file_pick_row, folder_pick_row, page_box, section,
};
use crate::i18n::text;
use crate::models::yandex_disk::{SetupConfig, UiEvent, is_flatpak};
use crate::utils::display_path;
use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;
use std::sync::mpsc;
use std::thread;

pub(crate) fn build_setup_page(
    window: &adw::ApplicationWindow,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    controls: &SetupControls,
) -> gtk::Box {
    let page = page_box();

    let auth_group = section(text("connect_yandex_disk"));
    let auth_hint = gtk::Label::new(Some(text("authentication_setup_hint")));
    auth_hint.add_css_class("diskette-muted");
    auth_hint.set_wrap(true);
    auth_hint.set_xalign(0.0);
    auth_group.append(&auth_hint);
    auth_group.append(&field_row(text("auth_file"), &controls.auth_entry));
    auth_group.append(&file_pick_row(
        window,
        text("choose_auth_file"),
        &controls.auth_entry,
    ));
    page.append(&auth_group);

    let folder_group = section(text("choose_sync_directory"));
    let folder_hint = gtk::Label::new(Some(text("sync_directory_hint")));
    folder_hint.add_css_class("diskette-muted");
    folder_hint.set_wrap(true);
    folder_hint.set_xalign(0.0);
    folder_group.append(&folder_hint);
    folder_group.append(&field_row(text("sync_folder"), &controls.sync_entry));
    if !is_flatpak() {
        folder_group.append(&folder_pick_row(
            window,
            text("choose_sync_folder"),
            &controls.sync_entry,
        ));
    }
    let default_button = gtk::Button::with_label(text("use_default_folder"));
    default_button.set_halign(gtk::Align::Start);
    folder_group.append(&default_button);
    page.append(&folder_group);

    let review_group = section(text("review_settings"));
    review_group.append(&field_row(text("exclude_dirs"), &controls.exclude_entry));

    let autostart_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let autostart_label = gtk::Label::new(Some(text("launch_sync_at_login")));
    autostart_label.set_xalign(0.0);
    autostart_label.set_hexpand(true);
    autostart_row.append(&autostart_label);
    autostart_row.append(&controls.autostart);
    review_group.append(&autostart_row);
    page.append(&review_group);

    let advanced_group = section(text("advanced_setup"));
    let advanced_hint = gtk::Label::new(Some(text("advanced_setup_hint")));
    advanced_hint.add_css_class("diskette-muted");
    advanced_hint.set_wrap(true);
    advanced_hint.set_xalign(0.0);
    advanced_group.append(&advanced_hint);
    advanced_group.append(&field_row(text("proxy_mode"), &controls.proxy_mode));
    advanced_group.append(&field_row(text("manual_proxy"), &controls.proxy_entry));
    page.append(&advanced_group);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::Start);
    let save_button = gtk::Button::with_label(text("save_configuration"));
    let token_button = gtk::Button::with_label(text("open_auth_page"));
    save_button.add_css_class("suggested-action");
    buttons.append(&save_button);
    buttons.append(&token_button);
    page.append(&buttons);

    {
        let sync_entry = controls.sync_entry.clone();
        default_button.connect_clicked(move |_| {
            sync_entry.set_text(&display_path(&SetupConfig::defaults().sync_dir));
        });
    }

    {
        let controls = controls.clone();
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        save_button.connect_clicked(move |_| {
            let setup = controls.read();
            append_output(
                &output_buffer,
                &format!("{}\n", text("saving_yandex_disk_configuration")),
            );
            let sender = sender.clone();
            thread::spawn(move || {
                let result = setup.save().map_err(|error| error.to_string());
                let _ = sender.send(UiEvent::SetupSaved(result));
            });
        });
    }

    {
        let controls = controls.clone();
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        token_button.connect_clicked(move |_| {
            request_token_after_setup(&sender, &output_buffer, &controls);
        });
    }

    page
}

pub(crate) fn request_token_after_setup(
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    controls: &SetupControls,
) {
    let setup = controls.read();
    append_output(
        output_buffer,
        &format!("{}\n", text("saving_yandex_disk_configuration")),
    );

    let sender = sender.clone();
    thread::spawn(move || {
        let auth_file = setup.auth_file.clone();
        let result = setup.save().map_err(|error| error.to_string());
        let _ = sender.send(UiEvent::TokenSetupSaved { auth_file, result });
    });
}
