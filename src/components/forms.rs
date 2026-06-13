use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::path::PathBuf;

pub(crate) fn entry_with_text(text: &str) -> gtk::Entry {
    let entry = gtk::Entry::new();
    entry.set_hexpand(true);
    entry.set_text(text);
    entry
}

pub(crate) fn optional_path(text: &glib::GString) -> Option<PathBuf> {
    optional_text(text).map(PathBuf::from)
}

pub(crate) fn optional_text(text: &glib::GString) -> Option<String> {
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_owned())
    }
}
