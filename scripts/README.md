# Rust CLI for the living catalog.

From the repo root:

```bash
cargo run -- validate
cargo run -- fetch-metadata   # needs GITHUB_TOKEN for complete data; prints diff summary
cargo run -- diff             # show changes (stars, releases, catalog) between snapshots
cargo run -- build-readme
cargo run -- build-radar
cargo run -- digest
cargo run -- discover         # optional: --days 14 --sources github,biorxiv
cargo test
```

Install the `nrb` binary:

```bash
cargo install --path .
nrb validate
```

`fetch-metadata` writes `data/metadata.json` and `data/snapshots/YYYY-MM-DD.json`. Without a token it still succeeds and marks metadata incomplete. It automatically computes and displays the diff against the previous snapshot, saving a summary to `data/diff.md`.

`diff` compares snapshots or metadata to reveal what changed: GitHub stars, new software releases/tags, cold repository revival, and catalog adjustments:
- `cargo run -- diff` (compares current metadata against previous snapshot)
- `cargo run -- diff --from 2026-09-07 --to 2026-09-18` (compares two snapshots)
- `cargo run -- diff --format markdown` (outputs GitHub markdown table)
- `cargo run -- diff --format summary` (outputs concise single-line summary)
- `cargo run -- diff --min-stars 5` (filters star changes >= 5)

`discover` writes `discover/candidates-YYYY-MM-DD.md` for human review. It does not edit `data/tools.yaml`. Sources are GitHub Search and bioRxiv only. GitHub needs `GITHUB_TOKEN` / `GH_TOKEN`; bioRxiv uses the public API. Missing tokens skip that source and mark the report incomplete.
