from scripts.common import try_command
from glob import glob


def format_code():
    print("[!] Running rust-fmt...")
    try_command(["cargo", "format"])

    print("[!] Running clang-format...")
    sources = (glob("src/lib/backend/**/*.cc", recursive=True) +
               glob("src/lib/backend/**/*.h", recursive=True))

    for source in sources:
        try_command(["clang-format", "-i", "--style=file", source])