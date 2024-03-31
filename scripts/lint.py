# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

import os
from scripts.common import try_command
from glob import glob


def lint_code(args) -> None:
    print("[!] Running clippy...")
    try_command(["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])

    print("[!] Running cppcheck...")
    try_command(["cppcheck", "--project=.cppcheck"])

    # if not args.agnostic:
    #     print("[!] Running clang-tidy...")
    #
    #     # Build required bridge header so clang-tidy can run.
    #     try_command(["cxxbridge", "src/lib/backend/bridge.rs", "--header", ">", "src/lib/backend/bridge.rs.h"])
    #
    #     sources = glob("src/lib/backend/**/*.cc", recursive=True)
    #
    #     args = "-x c++ -I../"
    #     if os.name == "Darwin":
    #         args += " -l framework=CoreFoundation -l framework=IOKit"
    #
    #     for source in sources:
    #         try_command(["clang-tidy", source, "--", *args.split(" ")])
    #
    #     os.remove("src/lib/backend/bridge.rs.h")
