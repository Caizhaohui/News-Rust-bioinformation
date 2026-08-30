use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use regex::Regex;
use serde_json::Value;

use crate::catalog::{
    catalog_index, github_repo_from_url, is_cataloged, load_yaml_list, normalize_url, utcnow,
    validate_tool_docs, CatalogIndex, Tool,
};
use crate::http::{HttpClient, ReqwestClient};
use crate::paths;

pub const VALID_SOURCES: &[&str] = &["github", "biorxiv"];
const GITHUB_SEARCH_URL: &str = "https://api.github.com/search/repositories";
const BIORXIV_DETAILS_URL: &str = "https://api.biorxiv.org/details/biorxiv";
const USER_AGENT: &str = "News-Rust-bioinformation-discover";

const GITHUB_TOPICS: &[&str] = &[
    "bioinformatics",
    "genomics",
    "proteomics",
    "metagenomics",
    "sequencing",
    "pangenome",
    "pangenomes",
    "long-read",
    "long-reads",
];
const GITHUB_KEYWORDS: &[&str] = &[
    "bioinformatics",
    "genomics",
    "proteomics",
    "metagenomics",
    "pangenome",
    "\"long-read\"",
    "sequencing",
];
const BIO_TERMS: &[&str] = &[
    "bioinformatics",
    "computational biology",
    "genomics",
    "genome",
    "proteomics",
    "proteome",
    "metagenomics",
    "metagenome",
    "sequencing",
    "sequence analysis",
    "pangenome",
    "long-read",
    "long read",
    "nanopore",
    "pacbio",
    "fasta",
    "fastq",
    "vcf",
    "htslib",
    "transcriptom",
    "single-cell",
    "single cell",
    "rna-seq",
    "rnaseq",
    "scrna",
    "phylogen",
    "crispr",
    "peptide",
    "protein structure",
    "protein design",
    "protein engineering",
    "directed evolution",
    "protein language model",
    "enzyme engineering",
    "mass spectrom",
    "microbiom",
    "variant call",
    "read align",
    "sequence align",
    "genome align",
    "aligner",
    "genome assembl",
    "annotation",
    "ngs",
];
const ALWAYS_EXCLUDE: &[&str] = &[
    r"slurm.*\b(tui|dashboard)\b",
    r"\b(tui|dashboard)\b.*slurm",
    r"cluster dashboard",
    r"hpc dashboard",
];
const EXCLUDE_IF_NO_BIO: &[&str] = &[
    r"\bcsv parser\b",
    r"\bcsv library\b",
    r"spreadsheet",
    r"\bxlsx\b",
    r"\bcsv\b",
];
const RUST_SIGNALS: &[&str] = &[
    r"\bwritten in rust\b",
    r"\bimplemented in rust\b",
    r"\busing rust\b",
    r"\brust-based\b",
    r"\brust language\b",
    r"\brust programming\b",
    r"\ba rust (tool|library|crate|package|implementation|caller|aligner)\b",
    r"\brust crate\b",
    r"\bcargo\b",
    r"\bcrates\.io\b",
];
const CATEGORY_HINTS: &[(&str, &[&str])] = &[
    (
        "crispr",
        &[
            "crispr",
            "cas9",
            "cas12",
            "cas13",
            "sgrna",
            "guide rna",
            "genome edit",
        ],
    ),
    (
        "bacterial-assembly",
        &[
            "bacterial assembl",
            "autocycler",
            "polypolish",
            "sparrowhawk-asm",
        ],
    ),
    (
        "bacterial-annotation",
        &[
            "prokaryotic annotation",
            "gene prediction",
            "prodigal",
            "bakta",
            "prokka",
            "orphos",
            "fraggenescan",
        ],
    ),
    (
        "prokaryotic-transcriptome",
        &[
            "prokaryotic transcript",
            "bacterial transcript",
            "transcript unit",
            "operon",
        ],
    ),
    (
        "phage-defense",
        &[
            "phage defense",
            "antiphage",
            "defense finder",
            "padloc",
            "anti-phage",
        ],
    ),
    (
        "resistance-genes",
        &[
            "antimicrobial",
            "amrfinder",
            "resfinder",
            "resistance gene",
            "abricate",
        ],
    ),
    (
        "transposons",
        &[
            "transposon",
            "insertion sequence",
            "isescan",
            "is element",
            "tn-seq",
            "tnseq",
        ],
    ),
    (
        "long-reads",
        &[
            "long-read",
            "long read",
            "nanopore",
            "pacbio",
            "oxford nanopore",
        ],
    ),
    (
        "assembly-and-pangenomes",
        &["pangenome", "genome assembl", "de bruijn", "gfa"],
    ),
    (
        "metagenomics",
        &[
            "metagenom",
            "microbiom",
            "16s rrna",
            "16s ",
            "mag dereplic",
            "unifrac",
            "dada2",
            "amplicon sequence variant",
        ],
    ),
    (
        "single-cell-and-rna",
        &[
            "single-cell",
            "single cell",
            "scrna",
            "rna-seq",
            "rnaseq",
            "transcriptom",
        ],
    ),
    (
        "protein-engineering",
        &[
            "protein design",
            "directed evolution",
            "protein engineering",
            "protein language model",
            "enzyme engineering",
            "ligandmpnn",
            "proteinmpnn",
            "inverse folding",
            "side-chain packing",
            "sidechain packing",
        ],
    ),
    (
        "proteomics-and-structure",
        &[
            "proteom",
            "peptide",
            "alphafold",
            "protein structure",
            "mass spectrom",
        ],
    ),
    (
        "variants-and-annotation",
        &["variant call", "vcf", "annotation"],
    ),
    (
        "alignment-and-mapping",
        &[
            "aligner",
            "read align",
            "sequence align",
            "genome align",
            "minimap",
        ],
    ),
    (
        "workflows-and-infrastructure",
        &["workflow", "nextflow", "snakemake"],
    ),
    ("visualization", &["visualiz", "genome browser"]),
    (
        "sequence-io-and-formats",
        &["fasta", "fastq", "bam parser", "cram"],
    ),
    (
        "learning-resources",
        &["awesome list", "tutorial", "learning resource"],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub name: String,
    pub url: String,
    pub source: String,
    pub why: String,
    pub repo: Option<String>,
    pub suggested_category: String,
    pub crates_io: Option<String>,
}

fn github_url_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)https?://(?:www\.)?github\.com/([A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?)/([A-Za-z0-9._-]+)")
            .unwrap()
    })
}

fn crates_io_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)https?://crates\.io/crates/([A-Za-z0-9_-]+)").unwrap())
}

pub fn parse_sources(raw: &str) -> Vec<String> {
    let parts: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_lowercase())
        .collect();
    let mut mapped = Vec::new();
    for part in parts {
        if part == "twitter" || part == "x" {
            continue;
        }
        if VALID_SOURCES.contains(&part.as_str()) && !mapped.contains(&part) {
            mapped.push(part);
        }
    }
    if mapped.is_empty() {
        VALID_SOURCES.iter().map(|s| (*s).to_string()).collect()
    } else {
        mapped
    }
}

pub fn looks_bio_related(text: &str) -> bool {
    let lower = text.to_lowercase();
    BIO_TERMS.iter().any(|term| lower.contains(term))
}

fn any_match(patterns: &[&str], text: &str) -> bool {
    patterns
        .iter()
        .any(|pat| Regex::new(pat).unwrap().is_match(text))
}

pub fn is_excluded(text: &str) -> bool {
    let lower = text.to_lowercase();
    if any_match(ALWAYS_EXCLUDE, &lower) {
        return true;
    }
    if looks_bio_related(&lower) {
        return false;
    }
    any_match(EXCLUDE_IF_NO_BIO, &lower)
}

pub fn has_rust_signal(text: &str) -> bool {
    let lower = text.to_lowercase();
    if !lower.contains("rust") {
        return false;
    }
    if any_match(RUST_SIGNALS, &lower) {
        return true;
    }
    lower.contains("github.com") && lower.contains("rust")
}

pub fn extract_github_repos(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    for caps in github_url_re().captures_iter(text) {
        let owner = caps.get(1).unwrap().as_str();
        let mut name = caps.get(2).unwrap().as_str();
        name = name.trim_end_matches([')', '.', ',', ';', ':', '"', '\'']);
        let name = name.strip_suffix(".git").unwrap_or(name);
        let repo = format!("{owner}/{name}");
        let Some(inferred) = github_repo_from_url(&format!("https://github.com/{repo}")) else {
            continue;
        };
        let key = inferred.to_lowercase();
        if seen.insert(key) {
            found.push(inferred);
        }
    }
    found
}

pub fn extract_crates_io(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    for caps in crates_io_re().captures_iter(text) {
        let url = format!("https://crates.io/crates/{}", caps.get(1).unwrap().as_str());
        if seen.insert(url.clone()) {
            found.push(url);
        }
    }
    found
}

pub fn suggest_category(text: &str) -> String {
    let lower = text.to_lowercase();
    for (category, hints) in CATEGORY_HINTS {
        if hints.iter().any(|hint| lower.contains(hint)) {
            return (*category).to_string();
        }
    }
    String::new()
}

fn json_str(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn blob(parts: &[Value]) -> String {
    let mut chunks = Vec::new();
    for part in parts {
        match part {
            Value::Array(items) => {
                let joined: Vec<String> = items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect();
                if !joined.is_empty() {
                    chunks.push(joined.join(" "));
                }
            }
            Value::String(s) if !s.is_empty() => chunks.push(s.clone()),
            Value::Null | Value::Bool(false) => {}
            other if !other.is_null() && *other != Value::Bool(false) => {
                if let Some(s) = other.as_str() {
                    if !s.is_empty() {
                        chunks.push(s.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    chunks.join(" ")
}

pub fn parse_github_item(item: &Value, query: &str) -> Option<Candidate> {
    if item.get("archived").and_then(Value::as_bool) == Some(true)
        || item.get("fork").and_then(Value::as_bool) == Some(true)
    {
        return None;
    }
    let url = json_str(item.get("html_url"));
    let repo = {
        let full = json_str(item.get("full_name"));
        if full.is_empty() {
            github_repo_from_url(&url)
        } else {
            Some(full)
        }
    };
    let mut name = json_str(item.get("name"));
    if name.is_empty() {
        if let Some(repo) = repo.as_deref() {
            name = repo.rsplit('/').next().unwrap_or("").to_string();
        }
    }
    let topics = item.get("topics").cloned().unwrap_or(Value::Null);
    let text = blob(&[
        Value::String(name.clone()),
        repo.as_deref()
            .map(|r| Value::String(r.into()))
            .unwrap_or(Value::Null),
        item.get("description").cloned().unwrap_or(Value::Null),
        topics,
    ]);
    if is_excluded(&text) || !looks_bio_related(&text) {
        return None;
    }
    if name.is_empty() || url.is_empty() {
        return None;
    }
    let mut why_bits = vec![format!("GitHub Search 命中 `{query}`。")];
    if let Some(stars) = item.get("stargazers_count").and_then(Value::as_i64) {
        why_bits.push(format!("★ {stars}。"));
    }
    let pushed = json_str(item.get("pushed_at"));
    let pushed = if pushed.len() >= 10 {
        &pushed[..10]
    } else {
        pushed.as_str()
    };
    if !pushed.is_empty() {
        why_bits.push(format!("pushed {pushed}。"));
    }
    let crates = extract_crates_io(&json_str(item.get("description")));
    Some(Candidate {
        name,
        url: {
            let normalized = normalize_url(&url);
            if normalized.is_empty() {
                url
            } else {
                normalized
            }
        },
        repo,
        source: "github".into(),
        why: why_bits.concat(),
        suggested_category: suggest_category(&text),
        crates_io: crates.into_iter().next(),
    })
}

pub fn parse_biorxiv_paper(paper: &Value) -> Option<Candidate> {
    let title = json_str(paper.get("title"));
    let abstract_text = json_str(paper.get("abstract"));
    let text = blob(&[
        Value::String(title.clone()),
        Value::String(abstract_text),
        paper.get("category").cloned().unwrap_or(Value::Null),
    ]);
    if is_excluded(&text) || !has_rust_signal(&text) {
        return None;
    }
    let repos = extract_github_repos(&text);
    let crates = extract_crates_io(&text);
    let doi = json_str(paper.get("doi"));
    let url = if let Some(repo) = repos.first() {
        format!("https://github.com/{repo}")
    } else if !doi.is_empty() {
        format!("https://www.biorxiv.org/content/{doi}")
    } else {
        return None;
    };
    let name = if let Some(repo) = repos.first() {
        repo.rsplit('/').next().unwrap_or("").to_string()
    } else {
        title
            .split(':')
            .next()
            .unwrap_or("")
            .trim()
            .chars()
            .take(80)
            .collect()
    };
    let why = format!(
        "bioRxiv 预印本标题/摘要含 Rust 编程信号：{}",
        title.trim().chars().take(160).collect::<String>()
    );
    Some(Candidate {
        name: if name.is_empty() {
            "biorxiv-paper".into()
        } else {
            name
        },
        url,
        repo: repos.into_iter().next(),
        source: "biorxiv".into(),
        why,
        suggested_category: suggest_category(&text),
        crates_io: crates.into_iter().next(),
    })
}

fn candidate_key(candidate: &Candidate) -> String {
    if let Some(repo) = candidate.repo.as_deref() {
        repo.to_lowercase()
    } else {
        let normalized = normalize_url(&candidate.url);
        if normalized.is_empty() {
            candidate.url.to_lowercase()
        } else {
            normalized
        }
    }
}

fn prefer_url(first: &Candidate, second: &Candidate) -> String {
    for item in [first, second] {
        if item.repo.is_some() && item.url.contains("github.com") {
            return item.url.clone();
        }
    }
    if first.url.is_empty() {
        second.url.clone()
    } else {
        first.url.clone()
    }
}

pub fn dedupe_candidates(candidates: Vec<Candidate>) -> Vec<Candidate> {
    let mut merged: HashMap<String, Candidate> = HashMap::new();
    let mut order = Vec::new();
    for candidate in candidates {
        let key = candidate_key(&candidate);
        if let Some(existing) = merged.get_mut(&key) {
            let mut sources = Vec::new();
            for source in format!("{},{}", existing.source, candidate.source).split(',') {
                let source = source.trim();
                if !source.is_empty() && !sources.iter().any(|s: &String| s == source) {
                    sources.push(source.to_string());
                }
            }
            let mut why = existing.why.clone();
            if !candidate.why.is_empty() && !existing.why.contains(&candidate.why) {
                why = format!("{} {}", existing.why, candidate.why)
                    .trim()
                    .to_string();
            }
            existing.url = prefer_url(existing, &candidate);
            if existing.repo.is_none() {
                existing.repo = candidate.repo.clone();
            }
            existing.source = sources.join(",");
            existing.why = why;
            if existing.suggested_category.is_empty() {
                existing.suggested_category = candidate.suggested_category.clone();
            }
            if existing.crates_io.is_none() {
                existing.crates_io = candidate.crates_io.clone();
            }
        } else {
            order.push(key.clone());
            merged.insert(key, candidate);
        }
    }
    order
        .into_iter()
        .filter_map(|key| merged.remove(&key))
        .collect()
}

pub fn split_new_and_known(
    candidates: Vec<Candidate>,
    index: &CatalogIndex,
) -> (Vec<Candidate>, Vec<Candidate>) {
    let mut new_ones = Vec::new();
    let mut known = Vec::new();
    for candidate in candidates {
        if is_cataloged(index, Some(&candidate.url), candidate.repo.as_deref()) {
            known.push(candidate);
        } else {
            new_ones.push(candidate);
        }
    }
    (new_ones, known)
}

pub fn build_github_queries(since: &str) -> Vec<String> {
    let extra = format!(" pushed:>={since} archived:false fork:false");
    let mut queries: Vec<String> = GITHUB_TOPICS
        .iter()
        .map(|topic| format!("language:Rust topic:{topic}{extra}"))
        .collect();
    queries.extend(
        GITHUB_KEYWORDS
            .iter()
            .map(|keyword| format!("language:Rust {keyword}{extra}")),
    );
    queries
}

fn github_headers(token: &str) -> Vec<(&str, String)> {
    vec![
        ("Authorization", format!("Bearer {token}")),
        ("Accept", "application/vnd.github+json".into()),
        ("X-GitHub-Api-Version", "2022-11-28".into()),
        ("User-Agent", USER_AGENT.into()),
    ]
}

pub fn fetch_github(
    client: &dyn HttpClient,
    token: Option<&str>,
    days: i64,
    now: DateTime<Utc>,
    sleep_s: f64,
    queries: Option<&[String]>,
) -> (Vec<Candidate>, String) {
    let Some(token) = token else {
        return (Vec::new(), "skipped: no GITHUB_TOKEN / GH_TOKEN".into());
    };
    let since = (now - ChronoDuration::days(days)).date_naive().to_string();
    let owned = queries.map(|q| q.to_vec());
    let query_list = owned.unwrap_or_else(|| build_github_queries(&since));
    let headers = github_headers(token);
    let mut found = Vec::new();
    let mut incomplete = false;
    let mut notes = Vec::new();
    for (index, query) in query_list.iter().enumerate() {
        if index > 0 && sleep_s > 0.0 {
            thread::sleep(Duration::from_secs_f64(sleep_s));
        }
        let query_params = vec![
            ("q", query.clone()),
            ("per_page", "100".into()),
            ("sort", "updated".into()),
            ("order", "desc".into()),
        ];
        let (status, payload) = match client.get_json(GITHUB_SEARCH_URL, &query_params, &headers) {
            Ok(result) => result,
            Err(err) => return (found, format!("incomplete: {err}")),
        };
        if status == 401 || status == 403 {
            incomplete = true;
            notes.push(format!("GitHub Search HTTP {status}"));
            break;
        }
        if status != 200 {
            incomplete = true;
            notes.push(format!("query `{query}` HTTP {status}"));
            continue;
        }
        if payload.get("incomplete_results").and_then(Value::as_bool) == Some(true) {
            incomplete = true;
        }
        if let Some(items) = payload.get("items").and_then(Value::as_array) {
            for item in items {
                if let Some(candidate) = parse_github_item(item, query) {
                    found.push(candidate);
                }
            }
        }
    }
    if incomplete {
        let extra = if notes.is_empty() {
            "partial GitHub Search results".into()
        } else {
            notes.join("; ")
        };
        (found, format!("incomplete: {extra}"))
    } else {
        (found, "ok".into())
    }
}

pub fn fetch_biorxiv(
    client: &dyn HttpClient,
    days: i64,
    now: DateTime<Utc>,
    sleep_s: f64,
) -> (Vec<Candidate>, String) {
    let start = (now - ChronoDuration::days(days)).date_naive().to_string();
    let end = now.date_naive().to_string();
    let headers = vec![("User-Agent", USER_AGENT.to_string())];
    let mut found = Vec::new();
    let mut cursor: i64 = 0;
    loop {
        let url = format!("{BIORXIV_DETAILS_URL}/{start}/{end}/{cursor}");
        let (status, payload) = match client.get_json(&url, &[], &headers) {
            Ok(result) => result,
            Err(err) => return (found, format!("incomplete: {err}")),
        };
        if status != 200 {
            return (found, format!("incomplete: bioRxiv HTTP {status}"));
        }
        let collection = payload
            .get("collection")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for paper in &collection {
            if let Some(candidate) = parse_biorxiv_paper(paper) {
                found.push(candidate);
            }
        }
        let messages = payload.get("messages").and_then(Value::as_array);
        let mut total = 0i64;
        let mut count = collection.len() as i64;
        if let Some(first) = messages.and_then(|m| m.first()) {
            if let Some(obj) = first.as_object() {
                total = obj.get("total").and_then(Value::as_i64).unwrap_or(0);
                count = obj.get("count").and_then(Value::as_i64).unwrap_or(count);
            }
        }
        if collection.is_empty() {
            break;
        }
        cursor += count.max(1);
        if total > 0 && cursor >= total {
            break;
        }
        if total == 0 && count < 100 {
            break;
        }
        if cursor >= 8000 {
            return (found, "incomplete: bioRxiv page cap reached".into());
        }
        if sleep_s > 0.0 {
            thread::sleep(Duration::from_secs_f64(sleep_s));
        }
    }
    (found, "ok".into())
}

pub fn render_report(
    today: &str,
    days: i64,
    sources: &[String],
    statuses: &HashMap<String, String>,
    candidates: &[Candidate],
    known_hits: &[Candidate],
) -> String {
    let incomplete = VALID_SOURCES.iter().any(|source| {
        sources.iter().any(|s| s == source)
            && !statuses
                .get(*source)
                .map(|status| status.starts_with("ok"))
                .unwrap_or(false)
    });
    let mut lines = vec![
        format!("# 新工具候选 · {today}"),
        String::new(),
        "手动发现报告，不是收录清单。勾选后再改 `data/tools.yaml`。不要手改 README / RADAR。"
            .into(),
        String::new(),
        "## 数据完整度".into(),
        String::new(),
    ];
    if incomplete {
        lines.push("**数据不完整。** 部分源未跑完，候选可能漏。".into());
        lines.push(String::new());
    }
    for source in VALID_SOURCES {
        let requested = sources.iter().any(|s| s == source);
        let mut status = statuses
            .get(*source)
            .cloned()
            .unwrap_or_else(|| "skipped: disabled".into());
        if !requested {
            status = "skipped: disabled".into();
        }
        lines.push(format!("- {source}：{status}"));
    }
    lines.push(String::new());
    lines.push(format!(
        "检索窗口：最近 {days} 天。源：{}。",
        if sources.is_empty() {
            "无".into()
        } else {
            sources.join(", ")
        }
    ));
    lines.push(String::new());
    lines.push("## 候选".into());
    lines.push(String::new());
    if candidates.is_empty() {
        lines.push("_本轮没有未收录候选。_".into());
        lines.push(String::new());
    } else {
        for (index, candidate) in candidates.iter().enumerate() {
            lines.extend([
                format!("### {}. {}", index + 1, candidate.name),
                String::new(),
                format!("- url: {}", candidate.url),
                format!("- repo: {}", candidate.repo.as_deref().unwrap_or("")),
                format!(
                    "- crates.io: {}",
                    candidate.crates_io.as_deref().unwrap_or("")
                ),
                format!("- 来源: {}", candidate.source),
                format!("- 为什么被捞到: {}", candidate.why),
                "- 已在目录: 否".into(),
                format!("- 建议 category: {}", candidate.suggested_category),
                "- 收录？是/否：".into(),
                String::new(),
            ]);
        }
    }
    lines.extend([
        "## 已收录仍被搜到".into(),
        String::new(),
        format!(
            "本轮命中已在 `tools.yaml`（含 retired）的条目 {} 条，不列入上方候选。",
            known_hits.len()
        ),
        String::new(),
    ]);
    lines.join("\n")
}

pub fn cmd_discover(root: &Path, days: i64, sources_raw: &str, output: Option<PathBuf>) -> i32 {
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
    let tools: Vec<Tool> = docs
        .iter()
        .filter_map(crate::catalog::value_to_tool)
        .collect();
    let days = days.clamp(1, 365);
    let sources = parse_sources(sources_raw);
    let index = catalog_index(&tools);
    let now = utcnow();
    let mut collected = Vec::new();
    let mut statuses = HashMap::new();

    if sources.iter().any(|s| s == "github") {
        let token = std::env::var("GITHUB_TOKEN")
            .ok()
            .or_else(|| std::env::var("GH_TOKEN").ok());
        match ReqwestClient::new() {
            Ok(client) => {
                let (cands, status) = fetch_github(&client, token.as_deref(), days, now, 2.0, None);
                eprintln!("github: {status} ({} hits)", cands.len());
                statuses.insert("github".into(), status);
                collected.extend(cands);
            }
            Err(err) => {
                let status = format!("incomplete: {err}");
                eprintln!("github: {status} (0 hits)");
                statuses.insert("github".into(), status);
            }
        }
    } else {
        statuses.insert("github".into(), "skipped: disabled".into());
    }

    if sources.iter().any(|s| s == "biorxiv") {
        match ReqwestClient::new() {
            Ok(client) => {
                let (cands, status) = fetch_biorxiv(&client, days, now, 0.2);
                eprintln!("biorxiv: {status} ({} hits)", cands.len());
                statuses.insert("biorxiv".into(), status);
                collected.extend(cands);
            }
            Err(err) => {
                let status = format!("incomplete: {err}");
                eprintln!("biorxiv: {status} (0 hits)");
                statuses.insert("biorxiv".into(), status);
            }
        }
    } else {
        statuses.insert("biorxiv".into(), "skipped: disabled".into());
    }

    let merged = dedupe_candidates(collected);
    let (mut new_ones, known_hits) = split_new_and_known(merged, &index);
    new_ones.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    let today = now.date_naive().to_string();
    let text = render_report(&today, days, &sources, &statuses, &new_ones, &known_hits);
    let mut path =
        output.unwrap_or_else(|| paths::discover_dir(root).join(format!("candidates-{today}.md")));
    if !path.is_absolute() {
        path = root.join(path);
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(err) = std::fs::write(&path, text) {
        eprintln!("{err}");
        return 1;
    }
    let shown = path.strip_prefix(root).unwrap_or(&path);
    println!(
        "Wrote {} ({} candidates, {} already listed)",
        shown.display(),
        new_ones.len(),
        known_hits.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Tool;
    use crate::http::MockHttp;
    use chrono::TimeZone;
    use serde_json::json;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 8, 30, 12, 0, 0).unwrap()
    }

    fn index_from(tools: &[Tool]) -> CatalogIndex {
        catalog_index(tools)
    }

    #[test]
    fn exclude_generic_csv_and_slurm() {
        assert!(is_excluded("A fast CSV parser for huge tables."));
        assert!(is_excluded("Slurm TUI dashboard for HPC clusters."));
        assert!(is_excluded(
            "Cluster dashboard with no biology-specific purpose."
        ));
        assert!(!is_excluded("Export VCF and FASTQ summaries as CSV."));
    }

    #[test]
    fn keep_bioinformatics_drop_unrelated_rust() {
        assert!(looks_bio_related("Rust aligner for long-read genomics."));
        assert!(!looks_bio_related("Generic web framework in Rust."));
        assert!(!looks_bio_related("CSV spreadsheet toolkit."));
    }

    #[test]
    fn biorxiv_needs_programming_rust_signal() {
        assert!(has_rust_signal(
            "We implemented the caller in Rust and released it on crates.io."
        ));
        assert!(has_rust_signal("A Rust-based toolkit for metagenomics."));
        assert!(!has_rust_signal(
            "Wheat rust resistance in genomic selection."
        ));
        assert!(!has_rust_signal("Rust-colored colonies were observed."));
    }

    #[test]
    fn extract_github_and_crates_links() {
        let text = "See https://github.com/lab/newtool.git and https://crates.io/crates/newtool plus https://github.com/topics/genomics.";
        assert_eq!(extract_github_repos(text), ["lab/newtool"]);
        assert_eq!(
            extract_crates_io(text),
            ["https://crates.io/crates/newtool"]
        );
    }

    #[test]
    fn parse_github_keeps_bio_repo() {
        let item = json!({
            "name": "newtool",
            "full_name": "lab/newtool",
            "html_url": "https://github.com/lab/newtool",
            "description": "A Rust aligner for long-read genomics.",
            "topics": ["bioinformatics", "rust"],
            "language": "Rust",
            "archived": false,
            "fork": false,
            "stargazers_count": 12,
            "pushed_at": "2026-08-29T00:00:00Z",
        });
        let cand = parse_github_item(&item, "language:Rust topic:genomics").unwrap();
        assert_eq!(cand.repo.as_deref(), Some("lab/newtool"));
        assert_eq!(cand.source, "github");
        assert!(cand.why.contains("topic:genomics"));
    }

    #[test]
    fn parse_github_drops_csv_and_unrelated() {
        let csv_item = json!({
            "name": "csvkit-rs",
            "full_name": "lab/csvkit-rs",
            "html_url": "https://github.com/lab/csvkit-rs",
            "description": "A fast CSV parser for huge tables.",
            "topics": ["csv", "rust"],
            "language": "Rust",
            "archived": false,
        });
        let web = json!({
            "name": "axum-demo",
            "full_name": "lab/axum-demo",
            "html_url": "https://github.com/lab/axum-demo",
            "description": "Demo web server.",
            "topics": ["web"],
            "language": "Rust",
            "archived": false,
        });
        assert!(parse_github_item(&csv_item, "language:Rust csv").is_none());
        assert!(parse_github_item(&web, "language:Rust bioinformatics").is_none());
    }

    #[test]
    fn parse_biorxiv_extracts_repo() {
        let paper = json!({
            "doi": "10.1101/2026.08.20.999999",
            "title": "Newtool: a Rust-based caller for long-read genomics",
            "abstract": "We implemented Newtool in Rust. Source: https://github.com/lab/newtool and https://crates.io/crates/newtool.",
            "date": "2026-08-20",
            "category": "bioinformatics",
        });
        let cand = parse_biorxiv_paper(&paper).unwrap();
        assert_eq!(cand.repo.as_deref(), Some("lab/newtool"));
        assert_eq!(cand.source, "biorxiv");
        assert_eq!(
            cand.crates_io.as_deref(),
            Some("https://crates.io/crates/newtool")
        );
    }

    #[test]
    fn parse_biorxiv_skips_plant_rust() {
        let paper = json!({
            "doi": "10.1101/2026.08.01.111111",
            "title": "Wheat rust resistance loci",
            "abstract": "We mapped stem rust resistance in wheat genomics.",
            "date": "2026-08-01",
            "category": "genomics",
        });
        assert!(parse_biorxiv_paper(&paper).is_none());
    }

    #[test]
    fn dedupe_same_repo_from_two_sources() {
        let a = Candidate {
            name: "newtool".into(),
            url: "https://github.com/lab/newtool".into(),
            repo: Some("lab/newtool".into()),
            source: "github".into(),
            why: "GitHub Search 命中 language:Rust topic:genomics。".into(),
            suggested_category: String::new(),
            crates_io: None,
        };
        let b = Candidate {
            name: "Newtool paper".into(),
            url: "https://www.biorxiv.org/content/10.1101/x".into(),
            repo: Some("Lab/Newtool".into()),
            source: "biorxiv".into(),
            why: "bioRxiv 摘要含 Rust + genomics。".into(),
            suggested_category: String::new(),
            crates_io: Some("https://crates.io/crates/newtool".into()),
        };
        let merged = dedupe_candidates(vec![a, b]);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].source.contains("github"));
        assert!(merged[0].source.contains("biorxiv"));
        assert_eq!(merged[0].url, "https://github.com/lab/newtool");
        assert_eq!(
            merged[0].crates_io.as_deref(),
            Some("https://crates.io/crates/newtool")
        );
    }

    #[test]
    fn split_filters_cataloged_including_retired() {
        let index = index_from(&[Tool {
            name: "old".into(),
            url: "https://github.com/o/old".into(),
            repo: Some("o/old".into()),
            category: "core-libraries".into(),
            description: "Test.".into(),
            status: Some("retired".into()),
            reason: None,
        }]);
        let known = Candidate {
            name: "old".into(),
            url: "https://github.com/o/old".into(),
            repo: Some("o/old".into()),
            source: "github".into(),
            why: "already listed".into(),
            suggested_category: String::new(),
            crates_io: None,
        };
        let fresh = Candidate {
            name: "newtool".into(),
            url: "https://github.com/lab/newtool".into(),
            repo: Some("lab/newtool".into()),
            source: "github".into(),
            why: "new".into(),
            suggested_category: String::new(),
            crates_io: None,
        };
        let (new_ones, known_hits) = split_new_and_known(vec![known, fresh], &index);
        assert_eq!(
            new_ones.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
            ["newtool"]
        );
        assert_eq!(
            known_hits
                .iter()
                .map(|c| c.repo.as_deref().unwrap())
                .collect::<Vec<_>>(),
            ["o/old"]
        );
    }

    #[test]
    fn known_hits_only_in_footer_and_include_blank() {
        let cand = Candidate {
            name: "newtool".into(),
            url: "https://github.com/lab/newtool".into(),
            repo: Some("lab/newtool".into()),
            source: "github".into(),
            why: "GitHub Search 命中 language:Rust topic:genomics。".into(),
            suggested_category: "long-reads".into(),
            crates_io: None,
        };
        let known = Candidate {
            name: "noodles".into(),
            url: "https://github.com/zaeleus/noodles".into(),
            repo: Some("zaeleus/noodles".into()),
            source: "github".into(),
            why: "already listed".into(),
            suggested_category: String::new(),
            crates_io: None,
        };
        let mut statuses = HashMap::new();
        statuses.insert("github".into(), "ok".into());
        statuses.insert("biorxiv".into(), "skipped: no token".into());
        let text = render_report(
            "2026-08-30",
            14,
            &["github".into(), "biorxiv".into()],
            &statuses,
            &[cand],
            &[known],
        );
        assert!(text.contains("**数据不完整。**"));
        assert!(text.contains("skipped: no token"));
        assert!(text.contains("收录？是/否："));
        assert!(text.contains("newtool"));
        let before = text.split("## 已收录仍被搜到").next().unwrap();
        assert!(!before.contains("zaeleus/noodles"));
        assert!(text.contains("本轮命中已在 `tools.yaml`（含 retired）的条目 1 条"));
        assert!(!text.contains("为什么值得写"));
    }

    #[test]
    fn disabled_source_is_not_incomplete() {
        let mut statuses = HashMap::new();
        statuses.insert("github".into(), "ok".into());
        statuses.insert("biorxiv".into(), "skipped: disabled".into());
        let text = render_report("2026-08-30", 14, &["github".into()], &statuses, &[], &[]);
        assert!(!text.contains("**数据不完整。**"));
    }

    #[test]
    fn suggest_category_can_be_empty() {
        assert_eq!(
            suggest_category("A Rust aligner for long-read genomics."),
            "long-reads"
        );
        assert_eq!(
            suggest_category("Fast sgRNA counting for CRISPR screens."),
            "crispr"
        );
        assert_eq!(
            suggest_category("Rust crates for protein language models and protein design."),
            "protein-engineering"
        );
        assert_eq!(
            suggest_category("16S microbiome ASV caller."),
            "metagenomics"
        );
        assert_eq!(
            suggest_category("Bacterial transcript units from mapped reads."),
            "prokaryotic-transcriptome"
        );
        assert_eq!(suggest_category("Miscellaneous notes."), "");
    }

    #[test]
    fn github_skips_without_token_and_does_not_call_http() {
        let session = MockHttp::new(200, json!({}));
        let (cands, status) = fetch_github(&session, None, 14, now(), 0.0, None);
        assert!(cands.is_empty());
        assert!(status.to_lowercase().contains("token"));
        assert!(session.calls.borrow().is_empty());
    }

    #[test]
    fn github_parses_mocked_search() {
        let session = MockHttp::new(
            200,
            json!({
                "items": [{
                    "name": "newtool",
                    "full_name": "lab/newtool",
                    "html_url": "https://github.com/lab/newtool",
                    "description": "A Rust aligner for long-read genomics.",
                    "topics": ["genomics", "rust"],
                    "language": "Rust",
                    "archived": false,
                    "fork": false,
                    "stargazers_count": 3,
                    "pushed_at": "2026-08-29T00:00:00Z"
                }]
            }),
        );
        let queries = vec!["language:Rust topic:genomics".to_string()];
        let (cands, status) = fetch_github(&session, Some("t"), 14, now(), 0.0, Some(&queries));
        assert!(status.starts_with("ok"));
        assert_eq!(
            cands
                .iter()
                .map(|c| c.repo.as_deref().unwrap())
                .collect::<Vec<_>>(),
            ["lab/newtool"]
        );
        assert!(!session.calls.borrow().is_empty());
    }

    #[test]
    fn biorxiv_parses_mocked_collection() {
        let session = MockHttp::new(
            200,
            json!({
                "messages": [{"status": "ok", "count": 1, "total": 1}],
                "collection": [{
                    "doi": "10.1101/2026.08.20.999999",
                    "title": "Newtool: a Rust-based caller",
                    "abstract": "Implemented in Rust for genomics. https://github.com/lab/newtool",
                    "date": "2026-08-20",
                    "category": "bioinformatics"
                }]
            }),
        );
        let (cands, status) = fetch_biorxiv(&session, 14, now(), 0.0);
        assert!(status.starts_with("ok"));
        assert_eq!(
            cands
                .iter()
                .map(|c| c.repo.as_deref().unwrap())
                .collect::<Vec<_>>(),
            ["lab/newtool"]
        );
    }

    #[test]
    fn parse_sources_and_github_queries() {
        assert_eq!(parse_sources("github,x"), ["github"]);
        assert_eq!(parse_sources("twitter"), ["github", "biorxiv"]);
        let queries = build_github_queries("2026-08-16");
        assert!(queries.iter().any(|q| q.contains("topic:bioinformatics")));
        assert!(queries.iter().any(|q| q.contains("genomics")));
        assert!(queries.iter().all(|q| q.contains("language:Rust")));
        assert!(queries.iter().all(|q| q.contains("pushed:>=2026-08-16")));
    }
}
