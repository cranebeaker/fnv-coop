"""Game-free test of plugin/coop/tools/gen_coop_data.py (the data files next to nvcoop.dll) with a mutation twin.

    python -I plugin/coop/tests/test_gen_coop_data.py          runs the checks, then the twins (each must make a check fail)
"""
import importlib.util
import json
import re
import shutil
import sys
import tempfile
from pathlib import Path

NL = chr(10)
HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
sys.path.insert(0, str(REPO / "tools" / "questdata"))
sys.path.insert(0, str(REPO / "tools" / "questdata" / "tests"))
import synth  # noqa: E402  (synthetic ESM builders)
spec = importlib.util.spec_from_file_location("gen_coop_data", HERE.parent / "tools" / "gen_coop_data.py")
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)


def run_gen(out, data=None):
    old = sys.argv
    sys.argv = ["gen_coop_data.py", "--out", str(out)] + (["--data", str(data)] if data is not None else [])
    try:
        gen.main()
    finally:
        sys.argv = old


def fake_game(td):
    """A data folder with a FalloutNV.esm and a DeadMoney.esm: a compressed PERK, a CELL whose EDID differs in case from the rules' spelling, a
    DLC MESG, and a DLC override of a base-game perk (it must resolve to the base game's id, not the DLC's)."""
    s = synth
    base = s.tes4() + s.grup("PERK", [s.rec("PERK", 0x0015D2BE, [s.sub("EDID", s.z("OldVaquero"))], flags=0x40000),
                                      s.rec("PERK", 0x0015B8E8, [s.sub("EDID", s.z("RangerTakedown"))])])         + s.grup("SCPT", [s.rec("SCPT", 0x00131F99, [s.sub("EDID", s.z("FortArrivalTriggerScript"))])])         + s.grup("CELL", [s.rec("CELL", 0x00133880, [s.sub("EDID", s.z("HooverDamIntCheckPoint"))]),
                          s.rec("CELL", 0x00133881, [s.sub("EDID", s.z("Deleted01"))], flags=0x20)])
    dlc = s.tes4(masters=("FalloutNV.esm",)) + s.grup("PERK", [s.rec("PERK", 0x0015B8E8, [s.sub("EDID", s.z("RangerTakedown"))])])         + s.grup("MESG", [s.rec("MESG", 0x01009C99, [s.sub("EDID", s.z("NVDLC04TempleCaesarConfirmMSG")), s.sub("ITXT", s.z("No.")), s.sub("ITXT", s.z("Yes."))])])
    (Path(td) / "FalloutNV.esm").write_bytes(base)
    (Path(td) / "DeadMoney.esm").write_bytes(dlc)
    return Path(td)


def check_esm(mutation=None):
    """The EDIDs the rules name are read from the masters (item 1 of W3a): returns a list of problems."""
    problems = []
    real_sweep, real_find = gen.esm_edids.sweep, gen.esm_edids.find
    if mutation == "esm_case_sensitive":
        # the engine ignores the case of an EDID; a sweep that does not finds nothing for the cell the rules spell `...Checkpoint`
        gen.esm_edids.sweep = lambda path, wanted: [r for r in real_sweep(path, wanted) if r[0] == wanted[r[0].lower()]]
    if mutation == "esm_ignored":
        gen.esm_edids.find = lambda data_dir, names, plugins=None: ({}, {})
    try:
        with tempfile.TemporaryDirectory() as g, tempfile.TemporaryDirectory() as out:
            data = fake_game(g)
            run_gen(out, data)
            edids = json.loads((Path(out) / "nvcoop_edids.json").read_text(encoding="utf-8"))
            missing = (Path(out) / "nvcoop_edids_missing.txt").read_text(encoding="utf-8").split()
            want = {"OldVaquero": "falloutnv.esm:15d2be",                      # a compressed record
                    "HooverDamIntCheckpoint": "falloutnv.esm:133880",         # matched without regard to case, keyed by the rules' spelling
                    "RangerTakedown": "falloutnv.esm:15b8e8",                 # the DLC override resolves to the defining plugin
                    "NVDLC04TempleCaesarConfirmMSG": "deadmoney.esm:009c99",  # a form a DLC defines
                    "FortArrivalTriggerScript": "falloutnv.esm:131f99",       # a view-party script of companion_sets.txt: the session names a running script by it (W3a item 4)
                    "NVDLC01Followers": "deadmoney.esm:0012fd"}               # the follower quests of plugin/dialogue/data/follower_rules.txt need no ESM
            for k, v in want.items():
                if edids.get(k) != v:
                    problems.append("EDID %s resolves to %r, wanted %s" % (k, edids.get(k), v))
                if k in missing:
                    problems.append("EDID %s is resolved and still listed as missing" % k)
            if gen.esm_edids.mesg_buttons(g, "DeadMoney.esm:009C99") != ["No.", "Yes."] or gen.esm_edids.mesg_buttons(g, "DeadMoney.esm:000001") is not None:
                problems.append("mesg_buttons does not return the ITXT texts in order (or invents a box)")
            if "Deleted01" in edids:
                problems.append("a deleted record's EDID was taken")
            if "OldWorldBlues" in missing or "TheFortSum" not in missing:
                problems.append("a name that is not in the (fake) masters must stay in the missing list")
            # cause_quests.json "keys" are quest name -> ref: no ESM needed
            for k in ("NVDLC03ThinkTankWeaponLock", "NVDLC04WeakPlayerScaling"):
                if k not in edids or k in missing:
                    problems.append("%s (a cause_quests key) is not resolved" % k)
        # no data folder: nothing is read, nothing breaks
        with tempfile.TemporaryDirectory() as empty, tempfile.TemporaryDirectory() as out:
            run_gen(out, empty)
            edids = json.loads((Path(out) / "nvcoop_edids.json").read_text(encoding="utf-8"))
            if "OldVaquero" in edids:
                problems.append("an EDID appeared from an empty data folder")
    finally:
        gen.esm_edids.sweep = real_sweep
        gen.esm_edids.find = real_find
    # the real thing, when the read-only game copy is on this machine: every name the rules use resolves
    real = gen.esm_edids.default_data_dir(gen.REPO) if mutation is None else None
    if real:
        with tempfile.TemporaryDirectory() as out:
            run_gen(out, real)
            left = (Path(out) / "nvcoop_edids_missing.txt").read_text(encoding="utf-8").split()
            # W1B assumed first = Yes, last = No; the game data says otherwise: the three Temple boxes are No. then Yes.
            for box in ("NVDLC04TempleCaesarConfirmMSG", "NVDLC04TempleNCRConfirmMSG", "NVDLC04TempleNukeAllConfirmMSG"):
                ed = json.loads((Path(out) / "nvcoop_edids.json").read_text(encoding="utf-8")).get(box)
                if not ed or gen.esm_edids.mesg_buttons(real, ed) != ["No.", "Yes."]:
                    problems.append("%s buttons are %r, the hooks_ui consent logic is written for ['No.', 'Yes.']" % (box, ed and gen.esm_edids.mesg_buttons(real, ed)))
            if left:
                problems.append("with the game's masters %d names are still unresolved: %s" % (len(left), ", ".join(left[:8])))
    return problems


def check_info_gate():
    """W3d item 2: the INFOs whose conditions the rules reduce over the party get their quest into nvcoop_infoquest.json (the hook that opens an INFO's condition scope)."""
    problems = []
    rules = {"quests": {"Jed": {"rules": [{"kind": "PartyAwareCond", "params": {"funcs": {"GetAV": "max-over-party", "HasPerk": "initiator"}}}]},
                        "Plain": {"rules": [{"kind": "SetPlayerControls", "params": {}}]}}}
    pac = gen.party_cond_funcs(rules)
    if pac != {"Jed": {"getactorvalue", "hasperk"}}:
        problems.append("party_cond_funcs read %r" % (pac,))
    infos = [{"ref": "HonestHearts.esm:009EB6", "quest": "HonestHearts.esm:009403", "conditions": [{"func": "GetIsID"}, {"func": "GetActorValue"}]},   # the gate (an end script would not matter)
             {"ref": "HonestHearts.esm:009EB7", "quest": "HonestHearts.esm:009403", "conditions": [{"func": "GetIsID"}]},                            # no function the rules reduce
             {"ref": "HonestHearts.esm:009EB8", "quest": "HonestHearts.esm:009403", "conditions": [{"func": "GetActorValue"}], "end": {"source": "x"}},   # has a script: the old rule maps it
             {"ref": "HonestHearts.esm:009EB9", "quest": "HonestHearts.esm:009404", "conditions": [{"func": "GetActorValue"}]},                     # a quest without the rule
             {"ref": "HonestHearts.esm:009EBA", "conditions": [{"func": "GetActorValue"}]}]                                                          # no quest
    got = gen.gate_info_quest(infos, {"honesthearts.esm:009403": "Jed", "honesthearts.esm:009404": "Plain"}, pac)
    if got != {"honesthearts.esm:009eb6": "honesthearts.esm:009403"}:
        problems.append("gate_info_quest picked %r" % (got,))
    # the shipped data: the Jed gate INFOs (009EB6 / 011525) and the Shaman's root count are in the map the module loads
    if (gen.QD / "infos.jsonl.gz").is_file():
        with tempfile.TemporaryDirectory() as out, tempfile.TemporaryDirectory() as no_esm:
            run_gen(out, no_esm)   # the INFO map needs out/questdata only; no ESM sweep
            iq = json.loads((Path(out) / "nvcoop_infoquest.json").read_text(encoding="utf-8"))
            for info in ("honesthearts.esm:009eb6", "honesthearts.esm:011525", "honesthearts.esm:0094aa"):
                if info not in iq:
                    problems.append("%s (a Jed / Shaman INFO) has no quest in nvcoop_infoquest.json" % info)
            if len(iq) < 7000:
                problems.append("nvcoop_infoquest.json shrank to %d entries" % len(iq))
    return problems


def check_pack_watch():
    """Guest pack facts: the watch list (nvcoop_pack_watch.json) is the item forms the rules and the quest conditions count, not the whole game."""
    problems = []
    rules = {"quests": {"Shaman": {"rules": [{"kind": "PooledItems", "params": {"items": ["HonestHearts.esm:00A84A", "TotemName"]}}, {"kind": "PartyAwareCond", "params": {"funcs": {"GetItemCount": "sum-over-party"}}}]},
                        "Other": {"rules": [{"kind": "SetPlayerControls", "params": {"items": ["NotAnItemRule"]}}]}}}
    infos = [{"ref": "FalloutNV.esm:000001", "conditions": [{"func": "GetItemCount", "p1": "FalloutNV.esm:0132AB", "op": ">=", "value": 1},
                                                              {"func": "GetActorValue", "p1": "FalloutNV.esm:000046"},       # not an item count
                                                              {"func": "GetItemCount", "p1": 5}]},                             # not a form
             {"ref": "FalloutNV.esm:000002", "conditions": [], "end": {"source": "player.RemoveItem Psycho 1" + NL + "if Player.GetItemCount Jet >= 2" + NL + "endif"}},   # RemoveItem is not a count; Jet is
             {"ref": "FalloutNV.esm:000003", "conditions": [], "begin": {"source": "set x to GetItemCount rSomeVar"}}]
    quests = [{"edid": "Q", "conditions": [{"func": "GetItemCount", "p1": "DeadMoney.esm:00ABCD"}], "stages": [{"conditions": [{"func": "GetItemCount", "p1": "FalloutNV.esm:0132AB"}]}],
               "script_data": {"source": "scn Q" + NL + "if player.getitemcount PsychoMangled > 0" + NL + "endif"}}]
    refs, names = gen.pack_watch_sources(rules, infos, quests)
    if refs != {"honesthearts.esm:00a84a", "falloutnv.esm:0132ab", "deadmoney.esm:00abcd"}:
        problems.append("pack_watch_sources refs: %r" % (sorted(refs),))
    if names != {"TotemName", "Jet", "rSomeVar", "PsychoMangled"}:
        problems.append("pack_watch_sources names: %r" % (sorted(names),))
    edids = {"totemname": "HonestHearts.esm:00F85D", "Psycho": "FalloutNV.esm:0091A6", "JET": "FalloutNV.esm:0091A7", "PsychoMangled": "FalloutNV.esm:0091A8"}
    watch, missing = gen.pack_watch(refs, names, edids)
    if watch != ["deadmoney.esm:00abcd", "falloutnv.esm:0091a7", "falloutnv.esm:0091a8", "falloutnv.esm:0132ab", "honesthearts.esm:00a84a", "honesthearts.esm:00f85d"]:
        problems.append("pack_watch list: %r" % (watch,))
    if missing != ["rSomeVar"]:
        problems.append("pack_watch missing: %r (a script variable is not an item)" % (missing,))
    # the shipped data: the Honest Hearts hand-in items (rules.json PooledItems) and the conditions of the game are in the file the module loads
    if (gen.QD / "infos.jsonl.gz").is_file():
        with tempfile.TemporaryDirectory() as out:
            real = gen.esm_edids.default_data_dir(gen.REPO)
            run_gen(out, real if real else tempfile.gettempdir() + "/no_such_data")
            doc = json.loads((Path(out) / "nvcoop_pack_watch.json").read_text(encoding="utf-8"))
            items = set(doc.get("items", []))
            if doc.get("schema") != 1:
                problems.append("nvcoop_pack_watch.json has no schema 1")
            for it in ("honesthearts.esm:00a84a", "honesthearts.esm:00f85d", "honesthearts.esm:0094bb", "honesthearts.esm:0093c4"):   # totems, lunchboxes, roots (PooledItems)
                if it not in items:
                    problems.append("%s (a pooled hand-in item) is not in nvcoop_pack_watch.json" % it)
            if not 100 <= len(items) <= 600:
                problems.append("the watch list has %d forms (a list of the conditions is a few hundred, not the whole game)" % len(items))
            if real and (Path(out) / "nvcoop_pack_watch_missing.txt").read_text(encoding="utf-8").split():
                problems.append("item names of scripts are unresolved with the game's masters: %s" % (Path(out) / "nvcoop_pack_watch_missing.txt").read_text(encoding="utf-8").split()[:6])
    return problems


def check(mutation=None):
    """Returns a list of problems."""
    problems = []
    real_cond, real_re = gen._condition_items, gen.GET_ITEM_COUNT
    if mutation == "watch_conditions_ignored":
        gen._condition_items = lambda o: iter(())   # twin: the item counts of the conditions are not on the list
    if mutation == "watch_scripts_ignored":
        gen.GET_ITEM_COUNT = re.compile(r"(?!x)x")   # twin: the item names the scripts count are not on the list
    try:
        problems += check_pack_watch()
    finally:
        gen._condition_items, gen.GET_ITEM_COUNT = real_cond, real_re
    if mutation in ("watch_conditions_ignored", "watch_scripts_ignored"):
        return problems
    real_gate = gen.gate_info_quest
    if mutation == "info_gate_ignored":
        gen.gate_info_quest = lambda infos, quest_edid, pac: {}   # twin: no INFO gets a quest for its conditions
    try:
        problems += check_info_gate()
    finally:
        gen.gate_info_quest = real_gate
    if mutation == "info_gate_ignored":
        return problems
    real_copy = shutil.copy2
    if mutation == "no_cause_copy":
        shutil.copy2 = lambda src, dst, *a, **k: None if Path(src).name == "cause_quests.json" else real_copy(src, dst, *a, **k)
    try:
        with tempfile.TemporaryDirectory() as td:
            out = Path(td)
            run_gen(out)
            cq = out / "nvcoop_cause_quests.json"
            if not cq.is_file():
                problems.append("nvcoop_cause_quests.json was not written (G02: the module loads it next to the rules)")
            else:
                data = json.loads(cq.read_text(encoding="utf-8"))
                if not data.get("scripts"):
                    problems.append("the cause map has no scripts")
            missing = (out / "nvcoop_edids_missing.txt").read_text(encoding="utf-8").split()
            edids = json.loads((out / "nvcoop_edids.json").read_text(encoding="utf-8"))
            if any(n in edids for n in missing):
                problems.append("a name is listed as missing and resolved at once")
    finally:
        shutil.copy2 = real_copy
    if mutation in ("esm_case_sensitive", "esm_ignored"):
        return check_esm(mutation)
    problems += check_esm(mutation)
    # the pure function: a name that nothing resolves is reported, a plain Plugin.esm:HEX id is not
    names = gen.unresolved_names({})
    if "OldVaquero" not in names and "RangerTakedown" not in names:
        problems.append("unresolved_names does not report the perk EDIDs the rules use")
    return problems


def main():
    bad = check()
    for p in bad:
        print("FAIL ", p)
    if bad:
        return 1
    caught = 0
    for m in ("no_cause_copy", "esm_case_sensitive", "esm_ignored", "info_gate_ignored", "watch_conditions_ignored", "watch_scripts_ignored"):
        if check(m):
            caught += 1
            print("ok    twin", m, "caught")
        else:
            print("FAIL  twin", m, "NOT caught")
            return 1
    print("gen_coop_data: ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
