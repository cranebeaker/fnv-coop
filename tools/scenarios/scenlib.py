"""Scenario format (schema 1): loading, game-free validation, command/expression building, jump planning.

A scenario is one JSON file in scenarios/<name>.json. Everything that names a game form goes through the scenario's
`forms` glossary: alias -> {"ref": "Plugin.esm:LOCAL6", "edid": "...", "sig": "ACHR"}. Steps then use "$alias"
(or a raw "Plugin.esm:LOCAL6" string). The validator resolves every ref against out/questdata/forms.json.gz (which holds
the EditorID and record type of each form), so an invented FormID, a wrong EditorID or a wrong record type is rejected
without launching the game. Runtime hex ids (load-order index << 24 | local id) come from the same load order the
questline sweep uses (tools/questdata/loadorder.json via plugin/snapshot/tools/sweep_config.py).

Console lines for a reference other than the player are ("exec", cmd, ref) actions: the bridge runs `cmd` with `ref` as
the console's calling reference (exec {cmd, ref}). In game (2026-10-21) the console rejected "<hexFormID>.Command"
(Script command "00103E61.GetOpenState" not found); "player." lines work and stay ("exec", line).

Pure functions only; the game-facing part is runner.py.
"""
import gzip
import json
import os
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SCEN_DIR = REPO / "scenarios"
QDATA = Path(os.environ.get("FNVCOOP_QUESTDATA") or REPO / "out" / "questdata")   # e.g. questdata of a TTW install
sys.path.insert(0, str(REPO / "plugin" / "snapshot" / "tools"))
import sweep_config  # noqa: E402  (L7 helper: load order, hex8, split_ref, reset ids of a sweep config)
sys.path.insert(0, str(REPO / "tools" / "runner"))
from console_ref import RefLineError, split_ref_line  # noqa: E402  (pure: "<ref>.<Func> args" -> ("<Func> args", ref))

SCHEMA = 1
# test-only twins: "noresolve" drops the FormID/EditorID/record-type checks; "hexprefix" builds the old "<hex>.Func"
# console lines again; "refline" drops the check that a {alias}.Func call is the first word of a console line / expr
MUTANT = os.environ.get("SCEN_MUTANT", "")
REF_RE = re.compile(r"^[A-Za-z0-9_]+\.es[mp]:[0-9A-F]{6}$")
ALIAS_RE = re.compile(r"^\$([A-Za-z][A-Za-z0-9_]*)$")
PLACEHOLDER_RE = re.compile(r"\{([A-Za-z][A-Za-z0-9_]*)\}")
ON_NAMED = ("host", "guest", "all")

# Engine objects that are not in forms.json.gz but whose ids are fixed by the game itself.
BUILTIN = {"FalloutNV.esm:000014": ("REFR", "PlayerRef"), "FalloutNV.esm:00000F": ("MISC", "Caps001"),
           "FalloutNV.esm:08ED03": ("AMMO", "Ammo9mm")}   # no quest names it, so the questdata index lacks it; arms the fight bots

REF_SIGS = {"REFR", "ACHR", "ACRE"}
ACTOR_SIGS = {"ACHR", "ACRE"}
ITEM_SIGS = {"MISC", "WEAP", "ARMO", "AMMO", "ALCH", "NOTE", "KEYM", "CHIP", "IMOD", "BOOK"}
SPAWN_SIGS = {"NPC_", "CREA"} | ITEM_SIGS | {"LVLI"}

# Capabilities a scenario step may need. status "present" = exists in the current bridge/plugin; "proposed" = the
# scenario is written for it but the bridge method does not exist yet (the step is reported as blocked, never as a pass).
CAPABILITIES = {
    "console": {"status": "present", "note": "bridge exec"},
    "quest_reset": {"status": "present", "method": "test.quest.reset"},
    "quest_setvar": {"status": "present", "method": "test.quest.setvar"},
    "load_save": {"status": "present", "method": "loadSave"},
    "save_game": {"status": "present", "method": "saveGame"},
    "coop_status": {"status": "present", "method": "coop.status"},
    "dialogue": {"status": "present", "method": "bot.say", "note": "T6a, behind nvcoop.ini [dev] bots=1; implemented, never run in game"},
    "message_box": {"status": "present", "method": "bot.menu.press", "note": "T6a, [dev] bots=1; implemented, never run in game"},
    "party_consent": {"status": "present", "method": "bot.consent", "note": "T6a, [dev] bots=1; Ready=0, Not yet=1; implemented, never run in game"},
    "vats_hit": {"status": "present", "method": "bot.vats", "note": "T6a, [dev] bots=1; the limb argument is unsupported (returns an error), so it is not sent"},
    "fast_travel": {"status": "present", "method": "test.fasttravel", "note": "T6a, [dev] bots=1; a MoveTo teleport across cells (reply genuine:false), not the real fast-travel call"},
    "ref_count": {"status": "present", "method": "dev.countRefs", "note": "T6a, [dev] bots=1; count of refs of a base form"},
    "actor_owner": {"status": "proposed", "method": "coop.owner", "note": "who owns an actor and the ownership epoch"},
    "session_leave": {"status": "proposed", "method": "coop.leave", "note": "drop one player out of the co-op session"},
    "session_join": {"status": "proposed", "method": "coop.join", "note": "rejoin the co-op session"},
}
PRESENT = {k for k, v in CAPABILITIES.items() if v["status"] == "present"}

# step op -> (required fields, optional fields)
OPS = {
    "setstage": ({"quest", "stage"}, set()),
    "setvar": ({"quest", "var", "value"}, set()),
    "moveto": ({"who", "to"}, set()),
    "spawn": ({"base", "at"}, {"count"}),
    "additem": ({"who", "item", "count"}, set()),
    "removeitem": ({"who", "item", "count"}, set()),
    "kill": ({"ref"}, set()),
    "resurrect": ({"ref"}, set()),
    "enable": ({"ref"}, set()),
    "disable": ({"ref"}, set()),
    "lock": ({"ref", "level"}, set()),
    "unlock": ({"ref"}, set()),
    "activate": ({"ref"}, set()),
    "recruit": ({"ref"}, set()),
    "console": ({"cmd"}, set()),
    "wait": ({"seconds"}, set()),
    "save": ({"name"}, set()),
    "load": ({"name"}, set()),
    "vats_hit": ({"target", "limb"}, set()),
    "fast_travel": ({"marker"}, set()),
    "talk": ({"npc", "greeting"}, set()),
    "pick": ({"topic", "info"}, set()),
    "message_click": ({"button"}, set()),
    "consent": ({"answer"}, set()),
    "leave": (set(), set()),
    "join": (set(), set()),
    "check": ({"kind"}, {"quest", "stage", "who", "ref", "item", "cell", "a", "b", "av", "expr", "cmp", "value", "agg",
                          "tol", "timeout", "base"}),
}
COMMON = {"id", "op", "on", "note", "needs", "parallel"}
OP_NEEDS = {"pick": "dialogue", "message_click": "message_box", "consent": "party_consent", "vats_hit": "vats_hit", "fast_travel": "fast_travel", "leave": "session_leave", "join": "session_join"}
# kind -> (required fields, the field that is the calling reference or None, expression template).
# The calling reference is never written as "<hex>.Func": the expression runs with it as exec's ref ("player" keeps the
# "player." prefix, a valid script reference).
CHECK_KINDS = {
    "stage": ({"quest"}, None, "GetStage {quest}"),
    "stage_done": ({"quest", "stage"}, None, "GetStageDone {quest} {stage}"),
    "item_count": ({"who", "item"}, "who", "GetItemCount {item}"),
    "dead": ({"ref"}, "ref", "GetDead"),
    "disabled": ({"ref"}, "ref", "GetDisabled"),
    "locked": ({"ref"}, "ref", "GetLocked"),
    "in_cell": ({"ref", "cell"}, "ref", "GetInCell {cell}"),
    "distance": ({"a", "b"}, "a", "GetDistance {b}"),
    "av": ({"ref", "av"}, "ref", "GetAV {av}"),
    "teammate": ({"ref"}, "ref", "GetPlayerTeammate"),
    "spawn_count": ({"base", "ref"}, None, None),   # call-based: dev.countRefs {base}
    "owner": ({"ref"}, None, None),                 # needs actor_owner (bridge method coop.owner); no NVSE expression
    "expr": ({"expr"}, None, None),
}
CMPS = {"==", "!=", "<", "<=", ">", ">=", "same"}
AGGS = {None, "sum", "min", "max"}
ACTOR_VALUES = {"Health", "Strength", "Perception", "Endurance", "Charisma", "Intelligence", "Agility", "Luck",
                "ActionPoints", "CarryWeight", "Aggression", "Confidence", "Fatigue"}


# ---------------------------------------------------------------------------------------------------- data
_CACHE = {}


def load_forms(root=QDATA):
    key = ("forms", str(root))
    if key not in _CACHE:
        with gzip.open(Path(root) / "forms.json.gz", "rt", encoding="utf-8") as f:
            forms = json.load(f)
        # forms.json.gz holds the forms that quests point at, so a quest nothing points at (Fallout3.esm MQ10 under TTW)
        # is missing there; every quest is in quests.json.
        qpath = Path(root) / "quests.json"
        if qpath.exists():
            for q in load_quests(root).values():
                if q.get("ref") and q["ref"] not in forms:
                    forms[q["ref"]] = {"sig": "QUST", "plugin": q["ref"].split(":")[0], "edid": q.get("edid"),
                                       "name": q.get("name")}
        _CACHE[key] = forms
    return _CACHE[key]


def load_dials(root=QDATA):
    key = ("dials", str(root))
    if key not in _CACHE:
        with gzip.open(Path(root) / "dials.json.gz", "rt", encoding="utf-8") as f:
            _CACHE[key] = {d["ref"]: {"sig": "DIAL", "edid": d.get("edid")} for d in json.load(f) if d.get("ref")}
    return _CACHE[key]


def load_infos(root, want):
    """INFO records are many; read only the wanted refs. -> ref -> {"sig":"INFO","text": first response}."""
    out = {}
    wanted = {w.split(":")[-1].upper().zfill(8) for w in want}
    with gzip.open(Path(root) / "infos.jsonl.gz", "rt", encoding="utf-8") as f:
        for line in f:
            if line[11:19] not in wanted:   # {"formid":"XXXXXXXX"
                continue
            d = json.loads(line)
            if d["formid"].upper() in wanted:
                rs = d.get("responses") or [{}]
                out["FalloutNV.esm:" + d["formid"][2:] if d["formid"].startswith("00") else d["formid"]] = {
                    "sig": "INFO", "text": rs[0].get("text", "")}
    return out


def load_quests(root=QDATA):
    """edid(lower) -> quest entry; also ref -> entry."""
    key = ("quests", str(root))
    if key not in _CACHE:
        d = json.loads((Path(root) / "quests.json").read_text(encoding="utf-8"))
        qs = d["quests"] if isinstance(d, dict) else d
        by = {}
        for q in qs:
            if q.get("edid"):
                by[q["edid"].lower()] = q
            if q.get("ref"):
                by[q["ref"]] = q
        _CACHE[key] = by
    return _CACHE[key]


def load_order():
    return sweep_config.load_loadorder()


def norm_ref(s):
    """'FalloutNV.esm:104df2' -> 'FalloutNV.esm:104DF2' (6 hex digits). Raises ValueError if not a ref."""
    if not isinstance(s, str) or ":" not in s:
        raise ValueError("not a ref: %r" % (s,))
    plug, _, loc = s.rpartition(":")
    return "%s:%06X" % (plug, int(loc, 16))


def runtime_hex(ref, order=None):
    plug, loc = sweep_config.split_ref(ref)
    return sweep_config.hex8(plug, loc, order or load_order())


# ---------------------------------------------------------------------------------------------------- loading
def load_scenario(path):
    p = Path(path)
    sc = json.loads(p.read_text(encoding="utf-8"))
    sc.setdefault("_file", str(p))
    return sc


def list_scenarios(directory=SCEN_DIR):
    return sorted(Path(directory).glob("*.json"))


def players_of(sc):
    return int(sc.get("players", 1))


def targets(sc, on):
    """Indices of players a step runs on. 'host' = 0, 'guest' = 1 (all other guests too if >2 players), 'all', or an int."""
    n = players_of(sc)
    on = "host" if on is None else on
    if on == "all":
        return list(range(n))
    if on == "host":
        return [0]
    if on == "guest":
        return list(range(1, n))
    if isinstance(on, int) and 0 <= on < n:
        return [on]
    raise ValueError("bad 'on': %r" % (on,))


def all_steps(sc):
    return list(sc.get("setup") or []) + list(sc.get("steps") or [])


def step_target(step, sc):
    default = "all" if step.get("op") == "check" else "host"
    return targets(sc, step.get("on", default))


# ---------------------------------------------------------------------------------------------------- resolution
class Resolver:
    """Alias/ref -> normalized ref -> runtime hex. Used by the validator and the runner."""

    def __init__(self, sc, order=None):
        self.forms = sc.get("forms") or {}
        self.order = order or load_order()

    def ref(self, v):
        if v == "player":
            return "FalloutNV.esm:000014"
        m = ALIAS_RE.match(v) if isinstance(v, str) else None
        if m:
            if m.group(1) not in self.forms:
                raise KeyError("unknown alias %s" % v)
            return norm_ref(self.forms[m.group(1)]["ref"])
        return norm_ref(v)

    def hex(self, v):
        if v == "player":
            return "player"
        return runtime_hex(self.ref(v), self.order)

    def subst(self, text):
        """Replace {alias} placeholders in a console line or expression with runtime hex ids ({player} -> player)."""
        def one(m):
            name = m.group(1)
            if name == "player":
                return "player"
            if name not in self.forms:
                raise KeyError("unknown placeholder {%s}" % name)
            return runtime_hex(norm_ref(self.forms[name]["ref"]), self.order)
        return PLACEHOLDER_RE.sub(one, text)


# ---------------------------------------------------------------------------------------------------- validation
def _quest_entry(res, v, quests):
    try:
        ref = res.ref(v)
    except Exception:
        return None
    return quests.get(ref)


def validate(sc, name=None, root=QDATA, order=None):
    """Return a list of error strings (empty = valid). Game-free."""
    errs = []
    e = errs.append
    forms = load_forms(root)
    quests = load_quests(root)
    order = order or load_order()
    if sc.get("schema") != SCHEMA:
        e("schema must be %d" % SCHEMA)
    nm = sc.get("name")
    if not nm or not re.fullmatch(r"[a-z0-9_]+", str(nm)):
        e("name must be lower_snake_case")
    if name and nm != name:
        e("name %r does not match file name %r" % (nm, name))
    for k in ("title", "description"):
        if not sc.get(k):
            e("missing %s" % k)
    n = sc.get("players", 1)
    if not isinstance(n, int) or not 1 <= n <= 4:
        e("players must be an int 1..4")
        n = 1
    start = sc.get("start") or {}
    if not start.get("save"):
        e("start.save is required (a save name, without .fos)")
    elif not re.fullmatch(r"[A-Za-z0-9_\-]+", start["save"]):
        e("start.save has odd characters")
    if "prepared" in start and not isinstance(start["prepared"], bool):
        e("start.prepared must be true or false (false = always load the plain start.save, not nvcoop_<save>_ready)")

    # forms glossary
    gl = sc.get("forms") or {}
    for alias, d in gl.items():
        if not re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*", alias) or alias == "player":
            e("forms: bad alias %r" % alias)
            continue
        ref = d.get("ref")
        try:
            ref = norm_ref(ref)
        except Exception:
            e("forms.%s: ref %r is not Plugin.esm:XXXXXX" % (alias, ref))
            continue
        if d.get("ref") != ref:
            e("forms.%s: ref %r must be written normalized (%s)" % (alias, d.get("ref"), ref))
        plug = ref.split(":")[0]
        if plug.lower() not in [p.lower() for p in order]:
            e("forms.%s: plugin %s is not in the load order" % (alias, plug))
        rec = forms.get(ref)
        if rec is None and ref in BUILTIN:
            rec = {"sig": BUILTIN[ref][0], "edid": BUILTIN[ref][1]}
        if rec is None and d.get("sig") == "DIAL":
            rec = load_dials(root).get(ref)
        if rec is None and d.get("sig") == "INFO":
            rec = load_infos(root, {ref}).get(ref)
        if rec is None:
            e("forms.%s: %s does not resolve in out/questdata (forms/dials/infos)" % (alias, ref))
            continue
        if rec.get("sig") == "INFO":
            if not d.get("line") or not rec.get("text", "").startswith(d["line"]):
                e("forms.%s: INFO line %r does not match the game data (%r)" % (alias, d.get("line"), rec.get("text", "")[:40]))
        elif not d.get("edid"):
            e("forms.%s: edid is required (copy it from forms.json.gz)" % alias)
        elif rec.get("edid") != d["edid"]:
            e("forms.%s: edid %r does not match the game data (%r)" % (alias, d["edid"], rec.get("edid")))
        if not d.get("sig"):
            e("forms.%s: sig is required" % alias)
        elif rec.get("sig") != d["sig"]:
            e("forms.%s: sig %r does not match the game data (%r)" % (alias, d["sig"], rec.get("sig")))
    res = Resolver({"forms": {a: d for a, d in gl.items() if isinstance(d, dict) and "ref" in d}}, order)

    def sig_of(v):
        try:
            ref = res.ref(v)
        except Exception:
            return None
        rec = forms.get(ref)
        if rec:
            return rec["sig"]
        g = gl.get(v[1:]) if isinstance(v, str) and v.startswith("$") else None
        if g and g.get("sig") in ("DIAL", "INFO"):
            return g["sig"]
        return BUILTIN.get(ref, (None,))[0]

    def need_form(where, v, sigs, allow_player=False):
        if v == "player":
            if not allow_player:
                e("%s: 'player' is not allowed here" % where)
            return
        try:
            res.ref(v)
        except KeyError as ex:
            e("%s: %s" % (where, ex.args[0]))
            return
        except Exception:
            e("%s: %r is not an alias or Plugin.esm:XXXXXX" % (where, v))
            return
        if ALIAS_RE.match(str(v)) is None:
            # raw refs must still resolve
            ref = norm_ref(v)
            if ref not in forms and ref not in BUILTIN:   # raw refs cannot name DIAL/INFO; use a glossary alias for those
                e("%s: %s does not resolve in out/questdata/forms.json.gz" % (where, ref))
                return
        s = sig_of(v)
        if s is None:
            e("%s: %r does not resolve in out/questdata/forms.json.gz" % (where, v))
        elif sigs and s not in sigs:
            e("%s: %r is a %s, expected one of %s" % (where, v, s, "/".join(sorted(sigs))))

    # reset
    rs = sc.get("reset")
    if not isinstance(rs, dict) or not (rs.get("group") or rs.get("quests")):
        e("reset must name a sweep 'group' (goodsprings) or a 'quests' list")
    else:
        if rs.get("group") and rs["group"] != "goodsprings":
            e("reset.group %r unknown (only 'goodsprings' is wired to an L7 sweep config)" % rs["group"])
        for q in rs.get("quests") or []:
            need_form("reset.quests", q, {"QUST"})

    # steps
    ids = set()
    steps = all_steps(sc)
    if not sc.get("steps"):
        e("steps is empty")
    checks = 0
    for i, st in enumerate(steps):
        w = "step %s" % (st.get("id") or i)
        sid = st.get("id")
        if not sid or not re.fullmatch(r"[a-z0-9_]+", str(sid)):
            e("%s: id must be lower_snake_case" % w)
        elif sid in ids:
            e("%s: duplicate id" % w)
        ids.add(sid)
        op = st.get("op")
        if op not in OPS:
            e("%s: unknown op %r" % (w, op))
            continue
        req, opt = OPS[op]
        for k in req:
            if k not in st:
                e("%s: op %s needs %r" % (w, op, k))
        for k in st:
            if k not in req | opt | COMMON:
                e("%s: unexpected key %r for op %s" % (w, k, op))
        try:
            targets(sc, st.get("on", "all" if op == "check" else "host"))
        except ValueError:
            e("%s: bad 'on' %r for %d player(s)" % (w, st.get("on"), n))
        if "parallel" in st and not isinstance(st["parallel"], bool):
            e("%s: parallel must be true/false" % w)
        for cap in ([OP_NEEDS[op]] if op in OP_NEEDS else []) + list(st.get("needs") or []):
            if cap not in CAPABILITIES:
                e("%s: unknown capability %r" % (w, cap))
        if op in ("setstage", "setvar"):
            need_form(w + ".quest", st.get("quest"), {"QUST"})
            q = _quest_entry(res, st.get("quest"), quests)
            if q is None:
                e("%s: quest %r has no entry in quests.json" % (w, st.get("quest")))
            elif op == "setstage":
                idx = {s.get("index") for s in q.get("stages") or []}
                if st.get("stage") not in idx:
                    e("%s: stage %r is not a stage of %s (stages %s)" % (w, st.get("stage"), q["edid"], sorted(idx)))
            else:
                vs = {v["name"].lower() for v in (q.get("script_data") or {}).get("vars") or []}
                if str(st.get("var", "")).lower() not in vs:
                    e("%s: %r is not a script variable of quest %s" % (w, st.get("var"), q["edid"]))
                if not isinstance(st.get("value"), (int, float)):
                    e("%s: value must be a number" % w)
        elif op == "moveto":
            need_form(w + ".who", st.get("who"), REF_SIGS, allow_player=True)
            need_form(w + ".to", st.get("to"), REF_SIGS)
        elif op == "spawn":
            need_form(w + ".base", st.get("base"), SPAWN_SIGS)
            need_form(w + ".at", st.get("at"), REF_SIGS, allow_player=True)
            if not isinstance(st.get("count", 1), int) or st.get("count", 1) < 1:
                e("%s: count must be a positive int" % w)
        elif op in ("additem", "removeitem"):
            need_form(w + ".who", st.get("who"), REF_SIGS, allow_player=True)
            need_form(w + ".item", st.get("item"), ITEM_SIGS)
            if not isinstance(st.get("count"), int) or st["count"] < 1:
                e("%s: count must be a positive int" % w)
        elif op in ("kill", "resurrect", "recruit"):
            need_form(w + ".ref", st.get("ref"), ACTOR_SIGS)
        elif op in ("enable", "disable", "activate"):
            need_form(w + ".ref", st.get("ref"), REF_SIGS)
        elif op in ("lock", "unlock"):
            need_form(w + ".ref", st.get("ref"), REF_SIGS)
            if op == "lock" and not isinstance(st.get("level"), int):
                e("%s: level must be an int" % w)
        elif op == "console":
            cmd = st.get("cmd")
            if not isinstance(cmd, str) or not cmd.strip():
                e("%s: cmd must be a non-empty string" % w)
            else:
                for m in PLACEHOLDER_RE.finditer(cmd):
                    if m.group(1) != "player" and m.group(1) not in gl:
                        e("%s: placeholder {%s} is not in forms" % (w, m.group(1)))
                if re.search(r"(?<![A-Za-z0-9_.])(?:0x)?[0-9A-Fa-f]{8}(?![A-Za-z0-9_])", cmd):
                    e("%s: raw FormID in cmd; use a {alias} placeholder so it is validated" % w)
                ref_line_error(w, cmd, e)
        elif op == "wait":
            if not isinstance(st.get("seconds"), (int, float)) or not 0 <= st["seconds"] <= 120:
                e("%s: seconds must be 0..120" % w)
        elif op == "save":
            if not str(st.get("name", "")).startswith("autosave") or not re.fullmatch(r"[A-Za-z0-9_]+", str(st.get("name"))):
                e("%s: save name must start with 'autosave' (engine rule, see nvbridge.save_game) and be [A-Za-z0-9_]" % w)
        elif op == "load":
            if not re.fullmatch(r"[A-Za-z0-9_]+", str(st.get("name", ""))):
                e("%s: bad save name" % w)
        elif op == "talk":
            need_form(w + ".npc", st.get("npc"), ACTOR_SIGS)
            need_form(w + ".greeting", st.get("greeting"), {"INFO"})
        elif op == "pick":
            need_form(w + ".topic", st.get("topic"), {"DIAL"})
            need_form(w + ".info", st.get("info"), {"INFO"})
        elif op == "consent":
            if st.get("answer") not in ("ready", "notyet"):
                e("%s: answer must be 'ready' or 'notyet'" % w)
        elif op == "message_click":
            if not isinstance(st.get("button"), int) or st["button"] < 0:
                e("%s: button must be a non-negative int" % w)
        elif op == "vats_hit":
            need_form(w + ".target", st.get("target"), ACTOR_SIGS)
            if st.get("limb") not in ("Head", "Torso", "LeftArm", "RightArm", "LeftLeg", "RightLeg"):
                e("%s: limb must be Head/Torso/LeftArm/RightArm/LeftLeg/RightLeg" % w)
        elif op == "fast_travel":
            need_form(w + ".marker", st.get("marker"), {"REFR"})
        elif op == "check":
            checks += 1
            _validate_check(w, st, e, need_form, gl)
    if checks == 0:
        e("scenario has no check step")
    if MUTANT == "noresolve":
        errs = [x for x in errs if "resolve" not in x and "game data" not in x]
    return errs


def _validate_check(w, st, e, need_form, glossary):
    kind = st.get("kind")
    if kind not in CHECK_KINDS:
        e("%s: unknown check kind %r" % (w, kind))
        return
    req = CHECK_KINDS[kind][0]
    for k in req:
        if k not in st:
            e("%s: check %s needs %r" % (w, kind, k))
    cmp_ = st.get("cmp", "==")
    if cmp_ not in CMPS:
        e("%s: cmp %r not in %s" % (w, cmp_, sorted(CMPS)))
    if cmp_ != "same" and "value" not in st:
        e("%s: check needs a 'value' (or cmp 'same')" % w)
    if st.get("agg") not in AGGS:
        e("%s: agg must be sum/min/max" % w)
    if st.get("agg") and cmp_ == "same":
        e("%s: agg and cmp 'same' cannot be combined" % w)
    if "timeout" in st and (not isinstance(st["timeout"], (int, float)) or not 0 < st["timeout"] <= 120):
        e("%s: timeout must be 0..120 seconds" % w)
    if kind == "stage" or kind == "stage_done":
        need_form(w + ".quest", st.get("quest"), {"QUST"})
    if kind in ("item_count",):
        need_form(w + ".who", st.get("who"), REF_SIGS, allow_player=True)
        need_form(w + ".item", st.get("item"), ITEM_SIGS)
    if kind in ("dead", "disabled", "locked", "av", "teammate", "owner"):
        need_form(w + ".ref", st.get("ref"), REF_SIGS)
    if kind == "av" and st.get("av") not in ACTOR_VALUES:
        e("%s: av %r not in %s" % (w, st.get("av"), sorted(ACTOR_VALUES)))
    if kind == "in_cell":
        need_form(w + ".ref", st.get("ref"), REF_SIGS, allow_player=True)
        need_form(w + ".cell", st.get("cell"), {"CELL"})
    if kind == "distance":
        need_form(w + ".a", st.get("a"), REF_SIGS, allow_player=True)
        need_form(w + ".b", st.get("b"), REF_SIGS, allow_player=True)
    if kind == "spawn_count":
        need_form(w + ".ref", st.get("ref"), REF_SIGS)
        need_form(w + ".base", st.get("base"), SPAWN_SIGS)
    if kind == "expr":
        for m in PLACEHOLDER_RE.finditer(str(st.get("expr", ""))):
            if m.group(1) != "player" and m.group(1) not in glossary:
                e("%s: placeholder {%s} is not in forms" % (w, m.group(1)))
        ref_line_error(w, str(st.get("expr", "")), e)


def ref_line_error(w, text, e):
    """A console line / expression may name one reference, and only as its first word: '{alias}.Func args'. It then runs
    with that reference as exec's calling reference. The console cannot run a '{alias}.Func' anywhere else."""
    if MUTANT == "refline":   # test-only twin: the check is gone
        return
    probe = PLACEHOLDER_RE.sub(lambda m: "player" if m.group(1) == "player" else "0000ABC1", text)
    try:
        split_ref_line(probe)
    except RefLineError:
        e("%s: a {alias}.Func reference call must be the first word of the line (it becomes the calling reference of "
          "exec {cmd, ref}), and a line can name only one" % w)


def validate_file(path, root=QDATA):
    p = Path(path)
    try:
        sc = load_scenario(p)
    except Exception as ex:   # noqa: BLE001
        return ["cannot read %s: %s" % (p.name, ex)]
    return validate(sc, name=p.stem, root=root)


# ---------------------------------------------------------------------------------------------------- translation
class StepError(Exception):
    pass


def needs_of(step):
    out = set(step.get("needs") or [])
    if step.get("op") in OP_NEEDS:
        out.add(OP_NEEDS[step["op"]])
    if step.get("op") == "check" and step.get("kind") == "owner":
        out.add("actor_owner")
    if step.get("op") == "check" and step.get("kind") == "spawn_count":
        out.add("ref_count")
    return out


def scenario_needs(sc):
    out = set()
    for st in all_steps(sc):
        out |= needs_of(st)
    return out


def on(who, text):
    """The exec action that runs `text` on `who` (a runtime hex id or "player"). The player keeps the "player." prefix;
    any other reference becomes exec's calling reference: ("exec", text, who). Never "<hex>.text"."""
    if MUTANT == "hexprefix" and who != "player":   # test-only twin: the old "<hex>.text" line that failed in game
        return ("exec", "%s.%s" % (who, text))
    return ("exec", "player.%s" % text) if who == "player" else ("exec", text, who)


def exec_line(line):
    """An exec action for a whole console line: a leading "<hex>.Func" moves into exec's ref (console_ref.split_ref_line).
    Raises StepError for a line that names a reference anywhere else."""
    try:
        cmd, ref = split_ref_line(line)
    except RefLineError as ex:
        raise StepError(str(ex)) from None
    return ("exec", cmd) if ref is None else ("exec", cmd, ref)


def step_actions(step, res):
    """Translate one non-check step to a list of actions:
         ("exec", console line) | ("exec", console line, calling ref hex) | ("call", method, params) | ("wait", seconds)
         | ("stage", quest hex, stage) | ("load", name) | ("save", name)
    """
    op = step["op"]
    h = res.hex
    if op == "setstage":
        return [("exec", "setstage %s %d" % (h(step["quest"]), step["stage"])), ("stage", h(step["quest"]), step["stage"])]
    if op == "setvar":
        return [("call", "test.quest.setvar", {"quest": h(step["quest"]), "var": step["var"], "value": float(step["value"])})]
    if op == "moveto":
        return [on(h(step["who"]), "moveto %s" % h(step["to"]))]
    if op == "spawn":
        return [on(h(step["at"]), "placeatme %s %d" % (h(step["base"]), step.get("count", 1)))]
    if op == "additem":
        return [on(h(step["who"]), "additem %s %d" % (h(step["item"]), step["count"]))]
    if op == "removeitem":
        return [on(h(step["who"]), "removeitem %s %d" % (h(step["item"]), step["count"]))]
    if op == "kill":
        return [on(h(step["ref"]), "kill")]
    if op == "resurrect":
        return [on(h(step["ref"]), "resurrect")]
    if op == "enable":
        return [on(h(step["ref"]), "enable")]
    if op == "disable":
        return [on(h(step["ref"]), "disable")]
    if op == "lock":
        return [on(h(step["ref"]), "lock %d" % step["level"])]
    if op == "unlock":
        return [on(h(step["ref"]), "unlock")]
    if op == "activate":
        return [on(h(step["ref"]), "activate player 1")]
    if op == "recruit":
        r = h(step["ref"])
        return [on(r, "enable"), on(r, "moveto player"), on(r, "setplayerteammate 1")]
    if op == "console":
        return [exec_line(res.subst(step["cmd"]))]
    if op == "wait":
        return [("wait", float(step["seconds"]))]
    if op == "save":
        return [("save", step["name"])]
    if op == "load":
        return [("load", step["name"])]
    if op == "talk":
        return [on(h(step["npc"]), "activate player 1")]   # opens the conversation; the greeting INFO is documentation + validated
    if op == "pick":
        return [("call", CAPABILITIES["dialogue"]["method"], {"info": res.ref(step["info"])})]
    if op == "message_click":
        return [("call", CAPABILITIES["message_box"]["method"], {"button": int(step["button"])})]
    if op == "consent":
        return [("call", CAPABILITIES["party_consent"]["method"], {"answer": step["answer"]})]
    if op == "vats_hit":
        return [("call", CAPABILITIES["vats_hit"]["method"], {"ref": h(step["target"])})]
    if op == "fast_travel":
        return [("call", "test.fasttravel", {"marker": h(step["marker"])})]
    if op == "leave":
        return [("call", "coop.leave", {})]
    if op == "join":
        return [("call", "coop.join", {})]
    raise StepError("no translation for op %s" % op)


def check_expr(step, res):
    """-> (expression for nv.eval, cmp, value, agg, tol, timeout_s, ref). The expression is None for kinds without one.
    ref is the calling reference for nv.eval(expr, ref=ref) (a runtime hex id), or None: the console has no
    "<hex>.Func" form, and the player stays the "player." prefix."""
    kind = step["kind"]
    ref = None
    if kind == "expr":
        try:
            expr, ref = split_ref_line(res.subst(step["expr"]))
        except RefLineError as ex:
            raise StepError(str(ex)) from None
    elif kind in ("owner", "spawn_count"):
        expr = None
    else:
        _, on_field, tpl = CHECK_KINDS[kind]
        fields = {}
        for k in ("quest", "ref", "who", "cell", "a", "b", "item", "base"):
            if k in step:
                fields[k] = res.hex(step[k])
        for k in ("stage", "av"):
            if k in step:
                fields[k] = step[k]
        expr = tpl.format(**fields)
        if on_field:
            ref = fields[on_field]
            if ref == "player" or MUTANT == "hexprefix":   # hexprefix: test-only twin, the old "<hex>.Func" expression
                expr, ref = "%s.%s" % (ref, expr), None
    return (expr, step.get("cmp", "=="), step.get("value"), step.get("agg"), step.get("tol", 0), float(step.get("timeout", 5)), ref)


def compare(vals, cmp_, value, agg, tol):
    """Pure comparison used by the runner. vals = one number per target player."""
    if cmp_ == "same":
        return max(vals) - min(vals) <= tol
    if agg == "sum":
        vals = [sum(vals)]
    elif agg == "min":
        vals = [min(vals)]
    elif agg == "max":
        vals = [max(vals)]
    ok = True
    for v in vals:
        if cmp_ == "==":
            ok &= abs(v - value) <= tol
        elif cmp_ == "!=":
            ok &= abs(v - value) > tol
        elif cmp_ == "<":
            ok &= v < value
        elif cmp_ == "<=":
            ok &= v <= value
        elif cmp_ == ">":
            ok &= v > value
        elif cmp_ == ">=":
            ok &= v >= value
    return bool(ok)


# ---------------------------------------------------------------------------------------------------- jump planning
WORLD_OPS = {"kill", "resurrect", "enable", "disable", "lock", "unlock", "activate", "recruit", "additem", "removeitem", "spawn",
             "console"}
WORLD_EFFECT_TYPES = {"EnableRef", "DisableRef", "KillActor", "ResurrectActor", "MoveRef"}


def stage_world_refs(quest_edid, stage, root=QDATA):
    """Refs a quest stage's script changes in the world (from L3 effects_static), or None when unknown (no data)."""
    p = Path(root) / "scenarios" / ("%s.json" % quest_edid)
    if not p.exists():
        return None
    key = ("qs", str(p))
    if key not in _CACHE:
        _CACHE[key] = json.loads(p.read_text(encoding="utf-8"))
    d = _CACHE[key]
    want = "setstage %s %d" % (quest_edid, stage)
    found = False
    out = set()
    for s in d.get("steps") or []:
        if str(s.get("host_edid", "")).lower() == want.lower():
            found = True
            for ef in (s.get("expect") or {}).get("effects_static") or []:
                if ef.get("type") in WORLD_EFFECT_TYPES and ef.get("ref"):
                    out.add(ef["ref"])
    return out if found else None


def footprint(sc, root=QDATA, order=None):
    """-> {"dirty": set(tokens), "touched": set(tokens), "quests": set(quest refs)}.
    Tokens are normalized refs, 'base:<ref>' for spawned forms, or '*' when a step changes the world in a way we cannot name
    (a raw console line, or a quest stage whose world effects are unknown)."""
    res = Resolver(sc, order)
    forms = load_forms(root)
    dirty, touched, quests = set(), set(), set()
    quests_all = load_quests(root)

    def t(v):
        if v == "player":
            return None
        return res.ref(v)

    for st in all_steps(sc):
        op = st["op"]
        for k in ("who", "to", "ref", "at", "item", "target", "marker", "cell", "a", "b", "npc", "greeting", "topic", "info"):
            if k in st and st[k] != "player" and isinstance(st[k], str):
                touched.add(t(st[k]))
        if op == "spawn" or (op == "check" and st.get("kind") == "spawn_count"):
            touched.add("base:" + t(st["base"]))
        if op == "spawn":
            dirty.add("base:" + t(st["base"]))
            dirty.add(t(st["at"]) if st["at"] != "player" else "*")
        elif op in ("setstage", "setvar"):
            q = t(st["quest"])
            quests.add(q)
            touched.add(q)
            if op == "setstage":
                qe = quests_all.get(q)
                refs = stage_world_refs(qe["edid"], st["stage"], root) if qe else None
                if refs is None:
                    dirty.add("*")
                else:
                    dirty |= {norm_ref(r) for r in refs}
        elif op == "moveto":
            if st["who"] != "player":
                dirty.add(t(st["who"]))
        elif op in WORLD_OPS:
            if op == "console":
                dirty.add("*")
            elif op in ("additem", "removeitem"):
                if st["who"] != "player":
                    dirty.add(t(st["who"]))
            else:
                dirty.add(t(st["ref"]))
        elif op == "vats_hit":
            dirty.add(t(st["target"]))
        elif op in ("load", "pick", "talk", "message_click", "consent"):
            dirty.add("*")   # a different save is in memory, or a dialogue/message result script ran   # a different save is now in memory
        elif op == "check":
            for k in ("quest", "ref", "who", "cell", "a", "b", "item"):
                if k in st and isinstance(st[k], str) and st[k] != "player":
                    touched.add(t(st[k]))
    for q in (sc.get("reset") or {}).get("quests") or []:
        quests.add(t(q))
    touched.discard(None)
    dirty.discard(None)
    return {"dirty": dirty, "touched": touched, "quests": quests}


def reset_ids(sc, order=None):
    """Runtime hex ids for test.quest.reset: the Goodsprings sweep group (L7 config) plus any quests the scenario names."""
    order = order or load_order()
    ids = []
    rs = sc.get("reset") or {}
    if rs.get("group") == "goodsprings":
        plan = sweep_config.load_config(str(QDATA / "m1_goodsprings.json"), order=order)
        ids += list(plan["reset_ids"])
    res = Resolver(sc, order)
    for q in rs.get("quests") or []:
        ids.append(res.hex(q))
    for st in all_steps(sc):
        if st["op"] in ("setstage", "setvar"):
            ids.append(res.hex(st["quest"]))
    seen, out = set(), []
    for i in ids:
        i = i.upper().zfill(8)
        if i not in seen:
            seen.add(i)
            out.append(i)
    return out


def plan_jump(state, sc, fp=None, force_reload=False):
    """Decide whether a jump to `sc` needs a save reload. Same rule as L7's reload-every mode: reload only when required.
    state = {"save", "dirty": [...], "failed": bool, "loaded": bool}. Returns {"reload": bool, "reasons": [...]}.
    A reload is required when: nothing is loaded; a different start save; the previous scenario failed or errored; the
    scenario forces it (start.reload); the previous scenario changed world objects this one touches (or changed something
    unnameable, token '*'); the scenario itself does world changes before its first reset-able point and the previous one
    dirtied anything at all."""
    reasons = []
    fp = fp or footprint(sc)
    want = (sc.get("start") or {}).get("save")
    if force_reload:
        reasons.append("reload forced")
    if not state or not state.get("loaded"):
        reasons.append("no save loaded on this instance yet")
    else:
        if state.get("save") != want:
            reasons.append("loaded save %r differs from start.save %r" % (state.get("save"), want))
        if state.get("failed"):
            reasons.append("previous scenario did not pass; the world may be damaged")
        if (sc.get("start") or {}).get("reload"):
            reasons.append("scenario sets start.reload")
        prev = set(state.get("dirty") or [])
        if "*" in prev:
            reasons.append("previous scenario changed the world in ways we cannot name ('*')")
        hit = sorted(prev & fp["touched"]) if "*" not in prev else []
        if hit:
            reasons.append("previous scenario changed refs this one uses: %s" % ", ".join(hit[:4]) + (" ..." if len(hit) > 4 else ""))
    return {"reload": bool(reasons), "reasons": reasons}
