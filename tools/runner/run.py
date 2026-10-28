#!/usr/bin/env python3
"""Multi-instance launcher + job runner for fnv-coop (L2).

    run.py provision --n 2 [--plugin-dll out/L1/nvcoop.dll] [--nvse-dir DIR]
                     [--core-dll out/L1split/nvcoop_mod.dll] [--ini SECTION.KEY=VALUE ...] [--no-pad]
        passes these options to `instances.py provision` (see its help). --ini is repeatable, e.g.
        --ini coop.enable=1 --ini dev.bots=1 --ini dev.animprobe=1. provision rewrites nvcoop.ini, so hand
        edits are lost. One --ini value applies to every instance: with --n 2 or more, the per-instance keys
        bridge.instance, coop.instanceId, coop.pipe and coop.role are an error (exit 2).
        --plugin-dll not given: out/L1/nvcoop.dll if it exists, else no plugin. --plugin-dll "": no plugin
        (the same as in instances.py). A --plugin-dll or --core-dll path that does not exist is an error
        (exit 2). --core-dll needs an explicit --plugin-dll (the split loader), else it is an error (exit 2).
    run.py up --n 2 [--wait-pipe 180]       launch instances (host=0, client=1..)
    run.py provision --pairs 3 [--port-base 27960] [--ini coop.enable=1 ...]
    run.py up --pairs 3                     three two-copy pairs at once: pair k = instances 2k (host) + 2k+1 (client) on
        co-op port base+k; `up --pairs` refuses when the free RAM cannot hold the copies (--no-ram-check overrides);
        then tools/bots/run_parallel.py runs one test per pair.
    run.py status
    run.py swap --instances 0,1 [--dll out/L1split/nvcoop_mod.dll] [--force] [--dry-run]
        hot DLL swap: copies the freshly built core next to each RUNNING copy and has its loader reload it, then checks
        that the copy runs exactly that build (no game restart; split build only). Exit 2 = refused, nothing changed.
        Details: tools/runner/hotswap.py.
    run.py down
    run.py job --id vms01 --n 2 [--timeout 900] -- python tools/runner/scenario.py ...
        leases n free instances (file locks), verifies pinned hashes, runs the command with
        NVCOOP_INSTANCES=0,1 in the env, copies logs/dumps to runs/<id>/, releases the lease.

Steam notes (empirically verified on FalloutNV.exe 1.4.0.525, Steam build):
  * Launching an instance exe WITHOUT Steam's env makes the game ask Steam to start the original game
    (a second FalloutNV.exe from the Steam dir appears ~7 s later). Launching with
    SteamAppId=SteamGameId=22380 (+SteamClientLaunch=1) in the environment prevents that.
  * The game has a single-instance mutex "FalloutNVGameMutex" (CreateMutexA). With multiInstance=1 in
    nvcoop.ini the nvcoop plugin renames it in-process (IAT hook: FalloutNVGameMutex-<pid>); the runner
    never touches another process's handles.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import threading
import sys
import time
from pathlib import Path
from typing import Dict, List, Optional

sys.path.insert(0, str(Path(__file__).resolve().parent))
import instances  # noqa: E402
import jobqueue  # noqa: E402
import hotswap  # noqa: E402
import pairs  # noqa: E402

REPO = Path(__file__).resolve().parents[2]
STEAM_ENV = {"SteamAppId": instances.STEAM_APPID, "SteamGameId": instances.STEAM_APPID, "SteamClientLaunch": "1"}
DEFAULT_DLL = REPO / "out" / "L1" / "nvcoop.dll"


GAMESLOT = REPO / "tools" / "gameslot.py"
OWNER = os.environ.get("FNV_LANE", "L2")


def gameslot_cli(*args: str) -> "tuple[int, str]":
    cp = subprocess.run([sys.executable, "-I", str(GAMESLOT), *args], capture_output=True, text=True)
    return cp.returncode, (cp.stdout + cp.stderr).strip()


class SlotBusy(Exception):
    pass


def slot_acquire(owner: str = OWNER, ttl: int = 900, wait: int = 3600) -> str:
    """The only way a runner launches/uses the game (CONTRACTS 'Game slot'). Idempotent for the same owner."""
    code, out = gameslot_cli("acquire", "--owner", owner, "--ttl", str(ttl), "--wait", str(wait))
    if code != 0:
        raise SlotBusy(out)
    return out


def slot_release(owner: str = OWNER) -> str:
    return gameslot_cli("release", "--owner", owner)[1]


def register_pid(pid: int, owner: str = OWNER) -> None:
    gameslot_cli("register-pid", "--owner", owner, "--pid", str(pid))


def state_path() -> Path:
    return instances.instances_root() / "state.json"


def load_state() -> Dict:
    try:
        return json.loads(state_path().read_text())
    except (OSError, ValueError):
        return {"pids": {}}


def save_state(s: Dict) -> None:
    state_path().write_text(json.dumps(s, indent=1))


# ------------------------------------------------------------------ hash pinning
def check_instance(i: int, dll_src: Optional[Path] = None, need_plugin: bool = False) -> List[str]:
    """Pinned-hash check before a job: exe/esm vs provisioning record, and plugin dll vs build output."""
    bad = instances.verify_pins(i)
    d = instances.instance_dir(i)
    dll = d / "Data" / "NVSE" / "Plugins" / "nvcoop.dll"
    if dll_src is not None and Path(dll_src).exists():
        if not dll.exists():
            bad.append("nvcoop.dll: not installed in instance")
        elif instances.sha256_file(dll) != instances.sha256_file(Path(dll_src)):
            bad.append("nvcoop.dll: differs from build output (re-provision)")
    elif need_plugin and not dll.exists():
        bad.append("nvcoop.dll: not installed in instance")
    return bad


# ------------------------------------------------------------------ logs
LOG_GLOBS = ["nvse*.log", "*.dmp", "*.mdmp", "RendererInfo.txt", "Data/NVSE/Plugins/*.log",
             "Data/NVSE/Plugins/*.dmp", "Data/NVSE/Plugins/nvcoop.ini", "Logs/**/*"]


def collect_logs(i: int, since: float, dest: Path) -> List[str]:
    """Copy logs/crash dumps modified after `since` from instance i into dest/inst<i>/."""
    d = instances.instance_dir(i)
    out = dest / f"inst{i}"
    got = []
    for pat in LOG_GLOBS:
        for f in d.glob(pat):
            if f.is_file() and (f.stat().st_mtime >= since or f.name == "nvcoop.ini"):
                rel = f.relative_to(d)
                tgt = out / rel
                tgt.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(f, tgt)
                got.append(str(rel))
    return got


def run_job(job_id: str, n: int, cmd: List[str], *, pool: int = 2, timeout_s: float = 900.0,
            lease_timeout_s: float = 3600.0, dll_src: Optional[Path] = None, need_plugin: bool = False,
            runs_dir: Optional[Path] = None, check_pins: bool = True, extra_env: Optional[Dict[str, str]] = None,
            use_game: bool = True, owner: str = OWNER, slot_wait_s: int = 3600) -> Dict:
    runs = Path(runs_dir or REPO / "runs") / job_id
    runs.mkdir(parents=True, exist_ok=True)
    result: Dict = {"job": job_id, "n": n, "cmd": cmd, "status": "error"}
    if use_game:
        try:
            slot_acquire(owner, ttl=int(timeout_s) + 300, wait=slot_wait_s)
        except SlotBusy as e:
            result.update(status="slot_busy", message=str(e))
            (runs / "result.json").write_text(json.dumps(result, indent=1))
            return result
        try:
            return run_job(job_id, n, cmd, pool=pool, timeout_s=timeout_s, lease_timeout_s=lease_timeout_s,
                           dll_src=dll_src, need_plugin=need_plugin, runs_dir=runs_dir, check_pins=check_pins,
                           extra_env=extra_env, use_game=False, owner=owner)
        finally:
            slot_release(owner)
    t_queue = time.time()
    with jobqueue.lease(n, pool=pool, job=job_id, timeout_s=lease_timeout_s) as inst:
        result.update(instances=inst, queued_s=round(time.time() - t_queue, 2))
        problems: List[str] = []
        if check_pins:
            for i in inst:
                problems += [f"inst{i}: {p}" for p in check_instance(i, dll_src, need_plugin)]
        if problems:
            result.update(status="pin_mismatch", problems=problems)
            (runs / "result.json").write_text(json.dumps(result, indent=1))
            return result
        env = dict(os.environ)
        env.update(extra_env or {})
        env["NVCOOP_INSTANCES"] = ",".join(map(str, inst))
        env["NVCOOP_JOB"] = job_id
        t0 = time.time()
        try:
            cp = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=timeout_s)
            result.update(status="pass" if cp.returncode == 0 else "fail", returncode=cp.returncode)
            (runs / "stdout.txt").write_text(cp.stdout)
            (runs / "stderr.txt").write_text(cp.stderr)
        except subprocess.TimeoutExpired as e:
            result.update(status="timeout")
            (runs / "stdout.txt").write_text((e.stdout or b"").decode(errors="replace") if isinstance(e.stdout, bytes) else (e.stdout or ""))
        result["duration_s"] = round(time.time() - t0, 2)
        result["logs"] = {str(i): collect_logs(i, t0, runs) for i in inst}
    (runs / "result.json").write_text(json.dumps(result, indent=1))
    return result


# ------------------------------------------------------------------ up / down / status
def _pipe_ready(i: int) -> bool:
    """True if the named pipe exists (os.path.exists is unreliable for pipes). Busy still counts as existing."""
    import ctypes
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    ctypes.set_last_error(0)
    if k32.WaitNamedPipeW(r"\\.\pipe\nvcoop-%d" % i, 1):
        return True
    return ctypes.get_last_error() != 2  # 2 == ERROR_FILE_NOT_FOUND


DETACH_FLAGS = 0x00000008 | 0x00000200 | 0x01000000  # DETACHED_PROCESS | NEW_PROCESS_GROUP | BREAKAWAY_FROM_JOB


def spawn_detached(cmd, cwd, env, startupinfo=None) -> subprocess.Popen:
    """Start a process that outlives us: not in our job object (breakaway), no console, own process group.
    Without this, a game started by a short-lived `run.py up` dies with it when the parent's job closes.
    If the job forbids breakaway (ACCESS_DENIED) fall back to the same flags without it."""
    kw = dict(cwd=cwd, env=env, startupinfo=startupinfo, close_fds=True,
              stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        return subprocess.Popen(cmd, creationflags=DETACH_FLAGS, **kw)
    except OSError:
        return subprocess.Popen(cmd, creationflags=DETACH_FLAGS & ~0x01000000, **kw)


def launch_instance(i: int, *, via_nvse: Optional[bool] = None) -> int:
    import winproc
    d = instances.instance_dir(i)
    loader = d / "nvse_loader.exe"
    exe = loader if (via_nvse if via_nvse is not None else loader.exists()) else d / "FalloutNV.exe"
    env = dict(os.environ)
    env.update(STEAM_ENV)
    env["NVCOOP_INSTANCE"] = str(i)
    si = subprocess.STARTUPINFO()  # start minimized, no activation (owner is using the PC)
    si.dwFlags |= subprocess.STARTF_USESHOWWINDOW
    visible = os.environ.get("NVCOOP_VISIBLE") == "1"  # watch the copies: shown without taking focus, never minimized
    si.wShowWindow = 4 if visible else 7  # SW_SHOWNOACTIVATE / SW_SHOWMINNOACTIVE
    p = spawn_detached([str(exe)], str(d), env, si)
    pid = p.pid
    if exe.name.lower() == "nvse_loader.exe":
        # loader spawns FalloutNV.exe and exits; find the child by image path
        for _ in range(100):
            for cand in winproc.list_pids("FalloutNV.exe"):
                if cand not in load_state()["pids"].values():
                    pid = cand
                    break
            else:
                time.sleep(0.2)
                continue
            break
    register_pid(pid)
    if not visible:
        threading.Thread(target=winproc.minimize_windows, args=(pid,), daemon=True).start()
    return pid


def start_diag_logger() -> None:
    """Start tools/diag/session_log.py detached (it exits itself when every copy is gone). NVCOOP_DIAG=0 turns it off."""
    if os.environ.get("NVCOOP_DIAG") == "0":
        return
    script = Path(__file__).resolve().parents[1] / "diag" / "session_log.py"
    if not script.exists():
        return
    env = dict(os.environ)
    try:
        spawn_detached([sys.executable, "-I", str(script), "--auto"], str(script.parents[2]), env)
    except OSError as e:
        print(f"session logger not started: {e}", file=sys.stderr)


def cmd_up(a) -> int:
    import winproc
    try:
        print(slot_acquire(OWNER, ttl=a.ttl, wait=a.slot_wait))
    except SlotBusy as e:
        print(f"game slot busy: {e}", file=sys.stderr)
        return 3
    start_diag_logger()
    st = load_state()
    running = winproc.list_pids("FalloutNV.exe")
    foreign = [p for p in running if p not in st["pids"].values()]
    if foreign and not a.allow_foreign:
        print(f"FalloutNV.exe already running (pids {foreign}) and not ours; refusing (use --allow-foreign)", file=sys.stderr)
        return 2
    ids = [a.only] if getattr(a, "only", None) is not None else list(range(a.n))   # --only I: start just that copy (its partner runs on another machine)
    for i in ids:
        if not (instances.instance_dir(i) / "instance.json").exists():
            print(f"instance {i} not provisioned; run provision first", file=sys.stderr)
            return 2
        if str(i) in st["pids"] and winproc.pid_alive(st["pids"][str(i)]):
            print(f"instance {i} already up (pid {st['pids'][str(i)]})")
            continue
        st["pids"][str(i)] = launch_instance(i)
        save_state(st)
        time.sleep(a.stagger)
        # strictly sequential: instance i must be up (plugin pipe) before i+1 starts
        if (instances.instance_dir(i) / 'Data/NVSE/Plugins/nvcoop.dll').exists():
            t0 = time.time()
            while not _pipe_ready(i) and time.time() - t0 < a.wait_pipe_each and winproc.pid_alive(st['pids'][str(i)]):
                time.sleep(1)
            if not _pipe_ready(i):
                print(f'instance {i}: pipe not ready (pid alive={winproc.pid_alive(st["pids"][str(i)])}, waited {time.time() - t0:.0f}s); aborting further launches', file=sys.stderr)
                return 3
    if a.wait_pipe:
        t0 = time.time()
        while time.time() - t0 < a.wait_pipe:
            if all(_pipe_ready(i) for i in ids):
                break
            time.sleep(1)
        print("pipes:", {i: _pipe_ready(i) for i in ids})
    return 0


def cmd_down(a) -> int:
    import winproc
    st = load_state()
    for i, pid in list(st["pids"].items()):
        if winproc.pid_alive(pid) and pid in winproc.list_pids("FalloutNV.exe"):  # never kill a reused pid
            winproc.kill_pid(pid)
            print(f"killed instance {i} (pid {pid})")
        st["pids"].pop(i)
    save_state(st)
    print(slot_release(OWNER))
    return 0


def cmd_status(a) -> int:
    import winproc
    st = load_state()
    root = instances.instances_root()
    rows = []
    for dd in sorted(p for p in root.glob("*") if p.name.isdigit()) if root.exists() else []:
        i = int(dd.name)
        pid = st["pids"].get(str(i))
        rows.append({"instance": i, "dir": str(dd), "provisioned": (dd / "instance.json").exists(),
                     "pid": pid, "alive": bool(pid and winproc.pid_alive(pid)),
                     "pipe": _pipe_ready(i), "lease": jobqueue.holder(i)})
    print(json.dumps(rows, indent=1))
    return 0


def provision_argv(a) -> List[str]:
    """The `instances.py provision` argv for the options of `run.py provision`. The --x=value form keeps a value
    that starts with '-' from being read as an option. --plugin-dll goes on whenever it was given, also as "",
    so that instances.py applies its default and "" means no plugin in both tools."""
    argv = ["provision"] + ([f"--pairs={a.pairs}", f"--port-base={a.port_base}", f"--first-pair={getattr(a, 'first_pair', 0)}"] if getattr(a, "pairs", 0)
                            else ["--n", str(a.n)]) + ([f"--plugin-dll={a.plugin_dll}"] if a.plugin_dll is not None else [])
    argv += ([f"--nvse-dir={a.nvse_dir}"] if a.nvse_dir else []) + ([f"--autoload={a.autoload}"] if a.autoload else [])
    argv += [f"--core-dll={a.core_dll}"] if a.core_dll else []
    argv += [f"--ini={spec}" for spec in a.ini]
    argv += ["--no-pad"] if getattr(a, "no_pad", False) else []
    return argv


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("provision")
    p.add_argument("--n", type=int, default=2)
    p.add_argument("--pairs", type=int, default=0, help="N two-copy pairs (2N instances), one co-op port per pair (see instances.py)")
    p.add_argument("--port-base", type=int, default=pairs.PORT_BASE, help="--pairs: co-op port of pair 0")
    p.add_argument("--first-pair", type=int, default=0, help="--pairs: provision only pairs K.. (running copies stay untouched)")
    # No default here: instances.py applies the default, so it can tell "not given" from "given".
    p.add_argument("--plugin-dll", default=None,
                   help="plugin dll; \"\" means no plugin (default: out/L1/nvcoop.dll if it exists, else no plugin)")
    p.add_argument("--nvse-dir")
    p.add_argument("--autoload", default="")
    p.add_argument("--core-dll", default=None,
                   help="split build: the reloadable core dll (nvcoop_mod.dll); needs an explicit --plugin-dll")
    p.add_argument("--ini", action="append", default=[], type=instances.ini_arg, metavar=instances.INI_FORM,
                   help="extra nvcoop.ini entry, repeatable, e.g. --ini coop.enable=1; with --n 2 or more, "
                        "the per-instance keys bridge.instance, coop.instanceId, coop.pipe and coop.role are an error")
    p.add_argument("--no-pad", action="store_true",
                   help="the copies ignore game controllers, so bot.move works with a pad present (see instances.py)")
    p = sub.add_parser("up")
    p.add_argument("--n", type=int, default=2)
    p.add_argument("--pairs", type=int, default=0, help="N two-copy pairs: --n 2N, and refuse when the free RAM is too small")
    p.add_argument("--no-ram-check", action="store_true", help="--pairs: launch even if the free RAM looks too small")
    p.add_argument("--only", type=int, default=None, help="start only this instance number (its partner runs on another machine)")
    p.add_argument("--wait-pipe", type=float, default=0)
    p.add_argument("--stagger", type=float, default=2.0)
    p.add_argument("--ttl", type=int, default=1800, help="game slot lease seconds (released by `down`)")
    p.add_argument("--slot-wait", type=int, default=3600)
    p.add_argument("--wait-pipe-each", type=float, default=240)
    p.add_argument("--allow-foreign", action="store_true")
    sub.add_parser("down")
    sub.add_parser("status")
    hotswap.add_arguments(sub.add_parser("swap", help="hot-swap the core DLL into running copies (no restart)"))
    p = sub.add_parser("job")
    p.add_argument("--id", required=True)
    p.add_argument("--n", type=int, default=2)
    p.add_argument("--pool", type=int, default=2)
    p.add_argument("--timeout", type=float, default=900)
    p.add_argument("--dll", default=str(DEFAULT_DLL))
    p.add_argument("--need-plugin", action="store_true")
    p.add_argument("--no-pin-check", action="store_true")
    p.add_argument("--no-game", action="store_true", help="job does not use the game: skip the game slot")
    p.add_argument("cmd_", nargs=argparse.REMAINDER)
    a = ap.parse_args(argv)
    if a.cmd == "provision":
        if a.pairs and a.n not in (2, 2 * a.pairs):  # 2 is the default of --n
            ap.error(f"--n {a.n} does not match --pairs {a.pairs} (that is {2 * a.pairs} instances); give only one")
        return instances.main(provision_argv(a))
    if a.cmd == "up":
        if a.pairs:
            a.n = 2 * a.pairs
            import winproc
            st = load_state()
            up = sum(1 for i in range(a.n) if str(i) in st["pids"] and winproc.pid_alive(st["pids"][str(i)]))
            why = None if a.no_ram_check else pairs.ram_problem(a.pairs, up, pairs.free_ram_bytes())
            if why:
                print(why, file=sys.stderr)
                return 2
        return cmd_up(a)
    if a.cmd == "down":
        return cmd_down(a)
    if a.cmd == "status":
        return cmd_status(a)
    if a.cmd == "swap":
        return hotswap.run_cli(a)
    cmd = [c for c in a.cmd_ if c != "--"]
    if not cmd:
        ap.error("job needs a command after --")
    r = run_job(a.id, a.n, cmd, pool=a.pool, timeout_s=a.timeout, dll_src=Path(a.dll), need_plugin=a.need_plugin,
                check_pins=not a.no_pin_check, use_game=not a.no_game)
    print(json.dumps(r, indent=1))
    return 0 if r["status"] == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
