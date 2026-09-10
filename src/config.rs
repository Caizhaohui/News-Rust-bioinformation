use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub activity: ActivityConfig,
    #[serde(default)]
    pub emerging: EmergingConfig,
    #[serde(default)]
    pub trending: TrendingConfig,
    #[serde(default)]
    pub readme: ReadmeConfig,
    #[serde(default)]
    pub snapshot: SnapshotConfig,
    #[serde(default)]
    pub radar: RadarConfig,
    #[serde(default = "default_snapshots_keep")]
    pub snapshots_keep: i64,
}

fn default_schema_version() -> u32 {
    2
}

fn default_snapshots_keep() -> i64 {
    8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityConfig {
    #[serde(default = "default_active_days")]
    pub active_days: i64,
    #[serde(default = "default_maintained_days")]
    pub maintained_days: i64,
    #[serde(default = "default_quiet_days")]
    pub quiet_days: i64,
}

impl Default for ActivityConfig {
    fn default() -> Self {
        Self {
            active_days: 90,
            maintained_days: 365,
            quiet_days: 730,
        }
    }
}

fn default_active_days() -> i64 {
    90
}
fn default_maintained_days() -> i64 {
    365
}
fn default_quiet_days() -> i64 {
    730
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergingConfig {
    #[serde(default = "default_emerging_max_stars")]
    pub max_stars: i64,
    #[serde(default = "default_emerging_max_age_days")]
    pub max_age_days: i64,
    #[serde(default = "default_emerging_max_inactive_days")]
    pub max_inactive_days: i64,
}

impl Default for EmergingConfig {
    fn default() -> Self {
        Self {
            max_stars: 150,
            max_age_days: 1095,
            max_inactive_days: 180,
        }
    }
}

fn default_emerging_max_stars() -> i64 {
    150
}
fn default_emerging_max_age_days() -> i64 {
    1095
}
fn default_emerging_max_inactive_days() -> i64 {
    180
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingConfig {
    #[serde(default = "default_trending_min_stars")]
    pub min_stars: i64,
    #[serde(default = "default_trending_min_gain_30d")]
    pub min_gain_30d: i64,
    #[serde(default = "default_trending_min_growth_30d")]
    pub min_growth_30d: f64,
}

impl Default for TrendingConfig {
    fn default() -> Self {
        Self {
            min_stars: 20,
            min_gain_30d: 10,
            min_growth_30d: 0.10,
        }
    }
}

fn default_trending_min_stars() -> i64 {
    20
}
fn default_trending_min_gain_30d() -> i64 {
    10
}
fn default_trending_min_growth_30d() -> f64 {
    0.10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadmeConfig {
    #[serde(default = "default_trending_limit")]
    pub trending_limit: usize,
    #[serde(default = "default_emerging_limit")]
    pub emerging_limit: usize,
    #[serde(default = "default_source_picks_limit")]
    pub source_picks_limit: usize,
}

impl Default for ReadmeConfig {
    fn default() -> Self {
        Self {
            trending_limit: 10,
            emerging_limit: 12,
            source_picks_limit: 12,
        }
    }
}

fn default_trending_limit() -> usize {
    10
}
fn default_emerging_limit() -> usize {
    12
}
fn default_source_picks_limit() -> usize {
    12
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    #[serde(default = "default_snapshot_keep_days")]
    pub keep_days: i64,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self { keep_days: 730 }
    }
}

fn default_snapshot_keep_days() -> i64 {
    730
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarConfig {
    #[serde(default = "default_stale_months")]
    pub stale_months: i32,
    #[serde(default = "default_cold_inactive_months")]
    pub cold_inactive_months: i32,
    #[serde(default = "default_min_star_delta")]
    pub min_star_delta: i64,
    #[serde(default = "default_true")]
    pub include_new_release: bool,
    #[serde(default = "default_true")]
    pub include_cold_repo_push: bool,
}

fn default_stale_months() -> i32 {
    18
}
fn default_cold_inactive_months() -> i32 {
    6
}
fn default_min_star_delta() -> i64 {
    5
}
fn default_true() -> bool {
    true
}

impl Default for RadarConfig {
    fn default() -> Self {
        Self {
            stale_months: 18,
            cold_inactive_months: 6,
            min_star_delta: 5,
            include_new_release: true,
            include_cold_repo_push: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema_version: 2,
            activity: ActivityConfig::default(),
            emerging: EmergingConfig::default(),
            trending: TrendingConfig::default(),
            readme: ReadmeConfig::default(),
            snapshot: SnapshotConfig::default(),
            radar: RadarConfig::default(),
            snapshots_keep: 8,
        }
    }
}

pub fn load_config(path: &Path) -> Config {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Config::default();
    };
    serde_yaml::from_str::<Config>(&text).unwrap_or_else(|_| Config::default())
}
