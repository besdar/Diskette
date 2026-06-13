use super::OptionControls;
use crate::components::append_output;
use crate::i18n::text;
use crate::models::yandex_disk::{CommandOptions, DiskRequest, UiEvent};
use crate::services::yandex_disk::run_request_with_progress;
use gtk::prelude::*;
use gtk4 as gtk;
use std::sync::mpsc;
use std::thread;

pub(crate) fn connect_command_button<F>(
    button: &gtk::Button,
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    option_controls: &OptionControls,
    build_request: F,
) where
    F: Fn(CommandOptions) -> DiskRequest + 'static,
{
    let sender = sender.clone();
    let output_buffer = output_buffer.clone();
    let option_controls = option_controls.clone();
    button.connect_clicked(move |_| {
        submit_request(
            &sender,
            &output_buffer,
            build_request(option_controls.read()),
        );
    });
}

pub(crate) fn submit_request(
    sender: &mpsc::Sender<UiEvent>,
    output_buffer: &gtk::TextBuffer,
    request: DiskRequest,
) {
    append_output(
        output_buffer,
        &format!("{} {}\n", text("queued"), request.command_line()),
    );
    let finish_sender = sender.clone();
    let progress_sender = finish_sender.clone();
    thread::spawn(move || {
        let output = run_request_with_progress(&request, move |progress| {
            let _ = progress_sender.send(UiEvent::CommandProgress(progress));
        });
        let _ = finish_sender.send(UiEvent::CommandFinished(output));
    });
}
