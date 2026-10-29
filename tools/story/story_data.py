"""Pure data side of the story run: the step script, the questline plan it is checked against, snapshot views and the first divergence.

    load_script(path)            -> Script (steps, npcs, refs resolved; ScriptError with one clear line)
    script_problems(script,plan) -> [str]   cross-checks against questlines/goodsprings/playthrough.json (and out/questdata when it is there)
    view_of(state, script, spec) -> the numbers of one copy that a step reports (stages, objectives, karma, reputation, xp, caps, items)
    first_diff(host_view, client_view) -> the first shared value the two copies disagree on, or None
    delta(before, after)         -> what one copy's view changed during a step

Nothing here talks to a game. The plan is read-only here: tools/questsweep/gen_plans.py owns questlines/*/playthrough.json.
"""
from __future__ import annotations

import gzip
import json
import os
import re
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SCRIPT = HERE / "goodsprings.json"
PLAN = ROOT / "questlines" / "goodsprings" / "playthrough.json"
MUTANT = os.environ.get("STORY_MUTANT", "")       # test-only: one deliberate bug in the runner (tools/story/run_tests.py)

FORM_RE = re.compile(r"^[A-Za-z0-9_ .\-]+\.es[mpl]:[0-9A-Fa-f]{1,8}$")
ACTIONS = ("goto", "talk", "pick", "end_dialogue", "activate", "take", "kill", "wait", "note", "expect_not", "answer_box")
WHO = ("host", "client", "both")


# the bridge names forms in lower case ('falloutnv.esm:1674b3'); the script and the plan use the game's own spelling
PLUGIN_CASE = {"falloutnv.esm": "FalloutNV.esm", "deadmoney.esm": "DeadMoney.esm", "honesthearts.esm": "HonestHearts.esm", "oldworldblues.esm": "OldWorldBlues.esm",
               "lonesomeroad.esm": "LonesomeRoad.esm", "gunrunnersarsenal.esm": "GunRunnersArsenal.esm", "classicpack.esm": "ClassicPack.esm",
               "mercenarypack.esm": "MercenaryPack.esm", "tribalpack.esm": "TribalPack.esm", "caravanpack.esm": "CaravanPack.esm"}


class ScriptError(Exception):
    pass


def norm_form(s: str) -> str:
    """'falloutnv.esm:0104c0f' -> 'FalloutNV.esm:104C0F' (the plugin name as given, the id as hex without leading zeros, upper case)."""
    plug, _, h = str(s).strip().rpartition(":")
    plug = PLUGIN_CASE.get(plug.lower(), plug)
    return f"{plug}:{int(h, 16):X}"


def hexid(form: str) -> str:
    return f"{int(str(form).rpartition(':')[2], 16):X}"


def snap_key(form: str) -> str:
    """The key the state snapshot uses for a form: Plugin.esm:XXXXXX (6 digits)."""
    plug, _, h = str(form).strip().rpartition(":")
    plug = PLUGIN_CASE.get(plug.lower(), plug)
    return f"{plug}:{int(h, 16):06X}"


class Script:
    def __init__(self, data: Dict[str, Any], path: Optional[Path] = None):
        self.data, self.path = data, path
        self.title = data.get("title", "story")
        self.line = data.get("line", "goodsprings")
        self.refs: Dict[str, str] = {k: norm_form(v) for k, v in (data.get("refs") or {}).items()}
        self.quests: Dict[str, str] = {k: norm_form(v) for k, v in (data.get("quests") or {}).items()}
        self.reps: Dict[str, str] = {k: norm_form(v) for k, v in (data.get("reps") or {}).items()}
        self.npcs: Dict[str, Dict[str, Any]] = {}
        for name, n in (data.get("npcs") or {}).items():
            self.npcs[name] = dict(n, ref=self.resolve(n["ref"]), topics=[dict(t, info=norm_form(t["info"])) for t in n.get("topics", [])])
        self.steps: List[Dict[str, Any]] = [self._resolve_step(s) for s in data.get("steps", [])]
        self.by_id = {s["id"]: s for s in self.steps}

    def resolve(self, v: Any) -> Any:
        """'$sunny' -> its form; a form string -> normalised; anything else unchanged."""
        if isinstance(v, str) and v.startswith("$"):
            if v[1:] not in self.refs:
                raise ScriptError(f"unknown ref name {v}")
            return self.refs[v[1:]]
        if isinstance(v, str) and FORM_RE.match(v):
            return norm_form(v)
        return v

    def _resolve_step(self, s: Dict[str, Any]) -> Dict[str, Any]:
        out = json.loads(json.dumps(s))
        for a in out.get("do", []):
            for k in ("ref", "npc"):
                if k in a:
                    a[k] = self.resolve(a[k])
            if "refs" in a:
                a["refs"] = [self.resolve(r) for r in a["refs"]]
            if "infos" in a:
                a["infos"] = [norm_form(self.resolve(i)) for i in a["infos"]]
        for ex in (out.get("expect") or {}).get("items", []):
            ex["item"] = self.resolve(ex["item"])
        for p in out.get("prep", []):
            if "give" in p:
                p["give"]["item"] = self.resolve(p["give"]["item"])
        return out

    def quest_ref(self, edid: str) -> str:
        if edid not in self.quests:
            raise ScriptError(f"quest {edid} is not in the script's quests map")
        return self.quests[edid]

    def topic(self, info: str) -> Optional[Dict[str, Any]]:
        for n in self.npcs.values():
            for t in n["topics"]:
                if t["info"] == info:
                    return dict(t, npc=n["ref"])
        return None


def load_script(path: Optional[Path] = None) -> Script:
    p = Path(path or SCRIPT)
    try:
        data = json.loads(p.read_text(encoding="utf-8"))
    except OSError as e:
        raise ScriptError(f"cannot read {p.name}: {e.strerror or e}") from None
    except ValueError as e:
        raise ScriptError(f"{p.name} is not json: {e}") from None
    return Script(data, p)


def load_plan(path: Optional[Path] = None) -> Dict[str, Any]:
    return json.loads(Path(path or PLAN).read_text(encoding="utf-8"))


def plan_index(plan: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    """edid -> {ref, stages{index: row}, trigger_info{info form: (stage, row)}}."""
    out: Dict[str, Dict[str, Any]] = {}
    for q in plan["quests"]:
        stages = {s["index"]: s for s in q["stages"]}
        trig = {}
        for s in q["stages"]:
            t = s.get("trigger")
            if t:
                trig[norm_form(t["info"])] = s["index"]
        out[q["edid"]] = {"ref": norm_form(q["ref"]), "stages": stages, "trigger_info": trig, "rules": q["rules"], "objectives": q.get("objectives", {})}
    return out


# ------------------------------------------------------------------ questdata (optional, only when out/questdata is there)
_INFOS: Optional[Dict[str, Dict[str, Any]]] = None


def infos_table() -> Dict[str, Dict[str, Any]]:
    """INFO form -> {text, type, speaker, topic} from out/questdata/infos.jsonl.gz, {} when the data is not there."""
    global _INFOS
    if _INFOS is None:
        _INFOS = {}
        f = ROOT / "out" / "questdata" / "infos.jsonl.gz"
        if f.is_file() and MUTANT != "no_questdata":
            try:
                names = {}
                dg = ROOT / "out" / "questdata" / "dials.json.gz"
                if dg.is_file():
                    with gzip.open(dg, "rt", encoding="utf-8") as fh:
                        names = {norm_form("FalloutNV.esm:" + d["formid"]): d.get("name", "") for d in json.load(fh) if d.get("type") == "topic"}
                with gzip.open(f, "rt", encoding="utf-8") as fh:
                    for line in fh:
                        d = json.loads(line)
                        r = d.get("responses") or [{}]
                        _INFOS[norm_form("FalloutNV.esm:" + d["formid"])] = {"text": r[0].get("text", ""), "type": d.get("type", ""), "topic": d.get("topic", ""),
                                                                             "speaker": d.get("speaker", ""), "prompt": names.get(norm_form(d.get("topic", "FalloutNV.esm:0")), ""),
                                                                             "quest": d.get("quest", "")}
            except (OSError, ValueError, KeyError):
                _INFOS = {}
    return _INFOS


def info_text(info: str, script: Optional[Script] = None, n: int = 70) -> str:
    """What the player says (the topic's name), from the script, else the game data, else the NPC's reply, else ''."""
    if script:
        t = script.topic(norm_form(info))
        if t and t.get("text"):
            return t["text"][:n]
    row = infos_table().get(norm_form(info)) or {}
    return (row.get("prompt") or row.get("text") or "")[:n]


# ------------------------------------------------------------------ script checks
def script_problems(script: Script, plan: Optional[Dict[str, Any]] = None, with_questdata: bool = True) -> List[str]:
    """Everything that can be wrong with the script without a game. [] = fine."""
    bad: List[str] = []
    plan = plan or load_plan()
    idx = plan_index(plan)
    if script.data.get("schema") != 1:
        bad.append("schema must be 1")
    for edid, ref in script.quests.items():
        if edid in idx and idx[edid]["ref"] != ref:
            bad.append(f"quest {edid}: script {ref} but the plan says {idx[edid]['ref']}")
    for name, ref in script.reps.items():
        if not FORM_RE.match(ref):
            bad.append(f"reputation {name}: {ref} is not a form")
    ids = set()
    infos = infos_table() if with_questdata else {}
    for s in script.steps:
        sid = s.get("id", "?")
        if sid in ids:
            bad.append(f"{sid}: step id used twice")
        ids.add(sid)
        for k in ("id", "title", "who", "box_s", "do", "goal"):
            if k not in s:
                bad.append(f"{sid}: no {k}")
        if s.get("who") not in WHO:
            bad.append(f"{sid}: who must be one of {WHO}")
        if float(s.get("box_s", 0)) < 20 or float(s.get("box_s", 0)) > 600:
            bad.append(f"{sid}: box_s {s.get('box_s')} outside 20..600 s")
        goal = s.get("goal") or {}
        for q, n in (goal.get("stages") or {}).items():
            if q not in script.quests:
                bad.append(f"{sid}: goal quest {q} is not in the quests map")
            elif q in idx and int(n) not in idx[q]["stages"]:
                bad.append(f"{sid}: goal {q} stage {n} is not a stage of the plan")
        for p in s.get("prep", []):
            if "give" in p:
                if p["give"].get("who") not in ("host", "client") or not FORM_RE.match(str(p["give"].get("item", ""))):
                    bad.append(f"{sid}: prep give needs who host|client and an item form")
            elif p.get("quest") not in script.quests:
                bad.append(f"{sid}: prep quest {p.get('quest')} is not in the quests map")
        who_seen = set()
        for a in s.get("do", []):
            if a.get("a") not in ACTIONS:
                bad.append(f"{sid}: unknown action {a.get('a')!r}")
                continue
            if a["a"] == "answer_box":
                if a.get("who") not in ("host", "client"):
                    bad.append(f"{sid}: answer_box needs who host or client")
                continue
            if a["a"] == "expect_not":
                if a.get("quest") not in script.quests or not isinstance(a.get("stage"), int):
                    bad.append(f"{sid}: expect_not needs a quest of the quests map and a stage")
                continue
            if a["a"] != "note" and a.get("who") not in ("host", "client"):
                bad.append(f"{sid}: action {a['a']} needs who host or client")
            who_seen.add(a.get("who"))
            for i in a.get("infos", []):
                if with_questdata and infos and i not in infos:
                    bad.append(f"{sid}: INFO {i} is not in out/questdata")
                tp = script.topic(i)
                if tp is None:
                    bad.append(f"{sid}: INFO {i} is not listed under any npc topics (the fake world and the report need its text)")
                else:
                    for q, e in idx.items():
                        if i in e["trigger_info"] and tp.get("sets") not in (None, [q, e["trigger_info"][i]]):
                            bad.append(f"{sid}: INFO {i} sets {tp.get('sets')} in the script but the plan says {q} {e['trigger_info'][i]}")
            for r in a.get("refs", []) + ([a["ref"]] if "ref" in a else []) + ([a["npc"]] if "npc" in a else []):
                if not FORM_RE.match(r):
                    bad.append(f"{sid}: {r!r} is not a form")
        if s.get("who") in ("host", "client") and who_seen - {s["who"], None}:
            bad.append(f"{sid}: step who={s['who']} has actions by {sorted(x for x in who_seen if x)}")
        if s.get("who") == "both" and not s.get("enter") and any(a.get("a") != "note" for a in s.get("do", [])) and not {"host", "client"} <= who_seen:
            bad.append(f"{sid}: step who=both needs actions by the host and by the client")
    for n in script.npcs.values():
        for t in n["topics"]:
            if with_questdata and infos and t["info"] not in infos:
                bad.append(f"npc {n.get('name')}: INFO {t['info']} is not in out/questdata")
            sets = t.get("sets")
            if sets:
                q, st = sets
                if q in idx and int(st) not in idx[q]["stages"]:
                    bad.append(f"npc {n.get('name')}: {t['info']} sets {q} {st}, not a stage of the plan")
    host_steps = sum(1 for s in script.steps if s["who"] == "host")
    client_steps = sum(1 for s in script.steps if s["who"] == "client")
    if script.steps and (host_steps == 0 or client_steps == 0):
        bad.append(f"the host and the client must both lead steps (host {host_steps}, client {client_steps})")
    return bad


# ------------------------------------------------------------------ views of one copy
def _obj_flags(o: Dict[str, Any]) -> str:
    return ("d" if o.get("displayed") else "") + ("c" if o.get("completed") else "")


def view_of(state: Dict[str, Any], script: Script, items: Optional[List[str]] = None) -> Dict[str, Any]:
    """The numbers a step reports for one copy, from a `state` snapshot ({shared, perPlayer}) in the shape of docs/snapshot-schema.md.

    quests: the script's quests, each {stage, running, completed, failed, done (stages done), obj {id: 'd'|'c'|'dc'} for objectives not at their default};
    karma, rep {name: [fame, infamy]}, xp, caps, inv {form: count} (every item the player holds), watched items by name."""
    shared = (state or {}).get("shared") or {}
    pp = (state or {}).get("perPlayer") or {}
    me = pp.get("local") or (next(iter(pp.values())) if pp else {}) or {}
    qs = shared.get("quests") or {}
    out: Dict[str, Any] = {"quests": {}, "rep": {}}
    for edid, ref in script.quests.items():
        q = qs.get(snap_key(ref))
        if q is None:
            out["quests"][edid] = None
            continue
        out["quests"][edid] = {"stage": int(q.get("currentStage", 0)), "running": bool(q.get("running")), "completed": bool(q.get("completed")),
                               "failed": bool(q.get("failed")), "done": sorted(int(x) for x in q.get("completedStages") or []),
                               "obj": {str(k): _obj_flags(v) for k, v in sorted((q.get("objectives") or {}).items(), key=lambda kv: int(kv[0])) if _obj_flags(v)}}
    for name, ref in script.reps.items():
        r = (shared.get("reputations") or {}).get(snap_key(ref)) or {}
        out["rep"][name] = [round(float(r.get("fame", 0)), 2), round(float(r.get("infamy", 0)), 2)]
    av = me.get("actorValues") or {}
    karma = shared.get("karma")
    if isinstance(av.get("Karma"), dict):
        karma = av["Karma"].get("current", karma)
    out["karma"] = round(float(karma), 2) if karma is not None else None
    xp = av.get("XP") if isinstance(av.get("XP"), dict) else av.get("Experience")
    out["xp"] = int(xp.get("current", 0)) if isinstance(xp, dict) else None
    out["caps"] = int(me["caps"]) if "caps" in me else None
    out["inv"] = {k: int(v.get("count", 0)) for k, v in (me.get("inventory") or {}).items() if isinstance(v, dict)}
    out["pos"] = me.get("position")
    return out


def stage_reached(q: Optional[Dict[str, Any]], n: int) -> bool:
    return bool(q) and (n in q["done"] or q["stage"] >= n)


def first_diff(h: Dict[str, Any], c: Dict[str, Any], quests: Optional[List[str]] = None) -> Optional[Dict[str, Any]]:
    """The first SHARED value (quest state, objectives, karma, reputation) on which the copies differ, in a fixed order; None when they agree.
    Items, xp and caps are per player and are not compared here."""
    if MUTANT == "divergence_blind":
        return None
    for edid in h["quests"]:
        if quests is not None and edid not in quests:
            continue
        a, b = h["quests"][edid], c["quests"].get(edid)
        if a is None and b is None:
            continue
        a, b = a or {}, b or {}
        for k in ("running", "stage", "completed", "failed", "done"):
            if a.get(k) != b.get(k):
                return {"key": f"{edid}.{k}", "host": a.get(k), "client": b.get(k)}
        ao, bo = a.get("obj", {}), b.get("obj", {})
        for oid in sorted(set(ao) | set(bo), key=int):
            if ao.get(oid, "") != bo.get(oid, ""):
                return {"key": f"{edid}.objective {oid}", "host": ao.get(oid, "-"), "client": bo.get(oid, "-")}
    if h.get("karma") != c.get("karma") and h.get("karma") is not None and c.get("karma") is not None:
        return {"key": "karma", "host": h["karma"], "client": c["karma"]}
    for name in h["rep"]:
        if h["rep"][name] != c["rep"].get(name):
            return {"key": f"reputation {name} [fame, infamy]", "host": h["rep"][name], "client": c["rep"].get(name)}
    return None


def delta(b: Dict[str, Any], a: Dict[str, Any]) -> Dict[str, Any]:
    """What changed in one copy's view during a step (only what changed)."""
    out: Dict[str, Any] = {}
    st, ob = {}, {}
    for edid, qa in a["quests"].items():
        qb = b["quests"].get(edid)
        if qa is None:
            continue
        if qb is None or qb["stage"] != qa["stage"] or qb["running"] != qa["running"]:
            st[edid] = [None if qb is None else qb["stage"], qa["stage"]] + (["running"] if qa["running"] and not (qb and qb["running"]) else [])
        if qa["completed"] and not (qb and qb["completed"]):
            st.setdefault(edid, [qa["stage"], qa["stage"]]).append("completed")
        before_o = (qb or {}).get("obj", {})
        for oid, fl in qa["obj"].items():
            if before_o.get(oid, "") != fl:
                ob[f"{edid}:{oid}"] = fl
        for oid in before_o:
            if oid not in qa["obj"]:
                ob[f"{edid}:{oid}"] = "-"
    if st:
        out["stages"] = st
    if ob:
        out["objectives"] = ob
    items = {k: a["inv"].get(k, 0) - b["inv"].get(k, 0) for k in set(a["inv"]) | set(b["inv"]) if a["inv"].get(k, 0) != b["inv"].get(k, 0)}
    if items:
        out["items"] = dict(sorted(items.items()))
    for k in ("xp", "caps", "karma"):
        if a.get(k) is not None and b.get(k) is not None and a[k] != b[k]:
            out[k] = round(a[k] - b[k], 2)
    rp = {n: [round(a["rep"][n][0] - b["rep"][n][0], 2), round(a["rep"][n][1] - b["rep"][n][1], 2)] for n in a["rep"]
          if n in b["rep"] and a["rep"][n] != b["rep"][n]}
    if rp:
        out["rep"] = rp
    return out


def brief_view(v: Dict[str, Any], quests: List[str]) -> Dict[str, Any]:
    """The part of a view a step table shows: stage per quest of the step, objectives set, karma, rep."""
    return {"stages": {q: (v["quests"].get(q) or {}).get("stage") for q in quests},
            "obj": {q: (v["quests"].get(q) or {}).get("obj", {}) for q in quests}, "karma": v.get("karma"), "rep": v.get("rep"), "xp": v.get("xp"), "caps": v.get("caps")}
