"""Owner-PC safety for prove.py (docs/PROOF-PLAN.md section 6). Everything that touches processes takes its OS access as an argument,
so the tests run without a game, without Steam and without killing anything.

  preflight(...)      -> None, or the ENV reason: a FalloutNV.exe runs from outside the test instance folders (the owner is playing),
                         Steam is not running, or the RAM guard fails
  ram_guard(...)      the RAM rule: every copy that is not running yet needs COPY_GB, and RESERVE_GB (a flag of prove.py) must stay free
  QueueLock           one queue runner at a time (lock file with the owner pid; a dead pid's lock is taken over)
  Owned               the pids prove.py itself launched, in out/proof/owned.json; kill_owned() refuses any other pid
"""
from __future__ import annotations

import ctypes
import json
import ntpath
import os
import re
import subprocess
import threading
import time
from ctypes import wintypes
from pathlib import Path
from typing import Callable, Dict, Iterable, List, Optional, Tuple

MUTANT = os.environ.get("PROOF_MUTANT", "")   # test-only: one deliberate bug per name (tools/proof/tests/run_tests.py)
GAME_EXE = "FalloutNV.exe"
# The game and prove.py's real runs are Windows only. Off Windows the process helpers below answer from /proc instead of kernel32 and
# tasklist, so the fake-world tests (stub runners, fake copies) run on Linux too. The Windows branches are unchanged.
WINDOWS = os.name == "nt"
COPY_GB = 0.7           # resident memory of one game copy (tools/runner/pairs.py COPY_GB)
RESERVE_GB = 1.5        # left for Windows and the test tools; prove.py --reserve-gb
GIB = 1 << 30
TEST_ENV = {"NVCOOP_TEST_HELPERS": "1"}   # every copy prove.py boots gets this (test.quest.reset and the other test helpers are gated by it)
STEAM_WINDOW_ERROR = "steam error"          # title of the dialog a signed-out Steam puts up ("Application load error 5:0000065434")
SIGNIN_SLACK_S = 30.0                       # clock / granularity slack between the Steam process start and the login stamp


class NotOwned(Exception):
    """Raised when something tries to stop a process that prove.py did not start."""


# ------------------------------------------------------------------ OS access (replaced in tests)
def _proc_pids() -> List[int]:
    return [int(d) for d in os.listdir("/proc") if d.isdigit()] if os.path.isdir("/proc") else []


def _proc_stat(pid: int) -> Optional[List[str]]:
    """The fields of /proc/<pid>/stat after the command name (state first), or None."""
    try:
        with open("/proc/%d/stat" % pid, encoding="utf-8", errors="replace") as f:
            return f.read().rsplit(")", 1)[1].split()
    except (OSError, IndexError):
        return None


def list_pids(image: str) -> List[int]:
    if not WINDOWS:
        pids = []
        for pid in _proc_pids():
            try:
                with open("/proc/%d/cmdline" % pid, "rb") as f:
                    argv0 = f.read().split(b"\0", 1)[0].decode("utf-8", "replace")
            except OSError:
                continue
            if argv0 and os.path.basename(argv0.replace("\\", "/")).lower() == image.lower():
                pids.append(pid)
        return pids
    out = subprocess.run(["tasklist", "/FI", f"IMAGENAME eq {image}", "/FO", "CSV", "/NH"], capture_output=True, text=True).stdout
    pids = []
    for ln in out.splitlines():
        parts = [p.strip('"') for p in ln.split('","')]
        if len(parts) > 1 and parts[0].strip('"').lower() == image.lower():
            pids.append(int(parts[1]))
    return pids


def image_path(pid: int) -> Optional[str]:
    """Full path of a process exe, or None when Windows will not tell (treated as foreign by the guard)."""
    if not WINDOWS:
        try:
            return os.readlink("/proc/%d/exe" % pid)
        except OSError:
            return None
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.OpenProcess.restype = wintypes.HANDLE
    k32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    h = k32.OpenProcess(0x1000, False, pid)
    if not h:
        return None
    try:
        buf = ctypes.create_unicode_buffer(1024)
        n = wintypes.DWORD(1024)
        k32.QueryFullProcessImageNameW.argtypes = [wintypes.HANDLE, wintypes.DWORD, wintypes.LPWSTR, ctypes.POINTER(wintypes.DWORD)]
        return buf.value if k32.QueryFullProcessImageNameW(h, 0, buf, ctypes.byref(n)) else None
    finally:
        k32.CloseHandle(h)


def pid_alive(pid: int) -> bool:
    if not WINDOWS:
        st = _proc_stat(pid)
        if MUTANT == "zombie_alive":
            return st is not None
        return st is not None and st[0] not in ("Z", "X")   # an exited child that was not reaped yet is a zombie, not alive
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.OpenProcess.restype = wintypes.HANDLE
    k32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    h = k32.OpenProcess(0x1000, False, pid)
    if not h:
        return False
    try:
        code = wintypes.DWORD()
        k32.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
        k32.GetExitCodeProcess(h, ctypes.byref(code))
        return code.value == 259
    finally:
        k32.CloseHandle(h)


def taskkill(pid: int) -> None:
    if not WINDOWS:
        try:
            os.kill(pid, 9)
        except OSError:
            pass
        return
    subprocess.run(["taskkill", "/PID", str(pid), "/F", "/T"], capture_output=True)


# ------------------------------------------------------------------ the owner-PC guard
def instance_roots(repo: Path, env: Optional[Dict[str, str]] = None) -> List[Path]:
    """Folders whose FalloutNV.exe belong to tests: ../fnv-instances* next to the repo, and $FNV_INSTANCES_ROOT."""
    env = os.environ if env is None else env
    roots = sorted(p for p in repo.parent.glob("fnv-instances*") if p.is_dir())
    if env.get("FNV_INSTANCES_ROOT"):
        roots.append(Path(env["FNV_INSTANCES_ROOT"]))
    return roots


def _norm(p: str) -> str:
    # A Windows path (C:\..., \\server\...) is compared with Windows rules on any OS; on Windows os.path already is ntpath.
    m = ntpath if re.match(r"^([A-Za-z]:[\\/]|\\\\)", p) else os.path
    return m.normcase(m.abspath(p)).rstrip("\\/") + m.sep


def foreign_game_procs(roots: Iterable[Path], pids: Optional[Callable[[str], List[int]]] = None,
                       path_of: Callable[[int], Optional[str]] = image_path) -> List[Tuple[int, str]]:
    """FalloutNV.exe processes that do not run from inside one of the roots. An unknown path counts as foreign."""
    if MUTANT == "foreign_exe_ignored":
        return []
    pids = pids or list_pids
    rs = [_norm(str(r)) for r in roots]
    bad = []
    for pid in pids(GAME_EXE):
        p = path_of(pid)
        if not p or not any(_norm(p).startswith(r) for r in rs):
            bad.append((pid, "unknown path" if not p else "outside the test instance folders"))
    return bad


def steam_running(pids: Optional[Callable[[str], List[int]]] = None) -> bool:
    return bool((pids or list_pids)("steam.exe"))


def boot_env(game_env: Optional[Dict[str, str]] = None) -> Dict[str, str]:
    """The environment of a copy prove.py boots: the test helpers switch always, plus the test's own manifest `game_env` on top."""
    out = {} if MUTANT == "helpers_env_missing" else dict(TEST_ENV)
    out.update({str(k): str(v) for k, v in (game_env or {}).items()})
    return out


def env_satisfied(recorded: Optional[Dict[str, str]], needed: Dict[str, str]) -> bool:
    """True when a copy that was started with `recorded` has every variable `needed` with the same value (so it may be reused)."""
    if MUTANT == "env_reuse_unchecked":
        return True
    rec = recorded or {}
    return all(rec.get(k) == v for k, v in needed.items())


# ------------------------------------------------------------------ is Steam signed in
def parse_loginusers(text: str) -> Optional[float]:
    """Login time (epoch seconds) of the current user in Steam's config/loginusers.vdf: the MostRecent user's Timestamp, or (newer Steam has
    no MostRecent key) the newest Timestamp of any user. None when there is none."""
    recent, newest = None, None
    for body in re.findall(r'"\d{5,}"\s*\{([^{}]*)\}', text):
        kv = dict((k.lower(), v) for k, v in re.findall(r'"([^"]+)"\s+"([^"]*)"', body))
        try:
            ts = float(kv.get("timestamp", ""))
        except ValueError:
            continue
        newest = max(newest or 0.0, ts)
        if kv.get("mostrecent") == "1":
            recent = max(recent or 0.0, ts)
    return recent if recent is not None else newest


def process_start(pid: int) -> Optional[float]:
    """Creation time of a process as epoch seconds, or None."""
    if not WINDOWS:
        st = _proc_stat(pid)
        try:
            with open("/proc/stat", encoding="utf-8") as f:
                btime = next(int(l.split()[1]) for l in f if l.startswith("btime "))
            return btime + int(st[19]) / os.sysconf("SC_CLK_TCK") if st else None   # field 22 of stat: start time in clock ticks
        except (OSError, StopIteration, ValueError, IndexError):
            return None
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.OpenProcess.restype = wintypes.HANDLE
    k32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    h = k32.OpenProcess(0x1000, False, pid)
    if not h:
        return None
    try:
        c, e, k, u = (wintypes.FILETIME() for _ in range(4))
        k32.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
        if not k32.GetProcessTimes(h, ctypes.byref(c), ctypes.byref(e), ctypes.byref(k), ctypes.byref(u)):
            return None
        ticks = (c.dwHighDateTime << 32) | c.dwLowDateTime          # 100 ns since 1601-01-01
        return ticks / 1e7 - 11644473600
    finally:
        k32.CloseHandle(h)


def real_steam_probe(pids: List[int]) -> Dict[str, object]:
    """What this PC says about Steam's login: the registry ActiveProcess (ActiveUser, pid), the MostRecent login time in loginusers.vdf and
    when the steam.exe process started. Any part that cannot be read is None / 0."""
    out: Dict[str, object] = {"active_user": 0, "active_pid": 0, "login_ts": None, "start": None}
    try:
        import winreg
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Valve\Steam\ActiveProcess") as k:
            for name, key in (("ActiveUser", "active_user"), ("pid", "active_pid")):
                try:
                    out[key] = int(winreg.QueryValueEx(k, name)[0])
                except OSError:
                    pass
    except (OSError, ImportError):
        pass
    try:
        import winreg
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Valve\Steam") as k:
            path = Path(str(winreg.QueryValueEx(k, "SteamPath")[0])) / "config" / "loginusers.vdf"
        out["login_ts"] = parse_loginusers(path.read_text(encoding="utf-8", errors="replace"))
    except (OSError, ImportError, ValueError):
        pass
    starts = [t for t in (process_start(p) for p in pids) if t]
    out["start"] = min(starts) if starts else None
    return out


def signed_in_problem(pids: List[int], probe: Dict[str, object]) -> Optional[str]:
    """None when Steam is signed in, else the reason. Signed in = the registry names a live steam.exe pid with a non zero ActiveUser (that value is
    stale or 0 right after a Steam restart, so it is not the only signal), or a login was written to loginusers.vdf after this steam.exe started."""
    if MUTANT == "steam_signin_ignored":
        return None
    if probe.get("active_user") and probe.get("active_pid") in pids:
        return None
    login, start = probe.get("login_ts"), probe.get("start")
    if isinstance(login, (int, float)) and isinstance(start, (int, float)) and login >= start - SIGNIN_SLACK_S:
        return None
    return ("Steam is not signed in (steam.exe runs but no login since it started: every copy would show 'Application load error "
            "5:0000065434' and its pipe would never come up); sign in to Steam and requeue")


def steam_problem(pids: Optional[Callable[[str], List[int]]] = None, probe: Optional[Callable[[List[int]], Dict[str, object]]] = None) -> Optional[str]:
    """None when Steam runs (and, with a probe, is signed in), else the ENV reason."""
    plist = (pids or list_pids)("steam.exe")
    if not plist:
        return "Steam is not running (copies launched without it ask Steam to start the original game)"
    return signed_in_problem(plist, probe(plist)) if probe is not None else None


def steam_error_window(titles: Iterable[str]) -> bool:
    return any(STEAM_WINDOW_ERROR in (t or "").strip().lower() for t in titles)


def window_titles(pid: int) -> List[str]:
    """Titles of the top level windows of a process (visible or not)."""
    if not WINDOWS:
        return []   # no Windows windows to look at
    user32 = ctypes.WinDLL("user32", use_last_error=True)
    out: List[str] = []
    proc = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)

    def cb(hwnd, _l):
        owner = wintypes.DWORD()
        user32.GetWindowThreadProcessId(hwnd, ctypes.byref(owner))
        if owner.value == pid:
            buf = ctypes.create_unicode_buffer(256)
            user32.GetWindowTextW(hwnd, buf, 256)
            if buf.value:
                out.append(buf.value)
        return True
    user32.EnumWindows(proc(cb), 0)
    return out


def wait_for_pipe(i: int, pid: int, *, up: Callable[[int], bool], alive: Callable[[int], bool], titles: Callable[[int], List[str]] = window_titles,
                  sleep: Callable[[float], None] = time.sleep, now: Callable[[], float] = time.time, limit: float = 300.0,
                  poll: float = 2.0) -> Optional[str]:
    """None when instance i's bridge pipe is up, else why not. A top window titled "Steam Error" (a signed-out Steam) fails at once
    with an ENV reason instead of waiting out the limit."""
    t0 = now()
    while now() - t0 < limit:
        if up(i):
            return None
        if MUTANT != "steam_error_ignored" and steam_error_window(titles(pid)):
            return (f"Steam is not signed in: instance {i} shows a 'Steam Error' window (Application load error), so its pipe will never come up; "
                    "sign in to Steam and requeue")
        if not alive(pid):
            return f"instance {i}: the game exited before its bridge pipe came up"
        sleep(poll)
    return f"instance {i}: bridge pipe nvcoop-{i} never came up ({limit:.0f} s)"


def ram_guard(need_copies: int, free_bytes: int, reserve_gb: float = RESERVE_GB, copy_gb: float = COPY_GB) -> Optional[str]:
    """None when `need_copies` more game copies fit: free RAM >= need_copies x copy_gb + reserve_gb (copies that already run are
    not counted, their memory is already out of free_bytes). 4 copies pass at exactly 4 x 0.7 + reserve. Else the reason."""
    if need_copies <= 0:
        return None
    if MUTANT == "ram_reserve_ignored":
        reserve_gb = 0.0
    free_gb = free_bytes / GIB
    need_gb = need_copies * copy_gb + reserve_gb
    if free_gb + 0.001 >= need_gb:      # 1 MB of slack: 4 x 0.7 is not exact in binary floating point
        return None
    return (f"{need_copies} more game copies need {need_copies * copy_gb:.1f} GB plus {reserve_gb:.1f} GB reserve, "
            f"but only {free_gb:.1f} GB is free (run fewer pairs, close other programs, or lower --reserve-gb)")


def slot_need_gb(new_copies: int, reserve_gb: float = RESERVE_GB, copy_gb: float = COPY_GB) -> float:
    """Free RAM (GB) the game slot must see before it hands out a new slot for `new_copies` copies not running yet: the same number as
    ram_guard (copies x 0.7 GB + the caller's reserve), 0 when nothing new will start."""
    if MUTANT == "slot_need_fixed":
        return 3.0
    return new_copies * copy_gb + reserve_gb if new_copies > 0 else 0.0


def preflight(repo: Path, copies: int, up_copies: int = 0, *, pids=None, path_of=image_path, free_ram: Optional[Callable[[], int]] = None,
              roots: Optional[Iterable[Path]] = None, ram_problem: Optional[Callable[..., Optional[str]]] = None,
              reserve_gb: float = RESERVE_GB, steam_probe: Optional[Callable[[List[int]], Dict[str, object]]] = None) -> Optional[str]:
    """None when it is safe to start tests, else the one-line ENV reason. `copies` is how many game copies must be running for the
    test(s) about to start (every pair that runs at the same time counts), `up_copies` how many of those already are.
    `ram_problem` (tests only) replaces the built in RAM guard. `steam_probe` answers "is Steam signed in" (real_steam_probe on this PC; with
    injected `pids` and no probe only "steam.exe runs" is checked)."""
    injected = pids is not None
    pids = pids or list_pids
    bad = foreign_game_procs(roots if roots is not None else instance_roots(repo), pids, path_of)
    if bad:
        return f"owner is playing: FalloutNV.exe pid {bad[0][0]} ({bad[0][1]}); the queue waits for the owner to quit"
    if MUTANT != "steam_not_checked":
        why = steam_problem(pids, steam_probe if steam_probe is not None else (None if injected else real_steam_probe))
        if why:
            return why
    if copies and free_ram is not None:
        if ram_problem is not None:
            why = ram_problem(max(1, (copies + 1) // 2), up_copies, free_ram())
        else:
            why = ram_guard(max(0, copies - up_copies), free_ram(), reserve_gb)
        if why:
            return "RAM guard: " + why[:160]
    return None


# ------------------------------------------------------------------ one queue at a time
class QueueLock:
    def __init__(self, path: Path, alive: Callable[[int], bool] = pid_alive):
        self.path, self.alive, self.held = Path(path), alive, False

    def holder(self) -> Optional[int]:
        try:
            pid = int(json.loads(self.path.read_text())["pid"])
        except (OSError, ValueError, KeyError, TypeError):
            return None
        return pid if self.alive(pid) else None

    def acquire(self) -> Optional[str]:
        """None when we hold the lock now, else the reason it is taken."""
        self.path.parent.mkdir(parents=True, exist_ok=True)
        for _ in range(3):
            try:
                fd = os.open(str(self.path), os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            except FileExistsError:
                h = self.holder()
                if h is not None and MUTANT != "lock_ignored":
                    return f"another queue runner is active (pid {h})"
                try:
                    self.path.unlink()          # stale: its owner is gone
                except OSError:
                    pass
                continue
            with os.fdopen(fd, "w") as f:
                json.dump({"pid": os.getpid(), "since": time.strftime("%Y-%m-%dT%H:%M:%S")}, f)
            self.held = True
            return None
        return "could not take the queue lock"

    def release(self) -> None:
        if self.held:
            try:
                self.path.unlink()
            except OSError:
                pass
            self.held = False


# ------------------------------------------------------------------ pids prove.py launched
class Owned:
    """{instance: {"pid", "booted_at"}} persisted in a json file. Only these may be stopped by prove.py."""

    def __init__(self, path: Path):
        self.path = Path(path)
        self._lock = threading.RLock()          # two pairs boot and stop copies from two threads

    def load(self) -> Dict[str, Dict[str, float]]:
        try:
            return json.loads(self.path.read_text())
        except (OSError, ValueError):
            return {}

    def save(self, d: Dict[str, Dict[str, float]]) -> None:
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.path.write_text(json.dumps(d))

    def add(self, instance: int, pid: int, when: Optional[float] = None, env: Optional[Dict[str, str]] = None) -> None:
        with self._lock:
            d = self.load()
            d[str(instance)] = {"pid": pid, "booted_at": when or time.time(), "env": dict(env or {})}
            self.save(d)

    def env_of(self, instance: int) -> Dict[str, str]:
        """The extra environment the copy was booted with ({} for an entry written before this was recorded: such a copy is not trusted)."""
        return dict(self.load().get(str(instance), {}).get("env") or {})

    def pids(self) -> List[int]:
        return [int(v["pid"]) for v in self.load().values()]

    def forget(self, instance: int) -> None:
        with self._lock:
            d = self.load()
            d.pop(str(instance), None)
            self.save(d)


def lease_pids(status: Dict[str, object], owner: str) -> List[int]:
    """The game pids registered on the live slot leases of `owner` (tools/gameslot.py status). The slot lease is where run.py launch_instance
    records every copy it starts for a lane, so these are the copies the owner launched, whatever happened to owned.json."""
    out: List[int] = []
    for s in (status.get("slots") or []):
        lease = s.get("lease") if isinstance(s, dict) else None
        if isinstance(lease, dict) and lease.get("owner") == owner and not s.get("free"):
            out.extend(int(p) for p in lease.get("pids", []))
    return sorted(set(out))


def adopt_running(owned: Owned, owner_pids: Iterable[int], dir_of: Callable[[int], Path], candidates: Iterable[int], *,
                  pids: Optional[Callable[[str], List[int]]] = None, path_of: Callable[[int], Optional[str]] = image_path,
                  started: Callable[[int], Optional[float]] = lambda pid: None) -> List[int]:
    """owned.json lost (a killed queue, a deleted file) while its copies still run: take back the FalloutNV.exe that
      * is registered on the owner's slot lease (the owner launched it), AND
      * runs from the instance folder <n> that dir_of(n) names (not the owner's own game), AND
      * is not in owned.json yet.
    Its start environment is unknown, so it is recorded with env {} (such a copy is restarted when the test needs the test helpers).
    Returns the instances adopted. Anything else - a game copy of another lane or of the owner - is left alone."""
    mine = set(int(p) for p in owner_pids)
    have = {int(v["pid"]) for v in owned.load().values()}
    folders = {_norm(str(dir_of(i))): i for i in candidates}
    got: List[int] = []
    for pid in (pids or list_pids)(GAME_EXE):
        if MUTANT != "adopt_foreign" and pid not in mine:
            continue
        if pid in have:
            continue
        p = path_of(pid)
        if not p:
            continue
        i = folders.get(_norm(os.path.dirname(p)))
        if i is None:
            continue
        old = owned.load().get(str(i))
        if old and int(old["pid"]) != pid and pid_alive(int(old["pid"])):
            continue            # two live copies claim the instance: keep the recorded one
        owned.add(i, pid, when=started(pid))
        got.append(i)
    return got


def kill_owned(pid: int, owned: Owned, *, pids: Optional[Callable[[str], List[int]]] = None, kill: Callable[[int], None] = taskkill) -> bool:
    """Stop pid only if prove.py launched it AND it is still a FalloutNV.exe (a reused pid is left alone). Anything else raises NotOwned."""
    if MUTANT != "kill_foreign":
        if int(pid) not in owned.pids():
            raise NotOwned(f"pid {pid} was not started by prove.py; refusing to stop it")
        if int(pid) not in (pids or list_pids)(GAME_EXE):
            return False
    kill(int(pid))
    return True
