"""Party saves, launcher side: which saves can a campaign be continued from, and which save of a guest belongs to a host save.

The design is docs/party-save.md. The plugin (plugin/coop/party_save.h) writes two kinds of small text files, and this module only reads them:

  <Saves>\\<save>.fos.fnvparty         next to a save made in a co-op session
        FNVCOOP-PARTY 1
        campaign=<16 hex digits>        the campaign (one per party that started from the start save)
        role=host|guest
        hostSave=<name>                 the host save this save belongs to (a host save names itself)
  <game>\\Data\\NVSE\\Plugins\\nvcoop_party_index.txt      a guest's index of the saves it made for a host save
        FNVCOOP-PARTY-INDEX 1
        <campaign>\\t<host save>\\t<guest save>\\t<unix time>        one line per (campaign, host save); the newest write wins

Save names are file names without the .fos (what the console's LoadGame takes). Standard library only.
"""
import ctypes
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Dict, List, Optional

_HERE = os.path.dirname(os.path.abspath(__file__))
for _d in (_HERE, os.path.join(os.path.dirname(_HERE), "installer")):
    if _d not in sys.path:
        sys.path.insert(0, _d)
import fnv_ini  # noqa: E402
import fnv_mut  # noqa: E402

SIDECAR_EXT = ".fnvparty"
SIDECAR_HEADER = "FNVCOOP-PARTY 1"
INDEX_REL = ("Data", "NVSE", "Plugins", "nvcoop_party_index.txt")
INDEX_HEADER = "FNVCOOP-PARTY-INDEX 1"
MAX_SAVE_NAME = 150          # the ini value and the console line both stay short
_CAMPAIGN = re.compile(r"^[0-9a-f]{16}$")
_BAD_NAME = re.compile(r'[\x00-\x1f\x7f\\/:*?"<>|]')


def norm_campaign(text) -> str:
    """The campaign id in canonical form (16 lower-case hex digits), or "" when it is not one (all zero is not one either)."""
    t = str(text or "").strip().lower()
    return t if _CAMPAIGN.match(t) and t != "0" * 16 else ""


def short_campaign(campaign: str) -> str:
    return campaign[:6]


def valid_save_name(name) -> bool:
    """A save name the launcher may put into nvcoop.ini and the plugin into a console line: no path, no quote, no edge spaces."""
    return (isinstance(name, str) and 0 < len(name) <= MAX_SAVE_NAME and name == name.strip() and not _BAD_NAME.search(name)
            and not name.lower().endswith(".fos") and _ini_safe(name))


def _ini_safe(name: str) -> bool:
    try:
        name.encode(fnv_ini.ini_encoding())
    except UnicodeEncodeError:
        return False
    return True


# ---- where the game keeps its saves (the same rule as the plugin: plugin/main.cpp GameSaveDir) -------------------------------------------

def documents_dir() -> str:
    """Where the game keeps My Games: FNV_DOCUMENTS_DIR (tests, unusual setups), else the shell Documents folder (which Windows may
    redirect to OneDrive), the home folder only as the last fallback. The same order as tools/runner/mygames.py."""
    env = os.environ.get("FNV_DOCUMENTS_DIR", "").strip()
    if env:
        return str(Path(os.path.expandvars(os.path.expanduser(env))).resolve())
    if sys.platform == "win32":
        buf = ctypes.create_unicode_buffer(260)
        try:
            if ctypes.windll.shell32.SHGetFolderPathW(None, 5, None, 0, buf) == 0 and buf.value:   # CSIDL_PERSONAL
                return buf.value
        except (OSError, AttributeError):
            pass
    return str(Path.home() / "Documents")


def game_save_dir(game, documents: Optional[str] = None) -> Path:
    """The folder the game saves into: the Fallout.ini it reads (My Games first, else the one next to the exe) says
    bUseMyGamesDirectory (default 1: <Documents>\\My Games\\FalloutNV\\) and SLocalSavePath (default Saves\\)."""
    game = Path(game)
    my_games = Path(documents if documents is not None else documents_dir()) / "My Games" / "FalloutNV"
    ini = my_games / "Fallout.ini"
    if not ini.is_file():
        ini = game / "Fallout.ini"
    text = fnv_ini.read_text(ini)
    use_my = (fnv_ini.get(text, "General", "bUseMyGamesDirectory") or "1").strip() not in ("0", "")
    local = (fnv_ini.get(text, "General", "SLocalSavePath") or "Saves\\").strip() or "Saves\\"
    return (my_games if use_my else game) / local.replace("\\", os.sep).rstrip("/\\")


# ---- the sidecar next to a save -------------------------------------------------------------------------------------------------------

def parse_meta(text: str) -> Dict[str, str]:
    lines = [ln.strip() for ln in text.splitlines() if ln.strip()]
    if not lines or lines[0] != SIDECAR_HEADER:
        return {}
    out: Dict[str, str] = {}
    for ln in lines[1:]:
        k, eq, v = ln.partition("=")
        if eq:
            out[k.strip()] = v.strip()
    return out


@dataclass
class HostSave:
    name: str
    campaign: str
    mtime: float


def list_host_saves(saves_dir) -> List[HostSave]:
    """The host's co-op saves, newest first: every .fos that has a sidecar with a campaign id and role=host."""
    out: List[HostSave] = []
    d = Path(saves_dir)
    try:
        sidecars = list(d.glob("*.fos" + SIDECAR_EXT))
    except OSError:
        return out
    for sc in sidecars:
        fos = Path(str(sc)[:-len(SIDECAR_EXT)])
        name = fos.stem
        try:
            meta = parse_meta(fnv_ini.read_text(sc))
            mtime = fos.stat().st_mtime
        except OSError:
            continue   # the save itself is gone
        camp = norm_campaign(meta.get("campaign"))
        if not camp or meta.get("role") != "host" or not valid_save_name(name):
            continue
        out.append(HostSave(name, camp, mtime))
    out.sort(key=lambda s: (-s.mtime, s.name.lower()))
    return out


def latest_host_save(saves_dir, campaign: str = "") -> Optional[HostSave]:
    """Continue's default: the newest co-op save (of one campaign when `campaign` is given)."""
    for s in list_host_saves(saves_dir):
        if not campaign or s.campaign == campaign:
            return s
    return None


# ---- a guest's index ---------------------------------------------------------------------------------------------------------------------

@dataclass
class IndexEntry:
    campaign: str
    host_save: str
    guest_save: str
    time: int


def parse_index(text: str) -> List[IndexEntry]:
    lines = text.splitlines()
    if not lines or lines[0].strip() != INDEX_HEADER:
        return []
    out: List[IndexEntry] = []
    for ln in lines[1:]:
        parts = ln.rstrip("\r\n").split("\t")
        if len(parts) != 4:
            continue
        camp = norm_campaign(parts[0])
        if not camp or not parts[1] or not parts[2] or not parts[3].strip().isdigit():
            continue
        out.append(IndexEntry(camp, parts[1], parts[2], int(parts[3])))
    return out


def read_index(game) -> List[IndexEntry]:
    return parse_index(fnv_ini.read_text(Path(game).joinpath(*INDEX_REL)))


def pick_guest_save(entries: List[IndexEntry], campaign: str, host_save: str,
                    exists: Callable[[str], bool] = lambda name: True) -> Optional[IndexEntry]:
    """The guest save to load when the host continues from `host_save` of `campaign`: the save made for exactly that host save, else the
    newest save of that campaign, else None (the guest then joins with the start save and makes its character again). Entries whose
    save file is gone (`exists` says no) never count."""
    camp = norm_campaign(campaign)
    if not camp:
        return None
    mine = [e for e in entries if (e.campaign == camp or fnv_mut.on("party_pick_any_campaign")) and exists(e.guest_save)]
    if not mine:
        return None
    if not fnv_mut.on("party_pick_newest_only"):
        exact = [e for e in mine if e.host_save.lower() == (host_save or "").lower()]
        if exact:
            return max(exact, key=lambda e: e.time)
    return max(mine, key=lambda e: e.time)


def guest_pick(game, campaign: str, host_save: str, documents: Optional[str] = None) -> Optional[str]:
    """What a guest's launcher does at the lobby's start: the save name to load, or None. Reads this PC's index and save folder."""
    saves = game_save_dir(game, documents)
    e = pick_guest_save(read_index(game), campaign, host_save, lambda n: (saves / (n + ".fos")).is_file())
    return e.guest_save if e and valid_save_name(e.guest_save) else None
