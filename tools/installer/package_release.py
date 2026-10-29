#!/usr/bin/env python3
"""Build the player release zip from build output: plugin, data files, installer, launcher, player guide, manifest, checksums.

    python -I tools/installer/package_release.py build --version 1.0.0 [--dll out/L1/nvcoop.dll] [--out out/release]
                                                 [--data-dir DIR ...] [--forbid TEXT ...] [--allow-missing] [--dry-run]
    python -I tools/installer/package_release.py verify out/release/FNV-CoOp-1.0.0.zip

What goes in (see PAYLOAD below): nvcoop.dll and the data files it reads from its own folder (rules, quest name maps, consent
table, guest-script list, volatile-key list), a default nvcoop.ini with co-op switched off (the launcher switches it on) and the feature switches on, the installer and launcher
(Python, no third-party packages), the launcher's Steam helper fnvsteam.exe (docs/steam.md; it uses the steam_api.dll of the player's own game, so no
Valve file is shipped), PLAYER-GUIDE.md, the licence texts, manifest.json (every payload file with size and
SHA-256, plus the supported game and xNVSE versions) and SHA256SUMS.txt (every file in the zip).

Checks before the zip is written, each one stops the build with a clear message:
  * a required file is missing (--allow-missing downgrades this to a warning, for a dry look at an incomplete build);
  * nvcoop.dll is not a 32-bit Windows DLL, or does not export the xNVSE plugin entry points; fnvsteam.exe is not a 32-bit Windows program
    (it loads the game's 32-bit steam_api.dll);
  * any shipped file contains a personal Windows path (C:\\Users\\<name>\\...) or a --forbid string, as plain text or UTF-16.
The zip is reproducible: same inputs and the same source date give the same bytes. After writing it, the zip is opened again
and checked against its own manifest and checksums, exactly like a player's installer will.
"""
import argparse
import json
import os
import random
import re
import sys
import tempfile
import time
import zipfile
from pathlib import Path
from typing import Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import fnv_find  # noqa: E402
import fnv_manifest  # noqa: E402
import fnv_mut  # noqa: E402
import fnv_pe  # noqa: E402

REPO = HERE.parent.parent
DEFAULT_NAME = "FNV-CoOp"
DEFAULT_STEAM_HELPER = REPO / "out" / "steamlink" / "fnvsteam.exe"
INSTALLER_FILES = ("fnv_mut.py", "fnv_pe.py", "fnv_ini.py", "fnv_find.py", "fnv_manifest.py", "fnv_install.py")
STEAM_HELPER = "fnvsteam.exe"
STEAM_APPID = b"22380\r\n"   # Fallout: New Vegas; the helper runs under the game's own AppID so Steam shows "playing Fallout: New Vegas"
LAUNCHER_FILES = ("fnv_launcher_core.py", "fnv_portcheck.py", "fnv_lobby.py", "fnv_steam.py", "fnv_terminal.py", "terminal.html", "VT323-Regular.ttf",
                  "VT323-OFL.txt", "launcher_gui.py")
DEFAULT_INI = ("; nvcoop.ini - settings of the co-op plugin. The launcher edits this file when you press Host or Join.\n"
               "; Co-op is OFF until you start the game from the launcher; the feature switches below only act while it is on.\n"
               "[coop]\n"
               "enable=0\n"
               "baseline=1\n"
               "dialogue=1\n"
               "[intents]\n"
               "enable=1\n"
               "dialogue=1\n"
               "dialogue_results=1\n"
               "stage=1\n"
               "pickup=0\n"
               "container=0\n"
               "[hooks]\n"
               "story_capture=1\n"
               "[live]\n"
               "actors=1\n"
               "avatars=1\n"
               "combat=1\n"
               "spawns=1\n"
               "social=1\n"
               "[avatar]\n"
               "enabled=1\n"
               "[combat]\n"
               "enabled=1\n"
               "gate=1\n"
               "native=1\n"
               "[anim]\n"
               "enabled=1\n"
               "[vats]\n"
               "enabled=1\n"
               "[areadigest]\n"
               "enabled=1\n"
               "[worldtime]\n"
               "enabled=1\n"
               "[loot]\n"
               "enabled=1\n"
               "[companions]\n"
               "enabled=1\n"
               "[rules]\n"
               "; Party tether (keeps the players together in a scene zone): on = hold and auto-warp (default), warn = only tell a far player, off = ignore.\n"
               "tether=on\n"
               "[bridge]\n"
               "; Test-rig switches: keep 0. minimized=1 hides the game window and keeps it minimized.\n"
               "minimized=0\n"
               "focusGuard=0\n")
# generic folder names that are not a person
HARMLESS_USERS = {"public", "default", "all users", "default user", "yourname", "your name", "username", "user", "name", "you", "<you>", "<name>", "%username%"}
_USER_PATH = re.compile(rb"[A-Za-z]:[\\/]{1,2}Users[\\/]{1,2}([^\\/\x00\r\n\"'<>|*?]{1,64})", re.I)
_USER_PATH_W = re.compile(rb"[A-Za-z]\x00:\x00(?:\\\x00|/\x00){1,2}U\x00s\x00e\x00r\x00s\x00(?:\\\x00|/\x00){1,2}((?:[^\\/\x00]\x00){1,64})", re.I)


class PackageError(RuntimeError):
    pass


def _loads_core(dll: str) -> bool:
    """nvcoop.dll is the split build's loader (it names the core DLL it loads)."""
    return Path(dll).is_file() and b"nvcoop_mod.dll" in Path(dll).read_bytes()


def _payload_spec(args) -> List[dict]:
    """name in Data/NVSE/Plugins, where to find it, whether the release is useless without it."""
    data_dirs = [Path(d) for d in args.data_dir] if args.data_dir else [REPO / "out" / "L10" / "plugin", REPO / "out" / "L1", REPO / "out" / "L10" / "data"]
    dll_dir = Path(args.dll).parent

    def first(name: str, extra: Optional[List[Path]] = None) -> Optional[Path]:
        for d in (extra or []) + data_dirs:
            if (d / name).is_file():
                return d / name
        return None

    return [
        {"name": "nvcoop.dll", "path": Path(args.dll) if Path(args.dll).is_file() else None, "required": True, "what": "the plugin"},
        # the split build (plugin/loader/build_split.cmd): nvcoop.dll is a small loader that loads the core from its own folder
        {"name": "nvcoop_mod.dll", "path": first("nvcoop_mod.dll", [dll_dir]) if _loads_core(args.dll) else None, "required": _loads_core(args.dll),
         "what": "the plugin core the loader nvcoop.dll loads"},
        {"name": "nvcoop_rules.json", "path": Path(args.rules) if Path(args.rules).is_file() else None, "required": True, "what": "co-op rules for every quest"},
        {"name": "nvcoop_edids.json", "path": first("nvcoop_edids.json", [dll_dir]), "required": True, "what": "quest name map (host)"},
        {"name": "nvcoop_infoquest.json", "path": first("nvcoop_infoquest.json", [dll_dir]), "required": True, "what": "dialogue to quest map (host)"},
        {"name": "nvcoop_guest_local_run.json", "path": first("nvcoop_guest_local_run.json", [dll_dir]), "required": True, "what": "scripts guests run themselves"},
        {"name": "nvcoop_cause_quests.json", "path": first("nvcoop_cause_quests.json", [dll_dir]), "required": True, "what": "script to quest map (rules of a quest's own scripts)"},
        {"name": "nvcoop_pack_watch.json", "path": first("nvcoop_pack_watch.json", [dll_dir]), "required": True, "what": "item forms a guest reports to the host (guest pack facts)"},
        {"name": "nvcoop_intents.json", "path": first("nvcoop_intents.json", [dll_dir]), "required": True, "what": "shared storage containers and gear stashes (guest intents)"},
        {"name": "nvcoop_consent_messages.txt", "path": Path(args.consent) if Path(args.consent).is_file() else None, "required": True, "what": "consent prompt table"},
        {"name": "nvcoop_volatile_keys.txt", "path": Path(args.volatile) if Path(args.volatile).is_file() else None, "required": True, "what": "list of script variables that differ per machine (every player must have the same list)"},
        # taken in a clean game (FalloutNV.esm only) right after the opening movie; the launcher lobby starts the party from it (plugin/main.cpp)
        {"name": "nvcoop_start.fos", "path": Path(args.start_save) if Path(args.start_save).is_file() else None, "required": True, "what": "the party's start save"},
    ]


def scan_forbidden(name: str, data: bytes, forbid: List[str]) -> List[str]:
    """Reasons this file must not be published: a personal Windows path, or one of the --forbid strings."""
    out = []
    if not fnv_mut.on("skip_privacy_scan"):
        for m in _USER_PATH.finditer(data):
            who = m.group(1).decode("latin-1").strip().lower()
            if who not in HARMLESS_USERS:
                out.append(f"{name}: contains a personal Windows path ({m.group(0)[:60].decode('latin-1')})")
                break
        for m in _USER_PATH_W.finditer(data):
            who = m.group(1).decode("utf-16le", errors="replace").strip().lower()
            if who not in HARMLESS_USERS:
                out.append(f"{name}: contains a personal Windows path (UTF-16)")
                break
        for f in forbid:
            if not f:
                continue
            if f.lower().encode("utf-8") in data.lower() or f.lower().encode("utf-16le") in data.lower():
                out.append(f"{name}: contains the forbidden text {f!r}")
    return out


_WHOLE_PATH = re.compile(rb"(?:(?<=\x00)|^)[A-Za-z]:[\\/]{1,2}Users[\\/]{1,2}([^\\/\x00]{1,64})[\\/][^\x00]*")


def scrub_paths(data: bytes) -> Tuple[bytes, int]:
    """Zero every C string that is nothing but a personal Windows path (the compiler can bake the source folder of a file into a
    DLL, for example as a default data path). The string becomes empty, which the code treats as "no such file". Strings that
    merely CONTAIN such a path are left alone, and scan_forbidden then refuses them. Returns (new bytes, strings blanked)."""
    if fnv_mut.on("no_scrub"):
        return data, 0
    out = bytearray(data)
    n = 0
    for m in _WHOLE_PATH.finditer(data):
        who = m.group(1).decode("latin-1").strip().lower()
        if who in HARMLESS_USERS:
            continue
        out[m.start():m.end()] = b"\x00" * (m.end() - m.start())
        n += 1
    return bytes(out), n


def check_dll(path: Path) -> List[str]:
    problems = []
    if fnv_mut.on("skip_dll_check"):
        return problems
    if fnv_pe.machine_type(path) != fnv_pe.MACHINE_I386:
        problems.append(f"{path.name} is not a 32-bit Windows DLL (New Vegas is a 32-bit game)")
    data = path.read_bytes()
    for sym in (b"NVSEPlugin_Query", b"NVSEPlugin_Load"):
        if sym not in data:
            problems.append(f"{path.name} does not export {sym.decode()}, so xNVSE would not load it")
    return problems


def _protocol_version() -> Optional[int]:
    try:
        t = (REPO / "synccore" / "include" / "fnvcoop" / "wire.h").read_text(encoding="utf-8")
    except OSError:
        return None
    m = re.search(r"kProtocolVersion\s*=\s*(\d+)", t)
    return int(m.group(1)) if m else None


def _cmd_script(lines: List[str]) -> bytes:
    return ("\r\n".join(lines) + "\r\n").encode("ascii")


FIND_PY = [
    ":findpy",
    "set \"PY=\"",
    "where py >nul 2>nul && (set \"PY=py -3\" & goto :eof)",
    "where python >nul 2>nul && (set \"PY=python\" & goto :eof)",
    "goto :eof",
    ":nopy",
    "echo Python 3 is needed for this step and was not found on this PC.",
    "echo Install it from the official page:  " + fnv_find.LINK_PYTHON,
    "echo (tick \"Add python.exe to PATH\" in its installer), then run this file again.",
    "pause",
    "exit /b 1",
]


def cmd_files(version: str) -> Dict[str, bytes]:
    head = ["@echo off", "setlocal", "cd /d \"%~dp0\"", "call :findpy", "if not defined PY goto nopy"]
    return {
        "Install.cmd": _cmd_script(head + ["%PY% installer\\fnv_install.py install", "echo.", "pause", "exit /b"] + FIND_PY),
        "Uninstall.cmd": _cmd_script(head + ["%PY% installer\\fnv_install.py uninstall", "echo.", "pause", "exit /b"] + FIND_PY),
        "Check.cmd": _cmd_script(head + ["%PY% installer\\fnv_install.py check", "echo.", "%PY% installer\\fnv_install.py status", "echo.", "pause", "exit /b"] + FIND_PY),
        "Launcher.cmd": _cmd_script(["@echo off", "setlocal", "cd /d \"%~dp0\"", "call :findpy", "if not defined PY goto nopy",
                                     "where pyw >nul 2>nul && (start \"\" pyw -3 launcher\\fnv_terminal.py & exit /b)",
                                     "%PY% launcher\\fnv_terminal.py", "exit /b"] + FIND_PY),
        "Launcher-Classic.cmd": _cmd_script(["@echo off", "setlocal", "cd /d \"%~dp0\"", "call :findpy", "if not defined PY goto nopy",
                                             "where pyw >nul 2>nul && (start \"\" pyw -3 launcher\\launcher_gui.py & exit /b)",
                                             "%PY% launcher\\launcher_gui.py", "exit /b"] + FIND_PY),
    }


def readme_first(version: str) -> bytes:
    text = (f"FNV Co-Op {version}: play Fallout: New Vegas together, 2 to 4 players.\n\n"
            "Every player needs: Fallout: New Vegas 1.4.0.525 (Steam or GOG), xNVSE 6.3.0 or newer, Windows,\n"
            f"and Python 3 (for the installer and launcher). Links: xNVSE {fnv_find.LINK_NVSE}  Python {fnv_find.LINK_PYTHON}\n\n"
            "1. Unzip this folder anywhere.\n"
            "2. Double-click Install.cmd. It finds your game, checks the version, backs up what it changes and copies the plugin.\n"
            "3. Double-click Launcher.cmd and type your courier name. One player picks HOST A GAME, the others JOIN A FRIEND with\n"
            "   the host's address. Everyone readies up in the lobby, the host starts, and every game starts a new game together.\n"
            "   No address and no router settings? Set the launcher's CONNECTION to STEAM: you join your Steam friends who host.\n\n"
            "Every player must use the same release. To remove everything: Uninstall.cmd. Your saves are never touched.\n"
            "Read PLAYER-GUIDE.md for the details and for fixing common problems.\n")
    return text.encode("utf-8")


def third_party_text() -> bytes:
    enet = REPO / "third_party" / "enet" / "LICENSE"
    lic = enet.read_text(encoding="utf-8", errors="replace") if enet.is_file() else "See https://github.com/lsalzman/enet"
    return ("Third-party software\n\n"
            "nvcoop.dll contains ENet (networking), MIT licence:\n\n" + lic.strip() + "\n\n"
            "Co-op over Steam uses the steam_api.dll of the player's own Fallout: New Vegas install and the Steam client already on the PC; no file\n"
            "from Valve is included.\n\n"
            "The launcher's terminal font is VT323 (The VT323 Project Authors), SIL Open Font License 1.1: launcher\\VT323-OFL.txt\n\n"
            "xNVSE is NOT included. Get it from its official page: " + fnv_find.LINK_NVSE + "\n").encode("utf-8")


def collect(args) -> Tuple[Dict[str, bytes], dict, List[str], List[str]]:
    """(files by zip path under the release folder, manifest, errors, warnings). Nothing is written to disk."""
    errors: List[str] = []
    warnings: List[str] = []
    files: Dict[str, bytes] = {}
    entries = []
    mt = 0.0
    forbid = list(args.forbid or [])
    env_forbid = os.environ.get("FNVCOOP_FORBID", "")
    forbid += [x for x in env_forbid.split(";") if x]
    for spec in _payload_spec(args):
        p = spec["path"]
        if p is None:
            (warnings if (args.allow_missing or not spec["required"] or fnv_mut.on("ignore_missing_required")) else errors).append(
                f"{spec['name']}: not found ({spec['what']})" + ("" if spec["required"] else " - optional, left out"))
            continue
        data = p.read_bytes()
        mt = max(mt, p.stat().st_mtime)
        if spec["name"] in ("nvcoop.dll", "nvcoop_mod.dll"):
            if spec["name"] == "nvcoop.dll":
                errors += check_dll(p)
            elif fnv_pe.machine_type(p) != fnv_pe.MACHINE_I386:
                errors.append(f"{spec['name']} is not a 32-bit Windows DLL (New Vegas is a 32-bit game)")
            data, blanked = scrub_paths(data)
            if blanked:
                warnings.append(f"{spec['name']}: blanked {blanked} string(s) that held a build-machine path (the packaged DLL differs from the built one; "
                                "fix the build so it does not embed source paths, see docs/tools/release.md)")
        files[f"payload/{spec['name']}"] = data
        entries.append({"src": f"payload/{spec['name']}", "dest": f"{fnv_manifest.DEST_PREFIX}{spec['name']}", "sha256": fnv_manifest.sha256_bytes(data),
                        "size": len(data), "keep_existing": False})
    ini = DEFAULT_INI.replace("\n", "\r\n").encode("ascii")
    files["payload/nvcoop.ini"] = ini
    entries.append({"src": "payload/nvcoop.ini", "dest": f"{fnv_manifest.DEST_PREFIX}nvcoop.ini", "sha256": fnv_manifest.sha256_bytes(ini),
                    "size": len(ini), "keep_existing": True})
    helper = Path(getattr(args, "steam_helper", None) or DEFAULT_STEAM_HELPER)
    if helper.is_file():
        data = helper.read_bytes()
        if fnv_pe.machine_type(helper) != fnv_pe.MACHINE_I386 and not fnv_mut.on("skip_helper_check"):
            errors.append(f"{STEAM_HELPER} is not a 32-bit Windows program (it has to load the game's 32-bit steam_api.dll)")
        data, blanked = scrub_paths(data)
        if blanked:
            warnings.append(f"{STEAM_HELPER}: blanked {blanked} string(s) that held a build-machine path")
        files[f"launcher/{STEAM_HELPER}"] = data
        mt = max(mt, helper.stat().st_mtime)
    else:
        (warnings if args.allow_missing else errors).append(
            f"launcher/{STEAM_HELPER}: not found (the Steam helper; build it: python -I tools/build/build.py --source tools/steamlink "
            "--build build/steamlink --target fnvsteam)")
    files["launcher/steam_appid.txt"] = STEAM_APPID
    for n in INSTALLER_FILES:
        files[f"installer/{n}"] = (HERE / n).read_bytes()
    for n in LAUNCHER_FILES:
        src = REPO / "tools" / "launcher" / n
        if src.is_file():
            files[f"launcher/{n}"] = src.read_bytes()
        else:
            (warnings if args.allow_missing else errors).append(f"launcher/{n}: not found")
    files.update(cmd_files(args.version))
    files["README-FIRST.txt"] = readme_first(args.version)
    files["THIRD-PARTY.txt"] = third_party_text()
    guide = REPO / "docs" / "PLAYER-GUIDE.md"
    if guide.is_file():
        files["PLAYER-GUIDE.md"] = guide.read_bytes()
    else:
        (warnings if args.allow_missing else errors).append("PLAYER-GUIDE.md: not found")
    lic = Path(args.license) if args.license else REPO / "LICENSE"
    if lic.is_file():
        files["LICENSE.txt"] = lic.read_bytes()
    else:
        warnings.append("LICENSE: not found, the zip has no licence file (pass --license)")
    for name, data in sorted(files.items()):
        errors += scan_forbidden(name, data, forbid)
    epoch = args.source_date_epoch if args.source_date_epoch is not None else int(os.environ.get("SOURCE_DATE_EPOCH", "0") or 0) or int(mt) or 1767225600
    manifest = {
        "schema": fnv_manifest.SCHEMA, "name": args.name, "version": args.version,
        "built_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(epoch)),
        "game_version": fnv_pe.format_version(fnv_find.SUPPORTED_GAME), "nvse_min": ".".join(map(str, fnv_find.MIN_NVSE)),
        "protocol": _protocol_version(), "files": entries,
    }
    files[fnv_manifest.MANIFEST_NAME] = (json.dumps(manifest, indent=1, sort_keys=True) + "\n").encode("utf-8")
    manifest["_epoch"] = epoch
    sums = fnv_manifest.sums_text([{"path": k, "sha256": fnv_manifest.sha256_bytes(v)} for k, v in files.items()])
    files[fnv_manifest.SUMS_NAME] = sums.encode("utf-8")
    return files, manifest, errors, warnings


def write_zip(path: Path, root: str, files: Dict[str, bytes], epoch: int) -> None:
    """Deterministic zip: sorted names, fixed times and attributes, one compression level."""
    dt = time.gmtime(max(epoch, 315532800))[:6]
    if fnv_mut.on("zip_nondeterministic"):
        dt = time.gmtime(max(epoch, 315532800) + random.randint(1, 10 ** 6))[:6]
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(".zip.tmp")
    with zipfile.ZipFile(tmp, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as z:
        for name in sorted(files):
            zi = zipfile.ZipInfo(f"{root}/{name}", dt)
            zi.compress_type = zipfile.ZIP_DEFLATED
            zi.create_system = 0
            zi.external_attr = 0o644 << 16
            z.writestr(zi, files[name], compresslevel=9)
    os.replace(tmp, path)


def verify_zip(zip_path: Path) -> List[str]:
    """Open the zip like a player: unpack, check the checksum list, then the manifest against the payload. Returns the problems."""
    problems: List[str] = []
    with tempfile.TemporaryDirectory(prefix="fnvcoop-verify-") as td:
        with zipfile.ZipFile(zip_path) as z:
            names = z.namelist()
            roots = {n.split("/", 1)[0] for n in names}
            if len(roots) != 1:
                return [f"the zip must hold exactly one top folder, it holds {sorted(roots)}"]
            for n in names:
                if n.startswith("/") or ".." in n.split("/") or ":" in n:
                    return [f"unsafe path in zip: {n}"]
            z.extractall(td)
        root = Path(td) / roots.pop()
        sums = root / fnv_manifest.SUMS_NAME
        if not sums.is_file():
            return ["SHA256SUMS.txt is missing"]
        if not fnv_mut.on("verify_skip_sums"):
            problems += fnv_manifest.check_sums(root, sums.read_text(encoding="utf-8"))
        listed = {ln.partition(" *")[2] for ln in sums.read_text(encoding="utf-8").splitlines() if ln.strip()}
        for p in root.rglob("*"):
            rel = p.relative_to(root).as_posix()
            if p.is_file() and rel not in listed and rel != fnv_manifest.SUMS_NAME:
                problems.append(f"{rel}: in the zip but not in SHA256SUMS.txt")
        try:
            m = fnv_manifest.load(root)
            problems += fnv_manifest.verify_payload(root, m)
        except fnv_manifest.ManifestError as e:
            problems.append(str(e))
    return problems


def build(args) -> int:
    if not re.match(r"^\d+\.\d+\.\d+([-.][A-Za-z0-9.]+)?$", args.version):
        print(f"bad --version {args.version!r}; use 1.0.0 or 1.0.0-rc1")
        return 2
    files, manifest, errors, warnings = collect(args)
    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("ERROR:", e)
    root = f"{args.name}-{args.version}"
    print(f"{len(files)} files, {sum(len(v) for v in files.values()) // 1024} KiB before compression, release folder {root}")
    if errors:
        print("Not packaged: fix the errors above.")
        return 1
    if args.dry_run:
        for n in sorted(files):
            print("  ", n)
        print("Dry run: no zip written.")
        return 0
    out = Path(args.out) / f"{root}.zip"
    write_zip(out, root, files, manifest["_epoch"])
    problems = verify_zip(out)
    if problems:
        for p in problems:
            print("VERIFY FAILED:", p)
        return 1
    digest = fnv_manifest.sha256_file(out)
    (out.with_suffix(".zip.sha256")).write_text(f"{digest} *{out.name}\n", encoding="utf-8")
    print(f"{out}\n  SHA-256 {digest}\nALL OK")
    return 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    b = sub.add_parser("build")
    b.add_argument("--version", required=True)
    b.add_argument("--name", default=DEFAULT_NAME)
    b.add_argument("--dll", default=str(REPO / "out" / "L1" / "nvcoop.dll"))
    b.add_argument("--rules", default=str(REPO / "docs" / "quest-policies" / "rules.json"))
    b.add_argument("--consent", default=str(REPO / "plugin" / "dialogue" / "data" / "consent_messages.txt"))
    b.add_argument("--volatile", default=str(REPO / "plugin" / "snapshot" / "data" / "volatile_keys.txt"))
    b.add_argument("--start-save", default=str(REPO / "plugin" / "coop" / "data" / "nvcoop_start.fos"))
    b.add_argument("--data-dir", action="append", help="folder with nvcoop_edids.json etc. (repeatable; searched before the defaults)")
    b.add_argument("--steam-helper", default=str(DEFAULT_STEAM_HELPER), help="the launcher's Steam helper (tools/steamlink), shipped as launcher\\fnvsteam.exe")
    b.add_argument("--license", help="licence file to include (default: LICENSE in the repo root)")
    b.add_argument("--out", default=str(REPO / "out" / "release"))
    b.add_argument("--forbid", action="append", help="text that must not appear in any shipped file (repeatable; also env FNVCOOP_FORBID, ';' separated)")
    b.add_argument("--source-date-epoch", type=int, default=None)
    b.add_argument("--allow-missing", action="store_true")
    b.add_argument("--dry-run", action="store_true")
    v = sub.add_parser("verify")
    v.add_argument("zip")
    a = ap.parse_args(argv)
    if a.cmd == "verify":
        probs = verify_zip(Path(a.zip))
        for p in probs:
            print("PROBLEM:", p)
        print("ALL OK" if not probs else f"{len(probs)} problem(s)")
        return 0 if not probs else 1
    return build(a)


if __name__ == "__main__":
    sys.exit(main())
