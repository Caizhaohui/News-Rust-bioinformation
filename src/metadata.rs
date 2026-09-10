use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::catalog::{parse_dt, utcnow, Tool};
use crate::config::ActivityConfig;
use crate::model::{RustRole, ToolDef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityLevel {
    Active,
    Maintained,
    Quiet,
    Inactive,
    Archived,
    Unknown,
}

impl ActivityLevel {
    pub fn badge(&self) -> &'static str {
        match self {
            ActivityLevel::Active => "🟢 Active",
            ActivityLevel::Maintained => "🟡 Maintained",
            ActivityLevel::Quiet => "🟠 Quiet",
            ActivityLevel::Inactive => "🔴 Inactive",
            ActivityLevel::Archived => "⚫ Archived",
            ActivityLevel::Unknown => "",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            ActivityLevel::Active => "🟢",
            ActivityLevel::Maintained => "🟡",
            ActivityLevel::Quiet => "🟠",
            ActivityLevel::Inactive => "🔴",
            ActivityLevel::Archived => "⚫",
            ActivityLevel::Unknown => "⚪",
        }
    }
}

impl fmt::Display for ActivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivityLevel::Active => write!(f, "active"),
            ActivityLevel::Maintained => write!(f, "maintained"),
            ActivityLevel::Quiet => write!(f, "quiet"),
            ActivityLevel::Inactive => write!(f, "inactive"),
            ActivityLevel::Archived => write!(f, "archived"),
            ActivityLevel::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReleaseInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepoRecord {
    #[serde(default)]
    pub stars: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_release: Option<ReleaseInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_release_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_release_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl RepoRecord {
    pub fn release_tag(&self) -> Option<&str> {
        if let Some(ref r) = self.latest_release {
            if let Some(ref t) = r.tag {
                return Some(t.as_str());
            }
        }
        self.latest_release_tag.as_deref()
    }

    pub fn release_at(&self) -> Option<&str> {
        if let Some(ref r) = self.latest_release {
            if let Some(ref a) = r.published_at {
                return Some(a.as_str());
            }
        }
        self.latest_release_at.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default, alias = "fetched_at")]
    pub generated_at: String,
    #[serde(default)]
    pub incomplete: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
    #[serde(default, alias = "repos")]
    pub repositories: BTreeMap<String, RepoRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

fn default_schema_version() -> u32 {
    2
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            schema_version: 2,
            generated_at: utcnow().to_rfc3339_opts(chrono::SecondsFormat::Micros, false),
            incomplete: false,
            urls: Vec::new(),
            repositories: BTreeMap::new(),
            date: None,
        }
    }
}

impl Metadata {
    pub fn empty(incomplete: bool) -> Self {
        Self {
            schema_version: 2,
            generated_at: utcnow().to_rfc3339_opts(chrono::SecondsFormat::Micros, false),
            incomplete,
            urls: Vec::new(),
            repositories: BTreeMap::new(),
            date: None,
        }
    }

    pub fn get_repo(&self, repo: &str) -> Option<&RepoRecord> {
        self.repositories.get(repo)
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
    metadata.repositories.get(repo?)
}

pub fn calculate_activity(
    record: Option<&RepoRecord>,
    config: &ActivityConfig,
    now: DateTime<Utc>,
) -> ActivityLevel {
    let Some(rec) = record else {
        return ActivityLevel::Unknown;
    };
    if rec.archived == Some(true) {
        return ActivityLevel::Archived;
    }
    let Some(pushed_str) = rec.pushed_at.as_deref() else {
        return ActivityLevel::Unknown;
    };
    let Some(pushed_dt) = parse_dt(Some(pushed_str)) else {
        return ActivityLevel::Unknown;
    };

    let days = (now - pushed_dt).num_days();
    if days <= config.active_days {
        ActivityLevel::Active
    } else if days <= config.maintained_days {
        ActivityLevel::Maintained
    } else if days <= config.quiet_days {
        ActivityLevel::Quiet
    } else {
        ActivityLevel::Inactive
    }
}

pub fn architecture_badge(tool: &ToolDef) -> &'static str {
    match tool.architecture.rust_role {
        RustRole::Native => "🦀 Native",
        RustRole::Hybrid => {
            if tool.is_python_facing() {
                "🐍 Rust × Python"
            } else {
                "🔀 Hybrid"
            }
        }
        RustRole::Binding => "🔗 Binding",
        RustRole::Experimental => "🧪 Experimental",
        RustRole::Unknown => "",
    }
}

pub fn tool_sort_key(tool: &Tool, metadata: &Metadata) -> (i64, i64, String) {
    let record = repo_record(metadata, tool.repo.as_deref());
    let pushed = record.and_then(|record| parse_dt(record.pushed_at.as_deref()));
    let stars = record.and_then(|record| record.stars);
    let pushed_ord = pushed.map(|dt| dt.timestamp()).unwrap_or(0);
    let star_ord = stars.unwrap_or(-1);
    (-star_ord, -pushed_ord, tool.name.to_lowercase())
}

pub fn tooldef_sort_key(tool: &ToolDef, metadata: &Metadata) -> (i64, i64, String) {
    let record = metadata.get_repo(&tool.repository);
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

pub fn format_v2_badges(
    tool: &ToolDef,
    metadata: &Metadata,
    config: &ActivityConfig,
    now: DateTime<Utc>,
) -> String {
    let record = metadata.get_repo(&tool.repository);
    let mut badges = Vec::new();

    let arch_badge = architecture_badge(tool);
    if !arch_badge.is_empty() {
        badges.push(format!("`{arch_badge}`"));
    }

    let activity = calculate_activity(record, config, now);
    let act_badge = activity.badge();
    if !act_badge.is_empty() {
        badges.push(format!("`{act_badge}`"));
    }

    if let Some(rec) = record {
        if let Some(stars) = rec.stars {
            badges.push(format!("`★ {stars}`"));
        }
    }

    if let Some(ref learning) = tool.learning {
        if learning.recommended {
            badges.push("`📖 Source Pick`".to_string());
        }
    }

    if badges.is_empty() {
        String::new()
    } else {
        format!(" {}", badges.join(" "))
    }
}
