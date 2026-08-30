"""Validate data/tools.yaml."""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from common import load_tools, validate_tools  # noqa: E402


def main() -> int:
    errors = validate_tools(load_tools())
    if errors:
        print("tools.yaml is invalid:")
        for item in errors:
            print(f"  - {item}")
        return 1
    print("tools.yaml ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
