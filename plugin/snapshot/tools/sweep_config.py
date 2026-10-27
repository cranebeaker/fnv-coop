"""Tolerant loader for questline sweep configs (questlines/<slug>/sweep.json, m1_goodsprings.json format + extras).

The 14 questline agents emit slightly different shapes; load_config() normalizes all of them to one plan:

  {"slug", "source", "loadorder": [...], "reset_ids": ["000842DD", ...], "warnings": [...],
   "quests": [{"edid", "ref": "Plugin.esm:LOCAL6", "hex": "RUNTIME8", "plugin", "setup": [cmd...], "run_last": bool,
               "stages": [{"index", "command", "setup": [cmd...], "variants": [{"name", "setup": [cmd...]}]}],
               "excluded": [{"index", "reason"}]}]}

* hex = (load-order index << 24 | local id) of `ref`; the console only accepts hex FormIDs, so DLC quests need their
  load-order prefix (DeadMoney 01, HonestHearts 02, OldWorldBlues 03, LonesomeRoad 04, ..., tools/questdata/loadorder.json).
  Any console id the config itself provides (console_id / runtime_formid / runtime_id / formid) is cross-checked and a
  mismatch is a warning (the computed one wins).
* command = the action under test (console / command / cmd / host / set_cmd / cmds[0] / default `setstage <hex> <index>`).
* setup = per-stage prerequisites in run order: quest-level setup, legion-style `setup.stages[<idx>]`, `stage_setup`,
  `chain_setup`, towns `chain` (predecessor stage numbers -> `setstage <hex> N`), stage `setup` / `suggested_setup`.
  Entries may be strings or {"cmd": ...}.
* Stages are excluded (not swept) when `sweep` is false, `exclude_reason` is set, `excluded`/`disposable_save` markers say so,
  or the quest's `excluded_stages` lists them; the reason is kept and reported.
* Order: config order, with `run_last` / `order_last` / `phase: last` / `run_mode: last` quests moved to the end.
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
LOADORDER = os.environ.get("FNVCOOP_LOADORDER") or os.path.join(REPO, "tools", "questdata", "loadorder.json")


def load_loadorder(path=LOADORDER):
    with open(path, encoding="utf-8") as f:
        return json.load(f)["plugins"]


def hex8(plugin, local, order):
    idx = [p.lower() for p in order].index(plugin.lower())
    return "%08X" % ((idx << 24) | (local & 0xFFFFFF))


def split_ref(ref):
    plug, _, loc = ref.rpartition(":")
    return plug, int(loc, 16)


def cmds(x):
    """Flatten any setup shape to a list of console command strings."""
    out = []
    if x is None or x is False:
        return out
    if isinstance(x, str):
        s = x.strip()
        return [s] if s else []
    if isinstance(x, dict):
        if "cmd" in x:
            return cmds(x["cmd"])
        if "command" in x:
            return cmds(x["command"])
        for k in ("quest", "ops", "cmds", "commands"):
            if k in x:
                out += cmds(x[k])
        return out
    if isinstance(x, (list, tuple)):
        for e in x:
            out += cmds(e)
    return out


def _stage_setup_from_quest(q, idx):
    """Quest-level per-stage setup tables: legion setup.stages, ncr stage_setup / chain_setup."""
    out = []
    su = q.get("setup")
    if isinstance(su, dict) and isinstance(su.get("stages"), dict):
        out += cmds(su["stages"].get(str(idx)))
    for key in ("stage_setup", "chain_setup"):
        v = q.get(key)
        if isinstance(v, dict):
            out += cmds(v.get(str(idx)))
    return out


def _variants(s, hexid):
    out = []
    for v in s.get("variants") or []:
        if isinstance(v, dict):
            out.append({"name": str(v.get("name") or v.get("id") or "variant%d" % (len(out) + 1)),
                        "setup": cmds(v.get("setup") if "setup" in v else v.get("cmds")),
                        "expect": v.get("expect")})
    return out


def _excluded_reason(s):
    if s.get("sweep") is False:
        return s.get("exclude_reason") or "sweep=false"
    if s.get("exclude_reason"):
        return s["exclude_reason"]
    if s.get("excluded"):
        return s.get("reason") or "excluded"
    return None


def load_config(path, order=None, quests_by_edid=None):
    order = order or load_loadorder()
    with open(path, encoding="utf-8") as f:
        cfg = json.load(f)
    slug = cfg.get("slug") or os.path.basename(os.path.dirname(os.path.abspath(path))) or "sweep"
    if os.path.basename(path) == "m1_goodsprings.json":
        slug = "goodsprings"
    plan = {"slug": slug, "source": path, "loadorder": order, "reset_ids": [], "warnings": [], "quests": []}
    warn = plan["warnings"].append
    top_last = set(cfg.get("order_last") or [])
    reset = []
    for k in ("reset_quests", "reset_ids"):
        reset += [str(x) for x in (cfg.get(k) or [])]
    for op in cfg.get("reset") or []:
        if isinstance(op, dict):
            reset += [str(x) for x in op.get("quests", [])]
    qs = cfg.get("quests")
    if isinstance(qs, dict):
        qs = list(qs.values())
    for q in qs or []:
        edid, ref = q.get("edid"), q.get("ref")
        if not edid or not ref or ":" not in ref:
            warn("quest without edid/ref skipped: %r" % (edid or q.get("formid")))
            continue
        try:
            plug, local = split_ref(ref)
            hx = hex8(plug, local, order)
        except (ValueError, IndexError):
            warn("%s: plugin of %s is not in the load order, skipped" % (edid, ref))
            continue
        given = q.get("console_id") or q.get("runtime_formid") or q.get("runtime_id") or q.get("formid")
        if given and str(given).upper().lstrip("0") != hx.lstrip("0"):
            warn("%s: config id %s differs from computed %s (computed wins)" % (edid, given, hx))
        qsetup = cmds(q.get("setup") if not isinstance(q.get("setup"), dict) else q["setup"].get("quest"))
        for k in ("reset_ids",):
            reset += [str(x) for x in (q.get(k) or [])]
        if isinstance(q.get("reset"), list):
            reset += [str(x) for x in q["reset"] if isinstance(x, str)]
        run_last = bool(q.get("run_last") or q.get("order_last") or edid in top_last or q.get("phase") == "last")
        excl = []
        for e in q.get("excluded_stages") or []:
            if isinstance(e, dict):
                excl.append({"index": e.get("index", e.get("stage")), "reason": e.get("reason", "excluded")})
            elif isinstance(e, int):
                excl.append({"index": e, "reason": "excluded_stages"})
        excl_idx = {e["index"] for e in excl}
        stages = []
        for s in q.get("stages") or []:
            if isinstance(s, int):
                s = {"index": s}
            idx = s.get("index")
            if idx is None:
                continue
            reason = _excluded_reason(s) or ("listed in excluded_stages" if idx in excl_idx else None)
            if reason:
                excl.append({"index": idx, "reason": reason})
                continue
            command = None
            for k in ("console", "command", "cmd", "host", "set_cmd"):
                if s.get(k):
                    command = cmds(s[k])[0] if cmds(s[k]) else None
                    break
            if command is None and s.get("cmds"):
                command = cmds(s["cmds"])[0] if cmds(s["cmds"]) else None
            if command is None:
                command = "setstage %s %d" % (hx, idx)
            setup = _stage_setup_from_quest(q, idx)
            setup += ["setstage %s %d" % (hx, int(n)) for n in (s.get("chain") or []) if isinstance(n, int)]
            setup += cmds(s.get("setup")) + cmds(s.get("suggested_setup"))
            if s.get("run_mode") == "last" or s.get("phase") == "last":
                run_last = run_last or False
            stages.append({"index": idx, "command": command, "setup": setup, "variants": _variants(s, hx),
                           "last": s.get("phase") == "last" or s.get("run_mode") == "last" or bool(s.get("disposable_save"))})
        plan["quests"].append({"edid": edid, "ref": "%s:%06X" % (plug, local), "hex": hx, "plugin": plug,
                               "setup": qsetup, "run_last": run_last, "stages": stages, "excluded": excl})
    plan["quests"].sort(key=lambda q: q["run_last"])   # stable: config order otherwise
    for q in plan["quests"]:
        q["stages"].sort(key=lambda s: s["last"])      # stable: stages keep their order, 'last' ones after
    seen = []
    by_local = {}
    for q in plan["quests"]:
        by_local.setdefault(int(q["hex"], 16) & 0xFFFFFF, []).append(q)
    for r in reset:
        r = "%08X" % int(r, 16)
        if r[:2] == "00":
            # A reset id without a load-order prefix that names a DLC quest of this config would reset the WRONG form
            # (the same local id in FalloutNV.esm, or nothing): upgrade it to the quest's real runtime id.
            dlc = [q for q in by_local.get(int(r, 16) & 0xFFFFFF, []) if not q["hex"].startswith("00")]
            if dlc and not any(q["hex"] == r for q in by_local.get(int(r, 16) & 0xFFFFFF, [])):
                warn("reset id %s has no load-order prefix; using %s (%s)" % (r, dlc[0]["hex"], dlc[0]["edid"]))
                r = dlc[0]["hex"]
        if r not in seen:
            seen.append(r)
    plan["reset_ids"] = seen or [q["hex"] for q in plan["quests"] if q["stages"]]
    return plan


def plan_stats(plan):
    stages = sum(len(q["stages"]) for q in plan["quests"])
    variants = sum(max(1, len(s["variants"])) for q in plan["quests"] for s in q["stages"])
    excluded = sum(len(q["excluded"]) for q in plan["quests"])
    return {"quests": len(plan["quests"]), "quests_with_stages": sum(1 for q in plan["quests"] if q["stages"]),
            "stages": stages, "runs": variants, "excluded": excluded}


_VERB = re.compile(r"^\s*([A-Za-z0-9_.]+)")


def verb(cmd):
    m = _VERB.match(cmd)
    v = m.group(1).lower() if m else ""
    return "set" if v == "set" else (v.split(".", 1)[1] if v.startswith(("player.", "playerref.")) and "." in v else v)
