use std::collections::BTreeMap;
use std::path::Path;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::catalog::parse_dt;
use crate::config::Config;
use crate::metadata::{snapshot_dates, Metadata, RepoRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepoTrend {
    #[serde(default)]
    pub stars_7d: i64,
    #[serde(default)]
    pub stars_30d: i64,
    #[serde(default)]
    pub stars_90d: i64,
    #[serde(default)]
    pub growth_30d: f64,
    #[serde(default)]
    pub trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendsData {
    pub generated_at: String,
    pub repositories: BTreeMap<String, RepoTrend>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompactSnapshot {
    pub date: String,
    pub repositories: BTreeMap<String, CompactRepoRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompactRepoRecord {
    pub stars: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks: Option<i64>,
}

pub fn create_compact_snapshot(metadata: &Metadata, date_str: &str) -> CompactSnapshot {
    let mut repositories = BTreeMap::new();
    for (repo, rec) in &metadata.repositories {
        repositories.insert(
            repo.clone(),
            CompactRepoRecord {
                stars: rec.stars,
                forks: rec.forks,
            },
        );
    }
    CompactSnapshot {
        date: date_str.to_string(),
        repositories,
    }
}

pub fn save_compact_snapshot(path: &Path, snapshot: &CompactSnapshot) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut text = serde_json::to_string_pretty(snapshot).map_err(|e| e.to_string())?;
    text.push('\n');
    std::fs::write(path, text).map_err(|e| e.to_string())
}

pub fn load_stars_from_snapshot(snap_path: &Path) -> BTreeMap<String, i64> {
    let mut map = BTreeMap::new();
    let Ok(text) = std::fs::read_to_string(snap_path) else {
        return map;
    };
    // Try parsing as CompactSnapshot or Metadata
    if let Ok(compact) = serde_json::from_str::<CompactSnapshot>(&text) {
        for (repo, rec) in compact.repositories {
            if let Some(s) = rec.stars {
                map.insert(repo, s);
            }
        }
        if !map.is_empty() {
            return map;
        }
    }
    if let Ok(meta) = serde_json::from_str::<Metadata>(&text) {
        for (repo, rec) in meta.repositories {
            if let Some(s) = rec.stars {
                map.insert(repo, s);
            }
        }
    }
    map
}

pub fn find_snapshot_near_days_ago(
    snapshot_dir: &Path,
    target_date: NaiveDate,
    max_tolerance_days: i64,
) -> Option<BTreeMap<String, i64>> {
    let dates = snapshot_dates(snapshot_dir);
    if dates.is_empty() {
        return None;
    }

    let mut best_diff = i64::MAX;
    let mut best_date_str = None;

    for d_str in &dates {
        if let Ok(d) = NaiveDate::parse_from_str(d_str, "%Y-%m-%d") {
            let diff = (d - target_date).num_days().abs();
            if diff < best_diff && diff <= max_tolerance_days {
                best_diff = diff;
                best_date_str = Some(d_str.clone());
            }
        }
    }

    let date_str = best_date_str?;
    let path = snapshot_dir.join(format!("{date_str}.json"));
    Some(load_stars_from_snapshot(&path))
}

pub fn compute_trends(
    metadata: &Metadata,
    snapshot_dir: &Path,
    config: &Config,
    now: DateTime<Utc>,
) -> TrendsData {
    let today = now.date_naive();
    let d7_target = today - chrono::Duration::days(7);
    let d30_target = today - chrono::Duration::days(30);
    let d90_target = today - chrono::Duration::days(90);

    let snap_7d = find_snapshot_near_days_ago(snapshot_dir, d7_target, 4);
    let snap_30d = find_snapshot_near_days_ago(snapshot_dir, d30_target, 10);
    let snap_90d = find_snapshot_near_days_ago(snapshot_dir, d90_target, 20);

    let mut repo_trends = BTreeMap::new();

    for (repo, rec) in &metadata.repositories {
        let Some(current_stars) = rec.stars else {
            continue;
        };

        let gain_7d = snap_7d
            .as_ref()
            .and_then(|m| m.get(repo))
            .map(|&past| (current_stars - past).max(0))
            .unwrap_or(0);

        let past_30d = snap_30d.as_ref().and_then(|m| m.get(repo)).copied();
        let gain_30d = past_30d
            .map(|past| (current_stars - past).max(0))
            .unwrap_or(0);
        let growth_30d = if let Some(past) = past_30d {
            if past > 0 {
                (gain_30d as f64) / (past as f64)
            } else if gain_30d > 0 {
                1.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let gain_90d = snap_90d
            .as_ref()
            .and_then(|m| m.get(repo))
            .map(|&past| (current_stars - past).max(0))
            .unwrap_or(gain_30d);

        let trend_label = if gain_30d >= config.trending.min_gain_30d
            || (growth_30d >= config.trending.min_growth_30d
                && current_stars >= config.trending.min_stars)
        {
            "rising".to_string()
        } else if gain_30d > 0 {
            "steady".to_string()
        } else {
            "quiet".to_string()
        };

        repo_trends.insert(
            repo.clone(),
            RepoTrend {
                stars_7d: gain_7d,
                stars_30d: gain_30d,
                stars_90d: gain_90d,
                growth_30d,
                trend: trend_label,
            },
        );
    }

    TrendsData {
        generated_at: now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        repositories: repo_trends,
    }
}

pub fn is_emerging(record: &RepoRecord, config: &Config, now: DateTime<Utc>) -> bool {
    let stars = record.stars.unwrap_or(0);
    if stars >= config.emerging.max_stars || stars == 0 {
        return false;
    }
    if record.archived == Some(true) {
        return false;
    }
    if let Some(pushed_dt) = parse_dt(record.pushed_at.as_deref()) {
        let days = (now - pushed_dt).num_days();
        if days > config.emerging.max_inactive_days {
            return false;
        }
    } else {
        return false;
    }

    if let Some(created_dt) = parse_dt(record.created_at.as_deref()) {
        let age_days = (now - created_dt).num_days();
        if age_days > config.emerging.max_age_days {
            return false;
        }
    }
    true
}

pub fn is_trending(record: &RepoRecord, trend: Option<&RepoTrend>, config: &Config) -> bool {
    let Some(t) = trend else {
        return false;
    };
    let stars = record.stars.unwrap_or(0);
    if record.archived == Some(true) {
        return false;
    }
    t.stars_30d >= config.trending.min_gain_30d
        || (t.growth_30d >= config.trending.min_growth_30d && stars >= config.trending.min_stars)
}

pub fn load_trends(path: &Path) -> Option<TrendsData> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn save_trends(path: &Path, trends: &TrendsData) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut text = serde_json::to_string_pretty(trends).map_err(|e| e.to_string())?;
    text.push('\n');
    std::fs::write(path, text).map_err(|e| e.to_string())
}
