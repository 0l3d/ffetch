#!/usr/bin/env python3

import os
import subprocess
import sys
from pathlib import Path


def main() -> int:
    project_root = Path(__file__).resolve().parent
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(project_root / "target")

    result = subprocess.run(
        ["cargo", "build", "--locked"],
        cwd=project_root,
        env=env,
        check=False,
    )
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
