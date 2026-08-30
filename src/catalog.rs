use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;

use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde_yaml::Value;
use url::Url;

pub const REQUIRED_FIELDS: &[&str] = &["name", "url", "category", "description"];
pub const ALLOWED_FIELDS: &[&str] = &["name", "url", "repo", "category", "description", "status"];
pub const VALID_STATUSES: &[&str] = &["retired"];

const GITHUB_RESERVED_OWNERS: &[&str] = &[
    "about",
    "account",
    "apps",
    "codespaces",
    "collections",
    "enterprise",
    "events",
    "explore",
    "features",
    "login",
    "marketplace",
    "new",
    "notifications",
    "organizations",
    "orgs",
    "search",
    "settings",
    "sponsors",
    "topics",
];

#[derive(Debug, Clone, Copy)]
pub struct Category {
    pub slug: &'static str,
    pub title: &'static str,
    pub children: &'static [Category],
}

impl Category {
    pub fn is_section(&self) -> bool {
        !self.children.is_empty()
    }
}

pub const CATEGORIES: &[Category] = &[
    Category {
        slug: "crispr",
        title: "CRISPR",
        children: &[],
    },
    Category {
        slug: "microbial-bioinformatics",
        title: "Microbial Bioinformatics",
        children: &[
            Category {
                slug: "bacterial-assembly",
                title: "Bacterial Genome Assembly",
                children: &[],
            },
            Category {
                slug: "bacterial-annotation",
                title: "Genome Annotation",
                children: &[],
            },
            Category {
                slug: "prokaryotic-transcriptome",
                title: "Prokaryotic Transcriptome",
                children: &[],
            },
            Category {
                slug: "metagenomics",
                title: "Metagenomics",
                children: &[],
            },
            Category {
                slug: "phage-defense",
                title: "Phage Defense Systems",
                children: &[],
            },
            Category {
                slug: "resistance-genes",
                title: "Resistance Genes",
                children: &[],
            },
            Category {
                slug: "transposons",
                title: "Transposon Systems",
                children: &[],
            },
        ],
    },
    Category {
        slug: "core-libraries",
        title: "Core Libraries",
        children: &[],
    },
    Category {
        slug: "sequence-io-and-formats",
        title: "Sequence IO and Formats",
        children: &[],
    },
    Category {
        slug: "alignment-and-mapping",
        title: "Alignment and Mapping",
        children: &[],
    },
    Category {
        slug: "variants-and-annotation",
        title: "Variants and Annotation",
        children: &[],
    },
    Category {
        slug: "long-reads",
        title: "Long Reads",
        children: &[],
    },
    Category {
        slug: "assembly-and-pangenomes",
        title: "Assembly and Pangenomes",
        children: &[],
    },
    Category {
        slug: "single-cell-and-rna",
        title: "Single-cell and RNA",
        children: &[],
    },
    Category {
        slug: "proteomics-and-structure",
        title: "Proteomics and Structure",
        children: &[],
    },
    Category {
        slug: "protein-engineering",
        title: "Protein Engineering",
        children: &[],
    },
    Category {
        slug: "workflows-and-infrastructure",
        title: "Workflows and Infrastructure",
        children: &[],
    },
    Category {
        slug: "visualization",
        title: "Visualization",
        children: &[],
    },
    Category {
        slug: "learning-resources",
        title: "Learning Resources and Related Lists",
        children: &[],
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    pub name: String,
    pub url: String,
    pub repo: Option<String>,
    pub category: String,
    pub description: String,
    pub status: Option<String>,
    pub reason: Option<String>,
}

impl Tool {
    pub fn is_retired(&self) -> bool {
        self.status.as_deref() == Some("retired")
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

pub fn heading_anchor(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

pub fn iter_category_nodes(nodes: &[Category]) -> impl Iterator<Item = &Category> {
    fn walk<'a>(nodes: &'a [Category], out: &mut Vec<&'a Category>) {
        for node in nodes {
            out.push(node);
            if !node.children.is_empty() {
                walk(node.children, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(nodes, &mut out);
    out.into_iter()
}

pub fn valid_categories() -> &'static HashSet<&'static str> {
    static CELL: OnceLock<HashSet<&'static str>> = OnceLock::new();
    CELL.get_or_init(|| {
        iter_category_nodes(CATEGORIES)
            .filter(|node| !node.is_section())
            .map(|node| node.slug)
            .collect()
    })
}

pub fn active_tools(tools: &[Tool]) -> Vec<&Tool> {
    tools.iter().filter(|tool| !tool.is_retired()).collect()
}

pub fn utcnow() -> DateTime<Utc> {
    Utc::now()
}

pub fn parse_dt(value: Option<&str>) -> Option<DateTime<Utc>> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }
    let text = value.replace('Z', "+00:00");
    DateTime::parse_from_rfc3339(&text)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|| {
            chrono::NaiveDateTime::parse_from_str(&text, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .map(|naive| Utc.from_utc_datetime(&naive))
        })
}

pub fn months_ago(now: DateTime<Utc>, months: i32) -> DateTime<Utc> {
    let mut year = now.year();
    let mut month = now.month() as i32 - months;
    while month <= 0 {
        month += 12;
        year -= 1;
    }
    let day = now.day().min(28);
    chrono::NaiveDate::from_ymd_opt(year, month as u32, day)
        .expect("months_ago date")
        .and_time(now.time())
        .and_utc()
}

fn mapping_str(map: &serde_yaml::Mapping, key: &str) -> Option<String> {
    map.get(Value::String(key.to_string()))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn mapping_keys(map: &serde_yaml::Mapping) -> HashSet<String> {
    map.keys()
        .filter_map(|key| key.as_str().map(str::to_string))
        .collect()
}

pub fn load_yaml_list(path: &Path) -> Result<Vec<Value>, String> {
    let text = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    let raw: Value = serde_yaml::from_str(&text).map_err(|err| err.to_string())?;
    match raw {
        Value::Sequence(seq) => Ok(seq),
        Value::Null => Ok(Vec::new()),
        _ => Err("data/tools.yaml must be a list".into()),
    }
}

pub fn load_tools(path: &Path) -> Result<Vec<Tool>, String> {
    let docs = load_yaml_list(path)?;
    let errors = validate_tool_docs(&docs);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    Ok(docs.iter().filter_map(value_to_tool).collect())
}

pub fn value_to_tool(value: &Value) -> Option<Tool> {
    let map = value.as_mapping()?;
    Some(Tool {
        name: mapping_str(map, "name")?,
        url: mapping_str(map, "url")?,
        repo: mapping_str(map, "repo"),
        category: mapping_str(map, "category")?,
        description: mapping_str(map, "description")?,
        status: mapping_str(map, "status"),
        reason: None,
    })
}

pub fn validate_tool_docs(items: &[Value]) -> Vec<String> {
    let mut errors = Vec::new();
    let mut urls = HashSet::new();
    for (index, item) in items.iter().enumerate() {
        let prefix = format!("item {}", index + 1);
        let Some(map) = item.as_mapping() else {
            errors.push(format!("{prefix}: expected a mapping"));
            continue;
        };
        for field in REQUIRED_FIELDS {
            match mapping_str(map, field) {
                Some(value) if !value.trim().is_empty() => {}
                _ => errors.push(format!("{prefix}: missing {field}")),
            }
        }
        let name = mapping_str(map, "name");
        let url = mapping_str(map, "url");
        let category = mapping_str(map, "category");
        let description = mapping_str(map, "description").unwrap_or_default();
        if let Some(url) = url.as_ref() {
            if !urls.insert(url.clone()) {
                errors.push(format!("{prefix}: duplicate url {url}"));
            }
        }
        if let Some(category) = category.as_ref() {
            if !valid_categories().contains(category.as_str()) {
                errors.push(format!("{prefix}: unknown category {category}"));
            }
        }
        if !description.is_empty() {
            let first = description.chars().next().unwrap();
            if !first.is_uppercase() && !first.is_ascii_digit() {
                errors.push(format!(
                    "{prefix} ({}): description should start with a capital",
                    name.as_deref().unwrap_or("")
                ));
            }
            if !description.ends_with('.') {
                errors.push(format!(
                    "{prefix} ({}): description should end with a period",
                    name.as_deref().unwrap_or("")
                ));
            }
        }
        if let Some(repo_val) = map.get(Value::String("repo".into())) {
            let repo = repo_val.as_str();
            let ok = repo
                .map(|repo| {
                    repo.matches('/').count() == 1 && repo.split('/').all(|part| !part.is_empty())
                })
                .unwrap_or(false);
            if !ok {
                errors.push(format!(
                    "{prefix} ({}): repo must be owner/name",
                    name.as_deref().unwrap_or("")
                ));
            }
        }
        if let Some(status_val) = map.get(Value::String("status".into())) {
            let status = status_val.as_str();
            if status.is_none_or(|status| !VALID_STATUSES.contains(&status)) {
                errors.push(format!(
                    "{prefix} ({}): status must be one of {:?}",
                    name.as_deref().unwrap_or(""),
                    {
                        let mut statuses = VALID_STATUSES.to_vec();
                        statuses.sort();
                        statuses
                    }
                ));
            }
        }
        let extra: Vec<String> = mapping_keys(map)
            .into_iter()
            .filter(|key| !ALLOWED_FIELDS.contains(&key.as_str()))
            .collect();
        if !extra.is_empty() {
            let mut extra = extra;
            extra.sort();
            errors.push(format!(
                "{prefix} ({}): unknown fields {extra:?}",
                name.as_deref().unwrap_or("")
            ));
        }
    }
    errors
}

pub fn normalize_url(url: &str) -> String {
    let raw = url.trim();
    if raw.is_empty() {
        return String::new();
    }
    let parsed = match Url::parse(raw) {
        Ok(parsed) => parsed,
        Err(_) => return raw.to_string(),
    };
    let scheme = if parsed.scheme().is_empty() {
        "https"
    } else {
        parsed.scheme()
    };
    let mut host = parsed.host_str().unwrap_or("").to_lowercase();
    if let Some(stripped) = host.strip_prefix("www.") {
        host = stripped.to_string();
    }
    let authority = if let Some(port) = parsed.port() {
        format!("{host}:{port}")
    } else {
        host
    };
    let mut path = parsed.path().trim_end_matches('/').to_string();
    if path.ends_with(".git") {
        path.truncate(path.len() - 4);
    }
    if authority.is_empty() {
        return raw.to_string();
    }
    format!("{scheme}://{authority}{path}")
}

fn reserved_owners() -> &'static HashSet<&'static str> {
    static CELL: OnceLock<HashSet<&'static str>> = OnceLock::new();
    CELL.get_or_init(|| GITHUB_RESERVED_OWNERS.iter().copied().collect())
}

pub fn github_repo_from_url(url: &str) -> Option<String> {
    let parsed = Url::parse(&normalize_url(url)).ok()?;
    let host = parsed.host_str()?.to_lowercase();
    if host != "github.com" {
        return None;
    }
    let parts: Vec<&str> = parsed
        .path()
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    if parts.len() < 2 {
        return None;
    }
    let owner = parts[0];
    let mut name = parts[1].to_string();
    if reserved_owners().contains(&owner.to_lowercase().as_str()) {
        return None;
    }
    if let Some(stripped) = name.strip_suffix(".git") {
        name = stripped.to_string();
    }
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some(format!("{owner}/{name}"))
}

#[derive(Debug, Clone)]
pub struct CatalogIndex {
    pub urls: HashSet<String>,
    pub repos: HashSet<String>,
}

pub fn catalog_index(tools: &[Tool]) -> CatalogIndex {
    let mut urls = HashSet::new();
    let mut repos = HashSet::new();
    for tool in tools {
        if !tool.url.trim().is_empty() {
            urls.insert(normalize_url(&tool.url));
            if let Some(inferred) = github_repo_from_url(&tool.url) {
                repos.insert(inferred.to_lowercase());
            }
        }
        if let Some(repo) = tool.repo.as_ref() {
            if !repo.trim().is_empty() {
                repos.insert(repo.trim().to_lowercase());
            }
        }
    }
    CatalogIndex { urls, repos }
}

pub fn is_cataloged(index: &CatalogIndex, url: Option<&str>, repo: Option<&str>) -> bool {
    if let Some(repo) = repo {
        if !repo.trim().is_empty() && index.repos.contains(&repo.trim().to_lowercase()) {
            return true;
        }
    }
    if let Some(url) = url {
        if !url.trim().is_empty() {
            if index.urls.contains(&normalize_url(url)) {
                return true;
            }
            if let Some(inferred) = github_repo_from_url(url) {
                if index.repos.contains(&inferred.to_lowercase()) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn cmd_validate(root: &Path) -> i32 {
    match load_yaml_list(&crate::paths::tools_path(root)) {
        Ok(docs) => {
            let errors = validate_tool_docs(&docs);
            if errors.is_empty() {
                println!("tools.yaml ok");
                0
            } else {
                println!("tools.yaml is invalid:");
                for item in errors {
                    println!("  - {item}");
                }
                1
            }
        }
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_url_strips_slash_query_and_git() {
        assert_eq!(
            normalize_url("https://WWW.GitHub.com/Lab/Tool.git/?tab=readme#x"),
            "https://github.com/Lab/Tool"
        );
    }

    #[test]
    fn github_repo_from_nested_and_reserved_paths() {
        assert_eq!(
            github_repo_from_url("https://github.com/Lab/Tool/issues/3").as_deref(),
            Some("Lab/Tool")
        );
        assert_eq!(
            github_repo_from_url("https://github.com/topics/bioinformatics"),
            None
        );
        assert_eq!(
            github_repo_from_url("https://crates.io/crates/noodles"),
            None
        );
    }

    #[test]
    fn known_url_and_trailing_slash() {
        let index = catalog_index(&[Tool {
            name: "noodles".into(),
            url: "https://github.com/zaeleus/noodles".into(),
            repo: Some("zaeleus/noodles".into()),
            category: "core-libraries".into(),
            description: "Test.".into(),
            status: None,
            reason: None,
        }]);
        assert!(is_cataloged(
            &index,
            Some("https://github.com/zaeleus/noodles/"),
            None
        ));
        assert!(is_cataloged(&index, None, Some("Zaeleus/Noodles")));
        assert!(!is_cataloged(
            &index,
            Some("https://github.com/lab/newtool"),
            None
        ));
    }

    #[test]
    fn retired_and_crates_url_still_count() {
        let index = catalog_index(&[
            Tool {
                name: "gone".into(),
                url: "https://github.com/o/gone".into(),
                repo: Some("o/gone".into()),
                category: "core-libraries".into(),
                description: "Test.".into(),
                status: Some("retired".into()),
                reason: None,
            },
            Tool {
                name: "crate-only".into(),
                url: "https://crates.io/crates/example".into(),
                repo: Some("org/example".into()),
                category: "core-libraries".into(),
                description: "Test.".into(),
                status: None,
                reason: None,
            },
        ]);
        assert!(is_cataloged(&index, None, Some("o/gone")));
        assert!(is_cataloged(
            &index,
            Some("https://github.com/org/example"),
            None
        ));
        assert!(is_cataloged(
            &index,
            Some("https://crates.io/crates/example"),
            None
        ));
    }
}
