#!/usr/bin/env python3
"""Generate docs/quest-policies/cause_quests.json: which quest's rules apply to what a SCRIPT commits (campaign gap G02).

Rules are keyed by quest, but a capture scope opened by a quest's own GameMode script or by an object script has the SCRIPT form as cause.source
(plugin/hooks engine_hooks.cpp, the weak-scope resolver), so PartyWarp / SetPlayerControls / SaveSuppress / SlideshowEnd / reward routing never matched
(RulesEngine::rulesFor(cause.source) only knows quests). RulesEngine::loadCauseQuests() reads this file and rewrites the source to the owning quest.

    python -I tools/questsweep/gen_cause_quests.py            # write docs/quest-policies/cause_quests.json  (needs out/questdata)
    python -I tools/questsweep/gen_cause_quests.py --check    # fail when the committed file is stale  (what the tests run; skips without out/questdata)

What is mapped
  scripts   every quest script of a quest that has rules  ->  that quest          (a script shared by two quests with rules is left out: ambiguous)
            every OBJECT script that SetStages exactly one quest with a scene-level rule (PartyWarp, Barrier, SlideshowEnd, SetPlayerControls, SaveSuppress,
            FastTravelLock, PartyTether)  ->  that quest: the endgame and battle controllers (YesManDeadSCRIPT -> VHDIndependentBattle, FortLuciusScript -> VMQ02 ...)
            CURATED below: the two object scripts that SetStage two such quests
            the scripts of a synthetic entry (wave1_data.SYNTHETIC, wave3b_data.SYNTHETIC)  ->  the entry name
            wave3b_data.CAUSE / derived_cause: the DLC set-piece object scripts (always win over the automatic rule above)
  keys      synthetic rules entries (trigger scripts that are no quest, e.g. the Think Tank weapon lock)  ->  the form that stands for them
An object script that is not listed (an elevator, a door, a stash) keeps its own cause: it moves only the activator, as before.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from typing import Any, Dict, List

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from questsweep import common  # noqa: E402

OUT = os.path.join(common.REPO, "docs", "quest-policies", "cause_quests.json")
SCENE_KINDS = {"PartyWarp", "Barrier", "SlideshowEnd", "SetPlayerControls", "SaveSuppress", "FastTravelLock", "PartyTether"}
SETSTAGE = re.compile(r"\bsetstage\s+(\w+)\s+\d+", re.I)

# Object scripts that SetStage two quests with scene rules: the one that carries the scene.
CURATED = {
    "MrHouseSCRIPT": ("VHDHouseBattle", "SetStage VMQHouse8 80 only asks; the move, lock and slideshow belong to the battle quest it starts (MrHouseSCRIPT, VHDHouseBattleSCRIPT)"),
    "VHDColonelMooreSCRIPT": ("VMQ05", "the NCR endgame confirmation on Colonel Moore moves and locks the party for VMQ05 (questlines/strip/policy_review.md, VMQ03a is only read)"),
}


def build() -> Dict[str, Any]:
    from questsweep import wave1_data as W1
    rules = common.load_json(common.RULES_JSON)["quests"]
    quests = common.load_json(os.path.join(common.QD, "quests.json"))["quests"]
    scripts = common.load_json(os.path.join(common.QD, "scripts.json.gz"))
    low = {k.lower(): k for k in rules}

    def norm(ref: str) -> str:
        plug, loc = common.split_ref(ref)
        return "%s:%06x" % (plug.lower(), loc)

    out_scripts: Dict[str, str] = {}
    owners: Dict[str, List[str]] = {}
    for q in quests:
        if q["edid"] in rules and q.get("script"):
            owners.setdefault(norm(q["script"]), []).append(q["edid"])
    for ref, qs in owners.items():
        if len(qs) == 1:
            out_scripts[ref] = qs[0]
    for s in scripts:
        if s["script"]["type"] != "object":
            continue
        hit = set()
        for m in SETSTAGE.finditer(s["script"]["source"]):
            k = low.get(m.group(1).lower())
            if k and any(r["kind"] in SCENE_KINDS for r in rules[k]["rules"]):
                hit.add(k)
        if s["edid"] in CURATED:
            hit = {CURATED[s["edid"]][0]}
        if len(hit) == 1:
            out_scripts[norm(s["ref"])] = next(iter(hit))
    keys: Dict[str, str] = {}
    by_edid = {s["edid"]: s for s in scripts}
    from questsweep import wave3b_data as W3
    for edid, (entry, _why) in W3.CAUSE.items():      # W3b: the DLC set pieces (speakers, kitchen traps, the nuke terminal, the Wreckage door ...)
        assert edid in by_edid, edid
        out_scripts[norm(by_edid[edid]["ref"])] = entry
    for ref, entry in W3.derived_cause(scripts).items():   # the OWB Sink devices -> the HQ-Buddy chain
        out_scripts[norm(ref)] = entry
    for key, spec in list(W1.SYNTHETIC.items()) + list(W3.SYNTHETIC.items()):
        for i, name in enumerate(spec["scripts"]):
            assert name in by_edid, name
            ref = norm(by_edid[name]["ref"])
            if i == 0:
                keys[key] = ref
            out_scripts[ref] = key
    return {"schema": 1,
            "note": "script / INFO / ref -> the rules entry whose rules apply to what it commits. Generated by tools/questsweep/gen_cause_quests.py; read by RulesEngine::loadCauseQuests.",
            "scripts": dict(sorted(out_scripts.items())), "keys": dict(sorted(keys.items()))}


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args(argv)
    if not common.have_questdata():
        print("SKIP: " + common.questdata_missing_reason())
        return 0
    doc = build()
    text = json.dumps(doc, indent=1) + "\n"
    if a.check:
        have = ""
        if os.path.exists(OUT):
            with open(OUT, encoding="utf-8") as f:
                have = f.read()
        if have != text:
            print("STALE: docs/quest-policies/cause_quests.json (run: python -I tools/questsweep/gen_cause_quests.py)")
            return 1
        print("ALL OK (%d scripts, %d keys)" % (len(doc["scripts"]), len(doc["keys"])))
        return 0
    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(text)
    print("wrote %s: %d scripts, %d keys" % (OUT, len(doc["scripts"]), len(doc["keys"])))
    return 0


if __name__ == "__main__":
    sys.exit(main())
