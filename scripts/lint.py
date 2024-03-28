from scripts.common import try_command


def lint_code():
    print("[!] Running clippy...")
    try_command(["cargo", "clippy"])

    # TODO: add a method of linting C/C++ that doesn't require compilation.
