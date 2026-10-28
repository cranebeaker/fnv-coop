#!/usr/bin/env python3
"""Provision isolated Fallout: New Vegas instances that cost ~0 disk (L2).

Each instance lives in <root>/<i>/ and mirrors the read-only Steam install with hardlinks
(fallback: symlinks, then copies). Only the INIs, Saves/, Data/NVSE/Plugins/ and a few
marker files are per-instance, writable, real files. The game dir is only ever READ.

    python instances.py provision --n 2 [--nvse-dir DIR] [--plugin-dll out/L1/nvcoop.dll]
                                  [--core-dll out/L1split/nvcoop_mod.dll] [--ini SECTION.KEY=VALUE ...]
    python instances.py show --n 2
    python instances.py destroy --n 2

provision rewrites Data/NVSE/Plugins/nvcoop.ini every time, so hand edits are lost. Put extra entries on the
command line instead: --ini is repeatable, e.g. --ini coop.enable=1 --ini dev.bots=1 --ini dev.animprobe=1.
One --ini value applies to every instance. So with --n 2 or more, the per-instance keys bridge.instance,
coop.instanceId, coop.pipe and coop.role are an error (exit 2). provision sets them for each instance.
nvcoop.ini is in the ANSI code page (the plugin reads it with GetPrivateProfileStringA). An --ini entry that
the ANSI code page cannot encode is an error (exit 2).
--plugin-dll not given: out/L1/nvcoop.dll if that file exists, else no plugin. --plugin-dll "": no plugin.
A --plugin-dll or --core-dll path that does not exist is an error (exit 2). --core-dll installs the split
build's reloadable core next to the loader and writes [loader] core=<its file name> (docs/tools/T3a-loader.md).
--core-dll needs an explicit --plugin-dll (the split loader), else it is an error (exit 2). The default
out/L1/nvcoop.dll can be the monolith, which never reads [loader] core.

--pairs N provisions N two-copy pairs at once (2N instances) so that several pairs can run on one PC: pair k is
instances 2k (host) and 2k+1 (client) on co-op port <port-base>+k (default 27960+k, 127.0.0.1), see pairs.py.
--first-pair K (with --pairs N) provisions only pairs K..N-1, so running copies of the earlier pairs stay untouched.
Without --pairs the port is not written and every client joins the plugin's default port, as before. With --pairs,
--ini coop.port / coop.host are an error (exit 2), and --n must match or be left out. show and destroy take --pairs too.

--no-pad writes [Interface] bDisable360Controller=1 into each copy's FalloutPrefs.ini, so the copies ignore game
controllers. Use it where a pad always answers (a Steam Deck: Steam Input shows its controls as one) and bots or the
input probe have to walk: in gamepad mode the game ignores the keyboard movement controls that bot.move holds.

Environment overrides: FNV_GAME_DIR, FNV_INSTANCES_ROOT.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import locale
import os
import re
import shutil
import sys
from pathlib import Path
from typing import Dict, List, Optional

sys.path.insert(0, str(Path(__file__).resolve().parent))
import mygames  # noqa: E402
import pairs  # noqa: E402

DEFAULT_GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Fallout New Vegas"
DEFAULT_ROOT = str(Path(__file__).resolve().parents[3] / "fnv-instances")
STEAM_APPID = "22380"

# files that must be real, per-instance (the game writes them)
PER_INSTANCE = {"fallout.ini", "falloutprefs.ini", "rendererinfo.txt", "steam_appid.txt"}
SKIP_DIRS = {"saves", "screenshots"}  # created empty per instance
PINNED_FILES = ["FalloutNV.exe", "Data/FalloutNV.esm"]

LINK_MODES = ("hardlink", "symlink", "copy")


def game_dir() -> Path:
    return Path(os.environ.get("FNV_GAME_DIR", DEFAULT_GAME))


def instances_root() -> Path:
    return Path(os.environ.get("FNV_INSTANCES_ROOT", DEFAULT_ROOT))


def instance_dir(i: int, root: Optional[Path] = None) -> Path:
    return Path(root or instances_root()) / str(int(i))


def sha256_file(p: Path, chunk: int = 1 << 20) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        while True:
            b = f.read(chunk)
            if not b:
                break
            h.update(b)
    return h.hexdigest()


# ---------------------------------------------------------------- INI editing
def ini_set(text: str, section: str, key: str, value: str) -> str:
    """Set key in [section] (case-insensitive), adding section/key if missing. Keeps CRLF."""
    nl = "\r\n" if "\r\n" in text else "\n"
    lines = text.split(nl) if text else []
    sec_re = re.compile(r"^\s*\[(.+?)\]\s*$")
    key_re = re.compile(r"^\s*" + re.escape(key) + r"\s*=", re.I)
    cur = None
    sec_end = None  # index after last non-blank line of target section
    for idx, ln in enumerate(lines):
        m = sec_re.match(ln)
        if m:
            if cur is not None and cur.lower() == section.lower():
                break
            cur = m.group(1)
            if cur.lower() == section.lower():
                sec_end = idx + 1
            continue
        if cur is not None and cur.lower() == section.lower():
            if key_re.match(ln):
                lines[idx] = f"{key}={value}"
                return nl.join(lines)
            if ln.strip():
                sec_end = idx + 1
    if sec_end is None:
        if lines and lines[-1].strip():
            lines.append("")
        lines += [f"[{section}]", f"{key}={value}"]
    else:
        lines.insert(sec_end, f"{key}={value}")
    return nl.join(lines)


def ini_get(text: str, section: str, key: str) -> Optional[str]:
    cur = None
    for ln in text.splitlines():
        m = re.match(r"^\s*\[(.+?)\]\s*$", ln)
        if m:
            cur = m.group(1)
            continue
        if cur and cur.lower() == section.lower():
            m = re.match(r"^\s*" + re.escape(key) + r"\s*=\s*(.*?)\s*$", ln, re.I)
            if m:
                return m.group(1)
    return None


# Fallout.ini (read from the game dir because [General] bUseMyGamesDirectory=0; the string
# "bUseMyGamesDirectory:General" is at file offset 0xc81cc0 of FalloutNV.exe 1.4.0.525)
FALLOUT_INI_EDITS = [
    ("General", "bUseMyGamesDirectory", "0"),   # INIs + Saves live in the instance dir
    ("General", "bAlwaysActive", "1"),          # keep ticking when the window is unfocused
    ("General", "SLocalSavePath", "Saves\\"),
    ("General", "SLocalMasterPath", "Data\\"),
    ("Controls", "bBackground Mouse", "1"),
    ("Controls", "bBackground Keyboard", "1"),
]
PREFS_EDITS = [
    ("Display", "bFull Screen", "0"),
    ("Display", "iSize W", "1280"),   # not 640x480: measured 125 ms/frame (8 fps, GPU allocation churn in the AMD driver) vs 15 ms at 1280x720 in a window
    ("Display", "iSize H", "720"),
    ("Display", "iPresentInterval", "0"),
    ("Display", "iMultiSample", "0"),
]
# --no-pad. Gamepad mode = an XInput pad answered and bDisable360Controller:Interface is 0 (docs/tools/T6a-actions.md); in it
# the game takes movement from the pad and ignores the keyboard controls bot.move holds (plugin/bots). Under Proton on a Steam
# Deck (2026-10-25), where Steam Input always answers as a pad, a 3 s bot.move forward moved the player 0 units; with this
# key set to 1 the same move went 1008 units. The key is in FalloutPrefs.ini, which the game reads after Fallout.ini.
NO_PAD_EDITS = [
    ("Interface", "bDisable360Controller", "1"),
]


class PrefsUnavailable(RuntimeError):
    pass


def display_adapter_name() -> Optional[str]:
    """The primary display adapter's name (EnumDisplayDevicesW), which is what the game's launcher writes as sD3DDevice."""
    if os.environ.get("NVCOOP_ADAPTER"):   # a remote (ssh) session cannot enumerate displays: the name is given instead
        return os.environ["NVCOOP_ADAPTER"]
    if os.name != "nt":
        return None
    import ctypes
    from ctypes import wintypes

    class DisplayDevice(ctypes.Structure):
        _fields_ = [("cb", wintypes.DWORD), ("DeviceName", wintypes.WCHAR * 32), ("DeviceString", wintypes.WCHAR * 128),
                    ("StateFlags", wintypes.DWORD), ("DeviceID", wintypes.WCHAR * 128), ("DeviceKey", wintypes.WCHAR * 128)]

    first, i = None, 0
    while True:
        dd = DisplayDevice()
        dd.cb = ctypes.sizeof(dd)
        if not ctypes.windll.user32.EnumDisplayDevicesW(None, i, ctypes.byref(dd), 0):
            break
        if dd.DeviceString and first is None:
            first = dd.DeviceString
        if dd.StateFlags & 0x4:  # DISPLAY_DEVICE_PRIMARY_DEVICE
            return dd.DeviceString
        i += 1
    return first


def default_prefs(adapter=None) -> str:
    """FalloutPrefs.ini for an owner who has none (the game never ran here): Fallout_default.ini, the game's own low.ini
    preset over it, and sD3DDevice. FalloutNV.exe 1.4.0.525 starts FalloutNVLauncher.exe instead of the game when the
    prefs have no sD3DDevice (checked in game on 2026-10-21: a 6-line prefs file and a defaults-only one both opened the
    launcher on the owner's desktop; the same defaults plus sD3DDevice started the game)."""
    g = game_dir()
    d = g / "Fallout_default.ini"
    text = d.read_text(encoding="latin-1") if d.exists() else ""
    low = g / "low.ini"
    if low.exists():
        sec = None
        for line in low.read_text(encoding="latin-1").splitlines():
            s = line.strip()
            if s.startswith("[") and s.endswith("]"):
                sec = s[1:-1]
            elif "=" in s and sec and not s.startswith(";"):
                k, v = s.split("=", 1)
                text = ini_set(text, sec, k.strip(), v.strip())
    name = (adapter or display_adapter_name)()
    if not name:
        raise PrefsUnavailable("no FalloutPrefs.ini in My Games and the display adapter name is unknown: start Fallout: New "
                               "Vegas once through Steam so its launcher writes FalloutPrefs.ini, then provision again")
    return ini_set(text, "Display", "sD3DDevice", f'"{name}"')


def _base_ini(name: str) -> str:
    """Start from the user's My Games ini (has the DLC archive list etc.), else the game default."""
    mg = mygames.my_games_dir() / name
    if mg.exists():
        return mg.read_text(encoding="latin-1")
    if name.lower() == "fallout.ini":
        d = game_dir() / "Fallout_default.ini"
        if d.exists():
            return d.read_text(encoding="latin-1")
    if name.lower() == "falloutprefs.ini":
        return default_prefs()
    return ""


def render_inis(no_pad: bool = False) -> Dict[str, str]:
    fo = _base_ini("Fallout.ini")
    play = os.environ.get("NVCOOP_PLAY") == "1"   # a person plays: only the focused copy may read the keyboard and mouse
    for s, k, v in FALLOUT_INI_EDITS:
        if play and k in ("bBackground Mouse", "bBackground Keyboard"):
            v = "0"
        fo = ini_set(fo, s, k, v)
    pr = _base_ini("FalloutPrefs.ini")
    for s, k, v in PREFS_EDITS + (NO_PAD_EDITS if no_pad else []):
        pr = ini_set(pr, s, k, v)
    if os.environ.get("NVCOOP_LEAN") == "1":
        fo, pr = _apply_lean(fo, pr)
    return {"Fallout.ini": fo, "FalloutPrefs.ini": pr}


# NVCOOP_LEAN=1: the pool's low-cost profile (shadows, grass, water, effects off, no sound) for a machine short of RAM. Not applied: the window
# size and the frame cap (a smaller window measured slower here, and a cap would distort fps scores) and keys whose effect is only a guess.
LEAN_SKIP = {"iSize W", "iSize H", "iFPSClamp", "iPresentInterval", "iMultiSample", "bAllow30Shaders", "iTexMipMapMinimum", "bUseRefractionShader",
             "bFXAA", "iMaxAnisotropy", "bTransparencyMultisampling"}


def _apply_lean(fo: str, pr: str):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "pool"))
    import lowcost
    texts = {lowcost.FO: fo, lowcost.PR: pr}
    for r in lowcost.rows(audio_off=True):
        if r.key not in LEAN_SKIP:
            texts[r.file] = ini_set(texts[r.file], r.section, r.key, r.value)
    return texts[lowcost.FO], texts[lowcost.PR]


# ---------------------------------------------------------------- linking
def _link(src: Path, dst: Path, modes=LINK_MODES) -> str:
    last = None
    for m in modes:
        try:
            if m == "hardlink":
                os.link(src, dst)
            elif m == "symlink":
                os.symlink(src, dst)
            else:
                shutil.copy2(src, dst)
            return m
        except OSError as e:
            last = e
    raise OSError(f"cannot link {src} -> {dst}: {last}")


def mirror_tree(src_root: Path, dst_root: Path, modes=LINK_MODES) -> Dict[str, int]:
    """Link every file of src_root into dst_root (real dirs). Idempotent. Counts per mode."""
    counts: Dict[str, int] = {m: 0 for m in LINK_MODES}
    counts["existing"] = 0
    for dirpath, dirnames, filenames in os.walk(src_root):
        rel = Path(dirpath).relative_to(src_root)
        dirnames[:] = [d for d in dirnames if not (rel == Path(".") and d.lower() in SKIP_DIRS)]
        out = dst_root / rel
        out.mkdir(parents=True, exist_ok=True)
        for fn in filenames:
            if rel == Path(".") and fn.lower() in PER_INSTANCE:
                continue
            if rel == Path(".") and fn.lower().endswith((".log", ".dmp", ".mdmp")):
                continue  # runtime output of the main install: never share (writes would go through the link)
            if [x.lower() for x in rel.parts[:3]] == ["data", "nvse", "plugins"]:
                continue  # plugins are per-instance copies (lanes test different dll builds)
            d = out / fn
            if d.exists() or d.is_symlink():
                counts["existing"] += 1
                continue
            counts[_link(Path(dirpath) / fn, d, modes)] += 1
    return counts


def _replace(src: Path, dst: Path) -> None:
    if dst.exists() or dst.is_symlink():
        dst.unlink()
    shutil.copy2(src, dst)


def plugin_plan(plugin_dll: Optional[Path] = None, core_dll: Optional[Path] = None,
                plugin_files: Optional[List[Path]] = None) -> "tuple[List[Path], Dict[str, Dict[str, str]]]":
    """The files to copy into Data/NVSE/Plugins, and the nvcoop.ini entries that they need.

    provision calls this before it writes anything. A named file that does not exist is an error (before, a
    missing --plugin-dll installed nothing and gave no message). Two files with the same name are an error,
    because one would overwrite the other. core_dll is the reloadable core of the split build (nvcoop_mod.dll,
    docs/tools/T3a-loader.md). It keeps its file name, and [loader] core= names it, so the loader finds it."""
    named = [("plugin dll (--plugin-dll)", plugin_dll), ("core dll (--core-dll)", core_dll)]
    named += [("plugin file", f) for f in plugin_files or []]
    files: List[Path] = []
    seen: Dict[str, Path] = {}
    for what, f in named:
        if not f:
            continue
        f = Path(f)
        if not f.is_file():
            raise FileNotFoundError(f"{what} not found: {f}")
        if f.name.lower() in seen:
            raise ValueError(f"{what} {f} has the same file name as {seen[f.name.lower()]}; "
                             "one would overwrite the other in Data/NVSE/Plugins")
        seen[f.name.lower()] = f
        files.append(f)
    ini = {"loader": {"core": Path(core_dll).name}} if core_dll else {}
    return files, ini


class UnsafeInstance(RuntimeError):
    """The instance layout would make provisioning write or delete outside the instance."""


def check_plugins_inside(d: Path) -> None:
    """Refuse when Data, Data/NVSE or Data/NVSE/Plugins of an instance is a symlink or junction, or resolves outside the
    instance. Provisioning deletes stale DLLs and writes nvcoop.ini there; through a linked Data folder that would change
    the game install itself."""
    base = Path(os.path.realpath(d))
    for p in (d / "Data", d / "Data" / "NVSE", d / "Data" / "NVSE" / "Plugins"):
        if not (p.exists() or p.is_symlink()):
            continue
        is_junction = getattr(p, "is_junction", lambda: False)()
        real = Path(os.path.realpath(p))
        if p.is_symlink() or is_junction or not real.is_relative_to(base):
            raise UnsafeInstance(f"{p} is a link (resolves to {real}); refusing to write plugins outside the instance")


def same_file(a: Path, b: Path) -> bool:
    try:
        return a.exists() and b.exists() and os.path.samefile(a, b)
    except OSError:
        return False


def stale_plugin_dlls(plugins: Path, keep: List[Path]) -> List[Path]:
    """nvcoop*.dll files in the instance Plugins folder that this provision does not install.

    xNVSE loads every DLL in Data/NVSE/Plugins. A plugin installed earlier under another file name (nvcoop_focus.dll
    next to nvcoop.dll) therefore runs as a second copy of the plugin: found in game on 2026-10-20, where three copies
    were loaded and an old one answered the bridge. Other DLLs (third-party xNVSE plugins) are not touched."""
    names = {Path(p).name.lower() for p in keep}
    return [f for f in sorted(plugins.glob("*.dll")) if f.name.lower().startswith("nvcoop") and f.name.lower() not in names]


DXVK_CONF = "d3d9.maxFrameRate = 60\n"   # the game's speed follows its frame rate: uncapped DXVK runs it at 100+ fps, i.e. too fast


def deps_dir() -> Path:
    """Cached third-party runtime files (tools/runner/fetch_deps.py fills it): out/cache/deps/{dxvk,nvse}."""
    return Path(os.environ.get("NVCOOP_DEPS_DIR") or Path(__file__).resolve().parents[2] / "out" / "cache" / "deps")


def install_dxvk(d: Path) -> bool:
    """DXVK's 32-bit d3d9.dll next to FalloutNV.exe, plus a 60 fps cap. Measured on an AMD RX 6600: under NVSE the game's own D3D9 path
    ran exteriors at about 6 fps with props and foliage jumping (the main thread sat in the driver creating and destroying GPU
    allocations); with DXVK the same copy runs at 60+ fps. Returns False when the cached dll is missing or NVCOOP_NO_DXVK=1."""
    if os.environ.get("NVCOOP_NO_DXVK") == "1":
        return False
    src = deps_dir() / "dxvk" / "d3d9.dll"
    if not src.is_file():
        return False
    _replace(src, d / "d3d9.dll")
    (d / "dxvk.conf").write_text(DXVK_CONF, encoding="ascii", newline="")
    return True


def provision(i: int, *, role: str = "host", nvse_dir: Optional[Path] = None,
              plugin_dll: Optional[Path] = None, plugin_files: Optional[List[Path]] = None,
              autoload: str = "", autoload_delay_frames: int = 600, modes=LINK_MODES,
              root: Optional[Path] = None, appid: Optional[str] = STEAM_APPID,
              extra_ini: Optional[Dict[str, Dict[str, str]]] = None, core_dll: Optional[Path] = None,
              port_base: Optional[int] = None, dxvk: bool = False, no_pad: bool = False) -> Dict:
    # Check every input before the first write: a missing dll or a bad ini entry must not leave a half instance.
    sources, plan_ini = plugin_plan(plugin_dll, core_dll, plugin_files)
    ini_extra = provision_ini(plan_ini, extra_ini)
    if port_base is not None:  # pair mode: instance i joins pair i//2 on its own port (pairs.py); role follows from i
        pairs.check_pair_ini(extra_ini)
        role = pairs.role_of(i)
        merge_ini(ini_extra, pairs.pair_ini(i, port_base))
    check_ini_entry("bridge", "autoload", autoload)  # autoload goes into nvcoop.ini too
    g = game_dir()
    if not (g / "FalloutNV.exe").exists():
        raise FileNotFoundError(f"game not found at {g}")
    d = instance_dir(i, root)
    d.mkdir(parents=True, exist_ok=True)
    plugins = d / "Data" / "NVSE" / "Plugins"
    check_plugins_inside(d)  # before mirror_tree: it must not write through a linked Data folder either
    counts = mirror_tree(g, d, modes)
    (d / "Saves").mkdir(exist_ok=True)
    plugins.mkdir(parents=True, exist_ok=True)
    check_plugins_inside(d)  # and again before anything is deleted from or written into Plugins
    for name, text in render_inis(no_pad).items():
        (d / name).write_text(text, encoding="latin-1", newline="")
    if appid:
        (d / "steam_appid.txt").write_text(appid, encoding="ascii")
    installed: List[str] = []
    if nvse_dir:  # xNVSE: loader + dlls at the instance root
        for f in Path(nvse_dir).iterdir():
            if f.is_file() and f.suffix.lower() in (".exe", ".dll") and f.name.lower().startswith(("nvse", "xnvse")):
                _replace(f, d / f.name)
                installed.append(f.name)
    if dxvk and install_dxvk(d):
        installed += ["d3d9.dll", "dxvk.conf"]
    removed: List[str] = []
    for f in stale_plugin_dlls(plugins, sources):
        f.unlink()
        removed.append("Data/NVSE/Plugins/" + f.name)
    for p in sources:
        if not same_file(p, plugins / p.name):  # a source that is already the installed file must not be deleted
            _replace(p, plugins / p.name)
        installed.append("Data/NVSE/Plugins/" + p.name)
    # The volatile-keys list goes next to nvcoop.dll (the plugin looks there first). The dll no longer bakes in the
    # source-tree path by default, because that path would carry the builder's user name into every built dll.
    vk = Path(__file__).resolve().parents[2] / "plugin" / "snapshot" / "data" / "volatile_keys.txt"
    vk_dst = plugins / "nvcoop_volatile_keys.txt"
    plugin_dlls = [p for p in sources if p.suffix.lower() == ".dll" and p.name.lower().startswith("nvcoop")]
    if plugin_dlls and vk.is_file() and not same_file(vk, vk_dst):
        _replace(vk, vk_dst)
        installed.append("Data/NVSE/Plugins/" + vk_dst.name)
    # The build puts the plugin's data files next to nvcoop.dll (nvcoop_guest_local_run.json: the per-machine scripts guests keep
    # running). Without them a guest blocks every DLC setup script and the session repairs hundreds of effects on each check.
    for dll in plugin_dlls:
        for data in sorted(Path(dll).parent.glob("nvcoop_*")):
            if data.suffix.lower() not in (".json", ".txt") or not data.is_file():
                continue
            dst = plugins / data.name
            if not same_file(data, dst):
                _replace(data, dst)
            if "Data/NVSE/Plugins/" + data.name not in installed:
                installed.append("Data/NVSE/Plugins/" + data.name)
    cfg = write_plugin_ini(d, i, role, autoload, autoload_delay_frames, ini_extra)
    meta = {
        "instance": i, "role": role, "dir": str(d), "pipe": rf"\\.\pipe\nvcoop-{i}",
        "exe": str(d / "FalloutNV.exe"), "link_counts": counts, "installed": installed, "removed": removed,
        "ini": cfg, "game_dir": str(g), "no_pad": no_pad,
        **({"pair": pairs.pair_of(i), "port": int(cfg["coop"].get("port") or 0)} if port_base is not None else {}),
        "pinned": {p: sha256_file(d / p) for p in PINNED_FILES if (d / p).exists()},
    }
    (d / "instance.json").write_text(json.dumps(meta, indent=1))
    return meta


def write_plugin_ini(d: Path, i: int, role: str, autoload: str = "", delay: int = 600,
                     extra: Optional[Dict[str, Dict[str, str]]] = None) -> Dict:
    """nvcoop.ini read by plugin/main.cpp ([bridge] instance/autoload/autoloadDelayFrames); [coop] is ours."""
    visible = os.environ.get("NVCOOP_VISIBLE") == "1"
    cfg = {
        "bridge": {"instance": str(i), "autoload": autoload, "autoloadDelayFrames": str(delay), "minimized": "0" if visible else "1",
                   "focusGuard": "1", "multiInstance": "1"},
        "coop": {"instanceId": str(i), "role": role, "pipe": rf"\\.\pipe\nvcoop-{i}"},
    }
    merge_ini(cfg, extra)
    text = ""
    for s, kv in cfg.items():
        text += f"[{s}]\n" + "".join(f"{k}={v}\n" for k, v in kv.items()) + "\n"
    pdir = d / "Data" / "NVSE" / "Plugins"
    pdir.mkdir(parents=True, exist_ok=True)
    ini = pdir / "nvcoop.ini"
    if ini.exists() or ini.is_symlink():
        ini.unlink()  # never write through a hardlink into the game dir
    ini.write_text(text, encoding=ini_encoding())  # check_ini_entry checks with the same encoding
    return cfg


INI_FORM = "SECTION.KEY=VALUE"


def ini_encoding() -> str:
    """The encoding of nvcoop.ini: the ANSI code page, because the plugin reads the file with
    GetPrivateProfileStringA. write_plugin_ini writes with this encoding and check_ini_entry checks with it."""
    getencoding = getattr(locale, "getencoding", None)  # Python 3.11+: the ANSI code page, also in UTF-8 mode
    return getencoding() if getencoding else locale.getpreferredencoding(False)


def check_ini_encodable(where: str, what: str, text: str) -> None:
    """Raise ValueError if ini_encoding() cannot encode text. Without this check, write_plugin_ini fails with
    UnicodeEncodeError after provision has copied the dlls, and it leaves a 0-byte nvcoop.ini."""
    enc = ini_encoding()
    try:
        str(text).encode(enc)
    except UnicodeEncodeError:
        raise ValueError(f"{where}: the {what} {text!r} has characters that the ANSI code page ({enc}) cannot "
                         "encode, and the plugin reads nvcoop.ini in that code page") from None


def check_ini_entry(section: str, key: Optional[str] = None, value: str = "") -> None:
    """Raise ValueError if the entry cannot be written as one line under one [section] of nvcoop.ini, in the
    encoding of write_plugin_ini. With key None, check only the section name."""
    where = f"{section}.{key}" if key is not None else f"[{section}]"
    if not str(section).strip() or any(c in str(section) for c in "[]\r\n\0"):
        raise ValueError(f"{where}: bad section name {section!r} (it is empty or contains [ ] or a line break)")
    check_ini_encodable(where, "section name", section)
    if key is None:
        return
    if not str(key).strip() or any(c in str(key) for c in "=[]\r\n\0") or str(key).lstrip()[:1] in (";", "#"):
        raise ValueError(f"{where}: bad key {key!r} (it is empty, contains = [ ] or a line break, or starts a comment)")
    check_ini_encodable(where, "key", key)
    if any(c in str(value) for c in "\r\n\0"):
        raise ValueError(f"{where}: the value {value!r} contains a line break")
    check_ini_encodable(where, "value", value)


def parse_ini_override(spec: str) -> "tuple[str, str, str]":
    """Parse one --ini SECTION.KEY=VALUE into (section, key, value). VALUE can be empty and can contain '='.
    Raise ValueError with the reason if the text is malformed."""
    if "=" not in spec:
        raise ValueError(f"{spec!r}: expected {INI_FORM}, the '=' is missing")
    name, value = spec.split("=", 1)
    if "." not in name:
        raise ValueError(f"{spec!r}: expected {INI_FORM}, the '.' between section and key is missing")
    section, key = name.split(".", 1)
    section, key, value = section.strip(), key.strip(), value.strip()
    check_ini_entry(section, key, value)
    return section, key, value


def ini_arg(spec: str) -> str:
    """argparse type for --ini (instances.py and run.py): reject a malformed entry with a clear message."""
    try:
        parse_ini_override(spec)
    except ValueError as e:
        raise argparse.ArgumentTypeError(str(e)) from None
    return spec


def ini_overrides(specs: Optional[List[str]]) -> Dict[str, Dict[str, str]]:
    """--ini SECTION.KEY=VALUE strings -> the extra_ini dict of provision(). A later entry wins."""
    out: Dict[str, Dict[str, str]] = {}
    for spec in specs or []:
        s, k, v = parse_ini_override(spec)
        merge_ini(out, {s: {k: v}})
    return out


def merge_ini(cfg: Dict[str, Dict[str, str]], extra: Optional[Dict[str, Dict[str, str]]]) -> Dict[str, Dict[str, str]]:
    """Merge extra into cfg in place and return cfg. Each entry is checked with check_ini_entry.

    Section and key names match case-insensitively, as in GetPrivateProfileString, so an override replaces the
    entry and keeps its first spelling. A case-sensitive merge wrote 'Role=client' after 'role=host'. The plugin
    modules then disagree: GetPrivateProfileString reads the first entry, and the coop parser reads the last."""
    for s, kv in (extra or {}).items():
        check_ini_entry(s)
        sec = next((x for x in cfg if x.lower() == str(s).lower()), str(s))
        dst = cfg.setdefault(sec, {})
        for k, v in kv.items():
            check_ini_entry(sec, k, v)
            key = next((x for x in dst if x.lower() == str(k).lower()), str(k))
            dst[key] = str(v)
    return cfg


def provision_ini(plan_ini: Optional[Dict[str, Dict[str, str]]],
                  extra_ini: Optional[Dict[str, Dict[str, str]]]) -> Dict[str, Dict[str, str]]:
    """The extra nvcoop.ini entries of provision(): the entries of plugin_plan, then extra_ini over them.
    So an explicit entry (--ini loader.core=...) wins over the [loader] core that --core-dll writes."""
    return merge_ini(merge_ini({}, plan_ini), extra_ini)


# Keys that provision sets to a different value in each instance (write_plugin_ini). Lower case.
PER_INSTANCE_INI_KEYS = (("bridge", "instance"), ("coop", "instanceid"), ("coop", "pipe"), ("coop", "role"))


def check_per_instance_ini(extra_ini: Optional[Dict[str, Dict[str, str]]], n: int) -> None:
    """Raise ValueError if extra_ini sets a per-instance key and n > 1. One --ini value applies to every
    instance, so all n instances would get the same instance number, pipe or role."""
    if n <= 1:
        return
    for s, kv in (extra_ini or {}).items():
        for k in kv:
            if (str(s).lower(), str(k).lower()) in PER_INSTANCE_INI_KEYS:
                raise ValueError(f"--ini {s}.{k}: this is a per-instance key, and one --ini value applies to all "
                                 f"{n} instances. Remove it (provision sets it for each instance), or use --n 1")


def load_meta(i: int, root: Optional[Path] = None) -> Dict:
    return json.loads((instance_dir(i, root) / "instance.json").read_text())


def destroy(i: int, root: Optional[Path] = None) -> None:
    """Remove an instance dir (removes link entries only; originals untouched)."""
    d = instance_dir(i, root)
    if not d.exists():
        return

    def onerr(fn, path, exc):  # read-only attribute on linked files
        os.chmod(path, 0o666)
        fn(path)
    shutil.rmtree(d, onerror=onerr)


def verify_pins(i: int, root: Optional[Path] = None, extra: Optional[List[str]] = None) -> List[str]:
    """Problems: pinned file missing / hash differs from the hash recorded at provisioning."""
    d = instance_dir(i, root)
    meta = load_meta(i, root)
    bad = []
    for rel, h in meta["pinned"].items():
        p = d / rel
        if not p.exists():
            bad.append(f"{rel}: missing")
        elif sha256_file(p) != h:
            bad.append(f"{rel}: hash changed")
    for rel in extra or []:
        if not (d / rel).exists():
            bad.append(f"{rel}: missing")
    return bad


DEFAULT_PLUGIN_DLL = Path(__file__).resolve().parents[2] / "out" / "L1" / "nvcoop.dll"


def plugin_args(plugin_dll: Optional[str], core_dll: Optional[str]) -> "tuple[Optional[Path], Optional[Path]]":
    """--plugin-dll and --core-dll of provision -> (plugin dll, core dll). None means: do not install that file.

    --plugin-dll not given (None): DEFAULT_PLUGIN_DLL if it exists, else no plugin. --plugin-dll "": no plugin.
    --core-dll needs an explicit, non-empty --plugin-dll, else ValueError. The default out/L1/nvcoop.dll can be
    the monolith, which never reads [loader] core, so the core would never load."""
    if core_dll and not plugin_dll:
        raise ValueError("--core-dll needs an explicit --plugin-dll (the split loader), for example "
                         "--plugin-dll out/L1split/nvcoop.dll --core-dll out/L1split/nvcoop_mod.dll. "
                         "The default out/L1/nvcoop.dll can be the monolith, which never reads [loader] core")
    if plugin_dll is None:
        dll = DEFAULT_PLUGIN_DLL if DEFAULT_PLUGIN_DLL.is_file() else None
    else:
        dll = Path(plugin_dll) if plugin_dll else None
    return dll, (Path(core_dll) if core_dll else None)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("provision")
    p.add_argument("--n", type=int, default=None, help="number of instances (default 2); not with --pairs")
    p.add_argument("--pairs", type=int, default=0,
                   help="N two-copy pairs (2N instances): pair k = instances 2k (host) + 2k+1 (client) on port "
                        "--port-base + k, so pairs never talk to each other")
    p.add_argument("--port-base", type=int, default=pairs.PORT_BASE, help="--pairs: co-op port of pair 0 (default 27960)")
    p.add_argument("--first-pair", type=int, default=0, help="--pairs: skip pairs below K (their copies may be running)")
    p.add_argument("--nvse-dir")
    p.add_argument("--plugin-dll", default=None,
                   help="plugin dll to install; a path that does not exist is an error; \"\" means no plugin "
                        "(default: out/L1/nvcoop.dll if it exists, else no plugin)")
    p.add_argument("--core-dll", default=None,
                   help="split build only: the reloadable core (nvcoop_mod.dll) to install next to the loader; "
                        "writes [loader] core=<file name>; needs an explicit --plugin-dll (the split loader)")
    p.add_argument("--ini", action="append", default=[], type=ini_arg, metavar=INI_FORM,
                   help="extra nvcoop.ini entry, repeatable, e.g. --ini coop.enable=1 --ini dev.bots=1; "
                        "with --n 2 or more, the per-instance keys bridge.instance, coop.instanceId, coop.pipe "
                        "and coop.role are an error")
    p.add_argument("--autoload", default="")
    p.add_argument("--no-appid", action="store_true")
    p.add_argument("--no-dxvk", action="store_true", help="do not install the cached DXVK d3d9.dll + 60 fps cap (python tools/runner/fetch_deps.py fills the cache)")
    p.add_argument("--no-pad", action="store_true",
                   help="the copies ignore game controllers (FalloutPrefs.ini bDisable360Controller=1), so bot.move can walk "
                        "where a pad always answers, as on a Steam Deck")
    p.add_argument("--modes", default=",".join(LINK_MODES))
    for c in ("show", "destroy"):
        q = sub.add_parser(c)
        q.add_argument("--n", type=int, default=None)
        q.add_argument("--pairs", type=int, default=0)
    a = ap.parse_args(argv)
    try:
        a.n = pairs.resolve_n(a.n, a.pairs)
        if a.cmd == "provision" and a.pairs:
            pairs.check_port_base(a.port_base, a.pairs)
            if not 0 <= a.first_pair < a.pairs:
                raise ValueError(f"--first-pair {a.first_pair} must be 0..{a.pairs - 1}")
        elif a.cmd == "provision" and a.first_pair:
            raise ValueError("--first-pair needs --pairs")
    except ValueError as e:
        print(f"error: {e}", file=sys.stderr)
        return 2
    if a.cmd == "provision":
        try:  # fail before the first write
            dll, core = plugin_args(a.plugin_dll, a.core_dll)
            _, plan_ini = plugin_plan(dll, core)
            extra_ini = ini_overrides(a.ini)
            check_per_instance_ini(extra_ini, a.n)
            if a.pairs:
                pairs.check_pair_ini(extra_ini)
            provision_ini(plan_ini, extra_ini)  # checks the [loader] core entry too
            check_ini_entry("bridge", "autoload", a.autoload)
        except (OSError, ValueError) as e:
            print(f"error: {e}", file=sys.stderr)
            return 2
        root = instances_root()
        root.mkdir(parents=True, exist_ok=True)
        free = shutil.disk_usage(root).free
        print(f"free disk: {free / 2**30:.1f} GiB")
        if free < 2 * 2**30:
            print("refusing: <2 GiB free", file=sys.stderr)
            return 2
        for i in range(2 * a.first_pair, a.n):
            try:
                m = provision(i, role="host" if i == 0 else "client",
                              nvse_dir=Path(a.nvse_dir) if a.nvse_dir else None,
                              plugin_dll=dll, core_dll=core, extra_ini=extra_ini,
                              autoload=a.autoload, modes=tuple(a.modes.split(",")),
                              appid=None if a.no_appid else STEAM_APPID,
                              port_base=a.port_base if a.pairs else None,
                              dxvk=not a.no_dxvk, no_pad=a.no_pad)
            except UnsafeInstance as e:
                print(f"error: instance {i}: {e}", file=sys.stderr)
                return 2
            tag = f" pair={m['pair']} port={m['port']}" if "pair" in m else ""
            print(f"instance {i}: {m['dir']} links={m['link_counts']}{tag}")
    elif a.cmd == "show":
        for i in range(a.n):
            print(json.dumps(load_meta(i), indent=1))
    else:
        for i in range(a.n):
            destroy(i)
            print("destroyed", i)
    return 0


if __name__ == "__main__":
    sys.exit(main())
