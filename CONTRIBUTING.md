# Contributing to Rust Bioinformatics Radar

`News-Rust-bioinformation` is a curated and continuously updated catalog and ecosystem radar of Rust software in computational biology.

All project data is curated in `data/tools.yaml`. **Do not edit `README.md` or `RADAR.md` manually.**

---

## Inclusion Guidelines

To be included in the radar, a project should meet these criteria:

- **Substantial Rust Core**: Written primarily or significantly in Rust (pure Rust CLI/library, or a Python package powered by a Rust core / PyO3).
- **Domain Relevance**: Directly useful for computational biology, genomics, transcriptomics, metagenomics, proteomics, or laboratory bioinformatics.
- **Public Availability**: Publicly accessible open-source repository (GitHub, etc.).
- **Specific Purpose**: Not a generic CSV viewer, system monitor, or unrelated utility.

---

## How to Add or Update a Tool

Edit `data/tools.yaml` directly or submit an issue using the [Tool Submission Template](.github/ISSUE_TEMPLATE/add-tool.yml).

### Entry Format (`tools.yaml` v2)

```yaml
tools:

  example-tool:
    name: example-tool
    repository: owner/repository
    description: High-throughput sequence analysis and interval indexing in Rust.
    category:
      primary: sequence-io
      secondary: []
    domains:
      - genomics
      - file-formats
    architecture:
      rust_role: native # native | hybrid | binding | experimental | unknown
      interfaces:
        - cli
        - library
      foreign_dependencies: []
    technologies:
      - rust
    formats:
      - BAM
      - VCF
    learning:
      recommended: false
    publications:
      - doi: 10.1093/bioinformatics/example
        title: "Example publication title."
        type: journal # journal | preprint | conference | software-paper
    status:
      catalog: active # active | retired | watch | excluded
```

### Pull Request Checklist

Before submitting your PR, ensure:

- [ ] Repository is substantially written in Rust or has an essential Rust core.
- [ ] Project is relevant to bioinformatics and computational biology.
- [ ] Description is concise, factual, and written in English.
- [ ] Primary category is selected from valid categories.
- [ ] Architecture `rust_role` is specified (`native`, `hybrid`, `binding`, `experimental`, or `unknown`).
- [ ] Publication DOI is added if a paper or preprint exists.
- [ ] `cargo run -- validate` passes with zero errors.
- [ ] `cargo run -- build-readme` updates `README.md` deterministically.

---

## Local Verification Commands

From the repository root:

```bash
# Validate data/tools.yaml against schema v2
cargo run -- validate

# Check schema and verify README is up-to-date
cargo run -- check

# Rebuild README.md
cargo run -- build-readme

# Rebuild weekly RADAR.md
cargo run -- build-radar

# Run all unit tests
cargo test
```
