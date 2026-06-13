use std::env;
use std::path::Path;

/// Flatpak builds install Yandex Disk into the app prefix.
pub(super) const FLATPAK_YANDEX_BINARY: &str = "/app/bin/yandex-disk";

pub(crate) fn yandex_binary() -> String {
    if let Ok(path) = env::var("DISKETTE_YANDEX_DISK")
        && !path.trim().is_empty()
    {
        return path;
    }

    let flatpak_binary = Path::new(FLATPAK_YANDEX_BINARY);
    if flatpak_binary.exists() {
        return flatpak_binary.to_string_lossy().into_owned();
    }

    "yandex-disk".to_owned()
}

pub(crate) fn shell_quote(value: &str) -> String {
    if value.chars().all(|ch| {
        ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-' | '=' | ':' | ',')
    }) {
        value.to_owned()
    } else {
        format!("'{}'", value.replace('\'', r"'\''"))
    }
}
