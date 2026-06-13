use crate::i18n::text;
use crate::utils::display_path;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

#[derive(Clone, Copy)]
enum PickerAction {
    OpenFile,
    SelectFolder,
}

pub(crate) fn folder_pick_row(
    window: &adw::ApplicationWindow,
    title: &'static str,
    entry: &gtk::Entry,
) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_halign(gtk::Align::End);
    let button = gtk::Button::with_label(text("choose"));
    row.append(&button);
    connect_picker(&button, window, title, entry, PickerAction::SelectFolder);
    row
}

pub(crate) fn file_pick_row(
    window: &adw::ApplicationWindow,
    title: &'static str,
    entry: &gtk::Entry,
) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_halign(gtk::Align::End);
    let button = gtk::Button::with_label(text("choose"));
    row.append(&button);
    connect_picker(&button, window, title, entry, PickerAction::OpenFile);
    row
}

pub(crate) fn file_or_folder_pick_row(
    window: &adw::ApplicationWindow,
    entry: &gtk::Entry,
) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_halign(gtk::Align::End);
    let file_button = gtk::Button::with_label(text("choose_file"));
    let folder_button = gtk::Button::with_label(text("choose_folder"));
    row.append(&file_button);
    row.append(&folder_button);
    connect_picker(
        &file_button,
        window,
        text("choose_file_to_publish"),
        entry,
        PickerAction::OpenFile,
    );
    connect_picker(
        &folder_button,
        window,
        text("choose_folder_to_publish"),
        entry,
        PickerAction::SelectFolder,
    );
    row
}

fn connect_picker(
    button: &gtk::Button,
    window: &adw::ApplicationWindow,
    title: &'static str,
    entry: &gtk::Entry,
    action: PickerAction,
) {
    let window = window.clone();
    let entry = entry.clone();
    button.connect_clicked(move |_| {
        let dialog = gtk::FileDialog::builder().title(title).modal(true).build();
        let entry = entry.clone();

        match action {
            PickerAction::OpenFile => {
                dialog.open(Some(&window), None::<&gio::Cancellable>, move |result| {
                    if let Ok(path) = result.and_then(|file| {
                        file.path().ok_or_else(|| {
                            glib::Error::new(
                                gio::IOErrorEnum::NotFound,
                                text("selected_file_no_local_path"),
                            )
                        })
                    }) {
                        entry.set_text(&display_path(&path));
                    }
                });
            }
            PickerAction::SelectFolder => {
                dialog.select_folder(Some(&window), None::<&gio::Cancellable>, move |result| {
                    if let Ok(path) = result.and_then(|file| {
                        file.path().ok_or_else(|| {
                            glib::Error::new(
                                gio::IOErrorEnum::NotFound,
                                text("selected_folder_no_local_path"),
                            )
                        })
                    }) {
                        entry.set_text(&display_path(&path));
                    }
                });
            }
        }
    });
}
