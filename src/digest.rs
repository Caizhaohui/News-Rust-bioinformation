use std::path::Path;

use crate::catalog::{load_yaml_list, utcnow, validate_tool_docs, value_to_tool, Tool};
use crate::config::{load_config, Config};
use crate::metadata::{load_metadata, previous_snapshot, repo_record, Metadata};
use crate::paths;
use crate::radar::compute_radar;

fn fact(tool: &Tool, metadata: &Metadata) -> String {
    let record = repo_record(metadata, tool.repo.as_deref());
    let mut bits = Vec::new();
    if let Some(reason) = tool.reason.as_deref() {
        bits.push(reason.to_string());
    }
    if let Some(stars) = record.and_then(|record| record.stars) {
        bits.push(format!("★ {stars}"));
    }
    if let Some(tag) = record.and_then(|record| record.latest_release_tag.as_deref()) {
        bits.push(format!("release {tag}"));
    }
    if bits.is_empty() {
        "见仓库".into()
    } else {
        bits.join("；")
    }
}

fn candidate_block(index: usize, tool: &Tool, metadata: &Metadata) -> String {
    format!(
        "### {index}. [{}]({})\n\n- 事实：{}\n- 为什么值得写：\n",
        tool.name,
        tool.url,
        fact(tool, metadata)
    )
}

pub fn build_digest(
    tools: &[Tool],
    metadata: &Metadata,
    previous: Option<&Metadata>,
    config: &Config,
) -> String {
    let radar = compute_radar(tools, metadata, previous, config, None);
    let today = utcnow().date_naive().to_string();
    let mut ranked = radar.new_entries.clone();
    ranked.extend(radar.active.clone());
    let mut seen = std::collections::HashSet::new();
    let mut unique = Vec::new();
    for tool in ranked {
        if seen.insert(tool.url.clone()) {
            unique.push(tool);
        }
    }
    let picks: Vec<Tool> = unique.into_iter().take(5).collect();
    let mut lines = vec![
        "---".into(),
        format!("title: Rust 生信动态 · {today}"),
        "description: 本周期 Rust 在生物信息学中的新工具、活跃仓库与值得盯的变化。".into(),
        "pubDate: 2099-01-01".into(),
        "tags: [Rust, 生物信息学, 生态跟踪]".into(),
        "lang: zh".into(),
        "draft: true".into(),
        "---".into(),
        String::new(),
        format!("# Rust 生信动态 · {today}"),
        String::new(),
        "机器生成的编辑提纲，不是成稿。删到 3–5 条后再写评述。发布时把 `pubDate` 改成真实日期，并把 `draft` 设为 `false`。".into(),
        String::new(),
        "## 本周要点（候选）".into(),
        String::new(),
    ];
    if !radar.baseline {
        lines.push(
            "当前只有一份快照，还不能做周对比。下周再刷新后才会出现 New / Active 候选。".into(),
        );
        lines.push(String::new());
    } else if picks.is_empty() {
        lines.push("本周期没有达到阈值的 New / Active 条目。可以写停更观察，或跳过这一期。".into());
        lines.push(String::new());
    } else {
        for (index, tool) in picks.iter().enumerate() {
            lines.push(candidate_block(index + 1, tool, metadata));
        }
    }
    lines.push("## 新工具".into());
    lines.push(String::new());
    if radar.new_entries.is_empty() {
        lines.push("_无。_".into());
    } else {
        for tool in &radar.new_entries {
            lines.push(format!(
                "- [{}]({}) — {}",
                tool.name,
                tool.url,
                fact(tool, metadata)
            ));
            lines.push("  - 为什么值得写：".into());
        }
    }
    lines.push(String::new());
    lines.push("## 值得盯".into());
    lines.push(String::new());
    if radar.active.is_empty() {
        lines.push("_无。_".into());
    } else {
        for tool in &radar.active {
            lines.push(format!(
                "- [{}]({}) — {}",
                tool.name,
                tool.url,
                fact(tool, metadata)
            ));
            lines.push("  - 为什么值得写：".into());
        }
    }
    lines.push(String::new());
    lines.push("## 停更观察".into());
    lines.push(String::new());
    if radar.stale.is_empty() {
        lines.push("_无。_".into());
    } else {
        for tool in &radar.stale {
            lines.push(format!(
                "- [{}]({}) — {}",
                tool.name,
                tool.url,
                tool.reason.as_deref().unwrap_or("")
            ));
        }
    }
    lines.extend([
        String::new(),
        "## 下一篇待查".into(),
        String::new(),
        "- ".into(),
        String::new(),
    ]);
    lines.join("\n")
}

pub fn cmd_digest(root: &Path) -> i32 {
    let docs = match load_yaml_list(&paths::tools_path(root)) {
        Ok(docs) => docs,
        Err(err) => {
            eprintln!("{err}");
            return 1;
        }
    };
    let errors = validate_tool_docs(&docs);
    if !errors.is_empty() {
        eprintln!("tools.yaml is invalid:");
        for item in errors {
            eprintln!("  - {item}");
        }
        return 1;
    }
    let tools: Vec<Tool> = docs.iter().filter_map(value_to_tool).collect();
    let today = utcnow().date_naive().to_string();
    let dir = paths::digest_dir(root);
    if let Err(err) = std::fs::create_dir_all(&dir) {
        eprintln!("{err}");
        return 1;
    }
    let path = dir.join(format!("rust-bio-digest-{today}.md"));
    let text = build_digest(
        &tools,
        &load_metadata(&paths::metadata_path(root)),
        previous_snapshot(&paths::snapshot_dir(root), None).as_ref(),
        &load_config(&paths::config_path(root)),
    );
    if let Err(err) = std::fs::write(&path, text) {
        eprintln!("{err}");
        return 1;
    }
    println!(
        "Wrote {}",
        path.strip_prefix(root).unwrap_or(&path).display()
    );
    0
}
