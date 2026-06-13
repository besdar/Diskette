use super::sync_folder::lock_flatpak_sync_entry;
use crate::components::{entry_with_text, optional_path, optional_text};
use crate::i18n::text;
use crate::models::yandex_disk::SetupConfig;
use crate::utils::display_path;
use gtk::prelude::*;
use gtk4 as gtk;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct SetupControls {
    pub(crate) sync_entry: gtk::Entry,
    pub(crate) auth_entry: gtk::Entry,
    pub(crate) exclude_entry: gtk::Entry,
    pub(crate) proxy_mode: gtk::DropDown,
    pub(crate) proxy_entry: gtk::Entry,
    pub(crate) autostart: gtk::Switch,
}

impl SetupControls {
    pub(crate) fn new(config: &SetupConfig) -> Self {
        let controls = Self {
            sync_entry: entry_with_text(&display_path(&config.sync_dir)),
            auth_entry: entry_with_text(&display_path(&config.auth_file)),
            exclude_entry: entry_with_text(config.exclude_dirs.as_deref().unwrap_or_default()),
            proxy_mode: gtk::DropDown::from_strings(&[text("auto"), text("no"), text("manual")]),
            proxy_entry: entry_with_text(config.proxy.as_deref().unwrap_or_default()),
            autostart: gtk::Switch::builder().active(config.autostart).build(),
        };

        lock_flatpak_sync_entry(&controls.sync_entry);

        if let Some(proxy) = config.proxy.as_deref() {
            match proxy.to_ascii_lowercase().as_str() {
                "no" => controls.proxy_mode.set_selected(1),
                value if value != "auto" => controls.proxy_mode.set_selected(2),
                _ => controls.proxy_mode.set_selected(0),
            }
        }

        controls
    }

    pub(crate) fn read(&self) -> SetupConfig {
        let defaults = SetupConfig::defaults();
        SetupConfig {
            sync_dir: optional_path(&self.sync_entry.text()).unwrap_or(defaults.sync_dir),
            auth_file: optional_path(&self.auth_entry.text()).unwrap_or(defaults.auth_file),
            exclude_dirs: optional_text(&self.exclude_entry.text()),
            proxy: setup_proxy_value(&self.proxy_mode, &self.proxy_entry),
            autostart: self.autostart.is_active(),
        }
    }

    pub(crate) fn sync_dir_or_default(&self) -> PathBuf {
        optional_path(&self.sync_entry.text()).unwrap_or_else(|| SetupConfig::defaults().sync_dir)
    }
}

fn setup_proxy_value(mode: &gtk::DropDown, manual: &gtk::Entry) -> Option<String> {
    match mode.selected() {
        0 => Some("auto".to_owned()),
        1 => Some("no".to_owned()),
        2 => optional_text(&manual.text()),
        _ => None,
    }
}
