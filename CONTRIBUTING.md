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

2. `category` must be one of:

- `core-libraries`
- `sequence-io-and-formats`
- `alignment-and-mapping`
- `variants-and-annotation`
- `long-reads`
- `assembly-and-pangenomes`
- `metagenomics`
- `single-cell-and-rna`
- `proteomics-and-structure`
- `workflows-and-infrastructure`
- `visualization`
- `learning-resources`

3. `url` is required and must be unique. `repo` is `owner/name` for GitHub projects only.
4. Optional `status: retired` moves an entry out of the main catalog into the README Retired section. Do not delete archived tools; mark them retired so radar history stays intact.

5. Rebuild locally:

```bash
python scripts/validate.py
python scripts/fetch_metadata.py
python scripts/build_readme.py
python scripts/build_radar.py
```

## Cadence

- Weekly (automation): refresh GitHub metadata, rebuild `README.md` and `RADAR.md`, keep the last 8 snapshots. Does not write digest drafts.
- Every 1–2 weeks (you): generate a Chinese outline, curate 3–5 items, publish on the personal site.

Generate an outline only when you are about to write:

```bash
python scripts/build_digest_draft.py
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
