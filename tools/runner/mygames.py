"""Where the game keeps the user's INIs and saves: <Documents>\\My Games\\FalloutNV.

FalloutNV.exe 1.4.0.525 finds Documents with SHGetFolderPathA (it is in the import table), so it follows a redirected
Documents folder (for example OneDrive\\Documents, the Windows 11 default for many accounts). We ask Windows the same
way. Never build the path from the home folder: %USERPROFILE%\\Documents can exist and still be the wrong folder.
Off Windows the game runs under Proton, and its Documents is the one in the game's Proton prefix (steamlib.py).

Environment override: FNV_DOCUMENTS_DIR (the Documents folder itself, for tests and unusual setups).
"""
from __future__ import annotations

import os
from pathlib import Path
from typing import Callable, Optional

CSIDL_PERSONAL = 0x0005  # the "Documents" shell folder
SHGFP_TYPE_CURRENT = 0


def shell_documents() -> Optional[Path]:
    """Documents as Windows reports it now (follows redirection). None off Windows or on error."""
    if os.name != "nt":
        return None
    import ctypes
    from ctypes import wintypes
    buf = ctypes.create_unicode_buffer(wintypes.MAX_PATH)
    try:
        hr = ctypes.windll.shell32.SHGetFolderPathW(None, CSIDL_PERSONAL, None, SHGFP_TYPE_CURRENT, buf)
    except (AttributeError, OSError):
        return None
    return Path(buf.value) if hr == 0 and buf.value else None


def platform_documents() -> Optional[Path]:
    """The Documents folder the game uses on this machine: the shell folder on Windows, the Documents of the game's Proton
    prefix elsewhere (the install in FNV_GAME_DIR first, then the one Steam has). None when there is no answer."""
    import steamlib
    if not steamlib.OFF_WINDOWS:
        return shell_documents()
    game = os.environ.get("FNV_GAME_DIR")
    return steamlib.find_documents(game=Path(game) if game and game.strip() else None)


def normalize_override(value: str) -> Path:
    """An operator-supplied folder: expand ~ and %VARS%. It must then be absolute: a relative path would select a
    different folder for each working directory a tool runs from, so it is refused instead of resolved."""
    p = Path(os.path.expandvars(os.path.expanduser(value.strip())))
    if not p.is_absolute():
        raise ValueError(f"FNV_DOCUMENTS_DIR must be an absolute path (after ~ and %VAR% expansion), got {value!r}")
    return p.resolve()


def documents_dir(resolver: Callable[[], Optional[Path]] = platform_documents) -> Path:
    env = os.environ.get("FNV_DOCUMENTS_DIR")
    if env and env.strip():
        return normalize_override(env)
    found = resolver()
    return found if found else Path.home() / "Documents"


def my_games_dir(resolver: Callable[[], Optional[Path]] = platform_documents) -> Path:
    return documents_dir(resolver) / "My Games" / "FalloutNV"


def saves_dir(resolver: Callable[[], Optional[Path]] = platform_documents) -> Path:
    return my_games_dir(resolver) / "Saves"
