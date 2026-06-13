use crate::components::yandex_disk::{OptionControls, submit_request};
use crate::components::{page_box, section};
use crate::i18n::text;
use crate::models::yandex_disk::{DiskRequest, UiEvent};
use crate::settings::COMMAND_DOCS_URL;
use crate::utils::open_uri;
use gtk::prelude::*;
use gtk4 as gtk;
use std::sync::mpsc;

pub(crate) fn build_advanced_page(
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
) -> gtk::Box {
    let page = page_box();
    page.append(&option_controls.widget());

    let group = section(text("reference"));
    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::Start);
    let help_button = gtk::Button::with_label(text("show_cli_help"));
    let command_docs = gtk::Button::with_label(text("command_docs"));
    buttons.append(&help_button);
    buttons.append(&command_docs);
    group.append(&buttons);
    page.append(&group);

    {
        let sender = sender.clone();
        let output_buffer = output_buffer.clone();
        help_button.connect_clicked(move |_| {
            submit_request(&sender, &output_buffer, DiskRequest::Help);
        });
    }

    command_docs.connect_clicked(move |_| {
        open_uri(COMMAND_DOCS_URL);
    });

    page
}
