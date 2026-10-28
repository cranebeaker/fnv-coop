"""Steam off Windows (SteamOS, a Steam Deck, any Linux): where Fallout: New Vegas and its Proton prefix are.

On Windows the tools ask Windows for Documents (mygames.shell_documents) and use the Steam folder in Program Files. Off
Windows neither works, so provision needed FNV_GAME_DIR and FNV_DOCUMENTS_DIR. This finds both the way Steam lays them
out: the Steam roots under the home folder, every library their steamapps/libraryfolders.vdf lists, the game's
appmanifest_22380.acf in one of those libraries, and the game's Proton prefix in the same library's
steamapps/compatdata/22380 (Steam keeps the prefix in the library the game is installed in). Under Proton the game reads
its INIs and saves from the prefix's drive_c/users/steamuser/Documents, so that is the Documents folder for My Games.

Read-only. The environment overrides still win: instances.game_dir() and mygames.documents_dir() only ask here when
FNV_GAME_DIR / FNV_DOCUMENTS_DIR are not set.
"""
from __future__ import annotations

import os
import sys
from pathlib import Path
from typing import Iterable, List, Optional

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "installer"))
import fnv_find  # noqa: E402  (the libraryfolders.vdf and appmanifest parsers the installer uses)

OFF_WINDOWS = os.name != "nt"   # the lookups below run only off Windows; tests switch this
APPID = "22380"
# Where Steam keeps itself under the home folder: the native client (SteamOS and most distributions, also reachable
# through the ~/.steam links) and the Flatpak.
ROOTS = (".steam/steam", ".steam/root", ".local/share/Steam", ".var/app/com.valvesoftware.Steam/.local/share/Steam")
PREFIX_DOCUMENTS = ("pfx", "drive_c", "users", "steamuser", "Documents")


def _read(p: Path) -> str:
    """A Steam text file, or "" when it can't be read. Steam on Linux writes UTF-8."""
    try:
        return p.read_text(encoding="utf-8-sig", errors="replace")
    except OSError:
        return ""


def _unique_dirs(paths: Iterable[Path]) -> List[Path]:
    """Existing folders in order, each once (~/.steam/steam is usually a link to ~/.local/share/Steam)."""
    out: List[Path] = []
    seen = set()
    for p in paths:
        if not p.is_dir():
            continue
        key = os.path.realpath(p)
        if key not in seen:
            seen.add(key)
            out.append(p)
    return out


def steam_roots(home: Optional[Path] = None) -> List[Path]:
    home = Path.home() if home is None else Path(home)
    return _unique_dirs(home / r for r in ROOTS)


def libraries(home: Optional[Path] = None) -> List[Path]:
    """Every Steam library: the roots themselves and the folders their libraryfolders.vdf lists (an SD card, a second disk)."""
    libs: List[Path] = []
    for root in steam_roots(home):
        libs.append(root)
        libs += [Path(p) for p in fnv_find.parse_libraryfolders(_read(root / "steamapps" / "libraryfolders.vdf"))]
    return _unique_dirs(libs)


def find_game(home: Optional[Path] = None) -> Optional[Path]:
    """The Steam install of the game (the folder with FalloutNV.exe), or None. Every library's appmanifest first: a moved
    game can leave an old "Fallout New Vegas" folder in an earlier library, and only the manifest names the current one.
    The usual folder names count only when no manifest names an existing game."""
    libs = libraries(home)
    for lib in libs:
        inst = fnv_find.parse_appmanifest_installdir(_read(lib / "steamapps" / f"appmanifest_{APPID}.acf"))
        g = lib / "steamapps" / "common" / inst if inst else None
        if g and (g / fnv_find.GAME_EXE).is_file():
            return g
    for lib in libs:
        for name in fnv_find.STEAM_NAMES:
            g = lib / "steamapps" / "common" / name
            if (g / fnv_find.GAME_EXE).is_file():
                return g
    return None


def prefix_dir(game: Path) -> Path:
    """The game's Proton prefix: <library>/steamapps/compatdata/22380/pfx (the library the game is installed in)."""
    return Path(game).parent.parent / "compatdata" / APPID / PREFIX_DOCUMENTS[0]


def prefix_documents(game: Path) -> Path:
    """The Documents folder of the game's Proton prefix: <library>/steamapps/compatdata/22380/pfx/.../Documents."""
    return prefix_dir(game) / Path(*PREFIX_DOCUMENTS[1:])


def find_documents(home: Optional[Path] = None, game: Optional[Path] = None) -> Optional[Path]:
    """The Documents folder of the game's Proton prefix, or None (no Steam install found, or the game never ran under
    Proton, so the prefix doesn't exist yet). `game` (FNV_GAME_DIR): use that install's prefix only, never another
    install's INIs."""
    g = Path(game) if game else find_game(home)
    if g is None:
        return None
    d = prefix_documents(g)
    return d if d.is_dir() else None


def main(argv: Optional[List[str]] = None) -> int:
    """Print one path, for shell commands that need it (docs/tools/proton-deck.md): `game` (the game folder), `prefix`
    (its Proton prefix, pfx) or `documents` (the Documents provision reads: FNV_DOCUMENTS_DIR when set, else the
    prefix's). FNV_GAME_DIR, when set, is the game."""
    import argparse
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("what", choices=("game", "prefix", "documents"))
    a = ap.parse_args(argv)
    if a.what == "documents" and os.environ.get("FNV_DOCUMENTS_DIR", "").strip():
        sys.path.insert(0, str(Path(__file__).resolve().parent))   # python -I leaves the script's folder off the path
        import mygames   # the same override parsing as mygames.documents_dir(), which provision uses
        try:
            path = mygames.normalize_override(os.environ["FNV_DOCUMENTS_DIR"])
        except ValueError as e:
            print(f"steamlib: {e}", file=sys.stderr)
            return 1
        if not path.is_dir():
            print(f"steamlib: FNV_DOCUMENTS_DIR {path} doesn't exist", file=sys.stderr)
            return 1
        print(path)
        return 0
    game = Path(os.environ["FNV_GAME_DIR"]) if os.environ.get("FNV_GAME_DIR") else find_game()
    if game is None:
        print("steamlib: no Steam install of Fallout: New Vegas under this home folder; set FNV_GAME_DIR", file=sys.stderr)
        return 1
    path = {"game": game, "prefix": prefix_dir(game), "documents": prefix_documents(game)}[a.what]
    if not path.is_dir():
        print(f"steamlib: {path} doesn't exist; start the game once through Steam", file=sys.stderr)
        return 1
    print(path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
