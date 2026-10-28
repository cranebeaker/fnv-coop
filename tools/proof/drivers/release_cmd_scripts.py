#!/usr/bin/env python3
"""prove.py piece for `release_cmd_scripts`: the release zip's Check.cmd, Install.cmd and Uninstall.cmd, run for real on a synthetic game folder.

    python -I tools/proof/drivers/release_cmd_scripts.py [--keep]

docs/INGAME-TESTS.md step 10 items 1 and 5, scripted. A synthetic release is packaged with tools/installer/package_release.py from synthetic
inputs (a 32-bit stand-in DLL), unzipped, and its own .cmd files are started with cmd.exe exactly as a player double-clicks them. The game is
a fake New Vegas folder (tools/installer/tests/fakegame.py: version 1.4.0.525, xNVSE 6.3.x, a save, Fallout.ini). To keep the scripts away
from this PC's real install, the processes get FNV_GAME_DIR and FNV_DOCUMENTS_DIR for the fake folders, and a `sitecustomize.py` on
PYTHONPATH (written next to the release, never shipped) that blanks the registry and drive scan of fnv_find and answers "the game is not
running". Nothing outside the scratch folder is read or written.

Scenarios (each prints PASS or FAIL lines):
  clean        Check.cmd reports 1.4.0.525, xNVSE, "not installed" and changes nothing; Install.cmd copies every payload file with the right hash,
               writes the install record, keeps the saves and the game INI byte for byte; Check.cmd then says installed; Uninstall.cmd leaves the
               game folder exactly as it was before (every file hash, no FNVCoOp folder, saves untouched)
  keeps        a nvcoop.ini the player changed survives Install.cmd of an upgrade and Uninstall.cmd restores the original
  old game     Install.cmd on a game with the wrong version, and on one without xNVSE: says so and changes nothing
  damaged      Install.cmd on a payload file that was altered after packaging: refuses before it touches the game
  no python    with no Python on PATH every .cmd prints the python.org hint instead of crashing
Exit 0 all pass, 1 a check failed. Windows only (cmd.exe); elsewhere exit 3.
"""
from __future__ import annotations

import argparse
import contextlib
import io
import os
import shutil
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path
from typing import Callable, Dict, List, Optional, Sequence, Tuple

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
for _p in (HERE, REPO / "tools" / "installer", REPO / "tools" / "installer" / "tests"):
    sys.path.insert(0, str(_p))
import drv_common as D  # noqa: E402
import fakegame as fg  # noqa: E402
import fnv_manifest  # noqa: E402
import package_release as pkg  # noqa: E402

SHIM = '''"""Test-only: keeps the installer scripts on the synthetic game (loaded because this folder is on PYTHONPATH)."""
import importlib.abc
import importlib.machinery
import sys


class _Hook(importlib.abc.MetaPathFinder):
    def find_spec(self, name, path, target=None):
        if name != "fnv_find":
            return None
        sys.meta_path.remove(self)
        spec = importlib.machinery.PathFinder.find_spec(name, path)
        real = spec.loader.exec_module

        def patched(mod):
            real(mod)
            mod.WinRegistry = mod.Registry          # no registry: no Steam, GOG or retail install of this PC
            mod.real_drives = lambda: []            # no drive scan for common folders
            mod.game_is_running = lambda runner=None: False
        spec.loader.exec_module = patched
        return spec


sys.meta_path.insert(0, _Hook())
'''

CMD_TIMEOUT = 180
Mutate = Callable[[Path], None]


# ------------------------------------------------------------------ the synthetic release and game
def fake_dll() -> bytes:
    return fg.make_pe((1, 0, 0, 0), tail=b"NVSEPlugin_Query\0NVSEPlugin_Load\0")


def package(work: Path, version: str = "1.0.0", dll: Optional[bytes] = None) -> Path:
    """Build a release zip from synthetic inputs and unpack it. -> the reference release folder (the player's 'unzipped' folder)."""
    inp = work / "in"
    inp.mkdir(parents=True, exist_ok=True)
    (inp / "nvcoop.dll").write_bytes(dll if dll is not None else fake_dll())
    (inp / "rules.json").write_text('{"rules": []}', encoding="utf-8")
    (inp / "consent.txt").write_text("# consent\n", encoding="utf-8")
    (inp / "volatile.txt").write_text("key\n", encoding="utf-8")
    (inp / "LICENSE").write_text("MIT test licence\n", encoding="utf-8")
    for n in ("nvcoop_edids.json", "nvcoop_infoquest.json", "nvcoop_guest_local_run.json", "nvcoop_intents.json", "nvcoop_cause_quests.json", "nvcoop_pack_watch.json"):
        (inp / n).write_text("{}", encoding="utf-8")
    out = work / "zip"
    argv = ["build", "--version", version, "--dll", str(inp / "nvcoop.dll"), "--rules", str(inp / "rules.json"), "--consent", str(inp / "consent.txt"),
            "--volatile", str(inp / "volatile.txt"), "--data-dir", str(inp), "--license", str(inp / "LICENSE"), "--out", str(out),
            "--source-date-epoch", "1767225600"]
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        code = pkg.main(argv)
    if code != 0:
        raise RuntimeError("packaging the synthetic release failed: " + buf.getvalue()[-300:])
    z = out / f"FNV-CoOp-{version}.zip"
    dest = work / "unzipped"
    with zipfile.ZipFile(z) as zf:
        zf.extractall(dest)
    return dest / f"FNV-CoOp-{version}"


class Rig:
    """One synthetic PC: a release folder to double-click from, a game folder, a Documents folder, and the environment the scripts see."""

    def __init__(self, work: Path, rel: Path, game: Path, docs: Path, path_env: Optional[str] = None):
        self.work, self.rel, self.game, self.docs = work, rel, game, docs
        self.shim = work / "shim"
        self.shim.mkdir(exist_ok=True)
        (self.shim / "sitecustomize.py").write_text(SHIM, encoding="utf-8")
        self.path_env = path_env

    def env(self) -> Dict[str, str]:
        e = dict(os.environ)
        e["FNV_GAME_DIR"] = str(self.game)
        e["FNV_DOCUMENTS_DIR"] = str(self.docs)
        e["PYTHONPATH"] = str(self.shim) + os.pathsep + str(REPO / "tools" / "installer")
        for k in ("ProgramFiles(x86)", "PYTHONHOME", "PYTHONSAFEPATH"):
            e.pop(k, None)
        if self.path_env is not None:
            e["PATH"] = self.path_env
        return e

    def run(self, script: str) -> Tuple[int, str]:
        """Start <script>.cmd the way a double-click does (cmd /c, working folder = the release folder, an empty pipe on stdin so `pause` ends; NUL would look like a console to Python and make the installer ask questions)."""
        cp = subprocess.run(["cmd", "/c", str(self.rel / script)], cwd=self.rel, env=self.env(), capture_output=True, text=True, errors="replace",
                            timeout=CMD_TIMEOUT, input="")
        return cp.returncode, (cp.stdout or "") + (cp.stderr or "")


def new_rig(work: Path, name: str, rel: Path, **game_kw) -> Rig:
    game = fg.make_game(work / name / "game", **game_kw)
    docs = work / name / "docs"
    fg.make_mygames(docs)
    return Rig(work / name, rel, game, docs)


def snapshot(root: Path) -> Dict[str, str]:
    return fg.tree(root)


# ------------------------------------------------------------------ the scenarios
def scenario_clean(c: D.Checks, rig: Rig, rel_payload: Dict[str, str]) -> None:
    before = snapshot(rig.game)
    docs_before = snapshot(rig.docs)
    code, text = rig.run("Check.cmd")
    c.check("clean: Check.cmd names the game version 1.4.0.525", "1.4.0.525" in text, text[-200:])
    nvse_line = text.split("xNVSE:", 1)[1].splitlines()[0] if "xNVSE:" in text else "not installed"
    c.check("clean: Check.cmd sees xNVSE", "not installed" not in nvse_line and "6.3" in nvse_line, nvse_line)
    c.check("clean: Check.cmd says the co-op plugin is not installed", "Co-op plugin: not installed" in text, text[-200:])
    c.check("clean: Check.cmd changed nothing", snapshot(rig.game) == before and snapshot(rig.docs) == docs_before)

    code, text = rig.run("Install.cmd")
    plug = rig.game / "Data" / "NVSE" / "Plugins"
    missing = [n for n in rel_payload if not (plug / n).is_file()]
    c.check("clean: Install.cmd copied every payload file into Data\\NVSE\\Plugins", not missing, f"missing {missing[:4]} in: {text[-200:]}")
    wrong = [n for n, h in rel_payload.items() if (plug / n).is_file() and fnv_manifest.sha256_file(plug / n) != h and n != "nvcoop.ini"]
    c.check("clean: every installed file has the packaged hash", not wrong, str(wrong[:4]))
    c.check("clean: Install.cmd wrote the install record (FNVCoOp\\install.json)", (rig.game / "FNVCoOp" / "install.json").is_file(), text[-200:])
    c.check("clean: Install.cmd left the saves and the game exe alone",
            snapshot(rig.game / "Saves") == {k[len("Saves/"):]: v for k, v in before.items() if k.startswith("Saves/") and k != "Saves"}
            and fnv_manifest.sha256_file(rig.game / "FalloutNV.exe") == before["FalloutNV.exe"])
    c.check("clean: Install.cmd did not edit the game's Fallout.ini", fnv_manifest.sha256_file(rig.game / "Fallout.ini") == before["Fallout.ini"])
    code, text = rig.run("Check.cmd")
    c.check("clean: Check.cmd afterwards says installed, release 1.0.0", "installed, release 1.0.0" in text, text[-200:])

    code, text = rig.run("Uninstall.cmd")
    after = snapshot(rig.game)
    c.check("clean: Uninstall.cmd removed the FNVCoOp folder", not (rig.game / "FNVCoOp").exists(), text[-200:])
    c.check("clean: Uninstall.cmd removed the plugin files", not any((plug / n).exists() for n in rel_payload), text[-200:])
    extra = sorted(set(after) - set(before))
    changed = sorted(k for k in before if k in after and before[k] != after[k])
    c.check("clean: after Uninstall.cmd the game folder is exactly as before (no new file, no changed hash, nothing lost)",
            not extra and not changed and set(before) <= set(after), f"new {extra[:3]} changed {changed[:3]} lost {sorted(set(before) - set(after))[:3]}")
    code, text = rig.run("Check.cmd")
    c.check("clean: Check.cmd afterwards says not installed", "Co-op plugin: not installed" in text, text[-200:])


def scenario_keeps_ini(c: D.Checks, rig: Rig) -> None:
    plug = rig.game / "Data" / "NVSE" / "Plugins"
    plug.mkdir(parents=True, exist_ok=True)
    (plug / "nvcoop.ini").write_text("[coop]\r\nenable=1\r\nrole=host\r\nport=31337\r\n", encoding="ascii")
    original = (plug / "nvcoop.ini").read_bytes()
    rig.run("Install.cmd")
    c.check("keeps: Install.cmd keeps the player's own nvcoop.ini", (plug / "nvcoop.ini").read_bytes() == original)
    rig.run("Uninstall.cmd")
    c.check("keeps: Uninstall.cmd gives the player's nvcoop.ini back", (plug / "nvcoop.ini").is_file() and (plug / "nvcoop.ini").read_bytes() == original)


def scenario_refused(c: D.Checks, rig: Rig, label: str, expect: str) -> None:
    before = snapshot(rig.game)
    code, text = rig.run("Install.cmd")
    c.check(f"{label}: Install.cmd says why ({expect!r})", expect.lower() in text.lower(), text[-240:])
    c.check(f"{label}: Install.cmd changed nothing", snapshot(rig.game) == before)


def scenario_no_python(c: D.Checks, work: Path, rel: Path) -> None:
    rig = new_rig(work, "nopy", rel)
    rig.path_env = os.path.join(os.environ.get("SystemRoot", r"C:\Windows"), "System32")
    before = snapshot(rig.game)
    for script in ("Check.cmd", "Install.cmd", "Uninstall.cmd"):
        code, text = rig.run(script)
        c.check(f"no python: {script} prints the python.org hint", "python.org" in text and "Python 3 is needed" in text, text[-200:])
    c.check("no python: nothing in the game folder changed", snapshot(rig.game) == before)


def run_scenarios(c: D.Checks, work: Path, mutate: Optional[Mutate] = None) -> None:
    rel = package(work / "pkg")
    payload = {p.name: fnv_manifest.sha256_file(p) for p in sorted((rel / "payload").iterdir())}
    if mutate:
        mutate(rel)          # twin hook: damage the unzipped release, the scenarios must notice
    scenario_clean(c, new_rig(work, "clean", rel), payload)
    scenario_keeps_ini(c, new_rig(work, "keeps", rel))
    scenario_refused(c, new_rig(work, "oldgame", rel, exe_version=(1, 3, 0, 452)), "old game", "1.4.0.525")
    scenario_refused(c, new_rig(work, "nonvse", rel, nvse=False), "no xNVSE", "xNVSE")
    rel2 = package(work / "pkg2")
    victim = rel2 / "payload" / "nvcoop_rules.json"
    victim.write_bytes(victim.read_bytes() + b" ")
    scenario_refused(c, new_rig(work, "damaged", rel2), "damaged", "damaged")
    scenario_no_python(c, work, rel)


def run(a, out: Callable[[str], None] = lambda s: print(s, flush=True), mutate: Optional[Mutate] = None) -> int:
    if os.name != "nt":
        return D.blocked_exit("the .cmd scripts need Windows (cmd.exe)", out)
    work = Path(tempfile.mkdtemp(prefix="fnv-cmd-"))
    c = D.Checks(out)
    try:
        run_scenarios(c, work, mutate)
    finally:
        if not a.keep:
            shutil.rmtree(work, ignore_errors=True)
    out(c.summary())
    return c.exit_code()


def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--keep", action="store_true", help="leave the scratch folder (it is under the system temp folder)")
    return run(ap.parse_args(argv))


if __name__ == "__main__":
    sys.exit(main())
