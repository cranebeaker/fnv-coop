#!/usr/bin/env python3
"""Generate the data files the coop module loads from the folder of nvcoop.dll:
  nvcoop_edids.json      {"VCG01": "falloutnv.esm:0xxxxx", ...}   quests (all scenarios) + every form with an EDID in forms.json (cells, ...)
  nvcoop_infoquest.json  {"<info ref>": "<quest ref>"}            INFOs that run a begin/end result script (cause.source -> owning quest), and (W3d) INFOs whose conditions call a
                         function the quest's PartyAwareCond rule reduces over the party: the hook that evaluates an INFO's conditions reduces them under that quest
  nvcoop_rules.json      copy of docs/quest-policies/rules.json
  nvcoop_cause_quests.json  copy of docs/quest-policies/cause_quests.json (G02: script / INFO / ref -> the quest whose rules apply)
  nvcoop_pack_watch.json    {"schema":1,"items":[refs]}            guest pack facts: the item forms the rules and the quest conditions count (PooledItems items, GetItemCount in INFO / quest
                            conditions and in the scripts of INFOs, quests and stages). A guest tells the host its InventoryWeight and the counts of exactly these forms, not its whole pack.
                            Names the masters cannot resolve are listed in nvcoop_pack_watch_missing.txt (not part of the release).
Rules name forms by EDID (perks, message boxes, markers, worldspaces, NPCs). out/questdata only knows some of them (quests, cells, the 43 PERK forms the
quest scripts touch); the rest are read from the game's ESM files (tools/questdata/edids.py, read-only, case-insensitive like the engine) when the
data folder is found: `--data DIR`, else $FNVCOOP_GAME_DATA, else games/fnv-instances/0/Data beside the repo. cause_quests.json "keys" (a quest name and
its ref) are merged as well. Without the ESMs nothing changes: names that are still unresolved after the merge are listed in nvcoop_edids_missing.txt
(not part of the release) and counted in the summary line. {"EDID": "plugin.esm:hex", ...} in plugin/coop/tools/extra_edids.json is merged last and
wins (committed override; empty by default, nothing is pasted by hand).
Refs are lower-cased `plugin:HEX` (6 digits), the same text form as nvhooks::refToStr. Output dir: out/L10/data (default).
    python -I plugin/coop/tools/gen_coop_data.py [--out DIR]
"""
import argparse, gzip, json, re, shutil, sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
QD = REPO / "out" / "questdata"
sys.path.insert(0, str(REPO / "tools" / "questdata"))
import edids as esm_edids  # noqa: E402  (tools/questdata/edids.py)


def norm(ref):
    plugin, local = ref.rsplit(":", 1)
    return "%s:%06x" % (plugin.lower(), int(local, 16))


NL = chr(10)
REF = re.compile(r"^[A-Za-z0-9_ ]+\.es[mp]:[0-9A-Fa-f]+$")


def companion_script_names():
    """The scripts that read the companion hired flags party-wide (plugin/companions/data/companion_sets.txt `view-party <ScriptEDID>`): the session names a running script by
    its form id (nvhooks::setScriptRunScope) and needs these in nvcoop_edids.json (W3a item 4)."""
    p = REPO / "plugin" / "companions" / "data" / "companion_sets.txt"
    names = []
    if p.is_file():
        for line in p.read_text(encoding="utf-8").splitlines():
            parts = line.split()
            if len(parts) == 2 and parts[0] == "view-party":
                names.append(parts[1])
    return names


def follower_quests():
    """{EDID: ref} of the follower quests the dialogue registry knows (plugin/dialogue/data/follower_rules.txt `quest <ref> <edid>`): NVDLC01Followers carries the
    Dead Money rCurrentFollower variable the session writes per player."""
    p = REPO / "plugin" / "dialogue" / "data" / "follower_rules.txt"
    out = {}
    if p.is_file():
        for line in p.read_text(encoding="utf-8").splitlines():
            parts = line.split()
            if len(parts) == 3 and parts[0] == "quest" and REF.match(parts[1]):
                out[parts[2]] = parts[1]
    return out


def party_cond_funcs(rules):
    """{quest EDID: set of lower-case condition functions its PartyAwareCond rule reduces over the party} (rules.json; the rules spell GetActorValue both ways)."""
    out = {}
    for q, v in rules["quests"].items():
        for r in v["rules"]:
            if r["kind"] == "PartyAwareCond":
                out.setdefault(q, set()).update(("getactorvalue" if k.lower() == "getav" else k.lower()) for k in r.get("params", {}).get("funcs", {}))
    return out


def gate_info_quest(infos, quest_edid, pac):
    """W3d item 2: INFOs WITHOUT a result script whose conditions call a function the rules reduce over the party for their quest (the Jed weight gate reads
    InventoryWeight, the Shaman and the pooled hand-ins count items). The dialogue menu evaluates those conditions outside any script, so the hook that opens the
    INFO's condition scope (nvhooks::setInfoQuestScopes) needs their quest: {info ref: quest ref}, both normalised. `infos` are the dicts of infos.jsonl, `quest_edid`
    maps a normalised quest ref to its EDID."""
    out = {}
    for d in infos:
        if d.get("begin") or d.get("end") or not d.get("quest"):
            continue
        funcs = pac.get(quest_edid.get(norm(d["quest"])))
        if funcs and funcs & {c.get("func", "").lower() for c in d.get("conditions", [])}:
            out[norm(d["ref"])] = norm(d["quest"])
    return out


GET_ITEM_COUNT = re.compile(r"\bgetitemcount\s+([A-Za-z_][A-Za-z0-9_]*)", re.I)


def _strings(o):
    """Every string inside nested dicts / lists (script sources hide in several places of the quest data)."""
    if isinstance(o, str):
        yield o
    elif isinstance(o, dict):
        for v in o.values():
            yield from _strings(v)
    elif isinstance(o, list):
        for v in o:
            yield from _strings(v)


def _condition_items(o):
    """Item refs of the GetItemCount conditions anywhere inside `o` (a condition is a dict with func and p1)."""
    if isinstance(o, dict):
        if str(o.get("func", "")).lower() == "getitemcount" and isinstance(o.get("p1"), str) and REF.match(o["p1"]):
            yield norm(o["p1"])
        for v in o.values():
            yield from _condition_items(v)
    elif isinstance(o, list):
        for v in o:
            yield from _condition_items(v)


def pack_watch_sources(rules, infos, quests):
    """Guest pack facts: which item forms do the rules and the quest data count? Returns (refs, names): `refs` are normalised plugin:hex forms (PooledItems entries that are refs,
    the p1 of every GetItemCount condition of an INFO or a quest), `names` the EDIDs still to resolve (PooledItems entries that are names, and the argument of `GetItemCount <name>` in
    the scripts of INFOs, quests and stages). `infos` are the dicts of infos.jsonl, `quests` the quests of quests.json."""
    refs, names = set(), set()
    for v in rules["quests"].values():
        for r in v["rules"]:
            if r["kind"] != "PooledItems":
                continue
            for x in r.get("params", {}).get("items", []):
                if REF.match(x):
                    refs.add(norm(x))
                else:
                    names.add(x)
    for d in infos:
        refs.update(_condition_items(d.get("conditions", [])))
        for s in _strings([d.get("begin"), d.get("end")]):
            names.update(m.group(1) for m in GET_ITEM_COUNT.finditer(s))
    for q in quests:
        refs.update(_condition_items(q))
        for s in _strings(q):
            names.update(m.group(1) for m in GET_ITEM_COUNT.finditer(s))
    return refs, names


def pack_watch(refs, names, edids):
    """(sorted item refs, sorted names nobody resolved). `edids` is the EDID -> ref map; the game matches an EDID without regard to case, so the lookup does too."""
    low = {k.lower(): v for k, v in edids.items()}
    out, missing = set(refs), set()
    for n in names:
        r = low.get(n.lower())
        if r:
            out.add(norm(r))
        else:
            missing.add(n)
    return sorted(out), sorted(missing)


def unresolved_names(edids, extra=()):
    """EDIDs that rules.json / cause_quests.json (and `extra`) name and nvcoop_edids.json cannot resolve (plain `Plugin.esm:HEX` ids are fine)."""
    rules = json.loads((REPO / "docs" / "quest-policies" / "rules.json").read_text(encoding="utf-8"))
    cq = json.loads((REPO / "docs" / "quest-policies" / "cause_quests.json").read_text(encoding="utf-8"))
    names = set()
    for q, v in rules["quests"].items():
        names.add(q)
        for r in v["rules"]:
            p = r.get("params", {})
            for k in ("cells", "markers", "topics", "boxes", "actors", "infos", "npcs", "all", "initiator", "recruiter"):
                if isinstance(p.get(k), list):
                    names.update(x for x in p[k] if isinstance(x, str))
            for k in ("anchor", "anchor_ref", "default"):
                if isinstance(p.get(k), str):
                    names.add(p[k])
    names.update(cq.get("scripts", {}).values())
    names.update(cq.get("infos", {}).values())
    names.update(cq.get("refs", {}).values())
    names.update(cq.get("keys", {}).keys())
    names.update(extra)
    return sorted(n for n in names if not REF.match(n) and n not in edids)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(REPO / "out" / "L10" / "data"))
    ap.add_argument("--data", default=None, help="folder with FalloutNV.esm and the DLC masters (read-only); default $FNVCOOP_GAME_DATA or games/fnv-instances/0/Data")
    a = ap.parse_args()
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    edids = {}
    for p in (QD / "scenarios").glob("*.json"):
        try:
            d = json.loads(p.read_text(encoding="utf-8"))
        except Exception:
            continue
        if d.get("quest") and d.get("ref"):
            edids[d["quest"]] = norm(d["ref"])
    forms = json.load(gzip.open(QD / "forms.json.gz", "rt", encoding="utf-8"))
    for ref, v in forms.items():
        if v.get("edid"):
            edids.setdefault(v["edid"], norm(ref))
    info_quest = {}
    infos = []
    with gzip.open(QD / "infos.jsonl.gz", "rt", encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            infos.append(d)
            if (d.get("begin") or d.get("end")) and d.get("quest"):
                info_quest[norm(d["ref"])] = norm(d["quest"])
    quest_edid = {norm(ref): v["edid"] for ref, v in forms.items() if v.get("edid")}
    quests_doc = json.loads((QD / "quests.json").read_text(encoding="utf-8")).get("quests", []) if (QD / "quests.json").is_file() else []
    rules_doc = json.loads((REPO / "docs" / "quest-policies" / "rules.json").read_text(encoding="utf-8"))
    gate = gate_info_quest(infos, quest_edid, party_cond_funcs(rules_doc))
    for k, v in gate.items():
        info_quest.setdefault(k, v)
    for k, v in json.loads((REPO / "docs" / "quest-policies" / "cause_quests.json").read_text(encoding="utf-8")).get("keys", {}).items():
        edids.setdefault(k, norm(v))
    for k, v in follower_quests().items():
        edids.setdefault(k, norm(v))
    data_dir = a.data or esm_edids.default_data_dir(REPO)
    watch_refs, watch_names = pack_watch_sources(rules_doc, infos, quests_doc)
    esm_found = esm_ambiguous = {}
    if data_dir:
        # one sweep of the masters for both lists: the names the rules use and the item names the scripts count
        ask = set(unresolved_names(edids, companion_script_names())) | {n for n in watch_names if n.lower() not in {k.lower() for k in edids}}
        esm_found, esm_ambiguous = esm_edids.find(data_dir, sorted(ask))
        for k, v in esm_found.items():
            edids.setdefault(k, norm(v))
    watch, watch_missing = pack_watch(watch_refs, watch_names, edids)
    extra = Path(__file__).with_name("extra_edids.json")
    if extra.is_file():
        for k, v in json.loads(extra.read_text(encoding="utf-8")).items():
            edids[k] = norm(v)
    (out / "nvcoop_edids.json").write_text(json.dumps(edids, sort_keys=True))
    (out / "nvcoop_pack_watch.json").write_text(json.dumps({"schema": 1, "items": watch}, sort_keys=True))
    (out / "nvcoop_pack_watch_missing.txt").write_text(NL.join(watch_missing) + (NL if watch_missing else ""), encoding="utf-8")
    (out / "nvcoop_infoquest.json").write_text(json.dumps(info_quest, sort_keys=True))
    shutil.copy2(REPO / "docs" / "quest-policies" / "rules.json", out / "nvcoop_rules.json")
    shutil.copy2(REPO / "docs" / "quest-policies" / "cause_quests.json", out / "nvcoop_cause_quests.json")
    missing = unresolved_names(edids, companion_script_names())
    (out / "nvcoop_edids_missing.txt").write_text("\n".join(missing) + ("\n" if missing else ""), encoding="utf-8")
    print("edids=%d infoquest=%d unresolved rule names=%d -> %s" % (len(edids), len(info_quest), len(missing), out))
    print("  pack watch list: %d item forms (%d from conditions and rules as refs, %d script names, %d of them unresolved)" % (len(watch), len(watch_refs), len(watch_names), len(watch_missing)))
    print("  esm data: %s (resolved from the masters: %d, two different forms for one EDID: %d)" % (data_dir or "not found, EDIDs not read from the ESMs", len(esm_found), len(esm_ambiguous)))
    for k, v in sorted(esm_ambiguous.items()):
        print("  ambiguous EDID %s: %s (first in load order used)" % (k, ", ".join(v)))
    for k in ("VCG01", "VMS01", "GSDocMitchellHouse"):
        print(" ", k, edids.get(k))


if __name__ == "__main__":
    main()
