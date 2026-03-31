use crate::config::Config;
use crate::util::truncate_with_ellipsis;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Debug, Serialize)]
struct BridgeRequest<'a> {
    session_id: Option<&'a str>,
    channel: &'a str,
    message: &'a str,
}

#[derive(Debug, Deserialize)]
struct BridgeResponse {
    message: Option<String>,
}

pub(super) async fn enhance_message_with_contemplation(
    config: &Config,
    session_id: Option<&str>,
    channel: &str,
    message: String,
) -> String {
    let bridge = &config.contemplation;
    if !bridge.enabled {
        return message;
    }

    let command = match bridge.command.as_deref().map(str::trim) {
        Some(cmd) if !cmd.is_empty() => cmd,
        _ => return message,
    };

    let request = BridgeRequest {
        session_id,
        channel,
        message: &message,
    };

    let payload = match serde_json::to_vec(&request) {
        Ok(buf) => buf,
        Err(error) => {
            tracing::debug!(%error, "failed to serialize contemplation bridge payload");
            return message;
        }
    };

    let mut child = match Command::new("bash")
        .arg("-lc")
        .arg(command)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            tracing::warn!(%error, command, "failed to spawn contemplation bridge command");
            return message;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(error) = stdin.write_all(&payload).await {
            tracing::debug!(%error, "failed to write contemplation bridge stdin");
            return message;
        }
    }

    let timeout_ms = bridge.timeout_ms.max(100);
    let output = match timeout(Duration::from_millis(timeout_ms), child.wait_with_output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(error)) => {
            tracing::debug!(%error, "contemplation bridge command failed");
            return message;
        }
        Err(_) => {
            tracing::warn!(timeout_ms, "contemplation bridge command timed out");
            return message;
        }
    };

    if !output.status.success() {
        return message;
    }

    let raw = match String::from_utf8(output.stdout) {
        Ok(raw) => raw,
        Err(error) => {
            tracing::debug!(%error, "contemplation bridge stdout is not valid UTF-8");
            return message;
        }
    };

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return message;
    }

    let mut enhanced = if let Ok(parsed) = serde_json::from_str::<BridgeResponse>(trimmed) {
        parsed.message.unwrap_or_else(|| trimmed.to_string())
    } else {
        trimmed.to_string()
    };

    let max_chars = bridge.max_output_chars.max(256);
    if enhanced.chars().count() > max_chars {
        enhanced = truncate_with_ellipsis(&enhanced, max_chars);
    }

    tracing::debug!(
        input_chars = message.chars().count(),
        output_chars = enhanced.chars().count(),
        "applied contemplation bridge enhancement"
    );
    enhanced
}
