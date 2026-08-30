use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::catalog::{parse_dt, utcnow, Tool};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub fetched_at: String,
    #[serde(default)]
    pub incomplete: bool,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default)]
    pub repos: BTreeMap<String, RepoRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepoRecord {
    #[serde(default)]
    pub stars: Option<i64>,
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default)]
    pub latest_release_at: Option<String>,
    #[serde(default)]
    pub latest_release_tag: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

impl Metadata {
    pub fn empty(incomplete: bool) -> Self {
        Self {
            fetched_at: utcnow().to_rfc3339_opts(chrono::SecondsFormat::Micros, false),
            incomplete,
            urls: Vec::new(),
            repos: BTreeMap::new(),
            date: None,
        }
    }
}

pub fn dump_json(path: &Path, payload: &Metadata) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let mut text = serde_json::to_string_pretty(payload).map_err(|err| err.to_string())?;
    text.push('\n');
    std::fs::write(path, text).map_err(|err| err.to_string())
}

pub fn load_json(path: &Path) -> Option<Metadata> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn load_metadata(path: &Path) -> Metadata {
    load_json(path).unwrap_or_else(|| Metadata::empty(true))
}

pub fn snapshot_dates(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut dates: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("json"))
        .filter_map(|entry| {
            let stem = entry.path().file_stem()?.to_str()?.to_string();
            if stem.chars().next()?.is_ascii_digit() {
                Some(stem)
            } else {
                None
            }
        })
        .collect();
    dates.sort();
    dates
}

pub fn load_snapshot(dir: &Path, date: &str) -> Option<Metadata> {
    load_json(&dir.join(format!("{date}.json")))
}

pub fn previous_snapshot(dir: &Path, today: Option<&str>) -> Option<Metadata> {
    let mut dates = snapshot_dates(dir);
    if dates.is_empty() {
        return None;
    }
    let today = today
        .map(str::to_string)
        .unwrap_or_else(|| utcnow().date_naive().to_string());
    if dates.last().map(String::as_str) == Some(today.as_str()) {
        dates.pop();
    }
    dates.last().and_then(|date| load_snapshot(dir, date))
}

pub fn prune_snapshots(dir: &Path, keep: i64) {
    let dates = snapshot_dates(dir);
    let extra = dates.len().saturating_sub(keep.max(0) as usize);
    for date in dates.into_iter().take(extra) {
        let _ = std::fs::remove_file(dir.join(format!("{date}.json")));
    }
}

pub fn repo_record<'a>(metadata: &'a Metadata, repo: Option<&str>) -> Option<&'a RepoRecord> {
    metadata.repos.get(repo?)
}

pub fn tool_sort_key(tool: &Tool, metadata: &Metadata) -> (i64, i64, String) {
    let record = repo_record(metadata, tool.repo.as_deref());
    let pushed = record.and_then(|record| parse_dt(record.pushed_at.as_deref()));
    let stars = record.and_then(|record| record.stars);
    let pushed_ord = pushed.map(|dt| dt.timestamp()).unwrap_or(0);
    let star_ord = stars.unwrap_or(-1);
    (-star_ord, -pushed_ord, tool.name.to_lowercase())
}

pub fn format_meta_suffix(tool: &Tool, metadata: &Metadata) -> String {
    let Some(repo) = tool.repo.as_deref() else {
        return String::new();
    };
    let Some(record) = repo_record(metadata, Some(repo)) else {
        return String::new();
    };
    if record.error.is_some() && record.stars.is_none() && record.pushed_at.is_none() {
        return format!(
            " (metadata unavailable: {})",
            record.error.as_deref().unwrap_or("")
        );
    }
    let mut parts = Vec::new();
    if let Some(stars) = record.stars {
        parts.push(format!("★ {stars}"));
    }
    if let Some(pushed) = parse_dt(record.pushed_at.as_deref()) {
        parts.push(format!("pushed {}", pushed.date_naive()));
    }
    if record.archived == Some(true) {
        parts.push("archived".into());
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" ({})", parts.join(", "))
    }
}
