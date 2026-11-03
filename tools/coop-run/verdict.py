"""Read the logs of one run and decide: pass or fail, and why. Pure functions, no processes.

Failure classes (the `class` field of verdict.json):
  launch   a viewer could not be started
  timeout  the run or a viewer did not finish in time
  crash    a viewer exited with an error, or vanished without a clean shutdown event
  log      a viewer logged an error event
  hollow   the run proved nothing: never connected, no traffic on the relay, no state dump,
           the chaos profile dropped nothing
  desync   shared state differs between host and guest
  assert   an expected end-state check failed
  repair   more desync repairs than the scenario allows
  harness  the runner itself failed
"""
import json
import os

import scenario as scn_mod

CLASSES = ("launch", "timeout", "crash", "log", "hollow", "desync", "assert", "repair", "harness")
PACKET_LIMIT = 3000  # bytes of JSON, under the 3 KB rule


def read_jsonl(path):
    """Returns (events, bad_line_count). A missing file gives ([], 0)."""
    events, bad = [], 0
    if not path or not os.path.exists(path):
        return events, bad
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                e = json.loads(line)
                if isinstance(e, dict) and "ev" in e:
                    events.append(e)
                else:
                    bad += 1
            except ValueError:
                bad += 1
    return events, bad


def read_json(path):
    if not path or not os.path.exists(path):
        return None
    try:
        with open(path, encoding="utf-8") as f:
            v = json.load(f)
        return v if isinstance(v, dict) else None
    except (ValueError, OSError):
        return None


def peer_summary(role, p):
    """p: {exit_code, timed_out, launch_error, log, state, stdout_tail} -> facts about one peer."""
    events, bad = read_jsonl(p.get("log"))
    names = [e["ev"] for e in events]
    shut = [e for e in events if e["ev"] == "shutdown"]
    return {
        "role": role,
        "exit_code": p.get("exit_code"),
        "timed_out": bool(p.get("timed_out")),
        "launch_error": p.get("launch_error"),
        "events": len(events),
        "bad_lines": bad,
        "started": "start" in names,
        "ready": "coop_ready" in names,
        "shutdown_clean": bool(shut and shut[-1].get("clean")),
        "errors": [e.get("msg", "") for e in events if e["ev"] == "error"],
        "repairs": sum(1 for n in names if n == "coop_repair"),
        "state": read_json(p.get("state")),
        "tail": events[-6:],
        "stdout_tail": p.get("stdout_tail", ""),
    }


def _relay_compact(r):
    if not r:
        return None
    out = {"profile": r.get("profile"), "armed": r.get("armed")}
    for d in ("g2h", "h2g"):
        s = r.get(d, {})
        out[d] = {k: s.get(k, 0) for k in ("in", "forwarded", "dropped", "duplicated", "reordered")}
    return out


def judge(scn, run):
    """run: {host, guest (peer dicts as for peer_summary), relay (stats dict or None), run_id,
    rerun, timed_out_run (bool), harness_error (str or None), duration_s, seed}"""
    checks = []
    host = peer_summary("host", run["host"])
    guest = peer_summary("guest", run["guest"])
    extras = tuple(peer_summary(r, run[r]) for r in ("guest2", "guest3") if run.get(r))
    everyone = (host, guest) + extras
    relay = run.get("relay")
    cls, reason = None, ""

    def fail(c, why):
        nonlocal cls, reason
        if cls is None:
            cls, reason = c, why

    if run.get("harness_error"):
        fail("harness", run["harness_error"])
    for p in everyone:
        if p["launch_error"]:
            fail("launch", "%s: %s" % (p["role"], p["launch_error"]))
    # a peer that died on its own is a crash; one we had to stop is a timeout
    for p in everyone:
        if p["exit_code"] not in (0, None) and not p["timed_out"]:
            fail("crash", "%s exited with code %s" % (p["role"], p["exit_code"]))
    if run.get("timed_out_run"):
        fail("timeout", "run exceeded timeout_s=%s" % scn["timeout_s"])
    for p in everyone:
        if p["timed_out"]:
            fail("timeout", "%s did not exit after the quit request" % p["role"])
    for p in everyone:
        if p["exit_code"] == 0 and p["started"] and not p["shutdown_clean"]:
            fail("crash", "%s has no clean shutdown event" % p["role"])
    for p in everyone:
        if p["errors"]:
            fail("log", "%s logged error: %s" % (p["role"], p["errors"][0][:120]))

    # hollow-pass guards: assert the mode we think we are in
    for p in everyone:
        if not p["started"]:
            fail("hollow", "%s wrote no start event (log missing or empty)" % p["role"])
        elif not p["ready"]:
            fail("hollow", "%s never reached coop_ready (single player fallback or no join)" % p["role"])
        elif p["state"] is None:
            fail("hollow", "%s wrote no state dump" % p["role"])
        elif p["state"].get("role") not in (None, p["role"], p["role"].rstrip("0123456789")):
            fail("hollow", "%s state dump says role %s" % (p["role"], p["state"].get("role")))
    if relay is None:
        fail("hollow", "no relay stats")
    else:
        g, h = relay.get("g2h", {}), relay.get("h2g", {})
        if not relay.get("armed") or g.get("in", 0) == 0 or h.get("in", 0) == 0:
            fail("hollow", "relay saw no traffic in both directions (g2h=%s h2g=%s)" % (g.get("in", 0), h.get("in", 0)))
        else:
            from chaos_relay import PROFILES
            prof = PROFILES.get(scn["network"])
            # only the datagrams that met the scenario's profile (the settle window is perfect)
            n = g.get("chaos_in", g["in"]) + h.get("chaos_in", h["in"])
            did = sum(d.get(k, 0) for d in (g, h) for k in ("dropped", "duplicated", "reordered"))
            if prof and prof.loss > 0 and n * prof.loss >= 8 and did == 0:
                fail("hollow", "profile %s should have dropped about %d of %d datagrams, relay changed none"
                     % (scn["network"], round(n * prof.loss), n))
    for o in scn["outages"]:
        evs = (relay or {}).get("events", [])
        if not any(e.get("ev") == "outage" for e in evs):
            fail("hollow", "scheduled outage never fired")
            break

    # state checks (only when both dumps exist)
    if all(p["state"] is not None for p in everyone):
        res = scn_mod.evaluate(scn["expect"], host["state"], guest["state"],
                               {p["role"]: p["state"] for p in extras} or None)
        checks += res
        auto = scn.get("hash_check", True)
        hh = scn_mod.get_path(host["state"], "hash.shared")
        for p in everyone[1:]:
            gh = scn_mod.get_path(p["state"], "hash.shared")
            if auto and hh is not scn_mod._MISSING and gh is not scn_mod._MISSING:
                ok, detail = scn_mod.hash_match(host["state"], p["state"])
                checks.append({"id": "auto.hash.shared" if p is guest else "auto.hash.shared#" + p["role"],
                               "ok": ok, "kind": "desync",
                               "detail": detail.replace("guest", p["role"], 1)})
        bad = [c for c in checks if not c["ok"]]
        for kind in ("desync", "assert"):
            first = next((c for c in bad if c["kind"] == kind), None)
            if first:
                fail(kind, "%s: %s" % (first["id"], first["detail"]))
    max_rep = scn["max_repairs"]
    if max_rep is None and scn["network"] == "perfect":
        max_rep = 0
    total_rep = sum(p["repairs"] for p in everyone)
    if max_rep is not None and total_rep > max_rep:
        fail("repair", "%d repairs (host %d, guest %d), allowed %d" %
             (total_rep, host["repairs"], guest["repairs"], max_rep))

    twin_note = ""
    if scn.get("expect_fail"):
        # A mutation twin: its assertions are meant to fail. Only an assert or desync failure
        # counts as the expected one; a crash, hollow run or timeout is still a real failure.
        if cls in ("assert", "desync"):
            twin_note = "twin failed as expected (%s)" % reason
            cls, reason = None, twin_note
        elif cls is None:
            cls, reason = "twin", "twin passed: its assertions did not fail, so they cannot tell the feature from nothing"

    v = {
        "schema": 1,
        "scenario": scn["name"],
        "run_id": run.get("run_id", ""),
        "result": "fail" if cls else "pass",
        "class": cls or ("twin" if twin_note else "pass"),
        "reason": reason or "all checks passed",
        "network": scn["network"],
        "seed": run.get("seed", scn["seed"]),
        "duration_s": round(run.get("duration_s", 0), 1),
        "rerun": run.get("rerun", ""),
        "checks": [{"id": c["id"], "ok": c["ok"], "detail": c["detail"][:160]} for c in checks],
        "relay": _relay_compact(relay),
        "peers": {p["role"]: {k: p[k] for k in ("exit_code", "timed_out", "events", "ready", "shutdown_clean", "repairs")}
                  for p in everyone},
    }
    if cls:
        v["packet"] = build_packet(scn, v, host, guest)
    return v


def _clip(s, n):
    s = str(s)
    return s if len(s) <= n else s[:n - 3] + "..."


def build_packet(scn, v, host, guest, limit=PACKET_LIMIT):
    """A compact failure report for a human or an agent: always under `limit` bytes of JSON."""
    failed = [{"id": c["id"], "detail": c["detail"]} for c in v["checks"] if not c["ok"]]
    for tail_n in (4, 2, 1, 0):
        for clip in (160, 80, 40):
            pk = {
                "scenario": scn["name"], "class": v["class"], "reason": _clip(v["reason"], 200),
                "network": scn["network"], "seed": v["seed"], "rerun": _clip(v["rerun"], 200),
                "failed_checks": [{"id": f["id"], "detail": _clip(f["detail"], clip)} for f in failed[:4]],
                "relay": v["relay"],
                "exit": {"host": host["exit_code"], "guest": guest["exit_code"]},
                "ready": {"host": host["ready"], "guest": guest["ready"]},
                "tail": {
                    "host": [_tail_ev(e, clip) for e in host["tail"][-tail_n:]] if tail_n else [],
                    "guest": [_tail_ev(e, clip) for e in guest["tail"][-tail_n:]] if tail_n else [],
                },
                "stdout": {"host": _clip(host["stdout_tail"][-200:], clip), "guest": _clip(guest["stdout_tail"][-200:], clip)},
            }
            if len(json.dumps(pk)) < limit:
                return pk
    pk.pop("tail", None)
    pk.pop("stdout", None)
    pk["failed_checks"] = pk["failed_checks"][:1]
    return pk


def _tail_ev(e, clip):
    return {k: (_clip(x, clip) if isinstance(x, str) else x) for k, x in e.items()}


def scorer_selftest():
    """Feed known bad runs to judge(); every one must FAIL with the right class.
    Returns a list of problems (empty means the scorer works)."""
    import tempfile
    problems = []
    s = scn_mod.validate({"name": "selftest", "cell": "X", "network": "perfect",
                          "expect": [{"id": "q", "path": "quests.Q.stage", "op": "eq", "value": 1}]})
    good_state = {"role": "x", "quests": {"Q": {"stage": 1}}, "hash": {"shared": "a"}}
    relay_ok = {"armed": True, "profile": "perfect", "g2h": {"in": 5}, "h2g": {"in": 5}, "events": []}
    with tempfile.TemporaryDirectory() as d:
        counter = [0]

        def peer(role, events=None, state=True, exit_code=0, **kw):
            counter[0] += 1
            lg = os.path.join(d, "%s.%d.jsonl" % (role, counter[0]))
            with open(lg, "w", encoding="utf-8") as f:
                for e in (events if events is not None else
                          [{"ev": "start"}, {"ev": "coop_ready"}, {"ev": "shutdown", "clean": True}]):
                    f.write(json.dumps(e) + "\n")
            st = os.path.join(d, "%s.%d.state.json" % (role, counter[0]))
            if state:
                with open(st, "w", encoding="utf-8") as f:
                    json.dump(dict(good_state, role=role), f)
            return dict({"exit_code": exit_code, "log": lg, "state": st}, **kw)

        def run(host, guest, relay=relay_ok, **kw):
            return judge(s, dict({"host": host, "guest": guest, "relay": relay}, **kw))

        cases = [
            ("pass baseline", "pass", run(peer("host"), peer("guest"))),
            ("missing result lines", "hollow", run(peer("host", events=[]), peer("guest"))),
            ("never ready", "hollow", run(peer("host", events=[{"ev": "start"}, {"ev": "shutdown", "clean": True}]), peer("guest"))),
            ("no state dump", "hollow", run(peer("host", state=False), peer("guest"))),
            ("no relay traffic", "hollow", run(peer("host"), peer("guest"), relay={"armed": False, "g2h": {"in": 0}, "h2g": {"in": 0}})),
            ("timeout", "timeout", run(peer("host", timed_out=True), peer("guest"))),
            ("whole run timeout", "timeout", run(peer("host"), peer("guest"), timed_out_run=True)),
            ("crash code", "crash", run(peer("host", exit_code=3), peer("guest"))),
            ("no shutdown event", "crash", run(peer("host", events=[{"ev": "start"}, {"ev": "coop_ready"}]), peer("guest"))),
            ("error event", "log", run(peer("host", events=[{"ev": "start"}, {"ev": "coop_ready"}, {"ev": "error", "msg": "x"}, {"ev": "shutdown", "clean": True}]), peer("guest"))),
            ("launch error", "launch", run(peer("host", launch_error="no exe"), peer("guest"))),
            ("repairs on perfect link", "repair", run(peer("host", events=[{"ev": "start"}, {"ev": "coop_ready"}, {"ev": "coop_repair"}, {"ev": "shutdown", "clean": True}]), peer("guest"))),
        ]
        for name, want, v in cases:
            if v["class"] != want:
                problems.append("%s: wanted %s, got %s (%s)" % (name, want, v["class"], v["reason"]))
            if want != "pass" and v["result"] != "fail":
                problems.append("%s: result was not fail" % name)
        # state-based failures need doctored dumps
        for name, want, edit in (("assert", "assert", lambda st: st["quests"]["Q"].update(stage=2)),
                                 ("desync hash", "desync", lambda st: st["hash"].update(shared="b"))):
            h, g = peer("host"), peer("guest")
            st = read_json(g["state"])
            edit(st)
            with open(g["state"], "w", encoding="utf-8") as f:
                json.dump(st, f)
            v = run(h, g)
            if v["class"] != want:
                problems.append("%s: wanted %s, got %s" % (name, want, v["class"]))
    return problems
