"""Crash catcher: which game copy died, with what exit code, at which address, and what the runner asked it last.

What the game side writes (read from the code, not guessed):
  * plugin/main.cpp CrashFilter, inside the dying process, one line in Data/NVSE/Plugins/nvcoop.log:
        CRASH code=C0000005 addr=0x... module=<path of the module> offset=<hex offset in that module> frames=<n>
  * [flight] crashDump (plugin/flight/flight_service.cpp): a flight recording dump of the last seconds (.nvfr) next to the logs
  * Windows error reporting minidumps (*.dmp / *.mdmp) when WER is set up for the game; read here for the exception address
The bridge itself logs no calls, so "last bridge call" comes from the ring file tools/proof/pf_trace.py keeps in the runner process.
"""
from __future__ import annotations

import ctypes
import json
import os
import re
import struct
import threading
from ctypes import wintypes
from pathlib import Path
from typing import Any, Dict, List, Optional

MUTANT = os.environ.get("PROOF_MUTANT", "")   # test-only: one deliberate bug per name (tools/proof/tests/run_tests.py)
CRASH_RE = re.compile(r"CRASH code=(?P<code>[0-9A-Fa-f]+) addr=(?P<addr>\S+) module=(?P<module>.*?) offset=(?P<offset>[0-9A-Fa-f]+)(?: frames=(?P<frames>\d+))?")


def parse_crash_line(text: str) -> Optional[Dict[str, Any]]:
    """The last CRASH line of an nvcoop.log as {code, eip, module, offset, frames}; module is the file name only."""
    hit = None
    for m in CRASH_RE.finditer(text):
        hit = m
    if not hit:
        return None
    mod = hit.group("module").replace("\\", "/").rsplit("/", 1)[-1]
    return {"code": hit.group("code").upper(), "eip": hit.group("addr"), "module": mod, "offset": hit.group("offset").upper(),
            "frames": int(hit.group("frames") or 0)}


def parse_minidump(data: bytes) -> Optional[Dict[str, Any]]:
    """Exception code and address from a Windows minidump's exception stream (type 6), or None when it has none."""
    if len(data) < 32 or data[:4] != b"MDMP":
        return None
    nstreams, rva = struct.unpack_from("<II", data, 8)
    for i in range(nstreams):
        off = rva + 12 * i
        if off + 12 > len(data):
            return None
        stype, size, srva = struct.unpack_from("<III", data, off)
        if stype == 6 and srva + 8 + 16 <= len(data):
            thread = struct.unpack_from("<I", data, srva)[0]
            code, _flags, _rec, addr = struct.unpack_from("<IIQQ", data, srva + 8)
            return {"code": f"{code:08X}", "eip": f"0x{addr:X}", "thread": thread}
    return None


WINDOWS = os.name == "nt"   # off Windows (the fake-world tests on Linux) a "handle" is the pid itself


def exit_code(handle: int) -> Optional[int]:
    if not WINDOWS:
        try:
            pid, status = os.waitpid(handle, os.WNOHANG)
        except ChildProcessError:   # not our child: alive or gone, with no exit code to read
            return None if os.path.exists("/proc/%d" % handle) else -1
        return None if pid == 0 else os.waitstatus_to_exitcode(status)
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
    c = wintypes.DWORD()
    if not k32.GetExitCodeProcess(handle, ctypes.byref(c)):
        return None
    return None if c.value == 259 else c.value


class Watcher:
    """Holds a process handle for each pid prove.py launched, so the exit code is still readable after the process is gone."""

    def __init__(self, open_handle=None, get_exit=None):
        if open_handle is None and not WINDOWS:
            open_handle = lambda pid: pid   # noqa: E731
        if open_handle is None:
            k32 = ctypes.WinDLL("kernel32", use_last_error=True)
            k32.OpenProcess.restype = wintypes.HANDLE
            k32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
            open_handle = lambda pid: k32.OpenProcess(0x1000, False, pid) or 0   # noqa: E731
        self._open = open_handle
        self._exit = get_exit or exit_code
        self.handles: Dict[int, int] = {}       # instance -> handle
        self.pids: Dict[int, int] = {}
        self.codes: Dict[int, int] = {}         # instance -> first exit code read: off Windows waitpid reaps, so it reads only once
        self._lock = threading.Lock()           # one thread per pair boots copies while the others poll

    def watch(self, instance: int, pid: int) -> None:
        h = self._open(pid)
        with self._lock:
            self.pids[instance] = pid
            self.handles[instance] = h
            self.codes.pop(instance, None)

    def unwatch(self, instance: int) -> None:
        with self._lock:
            self.handles.pop(instance, None)
            self.pids.pop(instance, None)
            self.codes.pop(instance, None)

    def exited(self, instances: Optional[List[int]] = None) -> List[Dict[str, Any]]:
        """[{instance, pid, exit_code}] for every watched copy that is gone now. With `instances` only those copies are looked at: a
        test on pair 0 must not be failed (or stopped) because a copy of pair 1 died."""
        out = []
        with self._lock:
            items = list(self.handles.items())
        for inst, h in items:
            if not h:
                continue
            if instances is not None and MUTANT != "watcher_unscoped" and inst not in instances:
                continue
            c = self.codes.get(inst) if MUTANT != "watcher_forgets_exit" else None
            if c is None:
                c = self._exit(h)
            if c is not None:
                with self._lock:
                    if self.handles.get(inst) == h:   # not unwatched or watched again meanwhile
                        self.codes[inst] = c
                out.append({"instance": inst, "pid": self.pids[inst], "exit_code": f"{c:08X}" if c > 255 else c})
        return out


def last_calls(trace_file: Path, n: int = 3) -> List[str]:
    """The last n bridge calls the runner made, newest last, as short strings. [] when no trace was recorded."""
    try:
        rows = json.loads(Path(trace_file).read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return []
    out = []
    for r in rows[-n:]:
        out.append(f"{r.get('inst', '?')}:{r.get('method')}" + (f" ERR {r['error']}" if r.get("error") else ""))
    return out


def collect(instance_dir: Path, since: float, trace_file: Optional[Path] = None) -> Dict[str, Any]:
    """Evidence for one dead copy: the CRASH line, a minidump address, the newest flight dump, the last bridge calls."""
    info: Dict[str, Any] = {}
    log = instance_dir / "Data" / "NVSE" / "Plugins" / "nvcoop.log"
    try:
        c = parse_crash_line(log.read_text(encoding="utf-8", errors="replace"))
        if c:
            info.update(c)
    except OSError:
        pass
    if "eip" not in info:
        for pat in ("*.dmp", "*.mdmp"):
            for f in sorted(instance_dir.glob(pat), key=lambda p: p.stat().st_mtime, reverse=True):
                if f.stat().st_mtime >= since:
                    try:
                        d = parse_minidump(f.read_bytes())
                    except OSError:
                        d = None
                    if d:
                        info.update(d)
                        info["dump"] = f.name
                        break
    if "eip" not in info:
        info["note"] = "no CRASH line in nvcoop.log and no minidump: the process died without running the plugin crash filter"
    if trace_file:
        info["last_calls"] = last_calls(trace_file)
    return info


def one_line(info: Dict[str, Any]) -> str:
    """Short text for the verdict reason."""
    if info.get("eip"):
        where = f"{info.get('module', '?')}+{info['offset']}" if info.get("offset") else info["eip"]
        return f"copy {info.get('instance', '?')} crashed code {info.get('code', '?')} at {where}"
    return f"copy {info.get('instance', '?')} exited (exit code {info.get('exit_code', '?')}), no crash record"
