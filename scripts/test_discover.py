"""Discover filters and report tests. HTTP is mocked; no live network."""

from __future__ import annotations

import sys
import unittest
from datetime import datetime, timezone
from pathlib import Path
from unittest.mock import Mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (  # noqa: E402
    catalog_index,
    github_repo_from_url,
    is_cataloged,
    normalize_url,
)
from discover import (  # noqa: E402
    Candidate,
    build_github_queries,
    dedupe_candidates,
    extract_crates_io,
    extract_github_repos,
    fetch_biorxiv,
    fetch_github,
    has_rust_signal,
    is_excluded,
    looks_bio_related,
    parse_biorxiv_paper,
    parse_github_item,
    parse_sources,
    render_report,
    split_new_and_known,
    suggest_category,
)

NOW = datetime(2026, 8, 30, 12, 0, tzinfo=timezone.utc)


def _index(*tools: dict) -> object:
    return catalog_index(list(tools))


class NormalizeTests(unittest.TestCase):
    def test_normalize_url_strips_slash_query_and_git(self) -> None:
        self.assertEqual(
            normalize_url("https://WWW.GitHub.com/Lab/Tool.git/?tab=readme#x"),
            "https://github.com/Lab/Tool",
        )

    def test_github_repo_from_nested_and_reserved_paths(self) -> None:
        self.assertEqual(
            github_repo_from_url("https://github.com/Lab/Tool/issues/3"),
            "Lab/Tool",
        )
        self.assertIsNone(github_repo_from_url("https://github.com/topics/bioinformatics"))
        self.assertIsNone(github_repo_from_url("https://crates.io/crates/noodles"))


class CatalogFilterTests(unittest.TestCase):
    def test_known_url_and_trailing_slash(self) -> None:
        index = _index(
            {
                "name": "noodles",
                "url": "https://github.com/zaeleus/noodles",
                "repo": "zaeleus/noodles",
            }
        )
        self.assertTrue(is_cataloged(index, url="https://github.com/zaeleus/noodles/"))
        self.assertTrue(is_cataloged(index, repo="Zaeleus/Noodles"))
        self.assertFalse(is_cataloged(index, url="https://github.com/lab/newtool"))

    def test_retired_and_crates_url_still_count(self) -> None:
        index = _index(
            {
                "name": "gone",
                "url": "https://github.com/o/gone",
                "repo": "o/gone",
                "status": "retired",
            },
            {
                "name": "crate-only",
                "url": "https://crates.io/crates/example",
                "repo": "org/example",
            },
        )
        self.assertTrue(is_cataloged(index, repo="o/gone"))
        self.assertTrue(is_cataloged(index, url="https://github.com/org/example"))
        self.assertTrue(is_cataloged(index, url="https://crates.io/crates/example"))


class KeywordFilterTests(unittest.TestCase):
    def test_exclude_generic_csv_and_slurm(self) -> None:
        self.assertTrue(is_excluded("A fast CSV parser for huge tables."))
        self.assertTrue(is_excluded("Slurm TUI dashboard for HPC clusters."))
        self.assertTrue(is_excluded("Cluster dashboard with no biology-specific purpose."))
        self.assertFalse(is_excluded("Export VCF and FASTQ summaries as CSV."))

    def test_keep_bioinformatics_drop_unrelated_rust(self) -> None:
        self.assertTrue(looks_bio_related("Rust aligner for long-read genomics."))
        self.assertFalse(looks_bio_related("Generic web framework in Rust."))
        self.assertFalse(looks_bio_related("CSV spreadsheet toolkit."))

    def test_biorxiv_needs_programming_rust_signal(self) -> None:
        self.assertTrue(has_rust_signal("We implemented the caller in Rust and released it on crates.io."))
        self.assertTrue(has_rust_signal("A Rust-based toolkit for metagenomics."))
        self.assertFalse(has_rust_signal("Wheat rust resistance in genomic selection."))
        self.assertFalse(has_rust_signal("Rust-colored colonies were observed."))


class LinkExtractTests(unittest.TestCase):
    def test_extract_github_and_crates_links(self) -> None:
        text = (
            "See https://github.com/lab/newtool.git and "
            "https://crates.io/crates/newtool plus https://github.com/topics/genomics."
        )
        self.assertEqual(extract_github_repos(text), ["lab/newtool"])
        self.assertEqual(extract_crates_io(text), ["https://crates.io/crates/newtool"])


class ParseSourceTests(unittest.TestCase):
    def test_parse_github_keeps_bio_repo(self) -> None:
        item = {
            "name": "newtool",
            "full_name": "lab/newtool",
            "html_url": "https://github.com/lab/newtool",
            "description": "A Rust aligner for long-read genomics.",
            "topics": ["bioinformatics", "rust"],
            "language": "Rust",
            "archived": False,
            "fork": False,
            "stargazers_count": 12,
            "pushed_at": "2026-08-29T00:00:00Z",
        }
        cand = parse_github_item(item, query="language:Rust topic:genomics")
        assert cand is not None
        self.assertEqual(cand.repo, "lab/newtool")
        self.assertEqual(cand.source, "github")
        self.assertIn("topic:genomics", cand.why)

    def test_parse_github_drops_csv_and_unrelated(self) -> None:
        csv_item = {
            "name": "csvkit-rs",
            "full_name": "lab/csvkit-rs",
            "html_url": "https://github.com/lab/csvkit-rs",
            "description": "A fast CSV parser for huge tables.",
            "topics": ["csv", "rust"],
            "language": "Rust",
            "archived": False,
        }
        web = {
            "name": "axum-demo",
            "full_name": "lab/axum-demo",
            "html_url": "https://github.com/lab/axum-demo",
            "description": "Demo web server.",
            "topics": ["web"],
            "language": "Rust",
            "archived": False,
        }
        self.assertIsNone(parse_github_item(csv_item, query="language:Rust csv"))
        self.assertIsNone(parse_github_item(web, query="language:Rust bioinformatics"))

    def test_parse_biorxiv_extracts_repo(self) -> None:
        paper = {
            "doi": "10.1101/2026.08.20.999999",
            "title": "Newtool: a Rust-based caller for long-read genomics",
            "abstract": (
                "We implemented Newtool in Rust. "
                "Source: https://github.com/lab/newtool and https://crates.io/crates/newtool."
            ),
            "date": "2026-08-20",
            "category": "bioinformatics",
        }
        cand = parse_biorxiv_paper(paper)
        assert cand is not None
        self.assertEqual(cand.repo, "lab/newtool")
        self.assertEqual(cand.source, "biorxiv")
        self.assertEqual(cand.crates_io, "https://crates.io/crates/newtool")

    def test_parse_biorxiv_skips_plant_rust(self) -> None:
        paper = {
            "doi": "10.1101/2026.08.01.111111",
            "title": "Wheat rust resistance loci",
            "abstract": "We mapped stem rust resistance in wheat genomics.",
            "date": "2026-08-01",
            "category": "genomics",
        }
        self.assertIsNone(parse_biorxiv_paper(paper))

class DedupeAndSplitTests(unittest.TestCase):
    def test_dedupe_same_repo_from_two_sources(self) -> None:
        a = Candidate(
            name="newtool",
            url="https://github.com/lab/newtool",
            repo="lab/newtool",
            source="github",
            why="GitHub Search 命中 language:Rust topic:genomics。",
        )
        b = Candidate(
            name="Newtool paper",
            url="https://www.biorxiv.org/content/10.1101/x",
            repo="Lab/Newtool",
            source="biorxiv",
            why="bioRxiv 摘要含 Rust + genomics。",
            crates_io="https://crates.io/crates/newtool",
        )
        merged = dedupe_candidates([a, b])
        self.assertEqual(len(merged), 1)
        self.assertIn("github", merged[0].source)
        self.assertIn("biorxiv", merged[0].source)
        self.assertEqual(merged[0].url, "https://github.com/lab/newtool")
        self.assertEqual(merged[0].crates_io, "https://crates.io/crates/newtool")

    def test_split_filters_cataloged_including_retired(self) -> None:
        index = _index(
            {
                "name": "old",
                "url": "https://github.com/o/old",
                "repo": "o/old",
                "status": "retired",
            }
        )
        known = Candidate(
            name="old",
            url="https://github.com/o/old",
            repo="o/old",
            source="github",
            why="already listed",
        )
        fresh = Candidate(
            name="newtool",
            url="https://github.com/lab/newtool",
            repo="lab/newtool",
            source="github",
            why="new",
        )
        new_ones, known_hits = split_new_and_known([known, fresh], index)
        self.assertEqual([c.name for c in new_ones], ["newtool"])
        self.assertEqual([c.repo for c in known_hits], ["o/old"])


class ReportTests(unittest.TestCase):
    def test_known_hits_only_in_footer_and_include_blank(self) -> None:
        cand = Candidate(
            name="newtool",
            url="https://github.com/lab/newtool",
            repo="lab/newtool",
            source="github",
            why="GitHub Search 命中 language:Rust topic:genomics。",
            suggested_category="long-reads",
        )
        known = Candidate(
            name="noodles",
            url="https://github.com/zaeleus/noodles",
            repo="zaeleus/noodles",
            source="github",
            why="already listed",
        )
        text = render_report(
            today="2026-08-30",
            days=14,
            sources=["github", "biorxiv"],
            statuses={
                "github": "ok",
                "biorxiv": "skipped: no token",
            },
            candidates=[cand],
            known_hits=[known],
        )
        self.assertIn("**数据不完整。**", text)
        self.assertIn("skipped: no token", text)
        self.assertIn("收录？是/否：", text)
        self.assertIn("newtool", text)
        self.assertNotIn("zaeleus/noodles", text.split("## 已收录仍被搜到")[0])
        self.assertIn("本轮命中已在 `tools.yaml`（含 retired）的条目 1 条", text)
        self.assertNotIn("为什么值得写", text)

    def test_disabled_source_is_not_incomplete(self) -> None:
        text = render_report(
            today="2026-08-30",
            days=14,
            sources=["github"],
            statuses={
                "github": "ok",
                "biorxiv": "skipped: disabled",
            },
            candidates=[],
            known_hits=[],
        )
        self.assertNotIn("**数据不完整。**", text)

    def test_suggest_category_can_be_empty(self) -> None:
        self.assertEqual(suggest_category("A Rust aligner for long-read genomics."), "long-reads")
        self.assertEqual(suggest_category("Fast sgRNA counting for CRISPR screens."), "crispr")
        self.assertEqual(suggest_category("Rust crates for protein language models and protein design."), "protein-engineering")
        self.assertEqual(suggest_category("16S microbiome ASV caller."), "metagenomics")
        self.assertEqual(suggest_category("Bacterial transcript units from mapped reads."), "prokaryotic-transcriptome")
        self.assertEqual(suggest_category("Miscellaneous notes."), "")


class TokenDegradeTests(unittest.TestCase):
    def test_github_skips_without_token_and_does_not_call_http(self) -> None:
        session = Mock()
        cands, status = fetch_github(session, token=None, days=14, now=NOW, sleep_s=0)
        self.assertEqual(cands, [])
        self.assertIn("token", status.lower())
        session.get.assert_not_called()

    def test_github_parses_mocked_search(self) -> None:
        session = Mock()
        session.get.return_value = Mock(
            status_code=200,
            json=lambda: {
                "items": [
                    {
                        "name": "newtool",
                        "full_name": "lab/newtool",
                        "html_url": "https://github.com/lab/newtool",
                        "description": "A Rust aligner for long-read genomics.",
                        "topics": ["genomics", "rust"],
                        "language": "Rust",
                        "archived": False,
                        "fork": False,
                        "stargazers_count": 3,
                        "pushed_at": "2026-08-29T00:00:00Z",
                    }
                ]
            },
        )
        cands, status = fetch_github(
            session,
            token="t",
            days=14,
            now=NOW,
            sleep_s=0,
            queries=["language:Rust topic:genomics"],
        )
        self.assertTrue(status.startswith("ok"))
        self.assertEqual([c.repo for c in cands], ["lab/newtool"])
        session.get.assert_called()

    def test_biorxiv_parses_mocked_collection(self) -> None:
        session = Mock()
        session.get.return_value = Mock(
            status_code=200,
            json=lambda: {
                "messages": [{"status": "ok", "count": 1, "total": 1}],
                "collection": [
                    {
                        "doi": "10.1101/2026.08.20.999999",
                        "title": "Newtool: a Rust-based caller",
                        "abstract": "Implemented in Rust for genomics. https://github.com/lab/newtool",
                        "date": "2026-08-20",
                        "category": "bioinformatics",
                    }
                ],
            },
        )
        cands, status = fetch_biorxiv(session, days=14, now=NOW, sleep_s=0)
        self.assertTrue(status.startswith("ok"))
        self.assertEqual([c.repo for c in cands], ["lab/newtool"])

    def test_parse_sources_and_github_queries(self) -> None:
        self.assertEqual(parse_sources("github,x"), ["github"])
        self.assertEqual(parse_sources("twitter"), ["github", "biorxiv"])
        queries = build_github_queries("2026-08-16")
        self.assertTrue(any("topic:bioinformatics" in q for q in queries))
        self.assertTrue(any(" language:Rust genomics " in f" {q} " or q.endswith("genomics") or "genomics " in q for q in queries))
        self.assertTrue(all("language:Rust" in q for q in queries))
        self.assertTrue(all("pushed:>=2026-08-16" in q for q in queries))


if __name__ == "__main__":
    unittest.main()
