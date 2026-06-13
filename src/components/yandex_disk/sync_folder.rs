use crate::i18n::text;
use crate::models::yandex_disk::is_flatpak;
use gtk::prelude::*;
use gtk4 as gtk;

pub(super) fn lock_flatpak_sync_entry(entry: &gtk::Entry) {
    if is_flatpak() {
        entry.set_editable(false);
        entry.set_tooltip_text(Some(text("flatpak_sync_folder_locked")));
    }
}
