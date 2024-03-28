# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

from scripts.common import try_command
from glob import glob


def format_code():
    print("[!] Running rust-fmt...")
    try_command(["cargo", "fmt"])

    print("[!] Running clang-format...")
    sources = (glob("src/lib/backend/**/*.cc", recursive=True) +
               glob("src/lib/backend/**/*.h", recursive=True))

    for source in sources:
        try_command(["clang-format", "-i", "--style=file", source])
