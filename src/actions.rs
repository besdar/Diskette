use crate::components::append_output;
use crate::components::yandex_disk::submit_request;
use crate::i18n::text;
use crate::models::yandex_disk::{DiskOutput, DiskRequest, SetupSave, UiEvent};
use crate::utils::open_uri_result;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::sync::mpsc;
use std::time::Duration;

pub(crate) struct EventPumpUi {
    pub(crate) output_buffer: gtk::TextBuffer,
    pub(crate) status_title: gtk::Label,
    pub(crate) status_detail: gtk::Label,
    pub(crate) header_status_indicator: gtk::Box,
    pub(crate) header_status_label: gtk::Label,
    pub(crate) activity_title: gtk::Label,
    pub(crate) activity_detail: gtk::Label,
    pub(crate) publish_link_entry: gtk::Entry,
    pub(crate) stack: gtk::Stack,
    pub(crate) main_navigation: gtk::StackSwitcher,
    pub(crate) refresh_button: gtk::Button,
}

pub(crate) fn attach_event_pump(
    receiver: mpsc::Receiver<UiEvent>,
    sender: mpsc::Sender<UiEvent>,
    ui: EventPumpUi,
) {
    glib::timeout_add_local(Duration::from_millis(120), move || {
        while let Ok(event) = receiver.try_recv() {
            match event {
                UiEvent::CommandProgress(progress) => {
                    append_output(&ui.output_buffer, &progress.text);
                    if let Some(uri) = progress.auth_url {
                        match open_uri_result(&uri) {
                            Ok(()) => append_output(
                                &ui.output_buffer,
                                &format!("{} {uri}\n", text("opening_authentication_page")),
                            ),
                            Err(error) => append_output(
                                &ui.output_buffer,
                                &format!(
                                    "{} {uri}: {error}\n",
                                    text("failed_to_open_authentication_page")
                                ),
                            ),
                        }
                    }
                }
                UiEvent::CommandFinished(output) => {
                    let completed_token = output.label == "token" && output.success;
                    append_output(&ui.output_buffer, &output.format_for_log());
                    update_status(
                        &ui.status_title,
                        &ui.status_detail,
                        &ui.header_status_indicator,
                        &ui.header_status_label,
                        &output,
                    );
                    update_activity(&ui.activity_title, &ui.activity_detail, &output);
                    update_publish_link(&ui.publish_link_entry, &output);
                    if completed_token {
                        ui.main_navigation.set_visible(true);
                        ui.refresh_button.set_visible(true);
                        ui.stack.set_visible_child_name("overview");
                    }
                }
                UiEvent::SetupSaved(Ok(save)) => {
                    append_setup_save(&ui.output_buffer, &save);
                }
                UiEvent::SetupSaved(Err(error)) => {
                    append_output(
                        &ui.output_buffer,
                        &format!("{} {error}\n", text("configuration_save_failed")),
                    );
                }
                UiEvent::TokenSetupSaved { auth_file, result } => match result {
                    Ok(save) => {
                        append_setup_save(&ui.output_buffer, &save);
                        submit_request(
                            &sender,
                            &ui.output_buffer,
                            DiskRequest::Token {
                                auth_file: Some(auth_file),
                            },
                        );
                    }
                    Err(error) => {
                        append_output(
                            &ui.output_buffer,
                            &format!("{} {error}\n", text("configuration_save_failed")),
                        );
                    }
                },
            }
        }

        glib::ControlFlow::Continue
    });
}

fn append_setup_save(output_buffer: &gtk::TextBuffer, save: &SetupSave) {
    let autostart = if save.autostart_enabled {
        format!(
            "{} {}",
            text("autostart_file"),
            save.autostart_file.display()
        )
    } else {
        text("autostart_disabled").to_owned()
    };
    append_output(
        output_buffer,
        &format!(
            "{} {}\n{autostart}\n",
            text("configuration_saved"),
            save.config_file.display()
        ),
    );
}

fn update_status(
    status_title: &gtk::Label,
    status_detail: &gtk::Label,
    header_status_indicator: &gtk::Box,
    header_status_label: &gtk::Label,
    output: &DiskOutput,
) {
    match output.label.as_str() {
        "token" if output.success => {
            status_title.set_text(text("yandex_disk_connected"));
            status_detail.set_text(text("start_or_status_after_token"));
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("connected"),
                "diskette-status-success",
            );
        }
        "token" => {
            status_title.set_text(text("authentication_failed"));
            status_detail.set_text(&output.summary());
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("error"),
                "diskette-status-error",
            );
        }
        "status" if output.success => {
            status_title.set_text(text("daemon_status"));
            status_detail.set_text(&output.summary());
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("daemon_status"),
                "diskette-status-success",
            );
        }
        "status" => {
            status_title.set_text(text("status_check_failed"));
            status_detail.set_text(&output.summary());
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("error"),
                "diskette-status-error",
            );
        }
        "start" | "stop" | "sync" | "foreground daemon" if output.success => {
            status_title.set_text(text("command_finished"));
            status_detail.set_text(&output.summary());
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("command_finished"),
                "diskette-status-warning",
            );
        }
        "start" | "stop" | "sync" | "foreground daemon" => {
            status_title.set_text(text("command_failed"));
            status_detail.set_text(&output.summary());
            set_visual_status(
                header_status_indicator,
                header_status_label,
                text("error"),
                "diskette-status-error",
            );
        }
        _ => {}
    }
}

fn update_activity(activity_title: &gtk::Label, activity_detail: &gtk::Label, output: &DiskOutput) {
    activity_title.set_text(&format!("{} {}", text("last_action"), output.label));
    activity_detail.set_text(&output.summary());
}

fn update_publish_link(publish_link_entry: &gtk::Entry, output: &DiskOutput) {
    if output.label != "publish" || !output.success {
        return;
    }

    if let Some(url) = first_url(&output.stdout).or_else(|| first_url(&output.stderr)) {
        publish_link_entry.set_text(&url);
    }
}

fn first_url(output: &str) -> Option<String> {
    output
        .split_whitespace()
        .map(|candidate| {
            candidate
                .trim_matches(|character| matches!(character, '"' | '\'' | '(' | ')' | ',' | '.'))
        })
        .find(|candidate| candidate.starts_with("https://") || candidate.starts_with("http://"))
        .map(ToOwned::to_owned)
}

fn set_visual_status(
    indicator: &gtk::Box,
    label: &gtk::Label,
    text_value: &str,
    status_class: &str,
) {
    for class in [
        "diskette-status-idle",
        "diskette-status-success",
        "diskette-status-warning",
        "diskette-status-error",
    ] {
        indicator.remove_css_class(class);
    }

    indicator.add_css_class(status_class);
    label.set_text(text_value);
}
