"""Makes the machine table that the upstream engine reads, with upstream's own capture tools.

The table holds what Windows and the display driver answer on this machine. It is made in the
order of upstream's launcher (`scripts/launcher/src/tables.rs`, `make_adapter_table`):
1. `d3d9-capture` (no arguments);
2. `nvapi-capture <FalloutNV.exe>` (a 32-bit tool);
3. the `CreateQuery` lines of `d3d9-capture device queries`.

The table describes one machine. Keep it out of the repository.

Usage: python -I mk_table.py <TOOLS FOLDER> <FalloutNV.exe> <OUT FILE>

TOOLS FOLDER holds `release/d3d9-capture.exe` and
`i686-pc-windows-msvc/release/nvapi-capture.exe` (a cargo target folder).
"""
import os
import subprocess
import sys


def run(args):
    done = subprocess.run(args, capture_output=True, timeout=120)
    if done.returncode != 0:
        sys.exit("%s: status %d\n%s" % (args[0], done.returncode, done.stderr.decode(errors="replace")))
    return done.stdout


def main(argv):
    if len(argv) != 4:
        sys.exit(__doc__)
    tools, game_exe, out = argv[1], argv[2], argv[3]
    d3d9 = os.path.join(tools, "release", "d3d9-capture.exe")
    nvapi = os.path.join(tools, "i686-pc-windows-msvc", "release", "nvapi-capture.exe")
    table = run([d3d9])
    table += run([nvapi, game_exe])
    for line in run([d3d9, "device", "queries"]).splitlines():
        if line.startswith(b"CreateQuery\t"):
            table += line + b"\n"
    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    with open(out, "wb") as fh:
        fh.write(table)
    print("%s: %d lines" % (out, table.count(b"\n")))


if __name__ == "__main__":
    main(sys.argv)
