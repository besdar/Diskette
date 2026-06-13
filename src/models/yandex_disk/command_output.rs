use super::DiskRequest;
use crate::i18n::text;

#[derive(Clone, Debug)]
pub(crate) struct DiskOutput {
    pub(crate) label: String,
    pub(crate) command_line: String,
    pub(crate) success: bool,
    pub(crate) exit_code: Option<i32>,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    streamed: bool,
}

impl DiskOutput {
    pub(crate) fn new(
        label: String,
        command_line: String,
        success: bool,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
        streamed: bool,
    ) -> Self {
        Self {
            label,
            command_line,
            success,
            exit_code,
            stdout,
            stderr,
            streamed,
        }
    }

    pub(crate) fn for_request(
        request: &DiskRequest,
        success: bool,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    ) -> Self {
        Self::new(
            request.label().to_owned(),
            request.command_line(),
            success,
            exit_code,
            stdout,
            stderr,
            false,
        )
    }

    pub(crate) fn summary(&self) -> String {
        if self.success {
            first_non_empty_line(&self.stdout)
                .or_else(|| first_non_empty_line(&self.stderr))
                .unwrap_or_else(|| text("command_finished_successfully").to_owned())
        } else {
            first_non_empty_line(&self.stderr)
                .or_else(|| first_non_empty_line(&self.stdout))
                .unwrap_or_else(|| text("command_failed_sentence").to_owned())
        }
    }

    pub(crate) fn format_for_log(&self) -> String {
        let status = if self.success {
            text("success")
        } else {
            text("failed")
        };
        let code = self.exit_code.map_or_else(
            || text("no_exit_code").to_owned(),
            |code| format!("exit {code}"),
        );
        let mut log = format!(
            "$ {}\n[{status}; {code}; {}]\n",
            self.command_line, self.label
        );

        if self.streamed {
            return log;
        }

        if !self.stdout.trim().is_empty() {
            log.push('\n');
            log.push_str(text("stdout_label"));
            log.push('\n');
            log.push_str(self.stdout.trim_end());
            log.push('\n');
        }

        if !self.stderr.trim().is_empty() {
            log.push('\n');
            log.push_str(text("stderr_label"));
            log.push('\n');
            log.push_str(self.stderr.trim_end());
            log.push('\n');
        }

        log
    }

    pub(crate) fn failed(label: &str, command_line: String, error: &anyhow::Error) -> Self {
        Self {
            label: label.to_owned(),
            command_line,
            success: false,
            exit_code: None,
            stdout: String::new(),
            stderr: error.to_string(),
            streamed: false,
        }
    }
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}
