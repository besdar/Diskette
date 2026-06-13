use std::env;
use std::path::Path;

pub(crate) fn yandex_binary() -> String {
    if let Ok(path) = env::var("DISKETTE_YANDEX_DISK") {
        if !path.trim().is_empty() {
            return path;
        }
    }

    let flatpak_extra = Path::new("/app/extra/usr/bin/yandex-disk");
    if flatpak_extra.exists() {
        return flatpak_extra.to_string_lossy().into_owned();
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
