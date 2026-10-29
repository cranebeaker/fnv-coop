"""In-game proof that a GUEST's blow on a host-owned NPC lands for both players (plugin/combat, docs/sync-model.md section 2 "Combat"). NEVER run by the code
agents: the coordinator runs it on the test PC against two live game instances (host = instance 0, guest = instance 1) in one co-op session with `[live] actors=1
avatars=1 combat=1` and `[dev] bots=1`.

    python -I plugin/combat/proof/melee_proof.py run --ref FalloutNV.esm:104F0A [--host 0] [--guest 1] [--punches 8] [--out out/combat/melee.json]
    python -I plugin/combat/proof/melee_proof.py --selftest          # game-free: the judge and the whole script against a fake session, with twins

Why it exists: 2026-10-29 the guest punched a non-hostile Goodsprings settler the host owns. On the guest that settler is a copy (restrained, AI off, driven from
snapshots). Nothing happened and the guest's log had no combat line. The punch must reach the owner as a hit request: the owner applies it natively and the settler turns
on the guest's puppet.

What it does (every step is a bridge call or a bot input; nothing needs a human):
  1. wiring: combat.status on both (running, six gate sites, pins ok), coop.owner on both (same owner and epoch, the host), GetRestrained of the NPC on both (reported; the
     guest's copy is the restrained one), the guest has no weapon equipped (its fists are what it hits with). NOT READY (exit 3) when any of it is wrong.
  2. place: the guest is teleported next to the NPC; before every punch it faces the NPC (a non-hostile NPC wanders); the punch is the real attack input (bot.fire).
  3. judge: combat.status before/after on both machines plus the NPC's health on both copies; one PASS/FAIL line per rule. On a FAIL the guest's last gate decision and
     the host's last rejections are printed: they say whether the punch never reached the hit funnel (gate.byPlayerOnCopy did not rise: the engine did not produce a
     hit on the copy, so the fix is on the engine side), was neutralised at the gate (neutralized rose), was lost on the wire (hitsSent rose, hitsReceived did not) or was
     refused by the owner (hitsRejected, protectedHitsBlocked: a protected story NPC takes three deliberate hits).

Exit code: 0 pass, 1 a rule failed, 2 usage / cannot reach the game, 3 not ready (nothing judged).
"""
from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))
import combat_proof  # noqa: E402  (flatten: combat.status -> one flat dict)

SETTLE_S = 2.0           # after the last punch: the owner's vitals reach the guest's copy
PUNCH_GAP_S = 1.2        # between punches (a punch animation is about a second)
NEAR_UNITS = 70.0        # the guest stands this close (fist reach is about 100)
HEALTH_SLACK = 1         # the guest's copy follows the owner's integer health within this much

Rule = Tuple[str, bool, str]


def delta(b: Dict[str, Any], a: Dict[str, Any], key: str) -> int:
    return int(a.get(key, 0)) - int(b.get(key, 0))


def judge(host_b: Dict[str, Any], host_a: Dict[str, Any], guest_b: Dict[str, Any], guest_a: Dict[str, Any], health: Dict[str, Any], *, mutate: int = 0) -> List[Rule]:
    """host_*/guest_*: flat combat.status before/after. health: {host_before, host_after, guest_after} of the NPC (None = unreadable)."""
    out: List[Rule] = []

    def h(k: str) -> int:
        return delta(host_b, host_a, k)

    def g(k: str) -> int:
        return delta(guest_b, guest_a, k)

    out.append(("both machines run combat with the gate on all six call sites and the native pins matching",
                all(m.get("running") and m.get("gateSites") == 6 and m.get("pinsOk") for m in (host_a, guest_a)) or mutate == 1,
                str([(m.get("running"), m.get("gateSites"), m.get("pinsOk")) for m in (host_a, guest_a)])))
    out.append(("no gate error and no native fault on either machine", (g("gateErrors") == 0 and h("gateErrors") == 0 and not int(host_a.get("nativeFaults", 0)) and
                                                                    not int(guest_a.get("nativeFaults", 0))) or mutate == 2, f"gateErrors guest {g('gateErrors')} host {h('gateErrors')}"))
    out.append(("the guest's punches reached the hit funnel on the copy (the engine produced a hit on it)", g("gateByPlayerOnCopy") >= 1 or mutate == 3,
                f"gate.byPlayerOnCopy +{g('gateByPlayerOnCopy')} (gate.byPlayer +{g('gateByPlayer')}, engineHits +{g('engineHits')})"))
    out.append(("none of them was neutralised at the guest's gate", g("neutralized") == 0 or mutate == 4, f"neutralized +{g('neutralized')} clamped +{g('clamped')}"))
    out.append(("the guest forwarded them to the owner as hit requests", g("hitsSent") >= 1 and g("clamped") >= 1 or mutate == 5, f"hitsSent +{g('hitsSent')} touchesSent +{g('touchesSent')}"))
    out.append(("the owner received them", h("hitsReceived") >= 1 or mutate == 6, f"hitsReceived +{h('hitsReceived')} (host), nacks sent +{h('nacksSent')}"))
    out.append(("the owner accepted and applied them (health off, an exact light blow, or a touch for a blow with no damage)",
                (h("hitsApplied") + h("lightHitsApplied") + h("touchesApplied")) >= 1 and h("hitsRejected") == 0 or mutate == 7,
                f"hitsApplied +{h('hitsApplied')} lightHitsApplied +{h('lightHitsApplied')} touchesApplied +{h('touchesApplied')} hitsRejected +{h('hitsRejected')} "
                f"protectedHitsBlocked +{h('protectedHitsBlocked')}"))
    out.append(("the owner's engine took the blow (native replay or damage line)", (h("damageApplied") + h("touchesEngine")) >= 1 or mutate == 8,
                f"damageApplied +{h('damageApplied')} touchesEngine +{h('touchesEngine')} engineRefused +{h('engineRefused')}"))
    out.append(("the NPC turned on the guest's puppet", h("provoked") >= 1 or mutate == 9, f"provoked +{h('provoked')}"))
    sent, applied = g("hitsSent") + g("hitRetries"), h("hitsApplied") + h("lightHitsApplied") + h("touchesApplied")
    out.append(("no blow was applied twice", applied <= sent or mutate == 10, f"sent {sent} (with retries) applied {applied}"))
    hb, ha, ga = health.get("host_before"), health.get("host_after"), health.get("guest_after")
    took = h("damageApplied") >= 1
    # whole-point damage must show on the owner's integer health; light blows (a few tenths each) may not move it by themselves, a touch never does
    out.append(("the NPC lost health on the owner when the owner applied whole-point damage, and never gained any",
                (ha is not None and hb is not None and ha <= hb and (ha < hb or not took)) or mutate == 11, f"host health {hb} -> {ha}, damageApplied +{h('damageApplied')}"))
    out.append(("the guest's copy shows the owner's health", (ha is not None and ga is not None and abs(ha - ga) <= HEALTH_SLACK) or mutate == 12, f"host {ha} guest {ga}"))
    return out


# ---- the session: real (bridge + bot) and fake (selftest) ------------------------------------------------------------------------------------


class NotReady(Exception):
    pass


class RealSession:
    def __init__(self, host: Any, guest: Any, ref: str):
        sys.path.insert(0, str(ROOT / "tools"))
        from bots import Bot  # noqa: E402
        self.nv = {"host": host, "guest": guest}
        self.ref = ref
        self.bot = Bot(guest, "melee-proof")

    def status(self, who: str) -> Dict[str, Any]:
        return combat_proof.flatten(self.nv[who].call("combat.status"))

    def owner(self, who: str) -> Dict[str, Any]:
        return self.nv[who].call("coop.owner", {"ref": self.ref})

    def restrained(self, who: str) -> Optional[int]:
        try:
            return int(self.nv[who].eval("GetRestrained", ref=self.ref))
        except Exception:  # noqa: BLE001 - not loaded on that copy
            return None

    def health(self, who: str) -> Optional[int]:
        try:
            return int(round(self.nv[who].eval("GetActorValue Health", ref=self.ref)))
        except Exception:  # noqa: BLE001
            return None

    def guest_unarmed(self) -> bool:
        try:
            return int(self.nv["guest"].eval("(player.GetEquippedObject 5) && 1")) == 0
        except Exception:  # noqa: BLE001
            return False

    def place_guest(self) -> None:
        self.bot.goto(self.ref, radius=NEAR_UNITS, mode="teleport")

    def punch(self) -> None:
        self.bot.look(ref=self.ref)
        self.bot.wait(0.3)
        self.nv["guest"].call("bot.fire", {"shots": 1})


def run_session(s: Any, punches: int, sleep: Callable[[float], None] = time.sleep) -> Tuple[List[Rule], Dict[str, Any]]:
    host_b, guest_b = s.status("host"), s.status("guest")
    why: List[str] = []
    for who, st in (("host", host_b), ("guest", guest_b)):
        if not st.get("running"):
            why.append(f"combat is not running on the {who}")
    oh, og = s.owner("host"), s.owner("guest")
    if not (oh.get("known") and og.get("known")):
        why.append(f"coop.owner does not know the NPC on both copies (host {oh}, guest {og})")
    elif (oh.get("owner"), oh.get("epoch")) != (og.get("owner"), og.get("epoch")) or oh.get("owner") != 0:
        why.append(f"the host does not own the NPC on both copies (host says {oh.get('owner')}/{oh.get('epoch')}, guest {og.get('owner')}/{og.get('epoch')})")
    if not s.guest_unarmed():
        why.append("the guest has a weapon equipped: unequip it, the proof punches with fists")
    if why:
        raise NotReady("; ".join(why))
    notes = {"restrained": {"host": s.restrained("host"), "guest": s.restrained("guest")}, "owner": oh}
    hb = s.health("host")
    s.place_guest()
    for _ in range(punches):
        s.punch()
        sleep(PUNCH_GAP_S)
    sleep(SETTLE_S)
    host_a, guest_a = s.status("host"), s.status("guest")
    health = {"host_before": hb, "host_after": s.health("host"), "guest_after": s.health("guest")}
    rules = judge(host_b, host_a, guest_b, guest_a, health)
    notes.update(health=health, guestLastGate=guest_a.get("lastGate"), hostRejections=host_a.get("rejections"), punches=punches)
    return rules, notes


# ---- self test (game-free) ---------------------------------------------------------------------------------------------------------------


class FakeSession:
    """A scripted two-copy world. `bug` picks a defect: 'neutral' (light blows neutralised at the guest's gate: the 2026-10-29 behaviour), 'nofunnel' (the engine
    never produces a hit on the copy), 'wire' (the owner never receives), 'noprovoke'."""

    def __init__(self, bug: str = "", touch: bool = False, armed: bool = False, owner: int = 0, light: bool = False):
        self.bug, self.touch, self.armed, self.own, self.light = bug, touch, armed, owner, light
        z = {"running": True, "gateSites": 6, "pinsOk": True}
        self.st = {"host": dict(z), "guest": dict(z)}
        self.hp = {"host": 100, "guest": 100}

    def status(self, who: str) -> Dict[str, Any]:
        return dict(self.st[who], lastGate="fake", rejections="")

    def owner(self, who: str) -> Dict[str, Any]:
        return {"known": True, "owner": self.own, "epoch": 3}

    def restrained(self, who: str) -> Optional[int]:
        return 1 if who == "guest" else 0

    def health(self, who: str) -> Optional[int]:
        return self.hp[who]

    def guest_unarmed(self) -> bool:
        return not self.armed

    def place_guest(self) -> None:
        pass

    def punch(self) -> None:
        g, h = self.st["guest"], self.st["host"]

        def bump(d: Dict[str, Any], **kw: int) -> None:
            for k, v in kw.items():
                d[k] = d.get(k, 0) + v
        if self.bug == "nofunnel":
            return
        bump(g, gateByPlayer=1, gateByPlayerOnCopy=1, engineHits=1)
        if self.bug == "neutral":
            bump(g, neutralized=1)
            return
        bump(g, clamped=1, hitsSent=1, touchesSent=1 if self.touch else 0)
        if self.bug == "wire":
            return
        bump(h, hitsReceived=1, hitsApplied=0 if (self.touch or self.light) else 1, touchesApplied=1 if self.touch else 0, lightHitsApplied=1 if self.light else 0,
             touchesEngine=1 if (self.touch or self.light) else 0, damageApplied=0 if (self.touch or self.light) else 1, provoked=0 if self.bug == "noprovoke" else 1)
        if not (self.touch or self.light):
            self.hp["host"] -= 1
            self.hp["guest"] -= 1


def selftest() -> int:
    failures = 0

    def expect(name: str, cond: bool) -> None:
        nonlocal failures
        if not cond:
            failures += 1
            print(f"  FAIL {name}")

    def go(**kw: Any) -> List[Rule]:
        rules, _notes = run_session(FakeSession(**kw), 3, sleep=lambda _s: None)
        return rules

    def passed(rules: List[Rule]) -> bool:
        return all(ok for _r, ok, _d in rules)

    expect("a healthy session with damage passes every rule", passed(go()))
    expect("a healthy session of touches (no health damage) passes every rule", passed(go(touch=True)))
    expect("a healthy session of light blows (exact fractions, the integer health may not move) passes every rule", passed(go(light=True)))
    for bug, what in (("neutral", "blows neutralised at the gate (2026-10-29)"), ("nofunnel", "no hit ever produced on the copy"), ("wire", "hits lost on the way to the owner"),
                      ("noprovoke", "the NPC does not turn on the puppet")):
        expect(f"{what} is caught", not passed(go(bug=bug)))
    try:
        run_session(FakeSession(armed=True), 3, sleep=lambda _s: None)
        expect("an armed guest is NOT READY", False)
    except NotReady:
        pass
    try:
        run_session(FakeSession(owner=1), 3, sleep=lambda _s: None)
        expect("a guest-owned NPC is NOT READY", False)
    except NotReady:
        pass
    # the judge's twins: each mutation switches ONE rule off (rule n is index n-1); the session that breaks exactly that rule must then slip through it
    for n, bug in ((3, "nofunnel"), (4, "neutral"), (6, "wire"), (9, "noprovoke")):
        s = FakeSession(bug=bug)
        hb, gb = s.status("host"), s.status("guest")
        for _ in range(3):
            s.punch()
        args = (hb, s.status("host"), gb, s.status("guest"), {"host_before": 100, "host_after": s.hp["host"], "guest_after": s.hp["guest"]})
        expect(f"twin {n}: the broken session fails rule {n}", not judge(*args)[n - 1][1])
        expect(f"twin {n}: the mutated judge lets rule {n} through", judge(*args, mutate=n)[n - 1][1])
    if failures:
        print(f"melee_proof selftest: {failures} failed")
        return 1
    print("melee_proof selftest: ALL OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    sub = ap.add_subparsers(dest="cmd")
    r = sub.add_parser("run")
    r.add_argument("--ref", required=True, help="Plugin.esm:HEX of a non-hostile NPC the host owns (a Goodsprings settler)")
    r.add_argument("--host", type=int, default=0)
    r.add_argument("--guest", type=int, default=1)
    r.add_argument("--punches", type=int, default=8)
    r.add_argument("--out", default="out/combat/melee.json")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    if args.cmd != "run":
        ap.print_help()
        return 2
    sys.path.insert(0, str(ROOT / "tools" / "runner"))
    from nvbridge import NvBridge  # noqa: E402
    try:
        with NvBridge.connect(instance=args.host, wait_s=10) as host, NvBridge.connect(instance=args.guest, wait_s=10) as guest:
            try:
                rules, notes = run_session(RealSession(host, guest, args.ref), args.punches)
            except NotReady as e:
                print(f"NOT READY: {e}")
                return 3
    except Exception as e:  # noqa: BLE001 - no game on the bridge, the game lost during the run
        print(f"cannot run: {type(e).__name__}: {e}")
        return 2
    for rule, ok, detail in rules:
        print(f"{'PASS' if ok else 'FAIL'}  {rule}   [{detail}]")
    ok_all = all(ok for _r, ok, _d in rules)
    if not ok_all:
        print(f"guest's last gate decision: {notes.get('guestLastGate')}")
        print(f"host's last rejections: {notes.get('hostRejections')}")
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps({"rules": [{"rule": a, "ok": b, "detail": c} for a, b, c in rules], "notes": notes}, indent=1), encoding="utf-8")
    print("PROOF OK" if ok_all else "PROOF FAILED")
    return 0 if ok_all else 1


if __name__ == "__main__":
    sys.exit(main())
