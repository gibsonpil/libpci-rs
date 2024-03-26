import subprocess
import sys


def try_command(args: list[str]):
    try:
        subprocess.run(args, capture_output=True, check=True)
    except subprocess.CalledProcessError as err:
        print(f"[x] ERROR: child command `{" ".join(err.cmd)}` exited with non-zero status code {err.returncode}.")
        exit(-1)
