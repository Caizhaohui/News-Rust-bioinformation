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
python -m unittest scripts.test_radar
```

`fetch_metadata.py` writes `data/metadata.json` and `data/snapshots/YYYY-MM-DD.json`. Without a token it still succeeds and marks metadata incomplete.
