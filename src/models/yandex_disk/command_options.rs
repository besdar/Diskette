use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub(crate) struct CommandOptions {
    pub(crate) config_file: Option<PathBuf>,
    pub(crate) sync_dir: Option<PathBuf>,
    pub(crate) auth_file: Option<PathBuf>,
    pub(crate) exclude_dirs: Option<String>,
    pub(crate) proxy: Option<String>,
    pub(crate) read_only: bool,
    pub(crate) overwrite: bool,
}

impl CommandOptions {
    pub(crate) fn append_args(&self, args: &mut Vec<String>) {
        push_path_arg(args, "--config", self.config_file.as_deref());
        push_path_arg(args, "--dir", self.sync_dir.as_deref());
        push_path_arg(args, "--auth", self.auth_file.as_deref());
        push_value_arg(args, "--exclude-dirs", self.exclude_dirs.as_deref());
        push_value_arg(args, "--proxy", self.proxy.as_deref());

        if self.read_only {
            args.push("--read-only".to_owned());
        }

        if self.overwrite {
            args.push("--overwrite".to_owned());
        }
    }

    pub(crate) fn ensure_sync_dir(&self) -> Result<()> {
        if let Some(path) = non_empty_path(self.sync_dir.as_deref()) {
            fs::create_dir_all(path)
                .with_context(|| format!("failed to create sync directory {}", path.display()))?;
        }

        Ok(())
    }
}

pub(crate) fn non_empty_path(path: Option<&Path>) -> Option<&Path> {
    path.filter(|path| !path.as_os_str().is_empty())
}

fn push_path_arg(args: &mut Vec<String>, name: &str, path: Option<&Path>) {
    if let Some(path) = non_empty_path(path) {
        args.push(format!("{name}={}", path.to_string_lossy()));
    }
}

fn push_value_arg(args: &mut Vec<String>, name: &str, value: Option<&str>) {
    if let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) {
        args.push(format!("{name}={value}"));
    }
}
