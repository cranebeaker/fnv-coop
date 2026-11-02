#!/usr/bin/env python3
"""Review the rows still marked `default` and promote the ones the old design clearly covers.

    python -I tools/policy-import/review_defaults.py [--policy crates/coop-policy/data/policy.json] \
        --old-rules <rules.json> [--pending <rules.pending.json>] [--check]

Rules, applied in this order to every row with status `default` (first match wins):

  R4  synthetic row (plugin `synthetic`): stays default, advance host (gate-safe).
  H1  the old table dropped a tether or warp for the row and it is still missing (--pending):
      stays default; T3/T4 `any` becomes initiator (gate-safe).
  R1  `Systems...` line at tier T0..T2: background quest run by the host. advance host, confirmed.
  R2  advance `any` in a `Main:` or `DLC:` line at T3/T4: a story stage is advanced by whoever talked.
      advance initiator, confirmed.
  R3  tier T4 (any line): advance any -> initiator, confirmed. A barrier is added only when the old
      table carried a Barrier rule, which no default row does, so none is added.
  R5  `Main:`, `Side:` or `DLC:` line at T3 with advance initiator, where the old table has an
      InitiatorDialogue rule for the quest (--old-rules): confirmed as is. Needing the old rule keeps the
      run idempotent: rows moved to initiator by R6 never had one and so are not promoted on a rerun.
  R6  every other T3/T4 row with advance `any`: advance initiator, stays default (gate-safe).

Rows that no rule promotes are listed with the reason. The run is deterministic and idempotent.
"""
import argparse
import json
import sys
from collections import Counter, OrderedDict


def write_policy(path, head, rows):
    with open(path, "w", encoding="utf8", newline="\n") as f:
        f.write("{\n")
        for k in ("schema", "policy", "shadow_classes", "groups"):
            f.write(' "%s": %s,\n' % (k, json.dumps(head[k], separators=(",", ":"))))
        f.write(' "quests": [\n')
        for i, r in enumerate(rows):
            f.write("  " + json.dumps(r, separators=(",", ":"), ensure_ascii=True) + ("," if i + 1 < len(rows) else "") + "\n")
        f.write(" ]\n}\n")


def set_advance(row, value):
    """Replace the advance value keeping the key position."""
    row["advance"] = value


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--policy", default="crates/coop-policy/data/policy.json")
    ap.add_argument("--old-rules", required=True, help="old rules.json (read-only), evidence for R5")
    ap.add_argument("--pending", help="old rules.pending.json, to hold rows whose dropped tether or warp is unresolved")
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()

    with open(a.policy, encoding="utf8") as f:
        pol = json.load(f, object_pairs_hook=OrderedDict)
    rows = pol["quests"]

    with open(a.old_rules, encoding="utf8") as f:
        old = json.load(f)["quests"]
    dialogue = {e for e, q in old.items() if any(x["kind"] == "InitiatorDialogue" for x in q["rules"])}

    dropped = {}
    if a.pending:
        with open(a.pending, encoding="utf8") as f:
            p = json.load(f)
        for e in p["tethers_dropped"]:
            dropped.setdefault(e, set()).add("tether")
        for e in p["warps_dropped"]:
            dropped.setdefault(e, set()).add("warp")

    moved = Counter()
    log = []
    remaining = []
    for r in rows:
        if r["status"] != "default":
            continue
        tier, line, adv = r["tier"], r["line"], r["advance"]
        story = line.startswith(("Main:", "DLC:"))
        rule, status, new_adv = None, "default", adv
        missing = sorted(k for k in dropped.get(r["edid"], ()) if r.get(k) is None)
        if r.get("synthetic") or r["plugin"] == "synthetic":
            rule, new_adv = "R4", "host"
        elif missing:
            rule = "H1"
            if tier in ("T3", "T4") and adv == "any":
                new_adv = "initiator"
        elif line.startswith("Systems") and tier in ("T0", "T1", "T2"):
            rule, status, new_adv = "R1", "confirmed", "host"
        elif story and tier in ("T3", "T4") and adv == "any":
            rule, status, new_adv = "R2", "confirmed", "initiator"
        elif tier == "T4":
            rule, status = "R3", "confirmed"
            if adv == "any":
                new_adv = "initiator"
        elif tier == "T3" and adv == "initiator" and r["edid"] in dialogue and line.startswith(("Main:", "Side:", "DLC:")):
            rule, status = "R5", "confirmed"
        elif tier in ("T3", "T4") and adv == "any":
            rule, new_adv = "R6", "initiator"
        if rule is None:
            remaining.append((r["edid"], "no rule: %s %s advance %s" % (tier, line, adv)))
            continue
        if status == "default":
            reason = {
                "R4": "synthetic row, no quest record to review",
                "H1": "dropped %s still unresolved" % "/".join(missing),
                "R6": "no old-design rule for %s %s advance any" % (tier, line),
            }[rule]
            remaining.append((r["edid"], "%s: %s" % (rule, reason)))
        if status != r["status"] or new_adv != adv:
            log.append("%-4s %-34s %s %-8s -> advance %-9s status %s" % (rule, r["edid"], tier, adv, new_adv, status))
        moved[(rule, status)] += 1
        r["status"] = status
        set_advance(r, new_adv)

    for l in log:
        print(l)
    print()
    for (rule, status), n in sorted(moved.items()):
        print("%s -> %-9s %d rows" % (rule, status, n))
    print()
    print("still default: %d" % len(remaining))
    for e, why in remaining:
        print("  %-34s %s" % (e, why))
    unsafe = [r["edid"] for r in rows if r["status"] == "default" and r["tier"] in ("T3", "T4") and r["advance"] == "any"]
    if unsafe:
        sys.exit("gate-unsafe default rows: %s" % unsafe)
    st = Counter(r["status"] for r in rows)
    print("confirmed %d default %d" % (st["confirmed"], st["default"]))
    if not a.check:
        write_policy(a.policy, pol, rows)


if __name__ == "__main__":
    main()
