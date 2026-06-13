use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use super::foreground_daemon::spawn_foreground_daemon;
use super::token_stream::run_token;
use crate::models::yandex_disk::{
    DiskOutput, DiskProgress, DiskRequest, shell_quote, yandex_binary,
};

pub(crate) fn run_request_with_progress<F>(request: &DiskRequest, mut progress: F) -> DiskOutput
where
    F: FnMut(DiskProgress),
{
    let label = request.label();
    let command_line = request.command_line();

    let result = prepare_request(request).and_then(|()| match request {
        DiskRequest::Token { auth_file } => {
            run_token(auth_file.as_deref(), &command_line, &mut progress)
        }
        DiskRequest::ForegroundDaemon(options) => spawn_foreground_daemon(options),
        _ => run_blocking(request),
    });

    result.unwrap_or_else(|error| DiskOutput::failed(label, command_line, &error))
}

fn prepare_request(request: &DiskRequest) -> Result<()> {
    match request {
        DiskRequest::Token { auth_file } => ensure_parent(auth_file.as_deref()),
        DiskRequest::Start(options)
        | DiskRequest::Status(options)
        | DiskRequest::Stop(options)
        | DiskRequest::Sync(options)
        | DiskRequest::ForegroundDaemon(options) => options.ensure_sync_dir(),
        DiskRequest::Publish { options, .. } | DiskRequest::Unpublish { options, .. } => {
            options.ensure_sync_dir()
        }
        DiskRequest::Help => Ok(()),
    }
}

fn run_blocking(request: &DiskRequest) -> Result<DiskOutput> {
    let binary = yandex_binary();
    let args = request.args();
    let output = Command::new(&binary)
        .args(&args)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("failed to run {}", shell_quote(&binary)))?;

    Ok(DiskOutput::for_request(
        request,
        output.status.success(),
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}

fn ensure_parent(path: Option<&Path>) -> Result<()> {
    if let Some(parent) = path.and_then(Path::parent) {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    Ok(())
}
