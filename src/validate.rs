use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::model::{Catalog, CategoryDef, ALLOWED_DOMAINS, ALLOWED_FORMATS, ALLOWED_INTERFACES};

pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn collect_category_slugs(
    categories: &indexmap::IndexMap<String, CategoryDef>,
) -> HashSet<String> {
    let mut slugs = HashSet::new();
    for (slug, cat) in categories {
        slugs.insert(slug.clone());
        for (child_slug, _) in &cat.children {
            slugs.insert(child_slug.clone());
            slugs.insert(format!("{slug}/{child_slug}"));
        }
    }
    slugs
}

pub fn validate_catalog(catalog: &Catalog) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if catalog.schema_version != 2 {
        errors.push(format!(
            "Invalid schema_version: expected 2, found {}",
            catalog.schema_version
        ));
    }

    if catalog.categories.is_empty() {
        errors.push("No categories defined in catalog".into());
    }

    let valid_category_slugs = collect_category_slugs(&catalog.categories);

    let mut seen_keys = HashSet::new();
    let mut seen_repos: HashMap<String, String> = HashMap::new();

    for (key, tool) in &catalog.tools {
        // Key validation
        if !seen_keys.insert(key.to_lowercase()) {
            errors.push(format!("Duplicate tool key: '{key}'"));
        }

        // Name
        if tool.name.trim().is_empty() {
            errors.push(format!("Tool '{key}': name cannot be empty"));
        }

        // Description
        if tool.description.trim().is_empty() {
            errors.push(format!("Tool '{key}': description cannot be empty"));
        }

        // Repository format
        let repo = tool.repository.trim();
        if repo.is_empty() {
            errors.push(format!("Tool '{key}': repository cannot be empty"));
        } else if !repo.contains('/') {
            errors.push(format!(
                "Tool '{key}': repository '{repo}' must be in 'owner/name' format"
            ));
        } else {
            let repo_lower = repo.to_lowercase();
            if let Some(existing_key) = seen_repos.get(&repo_lower) {
                errors.push(format!(
                    "Duplicate repository '{repo}' used in tools '{existing_key}' and '{key}'"
                ));
            } else {
                seen_repos.insert(repo_lower, key.clone());
            }
        }

        // Category validation
        let primary = &tool.category.primary;
        if !valid_category_slugs.contains(primary) {
            errors.push(format!(
                "Tool '{key}': unknown primary category '{primary}'. Valid categories include: {:?}",
                valid_category_slugs.iter().take(10).collect::<Vec<_>>()
            ));
        }

        for sec in &tool.category.secondary {
            if !valid_category_slugs.contains(sec) {
                errors.push(format!("Tool '{key}': unknown secondary category '{sec}'"));
            }
        }

        // Architecture
        for iface in &tool.architecture.interfaces {
            let iface_lower = iface.to_lowercase();
            if !ALLOWED_INTERFACES.contains(&iface_lower.as_str()) {
                errors.push(format!(
                    "Tool '{key}': invalid interface '{iface}'. Allowed: {:?}",
                    ALLOWED_INTERFACES
                ));
            }
        }

        // Domains
        for domain in &tool.domains {
            if !ALLOWED_DOMAINS.contains(&domain.as_str()) {
                warnings.push(format!(
                    "Tool '{key}': domain '{domain}' is not in standard domains list"
                ));
            }
        }

        // Formats
        for fmt in &tool.formats {
            if !ALLOWED_FORMATS.contains(&fmt.as_str()) {
                warnings.push(format!(
                    "Tool '{key}': format '{fmt}' is not in standard formats list"
                ));
            }
        }

        // Publications
        for (idx, pub_record) in tool.publications.iter().enumerate() {
            if pub_record.title.trim().is_empty() {
                errors.push(format!("Tool '{key}': publication #{idx} has empty title"));
            }
            if let Some(doi) = &pub_record.doi {
                let trimmed = doi.trim();
                if !trimmed.starts_with("10.") {
                    errors.push(format!(
                        "Tool '{key}': DOI '{trimmed}' does not start with standard prefix '10.'"
                    ));
                }
            }
        }

        // Learning
        if let Some(ref learning) = tool.learning {
            if learning.recommended && learning.topics.is_empty() {
                warnings.push(format!(
                    "Tool '{key}': marked recommended for learning but has no topics specified"
                ));
            }
        }
    }

    ValidationReport { errors, warnings }
}

pub fn load_and_validate(path: &Path) -> Result<(Catalog, ValidationReport), String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let catalog: Catalog = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML from {}: {}", path.display(), e))?;
    let report = validate_catalog(&catalog);
    Ok((catalog, report))
}

pub fn cmd_validate_v2(path: &Path) -> i32 {
    match load_and_validate(path) {
        Ok((catalog, report)) => {
            for warn in &report.warnings {
                eprintln!("WARNING: {warn}");
            }
            if !report.is_valid() {
                eprintln!("Validation failed with {} errors:", report.errors.len());
                for err in &report.errors {
                    eprintln!("  - {err}");
                }
                1
            } else {
                let active_count = catalog
                    .tools
                    .values()
                    .filter(|t| t.is_active_in_catalog())
                    .count();
                let retired_count = catalog.tools.values().filter(|t| t.is_retired()).count();
                println!(
                    "Validation passed: {} tools total ({} active, {} retired), {} categories.",
                    catalog.tools.len(),
                    active_count,
                    retired_count,
                    catalog.categories.len()
                );
                0
            }
        }
        Err(err) => {
            eprintln!("Validation error: {err}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use indexmap::IndexMap;

    fn sample_valid_catalog() -> Catalog {
        let mut categories = IndexMap::new();
        categories.insert(
            "crispr".to_string(),
            CategoryDef {
                name: "CRISPR".to_string(),
                description: None,
                order: 10,
                children: IndexMap::new(),
            },
        );

        let mut tools = IndexMap::new();
        tools.insert(
            "test-tool".to_string(),
            ToolDef {
                name: "test-tool".to_string(),
                repository: "test/test-tool".to_string(),
                url: None,
                description: "A test tool for unit testing.".to_string(),
                category: ToolCategory {
                    primary: "crispr".to_string(),
                    secondary: vec![],
                },
                domains: vec!["crispr".to_string()],
                architecture: Architecture {
                    rust_role: RustRole::Native,
                    interfaces: vec!["cli".to_string()],
                    foreign_dependencies: vec![],
                },
                technologies: vec!["rust".to_string()],
                formats: vec!["FASTA".to_string()],
                learning: None,
                publications: vec![Publication {
                    doi: Some("10.1093/bioinformatics/test".to_string()),
                    url: Some("https://doi.org/10.1093/bioinformatics/test".to_string()),
                    title: "Test Publication".to_string(),
                    pub_type: Some("journal".to_string()),
                }],
                status: ToolStatus {
                    catalog: CatalogStatus::Active,
                },
                notes: None,
            },
        );

        Catalog {
            schema_version: 2,
            categories,
            tools,
        }
    }

    #[test]
    fn valid_catalog_passes() {
        let cat = sample_valid_catalog();
        let report = validate_catalog(&cat);
        assert!(
            report.is_valid(),
            "Expected valid, got: {:?}",
            report.errors
        );
    }

    #[test]
    fn invalid_schema_version_fails() {
        let mut cat = sample_valid_catalog();
        cat.schema_version = 1;
        let report = validate_catalog(&cat);
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("schema_version")));
    }

    #[test]
    fn unknown_category_fails() {
        let mut cat = sample_valid_catalog();
        cat.tools.get_mut("test-tool").unwrap().category.primary = "non-existent".to_string();
        let report = validate_catalog(&cat);
        assert!(!report.is_valid());
        assert!(report
            .errors
            .iter()
            .any(|e| e.contains("unknown primary category")));
    }

    #[test]
    fn invalid_interface_fails() {
        let mut cat = sample_valid_catalog();
        cat.tools
            .get_mut("test-tool")
            .unwrap()
            .architecture
            .interfaces = vec!["invalid_interface".to_string()];
        let report = validate_catalog(&cat);
        assert!(!report.is_valid());
        assert!(report
            .errors
            .iter()
            .any(|e| e.contains("invalid interface")));
    }

    #[test]
    fn invalid_doi_fails() {
        let mut cat = sample_valid_catalog();
        cat.tools.get_mut("test-tool").unwrap().publications[0].doi = Some("bad-doi".to_string());
        let report = validate_catalog(&cat);
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("DOI 'bad-doi'")));
    }
}
