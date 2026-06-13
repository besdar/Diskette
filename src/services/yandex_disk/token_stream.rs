use anyhow::{Context, Result};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::models::yandex_disk::{
    DiskOutput, DiskProgress, DiskRequest, shell_quote, yandex_binary,
};

#[derive(Clone, Copy, Debug)]
enum OutputStream {
    Stdout,
    Stderr,
}

#[derive(Debug)]
struct StreamChunk {
    stream: OutputStream,
    text: String,
}

pub(super) fn run_token<F>(
    auth_file: Option<&Path>,
    command_line: &str,
    progress: &mut F,
) -> Result<DiskOutput>
where
    F: FnMut(DiskProgress),
{
    let binary = yandex_binary();
    let request = DiskRequest::Token {
        auth_file: auth_file.map(Path::to_path_buf),
    };
    let args = request.args();
    let mut child = Command::new(&binary)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to run {}", shell_quote(&binary)))?;

    let stdout = child.stdout.take().context("failed to capture stdout")?;
    let stderr = child.stderr.take().context("failed to capture stderr")?;
    let (sender, receiver) = mpsc::channel();
    let stdout_reader = spawn_stream_reader(stdout, OutputStream::Stdout, sender.clone());
    let stderr_reader = spawn_stream_reader(stderr, OutputStream::Stderr, sender);

    let mut stdout_text = String::new();
    let mut stderr_text = String::new();
    let mut opened_auth_url = false;
    let mut reported_auth_code = false;

    let status = loop {
        match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(chunk) => handle_stream_chunk(
                chunk,
                &mut stdout_text,
                &mut stderr_text,
                &mut opened_auth_url,
                &mut reported_auth_code,
                progress,
            ),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(status) = child.try_wait()? {
                    break status;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break child.wait()?,
        }
    };

    let _ = stdout_reader.join();
    let _ = stderr_reader.join();
    while let Ok(chunk) = receiver.try_recv() {
        handle_stream_chunk(
            chunk,
            &mut stdout_text,
            &mut stderr_text,
            &mut opened_auth_url,
            &mut reported_auth_code,
            progress,
        );
    }

    Ok(DiskOutput::new(
        request.label().to_owned(),
        command_line.to_owned(),
        status.success(),
        status.code(),
        stdout_text,
        stderr_text,
        true,
    ))
}

fn spawn_stream_reader<R>(
    mut reader: R,
    stream: OutputStream,
    sender: mpsc::Sender<StreamChunk>,
) -> thread::JoinHandle<()>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    let text = String::from_utf8_lossy(&buffer[..read]).into_owned();
                    if sender.send(StreamChunk { stream, text }).is_err() {
                        break;
                    }
                }
            }
        }
    })
}

fn handle_stream_chunk<F>(
    chunk: StreamChunk,
    stdout_text: &mut String,
    stderr_text: &mut String,
    opened_auth_url: &mut bool,
    reported_auth_code: &mut bool,
    progress: &mut F,
) where
    F: FnMut(DiskProgress),
{
    match chunk.stream {
        OutputStream::Stdout => stdout_text.push_str(&chunk.text),
        OutputStream::Stderr => stderr_text.push_str(&chunk.text),
    }

    let source_text = match chunk.stream {
        OutputStream::Stdout => stdout_text.as_str(),
        OutputStream::Stderr => stderr_text.as_str(),
    };

    let auth_url = if *opened_auth_url {
        None
    } else {
        extract_url(source_text).inspect(|_| {
            *opened_auth_url = true;
        })
    };

    let auth_code = if *reported_auth_code {
        None
    } else {
        extract_auth_code(source_text).inspect(|_| {
            *reported_auth_code = true;
        })
    };

    progress(DiskProgress {
        text: chunk.text,
        auth_url,
        auth_code,
    });
}

fn extract_url(text: &str) -> Option<String> {
    text.split_whitespace().find_map(|part| {
        let url = part.trim_matches(|ch: char| {
            matches!(
                ch,
                '"' | '\'' | '‘' | '’' | '(' | ')' | '[' | ']' | ',' | '.' | ';'
            )
        });

        if url.starts_with("https://") || url.starts_with("http://") {
            Some(url.to_owned())
        } else {
            None
        }
    })
}

fn extract_auth_code(text: &str) -> Option<String> {
    let lowercase_text = text.to_ascii_lowercase();
    let marker = "enter the code";
    let start = lowercase_text.rfind(marker)?;
    let tail = &text[start + marker.len()..];

    extract_quoted_auth_code(tail).or_else(|| {
        tail.split_whitespace()
            .map(|part| part.trim_matches(|ch: char| !ch.is_ascii_alphanumeric()))
            .find(|candidate| tail.contains(" within ") && is_auth_code(candidate))
            .map(ToOwned::to_owned)
    })
}

fn extract_quoted_auth_code(text: &str) -> Option<String> {
    let start = text.find(is_auth_quote)?;
    let candidate_start = start + text[start..].chars().next()?.len_utf8();
    let end = text[candidate_start..].find(is_auth_quote)?;
    let candidate = text[candidate_start..candidate_start + end].trim();

    if is_auth_code(candidate) {
        Some(candidate.to_owned())
    } else {
        None
    }
}

fn is_auth_quote(ch: char) -> bool {
    matches!(ch, '"' | '\'' | '‘' | '’')
}

fn is_auth_code(value: &str) -> bool {
    (4..=32).contains(&value.len()) && value.chars().all(|ch| ch.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::{extract_auth_code, extract_url};

    #[test]
    fn extracts_device_auth_code_from_cli_output() {
        let output = "Go to the page https://ya.ru/device and enter the code ‘nw7nnyht’ within 300 seconds.";

        assert_eq!(extract_url(output).as_deref(), Some("https://ya.ru/device"));
        assert_eq!(extract_auth_code(output).as_deref(), Some("nw7nnyht"));
    }

    #[test]
    fn waits_for_complete_quoted_device_auth_code() {
        let output = "Go to the page https://ya.ru/device and enter the code ‘nw7";

        assert_eq!(extract_auth_code(output), None);
    }
}
