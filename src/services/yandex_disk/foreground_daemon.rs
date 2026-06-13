use anyhow::{Context, Result};
use std::process::{Command, Stdio};

use crate::i18n::text;
use crate::models::yandex_disk::{
    CommandOptions, DiskOutput, DiskRequest, shell_quote, yandex_binary,
};

pub(super) fn spawn_foreground_daemon(options: &CommandOptions) -> Result<DiskOutput> {
    let request = DiskRequest::ForegroundDaemon(options.clone());
    let binary = yandex_binary();
    let args = request.args();
    let child = Command::new(&binary)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("failed to launch {}", shell_quote(&binary)))?;

    Ok(DiskOutput::new(
        request.label().to_owned(),
        request.command_line(),
        true,
        None,
        format!(
            "{} {}.",
            text("foreground_daemon_launched_with_pid"),
            child.id()
        ),
        String::new(),
        false,
    ))
}
