#!/usr/bin/env python3
"""Campaign fix wave 1, rule DATA (docs/CAMPAIGN-GAPS.md section 3, package P1). Every table here is a decision with a source; nothing is code.

rules_overlay.derive() calls ops_and_policy() and writes the result into tools/questsweep/rules_overlay.json, then `rules_overlay.py --apply` puts it into
docs/quest-policies/rules.json. The owner changes a decision by editing the table (or the committed overlay / rules.json entry) and re-running --derive --apply:
no C++ changes. The defaults are the recommended ones of docs/CAMPAIGN-GAPS.md section 4 (D1 to D15).

  G14 / D1   PerkRecipient   quest-reward and endgame perks to everyone, a trainer's perk to the player who talks, recruit perks to the recruiter, implants and organs
                             to the player (the owner decision of MASTER-PLAN says organs hit every player; D10 / D1 say per player, so per player is the default)
  G08 / D3   ChoiceConsent   party vote with the host breaking ties for the story forks outside the five end-game boxes
  G40 / D4   FriendlyFireGuard  accidental hits on named story NPCs are blocked, three direct hits in ten seconds are a deliberate attack
  G28        ExclusiveStages the alternative outcomes the lane reviews name (VMS20, VMS35, VMS19, VMS42, VMS40, Dead Money / Honest Hearts endings)
  G26        Barrier.stages  the chargen quests hold only the stage that hands the party on (VCG00 100, VCG01 200)
  G25 / D10  PartyTether.timeout_s + gather=auto-warp for the satellite
  G34 / D2   CompanionOwner owner=party for ED-E in Lonesome Road
  G37 / D8   PartyAwareCond  GetLevel = host for XP tiers, max-over-party for scaling
  G45 / D6   PartyAwareCond  the Jed weight gate reads the heaviest pack, collections pool their counts
  G42 / D9 D15 PerPlayerState for the Dead Money collar / toxic cloud / gas trap state machines
  G18        SetPlayerControls scope=initiator for the Think Tank weapon lock (a synthetic rules entry for the trigger script, see cause_quests.json "keys")
  G41 / D12 D13  the top-level "policy" object (death / late join)
"""
from __future__ import annotations

import gzip
import json
import os
import re
from typing import Any, Dict, List, Optional, Tuple

from questsweep import common

SOURCE_BACKLOG = "docs/CAMPAIGN-GAPS.md"

# ------------------------------------------------------------------------------------------------------------------------------- G14 perks
PERK_ALL = {   # quest rewards, endgame perks: every player (follows "full reward to everyone"); Big Brained is the patient's (PERK_INITIATOR, wave 3b)
    "powerarmortraining", "legionassault", "meatofchampionsperk", "khantrick", "nvdlc04dividesurvivorperk", "nvdlc04bearslayerperk",
    "nvdlc04deadmansburdenperk", "nvdlc04scourgeoftheeastperk", "nvdlc04markedperk",
}
PERK_INITIATOR = {   # a trainer, an implant, the body parts: the player who talks / is on the table (D1 c)
    "rangertakedown", "nvdlc03bigbrainedperk", "nvdlc03brainlessperk", "nvdlc03heartlessperk", "nvdlc03spinelessperk", "nvdlc03cardiacarrestperk", "nvdlc03x13robotcompliance",
    "nvdlc03implantgrxperk", "nvdlc03implanty3perk", "nvdlc03implanty7perk",
}
PERK_RECRUITER = {   # companion perks: the recruiter (MASTER-PLAN: "Companion recruit perks: owner only")
    "companionsuite", "betterhealing", "spotting", "enhancedsensors", "regularmaintenance", "fullmaintenance", "scribeassistant", "whiskeyrose", "stealthgirl",
    "calmheart", "handofvengeance", "oldvaquero", "bondsofsteel", "causelessrebel", "nvdlc01elijahlastwords", "nvdlc01elijahramblings", "nvdlc01inmyfootsteps",
    "nvdlc01sierramadremartiniperk", "nvdlc01coinoperator", "nvdlc01ghosthunter", "nvdlc04edenomines", "nvdlc04enhancedsensors",
}
PERK_RX = re.compile(r"\b(addperk|removeperk)\s+(\w+)", re.I)
IMPLANT_RX = re.compile(r"^implant\w+perk$", re.I)       # ImplantAgilityPerk, ImplantDTPerk ...: GenericPerkQUEST implants, the player on the table


def _scripts_of(quest: Dict[str, Any]):
    sd = quest.get("script_data") or {}
    yield "quest script", sd.get("source", "") or ""
    for st in quest["stages"]:
        for en in st["entries"]:
            yield "stage %d" % st["index"], (en.get("script") or {}).get("source", "") or ""


def perk_ops(rules: Dict[str, Any]) -> List[Dict[str, Any]]:
    quests = common.load_json(os.path.join(common.QD, "quests.json"))["quests"]
    by_ref = {q["ref"].lower(): q["edid"] for q in quests}
    forms = common.load_json(os.path.join(common.QD, "forms.json.gz"))
    perk_ref = {v["edid"].lower(): k for k, v in forms.items() if v.get("sig") == "PERK" and v.get("edid")}
    found: Dict[str, Dict[str, set]] = {}

    def note(edid: Optional[str], where: str, perk: str):
        if not edid or edid not in rules:
            return
        found.setdefault(edid, {}).setdefault(perk, set()).add(where)

    for q in quests:
        for where, src in _scripts_of(q):
            for m in PERK_RX.finditer(src):
                note(q["edid"], where, m.group(2))
    for r in common.load_jsonl_gz(os.path.join(common.QD, "infos.jsonl.gz")):
        txt = json.dumps(r.get("begin")) + json.dumps(r.get("end"))
        for m in PERK_RX.finditer(txt):
            note(by_ref.get((r.get("quest") or "").lower()), "info " + r["ref"], m.group(2))
    ops = []
    for edid in sorted(found):
        lists: Dict[str, List[str]] = {"all": [], "initiator": [], "recruiter": []}
        for perk in sorted(found[edid], key=str.lower):
            low = perk.lower()
            token = perk_ref.get(low, perk)
            if low in PERK_ALL:
                lists["all"].append(token)
            elif low in PERK_INITIATOR or IMPLANT_RX.match(perk):
                lists["initiator"].append(token)
            elif low in PERK_RECRUITER:
                lists["recruiter"].append(token)
        params = {k: v for k, v in lists.items() if v}
        if not params:
            continue
        ops.append({"quest": edid, "kind": "PerkRecipient", "mode": "merge", "params": params,
                    "why": "who gets the perk (D1: quest rewards and endgame perks everyone, trainer / implant / organ the player who talks, recruit perk the recruiter); "
                           "applies to dialogue results and intents, a stage result keeps the quest default (everyone)",
                    "source": "scan of AddPerk / RemovePerk in stage, quest-script and INFO result scripts; " + SOURCE_BACKLOG + " D1"})
    return ops


# ------------------------------------------------------------------------------------------------------------------------------- G08 forks
# (quest, key, ids, mode, timeout_s, why/source). `topics` are INFO refs, `boxes` message boxes (EDIDs: the runtime resolves them through nvcoop_edids.json).
CHOICES = [
    # NVDLC02DialogueDaniel (Joshua / Daniel / the MQ06 fail line) and NVDLC04MQ03b (the nuke boxes, by EditorID and form id) are in wave3b_data.py
    ("NVDLC01ElijahDialogue", "topics", ["DeadMoney.esm:011C68"], "vote", 90,
     "\"Then we better get to killing them\": bElijahTeamsWithPlayer=1, the MQ03 failure ending for everyone (questlines/deadmoney/dialogue.md; D3: party vote, host breaks ties)"),
]

# ------------------------------------------------------------------------------------------------------------------------------- G40 guards
# (quest, actors, source). Refs of placed actors; the combat layer asks RulesEngine::onProtectedHit with the victim ref.
FRIENDLY_FIRE = [
    ("NVDLC02MQ06", ["HonestHearts.esm:0093BE", "HonestHearts.esm:0093C2"],
     "Joshua and Daniel: OnStartCombat Player starts MQ06 and fails the whole DLC (NVDLC02JoshuaSCRIPT:63, NVDLC02DanielSCRIPT:39; questlines/honesthearts/policy_review.md; D4)"),
    ("VMQHouseFail", ["Lucky38DoorSec02REF", "Lucky38DoorSec03REF", "Lucky38DoorSec04REF", "Lucky38VictorEntranceREF", "Lucky38VictorElevatorREF",
                      "VL38CasinoSecuritronMkII01REF", "VL38CasinoSecuritronMkII02REF", "VL38CasinoSecuritronMkII03REF", "VL38CasinoSecuritronMkII04REF",
                      "VL38CasinoSecuritronMkII05REF"],
     "Lucky 38 Securitrons: any OnHit / OnDeath sets VMQHouseFail 1 (Lucky38SecuritronSCRIPT:6, questlines/house/policy_review.md F9; D4)"),
    ("VMQ05", ["VHDColonelMooreREF", "VAmbassadorCrockerREF"],
     "Colonel Moore and Ambassador Crocker: a death fails the NCR line for everyone (questlines/ncr/policy_review.md O1; D4)"),
    ("VMS35", ["FortCaesarRef"],
     "Caesar: FortCaesarScript OnHit player sets VMS35 255 and the Fort hostile (questlines/legion/policy_review.md; D4)"),
]

# ------------------------------------------------------------------------------------------------------------------------------- G28 forks
# (quest, stages, source). Stages are checked against out/questdata.
EXCLUSIVE_MORE = [
    ("VMS20", [100, 110], "VMS20 Boulder City hostage outcomes (100, five variants) vs the fail state 110 (questlines/towns/policy_review.md)"),
    ("VMS35", [100, 101, 105, 110, 120, 130], "VMS35 Caesar's tumor: cured by surgery 100 / killed in surgery 101 / cured with the Auto-Doc 105 / blamed 110 / not blamed 120 / killed with the Auto-Doc 130 (questlines/legion/policy_review.md M2)"),
    ("NVDLC01MQ03", [100, 255], "Dead Money ending: complete vs fail (Elijah team-up) (questlines/deadmoney/policy_review.md O3)"),
    ("NVDLC02MQ04", [100, 255], "Honest Hearts Joshua path: complete vs fail (questlines/honesthearts/policy_review.md B5)"),
    ("NVDLC02MQ05", [100, 255], "Honest Hearts Daniel path: complete vs fail (questlines/honesthearts/policy_review.md B5)"),
]

# ------------------------------------------------------------------------------------------------------------------------------- G26 / G25
BARRIER_STAGES = [
    ("VCG00", "Barrier", [100], "VCG00 stage 100 hands over to VCG01 (SetStage VCG01 0): the intro cinematic is the per-player step"),
    ("VCG01", "Barrier", [200], "VCG01 stage 200 ends chargen (StopQuest VCG01, StartQuest VMQ01): the per-player step is chargen itself"),
]
TETHER_TIMEOUT_S = 120   # D10: wait for the whole party up to this long, then auto-warp the rest (the satellite is destroyed after use)

# ------------------------------------------------------------------------------------------------------------------------------- G34
COMPANION_PARTY = [
    ("NVDLC04EDEFinaleQuest", "ED-E opens the silo and flies away for the whole party (D2: ED-E belongs to the party, the Lonesome Road plot needs him)"),
    ("NVDLC04DialogueEDE", "ED-E in Lonesome Road is a plot device shared by every player (D2)"),
]

# ------------------------------------------------------------------------------------------------------------------------------- G37 / G45
FUNCS = [
    # (quest, {function: reduction}, source)
    ("NVDLC01MQ01", {"GetLevel": "host"}, "Dead Money XP tiers read Player.GetLevel (NVDLC01MQ01Script): no per-player global exists, the host level decides (D8)"),
    ("NVDLC03MQ01", {"GetLevel": "host"}, "Old World Blues XP tiers read Player.GetLevel (NVDLC03MQ01SCRIPT): host level (D8)"),
    ("NVDLC04MQ03a", {"GetLevel": "max-over-party", "GetActorValue": "max-over-party"},
     "Lonesome Road weak-player scaling (bWeakPlayer): combat scaling reads the strongest party member (D8)"),
    ("NVDLC02DialogueCaravanMaster", {"GetActorValue": "max-over-party"},
     "Jed's departure gate compares InventoryWeight with 76 / 101: the heaviest pack in the party decides (D6; questlines/honesthearts/policy_review.md max-over-party)"),
    ("NVDLC02MQ00", {"IsKiller": "any-party-member"},
     "the caravan ambush (IsKiller Player) and the MQ00 fail path: any party member's kill counts (G45, questlines/honesthearts/policy_review.md)"),
    ("NVDLC02MQ03a", {"GetItemCount": "sum-over-party"}, "MQ03a needs 2 white-leg totems: pooled over the party (questlines/honesthearts/policy_review.md sum-over-party)"),
    ("NVDLC02MS01", {"GetItemCount": "sum-over-party"}, "MS01 needs 3 roots: pooled over the party"),
    ("NVDLC02MQ02c", {"GetItemCount": "sum-over-party"}, "MQ02c needs 5 lunchboxes: pooled over the party"),
    ("NVDLC02MQ02b", {"GetItemCount": "sum-over-party"}, "MQ02b needs 2 walkie-talkies: pooled over the party"),
]
# Trigger scripts with their own rules: rules entry (synthetic quest name) -> (form of the script that stands for it, scripts mapped to it, rules, source).
SYNTHETIC = {
    "NVDLC03ThinkTankWeaponLock": {
        "scripts": ["NVDLC03ThinkTankPacifyTriggerSCRIPT", "NVDLC03ThinkTankDoorSCRIPT"],
        "rules": [{"kind": "SetPlayerControls", "params": {"scope": "initiator", "release": "host-effect", "timeout_s": 120, "masks": ["0 0 1 0 0 0 0"]}}],
        "entry": {"tier": "T2", "questline": "DLC: Old World Blues", "curated": False},
        "source": "questlines/owb/policy_review.md moment 6: the pacification zone locks weapon drawing per player and the door releases it for that player"},
    "NVDLC04WeakPlayerScaling": {
        "scripts": ["NVDLC04DivideMarkedMenStealthBoySCRIPT", "NVDLC04DivideMarkedMenGrenadeSCRIPT", "NVDLC04MarkedMenTemplateSCRIPT"],
        "rules": [{"kind": "PartyAwareCond", "params": {"funcs": {"GetLevel": "max-over-party", "GetActorValue": "max-over-party"}}}],
        "entry": {"tier": "T2", "questline": "DLC: Lonesome Road", "curated": False},
        "source": "Marked Men gear scales with Player.GetLevel > 30: the strongest party member decides (D8)"},
}

# ------------------------------------------------------------------------------------------------------------------------------- G42
PER_PLAYER_STATE = []   # the collar (all 18 variables, down until a checkpoint, the per-slot scripts), the vault / suites / kitchen / entry / cloud state: wave3b_data.PER_PLAYER_STATE

# ------------------------------------------------------------------------------------------------------------------------------- G41
POLICY = {
    "death": {"mode": "downed", "revive_s": 60, "all_down": "reload-host-save"},
    "late_join": {"during_scene": "wait", "after_scene": "warp-to-zone"},
}
POLICY_SOURCE = "docs/CAMPAIGN-GAPS.md D12 (downed, then respawn at the last zone checkpoint, host save only when the whole party is down) and D13 (late joiners wait for the scene, then warp to the zone)"


# ------------------------------------------------------------------------------------------------------------------------------- build
def _find_script_ref(scripts: List[dict], edid: str) -> Optional[str]:
    for s in scripts:
        if s["edid"] == edid:
            return s["ref"]
    return None


def ops_and_policy(rules: Dict[str, Any], qd) -> Tuple[List[Dict[str, Any]], Dict[str, Any]]:
    ops: List[Dict[str, Any]] = []
    scripts = common.load_json(os.path.join(common.QD, "scripts.json.gz"))
    ops += perk_ops(rules)
    for quest, key, ids, mode, timeout, why in CHOICES:
        assert quest in rules, quest
        ops.append({"quest": quest, "kind": "ChoiceConsent", "mode": "add", "params": {key: ids, "mode": mode, "timeout_s": timeout},
                    "why": why, "source": "questlines (see why); " + SOURCE_BACKLOG + " D3"})
    for quest, actors, why in FRIENDLY_FIRE:
        assert quest in rules, quest
        ops.append({"quest": quest, "kind": "FriendlyFireGuard", "mode": "merge",
                    "params": {"actors": actors, "deliberate_hits": 3, "window_s": 10, "warn": "true"},
                    "why": "accidental hits on named story NPCs are blocked with a warning; three direct hits inside ten seconds (or the host's confirmation) are a deliberate attack that fails the line for everybody",
                    "source": why + "; " + SOURCE_BACKLOG + " D4"})
    for quest, stages, why in EXCLUSIVE_MORE:
        have = {s["index"] for s in qd.by_edid[quest]["stages"]}
        assert set(stages) <= have, "%s: stages %s are not stages of the quest" % (quest, sorted(set(stages) - have))
        assert quest in rules, quest
        ops.append({"quest": quest, "kind": "ExclusiveStages", "mode": "add", "params": {"stages": stages, "resolution": "first-wins"},
                    "why": "alternative outcomes, not progress: the first committed one wins in host order", "source": why})
    for quest, kind, stages, why in BARRIER_STAGES:
        have = {s["index"] for s in qd.by_edid[quest]["stages"]}
        assert set(stages) <= have, quest
        ops.append({"quest": quest, "kind": kind, "mode": "merge", "params": {"stages": stages},
                    "why": "without a stages list every stage request of the quest is held; a guest stage intent on this stage counts as the player's pass (RulesEngine::onGuestStageIntent)",
                    "source": why})
    for quest, why in COMPANION_PARTY:
        assert quest in rules, quest
        ops.append({"quest": quest, "kind": "CompanionOwner", "mode": "merge", "params": {"owner": "party"}, "why": "owner=party: no follower cap, any player may command it",
                    "source": why})
    for quest, funcs, why in FUNCS:
        op = {"quest": quest, "kind": "PartyAwareCond", "mode": "merge", "params": {"funcs": funcs}, "why": "reduction over the party for " + ", ".join(sorted(funcs)), "source": why}
        if quest not in rules:
            op["entry"] = {"tier": "T2", "questline": "DLC", "curated": False}
        ops.append(op)
    for quest, names, scope, why in PER_PLAYER_STATE:
        assert quest in rules, quest
        ops.append({"quest": quest, "kind": "PerPlayerState", "mode": "merge", "params": {"scope": scope, "vars": names},
                    "why": "per-slot copies of these variables (RulesEngine::beginActing / endActing); a player.kill in the quest hits only the acting player", "source": why})
    for key, spec in SYNTHETIC.items():
        for r in spec["rules"]:
            ops.append({"quest": key, "kind": r["kind"], "mode": "merge", "params": r["params"], "entry": spec["entry"],
                        "why": "synthetic rules entry for object scripts that are no quest (cause_quests.json keys / scripts)", "source": spec["source"]})
    # satellite tether: wait for the party up to a timeout, then auto-warp (D10)
    ops.append({"quest": "NVDLC03MQ00", "kind": "PartyTether", "mode": "merge", "params": {"timeout_s": TETHER_TIMEOUT_S, "gather": "auto-warp"},
                "match": {"anchor": "NVDLC03CrashedSatelliteREF"},
                "why": "the satellite is destroyed after use, nobody can enter later: wait up to the timeout, then auto-warp the rest (D10)",
                "source": "questlines/owb/policy_review.md; " + SOURCE_BACKLOG + " D10"})
    policy = {"policy": POLICY, "source": POLICY_SOURCE}
    return ops, policy
