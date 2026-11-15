"""Makes a scratch instance for a test run of the upstream engine.

The engine reads the folders of the player's machine from the machine table (rows
`SHGetFolderPathA`). A test run must not read or show the player's own Documents. This tool
copies the table and points its folder rows at a scratch folder, one for each instance (the
host, each guest).

The instance also gets copies of the player's `Fallout.ini` and `FalloutPrefs.ini`, changed to a
window of 1280 by 720. Without settings the game asks for its launcher and starts full screen.
It also gets a `plugins.txt` (the active plugins): the names of `--plugins`, else a copy of the
player's file. The player's own files are only read.

Usage: python -I mk_instance.py <INSTANCE FOLDER> [--table FILE] [--size W H] [--plugins A,B,..]

It prints the path of the instance's table. Set NV_D3D9_ADAPTER to it for the run.
"""
import os
import sys

# CSIDL values the game asks for: Documents, AppData, Local AppData.
SCRATCH = {"5": "Documents", "1a": "AppData/Roaming", "1c": "AppData/Local"}
GAME_FOLDER = ("My Games", "FalloutNV")
ANSI = "mbcs" if os.name == "nt" else "latin-1"
TAB = chr(9)


def folder_rows(table_text):
    """Yields (line index, columns) of the table's folder rows."""
    for i, line in enumerate(table_text.splitlines()):
        cols = line.split(TAB)
        if cols[0] == "SHGetFolderPathA" and len(cols) >= 4:
            yield i, cols


def instance_table(table_text, root):
    """The table with its folder rows pointed into `root` (a Windows path)."""
    lines = table_text.splitlines()
    for i, cols in folder_rows(table_text):
        csidl = cols[1].split(" ")[0].lower()
        if csidl in SCRATCH:
            path = os.path.join(root, *SCRATCH[csidl].split("/"))
            os.makedirs(path, exist_ok=True)
            cols[3] = path.encode(ANSI).hex()
            lines[i] = TAB.join(cols)
    return "".join(line + chr(10) for line in lines)


def player_folder(table_text, csidl):
    """The folder the table names for `csidl` (the machine's own), or None."""
    for _, cols in folder_rows(table_text):
        if cols[1].split(" ")[0].lower() == csidl:
            return bytes.fromhex(cols[3]).decode(ANSI)
    return None


def write_load_order(root, plugins, player_local):
    """Writes the instance's `plugins.txt` (the active plugins): the names given, else a copy of
    the player's file. Returns the number of plugins named."""
    if plugins:
        data = "".join(name + chr(13) + chr(10) for name in plugins).encode(ANSI)
    else:
        source = os.path.join(player_local or "", "FalloutNV", "plugins.txt")
        if not os.path.isfile(source):
            return 0
        with open(source, "rb") as fh:
            data = fh.read()
    target = os.path.join(root, "AppData", "Local", "FalloutNV")
    os.makedirs(target, exist_ok=True)
    with open(os.path.join(target, "plugins.txt"), "wb") as fh:
        fh.write(data)
    return sum(1 for line in data.splitlines() if line.strip() and not line.startswith(b"#"))


def windowed(ini_text, width, height):
    """The settings with a window of this size: three keys of the game's [Display] section."""
    wanted = {"bfull screen": "0", "isize w": str(width), "isize h": str(height)}
    out = []
    for line in ini_text.splitlines(keepends=True):
        key = line.split("=", 1)[0].strip().lower()
        if "=" in line and key in wanted:
            body = line.rstrip(chr(13) + chr(10))
            line = "%s=%s%s" % (body.split("=", 1)[0], wanted[key], line[len(body):])
        out.append(line)
    return "".join(out)


def copy_settings(player_docs, root, width, height):
    """Copies the player's two settings files into the instance, as a window. Names the copies."""
    copied = []
    for name in ("Fallout.ini", "FalloutPrefs.ini"):
        source = os.path.join(player_docs, *GAME_FOLDER, name)
        if not os.path.isfile(source):
            continue
        with open(source, encoding=ANSI, newline="") as fh:
            text = fh.read()
        target = os.path.join(root, "Documents", *GAME_FOLDER, name)
        with open(target, "w", encoding=ANSI, newline="") as fh:
            fh.write(windowed(text, width, height))
        copied.append(name)
    return copied


def main(argv):
    if len(argv) < 2:
        sys.exit(__doc__)
    root = os.path.abspath(argv[1])
    size = (1280, 720)
    if "--size" in argv:
        at = argv.index("--size")
        size = (int(argv[at + 1]), int(argv[at + 2]))
    table = os.path.join(os.environ.get("LOCALAPPDATA", ""), "nv-rs", "d3d9_adapter.tsv")
    if "--table" in argv:
        table = argv[argv.index("--table") + 1]
    with open(table, encoding="utf-8", newline="") as fh:
        text = fh.read()
    os.makedirs(os.path.join(root, "Documents", *GAME_FOLDER, "Saves"), exist_ok=True)
    out = os.path.join(root, "machine.tsv")
    with open(out, "w", encoding="utf-8", newline="") as fh:
        fh.write(instance_table(text, root))
    docs = player_folder(text, "5")
    copied = copy_settings(docs, root, *size) if docs else []
    plugins = []
    if "--plugins" in argv:
        plugins = [w for w in argv[argv.index("--plugins") + 1].split(",") if w]
    if write_load_order(root, plugins, player_folder(text, "1c")) == 0:
        print("warning: plugins.txt names no plugin (the game loads its main file only)",
              file=sys.stderr)
    if len(copied) != 2:
        print("warning: settings copied: %s (the game asks for its launcher without both)" % copied,
              file=sys.stderr)
    print(out)


if __name__ == "__main__":
    main(sys.argv)
