"""Static wiring check for the engine-facing glue of the co-op session (campaign fix wave 2, package P6).

coop_module.cpp and intents_module.cpp need the game to run, so no unit test can execute them. What can go wrong is that a call site the
campaign relies on (a hook observer, a provider, the guest dialogue frame, the save sidecar) gets deleted or never added. This script reads
the two files and fails when one of the REQUIRED call sites is missing, or when a FORBIDDEN leftover is back.

    python -I plugin/coop/tests/check_module_wiring.py              check the real files
    python -I plugin/coop/tests/check_module_wiring.py --self-test  also remove every required call site in turn (and add every forbidden one)
                                                                    and prove the check catches each: the mutation twins of this check

Each entry says which gap or handover item the call site serves.
"""
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
MODULE = "plugin/coop/coop_module.cpp"
INTENTS = "plugin/intents/engine/intents_module.cpp"
COMBAT = "plugin/combat/combat_module.cpp"
MAIN = "plugin/main.cpp"
CORE = "plugin/loader/core_entry.cpp"
COMPANIONS = "plugin/companions/companions_module.cpp"
ACTORS_PORT = "plugin/actors/engine/engine_port.cpp"
LIVE_GLUE = "plugin/coop/live/live_glue.cpp"

REQUIRED = {
    MODULE: [
        # P3 handover call sites (G05, G33)
        ("GuestDialogue::instance().init(", "P3 call 1: guest dialogue init before the runtime starts"),
        ("GuestDialogue::instance().sink()(e)", "P3 call 2: dialogue events reach the engine glue"),
        ("GuestDialogue::instance().infoReader()", "P3 call 3: the INFO reader of the guest glue"),
        ("GuestDialogue::instance().frame(g_rt->dialogue()", "P3 call 4: the per-frame poll of the DialogMenu"),
        ("GuestDialogue::instance().frame(nullptr", "P3 call 4: the glue lets go of a node that died with the runtime"),
        ("GuestDialogue::instance().statusJson()", "P3 call 8: coop.status carries the dialogue counters"),
        ("h.companionRules", "G33 / D2: follower caps from nvcoop.ini reach the dialogue node"),
        # P2 handover call sites (G23, G10, G42, G35, G04)
        ("setResultScope(OpenFactsScope)", "P2 handover: PlayerFactsScope around a guest's result script"),
        ("nvhooks::PlayerFactsScope", "P2 handover: the facts scope itself"),
        ("setPlayerKillObserver(", "G42: a script kill of one player"),
        ("setPlayerStashObserver(", "G35: gear moves of a guest's player"),
        ("setNoteObserver(", "G10: a note a guest picked up by itself"),
        ("setPartyConditions(", "G23: the evaluators loop over the party"),
        ("setPartyStateProvider(", "G23: per-member perk / note / owner answers"),
        ("makeSessionReducer(", "G23: the rules engine reduces the values"),
        ("presentMembers(", "G23: who is in the party"),
        ("setOrderApplier(ApplyOrder)", "guest side of the player orders"),
        ("setOrderHandler(HandleOrder)", "host side of the guest requests"),
        ("registerPlayerSlot(", "acting-player slots of the puppets"),
        # P1 handover (G02, G18, G24, G25, G41, owner switch tether)
        ("NVCOOP_MODULE_LATE(l10_coop", "the session frame runs after the avatar capture (one frame of avatar lag otherwise)"),
        ("glueSetRosterListener(", "P7: companion owners and party flags into the dialogue registry, reconcile fixes committed"),
        ("setGuestStagePassHook(", "W1B 3: a guest's SetStage on a barrier stage is its pass"),
        ("setPlayerRefProvider(", "W1B 5: player.kill is recorded with the player's own ref"),
        ("nvhooks::questVarName(", "W1B 2: per-player condition views"),
        ("nvhooks::questVarValue(", "W1B 6: acting copies read quest variables by name"),
        ("nvhooks::setQuestVarValue(", "W1B 6: ... and write them"),
        ("g_rt->setCauseQuests(", "G02: the cause -> quest map"),
        ("g_rt->setPolicy(ReadRulesPolicy())", "owner switch: nvcoop.ini [rules] tether / death.* reach the rules engine"),
        ("setHostEffectApplier(", "G18 / G25 / G41: the host applies what the rules engine commits for its own player"),
        ("setPlayerVarIO(", "G24: per-player quest variables around a guest's run"),
        ("setAnchorProbe(", "G25: anchor tether positions"),
        ("g_rt->reportCell(ref, wref)", "G25: exterior tethers get the worldspace"),
        ("g_rt->localPlayerDied()", "G41: the death policy"),
        ("setReloadHostSave(", "G41: everybody down"),
        ('op == "downed"', "G41: the downed state of the local player"),
        ('op == "respawn"', "G41: the respawn at the checkpoint"),
        ("MReady", "G27: the ready-check confirmation"),
        ("clearPlayerSlots(", "P4 request 2: the slot table is cleared on a load"),
        # P5 handover (G15, G20, G39)
        ("setScriptRunObserver(", "G15: same-frame ref var flips (noteRefScriptRan)"),
        ("nvworld::worldEffectKey(", "P5 handover 1: the coalescing key of every world effect"),
        ("nvworld::isEphemeral(", "P5 handover 2: a fade or a sound is not replayed to a late joiner"),
        # P4 handover (G35)
        # W3a (wiring leftovers)
        ("bindSlotFacts(", "W3a 3: GetEquipped / IsSneaking / region / level / sex / seat / action of a guest from the avatar layer"),
        ("glueSlotFacts(", "W3a 3: ... through the avatar service's Look, snapshot and Facts message"),
        ("setScriptRunScope(", "W3a 4: the acting guest's companion view around each script"),
        ("setViewPolicy(", "W3a 4: glueViewSlot / glueCurrentFollower reach the session"),
        ("glueViewSlot(", "W3a 4: which slot's hired flags a script reads"),
        ("glueCurrentFollower(", "W3a 4: NVDLC01Followers.rCurrentFollower per player"),
        ("glueViewPartyScripts(", "W3a 4: the view-party scripts are named by form id"),
        ("setControlsRouter(", "W3a 5: a guest-only lock does not lock the host"),
        # W3d (hooks and session for the DLC set pieces)
        ("setItemCountProbe(", "W3d 1: Options::itemCount (pooled hand-ins) reads the engine's GetItemCount of one player"),
        ("nvhooks::playerItemCount(", "W3d 1: ... through the original evaluator, no party reduction"),
        ("g_rt->perSlotScripts()", "W3d 1: the scripts the rules run once per slot are named"),
        ("setPerSlotScripts(", "W3d 1: ... and the hooks run them again for every guest with a puppet"),
        ("runScriptPerSlot(", "W3d 1: ... inside that guest's window (commit origin, quest variable copies)"),
        ("setInfoQuestScopes(", "W3d 2: the conditions of an INFO are reduced under its quest (Jed weight gate)"),
        ("nvhooks::perSlotStats()", "W3d: coop.status carries the per-slot run and INFO scope counters the in-game proofs read"),
        ("stashOrderJson(", "W3d 4: a typed gear strip travels to the guest with its type"),
        ("stashLine(", "W3d 4: ... and runs on the guest as RemoveAllTypedItems"),
        # guest pack facts (a guest's weight and watched item counts reach the host)
        ("glueSetPackWatch(", "guest pack facts: the watch list (nvcoop_pack_watch.json) is resolved to runtime form ids for the avatar service, both roles"),
        ("nvcoop_pack_watch.json", "guest pack facts: the generated watch list is loaded next to the dll"),
        ("bindSlotPack(", "guest pack facts: GetItemCount / InventoryWeight of a guest are answered from its Pack message"),
        ("gluePackCount(", "guest pack facts: ... the pooled hand-ins ask the same facts first (setItemCountProbe)"),
        ("nvhooks::GuestItemScope", "guest pack facts: a guest's hand-in is not taken from the host's own pack"),
        ("g_sidecars.onSave(", "P4 handover 8: the gear ledger is written next to the save"),
        ("objVarSnapshot(nvhooks::objVarCopies())", "follow-ups 1 item 3: the per-slot object variable copies are written next to the save"),
        ("objVarRestore(nvhooks::objVarCopies()", "follow-ups 1 item 3: the copies of the loaded save come back"),
        ("g_sidecars.onPostLoad(", "P4 handover 8: the ledger of the loaded save comes back"),
        ("nvcoop::intents::LedgerSnapshot()", "P4 handover 8"),
        ("nvcoop::intents::LedgerRestore(", "P4 handover 8"),
    ],
    MAIN: [("RunFrameCallbacks(", "frame order: late modules (the co-op session) after the capture modules")],
    CORE: [("RunFrameCallbacks(", "frame order in the split loader's core")],
    COMPANIONS: [("companionVet =", "W3a 6: the commit asks the companion book before a txn is sent"), ("vetOutgoing(", "W3a 6: ... through SocialSync")],
    LIVE_GLUE: [
        ("glue::ReadLocalFacts(", "the shipped avatar side forwards the Facts read (without it no Facts message ever left a guest in game)"),
        ("glue::ReadLocalPack(", "guest pack facts: the shipped avatar side forwards the pack read"),
    ],
    ACTORS_PORT: [("killerOf", "W3a 2 (G04): the kill of a copy is credited to the remote player whose shot did it")],
    COMBAT: [
        ("setKillerResolver(", "W3a 2 (G04): the actor port asks combat who to credit"),
        ("killWithAttribution(", "W3a 2 (G04): the credited kill goes through KillActor <killer>"),
        ("blockProtectedHit", "G40: the combat hit gate asks the rules before a player's hit lands on a protected story actor"),
        ("protectedHit(v, attacker, splash)", "G40: ... through Runtime::protectedHit"),
    ],
    INTENTS: [
        ("guestStagePassHook()", "W1B 3: the intents host asks the session's pass hook"),
        ("GuestDialogue::instance().onTalkRequested(", "P3 call 5: the guest's talk request goes to the host through the glue"),
        ("g_talkDeferred", "P3 call 5: the vanilla activation is suppressed while the host decides"),
        ("LedgerSnapshot()", "P4 handover 8: export for the save sidecar"),
        ("LedgerRestore(", "P4 handover 8: export for the save sidecar"),
    ],
}

# Leftovers that must not come back: (pattern, a line that matches it, why)
FORBIDDEN = {
    MODULE: [
        (r"g_scriptedLock\b", "bool locked = g_scriptedLock;", "G18: the coarse one-flag control lock is gone; the committed effects carry recipient and mask"),
        (r"\bWorldEffectKey\(", "auto k = WorldEffectKey(we);", "the local key switch was replaced by nvworld::worldEffectKey (P5 handover 1)"),
        (r"fndlg::makeInfoReader\(", "w.infoReader = fndlg::makeInfoReader(mem, &layout);", "the P3 glue owns the INFO reader (its layout is probed at run time)"),
    ],
}


def read(files):
    return {f: (REPO / f).read_text(encoding="utf-8", errors="replace") for f in files}


def check(texts):
    """List of problems for the given {file: text}."""
    problems = []
    for f, items in REQUIRED.items():
        t = texts.get(f, "")
        for token, why in items:
            if token not in t:
                problems.append(f"{f}: missing `{token}` ({why})")
    for f, items in FORBIDDEN.items():
        t = texts.get(f, "")
        for pat, sample, why in items:
            if re.search(pat, t):
                problems.append(f"{f}: still has /{pat}/ ({why})")
    return problems


def self_test(texts):
    bad = 0
    n = 0
    for f, items in REQUIRED.items():
        for token, why in items:
            mutated = dict(texts)
            mutated[f] = texts[f].replace(token, "")
            n += 1
            if not check(mutated):
                print(f"FAIL  twin: removing `{token}` from {f} was NOT caught")
                bad += 1
    for f, items in FORBIDDEN.items():
        for pat, sample, why in items:
            mutated = dict(texts)
            mutated[f] = texts[f] + "\n" + sample + "\n"
            n += 1
            if not check(mutated):
                print(f"FAIL  twin: adding back /{pat}/ to {f} was NOT caught")
                bad += 1
    print(f"{n} twins, {bad} not caught")
    return bad


def main(argv):
    texts = read(list(REQUIRED))
    problems = check(texts)
    for p in problems:
        print("FAIL ", p)
    rc = 1 if problems else 0
    if "--self-test" in argv:
        rc = 1 if self_test(texts) else rc
    print("module wiring: " + ("FAILED" if rc else "ALL OK"))
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
