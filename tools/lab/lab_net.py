"""The network pass of the lab: which chaos profiles exist, and what each one is allowed to add to a room's clean limits.

    profiles()                -> {name: {latency_ms, jitter_ms, loss, ...}} from tools/chaos/profiles.json (the source of truth; {} when the file is missing)
    extra_for(p)              -> (extra_s, extra_units): what a link with these numbers may add on top of the clean limits
    net_params(clean)         -> PARAMS entries "room.key@profile" for every profile (merged into lab_rooms.PARAMS)

Why these allowances (every threshold of a network pass is the clean limit plus this, so the numbers are derived, not guessed per profile):

* A change crosses the link once per leg: the host's own change is one leg (host to client), a guest's change is two (its intent goes to the host, the
  host's result comes back). One leg takes latency_ms plus up to jitter_ms. Two legs are the worst case for every timed check (LEGS = 2).
* The session runs on ENet. A lost reliable packet is sent again after the retransmit timeout, which ENet sets to the round trip time plus four times its
  variance: about 2 * latency + 4 * jitter. How many resends a message may need before the lab calls it too slow (RESENDS): one as soon as the link loses
  anything at all, a second when it also loses in bursts (the Gilbert-Elliott state of profiles.json: bad_wifi, mobile_tether, hell), a third when the base
  loss is 5 percent or more (hell).
* SAFETY = 2 on the sum, because the lab polls on one thread and a poll round is itself a bridge call, and a packet burst that eats the resends of two
  messages in a row is not a bug.
* A link with no loss and under 5 ms of latency (lan) adds nothing; any other profile adds at least FLOOR_S = 0.25 s (a frame or two of the game's own work).
* The puppet's position error grows with how stale the newest update is: the walking speed (WALK_SPEED, units per second) times (one leg of delay + the gap a
  lost burst leaves: GAP_MS per update lost, one update expected per lost packet, 1 / burst_exit packets per burst). Times 1.5 for the cases where two bursts follow
  each other. Position steps (the teleport limit) and the catch-up radius grow with it; the NPC walks at a third of the player's speed.

The resulting numbers (seconds, units) for the shipped profiles are printed by `python -I tools/lab/lab_net.py` and listed in docs/tools/lab.md.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any, Dict, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PROFILES_PATH = ROOT / "tools" / "chaos" / "profiles.json"

LEGS = 2                # a guest's change: intent to the host, result back
SAFETY = 2.0
FLOOR_S = 0.25
WALK_SPEED = 220.0      # units per second a player moves while the walk room steers him (the fake world uses the same)
GAP_MS = 50.0           # one avatar update every 50 ms (20 Hz)
POS_MARGIN = 1.5
NPC_SPEED_RATIO = 1.0 / 3.0

# the keys a network pass loosens: seconds (time limits) and units (position limits, with the share of the unit allowance each takes)
TIME_KEYS = ("spawn.connect_s", "spawn.puppet_wait_s", "spawn.digest_wait_s", "walk.lag_s", "fight.latency_s", "fight.weak_s", "loot.settle_s", "door.settle_s",
             "npc.handoff_s", "vats.settle_s", "vats.weak_s", "quest.settle_s",
             "parity.settle_s", "parity.punch_window_s")
UNIT_KEYS = {"spawn.puppet_err": 1.0, "walk.err_p95": 1.0, "walk.err_max": 1.0, "walk.jump": 1.0, "walk.near": 0.5, "npc.err_max": NPC_SPEED_RATIO, "npc.jump": NPC_SPEED_RATIO}
# keys that are not limits but how long a step takes: shorter in a network pass, to keep the whole run (clean + 2 profiles, 3 pairs) near 15 minutes
NET_IDLE_S = 10.0       # spawn.idle_s: the clean pass already watched 20 s of idle digest; a network pass keeps the part that shows the session survives the profile


def profiles() -> Dict[str, Dict[str, Any]]:
    try:
        raw = json.loads(PROFILES_PATH.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return {}
    return {k: v for k, v in raw.items() if not k.startswith("_") and isinstance(v, dict)}


def resends(p: Dict[str, Any]) -> int:
    loss = float(p.get("loss", 0) or 0)
    burst = float(p.get("burst_enter", 0) or 0) > 0
    n = 0
    if loss > 0 or burst:
        n += 1
    if burst:
        n += 1
    if loss >= 0.05:
        n += 1
    return n


def extra_for(p: Dict[str, Any]) -> Tuple[float, float]:
    """(extra seconds, extra units) a link with profile numbers `p` may add to the clean limits."""
    d, j = float(p.get("latency_ms", 0) or 0), float(p.get("jitter_ms", 0) or 0)
    r = resends(p)
    lossy = r > 0
    secs = SAFETY * (LEGS * (d + j) + r * (2 * d + 4 * j)) / 1000.0
    if lossy or d >= 5.0:
        secs = max(secs, FLOOR_S)
    burst = float(p.get("burst_enter", 0) or 0) > 0
    pkts = (1.0 / max(float(p.get("burst_exit", 0.5) or 0.5), 0.01)) if burst else (1.0 if lossy else 0.0)
    gap = GAP_MS * pkts
    units = POS_MARGIN * WALK_SPEED * ((d + j) + gap) / 1000.0
    return round(secs, 2), round(units, 0)


def net_params(clean: Dict[str, float]) -> Dict[str, float]:
    """PARAMS entries for every profile of profiles.json: `room.key@profile` = the clean value plus the profile's allowance."""
    out: Dict[str, float] = {}
    for name, p in profiles().items():
        secs, units = extra_for(p)
        for k in TIME_KEYS:
            if k in clean:
                out[f"{k}@{name}"] = round(clean[k] + secs, 2)
        for k, share in UNIT_KEYS.items():
            if k in clean:
                out[f"{k}@{name}"] = round(clean[k] + units * share, 0)
        out[f"spawn.idle_s@{name}"] = min(clean.get("spawn.idle_s", NET_IDLE_S), NET_IDLE_S)
    return out


def through_relay(stats: Dict[str, Any]) -> Tuple[bool, str]:
    """(True, "") when the relay forwarded or dropped packets in BOTH directions (relay.get_stats()): the client's traffic really crosses it. A client that kept
    talking to the host's real port leaves both counters at 0 (the same check as the twin_loopback_relay_bypass of the synccore tests)."""
    seen = {d: int((stats.get(d) or {}).get("seen", 0) or 0) for d in ("c2s", "s2c")}
    if all(seen.values()):
        return True, ""
    return False, f"packets seen: client to host {seen['c2s']}, host to client {seen['s2c']}"


def describe() -> str:
    rows = ["profile          one-way ms   loss   resends  +seconds  +units"]
    for name, p in profiles().items():
        s, u = extra_for(p)
        rows.append(f"{name:16s} {p.get('latency_ms')}+-{p.get('jitter_ms')}".ljust(34) + f"{p.get('loss')}".ljust(7) + f"{resends(p)}".ljust(9) + f"{s:<10g}{u:g}")
    return "\n".join(rows)


if __name__ == "__main__":
    print(describe())
    sys.exit(0)
