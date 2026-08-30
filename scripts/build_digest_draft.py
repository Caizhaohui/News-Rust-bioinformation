"""Write a Chinese editorial outline from JSON data, not from RADAR.md."""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import (  # noqa: E402
    ROOT,
    compute_radar,
    load_config,
    load_metadata,
    load_tools,
    previous_snapshot,
    repo_record,
    utcnow,
    validate_tools,
)

DIGEST_DIR = ROOT / "digest"


def _fact(tool: dict, metadata: dict) -> str:
    record = repo_record(metadata, tool.get("repo"))
    bits: list[str] = []
    if tool.get("reason"):
        bits.append(tool["reason"])
    stars = record.get("stars")
    if isinstance(stars, int):
        bits.append(f"★ {stars}")
    tag = record.get("latest_release_tag")
    if tag:
        bits.append(f"release {tag}")
    return "；".join(bits) if bits else "见仓库"


def _candidate_block(index: int, tool: dict, metadata: dict) -> str:
    return "\n".join(
        [
            f"### {index}. [{tool['name']}]({tool['url']})",
            "",
            f"- 事实：{_fact(tool, metadata)}",
            "- 为什么值得写：",
            "",
        ]
    )


def build_digest(tools: list[dict], metadata: dict, previous: dict | None, config: dict) -> str:
    radar = compute_radar(tools, metadata, previous, config)
    today = utcnow().date().isoformat()
    ranked = radar["new_entries"] + radar["active"]
    seen: set[str] = set()
    unique: list[dict] = []
    for tool in ranked:
        if tool["url"] in seen:
            continue
        seen.add(tool["url"])
        unique.append(tool)
    picks = unique[:5]

    lines = [
        "---",
        f"title: Rust 生信动态 · {today}",
        "description: 本周期 Rust 在生物信息学中的新工具、活跃仓库与值得盯的变化。",
        "pubDate: 2099-01-01",
        "tags: [Rust, 生物信息学, 生态跟踪]",
        "lang: zh",
        "draft: true",
        "---",
        "",
        f"# Rust 生信动态 · {today}",
        "",
        "机器生成的编辑提纲，不是成稿。删到 3–5 条后再写评述。发布时把 `pubDate` 改成真实日期，并把 `draft` 设为 `false`。",
        "",
        "## 本周要点（候选）",
        "",
    ]
    if not radar["baseline"]:
        lines += ["当前只有一份快照，还不能做周对比。下周再刷新后才会出现 New / Active 候选。", ""]
    elif not picks:
        lines += ["本周期没有达到阈值的 New / Active 条目。可以写停更观察，或跳过这一期。", ""]
    else:
        for index, tool in enumerate(picks, start=1):
            lines.append(_candidate_block(index, tool, metadata))

    lines += ["## 新工具", ""]
    if radar["new_entries"]:
        for tool in radar["new_entries"]:
            lines.append(f"- [{tool['name']}]({tool['url']}) — {_fact(tool, metadata)}")
            lines.append("  - 为什么值得写：")
    else:
        lines.append("_无。_")
    lines += ["", "## 值得盯", ""]
    if radar["active"]:
        for tool in radar["active"]:
            lines.append(f"- [{tool['name']}]({tool['url']}) — {_fact(tool, metadata)}")
            lines.append("  - 为什么值得写：")
    else:
        lines.append("_无。_")
    lines += ["", "## 停更观察", ""]
    if radar["stale"]:
        for tool in radar["stale"]:
            lines.append(f"- [{tool['name']}]({tool['url']}) — {tool.get('reason', '')}")
    else:
        lines.append("_无。_")
    lines += ["", "## 下一篇待查", "", "- ", ""]
    return "\n".join(lines)


def main() -> int:
    tools = load_tools()
    errors = validate_tools(tools)
    if errors:
        print("tools.yaml is invalid:", file=sys.stderr)
        for item in errors:
            print(f"  - {item}", file=sys.stderr)
        return 1
    today = utcnow().date().isoformat()
    path = DIGEST_DIR / f"rust-bio-digest-{today}.md"
    DIGEST_DIR.mkdir(parents=True, exist_ok=True)
    text = build_digest(tools, load_metadata(), previous_snapshot(), load_config())
    path.write_text(text, encoding="utf-8")
    print(f"Wrote {path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
