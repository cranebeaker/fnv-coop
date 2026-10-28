#!/usr/bin/env python3
"""Game-free tests for the installer, the release packager and the shared helpers, plus a mutation twin for every guarantee.

    python -I tools/installer/run_tests.py          prints ALL OK when every check passes and every twin is caught

A check builds a fake New Vegas folder and a fake release in a temp folder and runs the real installer code on them. A twin turns
one fault on (fnv_mut.use) and runs the check again: the check MUST fail, otherwise the test is too weak and the run fails.
Nothing here starts a game, touches the registry or writes outside out\\release\\tmp.
"""
import io
import json
import os
import shutil
import socket
import sys
import tempfile
import traceback
import zipfile
from contextlib import contextmanager, redirect_stdout
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE / "tests"))
import fakegame as fg  # noqa: E402
import fnv_find  # noqa: E402
import fnv_ini  # noqa: E402
import fnv_install as inst  # noqa: E402
import fnv_manifest  # noqa: E402
import fnv_mut  # noqa: E402
import fnv_pe  # noqa: E402
import package_release as pkg  # noqa: E402

REPO = HERE.parent.parent
SCRATCH = REPO / "out" / "release" / "tmp"


@contextmanager
def tmp():
    SCRATCH.mkdir(parents=True, exist_ok=True)
    d = tempfile.mkdtemp(prefix="t-", dir=str(SCRATCH))
    try:
        yield Path(d)
    finally:
        shutil.rmtree(d, ignore_errors=True)


def not_running():
    return False


def setup(td, **game_kw):
    g = fg.make_game(td / "game", **game_kw)
    rel = fg.make_release(td / "release")
    mg = fg.make_mygames(td / "docs")
    return g, rel, mg


def do_install(g, rel, mg, **kw):
    kw.setdefault("running", not_running)
    return inst.install(g, rel, mygames=mg, **kw)


def quiet(fn, *a, **kw):
    """Run fn with its printed output swallowed (the packager prints a report)."""
    buf = io.StringIO()
    with redirect_stdout(buf):
        return fn(*a, **kw)


def lines(r):
    return "\n".join(r.lines)


def rd(path):
    """The bytes of a file, or None when it is not there (so a missing file fails an assertion instead of raising)."""
    try:
        return Path(path).read_bytes()
    except OSError:
        return None


def must_raise(exc, fn):
    try:
        fn()
    except exc:
        return
    raise AssertionError(f"expected {exc.__name__}")


# ---- PE version reader ------------------------------------------------------------------------------------------------------

def check_pe_version():
    with tmp() as td:
        p = td / "a.exe"
        p.write_bytes(fg.make_pe((1, 4, 0, 525)))
        assert fnv_pe.file_version(p) == (1, 4, 0, 525), fnv_pe.file_version(p)
        p.write_bytes(fg.make_pe((0, 6, 3, 4)))
        assert fnv_pe.file_version(p) == (0, 6, 3, 4)
        assert fnv_pe.machine_type(p) == 0x14C
        p.write_bytes(fg.make_pe((1, 2, 3, 4), machine=0x8664))
        assert fnv_pe.machine_type(p) == 0x8664
        p.write_bytes(fg.make_pe(None))
        assert fnv_pe.file_version(p) is None
        p.write_bytes(b"MZ" + b"\0" * 10)
        assert fnv_pe.file_version(p) is None and fnv_pe.machine_type(p) is None
        p.write_bytes(os.urandom(2000))
        assert fnv_pe.file_version(p) is None
        full = fg.make_pe((1, 4, 0, 525))
        for cut in (10, 64, 100, 330, 600, 700, len(full) - 5):
            p.write_bytes(full[:cut])
            assert fnv_pe.file_version(p) in (None, (1, 4, 0, 525))   # a cut file never raises
        assert fnv_pe.file_version(td / "missing.exe") is None
        assert fnv_pe.parse_version("1.4.0.525") == (1, 4, 0, 525) and fnv_pe.parse_version("0, 6, 4, 9") == (0, 6, 4, 9)
        assert fnv_pe.parse_version("x") is None


# ---- finding the game ---------------------------------------------------------------------------------------------------------

class FakeReg(fnv_find.Registry):
    def __init__(self, values=None, subs=None):
        self.values = values or {}
        self.subs = subs or {}

    def get(self, hive, key, value):
        return self.values.get((hive, key, value))

    def subkeys(self, hive, key):
        return self.subs.get((hive, key), [])


def check_find_steam():
    with tmp() as td:
        steam = td / "Steam"
        lib2 = td / "Lib2"
        (steam / "steamapps").mkdir(parents=True)
        vdf = '"libraryfolders"\n{\n\t"0"\n\t{\n\t\t"path"\t\t"' + str(steam).replace("\\", "\\\\") + '"\n\t}\n\t"1"\n\t{\n\t\t"path"\t\t"' + str(lib2).replace("\\", "\\\\") + '"\n\t}\n}\n'
        (steam / "steamapps" / "libraryfolders.vdf").write_text(vdf, encoding="utf-8")   # Steam writes UTF-8
        (lib2 / "steamapps").mkdir(parents=True)
        (lib2 / "steamapps" / "appmanifest_22380.acf").write_text('"AppState"\n{\n\t"appid"\t\t"22380"\n\t"installdir"\t\t"FNV Custom Folder"\n}\n', encoding="utf-8")
        fg.make_game(lib2 / "steamapps" / "common" / "FNV Custom Folder")
        reg = FakeReg({("HKCU", r"Software\Valve\Steam", "SteamPath"): str(steam)})
        found = fnv_find.candidates(None, {}, reg, lambda: [])
        assert [c.source for c in found] == ["steam"], found
        assert found[0].path.name == "FNV Custom Folder" and found[0].edition == "steam"


def _encodable(text, enc):
    try:
        text.encode(enc)
        return True
    except UnicodeEncodeError:
        return False


def check_find_steam_ansi_library_path():
    """A Steam client on a system whose code page is not UTF-8 (a Russian Windows, cp1251) writes
    libraryfolders.vdf in the ANSI code page, and the library path may hold non-ASCII characters.
    Those must survive the read, or the install is never found - and nothing says why."""
    with tmp() as td:
        steam = td / "Steam"
        # A Cyrillic folder name ("Biblioteka Steam"), written with escapes so this file stays ASCII
        # and cannot be broken by the code page of whoever checks it out.
        # The file is written in THIS machine's ANSI code page (what Steam does here), so pick a non-ASCII
        # name that code page can hold: Cyrillic on cp1251, accented Latin on cp1252 and the like.
        enc = fnv_ini.ini_encoding()
        name = next(n for n in ("\u0411\u0438\u0431\u043b\u0438\u043e\u0442\u0435\u043a\u0430 Steam", "Biblioth\u00e8que Steam",
                                "Bibliot\u00e9ka Steam") if _encodable(n, enc))
        lib = td / name
        (steam / "steamapps").mkdir(parents=True)
        (lib / "steamapps" / "common").mkdir(parents=True)
        fg.make_game(lib / "steamapps" / "common" / "Fallout New Vegas")
        vdf = '"libraryfolders"\n{\n\t"0"\n\t{\n\t\t"path"\t\t"' + str(lib).replace("\\", "\\\\") + '"\n\t}\n}\n'
        (steam / "steamapps" / "libraryfolders.vdf").write_bytes(vdf.encode(enc))
        reg = FakeReg({("HKCU", r"Software\Valve\Steam", "SteamPath"): str(steam)})
        found = fnv_find.candidates(None, {}, reg, lambda: [])
        assert [c.path.name for c in found] == ["Fallout New Vegas"], found
        assert found[0].path == lib / "steamapps" / "common" / "Fallout New Vegas", found


def check_find_gog_and_retail():
    with tmp() as td:
        a = fg.make_game(td / "gogA")
        b = fg.make_game(td / "gogB")
        c = fg.make_game(td / "retail")
        reg = FakeReg({("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games\1454587428", "path"): str(a),
                       ("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games\999", "path"): str(b),
                       ("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games\999", "gameName"): "Fallout: New Vegas Ultimate Edition",
                       ("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games\111", "path"): str(td / "other"),
                       ("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games\111", "gameName"): "Something Else",
                       ("HKLM", r"SOFTWARE\WOW6432Node\Bethesda Softworks\FalloutNV", "Installed Path"): str(c)},
                      {("HKLM", r"SOFTWARE\WOW6432Node\GOG.com\Games"): ["999", "111"]})
        found = fnv_find.candidates(None, {}, reg, lambda: [])
        assert [x.path.name for x in found] == ["gogA", "gogB", "retail"], [x.path.name for x in found]


def check_find_common_and_order():
    with tmp() as td:
        d = td / "D"
        g = fg.make_game(d / "GOG Games" / "Fallout New Vegas")
        other = fg.make_game(td / "explicit")
        env = {"FNV_GAME_DIR": str(g)}
        found = fnv_find.candidates(str(other), env, FakeReg(), lambda: [str(d)])
        assert [x.source for x in found] == ["argument", "env"], found     # env and common point at the same folder: listed once
        (td / "empty").mkdir()
        assert fnv_find.candidates(str(td / "empty"), {}, FakeReg(), lambda: []) == []   # no FalloutNV.exe = not a candidate
        found = fnv_find.candidates(None, {}, FakeReg(), lambda: [str(d)])
        assert len(found) == 1 and found[0].source == "common"


def check_inspect():
    with tmp() as td:
        ok = fnv_find.inspect_game(fg.make_game(td / "ok", nvse_version=(0, 6, 4, 9)))
        assert ok.ok and ok.exe_version == (1, 4, 0, 525) and fnv_find.nvse_triple(ok.nvse_version) == (6, 4, 9), (ok.problems, ok.overridable)
        old = fnv_find.inspect_game(fg.make_game(td / "old", exe_version=(1, 3, 0, 452)))
        assert not old.ok and not old.problems and "1.3.0.452" in old.overridable[0] and "1.4.0.525" in old.overridable[0], old
        unread = fnv_find.inspect_game(fg.make_game(td / "unread", exe_version=None))
        assert unread.overridable and "Cannot read" in unread.overridable[0]
        nonvse = fnv_find.inspect_game(fg.make_game(td / "nonvse", nvse=False))
        assert nonvse.problems and "xNVSE is not installed" in nonvse.problems[0] and fnv_find.LINK_NVSE in nonvse.problems[0]
        oldnvse = fnv_find.inspect_game(fg.make_game(td / "oldnvse", nvse_version=(0, 6, 2, 0)))
        assert oldnvse.overridable and "too old" in oldnvse.overridable[0], oldnvse
        assert not fnv_find.inspect_game(td / "nothing").ok
        nodata = fnv_find.inspect_game(fg.make_game(td / "nodata", data=False))
        assert nodata.problems and "Data" in nodata.problems[0]
        noesm = fnv_find.inspect_game(fg.make_game(td / "noesm", esm=False))
        assert noesm.problems and "FalloutNV.esm" in noesm.problems[0]
        nostub = fnv_find.inspect_game(fg.make_game(td / "nostub", steam=False), edition="steam")
        assert any("nvse_steam_loader" in w for w in nostub.warnings)
        assert fnv_find.game_is_running(lambda: '"FalloutNV.exe","123","Console","1","90,000 K"')
        assert not fnv_find.game_is_running(lambda: "INFO: No tasks are running which match the specified criteria.")


# ---- ini ------------------------------------------------------------------------------------------------------------------------

def check_ini():
    t = "; my comment\r\n[coop]\r\nenable=0\r\nrole = host\r\n\r\n[other]\r\nx=1\r\n"
    out = fnv_ini.set_value(t, "coop", "enable", "1")
    assert out.split("\r\n")[2] == "enable=1" and "; my comment" in out and "[other]\r\nx=1" in out
    out = fnv_ini.set_value(out, "COOP", "ROLE", "client")
    assert fnv_ini.get(out, "coop", "role") == "client" and out.lower().count("role") == 1
    out = fnv_ini.set_value(out, "coop", "port", "27960")
    assert fnv_ini.get(out, "coop", "port") == "27960" and out.index("port=") < out.index("[other]")
    out = fnv_ini.set_value(out, "new", "a", "b")
    assert out.rstrip().endswith("[new]\r\na=b") and "\r\n" in out and "\n" not in out.replace("\r\n", "")
    assert fnv_ini.get("[a]\nk=v\n", "a", "k") == "v" and fnv_ini.get("", "a", "k") is None
    # injection: a value with a line break must not add keys or sections
    for bad in ("x\n[coop]\nrole=host", "x\rrole=host", "a\x00b"):
        must_raise(fnv_ini.IniError, lambda bad=bad: fnv_ini.set_value("[coop]\nenable=1\n", "coop", "name", bad))
    must_raise(fnv_ini.IniError, lambda: fnv_ini.set_value("", "co]op", "k", "v"))
    must_raise(fnv_ini.IniError, lambda: fnv_ini.set_value("", "coop", "k=x", "v"))
    with tmp() as td:
        p = td / "sub" / "x.ini"
        fnv_ini.write_atomic(p, "[a]\nk=v\n")
        assert fnv_ini.get(fnv_ini.read_text(p), "a", "k") == "v"
        assert not list(p.parent.glob("*.fnvcoop-tmp"))
        assert fnv_ini.read_text(td / "none.ini") == ""


# ---- manifest -------------------------------------------------------------------------------------------------------------------

def check_manifest_destinations():
    bad = ["../x.dll", "Data/NVSE/Plugins/../../FalloutNV.exe", "C:/Windows/x.dll", "/etc/passwd", "FalloutNV.exe", "Data/FalloutNV.esm",
           "Data/NVSE/Plugins/sub/x.dll", "Data\\NVSE\\Plugins\\..\\x.dll", "Data/NVSE/Plugins/", "", "//server/share/x.dll",
           "Data/NVSE/Plugins/a:b.dll", "Data/NVSE/Plugins/..", "Data/NVSE/Plugins/.", "Data/NVSE/Plugins/x.dll.", "Data/NVSE/Plugins/./x.dll", "Data/NVSE/Pluginsx/x.dll"]
    for d in bad:
        must_raise(fnv_manifest.ManifestError, lambda d=d: fnv_manifest.safe_dest(d))
    assert fnv_manifest.safe_dest("Data/NVSE/Plugins/nvcoop.dll") == "Data/NVSE/Plugins/nvcoop.dll"
    assert fnv_manifest.safe_dest("data\\nvse\\plugins\\nvcoop_rules.json") == "data/nvse/plugins/nvcoop_rules.json"
    with tmp() as td:   # a manifest that tries to escape is refused as a whole, nothing is installed
        rel = fg.make_release(td / "r", dests={"nvcoop.dll": "../FalloutNV.exe"})
        must_raise(fnv_manifest.ManifestError, lambda: fnv_manifest.load(rel))
        g = fg.make_game(td / "g")
        before = fg.tree(g)
        r = inst.install(g, rel, running=not_running, mygames=fg.make_mygames(td / "d"))
        assert not r.ok and fg.tree(g) == before, lines(r)
        rel2 = fg.make_release(td / "r2")
        m = json.loads((rel2 / "manifest.json").read_text())
        m["files"][1]["src"] = "../../outside.txt"
        (rel2 / "manifest.json").write_text(json.dumps(m))
        must_raise(fnv_manifest.ManifestError, lambda: fnv_manifest.load(rel2))
        m["files"][1]["src"] = "payload/nvcoop_rules.json"
        m["files"][2]["dest"] = m["files"][1]["dest"]
        (rel2 / "manifest.json").write_text(json.dumps(m))
        must_raise(fnv_manifest.ManifestError, lambda: fnv_manifest.load(rel2))


def check_manifest_detects_damage():
    with tmp() as td:
        rel = fg.make_release(td / "r")
        m = fnv_manifest.load(rel)
        assert fnv_manifest.verify_payload(rel, m) == []
        f = rel / "payload" / "nvcoop.dll"
        f.write_bytes(b"MZ-fake-dll-v2")        # same size, different bytes
        probs = fnv_manifest.verify_payload(rel, m)
        assert len(probs) == 1 and "SHA-256" in probs[0], probs
        f.write_bytes(b"short")
        assert "size" in fnv_manifest.verify_payload(rel, m)[0]
        f.unlink()
        assert "missing" in fnv_manifest.verify_payload(rel, m)[0]
        must_raise(fnv_manifest.ManifestError, lambda: fnv_manifest.load(td / "nope"))
    assert fnv_manifest.sums_text([{"path": "b", "sha256": "2"}, {"path": "a", "sha256": "1"}]) == "1 *a\n2 *b\n"


# ---- install --------------------------------------------------------------------------------------------------------------------

def check_install_fresh_and_status():
    with tmp() as td:
        g, rel, mg = setup(td)
        before_saves = fg.tree(g / "Saves")
        game_ini = (g / "Fallout.ini").read_bytes()
        r = do_install(g, rel, mg)
        assert r.ok, lines(r)
        pl = g / "Data" / "NVSE" / "Plugins"
        for name, data in fg.PAYLOAD_FILES.items():
            assert (pl / name).read_bytes() == data, name
        rec = inst.load_record(g)
        assert rec and rec["state"] == "installed" and rec["version"] == "1.0.0" and rec["game_version"] == "1.4.0.525"
        assert rec["created_dirs"] == ["Data/NVSE", "Data/NVSE/Plugins"], rec["created_dirs"]
        bdir = g / "FNVCoOp" / "backups" / rec["backup_ids"][0]
        meta = json.loads((bdir / "backup.json").read_text())
        kinds = sorted(Path(e["backup"]).name for e in meta["entries"])
        assert kinds == ["Fallout.ini", "Fallout.ini", "FalloutPrefs.ini"], kinds    # game dir + My Games copies; no plugin files existed
        assert (g / "Fallout.ini").read_bytes() == game_ini                           # the installer never edits the game INIs
        assert fg.tree(g / "Saves") == before_saves
        assert not list(pl.glob("*.fnvcoop-*"))
        st = inst.status(g)
        assert st.ok, lines(st)
        (pl / "nvcoop_rules.json").write_bytes(b"changed")
        (pl / "nvcoop_edids.json").unlink()
        st = inst.status(g)
        assert not st.ok and "CHANGED" in lines(st) and "MISSING" in lines(st)
        assert "Not installed" in lines(inst.status(td / "elsewhere"))


def check_install_dry_run():
    with tmp() as td:
        g, rel, mg = setup(td)
        before = fg.tree(g)
        r = do_install(g, rel, mg, dry_run=True)
        assert r.ok and "Dry run" in lines(r) and "nvcoop.dll" in lines(r)
        assert fg.tree(g) == before


def check_install_blocks_wrong_version():
    with tmp() as td:
        g, rel, mg = setup(td, exe_version=(1, 3, 0, 452))
        before = fg.tree(g)
        r = do_install(g, rel, mg)
        assert not r.ok and "1.3.0.452" in lines(r) and "Nothing was changed" in lines(r)
        assert fg.tree(g) == before
        r = do_install(g, rel, mg, force=True)
        assert r.ok and "overridden by --force" in lines(r)


def check_install_blocks_missing_nvse():
    with tmp() as td:
        g, rel, mg = setup(td, nvse=False)
        before = fg.tree(g)
        r = do_install(g, rel, mg, force=True)       # --force never overrides a missing xNVSE
        assert not r.ok and "xNVSE is not installed" in lines(r) and fg.tree(g) == before


def check_install_blocks_running_game():
    with tmp() as td:
        g, rel, mg = setup(td)
        before = fg.tree(g)
        r = do_install(g, rel, mg, running=lambda: True)
        assert not r.ok and "running" in lines(r) and fg.tree(g) == before
        do_install(g, rel, mg)
        r = inst.uninstall(g, running=lambda: True)
        assert not r.ok and inst.load_record(g) is not None


def check_install_blocks_damaged_payload():
    with tmp() as td:
        g, rel, mg = setup(td)
        (rel / "payload" / "nvcoop.dll").write_bytes(b"MZ-fake-dll-v9")
        before = fg.tree(g)
        r = do_install(g, rel, mg)
        assert not r.ok and "damaged" in lines(r) and fg.tree(g) == before


def check_install_refuses_folder_target():
    with tmp() as td:
        g, rel, mg = setup(td)
        (g / "Data" / "NVSE" / "Plugins" / "nvcoop.dll").mkdir(parents=True)
        r = do_install(g, rel, mg)
        assert not r.ok and "folder or a link" in lines(r) and inst.load_record(g) is None


def check_inside():
    with tmp() as td:
        (td / "a" / "b").mkdir(parents=True)
        assert inst.inside(td / "a", td / "a" / "b") and inst.inside(td / "a", td / "a")
        assert not inst.inside(td / "a", td / "a" / ".." / "x") and not inst.inside(td / "a", td)
        assert not inst.inside(td / "a", td / "ab")     # prefix of the name is not "inside"


def check_install_keeps_existing_ini_and_restores_it():
    with tmp() as td:
        g, rel, mg = setup(td)
        pl = g / "Data" / "NVSE" / "Plugins"
        pl.mkdir(parents=True)
        original = b"[coop]\r\nenable=1\r\nrole=host\r\nport=30000\r\n"
        (pl / "nvcoop.ini").write_bytes(original)
        assert do_install(g, rel, mg).ok
        assert rd(pl / "nvcoop.ini") == original, "install must keep the player's nvcoop.ini"
        (pl / "nvcoop.ini").write_bytes(b"[coop]\r\nenable=1\r\nrole=client\r\n")    # the launcher changes it later
        r = inst.uninstall(g, running=not_running)
        assert r.ok, lines(r)
        assert rd(pl / "nvcoop.ini") == original, "uninstall must restore the ini as it was before the install"


def check_install_backs_up_and_restores_old_files():
    with tmp() as td:
        g, rel, mg = setup(td)
        pl = g / "Data" / "NVSE" / "Plugins"
        pl.mkdir(parents=True)
        (pl / "nvcoop.dll").write_bytes(b"MZ-an-older-dev-build")
        (pl / "other_mod.dll").write_bytes(b"MZ-other-mod")
        assert do_install(g, rel, mg).ok
        assert (pl / "nvcoop.dll").read_bytes() == fg.PAYLOAD_FILES["nvcoop.dll"]
        rec = inst.load_record(g)
        assert "Data/NVSE/Plugins/nvcoop.dll" in rec["originals"], rec["originals"]
        r = inst.uninstall(g, running=not_running)
        assert r.ok, lines(r)
        assert rd(pl / "nvcoop.dll") == b"MZ-an-older-dev-build", "the older nvcoop.dll must come back"
        assert rd(pl / "other_mod.dll") == b"MZ-other-mod", "another mod's plugin must never be touched"
        assert not (pl / "nvcoop_rules.json").exists()
        assert not (g / "FNVCoOp").exists()


def check_uninstall_leaves_no_trace():
    with tmp() as td:
        g, rel, mg = setup(td)
        before = fg.tree(g)
        mg_before = fg.tree(mg)
        assert do_install(g, rel, mg).ok
        pl = g / "Data" / "NVSE" / "Plugins"
        (pl / "nvcoop.log").write_text("log")                     # files the plugin makes while it runs
        (pl / "nvcoop_cache").mkdir()
        (pl / "nvcoop_cache" / "core.dll").write_bytes(b"x")
        r = inst.uninstall(g, running=not_running, dry_run=True)
        assert r.ok and (pl / "nvcoop.dll").exists()               # a dry run removes nothing
        r = inst.uninstall(g, running=not_running)
        assert r.ok, lines(r)
        assert (pl / "nvcoop.log").exists() and (pl / "nvcoop.log").read_text() == "log", "logs are kept unless --remove-logs"
        assert (pl / "nvcoop_cache" / "core.dll").exists()
        shutil.rmtree(g / "Data" / "NVSE")                        # back to the starting state, then once more with --remove-logs
        assert do_install(g, rel, mg).ok
        (pl / "nvcoop.log").write_text("log")
        (pl / "nvcoop_cache").mkdir()
        (pl / "nvcoop_cache" / "core.dll").write_bytes(b"x")
        r = inst.uninstall(g, running=not_running, remove_logs=True)
        assert r.ok, lines(r)
        assert fg.tree(g) == before, set(fg.tree(g).items()) ^ set(before.items())
        assert fg.tree(mg) == mg_before
        assert "not installed" in lines(inst.uninstall(g, running=not_running)).lower()    # second uninstall is harmless
        assert do_install(g, rel, mg).ok                          # and the game can be installed again
        r = inst.uninstall(g, running=not_running, keep_backups=True)
        assert r.ok and (g / "FNVCoOp" / "backups").is_dir() and not (g / "FNVCoOp" / "install.json").exists()


def check_uninstall_keeps_user_changed_file():
    with tmp() as td:
        g, rel, mg = setup(td)
        assert do_install(g, rel, mg).ok
        pl = g / "Data" / "NVSE" / "Plugins"
        (pl / "nvcoop_rules.json").write_bytes(b'{"my": "edit"}')
        r = inst.uninstall(g, running=not_running)
        assert r.ok and not (pl / "nvcoop_rules.json").exists()
        saved = list((g / "FNVCoOp" / "removed").rglob("nvcoop_rules.json"))
        assert saved and saved[0].read_bytes() == b'{"my": "edit"}', "an edited file is copied aside, not destroyed"


class Boom(OSError):
    pass


def check_install_rolls_back_on_failure():
    with tmp() as td:
        g, rel, mg = setup(td)
        pl = g / "Data" / "NVSE" / "Plugins"
        pl.mkdir(parents=True)
        (pl / "nvcoop.dll").write_bytes(b"MZ-an-older-dev-build")
        before = fg.tree(g)
        calls = []

        def flaky(src, dst):
            calls.append(dst.name)
            if len(calls) == 3:
                raise Boom("disk full")
            inst.copy_verified(src, dst)
        r = do_install(g, rel, mg, copy_file=flaky)
        assert not r.ok and "put back" in lines(r) and len(calls) == 3, lines(r)
        assert fg.tree(g) == before, set(fg.tree(g).items()) ^ set(before.items())
        def denied(src, dst):
            raise PermissionError("access denied")
        r = do_install(g, rel, mg, copy_file=denied)
        assert not r.ok and "Run as administrator" in lines(r) and fg.tree(g) == before, lines(r)
        r = do_install(g, rel, mg)                       # and a clean retry works
        assert r.ok, lines(r)


def check_install_detects_bad_copy():
    with tmp() as td:
        g, rel, mg = setup(td)
        before = fg.tree(g)
        real = shutil.copyfile

        def corrupt(src, dst, **kw):
            real(src, dst, **kw)
            if str(src).endswith("nvcoop_rules.json"):
                with open(dst, "ab") as f:
                    f.write(b"!")
            return dst
        inst.shutil.copyfile = corrupt
        try:
            r = do_install(g, rel, mg)
        finally:
            inst.shutil.copyfile = real
        assert not r.ok and fg.tree(g) == before, lines(r)


def check_install_upgrade_keeps_originals():
    with tmp() as td:
        g, rel, mg = setup(td)
        pl = g / "Data" / "NVSE" / "Plugins"
        pl.mkdir(parents=True)
        (pl / "nvcoop.dll").write_bytes(b"MZ-an-older-dev-build")
        assert do_install(g, rel, mg).ok
        files2 = dict(fg.PAYLOAD_FILES)
        files2["nvcoop.dll"] = b"MZ-fake-dll-v2"
        rel2 = fg.make_release(td / "release2", version="1.1.0", files=files2)
        assert do_install(g, rel2, mg).ok
        assert (pl / "nvcoop.dll").read_bytes() == b"MZ-fake-dll-v2"
        rec = inst.load_record(g)
        assert rec["version"] == "1.1.0" and len(rec["backup_ids"]) == 2
        assert inst.uninstall(g, running=not_running).ok
        assert rd(pl / "nvcoop.dll") == b"MZ-an-older-dev-build", "after an upgrade the ORIGINAL file still comes back"


def check_restore_inis():
    with tmp() as td:
        g, rel, mg = setup(td)
        first = {n: (mg / n).read_bytes() for n in ("Fallout.ini", "FalloutPrefs.ini")}
        gd_first = (g / "Fallout.ini").read_bytes()
        assert do_install(g, rel, mg).ok
        (mg / "Fallout.ini").write_text("[General]\nbroken=1\n")
        (mg / "FalloutPrefs.ini").unlink()
        (g / "Fallout.ini").write_text("[General]\nbroken=2\n")
        r = inst.restore_inis(g, mygames=mg)
        assert r.ok, lines(r)
        assert rd(mg / "Fallout.ini") == first["Fallout.ini"] and rd(mg / "FalloutPrefs.ini") == first["FalloutPrefs.ini"]
        assert rd(g / "Fallout.ini") == gd_first
        assert (mg / "Fallout.ini.before-restore").exists()
        assert not inst.restore_inis(td / "other", mygames=mg).ok


def check_pick_game():
    with tmp() as td:
        a = fg.make_game(td / "a")
        b = fg.make_game(td / "b")
        msgs = []
        reg, dr = FakeReg(), lambda: []
        assert inst.pick_game(str(td / "nope"), env={}, reg=reg, drives=dr, log=msgs.append) is None
        # an explicit folder that is wrong must NOT fall back to another install that is found (that would install into the wrong game)
        assert inst.pick_game(str(td / "nope"), env={"FNV_GAME_DIR": str(a)}, reg=reg, drives=dr, log=msgs.append) is None
        assert inst.pick_game(str(a), env={}, reg=reg, drives=dr, log=msgs.append).path == a
        env = {"FNV_GAME_DIR": str(a)}
        assert inst.pick_game(None, env=env, reg=reg, drives=dr, log=msgs.append).path == a
        reg2 = FakeReg({("HKLM", r"SOFTWARE\WOW6432Node\Bethesda Softworks\FalloutNV", "Installed Path"): str(b)})
        assert inst.pick_game(None, env=env, reg=reg2, drives=dr, log=msgs.append) is None       # two installs, no way to ask
        assert inst.pick_game(None, env=env, reg=reg2, drives=dr, ask=lambda q: "2", log=msgs.append).path == b
        assert inst.pick_game(None, env={}, reg=reg, drives=dr, ask=lambda q: str(b), log=msgs.append).path == b
        assert inst.pick_game(None, env={}, reg=reg, drives=dr, log=msgs.append) is None


def check_cli():
    with tmp() as td:
        g, rel, mg = setup(td)
        docs = td / "docs"
        old_env = os.environ.get("FNV_DOCUMENTS_DIR")
        old_run = fnv_find.game_is_running
        os.environ["FNV_DOCUMENTS_DIR"] = str(docs)
        fnv_find.game_is_running = lambda runner=None: False
        old_game_dir = os.environ.pop("FNV_GAME_DIR", None)
        old_drives, old_reg = fnv_find.real_drives, fnv_find.WinRegistry
        fnv_find.real_drives = lambda: []          # no discovery on the real PC: the real game folder must stay out of reach
        fnv_find.WinRegistry = FakeReg
        try:
            assert inst.my_games_dir() == docs / "My Games" / "FalloutNV"
            run = lambda *a: quiet(inst.main, list(a))   # noqa: E731
            assert run("check", "--game", str(g)) == 0
            assert run("install", "--game", str(g), "--release", str(rel), "--dry-run") == 0 and inst.load_record(g) is None
            assert run("install", "--game", str(g), "--release", str(rel), "--yes") == 0 and inst.load_record(g) is not None
            assert run("status", "--game", str(g)) == 0
            assert run("restore-inis", "--game", str(g)) == 0
            assert run("uninstall", "--game", str(g), "--yes") == 0 and inst.load_record(g) is None and not (g / "FNVCoOp").exists()
            assert run("install", "--game", str(td / "nope"), "--release", str(rel), "--yes") == 2
            assert run("install", "--game", str(g), "--release", str(td / "norelease"), "--yes") == 1
            bad = fg.make_game(td / "bad", exe_version=(1, 3, 0, 452))
            assert run("install", "--game", str(bad), "--release", str(rel), "--yes") == 1
            assert run("check", "--game", str(bad)) == 1
        finally:
            fnv_find.game_is_running = old_run
            fnv_find.real_drives, fnv_find.WinRegistry = old_drives, old_reg
            if old_game_dir is not None:
                os.environ["FNV_GAME_DIR"] = old_game_dir
            if old_env is None:
                os.environ.pop("FNV_DOCUMENTS_DIR", None)
            else:
                os.environ["FNV_DOCUMENTS_DIR"] = old_env


def check_check_report():
    with tmp() as td:
        g = fg.make_game(td / "g")
        info, r = inst.check(g, running=not_running)
        assert r.ok and "1.4.0.525" in lines(r) and "6.3.4" in lines(r) and "not installed" in lines(r)
        info, r = inst.check(fg.make_game(td / "bad", exe_version=(1, 0, 0, 240)), running=not_running)
        assert not r.ok and "PROBLEM" in lines(r)


# ---- packaging ------------------------------------------------------------------------------------------------------------------

def fake_dll(tail=b"") -> bytes:
    return fg.make_pe((1, 0, 0, 0), tail=b"NVSEPlugin_Query\0NVSEPlugin_Load\0" + tail)


class Args:
    def __init__(self, td, **kw):
        self.version = "1.0.0"
        self.name = "FNV-CoOp"
        self.dll = str(td / "in" / "nvcoop.dll")
        self.rules = str(td / "in" / "rules.json")
        self.consent = str(td / "in" / "consent.txt")
        self.volatile = str(td / "in" / "volatile.txt")
        self.data_dir = [str(td / "in")]
        self.license = str(td / "in" / "LICENSE")
        self.out = str(td / "out")
        self.forbid = []
        self.source_date_epoch = 1767225600
        self.allow_missing = False
        self.dry_run = False
        self.__dict__.update(kw)


def pkg_inputs(td, dll=None):
    d = td / "in"
    d.mkdir(parents=True, exist_ok=True)
    (d / "nvcoop.dll").write_bytes(dll if dll is not None else fake_dll())
    (d / "rules.json").write_text('{"rules": []}')
    (d / "consent.txt").write_text("# consent\n")
    (d / "volatile.txt").write_text("key\n")
    (d / "LICENSE").write_text("MIT test licence\n")
    for n in ("nvcoop_edids.json", "nvcoop_infoquest.json", "nvcoop_guest_local_run.json", "nvcoop_intents.json", "nvcoop_cause_quests.json", "nvcoop_pack_watch.json"):
        (d / n).write_text("{}")


def check_package_builds_a_verified_zip():
    with tmp() as td:
        pkg_inputs(td)
        a = Args(td)
        assert quiet(pkg.build, a) == 0
        z = td / "out" / "FNV-CoOp-1.0.0.zip"
        assert z.is_file() and (td / "out" / "FNV-CoOp-1.0.0.zip.sha256").is_file()
        assert pkg.verify_zip(z) == []
        with zipfile.ZipFile(z) as zf:
            names = zf.namelist()
            root = "FNV-CoOp-1.0.0/"
            for need in ("manifest.json", "SHA256SUMS.txt", "Install.cmd", "Uninstall.cmd", "Launcher.cmd", "Check.cmd", "README-FIRST.txt",
                         "PLAYER-GUIDE.md", "LICENSE.txt", "THIRD-PARTY.txt", "payload/nvcoop.dll", "payload/nvcoop.ini",
                         "payload/nvcoop_rules.json", "payload/nvcoop_edids.json", "payload/nvcoop_infoquest.json",
                         "payload/nvcoop_guest_local_run.json", "payload/nvcoop_intents.json", "payload/nvcoop_cause_quests.json", "payload/nvcoop_pack_watch.json", "payload/nvcoop_consent_messages.txt", "payload/nvcoop_volatile_keys.txt",
                         "installer/fnv_install.py", "installer/fnv_manifest.py", "launcher/launcher_gui.py", "launcher/fnv_launcher_core.py",
                         "launcher/fnv_portcheck.py"):
                assert root + need in names, need
            assert not any("run_tests" in n or "/tests/" in n or "package_release" in n for n in names), "test and packaging code is not shipped"
            cmd = zf.read(root + "Install.cmd")
            assert b"\r\n" in cmd and b"\n" not in cmd.replace(b"\r\n", b"") and cmd.isascii() and b"python.org" in cmd
            m = json.loads(zf.read(root + "manifest.json"))
        assert m["game_version"] == "1.4.0.525" and m["nvse_min"] == "6.3.0" and m["version"] == "1.0.0"
        ini = [e for e in m["files"] if e["dest"].endswith("nvcoop.ini")][0]
        assert ini["keep_existing"] is True and all(e["dest"].startswith("Data/NVSE/Plugins/") for e in m["files"])
        # and the reference zip installs into a game exactly like a player's copy
        ex = td / "unz"
        with zipfile.ZipFile(z) as zf:
            zf.extractall(ex)
        g = fg.make_game(td / "game")
        mg = fg.make_mygames(td / "docs")
        r = inst.install(g, ex / "FNV-CoOp-1.0.0", running=not_running, mygames=mg)
        assert r.ok, lines(r)
        assert (g / "Data" / "NVSE" / "Plugins" / "nvcoop.dll").read_bytes() == (td / "in" / "nvcoop.dll").read_bytes()
        assert inst.status(g).ok


def check_package_is_reproducible():
    with tmp() as td:
        pkg_inputs(td)
        assert quiet(pkg.build, Args(td, out=str(td / "o1"))) == 0
        assert quiet(pkg.build, Args(td, out=str(td / "o2"))) == 0
        a = (td / "o1" / "FNV-CoOp-1.0.0.zip").read_bytes()
        b = (td / "o2" / "FNV-CoOp-1.0.0.zip").read_bytes()
        assert a == b, "same inputs must give the same zip"
        assert quiet(pkg.build, Args(td, out=str(td / "o3"), source_date_epoch=1700000000)) == 0
        assert (td / "o3" / "FNV-CoOp-1.0.0.zip").read_bytes() != a


def check_package_stops_on_missing_required():
    with tmp() as td:
        pkg_inputs(td)
        (td / "in" / "rules.json").unlink()
        assert quiet(pkg.build, Args(td)) == 1 and not (td / "out").exists()
        assert quiet(pkg.build, Args(td, allow_missing=True, dry_run=True)) == 0
        pkg_inputs(td)
        (td / "in" / "nvcoop_edids.json").unlink()
        assert quiet(pkg.build, Args(td)) == 1
        pkg_inputs(td)
        (td / "in" / "nvcoop_intents.json").unlink()    # without it the Fort, frisk and Dead Money lockers stay shared between players (G35)
        assert quiet(pkg.build, Args(td)) == 1
        pkg_inputs(td)
        (td / "in" / "volatile.txt").unlink()           # every player must have the same volatile-key list: required
        assert quiet(pkg.build, Args(td)) == 1
        pkg_inputs(td)
        assert quiet(pkg.build, Args(td, version="one")) == 2


def check_package_checks_the_dll():
    with tmp() as td:
        pkg_inputs(td, dll=fg.make_pe((1, 0, 0, 0), machine=0x8664, tail=b"NVSEPlugin_Query NVSEPlugin_Load"))
        assert quiet(pkg.build, Args(td)) == 1                    # 64-bit
        pkg_inputs(td, dll=fg.make_pe((1, 0, 0, 0)))
        assert quiet(pkg.build, Args(td)) == 1                    # no plugin exports
        pkg_inputs(td, dll=b"not a dll at all")
        assert quiet(pkg.build, Args(td)) == 1
        pkg_inputs(td)
        assert quiet(pkg.build, Args(td)) == 0


def check_package_privacy_scan():
    user = b"C:\\Users\\someone\\source\\x.cpp"
    assert pkg.scan_forbidden("a.dll", b"abc " + user, []), "ascii path"
    assert pkg.scan_forbidden("a.dll", "D:/Users/someone/x".encode("utf-16le"), []), "utf-16 path"
    assert pkg.scan_forbidden("a.dll", b"c:\\users\\SOMEONE\\x", [])
    assert not pkg.scan_forbidden("a.dll", b"C:\\Users\\Public\\x and C:\\Users\\Default\\y", [])
    assert pkg.scan_forbidden("a.txt", b"hello SecretName here", ["secretname"])
    assert pkg.scan_forbidden("a.txt", "hello SecretName".encode("utf-16le"), ["SECRETNAME"])
    assert not pkg.scan_forbidden("a.txt", b"nothing here", ["secretname"])
    with tmp() as td:
        pkg_inputs(td, dll=fake_dll(b"see " + user))
        assert quiet(pkg.build, Args(td)) == 1 and not (td / "out").exists(), "a dll with a personal path inside a longer string is never packaged"
        pkg_inputs(td)
        (td / "in" / "rules.json").write_text('{"note": "by SecretName"}')
        assert quiet(pkg.build, Args(td, forbid=["secretname"])) == 1
        assert quiet(pkg.build, Args(td)) == 0


def check_package_blanks_baked_build_paths():
    user = b"C:/Users/someone/<repo>/../proj/plugin/snapshot/data/volatile_keys.txt"
    data, n = pkg.scrub_paths(b"abc\0" + user + b"\0def\0")
    assert n == 1 and data == b"abc\0" + b"\0" * len(user) + b"\0def\0", data
    keep = b"x\0C:\\Users\\Public\\file.txt\0"
    assert pkg.scrub_paths(keep) == (keep, 0)
    inside = b"error in C:\\Users\\someone\\x.cpp: %s\0"
    assert pkg.scrub_paths(inside) == (inside, 0), "a path inside a longer string is not blanked"
    with tmp() as td:
        pkg_inputs(td, dll=fake_dll(user + b"\0"))
        assert quiet(pkg.build, Args(td)) == 0, "a path that is a whole string is blanked and the build goes through"
        with zipfile.ZipFile(td / "out" / "FNV-CoOp-1.0.0.zip") as z:
            dll = z.read("FNV-CoOp-1.0.0/payload/nvcoop.dll")
        assert b"someone" not in dll and b"NVSEPlugin_Load" in dll and len(dll) == len((td / "in" / "nvcoop.dll").read_bytes())
        m = json.loads(zipfile.ZipFile(td / "out" / "FNV-CoOp-1.0.0.zip").read("FNV-CoOp-1.0.0/manifest.json"))
        want = [e for e in m["files"] if e["src"] == "payload/nvcoop.dll"][0]
        assert want["sha256"] == fnv_manifest.sha256_bytes(dll), "the manifest describes the blanked DLL"


def check_verify_zip_detects_tampering():
    with tmp() as td:
        pkg_inputs(td)
        assert quiet(pkg.build, Args(td)) == 0
        z = td / "out" / "FNV-CoOp-1.0.0.zip"
        root = "FNV-CoOp-1.0.0/"

        def rewrite(change):
            out = td / "t.zip"
            with zipfile.ZipFile(z) as src, zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as dst:
                for n in src.namelist():
                    data = change(n, src.read(n))
                    if data is not None:
                        dst.writestr(n, data)
                change(None, None) and dst.writestr(root + "extra.txt", b"x")
            return out
        t = rewrite(lambda n, d: d + b"!" if n == root + "installer/fnv_install.py" else d)    # not in the manifest: only the checksum list sees it
        probs = pkg.verify_zip(t)
        assert probs and any("fnv_install.py" in p for p in probs), probs
        t = rewrite(lambda n, d: None if n == root + "Install.cmd" else d)
        assert any("missing" in p for p in pkg.verify_zip(t))
        t = rewrite(lambda n, d: (b"x" if n is None else d))
        assert any("not in SHA256SUMS" in p for p in pkg.verify_zip(t)), pkg.verify_zip(t)
        with zipfile.ZipFile(td / "two.zip", "w") as zf:
            zf.writestr("a/x", b"1")
            zf.writestr("b/y", b"2")
        assert pkg.verify_zip(td / "two.zip")
        with zipfile.ZipFile(td / "esc.zip", "w") as zf:
            zf.writestr("r/../evil.txt", b"1")
        assert pkg.verify_zip(td / "esc.zip")


# ---- the list -------------------------------------------------------------------------------------------------------------------

REAL = [check_pe_version, check_find_steam, check_find_steam_ansi_library_path, check_find_gog_and_retail,
        check_find_common_and_order, check_inspect, check_ini,
        check_manifest_destinations, check_manifest_detects_damage, check_install_fresh_and_status, check_install_dry_run,
        check_install_blocks_wrong_version, check_install_blocks_missing_nvse, check_install_blocks_running_game,
        check_install_blocks_damaged_payload, check_install_refuses_folder_target, check_inside,
        check_install_keeps_existing_ini_and_restores_it, check_install_backs_up_and_restores_old_files, check_uninstall_leaves_no_trace,
        check_uninstall_keeps_user_changed_file, check_install_rolls_back_on_failure, check_install_detects_bad_copy,
        check_install_upgrade_keeps_originals, check_restore_inis, check_pick_game, check_check_report, check_cli,
        check_package_builds_a_verified_zip, check_package_is_reproducible, check_package_stops_on_missing_required,
        check_package_checks_the_dll, check_package_privacy_scan, check_package_blanks_baked_build_paths,
        check_verify_zip_detects_tampering]

TWINS = [
    (check_pe_version, "pe_swap_words"),
    (check_find_steam, "find_ignore_libraries"),
    (check_find_steam_ansi_library_path, "find_no_ansi_fallback"),
    (check_find_common_and_order, "find_no_dedup"),
    *([(check_find_steam, "find_windows_only_separators"), (check_find_common_and_order, "find_windows_only_separators")]
      if os.name != "nt" else []),   # the old Windows-only path building is only wrong off Windows
    (check_inspect, "skip_version_check"), (check_inspect, "skip_nvse_check"), (check_inspect, "nvse_no_min"),
    (check_ini, "ini_allow_newline"),
    (check_manifest_destinations, "allow_traversal"), (check_manifest_destinations, "allow_any_dest"),
    (check_manifest_detects_damage, "skip_payload_hash"),
    (check_install_blocks_wrong_version, "skip_version_check"),
    (check_install_blocks_missing_nvse, "skip_nvse_check"),
    (check_install_blocks_running_game, "ignore_running"),
    (check_install_blocks_damaged_payload, "skip_payload_hash"),
    (check_install_fresh_and_status, "no_backup_inis"),
    (check_install_keeps_existing_ini_and_restores_it, "overwrite_ini"),
    (check_install_keeps_existing_ini_and_restores_it, "uninstall_no_restore"),
    (check_install_backs_up_and_restores_old_files, "no_backup"),
    (check_install_backs_up_and_restores_old_files, "uninstall_no_restore"),
    (check_install_backs_up_and_restores_old_files, "uninstall_remove_foreign"),
    (check_uninstall_leaves_no_trace, "uninstall_keep_files"),
    (check_uninstall_keeps_user_changed_file, "uninstall_delete_modified"),
    (check_install_rolls_back_on_failure, "no_rollback"),
    (check_install_detects_bad_copy, "no_verify_copy"),
    (check_install_upgrade_keeps_originals, "no_backup"),
    (check_pick_game, "pick_fallback"),
    (check_restore_inis, "no_backup_inis"),
    (check_package_is_reproducible, "zip_nondeterministic"),
    (check_package_stops_on_missing_required, "ignore_missing_required"),
    (check_package_privacy_scan, "skip_privacy_scan"),
    (check_package_blanks_baked_build_paths, "no_scrub"),
    (check_verify_zip_detects_tampering, "verify_skip_sums"),
    (check_package_checks_the_dll, "skip_dll_check"),
]


def real_game_fingerprint():
    """What the real game folders on this PC hold in Data\\NVSE\\Plugins (names, sizes, times) plus whether FNVCoOp exists. The tests
    must never change it; main() compares before and after."""
    out = {}
    try:
        for c in fnv_find.candidates():
            pl = c.path / "Data" / "NVSE" / "Plugins"
            out[str(c.path)] = (sorted((p.name, p.stat().st_size, p.stat().st_mtime_ns) for p in pl.glob("*")) if pl.is_dir() else None,
                                (c.path / "FNVCoOp").exists())
    except OSError:
        pass
    return out


def main() -> int:
    bad = []
    n = 0
    fingerprint = real_game_fingerprint()
    for fn in REAL:
        n += 1
        try:
            fn()
            print("ok   ", fn.__name__)
        except Exception:  # noqa: BLE001
            bad.append(fn.__name__)
            print("FAIL ", fn.__name__)
            traceback.print_exc()
    for fn, mut in TWINS:
        n += 1
        try:
            with fnv_mut.use(mut):
                fn()
        except AssertionError:
            print(f"ok    twin {fn.__name__}[{mut}] caught")
        except Exception:  # noqa: BLE001  (a crash is not a caught defect)
            bad.append(f"{fn.__name__}[{mut}]")
            print(f"FAIL  twin {fn.__name__}[{mut}] crashed instead of failing an assertion")
            traceback.print_exc()
        else:
            bad.append(f"{fn.__name__}[{mut}]")
            print(f"FAIL  twin {fn.__name__}[{mut}] NOT caught")
    if real_game_fingerprint() != fingerprint:
        bad.append("REAL GAME FOLDER CHANGED")
        print("FAIL  the tests changed a real game folder on this PC (Data\\NVSE\\Plugins or FNVCoOp)")
    print(f"\n{n} checks ({len(REAL)} real, {len(TWINS)} twins), {len(bad)} failed")
    if bad:
        print("FAILED:", ", ".join(bad))
        return 1
    print("ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
