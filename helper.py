#!/usr/bin/env python3

# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

# The helper is a Python script intended to run various maintenance tasks on the codebase.

import argparse
import scripts.lint
import scripts.format

if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="./helper.py")

    parser.add_argument(
        "command",
        choices=["lint", "format"],
        help="the action to be performed"
    )

    parser.add_argument(
        "-d", "--dry-run",
        action="store_true", default=0,
        help="prevents files from being changed when applicable, "
             "instead returning an error code if changes are needed."
    )

    parser.add_argument(
        "-a", "--agnostic",
        action="store_true", default=0,
        help="only performs platform agnostic actions."
    )

    args = parser.parse_args()

    if args.command == "lint":
        scripts.lint.lint_code(args)
    elif args.command == "format":
        scripts.format.format_code(args)
