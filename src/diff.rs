//! 生物信息学工具生态变更追踪与差异对比引擎 (Bioinformatics Ecosystem Diff Engine)
//!
//! 该模块用于对比任意两个时间点（快照或当前元数据）之间的生信工具状态，
//! 精确分析：
//! - GitHub Star 数的增减（净增量与增幅百分比）；
//! - 软件版本更新（Tag 变更与发布时间）；
//! - 工具目录增删与状态变更；
//! - 休眠生信软件的重新唤醒（Cold Repo Revived）。

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::catalog::{months_ago, parse_dt, utcnow};
use crate::config::Config;
use crate::metadata::{load_metadata, load_snapshot, previous_snapshot, Metadata};
use crate::model::Catalog;
use crate::paths;

/// 单个工具的 GitHub Star 变化记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StarChange {
    /// 仓库标识（如 `Huang-lab/fastVEP`）
    pub repository: String,
    /// 工具展示名称（如 `fastVEP`）
    pub tool_name: String,
    /// 仓库或项目链接
    pub url: String,
    /// 基准 Star 数
    pub old_stars: i64,
    /// 当前最新 Star 数
    pub new_stars: i64,
    /// 增量变化（可为正或负）
    pub delta: i64,
    /// 增幅百分比（例如 12.5 代表 +12.5%）
    pub growth_percent: f64,
}

/// 单个工具的版本发布/更新记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseChange {
    /// 仓库标识（如 `wdecoster/chopper`）
    pub repository: String,
    /// 工具展示名称（如 `chopper`）
    pub tool_name: String,
    /// 仓库或项目链接
    pub url: String,
    /// 上一个已知版本 Tag（若先前无 Release 则为 None）
    pub old_tag: Option<String>,
    /// 当前最新版本 Tag（如 `v0.14.1`）
    pub new_tag: String,
    /// 发布时间（ISO 8601 字符串）
    pub published_at: Option<String>,
}

/// 目录调整类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CatalogChangeType {
    /// 新收录入目录的生信工具
    Added,
    /// 从目录中移除的工具
    Removed,
    /// 标记为退休/归档状态的工具
    Retired,
}

/// 工具收录状态调整记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogChange {
    /// 仓库标识
    pub repository: String,
    /// 工具名称
    pub tool_name: String,
    /// 链接
    pub url: String,
    /// 调整类型
    pub change_type: CatalogChangeType,
}

/// 工具活跃度状态变化记录（如休眠仓库被唤醒）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivityChange {
    /// 仓库标识
    pub repository: String,
    /// 工具名称
    pub tool_name: String,
    /// 链接
    pub url: String,
    /// 变化描述（例如 "冷门仓库在休眠 8 个月后恢复推送"）
    pub description: String,
}

/// 综合变更差异报告
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiffReport {
    /// 基准时间或快照标签（例如 "2026-09-07"）
    pub from_label: String,
    /// 目标时间或快照标签（例如 "2026-09-18" 或 "current"）
    pub to_label: String,
    /// Star 增减列表（按增量绝对值降序排序）
    pub star_changes: Vec<StarChange>,
    /// 版本发布列表（按发布时间降序排序）
    pub release_changes: Vec<ReleaseChange>,
    /// 目录调整列表
    pub catalog_changes: Vec<CatalogChange>,
    /// 活跃度状态转变列表
    pub activity_changes: Vec<ActivityChange>,
}

impl DiffReport {
    /// 是否存在任何变更
    pub fn is_empty(&self) -> bool {
        self.star_changes.is_empty()
            && self.release_changes.is_empty()
            && self.catalog_changes.is_empty()
            && self.activity_changes.is_empty()
    }

    /// 过滤指定阈值的 Star 变化列表（delta >= min_stars，或 min_stars <= 0 时保留所有变化）
    pub fn filter_star_changes(&self, min_stars: i64) -> Vec<&StarChange> {
        self.star_changes
            .iter()
            .filter(|c| {
                if min_stars <= 0 {
                    true
                } else {
                    c.delta >= min_stars
                }
            })
            .collect()
    }

    /// 渲染终端高亮格式输出
    pub fn format_terminal(&self, min_stars: i64) -> String {
        let mut out = String::new();
        out.push_str("======================================================================\n");
        out.push_str(&format!(
            "         News-Rust-bioinformation Update Diff ({} -> {})\n",
            self.from_label, self.to_label
        ));
        out.push_str("======================================================================\n\n");

        if self.is_empty() {
            out.push_str("No updates or changes detected between snapshots.\n");
            return out;
        }

        // 1. 版本发布
        out.push_str(&format!(
            "🚀 New Releases ({}):\n",
            self.release_changes.len()
        ));
        if self.release_changes.is_empty() {
            out.push_str("   (none)\n");
        } else {
            for r in &self.release_changes {
                let tag_str = match &r.old_tag {
                    Some(old) if old != &r.new_tag => format!("{} -> {}", old, r.new_tag),
                    _ => r.new_tag.clone(),
                };
                let date_str = r
                    .published_at
                    .as_deref()
                    .and_then(|s| s.split('T').next())
                    .unwrap_or("recently");
                out.push_str(&format!(
                    "   • {:<20} {:<24} ({}) - {}\n",
                    r.tool_name, tag_str, date_str, r.url
                ));
            }
        }
        out.push('\n');

        // 2. Star 变化
        let filtered_stars = self.filter_star_changes(min_stars);
        out.push_str(&format!(
            "🌟 Star Changes ({}{}):\n",
            filtered_stars.len(),
            if min_stars > 1 {
                format!(", >= {}★", min_stars)
            } else {
                String::new()
            }
        ));
        if filtered_stars.is_empty() {
            out.push_str("   (none)\n");
        } else {
            for s in filtered_stars {
                let sign = if s.delta > 0 { "+" } else { "" };
                out.push_str(&format!(
                    "   • {:<20} {} -> {:<5} ({}{}, {:+.1}%)\n",
                    s.tool_name, s.old_stars, s.new_stars, sign, s.delta, s.growth_percent
                ));
            }
        }
        out.push('\n');

        // 3. 活跃度与休眠恢复
        if !self.activity_changes.is_empty() {
            out.push_str(&format!(
                "🔄 Revived / Cold Pushes ({}):\n",
                self.activity_changes.len()
            ));
            for a in &self.activity_changes {
                out.push_str(&format!("   • {:<20} {}\n", a.tool_name, a.description));
            }
            out.push('\n');
        }

        // 4. 目录增删
        if !self.catalog_changes.is_empty() {
            out.push_str(&format!(
                "📦 Catalog Changes ({}):\n",
                self.catalog_changes.len()
            ));
            for c in &self.catalog_changes {
                let type_badge = match c.change_type {
                    CatalogChangeType::Added => "NEW",
                    CatalogChangeType::Removed => "REMOVED",
                    CatalogChangeType::Retired => "RETIRED",
                };
                out.push_str(&format!(
                    "   • [{}] {:<20} ({})\n",
                    type_badge, c.tool_name, c.url
                ));
            }
            out.push('\n');
        }

        out.push_str("======================================================================\n");
        out
    }

    /// 渲染适用于 GitHub Step Summary 或 PR 说明的 Markdown 表格报告
    pub fn format_markdown(&self, min_stars: i64) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "## 📊 Ecosystem Update Diff (`{}` → `{}`)\n\n",
            self.from_label, self.to_label
        ));

        if self.is_empty() {
            out.push_str("> _No notable changes detected in this update cycle._\n\n");
            return out;
        }

        // 1. 版本发布
        if !self.release_changes.is_empty() {
            out.push_str(&format!(
                "### 🚀 New Releases ({})\n\n",
                self.release_changes.len()
            ));
            out.push_str("| Tool | Version Change | Release Date | Link |\n");
            out.push_str("|---|---|---|---|\n");
            for r in &self.release_changes {
                let tag_str = match &r.old_tag {
                    Some(old) if old != &r.new_tag => format!("`{}` → `{}`", old, r.new_tag),
                    _ => format!("`{}`", r.new_tag),
                };
                let date_str = r
                    .published_at
                    .as_deref()
                    .and_then(|s| s.split('T').next())
                    .unwrap_or("-");
                out.push_str(&format!(
                    "| [{}]({}) | {} | {} | [GitHub]({}) |\n",
                    r.tool_name, r.url, tag_str, date_str, r.url
                ));
            }
            out.push('\n');
        }

        // 2. Star 增长
        let filtered_stars = self.filter_star_changes(min_stars);
        if !filtered_stars.is_empty() {
            out.push_str(&format!(
                "### 🌟 Star Changes ({}{})\n\n",
                filtered_stars.len(),
                if min_stars > 1 {
                    format!(" >= {}★", min_stars)
                } else {
                    "".into()
                }
            ));
            out.push_str("| Tool | Stars | Delta | Growth | Link |\n");
            out.push_str("|---|---:|---:|---:|---|\n");
            for s in filtered_stars {
                let sign = if s.delta > 0 { "+" } else { "" };
                out.push_str(&format!(
                    "| [{}]({}) | {} → {} | **{}{}** | {:+.1}% | [GitHub]({}) |\n",
                    s.tool_name,
                    s.url,
                    s.old_stars,
                    s.new_stars,
                    sign,
                    s.delta,
                    s.growth_percent,
                    s.url
                ));
            }
            out.push('\n');
        }

        // 3. 活跃度与休眠恢复
        if !self.activity_changes.is_empty() {
            out.push_str(&format!(
                "### 🔄 Revived Repositories ({})\n\n",
                self.activity_changes.len()
            ));
            out.push_str("| Tool | Observation | Link |\n");
            out.push_str("|---|---|---|\n");
            for a in &self.activity_changes {
                out.push_str(&format!(
                    "| [{}]({}) | {} | [GitHub]({}) |\n",
                    a.tool_name, a.url, a.description, a.url
                ));
            }
            out.push('\n');
        }

        // 4. 目录增删
        if !self.catalog_changes.is_empty() {
            out.push_str(&format!(
                "### 📦 Catalog Adjustments ({})\n\n",
                self.catalog_changes.len()
            ));
            out.push_str("| Tool | Status | Link |\n");
            out.push_str("|---|---|---|\n");
            for c in &self.catalog_changes {
                let badge = match c.change_type {
                    CatalogChangeType::Added => "🟢 Added",
                    CatalogChangeType::Removed => "🔴 Removed",
                    CatalogChangeType::Retired => "⚪ Retired",
                };
                out.push_str(&format!(
                    "| [{}]({}) | {} | [GitHub]({}) |\n",
                    c.tool_name, c.url, badge, c.url
                ));
            }
            out.push('\n');
        }

        out
    }

    /// 渲染用于 Git Commit Message 的紧凑摘要
    pub fn format_commit_summary(&self) -> String {
        if self.is_empty() {
            return "no notable metadata changes".to_string();
        }

        let mut parts = Vec::new();

        // 提炼 releases 摘要
        if !self.release_changes.is_empty() {
            let names: Vec<String> = self
                .release_changes
                .iter()
                .take(3)
                .map(|r| format!("{} {}", r.tool_name, r.new_tag))
                .collect();
            let extra = if self.release_changes.len() > 3 {
                format!(" +{} more", self.release_changes.len() - 3)
            } else {
                "".into()
            };
            parts.push(format!(
                "{} releases ({}{})",
                self.release_changes.len(),
                names.join(", "),
                extra
            ));
        }

        // 提炼 top star 增长
        let positive_stars: Vec<&StarChange> =
            self.star_changes.iter().filter(|s| s.delta > 0).collect();
        if !positive_stars.is_empty() {
            let top: Vec<String> = positive_stars
                .iter()
                .take(3)
                .map(|s| format!("{}(+{})", s.tool_name, s.delta))
                .collect();
            let extra = if positive_stars.len() > 3 {
                format!(" +{} more", positive_stars.len() - 3)
            } else {
                "".into()
            };
            parts.push(format!("stars: {}{}", top.join(", "), extra));
        }

        // 目录变化
        if !self.catalog_changes.is_empty() {
            parts.push(format!("catalog: {} items", self.catalog_changes.len()));
        }

        if parts.is_empty() {
            "metadata refreshed".to_string()
        } else {
            parts.join("; ")
        }
    }

    /// 转换为格式化 JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// 计算两个元数据集合之间的差异
pub fn compute_diff(
    baseline: &Metadata,
    current: &Metadata,
    catalog: Option<&Catalog>,
    config: &Config,
    now: DateTime<Utc>,
) -> DiffReport {
    let from_label = baseline
        .date
        .clone()
        .or_else(|| {
            if !baseline.generated_at.is_empty() {
                Some(
                    baseline
                        .generated_at
                        .split('T')
                        .next()
                        .unwrap_or(&baseline.generated_at)
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_else(|| "baseline".to_string());

    let to_label = current
        .date
        .clone()
        .or_else(|| {
            if !current.generated_at.is_empty() {
                Some(
                    current
                        .generated_at
                        .split('T')
                        .next()
                        .unwrap_or(&current.generated_at)
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_else(|| now.date_naive().to_string());

    // 构建仓库到工具名称及 URL 的映射
    let mut repo_map: HashMap<String, (String, String)> = HashMap::new();
    if let Some(cat) = catalog {
        for tool in cat.tools.values() {
            repo_map.insert(
                tool.repository.to_lowercase(),
                (tool.name.clone(), tool.effective_url()),
            );
        }
    }

    let resolve_info = |repo: &str| -> (String, String) {
        if let Some(found) = repo_map.get(&repo.to_lowercase()) {
            found.clone()
        } else {
            let name = repo.split('/').next_back().unwrap_or(repo).to_string();
            let url = format!("https://github.com/{repo}");
            (name, url)
        }
    };

    let mut star_changes = Vec::new();
    let mut release_changes = Vec::new();
    let mut activity_changes = Vec::new();

    let cold_before = months_ago(now, config.radar.cold_inactive_months);

    // 1. 遍历当前所有仓库进行对比
    for (repo, curr_rec) in &current.repositories {
        let (tool_name, url) = resolve_info(repo);

        if let Some(base_rec) = baseline.repositories.get(repo) {
            // A. Star 变化
            if let (Some(curr_stars), Some(base_stars)) = (curr_rec.stars, base_rec.stars) {
                if curr_stars != base_stars {
                    let delta = curr_stars - base_stars;
                    let growth_percent = if base_stars > 0 {
                        (delta as f64 / base_stars as f64) * 100.0
                    } else if delta > 0 {
                        100.0
                    } else {
                        0.0
                    };
                    star_changes.push(StarChange {
                        repository: repo.clone(),
                        tool_name: tool_name.clone(),
                        url: url.clone(),
                        old_stars: base_stars,
                        new_stars: curr_stars,
                        delta,
                        growth_percent,
                    });
                }
            }

            // B. 版本发布变化
            let curr_tag = curr_rec.release_tag();
            let base_tag = base_rec.release_tag();
            let curr_rel_at = curr_rec.release_at().and_then(|s| parse_dt(Some(s)));
            let base_rel_at = base_rec.release_at().and_then(|s| parse_dt(Some(s)));

            let mut is_new_release = false;
            if let Some(curr_t) = curr_tag {
                if let Some(base_t) = base_tag {
                    if curr_t != base_t {
                        is_new_release = true;
                    } else if let (Some(curr_dt), Some(base_dt)) = (curr_rel_at, base_rel_at) {
                        if curr_dt > base_dt {
                            is_new_release = true;
                        }
                    }
                } else {
                    // 基准没有 tag，当前有 tag：检查发布时间是否在基准之后（防止因基准数据不全而误报历史旧版本）
                    if let Some(curr_dt) = curr_rel_at {
                        if let Some(base_pushed) = base_rec
                            .pushed_at
                            .as_deref()
                            .and_then(|s| parse_dt(Some(s)))
                        {
                            if curr_dt >= base_pushed {
                                is_new_release = true;
                            }
                        } else {
                            is_new_release = true;
                        }
                    } else {
                        is_new_release = true;
                    }
                }
            }

            if is_new_release {
                if let Some(new_t) = curr_tag {
                    release_changes.push(ReleaseChange {
                        repository: repo.clone(),
                        tool_name: tool_name.clone(),
                        url: url.clone(),
                        old_tag: base_tag.map(str::to_string),
                        new_tag: new_t.to_string(),
                        published_at: curr_rec.release_at().map(str::to_string),
                    });
                }
            }

            // C. 冷门仓库被唤醒
            let curr_pushed = curr_rec
                .pushed_at
                .as_deref()
                .and_then(|s| parse_dt(Some(s)));
            let base_pushed = base_rec
                .pushed_at
                .as_deref()
                .and_then(|s| parse_dt(Some(s)));
            if let (Some(curr_p), Some(base_p)) = (curr_pushed, base_pushed) {
                if base_p < cold_before && curr_p > base_p {
                    let months = (curr_p - base_p).num_days() / 30;
                    activity_changes.push(ActivityChange {
                        repository: repo.clone(),
                        tool_name: tool_name.clone(),
                        url: url.clone(),
                        description: format!("Pushed after ~{months} months of inactivity"),
                    });
                }
            }
        }
    }

    // 2. 目录增删检测 (基于 URLs)
    let mut catalog_changes = Vec::new();
    let base_urls: HashSet<&str> = baseline.urls.iter().map(String::as_str).collect();
    let curr_urls: HashSet<&str> = current.urls.iter().map(String::as_str).collect();

    // 新增
    for url in curr_urls.difference(&base_urls) {
        let repo = crate::catalog::github_repo_from_url(url).unwrap_or_default();
        let (tool_name, tool_url) = resolve_info(&repo);
        catalog_changes.push(CatalogChange {
            repository: repo,
            tool_name,
            url: if tool_url.is_empty() {
                url.to_string()
            } else {
                tool_url
            },
            change_type: CatalogChangeType::Added,
        });
    }

    // 移除
    for url in base_urls.difference(&curr_urls) {
        let repo = crate::catalog::github_repo_from_url(url).unwrap_or_default();
        let (tool_name, tool_url) = resolve_info(&repo);
        catalog_changes.push(CatalogChange {
            repository: repo,
            tool_name,
            url: if tool_url.is_empty() {
                url.to_string()
            } else {
                tool_url
            },
            change_type: CatalogChangeType::Removed,
        });
    }

    // 排序保证输出稳定
    star_changes.sort_by(|a, b| {
        b.delta
            .cmp(&a.delta)
            .then_with(|| a.tool_name.cmp(&b.tool_name))
    });
    release_changes.sort_by(|a, b| {
        b.published_at
            .cmp(&a.published_at)
            .then_with(|| a.tool_name.cmp(&b.tool_name))
    });
    catalog_changes.sort_by(|a, b| a.tool_name.cmp(&b.tool_name));
    activity_changes.sort_by(|a, b| a.tool_name.cmp(&b.tool_name));

    DiffReport {
        from_label,
        to_label,
        star_changes,
        release_changes,
        catalog_changes,
        activity_changes,
    }
}

/// 执行 diff 命令行子命令
pub fn cmd_diff(
    root: &Path,
    from: Option<String>,
    to: Option<String>,
    format: &str,
    min_stars: i64,
    output: Option<PathBuf>,
) -> i32 {
    let snap_dir = paths::snapshot_dir(root);
    let meta_path = paths::metadata_path(root);
    let tools_path = paths::tools_path(root);
    let config = crate::config::load_config(&paths::config_path(root));

    let catalog = crate::validate::load_and_validate(&tools_path)
        .ok()
        .map(|(cat, _)| cat);

    // 1. 加载基准元数据
    let baseline = if let Some(ref from_date) = from {
        match load_snapshot(&snap_dir, from_date) {
            Some(meta) => meta,
            None => {
                eprintln!("Error: baseline snapshot not found for date: {from_date}");
                return 1;
            }
        }
    } else {
        // 默认寻找前一份历史快照
        match previous_snapshot(&snap_dir, None) {
            Some(meta) => meta,
            None => {
                eprintln!("Error: no previous snapshot available to compare against. Use --from <DATE> to specify.");
                return 1;
            }
        }
    };

    // 2. 加载目标元数据
    let current = if let Some(ref to_date) = to {
        match load_snapshot(&snap_dir, to_date) {
            Some(meta) => meta,
            None => {
                eprintln!("Error: target snapshot not found for date: {to_date}");
                return 1;
            }
        }
    } else {
        load_metadata(&meta_path)
    };

    let report = compute_diff(&baseline, &current, catalog.as_ref(), &config, utcnow());

    let formatted = match format {
        "markdown" | "md" => report.format_markdown(min_stars),
        "json" => match report.to_json() {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to serialize diff to JSON: {e}");
                return 1;
            }
        },
        "summary" => report.format_commit_summary(),
        _ => report.format_terminal(min_stars),
    };

    if let Some(out_path) = output {
        if let Err(e) = std::fs::write(&out_path, &formatted) {
            eprintln!("Failed to write diff report to {}: {e}", out_path.display());
            return 1;
        }
        println!("Wrote diff report to {}", out_path.display());
    } else {
        print!("{formatted}");
        if !formatted.ends_with('\n') {
            println!();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{ReleaseInfo, RepoRecord};
    use std::collections::BTreeMap;

    type TestRecord<'a> = (
        &'a str,
        i64,
        Option<&'a str>,
        Option<&'a str>,
        Option<&'a str>,
    );

    fn make_test_meta(date: &str, urls: Vec<&str>, records: Vec<TestRecord<'_>>) -> Metadata {
        let mut repos = BTreeMap::new();
        for (name, stars, pushed_at, tag, rel_at) in records {
            let rel = tag.map(|t| ReleaseInfo {
                tag: Some(t.to_string()),
                published_at: rel_at.map(str::to_string),
                url: None,
            });
            repos.insert(
                name.to_string(),
                RepoRecord {
                    stars: Some(stars),
                    pushed_at: pushed_at.map(str::to_string),
                    latest_release: rel,
                    latest_release_tag: tag.map(str::to_string),
                    latest_release_at: rel_at.map(str::to_string),
                    ..Default::default()
                },
            );
        }
        Metadata {
            schema_version: 2,
            generated_at: format!("{date}T00:00:00Z"),
            incomplete: false,
            urls: urls.into_iter().map(str::to_string).collect(),
            repositories: repos,
            date: Some(date.to_string()),
        }
    }

    #[test]
    fn test_diff_star_changes() {
        let base = make_test_meta(
            "2026-09-01",
            vec!["https://github.com/org/tool-a"],
            vec![("org/tool-a", 100, None, None, None)],
        );
        let curr = make_test_meta(
            "2026-09-08",
            vec!["https://github.com/org/tool-a"],
            vec![("org/tool-a", 115, None, None, None)],
        );

        let report = compute_diff(&base, &curr, None, &Config::default(), utcnow());
        assert_eq!(report.star_changes.len(), 1);
        assert_eq!(report.star_changes[0].delta, 15);
        assert_eq!(report.star_changes[0].old_stars, 100);
        assert_eq!(report.star_changes[0].new_stars, 115);
        assert!((report.star_changes[0].growth_percent - 15.0).abs() < 1e-4);
    }

    #[test]
    fn test_diff_release_changes() {
        let base = make_test_meta(
            "2026-09-01",
            vec!["https://github.com/org/tool-a"],
            vec![(
                "org/tool-a",
                50,
                Some("2026-08-30T00:00:00Z"),
                Some("v1.0.0"),
                Some("2026-08-25T00:00:00Z"),
            )],
        );
        let curr = make_test_meta(
            "2026-09-08",
            vec!["https://github.com/org/tool-a"],
            vec![(
                "org/tool-a",
                55,
                Some("2026-09-05T00:00:00Z"),
                Some("v1.1.0"),
                Some("2026-09-04T00:00:00Z"),
            )],
        );

        let report = compute_diff(&base, &curr, None, &Config::default(), utcnow());
        assert_eq!(report.release_changes.len(), 1);
        assert_eq!(
            report.release_changes[0].old_tag,
            Some("v1.0.0".to_string())
        );
        assert_eq!(report.release_changes[0].new_tag, "v1.1.0");
    }

    #[test]
    fn test_diff_catalog_and_formatting() {
        let base = make_test_meta("2026-09-01", vec!["https://github.com/org/tool-a"], vec![]);
        let curr = make_test_meta(
            "2026-09-08",
            vec![
                "https://github.com/org/tool-a",
                "https://github.com/org/tool-b",
            ],
            vec![],
        );

        let report = compute_diff(&base, &curr, None, &Config::default(), utcnow());
        assert_eq!(report.catalog_changes.len(), 1);
        assert_eq!(
            report.catalog_changes[0].change_type,
            CatalogChangeType::Added
        );

        let md = report.format_markdown(1);
        assert!(md.contains("Ecosystem Update Diff"));

        let term = report.format_terminal(1);
        assert!(term.contains("Catalog Changes"));
    }
}
