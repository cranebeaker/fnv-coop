#!/usr/bin/env python3
"""Writes plugin/intents/data/nvcoop_intents.json (the shared gear containers that keep one stash per player, CAMPAIGN-GAPS G35).

The container list is curated here by EditorID; the form ids come from the game's own data (nvcoop_edids.json, built from the ESMs by
the L10 data step), never typed by hand, so a wrong id cannot get in. `--check` regenerates in memory and compares with the file on disk
(exit 1 on a difference, exit 0 and a note when the EditorID map is not at hand).

    python -I plugin/intents/tools/gen_intents_data.py [--edids out/L10/data/nvcoop_edids.json] [--out plugin/intents/data/nvcoop_intents.json]
    python -I plugin/intents/tools/gen_intents_data.py --check
"""
from __future__ import annotations

import argparse
import gzip
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]

# (EditorID of the container reference, what keeps its gear there). Only containers a script NAMES are here (the EditorID map is built from script
# references): the Fort, Dead Money and the barge. The friskings (Tops, Ultra-Luxe, Silver Rush, Gomorrah) and the Honest Hearts gear boxes are not named by any
# script in the data; add them here by EditorID when a scan finds the script that fills them.
CONTAINERS = [
    ("NVCCPlayerStorageREF", "Caesar's Fort: barge gate confiscation and return"),
    ("FortHumanoidCompanionContainer", "Caesar's Fort: humanoid companion gear"),
    ("FortCritterCompanionContainer", "Caesar's Fort: creature companion gear"),
    ("FortPlayerEquipmentContainerRef", "Caesar's Fort: pit fight gear"),
    ("NVDLC01PlayerEquipmentContainerRef", "Dead Money: the entry strip, the return trigger and the failure ending (RemoveAllTypedItems)"),
]


# `player.RemoveAllItems X`, `player.RemoveAllTypedItems X ...` (gear INTO the container X) and `X.RemoveAllItems player` (gear back OUT): the container a script names.
_GEAR = re.compile(r"^\s*(?:(?P<who>\w+)\.)?RemoveAll(?:Typed)?Items\s+(?P<arg>\w+)", re.I)


def gear_containers(scripts, prefix="NVDLC01"):
    """{container EditorID: [script EditorIDs]} for every line of the scripts whose EditorID starts with `prefix` that moves a PLAYER's gear to or from a named container
    (W3d item 4: the Dead Money strip, its return trigger and its failure ending). `scripts` are the dicts of out/questdata/scripts.json.gz."""
    out = {}
    for s in scripts:
        edid = s.get("edid", "")
        if not edid.lower().startswith(prefix.lower()):
            continue
        src = s.get("script", {}).get("source", "") if isinstance(s.get("script"), dict) else ""
        for line in src.splitlines():
            m = _GEAR.match(line.split(";", 1)[0])
            if not m:
                continue
            who, arg = (m.group("who") or "").lower(), m.group("arg")
            typed = "typed" in line.lower().split("removeall", 1)[1][:5]
            # player -> X (either command), X -> player (RemoveAllItems only: `NPC.RemoveAllTypedItems player` hands a companion's gear over, it is no gear stash)
            container = arg if who == "player" else (m.group("who") or "") if arg.lower() == "player" and not typed else ""
            if container and container.lower() != "player":
                out.setdefault(container, []).append(edid)
    return out


def uncovered(scripts, prefix="NVDLC01", listed=None):
    """The containers the scripts of `prefix` move a player's gear through that CONTAINERS (the stash list) does not name."""
    names = {n.lower() for n, _ in (CONTAINERS if listed is None else listed)}
    return sorted(c for c in gear_containers(scripts, prefix) if c.lower() not in names)


# ---- the stand-in retry list (plugin/intents/retry_list.h) ----------------------------------------------------------------------------------------------
# An activator the guest's puppet activated to no effect is redone once by the host's own player (the Broc flower says `GetActionRef == player`). A second run of an arbitrary
# OnActivate would pop message boxes on the host and throw switches twice, so only scripts that can be run that way are listed:
#   1. an OnActivate block tests `GetActionRef == player` / `IsActionRef player`;
#   2. no block but OnActivate / OnLoad / OnReset (a GameMode / MenuMode block is where a script waits for a message box or a menu);
#   3. every statement of OnActivate is a command the capture records or that touches only the world (SAFE_COMMANDS).
_PLAYER_TEST = re.compile(r"(getactionref\s*==\s*player\b|\bplayer\s*==\s*getactionref|isactionref\s+player\b|getactionref\s*==\s*playerref)", re.I)
NL = chr(10)
_BLOCK_OK = ("onactivate", "onload", "onreset")
SAFE_COMMANDS = frozenset((
    "if", "elseif", "else", "endif", "set", "let", "return", "additem", "setdestroyed", "playgroup", "rewardxp", "rewardkarma", "disable", "enable", "setstage", "startquest",
    "setobjectivedisplayed", "setobjectivecompleted", "additemhealthpercent", "addnote", "removenote", "completequest", "stopquest", "addreputation"))


def _blocks(source):
    """[(block name lower-case, [statement lines without comments])] of a script's source."""
    out, cur = [], None
    for line in source.replace(chr(13), "").split(chr(10)):
        text = line.split(";", 1)[0].strip()
        if not text:
            continue
        m = re.match(r"begin\s+(\w+)", text, re.I)
        if m:
            cur = [m.group(1).lower(), []]
        elif re.match(r"end\s*$", text, re.I):
            if cur:
                out.append((cur[0], cur[1]))
            cur = None
        elif cur:
            cur[1].append(text)
    return out


def retry_scripts(scripts):
    """[(script EditorID, 'plugin.esm:00ABCD')] of the object scripts a puppet's activation may be redone for (rules above). `scripts` are the dicts of scripts.json.gz."""
    out = []
    for s in scripts:
        sc = s.get("script")
        if not isinstance(sc, dict) or sc.get("type", "object") != "object":
            continue
        blocks = _blocks(sc.get("source", ""))
        act = [b for b in blocks if b[0] == "onactivate"]
        if not act or any(b[0] not in _BLOCK_OK for b in blocks):
            continue
        if not _PLAYER_TEST.search(NL.join(NL.join(b[1]) for b in act)):
            continue
        if any((re.match(r"(?:\w+\.)?(\w+)", line) or [None, ""])[1].lower() not in SAFE_COMMANDS for b in act for line in b[1]):
            continue
        ref = s.get("ref") or ""
        if ":" not in ref:
            continue
        plugin, fid = ref.rsplit(":", 1)
        out.append((s.get("edid", ""), "%s:%s" % (plugin.lower(), fid.lower().zfill(6))))
    return sorted(out, key=lambda x: x[1])


def build(edids: dict, scripts=None) -> dict:
    out = []
    for name, what in CONTAINERS:
        ref = edids.get(name)
        if not ref:
            raise SystemExit(f"gen_intents_data: EditorID {name} is not in the EditorID map")
        out.append({"name": name, "target": ref, "what": what})
    return {
        "_comment": "Generated by plugin/intents/tools/gen_intents_data.py from the game data. 'redirects' (a container per slot, needs a plugin that adds them) "
                    "is empty; 'stashes' are shared containers whose contents the host swaps per player (plugin/intents/storage_book.h); "
                    "'retryScripts' are the object scripts a puppet's activation to no effect may be redone for by the host's own player (plugin/intents/retry_list.h).",
        "redirects": [],
        "stashes": out,
        "retryScripts": [{"script": ref, "name": name} for name, ref in retry_scripts(scripts or [])],
    }


def render(doc: dict) -> str:
    return json.dumps(doc, indent=1) + "\n"


def main(argv=None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--edids", default=str(REPO / "out/L10/data/nvcoop_edids.json"))
    ap.add_argument("--out", default=str(REPO / "plugin/intents/data/nvcoop_intents.json"))
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args(argv)
    ed = Path(a.edids)
    if not ed.exists():
        print(f"gen_intents_data: {ed} not found: data not regenerated" + (" (check skipped)" if a.check else ""))
        return 0 if a.check else 1
    sp = REPO / "out/questdata/scripts.json.gz"
    if not sp.exists():
        print(f"gen_intents_data: {sp} not found: data not regenerated" + (" (check skipped)" if a.check else ""))
        return 0 if a.check else 1
    with gzip.open(sp, "rt", encoding="utf-8") as f:
        scripts = json.load(f)
    text = render(build(json.loads(ed.read_text(encoding="utf-8")), scripts))
    out = Path(a.out)
    if a.check:
        have = out.read_text(encoding="utf-8").replace("\r\n", "\n") if out.exists() else ""
        if have != text:
            print(f"gen_intents_data: {out} differs from what the game data gives; run without --check")
            return 1
        left = uncovered(scripts)   # W3d item 4: every Dead Money script that strips or returns a player's gear names a container the list has
        if left:
            print(f"gen_intents_data: Dead Money scripts move a player's gear through containers that are not in CONTAINERS: {', '.join(left)}")
            return 1
        print(f"gen_intents_data: {out.name} matches the game data ({len(CONTAINERS)} containers, {len(retry_scripts(scripts))} retry scripts)")
        return 0
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8", newline="\n")
    print(f"gen_intents_data: wrote {out} ({len(CONTAINERS)} containers, {len(retry_scripts(scripts))} retry scripts)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
