use super::command_line::{shell_quote, yandex_binary};
use super::command_options::{CommandOptions, non_empty_path};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) enum DiskRequest {
    Token {
        auth_file: Option<PathBuf>,
    },
    Start(CommandOptions),
    Status(CommandOptions),
    Stop(CommandOptions),
    Sync(CommandOptions),
    Publish {
        options: CommandOptions,
        path: String,
    },
    Unpublish {
        options: CommandOptions,
        path: String,
    },
    ForegroundDaemon(CommandOptions),
    Help,
}

impl DiskRequest {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Token { .. } => "token",
            Self::Start(_) => "start",
            Self::Status(_) => "status",
            Self::Stop(_) => "stop",
            Self::Sync(_) => "sync",
            Self::Publish { .. } => "publish",
            Self::Unpublish { .. } => "unpublish",
            Self::ForegroundDaemon(_) => "foreground daemon",
            Self::Help => "help",
        }
    }

    pub(crate) fn args(&self) -> Vec<String> {
        let mut args = Vec::new();

        match self {
            Self::Token { auth_file } => {
                args.push("token".to_owned());
                if let Some(path) = non_empty_path(auth_file.as_deref()) {
                    args.push(path.to_string_lossy().into_owned());
                }
            }
            Self::Start(options) => {
                args.push("start".to_owned());
                options.append_args(&mut args);
            }
            Self::Status(options) => {
                args.push("status".to_owned());
                options.append_args(&mut args);
            }
            Self::Stop(options) => {
                args.push("stop".to_owned());
                options.append_args(&mut args);
            }
            Self::Sync(options) => {
                args.push("sync".to_owned());
                options.append_args(&mut args);
            }
            Self::Publish { options, path } => {
                args.push("publish".to_owned());
                options.append_args(&mut args);
                args.push(path.clone());
            }
            Self::Unpublish { options, path } => {
                args.push("unpublish".to_owned());
                options.append_args(&mut args);
                args.push(path.clone());
            }
            Self::ForegroundDaemon(options) => {
                args.push("--no-daemon".to_owned());
                options.append_args(&mut args);
            }
            Self::Help => {
                args.push("--help".to_owned());
            }
        }

        args
    }

    pub(crate) fn command_line(&self) -> String {
        let mut parts = vec![shell_quote(&yandex_binary())];
        parts.extend(self.args().iter().map(|arg| shell_quote(arg)));
        parts.join(" ")
    }
}
