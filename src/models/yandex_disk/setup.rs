use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::setup_storage::{
    autostart_contents, autostart_file, config_file, default_sync_dir, is_flatpak, parse_value,
    quote_value, yandex_config_dir,
};

#[derive(Clone, Debug)]
pub(crate) struct SetupConfig {
    pub(crate) auth_file: PathBuf,
    pub(crate) sync_dir: PathBuf,
    pub(crate) exclude_dirs: Option<String>,
    pub(crate) proxy: Option<String>,
    pub(crate) autostart: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct SetupSave {
    pub(crate) config_file: PathBuf,
    pub(crate) autostart_file: PathBuf,
    pub(crate) autostart_enabled: bool,
}

impl SetupConfig {
    pub(crate) fn load() -> Self {
        let mut config = Self::defaults();
        let path = config_file();

        if let Ok(contents) = fs::read_to_string(path) {
            if let Some(auth) = parse_value(&contents, "auth") {
                config.auth_file = PathBuf::from(auth);
            }
            if let Some(dir) = parse_value(&contents, "dir") {
                config.sync_dir = PathBuf::from(dir);
            }
            config.exclude_dirs = parse_value(&contents, "exclude-dirs");
            config.proxy = parse_value(&contents, "proxy");
        }

        if is_flatpak() {
            config.sync_dir = default_sync_dir();
        }

        config.autostart = autostart_file().exists();
        config
    }

    pub(crate) fn defaults() -> Self {
        Self {
            auth_file: yandex_config_dir().join("passwd"),
            sync_dir: default_sync_dir(),
            exclude_dirs: None,
            proxy: None,
            autostart: false,
        }
    }

    pub(crate) fn save(&self) -> Result<SetupSave> {
        let config = self.normalized();
        let config_dir = yandex_config_dir();
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("failed to create {}", config_dir.display()))?;

        if let Some(parent) = config.auth_file.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        fs::create_dir_all(&config.sync_dir)
            .with_context(|| format!("failed to create {}", config.sync_dir.display()))?;

        let path = config_file();
        fs::write(&path, config.config_contents())
            .with_context(|| format!("failed to write {}", path.display()))?;

        let autostart = autostart_file();
        if config.autostart {
            if let Some(parent) = autostart.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            fs::write(&autostart, autostart_contents())
                .with_context(|| format!("failed to write {}", autostart.display()))?;
        } else if autostart.exists() {
            fs::remove_file(&autostart)
                .with_context(|| format!("failed to remove {}", autostart.display()))?;
        }

        Ok(SetupSave {
            config_file: path,
            autostart_file: autostart,
            autostart_enabled: config.autostart,
        })
    }

    fn normalized(&self) -> Self {
        let mut config = self.clone();
        if is_flatpak() {
            config.sync_dir = default_sync_dir();
        }
        config
    }

    fn config_contents(&self) -> String {
        let mut lines = vec![
            "# Path to the authorization data file".to_owned(),
            format!("auth={}", quote_value(&self.auth_file.to_string_lossy())),
            String::new(),
            "# Directory for storing a local copy of Yandex Disk.".to_owned(),
            format!("dir={}", quote_value(&self.sync_dir.to_string_lossy())),
        ];

        if let Some(exclude_dirs) = self
            .exclude_dirs
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            lines.push(String::new());
            lines.push("# Do not sync the specified directories.".to_owned());
            lines.push(format!("exclude-dirs={}", quote_value(exclude_dirs)));
        }

        if let Some(proxy) = self
            .proxy
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            lines.push(String::new());
            lines.push(
                "# Proxy server: auto, no, or protocol,address,port,login,password.".to_owned(),
            );
            lines.push(format!("proxy={}", quote_value(proxy)));
        }

        lines.push(String::new());
        lines.join("\n")
    }
}
