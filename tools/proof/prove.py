#!/usr/bin/env python3
"""prove.py: a script runs the proof tests, models only read verdicts (docs/PROOF-PLAN.md, docs/tools/proof.md).

    python -I tools/proof/prove.py run --test enemy_fight           # N consecutive runs of one test (N from the manifest)
    python -I tools/proof/prove.py run --step 2                     # every test of roadmap step 2
    python -I tools/proof/prove.py queue                            # the whole matrix in order; resumable (proven tests are skipped)
    python -I tools/proof/prove.py status [--step N] [--json]       # proven / failing / blocked per step and the percentage
    python -I tools/proof/prove.py check                            # validate manifest.json (scripts exist, ids unique)
    add --pairs 2 to run / queue to run two independent two-copy tests at the same time, one per pair (pair 0 = instances 0,1; pair 1 =
        instances 2,3). Three-copy and four-copy tests still run alone; --reserve-gb sets the RAM left free (default 1.5)
    add --fake to run / queue / status for the game-free self-test on the fake worlds (ledger out/proof/ledger.fake.jsonl)

stdout carries exactly ONE json verdict line per test run (and the status table for `status`); progress goes to stderr. Every verdict is
appended to out/proof/ledger.jsonl ("pair" says which pair ran it: 0 or 1, null for a game-free test). A failing run leaves a packet.md (under 3 KB) next to its logs. Exit: 0 all PASS, 1 any other verdict,
4 the queue stopped on an ENV problem (owner is playing, Steam off, slot busy, RAM), 2 bad arguments.
"""
from __future__ import annotations

import argparse
import glob
import hashlib
import json
import os
import queue as _queue
import re
import subprocess
import sys
import threading
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (str(HERE),):
    if _p not in sys.path:
        sys.path.insert(0, _p)

import pf_classify as C   # noqa: E402
import pf_crash as K      # noqa: E402
import pf_ledger as L     # noqa: E402
import pf_packet as P     # noqa: E402
import pf_safety as S     # noqa: E402
import pf_saves as V      # noqa: E402
import pf_sched as X      # noqa: E402

PY = [sys.executable, "-I", "-X", "utf8"]
OWNER = "proof"
OUT = ROOT / "out" / "proof"
SCHEMA_KEYS = ("test", "roadmap", "run", "of", "pair", "verdict", "class", "reason", "check", "repro", "counters", "rerun", "packet", "secs", "build")
METHOD_NOT_FOUND = -32601
READY_WAIT_S = 240.0     # how long a booted (or still loading) copy may take to serve requests before the test is ENV
EMIT_LOCK = threading.Lock()
MUTANT = os.environ.get("PROOF_MUTANT", "")   # test-only: one deliberate bug per name (tools/proof/tests/run_tests.py)
INI_ENCODING = "mbcs" if os.name == "nt" else None   # nvcoop.ini is in the ANSI code page; mbcs exists on Windows only (None: locale)


def err(*a) -> None:
    print(*a, file=sys.stderr, flush=True)


# ------------------------------------------------------------------ manifest
def load_manifest(path: Optional[Path] = None) -> Dict[str, Any]:
    m = json.loads((path or HERE / "manifest.json").read_text(encoding="utf-8"))
    local = HERE / "params.local.json"
    if path is None and local.exists():
        m["params"].update(json.loads(local.read_text(encoding="utf-8")))
    return m


def rel_or_abs(p) -> str:
    """`p` relative to the checkout (posix), or absolute when it is on another drive: relpath has no answer from D: to C:,
    which is how a CI runner lays out its checkout (D:) and its temp folder (C:)."""
    if MUTANT == "relpath_cross_drive":
        return Path(os.path.relpath(p, ROOT)).as_posix()
    try:
        return Path(os.path.relpath(p, ROOT)).as_posix()
    except ValueError:
        return Path(p).as_posix()


def instances_of(test: Dict[str, Any]) -> List[int]:
    return X.instances_for(test)


def _fill(tok: str, vars_: Dict[str, Any]) -> Tuple[Optional[str], Optional[str]]:
    """(text, missing param name). A value of None counts as missing."""
    missing = None

    def sub(m):
        nonlocal missing
        v = vars_.get(m.group(1))
        if v is None:
            missing = missing or m.group(1)
            return ""
        return str(v)
    out = re.sub(r"\{(\w+)\}", sub, tok)
    return (None, missing) if missing else (out, None)


def expand(argv: List[Any], vars_: Dict[str, Any]) -> Tuple[Optional[List[str]], Optional[str]]:
    """Fill the placeholders. -> (argv, None) or (None, name of the first required param with no value). A list inside argv is optional."""
    out: List[str] = []
    for tok in argv:
        if isinstance(tok, list):
            grp, miss = [], None
            for t in tok:
                v, m = _fill(t, vars_)
                miss = miss or m
                grp.append(v)
            if not miss:
                out += grp
            continue
        if tok == "{instances_sp}":
            out += str(vars_["instances_sp"]).split()
            continue
        v, m = _fill(tok, vars_)
        if m:
            return None, m
        out.append(v)
    return out, None


def shell_join(argv: List[str]) -> str:
    return " ".join(f'"{a}"' if (not a or " " in a) else a for a in argv)


def build_id(fake: bool = False) -> str:
    if fake:
        return "fake"
    h = hashlib.sha1()
    found = False
    for rel in ("out/L1/nvcoop.dll", "out/L1split/nvcoop.dll", "out/L1split/nvcoop_mod.dll"):
        p = ROOT / rel
        if p.exists():
            found = True
            h.update(p.read_bytes())
    return h.hexdigest()[:10] if found else "nobuild"


# ------------------------------------------------------------------ live access (replaced by NullEnv in --fake and in tests)
class NullEnv:
    """No game: nothing to check, nothing to boot, no counters. Used by --fake and by the unit tests as a base class."""
    live = False

    def __init__(self):
        self.watcher: Optional[K.Watcher] = None

    def preflight(self, test) -> Optional[str]:
        return None

    def hold(self, instances) -> None:
        """A test is about to use these copies (pairs run at the same time: the RAM guard counts every copy in use)."""

    def unhold(self, instances) -> None:
        pass

    def pairs_allowed(self, requested: int) -> Tuple[int, str]:
        """-> (how many pairs may run at the same time, a note when that is fewer than requested)."""
        return requested, ""

    def prepare(self, test, instances) -> Tuple[Optional[str], Optional[str], bool]:
        """-> (env_reason, build_reason, fresh)."""
        return None, None, True

    def snapshot(self, instances, methods) -> Dict[str, Dict[int, Any]]:
        return {}

    def inspect(self, instances) -> Tuple[Optional[list], List[str]]:
        return None, []

    def crashes(self, instances: Optional[List[int]] = None) -> List[Dict[str, Any]]:
        return []

    def crash_info(self, instance: int, since: float, trace: Optional[Path]) -> Dict[str, Any]:
        return {"instance": instance}

    def warn_lines(self, instances) -> Dict[int, List[str]]:
        return {}

    def finish(self, stop: bool, instances: Optional[List[int]] = None) -> None:
        pass


class LiveEnv(NullEnv):
    """The owner's PC: guard, slot, boot, bridge snapshots, crash watch. Never starts or stops anything it did not start itself."""
    live = True

    def __init__(self, args):
        super().__init__()
        self.args = args
        self.owned = S.Owned(OUT / "owned.json")
        self.watcher = K.Watcher()
        self.slot_held = False
        self.log_since: Dict[int, float] = {}
        self.active: Dict[int, int] = {}                # instance -> tests using it now (pairs run at the same time)
        self._mu = threading.Lock()                     # guards `active`
        self._prep = threading.RLock()                  # one pair boots at a time: launch_instance finds its game process by elimination
        sys.path.insert(0, str(ROOT / "tools" / "runner"))
        import instances as inst   # noqa: WPS433
        import pairs               # noqa: WPS433
        self.inst, self.pairs = inst, pairs
        # adopt copies this tool booted earlier and that still run
        for i, v in self.owned.load().items():
            if S.pid_alive(int(v["pid"])) and int(v["pid"]) in S.list_pids(S.GAME_EXE):
                self.watcher.watch(int(i), int(v["pid"]))
            else:
                self.owned.forget(int(i))
        self.adopt_orphans()

    # -- helpers
    def up(self, i: int) -> bool:
        return self.pairs.pipe_up(i)

    def adopt_orphans(self) -> List[int]:
        """owned.json lost after a killed queue: take back the copies our lane (slot owner `proof`) launched from the instance folders and that still run."""
        try:
            code, out = self._slot(["status"])
            st = json.loads(out) if code == 0 else {}
        except (ValueError, OSError):
            return []
        if MUTANT == "orphans_not_adopted":
            return []
        got = S.adopt_running(self.owned, S.lease_pids(st, OWNER), self.idir, range(2 * X.MAX_PAIRS + 2), pids=S.list_pids, path_of=S.image_path,
                              started=S.process_start)
        for i in got:
            pid = int(self.owned.load()[str(i)]["pid"])
            self.watcher.watch(i, pid)
            err(f"adopted copy {i} (pid {pid}): launched by this tool, owned.json had lost it")
        return got

    def idir(self, i: int) -> Path:
        return self.inst.instance_dir(i)

    def _slot(self, args: List[str]) -> Tuple[int, str]:
        cp = subprocess.run(PY + [str(ROOT / "tools" / "gameslot.py")] + args, capture_output=True, text=True)
        return cp.returncode, (cp.stdout + cp.stderr).strip()

    def hold(self, instances) -> None:
        with self._mu:
            for i in instances:
                self.active[i] = self.active.get(i, 0) + 1

    def unhold(self, instances) -> None:
        with self._mu:
            for i in instances:
                if self.active.get(i, 0) <= 1:
                    self.active.pop(i, None)
                else:
                    self.active[i] -= 1

    def in_use(self, extra=()) -> List[int]:
        with self._mu:
            return sorted(set(self.active) | set(extra))

    def pairs_allowed(self, requested: int) -> Tuple[int, str]:
        if requested <= 1:
            return 1, ""
        up = sum(1 for i in range(2 * requested) if self.up(i))
        n = X.pairs_that_fit(requested, self.pairs.free_ram_bytes(), up, getattr(self.args, 'reserve_gb', S.RESERVE_GB), S.COPY_GB)
        if n < requested:
            return n, f"only {n} pair(s) fit the free RAM (copy {S.COPY_GB} GB, reserve {getattr(self.args, 'reserve_gb', S.RESERVE_GB)} GB): running {n} at a time"
        return n, ""

    def preflight(self, test) -> Optional[str]:
        instances = instances_of(test)
        launches = int(test.get("launches", 0))       # copies the test's own driver starts (split-screen smoke): same owner-PC guard and RAM rule
        if not instances and not launches:
            return None
        target = self.in_use(instances)               # the copies of every test running beside this one need RAM too
        up = sum(1 for i in target if self.up(i))
        return S.preflight(ROOT, len(target) + launches, up, free_ram=self.pairs.free_ram_bytes,
                           reserve_gb=getattr(self.args, 'reserve_gb', S.RESERVE_GB))

    # -- switches, methods, boot
    def instance_meta(self, i: int) -> Dict[str, Any]:
        try:
            return json.loads((self.idir(i) / "instance.json").read_text(encoding="utf-8"))
        except (OSError, ValueError):
            return {}

    def ini_problem(self, test, instances) -> Optional[str]:
        import configparser
        parallel = int(getattr(self.args, "pairs", 1) or 1) > 1
        bad = X.layout_problem(test, instances, parallel, self.instance_meta)
        if bad:
            return bad
        for i in instances:
            p = self.idir(i) / "Data" / "NVSE" / "Plugins" / "nvcoop.ini"
            cp = configparser.ConfigParser(strict=False, interpolation=None)
            try:
                cp.read(p, encoding=INI_ENCODING)
            except (OSError, configparser.Error) as e:
                return f"instance {i}: cannot read nvcoop.ini ({e})"
            if not p.exists():
                return f"instance {i} is not provisioned"

            def get(spec: str) -> Optional[str]:
                sec, key = spec.split("=")[0].split(".", 1)
                for s in cp.sections():
                    if s.lower() == sec.lower() and cp.has_option(s, key):
                        return cp.get(s, key).split(";")[0].strip()
                return None
            for spec in test.get("ini", []):
                want = spec.split("=", 1)[1]
                if (get(spec) or "") != want and not (want == "0" and get(spec) is None):
                    return f"instance {i}: nvcoop.ini needs {spec} (has {get(spec)!r})"
            for key in test.get("ini_absent", []):
                if (get(key + "=") or "0") not in ("0", ""):
                    return f"instance {i}: nvcoop.ini must not switch {key} on (has {get(key + '=')!r})"
        return None

    def provision_cmd(self, test, instances) -> List[str]:
        cmd = ["tools/runner/run.py", "provision"]          # repo relative: shown to readers, run through PY by prepare()
        cmd += X.provision_args(test, instances, int(getattr(self.args, "pairs", 1) or 1) > 1)
        split = ROOT / "out" / "L1split"
        if (split / "nvcoop.dll").exists() and (split / "nvcoop_mod.dll").exists():
            cmd += ["--plugin-dll", "out/L1split/nvcoop.dll", "--core-dll", "out/L1split/nvcoop_mod.dll"]
        for spec in test.get("ini", []):
            cmd += [f"--ini={spec}"]
        return cmd

    def build_problem(self, instances) -> Optional[str]:
        return X.build_problem(instances, self.idir, X.expected_build(ROOT))

    def seed_saves(self, instances) -> List[str]:
        """Every copy gets the saves the scenarios start from (copies made by provision have an empty Saves folder: "loadSave: timed out")."""
        import mygames   # noqa: WPS433
        return V.seed_saves(list(instances), lambda i: self.idir(i) / "Saves", owner_saves=lambda: mygames.saves_dir())

    def wait_ready(self, instances) -> Optional[str]:
        sys.path.insert(0, str(ROOT / "tools" / "runner"))
        import nvready   # noqa: WPS433
        why = nvready.wait_ready_ids(list(instances), READY_WAIT_S)
        return None if not why else f"cannot reach the game instances: {why}"

    def methods_missing(self, instances, methods) -> List[str]:
        sys.path.insert(0, str(ROOT / "tools" / "runner"))
        import nvbridge
        miss = []
        for i in instances[:1]:
            try:
                with nvbridge.NvBridge.connect(i, wait_s=5) as nv:
                    for m in methods:
                        try:
                            nv.call(m, {}, timeout_s=10)
                        except nvbridge.RpcError as e:
                            if e.code == METHOD_NOT_FOUND:
                                miss.append(m)
                        except nvbridge.BridgeError:
                            pass
            except nvbridge.BridgeError:
                pass
        return miss

    def _acquire_slot(self, copies: int, ttl: int, new_copies: int = 0) -> Optional[str]:
        slots = max(1, (copies + 1) // 2)      # re-acquiring as the same owner renews and adds slots: two pairs hold two
        # the slot's RAM check follows what this caller needs (the copies it has yet to boot x 0.7 GB + its --reserve-gb), the same number as the guard
        need = S.slot_need_gb(new_copies, getattr(self.args, 'reserve_gb', S.RESERVE_GB))
        code, out = self._slot(["acquire", "--owner", OWNER, "--ttl", str(ttl), "--slots", str(slots), "--wait", "120", "--need-gb", f"{need:.2f}"])
        self.slot_held = code == 0
        return None if code == 0 else f"game slot busy: {out[:120]}"

    def _boot(self, i: int, env_extra: Dict[str, str]) -> Optional[str]:
        import importlib
        run = importlib.import_module("run")
        saved = dict(os.environ)
        try:
            os.environ["FNV_LANE"] = OWNER                  # run.py registers the pid on OUR slot lease
            os.environ.update(env_extra)               # S.boot_env(): NVCOOP_TEST_HELPERS=1 plus the test's game_env
            pid = run.launch_instance(i)
            st = run.load_state()
            st["pids"][str(i)] = pid
            run.save_state(st)
        finally:
            os.environ.clear()
            os.environ.update(saved)                        # one test's game_env must not leak into the next boot
            os.environ["FNV_LANE"] = OWNER
        self.owned.add(i, pid, env=env_extra)
        self.watcher.watch(i, pid)
        msg = S.wait_for_pipe(i, pid, up=self.up, alive=S.pid_alive)
        if msg and msg.startswith("Steam is not"):
            self.finish(True, [i])                          # do not leave the "Steam Error" dialog behind
        return msg

    def prepare(self, test, instances) -> Tuple[Optional[str], Optional[str], bool]:
        if not instances:
            return None, None, True
        with self._prep:
            return self._prepare(test, instances)

    def _prepare(self, test, instances) -> Tuple[Optional[str], Optional[str], bool]:
        why = self._acquire_slot(len(self.in_use(instances)), int(test.get("timeout_s", 900)) + 600, sum(1 for i in instances if not self.up(i)))
        if why:
            return why, None, False
        want_env = S.boot_env(test.get("game_env"))
        self.adopt_orphans()
        own0 = self.owned.load()
        stale = [i for i in instances if self.up(i) and str(i) in own0 and not S.env_satisfied(self.owned.env_of(i), want_env)]
        if stale:                                         # booted without the env this test needs (test helpers): only a restart can fix that
            err(f"restarting copies {stale}: started without {want_env}")
            self.finish(True, stale)
            t0 = time.time()
            while any(self.up(i) for i in stale) and time.time() - t0 < 30:
                time.sleep(1)
        bad = self.ini_problem(test, instances)
        build_bad = self.build_problem(instances)         # every pair runs the build pair 0 runs, or the test is refused as BUILD
        if (bad or build_bad) and self.args.provision and all(not self.up(i) or str(i) in self.owned.load() for i in instances):
            self.finish(True, instances)                  # only these copies: the other pair may be mid-test
            t0 = time.time()
            while any(self.up(i) for i in instances) and time.time() - t0 < 30:   # a dying game still holds its DLLs: provision after it is gone
                time.sleep(1)
            pc = self.provision_cmd(test, instances)
            err("provisioning: python -I " + shell_join(pc))
            subprocess.run(PY + [str(ROOT / pc[0])] + pc[1:], cwd=ROOT, capture_output=True, text=True)
            bad = self.ini_problem(test, instances)
            build_bad = self.build_problem(instances)
        for line in (self.seed_saves(instances) if MUTANT != "saves_not_seeded" else []):
            err("saves: " + line)
        if bad:
            return None, f"switch off or instance missing: {bad}. Fix: python -I {shell_join(self.provision_cmd(test, instances))}", False
        if build_bad:
            return None, f"build mismatch: {build_bad}. Fix: python -I {shell_join(self.provision_cmd(test, instances))}", False
        for i in instances:
            if not self.up(i):
                if not self.args.boot:
                    return f"bridge pipe nvcoop-{i} is not up and --no-boot was given", None, False
                msg = self._boot(i, want_env)
                if msg:
                    return msg, None, False
                time.sleep(2)
        why = self.wait_ready(instances) if MUTANT != "ready_skipped" else None   # a pipe that is up is not a game that serves requests (still booting, or loading a save)
        if why:
            return why, None, False
        miss = self.methods_missing(instances, test.get("requires_methods", []))
        if miss:
            return None, "bridge method missing in this build: " + ", ".join(miss), False
        own = self.owned.load()
        fresh = all(str(i) in own and not self._swapped(i, own[str(i)]["booted_at"]) for i in instances)
        return None, None, fresh

    def _swapped(self, i: int, booted_at: float) -> bool:
        d = self.idir(i) / "Data" / "NVSE" / "Plugins"
        return any(f.exists() and f.stat().st_mtime > booted_at + 1 for f in (d / "nvcoop_mod.dll", d / "nvcoop.dll"))

    # -- evidence
    def snapshot(self, instances, methods) -> Dict[str, Dict[int, Any]]:
        if not methods:
            return {}
        import nvbridge
        out: Dict[str, Dict[int, Any]] = {}
        for i in instances:
            try:
                with nvbridge.NvBridge.connect(i, wait_s=3) as nv:
                    for m in methods:
                        try:
                            out.setdefault(m, {})[i] = nv.call(m, {}, timeout_s=10)
                        except nvbridge.BridgeError:
                            continue
            except nvbridge.BridgeError:
                continue
        return out

    def inspect(self, instances) -> Tuple[Optional[list], List[str]]:
        cp = subprocess.run(PY + [str(ROOT / "tools" / "bots" / "inspect_live.py"), "--instances", ",".join(map(str, instances)), "--json", "--wait", "3"],
                            capture_output=True, text=True, timeout=90)
        try:
            reports = json.loads(cp.stdout)
        except ValueError:
            return None, []
        sys.path.insert(0, str(ROOT / "tools" / "bots"))
        import inspect_live   # noqa: WPS433
        return reports, inspect_live.problems(reports)

    def crashes(self, instances: Optional[List[int]] = None) -> List[Dict[str, Any]]:
        return self.watcher.exited(instances)

    def crash_info(self, instance: int, since: float, trace: Optional[Path]) -> Dict[str, Any]:
        d = K.collect(self.idir(instance), since, trace)
        d["instance"] = instance
        return d

    def warn_lines(self, instances) -> Dict[int, List[str]]:
        out = {}
        for i in instances:
            try:
                out[i] = P.warn_lines((self.idir(i) / "Data" / "NVSE" / "Plugins" / "nvcoop.log").read_text(encoding="utf-8", errors="replace"))
            except OSError:
                pass
        return out

    def finish(self, stop: bool, instances: Optional[List[int]] = None) -> None:
        """Stop the copies prove.py started (only `instances` when given) and, when everything is stopped, release the slot."""
        if stop:
            run = None
            try:
                import importlib
                run = importlib.import_module("run")
            except Exception:       # noqa: BLE001
                pass
            for i, v in list(self.owned.load().items()):
                if instances is not None and int(i) not in instances:
                    continue
                try:
                    S.kill_owned(int(v["pid"]), self.owned)
                except S.NotOwned:
                    pass
                self.owned.forget(int(i))
                self.watcher.unwatch(int(i))
                if run:
                    st = run.load_state()
                    if st["pids"].pop(str(i), None) is not None:
                        run.save_state(st)
            if self.slot_held and instances is None:
                self._slot(["release", "--owner", OWNER])
                self.slot_held = False


# ------------------------------------------------------------------ running commands
def start(argv: List[str], out: Path, live: bool, trace: Optional[Path]) -> subprocess.Popen:
    full = list(argv)
    if full[0].endswith(".py"):
        full = PY + ([str(HERE / "pf_trace.py"), str(trace)] if (live and trace) else []) + [str(ROOT / full[0])] + full[1:]
    f = open(out, "w", encoding="utf-8", errors="replace")
    # Off Windows the runner leads its own process group, so kill_tree stops the children it started too (taskkill /T on Windows).
    group = {} if os.name == "nt" else {"start_new_session": True}
    return subprocess.Popen(full, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, **group)


def kill_tree(p: subprocess.Popen) -> None:
    """Stops a runner this tool started (and its children). Game copies are not its children: they are launched detached."""
    if p.poll() is None:
        if os.name == "nt":
            subprocess.run(["taskkill", "/PID", str(p.pid), "/F", "/T"], capture_output=True)
        else:
            try:
                leads = os.getpgid(p.pid) == p.pid and MUTANT != "kill_runner_only"
            except ProcessLookupError:
                leads = False
            if leads:
                os.killpg(p.pid, 9)   # the group start() made: the runner and the children it started, nothing else
            else:
                p.kill()
        try:
            p.wait(5)
        except subprocess.TimeoutExpired:
            pass


def run_commands(cmds: List[Dict[str, Any]], argvs: List[List[str]], outdir: Path, timeout_s: float, env: NullEnv, poll: float = 1.0,
                 instances: Optional[List[int]] = None) -> Dict[str, Any]:
    """Run the commands: `concurrent` ones in the background, the others one after the other (stop at the first failure).
    -> {"results": [{name, code, out}], "timeout": bool, "crash": [..], "secs": float}"""
    outdir.mkdir(parents=True, exist_ok=True)
    trace = outdir / "bridge_trace.json"
    t0 = time.time()
    res: List[Dict[str, Any]] = []
    bg: List[Tuple[Dict[str, Any], subprocess.Popen, Path]] = []
    crash: List[Dict[str, Any]] = []
    timed_out = False
    live = env.live

    def watch(p: subprocess.Popen) -> str:
        """'' while running or finished normally; 'timeout' or 'crash' when the run must stop."""
        nonlocal timed_out
        while p.poll() is None:
            dead = env.crashes(instances)            # only this test's copies: a death on the other pair is not this test's crash
            if dead:
                crash.extend(dead)
                return "crash"
            if time.time() - t0 > timeout_s:
                timed_out = True
                return "timeout"
            time.sleep(poll)
        return ""

    try:
        for c, a in zip(cmds, argvs):
            if c.get("concurrent"):
                o = outdir / f"{c['name']}.out.txt"
                bg.append((c, start(a, o, live, None), o))
        for c, a in zip(cmds, argvs):
            if c.get("concurrent"):
                continue
            o = outdir / f"{c['name']}.out.txt"
            p = start(a, o, live, trace)
            stop = watch(p)
            if stop:
                kill_tree(p)
            res.append({"name": c["name"], "code": p.returncode if not stop else (-1 if stop == "timeout" else -2), "out": str(o), "argv": a})
            if stop or p.returncode != 0:
                break
        main_failed = any(r["code"] != 0 for r in res)
        for c, p, o in bg:
            if not (timed_out or crash or main_failed):
                stop = watch(p)
                if stop:
                    kill_tree(p)
            kill_tree(p)
            res.append({"name": c["name"], "code": p.returncode if p.returncode is not None else -1, "out": str(o), "argv": []})
    finally:
        for _c, p, _o in bg:
            kill_tree(p)
    return {"results": res, "timeout": timed_out, "crash": crash, "secs": round(time.time() - t0, 1), "trace": trace}


# ------------------------------------------------------------------ reading what a runner left behind
def read_text(path: str, limit: int = 200_000) -> str:
    try:
        return Path(path).read_text(encoding="utf-8", errors="replace")[-limit:]
    except OSError:
        return ""


def newest(outdir: Path, pattern: str) -> Optional[Path]:
    found = sorted(glob.glob(str(outdir / "**" / pattern), recursive=True), key=os.path.getmtime, reverse=True)
    return Path(found[0]) if found else None


def read_failure(outdir: Path) -> Optional[Dict[str, Any]]:
    p = newest(outdir, "failure.json")
    if not p:
        return None
    try:
        d = json.loads(p.read_text(encoding="utf-8"))
        d["_path"] = str(p)
        return d
    except (OSError, ValueError):
        return None


def summary_reason(outdir: Path) -> Optional[str]:
    p = newest(outdir, "summary.json")
    if not p:
        return None
    try:
        d = json.loads(p.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return None
    for r in d.get("results", []):
        if not r.get("ok") and r.get("reason"):
            return f"{r.get('playbook')}: {r['reason']}"
    return None


def from_failure(outdir: Path) -> Optional[Dict[str, Any]]:
    """Replay the recordings of the newest failure under outdir. -> one tools/flight/from_failure.py report, or None."""
    if not newest(outdir, "failure.json"):
        return None
    try:
        cp = subprocess.run(PY + [str(ROOT / "tools" / "flight" / "from_failure.py"), str(outdir), "--json"], capture_output=True, text=True, timeout=240, cwd=ROOT)
        reps = json.loads(cp.stdout)
    except (subprocess.SubprocessError, ValueError, OSError):
        return None
    reps = [r for r in reps if isinstance(r, dict)]
    pick = next((r for r in reps if r.get("verdict") == "reproduced"), reps[0] if reps else None)
    return pick


RERUN_LINE = re.compile(r"^\s*(python -I tools/\S+.*--(?:from|only)-step \d+.*)$", re.M)


# ------------------------------------------------------------------ one run
class Prover:
    def __init__(self, manifest: Dict[str, Any], args, env: NullEnv, ledger: Optional[Path] = None, stamp: Optional[str] = None):
        self.m, self.args, self.env = manifest, args, env
        self.fake = bool(getattr(args, "fake", False))
        self.ledger = ledger or (OUT / ("ledger.fake.jsonl" if self.fake else "ledger.jsonl"))
        self.stamp = stamp or time.strftime("%Y%m%d-%H%M%S")
        self.build = build_id(self.fake)
        self.params = dict(manifest["params"])
        self.tests = {t["id"]: t for t in manifest["tests"]}

    # -- vars
    def vars_for(self, test, outdir: Path) -> Dict[str, Any]:
        ins = instances_of(test)
        v = dict(self.params)
        v.update({"instances": ",".join(map(str, ins)), "instances_sp": " ".join(map(str, ins)), "outdir": outdir.relative_to(ROOT).as_posix()
                  if outdir.is_relative_to(ROOT) else str(outdir), "i0": ins[0] if ins else 0, "host": ins[0] if ins else 0,
                  "client": ins[1] if len(ins) > 1 else 1})
        if self.env.live:
            v["inst_dir0"] = rel_or_abs(self.env.idir(ins[0])) if ins else "."    # type: ignore[attr-defined]
        else:
            v["inst_dir0"] = "."
        v["game_dir"] = "."
        return v

    def verdict_line(self, test, run, of, verdict, cls, reason, check, repro, counters, rerun, packet, secs) -> Dict[str, Any]:
        home = str(Path.home())
        pair = X.pair_of(instances_of(test)) if MUTANT != "pair_missing" else None
        return {"test": test["id"], "roadmap": test["roadmap"], "run": run, "of": of, "pair": pair, "verdict": verdict, "class": cls,
                "reason": P.scrub(reason or "", home)[:200], "check": P.scrub(check, home) if check else check, "repro": repro, "counters": counters,
                "rerun": P.scrub(rerun or "", home), "packet": packet, "secs": secs, "build": self.build}

    def emit(self, v: Dict[str, Any], fresh: bool, clean: bool) -> Dict[str, Any]:
        rec = dict(v)
        rec.update({"fresh": fresh, "clean": clean, "fake": self.fake})
        if getattr(self.args, "round", None):
            rec["round"] = self.args.round       # retest round: a restarted queue skips what has a verdict in it (pf_ledger.round_done)
        if self.is_short(v["test"]):
            rec["short"] = True                  # asked for with --runs below the test's N: recorded, never counts as proven
        with EMIT_LOCK:                          # two pairs finish at the same time: one whole line each, one writer at the ledger
            print(json.dumps({k: v[k] for k in SCHEMA_KEYS}, separators=(",", ":")), flush=True)
            L.append(self.ledger, rec)
        return v

    def is_short(self, test_id: str) -> bool:
        asked = int(getattr(self.args, "runs", 0) or 0)
        t = self.tests.get(test_id)
        return MUTANT != "short_unmarked" and t is not None and 0 < asked < int(t.get("n", 1))

    def rerun_hint(self, test, argvs_text: str) -> str:
        return f"python -I tools/proof/prove.py run --test {test['id']} --runs 1" + (" --fake" if self.fake else "")

    # -- the run itself
    def run_once(self, test: Dict[str, Any], run: int, of: int, attempt: str = "") -> Tuple[Dict[str, Any], bool]:
        """-> (verdict dict, fresh). Writes logs under out/proof/<stamp>/<test>/run<k>[<attempt>]."""
        t_start = time.time()
        outdir = OUT / self.stamp / test["id"] / f"run{run}{attempt}"
        ins = instances_of(test)
        rerun = self.rerun_hint(test, "")
        counters: Dict[str, float] = {}

        def done(verdict, cls, reason, check, repro=None, packet=None, fresh=False, counters=None):
            v = self.verdict_line(test, run, of, verdict, cls, reason, check, repro, counters or {}, rerun, packet, round(time.time() - t_start, 1))
            return v, fresh

        if not self.fake and test.get("blocked_reason"):
            return done("BLOCKED", "TEST", "blocked: " + test["blocked_reason"], "manifest")
        cmds = test.get("fake_commands") if self.fake else test["commands"]
        if not cmds:
            return done("BLOCKED", "TEST", "no fake world for this test", "manifest")
        vars_ = self.vars_for(test, outdir)
        argvs, missing = [], None
        for c in cmds:
            a, miss = expand(c["argv"], vars_)
            if miss:
                missing = miss
                break
            if self.fake and self.args.bug and test.get("fake_bug") and c["argv"][0].endswith(("run_fight.py", "run_coop.py", "run_vats.py", "run_questlines.py")):
                a = a + ["--bug", self.args.bug]
            argvs.append(a)
        if missing:
            return done("BLOCKED", "TEST", f"blocked: param {missing} has no value (set it in tools/proof/params.local.json)", "param")
        rerun = "python -I " + shell_join(argvs[-1]) if argvs else rerun

        if test.get("alone") and not self.fake:
            self.env.finish(True)                     # the driver starts its own copies and wants a machine with none of ours running
        env_why = self.env.preflight(test) if not self.fake else None
        if env_why:
            return done("ENV", "ENV", env_why, "preflight")
        env_why, build_why, fresh = self.env.prepare(test, ins)
        if env_why:
            return done("ENV", "ENV", env_why, "prepare")
        if build_why:
            return done("BLOCKED", "BUILD", build_why, "prepare")
        if test.get("preflight") and not self.fake:
            pa, miss = expand(test["preflight"], vars_)
            if pa:
                cp = subprocess.run(PY + [str(ROOT / pa[0])] + pa[1:], cwd=ROOT, capture_output=True, text=True, timeout=180)
                if cp.returncode == 3:
                    return done("BLOCKED", "BUILD", C.reason_from_output(cp.stdout + cp.stderr), "preflight", fresh=fresh)
                if cp.returncode == 2:
                    return done("ENV", "ENV", "preflight could not reach the game instances", "preflight", fresh=fresh)

        before = C.snapshot_flat(self.env.snapshot(ins, test.get("snapshot", [])))
        run_info = run_commands(cmds, argvs, outdir, float(test.get("timeout_s", 900)), self.env, instances=ins)
        results = run_info["results"]
        failing = next((r for r in results if r["code"] != 0), None)
        text = "\n".join(read_text(r["out"]) for r in results)
        code = 0 if failing is None else failing["code"]
        crash = None
        crash_since = t_start - 5
        if run_info["crash"]:
            first = run_info["crash"][0]
            crash = self.env.crash_info(first["instance"], crash_since, run_info["trace"])
            crash.update({"exit_code": first["exit_code"]})
        # evidence, taken before anything is torn down
        insp, insp_problems = (None, [])
        after: Dict[str, float] = before
        if failing is not None or run_info["timeout"] or crash:
            if not crash:
                insp, insp_problems = self.env.inspect(ins) if ins else (None, [])
        if not crash:
            after = C.snapshot_flat(self.env.snapshot(ins, test.get("snapshot", [])))
        mv = C.moved(before, after)
        bad = C.unclean(mv, bool(test.get("allow_repairs")), test.get("must_zero"))
        failure = read_failure(outdir)
        ff = None
        if failing is not None and not crash and not run_info["timeout"]:
            ff = from_failure(outdir)
        ev = {"timeout": run_info["timeout"], "crash": crash, "exit": code if not run_info["timeout"] else None, "setup_text": text,
              "from_failure": (ff or {}).get("verdict"), "sync_symptoms": ((ff or {}).get("status") or {}).get("symptoms", []),
              "inspect_sync": [p for p in insp_problems if C.SYNC_PROBLEM.search(p)], "bad_counters": bad if (failing is not None or bad) else [],
              "failure_kind": (failure or {}).get("kind"), "reason": (failure or {}).get("reason") or text[-400:]}
        if run_info["timeout"] and not ev["exit"]:
            ev["exit"] = -1
        verdict = C.classify(ev)

        # reason, check, rerun line
        if verdict["verdict"] == "PASS":
            reason, check = "", None
        else:
            if crash:
                reason = K.one_line(crash)
                last = (crash.get("last_calls") or [])[-1:]
                reason = reason + (f"; last bridge call {last[0]}" if last else "")
            elif run_info["timeout"]:
                reason = f"timeout after {test.get('timeout_s', 900)} s in {results[-1]['name'] if results else '?'}"
            elif bad and code == 0:
                reason = "test passed but not clean: " + ", ".join(bad)[:150]
            else:
                reason = (failure or {}).get("reason") or summary_reason(outdir) or C.reason_from_output(text)
            check = C.check_name(failure, reason, (failing or {}).get("name", "run"))
            m = RERUN_LINE.search(text)
            if m:
                rerun = m.group(1).strip()
        cls = verdict["class"]
        out_v = verdict["verdict"]
        if out_v == "PASS" and bad:       # cannot happen (bad forces a failure) but keep the rule explicit
            out_v, cls = "FAIL", "SYNC"
        sus = C.suspicious_moved(mv)
        packet_rel = None
        if out_v != "PASS":
            ctx = {"verdict": self.verdict_line(test, run, of, out_v, cls, reason, check, verdict["repro"], sus, rerun, None, 0, ), "failure": failure,
                   "inspect": insp, "inspect_problems": insp_problems, "moved": {C.strip_inst(k): d for k, d in mv.items()}, "from_failure": ff,
                   "crash": crash, "warn_lines": self.env.warn_lines(ins) if ins else {}, "home": str(Path.home())}
            pk = P.write(outdir, ctx)
            packet_rel = pk.relative_to(ROOT).as_posix() if pk.is_relative_to(ROOT) else str(pk)
        v = self.verdict_line(test, run, of, out_v, cls, reason, check, verdict["repro"], sus, rerun, packet_rel, round(time.time() - t_start, 1))
        return v, fresh

    def run_with_rerun(self, test, run, of, rerun_on_fail=True) -> Dict[str, Any]:
        v, fresh = self.run_once(test, run, of)
        if v["verdict"] == "FAIL" and rerun_on_fail and not self.args.no_rerun:
            err(f"{test['id']} run {run}: FAIL ({v['class']}), rerunning once to detect FLAKY")
            v2, _ = self.run_once(test, run, of, attempt="-rerun")
            if v2["verdict"] == "PASS":
                v = dict(v, verdict="PASS" if MUTANT == "flaky_as_pass" else "FLAKY")
                v["reason"] = ("passed on the automatic rerun; first failure: " + v["reason"])[:200]
            else:
                v = dict(v, reason=(v["reason"] + " (rerun failed too)")[:200])
        self.emit(v, fresh, v["verdict"] == "PASS")
        return v

    def run_test(self, test, runs: Optional[int] = None, start: int = 1) -> List[Dict[str, Any]]:
        """Up to N consecutive runs, stopping at the first one that is not PASS."""
        of = int(test.get("n", 1))
        total = runs or of
        out = []
        for k in range(start, start + total):
            v = self.run_with_rerun(test, min(k, of), of)
            out.append(v)
            if v["verdict"] != "PASS":
                break
        return out


# ------------------------------------------------------------------ commands
def bug_tests(path: Any) -> List[str]:
    """Test ids of the open rows of a bugs file (out/proof/bugs3.json: {"rows": [{"test", "status"}]}). A row marked fixed / closed / done is not
    retested; the same test in several rows counts once."""
    d = json.loads(Path(path).read_text(encoding="utf-8"))
    rows = d.get("rows", []) if isinstance(d, dict) else d
    done = ("fixed", "closed", "done", "wontfix", "resolved")
    out: List[str] = []
    for r in rows:
        tid = r.get("test")
        if tid and str(r.get("status", "open")).lower() not in done and tid not in out:
            out.append(tid)
    return out


def select(m: Dict[str, Any], args) -> List[Dict[str, Any]]:
    ts = m["tests"]
    asked = [x for t in (getattr(args, "test", None) or []) for x in t.split(",") if x]
    if getattr(args, "from_bugs", None):
        asked += [x for x in bug_tests(args.from_bugs) if x not in asked]
    if asked:
        args.test = asked
    if getattr(args, "test", None):
        want = [x for t in args.test for x in t.split(",")]
        unknown = [w for w in want if w not in {t["id"] for t in ts}]
        if unknown:
            raise SystemExit(f"unknown test(s) {unknown}; ids: " + ", ".join(t["id"] for t in ts))
        ts = [t for t in ts if t["id"] in want]
    if getattr(args, "step", None):
        ts = [t for t in ts if t["roadmap"] == args.step or args.step in t.get("also_roadmap", [])]
    return sorted(ts, key=lambda t: (t["roadmap"], m["tests"].index(t)))


def make_env(args) -> NullEnv:
    return NullEnv() if args.fake else LiveEnv(args)


def cmd_run(args) -> int:
    m = load_manifest()
    tests = select(m, args)
    if not tests:
        err("no test selected: give --test ID or --step N")
        return 2
    return drive(m, args, tests, queue=False)


def cmd_queue(args) -> int:
    m = load_manifest()
    return drive(m, args, select(m, args), queue=True)


def plan_test(pv: "Prover", args, t: Dict[str, Any], queue: bool) -> Optional[Tuple[int, int]]:
    """-> (first run number, number of runs) for a test, or None when the queue skips it (proven, or blocked and already reported)."""
    entries = L.load(pv.ledger)
    rnd = getattr(args, "round", None)
    if rnd and L.round_done(entries, t["id"], rnd, bool(args.fake)):
        err(f"skip {t['id']}: has its verdict in round {rnd}")
        return None
    if not queue:
        return 1, (args.runs or None)          # type: ignore[return-value]
    build = None if args.fake else pv.build
    st = L.state_of(t, entries, build, include_fake=args.fake)
    if st["state"] == "proven" and MUTANT != "queue_runs_ignored_proven":
        err(f"skip {t['id']}: proven")
        return None
    if st["state"] == "blocked" and not args.fake:
        last = [e for e in entries if e.get("test") == t["id"]][-1:]
        if last and last[0].get("verdict") == "BLOCKED" and last[0].get("reason", "").startswith("blocked:"):
            return None
    n = int(t.get("n", 1))
    asked = int(getattr(args, "runs", 0) or 0)
    if asked and asked != n:
        # --runs overrides this queue's N. Below N the passes are "short" (recorded, never proven): what a restarted queue has already
        # done is every clean entry at the end of the ledger, short or not, so it resumes where it stopped and never starts over.
        s = L.streak(entries, t["id"], build, args.fake)
        have = s["any_clean"] if MUTANT != "queue_runs_resume_blind" else 0
        if have >= asked:
            err(f"skip {t['id']}: {have} clean run(s) already, --runs {asked}")
            return None
        return have + 1, asked - have
    have = st["clean"] if st["state"] == "partial" else 0
    return have + 1, max(1, n - have)


def drive(m, args, tests, queue: bool) -> int:
    """Run the selected tests. With --pairs N up to N tests run at the same time (pf_sched.pick says which may start next); with the
    default of one pair they run one after the other, exactly as before. Verdict lines appear in the order the tests finish."""
    lock = S.QueueLock(OUT / ("queue.fake.lock" if args.fake else "queue.lock"))
    why = lock.acquire()
    if why:
        print(json.dumps({"test": None, "verdict": "ENV", "class": "ENV", "reason": why[:200]}), flush=True)
        return 4
    env = make_env(args)
    pv = Prover(m, args, env)
    rc, env_streak = 0, 0
    pairs, note = env.pairs_allowed(max(1, int(getattr(args, "pairs", 1) or 1)))
    if note:
        err(note)
    done: "_queue.Queue" = _queue.Queue()
    running: Dict[int, List[int]] = {}           # worker number -> the instances it uses
    pending = list(tests)
    halted, crashed = False, None
    started: Dict[int, Dict[str, Any]] = {}      # worker number -> its test
    deferred: List[Dict[str, Any]] = []          # tests the RAM guard refused: the queue moves on and tries them once more at the end
    retrying = False

    def work(n: int, t: Dict[str, Any], ins: List[int], start: int, runs: Optional[int]) -> None:
        try:
            env.hold(ins)
            try:
                done.put((n, pv.run_test(t, runs, start), None))
            finally:
                env.unhold(ins)
        except BaseException as e:               # noqa: BLE001  a bug in one worker must not leave the queue waiting for it
            done.put((n, None, e))

    seq = 0
    try:
        while True:
            while pending and not halted:
                t = pending[0]
                plan = plan_test(pv, args, t, queue)
                if plan is None:
                    pending.pop(0)
                    continue
                ins = X.pick(t, list(running.values()), pairs)
                if ins is None:
                    break
                pending.pop(0)
                t = dict(t, pair=X.pair_of(ins) or 0) if X.copies_of(t) == 2 else t
                seq += 1
                running[seq] = X.claim(t, ins, pairs)
                started[seq] = t
                err(f"start {t['id']}" + (f" on pair {X.pair_of(ins)}" if ins else "") + (f" (instances {','.join(map(str, ins))})" if ins else ""))
                th = threading.Thread(target=work, args=(seq, t, ins, plan[0], plan[1]), daemon=True)
                th.start()
            if not running:
                if deferred and not halted and not pending:
                    pending, deferred, retrying = deferred, [], True
                    err("retrying the tests the RAM guard refused: " + ", ".join(x["id"] for x in pending))
                    continue
                break
            while True:
                try:
                    n, vs, exc = done.get(timeout=1)         # a timeout so Ctrl+C still gets through on Windows
                    break
                except _queue.Empty:
                    continue
            running.pop(n, None)
            tdone = started.pop(n, None)
            if exc is not None:
                halted, crashed = True, crashed or exc
                continue
            last = vs[-1]
            ram = (last["verdict"] == "ENV" and last["reason"].startswith("RAM guard") and queue and MUTANT != "ram_halts_queue")
            if ram:
                # This test needs more copies than fit in the free RAM right now. That says nothing about the machine being unusable for the
                # others, so the verdict stays on the ledger as ENV and the queue goes on; once everything else ran, the test gets one more try.
                if tdone is not None and not retrying:
                    deferred.append(tdone)
                elif rc != 4:
                    rc = 1
                continue
            if last["verdict"] != "PASS" and rc != 4:
                rc = 1
            if last["verdict"] == "ENV":
                env_streak += 1
                if queue and (env_streak >= 3 or (last["reason"].startswith(("owner is playing", "Steam is not", "game slot busy")))):
                    err("queue stopped on an ENV problem: " + last["reason"])
                    rc = 4
                    halted = True
            else:
                env_streak = 0
    finally:
        env.finish(stop=not getattr(args, "keep_up", False))
        lock.release()
    if crashed is not None:
        raise crashed
    return rc


def cmd_status(args) -> int:
    m = load_manifest()
    path = OUT / ("ledger.fake.jsonl" if args.fake else "ledger.jsonl")
    rows = L.step_table(m, L.load(path), None if args.fake else build_id(False), include_fake=args.fake)
    if args.step:
        rows = [r for r in rows if r["step"] == args.step]
    if args.json:
        print(json.dumps({"build": build_id(args.fake), "steps": [{k: r[k] for k in ("step", "total", "proven", "pct", "counts")} | {"tests": {t: s["state"] for t, s in r["tests"].items()}}
                                                               for r in rows]}))
    else:
        print(L.render_table(rows, verbose=not args.brief))
    return 0


WIRING_LANES = ("actors", "avatars", "combat", "spawns")
LOAD_SCENARIO = "load_scenario.py"


def _script(c: Dict[str, Any]) -> str:
    return str(c["argv"][0]).rsplit("/", 1)[-1] if c.get("argv") else ""


def reads_lone_npc(c: Dict[str, Any]) -> bool:
    """A single-copy command that reads an NPC of a loaded world on a copy that was just booted (the two-copy fight tests jump to a scenario themselves)."""
    return _script(c) == "proof_actor_port_save_check.py" or (_script(c) == "npc_proof.py" and "caps" in c["argv"][1:2])


def wiring_problem(t: Dict[str, Any]) -> Optional[str]:
    """A test whose command runs live_wiring_proof.py must provision exactly the live.* lanes that command expects: the proof says "nothing is
    on that was not asked for", so extra lanes in `ini` fail it (actor_port_two_copy: got avatars=1 combat=1 spawns=1, wanted actors only)."""
    if MUTANT == "wiring_unchecked":
        return None
    for c in t.get("commands") or []:
        a = c["argv"]
        if _script(c) == "live_wiring_proof.py" and "--expect" in a:
            want = {x for x in str(a[a.index("--expect") + 1]).split(",") if x and x != "none"}
            have = {spec.split("=")[0].split(".", 1)[1] for spec in t.get("ini", []) if spec.startswith("live.") and spec.split("=", 1)[1] == "1"} & set(WIRING_LANES)
            if have != want:
                return f"ini switches lanes {sorted(have)} but the wiring command expects {sorted(want)}"
    return None


def npc_problem(t: Dict[str, Any]) -> Optional[str]:
    """A test that reads a live NPC needs a loaded world next to him: it must say so (`needs_npc`) and its first command must load the scenario."""
    if MUTANT == "npc_load_unchecked":
        return None
    cmds = t.get("commands") or []
    uses = any(reads_lone_npc(c) for c in cmds)
    if uses and not t.get("needs_npc"):
        return "reads a live NPC but is not marked needs_npc (a booted copy has no save loaded)"
    if t.get("needs_npc") and (not cmds or _script(cmds[0]) != LOAD_SCENARIO):
        return "needs_npc: the first command must be drivers/load_scenario.py"
    return None


# Proof scripts that need the player standing in a loaded cell. A freshly booted copy sits in the main menu: the avatar runtime is created only
# when the player has a cell and 3D, so these scripts read "avatar runtime not created yet (no player in a cell)" / "waiting for the player" /
# "the host has no puppet for player 1" (sweep 2026-10-22 13:50: puppet_loopback, puppet_two_copy, avatars_phases, avatars_lifecycle, social_proof;
# loot_proof and worldtime_proof stand the players at a container / door of a scenario).
WORLD_SCRIPTS = ("proof_puppet.py", "proof_avatars.py", "social_proof.py", "loot_proof.py", "worldtime_proof.py")


def world_problem(t: Dict[str, Any]) -> Optional[str]:
    """A test whose script needs the player in a loaded cell must say so (`needs_world`) and its first command must load the scenario."""
    if MUTANT == "world_load_unchecked" or t.get("blocked_reason"):
        return None
    cmds = t.get("commands") or []
    uses = any(_script(c) in WORLD_SCRIPTS for c in cmds)
    if uses and not t.get("needs_world"):
        return "needs a player in a loaded cell but is not marked needs_world (a booted copy sits in the main menu)"
    if t.get("needs_world") and (not cmds or _script(cmds[0]) != LOAD_SCENARIO):
        return "needs_world: the first command must be drivers/load_scenario.py"
    return None


def coop_problem(t: Dict[str, Any]) -> Optional[str]:
    """A test that boots game copies must say whether co-op is on: copies that talk to each other need coop.enable=1 in `ini` (role and port are
    written by provisioning per pair), a lone copy coop.enable=0. Left out, the switch keeps whatever the previous test provisioned
    (actor_port_two_copy ran on copies a single-copy test had left at coop.enable=0 and was BLOCKED)."""
    if MUTANT == "coop_unchecked" or t.get("blocked_reason"):
        return None
    c = int(t.get("copies", 0) or 0)
    if c <= 0:
        return None
    want = "1" if c >= 2 else "0"
    has = [s.split("=", 1)[1] for s in t.get("ini", []) if s.startswith("coop.enable=")]
    if not has:
        return f"{c} cop{'y' if c == 1 else 'ies'} but ini has no coop.enable={want}"
    if has[-1] != want:
        return f"{c} cop{'y' if c == 1 else 'ies'} but ini has coop.enable={has[-1]} (needs {want})"
    return None


def base_problem(params: Dict[str, Any]) -> Optional[str]:
    """spawn_proof.py rolls an encounter with the console line `player.PlaceAtMe <base> 1` run through the bridge's exec, and that console
    does not resolve editor ids (internal lane notes). The sweep of 2026-10-22 gave it "RadRoach": the line made nothing, on any copy, and the
    step read "exactly one more actor (saw [0, 0, 0])". The base must be a hex FormID (FalloutNV.esm has no creature "RadRoach"; the
    radroach is CrRadroach = 0001CF7C)."""
    if MUTANT == "spawn_base_unchecked":
        return None
    base = params.get("spawn_base")
    if base is not None and not re.fullmatch(r"[0-9A-Fa-f]{1,8}", str(base)):
        return f"params.spawn_base {base!r} is not a hex FormID: the bridge console does not resolve editor ids, so the roll makes nothing"
    return None


def weapon_problem(params: Dict[str, Any]) -> List[str]:
    """The VATS weapons are put in the inventory with the console line `additem <hex>`: the console does not resolve editor ids, so a param like
    "WeapNV9mmPistol" (tools/proof/params.local.json, sweep 2026-10-22) must be `Plugin.esm:HEX` (FalloutNV.esm:0E3778 is the 9mm pistol) or null."""
    if MUTANT == "weapon_edid_unchecked":
        return []
    return [f"params.{k} {v!r} is not a Plugin.esm:HEX FormID: the bridge console cannot resolve editor ids (additem makes nothing)"
            for k, v in sorted(params.items()) if k.startswith("weapon_") and v is not None and not re.fullmatch(r"[\w .\-]+\.(?:esm|esp):[0-9A-Fa-f]{1,8}", str(v))]


def check_manifest(m: Dict[str, Any]) -> List[str]:
    """Problems with the manifest: duplicate ids, missing scripts, bad step numbers, unknown placeholders."""
    bad, seen = [], set()
    if base_problem(m["params"]):
        bad.append(base_problem(m["params"]))
    bad.extend(weapon_problem(m["params"]))
    known = set(m["params"]) | {"instances", "instances_sp", "outdir", "i0", "host", "client", "inst_dir0", "game_dir"}
    for t in m["tests"]:
        if t["id"] in seen:
            bad.append(f"duplicate id {t['id']}")
        seen.add(t["id"])
        if not 1 <= t["roadmap"] <= 10:
            bad.append(f"{t['id']}: roadmap {t['roadmap']}")
        for key in ("commands", "fake_commands"):
            for c in t.get(key, []) or []:
                if not c["argv"] or not (ROOT / c["argv"][0]).exists():
                    bad.append(f"{t['id']}: script {c['argv'][:1]} does not exist")
                flat = [x for tok in c["argv"] for x in (tok if isinstance(tok, list) else [tok])]
                for tok in flat:
                    for name in re.findall(r"\{(\w+)\}", tok):
                        if name not in known:
                            bad.append(f"{t['id']}: unknown placeholder {{{name}}}")
        if not t.get("blocked_reason") and not t.get("commands"):
            bad.append(f"{t['id']}: no commands")
        for why in (wiring_problem(t), npc_problem(t), world_problem(t), coop_problem(t)):
            if why:
                bad.append(f"{t['id']}: {why}")
    return sorted(set(bad))


def cmd_check(args) -> int:
    bad = check_manifest(load_manifest())
    for b in bad:
        print(b)
    print("manifest: OK" if not bad else f"manifest: {len(bad)} problem(s)")
    return 1 if bad else 0


def main(argv: Optional[List[str]] = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    def common(p):
        p.add_argument("--fake", action="store_true", help="game-free self-test on the fake worlds (ledger.fake.jsonl)")
        p.add_argument("--bug", default="", help="(--fake) switch on a deliberate bug of the fake world; the test must then FAIL")
        p.add_argument("--no-rerun", action="store_true", help="do not rerun a FAIL once to detect FLAKY")
        p.add_argument("--no-boot", dest="boot", action="store_false", help="never launch game copies; ENV when a pipe is down")
        p.add_argument("--provision", dest="provision", action="store_true", default=MUTANT != "provision_default_off",
                       help="(the default) re-provision stopped or own copies when nvcoop.ini lacks the test switches; copies the owner started are never touched")
        p.add_argument("--no-provision", dest="provision", action="store_false", help="never re-provision: BLOCKED with the provision command when the switches differ")
        p.add_argument("--round", metavar="NAME", help="retest round: every verdict line is tagged with NAME, and a restarted queue with the same NAME skips the "
                       "tests that already have a verdict in it (ENV and BLOCKED-by-setup do not count) instead of starting from the top")
        p.add_argument("--keep-up", action="store_true", help="leave the copies prove.py booted running (and the slot held) at the end")
        p.add_argument("--pairs", type=int, default=1, choices=range(1, X.MAX_PAIRS + 1),
                       help="run up to this many independent two-copy tests at the same time: pair 0 = instances 0,1, pair 1 = instances 2,3 "
                            "(tests with 3 or more copies still run alone); fewer when the free RAM cannot hold the copies")
        p.add_argument("--reserve-gb", type=float, default=S.RESERVE_GB,
                       help="RAM left free for Windows and the tools when checking that the copies fit (each copy needs %.1f GB; default %%(default)s)" % S.COPY_GB)
    def pick(p):
        p.add_argument("--test", "--tests", dest="test", action="append", help="only these tests (comma list; may be repeated): a retest")
        p.add_argument("--from-bugs", metavar="FILE", help="only the tests of the open rows of a bugs file (out/proof/bugs3.json)")
    p = sub.add_parser("run")
    pick(p)
    p.add_argument("--step", type=int)
    p.add_argument("--runs", type=int, default=0, help="number of consecutive runs (default: the N of the manifest)")
    common(p)
    p = sub.add_parser("queue")
    p.add_argument("--step", type=int)
    pick(p)
    p.add_argument("--runs", type=int, default=0,
                   help="override every test's N for this queue (1 = a bug-finding pass). Below a test's manifest N the passes are recorded but "
                        "never count as proven; status says so")
    common(p)
    p = sub.add_parser("status")
    p.add_argument("--step", type=int)
    p.add_argument("--json", action="store_true")
    p.add_argument("--brief", action="store_true", help="one line per step")
    p.add_argument("--fake", action="store_true")
    sub.add_parser("check")
    a = ap.parse_args(argv)
    return {"run": cmd_run, "queue": cmd_queue, "status": cmd_status, "check": cmd_check}[a.cmd](a)


if __name__ == "__main__":
    sys.exit(main())
