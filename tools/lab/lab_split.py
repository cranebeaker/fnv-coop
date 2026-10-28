"""The two-PC smoke of the lab: the HOST copy runs on the second PC, the CLIENT copy on this PC, joined over the real LAN.

    python -I tools/lab/lab.py run --split                   # spawn, walk and quest rooms, then the LAN numbers
    python -I tools/lab/lab.py run --split --rooms spawn     # fewer rooms

What it reuses (nothing here is new machinery): tools/bench/remote.py (RemoteHost: ssh, the scheduled task that starts the game in the logged-in desktop session,
tools/runner/pipe_relay.py on the second PC and the ssh tunnel on port 27971, NVCOOP_REMOTE_0 so every NvBridge.connect(0) goes there), prove.py's provisioning
and boot for the one local copy (instance 1), the lab plugin installer for both copies, and the lab rooms unchanged.

Rules it keeps (each one a BLOCKED row with the reason, never a retry that works around it):
* only ONE game copy runs on this PC, and only when this PC has MIN_FREE_GB or more free RAM (checked first, before anything is started);
* Steam must run and be signed in on BOTH PCs; the lab never starts Steam, a game it did not start, or a process of anyone else;
* the local copy is the lab's own instance folder (fnv-instances\\1); the stock install is never touched (a guard refuses any instance folder that is not under the instances root);
* no simulated input anywhere: the game window on the second PC is started by the scheduled task, then only the bridge is used.

The LAN numbers next to the sync timings: ICMP ping to the second PC (min, avg, max), and the bridge round trip of both copies (the time a `coop.status` call takes: the host's
goes over the ssh tunnel, the client's over the local pipe). The co-op transport has no round trip time field (`coop.status` transport counts datagrams and bytes only), so the
bridge round trip is what is reported, and the table says so.
"""
from __future__ import annotations

import os
import re
import statistics
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (ROOT / "tools" / "bench", HERE):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_live                      # noqa: E402
import lab_core as K                 # noqa: E402

MIN_FREE_GB = 3.0                    # this PC: the one local copy needs about 1.5 GB (3 GB while it loads) on top of what the owner's programs use
DEFAULT_ROOMS = "spawn,walk,quest"
HOST_INSTANCE, CLIENT_INSTANCE = 0, 1     # the bench's split convention (NVCOOP_REMOTE_0 is the host): the local copy is instance 1
REMOTE_REPO = "C:/fnv/fnv-coop"
REMOTE_INSTANCE_DIR = r"C:\fnv\fnv-instances\0"
STEAM_USER_KEY = r"HKCU\Software\Valve\Steam\ActiveProcess"
PING_COUNT = 10
BRIDGE_SAMPLES = 20


# ------------------------------------------------------------------ guards (pure: every fact comes in as an argument, so the tests need no PC)
def guard_reasons(free_gb: float, local_steam: bool, local_user: bool, remote_steam: bool, remote_user: bool, local_games: List[int], remote_ok: bool,
                  min_free_gb: float = MIN_FREE_GB) -> List[str]:
    """Why the two-PC smoke may not start, all of it at once (empty list: go). The order is the order the facts are listed to the owner."""
    why: List[str] = []
    if free_gb < min_free_gb and K.MUTANT != "split_ram_unchecked":      # twin: the RAM check is skipped
        why.append(f"this PC has {free_gb:.1f} GB of free RAM, under the {min_free_gb:g} GB the one local game copy needs: refusing (close Brave, Discord and the like, "
                   f"or run both copies on the second PC with lab.py run)")
    if not local_steam and K.MUTANT != "split_steam_unchecked":          # twin: Steam is not checked
        why.append("Steam is not running on this PC (the lab does not start it: start Steam, sign in, and run again)")
    elif not local_user:
        why.append("Steam is running on this PC but nobody is signed in (sign in; the lab does not do it)")
    if local_games:
        why.append(f"a Fallout copy is already running on this PC (pid {', '.join(map(str, local_games[:3]))}); only one may run here and the lab does not stop what it did not start")
    if not remote_ok:
        why.append("the second PC cannot be reached over ssh without a password (ssh dino)")
    else:
        if not remote_steam:
            why.append("Steam is not running on the second PC (the lab does not start it)")
        elif not remote_user:
            why.append("Steam is running on the second PC but nobody is signed in")
    return why


def instance_dir_ok(path: Path, root: Path) -> bool:
    """The local copy must be one of the lab's own instance folders; the stock install (anything outside the instances root) is refused."""
    try:
        p, r = Path(path).resolve(), Path(root).resolve()
        p.relative_to(r)
    except (ValueError, OSError):
        return False
    return "program files" not in str(p).lower() and "steamapps" not in str(p).lower()


def parse_ping(text: str) -> Optional[Dict[str, float]]:
    """min / avg / max milliseconds and the loss of a Windows `ping -n N` output (English summary, or the per-reply times when the summary is localized)."""
    m = re.search(r"Minimum = (\d+)ms, Maximum = (\d+)ms, Average = (\d+)ms", text)
    loss = re.search(r"\((\d+)% loss\)", text)
    if m:
        return {"min": float(m.group(1)), "avg": float(m.group(3)), "max": float(m.group(2)), "loss_pct": float(loss.group(1)) if loss else 0.0}
    times = [float(t) for t in re.findall(r"time[=<](\d+)ms", text)]
    if not times:
        return None
    return {"min": min(times), "avg": round(sum(times) / len(times), 1), "max": max(times), "loss_pct": float(loss.group(1)) if loss else 0.0}


def percentile(vals: List[float], q: float) -> float:
    s = sorted(vals)
    return s[min(len(s) - 1, int(q * len(s)))] if s else 0.0


def bridge_rtt(nv, samples: int = BRIDGE_SAMPLES, clock: Callable[[], float] = time.perf_counter) -> Dict[str, float]:
    """Milliseconds a `coop.status` call takes on this bridge (p50, p95, max). A bridge that does not answer raises."""
    ms: List[float] = []
    for _ in range(samples):
        t0 = clock()
        nv.call("coop.status", {}, timeout_s=10)
        ms.append((clock() - t0) * 1000.0)
    return {"p50": round(statistics.median(ms), 1), "p95": round(percentile(ms, 0.95), 1), "max": round(max(ms), 1), "n": len(ms)}


def transport_rtt(status: Dict[str, Any]) -> Optional[float]:
    """A round trip time the co-op transport reports about itself, if this build has one (coop.status transport.rtt* / ping*), else None."""
    tr = (status or {}).get("transport") or {}
    for k, v in tr.items():
        if isinstance(v, (int, float)) and not isinstance(v, bool) and re.search(r"rtt|ping|roundtrip", k, re.I):
            return float(v)
    return None


def lan_info(ip: str, host_nv, client_nv, run: Callable[..., Any] = subprocess.run) -> Dict[str, Any]:
    """The real-LAN numbers: ICMP ping to the second PC, the bridge round trip of both copies, the transport's own round trip when it has one."""
    out: Dict[str, Any] = {"ip": ip}
    try:
        cp = run(["ping", "-n", str(PING_COUNT), ip], capture_output=True, text=True, timeout=60)
        out["ping_ms"] = parse_ping(cp.stdout or "")
    except Exception as e:   # noqa: BLE001
        out["ping_error"] = str(e)[:120]
    for key, nv in (("bridge_host_ms", host_nv), ("bridge_client_ms", client_nv)):
        try:
            out[key] = bridge_rtt(nv)
        except Exception as e:   # noqa: BLE001
            out[key + "_error"] = str(e)[:120]
    try:
        out["transport_rtt_ms"] = transport_rtt(client_nv.call("coop.status", {}, timeout_s=10))
    except Exception:   # noqa: BLE001
        out["transport_rtt_ms"] = None
    out["source"] = "co-op transport" if out.get("transport_rtt_ms") is not None else "bridge round trip (the transport has no rtt field in this build)"
    return out


# ------------------------------------------------------------------ the machines
def local_steam_facts(run: Callable[..., Any] = subprocess.run) -> Tuple[bool, bool, List[int]]:
    """(steam.exe runs here, somebody is signed in, pids of FalloutNV.exe running here). Read-only: tasklist and one registry value."""
    def tasks(name: str) -> List[int]:
        cp = run(["tasklist", "/FI", f"IMAGENAME eq {name}", "/FO", "CSV", "/NH"], capture_output=True, text=True, timeout=30)
        return [int(m.group(1)) for m in re.finditer(r'"[^"]+","(\d+)"', cp.stdout or "")]
    steam = bool(tasks("steam.exe"))
    cp = run(["reg", "query", STEAM_USER_KEY, "/v", "ActiveUser"], capture_output=True, text=True, timeout=30)
    m = re.search(r"ActiveUser\s+REG_DWORD\s+0x([0-9a-fA-F]+)", cp.stdout or "")
    user = bool(m and int(m.group(1), 16) != 0)
    return steam, user, tasks("FalloutNV.exe")


def remote_steam_facts(remote) -> Tuple[bool, bool, bool]:
    """(second PC reachable, steam.exe runs there, somebody is signed in there), read over ssh."""
    if not remote.reachable():
        return False, False, False
    t = remote.sh('tasklist /FI "IMAGENAME eq steam.exe" /FO CSV /NH', 30)
    steam = "steam.exe" in (t.stdout or "").lower()
    r = remote.sh(f'reg query "{STEAM_USER_KEY}" /v ActiveUser', 30)
    m = re.search(r"ActiveUser\s+REG_DWORD\s+0x([0-9a-fA-F]+)", r.stdout or "")
    return True, steam, bool(m and int(m.group(1), 16) != 0)


class LabRemoteHost:
    """bench/remote.py's RemoteHost with the host script that also installs the lab plugin. Built on demand (the import needs nothing else)."""

    @staticmethod
    def make(ssh: str, ip: str, log: Callable[[str], None]):
        import remote as remote_mod            # tools/bench/remote.py (the folder tools/bench/remote/ holds the scripts)

        class Lab(remote_mod.RemoteHost):
            def push_lab(self, esp: Path, layout: Path) -> None:
                self.sh(r"if not exist C:\fnv\fnv-coop\out\lab mkdir C:\fnv\fnv-coop\out\lab", 30)
                for f in (esp, layout):
                    r = self._run(["scp", "-o", "BatchMode=yes", str(f), f"{self.ssh_name}:{remote_mod.REMOTE_ROOT}/out/lab/{f.name}"], 60)
                    if r.returncode != 0:
                        raise RuntimeError(f"could not copy {f.name} to {self.ssh_name}: {(r.stderr or '')[:120]}")
                r = self._run(["scp", "-o", "BatchMode=yes", str(ROOT / "tools/bench/remote/lab_host_up.ps1"), f"{self.ssh_name}:{remote_mod.REMOTE_ROOT}/tools/bench/remote/lab_host_up.ps1"], 60)
                if r.returncode != 0:
                    raise RuntimeError(f"could not copy lab_host_up.ps1 to {self.ssh_name}: {(r.stderr or '')[:120]}")

            def up(self, boot_wait_s: float = 300):
                # the same sequence as RemoteHost.up with lab_host_up.ps1 (installs NVCoopLab.esp into the host copy first)
                import time as _t
                if not self.reachable():
                    raise RuntimeError(f"cannot reach {self.ssh_name} over ssh without a password (key login)")
                self.push()
                self.down()
                self.ps("task.ps1", "-Script C:/fnv/fnv-coop/tools/bench/remote/lab_host_up.ps1 -Wait 5", timeout=60)
                end, text = _t.time() + boot_wait_s, ""
                while _t.time() < end:
                    text = (self.sh("type C:\\fnv\\up_out.txt", 20).stdout or "")
                    if "pipes:" in text:
                        break
                    _t.sleep(4)
                if "pipes: {0: True}" not in text:
                    raise RuntimeError(f"the host copy on {self.ssh_name} did not bring up its bridge pipe: {text[-160:]!r}")
                self.ps("task.ps1", "-Script C:/fnv/fnv-coop/tools/bench/remote/relay_up.ps1 -Wait 3", timeout=60)
                self.tunnel = self._popen(["ssh", "-N", "-o", "BatchMode=yes", "-o", "ExitOnForwardFailure=yes", "-o", "ServerAliveInterval=20",
                                           "-L", f"{self.port}:127.0.0.1:{self.port}", self.ssh_name], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                os.environ["NVCOOP_REMOTE_0"] = f"127.0.0.1:{self.port}"
                self.log(f"host copy up on {self.ssh_name} ({self.ip}); its bridge is forwarded to 127.0.0.1:{self.port}")
        return Lab(ssh=ssh, ip=ip, log=log)


class SplitEnv(lab_live.LiveEnv):
    """One pair: host = instance 0 on the second PC (bridge through the tunnel), client = instance 1 on this PC. Everything else is LiveEnv."""

    def __init__(self, layout, args, log: Callable[[str], None] = lambda s: None, lab_dir: Optional[Path] = None, remote=None,
                 facts: Optional[Callable[[], Dict[str, Any]]] = None):
        super().__init__(layout, args, log, lab_dir)
        self.remote = remote or LabRemoteHost.make(getattr(args, "remote", "dino"), getattr(args, "remote_ip", "192.168.0.10"), log)
        self.facts_fn = facts or self.facts
        self.lan: Dict[str, Any] = {}
        self.remote_started = False
        self._ini_backup: Optional[Tuple[Path, str]] = None
        self.min_free_gb = float(getattr(args, "min_free_gb", MIN_FREE_GB))

    # ---- what is true now
    def facts(self) -> Dict[str, Any]:
        steam, user, games = local_steam_facts()
        ok, rsteam, ruser = remote_steam_facts(self.remote)
        return {"free_gb": self.pv.pairs.free_ram_bytes() / (1 << 30), "local_steam": steam, "local_user": user, "local_games": games,
                "remote_ok": ok, "remote_steam": rsteam, "remote_user": ruser}

    def pairs_allowed(self, n: int):
        return 1, ("the two-PC smoke runs one pair" if n > 1 else "")

    def client_ini(self) -> Path:
        return self.pv.idir(CLIENT_INSTANCE) / "Data" / "NVSE" / "Plugins" / "nvcoop.ini"

    def prepare(self, pair: int):
        f = self.facts_fn()
        why = guard_reasons(f["free_gb"], f["local_steam"], f["local_user"], f["remote_steam"], f["remote_user"], f["local_games"], f["remote_ok"], self.min_free_gb)
        if why:
            return "ENV", "; ".join(why)
        if not instance_dir_ok(self.pv.idir(CLIENT_INSTANCE), self.pv.inst.instances_root()):
            return "ENV", f"refusing: {self.pv.idir(CLIENT_INSTANCE)} is not one of the lab's instance folders"
        if not self.install_script.is_file():
            return "BLOCKED", self.install(CLIENT_INSTANCE) or "install_lab.py missing"
        try:
            self.remote.push_lab(self.esp, self.lab_dir / "layout.json")
        except Exception as e:   # noqa: BLE001
            return "ENV", f"the second PC did not take the lab plugin: {str(e)[:160]}"
        test = {"id": "lab-split", "copies": 1, "pair": 0, "n": 1, "ini": lab_live.LAB_INI, "timeout_s": 1800, "snapshot": [], "commands": [],
                "requires_methods": ["coop.status", "coop.owner", "bot.status", "avatar.status"], "also_roadmap": []}
        ins = [CLIENT_INSTANCE]
        self.pv.hold(ins)
        self.held.update(ins)
        pv = self.pv                                    # (prove's own preflight counts copies in pairs; the guards above are this smoke's, for one copy)
        slot = pv._acquire_slot(1, 3600, 1)             # noqa: SLF001 (the game slot lease: the same one every lab/proof run takes)
        if slot:
            return "ENV", slot
        problem = pv.ini_problem(test, ins) or pv.build_problem(ins)
        if problem:
            pc = pv.provision_cmd(test, ins)
            self.log("provisioning the local client: python -I " + " ".join(pc))
            subprocess.run(lab_live.PY + [str(ROOT / pc[0])] + pc[1:], cwd=ROOT, capture_output=True, text=True)
            problem = pv.ini_problem(test, ins) or pv.build_problem(ins)
            if problem:
                return "BLOCKED", f"the local client copy is not provisioned: {problem}"
        for line in pv.seed_saves(ins):
            self.log("saves: " + line)
        try:
            self.remote_started = True
            self.remote.up()                              # host copy on the second PC, tunnel, NVCOOP_REMOTE_0
        except Exception as e:   # noqa: BLE001
            return "ENV", f"the host copy on the second PC did not come up: {str(e)[:200]}"
        ini = self.client_ini()
        self._ini_backup = (ini, ini.read_text(encoding="latin-1"))
        import phases                                      # tools/bench/phases.py: the bench's own ini pointer
        phases.point_client_at_relay(ini, True, host=self.remote.ip, port=phases.HOST_PORT)
        msg = pv._boot(CLIENT_INSTANCE, {})                # noqa: SLF001 (installs the lab plugin first, see LiveEnv.__init__)
        if msg:
            return "ENV", msg
        time.sleep(2)
        why = pv.wait_ready(ins)
        if why:
            return "ENV", why
        import nvbridge
        self.nvs[HOST_INSTANCE] = nvbridge.NvBridge.connect(HOST_INSTANCE, wait_s=30)       # NVCOOP_REMOTE_0: the tunnel
        self.nvs[CLIENT_INSTANCE] = nvbridge.NvBridge.connect(CLIENT_INSTANCE, wait_s=30)
        for i in (HOST_INSTANCE, CLIENT_INSTANCE):
            if self.layout.plugin.lower() not in [p.lower() for p in self.nvs[i].plugins()]:
                return "BLOCKED", f"copy {i} runs without {self.layout.plugin} (started before the plugin was installed: lab.py down, then run again)"
        return None

    def enter(self, pair: int):
        base = super().enter(pair)

        def go():
            base()
            try:
                self.lan = lan_info(self.remote.ip, self.nvs[HOST_INSTANCE], self.nvs[CLIENT_INSTANCE])
            except Exception as e:   # noqa: BLE001 - the numbers are a bonus, the rooms are the test
                self.lan = {"error": str(e)[:160]}
        return go

    def split_info(self) -> Dict[str, Any]:
        return dict(self.lan, host="second PC", client="this PC")

    def finish(self) -> None:
        super().finish()
        if self._ini_backup is not None:
            try:
                self._ini_backup[0].write_text(self._ini_backup[1], encoding="latin-1", newline="")
            except OSError:
                pass
            self._ini_backup = None
        if self.remote_started or K.MUTANT == "split_stops_unstarted":      # twin: stops the second PC's copies even when this run started nothing
            # only what this run started: a refused run leaves the second PC alone
            try:
                self.remote.stop()       # the host copy this run started on the second PC, its pipe relay and the tunnel
            except Exception:   # noqa: BLE001
                pass
