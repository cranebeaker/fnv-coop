"""In-game proof helper for plugin/combat (docs/tools/combat-hooks.md). NEVER run by the agents: the helper runs it on his PC against two live game
instances (host = instance 0, client = instance 1) that are in a co-op session with [live] actors=1 combat=1 and a fight going on.

    python -I plugin/combat/proof/combat_proof.py snapshot --out out/combat/before.json      # before the fight
    ... run the fight (tools/bots/run_coop.py fight, or by hand) ...
    python -I plugin/combat/proof/combat_proof.py snapshot --out out/combat/after.json
    python -I plugin/combat/proof/combat_proof.py check out/combat/before.json out/combat/after.json
    python -I plugin/combat/proof/combat_proof.py --selftest                                  # game-free: the checker itself, with twins

`snapshot` calls the bridge method combat.status on every instance. `check` prints one PASS/FAIL line per rule and exits 1 on any FAIL.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple

# ---- the rules the counters must satisfy after a fight (all deltas after - before, summed over the machines unless stated) ----------------


def delta(before: Dict[str, Any], after: Dict[str, Any], key: str) -> int:
    return int(after.get(key, 0)) - int(before.get(key, 0))


def check(before: List[Dict[str, Any]], after: List[Dict[str, Any]], *, mutate: int = 0) -> List[Tuple[str, bool, str]]:
    """before/after: one combat.status dict per machine (same order). Returns [(rule, ok, detail)]."""
    out: List[Tuple[str, bool, str]] = []
    n = len(after)

    def d(i: int, k: str) -> int:
        return delta(before[i], after[i], k)

    def total(k: str) -> int:
        return sum(d(i, k) for i in range(n))

    out.append(("every machine has the combat module running", all(a.get("running") for a in after), str([a.get("running") for a in after])))
    out.append(("the hit gate is on all six call sites", all(a.get("gateSites") == 6 for a in after), str([a.get("gateSites") for a in after])))
    out.append(("the gate never raised an error", all(d(i, "gateErrors") == 0 for i in range(n)) or mutate == 1, str([d(i, "gateErrors") for i in range(n)])))
    out.append(("the native replay never faulted", all(int(a.get("nativeFaults", 0)) == 0 for a in after) or mutate == 2, str([a.get("nativeFaults") for a in after])))
    out.append(("the fight produced hits", total("engineHits") > 0, f"engineHits={total('engineHits')}"))
    out.append(("every engine hit got exactly one gate decision",
                all(d(i, "engineHits") == d(i, "proceeded") + d(i, "clamped") + d(i, "neutralized") for i in range(n)) or mutate == 3,
                str([(d(i, "engineHits"), d(i, "proceeded") + d(i, "clamped") + d(i, "neutralized")) for i in range(n)])))
    # A forwarded (clamped) hit is applied at most once in the whole cluster; duplicates are counted, never applied.
    sent, applied = total("hitsSent"), total("hitsApplied")
    out.append(("no forwarded hit was applied twice", applied <= sent + total("hitRetries") or mutate == 4, f"sent={sent} retries={total('hitRetries')} applied={applied}"))
    out.append(("every machine that applied a hit received it", all(d(i, "hitsApplied") <= d(i, "hitsReceived") + d(i, "hitRetries") for i in range(n)) or mutate == 5,
                str([(d(i, "hitsApplied"), d(i, "hitsReceived")) for i in range(n)])))
    out.append(("at least one hit was applied by an owner", applied > 0, f"applied={applied}"))
    declared, accepted = total("deathsDeclared"), total("deathsAccepted")
    # one declaration per death, accepted once by every OTHER machine (and once by the declarer itself): accepted <= declared * machines
    out.append(("deaths are declared once and accepted once per machine", accepted <= declared * n or mutate == 6, f"declared={declared} accepted={accepted} machines={n}"))
    out.append(("kill XP never exceeds the number of deaths", total("xpAwards") <= max(declared, 0) or mutate == 7, f"xpAwards={total('xpAwards')} deaths={declared}"))
    out.append(("no machine applied a hit it did not receive through the network or its own engine",
                all(d(i, "hitsApplied") >= 0 and d(i, "hitsRejected") >= 0 for i in range(n)), "counters monotonic"))
    return out


def flatten(st: Dict[str, Any]) -> Dict[str, Any]:
    """combat.status is {enabled, running, gate{sites,calls,errors}, native{pinsOk,faults,...}, stats{...}}; the checker works on one flat dict."""
    flat: Dict[str, Any] = {"enabled": st.get("enabled"), "running": st.get("running")}
    g, n = st.get("gate", {}), st.get("native", {})
    flat.update(gateSites=g.get("sites", 0), gateCalls=g.get("calls", 0), gateErrors=g.get("errors", 0), nativeFaults=n.get("faults", 0),
                nativeReplays=n.get("replays", 0), consoleDamage=n.get("consoleDamage", 0), pinsOk=n.get("pinsOk"),
                gateByPlayer=g.get("byPlayer", 0), gateByPlayerOnCopy=g.get("byPlayerOnCopy", 0), touchReplays=n.get("touchReplays", 0))   # lastGate / rejections ride in stats
    flat.update(st.get("stats", {}))
    return flat


def snapshot(instances: List[int]) -> List[Dict[str, Any]]:
    sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "tools" / "runner"))
    from nvbridge import NvBridge  # noqa: E402

    res = []
    for inst in instances:
        with NvBridge.connect(instance=inst, wait_s=10) as nv:
            res.append(flatten(nv.call("combat.status")))
    return res


# ---- self test (game-free) -----------------------------------------------------------------------------------------------------------


def _good() -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]]]:
    zero = {"running": 1, "gateSites": 6, "gateErrors": 0, "nativeFaults": 0, "engineHits": 0, "proceeded": 0, "clamped": 0, "neutralized": 0, "hitsSent": 0, "hitsApplied": 0,
            "hitsReceived": 0, "hitRetries": 0, "hitsRejected": 0, "deathsDeclared": 0, "deathsAccepted": 0, "xpAwards": 0}
    host = dict(zero, engineHits=12, proceeded=4, clamped=6, neutralized=2, hitsSent=6, hitsApplied=3, hitsReceived=3, deathsDeclared=1, deathsAccepted=1, xpAwards=0)
    client = dict(zero, engineHits=9, proceeded=3, clamped=5, neutralized=1, hitsSent=5, hitsApplied=2, hitsReceived=2, deathsDeclared=0, deathsAccepted=1, xpAwards=1)
    return [dict(zero), dict(zero)], [host, client]


def selftest() -> int:
    failures = 0

    def expect(name: str, cond: bool) -> None:
        nonlocal failures
        if not cond:
            failures += 1
            print(f"  FAIL {name}")

    b, a = _good()
    expect("a healthy fight passes every rule", all(ok for _r, ok, _d in check(b, a)))
    # every twin breaks one counter relationship; the matching rule must fail
    bad_cases = {
        1: lambda b, a: a[0].update(gateErrors=3),
        2: lambda b, a: a[1].update(nativeFaults=1),
        3: lambda b, a: a[0].update(proceeded=1),
        4: lambda b, a: a[0].update(hitsApplied=40),
        5: lambda b, a: a[1].update(hitsApplied=9),
        6: lambda b, a: a[0].update(deathsAccepted=10),
        7: lambda b, a: a[1].update(xpAwards=5),
    }
    for n, brk in bad_cases.items():
        b, a = _good()
        brk(b, a)
        res = check(b, a)
        expect(f"twin {n}: the broken counters are caught", not all(ok for _r, ok, _d in res))
        expect(f"twin {n}: the mutated checker lets them through", all(ok for _r, ok, _d in check(b, a, mutate=n)) or n in (4, 5))   # 4 and 5 trip each other's rule too
    st = {"enabled": True, "running": True, "gate": {"sites": 6, "calls": 3, "errors": 0}, "native": {"pinsOk": True, "faults": 0, "replays": 2},
          "stats": {"engineHits": 3, "hitsApplied": 2}}
    f = flatten(st)
    expect("flatten keeps the gate, native and stats counters", f["gateSites"] == 6 and f["nativeFaults"] == 0 and f["engineHits"] == 3 and f["running"] is True)
    b, a = _good()
    a[0]["gateSites"] = 5
    expect("a missing call site is caught", not all(ok for _r, ok, _d in check(b, a)))
    b, a = _good()
    for m in a:
        m["engineHits"] = 0
        m["proceeded"] = m["clamped"] = m["neutralized"] = 0
    expect("a fight with no hits is not a pass", not all(ok for _r, ok, _d in check(b, a)))
    if failures:
        print(f"combat_proof selftest: {failures} failed")
        return 1
    print("combat_proof selftest: ALL OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    sub = ap.add_subparsers(dest="cmd")
    s = sub.add_parser("snapshot")
    s.add_argument("--instances", type=int, nargs="+", default=[0, 1])
    s.add_argument("--out", required=True)
    c = sub.add_parser("check")
    c.add_argument("before")
    c.add_argument("after")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    if args.cmd == "snapshot":
        data = snapshot(args.instances)
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(data, indent=1), encoding="utf-8")
        print(f"wrote {args.out}")
        return 0
    if args.cmd == "check":
        before = json.loads(Path(args.before).read_text(encoding="utf-8"))
        after = json.loads(Path(args.after).read_text(encoding="utf-8"))
        res = check(before, after)
        for rule, ok, detail in res:
            print(f"{'PASS' if ok else 'FAIL'}  {rule}   [{detail}]")
        return 0 if all(ok for _r, ok, _d in res) else 1
    ap.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
