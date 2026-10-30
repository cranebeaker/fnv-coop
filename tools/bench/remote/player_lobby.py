#!/usr/bin/env python3
"""Player-path lobby check on the second PC: drive the two terminal launchers that player_lobby_run.ps1 started (host = game copy A,
friend = copy B) through their local web API, exactly the calls the launcher page makes: settings, HOST, JOIN, READY, BEGIN. Then follow
both games through the start save, the party hold and the opening movie to Doc Mitchell's house over the game bridge (pipe 0 = host,
pipe 1 = friend), and write a report. No mouse or keyboard input is simulated.

    py -3 tools\\bench\\remote\\player_lobby.py --out C:\\fnv\\player\\lobby_report.json
"""
import argparse
import json
import sys
import time
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent.parent / "runner"))
from nvbridge import NvBridge  # noqa: E402

KEY = "lab"


def api(port, path, body=None, timeout=10):
    req = urllib.request.Request(f"http://127.0.0.1:{port}{path}", method="GET" if body is None else "POST",
                                 data=None if body is None else json.dumps(body).encode(), headers={"X-Key": KEY, "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return json.loads(r.read() or b"{}")


def wait(what, fn, seconds, every=0.5):
    end = time.monotonic() + seconds
    last = None
    while time.monotonic() < end:
        try:
            last = fn()
            if last:
                return last
        except Exception as e:  # noqa: BLE001  not up yet
            last = f"{type(e).__name__}: {e}"
        time.sleep(every)
    raise TimeoutError(f"{what}: gave up after {seconds} s (last: {last})")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--host-port", type=int, default=47101)
    ap.add_argument("--guest-port", type=int, default=47102)
    ap.add_argument("--game-a", default=r"C:\fnv\player\A")
    ap.add_argument("--game-b", default=r"C:\fnv\player\B")
    ap.add_argument("--out", default=r"C:\fnv\player\lobby_report.json")
    ap.add_argument("--minutes", type=float, default=8.0, help="time allowed from BEGIN to both players at Doc's")
    a = ap.parse_args()
    H, G = a.host_port, a.guest_port
    rep = {"steps": [], "ok": False}

    def step(name, **kw):
        rep["steps"].append(dict(name=name, t=round(time.time(), 1), **kw))
        print(name, json.dumps(kw)[:300], flush=True)
        Path(a.out).write_text(json.dumps(rep, indent=1), encoding="utf-8")

    try:
        for port in (H, G):
            wait(f"launcher on {port}", lambda p=port: api(p, "/api/state"), 30)
        rep["version"] = api(H, "/api/state")["version"]
        step("launchers up", version=rep["version"])
        api(H, "/api/settings", {"name": "Six", "game_dir": a.game_a, "second_copy": True, "connection": "ip"})
        api(G, "/api/settings", {"name": "Cass", "game_dir": a.game_b, "second_copy": True, "connection": "ip"})
        for port, who in ((H, "host"), (G, "friend")):
            g = api(port, "/api/state")["game"]
            step(f"{who} game check", line=g["line"], ok=g["ok"])
            if not g["ok"]:
                raise RuntimeError(f"{who}: {g['line']}")
        r = api(H, "/api/host", {})
        step("HOST", **r)
        if not r.get("ok"):
            raise RuntimeError(r)
        r = api(G, "/api/join", {"address": "127.0.0.1"})
        step("JOIN", **r)
        wait("friend in the lobby", lambda: (api(G, "/api/state")["lobby"] or {}).get("state") == "lobby", 20)
        r = api(G, "/api/ready", {"ready": True})
        step("READY", **r)
        wait("host can begin", lambda: (api(H, "/api/state")["lobby"] or {}).get("can_start"), 20)
        step("lobby", host=api(H, "/api/state")["lobby"], friend=api(G, "/api/state")["lobby"])
        r = api(H, "/api/start", {})
        step("BEGIN", **r)
        if not r.get("ok"):
            raise RuntimeError(r)
        wait("both launchers started their game", lambda: api(H, "/api/state")["launched"] and api(G, "/api/state")["launched"], 30)
        step("games launched", host_log=api(H, "/api/state")["log"][-4:], friend_log=api(G, "/api/state")["log"][-4:])
    except Exception as e:  # noqa: BLE001
        step("FAILED in the launcher part", error=str(e))
        return 1

    # Keep the launchers alive (they exit 20 s after the page stops asking) only until the games run; then follow the games.
    deadline = time.monotonic() + a.minutes * 60
    bridges = {}
    for inst, who in ((0, "host"), (1, "friend")):
        try:
            bridges[who] = NvBridge.connect(instance=inst, wait_s=180)
            step(f"{who} game answers", ping=bridges[who].ping())
        except Exception as e:  # noqa: BLE001
            step(f"{who} game does not answer", error=str(e))
    if len(bridges) < 2:
        return 1
    seen = {}
    while time.monotonic() < deadline:
        state = {}
        for who, nv in bridges.items():
            try:
                cell = nv.exec("GetInCell GSDocMitchellHouse").get("output", "")
                vcg = nv.stage("VCG00")
                state[who] = {"atDoc": "1" in cell.split(">>")[-1], "VCG00": vcg}
            except Exception as e:  # noqa: BLE001
                state[who] = {"error": str(e)[:200]}
        if state != seen:
            step("game state", **state)
            seen = state
        if all(s.get("atDoc") and s.get("VCG00", 0) >= 100 for s in state.values()):
            rep["ok"] = True
            break
        time.sleep(3)
    for who, nv in bridges.items():
        try:
            step(f"{who} screenshot", **nv.screenshot())
        except Exception as e:  # noqa: BLE001
            step(f"{who} screenshot failed", error=str(e)[:200])
    step("RESULT", ok=rep["ok"])
    return 0 if rep["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
