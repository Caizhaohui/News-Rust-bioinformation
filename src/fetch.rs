use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

use crate::catalog::{load_yaml_list, utcnow, validate_tool_docs, value_to_tool};
use crate::config::load_config;
use crate::http::{HttpClient, ReqwestClient};
use crate::metadata::{dump_json, prune_snapshots, Metadata, RepoRecord};
use crate::paths;

const GRAPHQL_URL: &str = "https://api.github.com/graphql";
const BATCH_SIZE: usize = 20;

fn alias(index: usize) -> String {
    format!("r{index}")
}

pub fn build_query(batch: &[(String, String, String)]) -> String {
    let mut fields = Vec::new();
    for (index, (_repo, owner, name)) in batch.iter().enumerate() {
        let alias = alias(index);
        fields.push(format!(
            r#"
            {alias}: repository(owner: "{owner}", name: "{name}") {{
              stargazerCount
              pushedAt
              isArchived
              latestRelease {{ publishedAt tagName }}
            }}
            "#
        ));
    }
    format!("query {{\n{}\n}}", fields.join("\n"))
}

fn empty_record(error: impl Into<String>) -> RepoRecord {
    RepoRecord {
        stars: None,
        pushed_at: None,
        archived: None,
        latest_release_at: None,
        latest_release_tag: None,
        error: Some(error.into()),
    }
}

pub fn fetch_batch(
    client: &dyn HttpClient,
    token: &str,
    batch: &[(String, String, String)],
) -> Result<BTreeMap<String, RepoRecord>, String> {
    let query = build_query(batch);
    let body = json!({ "query": query });
    let headers: [(&str, String); 2] = [
        ("Authorization", format!("bearer {token}")),
        ("Accept", "application/vnd.github+json".into()),
    ];
    let header_refs: Vec<(&str, String)> = headers.iter().map(|(k, v)| (*k, v.clone())).collect();
    let (status, payload) = client.post_json(GRAPHQL_URL, &body, &header_refs)?;
    if status != 200 {
        return Err(format!("GraphQL HTTP {status}: {payload}"));
    }
    if payload.get("errors").is_some() && payload.get("data").is_none() {
        return Err(format!(
            "GraphQL errors: {}",
            payload.get("errors").unwrap_or(&Value::Null)
        ));
    }
    let data = payload.get("data").cloned().unwrap_or(Value::Null);
    let mut errors_by_alias = BTreeMap::new();
    if let Some(errors) = payload.get("errors").and_then(Value::as_array) {
        for err in errors {
            if let Some(path) = err.get("path").and_then(Value::as_array) {
                if let Some(first) = path.first().and_then(Value::as_str) {
                    errors_by_alias.insert(
                        first.to_string(),
                        err.get("message")
                            .and_then(Value::as_str)
                            .unwrap_or("graphql error")
                            .to_string(),
                    );
                }
            }
        }
    }
    let mut results = BTreeMap::new();
    for (index, (repo, _owner, _name)) in batch.iter().enumerate() {
        let alias = alias(index);
        let node = data.get(&alias);
        if node.is_none() || node == Some(&Value::Null) {
            results.insert(
                repo.clone(),
                empty_record(
                    errors_by_alias
                        .get(&alias)
                        .cloned()
                        .unwrap_or_else(|| "not found".into()),
                ),
            );
            continue;
        }
        let node = node.unwrap();
        let release = node.get("latestRelease").cloned().unwrap_or(Value::Null);
        results.insert(
            repo.clone(),
            RepoRecord {
                stars: node.get("stargazerCount").and_then(Value::as_i64),
                pushed_at: node
                    .get("pushedAt")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                archived: node.get("isArchived").and_then(Value::as_bool),
                latest_release_at: release
                    .get("publishedAt")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                latest_release_tag: release
                    .get("tagName")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                error: None,
            },
        );
    }
    Ok(results)
}

pub fn cmd_fetch_metadata(root: &Path) -> i32 {
    let tools_path = paths::tools_path(root);
    let docs = match load_yaml_list(&tools_path) {
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
    let tools: Vec<_> = docs.iter().filter_map(value_to_tool).collect();
    let token = std::env::var("GITHUB_TOKEN")
        .ok()
        .or_else(|| std::env::var("GH_TOKEN").ok())
        .filter(|value| !value.is_empty());
    let urls: Vec<String> = tools.iter().map(|tool| tool.url.clone()).collect();
    let mut repos: Vec<String> = tools.iter().filter_map(|tool| tool.repo.clone()).collect();
    repos.sort();
    repos.dedup();
    let parsed: Vec<(String, String, String)> = repos
        .into_iter()
        .filter_map(|repo| {
            let (owner, name) = repo.split_once('/')?;
            Some((repo.clone(), owner.to_string(), name.to_string()))
        })
        .collect();

    let mut records = BTreeMap::new();
    let mut incomplete = false;
    if let Some(token) = token {
        match ReqwestClient::new() {
            Ok(client) => {
                let mut failed = None;
                for chunk in parsed.chunks(BATCH_SIZE) {
                    match fetch_batch(&client, &token, chunk) {
                        Ok(batch) => records.extend(batch),
                        Err(err) => {
                            failed = Some(err);
                            break;
                        }
                    }
                }
                if let Some(err) = failed {
                    incomplete = true;
                    eprintln!("Fetch failed: {err}");
                    for (repo, _, _) in &parsed {
                        records
                            .entry(repo.clone())
                            .or_insert_with(|| empty_record(err.clone()));
                    }
                }
            }
            Err(err) => {
                incomplete = true;
                eprintln!("Fetch failed: {err}");
                for (repo, _, _) in &parsed {
                    records.insert(repo.clone(), empty_record(err.clone()));
                }
            }
        }
    } else {
        incomplete = true;
        for (repo, _, _) in &parsed {
            records.insert(repo.clone(), empty_record("no_token"));
        }
        eprintln!("No GITHUB_TOKEN; writing incomplete metadata.");
    }

    let metadata = Metadata {
        fetched_at: utcnow().to_rfc3339_opts(chrono::SecondsFormat::Micros, false),
        incomplete,
        urls,
        repos: records.clone(),
        date: None,
    };
    let meta_path = paths::metadata_path(root);
    if let Err(err) = dump_json(&meta_path, &metadata) {
        eprintln!("{err}");
        return 1;
    }
    let today = utcnow().date_naive().to_string();
    let mut snapshot = metadata;
    snapshot.date = Some(today.clone());
    let snap_dir = paths::snapshot_dir(root);
    if let Err(err) = dump_json(&snap_dir.join(format!("{today}.json")), &snapshot) {
        eprintln!("{err}");
        return 1;
    }
    let keep = load_config(&paths::config_path(root)).snapshots_keep;
    prune_snapshots(&snap_dir, keep);
    println!(
        "Wrote {} for {} repos",
        meta_path.strip_prefix(root).unwrap_or(&meta_path).display(),
        records.len()
    );
    0
}
