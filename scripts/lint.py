# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

import platform, os
from scripts.common import try_command
from glob import glob


def lint_code(args) -> None:
    print("[!] Running clippy...")
    try_command(["cargo clippy --all-targets --all-features -- -D warnings"])

    print("[!] Running cppcheck...")
    try_command(["cppcheck --project=.cppcheck"])

    if not args.agnostic:
        print("[!] Running clang-tidy...")

        # Build required bridge header so clang-tidy can run.
        try_command(["cxxbridge src/lib/backend/bridge.rs --header > src/lib/backend/bridge.rs.h"])

        sources = glob("src/lib/backend/**/*.cc", recursive=True)

        tidy_args = "-x c++ -I../"
        if platform.system() == "Darwin":
            tidy_args += " -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk"

        for source in sources:
            if args.dry_run:
                try_command([f"clang-tidy {source} -- {tidy_args}"])
            else:
                try_command([f"clang-tidy --fix {source} -- {tidy_args}"])

        os.remove("src/lib/backend/bridge.rs.h")
