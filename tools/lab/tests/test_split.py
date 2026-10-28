"""Offline tests of the two-PC smoke (no game, no ssh): the guards, the LAN numbers, the refusals that must start nothing, the report block, the command line.

    python -I tools/lab/tests/test_split.py
    python -I tools/lab/run_tests.py          # also the mutation twins (LAB_MUTANT=<name>)
"""
from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
LAB = HERE.parent
ROOT = LAB.parent.parent
sys.path.insert(0, str(LAB))

import lab                       # noqa: E402
import lab_layout as LY          # noqa: E402
import lab_report as R           # noqa: E402
import lab_split as S            # noqa: E402

PING = """Pinging 192.168.0.10 with 32 bytes of data:
Reply from 192.168.0.10: bytes=32 time=2ms TTL=128
Reply from 192.168.0.10: bytes=32 time=1ms TTL=128

Ping statistics for 192.168.0.10:
    Packets: Sent = 10, Received = 9, Lost = 1 (10% loss),
Approximate round trip times in milli-seconds:
    Minimum = 1ms, Maximum = 4ms, Average = 2ms
"""
GOOD = dict(free_gb=6.0, local_steam=True, local_user=True, remote_steam=True, remote_user=True, local_games=[], remote_ok=True)


class Guards(unittest.TestCase):
    def test_a_ready_pc_has_no_reason_to_refuse(self):
        self.assertEqual(S.guard_reasons(**GOOD), [])

    def test_under_three_gb_free_is_refused_with_a_clear_message(self):          # twin split_ram_unchecked
        why = S.guard_reasons(**dict(GOOD, free_gb=1.66))
        self.assertEqual(len(why), 1)
        self.assertIn("1.7 GB of free RAM", why[0])
        self.assertIn("refusing", why[0])
        self.assertIn("3 GB", why[0])
        self.assertEqual(S.guard_reasons(**dict(GOOD, free_gb=3.0)), [])

    def test_steam_not_running_here_is_a_reason_and_the_lab_does_not_start_it(self):          # twin split_steam_unchecked
        why = S.guard_reasons(**dict(GOOD, local_steam=False, local_user=False))
        self.assertEqual(len(why), 1)
        self.assertIn("Steam is not running on this PC", why[0])
        self.assertIn("does not start it", why[0])

    def test_steam_running_but_nobody_signed_in_is_a_reason(self):
        why = S.guard_reasons(**dict(GOOD, local_user=False))
        self.assertIn("nobody is signed in", why[0])
        why = S.guard_reasons(**dict(GOOD, remote_user=False))
        self.assertIn("second PC", why[0])
        self.assertIn("nobody is signed in", why[0])

    def test_steam_not_running_on_the_second_pc_is_a_reason(self):
        self.assertIn("Steam is not running on the second PC", S.guard_reasons(**dict(GOOD, remote_steam=False))[0])

    def test_an_unreachable_second_pc_is_a_reason_and_hides_the_steam_facts_it_could_not_read(self):
        why = S.guard_reasons(**dict(GOOD, remote_ok=False, remote_steam=False, remote_user=False))
        self.assertEqual(len(why), 1)
        self.assertIn("ssh", why[0])

    def test_a_game_already_running_here_is_never_stopped_just_refused(self):
        why = S.guard_reasons(**dict(GOOD, local_games=[4242]))
        self.assertIn("pid 4242", why[0])
        self.assertIn("does not stop what it did not start", why[0])

    def test_every_reason_is_reported_at_once_ram_first(self):
        why = S.guard_reasons(**dict(GOOD, free_gb=1.0, local_steam=False, local_games=[1], remote_ok=False))
        self.assertEqual(len(why), 4)
        self.assertIn("free RAM", why[0])

    def test_the_local_copy_must_be_an_instance_folder_never_the_stock_install(self):
        with tempfile.TemporaryDirectory() as t:
            root = Path(t) / "fnv-instances"
            (root / "1").mkdir(parents=True)
            self.assertTrue(S.instance_dir_ok(root / "1", root))
            self.assertFalse(S.instance_dir_ok(Path(r"C:\Program Files (x86)\Steam\steamapps\common\Fallout New Vegas"), root))
            self.assertFalse(S.instance_dir_ok(Path(t) / "elsewhere", root))
            self.assertFalse(S.instance_dir_ok(root / ".." / "steamapps" / "x", root))


class Numbers(unittest.TestCase):
    def test_parse_ping_reads_the_summary_and_the_loss(self):
        p = S.parse_ping(PING)
        self.assertEqual((p["min"], p["avg"], p["max"], p["loss_pct"]), (1.0, 2.0, 4.0, 10.0))

    def test_parse_ping_falls_back_to_the_reply_times_and_says_none_for_no_answer(self):
        p = S.parse_ping("Reply from x: bytes=32 time=3ms TTL=128\nReply from x: bytes=32 time=5ms TTL=128\n")
        self.assertEqual((p["min"], p["avg"], p["max"]), (3.0, 4.0, 5.0))
        self.assertIsNone(S.parse_ping("Request timed out.\nRequest timed out.\n"))

    def test_bridge_rtt_times_each_call(self):
        ticks = iter(x / 1000.0 for x in range(0, 100000, 7))
        nv = SimpleNamespace(call=lambda *a, **k: {"ok": True})
        r = S.bridge_rtt(nv, 5, clock=lambda: next(ticks))
        self.assertEqual(r["n"], 5)
        self.assertAlmostEqual(r["p50"], 7.0, places=1)

    def test_a_transport_with_a_round_trip_field_is_used_and_one_without_is_not(self):
        self.assertEqual(S.transport_rtt({"transport": {"kind": "enet", "rttMs": 3, "connected": True}}), 3.0)
        self.assertIsNone(S.transport_rtt({"transport": {"kind": "enet", "datagramsIn": 5, "connected": True}}))
        self.assertIsNone(S.transport_rtt({}))

    def test_lan_info_gathers_ping_both_bridges_and_says_where_the_rtt_comes_from(self):
        nv = SimpleNamespace(call=lambda *a, **k: {"transport": {"kind": "enet"}})
        info = S.lan_info("192.168.0.10", nv, nv, run=lambda *a, **k: SimpleNamespace(stdout=PING))
        self.assertEqual(info["ping_ms"]["avg"], 2.0)
        self.assertIn("p95", info["bridge_host_ms"])
        self.assertIn("p95", info["bridge_client_ms"])
        self.assertIsNone(info["transport_rtt_ms"])
        self.assertIn("bridge round trip", info["source"])

    def test_lan_info_survives_a_bridge_that_does_not_answer(self):
        def boom(*a, **k):
            raise OSError("pipe closed")
        info = S.lan_info("1.2.3.4", SimpleNamespace(call=boom), SimpleNamespace(call=boom), run=lambda *a, **k: SimpleNamespace(stdout=""))
        self.assertIsNone(info["ping_ms"])
        self.assertIn("pipe closed", info["bridge_host_ms_error"])


class Remote:
    """The second PC, recording what was asked of it. Anything that starts or stops a game is a mistake in a refused run."""

    def __init__(self):
        self.calls = []
        self.ip = "192.168.0.10"

    def push_lab(self, *a):
        self.calls.append("push_lab")

    def up(self):
        self.calls.append("up")

    def stop(self):
        self.calls.append("stop")


def env_with(facts, remote=None):
    env = object.__new__(S.SplitEnv)
    env.remote = remote or Remote()
    env.facts_fn = lambda: facts
    env.min_free_gb = 3.0
    env.remote_started = False
    env._ini_backup = None
    env.log = lambda s: None
    env.pv = SimpleNamespace(idir=lambda i: Path("C:/x/fnv-instances") / str(i), inst=SimpleNamespace(instances_root=lambda: Path("C:/x/fnv-instances")),
                             finish=lambda *a: None, unhold=lambda x: None, hold=lambda x: None)
    env.nvs, env.held, env.args = {}, set(), SimpleNamespace(keep=False)
    env.chaos, env.net_prof = {}, {}
    return env


class Refusals(unittest.TestCase):
    def test_low_ram_refuses_before_anything_is_pushed_started_or_stopped(self):
        env = env_with(dict(GOOD, free_gb=1.66))
        cls, why = env.prepare(0)
        self.assertEqual(cls, "ENV")
        self.assertIn("refusing", why)
        self.assertEqual(env.remote.calls, [])

    def test_steam_missing_on_either_pc_blocks_and_starts_nothing(self):
        for facts in (dict(GOOD, local_steam=False), dict(GOOD, remote_steam=False), dict(GOOD, local_user=False)):
            env = env_with(facts)
            cls, why = env.prepare(0)
            self.assertEqual(cls, "ENV", facts)
            self.assertIn("Steam", why)
            self.assertEqual(env.remote.calls, [])

    def test_finish_after_a_refusal_does_not_stop_anything_on_the_second_pc(self):          # twin split_stops_unstarted
        env = env_with(dict(GOOD, free_gb=1.0))
        env.prepare(0)
        env.finish()
        self.assertEqual(env.remote.calls, [], "this run started nothing there, so it stops nothing there")

    def test_finish_stops_what_the_run_started(self):
        env = env_with(GOOD)
        env.remote_started = True
        env.finish()
        self.assertEqual(env.remote.calls, ["stop"])

    def test_the_client_ini_is_put_back(self):
        with tempfile.TemporaryDirectory() as t:
            ini = Path(t) / "nvcoop.ini"
            ini.write_text("[coop]\nhost=192.168.0.10\n", encoding="latin-1")
            env = env_with(GOOD)
            env._ini_backup = (ini, "[coop]\nrole=client\n")
            env.finish()
            self.assertEqual(ini.read_text(encoding="latin-1"), "[coop]\nrole=client\n")

    def test_a_refused_run_through_the_runner_gives_blocked_env_rows_and_exit_4(self):
        lay = LY.Layout(LY.sample_layout())
        env = env_with(dict(GOOD, free_gb=1.66))
        env.worlds = None
        env.build_id = lambda: "x"
        env.pairs_allowed = lambda n: (1, "")
        res = lab.execute(lay, env, ["spawn", "walk", "quest"], 1, 600.0, 120.0, dict(lab.PARAMS), 0, "20990101-000000", "split", lambda s: None)
        self.assertEqual(res["verdict"], "BLOCKED")
        self.assertEqual(res["class"], "ENV")
        self.assertTrue(all("refusing" in r["reason"] for r in res["rooms"]))
        self.assertEqual({r["room"] for r in res["rooms"]}, {"spawn", "walk", "quest"})
        self.assertEqual(R.exit_code(res["verdict"], res["class"]), 4)


class Report(unittest.TestCase):
    def test_the_split_block_has_the_lan_numbers_and_the_room_timings(self):
        res = {"stamp": "s", "verdict": "PASS", "build": "b", "mode": "split", "pairs": 1, "seconds": 90, "layout_name": "l", "notes": [], "previous": {"stamp": None, "changes": []},
               "rooms": [{"room": "spawn", "pair": 0, "verdict": "PASS", "class": None, "seconds": 80, "checks": [], "metrics": {"puppet_seen_ms": 410}},
                         {"room": "quest", "pair": 0, "verdict": "PASS", "class": None, "seconds": 9, "checks": [], "metrics": {"host_stage_ms": 40, "client_stage_ms": 90}}],
               "split": {"ip": "192.168.0.10", "ping_ms": {"min": 1, "avg": 2, "max": 4, "loss_pct": 0}, "bridge_host_ms": {"p50": 12.0, "p95": 30.0},
                         "bridge_client_ms": {"p50": 3.0, "p95": 6.0}, "transport_rtt_ms": None}}
        txt = R.render_summary(res)
        self.assertIn("## Two-PC LAN", txt)
        self.assertIn("ping: min 1 / avg 2 / max 4 ms", txt)
        self.assertIn("host (over the ssh tunnel): p50 12 ms, p95 30 ms", txt)
        self.assertIn("none in this build", txt)
        self.assertIn("| quest | PASS stage 40/90ms |", txt)
        self.assertLess(len(txt.encode("utf-8")), 5000)


class Cli(unittest.TestCase):
    def cli(self, *args):
        cp = subprocess.run([sys.executable, "-I", str(LAB / "lab.py")] + list(args), capture_output=True, text=True, cwd=ROOT, timeout=300)
        return cp.returncode, cp.stdout, cp.stderr

    def test_split_does_not_combine_with_net_or_fake(self):
        self.assertEqual(self.cli("run", "--split", "--net", "good_wifi")[0], 2)
        self.assertEqual(self.cli("run", "--split", "--fake")[0], 2)

    def test_split_takes_the_remote_options(self):
        cp = subprocess.run([sys.executable, "-I", str(LAB / "lab.py"), "run", "--help"], capture_output=True, text=True, cwd=ROOT)
        for need in ("--split", "--remote ", "--remote-ip", "--min-free-gb"):
            self.assertIn(need, cp.stdout)


class Files(unittest.TestCase):
    def test_the_host_script_installs_the_lab_plugin_then_brings_up_the_one_copy(self):
        txt = (ROOT / "tools" / "bench" / "remote" / "lab_host_up.ps1").read_text(encoding="utf-8")
        self.assertIn("install_lab.py", txt)
        self.assertIn("--instance-dir C:\\fnv\\fnv-instances\\0", txt)
        self.assertIn("run.py up --n 1", txt)
        self.assertIn("C:\\fnv\\up_out.txt", txt)
        self.assertLess(txt.index("install_lab.py"), txt.index("run.py up"))

    def test_nothing_in_the_split_code_simulates_input_or_starts_steam(self):
        src = (LAB / "lab_split.py").read_text(encoding="utf-8").lower()
        for bad in ("sendkeys", "mouse_event", "pyautogui", "steam://", "os.startfile", "popen([\"steam", "popen([\"start"):
            self.assertNotIn(bad, src)

    def test_no_em_dash_or_personal_path(self):
        for p in (LAB / "lab_split.py", HERE / "test_split.py", ROOT / "tools" / "bench" / "remote" / "lab_host_up.ps1"):
            src = p.read_text(encoding="utf-8")
            self.assertNotIn(chr(0x2014), src, p.name)
            self.assertNotIn("mth" + "wj", src.lower(), p.name)


if __name__ == "__main__":
    unittest.main(verbosity=1)
