"""provision --no-pad: the copies ignore game controllers (FalloutPrefs.ini [Interface] bDisable360Controller=1).
In gamepad mode the game ignores the keyboard movement controls that bot.move holds. Under Proton on a Steam Deck, where
Steam Input always answers as a pad, a 3 s bot.move forward moved the player 0 units; with the key set to 1, 1008 units
(2026-10-25). Without --no-pad the owner's own value stays, so a person playing a copy keeps their controller.
Twins: render_inis ignoring the flag, a restore that drops it, a key written twice, a broken instance.json read as
"no --no-pad", a no_pad of "false" read through bool(), and run.py provision not forwarding the flag must each fail
the check.
Run: python -I -m unittest discover -s tools/runner/tests -p test_no_pad.py
"""
from __future__ import annotations

import contextlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
sys.path.insert(0, str(HERE.parents[1] / "pool"))
import instances  # noqa: E402
import lowcost  # noqa: E402

KEY = ("Interface", "bDisable360Controller")


def import_run():
    """tools/runner/run.py. Its job queue imports msvcrt (Windows only, never called here), so off Windows a stand-in is
    in sys.modules just for the import; subprocess is imported first, since it picks its platform code from msvcrt."""
    import subprocess  # noqa: F401
    import types
    stub = sys.platform != "win32" and "msvcrt" not in sys.modules
    if stub:
        sys.modules["msvcrt"] = types.ModuleType("msvcrt")
    try:
        import run
    finally:
        if stub:
            sys.modules.pop("msvcrt", None)
    return run


def run_forwarding_errors(provision_argv=None) -> list:
    """What `run.py provision` gets wrong about --no-pad: instances.py must get it exactly when it was given."""
    run, seen = import_run(), []
    patches = [mock.patch.object(run.instances, "main", lambda argv: seen.append(argv) or 0)]
    if provision_argv:
        patches.append(mock.patch.object(run, "provision_argv", provision_argv))
    with contextlib.ExitStack() as stack:
        for p in patches:
            stack.enter_context(p)
        run.main(["provision", "--n", "1", "--plugin-dll", "", "--no-pad"])
        run.main(["provision", "--n", "1", "--plugin-dll", ""])
    got = [argv.count("--no-pad") for argv in seen]
    return [] if got == [1, 0] else [f"--no-pad reached instances.py {got} times (want [1, 0])"]


def no_pad_errors(prefs: str) -> list:
    """What is wrong with a FalloutPrefs.ini that should make the game ignore pads."""
    errs = []
    if instances.ini_get(prefs, *KEY) != "1":
        errs.append(f"bDisable360Controller is {instances.ini_get(prefs, *KEY)!r}, not 1: the game stays in gamepad mode")
    n = sum(1 for ln in prefs.splitlines() if ln.strip().lower().startswith("bdisable360controller"))
    if n != 1:
        errs.append(f"bDisable360Controller appears {n} times")
    return errs


class NoPad(unittest.TestCase):
    def setUp(self):
        td = tempfile.TemporaryDirectory()
        self.addCleanup(td.cleanup)
        root = Path(td.name)
        self.game = root / "game"
        self.game.mkdir()
        (self.game / "FalloutNV.exe").write_bytes(b"MZ")
        docs = root / "docs"
        mg = docs / "My Games" / "FalloutNV"
        mg.mkdir(parents=True)
        (mg / "Fallout.ini").write_text("[General]\r\nbAlwaysActive=0\r\n", encoding="latin-1")
        # as on the Deck: the owner's prefs have the key, set to 0
        (mg / "FalloutPrefs.ini").write_text('[Display]\r\nsD3DDevice="Owner GPU"\r\n\r\n[Interface]\r\nbDisable360Controller=0\r\n'
                                             'fDlgFocus=2.1000\r\n', encoding="latin-1")
        self.root = root / "instances"
        env = mock.patch.dict(os.environ, {"FNV_GAME_DIR": str(self.game), "FNV_DOCUMENTS_DIR": str(docs),
                                           "FNV_INSTANCES_ROOT": str(self.root)})
        env.start()
        self.addCleanup(env.stop)

    def provision(self, no_pad: bool) -> Path:
        m = instances.provision(0, plugin_dll=None, no_pad=no_pad, root=self.root, appid=None)
        return Path(m["dir"])

    def test_no_pad_sets_the_key_once(self):
        self.assertEqual(no_pad_errors(instances.render_inis(no_pad=True)["FalloutPrefs.ini"]), [])

    def test_default_keeps_the_owners_value(self):
        self.assertEqual(instances.ini_get(instances.render_inis()["FalloutPrefs.ini"], *KEY), "0")

    def test_provision_writes_it_and_records_it(self):
        d = self.provision(True)
        self.assertEqual(no_pad_errors((d / "FalloutPrefs.ini").read_text(encoding="latin-1")), [])
        self.assertIs(json.loads((d / "instance.json").read_text())["no_pad"], True)
        d = self.provision(False)
        self.assertEqual(instances.ini_get((d / "FalloutPrefs.ini").read_text(encoding="latin-1"), *KEY), "0")
        self.assertIs(json.loads((d / "instance.json").read_text())["no_pad"], False)

    def test_cli_passes_the_flag(self):
        seen = []
        fake = lambda i, **kw: seen.append(kw["no_pad"]) or {"dir": str(self.root / str(i)), "link_counts": {}}  # noqa: E731
        with mock.patch.object(instances, "provision", fake), mock.patch("sys.stdout"):
            self.assertEqual(instances.main(["provision", "--n", "1", "--plugin-dll", "", "--no-pad"]), 0)
            self.assertEqual(instances.main(["provision", "--n", "1", "--plugin-dll", ""]), 0)
        self.assertEqual(seen, [True, False])

    def test_run_py_provision_forwards_the_flag(self):
        """run.py provision is the other provision command; it passes --no-pad on to instances.py (review)."""
        self.assertEqual(run_forwarding_errors(), [])

    def test_lowcost_restore_keeps_no_pad(self):
        d = self.provision(True)
        lowcost.restore(d, check=False)
        self.assertEqual(no_pad_errors((d / "FalloutPrefs.ini").read_text(encoding="latin-1")), [])

    def test_restore_without_instance_json_keeps_the_owners_value(self):
        d = self.provision(True)
        (d / "instance.json").unlink()
        lowcost.restore(d, check=False)
        self.assertEqual(instances.ini_get((d / "FalloutPrefs.ini").read_text(encoding="latin-1"), *KEY), "0")

    def check_broken_instance_json_stops_restore(self, provisioned_no_pad=None, text='{"instance": 0, "no_p'):
        """A broken instance.json (truncated by default) must stop restore before it writes: the copy's prefs stay as they are."""
        d = self.provision(True)
        (d / "instance.json").write_text(text)
        before = (d / "FalloutPrefs.ini").read_bytes()
        patch = mock.patch.object(lowcost, "provisioned_no_pad", provisioned_no_pad) if provisioned_no_pad else contextlib.nullcontext()
        with patch, self.assertRaises(lowcost.UnsafeTarget):
            lowcost.restore(d, check=False)
        self.assertEqual((d / "FalloutPrefs.ini").read_bytes(), before)

    def test_broken_instance_json_stops_restore(self):
        self.check_broken_instance_json_stops_restore()
        d = self.provision(True)
        (d / "instance.json").write_text("[1, 2]")
        with self.assertRaises(lowcost.UnsafeTarget):
            lowcost.restore(d, check=False)

    def test_non_boolean_no_pad_stops_restore(self):
        """"no_pad": "false" (a string) is neither --no-pad nor its absence (review)."""
        for value in ('"false"', '0', 'null'):
            with self.subTest(no_pad=value):
                self.check_broken_instance_json_stops_restore(text='{"instance": 0, "no_pad": %s}' % value)

    # ---- twins
    def test_twin_run_py_dropping_the_flag_is_caught(self):
        run = import_run()
        real = run.provision_argv
        dropping = lambda a: [x for x in real(a) if x != "--no-pad"]  # noqa: E731  (the first version never forwarded it)
        self.assertNotEqual(run_forwarding_errors(dropping), [])

    def test_twin_no_pad_coerced_with_bool_is_caught(self):
        def coercing(instance_dir):   # the second version: valid JSON, but no_pad passed through bool()
            return bool(json.loads((Path(instance_dir) / "instance.json").read_text()).get("no_pad"))
        with self.assertRaises(AssertionError):
            self.check_broken_instance_json_stops_restore(coercing, text='{"instance": 0, "no_pad": "false"}')

    def test_twin_broken_instance_json_read_as_no_flag_is_caught(self):
        def lenient(instance_dir):   # the first version: anything unreadable counted as "no --no-pad"
            try:
                return bool(json.loads((Path(instance_dir) / "instance.json").read_text()).get("no_pad"))
            except (OSError, ValueError, AttributeError):
                return False
        with self.assertRaises(AssertionError):
            self.check_broken_instance_json_stops_restore(lenient)

    def test_twin_render_ignoring_the_flag_is_caught(self):
        with mock.patch.object(instances, "NO_PAD_EDITS", []):
            self.assertNotEqual(no_pad_errors(instances.render_inis(no_pad=True)["FalloutPrefs.ini"]), [])

    def test_twin_restore_dropping_the_flag_is_caught(self):
        d = self.provision(True)
        with mock.patch.object(lowcost, "provisioned_no_pad", lambda _d: False):
            lowcost.restore(d, check=False)
        self.assertNotEqual(no_pad_errors((d / "FalloutPrefs.ini").read_text(encoding="latin-1")), [])

    def test_twin_key_written_twice_is_caught(self):
        prefs = instances.render_inis()["FalloutPrefs.ini"] + "\r\n[Interface]\r\nbDisable360Controller=1\r\n"
        self.assertNotEqual(no_pad_errors(prefs), [])


if __name__ == "__main__":
    unittest.main()
