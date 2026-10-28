"""Windows process helpers for the runner (ctypes only). No cross-process handle manipulation:
the single-instance mutex is released by the nvcoop plugin itself (nvcoop.ini multiInstance=1)."""
from __future__ import annotations

import ctypes
import os
import subprocess
import time
from ctypes import wintypes
from typing import List

MUTANT = os.environ.get("RUNNER_MUTANT", "")     # test-only (tools/runner/tests/test_unthrottle.py)
k32 = ctypes.WinDLL("kernel32", use_last_error=True)
PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
k32.OpenProcess.restype = wintypes.HANDLE
k32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
k32.CloseHandle.argtypes = [wintypes.HANDLE]


def pid_alive(pid: int) -> bool:
    h = k32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, pid)
    if not h:
        return False
    try:
        code = wintypes.DWORD()
        k32.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
        k32.GetExitCodeProcess(h, ctypes.byref(code))
        return code.value == 259  # STILL_ACTIVE
    finally:
        k32.CloseHandle(h)


def list_pids(image_name: str) -> List[int]:
    out = subprocess.run(["tasklist", "/FI", f"IMAGENAME eq {image_name}", "/FO", "CSV", "/NH"],
                         capture_output=True, text=True).stdout
    pids = []
    for ln in out.splitlines():
        parts = [p.strip('"') for p in ln.split('","')]
        if len(parts) > 1 and parts[0].strip('"').lower() == image_name.lower():
            pids.append(int(parts[1]))
    return pids


def kill_pid(pid: int) -> None:
    subprocess.run(["taskkill", "/PID", str(pid), "/F", "/T"], capture_output=True)


# ---- Windows 11 power throttling of the test copies ----
# A copy whose window is minimized is "not visible" to Windows. Since Windows 11 such a process (1) gets its timer resolution requests ignored (Sleep(1) lasts a whole
# 15.6 ms tick, whatever timeBeginPeriod asked) and (2) may be run in EcoQoS (execution speed throttling). The test copies are minimized on purpose (the owner's desktop),
# so both can hit them. SetProcessInformation(ProcessPowerThrottling) with the control bit set and the state bit CLEAR opts the process out of that throttling.
PROCESS_SET_INFORMATION = 0x0200
ProcessPowerThrottling = 4
THROTTLE_VERSION = 1
THROTTLE_EXECUTION_SPEED = 0x1
THROTTLE_IGNORE_TIMER_RESOLUTION = 0x4


class PowerThrottlingState(ctypes.Structure):
    _fields_ = [("Version", wintypes.ULONG), ("ControlMask", wintypes.ULONG), ("StateMask", wintypes.ULONG)]


def _set_throttle(pid: int, control: int, state: int):
    """SetProcessInformation(ProcessPowerThrottling) on pid -> (ok, windows error code)."""
    h = k32.OpenProcess(PROCESS_SET_INFORMATION | PROCESS_QUERY_LIMITED_INFORMATION, False, pid)
    if not h:
        return False, ctypes.get_last_error()
    try:
        st = PowerThrottlingState(THROTTLE_VERSION, control, state)
        k32.SetProcessInformation.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD]
        k32.SetProcessInformation.restype = wintypes.BOOL
        ok = bool(k32.SetProcessInformation(h, ProcessPowerThrottling, ctypes.byref(st), ctypes.sizeof(st)))
        return ok, 0 if ok else ctypes.get_last_error()
    finally:
        k32.CloseHandle(h)


def unthrottle(pid: int) -> dict:
    """Opt the process out of Windows power throttling (execution speed and, on Windows 11, the ignoring of timer resolution requests while minimized).
    -> {"pid", "ok", "mask"} where mask is the control mask that was accepted (a Windows 10 build refuses the timer bit: then only the execution speed bit is set)."""
    for control in (THROTTLE_EXECUTION_SPEED | THROTTLE_IGNORE_TIMER_RESOLUTION, THROTTLE_EXECUTION_SPEED):
        ok, err = _set_throttle(pid, control, control if MUTANT == "throttle_state_set" else 0)     # twin: the state bits are SET (opts the process IN to the throttling)
        if ok:
            return {"pid": pid, "ok": True, "mask": control}
    return {"pid": pid, "ok": False, "mask": 0, "error": err}


def throttle_state(pid: int) -> dict:
    """What the process currently has: {"control": mask, "state": mask} (state bit set = throttled / ignored), or {} when it cannot be read."""
    h = k32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, pid)
    if not h:
        return {}
    try:
        st = PowerThrottlingState(THROTTLE_VERSION, 0, 0)
        k32.GetProcessInformation.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD]
        k32.GetProcessInformation.restype = wintypes.BOOL
        if not k32.GetProcessInformation(h, ProcessPowerThrottling, ctypes.byref(st), ctypes.sizeof(st)):
            return {}
        return {"control": int(st.ControlMask), "state": int(st.StateMask)}
    finally:
        k32.CloseHandle(h)


def minimize_windows(pid: int, timeout_s: float = 60.0, settle_s: float = 3.0) -> int:
    """Minimize every top-level window of pid WITHOUT activating it (SW_SHOWMINNOACTIVE=7 via
    ShowWindowAsync). Never touches the foreground or sends input. Keeps re-minimizing for settle_s
    after the last new window appears (the game recreates its window after the splash)."""
    u32 = ctypes.WinDLL("user32", use_last_error=True)
    seen = set()
    EnumProc = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)
    u32.IsIconic.argtypes = [wintypes.HWND]
    u32.IsWindowVisible.argtypes = [wintypes.HWND]
    u32.ShowWindowAsync.argtypes = [wintypes.HWND, ctypes.c_int]

    def cb(hwnd, _):
        p = wintypes.DWORD()
        u32.GetWindowThreadProcessId(hwnd, ctypes.byref(p))
        if p.value == pid and u32.IsWindowVisible(hwnd) and not u32.IsIconic(hwnd):
            u32.ShowWindowAsync(hwnd, 7)
            seen.add(hwnd)
        return True
    proc = EnumProc(cb)
    t0 = last = time.time()
    while time.time() - t0 < timeout_s and pid_alive(pid):
        n = len(seen)
        u32.EnumWindows(proc, 0)
        if len(seen) != n:
            last = time.time()
        elif seen and time.time() - last > settle_s:
            break
        time.sleep(0.1)
    return len(seen)


def proc_stats(pid: int) -> dict:
    """Working set (MB) and cumulative CPU seconds of pid (ctypes, no deps)."""
    class PMC(ctypes.Structure):
        _fields_ = [("cb", wintypes.DWORD), ("PageFaultCount", wintypes.DWORD), ("PeakWorkingSetSize", ctypes.c_size_t),
                    ("WorkingSetSize", ctypes.c_size_t), ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
                    ("QuotaPagedPoolUsage", ctypes.c_size_t), ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                    ("QuotaNonPagedPoolUsage", ctypes.c_size_t), ("PagefileUsage", ctypes.c_size_t),
                    ("PeakPagefileUsage", ctypes.c_size_t)]
    h = k32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, pid)
    if not h:
        return {}
    try:
        pm = PMC()
        pm.cb = ctypes.sizeof(pm)
        psapi = ctypes.WinDLL("psapi")
        psapi.GetProcessMemoryInfo.argtypes = [wintypes.HANDLE, ctypes.POINTER(PMC), wintypes.DWORD]
        psapi.GetProcessMemoryInfo(h, ctypes.byref(pm), pm.cb)
        ft = [wintypes.FILETIME() for _ in range(4)]
        k32.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
        k32.GetProcessTimes(h, *[ctypes.byref(f) for f in ft])
        t = lambda f: ((f.dwHighDateTime << 32) | f.dwLowDateTime) / 1e7  # noqa: E731
        return {"ws_mb": round(pm.WorkingSetSize / 2**20), "cpu_s": round(t(ft[2]) + t(ft[3]), 2)}
    finally:
        k32.CloseHandle(h)


def system_memory() -> dict:
    class MS(ctypes.Structure):
        _fields_ = [("dwLength", wintypes.DWORD), ("dwMemoryLoad", wintypes.DWORD), ("ullTotalPhys", ctypes.c_ulonglong),
                    ("ullAvailPhys", ctypes.c_ulonglong), ("ullTotalPageFile", ctypes.c_ulonglong),
                    ("ullAvailPageFile", ctypes.c_ulonglong), ("ullTotalVirtual", ctypes.c_ulonglong),
                    ("ullAvailVirtual", ctypes.c_ulonglong), ("ullAvailExtendedVirtual", ctypes.c_ulonglong)]
    m = MS()
    m.dwLength = ctypes.sizeof(m)
    k32.GlobalMemoryStatusEx(ctypes.byref(m))
    return {"total_gb": round(m.ullTotalPhys / 2**30, 1), "avail_gb": round(m.ullAvailPhys / 2**30, 1),
            "load_pct": m.dwMemoryLoad}
