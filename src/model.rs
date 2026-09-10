use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Catalog {
    pub schema_version: u32,
    #[serde(default)]
    pub categories: IndexMap<String, CategoryDef>,
    #[serde(default)]
    pub tools: IndexMap<String, ToolDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryDef {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub order: u32,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub children: IndexMap<String, CategoryDef>,
}

impl CategoryDef {
    pub fn is_section(&self) -> bool {
        !self.children.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolDef {
    pub name: String,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub description: String,
    pub category: ToolCategory,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<String>,
    pub architecture: Architecture,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technologies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formats: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learning: Option<Learning>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publications: Vec<Publication>,
    pub status: ToolStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ToolDef {
    pub fn effective_url(&self) -> String {
        if let Some(ref custom_url) = self.url {
            custom_url.clone()
        } else if self.repository.starts_with("http://") || self.repository.starts_with("https://")
        {
            self.repository.clone()
        } else {
            format!("https://github.com/{}", self.repository)
        }
    }

    pub fn is_retired(&self) -> bool {
        self.status.catalog == CatalogStatus::Retired
    }

    pub fn is_active_in_catalog(&self) -> bool {
        self.status.catalog == CatalogStatus::Active
    }

    pub fn primary_category(&self) -> &str {
        &self.category.primary
    }

    pub fn is_python_facing(&self) -> bool {
        self.architecture
            .interfaces
            .iter()
            .any(|iface| iface.eq_ignore_ascii_case("python"))
    }

    pub fn to_v1_tool(&self) -> crate::catalog::Tool {
        crate::catalog::Tool {
            name: self.name.clone(),
            url: self.effective_url(),
            repo: Some(self.repository.clone()),
            category: self.category.primary.clone(),
            description: self.description.clone(),
            status: if self.is_retired() {
                Some("retired".into())
            } else {
                None
            },
            reason: None,
            papers: self
                .publications
                .iter()
                .map(|p| crate::catalog::Paper {
                    title: p.title.clone(),
                    url: p.url.clone().unwrap_or_else(|| {
                        if let Some(ref d) = p.doi {
                            format!("https://doi.org/{d}")
                        } else {
                            String::new()
                        }
                    }),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCategory {
    pub primary: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secondary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Architecture {
    pub rust_role: RustRole,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub foreign_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RustRole {
    Native,
    Hybrid,
    Binding,
    Experimental,
    Unknown,
}

impl fmt::Display for RustRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustRole::Native => write!(f, "native"),
            RustRole::Hybrid => write!(f, "hybrid"),
            RustRole::Binding => write!(f, "binding"),
            RustRole::Experimental => write!(f, "experimental"),
            RustRole::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Learning {
    #[serde(default)]
    pub recommended: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<LearningLevel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LearningLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Publication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub title: String,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub pub_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolStatus {
    pub catalog: CatalogStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CatalogStatus {
    Active,
    Retired,
    Watch,
    Excluded,
}

impl fmt::Display for CatalogStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CatalogStatus::Active => write!(f, "active"),
            CatalogStatus::Retired => write!(f, "retired"),
            CatalogStatus::Watch => write!(f, "watch"),
            CatalogStatus::Excluded => write!(f, "excluded"),
        }
    }
}

pub const ALLOWED_INTERFACES: &[&str] = &[
    "cli", "library", "python", "r", "web", "workflow", "api", "wasm",
];

pub const ALLOWED_DOMAINS: &[&str] = &[
    "genomics",
    "microbial-genomics",
    "metagenomics",
    "transcriptomics",
    "single-cell",
    "proteomics",
    "phylogenetics",
    "population-genetics",
    "structural-biology",
    "genome-editing",
    "crispr",
    "sequence-analysis",
    "machine-learning",
    "workflow",
    "visualization",
    "file-formats",
    "variant-calling",
    "pangenome",
    "long-reads",
    "epigenomics",
];

pub const ALLOWED_FORMATS: &[&str] = &[
    "FASTA", "FASTQ", "SAM", "BAM", "CRAM", "VCF", "BCF", "BED", "GFF", "GTF", "PAF", "BigWig",
    "BigBed", "Newick", "PDB", "mmCIF", "HDF5", "AnnData", "Zarr",
];
