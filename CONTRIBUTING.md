# Contributing

This repository (`News-Rust-bioinformation`) is a living catalog of Rust software used in computational biology. Edit `data/tools.yaml`. Do not hand-edit `README.md` or `RADAR.md`.

## Inclusion

Add a project when all of the following hold:

- It is written primarily in Rust (bindings should say so in the description).
- It is directly useful for computational biology, genomics, proteomics, or related lab informatics.
- It is publicly reachable (source, docs, or crates.io page).
- It is not a generic CSV, spreadsheet, or cluster-dashboard tool with no biology-specific purpose.

Prefer a short, objective English description: capital letter, ends with a period, does not repeat the project name.

## Adding a tool

1. Append an entry to `data/tools.yaml`:

```yaml
- name: example
  url: https://github.com/org/example
  repo: org/example
  category: core-libraries
  description: One-sentence description.
```

2. `category` must be one of the leaf slugs below (not a section heading):

- `crispr`
- Microbial Bioinformatics (README section only; do not use as `category`):
  - `bacterial-assembly`
  - `bacterial-annotation`
  - `prokaryotic-transcriptome`
  - `metagenomics`
  - `phage-defense`
  - `resistance-genes`
  - `transposons`
- `core-libraries`
- `sequence-io-and-formats`
- `alignment-and-mapping`
- `variants-and-annotation`
- `long-reads`
- `assembly-and-pangenomes`
- `single-cell-and-rna`
- `proteomics-and-structure`
- `protein-engineering`
- `workflows-and-infrastructure`
- `visualization`
- `learning-resources`

3. `url` is required and must be unique. `repo` is `owner/name` for GitHub projects only.
4. Optional `status: retired` moves an entry out of the main catalog into the README Retired section. Do not delete archived tools; mark them retired so radar history stays intact.
5. Optional `papers` is a list of `{title, url}` introducing articles (DOI or preprint). Omit the field when there is no paper; do not use an empty list.

6. Rebuild locally:

```bash
cargo run -- validate
cargo run -- fetch-metadata   # needs GITHUB_TOKEN for complete data
cargo run -- build-readme
cargo run -- build-radar
```

Or install the `nrb` binary with `cargo install --path .` and run those subcommands directly.

## Finding new tools

Already-listed tools are refreshed by the weekly workflow (stars / push / archived / radar). New tools are **not** auto-added and `tools.yaml` is never written by discovery.

1. Manually run `cargo run -- discover` or the `Discover new tool candidates` GitHub Action (`workflow_dispatch` only).
2. Open `discover/candidates-YYYY-MM-DD.md` and fill `收录？是/否`.
3. For “是” items, append them to `data/tools.yaml` using the schema above (Rust-first, computational biology, public, no generic CSV / Slurm TUI).
4. Rebuild with `nrb validate` / `nrb fetch-metadata` / `nrb build-readme` / `nrb build-radar`.

Sources: GitHub Search and bioRxiv. X search is not enabled. Missing GitHub token skips that source and marks the report incomplete. crates.io is not scanned; a crates.io link mentioned by another source may still appear.

## Cadence

- Weekly (automation): refresh GitHub metadata, rebuild `README.md` and `RADAR.md`, keep the last 8 snapshots. Does not write digest drafts or discover new tools.
- New tools (manual): run discover → review candidates → edit `data/tools.yaml`.
- Every 1–2 weeks (you): generate a Chinese outline, curate 3–5 items, publish on the personal site.

Generate an outline only when you are about to write:

```bash
cargo run -- digest
```

Or run the `Generate digest draft` GitHub Action (`workflow_dispatch`).

## Writing checklist (personal site)

1. Open `digest/rust-bio-digest-YYYY-MM-DD.md`.
2. Keep 3–5 items you can actually comment on. Delete the rest.
3. Fill the empty “为什么值得写” notes. Do not publish star-delta lists as a post.
4. Copy the file to `Caizhaohui/personal-website` `src/content/blog/`.
5. Set `pubDate` to the real publish date and `draft: false`.
6. Confirm `pnpm build` succeeds.
7. Update `data/tools.yaml` if the post adds or retires a tool. Set `status: retired` on archived projects instead of deleting them.
