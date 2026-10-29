"""The step engine of the story run: real player actions on a host copy and a client copy, a goal per step, the first divergence.

A step is a dict of tools/story/goodsprings.json. `run_step(ctx, step, script, rep)` plays it through a lab_core.Ctx (so the time box, fail-fast checks,
the digest clean rule, evidence and the crash probe are the lab's own) and fills `rep` with what the report shows: the picks of the initiator, the
stages and objectives on both copies before and after, the items, XP, karma and reputation each copy gained, and the first value the copies disagree on.

StoryMachine is the lab's Machine plus the story verbs (talk, choices, say, quests, a state snapshot). tools/story/story_live.py drives the bridge,
tools/story/storyfake.py is the game-free world. STORY_MUTANT=<name> switches on one deliberate bug of the runner for the twins in run_tests.py.
"""
from __future__ import annotations

import math
import sys
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (HERE, ROOT / "tools" / "lab"):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_core as K                                  # noqa: E402
from lab_core import CLIENT, HOST, Blocked, Ctx, Machine   # noqa: E402
import story_data as D                                # noqa: E402

MUTANT = D.MUTANT

# the numbers a step compares against (--param name=value changes one for a run); the first in-game run may move them
PARAMS: Dict[str, float] = {
    "story.settle_s": 20.0,          # after the goal is met on one copy, the other copy shows it within this
    "story.prep_s": 25.0,            # a console prep (host) shows on the client within this
    "story.walk_s": 25.0,            # a walk to a ref; a walk that fails is replaced by a teleport and the step says so
    "story.talk_s": 20.0,            # the dialogue menu opens within this (a guest waits for the host to grant the conversation)
    "story.choices_s": 15.0,         # the open dialogue offers its lines within this
    "story.pick_s": 10.0,            # after a pick, the next lines (or the end of the dialogue) show within this
    "story.volleys": 24.0,
    "story.weak_hp": 5.0,            # the host (owner) sets a target's health this low before the shooting, so one pistol hit kills it (the lab's fight room does the same)
    "story.weak_s": 15.0,            # the weakened health shows on both copies within this
    "story.gone_s": 15.0,            # a picked-up plant is gone on the other copy within this
    "story.settle_world_s": 30.0,    # message boxes closed on both copies and the puppets back in their players' cells within this (before a shot or a talk)
}


class StoryMachine(Machine):
    """What the story steps need on top of the lab's Machine. Methods that act return (ok, why)."""

    def quest_running(self, quest: str) -> bool: raise NotImplementedError
    def stage_done(self, quest: str, stage: int) -> bool: raise NotImplementedError
    def objective(self, quest: str, obj: int) -> Tuple[bool, bool]: raise NotImplementedError       # (displayed, completed)
    def start_quest(self, quest: str) -> Tuple[bool, str]: raise NotImplementedError
    def state(self) -> Dict[str, Any]: raise NotImplementedError                                     # {shared, perPlayer} (docs/snapshot-schema.md)
    def walk_ref(self, ref: str, dist: float, timeout_s: float, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]: raise NotImplementedError
    def talk(self, npc: str, timeout_s: float) -> Tuple[bool, str]: raise NotImplementedError
    def choices(self) -> List[Dict[str, Any]]: raise NotImplementedError                            # [{idx, info}] of the open dialogue; [] when closed
    def in_dialogue(self) -> bool: raise NotImplementedError
    def say(self, info: str, timeout_s: float) -> Tuple[bool, str, Dict[str, Any]]: raise NotImplementedError
    def end_dialogue(self) -> Tuple[bool, str]: raise NotImplementedError
    def activate(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def enable_controls(self) -> Tuple[bool, str]: raise NotImplementedError                          # console EnablePlayerControls: a quest script's DisablePlayerControls may still hold
    def scene_pass(self, quest: str) -> Tuple[bool, str]: raise NotImplementedError                   # this player's own pass of a barrier stage (coop.scenePass)
    def message_box(self) -> bool: raise NotImplementedError                                          # a message box is the top menu of this copy
    def press_box(self, button: int) -> Tuple[bool, str]: raise NotImplementedError
    def puppets_follow(self, player_cell: int) -> Tuple[bool, str]: return True, ""                  # the puppet this copy shows of the other player is driven, in the cell that player is in (avatar.status); (ok, what is off)
    def give_item(self, item: str, n: int) -> Tuple[bool, str]: raise NotImplementedError             # console additem to this copy's player (start states only)

    def distance_to(self, ref: str) -> Optional[float]:
        """The 3D distance from this copy's player to the ref, None when it cannot be read. The engine opens no conversation menu at about 250 units, whatever the 2D radius of the walk said."""
        return None

    def harvested(self, ref: str) -> bool:
        """The plant / pickup is no longer there to take. The default is the lab's: disabled or deleted. A harvest plant ends in `setdestroyed 1` instead (its script marks itself), see StoryLiveMachine."""
        return not self.enabled(ref)


JUNK_LINES = ("goodbye", "good bye", "bye", "see you", "farewell", "until next time", "never mind", "nevermind", "later", "i should go", "i have to go", "end tutorial")      # a fallback pick avoids the lines that end the talk when others are on offer


def host_of(ctx: Ctx, who: str) -> StoryMachine:
    if MUTANT == "host_does_all":         # twin: the client's actions are done by the host
        return ctx.host
    return ctx.host if who == HOST else ctx.client


def snap_views(ctx: Ctx, script: D.Script) -> Dict[str, Dict[str, Any]]:
    """Both copies' views of the script's quests, reputations, karma, XP, caps and inventory, read at the same time."""
    out = ctx.par(lambda: D.view_of(ctx.host.state(), script), lambda: D.view_of(ctx.client.state(), script))
    return {HOST: out[HOST], CLIENT: out[CLIENT]}


# ------------------------------------------------------------------ prep: the start state of a step (the only console use)
def reached(m: StoryMachine, q: str, n: Optional[int]) -> bool:
    return bool(m.quest_running(q)) if n is None else bool(m.stage_done(q, n) or m.stage(q) >= n)


PLAN_IDX: Dict[str, Any] = {}


def stages_to_run(edid: str, want: int, have: int) -> List[int]:
    """The plan's stages of a quest above `have` up to `want`, in order, `want` last: a stage's result script enables its refs and starts its scenes, so a start state
    reached by setting only the last stage lacks what the earlier ones switch on (the geckos of VCG02 stage 25 were never enabled when the prep set 45)."""
    idx = [i for i in sorted(((PLAN_IDX.get(edid) or {}).get("stages") or {})) if have < i < want]
    return idx + [want]


def do_prep(ctx: Ctx, script: D.Script, step: Dict[str, Any], rep: Dict[str, Any]) -> None:
    forced: List[str] = rep.setdefault("prep", [])
    for p in step.get("prep", []):
        if "give" in p:
            g = p["give"]
            m = host_of(ctx, g["who"])
            if m.item_count(g["item"]) < int(g.get("count", 1)):
                ok, why = m.give_item(g["item"], int(g.get("count", 1)))
                ctx.check(f"prep: the {m.name} is given {g['item']}", ok, "given", why or "given", cls="TEST", area="tools/story (prep)")
                forced.append(f"additem {m.name} {g['item'].rpartition(':')[2]}")
            continue
        edid = p["quest"]
        q = script.quest_ref(edid)
        want = p.get("stage")
        cmds: List[str] = []
        if p.get("start") and not ctx.host.quest_running(q):
            ok, why = ctx.host.start_quest(q)
            ctx.check(f"prep: the host starts {edid}", ok, "started", why or "started", cls="TEST", area="tools/story (prep)")
            cmds.append(f"startquest {edid}")
            ctx.wait(f"prep: {edid} is running on the client", lambda q=q: (bool(ctx.client.quest_running(q)), {HOST: ctx.host.quest_running(q), CLIENT: ctx.client.quest_running(q)}),
                     ctx.p("prep_s"), True, area="plugin/hooks (quest start), synccore")
        if want is not None and not reached(ctx.host, q, want):
            for n in stages_to_run(edid, int(want), ctx.host.stage(q)):
                if n != want and ctx.host.stage_done(q, n):
                    continue
                ok, why = ctx.host.set_stage(q, n)
                ctx.check(f"prep: the host sets {edid} to stage {n}", ok, "set", why or "set", cls="TEST", area="tools/story (prep)")
                cmds.append(f"setstage {edid} {n}")
                if n != want:
                    ctx.sleep(0.5)
        if not cmds:
            continue
        forced.extend(cmds)
        if p.get("pass") and want is not None:           # a barrier stage waits for every player: each player passes it (the playbooks' scene_pass)
            for m in ctx.ms:
                ok, why = m.scene_pass(q)
                cmds.append(f"scenePass {edid} {m.name}")
                if not ok:
                    rep.setdefault("notes", []).append(f"{m.name}: scenePass {edid} refused: {(why or '')[:80]}")
            forced.extend(cmds[-len(ctx.ms):])
        if want is not None:
            ctx.wait(f"prep: the client shows {edid} stage {want}", lambda q=q, w=want: (reached(ctx.client, q, w), {HOST: ctx.host.stage(q), CLIENT: ctx.client.stage(q)}),
                     ctx.p("prep_s"), f">= {want}", area="plugin/hooks (quest stage), synccore, plugin/snapshot")
    if forced:
        for m in ctx.ms:
            m.enable_controls()          # the stage scripts of the opening disable the player's controls; set by console they are never enabled again
        ctx.sleep(1.0)


# ------------------------------------------------------------------ actions
def a_goto(ctx: Ctx, a: Dict[str, Any], rep: Dict[str, Any]) -> None:
    m = host_of(ctx, a["who"])
    t0 = ctx.now()
    dist = float(a.get("dist", 120))
    ok, why = m.walk_ref(a["ref"], dist, ctx.p("walk_s"), tick=ctx.tick)
    rec = {"who": m.name, "ref": a["ref"], "walked": ok, "secs": round(ctx.now() - t0, 1)}
    if ok and MUTANT != "no_reach_check":
        # A walk that says ok ends inside a 2D radius; the engine's Activate of an NPC opens no menu at 247 units in 3D (the floor of an interior counts). One closer try, then the
        # step gets there by the teleport below instead of talking to the air.
        limit = max(1.5 * dist, 150.0)
        d3 = m.distance_to(a["ref"])
        if d3 is not None and d3 > limit:
            m.walk_ref(a["ref"], min(dist / 2.0, 40.0), ctx.p("walk_s") / 2, tick=ctx.tick)
            d3 = m.distance_to(a["ref"])
            if d3 is not None and d3 > limit:
                ok, why = False, f"the walk ended {d3:.0f} units from it in 3D (limit {limit:.0f}); the engine opens no menu at that distance"
        rec["dist3d"] = None if d3 is None else round(d3)
    if not ok:
        tok, twhy = m.goto_ref(a["ref"], 0.0)          # a walk the engine's navigation cannot do: the step goes on from the spot (and says so)
        if not tok:
            ctx.sleep(1.0)                              # a patrolling NPC moved between the read of its place and the hop: once more
            tok, twhy = m.goto_ref(a["ref"], 0.0)
        rec["teleported"] = tok
        if tok:   # the teleport has a radius too; the engine opens no menu from 250 units in 3D (second story run), so look again and hop once more next to it
            d3 = m.distance_to(a["ref"])
            if d3 is not None and d3 > max(1.5 * dist, 150.0) and MUTANT != "no_reach_check":
                m.goto_ref(a["ref"], 0.0)
                d3 = m.distance_to(a["ref"])
            rec["dist3d_after_teleport"] = None if d3 is None else round(d3)
        rec["why"] = (why or "")[:160]
        rep.setdefault("notes", []).append(f"{m.name} could not walk to {a['ref']} ({(why or '')[:90]}); {'teleported' if tok else 'teleport failed: ' + (twhy or '')[:60]}")
        ctx.check(f"{m.name} gets to {a['ref']}", tok, "arrived", twhy or why or "arrived", cls="ENV", area="tools/story (route), tools/bots (walk)")
    rep.setdefault("walks", []).append(rec)


def a_talk(ctx: Ctx, a: Dict[str, Any], rep: Dict[str, Any]) -> None:
    m = host_of(ctx, a["who"])
    if a.get("together", True):
        # a party travels together: the host runs a guest's conversation and needs the NPC's cell loaded and the guest's puppet near him (the intents reach check,
        # `rejected.OutOfRange` / `NotLoaded` in intents.status), so the other player comes along
        other = ctx.client if m is ctx.host else ctx.host
        ok, why = other.walk_ref(a["npc"], 300.0, ctx.p("walk_s") / 2, tick=ctx.tick)
        if not ok:
            other.goto_ref(a["npc"], 250.0)
            rep.setdefault("notes", []).append(f"{other.name} came along to {a['npc'].rpartition(':')[2]} by teleport")
        ctx.sleep(2.0)
    settle_world(ctx, rep, f"{m.name} talks")
    if MUTANT != "boxes_not_cleared":
        # hint and tutorial boxes queue up while a copy walks about (4 of them in the second story run) and eat the input: the conversation menu then never opens
        n = 0
        while n < 10 and m.message_box():
            m.press_box(0)
            n += 1
            ctx.sleep(0.5)
        if n:
            rep.setdefault("notes", []).append(f"{m.name}: {n} message box(es) closed before the talk")
    t0 = ctx.now()
    ok, why = m.talk(a["npc"], ctx.p("talk_s"))
    ms = int(round((ctx.now() - t0) * 1000))
    rep.setdefault("talk_ms", {})[m.name] = ms
    guest = m.name == CLIENT
    if not ok and a.get("open_ok") and "already_in_dialogue" in (why or ""):   # the NPC greeted first (Sunny's forced greeting): the menu is open, which is all the step needs
        ok, why = True, "open (the NPC greeted first)"
    ctx.check(f"{m.name} talks to {a['npc']} and the dialogue opens", ok, "dialogue open", why or "open", cls="SYNC" if guest else "TEST",
              area="plugin/dialogue, plugin/intents (guest talk request, host grant)" if guest else "tools/story (route), tools/bots (talk)")
    ctx.metric(f"{m.name}_talk_ms", ms)


def _choose(offered: List[Dict[str, Any]], wanted: List[str], picked: set, fallback: str, script: D.Script) -> Tuple[Optional[Dict[str, Any]], str]:
    by = {D.norm_form(c["info"]): c for c in offered if c.get("info")}
    for w in wanted:
        if w in by and w not in picked:
            return by[w], "script"
    if fallback in ("first", "talky"):
        rest = [c for c in offered if D.norm_form(c.get("info", "")) not in picked]
        talky = [c for c in rest if not any(j in D.info_text(c.get("info", ""), script).lower() for j in JUNK_LINES)]
        if fallback == "talky" and MUTANT != "talky_takes_junk":      # never the lines that end the talk or the tutorial: when only those are left the step stops and its goal says what was missing
            return (talky[0], "fallback") if talky else (None, "")
        if talky or rest:
            return (talky or rest)[0], "fallback"
    return None, ""


def a_pick(ctx: Ctx, a: Dict[str, Any], script: D.Script, rep: Dict[str, Any]) -> None:
    """The initiator picks lines: the script's INFOs first (in order, each once), else the first line not said yet (fallback first) until the goal stage shows on the
    HOST (stop_when), the talk is over, or `max` picks. Every pick is recorded: INFO, the words, how it was chosen, what the engine said."""
    m = host_of(ctx, a["who"])
    wanted = [D.norm_form(i) for i in a.get("infos", [])]
    fallback = a.get("fallback", "none")
    picked: set = set()
    stop = a.get("stop_when")
    picks: List[Dict[str, Any]] = rep.setdefault("picks", [])
    for n in range(int(a.get("max", 6))):
        if stop and reached(ctx.host, script.quest_ref(stop["quest"]), int(stop["stage"])):
            break
        offered: List[Dict[str, Any]] = []
        end = ctx.now() + ctx.p("choices_s")
        while ctx.now() < end:
            ctx.tick()
            offered = m.choices()
            if offered or not m.in_dialogue():
                break
            ctx.sleep(0.4)
        if not offered:
            if n == 0 and wanted and not a.get("optional"):     # optional: the game opens the talk by itself (Doc's farewell); no talk is no pick, the goal says the rest
                ctx.check(f"{m.name}'s dialogue offers its lines", False, "lines on offer", {"offered": [], "in_dialogue": m.in_dialogue()},
                          cls="SYNC" if m.name == CLIENT else "TEST", area="plugin/dialogue (guest topic list)" if m.name == CLIENT else "tools/story (route)")
            break
        pick, how = _choose(offered, wanted, picked, fallback, script)
        if pick is None:
            if n == 0 and wanted:
                ctx.check(f"{m.name} is offered one of the scripted lines {wanted}", False, wanted, [(c.get("info"), c.get("topic")) for c in offered][:8],
                          cls="TEST", area="tools/story (script), plugin/dialogue (conditions, say-once state)")
            break
        info = D.norm_form(pick["info"])
        t0 = ctx.now()
        watch = a.get("watch") or []
        items_before: Dict[str, Any] = {}
        if watch:
            for mm in ctx.ms:
                try:
                    items_before[mm.name] = {w: mm.item_count(w) for w in watch}
                except Exception as e:   # noqa: BLE001 - evidence only
                    items_before[mm.name] = {"error": str(e)[:60]}
        ok, why, extra = m.say(info, ctx.p("pick_s"))
        ready_end = ctx.now() + float(a.get("ready_s", 0))
        while not ok and ("not ready" in (why or "") or "not offered" in (why or "")) and ctx.now() < ready_end:   # the NPC is still talking (a forced farewell plays 3 INFOs): wait for the menu to ask for a pick
            ctx.sleep(1.0)
            offered = m.choices() or offered
            pick, how = _choose(offered, wanted, picked, fallback, script)
            if pick is None:
                break
            info = D.norm_form(pick["info"])
            ok, why, extra = m.say(info, ctx.p("pick_s"))
        rec = {"who": m.name, "info": info, "text": D.info_text(info, script, 80), "how": how, "offered_infos": [c.get("info", "").rpartition(":")[2] for c in offered][:6], "offered_topics": [c.get("topic", "") for c in offered][:8], "said": bool(ok), "ms": int(round((ctx.now() - t0) * 1000)), "offered": len(offered)}
        if extra.get("speech_check"):
            rec["speech_check"] = True
        if wanted and how != "script" and not picked:   # the scripted line was not on offer: is its quest running on each copy? (S03 / S11, guest topic lists)
            quests = sorted({(D.infos_table().get(w) or {}).get("quest", "") for w in wanted} - {""})
            seen: Dict[str, Any] = {}
            for mm in ctx.ms:
                for q in quests:
                    try:
                        seen.setdefault(mm.name, {})[q.rpartition(":")[2]] = mm.quest_running(q)
                    except Exception as e:   # noqa: BLE001 - evidence only
                        seen.setdefault(mm.name, {})[q.rpartition(":")[2]] = str(e)[:60]
            rec["wanted_quest_running"] = seen
        if not ok:
            rec["why"] = (why or "")[:200]
        if MUTANT != "picks_unrecorded":
            picks.append(rec)
        picked.add(info)
        ctx.check(f"{m.name} picks {info} ({rec['text'][:40]})", ok, "said", why or "said", cls="SYNC" if m.name == CLIENT else "TEST",
                  area="plugin/dialogue (guest pick), tools/bots (bot.say)" if m.name == CLIENT else "tools/bots (bot.say)")
        if watch and ok:        # a reward that paid more than once shows as the count climbing after the pick: sampled right away and 2 s and 6 s later
            def _counts() -> Dict[str, Any]:
                out: Dict[str, Any] = {}
                for mm in ctx.ms:
                    try:
                        out[mm.name] = {w: mm.item_count(w) for w in watch}
                    except Exception as e:   # noqa: BLE001 - evidence only
                        out[mm.name] = {"error": str(e)[:60]}
                return out
            rec["items_before"] = items_before
            rec["items_after"] = {"0s": _counts()}
            ctx.sleep(2.0)
            rec["items_after"]["2s"] = _counts()
            ctx.sleep(4.0)
            rec["items_after"]["6s"] = _counts()
        ctx.sleep(0.8)
        end = ctx.now() + ctx.p("pick_s")          # the next lines (or the end of the talk) show up
        while ctx.now() < end and m.in_dialogue() and not m.choices():
            ctx.sleep(0.4)


def a_take(ctx: Ctx, a: Dict[str, Any], rep: Dict[str, Any]) -> None:
    m = host_of(ctx, a["who"])
    # a plant that is already harvested (or still disabled by its quest) makes the whole step vacuous: its start state must have it standing in the world
    ctx.check(f"{a['ref']} stands in the world on the {m.name} copy before the pickup", not m.harvested(a["ref"]), "in the world", "already gone or disabled (the quest stage that enables it was not reached)",
              cls="TEST", area="tools/story (start state of the step)")
    ok, why = m.pickup(a["ref"])
    ctx.check(f"{m.name} picks up {a['ref']}", ok, "picked up", why or "picked up", cls="TEST", area="tools/story (route), plugin/loot, plugin/intents (pickup)")
    # the harvest plants mark themselves with `setdestroyed 1`: they stay enabled, so "gone" is the machine's harvested(), not the enabled flag
    if MUTANT == "take_checks_enabled":
        ctx.agree(f"{a['ref']} is gone on both copies", lambda x: x.enabled(a["ref"]), ctx.p("gone_s"), False, area="plugin/loot, plugin/intents (pickup claim), plugin/hooks_world")
    else:
        ctx.agree(f"{a['ref']} is gone on both copies", lambda x: x.harvested(a["ref"]), ctx.p("gone_s"), True, area="plugin/loot, plugin/intents (pickup claim), plugin/hooks_world")


def _weaken(ctx: Ctx, ref: str) -> None:
    """The lab's way (lab_rooms.room_fight): the host, owner of the actor, holds the hostile where it stands (a free gecko ran up to the shooter, out of view and inside aim_at's
    100 units, and 24 volleys left it at health 11: story run 20261028-104118) and makes it weak, so one pistol hit kills it. The shot and the death that crosses the link are still real."""
    if MUTANT == "kill_unweakened":      # twin: the story's hostiles are shot at full health, free to move
        return
    ok, why = ctx.host.hold(ref)
    ctx.check(f"the host holds {ref} in place", ok, "held", why or "held", cls="TEST", area="tools/lab (SetRestrained)")
    ok, why = ctx.host.set_health(ref, ctx.p("weak_hp"))
    ctx.check(f"the host weakens {ref}", ok, "health set", why or "health set", cls="TEST", area="tools/lab (SetAV Health)")
    ctx.agree(f"{ref} shows the weakened health on both copies", lambda x, r=ref: x.health(r) <= ctx.p("weak_hp") + 1.0, ctx.p("weak_s"), True,
              area="plugin/actors (owner's health to the other copy)")


def _puppets_off(ctx: Ctx) -> List[str]:
    """Each copy's puppet of the other player is driven and stands in the cell the OTHER copy's player is in."""
    out = []
    for m, other in ((ctx.host, ctx.client), (ctx.client, ctx.host)):
        ok, why = m.puppets_follow(int(other.cell_of()))
        if not ok:
            out.append(f"{m.name}: {why}")
    return out


def settle_world(ctx: Ctx, rep: Dict[str, Any], what: str) -> None:
    """Both copies' message boxes closed and every puppet driven in its player's cell. The DLC setup scripts (per machine) show their load messages a few seconds after a player leaves
    Doc Mitchell's house, on BOTH copies, and a modal box pauses the avatar layer: the puppets stay where they were spawned (the house cell, "Loading3D") while their players go on, and
    the host judges the client's shots against that cell ("attacker in another cell: player#1 cell 1064441, victim cell 924350", S05 run 20261028-113423; the same stale puppet is
    behind the guest talks the host refuses as OutOfRange / NotLoaded). Boxes keep coming one after the other, so they are pressed until none shows for two seconds."""
    if MUTANT in ("no_settle_world", "boxes_not_cleared"):      # twins: shoot and talk with the boxes up and the puppets frozen in the old cell
        return
    end = ctx.now() + ctx.p("settle_world_s")
    quiet_since: Optional[float] = None
    pressed = 0
    while ctx.now() < end:
        ctx.tick()
        up = [m for m in ctx.ms if m.message_box()]
        for m in up:
            m.press_box(0)
            pressed += 1
        if up:
            quiet_since = None
            ctx.sleep(0.5)
            continue
        if quiet_since is None:
            quiet_since = ctx.now()
        bad = _puppets_off(ctx)
        if not bad and ctx.now() - quiet_since >= 2.0:
            break
        ctx.sleep(0.5)
    if pressed:
        rep.setdefault("notes", []).append(f"{pressed} message box(es) closed on the two copies before {what}")
    bad = _puppets_off(ctx)
    ctx.check(f"the puppets follow their players before {what}", not bad, "every puppet in its player's cell", "; ".join(bad) or "ok", cls="SYNC",
              area="plugin/avatar (a puppet stays in the cell it was spawned in), tools/story (message boxes)")


STAND_OFF = 300.0     # the far side of the target is this far from it (the pistol is accurate from 150 to 500 units, lab SHOT_MIN / SHOT_MAX)


def opposite_spot(target: Tuple[float, float, float], other: Tuple[float, float, float], dist: float = STAND_OFF) -> Tuple[float, float, float]:
    """A spot `dist` units from the target on the side AWAY from the other player: the line from there to the target never passes the other player. (Story run 20261028-113423:
    the host stood next to the first gecko, which is next to the second; 19 of the client's 21 hits landed on the host's puppet in the line of fire, "player-vs-player damage is off".)"""
    dx, dy = target[0] - other[0], target[1] - other[1]
    n = math.hypot(dx, dy)
    if n < 1.0:
        dx, dy, n = 1.0, 0.0, 1.0
    return (target[0] + dx / n * dist, target[1] + dy / n * dist, target[2])


def stand_clear(ctx: Ctx, m: StoryMachine, ref: str, rep: Dict[str, Any]) -> None:
    """The shooter hops to the far side of the target, so the other player is not between them. Both positions are read on the shooter's own copy (the other player is its puppet)."""
    if MUTANT == "stand_anywhere":       # twin: the shooter fires from wherever the walk left it
        return
    other = ctx.client if m is ctx.host else ctx.host
    try:
        tp = m.pos(ref)
        op = other.pos()
        if math.dist(tp, op) > 6000.0:   # another space: no line to worry about
            return
    except NotImplementedError:
        return
    x, y, z = opposite_spot(tp, op)
    ok, why = m.goto_pos(x, y, z)
    rep.setdefault("notes", []).append(f"{m.name} stood on the far side of {ref.rpartition(':')[2]} from the {other.name}" + ("" if ok else f" (the hop failed: {(why or '')[:60]})"))


def a_kill(ctx: Ctx, a: Dict[str, Any], rep: Dict[str, Any]) -> None:
    m = host_of(ctx, a["who"])
    ok, why = m.arm()
    ctx.check(f"{m.name} is armed", ok, "armed", why or "armed", cls="TEST", area="tools/lab (arm)")
    if hasattr(m, "draw_weapon"):
        m.draw_weapon()                          # a holstered weapon takes the trigger pulls and fires nothing ("weapon still holstered after the shots")
    for ref in a["refs"]:
        if not m.alive(ref):
            continue
        _weaken(ctx, ref)                        # before the walk: a free gecko runs up to the shooter while it walks
        ok, why = m.walk_ref(ref, 400.0, ctx.p("walk_s"), tick=ctx.tick)
        if not ok:
            m.goto_ref(ref, 0.0)
            m.walk_ref(ref, 400.0, 10.0, tick=ctx.tick)
            rep.setdefault("notes", []).append(f"{m.name} teleported near {ref}: {(why or '')[:80]}")
        settle_world(ctx, rep, f"{m.name} shoots")
        stand_clear(ctx, m, ref, rep)
        settle_world(ctx, rep, f"{m.name} shoots")      # the hop is a cell-local teleport; the puppets must be back before the first shot
        ok, why = m.shoot(ref, int(ctx.p("volleys")))
        # a shot that killed nothing: both copies' combat counters (the actor may belong to the other player: then the rejection is on ITS copy; Goodsprings S15, run 20261028-045950)
        words = "" if ok else " | ".join(f"{x.name} combat: {x.combat_words()}" for x in ctx.ms if hasattr(x, "combat_words"))
        ctx.check(f"{m.name} shoots {ref} dead", ok, "dead", why or "dead", note=words, cls="TEST", area="tools/lab (shoot), plugin/combat")
        ctx.agree(f"{ref} is dead on both copies", lambda x, r=ref: x.alive(r), ctx.p("settle_s"), False, area="plugin/combat, plugin/actors")


def run_actions(ctx: Ctx, script: D.Script, step: Dict[str, Any], rep: Dict[str, Any]) -> None:
    for a in step.get("do", []):
        kind = a["a"]
        ctx.tick()
        if kind == "goto":
            a_goto(ctx, a, rep)
        elif kind == "talk":
            a_talk(ctx, a, rep)
        elif kind == "pick":
            a_pick(ctx, a, script, rep)
        elif kind == "end_dialogue":
            m = host_of(ctx, a["who"])
            ok, why = m.end_dialogue()
            if not ok:
                rep.setdefault("notes", []).append(f"{m.name}: the dialogue did not close ({(why or '')[:80]})")
        elif kind == "activate":
            m = host_of(ctx, a["who"])
            ok, why = m.activate(a["ref"])
            ctx.check(f"{m.name} activates {a['ref']}", ok, "activated", why or "activated", cls="TEST", area="tools/story (route), plugin/intents (guest activate)")
        elif kind == "take":
            a_take(ctx, a, rep)
        elif kind == "kill":
            a_kill(ctx, a, rep)
        elif kind == "wait":
            left = float(a.get("seconds", 1))
            while left > 0:                       # in slices, so the time box is checked while waiting
                ctx.sleep(min(1.0, left))
                left -= 1.0
        elif kind == "answer_box":
            m = host_of(ctx, a["who"])
            end = ctx.now() + float(a.get("wait_s", 40))
            while ctx.now() < end and not m.message_box():
                ctx.sleep(0.5)
            seen = m.message_box()
            if seen:
                ok, why = m.press_box(int(a.get("button", 0)))
                rep.setdefault("boxes", []).append({"who": m.name, "button": int(a.get("button", 0)), "ok": ok, "why": (why or "")[:100]})
                ctx.check(f"{m.name} answers the message box ({a.get('why', 'button ' + str(a.get('button', 0)))})", ok, "pressed", why or "pressed", cls="TEST", area="tools/bots (bot.menu.press), plugin/hooks_ui (message routing)")
            else:
                rep.setdefault("boxes", []).append({"who": m.name, "button": None, "ok": False, "why": "no message box came up"})
                ctx.check(f"a message box comes up on the {m.name} copy ({a.get('why', '')})"[:150], bool(a.get("optional")), "a box", "no box in %g s" % float(a.get("wait_s", 40)), cls="SYNC",
                          area="plugin/hooks_ui (ShowMessage / Activate of a message ref raised on the origin player), plugin/dialogue (INFO end script)")
        elif kind == "expect_not":
            ctx.sleep(float(a.get("wait_s", 2)))
            q = script.quest_ref(a["quest"])
            got = {m.name: reached(m, q, int(a["stage"])) for m in ctx.ms}
            ctx.check(f"{a['quest']} stage {a['stage']} has NOT fired yet: {a.get('why', '')}"[:150], not any(got.values()), False, got, cls="SYNC", area="synccore (barrier), plugin/hooks, plugin/intents (guest stage pass)")


# ------------------------------------------------------------------ the goal
def _quest_ok(m: StoryMachine, script: D.Script, q: str, n: int) -> bool:
    return reached(m, script.quest_ref(q), n)


def goal_stages(ctx: Ctx, script: D.Script, step: Dict[str, Any], rep: Dict[str, Any]) -> None:
    goal = (step.get("goal") or {}).get("stages") or {}
    if not goal:
        return
    t_act = ctx.now()
    seen: Dict[str, Optional[float]] = {HOST: None, CLIENT: None}
    guest_pick = step.get("guest_dialogue") and any(p.get("who") == CLIENT for p in rep.get("picks", []))
    picks_ok = bool(rep.get("picks")) and all(p["said"] for p in rep["picks"])

    def both_state() -> Dict[str, Any]:
        return {m.name: {q: m.stage(script.quest_ref(q)) for q in goal} for m in ctx.ms}

    def host_or_client() -> Tuple[bool, Any]:
        for m in ctx.ms:
            if seen[m.name] is None and all(_quest_ok(m, script, q, int(n)) for q, n in goal.items()):
                seen[m.name] = ctx.now()
        return (seen[HOST] is not None or seen[CLIENT] is not None), both_state()

    # phase 1: somebody advanced. Neither copy: the route or the guest's pick did not reach the host.
    try:
        ctx.wait("the quest reaches the goal on one copy: " + ", ".join(f"{q} {n}" for q, n in goal.items()), host_or_client,
                 max(10.0, ctx.deadline - ctx.now() - ctx.p("settle_s") - 5.0), {q: f">= {n}" for q, n in goal.items()}, poll_s=0.5,
                 cls="SYNC" if (guest_pick and picks_ok) else "TEST",
                 area="plugin/dialogue, plugin/intents (guest pick: the host runs the result script as the guest)" if guest_pick else "tools/story (route), plugin/hooks (quest stage)",
                 note="the step's actions all said ok" if picks_ok else "")
    except K.Fail:
        rep["stage_ms"] = {}
        raise
    ctx.wait("the other copy shows it too", lambda: (host_or_client()[0] and seen[HOST] is not None and seen[CLIENT] is not None, both_state()), ctx.p("settle_s"),
             {q: f">= {n}" for q, n in goal.items()}, poll_s=0.5, cls="SYNC", area="plugin/hooks (quest stage), synccore, plugin/snapshot")
    rep["stage_ms"] = {k: (None if v is None else int(round((v - t_act) * 1000))) for k, v in seen.items()}
    for k, v in rep["stage_ms"].items():
        if v is not None:
            ctx.metric(f"{k}_stage_ms", max(v, 0))


def goal_objectives(ctx: Ctx, script: D.Script, step: Dict[str, Any]) -> None:
    for q, objs in ((step.get("goal") or {}).get("objectives") or {}).items():
        ref = script.quest_ref(q)
        for oid, flags in objs.items():
            want = (("d" in flags) or ("c" in flags), "c" in flags)

            def probe(oid=oid, want=want):
                got = {m.name: m.objective(ref, int(oid)) for m in ctx.ms}
                return all((g[0] or g[1]) >= want[0] and g[1] >= want[1] for g in got.values()), got
            ctx.wait(f"{q} objective {oid} is {'completed' if 'c' in flags else 'displayed'} on both copies", probe, ctx.p("settle_s") + 10, flags, poll_s=0.5,
                     area="plugin/hooks (objectives), synccore, plugin/snapshot")


def plan_expected_objectives(plan_idx: Dict[str, Any], q: str, n: int) -> List[Tuple[str, int, str]]:
    """The objective flags the plan says stage n of quest q sets (only the ones switched ON): [(edid, objective, 'displayed'|'completed')]."""
    row = (plan_idx.get(q) or {}).get("stages", {}).get(n) or {}
    out = []
    for qid, o, kind, v in (row.get("fx") or {}).get("obj", []):
        if v:
            edid = next((e for e, i in plan_idx.items() if i["ref"].endswith(":" + qid.lstrip("0").upper())), None)
            if edid == q:
                out.append((edid, int(o), kind))
    return out


# ------------------------------------------------------------------ one step
def run_step(ctx: Ctx, step: Dict[str, Any], script: D.Script, rep: Dict[str, Any], plan_idx: Dict[str, Any]) -> None:
    rep["actor"] = step["who"]
    PLAN_IDX.clear()
    PLAN_IDX.update(plan_idx)
    if step.get("enter") and ctx.enter_hook:
        ctx.enter_hook()
        ctx.rebase()
    ctx.close_menus()
    for m in ctx.ms:                           # a talk an earlier step left open would refuse the next talk ("already_in_dialogue")
        if not step.get("enter") and m.in_dialogue():
            m.end_dialogue()
    do_prep(ctx, script, step, rep)
    before = snap_views(ctx, script)
    rep["before"] = {k: D.brief_view(v, [step["quest"]]) for k, v in before.items()}
    rep["_before_full"] = before
    rep["_intents0"] = intents_now(ctx)
    rep["_dlg0"] = dialogue_now(ctx)
    run_actions(ctx, script, step, rep)
    goal_stages(ctx, script, step, rep)
    goal_objectives(ctx, script, step)
    ctx.sleep(1.0)
    after = snap_views(ctx, script)
    rep["_after_full"] = after
    fill_after(rep, before, after, step)
    if rep.get("_dlg0") or any(dialogue_now(ctx).values()):
        rep["dialogue"] = {"before": rep.get("_dlg0") or {}, "after": dialogue_now(ctx)}
    mine = sorted({step["quest"]} | set(((step.get("goal") or {}).get("stages") or {})) | set(((step.get("goal") or {}).get("objectives") or {})) | {p["quest"] for p in step.get("prep", []) if "quest" in p})
    div = D.first_diff(after[HOST], after[CLIENT], mine)
    rep["divergence"] = div
    ctx.check("host and client agree on quest state, objectives, karma and reputation", div is None, "no difference", div, area="plugin/hooks, plugin/snapshot, synccore, plugin/areadigest")
    for q, n in ((step.get("goal") or {}).get("stages") or {}).items():
        for edid, o, kind in plan_expected_objectives(plan_idx, q, int(n)):
            flag = "c" if kind == "completed" else "d"
            got = {k: (v["quests"].get(edid) or {}).get("obj", {}).get(str(o), "") for k, v in after.items()}
            ctx.check(f"the plan's objective effect of {q} stage {n} holds on both copies: {edid} objective {o} {kind}", all(flag in g or (flag == "d" and "c" in g) for g in got.values()),
                      kind, got, area="plugin/hooks (objectives), synccore")
    check_expect(ctx, step, after, before, rep)


def fill_after(rep: Dict[str, Any], before: Dict[str, Any], after: Dict[str, Any], step: Dict[str, Any]) -> None:
    rep["after"] = {k: D.brief_view(v, [step["quest"]]) for k, v in after.items()}
    rep["delta"] = {k: D.delta(before[k], after[k]) for k in (HOST, CLIENT)}


def check_expect(ctx: Ctx, step: Dict[str, Any], after: Dict[str, Any], before: Dict[str, Any], rep: Dict[str, Any]) -> None:
    ex = step.get("expect") or {}
    for it in ex.get("items", []):
        key = D.snap_key(it["item"])
        d = {k: after[k]["inv"].get(key, 0) - before[k]["inv"].get(key, 0) for k in (HOST, CLIENT)}
        who = it["who"]
        want = int(it["delta"])
        holder = ctx.host if who == HOST else ctx.client
        b0 = before[who]["inv"].get(key, 0)
        try:                                  # an item can arrive a moment after the pickup (the host grants a guest's claim): wait for it before judging
            ctx.wait(f"the {who} has the item {it['item']} within {ctx.p('gone_s'):.0f} s", lambda: (holder.item_count(it["item"]) - b0 == want, holder.item_count(it["item"]) - b0),
                     ctx.p("gone_s"), want, cls="SYNC", area="plugin/loot, plugin/intents (pickup claim), plugin/hooks (AddItem)")
        except K.Fail:
            raise
        ok = True
        other = CLIENT if who == HOST else HOST
        if d[other]:
            rep.setdefault("notes", []).append(f"{it['item']} also reached the {other} (+{d[other]}): quest items go to everyone, or a duplicate")
        ctx.check(f"the {who} gains {want} x {it['item']}", ok, want, d, cls="SYNC", area="plugin/loot, plugin/intents (pickup), plugin/hooks (AddItem)")
    rw = ex.get("reward")
    if rw:
        xp = {k: (None if after[k].get("xp") is None else after[k]["xp"] - (before[k].get("xp") or 0)) for k in (HOST, CLIENT)}
        rep["xp"] = xp
        who = [HOST, CLIENT] if rw.get("who") == "all" else [step["who"]]
        if all(xp[k] is not None for k in who):
            ctx.check(f"the quest reward of {rw['xp']} XP reaches {', '.join(who)} exactly once", all(xp[k] == int(rw["xp"]) for k in who), int(rw["xp"]), xp, cls="SYNC",
                      area="plugin/hooks (RewardXP), synccore (reward routing), docs/quest-policies")
        else:
            ctx.skip("XP reward", "this build's snapshot has no XP actor value")


def intents_now(ctx: Ctx) -> Dict[str, Dict[str, float]]:
    out: Dict[str, Dict[str, float]] = {}
    for m in ctx.ms:
        if hasattr(m, "intents_counters"):
            try:
                out[m.name] = dict(m.intents_counters())
            except Exception:   # noqa: BLE001 - evidence only
                out[m.name] = {}
    return out


def dialogue_now(ctx: Ctx) -> Dict[str, Dict[str, Any]]:
    """The dialogue module's own numbers on each copy (picks, line ends, script runs of the dialogue host, said-once bytes): evidence for the dialogue findings."""
    out: Dict[str, Dict[str, Any]] = {}
    for m in ctx.ms:
        if hasattr(m, "dialogue_status"):
            try:
                out[m.name] = m.dialogue_status()
            except Exception:   # noqa: BLE001 - evidence only
                out[m.name] = {}
    return out


def state_at_failure(ctx: Ctx, script: D.Script, step: Dict[str, Any], rep: Dict[str, Any]) -> None:
    """A step that stopped early still reports what both copies showed at that moment."""
    try:
        now = snap_views(ctx, script)
    except Exception as e:   # noqa: BLE001 - a lost game is reported by the check that saw it
        rep["state_error"] = f"{type(e).__name__}: {str(e)[:100]}"
        return
    ev: Dict[str, Any] = {}
    base = rep.get("_intents0") or {}
    for name, now_c in intents_now(ctx).items():
        # what the guest-intents service counted DURING the step (rejected.OutOfRange = the host judged the guest too far from the target, NotLoaded = the host has not loaded it)
        ch = {k: v - (base.get(name) or {}).get(k, 0) for k, v in now_c.items() if v != (base.get(name) or {}).get(k, 0)}
        ev[name] = dict(list(ch.items())[:10])
    if any(ev.values()):
        rep["intents"] = ev
    if rep.get("_dlg0") or any(dialogue_now(ctx).values()):
        rep["dialogue"] = {"before": rep.get("_dlg0") or {}, "after": dialogue_now(ctx)}
    before = rep.get("_before_full")
    rep["after"] = {k: D.brief_view(v, [step["quest"]]) for k, v in now.items()}
    if before:
        rep["delta"] = {k: D.delta(before[k], now[k]) for k in (HOST, CLIENT)}
    rep["divergence"] = D.first_diff(now[HOST], now[CLIENT], sorted({step["quest"]} | set(((step.get("goal") or {}).get("stages") or {}))))
    rep["_after_full"] = now


def strip_private(rep: Dict[str, Any]) -> Dict[str, Any]:
    return {k: v for k, v in rep.items() if not k.startswith("_")}


def run_story_step(step: Dict[str, Any], script: D.Script, plan_idx: Dict[str, Any], ctx: Ctx) -> Dict[str, Any]:
    """One step as a result row (never raises): the lab's room row (verdict, checks, first divergence, counters) plus a `story` block."""
    rep: Dict[str, Any] = {}

    def fn(c: Ctx):
        run_step(c, step, script, rep, plan_idx)
    row = K.run_room(step["id"], fn, ctx)
    if row["verdict"] != "PASS" and "after" not in rep and "state_error" not in rep:
        state_at_failure(ctx, script, step, rep)
    row["title"] = step["title"]
    row["who"] = step["who"]
    row["quest"] = step.get("quest")
    row["guest_dialogue"] = bool(step.get("guest_dialogue"))
    row["story"] = strip_private(rep)
    return row
