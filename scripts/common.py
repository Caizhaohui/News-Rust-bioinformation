"""Shared paths, schema, radar classification, and I/O for catalog scripts."""

from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any
from urllib.parse import urlparse, urlunparse

import yaml

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "data"
TOOLS_PATH = DATA / "tools.yaml"
CONFIG_PATH = DATA / "config.yaml"
METADATA_PATH = DATA / "metadata.json"
SNAPSHOT_DIR = DATA / "snapshots"
DISCOVER_DIR = ROOT / "discover"

GITHUB_RESERVED_OWNERS = {
    "about",
    "account",
    "apps",
    "codespaces",
    "collections",
    "enterprise",
    "events",
    "explore",
    "features",
    "login",
    "marketplace",
    "new",
    "notifications",
    "organizations",
    "orgs",
    "search",
    "settings",
    "sponsors",
    "topics",
}

CATEGORIES: list[tuple[str, str]] = [
    ("core-libraries", "Core Libraries"),
    ("sequence-io-and-formats", "Sequence IO and Formats"),
    ("alignment-and-mapping", "Alignment and Mapping"),
    ("variants-and-annotation", "Variants and Annotation"),
    ("crispr", "CRISPR"),
    ("long-reads", "Long Reads"),
    ("assembly-and-pangenomes", "Assembly and Pangenomes"),
    ("metagenomics", "Metagenomics"),
    ("single-cell-and-rna", "Single-cell and RNA"),
    ("proteomics-and-structure", "Proteomics and Structure"),
    ("protein-engineering", "Protein Engineering"),
    ("workflows-and-infrastructure", "Workflows and Infrastructure"),
    ("visualization", "Visualization"),
    ("learning-resources", "Learning Resources and Related Lists"),
]

CATEGORY_TITLES = dict(CATEGORIES)
VALID_CATEGORIES = set(CATEGORY_TITLES)

REQUIRED_FIELDS = ("name", "url", "category", "description")
ALLOWED_FIELDS = {"name", "url", "repo", "category", "description", "status"}
VALID_STATUSES = {"retired"}


def is_retired(tool: dict[str, Any]) -> bool:
    return tool.get("status") == "retired"


def active_tools(tools: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [tool for tool in tools if not is_retired(tool)]


def utcnow() -> datetime:
    return datetime.now(timezone.utc)


def parse_dt(value: str | None) -> datetime | None:
    if not value:
        return None
    text = value.replace("Z", "+00:00")
    try:
        dt = datetime.fromisoformat(text)
    except ValueError:
        return None
    if dt.tzinfo is None:
        dt = dt.replace(tzinfo=timezone.utc)
    return dt.astimezone(timezone.utc)


def months_ago(now: datetime, months: int) -> datetime:
    year = now.year
    month = now.month - months
    while month <= 0:
        month += 12
        year -= 1
    day = min(now.day, 28)
    return now.replace(year=year, month=month, day=day)


def load_yaml(path: Path) -> Any:
    with path.open(encoding="utf-8") as handle:
        return yaml.safe_load(handle) or {}


def load_config() -> dict[str, Any]:
    config = load_yaml(CONFIG_PATH)
    radar = config.setdefault("radar", {})
    radar.setdefault("stale_months", 18)
    radar.setdefault("cold_inactive_months", 6)
    active = radar.setdefault("active", {})
    active.setdefault("min_star_delta", 5)
    active.setdefault("include_new_release", True)
    active.setdefault("include_cold_repo_push", True)
    config.setdefault("snapshots", {}).setdefault("keep", 8)
    return config


def load_tools() -> list[dict[str, Any]]:
    raw = load_yaml(TOOLS_PATH)
    if not isinstance(raw, list):
        raise ValueError("data/tools.yaml must be a list")
    return raw


def normalize_url(url: str) -> str:
    raw = (url or "").strip()
    if not raw:
        return ""
    parsed = urlparse(raw)
    scheme = (parsed.scheme or "https").lower()
    netloc = parsed.netloc.lower()
    if netloc.startswith("www."):
        netloc = netloc[4:]
    path = parsed.path.rstrip("/")
    if path.endswith(".git"):
        path = path[:-4]
    return urlunparse((scheme, netloc, path, "", "", ""))


def github_repo_from_url(url: str) -> str | None:
    parsed = urlparse(normalize_url(url))
    if parsed.netloc not in {"github.com"}:
        return None
    parts = [part for part in parsed.path.split("/") if part]
    if len(parts) < 2:
        return None
    owner, name = parts[0], parts[1]
    if owner.lower() in GITHUB_RESERVED_OWNERS:
        return None
    name = name.removesuffix(".git")
    if not owner or not name:
        return None
    return f"{owner}/{name}"


@dataclass(frozen=True)
class CatalogIndex:
    urls: frozenset[str]
    repos: frozenset[str]


def catalog_index(tools: list[dict[str, Any]]) -> CatalogIndex:
    urls: set[str] = set()
    repos: set[str] = set()
    for tool in tools:
        url = tool.get("url")
        if isinstance(url, str) and url.strip():
            urls.add(normalize_url(url))
            inferred = github_repo_from_url(url)
            if inferred:
                repos.add(inferred.lower())
        repo = tool.get("repo")
        if isinstance(repo, str) and repo.strip():
            repos.add(repo.strip().lower())
    return CatalogIndex(urls=frozenset(urls), repos=frozenset(repos))


def is_cataloged(
    index: CatalogIndex,
    url: str | None = None,
    repo: str | None = None,
) -> bool:
    if isinstance(repo, str) and repo.strip():
        if repo.strip().lower() in index.repos:
            return True
    if isinstance(url, str) and url.strip():
        if normalize_url(url) in index.urls:
            return True
        inferred = github_repo_from_url(url)
        if inferred and inferred.lower() in index.repos:
            return True
    return False


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def dump_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2, ensure_ascii=False)
        handle.write("\n")


def empty_metadata(incomplete: bool = True) -> dict[str, Any]:
    return {
        "fetched_at": utcnow().isoformat(),
        "incomplete": incomplete,
        "urls": [],
        "repos": {},
    }


def load_metadata() -> dict[str, Any]:
    return load_json(METADATA_PATH) or empty_metadata()


def snapshot_dates() -> list[str]:
    if not SNAPSHOT_DIR.exists():
        return []
    dates = [p.stem for p in SNAPSHOT_DIR.glob("*.json") if p.stem[0].isdigit()]
    return sorted(dates)


def load_snapshot(date: str) -> dict[str, Any] | None:
    return load_json(SNAPSHOT_DIR / f"{date}.json")


def previous_snapshot(today: str | None = None) -> dict[str, Any] | None:
    dates = snapshot_dates()
    if not dates:
        return None
    if today is None:
        today = utcnow().date().isoformat()
    if dates[-1] == today:
        dates = dates[:-1]
    if not dates:
        return None
    return load_snapshot(dates[-1])


def prune_snapshots(keep: int) -> None:
    dates = snapshot_dates()
    extra = dates[: max(0, len(dates) - keep)]
    for date in extra:
        (SNAPSHOT_DIR / f"{date}.json").unlink(missing_ok=True)


def repo_record(metadata: dict[str, Any], repo: str | None) -> dict[str, Any]:
    if not repo:
        return {}
    return (metadata.get("repos") or {}).get(repo) or {}


def tool_sort_key(tool: dict[str, Any], metadata: dict[str, Any]) -> tuple:
    record = repo_record(metadata, tool.get("repo"))
    pushed = parse_dt(record.get("pushed_at"))
    stars = record.get("stars")
    pushed_ord = pushed.timestamp() if pushed else 0.0
    star_ord = stars if isinstance(stars, int) else -1
    return (-pushed_ord, -star_ord, tool["name"].lower())


def format_meta_suffix(tool: dict[str, Any], metadata: dict[str, Any]) -> str:
    repo = tool.get("repo")
    if not repo:
        return ""
    record = repo_record(metadata, repo)
    if record.get("error") and not record.get("stars") and not record.get("pushed_at"):
        return f" (metadata unavailable: {record.get('error')})"
    parts: list[str] = []
    stars = record.get("stars")
    if isinstance(stars, int):
        parts.append(f"★ {stars}")
    pushed = parse_dt(record.get("pushed_at"))
    if pushed:
        parts.append(f"pushed {pushed.date().isoformat()}")
    if record.get("archived"):
        parts.append("archived")
    if not parts:
        return ""
    return f" ({', '.join(parts)})"


def validate_tools(tools: list[dict[str, Any]]) -> list[str]:
    errors: list[str] = []
    urls: set[str] = set()
    for index, tool in enumerate(tools, start=1):
        prefix = f"item {index}"
        if not isinstance(tool, dict):
            errors.append(f"{prefix}: expected a mapping")
            continue
        for field in REQUIRED_FIELDS:
            value = tool.get(field)
            if not isinstance(value, str) or not value.strip():
                errors.append(f"{prefix}: missing {field}")
        name = tool.get("name")
        url = tool.get("url")
        category = tool.get("category")
        description = tool.get("description", "")
        if isinstance(url, str):
            if url in urls:
                errors.append(f"{prefix}: duplicate url {url}")
            urls.add(url)
        if isinstance(category, str) and category not in VALID_CATEGORIES:
            errors.append(f"{prefix}: unknown category {category}")
        if isinstance(description, str) and description:
            if not description[0].isupper() and not description[0].isdigit():
                errors.append(f"{prefix} ({name}): description should start with a capital")
            if not description.endswith("."):
                errors.append(f"{prefix} ({name}): description should end with a period")
        repo = tool.get("repo")
        if repo is not None:
            if not isinstance(repo, str) or repo.count("/") != 1 or not all(repo.split("/")):
                errors.append(f"{prefix} ({name}): repo must be owner/name")
        status = tool.get("status")
        if status is not None and status not in VALID_STATUSES:
            errors.append(f"{prefix} ({name}): status must be one of {sorted(VALID_STATUSES)}")
        extra = set(tool) - ALLOWED_FIELDS
        if extra:
            errors.append(f"{prefix} ({name}): unknown fields {sorted(extra)}")
    return errors


def compute_radar(
    tools: list[dict[str, Any]],
    current: dict[str, Any],
    previous: dict[str, Any] | None,
    config: dict[str, Any],
    now: datetime | None = None,
) -> dict[str, list[dict[str, Any]]]:
    now = now or utcnow()
    radar_cfg = config["radar"]
    active_cfg = radar_cfg["active"]
    stale_before = months_ago(now, int(radar_cfg["stale_months"]))
    cold_before = months_ago(now, int(radar_cfg["cold_inactive_months"]))

    tracked = active_tools(tools)
    current_urls = {t["url"] for t in tracked}
    previous_urls = set((previous or {}).get("urls") or [])
    prev_repos = (previous or {}).get("repos") or {}

    by_url = {t["url"]: t for t in tracked}
    new_entries = [by_url[url] for url in sorted(current_urls - previous_urls) if url in by_url]

    active: list[dict[str, Any]] = []
    stale: list[dict[str, Any]] = []
    watch: list[dict[str, Any]] = []

    if previous is None:
        for tool in tracked:
            record = repo_record(current, tool.get("repo"))
            error = record.get("error")
            if error:
                watch.append({**tool, "reason": str(error)})
            pushed = parse_dt(record.get("pushed_at"))
            if record.get("archived") or (pushed and pushed < stale_before):
                reason = "archived" if record.get("archived") else "no push in 18 months"
                stale.append({**tool, "reason": reason})
        return {
            "new_entries": [],
            "active": [],
            "stale": stale,
            "watch": watch,
            "baseline": False,
        }

    for tool in tracked:
        repo = tool.get("repo")
        record = repo_record(current, repo)
        prev = prev_repos.get(repo or "") or {}
        error = record.get("error")
        if error:
            watch.append({**tool, "reason": str(error)})
            continue

        pushed = parse_dt(record.get("pushed_at"))
        if record.get("archived") or (pushed and pushed < stale_before):
            reason = "archived" if record.get("archived") else "no push in 18 months"
            stale.append({**tool, "reason": reason})

        if tool["url"] not in previous_urls:
            continue

        reasons: list[str] = []
        stars = record.get("stars")
        prev_stars = prev.get("stars")
        if (
            isinstance(stars, int)
            and isinstance(prev_stars, int)
            and stars - prev_stars >= int(active_cfg["min_star_delta"])
        ):
            reasons.append(f"stars {prev_stars} -> {stars}")

        if active_cfg.get("include_new_release"):
            rel = parse_dt(record.get("latest_release_at"))
            prev_rel = parse_dt(prev.get("latest_release_at"))
            if rel and (prev_rel is None or rel > prev_rel):
                tag = record.get("latest_release_tag") or "release"
                reasons.append(f"new release {tag}")

        if active_cfg.get("include_cold_repo_push"):
            prev_pushed = parse_dt(prev.get("pushed_at"))
            if (
                pushed
                and prev_pushed
                and prev_pushed < cold_before
                and pushed > prev_pushed
            ):
                reasons.append("cold repo pushed")

        if reasons:
            active.append({**tool, "reason": "; ".join(reasons)})

    for tool in tools:
        if not is_retired(tool):
            continue
        record = repo_record(current, tool.get("repo"))
        error = record.get("error")
        if error:
            watch.append({**tool, "reason": str(error)})

    return {
        "new_entries": new_entries,
        "active": active,
        "stale": stale,
        "watch": watch,
        "baseline": True,
    }
