use super::command_line::FLATPAK_YANDEX_BINARY;
use crate::settings::APP_ID;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn config_file() -> PathBuf {
    yandex_config_dir().join("config.cfg")
}

pub(super) fn yandex_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| home_dir().join(".config"))
        .join("yandex-disk")
}

pub(super) fn default_sync_dir() -> PathBuf {
    home_dir().join("Yandex.Disk")
}

pub(crate) fn sync_dir_open_path(sync_dir: &Path) -> PathBuf {
    if is_flatpak() && sync_dir == default_sync_dir() {
        flatpak_persisted_sync_dir()
    } else {
        sync_dir.to_path_buf()
    }
}

pub(crate) fn flatpak_persisted_sync_dir() -> PathBuf {
    let app_id = env::var("FLATPAK_ID").unwrap_or_else(|_| APP_ID.to_owned());
    home_dir()
        .join(".var")
        .join("app")
        .join(app_id)
        .join("Yandex.Disk")
}

pub(crate) fn is_flatpak() -> bool {
    env::var("FLATPAK_ID").is_ok()
}

pub(super) fn autostart_file() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| home_dir().join(".config"))
        .join("autostart")
        .join(format!("{APP_ID}.yandex-disk.desktop"))
}

pub(super) fn autostart_contents() -> String {
    let exec = if env::var("FLATPAK_ID").is_ok() {
        format!("flatpak run --command={FLATPAK_YANDEX_BINARY} {APP_ID} start")
    } else {
        "yandex-disk start".to_owned()
    };

    format!(
        "[Desktop Entry]\nType=Application\nName=Yandex Disk Sync\nExec={exec}\nNoDisplay=true\nX-GNOME-Autostart-enabled=true\n"
    )
}

pub(super) fn parse_value(contents: &str, key: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            return None;
        }

        let (line_key, value) = trimmed.split_once('=')?;
        if line_key.trim() == key {
            Some(unquote_value(value.trim()))
        } else {
            None
        }
    })
}

pub(super) fn quote_value(value: &str) -> String {
    let escaped = value.replace('\\', r"\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

fn unquote_value(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        value[1..value.len() - 1]
            .replace("\\\"", "\"")
            .replace(r"\\", r"\")
    } else {
        value.to_owned()
    }
}
