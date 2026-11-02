#!/usr/bin/env python3
"""One-off converter: old co-op mod quest policy table -> crates/coop-policy/data/policy.json.

Run once (kept for provenance, the output is committed):

    python -I tools/policy-import/import.py --old <old-mod-checkout> --out crates/coop-policy/data/policy.json

Inputs, all read-only, under the old checkout:
  docs/quest-policies/rules.json    per-quest rule list (the 483 quest table), plus the global policy block
  docs/quest-policies/policies.json hand-written scene notes (179 story quests, 43 curated)
  out/questdata/quests.json         quest records, used only to attach form ids and display names

Semantics only are carried over. Mapping of old rule kinds to row fields:

  RewardAll            -> reward = everyone (the default, so omitted)
  PooledItems          -> pooled_items (party-wide counted items)
  QuestItemDup         -> quest_items = all (default, omitted)
  ShareKarmaRep        -> karma_rep shared (default, omitted)
  RemoveItemPerOwner   -> payment by the initiator only (default, omitted)
  InitiatorDialogue    -> advance = initiator
  PartyTether          -> tether
  Barrier              -> barrier (all players pass, one-way transitions)
  ReadyCheck           -> ready
  ExclusiveStages      -> exclusive
  ChoiceConsent        -> consent
  PartyWarp            -> warp
  SetPlayerControls    -> controls
  SaveSuppress         -> save_suppress
  FastTravelLock       -> fast_travel_lock
  SlideshowEnd         -> host_decision = ending
  CompanionOwner       -> companion
  PerkRecipient        -> perks
  PerPlayerState/Vars  -> shadow
  PerPlayerScene       -> scope = per_player
  GuestLocalRun        -> guest_local
  FriendlyFireGuard    -> friendly_fire
  OncePerEvent         -> once
  ExcludeFromGate      -> gate_exempt
  PartyAwareCond       -> party_cond (function -> reducer)

Status: a row is `confirmed` when the old table marks the quest as hand-curated, `default` otherwise.
Default rows are rules derived from risk categories and never reviewed by hand.
"""
import argparse
import json
import re
import sys
from collections import Counter, OrderedDict

SYNTHETIC_BASE = 0xFD000000

# Final faction battles: one outcome per playthrough, the first one committed wins.
GROUPS = [
    OrderedDict(
        name="final_battle",
        members=["VHDLegionBattle", "VHDHouseBattle", "VHDIndependentBattle"],
        note="Caesar, House and Yes Man endings are mutually exclusive; first committed wins, host breaks races.",
    ),
]

SHADOW_CLASSES = OrderedDict(
    skills="per_player",
    perks="per_player",
    followers="per_player",
    organs="per_player",
    implants="per_player",
    collar="per_player",
    casino="per_player",
    storage="per_player",
    quest_stage="per_player",
    quest_var="per_player",
    journal_focus="per_player",
    achievements="per_player",
    karma="shared",
    reputation="shared",
    faction="shared",
    quest_item="shared",
    world="shared",
)


def clean_text(s):
    s = s.replace("—", ",").replace("–", "-").replace("‘", "'").replace("’", "'")
    s = s.replace("“", '"').replace("”", '"')
    s = re.sub(r"[^\x20-\x7e]", " ", s)
    return re.sub(r"\s+", " ", s).strip()


def trim(s, n=260):
    s = clean_text(s)
    if len(s) <= n:
        return s
    cut = s[:n]
    p = max(cut.rfind(". "), cut.rfind("; "))
    if p > n // 2:
        return cut[: p + 1]
    return cut[: cut.rfind(" ")] + "..."


def hexid(v):
    return "0x%08X" % v


def build_row(edid, q, meta, note):
    rules = q["rules"]
    by = {}
    for r in rules:
        by.setdefault(r["kind"], []).append(r["params"])
    has = lambda k: k in by
    first = lambda k: by[k][0]

    line = q["questline"]
    row = OrderedDict()
    row["edid"] = edid
    row["id"] = hexid(meta["id"])
    if meta["synthetic"]:
        row["synthetic"] = True
    row["plugin"] = meta["plugin"]
    row["tier"] = q["tier"]
    row["line"] = clean_text(line)
    row["status"] = "confirmed" if q.get("curated") else "default"

    # who may advance a stage
    if has("ExcludeFromGate"):
        adv = "any"
    elif has("SlideshowEnd") or "timer" in line.lower() or line.lower().startswith("systems: ending"):
        adv = "host"
    elif has("InitiatorDialogue"):
        adv = "initiator"
    else:
        adv = "any"
    row["advance"] = adv

    if has("PerPlayerScene"):
        row["scope"] = "per_player"
    if has("ExcludeFromGate"):
        row["gate_exempt"] = True

    if has("SlideshowEnd"):
        row["host_decision"] = "ending"
    elif line.lower().startswith("systems: ending"):
        row["host_decision"] = "ending"
    elif has("OncePerEvent") or "timer" in line.lower():
        row["host_decision"] = "time"
    elif has("PartyWarp") and any(p.get("scope") == "all" for p in by.get("SetPlayerControls", [])):
        row["host_decision"] = "cinematic"

    if has("PooledItems"):
        items = []
        for p in by["PooledItems"]:
            items += p.get("items", [])
        row["pooled_items"] = sorted(set(items))

    if has("PartyTether"):
        p = first("PartyTether")
        t = OrderedDict()
        if p.get("cells"):
            t["cells"] = p["cells"]
        if p.get("anchor"):
            t["anchor"] = p["anchor"]
        if p.get("radius"):
            t["radius"] = p["radius"]
        if p.get("stages"):
            t["stages"] = p["stages"]
        t["gather"] = "auto_warp" if p.get("gather") == "auto-warp" else "wait"
        if p.get("timeout_s"):
            t["timeout_s"] = p["timeout_s"]
        row["tether"] = t
    if has("Barrier"):
        stages, before = [], None
        for p in by["Barrier"]:
            stages += p.get("stages", [])
            before = before or p.get("before")
        b = OrderedDict()
        if stages:
            b["stages"] = sorted(set(stages))
        if before:
            b["before"] = before
        row["barrier"] = b
    if has("ReadyCheck"):
        p = first("ReadyCheck")
        row["ready"] = OrderedDict(infos=p.get("infos", []), timeout_s=p.get("timeout_s", 120))
    if has("ExclusiveStages"):
        stages = []
        for p in by["ExclusiveStages"]:
            stages += p["stages"]
        row["exclusive"] = OrderedDict(stages=sorted(set(stages)))
    if has("ChoiceConsent"):
        p = first("ChoiceConsent")
        c = OrderedDict(mode=p.get("mode", "vote"), timeout_s=p.get("timeout_s", 90))
        if p.get("topics"):
            c["topics"] = p["topics"]
        if p.get("boxes"):
            c["boxes"] = p["boxes"]
        row["consent"] = c
    if has("PartyWarp"):
        p = first("PartyWarp")
        w = OrderedDict(markers=p.get("markers", []))
        if p.get("absent") == "warp":
            w["absent"] = "warp"
        if p.get("range"):
            w["range"] = p["range"]
        row["warp"] = w
    if has("SetPlayerControls"):
        p = first("SetPlayerControls")
        c = OrderedDict()
        if p.get("scope") == "initiator":
            c["scope"] = "initiator"
        c["timeout_s"] = p.get("timeout_s", 120)
        if p.get("masks"):
            c["masks"] = p["masks"]
        row["controls"] = c
    if has("SaveSuppress"):
        row["save_suppress"] = True
    if has("FastTravelLock"):
        row["fast_travel_lock"] = True
    if has("CompanionOwner"):
        p = first("CompanionOwner")
        c = OrderedDict(owner=p.get("owner", "recruiter"))
        if p.get("npcs"):
            c["npcs"] = p["npcs"]
        row["companion"] = c
    if has("PerkRecipient"):
        p = first("PerkRecipient")
        pk = OrderedDict()
        for k in ("all", "initiator", "recruiter"):
            if p.get(k):
                pk[k] = p[k]
        row["perks"] = pk
    shadow_src = by.get("PerPlayerState", []) + by.get("PerPlayerVars", [])
    if shadow_src:
        sh = OrderedDict()
        sh["scope"] = (shadow_src[0].get("scope") or "vars")
        for k in ("vars", "objvars", "scripts", "checkpoint"):
            vals = []
            for p in shadow_src:
                vals += p.get(k, [])
            if vals:
                sh[k] = vals
        if any(p.get("down") for p in shadow_src):
            sh["down"] = "until_checkpoint"
        if "vars" not in sh:
            sh["vars"] = []
        row["shadow"] = sh
    if has("GuestLocalRun"):
        sc = []
        for p in by["GuestLocalRun"]:
            sc += p.get("scripts", [])
        row["guest_local"] = sc
    if has("FriendlyFireGuard"):
        p = first("FriendlyFireGuard")
        row["friendly_fire"] = OrderedDict(
            actors=p.get("actors", []),
            deliberate_hits=p.get("deliberate_hits", 3),
            window_s=p.get("window_s", 10),
        )
    if has("OncePerEvent"):
        p = first("OncePerEvent")
        row["once"] = OrderedDict(window_s=p.get("window_s", 600), effects=p.get("effects", []))
    if has("PartyAwareCond") and first("PartyAwareCond").get("funcs"):
        row["party_cond"] = OrderedDict(sorted(first("PartyAwareCond")["funcs"].items()))
    if note:
        row["note"] = note
    return row


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--old", required=True, help="root of the old co-op mod checkout")
    ap.add_argument("--out", required=True)
    a = ap.parse_args()

    def load(rel):
        with open(a.old.rstrip("/\\") + "/" + rel, encoding="utf8") as f:
            return json.load(f)

    rules = load("docs/quest-policies/rules.json")
    notes_src = {p["edid"]: p for p in load("docs/quest-policies/policies.json")}
    qrecs = {q["edid"]: q for q in load("out/questdata/quests.json")["quests"]}

    banned = re.compile("|".join(["nv" + "mp", "nv:" + "mp", "new vegas multi" + "player"]), re.I)

    rows = []
    synth = 0
    for edid in sorted(rules["quests"]):
        q = rules["quests"][edid]
        rec = qrecs.get(edid)
        if rec:
            meta = dict(id=int(rec["formid"], 16), plugin=rec["plugin"], synthetic=False)
        else:
            meta = dict(id=SYNTHETIC_BASE + synth, plugin="synthetic", synthetic=True)
            synth += 1
        note = ""
        cur = notes_src.get(edid, {}).get("curated")
        if isinstance(cur, dict) and cur.get("rule"):
            note = trim(cur["rule"])
            if banned.search(note):
                note = ""
        rows.append(build_row(edid, q, meta, note))

    ids = Counter(r["id"] for r in rows)
    dup = [k for k, v in ids.items() if v > 1]
    if dup:
        sys.exit("duplicate form ids: %s" % dup)

    gm = []
    for g in GROUPS:
        gm.append(OrderedDict(name=g["name"], members=g["members"], note=g["note"]))

    pol = rules["policy"]
    out = OrderedDict()
    out["schema"] = 1
    out["policy"] = OrderedDict(
        death=OrderedDict(
            mode=pol["death"]["mode"],
            revive_s=pol["death"]["revive_s"],
            all_down=pol["death"]["all_down"],
        ),
        late_join=OrderedDict(during_scene=pol["late_join"]["during_scene"], after_scene=pol["late_join"]["after_scene"]),
        barrier_timeout_s=120,
        tether_timeout_s=120,
        follower_cap=OrderedDict(humanoid=1, critter=1),
        party_cap=4,
    )
    out["shadow_classes"] = SHADOW_CLASSES
    out["groups"] = gm

    with open(a.out, "w", encoding="utf8", newline="\n") as f:
        f.write("{\n")
        for k in ("schema", "policy", "shadow_classes", "groups"):
            f.write(' "%s": %s,\n' % (k, json.dumps(out[k], separators=(",", ":"))))
        f.write(' "quests": [\n')
        for i, r in enumerate(rows):
            f.write("  " + json.dumps(r, separators=(",", ":"), ensure_ascii=True) + ("," if i + 1 < len(rows) else "") + "\n")
        f.write(" ]\n}\n")
    st = Counter(r["status"] for r in rows)
    print("rows %d confirmed %d default %d synthetic ids %d" % (len(rows), st["confirmed"], st["default"], synth))


if __name__ == "__main__":
    main()
