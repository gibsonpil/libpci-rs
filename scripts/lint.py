# Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
# SPDX-License-Identifier: BSD-3-Clause

from scripts.common import try_command


def lint_code():
    print("[!] Running clippy...")
    try_command(["cargo", "clippy"])

    # TODO: add a method of linting C/C++ that doesn't require compilation.
