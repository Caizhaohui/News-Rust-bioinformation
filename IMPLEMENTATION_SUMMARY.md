# News-Rust-bioinformation v2 Implementation Summary

This document summarizes the full implementation and architectural upgrade of `News-Rust-bioinformation` from a living catalog into **Rust Bioinformatics Radar v2**, following the engineering specifications in `News-Rust-bioinformation v2 架构设计与实施规格.md`.

---

## 1. Implemented Features

### 1.1 Curated Source of Truth vs. Generated Cache Separation
- **`data/tools.yaml`** is now strictly the curated human-edited source of truth. All dynamic numbers (stars, forks, open issues, push timestamps, release tags) are stored only in `data/metadata.json`, `data/snapshots/`, and `data/trends.json`.
- Introduced strongly typed Serde models in `src/model.rs` for:
  - `Catalog` (`schema_version: 2`, categories map, tools map)
  - `CategoryDef` (hierarchical categories with orders, descriptions, and children)
  - `ToolDef` (name, repository, url, description, category, domains, architecture, technologies, formats, learning, publications, status, notes)
  - `Architecture` (`rust_role`: `native`, `hybrid`, `binding`, `experimental`, `unknown`; `interfaces`; `foreign_dependencies`)
  - `Publication` (`doi`, `url`, `title`, `type`)
  - `Learning` (`recommended`, `level`: `beginner`, `intermediate`, `advanced`, `topics`)
  - `ToolStatus` (`catalog`: `active`, `retired`, `watch`, `excluded`)

### 1.2 Formal Schemas & Validation
- Created standard JSON Schemas:
  - `schema/tools.schema.json`
  - `schema/config.schema.json`
- Implemented `src/validate.rs` checking:
  - `schema_version == 2`
  - Unique tool keys and repository names (case-insensitive deduplication)
  - Category and subcategory path validity
  - Valid architecture roles and interface enums
  - Valid standard domains and formats
  - Valid DOI prefixes (`10.`)

### 1.3 Automatic Activity Classification & Badges
- Dynamic activity computation based on days elapsed since last push (configured in `data/config.yaml`):
  - `🟢 Active`: pushed within 90 days
  - `🟡 Maintained`: pushed within 91–365 days
  - `🟠 Quiet`: pushed within 366–730 days
  - `🔴 Inactive`: no push for >730 days
  - `⚫ Archived`: repository marked archived on GitHub
- Architecture badge formatting:
  - `🦀 Native` for native Rust core
  - `🐍 Rust × Python` for Python-facing hybrid tools
  - `🔀 Hybrid` for other multi-language systems
  - `🔗 Binding` for foreign library wrappers
  - `🧪 Experimental` for experimental prototypes

### 1.4 Trends & Ecosystem Snapshots
- Implemented `src/trends.rs` computing:
  - Compact snapshots (`data/snapshots/YYYY-MM-DD.json`) storing `{ stars, forks }`
  - 7d, 30d, and 90d star growth deltas and percentage rates (`data/trends.json`)
  - Transparent rule-based criteria for 🔥 Trending and 🌱 Emerging projects

### 1.5 Deterministic README v2 Redesign
- Redesigned `src/readme.rs` layout:
  1. Header with automatically generated statistics bar (Total, Active, Pure Rust, Rust × Python, Last Updated date)
  2. Table of Contents
  3. **Ecosystem at a Glance** table
  4. **🔥 Trending** projects section (top 30d star gainers)
  5. **🌱 Emerging Projects** section (<150 stars, recent active push)
  6. **📖 Recommended Source Code** table (curated educational picks)
  7. **🧬 Bioinformatics Applications** (CRISPR first, followed by Microbial subsections and classic bioinformatics categories)
  8. **🦀 Core Rust Libraries**
  9. **🐍 Rust × Python** dedicated section
  10. **⚙️ Infrastructure & Workflows**
  11. **📚 Learning Resources**
  12. **🗄️ Retired** projects list
  13. **📊 Methodology & Activity** explanation
  14. **🤝 Contributing & Related Resources**
- Rebuilding is 100% deterministic with date-only timestamps.

---

## 2. Schema Migration

- Implemented `src/migration.rs` and CLI command `nrb migrate-v2`.
- **Zero Data Loss Guarantee**:
  - Pre-migration: 211 tools, 44 publications.
  - Post-migration: **211 tools**, **44 publications** (100% preserved).
  - 202 active tools, 9 retired tools preserved.
  - Known architectures (e.g. `noodles`, `rust-bio`, `skani`, `sylph`, `sourmash`, `rust-htslib`, `rasusa`) accurately annotated; ambiguous entries safely defaulted to `rust_role: unknown` without guessing.

---

## 3. CLI Commands (`nrb`)

| Command | Description |
|---|---|
| `nrb validate` | Validates `data/tools.yaml` against v2 schema and integrity rules |
| `nrb check` | Validates schema and verifies `README.md` is in sync with `tools.yaml` |
| `nrb fetch-metadata` | Fetches GitHub GraphQL metadata, saves compact snapshot, and computes trends |
| `nrb snapshot` | Generates a compact snapshot of repository metadata for today |
| `nrb trends` | Recomputes 7d/30d/90d trends from historical snapshots |
| `nrb build-readme` | Deterministically compiles `README.md` from `tools.yaml`, `metadata.json`, and `trends.json` |
| `nrb build-radar` | Generates `RADAR.md` weekly ecosystem delta |
| `nrb digest` | Generates an editorial draft in `digest/` |
| `nrb discover` | Searches GitHub and bioRxiv for candidate tools without touching catalog |
| `nrb migrate-v2` | One-shot migration of v1 tools sequence to v2 schema |

---

## 4. Workflow Changes

1. `.github/workflows/validate.yml`:
   - Runs on all PRs and pushes to `main`.
   - Executes `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo run -- check`.
2. `.github/workflows/update-metadata.yml`:
   - Runs daily at 03:17 UTC via cron and `workflow_dispatch`.
   - Fetches GitHub metadata, rebuilds README and RADAR, commits only if changes occur.
3. `.github/workflows/weekly-radar.yml`:
   - Runs weekly on Mondays at 04:00 UTC.
   - Refreshes metadata, generates compact snapshot, computes trends, and updates radar and README.
4. `.github/ISSUE_TEMPLATE/add-tool.yml`:
   - Standardized GitHub Issue Form for tool submissions adhering to the v2 schema.

---

## 5. Verification & Tests

- **Unit Tests**: 36 tests passing in `cargo test` (including v2 catalog validation, v1 migration preservation, URL normalization, radar deltas, and candidate discovery filters).
- **Format**: `cargo fmt --check` passes cleanly.
- **Clippy**: `cargo clippy --all-targets --all-features -- -D warnings` passes with 0 warnings.
- **Validation**: `cargo run -- validate` passes: `Validation passed: 211 tools total (202 active, 9 retired), 15 categories.`
- **Check**: `cargo run -- check` passes: `All checks passed successfully.`

---

## 6. Known Limitations & Future Work

- **Architecture Refinements**: For long-tail tools migrated with `rust_role: unknown`, maintainers can progressively curate whether they are pure native or bindings as they are reviewed.
- **Publication Metadata Enrichment (v2.2)**: Automated Crossref/PubMed DOI title and citation metadata lookup can be added in future iterations.
- **Static Website Frontend (v3)**: `data/tools.yaml`, `data/metadata.json`, and `data/trends.json` are now clean, machine-readable static datasets ready to power a future static website without server-side databases.
