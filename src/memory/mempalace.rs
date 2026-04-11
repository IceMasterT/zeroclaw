use super::sqlite::SqliteMemory;
use super::traits::{Memory, MemoryCategory, MemoryEntry};
use async_trait::async_trait;
use chrono::Local;
use parking_lot::Mutex;
use serde::Deserialize;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::time::timeout;

pub struct MempalaceMemory {
    local: SqliteMemory,
    python_cmd: String,
    palace_path: Option<String>,
    recall_timeout: Duration,
    local_hit_threshold: usize,
    failure_cooldown: Duration,
    last_failure_at: Mutex<Option<Instant>>,
}

impl MempalaceMemory {
    const DEFAULT_PYTHON_CMD: &'static str = "python3";
    const DEFAULT_RECALL_TIMEOUT_MS: u64 = 900;
    const DEFAULT_LOCAL_HIT_THRESHOLD: usize = 3;
    const DEFAULT_FAILURE_COOLDOWN_MS: u64 = 20_000;

    fn default_python_command() -> String {
        let venv_python = std::env::var("HOME")
            .ok()
            .map(|home| format!("{home}/.zeroclaw/venvs/mempalace/bin/python"));

        if let Some(path) = venv_python
            .as_deref()
            .filter(|path| std::path::Path::new(path).exists())
        {
            return path.to_string();
        }

        Self::DEFAULT_PYTHON_CMD.to_string()
    }

    pub fn new(local: SqliteMemory) -> Self {
        let python_cmd = std::env::var("ZEROCLAW_MEMPALACE_PYTHON")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(Self::default_python_command);
        let palace_path = std::env::var("ZEROCLAW_MEMPALACE_PATH")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let recall_timeout = Self::read_env_duration_ms(
            "ZEROCLAW_MEMPALACE_RECALL_TIMEOUT_MS",
            Self::DEFAULT_RECALL_TIMEOUT_MS,
            50,
        );
        let local_hit_threshold = Self::read_env_usize(
            "ZEROCLAW_MEMPALACE_LOCAL_HIT_THRESHOLD",
            Self::DEFAULT_LOCAL_HIT_THRESHOLD,
            1,
        );
        let failure_cooldown = Self::read_env_duration_ms(
            "ZEROCLAW_MEMPALACE_FAILURE_COOLDOWN_MS",
            Self::DEFAULT_FAILURE_COOLDOWN_MS,
            500,
        );

        Self {
            local,
            python_cmd,
            palace_path,
            recall_timeout,
            local_hit_threshold,
            failure_cooldown,
            last_failure_at: Mutex::new(None),
        }
    }

    fn read_env_usize(name: &str, default: usize, min: usize) -> usize {
        std::env::var(name)
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .map_or(default, |v| v.max(min))
    }

    fn read_env_duration_ms(name: &str, default_ms: u64, min_ms: u64) -> Duration {
        let millis = std::env::var(name)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map_or(default_ms, |v| v.max(min_ms));
        Duration::from_millis(millis)
    }

    fn in_failure_cooldown(&self) -> bool {
        let guard = self.last_failure_at.lock();
        guard
            .as_ref()
            .is_some_and(|last| last.elapsed() < self.failure_cooldown)
    }

    fn mark_failure_now(&self) {
        let mut guard = self.last_failure_at.lock();
        *guard = Some(Instant::now());
    }

    fn clear_failure(&self) {
        let mut guard = self.last_failure_at.lock();
        *guard = None;
    }

    fn merge_results(
        primary_results: Vec<MemoryEntry>,
        secondary_results: Vec<MemoryEntry>,
        limit: usize,
    ) -> Vec<MemoryEntry> {
        if limit == 0 {
            return Vec::new();
        }

        let mut merged = Vec::new();
        let mut seen = HashSet::new();

        for entry in primary_results.into_iter().chain(secondary_results) {
            let signature = format!(
                "{}\u{0}{}",
                entry.key.to_lowercase(),
                entry.content.to_lowercase()
            );

            if seen.insert(signature) {
                merged.push(entry);
                if merged.len() >= limit {
                    break;
                }
            }
        }

        merged
    }

    async fn run_mempalace_search(&self, query: &str, limit: usize) -> anyhow::Result<String> {
        const SEARCH_SCRIPT: &str = r#"
import json
import sys

from mempalace.config import MempalaceConfig
from mempalace.searcher import search_memories

query = sys.argv[1]
palace_arg = sys.argv[2]
n_results = int(sys.argv[3])

palace_path = palace_arg if palace_arg != "__AUTO__" else MempalaceConfig().palace_path
result = search_memories(query=query, palace_path=palace_path, n_results=n_results)
print(json.dumps(result, ensure_ascii=False))
"#;

        let palace_arg = self
            .palace_path
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or("__AUTO__");

        let mut cmd = Command::new(&self.python_cmd);
        cmd.arg("-c")
            .arg(SEARCH_SCRIPT)
            .arg(query)
            .arg(palace_arg)
            .arg(limit.to_string());

        let output = timeout(self.recall_timeout, cmd.output())
            .await
            .map_err(|_| {
                anyhow::anyhow!(
                    "mempalace search timed out after {}ms",
                    self.recall_timeout.as_millis()
                )
            })??;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("mempalace search command failed: {stderr}");
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn recall_from_mempalace(
        &self,
        query: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        #[derive(Deserialize)]
        struct MempalaceHit {
            text: String,
            wing: String,
            room: String,
            source_file: String,
            similarity: Option<f64>,
        }

        #[derive(Deserialize)]
        struct MempalaceSearch {
            error: Option<String>,
            results: Option<Vec<MempalaceHit>>,
        }

        let output = self.run_mempalace_search(query, limit).await?;
        let parsed: MempalaceSearch = serde_json::from_str(&output)
            .map_err(|e| anyhow::anyhow!("mempalace search output parse failed: {e}"))?;

        if let Some(error) = parsed.error {
            anyhow::bail!("mempalace search returned error: {error}");
        }

        let now = Local::now().to_rfc3339();
        Ok(parsed
            .results
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(idx, hit)| MemoryEntry {
                id: format!("mempalace:{idx}"),
                key: format!(
                    "mempalace:{}:{}:{}",
                    hit.wing.trim(),
                    hit.room.trim(),
                    hit.source_file.trim()
                ),
                content: hit.text,
                category: MemoryCategory::Conversation,
                timestamp: now.clone(),
                session_id: None,
                score: hit.similarity,
            })
            .collect())
    }
}

#[async_trait]
impl Memory for MempalaceMemory {
    fn name(&self) -> &str {
        "mempalace"
    }

    async fn store(
        &self,
        key: &str,
        content: &str,
        category: MemoryCategory,
        session_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.local.store(key, content, category, session_id).await
    }

    async fn recall(
        &self,
        query: &str,
        limit: usize,
        session_id: Option<&str>,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        let local_results = self.local.recall(query, limit, session_id).await?;

        if limit == 0
            || local_results.len() >= self.local_hit_threshold
            || self.in_failure_cooldown()
        {
            return Ok(local_results);
        }

        match self.recall_from_mempalace(query, limit).await {
            Ok(remote_results) => {
                self.clear_failure();
                Ok(Self::merge_results(local_results, remote_results, limit))
            }
            Err(error) => {
                self.mark_failure_now();
                tracing::debug!(
                    command = %self.python_cmd,
                    error = %error,
                    "Mempalace recall bridge failed; sqlite remains authoritative"
                );
                Ok(local_results)
            }
        }
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<MemoryEntry>> {
        self.local.get(key).await
    }

    async fn list(
        &self,
        category: Option<&MemoryCategory>,
        session_id: Option<&str>,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        self.local.list(category, session_id).await
    }

    async fn forget(&self, key: &str) -> anyhow::Result<bool> {
        self.local.forget(key).await
    }

    async fn count(&self) -> anyhow::Result<usize> {
        self.local.count().await
    }

    async fn health_check(&self) -> bool {
        self.local.health_check().await
    }

    async fn reindex(
        &self,
        progress_callback: Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
    ) -> anyhow::Result<usize> {
        let _ = progress_callback;
        self.local.reindex().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn mempalace_backend_reports_expected_name() {
        let tmp = TempDir::new().unwrap();
        let sqlite = SqliteMemory::new(tmp.path()).unwrap();
        let memory = MempalaceMemory::new(sqlite);
        assert_eq!(memory.name(), "mempalace");
    }

    #[tokio::test]
    async fn mempalace_backend_keeps_local_store_even_without_bridge() {
        let tmp = TempDir::new().unwrap();
        let sqlite = SqliteMemory::new(tmp.path()).unwrap();
        let memory = MempalaceMemory::new(sqlite);

        memory
            .store(
                "mempalace_key",
                "local first",
                MemoryCategory::Conversation,
                None,
            )
            .await
            .unwrap();

        let stored = memory.get("mempalace_key").await.unwrap();
        assert!(stored.is_some());
        assert_eq!(stored.unwrap().content, "local first");
    }
}
