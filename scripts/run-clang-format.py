# This is just a quick and dirty Python script to run clang-format
# on all the backend source files.

from glob import glob
import os

sources = (glob("src/backend/**/*.cc", recursive=True) +
           glob("src/backend/**/*.h", recursive=True))

for source in sources:
    os.system(f"clang-format -i --style=file {source}")
