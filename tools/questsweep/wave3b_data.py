#!/usr/bin/env python3
"""Campaign fix wave 3, package W3b: the DLC set-piece rule DATA (docs/CAMPAIGN-GAPS.md G42 Dead Money, G43 Old World Blues, G44 Lonesome Road, G45 Honest Hearts).

Same contract as wave1_data.py: every table is a decision with a source, `ops(rules, qd, refs)` turns the tables into overlay ops, rules_overlay.derive() writes them into
tools/questsweep/rules_overlay.json and `rules_overlay.py --apply` puts them into docs/quest-policies/rules.json. gen_cause_quests.py reads CAUSE / SYNTHETIC / DERIVED_CAUSE
from here. Every form id that ends up in the data is resolved through tools/questsweep/refcheck.py (out/questdata, else the game's .esm); a name that does not resolve stops the
derivation, and tools/questsweep/tests/test_w3b.py re-checks the committed rules.json against the same resolver. Nothing in this file is a guess about the game data: a stage list
is checked against quests.json, a variable name against the script's own variable list, a script against scripts.json.

What the wave 1 data left open, found by auditing every DLC rule against the stage setters (tools: see the `why` of each op):
  * 19 Barrier / PartyTether rules of the four DLCs (on quests that have stages) had no `stages` list. The engine then holds EVERY stage of the quest (Barrier first), so the first SetStage of a quest with a
    tether waited for the whole party (or two minutes) and a Barrier waited for passes nothing sends. They are scoped to the stage that is the set piece (STAGE_HOLDS) or removed (DROPS).
  * the per-player variable lists named variables that no script has ('perks', 'items', 'bRestoreBodyParts' on a quest that does not declare it): VARS_REPAIR.
  * the object scripts that carry the set pieces (speakers, vault timer, kitchen traps, AutoDoc, Teleporter, the nuke terminal, the Wreckage door ...) were not in the cause map, so
    no quest rule applied to what they commit: CAUSE, DERIVED_CAUSE, SYNTHETIC.
"""
from __future__ import annotations

import os
import re
import sys
from typing import Any, Dict, List, Optional, Tuple

from questsweep import common

SOURCE = "docs/CAMPAIGN-GAPS.md"

# ====================================================================================================================================== stage scoping (G42 G43 G44)
# (quest, kind, stages, why). Merged into the existing rule; the stage numbers are checked against quests.json. "Stage setters" = the scripts / INFOs that SetStage the stage
# (tools: a SetStage scan of scripts.json, quests.json stage results and infos.jsonl.gz).
STAGE_HOLDS: List[Tuple[str, str, List[int], str]] = [
    # --- Dead Money (questlines/deadmoney/policy_review.md section 4 and 5)
    ("NVDLC01MQ00", "PartyTether", [100], "stage 100 is the gas trap (NVDLC01VillaTravelTriggerScript SetStage MQ00 100): the party gathers in the bunker (auto-warp), stage 10 is the quest start and must not wait"),
    ("NVDLC01MQ01d", "PartyTether", [100], "stage 100 hands over to Act 2 at the fountain briefing; stage 10 only displays the objective"),
    ("NVDLC01MQ02", "PartyTether", [50], "stage 50 is the bell tower trigger (the only shared moment of the three Gala tasks, policy_review.md section 4 moment 4); 10 / 100 / 255 are not"),
    ("NVDLC01MQ02c", "PartyTether", [100], "stage 100 is the switching-station elevator (NVDLC01Gala02ElevatorDoorSCRIPT SetStage MQ02c 100, which moves the activator); 10 to 70 are objectives of one player's walk"),
    ("NVDLC01MQ03", "PartyTether", [30], "stage 30 is the Suites elevator intercom (NVDLC01SuitesElevatorIntercomScpt) that hands the party down to the vault; 45 to 100 happen below"),
    ("NVDLC01MQ03a", "PartyTether", [20], "stage 20 (NVDLC01MQ03aSCRIPT) starts the kitchen stealth mission: everybody must be in the Restaurant before gas traps can spot anybody"),
    ("NVDLC01MQ03b", "PartyTether", [20], "stage 20 is the Dean talk-down in the Theater (INFOs 01062C / 01062D / 010C5B / 010C61): fight or talk is a party scene"),
    ("NVDLC01MQ03c", "PartyTether", [20], "stage 20 is the Christine talk-down in the Suites (INFOs 00A806 / 011BFE / 013464 / 013465 / 013B15)"),
    # --- Old World Blues (questlines/owb/policy_review.md moments 7 and 12)
    ("NVDLC03MQ03", "PartyTether", [20], "stage 20 (NVDLC03X8TriggerSCRIPT) is the X-8 entry that starts the experiments: the party gathers in the school (wait), moment 7"),
    ("NVDLC03MQ05", "PartyTether", [60, 70], "stages 60 (kill branch) and 70 (peace) decide how the Think Tank ends, the door then starts the ending: the decision waits until everybody is inside, moment 12"),
    # --- Lonesome Road (questlines/lonesomeroad/policy_review.md section 3)
    ("NVDLC04MQ01c", "PartyTether", [20], "stage 20 (NVDLC04MQ01cTriggerSCRIPT) is the launch view: the party is pulled to the viewing marker (auto-warp)"),
    ("NVDLC04MQ02", "PartyTether", [10], "stage 10 (NVDLC04DivideDoorSCRIPT / SunstoneTriggerREFSCRIPT) is the Divide descent through the Silo 2 elevator (auto-warp)"),
    ("NVDLC04MQ03a", "PartyTether", [10], "stage 10 is the temple door: the party enters the Ulysses temple together (wait)"),
    ("NVDLC04MQ03b", "PartyTether", [30, 255], "stage 255 is the launch terminal decision and 30 the temple exit door: both move the whole party (auto-warp), 10 / 20 / 100 do not"),
    ("NVDLC04MQ04", "PartyTether", [20], "stage 20 is the exit door of the nuke path (NVDLC04UlyssesTempleExitDoorSCRIPT)"),
]

# (quest, kind, why): a rule the lane proposed that cannot work as written.
DROPS: List[Tuple[str, str, str]] = [
    ("NVDLC01MQ02", "Barrier", "an all-players-pass Barrier with no stages holds every SetStage of MQ02 for passes that nothing sends (only chargen sends them, G26); the bell tower is held by the tether"),
    ("NVDLC01MQ03", "Barrier", "same as MQ02: the vault ending moves the party with PartyWarp, SlideshowEnd and the tether on stage 30, not with passes"),
    ("NVDLC01MQ03a", "Barrier", "same: the kitchen is held by the Restaurant tether on stage 20"),
    ("NVDLC03MQ05", "Barrier", "same: the Think Tank decision is held by the tether on stages 60 / 70; a Barrier on every stage held all ten stages (the tether was never reached, rules.cpp checks Barrier first)"),
]

# ====================================================================================================================================== per-player state (G42 G43)
DM_COLLAR_VARS = ["iNumRadii", "fCollarTimer", "fChristinesTimer", "bChristineDoOnce", "bCollarDoOnce", "fTimeToNextBeep", "fIntervalBetweenBeeps", "bBoom", "bHalfwayMsg",
                  "rFollower", "rDeadFollower", "fDeathTimer", "iDiedInCasino", "bDeanDead", "bChristineDead", "bDogDead", "fDeathTimeToNextBeep", "fDeathIntervalBetweenBeeps"]
# the collar rule: death is down-until-a-checkpoint (D9). A checkpoint is any story interior a collar cannot reach (the speakers only cover the Villa streets and the Gala floors);
# leaving the death radius is the loading screen into one of them, exactly the original rule for a companion's death (iDiedInCasino: "makes it to another part of the casino").
DM_CHECKPOINTS = ["NVDLC01Suites", "NVDLC01Restaurant", "NVDLC01Theater", "NVDLC01Gala02"]

# (quest, params, why). PerPlayerState; scripts run once per slot (objects and quest scripts), objvars keep one copy per slot of an object script's own variables.
PER_PLAYER_STATE: List[Tuple[str, Dict[str, Any], str]] = [
    ("NVDLC01BombCollarQuest",
     {"scope": "collar", "vars": DM_COLLAR_VARS, "down": "until-checkpoint", "checkpoint": DM_CHECKPOINTS,
      "scripts": ["NVDLC01BombCollarQuestSCRIPT", "NVDLC01RadioSpeakerSCRIPT"], "objvars": ["NVDLC01RadioSpeakerSCRIPT.bInRadius"]},
     "G42: every wearer has their own countdown (iNumRadii, fCollarTimer, bBoom ...), the speakers count each player's radius (bInRadius is the speaker's per-player flag), an explosion "
     "kills only its wearer and the collar death is downed until a checkpoint (D9). Variables are all 18 of NVDLC01BombCollarQuestSCRIPT; checked against the script"),
    ("NVDLC01MQ03",
     {"scope": "collar", "down": "until-checkpoint", "checkpoint": DM_CHECKPOINTS,
      "scripts": ["NVDLC01VaultCollarBeepScript", "NVDLC01VaultKillTriggerScript", "NVDLC01KitchenGasTrapSCRIPT", "NVDLC01KitchenSetupSCRIPT", "NVDLC01RestaurantInteriorDoorScript", "NVDLC01DogScript"],
      "objvars": ["NVDLC01VaultCollarBeepScript.bCollarOn", "NVDLC01VaultCollarBeepScript.fTimer", "NVDLC01VaultCollarBeepScript.fBeepInterval", "NVDLC01VaultCollarBeepScript.fBeepNext",
                  "NVDLC01VaultCollarBeepScript.fExplosionTimer", "NVDLC01VaultCollarBeepScript.iCount"]},
     "G42: the vault's 70 s timer (VaultCollarBeepScript: Player.Kill after fTimer 70), the vault kill volume, the three kitchen gas traps (player.kill) and Dog's death (player.kill) each "
     "kill the player the script ran for, not the host; the timer state is one copy per slot"),
    ("NVDLC01MQ03c",
     {"scope": "collar", "down": "until-checkpoint", "checkpoint": DM_CHECKPOINTS,
      "scripts": ["NVDLC01SuitesSetupSCRIPT", "NVDLC01ChristineSCRIPT"],
      "objvars": ["NVDLC01SuitesSetupSCRIPT.fCollarTimer", "NVDLC01SuitesSetupSCRIPT.fBeepTimer", "NVDLC01SuitesSetupSCRIPT.bCollar", "NVDLC01SuitesSetupSCRIPT.nKaboom"]},
     "G42: the Executive Suites 16 s timer after Christine dies (SuitesSetupSCRIPT fCollarTimer, then player.kill) and Christine's OnHit punishment are per slot"),
    ("NVDLC01MQ02",
     {"scope": "collar", "down": "until-checkpoint", "checkpoint": DM_CHECKPOINTS, "scripts": ["NVDLC01DeanScript"]},
     "G42: Dean's OnHit punishment (Player.KillActor Player 1) kills the attacker, not the host"),
    ("NVDLC01MQ00",
     {"scope": "entry", "scripts": ["NVDLC01VillaTravelTriggerScript"],
      "objvars": ["NVDLC01VillaTravelTriggerScript." + v for v in ("StartUp", "EndStartUp", "TimerA", "TimerB", "TimerC", "TimerD", "TimerE", "TimerZ", "bTimerZDone")]},
     "G42: the entry gas trap (OnTrigger player) fired for the host only: the trap's state machine runs once per slot, each player is gassed, knocked out and stripped for their own, and "
     "the SetStage MQ00 100 it ends with is held by the bunker tether until the party is in"),
    ("NVDLC01GlobalToxicDamage",
     {"scope": "cloud", "scripts": ["NVDLC01GlobalToxicDamageSCRIPT"]},
     "G42: the Villa-wide toxic tick (PlayerREF.DamageActorValue Health 3, Hardcore only) runs once per slot, with that slot's own health, interior / worldspace test and Hardcore flag; "
     "the damage goes to the slot the tick ran for (PerPlayerScene) so nobody pays for anybody's tick"),
    ("NVDLC01ToxicCloud",
     {"scope": "cloud", "vars": ["bToxicCloudOn", "iToxicCloudsActive", "iHealthDamage", "fTimer", "bWeatherChanged", "fDialogueDelayTimer"], "scripts": ["NVDLC01ToxicCloudSCRIPT"]},
     "G42: the cloud trigger counters and the per-update damage (PlayerREF.DamageActorValue Health iHealthDamage) are one copy per slot: a player standing in two clouds takes two ticks, "
     "a player outside none"),
    ("NVDLC03GenericPerkQUEST",
     {"scope": "implants", "scripts": ["NVDLC03GenericPerkQuestSCRIPT"],
      "vars": None},   # filled from the script's own variables by ops(): every nImplant / nRads / Y3 / Y7 / health / chem accumulator except the two shared story flags
     "G43: the implant perk loop (GRX, Y-3, Y-7, Heartless chem / stim totals) reads Player.HasPerk and counts that player's own rads, drinks and healing: one loop per slot with its own "
     "accumulators. The two *Completed flags are story state and stay shared"),
]
GENERIC_PERK_SHARED = {"bKleinDestineCompleted", "bMobiusStripCompleted"}

# (quest, kind, params, why): variable lists the earlier data got wrong. The script is the quest's own script; names are checked against it.
VARS_REPAIR: List[Tuple[str, str, Optional[List[str]], str]] = [
    ("NVDLC03MQ05", "PerPlayerVars", ["bRestoredBrain", "bRestoredHeart", "bRestoredSpine"],
     "G43 (D1, D10): organs are per player. The old list named bRestoreBodyParts / bImplants / nButton, which are AutoDoc object variables and not variables of NVDLC03MQ05SCRIPT. The jars read bRestoredBrain / Heart / Spine"),
    ("NVDLC03DialogueHQBuddies", "PerPlayerVars", ["bPsychEval", "bAwaitingInput", "iButtonVar", "bLaunchMenu"],
     "G43: the trait-respec / launch menu state of DialogueHQBuddiesSCRIPT belongs to the player at the menu. The old list named 'perks', which is no variable"),
    ("NVDLC03HQBuddy", "PerPlayerVars", None, "G43: HQBuddyQuestSCRIPT polls shared Sink progress (iAutoDoc, iSink ...); its old per-player list (bAwaitingInput, nButton, perks) names no variable of that script"),
    ("NVDLC03GenericPerkQUEST", "PerPlayerVars", None, "G43: replaced by the PerPlayerState above (same variables, plus the per-slot run); 'perks' and 'items' were no variables"),
]

# ====================================================================================================================================== OWB: AutoDoc, Teleporter (G43)
ORGAN_PERKS = ["NVDLC03BrainlessPerk", "NVDLC03HeartlessPerk", "NVDLC03SpinelessPerk", "NVDLC03BigBrainedPerk", "NVDLC03CardiacArrestPerk", "NVDLC03ReinforcedSpinePerk"]
IMPLANT_PERKS_FILTER = re.compile(r"^NVDLC03Implant\w+Perk$", re.I)
AUTODOC_USER_VARS = ["bTalkAfterMessage", "bMenuAfterMessage", "bAwaitingInput", "bCallAutoDocUI", "bCallNoBarberMSG", "bCallNoSurgeryMSG", "nButton", "bPsychEvalUI", "bPsychEvalDoOnce",
                     "bPsychEvalAwaitingInput", "bRestoreBodyParts", "bImplants", "nImplantC13Installed", "nImplantM5Installed", "nImplantY3Installed", "nImplantY7Installed",
                     "nBrainlessSwapMsg", "nHeartlessSwapMsg", "nSpinelessSwapMsg", "nBigBrainedSwapMsg", "nCardiacArrestSwapMsg", "nReinforcedSpineSwapMsg",
                     "bMakeUpMindOnce", "bFeelForHeartOnce", "bFindSpineOnce", "bBrainGainOnce", "bHeartGainOnce", "bSpineGainOnce"]
TELEPORTER_RANGE = 1500   # units: the same radius the satellite tether uses (D10: "moves the firer and party members in range")

# ====================================================================================================================================== cause map additions (G02 for the set pieces)
# script EDID -> (rules entry, why). The script's commits then use that entry's rules (gen_cause_quests.py).
CAUSE: Dict[str, Tuple[str, str]] = {
    # Dead Money
    "NVDLC01RadioSpeakerSCRIPT": ("NVDLC01BombCollarQuest", "the speakers write NVDLC01BombCollarQuest.iNumRadii: their commits belong to the collar state machine"),
    "NVDLC01VaultKillTriggerScript": ("NVDLC01MQ03", "Player.Kill in the vault: per-slot kill of MQ03's collar rule"),
    "NVDLC01RestaurantInteriorDoorScript": ("NVDLC01MQ03", "player.kill player 0 + SetStage MQ03 255 at the kitchen door"),
    "NVDLC01KitchenSetupSCRIPT": ("NVDLC01MQ03", "player.kill player 0 + SetStage MQ03 255 when Dog spots the player"),
    "NVDLC01DogScript": ("NVDLC01MQ03", "Dog's OnHit Player.KillActor and his death (player.kill + SetStage MQ03 255)"),
    "NVDLC01SuitesSetupSCRIPT": ("NVDLC01MQ03c", "the 16 s Suites timer (player.kill player 1)"),
    "NVDLC01ChristineSCRIPT": ("NVDLC01MQ03c", "Christine's OnHit Player.KillActor Player 1 and SetStage MQ03c 30"),
    # Honest Hearts: the caravan ambush, the Narrator and the doors that leap the clock by 14 days (D7)
    "NVDLC02JedSCRIPT": ("NVDLC02MQ00", "Jed's death (GetDead && stage < 100 -> SetStage MQ00 255, else IsKiller Player -> bPlayerInterference)"),
    "NVDLC02NarratorSCRIPT": ("NVDLC02MQ00", "the Narrator sets MQ00 100 and leaps GameDay + 14"),
    "NVDLC02ZionDoorSCRIPT": ("NVDLC02MQ00", "the Zion door leaps GameDay + 14 (D7: once per party event)"),
    "NVDLC02MojaveDoorSCRIPT": ("NVDLC02MQ00", "the Mojave door leaps GameDay + 14 (D7: once per party event)"),
    # Lonesome Road: nuke terminal, Ulysses battle, exit doors, the Mojave door
    "NVDLC04TempleLaunchTerminalSCRIPT": ("NVDLC04MQ03b", "the launch terminal sets MQ03b 255 and MQ04 10 and nMissileTarget (the nuke decision)"),
    "NVDLC04UlyssesBattleCodeboxSCRIPT": ("NVDLC04MQ03a", "the battle controller: bWeakPlayer, iMarkedMenDeadCount (the Marked Men kill counters), SetStage MQ03a 100 / MQ03b 10"),
    "NVDLC04UlyssesTempleExitDoorSCRIPT": ("NVDLC04MQ03b", "the temple exit door sets MQ03b 30 or MQ04 20"),
    "NVDLC04CompletionTriggerSCRIPT": ("NVDLC04MQ03b", "the 18 s completion timer sets MQ03b 100 and MQ04 100"),
    "NVDLC04MojaveDoorSCRIPT": ("NVDLC04MQ00", "the Mojave door leaps GameDay + 3 like the Wreckage door (D7: once per party event)"),
    # Old World Blues
    "NVDLC03SinkAutoDocSCRIPT": ("NVDLC03AutoDoc", "the AutoDoc: body parts, implants, the fade: all per player"),
    "NVDLC03WeapTeleporterSCRIPT": ("NVDLC03Teleporter", "the Teleporter: the firer plus the party members in range"),
}
# OWB object scripts that set NVDLC03HQBuddy variables (the Sink devices): the HQ-Buddy chain's rules apply to them.
DERIVED_CAUSE_QUEST = "NVDLC03HQBuddy"
DERIVED_CAUSE_EXCLUDE = {"NVDLC03SinkAutoDocSCRIPT"}   # CAUSE above wins
HQ_BUDDY_RX = re.compile(r"(?i)\bNVDLC03HQBuddy\s*\.\s*\w+")

# synthetic rules entries (object scripts that are no quest): name -> spec. gen_cause_quests maps the scripts to the entry.
SYNTHETIC: Dict[str, Dict[str, Any]] = {
    "NVDLC03AutoDoc": {
        "scripts": ["NVDLC03SinkAutoDocSCRIPT"],
        "entry": {"tier": "T3", "questline": "DLC: Old World Blues", "curated": False},
        "source": "questlines/owb/policy_review.md moments 3 and 6, G43: the AutoDoc serves one patient at a time and every player has their own organs and implants (D1 c, D10)"},
    "NVDLC03Teleporter": {
        "scripts": ["NVDLC03WeapTeleporterSCRIPT"],
        "entry": {"tier": "T3", "questline": "DLC: Old World Blues", "curated": False},
        "source": "questlines/owb/policy_review.md moment 10, D10: the Teleporter moves the firer and the party members in range"},
}

# ====================================================================================================================================== Honest Hearts (G45)
HH_POOLED = [   # (quest, items, why)
    ("NVDLC02MQ03a", ["NVDLC02WhiteLegTotem"], "MQ03a stage 255 removes the 2 totems"),
    ("NVDLC02MQ03d", ["NVDLC02WhiteLegTotem"], "MQ03d stages 20 and 255 remove the 2 totems"),
    ("NVDLC02MQ06", ["NVDLC02WhiteLegTotem", "NVDLC02WalkieTalkie", "NVDLC02LilScoutLunch"], "MQ06 stage 10 (the DLC fail path) removes the quest items of the whole DLC"),
    ("NVDLC02DialogueDaniel", ["NVDLC02WhiteLegTotem", "NVDLC02LilScoutLunch"], "INFO 0093AF removes the totems, INFO 0093AB the 5 lunchboxes"),
    ("NVDLC02DialogueShaman", ["NVDLC02SacredDaturaRoot"], "INFO 0094AA takes the 3 roots (MS01)"),
    ("NVDLC02MS01", ["NVDLC02SacredDaturaRoot"], "MS01 counts 3 roots over the party"),
    ("NVDLC02MQ02b", ["NVDLC02WalkieTalkie"], "MQ02b counts / removes the 2 walkie-talkies"),
    ("NVDLC02MQ02c", ["NVDLC02LilScoutLunch"], "MQ02c counts / removes the 5 lunchboxes"),
    ("NVDLC02MQ02d", ["NVDLC02WalkieTalkie", "NVDLC02LilScoutLunch"], "MQ02d stages 100 and 255 remove the walkie-talkies and lunchboxes"),
]
HH_SUM_COND = [   # (quest, why): the sum reducer on a hand-in INFO that the wave 1 data left on `initiator`
    ("NVDLC02DialogueShaman", "INFO 0094AA needs GetItemCount SacredDaturaRoot >= 3: the party's roots together, or MS01 can advance on the pooled count while nobody can hand them in"),
]
HH_GUARDED = ["NVDLC02JedREF", "NVDLC02StellaREF", "NVDLC02RickyREF"]   # placed caravan actors with an EditorID in the game data (the guards are unnamed ACHRs)
TIME_LEAP_GLOBALS = ["GameHour", "GameDay", "GameMonth", "GameYear"]
ONCE_EVENTS = [   # (quest, window_s, why)
    ("NVDLC02MQ00", 600, "D7: the 14 day leap of the Narrator / Zion door / Mojave door happens once per party event; the host clock is the only clock"),
    ("NVDLC04MQ00", 600, "D7: the +3 days of the Canyon Wreckage door and the Mojave door happen once per party event"),
]
EDE_QUESTS = ["NVDLC04MQ01a", "NVDLC04MQ03a", "NVDLC04MQ03b", "NVDLC04MQ04"]   # ED-E acts in all of these (docs/handover/P7-requests.md item 3)
EDE_REF = "NVDLC04EDEREF"
NUKE_BOXES = ["NVDLC04TempleCaesarConfirmMSG", "NVDLC04TempleNCRConfirmMSG", "NVDLC04TempleNukeAllConfirmMSG"]


# ====================================================================================================================================== build
class Ctx:
    def __init__(self, rules: Dict[str, Any], qd, refs) -> None:
        self.rules, self.qd, self.refs = rules, qd, refs
        scripts = common.load_json(os.path.join(common.QD, "scripts.json.gz"))
        self.script_by_edid = {s["edid"]: s for s in scripts}
        self.ops: List[Dict[str, Any]] = []

    def script_vars(self, edid: str) -> List[str]:
        s = self.script_by_edid.get(edid)
        if not s:
            raise KeyError("script %s is not in out/questdata/scripts.json.gz" % edid)
        return [v["name"] for v in s["script"].get("vars", [])]

    def quest_script_vars(self, quest: str) -> List[str]:
        q = self.qd.by_edid[quest]
        return [v["name"] for v in (q.get("script_data") or {}).get("vars", [])]

    def ref(self, name: str) -> str:
        return self.refs.ref_of(name)

    def ref_info(self, ref: str) -> str:
        """INFO refs are not named: check them against the INFO table instead of the EditorID index."""
        if not hasattr(self, "_infos"):
            self._infos = {r["ref"] for r in common.load_jsonl_gz(os.path.join(common.QD, "infos.jsonl.gz"))}
        assert ref in self._infos, "INFO %s does not exist in out/questdata" % ref
        return ref

    def add(self, quest: str, kind: str, mode: str, params: Dict[str, Any], why: str, source: str, **extra: Any) -> None:
        op = {"quest": quest, "kind": kind, "mode": mode, "params": params, "why": why, "source": source}
        op.update(extra)
        self.ops.append(op)


def ops(rules: Dict[str, Any], qd, refs) -> List[Dict[str, Any]]:
    c = Ctx(rules, qd, refs)
    # ---- stage scoping
    for quest, kind, stages, why in STAGE_HOLDS:
        assert quest in rules and any(r["kind"] == kind for r in rules[quest]["rules"]), (quest, kind)
        have = {s["index"] for s in qd.by_edid[quest]["stages"]}
        assert set(stages) <= have, "%s: stages %s are not stages of the quest" % (quest, sorted(set(stages) - have))
        c.add(quest, kind, "merge", {"stages": stages}, "scope the hold to the set-piece stage: without a stages list the engine holds EVERY stage of the quest", why)
    for quest, kind, why in DROPS:
        assert quest in rules, quest        # the rule may already be gone (derive runs on an applied rules.json too)
        c.add(quest, kind, "drop", {}, "removed: " + why, "stage-less all-players-pass Barrier (rules.cpp holds every stage of the quest, nothing sends a pass outside chargen)")
    # ---- per-player state
    for quest, params, why in PER_PLAYER_STATE:
        p = dict(params)
        if p.get("vars", 0) is None:
            sv = c.script_vars("NVDLC03GenericPerkQuestSCRIPT")
            p["vars"] = [v for v in sv if v not in GENERIC_PERK_SHARED]
        if "vars" in p:
            have = set(c.quest_script_vars(quest))
            assert set(p["vars"]) <= have, "%s: vars %s are no variables of its script" % (quest, sorted(set(p["vars"]) - have))
        for n in p.get("scripts", []):
            c.script_vars(n)
        for ov in p.get("objvars", []):
            sc, var = ov.split(".")
            assert var in c.script_vars(sc), "%s: %s has no variable %s" % (quest, sc, var)
        if "checkpoint" in p:
            for cell in p["checkpoint"]:
                r = refs.resolve(cell)
                assert r and r[1] in ("CELL", "WRLD"), "%s: checkpoint %s is not a cell or worldspace (%r)" % (quest, cell, r)
        c.add(quest, "PerPlayerState", "merge", p, why, "questlines/deadmoney/policy_review.md section 8, " + SOURCE + " D9 / D15")
    for quest, kind, names, why in VARS_REPAIR:
        assert quest in rules, quest
        if names is None:
            c.add(quest, kind, "drop", {}, "removed: " + why, "audit of the per-player variable lists against the scripts' own variable lists (tools/questsweep/tests/test_w3b.py)")
            continue
        have = set(c.quest_script_vars(quest))
        assert set(names) <= have, "%s: %s are no variables of its script" % (quest, sorted(set(names) - have))
        c.add(quest, kind, "merge", {"vars": names}, why, "audit of the per-player variable lists against the scripts' own variable lists")
    # ---- Old World Blues: perks of the organs go to the player on the table
    organs = [c.ref(n) for n in ORGAN_PERKS]
    impl = sorted(set(_implant_perks(refs)))
    initiator = organs + [c.ref(n) for n in impl]
    c.add("NVDLC03AutoDoc", "PerkRecipient", "add", {"initiator": initiator, "all": []},
          "G43 / D1 c / D10: the organs, their replacements and the implants the AutoDoc installs go to the patient (the player who activated it), nobody else",
          "questlines/owb/policy_review.md moment 6; docs/CAMPAIGN-GAPS.md D1", entry=SYNTHETIC["NVDLC03AutoDoc"]["entry"])
    c.add("NVDLC03AutoDoc", "SetPlayerControls", "merge", {"scope": "initiator", "release": "host-effect", "timeout_s": 60, "masks": ["1 1 1 1 1 1 1"]},
          "G43: the AutoDoc fade (DisablePlayerControls with no arguments, then EnablePlayerControls from NVDLC03AutoDocTimer) locks the patient only; spectators keep playing",
          "questlines/owb/policy_review.md moment 6 (the patch asked for scope origin and rules.json did not have it)", entry=SYNTHETIC["NVDLC03AutoDoc"]["entry"])
    for kind, params in (("PerPlayerScene", {}), ("RemoveItemPerOwner", {}),
                         ("PartyAwareCond", {"funcs": {"GetItemCount": "initiator", "HasPerk": "initiator", "IsActionRef": "initiator", "GetActorValue": "initiator"}})):
        c.add("NVDLC03AutoDoc", kind, "merge", params,
              "G43: the caps for an implant (Player.RemoveItem Caps001 8000 / 10000 / 12000 / 20000) and every item or perk check come from the patient, not from the host",
              "questlines/owb/policy_review.md moment 6", entry=SYNTHETIC["NVDLC03AutoDoc"]["entry"])
    av = c.script_vars("NVDLC03SinkAutoDocSCRIPT")
    assert set(AUTODOC_USER_VARS) <= set(av), sorted(set(AUTODOC_USER_VARS) - set(av))
    c.add("NVDLC03AutoDoc", "PerPlayerState", "merge",
          {"scope": "autodoc", "scripts": ["NVDLC03SinkAutoDocSCRIPT"], "objvars": ["NVDLC03SinkAutoDocSCRIPT." + v for v in AUTODOC_USER_VARS]},
          "G43 / G07: the AutoDoc menu (OnActivate, GameMode, MenuMode 1001) keeps one copy per patient of its menu state, swap-message queue and one-shot flags; the shared unlock flags "
          "(bAutoDocUnlocked, bBarberActive, bImplant*Active) stay shared", "questlines/owb/gaps.md, policy_review.md moment 6", entry=SYNTHETIC["NVDLC03AutoDoc"]["entry"])
    c.add("NVDLC03AutoDocTimer", "SetPlayerControls", "merge", {"scope": "initiator"},
          "G43: the timer that ends the AutoDoc scene releases the patient (the same lock source as the AutoDoc entry), not the party", "questlines/owb/policy_review.md moment 6")
    c.add("NVDLC03HQBuddy", "SetPlayerControls", "merge", {"scope": "initiator"},
          "G43: the Sink device unlock scenes lock the player at the device only", "questlines/owb/policy_review.md moment 8")
    # ---- Old World Blues: the Teleporter
    markers = ["MojaveDriveInMarkerHeadingREF", "NVDLC03SinkArrivalMarker"]
    for m in markers:
        assert refs.resolve(m), m
    c.add("NVDLC03Teleporter", "PartyWarp", "merge", {"markers": markers, "mode": "slot-offset", "absent": "hold", "range": TELEPORTER_RANGE},
          "G43 / D10: the Teleporter (OnFire, Player.MoveTo MojaveDriveInMarkerHeadingREF / NVDLC03SinkArrivalMarker) moves the firer and the party members within %d units" % TELEPORTER_RANGE,
          "questlines/owb/policy_review.md moment 10; docs/CAMPAIGN-GAPS.md D10", entry=SYNTHETIC["NVDLC03Teleporter"]["entry"])
    c.add("NVDLC03Teleporter", "PartyAwareCond", "merge",
          {"funcs": {"IsInCombat": "any-party-member", "GetEquipped": "initiator", "IsInInterior": "initiator", "IsPlayerInRegion": "initiator"}},
          "G43: the Teleporter's gates (Player.IsInCombat, IsInInterior, GetEquipped HVExplosiveCollar, IsPlayerInRegion) are the firer's, except combat which blocks the party",
          "questlines/owb/policy_review.md moment 10", entry=SYNTHETIC["NVDLC03Teleporter"]["entry"])
    # ---- Honest Hearts
    for quest, items, why in HH_POOLED:
        assert quest in rules, quest
        refs_ = [c.ref(i) for i in items]
        c.add(quest, "PooledItems", "merge", {"items": refs_}, "G45: " + why + "; a hand-in of N items is charged to the players who hold them (RulesEngine::expandPooled), not N to everybody or N to the initiator",
              "questlines/honesthearts/policy_review.md sum-over-party; docs/CAMPAIGN-GAPS.md D6")
    for quest, why in HH_SUM_COND:
        c.add(quest, "PartyAwareCond", "merge", {"funcs": {"GetItemCount": "sum-over-party"}}, "G45: " + why, "questlines/honesthearts/policy_review.md")
    c.add("NVDLC02MQ00", "ExclusiveStages", "add", {"stages": [100, 255], "resolution": "first-wins"},
          "G45: stage 100 (arrival) and the fail path 255 (a guard, Ricky, Stella or Jed died before 100) are alternatives: the first one committed wins, a second player's kill cannot fail an arrived party",
          "questlines/honesthearts/policy_review.md B5 / the MQ00 fail path (the five guard scripts: OnDeath GetStage MQ00 < 100 -> SetStage MQ00 255)")
    c.add("NVDLC02MQ00", "FriendlyFireGuard", "add", {"actors": [c.ref(n) for n in HH_GUARDED], "deliberate_hits": 3, "window_s": 10, "warn": "true"},
          "G45 / D4: an accidental hit on the caravan (splash, one stray bullet) does not fail the DLC for everybody; three direct hits in ten seconds still do",
          "questlines/honesthearts/policy_review.md; docs/CAMPAIGN-GAPS.md D4")
    c.add("NVDLC02DialogueDaniel", "ChoiceConsent", "merge",
          {"topics": ["HonestHearts.esm:010718", "HonestHearts.esm:010723", "HonestHearts.esm:0093AA", c.ref_info("HonestHearts.esm:01049F")], "mode": "vote", "timeout_s": 90},
          "G45 / D3: INFO 01049F (\"You just don't learn, do you?\") ends with SetStage MQ06 10, the DLC fail path, the same irreversible fork as Joshua / Daniel: a party vote, host breaks ties",
          "questlines/honesthearts/policy_review.md ChoiceConsent; docs/CAMPAIGN-GAPS.md D3")
    # ---- time leaps (D7)
    for quest, window, why in ONCE_EVENTS:
        c.add(quest, "OncePerEvent", "add", {"globals": [c.ref(g) for g in TIME_LEAP_GLOBALS], "effects": ["AdvanceTime"], "window_s": window}, "G44: " + why, "docs/CAMPAIGN-GAPS.md D7, docs/tools/world-time.md")
    # ---- Lonesome Road: ED-E belongs to the party from the first quest he acts in
    ede = c.ref(EDE_REF)
    for quest in EDE_QUESTS:
        assert quest in rules, quest
        c.add(quest, "CompanionOwner", "merge", {"owner": "party", "npcs": [ede]},
              "G44 / D2: ED-E (NVDLC04EDEREF) is party-owned in every quest he acts in, not only in DialogueEDE and the finale; limited to him so Rex stays the recruiter's",
              "docs/handover/P7-requests.md item 3")
    c.add("NVDLC04MQ03b", "ChoiceConsent", "merge", {"boxes": NUKE_BOXES + [c.ref(b) for b in NUKE_BOXES], "mode": "vote", "timeout_s": 90},
          "G44 / D3: the three nuke confirm boxes by EditorID and by form id (LonesomeRoad.esm:009C99 / 009C98 / 00BF02, resolved in the game data), so the vote does not depend on nvcoop_edids.json",
          "questlines/lonesomeroad/policy_review.md; docs/CAMPAIGN-GAPS.md D3")
    return c.ops


def _implant_perks(refs) -> List[str]:
    out = []
    for n, lst in refs.by_edid.items():
        if IMPLANT_PERKS_FILTER.match(n):
            for ref in lst:
                if refs.by_ref[ref][0] == "PERK":
                    out.append(refs.by_ref[ref][1])
    return out


def derived_cause(scripts: List[Dict[str, Any]]) -> Dict[str, str]:
    """script ref (Plugin.esm:HEX) -> rules entry for the OWB Sink device scripts (they all set NVDLC03HQBuddy variables)."""
    out: Dict[str, str] = {}
    for s in scripts:
        if s["edid"].startswith("NVDLC03") and s["script"]["type"] == "object" and s["edid"] not in DERIVED_CAUSE_EXCLUDE and s["edid"] not in CAUSE:
            body = "\n".join(l.split(";")[0] for l in s["script"]["source"].split("\n"))
            if HQ_BUDDY_RX.search(body):
                out[s["ref"]] = DERIVED_CAUSE_QUEST
    return out
