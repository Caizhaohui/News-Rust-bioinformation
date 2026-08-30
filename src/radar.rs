use chrono::{DateTime, Utc};

use crate::catalog::{active_tools, months_ago, parse_dt, utcnow, Tool};
use crate::config::Config;
use crate::metadata::{repo_record, Metadata};

#[derive(Debug, Clone, Default)]
pub struct Radar {
    pub new_entries: Vec<Tool>,
    pub active: Vec<Tool>,
    pub stale: Vec<Tool>,
    pub watch: Vec<Tool>,
    pub baseline: bool,
}

pub fn compute_radar(
    tools: &[Tool],
    current: &Metadata,
    previous: Option<&Metadata>,
    config: &Config,
    now: Option<DateTime<Utc>>,
) -> Radar {
    let now = now.unwrap_or_else(utcnow);
    let stale_before = months_ago(now, config.radar.stale_months);
    let cold_before = months_ago(now, config.radar.cold_inactive_months);
    let tracked = active_tools(tools);
    let current_urls: std::collections::HashSet<&str> =
        tracked.iter().map(|tool| tool.url.as_str()).collect();
    let previous_urls: std::collections::HashSet<&str> = previous
        .map(|meta| meta.urls.iter().map(String::as_str).collect())
        .unwrap_or_default();
    let empty_repos = Default::default();
    let prev_repos = previous.map(|meta| &meta.repos).unwrap_or(&empty_repos);

    let mut by_url = std::collections::BTreeMap::new();
    for tool in &tracked {
        by_url.insert(tool.url.as_str(), (*tool).clone());
    }
    let mut new_urls: Vec<&str> = current_urls.difference(&previous_urls).copied().collect();
    new_urls.sort_unstable();
    let new_entries: Vec<Tool> = new_urls
        .into_iter()
        .filter_map(|url| by_url.get(url).cloned())
        .collect();

    let mut active = Vec::new();
    let mut stale = Vec::new();
    let mut watch = Vec::new();

    let Some(_previous) = previous else {
        for tool in tracked {
            let record = repo_record(current, tool.repo.as_deref());
            if let Some(error) = record.and_then(|record| record.error.as_deref()) {
                watch.push(tool.clone().with_reason(error));
            }
            let pushed = record.and_then(|record| parse_dt(record.pushed_at.as_deref()));
            let archived = record.and_then(|record| record.archived) == Some(true);
            if archived || pushed.map(|dt| dt < stale_before).unwrap_or(false) {
                let reason = if archived {
                    "archived"
                } else {
                    "no push in 18 months"
                };
                stale.push(tool.clone().with_reason(reason));
            }
        }
        return Radar {
            new_entries: Vec::new(),
            active: Vec::new(),
            stale,
            watch,
            baseline: false,
        };
    };

    for tool in tracked {
        let repo = tool.repo.as_deref().unwrap_or("");
        let record = repo_record(current, tool.repo.as_deref());
        let prev = prev_repos.get(repo);
        if let Some(error) = record.and_then(|record| record.error.as_deref()) {
            watch.push(tool.clone().with_reason(error));
            continue;
        }
        let pushed = record.and_then(|record| parse_dt(record.pushed_at.as_deref()));
        let archived = record.and_then(|record| record.archived) == Some(true);
        if archived || pushed.map(|dt| dt < stale_before).unwrap_or(false) {
            let reason = if archived {
                "archived"
            } else {
                "no push in 18 months"
            };
            stale.push(tool.clone().with_reason(reason));
        }
        if !previous_urls.contains(tool.url.as_str()) {
            continue;
        }
        let mut reasons = Vec::new();
        let stars = record.and_then(|record| record.stars);
        let prev_stars = prev.and_then(|record| record.stars);
        if let (Some(stars), Some(prev_stars)) = (stars, prev_stars) {
            if stars - prev_stars >= config.radar.min_star_delta {
                reasons.push(format!("stars {prev_stars} -> {stars}"));
            }
        }
        if config.radar.include_new_release {
            let rel = record.and_then(|record| parse_dt(record.latest_release_at.as_deref()));
            let prev_rel = prev.and_then(|record| parse_dt(record.latest_release_at.as_deref()));
            if let Some(_rel) = rel {
                if prev_rel.map(|prev_rel| _rel > prev_rel).unwrap_or(true) {
                    let tag = record
                        .and_then(|record| record.latest_release_tag.as_deref())
                        .unwrap_or("release");
                    reasons.push(format!("new release {tag}"));
                }
            }
        }
        if config.radar.include_cold_repo_push {
            let prev_pushed = prev.and_then(|record| parse_dt(record.pushed_at.as_deref()));
            if let (Some(pushed), Some(prev_pushed)) = (pushed, prev_pushed) {
                if prev_pushed < cold_before && pushed > prev_pushed {
                    reasons.push("cold repo pushed".into());
                }
            }
        }
        if !reasons.is_empty() {
            active.push(tool.clone().with_reason(reasons.join("; ")));
        }
    }

    for tool in tools {
        if !tool.is_retired() {
            continue;
        }
        if let Some(error) =
            repo_record(current, tool.repo.as_deref()).and_then(|record| record.error.as_deref())
        {
            watch.push(tool.clone().with_reason(error));
        }
    }

    Radar {
        new_entries,
        active,
        stale,
        watch,
        baseline: true,
    }
}
