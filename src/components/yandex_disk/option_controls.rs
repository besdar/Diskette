use super::sync_folder::lock_flatpak_sync_entry;
use crate::components::{entry_with_text, field_row, optional_path, optional_text, section};
use crate::i18n::text;
use crate::models::yandex_disk::{CommandOptions, SetupConfig, config_file};
use crate::utils::display_path;
use gtk::prelude::*;
use gtk4 as gtk;

#[derive(Clone)]
pub(crate) struct OptionControls {
    config_entry: gtk::Entry,
    sync_entry: gtk::Entry,
    auth_entry: gtk::Entry,
    exclude_entry: gtk::Entry,
    proxy_mode: gtk::DropDown,
    proxy_entry: gtk::Entry,
    read_only: gtk::CheckButton,
    overwrite: gtk::CheckButton,
}

impl OptionControls {
    pub(crate) fn new(config: &SetupConfig) -> Self {
        let controls = Self {
            config_entry: entry_with_text(&display_path(&config_file())),
            sync_entry: entry_with_text(&display_path(&config.sync_dir)),
            auth_entry: entry_with_text(&display_path(&config.auth_file)),
            exclude_entry: entry_with_text(config.exclude_dirs.as_deref().unwrap_or_default()),
            proxy_mode: gtk::DropDown::from_strings(&[
                text("default"),
                text("auto"),
                text("no"),
                text("manual"),
            ]),
            proxy_entry: entry_with_text(config.proxy.as_deref().unwrap_or_default()),
            read_only: gtk::CheckButton::with_label(text("read_only_sync")),
            overwrite: gtk::CheckButton::with_label(text("overwrite_local_conflicts")),
        };

        lock_flatpak_sync_entry(&controls.sync_entry);

        if let Some(proxy) = config.proxy.as_deref() {
            match proxy.to_ascii_lowercase().as_str() {
                "auto" => controls.proxy_mode.set_selected(1),
                "no" => controls.proxy_mode.set_selected(2),
                _ => controls.proxy_mode.set_selected(3),
            }
        }

        controls
    }

    pub(crate) fn read(&self) -> CommandOptions {
        CommandOptions {
            config_file: optional_path(&self.config_entry.text()),
            sync_dir: optional_path(&self.sync_entry.text()),
            auth_file: optional_path(&self.auth_entry.text()),
            exclude_dirs: optional_text(&self.exclude_entry.text()),
            proxy: command_proxy_value(&self.proxy_mode, &self.proxy_entry),
            read_only: self.read_only.is_active(),
            overwrite: self.overwrite.is_active(),
        }
    }

    pub(crate) fn widget(&self) -> gtk::Box {
        let groups = gtk::Box::new(gtk::Orientation::Vertical, 14);

        let paths = section(text("paths"));
        paths.append(&field_row(text("config_file"), &self.config_entry));
        paths.append(&field_row(text("sync_folder"), &self.sync_entry));
        paths.append(&field_row(text("auth_file"), &self.auth_entry));
        groups.append(&paths);

        let sync = section(text("sync_settings"));
        sync.append(&field_row(text("exclude_dirs"), &self.exclude_entry));
        sync.append(&self.read_only);
        sync.append(&self.overwrite);
        groups.append(&sync);

        let network = section(text("advanced"));
        network.append(&field_row(text("proxy_mode"), &self.proxy_mode));
        network.append(&field_row(text("manual_proxy"), &self.proxy_entry));
        groups.append(&network);

        groups
    }
}

fn command_proxy_value(mode: &gtk::DropDown, manual: &gtk::Entry) -> Option<String> {
    match mode.selected() {
        1 => Some("auto".to_owned()),
        2 => Some("no".to_owned()),
        3 => optional_text(&manual.text()),
        _ => None,
    }
}
