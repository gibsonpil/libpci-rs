# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

#!/usr/bin/env python3

# This is a useful automation script to be used in conjunction
# with the modules in the scripts folder.

import argparse
import scripts.lint
import scripts.format

if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="./helper.py")
    parser.add_argument(
        "command",
        choices=["lint", "format"]
    )
    args = parser.parse_args()

    if args.command == "lint":
        scripts.lint.lint_code()
    elif args.command == "format":
        scripts.format.format_code()
