# Rust CLI for the living catalog.

From the repo root:

```bash
cargo run -- validate
cargo run -- fetch-metadata   # needs GITHUB_TOKEN for complete data
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

`fetch-metadata` writes `data/metadata.json` and `data/snapshots/YYYY-MM-DD.json`. Without a token it still succeeds and marks metadata incomplete.

`discover` writes `discover/candidates-YYYY-MM-DD.md` for human review. It does not edit `data/tools.yaml`. Sources are GitHub Search and bioRxiv only. GitHub needs `GITHUB_TOKEN` / `GH_TOKEN`; bioRxiv uses the public API. Missing tokens skip that source and mark the report incomplete.
