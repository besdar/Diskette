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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StorageStatus {
    pub(crate) path: Option<String>,
    pub(crate) total: String,
    pub(crate) used: String,
    pub(crate) available: String,
    pub(crate) max_file_size: Option<String>,
    pub(crate) trash_size: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DaemonStatus {
    Running,
    Stopped,
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

    pub(crate) fn storage_status(&self) -> Option<StorageStatus> {
        if self.label == "status" && self.success {
            StorageStatus::parse(&self.stdout)
        } else {
            None
        }
    }

    pub(crate) fn daemon_status(&self) -> Option<DaemonStatus> {
        match self.label.as_str() {
            "status" if self.success => Some(DaemonStatus::Running),
            "status" => Some(DaemonStatus::Stopped),
            "start" | "foreground daemon" if self.success => Some(DaemonStatus::Running),
            "stop" if self.success => Some(DaemonStatus::Stopped),
            "start" | "stop" | "foreground daemon" => {
                daemon_status_from_output(&self.stdout, &self.stderr)
            }
            _ => None,
        }
    }
}

impl StorageStatus {
    fn parse(output: &str) -> Option<Self> {
        let mut path = None;
        let mut total = None;
        let mut used = None;
        let mut available = None;
        let mut max_file_size = None;
        let mut trash_size = None;

        for line in output.lines().map(str::trim) {
            if let Some(value) = line.strip_prefix("Path to Yandex.Disk directory:") {
                path = Some(unquote_cli_value(value.trim()));
                continue;
            }

            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            let value = value.trim().to_owned();

            match key.trim().to_ascii_lowercase().as_str() {
                "total" => total = Some(value),
                "used" => used = Some(value),
                "available" => available = Some(value),
                "max file size" => max_file_size = Some(value),
                "trash size" => trash_size = Some(value),
                _ => {}
            }
        }

        Some(Self {
            path,
            total: total?,
            used: used?,
            available: available?,
            max_file_size,
            trash_size,
        })
    }

    pub(crate) fn used_fraction(&self) -> Option<f64> {
        let used = parse_size_to_bytes(&self.used)?;
        let total = parse_size_to_bytes(&self.total)?;

        if total > 0.0 {
            Some((used / total).clamp(0.0, 1.0))
        } else {
            None
        }
    }
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

fn unquote_cli_value(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 && value.starts_with('\'') && value.ends_with('\'') {
        value[1..value.len() - 1].to_owned()
    } else {
        value.to_owned()
    }
}

fn parse_size_to_bytes(value: &str) -> Option<f64> {
    let mut parts = value.split_whitespace();
    let number = parts.next()?.replace(',', ".").parse::<f64>().ok()?;
    let unit = parts.next().unwrap_or("B").to_ascii_lowercase();
    let multiplier = match unit.as_str() {
        "b" | "byte" | "bytes" => 1.0,
        "kb" | "kib" => 1024.0,
        "mb" | "mib" => 1024.0_f64.powi(2),
        "gb" | "gib" => 1024.0_f64.powi(3),
        "tb" | "tib" => 1024.0_f64.powi(4),
        _ => return None,
    };

    Some(number * multiplier)
}

fn daemon_status_from_output(stdout: &str, stderr: &str) -> Option<DaemonStatus> {
    let output = format!("{stdout}\n{stderr}").to_ascii_lowercase();

    if output.contains("daemon is not running")
        || output.contains("daemon not running")
        || output.contains("not running")
        || output.contains("not started")
        || output.contains("демон не запущ")
        || output.contains("не запущен")
    {
        return Some(DaemonStatus::Stopped);
    }

    if output.contains("synchronization core status:")
        || output.contains("daemon is running")
        || output.contains("already running")
        || output.contains("демон уже запущ")
        || output.contains("демон запущ")
    {
        return Some(DaemonStatus::Running);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{DaemonStatus, DiskOutput};

    #[test]
    fn parses_storage_status_from_yandex_disk_status_output() {
        let output = DiskOutput::new(
            "status".to_owned(),
            "yandex-disk status".to_owned(),
            true,
            Some(0),
            "Synchronization core status: idle
Path to Yandex.Disk directory: '/home/user/Yandex.Disk'
\tTotal: 30 GB
\tUsed: 38.93 MB
\tAvailable: 29.96 GB
\tMax file size: 1024 MB
\tTrash size: 0 B
"
            .to_owned(),
            String::new(),
            false,
        );

        let storage = output.storage_status().expect("storage details");

        assert_eq!(storage.path.as_deref(), Some("/home/user/Yandex.Disk"));
        assert_eq!(storage.total, "30 GB");
        assert_eq!(storage.used, "38.93 MB");
        assert_eq!(storage.available, "29.96 GB");
        assert_eq!(storage.max_file_size.as_deref(), Some("1024 MB"));
        assert_eq!(storage.trash_size.as_deref(), Some("0 B"));
        assert!(storage.used_fraction().is_some_and(|value| value > 0.0));
    }

    #[test]
    fn ignores_non_status_output_for_storage_status() {
        let output = DiskOutput::new(
            "sync".to_owned(),
            "yandex-disk sync".to_owned(),
            true,
            Some(0),
            "Directory synced".to_owned(),
            String::new(),
            false,
        );

        assert_eq!(output.storage_status(), None);
    }

    #[test]
    fn successful_status_means_daemon_is_running() {
        let output = DiskOutput::new(
            "status".to_owned(),
            "yandex-disk status".to_owned(),
            true,
            Some(0),
            "Synchronization core status: idle".to_owned(),
            String::new(),
            false,
        );

        assert_eq!(output.daemon_status(), Some(DaemonStatus::Running));
    }

    #[test]
    fn failed_status_means_daemon_is_stopped() {
        let output = DiskOutput::new(
            "status".to_owned(),
            "yandex-disk status".to_owned(),
            false,
            Some(1),
            String::new(),
            "Error: daemon is not running".to_owned(),
            false,
        );

        assert_eq!(output.daemon_status(), Some(DaemonStatus::Stopped));
    }

    #[test]
    fn daemon_control_commands_report_expected_states() {
        let started = DiskOutput::new(
            "start".to_owned(),
            "yandex-disk start".to_owned(),
            true,
            Some(0),
            String::new(),
            String::new(),
            false,
        );
        let stopped = DiskOutput::new(
            "stop".to_owned(),
            "yandex-disk stop".to_owned(),
            true,
            Some(0),
            String::new(),
            String::new(),
            false,
        );
        let already_running = DiskOutput::new(
            "start".to_owned(),
            "yandex-disk start".to_owned(),
            false,
            Some(1),
            String::new(),
            "Daemon is already running".to_owned(),
            false,
        );

        assert_eq!(started.daemon_status(), Some(DaemonStatus::Running));
        assert_eq!(stopped.daemon_status(), Some(DaemonStatus::Stopped));
        assert_eq!(already_running.daemon_status(), Some(DaemonStatus::Running));
    }
}
