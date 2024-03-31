# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

from scripts.common import try_command
from glob import glob


def format_code(args) -> None:
    print("[!] Running rust-fmt...")
    if args.dry_run:
        try_command(["cargo", "fmt"])
    else:
        try_command(["cargo", "fmt", "--", "--check"])

    print("[!] Running clang-format...")
    sources = (glob("src/lib/backend/**/*.cc", recursive=True) +
               glob("src/lib/backend/**/*.h", recursive=True))

    for source in sources:
        if args.dry_run:
            try_command(["clang-format", "--dry-run", "-Werror", "-i", "--style=file", source])
        else:
            try_command(["clang-format", "-i", "--style=file", source])

