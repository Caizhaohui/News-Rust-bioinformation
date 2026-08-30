# Python helpers for the living catalog.

Install:

```bash
pip install -r scripts/requirements.txt
```

Commands (from repo root):

```bash
python scripts/validate.py
python scripts/fetch_metadata.py   # needs GITHUB_TOKEN for complete data
python scripts/build_readme.py
python scripts/build_radar.py
python scripts/build_digest_draft.py
python scripts/discover.py         # optional: --days 14 --sources github,biorxiv
python -m unittest discover -s scripts -p "test_*.py"
```

`fetch_metadata.py` writes `data/metadata.json` and `data/snapshots/YYYY-MM-DD.json`. Without a token it still succeeds and marks metadata incomplete.

`discover.py` writes `discover/candidates-YYYY-MM-DD.md` for human review. It does not edit `data/tools.yaml`. Sources are GitHub Search and bioRxiv only. GitHub needs `GITHUB_TOKEN` / `GH_TOKEN`; bioRxiv uses the public API. Missing tokens skip that source and mark the report incomplete.
