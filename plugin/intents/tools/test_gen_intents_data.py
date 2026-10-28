"""Game-free test of the Dead Money half of plugin/intents/tools/gen_intents_data.py (W3d item 4, CAMPAIGN-GAPS G35) with mutation twins.

Dead Money strips the player at the Sierra Madre entry (`player.RemoveAllItems NVDLC01PlayerEquipmentContainerRef`, NVDLC01VillaTravelTriggerScript), hands the gear back
at the end (`NVDLC01PlayerEquipmentContainerRef.RemoveAllItems Player`, NVDLC01ReturnEquipmentTriggerSCRIPT) and takes the weapons at the failure ending
(`Player.RemoveAllTypedItems NVDLC01PlayerEquipmentContainerRef 1 0 40`, NVDLC01MQ03Script). Every container a Dead Money script moves a player's gear through must
be in the stash list, or two players' gear is mixed in that container.

    python -I plugin/intents/tools/test_gen_intents_data.py       runs the checks, then the twins (each must make a check fail)
"""
import gzip
import importlib.util
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
spec = importlib.util.spec_from_file_location("gen_intents_data", HERE / "gen_intents_data.py")
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)

STRIP = "NVDLC01PlayerEquipmentContainerRef"


def script(edid, source):
    return {"edid": edid, "script": {"source": source}}


SCRIPTS = [
    script("NVDLC01VillaTravelTriggerScript", "BEGIN GameMode\n\tplayer.RemoveAllItems %s\n\t; player.RemoveAllItems NVDLC01CommentedOutREF\nEND\n" % STRIP),
    script("NVDLC01ReturnEquipmentTriggerSCRIPT", "BEGIN OnTrigger Player\n\t%s.RemoveAllItems Player\nEND\n" % STRIP),
    script("NVDLC01MQ03Script", "BEGIN GameMode\n\tPlayer.RemoveAllTypedItems %s 1 0 40;\nEND\n" % STRIP),
    script("NVDLC01CompanionGearScript", "BEGIN GameMode\n\tNVDLC01DeanREF.RemoveAllTypedItems player 0 1 24 NVDLC01DeanGear\nEND\n"),   # a companion's gear into the player: no container
    script("NVDLC02FireTriggerSCRIPT", "BEGIN GameMode\n\tNVDLC02JoshuaGearTransferBoxREF.RemoveAllItems Player\nEND\n"),                         # another DLC: not this check
]


def problems(listed=None, prefix="NVDLC01", scripts=None):
    out = []
    scripts = SCRIPTS if scripts is None else scripts
    found = gen.gear_containers(scripts, prefix)
    if set(found) != {STRIP}:
        out.append("gear_containers found %r, wanted just %s" % (sorted(found), STRIP))
    elif sorted(found[STRIP]) != ["NVDLC01MQ03Script", "NVDLC01ReturnEquipmentTriggerSCRIPT", "NVDLC01VillaTravelTriggerScript"]:
        out.append("the strip, the return and the failure ending are not all seen: %r" % (found[STRIP],))
    left = gen.uncovered(scripts, prefix, listed)
    if left:
        out.append("containers the stash list lacks: %s" % ", ".join(left))
    return out


def real_data_problems():
    out = []
    sp = REPO / "out" / "questdata" / "scripts.json.gz"
    if sp.is_file():
        with gzip.open(sp, "rt", encoding="utf-8") as f:
            scripts = json.load(f)
        found = gen.gear_containers(scripts, "NVDLC01")
        if STRIP not in found or len(found[STRIP]) < 3:
            out.append("the real Dead Money scripts: %r" % (found,))
        left = gen.uncovered(scripts)
        if left:
            out.append("the real Dead Money scripts move gear through containers that are not stashes: %s" % ", ".join(left))
    data = json.loads((REPO / "plugin" / "intents" / "data" / "nvcoop_intents.json").read_text(encoding="utf-8"))
    st = {s["name"]: s["target"] for s in data["stashes"]}
    if st.get(STRIP) != "deadmoney.esm:00efcc":
        out.append("nvcoop_intents.json does not carry %s (deadmoney.esm:00efcc): %r" % (STRIP, st.get(STRIP)))
    return out


# ---- the stand-in retry list (follow-ups 1, item 2) ----
NL = chr(10)
def obj(edid, ref, source):
    return {"edid": edid, "ref": "FalloutNV.esm:" + ref, "plugin": "FalloutNV.esm", "script": {"type": "object", "source": source}}


PLANT = "scn BrocFlowerScript" + NL + "int State" + NL + "begin onActivate" + NL + "	if State == 0 && GetActionRef == player" + NL + "		player.additem BrocFlower 1" + NL + "		set State to 1" + NL + "		playGroup Backward 1" + NL + "		setdestroyed 1" + NL + "	endif" + NL + "end" + NL + "begin onLoad" + NL + "	if State == 0" + NL + "		playGroup Forward 1" + NL + "	endif" + NL + "end" + NL
MACHINE = "scn VendScript" + NL + "begin OnActivate" + NL + "	if IsActionRef player" + NL + "		ShowMessage VendMsg" + NL + "	endif" + NL + "end" + NL
VALVE = "scn ValveScript" + NL + "begin OnActivate" + NL + "	if IsActionRef player == 1" + NL + "		set repair to 1" + NL + "	endif" + NL + "end" + NL + "begin GameMode" + NL + "	set b to getbuttonpressed" + NL + "end" + NL
NPCONLY = "scn AnyoneScript" + NL + "begin OnActivate" + NL + "	player.additem Junk 1" + NL + "end" + NL
SWITCH = "scn SwitchScript" + NL + "begin OnActivate" + NL + "	if GetActionRef == Player" + NL + "		activate GeneratorREF" + NL + "	endif" + NL + "end" + NL
RETRY_SCRIPTS = [obj("BrocFlowerScript", "0E9ACE", PLANT), obj("VendScript", "000011", MACHINE), obj("ValveScript", "000012", VALVE), obj("AnyoneScript", "000013", NPCONLY),
                 obj("SwitchScript", "000014", SWITCH)]


def retry_problems(scripts=None):
    out = []
    got = gen.retry_scripts(RETRY_SCRIPTS if scripts is None else scripts)
    if got != [("BrocFlowerScript", "falloutnv.esm:0e9ace")]:
        out.append("retry_scripts gave %r, wanted just the harvest plant" % (got,))
    return out


def real_retry_problems():
    out = []
    data = json.loads((REPO / "plugin" / "intents" / "data" / "nvcoop_intents.json").read_text(encoding="utf-8"))
    names = {r["name"]: r["script"] for r in data.get("retryScripts", [])}
    for want in ("BrocFlowerScript", "XanderRootScript"):
        if want not in names:
            out.append("nvcoop_intents.json retryScripts lacks %s" % want)
    sp = REPO / "out" / "questdata" / "scripts.json.gz"
    if sp.is_file():
        with gzip.open(sp, "rt", encoding="utf-8") as f:
            scripts = json.load(f)
        by_edid = {s["edid"]: s for s in scripts}
        for name in names:
            src = by_edid[name]["script"]["source"].lower()
            for bad in ("showmessage", "startconversation", "gamemode", "menumode", "moveto ", ".moveto"):
                if bad in src.replace(NL + "begin onactivate", ""):
                    out.append("the real retry script %s uses %s" % (name, bad))
    return out


def with_patch(name, value, fn):
    old = getattr(gen, name)
    setattr(gen, name, value)
    try:
        return fn()
    finally:
        setattr(gen, name, old)


def main():
    bad = problems() + real_data_problems() + retry_problems() + real_retry_problems()
    for p in bad:
        print("FAIL ", p)
    if bad:
        return 1
    twins = {
        "strip container left out of the list": lambda: problems(listed=[c for c in gen.CONTAINERS if c[0] != STRIP]),
        "return trigger form not read (X.RemoveAllItems Player)": lambda: problems(scripts=[s for s in SCRIPTS if s["edid"] != "NVDLC01ReturnEquipmentTriggerSCRIPT"]),
        "typed strip not read (RemoveAllTypedItems)": lambda: problems(scripts=[s for s in SCRIPTS if s["edid"] != "NVDLC01MQ03Script"]),
        "another DLC's script counted": lambda: problems(prefix="NVDLC0"),
        "retry: every command is accepted (a vending machine's message box)": lambda: with_patch("SAFE_COMMANDS", frozenset(["showmessage", "activate"]) | gen.SAFE_COMMANDS, retry_problems),
        "retry: GameMode blocks are accepted (a valve waiting for a message box)": lambda: with_patch("_BLOCK_OK", ("onactivate", "onload", "onreset", "gamemode"), retry_problems),
        "retry: the player test is dropped (any OnActivate with AddItem)": lambda: with_patch("_PLAYER_TEST", __import__("re").compile("."), retry_problems),
    }
    for name, f in twins.items():
        if f():
            print("ok    twin", name, "caught")
        else:
            print("FAIL  twin", name, "NOT caught")
            return 1
    print("gen_intents_data (Dead Money strip containers, stand-in retry list): ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
