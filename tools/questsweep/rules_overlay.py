#!/usr/bin/env python3
"""Rules overlay: the policy gaps of the questline lanes that are rules problems, applied to docs/quest-policies/rules.json.

Why an overlay: rules.json is generated (tools/coverage/merge_rules.py) from a fixed vocabulary of 16 kinds; everything the lanes
proposed beyond it was parked in rules.pending.json. Since round 8 the RulesEngine (synccore/src/rules.cpp) implements GuestLocalRun,
PerPlayerVars, ExclusiveStages, ReadyCheck, the per-stage `stages` list, Barrier `before`, PartyTether anchor + radius and the
PerPlayerScene run/scope pair, so the parked rules can be switched on. This tool does that, idempotently:

    python -I tools/questsweep/rules_overlay.py --derive   # rebuild tools/questsweep/rules_overlay.json (needs out/questdata)
    python -I tools/questsweep/rules_overlay.py --apply    # apply the committed overlay to rules.json (offline)
    python -I tools/questsweep/rules_overlay.py --check    # rules.json holds the whole overlay and every rule validates (what the tests run)

A regenerated rules.json (tools/coverage/build_rules.py) loses the overlay; `--check` then fails and `--apply` puts it back.

What is switched on (each op carries its reason and source):
  GuestLocalRun      every quest of plugin/hooks/data/guest_local_run.json whose stages and INFOs carry no shared story effect gets the rule
                     with its own script; the DLC setup quests (NVDLC01..04MQ00, whose stage results start the DLC story and must keep
                     replicating) do NOT get it, their setup SCRIPTS are listed on VDLCPackQuest's rule instead (guestLocalScripts()).
  PerPlayerScene     run=every-machine + scope=local-machine for the 41 lane-decided per-machine quests (the engine reads the pair).
  PerPlayerVars      the per-player script variables the Old World Blues lane named (identifiers only, no wildcards).
  ExclusiveStages    the documented alternative outcomes: Nipton, the Primm sheriff endings, Yes Man army upgrade or destroy.
  ReadyCheck         the INFO-based one-way points: Mr. House "I'm ready", Yes Man "Are you ready?" topics 067 / 073.
  stages / before    Barrier and PartyTether only where the lane showed they belong (VMQ05 100, vHDBattleController 30 and 10,
                     VMQHouse1 25, slideshow barriers); without the list the engine holds EVERY stage of the quest.
  PartyTether        NVDLC03MQ00 around the crashed satellite (anchor + radius, stage 100).
  wave 3b            tools/questsweep/wave3b_data.py (the DLC set pieces: stage-scoped holds, per-player collar / AutoDoc / cloud state, PooledItems, OncePerEvent, the Teleporter range,
                     ED-E party-owned in every quest he acts in, the repaired per-player variable lists; op mode "drop" removes a rule that cannot work as written)
  wave 1             tools/questsweep/wave1_data.py (PerkRecipient, ChoiceConsent, FriendlyFireGuard, more ExclusiveStages, chargen Barrier stages, owner=party,
                     host / max / sum reductions, PerPlayerState, the synthetic trigger-script entries and the top-level "policy" object): every table there has its source.
"""
from __future__ import annotations

import argparse
import copy
import json
import os
import re
import sys
from typing import Any, Dict, List, Optional, Tuple

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from questsweep import common  # noqa: E402

OVERLAY_JSON = os.path.join(HERE, "rules_overlay.json")
ORIGIN = "questsweep"
SCHEMA = 1

# Documented decisions (lane review documents named in `source`). Stage numbers and INFO ids are checked against out/questdata by derive().
EXCLUSIVE = [
    ("VNipton", [30, 35, 70], "questlines/prologue/policy_review.md P09: rescued vs captives killed vs failure race across initiators"),
    ("nVPrimmDeputyConv", [110, 120, 130], "questlines/prologue/policy_review.md P05: the three sheriff endings are alternatives"),
    ("VMQYesMan01a", [100, 110], "questlines/yesman/policy_review.md section 4: upgrade (100) or destroy (110) the Securitron army, mutually exclusive, both complete the quest"),
]
READY = [
    ("VDialogueMrHouse", ["FalloutNV.esm:15EB69"], "questlines/house/policy_review.md F4: \"I'm ready. Let's finish this.\" is irreversible (controls off, save, warp)"),
    ("VMQTops", ["FalloutNV.esm:160723", "FalloutNV.esm:160735"],
     "questlines/yesman/policy_review.md moment 5: Yes Man topics 067 / 073 begin SetStage VMQYesMan03 100, the point of no return"),
]
STAGE_LISTS = [       # (quest, kind, stages, source): the pending Barrier.stages / PartyTether.stages values
    ("VMQ05", "Barrier", [100], "rules.pending.json Barrier.stages (T6b pnr_vote plays VMQ05 100)"),
    ("vHDBattleController", "Barrier", [30], "rules.pending.json Barrier.stages"),
    ("VMQ05", "PartyTether", [60, 100], "rules.pending.json PartyTether.stages"),
    ("vHDBattleController", "PartyTether", [10], "rules.pending.json PartyTether.stages"),
    ("VMQHouse1", "PartyTether", [25], "rules.pending.json PartyTether.stages"),
]
TETHER_ANCHOR = ("NVDLC03MQ00", {"anchor": "NVDLC03CrashedSatelliteREF", "radius": 1500, "gather": "auto-warp", "stages": [100]},
                 "questlines/owb/policy_review.md: satellite activation is one-way; the MenuMode button sets stage 100")
DLC_SETUP = ["NVDLC01MQ00", "NVDLC02MQ00", "NVDLC03MQ00", "NVDLC04MQ00", "NVDLC05MQ00"]
SCRIPT_HOLDER = "VDLCPackQuest"
IDENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


# ------------------------------------------------------------------------------------------------------------- derive
def derive(rules: Dict[str, Any]) -> Dict[str, Any]:
    from questsweep import gen_plans as G
    pending = common.load_json(common.RULES_PENDING)
    guest = common.load_json(common.GUEST_LOCAL_JSON)
    qd = G.QData()
    own: Dict[str, List[str]] = {}
    for r in common.load_jsonl_gz(os.path.join(common.QD, "infos.jsonl.gz")):
        own.setdefault(r.get("quest"), []).append(r["ref"])
    ops: List[Dict[str, Any]] = []

    def story_bearing(edid: str) -> Optional[str]:
        q = qd.by_edid.get(edid)
        if not q:
            return None
        for st in q["stages"]:
            if G.stage_block(qd, q, st["index"])["fx"]:
                return f"stage {st['index']} of {edid} has shared effects (objectives, quests, items or refs)"
        return None

    # GuestLocalRun
    holders: List[str] = []
    for it in guest["scripts"]:
        e, script = it["quest"], it["script"]
        why = story_bearing(e)
        if e == SCRIPT_HOLDER:
            continue
        if why:
            holders.append(script)
            continue
        op = {"quest": e, "kind": "GuestLocalRun", "mode": "merge", "params": {"scripts": [script]},
              "why": it.get("reason") or "per-machine quest script (plugin/hooks/data/guest_local_run.json): runs on every machine, nothing replicated",
              "source": "plugin/hooks/data/guest_local_run.json"}
        mine_only = e in rules and rules[e].get("questline") == "Systems: dlc_setup" and all(r.get("origin") == ORIGIN for r in rules[e]["rules"])
        if e not in rules or mine_only:       # a quest the generated rules never listed (no rules at all): give it an entry
            op["entry"] = {"tier": "T1", "questline": "Systems: dlc_setup", "curated": False}
        ops.append(op)
    own_script = next(it["script"] for it in guest["scripts"] if it["quest"] == SCRIPT_HOLDER)
    ops.append({"quest": SCRIPT_HOLDER, "kind": "GuestLocalRun", "mode": "merge", "params": {"scripts": [own_script] + sorted(holders)},
                "why": "the DLC setup quests' stage results start the DLC story and must keep replicating, so only their object scripts are listed here: " + ", ".join(sorted(holders)),
                "source": "plugin/hooks/data/guest_local_run.json (story-bearing quests: " + ", ".join(e for e in DLC_SETUP if story_bearing(e)) + ")"})

    # PerPlayerScene run / scope
    pr = {(x["quest"], x["param"]): x["value"] for x in pending["pending_params"] if x["kind"] == "PerPlayerScene"}
    for (e, p), v in sorted(pr.items()):
        if p == "run" and v == "every-machine" and pr.get((e, "scope")) == "local-machine" and e in rules:
            ops.append({"quest": e, "kind": "PerPlayerScene", "mode": "merge", "params": {"run": "every-machine", "scope": "local-machine"},
                        "why": "lane-decided per-machine scene: the engine drops its txns (runs on every machine)", "source": "rules.pending.json PerPlayerScene.run/scope"})

    # PerPlayerVars
    for x in pending["pending_params"]:
        if x["kind"] == "PerPlayerScene" and x["param"] == "per_player" and x["quest"] in rules:
            names = [n for n in x["value"] if IDENT.match(n)]
            if x["quest"].startswith("NVDLC"):    # a DLC list may only name variables the quest's own script declares (wave 3b audit: 'perks', 'items', AutoDoc object variables)
                have = {v["name"] for v in (qd.by_edid[x["quest"]].get("script_data") or {}).get("vars", [])} if x["quest"] in qd.by_edid else set()
                names = [n for n in names if n in have]
            if names:
                ops.append({"quest": x["quest"], "kind": "PerPlayerVars", "mode": "merge", "params": {"vars": names},
                            "why": "per-player script variables (host-side copy per slot); wildcards and prose entries of the lane list are not variables and are left out: "
                                   + ", ".join(n for n in x["value"] if not IDENT.match(n)) if len(names) != len(x["value"]) else "per-player script variables (host-side copy per slot)",
                            "source": "rules.pending.json PerPlayerScene.per_player"})

    for e, stages, src in EXCLUSIVE:
        q = qd.by_edid[e]
        have = {s["index"] for s in q["stages"]}
        assert set(stages) <= have, f"{e}: stages {sorted(set(stages) - have)} are not stages of the quest"
        ops.append({"quest": e, "kind": "ExclusiveStages", "mode": "add", "params": {"stages": stages, "resolution": "first-wins"},
                    "why": "alternative outcomes, not progress: the first committed one wins in host order", "source": src})
    for e, infos, src in READY:
        refs = {r["ref"] for r in common.load_jsonl_gz(os.path.join(common.QD, "infos.jsonl.gz")) if r["ref"] in infos}
        assert refs == set(infos), f"{e}: INFOs {sorted(set(infos) - refs)} do not exist"
        ops.append({"quest": e, "kind": "ReadyCheck", "mode": "add",
                    "params": {"infos": infos, "timeout_s": 120, "on_timeout": "cancel", "confirm": "all-players"},
                    "why": "an irreversible INFO result: hold it until every joined player confirms; on timeout cancel so an AFK player cannot drag the party in",
                    "source": src})
    for e, kind, stages, src in STAGE_LISTS:
        have = {s["index"] for s in qd.by_edid[e]["stages"]}
        assert set(stages) <= have, f"{e}: stages {stages} not in the quest"
        ops.append({"quest": e, "kind": kind, "mode": "merge", "params": {"stages": stages},
                    "why": "without a stages list the engine holds every stage request of the quest", "source": src})
    for x in pending["pending_params"]:
        if x["kind"] == "Barrier" and x["param"] == "before" and x["value"] == "slideshow" and x["quest"] in rules:
            ops.append({"quest": x["quest"], "kind": "Barrier", "mode": "merge", "params": {"before": "slideshow"},
                        "why": "controls of everyone stay locked from the barrier release until the committed SlideshowEnd",
                        "source": "rules.pending.json Barrier.before"})
    e, params, src = TETHER_ANCHOR
    ops.append({"quest": e, "kind": "PartyTether", "mode": "add", "params": params,
                "why": "the satellite button is one-way: refuse until every slot is inside the radius", "source": src})
    from questsweep import wave1_data as W1
    w1_ops, w1_policy = W1.ops_and_policy(rules, qd)
    ops += w1_ops
    from questsweep import refcheck, wave3b_data as W3
    ops += W3.ops(rules, qd, refcheck.Refs())
    return {"schema": SCHEMA, "note": "Applied to docs/quest-policies/rules.json by tools/questsweep/rules_overlay.py; derived from rules.pending.json, "
            "plugin/hooks/data/guest_local_run.json and the lane review documents named in `source`.", "ops": ops,
            "policy": w1_policy["policy"], "policy_source": w1_policy["source"]}


# -------------------------------------------------------------------------------------------------------------- apply
def _find(rules_of_quest: List[dict], kind: str, match: Optional[Dict[str, Any]] = None) -> Optional[dict]:
    """First rule of `kind`; with `match` the first whose params hold every key / value of it (picks one tether among several)."""
    for r in rules_of_quest:
        if r["kind"] == kind and (not match or all(r.get("params", {}).get(k) == v for k, v in match.items())):
            return r
    return None


def _merge(dst: Dict[str, Any], src: Dict[str, Any]) -> bool:
    changed = False
    for k, v in src.items():
        if k == "scripts":      # lists of ids are unioned, order kept
            cur = list(dst.get(k, []))
            new = cur + [x for x in v if x not in cur]
            if new != dst.get(k):
                dst[k] = new
                changed = True
        elif k == "funcs" and isinstance(v, dict):    # PartyAwareCond: the function -> reduction map is merged, the op's entries win
            cur = dict(dst.get(k, {}))
            new = {**cur, **v}
            if new != dst.get(k):
                dst[k] = new
                changed = True
        elif dst.get(k) != v:
            dst[k] = copy.deepcopy(v)
            changed = True
    return changed


def apply_ops(rules: Dict[str, Any], ops: List[Dict[str, Any]]) -> int:
    """Applies the ops to the `quests` dict in place; returns how many changed something. Idempotent."""
    changed = 0
    for op in ops:
        if op["mode"] == "drop":
            q = rules.get(op["quest"])
            before = len(q["rules"]) if q else 0
            if q:
                q["rules"] = [r for r in q["rules"] if not (r["kind"] == op["kind"] and (not op.get("match") or all(r.get("params", {}).get(k) == v for k, v in op["match"].items())))]
            changed += before - (len(q["rules"]) if q else 0)
            continue
        q = rules.get(op["quest"])
        if q is None:
            if "entry" not in op:
                raise KeyError(f"overlay names quest {op['quest']!r} which is not in rules.json")
            q = rules[op["quest"]] = {**copy.deepcopy(op["entry"]), "rules": []}
            changed += 1
        lst = q["rules"]
        if op["mode"] == "add":
            if any(r["kind"] == op["kind"] and all(r["params"].get(k) == v for k, v in op["params"].items()) for r in lst):
                continue
            lst.append({"kind": op["kind"], "params": copy.deepcopy(op["params"]), "origin": ORIGIN})
            changed += 1
            continue
        r = _find(lst, op["kind"], op.get("match"))
        if r is None:
            lst.append({"kind": op["kind"], "params": copy.deepcopy(op["params"]), "origin": ORIGIN})
            changed += 1
        elif _merge(r["params"], op["params"]):
            changed += 1
    return changed


def held(rules: Dict[str, Any], ops: List[Dict[str, Any]]) -> List[str]:
    """Ops whose effect is missing from `rules` (empty when the overlay is fully applied)."""
    miss = []
    for op in ops:
        q = rules.get(op["quest"])
        if op["mode"] == "drop":
            if q and any(r["kind"] == op["kind"] and (not op.get("match") or all(r.get("params", {}).get(k) == v for k, v in op["match"].items())) for r in q["rules"]):
                miss.append("%s/%s still present (op drop)" % (op["quest"], op["kind"]))
            continue
        ok = False
        if q:
            for r in q["rules"]:
                if r["kind"] != op["kind"]:
                    continue
                p = r["params"]
                if op["mode"] == "add" and all(p.get(k) == v for k, v in op["params"].items()):
                    ok = True
                elif op["mode"] == "merge" and all((all(x in p.get(k, []) for x in v) if k == "scripts" else
                                                    all(p.get(k, {}).get(fk) == fv for fk, fv in v.items()) if k == "funcs" else p.get(k) == v) for k, v in op["params"].items()):
                    ok = True
        if not ok:
            miss.append(f"{op['quest']}/{op['kind']} {json.dumps(op['params'])[:80]}")
    return miss


# ------------------------------------------------------------------------------------------------------ validation
def validate(rules: Dict[str, Any]) -> List[str]:
    """The load-time checks of synccore/src/rules.cpp validateRule() for the kinds the overlay touches (every quest of the file)."""
    errs = []
    for edid, q in rules.items():
        for r in q["rules"]:
            k, p = r["kind"], r.get("params", {})
            who = f"{edid}/{k}"
            if "stages" in p:
                if not isinstance(p["stages"], list) or not all(isinstance(s, int) and not isinstance(s, bool) and s >= 0 for s in p["stages"]):
                    errs.append(f"{who}: stages must be a list of whole numbers")
            if k == "PartyTether":
                cells = p.get("cells") or []
                anchor = p.get("anchor") or p.get("anchor_ref") or ""
                if not cells and not anchor:
                    errs.append(f"{who}: needs a non-empty cells list or anchor + radius")
                if anchor and not (isinstance(p.get("radius"), (int, float)) and p["radius"] > 0):
                    errs.append(f"{who}: anchor tethers need a radius > 0")
            elif k == "ExclusiveStages":
                if len(p.get("stages", [])) < 2:
                    errs.append(f"{who}: needs at least two stages")
                if p.get("resolution", "first-wins") != "first-wins":
                    errs.append(f"{who}: only resolution first-wins exists")
            elif k == "ReadyCheck":
                if not p.get("infos"):
                    errs.append(f"{who}: needs infos")
                if p.get("on_timeout", "cancel") not in ("cancel", "proceed"):
                    errs.append(f"{who}: on_timeout must be cancel or proceed")
                if p.get("confirm", "all-players") != "all-players":
                    errs.append(f"{who}: only confirm all-players exists")
                if "timeout_s" in p and not (isinstance(p["timeout_s"], (int, float)) and p["timeout_s"] > 0):
                    errs.append(f"{who}: timeout_s must be > 0")
                for i in p.get("infos", []):
                    if not re.match(r"^[A-Za-z0-9_.]+\.es[mp]:[0-9A-Fa-f]{1,8}$", str(i)):
                        errs.append(f"{who}: info '{i}' is not Plugin.esm:HEX")
            elif k == "PerPlayerVars":
                if not p.get("vars"):
                    errs.append(f"{who}: needs vars")
            elif k == "Barrier":
                if p.get("before", "slideshow") != "slideshow":
                    errs.append(f"{who}: before may only be slideshow")
            elif k == "GuestLocalRun":
                for s in p.get("scripts", []):
                    if not re.match(r"^[A-Za-z0-9_.]+\.es[mp]:[0-9A-Fa-f]{1,8}$", str(s)):
                        errs.append(f"{who}: script '{s}' is not Plugin.esm:HEX")
            elif k == "ChoiceConsent":
                ids = list(p.get("topics", [])) + list(p.get("boxes", []))
                if not ids:
                    errs.append(f"{who}: needs topics or boxes")
                for i in ids:
                    if not (re.match(r"^[A-Za-z0-9_.]+\.es[mp]:[0-9A-Fa-f]{1,8}$", str(i)) or IDENT.match(str(i))):
                        errs.append(f"{who}: '{i}' is neither Plugin.esm:HEX nor an EditorID")
                if p.get("mode", "vote") not in ("vote", "unanimous", "host", "initiator"):
                    errs.append(f"{who}: mode must be vote, unanimous, host or initiator")
                if "timeout_s" in p and not (isinstance(p["timeout_s"], (int, float)) and p["timeout_s"] > 0):
                    errs.append(f"{who}: timeout_s must be > 0")
            elif k == "PerkRecipient":
                if p.get("default", "all") not in ("all", "initiator", "recruiter"):
                    errs.append(f"{who}: default must be all, initiator or recruiter")
                if not (p.get("default") or p.get("all") or p.get("initiator") or p.get("recruiter")):
                    errs.append(f"{who}: names no perk and no default")
                for key in ("all", "initiator", "recruiter"):
                    if key in p and not isinstance(p[key], list):
                        errs.append(f"{who}: {key} must be a list")
            elif k == "FriendlyFireGuard":
                if not p.get("actors"):
                    errs.append(f"{who}: needs actors")
                if "deliberate_hits" in p and not (isinstance(p["deliberate_hits"], (int, float)) and p["deliberate_hits"] >= 1):
                    errs.append(f"{who}: deliberate_hits must be >= 1")
                if "window_s" in p and not (isinstance(p["window_s"], (int, float)) and p["window_s"] > 0):
                    errs.append(f"{who}: window_s must be > 0")
            elif k == "PerPlayerState":
                if not p.get("vars") and not p.get("scope"):
                    errs.append(f"{who}: needs vars or a scope")
                if p.get("down", "timed") not in ("until-checkpoint", "timed"):
                    errs.append(f"{who}: down must be until-checkpoint or timed")
                for key in ("scripts", "objvars", "checkpoint"):
                    if key in p and not isinstance(p[key], list):
                        errs.append(f"{who}: {key} must be a list")
                for n in p.get("objvars", []):
                    if not re.match(r"^[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*$", str(n)):
                        errs.append(f"{who}: objvars entry '{n}' is not Script.var")
                for n in p.get("scripts", []):
                    if not IDENT.match(str(n)):
                        errs.append(f"{who}: scripts entry '{n}' is not an EditorID")
            elif k == "PooledItems":
                if not p.get("items"):
                    errs.append(f"{who}: needs items")
            elif k == "OncePerEvent":
                if not p.get("globals") and not p.get("effects"):
                    errs.append(f"{who}: needs globals or effects")
                for e in p.get("effects", []):
                    if e not in ("AdvanceTime", "SetGlobal"):
                        errs.append(f"{who}: effects entry '{e}' must be AdvanceTime or SetGlobal")
                if "window_s" in p and not (isinstance(p["window_s"], (int, float)) and p["window_s"] > 0):
                    errs.append(f"{who}: window_s must be > 0")
            elif k == "PartyWarp":
                if "range" in p and not (isinstance(p["range"], (int, float)) and p["range"] > 0):
                    errs.append(f"{who}: range must be > 0")
            elif k == "SetPlayerControls":
                if p.get("scope", "all") not in ("all", "initiator"):
                    errs.append(f"{who}: scope must be all or initiator")
            elif k == "CompanionOwner":
                if p.get("owner", "recruiter") not in ("recruiter", "party"):
                    errs.append(f"{who}: owner must be recruiter or party")
    return errs


def load_overlay() -> Dict[str, Any]:
    return common.load_json(OVERLAY_JSON)


def dump_rules(path: str, doc: Dict[str, Any]) -> None:
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(doc, indent=1))      # the exact format of the generated file (no trailing newline)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--derive", action="store_true")
    g.add_argument("--apply", action="store_true")
    g.add_argument("--check", action="store_true")
    ap.add_argument("--rules", default=common.RULES_JSON)
    a = ap.parse_args(argv)
    doc = common.load_json(a.rules)
    if a.derive:
        if not common.have_questdata():
            print("SKIP: " + common.questdata_missing_reason())
            return 0
        ov = derive(doc["quests"])
        with open(OVERLAY_JSON, "w", encoding="utf-8", newline="\n") as f:
            f.write(json.dumps(ov, indent=1) + "\n")
        print(f"wrote {OVERLAY_JSON}: {len(ov['ops'])} ops")
        return 0
    ov = load_overlay()
    if a.apply:
        n = apply_ops(doc["quests"], ov["ops"])
        if ov.get("policy") and doc.get("policy") != ov["policy"]:
            doc["policy"] = copy.deepcopy(ov["policy"])
            n += 1
        errs = validate(doc["quests"])
        if errs:
            print("\n".join(errs))
            return 1
        if n:
            dump_rules(a.rules, doc)
        print(f"{n} change(s) applied" if n else "rules.json already holds the overlay")
        return 0
    miss = held(doc["quests"], ov["ops"])
    if ov.get("policy") and doc.get("policy") != ov["policy"]:
        miss.append("policy " + json.dumps(ov["policy"])[:80])
    errs = validate(doc["quests"])
    for m in miss:
        print("MISSING " + m)
    for e in errs:
        print("INVALID " + e)
    print("ALL OK" if not (miss or errs) else f"{len(miss)} missing, {len(errs)} invalid: run python -I tools/questsweep/rules_overlay.py --apply")
    return 1 if (miss or errs) else 0


if __name__ == "__main__":
    sys.exit(main())
