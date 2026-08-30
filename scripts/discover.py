"""Manually discover candidate Rust bioinformatics tools. Does not edit tools.yaml."""

from __future__ import annotations

import argparse
import os
import re
import sys
import time
from dataclasses import dataclass, replace
from datetime import datetime, timedelta
from pathlib import Path
from typing import Any

import requests

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (  # noqa: E402
    DISCOVER_DIR,
    ROOT,
    CatalogIndex,
    catalog_index,
    github_repo_from_url,
    is_cataloged,
    load_tools,
    normalize_url,
    utcnow,
    validate_tools,
)

VALID_SOURCES = ("github", "biorxiv")
GITHUB_SEARCH_URL = "https://api.github.com/search/repositories"
BIORXIV_DETAILS_URL = "https://api.biorxiv.org/details/biorxiv"
USER_AGENT = "News-Rust-bioinformation-discover"

GITHUB_TOPICS = (
    "bioinformatics",
    "genomics",
    "proteomics",
    "metagenomics",
    "sequencing",
    "pangenome",
    "pangenomes",
    "long-read",
    "long-reads",
)
GITHUB_KEYWORDS = (
    "bioinformatics",
    "genomics",
    "proteomics",
    "metagenomics",
    "pangenome",
    '"long-read"',
    "sequencing",
)

BIO_TERMS = (
    "bioinformatics",
    "computational biology",
    "genomics",
    "genome",
    "proteomics",
    "proteome",
    "metagenomics",
    "metagenome",
    "sequencing",
    "sequence analysis",
    "pangenome",
    "long-read",
    "long read",
    "nanopore",
    "pacbio",
    "fasta",
    "fastq",
    "vcf",
    "htslib",
    "transcriptom",
    "single-cell",
    "single cell",
    "rna-seq",
    "rnaseq",
    "scrna",
    "phylogen",
    "crispr",
    "peptide",
    "protein structure",
    "protein design",
    "protein engineering",
    "directed evolution",
    "protein language model",
    "enzyme engineering",
    "mass spectrom",
    "microbiom",
    "variant call",
    "read align",
    "sequence align",
    "genome align",
    "aligner",
    "genome assembl",
    "annotation",
    "ngs",
)

ALWAYS_EXCLUDE = (
    r"slurm.*\b(tui|dashboard)\b",
    r"\b(tui|dashboard)\b.*slurm",
    r"cluster dashboard",
    r"hpc dashboard",
)
EXCLUDE_IF_NO_BIO = (
    r"\bcsv parser\b",
    r"\bcsv library\b",
    r"spreadsheet",
    r"\bxlsx\b",
    r"\bcsv\b",
)

RUST_SIGNALS = (
    r"\bwritten in rust\b",
    r"\bimplemented in rust\b",
    r"\busing rust\b",
    r"\brust-based\b",
    r"\brust language\b",
    r"\brust programming\b",
    r"\ba rust (tool|library|crate|package|implementation|caller|aligner)\b",
    r"\brust crate\b",
    r"\bcargo\b",
    r"\bcrates\.io\b",
)

CATEGORY_HINTS: list[tuple[str, tuple[str, ...]]] = [
    ("crispr", ("crispr", "cas9", "cas12", "cas13", "sgrna", "guide rna", "genome edit")),
    ("long-reads", ("long-read", "long read", "nanopore", "pacbio", "oxford nanopore")),
    ("assembly-and-pangenomes", ("pangenome", "genome assembl", "de bruijn", "gfa")),
    ("metagenomics", ("metagenom", "microbiom")),
    ("single-cell-and-rna", ("single-cell", "single cell", "scrna", "rna-seq", "rnaseq", "transcriptom")),
    ("protein-engineering", (
        "protein design",
        "directed evolution",
        "protein engineering",
        "protein language model",
        "enzyme engineering",
        "ligandmpnn",
        "proteinmpnn",
        "inverse folding",
        "side-chain packing",
        "sidechain packing",
    )),
    ("proteomics-and-structure", ("proteom", "peptide", "alphafold", "protein structure", "mass spectrom")),
    ("variants-and-annotation", ("variant call", "vcf", "annotation")),
    ("alignment-and-mapping", ("aligner", "read align", "sequence align", "genome align", "minimap")),
    ("workflows-and-infrastructure", ("workflow", "nextflow", "snakemake")),
    ("visualization", ("visualiz", "genome browser")),
    ("sequence-io-and-formats", ("fasta", "fastq", "bam parser", "cram")),
    ("learning-resources", ("awesome list", "tutorial", "learning resource")),
]

GITHUB_URL_RE = re.compile(
    r"https?://(?:www\.)?github\.com/([A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?)/([A-Za-z0-9._-]+)",
    re.I,
)
CRATES_IO_RE = re.compile(r"https?://crates\.io/crates/([A-Za-z0-9_-]+)", re.I)


@dataclass
class Candidate:
    name: str
    url: str
    source: str
    why: str
    repo: str | None = None
    suggested_category: str = ""
    crates_io: str | None = None


def parse_sources(raw: str) -> list[str]:
    parts = [part.strip().lower() for part in (raw or "").split(",") if part.strip()]
    mapped: list[str] = []
    for part in parts:
        if part in {"twitter", "x"}:
            continue
        if part in VALID_SOURCES and part not in mapped:
            mapped.append(part)
    return mapped or list(VALID_SOURCES)


def looks_bio_related(text: str) -> bool:
    lower = (text or "").lower()
    return any(term in lower for term in BIO_TERMS)


def is_excluded(text: str) -> bool:
    lower = (text or "").lower()
    if any(re.search(pattern, lower) for pattern in ALWAYS_EXCLUDE):
        return True
    if looks_bio_related(lower):
        return False
    return any(re.search(pattern, lower) for pattern in EXCLUDE_IF_NO_BIO)


def has_rust_signal(text: str) -> bool:
    lower = (text or "").lower()
    if "rust" not in lower:
        return False
    if any(re.search(pattern, lower) for pattern in RUST_SIGNALS):
        return True
    if "github.com" in lower and "rust" in lower:
        return True
    return False


def extract_github_repos(text: str) -> list[str]:
    found: list[str] = []
    seen: set[str] = set()
    for match in GITHUB_URL_RE.finditer(text or ""):
        owner, name = match.group(1), match.group(2).rstrip(").,;:\"'")
        if name.endswith(".git"):
            name = name[:-4]
        repo = f"{owner}/{name}"
        inferred = github_repo_from_url(f"https://github.com/{repo}")
        if not inferred:
            continue
        key = inferred.lower()
        if key not in seen:
            seen.add(key)
            found.append(inferred)
    return found


def extract_crates_io(text: str) -> list[str]:
    found: list[str] = []
    seen: set[str] = set()
    for match in CRATES_IO_RE.finditer(text or ""):
        url = f"https://crates.io/crates/{match.group(1)}"
        if url not in seen:
            seen.add(url)
            found.append(url)
    return found


def suggest_category(text: str) -> str:
    lower = (text or "").lower()
    for category, hints in CATEGORY_HINTS:
        if any(hint in lower for hint in hints):
            return category
    return ""


def _blob(*parts: Any) -> str:
    chunks: list[str] = []
    for part in parts:
        if isinstance(part, (list, tuple)):
            chunks.append(" ".join(str(item) for item in part if item))
        elif part:
            chunks.append(str(part))
    return " ".join(chunks)


def parse_github_item(item: dict[str, Any], query: str) -> Candidate | None:
    if item.get("archived") or item.get("fork"):
        return None
    url = item.get("html_url") or ""
    repo = item.get("full_name") or github_repo_from_url(url)
    name = item.get("name") or (repo.split("/")[-1] if repo else "")
    text = _blob(name, repo, item.get("description"), item.get("topics"))
    if is_excluded(text) or not looks_bio_related(text):
        return None
    if not name or not url:
        return None
    stars = item.get("stargazers_count")
    pushed = (item.get("pushed_at") or "")[:10]
    why_bits = [f"GitHub Search 命中 `{query}`。"]
    if isinstance(stars, int):
        why_bits.append(f"★ {stars}。")
    if pushed:
        why_bits.append(f"pushed {pushed}。")
    crates = extract_crates_io(item.get("description") or "")
    return Candidate(
        name=name,
        url=normalize_url(url) or url,
        repo=repo,
        source="github",
        why="".join(why_bits),
        suggested_category=suggest_category(text),
        crates_io=crates[0] if crates else None,
    )


def parse_biorxiv_paper(paper: dict[str, Any]) -> Candidate | None:
    title = paper.get("title") or ""
    abstract = paper.get("abstract") or ""
    text = _blob(title, abstract, paper.get("category"))
    if is_excluded(text) or not has_rust_signal(text):
        return None
    repos = extract_github_repos(text)
    crates = extract_crates_io(text)
    doi = paper.get("doi") or ""
    url = f"https://github.com/{repos[0]}" if repos else (f"https://www.biorxiv.org/content/{doi}" if doi else "")
    if not url:
        return None
    name = repos[0].split("/")[-1] if repos else title.split(":")[0].strip()[:80]
    why = f"bioRxiv 预印本标题/摘要含 Rust 编程信号：{title.strip()[:160]}"
    return Candidate(
        name=name or "biorxiv-paper",
        url=url,
        repo=repos[0] if repos else None,
        source="biorxiv",
        why=why,
        suggested_category=suggest_category(text),
        crates_io=crates[0] if crates else None,
    )


def _candidate_key(candidate: Candidate) -> str:
    if candidate.repo:
        return candidate.repo.lower()
    return normalize_url(candidate.url) or candidate.url.lower()


def _prefer_url(first: Candidate, second: Candidate) -> str:
    for item in (first, second):
        if item.repo and "github.com" in (item.url or ""):
            return item.url
    return first.url or second.url


def dedupe_candidates(candidates: list[Candidate]) -> list[Candidate]:
    merged: dict[str, Candidate] = {}
    for candidate in candidates:
        key = _candidate_key(candidate)
        existing = merged.get(key)
        if existing is None:
            merged[key] = candidate
            continue
        sources: list[str] = []
        for source in f"{existing.source},{candidate.source}".split(","):
            source = source.strip()
            if source and source not in sources:
                sources.append(source)
        why = existing.why
        if candidate.why and candidate.why not in existing.why:
            why = f"{existing.why} {candidate.why}".strip()
        merged[key] = replace(
            existing,
            url=_prefer_url(existing, candidate),
            repo=existing.repo or candidate.repo,
            source=",".join(sources),
            why=why,
            suggested_category=existing.suggested_category or candidate.suggested_category,
            crates_io=existing.crates_io or candidate.crates_io,
        )
    return list(merged.values())


def split_new_and_known(
    candidates: list[Candidate],
    index: CatalogIndex,
) -> tuple[list[Candidate], list[Candidate]]:
    new_ones: list[Candidate] = []
    known: list[Candidate] = []
    for candidate in candidates:
        if is_cataloged(index, url=candidate.url, repo=candidate.repo):
            known.append(candidate)
        else:
            new_ones.append(candidate)
    return new_ones, known


def build_github_queries(since: str) -> list[str]:
    extra = f" pushed:>={since} archived:false fork:false"
    queries = [f"language:Rust topic:{topic}{extra}" for topic in GITHUB_TOPICS]
    queries.extend(f"language:Rust {keyword}{extra}" for keyword in GITHUB_KEYWORDS)
    return queries


def _github_headers(token: str) -> dict[str, str]:
    return {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": USER_AGENT,
    }


def fetch_github(
    session: requests.Session,
    token: str | None,
    days: int,
    now: datetime,
    sleep_s: float = 2.0,
    queries: list[str] | None = None,
) -> tuple[list[Candidate], str]:
    if not token:
        return [], "skipped: no GITHUB_TOKEN / GH_TOKEN"
    since = (now - timedelta(days=days)).date().isoformat()
    query_list = queries if queries is not None else build_github_queries(since)
    session.headers.update(_github_headers(token))
    found: list[Candidate] = []
    incomplete = False
    notes: list[str] = []
    try:
        for index, query in enumerate(query_list):
            if index and sleep_s:
                time.sleep(sleep_s)
            response = session.get(
                GITHUB_SEARCH_URL,
                params={"q": query, "per_page": 100, "sort": "updated", "order": "desc"},
                timeout=60,
            )
            if response.status_code in {401, 403}:
                incomplete = True
                notes.append(f"GitHub Search HTTP {response.status_code}")
                break
            if response.status_code != 200:
                incomplete = True
                notes.append(f"query `{query}` HTTP {response.status_code}")
                continue
            payload = response.json()
            if payload.get("incomplete_results"):
                incomplete = True
            for item in payload.get("items") or []:
                candidate = parse_github_item(item, query)
                if candidate:
                    found.append(candidate)
    except Exception as exc:  # noqa: BLE001
        return found, f"incomplete: {exc}"
    if incomplete:
        extra = "; ".join(notes) if notes else "partial GitHub Search results"
        return found, f"incomplete: {extra}"
    return found, "ok"


def fetch_biorxiv(
    session: requests.Session,
    days: int,
    now: datetime,
    sleep_s: float = 0.2,
) -> tuple[list[Candidate], str]:
    start = (now - timedelta(days=days)).date().isoformat()
    end = now.date().isoformat()
    session.headers.update({"User-Agent": USER_AGENT})
    found: list[Candidate] = []
    cursor = 0
    try:
        while True:
            url = f"{BIORXIV_DETAILS_URL}/{start}/{end}/{cursor}"
            response = session.get(url, timeout=60)
            if response.status_code != 200:
                return found, f"incomplete: bioRxiv HTTP {response.status_code}"
            payload = response.json()
            collection = payload.get("collection") or []
            for paper in collection:
                candidate = parse_biorxiv_paper(paper)
                if candidate:
                    found.append(candidate)
            messages = payload.get("messages") or []
            total = 0
            count = len(collection)
            if messages and isinstance(messages[0], dict):
                total = int(messages[0].get("total") or 0)
                count = int(messages[0].get("count") or count)
            if not collection:
                break
            cursor += max(count, 1)
            if total and cursor >= total:
                break
            if not total and count < 100:
                break
            if cursor >= 8000:
                return found, "incomplete: bioRxiv page cap reached"
            if sleep_s:
                time.sleep(sleep_s)
    except Exception as exc:  # noqa: BLE001
        return found, f"incomplete: {exc}"
    return found, "ok"


def render_report(
    *,
    today: str,
    days: int,
    sources: list[str],
    statuses: dict[str, str],
    candidates: list[Candidate],
    known_hits: list[Candidate],
) -> str:
    incomplete = any(
        source in sources and not str(statuses.get(source, "")).startswith("ok")
        for source in VALID_SOURCES
    )
    lines = [
        f"# 新工具候选 · {today}",
        "",
        "手动发现报告，不是收录清单。勾选后再改 `data/tools.yaml`。不要手改 README / RADAR。",
        "",
        "## 数据完整度",
        "",
    ]
    if incomplete:
        lines += ["**数据不完整。** 部分源未跑完，候选可能漏。", ""]
    for source in VALID_SOURCES:
        requested = source in sources
        status = statuses.get(source, "skipped: disabled")
        if not requested:
            status = "skipped: disabled"
        lines.append(f"- {source}：{status}")
    lines += [
        "",
        f"检索窗口：最近 {days} 天。源：{', '.join(sources) or '无'}。",
        "",
        "## 候选",
        "",
    ]
    if not candidates:
        lines += ["_本轮没有未收录候选。_", ""]
    else:
        for index, candidate in enumerate(candidates, start=1):
            lines += [
                f"### {index}. {candidate.name}",
                "",
                f"- url: {candidate.url}",
                f"- repo: {candidate.repo or ''}",
                f"- crates.io: {candidate.crates_io or ''}",
                f"- 来源: {candidate.source}",
                f"- 为什么被捞到: {candidate.why}",
                "- 已在目录: 否",
                f"- 建议 category: {candidate.suggested_category}",
                "- 收录？是/否：",
                "",
            ]
    lines += [
        "## 已收录仍被搜到",
        "",
        f"本轮命中已在 `tools.yaml`（含 retired）的条目 {len(known_hits)} 条，不列入上方候选。",
        "",
    ]
    return "\n".join(lines)


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Manually discover candidate Rust bioinformatics tools. Does not edit tools.yaml."
    )
    parser.add_argument("--days", type=int, default=14, help="Look-back window in days (default 14, max 365).")
    parser.add_argument(
        "--sources",
        default="github,biorxiv",
        help="Comma-separated sources: github,biorxiv",
    )
    parser.add_argument("--output", type=Path, default=None, help="Override report path.")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    tools = load_tools()
    errors = validate_tools(tools)
    if errors:
        print("tools.yaml is invalid:", file=sys.stderr)
        for item in errors:
            print(f"  - {item}", file=sys.stderr)
        return 1

    days = max(1, min(int(args.days), 365))
    sources = parse_sources(args.sources)
    index = catalog_index(tools)
    now = utcnow()
    collected: list[Candidate] = []
    statuses: dict[str, str] = {}

    if "github" in sources:
        token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
        session = requests.Session()
        try:
            cands, status = fetch_github(session, token, days, now)
        except Exception as exc:  # noqa: BLE001
            cands, status = [], f"incomplete: {exc}"
        collected.extend(cands)
        statuses["github"] = status
        print(f"github: {status} ({len(cands)} hits)", file=sys.stderr)
    else:
        statuses["github"] = "skipped: disabled"

    if "biorxiv" in sources:
        session = requests.Session()
        try:
            cands, status = fetch_biorxiv(session, days, now)
        except Exception as exc:  # noqa: BLE001
            cands, status = [], f"incomplete: {exc}"
        collected.extend(cands)
        statuses["biorxiv"] = status
        print(f"biorxiv: {status} ({len(cands)} hits)", file=sys.stderr)
    else:
        statuses["biorxiv"] = "skipped: disabled"

    merged = dedupe_candidates(collected)
    new_ones, known_hits = split_new_and_known(merged, index)
    new_ones.sort(key=lambda item: (item.source, item.name.lower()))
    today = now.date().isoformat()
    text = render_report(
        today=today,
        days=days,
        sources=sources,
        statuses=statuses,
        candidates=new_ones,
        known_hits=known_hits,
    )
    path = Path(args.output) if args.output else DISCOVER_DIR / f"candidates-{today}.md"
    if not path.is_absolute():
        path = ROOT / path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")
    try:
        shown = path.relative_to(ROOT)
    except ValueError:
        shown = path
    print(f"Wrote {shown} ({len(new_ones)} candidates, {len(known_hits)} already listed)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
