use std::path::Path;

use serde_yaml::Value;

#[derive(Debug, Clone)]
pub struct Config {
    pub radar: RadarConfig,
    pub snapshots_keep: i64,
}

#[derive(Debug, Clone)]
pub struct RadarConfig {
    pub stale_months: i32,
    pub cold_inactive_months: i32,
    pub min_star_delta: i64,
    pub include_new_release: bool,
    pub include_cold_repo_push: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            radar: RadarConfig {
                stale_months: 18,
                cold_inactive_months: 6,
                min_star_delta: 5,
                include_new_release: true,
                include_cold_repo_push: true,
            },
            snapshots_keep: 8,
        }
    }
}

fn mapping(value: &Value) -> Option<&serde_yaml::Mapping> {
    value.as_mapping()
}

fn get_i64(map: &serde_yaml::Mapping, key: &str, default: i64) -> i64 {
    map.get(Value::String(key.into()))
        .and_then(Value::as_i64)
        .unwrap_or(default)
}

fn get_bool(map: &serde_yaml::Mapping, key: &str, default: bool) -> bool {
    map.get(Value::String(key.into()))
        .and_then(Value::as_bool)
        .unwrap_or(default)
}

fn nested<'a>(map: &'a serde_yaml::Mapping, key: &str) -> Option<&'a serde_yaml::Mapping> {
    map.get(Value::String(key.into())).and_then(mapping)
}

pub fn load_config(path: &Path) -> Config {
    let mut config = Config::default();
    let Ok(text) = std::fs::read_to_string(path) else {
        return config;
    };
    let Ok(raw) = serde_yaml::from_str::<Value>(&text) else {
        return config;
    };
    let Some(root) = mapping(&raw) else {
        return config;
    };
    if let Some(radar) = nested(root, "radar") {
        config.radar.stale_months = get_i64(radar, "stale_months", 18) as i32;
        config.radar.cold_inactive_months = get_i64(radar, "cold_inactive_months", 6) as i32;
        if let Some(active) = nested(radar, "active") {
            config.radar.min_star_delta = get_i64(active, "min_star_delta", 5);
            config.radar.include_new_release = get_bool(active, "include_new_release", true);
            config.radar.include_cold_repo_push = get_bool(active, "include_cold_repo_push", true);
        }
    }
    if let Some(snapshots) = nested(root, "snapshots") {
        config.snapshots_keep = get_i64(snapshots, "keep", 8);
    }
    config
}
