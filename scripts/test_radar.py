"""Radar classification tests. Run: python -m unittest scripts.test_radar"""

from __future__ import annotations

import sys
import unittest
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import CATEGORIES, VALID_CATEGORIES, compute_radar, tool_sort_key  # noqa: E402
from build_readme import build_readme  # noqa: E402

NOW = datetime(2026, 8, 30, tzinfo=timezone.utc)
CONFIG = {
    "radar": {
        "stale_months": 18,
        "cold_inactive_months": 6,
        "active": {
            "min_star_delta": 5,
            "include_new_release": True,
            "include_cold_repo_push": True,
        },
    }
}


def tool(name: str, repo: str, url: str | None = None) -> dict:
    return {
        "name": name,
        "url": url or f"https://github.com/{repo}",
        "repo": repo,
        "category": "core-libraries",
        "description": "Test.",
    }


class RadarTests(unittest.TestCase):
    def test_no_baseline_skips_new_and_active(self) -> None:
        tools = [tool("a", "o/a")]
        current = {
            "urls": [tools[0]["url"]],
            "repos": {
                "o/a": {
                    "stars": 10,
                    "pushed_at": "2026-08-29T00:00:00Z",
                    "archived": False,
                }
            },
        }
        radar = compute_radar(tools, current, None, CONFIG, now=NOW)
        self.assertFalse(radar["baseline"])
        self.assertEqual(radar["new_entries"], [])
        self.assertEqual(radar["active"], [])

    def test_new_entry_from_url_diff(self) -> None:
        old = tool("a", "o/a")
        new = tool("b", "o/b")
        previous = {"urls": [old["url"]], "repos": {}}
        current = {"urls": [old["url"], new["url"]], "repos": {}}
        radar = compute_radar([old, new], current, previous, CONFIG, now=NOW)
        self.assertEqual([t["name"] for t in radar["new_entries"]], ["b"])

    def test_star_delta_and_ordinary_push(self) -> None:
        t = tool("a", "o/a")
        previous = {
            "urls": [t["url"]],
            "repos": {
                "o/a": {
                    "stars": 10,
                    "pushed_at": "2026-08-20T00:00:00Z",
                    "archived": False,
                    "latest_release_at": "2026-01-01T00:00:00Z",
                }
            },
        }
        quiet = {
            "urls": [t["url"]],
            "repos": {
                "o/a": {
                    "stars": 12,
                    "pushed_at": "2026-08-29T00:00:00Z",
                    "archived": False,
                    "latest_release_at": "2026-01-01T00:00:00Z",
                }
            },
        }
        radar = compute_radar([t], quiet, previous, CONFIG, now=NOW)
        self.assertEqual(radar["active"], [])

        jumped = {
            "urls": [t["url"]],
            "repos": {
                "o/a": {
                    "stars": 20,
                    "pushed_at": "2026-08-29T00:00:00Z",
                    "archived": False,
                    "latest_release_at": "2026-01-01T00:00:00Z",
                }
            },
        }
        radar = compute_radar([t], jumped, previous, CONFIG, now=NOW)
        self.assertEqual(len(radar["active"]), 1)
        self.assertIn("stars", radar["active"][0]["reason"])

    def test_new_entry_not_also_active(self) -> None:
        old = tool("a", "o/a")
        new = tool("b", "o/b")
        previous = {"urls": [old["url"]], "repos": {}}
        current = {
            "urls": [old["url"], new["url"]],
            "repos": {
                "o/b": {
                    "stars": 10,
                    "pushed_at": "2026-08-29T00:00:00Z",
                    "archived": False,
                    "latest_release_at": "2026-08-20T00:00:00Z",
                    "latest_release_tag": "v1.0",
                }
            },
        }
        radar = compute_radar([old, new], current, previous, CONFIG, now=NOW)
        self.assertEqual([t["name"] for t in radar["new_entries"]], ["b"])
        self.assertEqual(radar["active"], [])

    def test_retired_skipped_in_new_and_stale(self) -> None:
        live = tool("live", "o/live")
        gone = tool("gone", "o/gone")
        gone["status"] = "retired"
        previous = {"urls": [live["url"]], "repos": {}}
        current = {
            "urls": [live["url"], gone["url"]],
            "repos": {
                "o/live": {
                    "stars": 3,
                    "pushed_at": "2026-08-29T00:00:00Z",
                    "archived": False,
                },
                "o/gone": {
                    "stars": 1,
                    "pushed_at": "2024-01-01T00:00:00Z",
                    "archived": True,
                },
            },
        }
        radar = compute_radar([live, gone], current, previous, CONFIG, now=NOW)
        self.assertEqual([t["name"] for t in radar["new_entries"]], [])
        self.assertEqual([t["name"] for t in radar["stale"]], [])

    def test_stale_and_watch(self) -> None:
        stale = tool("old", "o/old")
        missing = tool("gone", "o/gone")
        current = {
            "urls": [stale["url"], missing["url"]],
            "repos": {
                "o/old": {
                    "stars": 1,
                    "pushed_at": "2024-01-01T00:00:00Z",
                    "archived": False,
                },
                "o/gone": {"error": "not found"},
            },
        }
        previous = {"urls": [stale["url"], missing["url"]], "repos": current["repos"]}
        radar = compute_radar([stale, missing], current, previous, CONFIG, now=NOW)
        self.assertEqual([t["name"] for t in radar["stale"]], ["old"])
        self.assertEqual([t["name"] for t in radar["watch"]], ["gone"])


class SortKeyTests(unittest.TestCase):
    def test_sorts_by_stars_then_push_then_name(self) -> None:
        tools = [
            tool("zeta", "o/zeta"),
            tool("alpha", "o/alpha"),
            tool("mid", "o/mid"),
            tool("recent-low", "o/recent-low"),
            tool("no-stars", "o/no-stars"),
            {"name": "orphan", "url": "https://example.org/orphan", "category": "core-libraries", "description": "Test."},
        ]
        metadata = {
            "repos": {
                "o/zeta": {"stars": 100, "pushed_at": "2026-01-01T00:00:00Z"},
                "o/alpha": {"stars": 100, "pushed_at": "2026-01-01T00:00:00Z"},
                "o/mid": {"stars": 50, "pushed_at": "2026-08-01T00:00:00Z"},
                "o/recent-low": {"stars": 10, "pushed_at": "2026-08-29T00:00:00Z"},
                "o/no-stars": {"pushed_at": "2026-08-30T00:00:00Z"},
            }
        }
        ordered = sorted(tools, key=lambda t: tool_sort_key(t, metadata))
        self.assertEqual(
            [t["name"] for t in ordered],
            ["alpha", "zeta", "mid", "recent-low", "no-stars", "orphan"],
        )


class CategoryHierarchyTests(unittest.TestCase):
    def test_bacterial_section_is_not_a_tool_category(self) -> None:
        self.assertEqual(CATEGORIES[0].slug, "crispr")
        self.assertEqual(CATEGORIES[1].slug, "bacterial-bioinformatics")
        self.assertTrue(CATEGORIES[1].is_section)
        self.assertNotIn("bacterial-bioinformatics", VALID_CATEGORIES)
        for slug in (
            "bacterial-assembly",
            "bacterial-annotation",
            "prokaryotic-transcriptome",
            "microbiome",
            "phage-defense",
            "resistance-genes",
            "transposons",
        ):
            self.assertIn(slug, VALID_CATEGORIES)

    def test_readme_lists_section_then_children(self) -> None:
        tools = [
            {
                "name": "demo",
                "url": "https://example.org/demo",
                "category": "bacterial-assembly",
                "description": "Test.",
            }
        ]
        text = build_readme(tools, {"repos": {}})
        self.assertIn("### Bacterial Bioinformatics", text)
        self.assertIn("#### Bacterial Genome Assembly", text)
        self.assertIn("#### Phage Defense Systems", text)
        self.assertIn("- [Bacterial Bioinformatics](#bacterial-bioinformatics)", text)
        self.assertIn("  - [Genome Annotation](#genome-annotation)", text)
        crispr = text.index("### CRISPR") if "### CRISPR" in text else -1
        bacterial = text.index("### Bacterial Bioinformatics")
        core = text.index("### Core Libraries") if "### Core Libraries" in text else len(text)
        self.assertLess(bacterial, core)
        if crispr >= 0:
            self.assertLess(crispr, bacterial)


if __name__ == "__main__":
    unittest.main()
