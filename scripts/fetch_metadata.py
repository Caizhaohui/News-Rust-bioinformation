"""Fetch GitHub metadata via GraphQL and write metadata.json plus a snapshot."""

from __future__ import annotations

import os
import sys
from pathlib import Path

import requests

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (  # noqa: E402
    METADATA_PATH,
    SNAPSHOT_DIR,
    dump_json,
    load_config,
    load_tools,
    prune_snapshots,
    utcnow,
    validate_tools,
)

GRAPHQL_URL = "https://api.github.com/graphql"
BATCH_SIZE = 20


def _alias(index: int) -> str:
    return f"r{index}"


def build_query(batch: list[tuple[str, str, str]]) -> str:
    fields = []
    for index, (_repo, owner, name) in enumerate(batch):
        alias = _alias(index)
        fields.append(
            f"""
            {alias}: repository(owner: "{owner}", name: "{name}") {{
              stargazerCount
              pushedAt
              isArchived
              latestRelease {{ publishedAt tagName }}
            }}
            """
        )
    return "query {\n" + "\n".join(fields) + "\n}"


def fetch_batch(session: requests.Session, batch: list[tuple[str, str, str]]) -> dict[str, dict]:
    response = session.post(GRAPHQL_URL, json={"query": build_query(batch)}, timeout=60)
    payload = response.json()
    if response.status_code != 200:
        raise RuntimeError(f"GraphQL HTTP {response.status_code}: {payload}")
    if payload.get("errors") and not payload.get("data"):
        raise RuntimeError(f"GraphQL errors: {payload['errors']}")
    data = payload.get("data") or {}
    errors_by_alias: dict[str, str] = {}
    for err in payload.get("errors") or []:
        path = err.get("path") or []
        if path:
            errors_by_alias[str(path[0])] = err.get("message") or "graphql error"
    results: dict[str, dict] = {}
    for index, (repo, _owner, _name) in enumerate(batch):
        alias = _alias(index)
        node = data.get(alias)
        if not node:
            results[repo] = {
                "stars": None,
                "pushed_at": None,
                "archived": None,
                "latest_release_at": None,
                "latest_release_tag": None,
                "error": errors_by_alias.get(alias, "not found"),
            }
            continue
        release = node.get("latestRelease") or {}
        results[repo] = {
            "stars": node.get("stargazerCount"),
            "pushed_at": node.get("pushedAt"),
            "archived": node.get("isArchived"),
            "latest_release_at": release.get("publishedAt"),
            "latest_release_tag": release.get("tagName"),
            "error": None,
        }
    return results


def main() -> int:
    tools = load_tools()
    errors = validate_tools(tools)
    if errors:
        print("tools.yaml is invalid:", file=sys.stderr)
        for item in errors:
            print(f"  - {item}", file=sys.stderr)
        return 1

    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    urls = [t["url"] for t in tools]
    repos = sorted({t["repo"] for t in tools if t.get("repo")})
    parsed: list[tuple[str, str, str]] = []
    for repo in repos:
        owner, name = repo.split("/", 1)
        parsed.append((repo, owner, name))

    records: dict[str, dict] = {}
    incomplete = False
    if not token:
        incomplete = True
        for repo, _owner, _name in parsed:
            records[repo] = {
                "stars": None,
                "pushed_at": None,
                "archived": None,
                "latest_release_at": None,
                "latest_release_tag": None,
                "error": "no_token",
            }
        print("No GITHUB_TOKEN; writing incomplete metadata.", file=sys.stderr)
    else:
        session = requests.Session()
        session.headers.update(
            {
                "Authorization": f"bearer {token}",
                "Accept": "application/vnd.github+json",
            }
        )
        try:
            for start in range(0, len(parsed), BATCH_SIZE):
                batch = parsed[start : start + BATCH_SIZE]
                records.update(fetch_batch(session, batch))
        except Exception as exc:  # noqa: BLE001
            incomplete = True
            print(f"Fetch failed: {exc}", file=sys.stderr)
            for repo, _owner, _name in parsed:
                records.setdefault(
                    repo,
                    {
                        "stars": None,
                        "pushed_at": None,
                        "archived": None,
                        "latest_release_at": None,
                        "latest_release_tag": None,
                        "error": str(exc),
                    },
                )

    metadata = {
        "fetched_at": utcnow().isoformat(),
        "incomplete": incomplete,
        "urls": urls,
        "repos": records,
    }
    dump_json(METADATA_PATH, metadata)

    today = utcnow().date().isoformat()
    snapshot = dict(metadata)
    snapshot["date"] = today
    dump_json(SNAPSHOT_DIR / f"{today}.json", snapshot)
    prune_snapshots(int(load_config()["snapshots"]["keep"]))
    print(f"Wrote {METADATA_PATH.relative_to(METADATA_PATH.parents[1])} for {len(records)} repos")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
