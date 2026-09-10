use indexmap::IndexMap;
use serde::Deserialize;
use std::path::Path;

use crate::model::{
    Architecture, Catalog, CatalogStatus, CategoryDef, Learning, LearningLevel, Publication,
    RustRole, ToolCategory, ToolDef, ToolStatus,
};

#[derive(Debug, Deserialize)]
struct V1Tool {
    name: String,
    url: String,
    repo: Option<String>,
    category: String,
    description: String,
    status: Option<String>,
    #[serde(default)]
    papers: Vec<V1Paper>,
}

#[derive(Debug, Deserialize)]
struct V1Paper {
    title: String,
    url: String,
}

pub fn default_v2_categories() -> IndexMap<String, CategoryDef> {
    let mut categories = IndexMap::new();

    categories.insert(
        "crispr".to_string(),
        CategoryDef {
            name: "CRISPR".to_string(),
            description: Some(
                "CRISPR screening, guide design, genome editing and related tools.".to_string(),
            ),
            order: 10,
            children: IndexMap::new(),
        },
    );

    let mut microbial_children = IndexMap::new();
    microbial_children.insert(
        "bacterial-assembly".to_string(),
        CategoryDef {
            name: "Bacterial Genome Assembly".to_string(),
            description: None,
            order: 10,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "genome-annotation".to_string(),
        CategoryDef {
            name: "Genome Annotation".to_string(),
            description: None,
            order: 20,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "prokaryotic-transcriptome".to_string(),
        CategoryDef {
            name: "Prokaryotic Transcriptome".to_string(),
            description: None,
            order: 30,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "metagenomics".to_string(),
        CategoryDef {
            name: "Metagenomics".to_string(),
            description: None,
            order: 40,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "phage-defense".to_string(),
        CategoryDef {
            name: "Phage Defense Systems".to_string(),
            description: None,
            order: 50,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "antimicrobial-resistance".to_string(),
        CategoryDef {
            name: "Antimicrobial Resistance".to_string(),
            description: None,
            order: 60,
            children: IndexMap::new(),
        },
    );
    microbial_children.insert(
        "transposons".to_string(),
        CategoryDef {
            name: "Transposon Systems".to_string(),
            description: None,
            order: 70,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "microbial".to_string(),
        CategoryDef {
            name: "Microbial Bioinformatics".to_string(),
            description: Some(
                "Genomics and bioinformatics tools focused on microorganisms.".to_string(),
            ),
            order: 20,
            children: microbial_children,
        },
    );

    categories.insert(
        "core-libraries".to_string(),
        CategoryDef {
            name: "Core Libraries".to_string(),
            description: None,
            order: 30,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "sequence-io".to_string(),
        CategoryDef {
            name: "Sequence IO and Formats".to_string(),
            description: None,
            order: 40,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "alignment".to_string(),
        CategoryDef {
            name: "Alignment and Mapping".to_string(),
            description: None,
            order: 50,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "variants".to_string(),
        CategoryDef {
            name: "Variants and Annotation".to_string(),
            description: None,
            order: 60,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "long-reads".to_string(),
        CategoryDef {
            name: "Long Reads".to_string(),
            description: None,
            order: 70,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "assembly-pangenome".to_string(),
        CategoryDef {
            name: "Assembly and Pangenomes".to_string(),
            description: None,
            order: 80,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "single-cell-rna".to_string(),
        CategoryDef {
            name: "Single-cell and RNA".to_string(),
            description: None,
            order: 90,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "proteomics".to_string(),
        CategoryDef {
            name: "Proteomics and Structure".to_string(),
            description: None,
            order: 100,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "protein-engineering".to_string(),
        CategoryDef {
            name: "Protein Engineering".to_string(),
            description: None,
            order: 110,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "infrastructure".to_string(),
        CategoryDef {
            name: "Workflows and Infrastructure".to_string(),
            description: None,
            order: 120,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "visualization".to_string(),
        CategoryDef {
            name: "Visualization".to_string(),
            description: None,
            order: 130,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "rust-python".to_string(),
        CategoryDef {
            name: "Rust × Python".to_string(),
            description: Some(
                "Python-facing bioinformatics packages powered partially or primarily by Rust."
                    .to_string(),
            ),
            order: 140,
            children: IndexMap::new(),
        },
    );

    categories.insert(
        "learning".to_string(),
        CategoryDef {
            name: "Learning Resources".to_string(),
            description: None,
            order: 200,
            children: IndexMap::new(),
        },
    );

    categories
}

pub fn map_v1_category_to_v2(v1: &str) -> String {
    match v1 {
        "bacterial-annotation" => "genome-annotation".to_string(),
        "resistance-genes" => "antimicrobial-resistance".to_string(),
        "sequence-io-and-formats" => "sequence-io".to_string(),
        "alignment-and-mapping" => "alignment".to_string(),
        "variants-and-annotation" => "variants".to_string(),
        "assembly-and-pangenomes" => "assembly-pangenome".to_string(),
        "single-cell-and-rna" => "single-cell-rna".to_string(),
        "proteomics-and-structure" => "proteomics".to_string(),
        "workflows-and-infrastructure" => "infrastructure".to_string(),
        "learning-resources" => "learning".to_string(),
        other => other.to_string(),
    }
}

pub fn make_slug(name: &str) -> String {
    let mut slug = name
        .trim()
        .to_lowercase()
        .replace([' ', '_'], "-")
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }
    slug.trim_matches('-').to_string()
}

pub fn extract_doi(url: &str) -> Option<String> {
    if let Some(pos) = url.find("10.") {
        let candidate = &url[pos..];
        let doi = candidate.trim_end_matches('/').trim();
        Some(doi.to_string())
    } else {
        None
    }
}

pub fn migrate_v1_to_v2(v1_yaml: &str) -> Result<Catalog, String> {
    let old_tools: Vec<V1Tool> =
        serde_yaml::from_str(v1_yaml).map_err(|e| format!("Failed to parse v1 tools.yaml: {e}"))?;

    let mut tools = IndexMap::new();

    for t in old_tools {
        let repo = if let Some(ref r) = t.repo {
            r.clone()
        } else if let Some(stripped) = t.url.strip_prefix("https://github.com/") {
            stripped.trim_end_matches('/').to_string()
        } else if t.url == "https://rust-bio.github.io/" {
            "rust-bio/rust-bio.github.io".to_string()
        } else {
            return Err(format!("Cannot determine repository for tool: {}", t.name));
        };

        let mut key = make_slug(&t.name);
        if key.is_empty() {
            key = repo.replace('/', "-").to_lowercase();
        }
        if tools.contains_key(&key) {
            key = format!("{}-{}", key, repo.split('/').next().unwrap_or("tool"));
        }

        let primary_cat = map_v1_category_to_v2(&t.category);

        let is_retired = t.status.as_deref() == Some("retired");
        let catalog_status = if is_retired {
            CatalogStatus::Retired
        } else {
            CatalogStatus::Active
        };

        // Architecture inference: known projects or default unknown
        let (rust_role, interfaces, foreign_deps) = match t.name.as_str() {
            "noodles" => (
                RustRole::Native,
                vec!["cli".into(), "library".into()],
                vec![],
            ),
            "rust-bio" => (RustRole::Native, vec!["library".into()], vec![]),
            "skani" => (RustRole::Native, vec!["cli".into()], vec![]),
            "sylph" => (RustRole::Native, vec!["cli".into()], vec![]),
            "rust-htslib" => (
                RustRole::Binding,
                vec!["library".into()],
                vec!["HTSlib".into()],
            ),
            "sourmash" => (
                RustRole::Hybrid,
                vec!["cli".into(), "python".into(), "library".into()],
                vec![],
            ),
            "rasusa" => (RustRole::Native, vec!["cli".into()], vec![]),
            _ => {
                let desc_lower = t.description.to_lowercase();
                if desc_lower.contains("python binding")
                    || desc_lower.contains("python package")
                    || desc_lower.contains("pyo3")
                {
                    (
                        RustRole::Hybrid,
                        vec!["python".into(), "library".into()],
                        vec![],
                    )
                } else if desc_lower.contains("binding") || desc_lower.contains("wrapper") {
                    (RustRole::Binding, vec!["library".into()], vec![])
                } else {
                    (RustRole::Unknown, vec!["cli".into()], vec![])
                }
            }
        };

        // Learning curation for notable projects
        let learning = match t.name.as_str() {
            "noodles" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Advanced),
                topics: vec![
                    "bioinformatics IO".into(),
                    "binary file formats".into(),
                    "API design".into(),
                ],
            }),
            "rust-bio" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Intermediate),
                topics: vec!["algorithms".into(), "API design".into()],
            }),
            "skani" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Intermediate),
                topics: vec!["k-mer algorithms".into(), "MinHash".into(), "rayon".into()],
            }),
            "sylph" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Intermediate),
                topics: vec!["MinHash".into(), "metagenomics".into()],
            }),
            "sourmash" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Intermediate),
                topics: vec!["Python + Rust architecture".into(), "PyO3".into()],
            }),
            "rasusa" => Some(Learning {
                recommended: true,
                level: Some(LearningLevel::Beginner),
                topics: vec!["CLI design".into(), "FASTQ handling".into()],
            }),
            _ => None,
        };

        let mut publications = Vec::new();
        for p in t.papers {
            let doi = extract_doi(&p.url);
            let pub_type = if p.url.contains("biorxiv") || p.url.contains("arxiv") {
                Some("preprint".into())
            } else {
                Some("journal".into())
            };
            publications.push(Publication {
                doi,
                url: Some(p.url),
                title: p.title,
                pub_type,
            });
        }

        let custom_url = if t.url != format!("https://github.com/{repo}") {
            Some(t.url)
        } else {
            None
        };

        let tool_def = ToolDef {
            name: t.name,
            repository: repo,
            url: custom_url,
            description: t.description,
            category: ToolCategory {
                primary: primary_cat,
                secondary: Vec::new(),
            },
            domains: Vec::new(),
            architecture: Architecture {
                rust_role,
                interfaces,
                foreign_dependencies: foreign_deps,
            },
            technologies: Vec::new(),
            formats: Vec::new(),
            learning,
            publications,
            status: ToolStatus {
                catalog: catalog_status,
            },
            notes: None,
        };

        tools.insert(key, tool_def);
    }

    Ok(Catalog {
        schema_version: 2,
        categories: default_v2_categories(),
        tools,
    })
}

pub fn run_migration(v1_path: &Path, v2_path: &Path) -> Result<(), String> {
    let v1_text = std::fs::read_to_string(v1_path)
        .map_err(|e| format!("Failed to read {}: {}", v1_path.display(), e))?;
    let catalog = migrate_v1_to_v2(&v1_text)?;
    let v2_yaml = serde_yaml::to_string(&catalog)
        .map_err(|e| format!("Failed to serialize v2 catalog: {e}"))?;
    std::fs::write(v2_path, v2_yaml)
        .map_err(|e| format!("Failed to write {}: {}", v2_path.display(), e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::validate_catalog;

    #[test]
    fn test_migrate_actual_tools_yaml() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/tools.yaml");
        let text = std::fs::read_to_string(&path).expect("read data/tools.yaml");
        let catalog = if text.contains("schema_version: 2") {
            serde_yaml::from_str::<Catalog>(&text).expect("parse v2 catalog")
        } else {
            migrate_v1_to_v2(&text).expect("migrate_v1_to_v2")
        };

        assert_eq!(catalog.schema_version, 2);
        assert_eq!(catalog.tools.len(), 211, "All 211 tools must be preserved");

        let total_pubs: usize = catalog.tools.values().map(|t| t.publications.len()).sum();
        assert_eq!(total_pubs, 44, "All 44 publications must be preserved");

        let validation = validate_catalog(&catalog);
        assert!(
            validation.is_valid(),
            "Migrated catalog must be valid, but had errors: {:?}",
            validation.errors
        );

        let noodles = catalog
            .tools
            .values()
            .find(|t| t.name == "noodles")
            .expect("noodles found");
        assert_eq!(noodles.architecture.rust_role, RustRole::Native);
        assert!(noodles.learning.as_ref().unwrap().recommended);

        let htslib = catalog
            .tools
            .values()
            .find(|t| t.name == "rust-htslib")
            .expect("rust-htslib found");
        assert_eq!(htslib.architecture.rust_role, RustRole::Binding);

        let sourmash = catalog
            .tools
            .values()
            .find(|t| t.name == "sourmash")
            .expect("sourmash found");
        assert_eq!(sourmash.architecture.rust_role, RustRole::Hybrid);

        let retired_count = catalog.tools.values().filter(|t| t.is_retired()).count();
        assert!(retired_count > 0, "Retired tools must be preserved");
    }

    #[test]
    fn test_migrate_v1_snippet() {
        let v1_yaml = r#"
- name: test-tool
  url: https://github.com/example/test-tool
  repo: example/test-tool
  category: bacterial-annotation
  description: Gene prediction in bacterial genomes.
  papers:
    - title: "A paper on test-tool."
      url: https://doi.org/10.1093/bioinformatics/btv123
"#;
        let catalog = migrate_v1_to_v2(v1_yaml).expect("migrate snippet");
        assert_eq!(catalog.schema_version, 2);
        assert_eq!(catalog.tools.len(), 1);
        let tool = catalog.tools.get("test-tool").expect("found tool");
        assert_eq!(tool.category.primary, "genome-annotation");
        assert_eq!(tool.publications.len(), 1);
        assert_eq!(
            tool.publications[0].doi.as_deref(),
            Some("10.1093/bioinformatics/btv123")
        );
    }
}
