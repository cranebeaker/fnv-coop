"""Unit tests for tools/proof (no game, no Steam, no real kill). Run through run_tests.py, which also runs the mutation twins.

    python -I tools/proof/tests/test_proof.py
"""
from __future__ import annotations

import contextlib
import io
import json
import os
import struct
import sys
import tempfile
import time
import unittest
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
PROOF = HERE.parent
ROOT = PROOF.parent.parent
sys.path.insert(0, str(PROOF))

import pf_classify as C   # noqa: E402
import pf_crash as K      # noqa: E402
import pf_ledger as L     # noqa: E402
import pf_packet as P     # noqa: E402
import pf_safety as S     # noqa: E402
import pf_saves as V      # noqa: E402
import pf_sched as X      # noqa: E402
import prove              # noqa: E402

STUB = "tools/proof/tests/stub_runner.py"
FAKE_HOME = "C:" + "\\Us" + "ers\\someone"      # built in pieces so no scanner mistakes a test constant for a real path


def test(**kw):
    t = {"id": "t1", "roadmap": 2, "n": 1, "copies": 2, "snapshot": [], "commands": [{"name": "stub", "argv": [STUB, "0"]}], "also_roadmap": []}
    t.update(kw)
    return t


class Classify(unittest.TestCase):
    def test_table(self):
        rows = [
            ({"exit": 0}, ("PASS", None)),
            ({"timeout": True}, ("TIMEOUT", "ENV")),
            ({"env_reason": "owner is playing"}, ("ENV", "ENV")),
            ({"crash": {"eip": "0x1"}, "exit": -2}, ("CRASH", "CRASH")),
            ({"build_reason": "bridge method missing"}, ("BLOCKED", "BUILD")),
            ({"exit": 3}, ("BLOCKED", "BUILD")),
            ({"exit": 2, "setup_text": "cannot reach the game instances: x"}, ("ENV", "ENV")),
            ({"exit": 2, "setup_text": "unrecognized arguments"}, ("FAIL", "TEST")),
            ({"exit": 1, "from_failure": "reproduced"}, ("FAIL", "SYNC")),
            ({"exit": 1, "from_failure": "not-reproduced", "inspect_sync": ["instance 1: 2 digest mismatch(es), 0 repaired"]}, ("FAIL", "SYNC")),
            ({"exit": 1, "from_failure": "not-reproduced"}, ("FAIL", "TEST")),
            ({"exit": 1, "from_failure": "no-recordings", "failure_kind": "snapdiff"}, ("FAIL", "SYNC")),
            ({"exit": 1, "from_failure": "no-recordings", "failure_kind": "arrange"}, ("FAIL", "TEST")),
            ({"exit": 1}, ("FAIL", "TEST")),
            ({"exit": 1, "failure_kind": "bridge", "reason": "RpcError [-32601] method not found"}, ("BLOCKED", "BUILD")),
            ({"exit": 0, "bad_counters": ["0:coop.status.desyncs.digests.repairs +2"]}, ("FAIL", "SYNC")),
        ]
        for ev, (verdict, cls) in rows:
            r = C.classify(ev)
            self.assertEqual((r["verdict"], r["class"]), (verdict, cls), ev)

    def test_reproduced_sync_is_never_test(self):
        r = C.classify({"exit": 1, "from_failure": "reproduced"})
        self.assertEqual(r["class"], "SYNC", "a reproduced divergence must be routed as SYNC, not as a test problem")
        self.assertEqual(r["repro"], "REPRODUCED")

    def test_counters(self):
        before = C.snapshot_flat({"coop.status": {0: {"desyncs": {"digests": {"repairs": 0, "ok": 5}}, "lastSeq": 10}}})
        after = C.snapshot_flat({"coop.status": {0: {"desyncs": {"digests": {"repairs": 2, "ok": 9}}, "lastSeq": 90}}})
        mv = C.moved(before, after)
        self.assertEqual(mv["0:coop.status.desyncs.digests.repairs"], 2)
        sus = C.suspicious_moved(mv)
        self.assertEqual(list(sus), ["coop.status.desyncs.digests.repairs"])      # ok and lastSeq are noise
        self.assertTrue(C.unclean(mv))
        self.assertEqual(C.unclean(mv, allow_repairs=True), [])
        self.assertEqual(C.unclean({"1:areadigest.status.heals.total": 1}), ["1:areadigest.status.heals.total +1"])
        self.assertEqual(C.unclean({"0:coop.loot.hostStats.closeMismatch": 1}, True, ["closeMismatch"]), ["0:coop.loot.hostStats.closeMismatch +1"])

    def test_reason_and_check(self):
        text = "bridge methods\n  PRESENT x\nFAIL  enemy_fight  4 steps  it fell twice\nsummary: x"
        self.assertTrue(C.reason_from_output(text).startswith("FAIL"))
        self.assertLessEqual(len(C.reason_from_output("FAIL " + "x" * 500)), 200)
        self.assertEqual(C.check_name({"step": "04-both kill"}, "", "run"), "04-both kill")


class Packet(unittest.TestCase):
    def ctx(self, lines=120, width=300):
        v = {"test": "enemy_fight", "run": 3, "of": 3, "verdict": "FAIL", "class": "SYNC", "reason": "r" * 200, "check": "c" * 90, "repro": "REPRODUCED",
             "rerun": "python -I tools/bots/run_fight.py --instances 0,1 --which fight --from-step 4 --timescale 3"}
        return {"verdict": v, "failure": {"playbook": "enemy_fight", "step": "s" * 100, "kind": "check",
                                          "machines": [{"name": "host", "role": "host", "status": {"desyncs": {"digests": {"ok": 1, "mismatch": 2, "repairs": 3}}}}] * 3},
                "inspect": [{"name": "0", "reachable": False, "error": "e" * 200}], "inspect_problems": ["p" * 200] * 6,
                "moved": {f"coop.status.counter{i}": i + 1 for i in range(40)},
                "from_failure": {"verdict": "reproduced", "agreement": "a" * 300, "primary": {"class": "diverge", "key": "k" * 80, "at_ms": 1.5, "suspects": [1]},
                                 "case": "tools/flight/regress/x.json"},
                "crash": {"module": "FalloutNV.exe", "offset": "1234", "eip": "0xDEADBEEF", "last_calls": ["0:coop.status"] * 3},
                "warn_lines": {0: ["WARN line %d %s" % (i, "w" * width) for i in range(lines)], 1: ["ERR %d %s" % (i, "e" * width) for i in range(lines)]},
                "home": FAKE_HOME}

    def test_under_three_kb(self):
        text = P.build(self.ctx())
        self.assertLess(len(text.encode("utf-8")), 3072)
        self.assertIn("enemy_fight run 3/3", text)
        self.assertIn("rerun:", text)
        self.assertIn("crash", text)

    def test_small_packet_keeps_log_lines(self):
        text = P.build(self.ctx(lines=3, width=20))
        self.assertIn("WARN line 2", text)

    def test_no_user_path(self):
        c = self.ctx(lines=3, width=10)
        c["home"] = ""                       # the pattern alone must catch it, not the known home folder
        c["warn_lines"][0].append("opened " + FAKE_HOME + "\\Documents\\x.esm failed")
        self.assertNotIn("someone", P.build(c))

    def test_write_saves_full_lines(self):
        with tempfile.TemporaryDirectory() as d:
            p = P.write(Path(d), self.ctx())
            self.assertTrue(p.exists() and (Path(d) / "warn_inst0.txt").exists())
            self.assertEqual(len((Path(d) / "warn_inst0.txt").read_text().splitlines()), 120)
            self.assertLess(p.stat().st_size, 3072)


class Ledger(unittest.TestCase):
    def e(self, verdict="PASS", fresh=False, **kw):
        d = {"test": "t1", "verdict": verdict, "class": None if verdict == "PASS" else "SYNC", "fresh": fresh, "clean": verdict == "PASS", "build": "b1"}
        d.update(kw)
        return d

    def test_proven_needs_n_clean_and_a_fresh_one(self):
        t = {"id": "t1", "n": 3}
        self.assertEqual(L.state_of(t, [self.e(), self.e(), self.e()])["state"], "partial")          # no fresh boot among them
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e(), self.e()])["state"], "proven")
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e()])["state"], "partial")
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e(), self.e("FAIL")])["state"], "failing")
        self.assertEqual(L.state_of(t, [self.e("FAIL"), self.e(fresh=True), self.e(), self.e()])["state"], "proven")
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e(), self.e("FLAKY"), self.e()])["state"], "partial")

    def test_env_does_not_break_a_streak_and_fake_never_counts(self):
        t = {"id": "t1", "n": 2}
        env = self.e("ENV", **{"class": "ENV"})
        self.assertEqual(L.state_of(t, [self.e(fresh=True), env, self.e()])["state"], "proven")
        self.assertEqual(L.state_of(t, [self.e(fresh=True, fake=True), self.e(fake=True)])["state"], "not_run")
        self.assertEqual(L.state_of(t, [self.e(fresh=True, fake=True), self.e(fake=True)], include_fake=True)["state"], "proven")

    def test_short_passes_never_count_toward_proven_and_status_says_so(self):
        """A pass recorded by --runs below N is kept but is not one of the N runs."""
        t = {"id": "t1", "n": 3}
        sh = self.e(short=True)
        s = L.state_of(t, [self.e(fresh=True, short=True), sh, sh])
        self.assertEqual((s["state"], s["clean"], s["short"]), ("not_run", 0, 3))
        self.assertIn("3 short pass(es)", s["detail"])
        self.assertIn("--runs", s["detail"])
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e(), sh, self.e()])["state"], "proven", "a short pass neither extends nor breaks the streak")
        self.assertEqual(L.state_of(t, [self.e(fresh=True), self.e(), self.e(), self.e("FAIL", short=True)])["state"], "failing", "a short FAIL still fails")
        self.assertEqual(L.streak([self.e(), sh], "t1")["any_clean"], 2)

    def test_other_build_does_not_count(self):
        t = {"id": "t1", "n": 1}
        self.assertEqual(L.state_of(t, [self.e(fresh=True, build="old")], build="new")["state"], "not_run")

    def test_blocked_and_table(self):
        m = {"tests": [{"id": "a", "roadmap": 1, "n": 1}, {"id": "b", "roadmap": 1, "n": 1, "blocked_reason": "hand step"},
                       {"id": "c", "roadmap": 2, "n": 1, "also_roadmap": [1]}]}
        rows = L.step_table(m, [{"test": "a", "verdict": "PASS", "clean": True, "fresh": True}])
        self.assertEqual((rows[0]["total"], rows[0]["proven"], rows[0]["pct"]), (3, 1, 33))
        self.assertEqual(rows[0]["tests"]["b"]["state"], "blocked")
        self.assertIn("step  1", L.render_table(rows))

    def test_file_roundtrip_skips_torn_line(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "l.jsonl"
            L.append(p, {"test": "a", "verdict": "PASS"})
            with open(p, "a") as f:
                f.write('{"test": "b", "verd')
            L.append(p, {"test": "c", "verdict": "FAIL"})
            self.assertEqual([r["test"] for r in L.load(p)], ["a", "c"])


class Safety(unittest.TestCase):
    ROOTS = [Path(r"C:\games\fnv-instances"), Path(r"C:\games\fnv-instances-gate")]

    def pre(self, procs, steam=True, ram=None):
        def pids(img):
            return [p for p, _ in procs] if img == "FalloutNV.exe" else ([4] if steam and img == "steam.exe" else [])
        paths = dict(procs)
        return S.preflight(ROOT, 2, 0, pids=pids, path_of=lambda p: paths.get(p), roots=self.ROOTS, free_ram=lambda: 8 << 30,
                           ram_problem=lambda *a: ram)

    def test_owner_playing_is_env(self):
        why = self.pre([(10, r"C:\Program Files (x86)\Steam\steamapps\common\Fallout New Vegas\FalloutNV.exe")])
        self.assertTrue(why and why.startswith("owner is playing"), why)

    def test_unknown_path_counts_as_foreign(self):
        self.assertTrue(self.pre([(10, None)]))

    def test_test_instances_are_fine(self):
        self.assertIsNone(self.pre([(10, r"C:\games\fnv-instances\0\FalloutNV.exe"), (11, r"C:\games\fnv-instances-gate\1\FalloutNV.exe")]))

    def test_sibling_name_trick_is_foreign(self):
        self.assertTrue(S.foreign_game_procs(self.ROOTS, lambda i: [10], lambda p: r"C:\games\fnv-instancesX\0\FalloutNV.exe"))

    def test_steam_off_is_env(self):
        self.assertIn("Steam", self.pre([], steam=False))

    def test_ram_guard(self):
        self.assertIn("RAM guard", self.pre([], ram="refusing 2 pair(s)"))

    def test_lock_one_queue_at_a_time(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "q.lock"
            a, b = S.QueueLock(p, alive=lambda pid: True), S.QueueLock(p, alive=lambda pid: True)
            self.assertIsNone(a.acquire())
            self.assertIn("another queue runner", b.acquire() or "")
            a.release()
            self.assertIsNone(b.acquire())
            b.release()
            p.write_text(json.dumps({"pid": 99999}))
            self.assertIsNone(S.QueueLock(p, alive=lambda pid: False).acquire(), "a dead owner's lock is taken over")

    def test_never_kills_a_process_it_did_not_start(self):
        with tempfile.TemporaryDirectory() as d:
            owned = S.Owned(Path(d) / "o.json")
            owned.add(0, 111)
            killed = []
            with self.assertRaises(S.NotOwned):
                S.kill_owned(222, owned, pids=lambda i: [111, 222], kill=killed.append)
            self.assertEqual(killed, [])
            self.assertTrue(S.kill_owned(111, owned, pids=lambda i: [111, 222], kill=killed.append))
            self.assertEqual(killed, [111])
            self.assertFalse(S.kill_owned(111, owned, pids=lambda i: [], kill=killed.append), "a reused pid that is no FalloutNV.exe is left alone")
            self.assertEqual(killed, [111])


class Crash(unittest.TestCase):
    def test_crash_line(self):
        log = "a\n[nvcoop] CRASH code=C0000005 addr=0x00AB1234 module=C:\\g\\fnv-instances\\0\\Data\\NVSE\\Plugins\\nvcoop_mod.dll offset=1F2A0 frames=1204\nb"
        c = K.parse_crash_line(log)
        self.assertEqual((c["code"], c["module"], c["offset"], c["frames"]), ("C0000005", "nvcoop_mod.dll", "1F2A0", 1204))
        self.assertIsNone(K.parse_crash_line("nothing here"))
        self.assertIn("nvcoop_mod.dll+1F2A0", K.one_line(dict(c, instance=1)))

    def test_minidump_exception_stream(self):
        # header (32 bytes), directory with one entry at 32, exception stream at 64
        hdr = b"MDMP" + struct.pack("<IIIIII", 0xA793, 1, 32, 0, 0, 0) + b"\0" * 4
        entry = struct.pack("<III", 6, 168, 64)
        stream = struct.pack("<II", 77, 0) + struct.pack("<IIQQ", 0xC0000005, 0, 0, 0x004A1B2C) + b"\0" * 140
        data = hdr + entry + b"\0" * (64 - len(hdr) - len(entry)) + stream
        self.assertEqual(K.parse_minidump(data), {"code": "C0000005", "eip": "0x4A1B2C", "thread": 77})
        self.assertIsNone(K.parse_minidump(b"nope"))

    def test_watcher_reads_exit_after_the_process_is_gone(self):
        state = {10: None}
        w = K.Watcher(open_handle=lambda pid: pid, get_exit=lambda h: state[h])
        w.watch(0, 10)
        self.assertEqual(w.exited(), [])
        state[10] = 0xC0000005
        self.assertEqual(w.exited(), [{"instance": 0, "pid": 10, "exit_code": "C0000005"}])

    def test_collect_reads_log_and_trace(self):
        with tempfile.TemporaryDirectory() as d:
            d = Path(d)
            (d / "Data" / "NVSE" / "Plugins").mkdir(parents=True)
            (d / "Data" / "NVSE" / "Plugins" / "nvcoop.log").write_text("CRASH code=C0000005 addr=0x1 module=x\\nvcoop.dll offset=AB frames=3\n")
            tr = d / "t.json"
            tr.write_text(json.dumps([{"inst": "-1", "method": "coop.status"}, {"inst": "-0", "method": "ref.info", "error": "ConnectionLost: gone"}]))
            info = K.collect(d, 0, tr)
            self.assertEqual(info["offset"], "AB")
            self.assertEqual(info["last_calls"][-1], "-0:ref.info ERR ConnectionLost: gone")


class Manifest(unittest.TestCase):
    def setUp(self):
        self.m = prove.load_manifest(PROOF / "manifest.json")

    def test_valid(self):
        self.assertEqual(prove.check_manifest(self.m), [])

    def test_every_step_has_tests_and_n_is_sane(self):
        steps = {t["roadmap"] for t in self.m["tests"]} | {s for t in self.m["tests"] for s in t.get("also_roadmap", [])}
        self.assertEqual(steps, set(range(1, 11)))
        for t in self.m["tests"]:
            self.assertIn(t["n"], (1, 2, 3), t["id"])
            self.assertIn(t["copies"], (0, 1, 2, 3, 4), t["id"])
            if t.get("blocked_reason"):
                self.assertGreater(len(t["blocked_reason"]), 20)

    def test_plan_table_is_covered(self):
        ids = {t["id"] for t in self.m["tests"]}
        for need in ("actor_port", "live_wiring_all", "npc_caps", "npc_sync", "combat_proof", "spawn_proof", "enemy_fight", "enemy_handoff", "enemy_spawn",
                     "corpse_loot_race", "enemy_playbooks_offline", "puppet_loopback", "avatars_phases", "vats_classes", "social_proof", "loot_proof", "loot_race",
                     "door_terminal_container_matrix", "worldtime_proof", "pnr_vote", "fast_travel_tether", "save_load_rejoin", "area_proof", "qs_goodsprings",
                     "qs_yesman", "qs_prologue", "qs_house", "intents_proof", "story_capture", "ql_deadmoney", "release_cmd_scripts", "lan_two_pc"):
            self.assertIn(need, ids)

    def test_step10_pieces_and_avatar_params_are_runnable(self):
        by = {t["id"]: t for t in self.m["tests"]}
        vars_ = dict(self.m["params"], instances="0,1", instances_sp="0 1", host=0, client=1, i0=0, inst_dir0=".", game_dir=".", outdir="out/x")
        for tid in ("release_cmd_scripts", "launcher_loopback", "public_tree_build", "splitscreen_smoke", "enemy_chaos_bad_wifi", "avatars_lifecycle"):
            t = by[tid]
            self.assertFalse(t.get("blocked_reason"), tid)
            for c in t["commands"]:
                argv, missing = prove.expand(c["argv"], vars_)
                self.assertIsNone(missing, f"{tid}: param {missing} has no value in the manifest")
                self.assertTrue((ROOT / argv[0]).exists(), argv[0])
        a = by["avatars_lifecycle"]["commands"][-1]["argv"]   # the scenario load comes first, the proof last
        self.assertEqual(self.m["params"]["avatar_save"], "autosave", "the save every scenario in scenarios/ starts from")
        self.assertEqual((self.m["params"]["avatar_interior"], self.m["params"]["avatar_exterior"]), ("GSDocMitchellHouse", "Goodsprings"))
        self.assertIn("{avatar_interior}", a)
        ch = by["enemy_chaos_bad_wifi"]["commands"][0]["argv"]
        self.assertEqual(ch[0], "tools/chaos/chaos_pair.py")
        self.assertEqual(ch[ch.index("--info-file") + 1], ch[ch.index("--chaos-rpc") + 1], "the wrapper writes the info file the fight reads")
        self.assertEqual((ch[ch.index("--chaos") + 1], ch[ch.index("--chaos-seed") + 1]), ("bad_wifi", "7"))
        ss = by["splitscreen_smoke"]
        self.assertTrue(ss["alone"] and ss["launches"] == 2 and ss["copies"] == 0)

    def test_expand(self):
        v = {"a": "1", "b": None, "instances_sp": "0 1 2"}
        self.assertEqual(prove.expand(["x", "{a}", "{instances_sp}"], v), (["x", "1", "0", "1", "2"], None))
        self.assertEqual(prove.expand(["x", "{b}"], v), (None, "b"))
        self.assertEqual(prove.expand(["x", ["--opt", "{b}"], ["--y", "{a}"]], v), (["x", "--y", "1"], None))

    def test_instances_of(self):
        self.assertEqual(prove.instances_of({"copies": 2}), [0, 1])
        self.assertEqual(prove.instances_of({"copies": 2, "pair": 1}), [2, 3])
        self.assertEqual(prove.instances_of({"copies": 3}), [0, 1, 2])
        self.assertEqual(prove.instances_of({"copies": 0}), [])


class LiveStub(prove.NullEnv):
    """A pretend game: canned answers for everything that would touch a game."""
    live = True

    def __init__(self, **kw):
        super().__init__()
        self.kw = kw
        self.polls = 0
        self.snaps = []

    def idir(self, i):
        return Path(tempfile.gettempdir())

    def preflight(self, test):
        return self.kw.get("env")

    def prepare(self, test, instances):
        return self.kw.get("env"), self.kw.get("build"), self.kw.get("fresh", True)

    def snapshot(self, instances, methods):
        if self.kw.get("repairs") is None:
            return {}
        self.snaps.append(1)       # odd calls are the "before" snapshot of a run, even calls the "after"
        return {"coop.status": {0: {"desyncs": {"digests": {"repairs": 0 if len(self.snaps) % 2 else self.kw["repairs"]}}}}}

    def inspect(self, instances):
        return [{"name": "0", "reachable": True}], self.kw.get("problems", [])

    def crashes(self, instances=None):
        self.polls += 1
        return [{"instance": 1, "pid": 5, "exit_code": "C0000005"}] if self.kw.get("crash") and self.polls > 1 else []

    def crash_info(self, instance, since, trace):
        return {"instance": instance, "code": "C0000005", "eip": "0x4A1B2C", "module": "nvcoop_mod.dll", "offset": "9F10", "last_calls": ["1:ref.info"]}

    def warn_lines(self, instances):
        return {0: ["WARN something %d" % i for i in range(50)]}


class Pipeline(unittest.TestCase):
    """The whole run: stub runner -> evidence -> classification -> packet -> one json line -> ledger."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.old_out = prove.OUT
        prove.OUT = self.d / "out"
        self.args = SimpleNamespace(fake=True, bug="", no_rerun=False, boot=False, provision=False, keep_up=False)

    def tearDown(self):
        prove.OUT = self.old_out
        self.tmp.cleanup()

    def prover(self, env=None, fake=False, **args):
        a = SimpleNamespace(**dict(vars(self.args), **args))
        a.fake = fake
        m = {"params": {}, "tests": []}
        p = prove.Prover(m, a, env or LiveStub(), ledger=self.d / "ledger.jsonl", stamp="s")
        p.fake = fake
        return p

    def run_t(self, p, t, runs=None):
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(io.StringIO()):
            vs = p.run_test(t, runs)
        lines = [l for l in buf.getvalue().splitlines() if l.strip()]
        return vs, lines

    def test_pass_prints_one_json_line_and_appends_ledger(self):
        p = self.prover()
        vs, lines = self.run_t(p, test(), 1)
        self.assertEqual(len(lines), 1)
        v = json.loads(lines[0])
        self.assertEqual(list(v), list(prove.SCHEMA_KEYS))
        self.assertEqual((v["verdict"], v["class"]), ("PASS", None))
        led = L.load(self.d / "ledger.jsonl")
        self.assertEqual(len(led), 1)
        self.assertTrue(led[0]["clean"] and led[0]["fresh"])

    def test_fail_then_pass_is_flaky(self):
        flag = self.d / "flag"
        t = test(commands=[{"name": "stub", "argv": [STUB, "0", "--flaky", str(flag)]}])
        vs, lines = self.run_t(self.prover(), t, 1)
        self.assertEqual(len(lines), 1)
        self.assertEqual(json.loads(lines[0])["verdict"], "FLAKY")
        self.assertFalse(L.is_clean(L.load(self.d / "ledger.jsonl")[0]))

    def test_fail_twice_with_sync_symptom_is_sync_with_packet(self):
        env = LiveStub(problems=["instance 1: 2 digest mismatch(es), 0 repaired"])
        t = test(commands=[{"name": "stub", "argv": [STUB, "1"]}], copies=2)
        vs, lines = self.run_t(self.prover(env), t, 1)
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"], v["repro"]), ("FAIL", "SYNC", "NOT REPRODUCED"))
        self.assertIn("health host 40", v["reason"])
        pk = Path(v["packet"]) if Path(v["packet"]).is_absolute() else ROOT / v["packet"]
        self.assertTrue(pk.exists() and pk.stat().st_size < 3072)
        self.assertIn("digest mismatch", pk.read_text())

    def test_reproduced_failure_is_routed_as_sync(self):
        old = prove.from_failure
        prove.from_failure = lambda outdir: {"verdict": "reproduced", "status": {"symptoms": []}, "primary": {"class": "diverge", "key": "k", "at_ms": 1, "suspects": [1]}}
        try:
            vs, lines = self.run_t(self.prover(), test(commands=[{"name": "stub", "argv": [STUB, "1"]}]), 1)
        finally:
            prove.from_failure = old
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"], v["repro"]), ("FAIL", "SYNC", "REPRODUCED"))

    def test_exit_3_is_build(self):
        vs, lines = self.run_t(self.prover(), test(commands=[{"name": "stub", "argv": [STUB, "3"]}]), 1)
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"]), ("BLOCKED", "BUILD"))

    def test_crash_is_caught_with_address_and_last_call(self):
        t = test(commands=[{"name": "stub", "argv": [STUB, "0", "--sleep", "60"]}])
        t0 = time.time()
        vs, lines = self.run_t(self.prover(LiveStub(crash=True)), t, 1)
        self.assertLess(time.time() - t0, 20, "the runner must be stopped when a copy dies")
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"]), ("CRASH", "CRASH"))
        self.assertIn("nvcoop_mod.dll+9F10", v["reason"])
        self.assertIn("1:ref.info", v["reason"])

    def test_timeout_is_env(self):
        t = test(timeout_s=1, commands=[{"name": "stub", "argv": [STUB, "0", "--sleep", "60"]}])
        vs, lines = self.run_t(self.prover(), t, 1)
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"]), ("TIMEOUT", "ENV"))

    def test_pass_with_digest_repairs_is_not_clean(self):
        t = test(snapshot=["coop.status"])
        vs, lines = self.run_t(self.prover(LiveStub(repairs=2)), t, 1)
        v = json.loads(lines[0])
        self.assertEqual((v["verdict"], v["class"]), ("FAIL", "SYNC"))
        self.assertIn("repairs", v["reason"])
        self.assertEqual(list(v["counters"]), ["coop.status.desyncs.digests.repairs"])

    def test_blocked_test_never_runs(self):
        marker = self.d / "ran"
        t = test(blocked_reason="hand step: close the menu by hand", commands=[{"name": "stub", "argv": [STUB, "0", "--flaky", str(marker)]}])
        vs, lines = self.run_t(self.prover(), t, 1)
        v = json.loads(lines[0])
        self.assertEqual(v["verdict"], "BLOCKED")
        self.assertFalse(marker.exists())

    def test_missing_param_blocks(self):
        t = test(commands=[{"name": "stub", "argv": [STUB, "0", "{weapon}"]}])
        p = self.prover()
        p.params["weapon"] = None
        vs, lines = self.run_t(p, t, 1)
        self.assertIn("weapon", json.loads(lines[0])["reason"])

    def test_env_problem_runs_nothing(self):
        marker = self.d / "ran"
        t = test(commands=[{"name": "stub", "argv": [STUB, "0", "--flaky", str(marker)]}])
        vs, lines = self.run_t(self.prover(LiveStub(env="owner is playing: FalloutNV.exe pid 1")), t, 1)
        self.assertEqual(json.loads(lines[0])["verdict"], "ENV")
        self.assertFalse(marker.exists())

    def test_consecutive_runs_stop_at_first_failure_and_prove(self):
        p = self.prover()
        t = test(n=3)
        vs, lines = self.run_t(p, t)
        self.assertEqual([json.loads(l)["run"] for l in lines], [1, 2, 3])
        self.assertEqual(L.state_of(t, L.load(self.d / "ledger.jsonl"))["state"], "proven")
        t2 = test(id="t2", n=3, commands=[{"name": "stub", "argv": [STUB, "1"]}])
        vs, lines = self.run_t(p, t2)
        self.assertEqual(len(lines), 1, "a failing first run ends the sequence")

    def test_no_user_path_in_the_verdict(self):
        t = test(commands=[{"name": "stub", "argv": [STUB, "1", "--line", "FAIL cannot open " + FAKE_HOME + "/x.esm"]}])
        vs, lines = self.run_t(self.prover(), t, 1)
        self.assertNotIn("someone", lines[0])

    def test_verdict_reason_is_capped(self):
        t = test(commands=[{"name": "stub", "argv": [STUB, "1", "--line", "FAIL " + "z" * 600]}])
        vs, lines = self.run_t(self.prover(), t, 1)
        self.assertLessEqual(len(json.loads(lines[0])["reason"]), 200)


class Queue(unittest.TestCase):
    """prove.drive(): resumable, and it stops on an ENV problem instead of failing every test."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.old = (prove.OUT, prove.make_env)
        prove.OUT = self.d / "out"
        self.m = {"params": {}, "tests": [test(id=f"q{i}", roadmap=2, n=2, commands=[{"name": "stub", "argv": [STUB, "0"]}],
                                                  fake_commands=[{"name": "stub", "argv": [STUB, "0"]}]) for i in range(3)]}

    def tearDown(self):
        prove.OUT, prove.make_env = self.old
        self.tmp.cleanup()

    def drive(self, fake, **kw):
        args = SimpleNamespace(fake=fake, bug="", no_rerun=False, boot=False, provision=False, keep_up=False, runs=0)
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(io.StringIO()):
            rc = prove.drive(self.m, args, self.m["tests"], queue=True)
        return rc, [json.loads(l) for l in buf.getvalue().splitlines() if l.startswith("{")]

    def test_resume_skips_proven_tests(self):
        rc, v = self.drive(True)
        self.assertEqual((rc, len(v)), (0, 6))
        rc, v = self.drive(True)
        self.assertEqual((rc, len(v)), (0, 0), "everything is proven: nothing runs again")

    def drive_runs(self, runs, tests=None, pairs=1):
        args = SimpleNamespace(fake=True, bug="", no_rerun=False, boot=False, provision=False, keep_up=False, runs=runs, pairs=pairs)
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(io.StringIO()):
            rc = prove.drive(self.m, args, tests or self.m["tests"], queue=True)
        return rc, [json.loads(l) for l in buf.getvalue().splitlines() if l.startswith("{")]

    def ledger(self):
        return L.load(prove.OUT / "ledger.fake.jsonl")

    def test_queue_runs_overrides_n_and_the_short_passes_never_prove(self):
        rc, v = self.drive_runs(1)
        self.assertEqual((rc, len(v)), (0, 3), "--runs 1: one run per test although the manifest says n=2")
        self.assertTrue(all(e.get("short") for e in self.ledger()), "recorded as short")
        rows = L.step_table(self.m, self.ledger(), None, include_fake=True)
        self.assertEqual(rows[0]["proven"], 0)
        self.assertIn("short pass", rows[0]["tests"]["q0"]["detail"])
        rc, v = self.drive_runs(0)
        self.assertEqual(len(v), 6, "a normal queue still has to run both N runs of every test")
        self.assertEqual(L.step_table(self.m, self.ledger(), None, include_fake=True)[0]["proven"], 3)

    def test_queue_with_runs_resumes_where_it_stopped(self):
        rc, v = self.drive_runs(1, tests=self.m["tests"][:1])
        self.assertEqual([x["test"] for x in v], ["q0"])
        rc, v = self.drive_runs(1)
        self.assertEqual([x["test"] for x in v], ["q1", "q2"], "q0 passed already: not run again")
        rc, v = self.drive_runs(1, pairs=2)
        self.assertEqual(v, [], "restarted with another --pairs: nothing left to do")

    def test_queue_resumes_across_different_runs(self):
        self.drive_runs(1)
        rc, v = self.drive_runs(3, pairs=2)
        self.assertEqual(len(v), 6, "--runs 3 after a --runs 1 pass: the 3 clean runs on the ledger are the first of 3 per test, 2 more each")
        rc, v = self.drive_runs(3)
        self.assertEqual(v, [], "done")

    def test_queue_runs_skips_proven_tests(self):
        self.drive_runs(0)
        rc, v = self.drive_runs(3)
        self.assertEqual(v, [], "proven on this build: --runs does not run them again")
        rc, v = self.drive_runs(1)
        self.assertEqual(v, [])

    def test_queue_and_run_provision_by_default_and_take_runs(self):
        """actor_port_two_copy was BLOCKED "needs coop.enable=1 (has '0')" on copies the test before it had provisioned: without --provision nothing fixed that."""
        seen = {}
        old = (prove.cmd_queue, prove.cmd_run)
        prove.cmd_queue = lambda a: seen.setdefault("queue", a) and 0
        prove.cmd_run = lambda a: seen.setdefault("run", a) and 0
        try:
            prove.main(["queue", "--runs", "1"])
            prove.main(["run", "--test", "x", "--no-provision"])
        finally:
            prove.cmd_queue, prove.cmd_run = old
        self.assertTrue(seen["queue"].provision)
        self.assertEqual(seen["queue"].runs, 1)
        self.assertFalse(seen["run"].provision)

    def test_env_problem_stops_the_queue(self):
        prove.make_env = lambda args: LiveStub(env="owner is playing: FalloutNV.exe pid 7 (outside the test instance folders)")
        rc, v = self.drive(False)
        self.assertEqual(rc, 4)
        self.assertEqual([x["verdict"] for x in v], ["ENV"], "one ENV verdict, then the queue stops")

    def test_second_queue_is_refused(self):
        lock = S.QueueLock(prove.OUT / "queue.fake.lock", alive=lambda pid: True)
        self.assertIsNone(lock.acquire())
        lock.alive = lambda pid: True
        rc, v = self.drive(True)
        self.assertEqual(rc, 4)
        self.assertIn("another queue runner", v[0]["reason"])
        lock.release()


class Sched(unittest.TestCase):
    """pf_sched: which tests may run at the same time (prove.py --pairs N)."""

    def t(self, copies, **kw):
        d = {"id": f"c{copies}", "roadmap": 2, "n": 1, "copies": copies, "pair": 0}
        d.update(kw)
        return d

    def test_two_pairs_get_two_different_pairs(self):
        t2 = self.t(2)
        self.assertEqual(X.pick(t2, [], 2), [0, 1])
        self.assertEqual(X.pick(t2, [[0, 1]], 2), [2, 3])
        self.assertEqual(X.pick(t2, [[2, 3]], 2), [0, 1])
        self.assertIsNone(X.pick(t2, [[0, 1], [2, 3]], 2), "both pairs busy: wait")
        self.assertIsNone(X.pick(t2, [[0, 1]], 1), "one pair (the default) runs one test at a time")
        self.assertEqual(X.pick(t2, [], 1), [0, 1])

    def test_three_copy_test_runs_alone(self):
        t3, t4 = self.t(3), self.t(4)
        self.assertEqual(X.pick(t3, [], 2), [0, 1, 2])
        self.assertIsNone(X.pick(t3, [[0, 1]], 2), "a pair is running: the three-copy test waits")
        self.assertIsNone(X.pick(t3, [[]], 2), "even a game-free test running beside it is too much")
        self.assertEqual(X.pick(t4, [], 2), [0, 1, 2, 3])
        self.assertIsNone(X.pick(self.t(2), [[0, 1, 2]], 2), "nothing starts beside a three-copy test")
        self.assertIsNone(X.pick(self.t(1), [[0, 1, 2]], 2))
        self.assertIsNone(X.pick(self.t(0), [[0, 1, 2]], 2))

    def test_single_copy_and_game_free_tests(self):
        self.assertEqual(X.pick(self.t(1), [[2, 3]], 2), [0])
        self.assertEqual(X.pick(self.t(2), [[0]], 2), [2, 3], "a one-copy test on instance 0 leaves pair 1 free")
        self.assertEqual(X.pick(self.t(0), [[0, 1]], 2), [])
        self.assertIsNone(X.pick(self.t(0), [[0, 1], [2, 3]], 2), "never more tests than pairs")
        self.assertIsNone(X.pick(self.t(1), [[0, 1]], 2), "instance 0 is taken")

    def test_pinned_pair_is_kept(self):
        t = self.t(2, pair=1)
        self.assertEqual(X.pick(t, [], 2), [2, 3])
        self.assertIsNone(X.pick(t, [[2, 3]], 2))

    def test_alone_test_blocks_the_machine_although_it_has_no_copies(self):
        a = self.t(0, alone=True, launches=2)
        self.assertEqual(X.pick(a, [], 2), [])
        self.assertIsNone(X.pick(a, [[0, 1]], 2))
        self.assertIsNone(X.pick(a, [[]], 2), "not even beside a game-free test")
        for pairs in (1, 2):
            claim = X.claim(a, [], pairs)
            self.assertGreaterEqual(len(claim), 3, "registered as exclusive so nothing starts beside it")
            self.assertIsNone(X.pick(self.t(2), [claim], pairs))
            self.assertIsNone(X.pick(self.t(0), [claim], pairs))
        self.assertEqual(X.claim(self.t(2), [2, 3], 2), [2, 3], "an ordinary test claims only its own copies")

    def test_pair_of(self):
        self.assertEqual([X.pair_of([0, 1]), X.pair_of([2, 3]), X.pair_of([0, 1, 2]), X.pair_of([])], [0, 1, 0, None])

    def test_pairs_that_fit_the_ram(self):
        gib = 1 << 30
        self.assertEqual(X.pairs_that_fit(2, int(4.3 * gib), 0, 1.5, 0.7), 2)
        self.assertEqual(X.pairs_that_fit(2, int(4.0 * gib), 0, 1.5, 0.7), 1)
        self.assertEqual(X.pairs_that_fit(2, int(1.0 * gib), 0, 1.5, 0.7), 1, "never below one pair: the single-test guard decides")
        self.assertEqual(X.pairs_that_fit(2, int(1.6 * gib), 2, 1.5, 0.7), 1, "two copies already run, no room for two more")
        self.assertEqual(X.pairs_that_fit(2, int(3.0 * gib), 2, 1.5, 0.7), 2, "two run, two more need 1.4 GB beyond the reserve")
        self.assertEqual(X.pairs_that_fit(1, int(30 * gib), 0, 1.5, 0.7), 1)

    def test_provision_arguments_keep_the_other_pair_alone(self):
        self.assertEqual(X.provision_args(self.t(2), [2, 3], True), ["--pairs=2", "--first-pair=1"])
        self.assertEqual(X.provision_args(self.t(2), [0, 1], True), ["--pairs=1", "--first-pair=0"])
        self.assertEqual(X.provision_args(self.t(2), [0, 1], False), ["--n", "2"], "one pair at a time: as before")
        self.assertEqual(X.provision_args(self.t(2, pair=1), [2, 3], False), ["--pairs=2", "--first-pair=1"])
        self.assertEqual(X.provision_args(self.t(3), [0, 1, 2], True), ["--n", "3"])
        self.assertEqual(X.provision_args(self.t(4), [0, 1, 2, 3], True), ["--pairs=2"])

    def test_layout_problem(self):
        pair_mode = lambda i: {"pair": i // 2}
        plain = lambda i: {}
        self.assertIsNone(X.layout_problem(self.t(2), [2, 3], True, pair_mode))
        self.assertIn("pair 1", X.layout_problem(self.t(2), [2, 3], True, plain) or "")
        self.assertIsNone(X.layout_problem(self.t(2), [0, 1], False, plain), "a single pair needs no pair mode")
        self.assertIsNone(X.layout_problem(self.t(3), [0, 1, 2], True, plain))
        self.assertIn("plain client", X.layout_problem(self.t(3), [0, 1, 2], True, pair_mode) or "",
                      "instance 2 provisioned as host of pair 1 cannot be the third copy of pair 0")
        self.assertIsNone(X.layout_problem(self.t(4), [0, 1, 2, 3], True, pair_mode))
        self.assertTrue(X.layout_problem(self.t(4), [0, 1, 2, 3], True, plain))


class Ram(unittest.TestCase):
    GIB = 1 << 30

    def test_four_copies_fit_at_exactly_copies_plus_reserve(self):
        free = int((4 * 0.7 + 1.5) * self.GIB)
        self.assertIsNone(S.ram_guard(4, free), "free RAM >= 4 x 0.7 GB + 1.5 GB reserve must allow 4 copies")
        self.assertIsNone(S.ram_guard(4, free + self.GIB))

    def test_reserve_is_kept_free(self):
        self.assertIn("reserve", S.ram_guard(4, int((4 * 0.7 + 1.0) * self.GIB)) or "")
        self.assertTrue(S.ram_guard(4, int(4.2 * self.GIB)), "0.1 GB short of 4 copies + reserve")

    def test_reserve_is_a_flag(self):
        free = int(3.5 * self.GIB)
        self.assertTrue(S.ram_guard(4, free))
        self.assertIsNone(S.ram_guard(4, free, reserve_gb=0.5))
        self.assertTrue(S.ram_guard(4, int(4.0 * self.GIB), reserve_gb=2.0))
        self.assertEqual(S.RESERVE_GB, 1.5)

    def test_only_copies_not_running_yet_count(self):
        self.assertIsNone(S.ram_guard(0, 0))
        self.assertIsNone(S.ram_guard(2, int(2.9 * self.GIB)))
        self.assertTrue(S.ram_guard(2, int(2.8 * self.GIB)))

    def test_preflight_counts_copies_that_run_beside_it(self):
        pids = lambda img: [4] if img == "steam.exe" else []
        kw = dict(pids=pids, roots=[Path(r"C:\games\fnv-instances")], path_of=lambda p: None)
        free = lambda: int(4.3 * self.GIB)
        self.assertIsNone(S.preflight(ROOT, 4, 0, free_ram=free, **kw))
        self.assertIn("RAM guard", S.preflight(ROOT, 4, 0, free_ram=lambda: int(4.0 * self.GIB), **kw) or "")
        self.assertIsNone(S.preflight(ROOT, 4, 2, free_ram=lambda: int(2.9 * self.GIB), **kw), "two of the four already run")
        self.assertIn("RAM guard", S.preflight(ROOT, 4, 0, free_ram=free, reserve_gb=2.5, **kw) or "")


@unittest.skipIf(os.name == "nt", "the /proc answers of the process helpers are for Linux; Windows keeps kernel32")
@unittest.skipIf(os.name != "nt" and not os.path.isdir("/proc/self"), "no /proc here (not mounted, or not Linux): the helpers answer from it")
class PosixProcessHelpers(unittest.TestCase):
    """pf_safety / pf_crash off Windows, on real child processes (only processes this test starts are touched)."""

    def child(self, code="import time; time.sleep(30)"):
        import subprocess
        p = subprocess.Popen([sys.executable, "-c", code])
        self.addCleanup(lambda: (p.poll() is None and p.kill(), p.wait()))
        return p

    def test_pid_alive_running_exited_and_reaped(self):
        p = self.child("import sys; sys.exit(3)")
        deadline = time.time() + 10
        while time.time() < deadline and S._proc_stat(p.pid) and S._proc_stat(p.pid)[0] not in ("Z", "X"):
            time.sleep(0.02)
        self.assertFalse(S.pid_alive(p.pid), "an exited child that was not reaped yet (a zombie) is not alive")
        p.wait()
        self.assertFalse(S.pid_alive(p.pid), "a reaped pid is not alive")
        q = self.child()
        self.assertTrue(S.pid_alive(q.pid))

    def test_list_pids_image_path_and_start_time(self):
        p = self.child()
        name = os.path.basename(sys.executable)
        deadline = time.time() + 10
        while time.time() < deadline and p.pid not in S.list_pids(name):
            time.sleep(0.02)
        self.assertIn(p.pid, S.list_pids(name))
        self.assertNotIn(p.pid, S.list_pids("FalloutNV.exe"))
        self.assertEqual(os.path.realpath(S.image_path(p.pid)), os.path.realpath(sys.executable))
        self.assertLess(abs(S.process_start(p.pid) - time.time()), 30)
        self.assertEqual(S.window_titles(p.pid), [])

    def test_taskkill_stops_the_child_and_the_watcher_reads_its_exit(self):
        p = self.child()
        w = K.Watcher()
        w.watch(0, p.pid)
        self.assertEqual(w.exited(), [])
        S.taskkill(p.pid)
        deadline = time.time() + 10
        got = []
        while time.time() < deadline and not got:
            got = w.exited()
            time.sleep(0.02)
        self.assertEqual(got, [{"instance": 0, "pid": p.pid, "exit_code": -9}], "killed by SIGKILL")
        self.assertEqual(w.exited(), got, "the child is reaped now, and the second check still reads -9")

    def test_kill_tree_stops_the_runner_and_the_child_it_started(self):
        with tempfile.TemporaryDirectory() as d:
            pidf = Path(d) / "grandchild.pid"
            code = ("import subprocess, sys, time; g = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)']); "
                    "open(sys.argv[1], 'w').write(str(g.pid)); time.sleep(30)")
            p = prove.start([sys.executable, "-c", code, str(pidf)], Path(d) / "out.txt", False, None)
            self.addCleanup(lambda: (p.poll() is None and p.kill(), p.wait()))
            deadline = time.time() + 10
            while time.time() < deadline and not (pidf.is_file() and pidf.read_text()):
                time.sleep(0.02)
            g = int(pidf.read_text())
            self.addCleanup(lambda: S.pid_alive(g) and os.kill(g, 9))   # only if the bug left it: a process this test started
            prove.kill_tree(p)
            self.assertIsNotNone(p.poll(), "the runner is stopped")
            deadline = time.time() + 5
            while time.time() < deadline and S.pid_alive(g):
                time.sleep(0.02)
            self.assertFalse(S.pid_alive(g), "the child the runner started is stopped with it")


class WatcherScope(unittest.TestCase):   # injected handles: runs on any OS (the Watcher only loads kernel32 when none are given)
    def test_a_test_only_sees_the_copies_of_its_own_pair(self):
        w = K.Watcher(open_handle=lambda pid: pid, get_exit=lambda h: 0xC0000005 if h == 31 else None)
        w.watch(0, 30)
        w.watch(2, 31)                 # pair 1's host has died
        self.assertEqual(w.exited([0, 1]), [], "pair 0's test must not be failed by a death on pair 1")
        self.assertEqual([d["instance"] for d in w.exited([2, 3])], [2])
        self.assertEqual([d["instance"] for d in w.exited()], [2], "no scope: every copy")
        w.unwatch(2)
        self.assertEqual(w.exited(), [])

    def test_a_dead_copy_keeps_its_exit_code_until_it_is_unwatched(self):
        reads = {31: [7, -1, -1]}   # off Windows waitpid reaps on the first read, and later reads find no child
        w = K.Watcher(open_handle=lambda pid: pid, get_exit=lambda h: reads[h].pop(0) if reads.get(h) else None)
        w.watch(2, 31)
        self.assertEqual([d["exit_code"] for d in w.exited()], [7])
        self.assertEqual([d["exit_code"] for d in w.exited([2])], [7], "the second check must report the same exit")
        w.unwatch(2)
        reads[31] = [None]
        w.watch(2, 31)                 # watched again (a new copy on the same pid): the old code is gone
        self.assertEqual(w.exited(), [])


class OwnedLock(unittest.TestCase):
    def test_two_threads_adding_copies_lose_none(self):
        import threading
        with tempfile.TemporaryDirectory() as d:
            owned = S.Owned(Path(d) / "o.json")
            ts = [threading.Thread(target=lambda k=k: [owned.add(k * 100 + j, 1000 + k * 100 + j) for j in range(30)]) for k in range(4)]
            [t.start() for t in ts]
            [t.join() for t in ts]
            self.assertEqual(len(owned.load()), 120)


class Parallel(unittest.TestCase):
    """prove.drive() with --pairs 2: independent pairs at the same time, big tests alone, one ledger, whole verdict lines."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.old = (prove.OUT, prove.make_env)
        prove.OUT = self.d / "out"
        prove.make_env = lambda args: prove.NullEnv()            # nothing of this machine (Steam, RAM, the owner's game) is consulted
        self.log = self.d / "stub.log"

    def tearDown(self):
        prove.OUT, prove.make_env = self.old
        self.tmp.cleanup()

    def mk(self, spec, code="0"):
        out = []
        for name, copies in spec:
            c = [{"name": "stub", "argv": [STUB, code, "--sleep", "1.2", "--log", str(self.log), name]}]
            out.append(test(id=name, roadmap=2, copies=copies, commands=c, fake_commands=c))
        return out

    def drive(self, tests, pairs, fake=True, queue=False, runs=1):
        m = {"params": {}, "tests": tests}
        args = SimpleNamespace(fake=fake, bug="", no_rerun=True, boot=False, provision=False, keep_up=False, runs=runs, pairs=pairs)
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(io.StringIO()):
            rc = prove.drive(m, args, tests, queue=queue)
        return rc, [json.loads(l) for l in buf.getvalue().splitlines() if l.startswith("{")]

    def spans(self):
        out = {}
        for f in sorted(self.d.glob("stub.log.*")):              # one file per stub: see stub_runner.py
            for ln in f.read_text().splitlines():
                name, what, t = ln.split()
                out.setdefault(name, {})[what] = float(t)
        return out

    @staticmethod
    def overlap(a, b):
        return a["start"] < b["end"] and b["start"] < a["end"]

    def test_stubs_that_start_together_each_keep_their_own_log(self):
        """Two stubs appending to one file lost lines when they started at the same moment (KeyError 'end' in the tests below)."""
        import subprocess
        procs = [subprocess.Popen([sys.executable, "-I", str(ROOT / STUB), "0", "--log", str(self.log), n], cwd=ROOT, stdout=subprocess.DEVNULL,
                                  env=dict(os.environ)) for n in ("a", "b", "c", "d")]
        for p in procs:
            p.wait()
        sp = self.spans()
        self.assertEqual(sorted(sp), ["a", "b", "c", "d"])
        self.assertTrue(all(set(v) == {"start", "end"} for v in sp.values()), sp)
        self.assertFalse(self.log.exists(), "no shared log file")

    def test_two_two_copy_tests_run_at_the_same_time_on_two_pairs(self):
        rc, v = self.drive(self.mk([("a", 2), ("b", 2)]), 2)
        self.assertEqual((rc, len(v)), (0, 2))
        self.assertEqual(sorted(x["pair"] for x in v), [0, 1], "one verdict per pair")
        sp = self.spans()
        self.assertTrue(self.overlap(sp["a"], sp["b"]), "the two tests must overlap in time")

    def test_one_pair_runs_one_after_the_other_as_before(self):
        rc, v = self.drive(self.mk([("a", 2), ("b", 2)]), 1)
        self.assertEqual((rc, [x["pair"] for x in v]), (0, [0, 0]))
        sp = self.spans()
        self.assertFalse(self.overlap(sp["a"], sp["b"]))

    def test_three_copy_test_runs_alone_and_blocks_the_ones_behind_it(self):
        rc, v = self.drive(self.mk([("a", 2), ("big", 3), ("b", 2), ("c", 2)]), 2)
        self.assertEqual((rc, len(v)), (0, 4))
        sp = self.spans()
        for other in ("a", "b", "c"):
            self.assertFalse(self.overlap(sp["big"], sp[other]), f"the three-copy test overlapped {other}")
        self.assertLessEqual(sp["a"]["end"], sp["big"]["start"], "queue order: a, then big alone, then b and c")
        self.assertLessEqual(sp["big"]["end"], min(sp["b"]["start"], sp["c"]["start"]))
        self.assertTrue(self.overlap(sp["b"], sp["c"]), "b and c are two-copy tests: they share the time on two pairs")
        self.assertEqual({x["test"]: x["pair"] for x in v}["big"], 0)

    def test_alone_test_with_no_copies_still_runs_alone(self):
        ts = self.mk([("a", 2), ("solo", 0), ("b", 2), ("c", 2)])
        ts[1]["alone"] = True
        rc, v = self.drive(ts, 2)
        self.assertEqual((rc, len(v)), (0, 4))
        sp = self.spans()
        for other in ("a", "b", "c"):
            self.assertFalse(self.overlap(sp["solo"], sp[other]), f"the alone test overlapped {other}")
        self.assertLessEqual(sp["a"]["end"], sp["solo"]["start"])
        self.assertLessEqual(sp["solo"]["end"], min(sp["b"]["start"], sp["c"]["start"]))

    def test_verdict_has_the_pair_field_and_game_free_tests_have_none(self):
        rc, v = self.drive(self.mk([("free", 0), ("one", 1)]), 2)
        self.assertEqual({x["test"]: x["pair"] for x in v}, {"free": None, "one": 0})
        self.assertIn("pair", prove.SCHEMA_KEYS)

    def test_ledger_has_every_verdict_once_with_its_pair(self):
        rc, v = self.drive(self.mk([("a", 2), ("b", 2), ("c", 2), ("d", 2)]), 2)
        led = L.load(prove.OUT / "ledger.fake.jsonl")
        self.assertEqual(sorted(e["test"] for e in led), ["a", "b", "c", "d"])
        self.assertEqual(sorted(e["pair"] for e in led), [0, 0, 1, 1])

    def test_a_failing_pair_does_not_stop_the_other(self):
        ts = self.mk([("a", 2), ("b", 2)])
        ts[0]["commands"] = ts[0]["fake_commands"] = [{"name": "stub", "argv": [STUB, "1", "--sleep", "1.2", "--log", str(self.log), "a"]}]
        rc, v = self.drive(ts, 2)
        self.assertEqual(rc, 1)
        self.assertEqual({x["test"]: x["verdict"] for x in v}, {"a": "FAIL", "b": "PASS"})

    def test_env_problem_stops_the_queue_with_two_pairs_running(self):
        prove.make_env = lambda args: LiveStub(env="owner is playing: FalloutNV.exe pid 7 (outside the test instance folders)")
        ts = self.mk([("a", 2), ("b", 2), ("c", 2), ("d", 2)])
        rc, v = self.drive(ts, 2, fake=False, queue=True)
        self.assertEqual(rc, 4)
        self.assertLessEqual(len(v), 2, "the two that started together report ENV; nothing else is started")
        self.assertTrue(all(x["verdict"] == "ENV" for x in v))

    def ram_env(self, refuse_times):
        """A pretend PC whose RAM guard refuses test 'big' (it needs more copies than fit) its first `refuse_times` tries."""
        tries = {"n": 0}

        class RamStub(LiveStub):
            def preflight(self, test):
                if test["id"] == "big":
                    tries["n"] += 1
                    if tries["n"] <= refuse_times:
                        return "RAM guard: 3 more game copies need 2.1 GB plus 0.2 GB reserve, but only 2.1 GB is free"
                return None
        prove.make_env = lambda args: RamStub()
        return tries

    def test_ram_guard_for_one_test_does_not_halt_the_queue(self):
        """enemy_parallel_pairs (4 copies) did not fit: the whole queue stopped with exit 4 and nothing behind it ran."""
        self.ram_env(99)
        ts = self.mk([("a", 2), ("big", 2), ("c", 2), ("d", 2)])
        rc, v = self.drive(ts, 1, fake=False, queue=True)
        self.assertNotEqual(rc, 4, "RAM guard is not a reason to stop")
        self.assertEqual(rc, 1, "the test that never fit is still not a pass")
        self.assertEqual([x["test"] for x in v if x["verdict"] == "PASS"], ["a", "c", "d"], "the tests behind it ran")
        env = [x for x in v if x["verdict"] == "ENV"]
        self.assertEqual([x["test"] for x in env], ["big", "big"], "recorded as ENV, then tried once more at the end")
        self.assertEqual([x["test"] for x in v][-1], "big", "the retry comes after the others")
        ledger = [json.loads(l) for l in (prove.OUT / "ledger.jsonl").read_text().splitlines()]
        self.assertEqual(sum(1 for e in ledger if e["test"] == "big" and e["verdict"] == "ENV"), 2)

    def test_ram_guard_refusal_is_retried_at_the_end_and_can_pass(self):
        tries = self.ram_env(1)
        ts = self.mk([("a", 2), ("big", 2), ("c", 2)])
        rc, v = self.drive(ts, 1, fake=False, queue=True)
        self.assertEqual(rc, 0, v)
        self.assertEqual([(x["test"], x["verdict"]) for x in v], [("a", "PASS"), ("big", "ENV"), ("c", "PASS"), ("big", "PASS")])
        self.assertEqual(tries["n"], 2)

    def test_a_crash_of_one_worker_is_not_swallowed(self):
        ts = self.mk([("a", 2)])
        old = prove.Prover.run_test
        prove.Prover.run_test = lambda self, *a, **k: (_ for _ in ()).throw(RuntimeError("boom"))
        try:
            with self.assertRaises(RuntimeError):
                self.drive(ts, 2)
        finally:
            prove.Prover.run_test = old
        self.assertFalse((prove.OUT / "queue.fake.lock").exists(), "the lock is released")


# ------------------------------------------------------------------ harness bugs of the first real sweep (2026-10-22)
def live_env(tmp: Path, **over):
    """A LiveEnv with no game behind it: copies are a dict of "up" flags, boots and stops are recorded, nothing touches a process."""
    import threading
    env = object.__new__(prove.LiveEnv)
    env.args = SimpleNamespace(boot=True, provision=False, pairs=1, reserve_gb=0.2)
    env.owned = S.Owned(tmp / "owned.json")
    env.watcher = K.Watcher()
    env.slot_held, env.log_since, env.active = False, {}, {}
    env._mu, env._prep = threading.Lock(), threading.RLock()
    env.alive, env.boots, env.stops, env.slot_calls = {}, [], [], []
    env.up = lambda i: env.alive.get(i, False)
    env.idir = lambda i: tmp
    env.ini_problem = lambda test, instances: None
    env.methods_missing = lambda instances, methods: []
    env.build_problem = lambda instances: None
    env.seed_saves = lambda instances: []
    env.wait_ready = lambda instances: None
    env.build_problem = lambda instances: None
    env.seed_saves = lambda instances: []
    env.wait_ready = lambda instances: None
    env._swapped = lambda i, t: False

    def boot(i, env_extra):
        env.boots.append((i, dict(env_extra)))
        env.alive[i] = True
        env.owned.add(i, 1000 + i, env=env_extra)
        return None

    def finish(stop, instances=None):
        env.stops.append(list(instances) if instances is not None else None)
        for i in instances or list(env.alive):
            env.alive[i] = False
            env.owned.forget(i)

    def slot(args):
        env.slot_calls.append(args)
        return 0, "OK"
    env._boot, env.finish, env._slot = boot, finish, slot
    for k, v in over.items():
        setattr(env, k, v)
    return env


class BootEnv(unittest.TestCase):
    """Item 1: every copy prove.py boots has NVCOOP_TEST_HELPERS=1 (test.quest.reset said "test helpers disabled"); a running copy is reused only
    when it was started with the env the test needs."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)

    def tearDown(self):
        self.tmp.cleanup()

    def test_boot_env_always_has_the_helpers_and_adds_the_manifests_game_env(self):
        self.assertEqual(S.boot_env(None), {"NVCOOP_TEST_HELPERS": "1"})
        self.assertEqual(S.boot_env({"OTHER": "2"}), {"NVCOOP_TEST_HELPERS": "1", "OTHER": "2"})
        self.assertEqual(S.boot_env({"NVCOOP_TEST_HELPERS": "0"})["NVCOOP_TEST_HELPERS"], "0", "a test may set it itself")

    def test_a_test_without_game_env_still_boots_its_copies_with_the_helpers(self):
        env = live_env(self.d)
        why = env.prepare(test(id="npc_sync", copies=2), [0, 1])
        self.assertEqual(why[:2], (None, None))
        self.assertEqual([(i, e.get("NVCOOP_TEST_HELPERS")) for i, e in env.boots], [(0, "1"), (1, "1")])

    def test_boot_puts_the_env_around_the_launch_and_takes_it_off_again(self):
        import types
        seen = {}

        def launch(i):
            seen["env"] = os.environ.get("NVCOOP_TEST_HELPERS")
            return 4242
        sys.modules["run"] = types.SimpleNamespace(launch_instance=launch, load_state=lambda: {"pids": {}}, save_state=lambda st: None)
        env = live_env(self.d)
        del env._boot            # the real one
        env.up = lambda i: True
        saved = os.environ.pop("NVCOOP_TEST_HELPERS", None)
        try:
            self.assertIsNone(env._boot(0, S.boot_env({})))
            self.assertEqual(seen["env"], "1")
            self.assertNotIn("NVCOOP_TEST_HELPERS", os.environ, "one boot's env must not leak into the next")
            self.assertEqual(env.owned.env_of(0), {"NVCOOP_TEST_HELPERS": "1"})
        finally:
            sys.modules.pop("run", None)
            if saved is not None:
                os.environ["NVCOOP_TEST_HELPERS"] = saved

    def test_a_running_copy_started_without_the_helpers_is_restarted(self):
        env = live_env(self.d)
        env.alive[0] = True
        env.owned.add(0, 1000)                         # booted by an older prove.py: no env recorded
        why = env.prepare(test(id="npc_sync", copies=1), [0])
        self.assertEqual(why[:2], (None, None))
        self.assertEqual(env.stops, [[0]], "the stale copy was stopped (only that copy)")
        self.assertEqual([i for i, _ in env.boots], [0])
        self.assertEqual(env.owned.env_of(0)["NVCOOP_TEST_HELPERS"], "1")

    def test_a_running_copy_with_the_env_is_reused(self):
        env = live_env(self.d)
        env.alive[0] = True
        env.owned.add(0, 1000, env=S.boot_env({}))
        env.prepare(test(id="x", copies=1), [0])
        self.assertEqual((env.stops, env.boots), ([], []))

    def test_a_copy_that_needs_more_than_it_was_booted_with_is_restarted(self):
        env = live_env(self.d)
        env.alive[0] = True
        env.owned.add(0, 1000, env=S.boot_env({}))
        env.prepare(test(id="x", copies=1, game_env={"EXTRA": "1"}), [0])
        self.assertEqual(env.stops, [[0]])
        self.assertEqual(env.boots[0][1]["EXTRA"], "1")

    def test_a_copy_the_user_started_is_left_alone(self):
        env = live_env(self.d)
        env.alive[0] = True                            # up, not in owned.json: prove.py never stops what it did not start
        env.prepare(test(id="x", copies=1), [0])
        self.assertEqual((env.stops, env.boots), ([], []))


class SlotRam(unittest.TestCase):
    """Item 2: the game slot's RAM check follows the caller's need (new copies x 0.7 GB + --reserve-gb), as the caller's own guard does."""

    def test_need_is_copies_times_copy_gb_plus_reserve(self):
        self.assertAlmostEqual(S.slot_need_gb(2, 0.2), 1.6)
        self.assertAlmostEqual(S.slot_need_gb(1, 1.5), 2.2)
        self.assertEqual(S.slot_need_gb(0, 1.5), 0.0, "nothing new to boot: nothing to ask for")

    def test_prove_passes_its_numbers_to_the_slot(self):
        with tempfile.TemporaryDirectory() as d:
            env = live_env(Path(d))
            self.assertIsNone(env._acquire_slot(2, 900, new_copies=2))
            a = env.slot_calls[0]
            self.assertEqual(a[a.index("--need-gb") + 1], "1.60", a)
            env.args.reserve_gb = 1.5
            env._acquire_slot(4, 900, new_copies=4)
            a = env.slot_calls[1]
            self.assertEqual(a[a.index("--need-gb") + 1], "4.30", a)

    def test_prepare_counts_only_the_copies_that_are_not_running_yet(self):
        with tempfile.TemporaryDirectory() as d:
            env = live_env(Path(d))
            env.alive[0] = True
            env.owned.add(0, 1000, env=S.boot_env({}))
            env.prepare(test(id="x", copies=2), [0, 1])
            a = env.slot_calls[0]
            self.assertEqual(a[a.index("--need-gb") + 1], "0.90", "one copy still to boot: 0.7 + 0.2")


class SteamSignedIn(unittest.TestCase):
    """Item 3: a running steam.exe is not enough: a signed-out Steam makes every copy show "Application load error 5:0000065434"."""
    START = 1_000_000.0

    def probe(self, **kw):
        d = {"active_user": 0, "active_pid": 0, "login_ts": None, "start": self.START}
        d.update(kw)
        return d

    def test_loginusers_most_recent_or_newest(self):
        vdf = '"users"\n{\n"76561198000000001"\n{\n"MostRecent" "0"\n"Timestamp" "900"\n}\n"76561198000000002"\n{\n"MostRecent" "1"\n"Timestamp" "500"\n}\n}'
        self.assertEqual(S.parse_loginusers(vdf), 500.0)
        newer = '"users"\n{\n"76561198000000001"\n{\n"timestamp" "900"\n}\n"76561198000000002"\n{\n"timestamp" "1500"\n}\n}'
        self.assertEqual(S.parse_loginusers(newer), 1500.0, "new Steam has no MostRecent key: the newest login")
        self.assertIsNone(S.parse_loginusers("garbage"))

    def test_signals(self):
        ok = S.signed_in_problem
        self.assertIsNone(ok([7], self.probe(active_user=99, active_pid=7)), "registry ActiveUser for the steam.exe that runs")
        self.assertIsNone(ok([7], self.probe(active_user=0, login_ts=self.START + 6)), "ActiveUser is 0 right after a Steam restart, a login came after the start")
        self.assertIsNone(ok([7], self.probe(active_user=99, active_pid=5, login_ts=self.START + 6)), "stale pid in the registry, login after the start")

    def test_signed_out_is_refused_with_a_clear_reason(self):
        for probe in (self.probe(), self.probe(login_ts=self.START - 3600), self.probe(active_user=99, active_pid=5, login_ts=self.START - 600),
                      self.probe(active_user=99, active_pid=5), self.probe(start=None, login_ts=self.START)):
            why = S.signed_in_problem([7], probe)
            self.assertTrue(why and why.startswith("Steam is not signed in"), (probe, why))

    def pre(self, probe):
        return S.preflight(ROOT, 2, 0, pids=lambda img: [7] if img == "steam.exe" else [], roots=[Path(r"C:\games\fnv-instances")],
                           free_ram=lambda: 8 << 30, ram_problem=lambda *a: None, steam_probe=lambda pids: probe)

    def test_preflight_is_env_when_steam_runs_but_is_signed_out(self):
        why = self.pre(self.probe())
        self.assertTrue(why and why.startswith("Steam is not signed in"), why)
        self.assertIsNone(self.pre(self.probe(active_user=5, active_pid=7)))

    def test_a_steam_error_window_fails_fast_as_env(self):
        t = [0.0]

        def sleep(s):
            t[0] += s
        why = S.wait_for_pipe(0, 55, up=lambda i: False, alive=lambda p: True, titles=lambda p: ["Steam Error"], sleep=sleep, now=lambda: t[0])
        self.assertTrue(why and why.startswith("Steam is not signed in"), why)
        self.assertLess(t[0], 10, "no 300 s wait")

    def test_a_copy_that_never_comes_up_still_times_out_and_a_dead_one_is_reported(self):
        t = [0.0]

        def sleep(s):
            t[0] += s
        why = S.wait_for_pipe(0, 55, up=lambda i: False, alive=lambda p: True, titles=lambda p: ["Fallout: New Vegas"], sleep=sleep, now=lambda: t[0])
        self.assertIn("never came up (300 s)", why)
        why = S.wait_for_pipe(0, 55, up=lambda i: False, alive=lambda p: False, titles=lambda p: [], sleep=sleep, now=lambda: t[0])
        self.assertIn("exited", why)
        self.assertIsNone(S.wait_for_pipe(0, 55, up=lambda i: True, alive=lambda p: True, titles=lambda p: ["Steam Error"]))

    def test_the_queue_halts_on_the_new_reasons(self):
        for why in (S.signed_in_problem([7], self.probe()), S.wait_for_pipe(0, 1, up=lambda i: False, alive=lambda p: True, titles=lambda p: ["Steam Error"])):
            self.assertTrue(why.startswith("Steam is not"), "drive() stops the queue on reasons that start with 'Steam is not'")

    def test_steam_error_window_titles(self):
        self.assertTrue(S.steam_error_window(["Fallout", "Steam Error"]))
        self.assertTrue(S.steam_error_window(["steam error "]))
        self.assertFalse(S.steam_error_window(["Fallout: New Vegas", ""]))

    def test_this_pcs_probe_runs(self):
        p = S.real_steam_probe([])
        self.assertEqual(set(p), {"active_user", "active_pid", "login_ts", "start"})


class ManifestSetup(unittest.TestCase):
    """Items 4 to 6: what the manifest provisions and sets up must fit what the test's scripts expect."""

    def setUp(self):
        self.m = prove.load_manifest(PROOF / "manifest.json")
        self.t = {t["id"]: t for t in self.m["tests"]}

    def test_actor_port_two_copy_provisions_what_its_wiring_step_expects(self):
        t = self.t["actor_port_two_copy"]
        self.assertIsNone(prove.wiring_problem(t))
        self.assertEqual({s for s in t["ini"] if s.startswith("live.")}, {"live.actors=1"})
        self.assertTrue({"live.avatars", "live.combat", "live.spawns"} <= set(t["ini_absent"]))
        # every wiring test of the manifest provisions exactly what it expects
        self.assertEqual([i for i, x in self.t.items() if prove.wiring_problem(x)], [])

    def test_wiring_mismatch_is_reported(self):
        t = dict(self.t["actor_port_two_copy"], ini=self.t["live_wiring_all"]["ini"])
        why = prove.wiring_problem(t)
        self.assertTrue(why and "avatars" in why and "expects ['actors']" in why, why)
        self.assertTrue(any("actor_port_two_copy" in b for b in prove.check_manifest({"params": self.m["params"], "tests": [t]})))

    def test_every_game_test_says_whether_coop_is_on(self):
        """Two or more copies need coop.enable=1 and one copy coop.enable=0 in the manifest ini: provisioning keeps whatever the previous test left otherwise."""
        self.assertEqual([i for i, x in self.t.items() if prove.coop_problem(x)], [])
        self.assertEqual(prove.check_manifest(self.m), [])
        for tid, x in self.t.items():
            if x.get("copies", 0) >= 2 and not x.get("blocked_reason"):
                self.assertIn("coop.enable=1", x["ini"], tid)

    def test_a_two_copy_test_without_coop_enable_is_flagged(self):
        t = dict(self.t["actor_port_two_copy"], ini=[s for s in self.t["actor_port_two_copy"]["ini"] if not s.startswith("coop.enable")])
        self.assertIn("coop.enable=1", prove.coop_problem(t) or "")
        self.assertTrue(any("actor_port_two_copy" in b and "coop.enable" in b for b in prove.check_manifest({"params": self.m["params"], "tests": [t]})))
        off = dict(t, ini=t["ini"] + ["coop.enable=0"])
        self.assertTrue(prove.coop_problem(off))
        lone = dict(self.t["npc_caps"], ini=[s for s in self.t["npc_caps"]["ini"] if not s.startswith("coop.enable")])
        self.assertIn("coop.enable=0", prove.coop_problem(lone) or "")
        self.assertIsNone(prove.coop_problem(dict(self.t["lan_two_pc"])), "a blocked test is not provisioned")

    def test_lone_npc_tests_load_the_scenario_first(self):
        for tid in ("npc_caps", "actor_port_save_check"):
            t = self.t[tid]
            self.assertTrue(t["needs_npc"])
            self.assertEqual(t["commands"][0]["argv"][0], "tools/proof/drivers/load_scenario.py")
            self.assertEqual(t["commands"][0]["argv"][1:3], ["--instances", "{i0}"])
            self.assertIsNone(prove.npc_problem(t))

    def test_a_lone_npc_test_without_the_scenario_is_reported(self):
        bare = dict(self.t["npc_caps"], commands=self.t["npc_caps"]["commands"][1:])
        self.assertTrue(prove.npc_problem(bare))
        unmarked = dict(self.t["npc_caps"])
        unmarked.pop("needs_npc")
        self.assertTrue(prove.npc_problem(unmarked))
        self.assertTrue(prove.check_manifest({"params": self.m["params"], "tests": [bare]}))

    def test_tests_that_need_the_player_in_a_cell_load_the_scenario_first(self):
        # sweep 2026-10-22 13:50: five tests ran from the main menu ("avatar runtime not created yet (no player in a cell)")
        ids = ("puppet_loopback", "puppet_two_copy", "avatars_phases", "avatars_lifecycle", "social_proof", "loot_proof", "worldtime_proof")
        for tid in ids:
            t = self.t[tid]
            self.assertTrue(t["needs_world"], tid)
            argv = t["commands"][0]["argv"]
            self.assertEqual(argv[0], "tools/proof/drivers/load_scenario.py", tid)
            self.assertEqual(argv[1:3], ["--instances", "{i0}" if t["copies"] == 1 else "{instances}"], tid)
            self.assertIsNone(prove.world_problem(t), tid)
        by_script = {c["argv"][0].rsplit("/", 1)[-1] for t in self.m["tests"] if not t.get("blocked_reason") for c in t["commands"]}
        for s in prove.WORLD_SCRIPTS:
            self.assertIn(s, by_script, s)

    def test_a_world_test_without_the_scenario_or_the_mark_is_reported(self):
        base = self.t["avatars_phases"]
        bare = dict(base, commands=base["commands"][1:])
        self.assertTrue(prove.world_problem(bare))
        unmarked = dict(base)
        unmarked.pop("needs_world")
        self.assertTrue(prove.world_problem(unmarked))
        self.assertTrue(any("avatars_phases" in b for b in prove.check_manifest({"params": self.m["params"], "tests": [bare]})))

    def test_the_vats_weapons_are_form_ids_not_editor_ids(self):
        # sweep 2026-10-22: params.local.json held "WeapNV9mmPistol": `additem` on an editor id makes nothing
        for k in ("weapon_gun", "weapon_melee", "weapon_explosive", "weapon_energy"):
            self.assertRegex(self.m["params"][k], r"^FalloutNV\.esm:[0-9A-F]{1,8}$", k)
        self.assertEqual(prove.weapon_problem(self.m["params"]), [])
        bad = dict(self.m["params"], weapon_gun="WeapNV9mmPistol")
        self.assertTrue(prove.weapon_problem(bad))
        self.assertTrue(any("weapon_gun" in b for b in prove.check_manifest({"params": bad, "tests": []})))
        self.assertEqual(prove.weapon_problem(dict(bad, weapon_gun=None)), [])
        vm = self.t["vats_world_moves"]["commands"][0]["argv"]
        self.assertEqual(vm[vm.index("--weapon") + 1], "any={weapon_gun}", "the world-moves test equips a known weapon instead of hoping the save holds one")

    def test_the_spawn_test_rolls_with_a_hex_form_id_not_an_editor_id(self):
        # sweep 2026-10-22: --base RadRoach made nothing on any copy (editor ids do not resolve in the bridge console; no creature has that id)
        self.assertRegex(self.m["params"]["spawn_base"], r"^[0-9A-Fa-f]{1,8}$")
        self.assertIsNone(prove.base_problem(self.m["params"]))
        self.assertEqual(self.m["params"]["spawn_base"], "0001CF7C")        # CrRadroach in FalloutNV.esm
        argv = self.t["spawn_proof"]["commands"][0]["argv"]
        self.assertEqual(argv[argv.index("--base") + 1], "{spawn_base}")

    def test_an_editor_id_as_the_spawn_base_is_reported(self):
        params = dict(self.m["params"], spawn_base="RadRoach")
        self.assertIn("not a hex FormID", prove.base_problem(params))
        self.assertTrue(any("spawn_base" in b for b in prove.check_manifest({"params": params, "tests": []})))

    def test_the_scenario_driver_exists_and_places_the_player_at_the_npc(self):
        self.assertTrue((ROOT / "tools/proof/drivers/load_scenario.py").exists())
        sc = json.loads((ROOT / "scenarios/goodsprings_enemies.json").read_text(encoding="utf-8"))
        self.assertEqual(sc["forms"]["gang1"]["ref"], self.m["params"]["npc_ref"])


class CrossDrive(unittest.TestCase):
    """An instance folder on another drive than the checkout (CI: checkout on D:, temp on C:) is passed on absolute."""
    def other_drive(self, p, start=None):
        raise ValueError("path is on mount 'C:', start on mount 'D:'")

    def test_an_instance_dir_on_another_drive_is_passed_absolute(self):
        real = os.path.relpath
        prove.os.path.relpath = self.other_drive
        try:
            got = prove.rel_or_abs(Path("/elsewhere") / "fnv-instances" / "0")
        finally:
            prove.os.path.relpath = real
        self.assertEqual(got, (Path("/elsewhere") / "fnv-instances" / "0").as_posix())

    def test_an_instance_dir_inside_the_checkout_stays_relative(self):
        self.assertEqual(prove.rel_or_abs(ROOT / "out" / "inst" / "0"), "out/inst/0")


class RealFakeWorld(unittest.TestCase):
    """prove.py --fake on the existing fake co-op world: a clean fight passes, the double_damage bug is classified SYNC."""

    def run_prove(self, *args):
        import subprocess
        cp = subprocess.run([sys.executable, "-I", str(PROOF / "prove.py")] + list(args), cwd=ROOT, capture_output=True, text=True, timeout=240)
        lines = [l for l in cp.stdout.splitlines() if l.startswith("{")]
        return cp.returncode, [json.loads(l) for l in lines]

    def test_fake_fight_passes_and_bug_is_sync(self):
        rc, v = self.run_prove("run", "--test", "enemy_fight", "--fake", "--runs", "1")
        self.assertEqual((rc, len(v), v[0]["verdict"]), (0, 1, "PASS"))
        rc, v = self.run_prove("run", "--test", "enemy_fight", "--fake", "--runs", "1", "--bug", "double_damage")
        self.assertEqual((rc, len(v), v[0]["verdict"], v[0]["class"]), (1, 1, "FAIL", "SYNC"))
        self.assertTrue(v[0]["packet"] and (ROOT / v[0]["packet"]).stat().st_size < 3072)
        self.assertIn("--from-step", v[0]["rerun"])

    def test_fake_two_pairs(self):
        rc, v = self.run_prove("run", "--test", "enemy_fight,enemy_handoff", "--fake", "--runs", "1", "--pairs", "2")
        self.assertEqual((rc, len(v)), (0, 2))
        self.assertEqual(sorted(x["pair"] for x in v), [0, 1], "the two tests ran on pair 0 and pair 1")
        self.assertTrue(all(x["verdict"] == "PASS" for x in v))
        rc, v = self.run_prove("run", "--test", "enemy_fight,enemy_handoff", "--fake", "--runs", "1", "--pairs", "2", "--bug", "double_damage", "--no-rerun")
        self.assertEqual(rc, 1)
        self.assertTrue(all((x["verdict"], x["class"]) == ("FAIL", "SYNC") for x in v) and len(v) == 2, v)

    def test_more_than_two_pairs_is_refused(self):
        import subprocess
        cp = subprocess.run([sys.executable, "-I", str(PROOF / "prove.py"), "run", "--test", "enemy_fight", "--fake", "--pairs", "3"],
                            cwd=ROOT, capture_output=True, text=True, timeout=60)
        self.assertEqual(cp.returncode, 2)


class PairBuildAndSaves(unittest.TestCase):
    """2026-10-22 sweep, items 1 and 2: pair 1 ran with another build than pair 0 ("bridge method missing in this build: combat.status") and with empty
    Saves folders ("loadSave: timed out waiting for save to load"); a copy was used before its game served requests."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.split = self.d / "out" / "L1split"
        self.split.mkdir(parents=True)
        (self.split / "nvcoop.dll").write_bytes(b"loader-1")
        (self.split / "nvcoop_mod.dll").write_bytes(b"core-1")
        for i in range(4):
            self.install(i)

    def tearDown(self):
        self.tmp.cleanup()

    def idir(self, i):
        return self.d / "inst" / str(i)

    def install(self, i, loader=b"loader-1", core=b"core-1", ini="[loader]\ncore=nvcoop_mod.dll\n"):
        p = self.idir(i) / "Data" / "NVSE" / "Plugins"
        p.mkdir(parents=True, exist_ok=True)
        (p / "nvcoop.dll").write_bytes(loader)
        if core is not None:
            (p / "nvcoop_mod.dll").write_bytes(core)
        (p / "nvcoop.ini").write_text(ini)

    def problem(self, instances=(0, 1, 2, 3)):
        return X.build_problem(list(instances), self.idir, X.expected_build(self.d))

    def test_four_identical_copies_are_fine(self):
        self.assertIsNone(self.problem())

    def test_a_pair_one_copy_with_another_core_is_a_build_problem(self):
        self.install(2, core=b"core-OLD")
        self.assertIn("instance 2: nvcoop_mod.dll is not the build", self.problem())
        self.assertIsNone(self.problem([0, 1]), "pair 0 alone is fine")

    def test_a_missing_core_a_stale_core_and_a_loader_that_would_not_load_it(self):
        self.install(3, core=None)
        (self.idir(3) / "Data" / "NVSE" / "Plugins" / "nvcoop_mod.dll").unlink()
        self.assertIn("instance 3: nvcoop_mod.dll is not installed", self.problem())
        self.install(3)
        self.install(1, ini="[coop]\nenable=1\n")
        self.assertIn("[loader] core", self.problem())
        (self.split / "nvcoop_mod.dll").unlink()                    # monolith: out/L1/nvcoop.dll only
        (self.d / "out" / "L1").mkdir()
        (self.d / "out" / "L1" / "nvcoop.dll").write_bytes(b"loader-1")
        self.assertIn("monolith", self.problem([0]))

    def prep_env(self, **over):
        (self.d / "env").mkdir(exist_ok=True)
        env = live_env(self.d / "env", **over)
        env.args.provision = True
        env.provision_cmd = lambda t, i: ["tools/runner/run.py", "provision"]
        return env

    def test_prepare_refuses_a_build_mismatch_as_build_when_provisioning_does_not_fix_it(self):
        env = self.prep_env()
        env.build_problem = lambda instances: "instance 2: nvcoop_mod.dll is not the build in L1split/"
        import unittest.mock as M
        with M.patch.object(prove.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="", stderr="")):
            env_reason, build_reason, _ = env.prepare(test(id="x", copies=2, pair=1), [2, 3])
        self.assertIsNone(env_reason)
        self.assertIn("build mismatch", build_reason)
        self.assertEqual(env.boots, [], "no copy is booted on a mismatching build")

    def test_prepare_reprovisions_a_mismatching_pair_and_then_runs(self):
        env = self.prep_env()
        state = {"bad": "instance 2: nvcoop_mod.dll is not the build"}
        env.build_problem = lambda instances: state["bad"]
        import unittest.mock as M

        def fake_run(argv, **kw):
            state["bad"] = None
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        with M.patch.object(prove.subprocess, "run", side_effect=fake_run) as run:
            out = env.prepare(test(id="x", copies=2, pair=1), [2, 3])
        self.assertEqual(out[:2], (None, None))
        self.assertEqual(run.call_count, 1)
        self.assertEqual([i for i, _ in env.boots], [2, 3])

    def test_a_copy_that_does_not_serve_requests_yet_is_env_and_is_not_asked_for_methods(self):
        env = self.prep_env()
        env.wait_ready = lambda instances: "cannot reach the game instances: copy 0 never became ready in 240 s"
        asked = []
        env.methods_missing = lambda instances, methods: asked.append(1) or []
        out = env.prepare(test(id="x", copies=2, requires_methods=["combat.status"]), [0, 1])
        self.assertIn("never became ready", out[0])
        self.assertEqual(asked, [])

    def test_prepare_seeds_the_saves_of_every_copy(self):
        env = self.prep_env()
        seen = []
        env.seed_saves = lambda instances: seen.append(list(instances)) or []
        env.prepare(test(id="x", copies=2, pair=1), [2, 3])
        self.assertEqual(seen, [[2, 3]])

    # ---- saves
    def fill(self, i, names):
        s = self.idir(i) / "Saves"
        s.mkdir(parents=True, exist_ok=True)
        for n in names:
            (s / n).write_bytes(("data-of-%d-%s" % (i, n)).encode())
            os.utime(s / n, (1_600_000_000, 1_600_000_000))

    def saves_of(self, i):
        return {p.name for p in (self.idir(i) / "Saves").iterdir()}

    def test_an_empty_copy_gets_the_save_family_of_a_donor_of_its_own_role(self):
        fam = ["autosave.fos", "nvcoop_autosave_ready.fos", "nvcoop_autosave_ready.fnvcoop", "nvcoop_prepared.json"]
        self.fill(0, fam + ["host-only.fnvspawn"])
        self.fill(1, fam)
        done = V.seed_saves([2, 3], lambda i: self.idir(i) / "Saves", now=lambda: 2_000_000_000)
        self.assertEqual(self.saves_of(2), set(fam + ["host-only.fnvspawn"]), "the host copy of pair 1 is seeded from instance 0")
        self.assertEqual(self.saves_of(3), set(fam))
        self.assertEqual((self.idir(3) / "Saves" / "autosave.fos").read_bytes(), b"data-of-1-autosave.fos", "the client copy is seeded from a client")
        self.assertEqual(os.stat(self.idir(3) / "Saves" / "autosave.fos").st_mtime, 1_600_000_000, "mtime kept: the prepared-save manifest compares it")
        self.assertTrue(done)

    def test_seeding_never_overwrites_and_skips_what_the_game_is_writing(self):
        self.fill(0, ["autosave.fos", "fresh.fos"])
        self.fill(2, ["autosave.fos"])
        os.utime(self.idir(0) / "Saves" / "fresh.fos", (2_000_000_000, 2_000_000_000))
        V.seed_saves([2], lambda i: self.idir(i) / "Saves", now=lambda: 2_000_000_001)
        self.assertEqual((self.idir(2) / "Saves" / "autosave.fos").read_bytes(), b"data-of-2-autosave.fos")
        self.assertNotIn("fresh.fos", self.saves_of(2))

    def test_the_owners_autosave_is_the_last_resort(self):
        own = self.d / "owner"
        own.mkdir()
        (own / "autosave.fos").write_bytes(b"owner")
        done = V.seed_saves([2], lambda i: self.idir(i) / "Saves", owner_saves=lambda: own, now=lambda: 2_000_000_000)
        self.assertEqual((self.idir(2) / "Saves" / "autosave.fos").read_bytes(), b"owner")
        self.assertEqual(done, ["2: autosave.fos <- owner"])
        self.assertEqual(V.missing_start_save([2, 3], lambda i: self.idir(i) / "Saves"), [3])



class OrphanCopies(unittest.TestCase):
    """Item 7 (queue restart, 2026-10-22): owned.json was gone but the copies prove.py launched still ran. prove.py reported BLOCKED BUILD
    ("instance 2 is not provisioned as a member of pair 1", "instance 0: nvcoop.ini needs coop.enable=0 (has '1')") because it only stops and
    re-provisions copies it can prove are its own. The slot lease of owner `proof` records every pid run.py launched for it."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.inst = self.d / "fnv-instances"
        self.paths = {101: self.inst / "0" / "FalloutNV.exe", 102: self.inst / "2" / "FalloutNV.exe",
                      103: self.d / "Steam" / "FalloutNV.exe", 104: self.inst / "3" / "FalloutNV.exe"}
        self.status = {"slots": [{"slot": 0, "free": False, "lease": {"owner": "proof", "pids": [101, 102, 103]}},
                                 {"slot": 1, "free": False, "lease": {"owner": "L5", "pids": [104]}},
                                 {"slot": 2, "free": True, "lease": {"owner": "proof", "pids": [999]}}]}

    def tearDown(self):
        self.tmp.cleanup()

    def adopt(self, owned=None):
        owned = owned or S.Owned(self.d / "owned.json")
        got = S.adopt_running(owned, S.lease_pids(self.status, "proof"), lambda i: self.inst / str(i), range(6),
                              pids=lambda image: sorted(self.paths), path_of=lambda pid: str(self.paths.get(pid)), started=lambda pid: 5000.0 + pid)
        return owned, got

    def test_lease_pids_are_the_live_leases_of_that_owner_only(self):
        self.assertEqual(S.lease_pids(self.status, "proof"), [101, 102, 103])
        self.assertEqual(S.lease_pids(self.status, "L5"), [104])
        self.assertEqual(S.lease_pids({}, "proof"), [])

    def test_the_copies_the_lane_launched_from_the_instance_folders_are_adopted(self):
        owned, got = self.adopt()
        self.assertEqual(sorted(got), [0, 2])
        self.assertEqual({k: v["pid"] for k, v in owned.load().items()}, {"0": 101, "2": 102})
        self.assertEqual(owned.load()["0"]["booted_at"], 5101.0)
        self.assertEqual(owned.env_of(0), {}, "the start environment is unknown: the copy is not trusted to have the test helpers")

    def test_a_copy_of_another_lane_and_the_owners_own_game_are_left_alone(self):
        owned, got = self.adopt()
        self.assertNotIn(3, got, "pid 104 runs from instance 3 but sits on lane L5's lease")
        self.assertNotIn(103, [v["pid"] for v in owned.load().values()], "pid 103 is on our lease but runs outside the instance folders")
        with self.assertRaises(S.NotOwned):
            S.kill_owned(104, owned, pids=lambda image: [104], kill=lambda pid: None)

    def test_an_already_owned_copy_is_not_replaced(self):
        owned = S.Owned(self.d / "owned.json")
        owned.add(0, 101, when=1.0, env={"NVCOOP_TEST_HELPERS": "1"})
        owned, got = self.adopt(owned)
        self.assertEqual(got, [2])
        self.assertEqual(owned.env_of(0), {"NVCOOP_TEST_HELPERS": "1"}, "the entry that knew its env stays")

    def env_with_orphans(self):
        import unittest.mock as mock
        env = live_env(self.d, args=SimpleNamespace(boot=True, provision=True, pairs=1, reserve_gb=0.2))
        env._slot = lambda args: (0, json.dumps(self.status)) if args[:1] == ["status"] else (0, "OK")
        env.idir = lambda i: self.inst / str(i)
        env.alive = {0: True, 1: False}
        problems = ["instance 0: nvcoop.ini needs coop.enable=0 (has '1')"]
        env.ini_problem = lambda test, instances: problems.pop(0) if problems else None
        env.provision_cmd = lambda test, instances: ["tools/runner/run.py", "provision", "--pairs", "1"]
        ran = []
        pats = [mock.patch.object(S, "list_pids", lambda image: [101]), mock.patch.object(S, "image_path", lambda pid: str(self.paths[pid])),
                mock.patch.object(S, "process_start", lambda pid: 77.0),
                mock.patch.object(prove.subprocess, "run", lambda *a, **k: ran.append(a[0]) or SimpleNamespace(returncode=0, stdout="", stderr="")),
                mock.patch.object(prove.time, "sleep", lambda s: None)]
        for p in pats:
            p.start()
            self.addCleanup(p.stop)
        return env, ran

    def test_a_running_copy_with_the_wrong_ini_is_stopped_reprovisioned_and_rebooted_although_owned_json_was_lost(self):
        env, ran = self.env_with_orphans()
        self.assertEqual(env.owned.load(), {}, "owned.json is gone")
        why = env.prepare(test(id="puppet_loopback", copies=1), [0])
        self.assertIsNone(why[1], "not BLOCKED: " + str(why))
        self.assertTrue(env.stops and all(s == [0] for s in env.stops), "only that copy was stopped: " + str(env.stops))
        self.assertTrue(any("provision" in " ".join(map(str, a)) for a in ran), "provision ran")
        self.assertEqual([i for i, _ in env.boots], [0], "and the copy was booted again")

    def test_twin_without_adoption_the_same_copy_is_blocked(self):
        old = prove.MUTANT
        prove.MUTANT = "orphans_not_adopted"
        try:
            env, ran = self.env_with_orphans()
            why = env.prepare(test(id="puppet_loopback", copies=1), [0])
        finally:
            prove.MUTANT = old
        self.assertIn("coop.enable=0", why[1] or "")      # the BLOCKED the sweep saw
        self.assertEqual((env.stops, env.boots), ([], []))


class RetestMode(unittest.TestCase):
    """Item 8: queue --tests a,b / --from-bugs FILE run only those tests; --round NAME makes a restarted queue skip what has a verdict already."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = Path(self.tmp.name)
        self.old = (prove.OUT, prove.make_env)
        prove.OUT = self.d / "out"
        # every test FAILS: a failed test is not skipped by the "proven" / "clean runs" resume rules, so only the round can make the restart skip it
        self.m = {"params": {}, "tests": [test(id=f"q{i}", roadmap=2, n=2, commands=[{"name": "stub", "argv": [STUB, "1"]}],
                                                  fake_commands=[{"name": "stub", "argv": [STUB, "1"]}]) for i in range(4)]}

    def tearDown(self):
        prove.OUT, prove.make_env = self.old
        self.tmp.cleanup()

    def args(self, **kw):
        base = dict(fake=True, bug="", no_rerun=False, boot=False, provision=False, keep_up=False, runs=1, pairs=1)
        base.update(kw)
        return SimpleNamespace(**base)

    def drive(self, tests=None, **kw):
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(io.StringIO()):
            rc = prove.drive(self.m, self.args(**kw), tests if tests is not None else self.m["tests"], queue=True)
        return rc, [json.loads(l) for l in buf.getvalue().splitlines() if l.startswith("{")]

    def bugs(self, rows):
        p = self.d / "bugs3.json"
        p.write_text(json.dumps({"status": "open", "pass_tests": ["q0"], "rows": rows}))
        return p

    def test_tests_option_selects_only_those_tests_in_manifest_order(self):
        for form in (["q2,q0"], ["q2", "q0"]):
            got = prove.select(self.m, SimpleNamespace(test=list(form), step=None))
            self.assertEqual([t["id"] for t in got], ["q0", "q2"])
        with self.assertRaises(SystemExit):
            prove.select(self.m, SimpleNamespace(test=["nope"], step=None))

    def test_the_cli_takes_tests_and_from_bugs_and_round(self):
        seen = {}
        old = prove.cmd_queue
        prove.cmd_queue = lambda a: seen.update(vars(a)) or 0
        try:
            prove.main(["queue", "--tests", "q1,q3", "--round", "r7", "--from-bugs", "x.json", "--fake"])
        finally:
            prove.cmd_queue = old
        self.assertEqual((seen["test"], seen["round"], seen["from_bugs"]), (["q1,q3"], "r7", "x.json"))

    def test_from_bugs_takes_the_open_rows_once_each(self):
        p = self.bugs([{"test": "q3", "status": "open"}, {"test": "q1", "status": "fixed"}, {"test": "q2"}, {"test": "q3", "status": "open"}])
        self.assertEqual(prove.bug_tests(p), ["q3", "q2"])
        args = SimpleNamespace(test=["q0"], step=None, from_bugs=p)
        self.assertEqual([t["id"] for t in prove.select(self.m, args)], ["q0", "q2", "q3"], "--tests and --from-bugs add up")

    def test_a_restarted_round_skips_the_tests_that_have_a_verdict_in_it(self):
        rc, v = self.drive(self.m["tests"][:2], round="r1")
        self.assertEqual([x["test"] for x in v], ["q0", "q1"])
        self.assertTrue(all(e.get("round") == "r1" for e in L.load(prove.OUT / "ledger.fake.jsonl")))
        rc, v = self.drive(round="r1")
        self.assertEqual([x["test"] for x in v], ["q2", "q3"], "q0 and q1 have their verdict in round r1: the restart goes on from q2")
        rc, v = self.drive(round="r2")
        self.assertEqual(len(v), 4, "a new round starts from the top")
        rc, v = self.drive()
        self.assertEqual(len(v), 4, "no --round: nothing is skipped (a failed test is tried again)")

    def test_twin_a_queue_that_ignores_the_round_starts_from_the_top(self):
        self.drive(self.m["tests"][:2], round="r1")
        old = L.MUTANT
        L.MUTANT = "round_ignored"
        try:
            rc, v = self.drive(round="r1")
        finally:
            L.MUTANT = old
        self.assertEqual(len(v), 4)

    def test_round_done_rules(self):
        def e(**k):
            return dict(dict(test="t", round="r", fake=False, run=1, of=2, verdict="PASS"), **k)
        self.assertFalse(L.round_done([e()], "t", "r"), "run 1 of 2 passed: the series is not finished")
        self.assertTrue(L.round_done([e(run=2)], "t", "r"))
        self.assertTrue(L.round_done([e(verdict="FAIL")], "t", "r"))
        self.assertTrue(L.round_done([e(verdict="FLAKY")], "t", "r"))
        self.assertFalse(L.round_done([e(verdict="ENV")], "t", "r"), "ENV says nothing about the build")
        self.assertFalse(L.round_done([e(verdict="BLOCKED", reason="instance 0 is not provisioned")], "t", "r"))
        self.assertTrue(L.round_done([e(verdict="BLOCKED", reason="blocked: a hand step")], "t", "r"))
        self.assertFalse(L.round_done([e(verdict="FAIL", round="other")], "t", "r"))
        self.assertFalse(L.round_done([e(verdict="FAIL", fake=True)], "t", "r"))
        self.assertFalse(L.round_done([e(verdict="FAIL")], "t", None))


if __name__ == "__main__":
    unittest.main(verbosity=1)
