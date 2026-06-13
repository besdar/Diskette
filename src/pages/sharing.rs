use crate::components::yandex_disk::{OptionControls, submit_request};
use crate::components::{
    append_output, field_row, file_or_folder_pick_row, optional_text, page_box, section,
};
use crate::i18n::text;
use crate::models::yandex_disk::{DiskRequest, UiEvent};
use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;
use std::sync::mpsc;

pub(crate) fn build_sharing_page(
    window: &adw::ApplicationWindow,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
) -> (gtk::Box, gtk::Entry) {
    let page = page_box();
    let group = section(text("public_links"));
    let path_entry = gtk::Entry::new();
    path_entry.set_hexpand(true);
    path_entry.set_placeholder_text(Some(text("file_or_folder_path")));
    group.append(&field_row(text("path"), &path_entry));
    group.append(&file_or_folder_pick_row(window, &path_entry));

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::Start);
    let publish = gtk::Button::with_label(text("publish"));
    let unpublish = gtk::Button::with_label(text("unpublish"));
    publish.add_css_class("suggested-action");
    buttons.append(&publish);
    buttons.append(&unpublish);
    group.append(&buttons);
    page.append(&group);

    let result_group = section(text("published_link"));
    let result_hint = gtk::Label::new(Some(text("published_link_hint")));
    result_hint.add_css_class("diskette-muted");
    result_hint.set_wrap(true);
    result_hint.set_xalign(0.0);
    result_group.append(&result_hint);

    let result_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let link_entry = gtk::Entry::new();
    link_entry.set_editable(false);
    link_entry.set_hexpand(true);
    link_entry.set_placeholder_text(Some(text("published_link_placeholder")));
    let copy_button = gtk::Button::with_label(text("copy_link"));
    result_row.append(&link_entry);
    result_row.append(&copy_button);
    result_group.append(&result_row);
    page.append(&result_group);

    {
        let link_entry = link_entry.clone();
        let output_buffer = output_buffer.clone();
        copy_button.connect_clicked(move |_| {
            let link = link_entry.text();
            if link.trim().is_empty() {
                return;
            }
            link_entry.display().clipboard().set_text(&link);
            append_output(
                &output_buffer,
                &format!("{}\n", text("published_link_copied")),
            );
        });
    }

    {
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        let option_controls = option_controls.clone();
        let path_entry = path_entry.clone();
        publish.connect_clicked(move |_| {
            let Some(path) = optional_text(&path_entry.text()) else {
                append_output(
                    &output_buffer,
                    &format!("{}\n", text("publish_requires_path")),
                );
                return;
            };
            submit_request(
                &sender,
                &output_buffer,
                DiskRequest::Publish {
                    options: option_controls.read(),
                    path,
                },
            );
        });
    }

    {
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        let option_controls = option_controls.clone();
        let path_entry = path_entry.clone();
        unpublish.connect_clicked(move |_| {
            let Some(path) = optional_text(&path_entry.text()) else {
                append_output(
                    &output_buffer,
                    &format!("{}\n", text("unpublish_requires_path")),
                );
                return;
            };
            submit_request(
                &sender,
                &output_buffer,
                DiskRequest::Unpublish {
                    options: option_controls.read(),
                    path,
                },
            );
        });
    }

    (page, link_entry)
}
