"""Scenario files: load, validate, turn into viewer command lines, judge end state.

A scenario is one JSON file (see scenarios/ and README.md). Nothing here starts a process.
"""
import json
import os

from chaos_relay import PROFILES

SCHEMA = 1
OPS = ("eq", "ne", "ge", "le", "gt", "lt", "near", "present", "absent", "dist_from", "sum")
ON = ("host", "guest", "both", "equal")
SCRIPT_KEYS = ("run", "stage", "talk", "choose", "say", "key")


class ScenarioError(ValueError):
    pass


def load(path):
    with open(path, encoding="utf-8") as f:
        raw = json.load(f)
    raw.setdefault("name", os.path.splitext(os.path.basename(path))[0])
    return validate(raw)


def validate(s):
    """Fill defaults, raise ScenarioError listing every problem."""
    errs = []
    if not isinstance(s, dict):
        raise ScenarioError("scenario must be a JSON object")
    s = dict(s)
    s.setdefault("schema", SCHEMA)
    s.setdefault("description", "")
    s.setdefault("new_game", False)
    s.setdefault("network", "perfect")
    s.setdefault("seed", 1)
    s.setdefault("duration_s", 20)
    s.setdefault("settle_s", 3)
    s.setdefault("timeout_s", 180)
    s.setdefault("outages", [])
    s.setdefault("max_repairs", None)
    s.setdefault("expect", [])
    s.setdefault("expect_fail", False)
    for role in ("host", "guest"):
        s.setdefault(role, {})
        s[role].setdefault("script", [])
        s[role].setdefault("extra_args", [])
    if not s.get("name") or not isinstance(s["name"], str):
        errs.append("name is required")
    if not s.get("cell") and not s["new_game"]:
        errs.append("cell is required unless new_game is true")
    if s["network"] not in PROFILES:
        errs.append("network %r is not a profile (%s)" % (s["network"], ", ".join(sorted(PROFILES))))
    for k in ("duration_s", "settle_s", "timeout_s"):
        if not isinstance(s[k], (int, float)) or s[k] < 0:
            errs.append("%s must be a number >= 0" % k)
    if not isinstance(s["seed"], int):
        errs.append("seed must be an integer")
    if s.get("chaos_s") is not None and not isinstance(s["chaos_s"], (int, float)):
        errs.append("chaos_s must be a number")
    for i, o in enumerate(s["outages"]):
        if o.get("dir", "both") not in ("both", "g2h", "h2g") or "at" not in o or "seconds" not in o:
            errs.append("outages[%d] needs at, seconds and optional dir both|g2h|h2g" % i)
    for role in ("host", "guest"):
        for i, line in enumerate(s[role]["script"]):
            keys = [k for k in SCRIPT_KEYS if k in line]
            if len(keys) != 1:
                errs.append("%s.script[%d] needs exactly one of %s" % (role, i, "/".join(SCRIPT_KEYS)))
                continue
            k = keys[0]
            if "at" in line and (not isinstance(line["at"], (int, float)) or line["at"] < 0):
                errs.append("%s.script[%d].at must be seconds >= 0" % (role, i))
            if "at" in line and k in ("talk", "choose", "say"):
                errs.append("%s.script[%d]: %s cannot have a time (it is a startup flag)" % (role, i, k))
            if k == "stage" and (not isinstance(line["stage"], str) or len(line["stage"].split()) != 2):
                errs.append('%s.script[%d].stage must be "QUEST STAGE"' % (role, i))
            if k == "key":
                if not isinstance(line["key"], str) or not line["key"].strip() or len(line["key"].split()) != 1:
                    errs.append('%s.script[%d].key must be "KEY" or "KEY:HOLD" (no spaces)' % (role, i))
                elif ":" in line["key"]:
                    hold = line["key"].split(":", 1)[1]
                    try:
                        if float(hold) <= 0:
                            raise ValueError
                    except ValueError:
                        errs.append("%s.script[%d].key hold must be a number > 0" % (role, i))
                if "at" not in line:
                    errs.append("%s.script[%d].key needs a time (at)" % (role, i))
            if k == "choose" and not (isinstance(line["choose"], list) and all(isinstance(n, int) for n in line["choose"])):
                errs.append("%s.script[%d].choose must be a list of integers" % (role, i))
    ids = set()
    for i, e in enumerate(s["expect"]):
        eid = e.get("id", "e%d" % i)
        e["id"] = eid
        if eid in ids:
            errs.append("expect id %r repeated" % eid)
        ids.add(eid)
        e.setdefault("op", "eq")
        e.setdefault("on", "both")
        if e["op"] not in OPS:
            errs.append("expect %s: op %r not in %s" % (eid, e["op"], OPS))
        if e["on"] not in ON:
            errs.append("expect %s: on %r not in %s" % (eid, e["on"], ON))
        if not isinstance(e.get("path"), str) or not e["path"]:
            errs.append("expect %s: path is required" % eid)
        if e["on"] != "equal" and e["op"] not in ("present", "absent") and "value" not in e:
            errs.append("expect %s: value is required" % eid)
        if e["op"] == "near" and "tol" not in e:
            errs.append("expect %s: near needs tol" % eid)
        if e["op"] == "sum":
            add = e.get("add", [])
            if not (isinstance(add, list) and all(isinstance(x, str) for x in add)):
                errs.append("expect %s: sum add must be a list of paths" % eid)
            if e.get("cmp") not in ("eq", "ge", "le", "gt", "lt"):
                errs.append("expect %s: sum needs cmp eq|ge|le|gt|lt" % eid)
            if not isinstance(e.get("value"), (int, float)):
                errs.append("expect %s: sum needs a numeric value" % eid)
        if e["op"] == "dist_from":
            ref = e.get("from")
            if not (isinstance(ref, list) and len(ref) >= 2 and all(isinstance(n, (int, float)) for n in ref)):
                errs.append("expect %s: dist_from needs from: [x, y(, z)]" % eid)
            if e.get("cmp") not in ("ge", "le", "gt", "lt"):
                errs.append("expect %s: dist_from needs cmp ge|le|gt|lt" % eid)
            if not isinstance(e.get("value"), (int, float)):
                errs.append("expect %s: dist_from needs a numeric value (units)" % eid)
    if errs:
        raise ScenarioError("; ".join(errs))
    return s


def chaos_seconds(s):
    return s["duration_s"] if s.get("chaos_s") is None else s["chaos_s"]


def relay_schedule(s):
    """Timed relay actions, seconds after the relay is armed (first guest datagram)."""
    out = [{"at": o["at"], "outage": o.get("dir", "both"), "seconds": o["seconds"]} for o in s["outages"]]
    if s["network"] != "perfect":
        out.append({"at": chaos_seconds(s), "profile": "perfect"})
    return sorted(out, key=lambda a: a["at"])


def script_args(script):
    """Viewer flags for a script (list of lines) using the existing args.rs flags."""
    args = []
    for line in script:
        if "run" in line:
            if "at" in line:
                args += ["--run-at", _num(line["at"]), line["run"]]
            else:
                args += ["--run", line["run"]]
        elif "stage" in line:
            if "at" in line:
                args += ["--run-at", _num(line["at"]), "SetStage " + line["stage"]]
            else:
                args += ["--stage"] + line["stage"].split()
        elif "talk" in line:
            if line["talk"]:
                args += ["--talk"]
        elif "choose" in line:
            args += ["--choose", ",".join(str(n) for n in line["choose"])]
        elif "say" in line:
            args += ["--say", line["say"]]
        elif "key" in line:
            args += ["--key-at", _num(line["at"]), line["key"]]
    return args


def _num(x):
    return str(int(x)) if float(x).is_integer() else repr(float(x))


def viewer_args(s, role, data, ports, files, quit_after, background=True, screen_size="640,360"):
    """Arguments after the executable. ports: {'host': int, 'join': 'ADDR:PORT'}.
    files: {'log','state','quit'} for this role."""
    args = [data]
    if s.get("cell"):
        args.append(s["cell"])
    if background:
        args += ["--background", "--screen-size", screen_size]
    args += ["--no-movies", "--no-hud", "--answer-boxes"]
    if s["new_game"]:
        args.append("--new-game")
    if s.get("character"):
        args += ["--character", s["character"]]
    args += script_args(s[role]["script"])
    args += s[role]["extra_args"]
    if role == "host":
        args += ["--coop-host", str(ports["host"])]
    else:
        args += ["--coop-join", ports["join"]]
    # The harness joins by plain HOST:PORT through a relay, which has no per-session token:
    # both sides use the fixed test token (the viewer refuses a plain address without it).
    args.append("--coop-test-token")
    args += ["--coop-log", files["log"], "--coop-state-dump", files["state"],
             "--coop-quit-file", files["quit"], "--coop-quit-after", _num(quit_after)]
    return args


# ---- assertions on the shared state -------------------------------------------------

_MISSING = object()


def get_path(state, path):
    cur = state
    for part in path.split("."):
        if isinstance(cur, dict) and part in cur:
            cur = cur[part]
        elif isinstance(cur, list) and part.isdigit() and int(part) < len(cur):
            cur = cur[int(part)]
        else:
            return _MISSING
    return cur


def _near(a, b, tol):
    if isinstance(a, (list, tuple)) and isinstance(b, (list, tuple)):
        n = min(len(a), len(b))  # [x,y,z] against [x,y,z,heading] compares the common prefix
        return n > 0 and all(_near(a[i], b[i], tol) for i in range(n))
    try:
        return abs(float(a) - float(b)) <= tol
    except (TypeError, ValueError):
        return False


def _dims(v, e):
    """Optional "dims": N keeps the first N components of a list (2 = the XY plane)."""
    n = e.get("dims")
    return v[:n] if isinstance(n, int) and isinstance(v, (list, tuple)) else v


def _cmp(op, got, want, tol):
    try:
        if op == "eq":
            return got == want
        if op == "ne":
            return got != want
        if op == "ge":
            return got >= want
        if op == "le":
            return got <= want
        if op == "gt":
            return got > want
        if op == "lt":
            return got < want
        if op == "near":
            return _near(got, want, tol)
    except TypeError:
        return False
    return False


def dist_xy(a, b):
    """Distance in the XY plane between two points (lists with at least x and y), or None."""
    try:
        return ((float(a[0]) - float(b[0])) ** 2 + (float(a[1]) - float(b[1])) ** 2) ** 0.5
    except (TypeError, ValueError, IndexError, KeyError):
        return None


def _short(v):
    t = "<missing>" if v is _MISSING else json.dumps(v, sort_keys=True)
    return t if len(t) <= 80 else t[:77] + "..."


def evaluate(expect, host_state, guest_state):
    """Returns a list of {id, ok, kind, detail}. kind is 'assert' or 'desync' (equal checks)."""
    out = []
    peers = {"host": host_state, "guest": guest_state}
    for e in expect:
        eid, path, op = e["id"], e["path"], e["op"]
        if e["on"] == "equal":
            a, b = get_path(host_state, path), get_path(guest_state, path)
            if a is _MISSING or b is _MISSING:
                ok = False
            elif "tol" in e:
                ok = _near(_dims(a, e), _dims(b, e), e["tol"])
            else:
                ok = a == b
            out.append({"id": eid, "ok": ok, "kind": "desync",
                        "detail": "%s host=%s guest=%s" % (path, _short(a), _short(b))})
            continue
        who = ("host", "guest") if e["on"] == "both" else (e["on"],)
        for w in who:
            got = get_path(peers[w], path)
            if op == "sum":
                parts = [path] + list(e.get("add", []))
                vals = [get_path(peers[w], x) for x in parts]
                total = sum(v for v in vals if isinstance(v, (int, float)) and not isinstance(v, bool))
                ok = _cmp(e["cmp"], total, e["value"], 0)
                out.append({"id": eid if len(who) == 1 else "%s@%s" % (eid, w), "ok": ok, "kind": "assert",
                            "detail": "%s sum(%s) %s %s (got %s; missing counts 0)" % (
                                w, ", ".join(parts), e["cmp"], e["value"], total)})
                continue
            if op == "dist_from":
                d = None if got is _MISSING else dist_xy(got, e["from"])
                ok = d is not None and _cmp(e["cmp"], d, e["value"], 0)
                out.append({"id": eid if len(who) == 1 else "%s@%s" % (eid, w), "ok": ok, "kind": "assert",
                            "detail": "%s %s dist_from %s %s %s (got %s)" % (
                                w, path, _short(e["from"]), e["cmp"], e["value"],
                                "<missing>" if d is None else "%.1f" % d)})
                continue
            if op == "present":
                ok = got is not _MISSING
            elif op == "absent":
                ok = got is _MISSING
            elif got is _MISSING:
                ok = False
            else:
                ok = _cmp(op, _dims(got, e), _dims(e.get("value"), e), e.get("tol", 0))
            want = "" if op in ("present", "absent") else " %s" % _short(e.get("value"))
            out.append({"id": eid if len(who) == 1 else "%s@%s" % (eid, w), "ok": ok, "kind": "assert",
                        "detail": "%s %s %s%s (got %s)" % (w, path, op, want, _short(got))})
    return out
