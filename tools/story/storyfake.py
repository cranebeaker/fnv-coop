"""A game-free two-copy Goodsprings for the story steps: the host and the client each hold their own view; what one does reaches the other as a message that
arrives `latency` seconds later. The host is authoritative: a guest's dialogue pick travels to the host, which runs the result (the stage with its effects from
questlines/goodsprings/playthrough.json, through the same Model that tools/bots/playbooks/questlines uses) and sends the result back.

    python -I tools/story/story.py run --fake [--bug NAME]      # the whole story on this world; with a bug the matching step must FAIL

What is modelled by the PLAN is the plan's (stage effects, rewards, barriers). What the plan does not say (which kill or pickup moves which quest) is the small table
KILL_RULES / FAKE_RULES below, marked as the fake's own. The real game's answers come from a run in a game, not from here.
"""
from __future__ import annotations

import heapq
import math
import sys
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Set, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (HERE, ROOT / "tools", ROOT / "tools" / "lab"):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

from lab_core import CLIENT, HOST                      # noqa: E402
from labfake import VClock, ConnectionLost             # noqa: E402,F401
from story_core import StoryMachine                    # noqa: E402
import story_data as D                                 # noqa: E402
from bots.playbooks.questlines.model import Model      # noqa: E402
from bots.playbooks.questlines.plan import Plan, key   # noqa: E402

DT = 0.05
FULL_HP = 60.0
BUGS = {
    "start_unsynced": "the client never sees a quest the host started (the old 'quest X running on every machine' timeout)",
    "stage_unsynced": "the client never sees a stage the host set",
    "slow_stage": "a stage reaches the client 40 s late",
    "stage_rollback": "the client's stage drops back 6 s after the host set a later one",
    "objective_unsynced": "the client gets the stage but not its objectives",
    "guest_talk_vetoed": "the host never grants the guest's talk: the dialogue does not open on the client",
    "guest_lines_missing": "the client's open dialogue lists no lines",
    "guest_pick_lost": "the guest's pick never reaches the host: the result script never runs",
    "guest_result_twice": "the host runs the result of a guest-led stage twice: the initiator's reward doubled",
    "reward_host_only": "a reward-for-all quest pays the host only",
    "rep_unsynced": "a reputation change reaches the host only",
    "pickup_unsynced": "a picked-up plant stays in the world on the other copy",
    "kill_unseen": "a death the client caused never reaches the host",
    "barrier_open": "a barrier stage fires after the first player passed",
    "digest_noise": "every guest pick raises the digest repair counter",
}
# not bugs of the product but quirks of the real game the runner must live with (a run with one passes; the twins of the runner decide whether it did so for the right reason)
QUIRKS = {
    "boxes_queued":    "message boxes (hints, tutorials) are queued on a copy that has walked to an NPC: they eat the input and the conversation menu does not open",
    "walk_stops_far": "a walk says ok but ends 247 units from the NPC in 3D (the interior floor): the conversation menu does not open there",
    "dlc_boxes":      "the DLC setup scripts' load messages come up on BOTH copies when a player leaves Doc Mitchell's house; while the host's box is up its avatar layer is paused and it refuses the client's hits (attacker in another cell)",
    "host_in_line":   "the host stands next to the first gecko, which is next to the second: the client's hits land on the host's puppet (player-vs-player damage is off) unless it shoots from the far side",
    "tough_geckos":   "a hostile at full health that is free to move survives every volley (S05 in game: 24 volleys left a gecko at health 11, 53 units from the shooter, no line of sight)",
}
KILL_RULES: List[Tuple[Tuple[str, ...], Tuple[str, int]]] = [
    (("gecko1", "gecko2"), ("VCG02", 50)),                       # fake's own: both geckos at the well dead ends the hunt
    (("gang1", "gang2", "gang3"), ("VMS16", 80)),                 # the gunfight is won
]
PLANT_ITEM = {"broc": "broc_item", "xander": "xander_item"}      # fake's own: which item a plant gives
REP_RULES = {("VMS16", 100): ("Goodsprings", 3.0)}               # fake's own: ModReputation of the completion script


class Quest:
    def __init__(self) -> None:
        self.stage = 0
        self.done: Set[int] = set()
        self.running = False
        self.completed = False
        self.obj: Dict[int, List[bool]] = {}


class Side:
    def __init__(self, name: str) -> None:
        self.name = name
        self.near = ""
        self.cell = 1
        self.quests: Dict[str, Quest] = {}
        self.enabled: Dict[str, bool] = {}
        self.destroyed: Set[str] = set()      # harvest plants end in `setdestroyed 1`: still enabled, no longer there to take
        self.dist3d = 40.0                    # 3D distance to `near`
        self.dead: Set[str] = set()
        self.hp: Dict[str, float] = {}        # health of an actor this copy was told about (SetAV Health); an actor never weakened shows FULL_HP
        self.held: Set[str] = set()           # actors restrained (SetRestrained 1) by their owner
        self.inv: Dict[str, int] = {}
        self.xp = 100
        self.caps = 500
        self.karma = 0.0
        self.rep: Dict[str, List[float]] = {}
        self.dialogue: Optional[Dict[str, Any]] = None
        self.repairs = 0
        self.armed = False
        self.stood_clear = False

    def q(self, qk: str) -> Quest:
        return self.quests.setdefault(qk, Quest())


class World:
    def __init__(self, script: D.Script, plan: Dict[str, Any], bugs=(), latency: float = 0.3):
        self.script, self.plan_data, self.bugs, self.latency = script, plan, set(bugs), latency
        unknown = self.bugs - set(BUGS) - set(QUIRKS)
        if unknown:
            raise ValueError(f"unknown fake bug(s) {sorted(unknown)}; the names are in storyfake.BUGS")
        self.plan = Plan(plan)
        self.t = 0.0
        self.q: List[Tuple[float, int, Callable[[], None]]] = []
        self.n = 0
        self.clock = VClock(self)
        self.sides = {HOST: Side(HOST), CLIENT: Side(CLIENT)}
        self.box = {}
        self.noise: Set[str] = set()      # copies with queued hint boxes (quirk boxes_queued)
        self.reset()

    def has(self, bug: str) -> bool:
        return bug in self.bugs

    def reset(self) -> None:
        """The state after S00's start: the six quests reset, both players next to Doc Mitchell."""
        self.model = Model(self.plan)
        self.said: Set[str] = set()
        self.barrier: Dict[Tuple[str, int], Set[str]] = {}
        self.digest_ok = 0
        self.last_digest = 0.0
        enabled_by_plan = {D.norm_form(r) for (qk, n), row in self.plan.stage_rows.items() for r in row["fx"].get("enable", [])}
        for s in self.sides.values():
            s.quests = {}
            s.near = self.script.refs.get("doc", "")
            s.dialogue = None
            s.inv = {}
            s.dead = set()
            s.enabled = {r: False for r in enabled_by_plan}
            s.destroyed = set()
            s.dist3d = 40.0
            s.rep = {n: [15.0, 0.0] for n in self.script.reps}
            s.xp, s.caps, s.karma = 100, 500, 0.0
            for edid, ref in self.script.quests.items():
                s.q(key(D.hexid(ref).zfill(8)))

    # ---- messages
    def send(self, to: str, fn: Callable[[Side], None], extra: float = 0.0) -> None:
        self.n += 1
        heapq.heappush(self.q, (self.t + self.latency + extra, self.n, lambda: fn(self.sides[to])))

    def in_flight(self) -> bool:
        return bool(self.q)

    def advance(self, dt: float) -> None:
        self.t += dt
        while self.q and self.q[0][0] <= self.t:
            heapq.heappop(self.q)[2]()
        if self.t - self.last_digest >= 1.0 and not self.q:
            self.last_digest = self.t
            self.digest_ok += 1

    # ---- the host's authority: a stage with its effects, then the same effects on the client
    def qk(self, edid: str) -> str:
        return key(D.hexid(self.script.quest_ref(edid)).zfill(8))

    def edid_of(self, qk: str) -> str:
        for e, r in self.script.quests.items():
            if key(D.hexid(r).zfill(8)) == qk:
                return e
        return qk

    def host_set(self, qk: str, n: int, initiator: str = HOST, _again: bool = False) -> None:
        res = self.model.apply(qk, n)
        for (hq, hn, kind) in res.held:
            if kind == "barrier":
                self.barrier.setdefault((hq, hn), set())
        self.commit(res, initiator)

    def commit(self, res, initiator: str) -> None:
        reward_all = any((self.plan.rules(q) or {}).get("reward_all") for q, _n in res.stages)
        who = [HOST, CLIENT] if (reward_all or initiator == "both") else [initiator]
        if self.has("reward_host_only"):
            who = [HOST]
        reps = {REP_RULES[(self.edid_of(q), n)] for q, n in res.stages if (self.edid_of(q), n) in REP_RULES}

        def apply(s: Side, host: bool) -> None:
            for q, n in res.stages:
                qu = s.q(q)
                qu.done.add(n)
                qu.stage = max(qu.stage, n)
            for (q, o), flags in res.obj.items():
                if host or not self.has("objective_unsynced"):
                    for kind, v in flags.items():
                        cur = s.q(q).obj.setdefault(o, [False, False])
                        cur[0 if kind == "displayed" else 1] = bool(v)
                        if kind == "completed" and v:
                            cur[0] = cur[0] or True
            for r, en in res.enabled.items():
                s.enabled[r] = en
            for r in res.dead:
                s.dead.add(r)
            for q, what in res.quests.items():
                qu = s.q(q)
                if what == "start":
                    qu.running = True
                elif what in ("stop", "complete"):
                    qu.completed = qu.completed or what == "complete"
                    qu.running = False
            if s.name in who:
                for item, c in res.items.items():
                    if item == "xp":
                        s.xp += c * (2 if (self.has("guest_result_twice") and initiator == CLIENT) else 1)
                    else:
                        s.inv[D.snap_key(item)] = s.inv.get(D.snap_key(item), 0) + c
            s.karma += res.karma
            for name, amount in reps:
                if host or not self.has("rep_unsynced"):
                    s.rep[name][0] += amount
        apply(self.sides[HOST], True)
        if not res.stages and not res.obj and not res.enabled and not res.dead and not res.quests:
            return
        if self.has("stage_unsynced") or (self.has("start_unsynced") and res.quests):
            return
        extra = 40.0 if self.has("slow_stage") else 0.0
        self.send(CLIENT, lambda s: apply(s, False), extra)
        if self.has("stage_rollback"):
            def rollback(s: Side, res=res):
                for q, n in res.stages:
                    s.q(q).stage = 0
                    s.q(q).done.discard(n)
            self.send(CLIENT, rollback, 6.0)

    def host_set_with_repeat(self, qk: str, n: int, initiator: str) -> None:
        self.host_set(qk, n, initiator)

    def release(self, hq: str, hn: int) -> None:
        self.commit(self.model.release(hq, hn), "both")

    def passed(self, side: str) -> None:
        """A player's scene pass for every held barrier (the guest's own SetStage intent, the host's own script run)."""
        for (hq, hn), who in list(self.barrier.items()):
            who.add(side)
            if (len(who) >= 2) or self.has("barrier_open"):
                del self.barrier[(hq, hn)]
                self.release(hq, hn)

    def host_event(self, edid: str, n: int, initiator: str) -> None:
        self.host_set(self.qk(edid), n, initiator)

    def kill_rules(self, initiator: str) -> None:
        host = self.sides[HOST]
        for names, (edid, n) in KILL_RULES:
            refs = [self.script.refs[x] for x in names]
            if all(r in host.dead for r in refs) and n not in host.q(self.qk(edid)).done:
                self.host_event(edid, n, initiator)

    def host_obj(self, edid: str, o: int, kind: str, initiator: str) -> None:
        from bots.playbooks.questlines.model import Result
        res = Result()
        res.obj[(self.qk(edid), o)] = {kind: 1}
        self.commit(res, initiator)

    def greeting(self, npc: str, who: str) -> None:
        """Fake's own: what an NPC's greeting does when the player talks to him (the plan only knows the stages)."""
        h = self.sides[HOST]
        sunny, ringo = self.script.refs.get("sunny"), self.script.refs.get("ringo")
        both = [D.snap_key(self.script.refs[x]) for x in ("broc_item", "xander_item")]
        talker = self.sides[who]
        if npc == sunny and h.q(self.qk("VCG03")).stage >= 10 and all(talker.inv.get(b, 0) > 0 for b in both):
            self.host_obj("VCG03", 20, "completed", who)
        if npc == ringo and h.q(self.qk("VMS16")).stage >= 80:
            self.host_event("VMS16", 100, who)


class FakeMachine(StoryMachine):
    def __init__(self, world: World, name: str) -> None:
        self.w, self.name, self.role = world, name, name
        self.s = world.sides[name]

    # ---- plumbing
    def _near(self, ref: str) -> bool:
        return self.s.near == ref and self.s.dist3d <= 200.0      # the engine opens no menu from farther away

    def distance_to(self, ref: str) -> Optional[float]:
        return self.s.dist3d if self.s.near == ref else 9999.0

    def harvested(self, ref: str) -> bool:
        return (not self.enabled(ref)) or ref in self.s.destroyed

    def _ref(self, name: str) -> str:
        return self.w.script.refs.get(name, name)

    # ---- lab Machine
    def status(self) -> Dict[str, Any]:
        d = {"ok": self.w.digest_ok, "mismatch": 0, "repairs": self.w.sides[HOST].repairs}
        if self.name == HOST:
            return {"role": "host", "host": {"peers": [{"joined": True}], "allSynced": True}, "desyncs": {"digests": d, "desyncs": []}}
        return {"role": "client", "client": {"joined": True}, "transport": {"connected": True}, "desyncs": {"digests": {"ok": 0, "mismatch": 0, "repairs": 0}}}

    def pos(self, ref: Optional[str] = None) -> Tuple[float, float, float]:
        return (1000.0, 0.0, 0.0) if ref else (0.0, 0.0, 0.0)      # the fake's geometry: the target is 1000 units east of both players

    def goto_pos(self, x: float, y: float, z: float) -> Tuple[bool, str]:
        self.s.stood_clear = True
        return True, ""

    def cell_of(self, ref: Optional[str] = None) -> int:
        return self.s.cell

    def exists(self, ref: str) -> bool:
        return True

    def ref_id(self, ref: str) -> str:
        return D.hexid(ref)

    def alive(self, ref: str) -> bool:
        return ref not in self.s.dead

    def enabled(self, ref: str) -> bool:
        return self.s.enabled.get(ref, True)

    def close_menus(self) -> Tuple[bool, str]:
        return True, ""

    def item_count(self, item: str, holder: Optional[str] = None) -> int:
        return self.s.inv.get(D.snap_key(item), 0)

    def give_item(self, item: str, n: int) -> Tuple[bool, str]:
        self.s.inv[D.snap_key(item)] = self.s.inv.get(D.snap_key(item), 0) + n
        return True, ""

    def goto_ref(self, ref: str, dist: float = 0.0) -> Tuple[bool, str]:
        self.s.near = ref
        if self.w.has("dlc_boxes"):
            self.w.noise.update({HOST, CLIENT})
        self.s.dist3d = 40.0              # a teleport puts the player next to it, floor and all
        return True, ""

    def scene_pass(self, quest: str) -> Tuple[bool, str]:
        me = self.name
        if me == HOST:
            self.w.passed(me)
        else:
            self.w.send(HOST, lambda h: self.w.passed(me))
        return True, ""

    def message_box(self) -> bool:
        return self.w.box.get(self.name, False) or self.name in self.w.noise

    def press_box(self, button: int) -> Tuple[bool, str]:
        if not self.w.box.get(self.name) and self.name in self.w.noise:
            self.w.noise.discard(self.name)           # a hint box: closing it changes nothing
            return True, ""
        if self.w.box.get(self.name):
            me = self.name

            def go() -> None:
                self.w.host_event("VCG01", 200, me)                 # the farewell's end script (timer event 3) asks for the end of the quest: a barrier stage, held until both passed
                self.w.passed(me)
            if me == HOST:
                go()
            else:
                self.w.send(HOST, lambda h: go())
        self.w.box[self.name] = False
        return True, ""

    def enable_controls(self) -> Tuple[bool, str]:
        return True, ""

    def draw_weapon(self) -> bool:
        return True

    def arm(self) -> Tuple[bool, str]:
        self.s.armed = True
        return True, ""

    def health(self, ref: str) -> float:
        return 0.0 if ref in self.s.dead else self.s.hp.get(ref, FULL_HP)

    def set_health(self, ref: str, hp: float) -> Tuple[bool, str]:
        """The owner (the host) sets it; the other copy shows it a moment later."""
        if self.name != HOST:
            return False, "only the owner of the actor can set its health"
        self.s.hp[ref] = hp
        self.w.send(CLIENT, lambda c: c.hp.__setitem__(ref, hp))
        return True, ""

    def hold(self, ref: str) -> Tuple[bool, str]:
        if self.name != HOST:
            return False, "only the owner of the actor can restrain it"
        self.s.held.add(ref)
        return True, ""

    # ---- quests
    def _qk(self, quest: str) -> str:
        return key(D.hexid(quest).zfill(8))

    def quest_running(self, quest: str) -> bool:
        return self.s.q(self._qk(quest)).running

    def stage(self, quest: str) -> int:
        return self.s.q(self._qk(quest)).stage

    def stage_done(self, quest: str, stage: int) -> bool:
        return stage in self.s.q(self._qk(quest)).done

    def objective(self, quest: str, obj: int) -> Tuple[bool, bool]:
        d, c = self.s.q(self._qk(quest)).obj.get(int(obj), [False, False])
        return d, c

    def start_quest(self, quest: str) -> Tuple[bool, str]:
        if self.name != HOST:
            return False, "only the host starts quests"
        qk = self._qk(quest)
        self.s.q(qk).running = True
        if not self.w.has("start_unsynced"):
            self.w.send(CLIENT, lambda c, qk=qk: setattr(c.q(qk), "running", True))
        return True, ""

    def set_stage(self, quest: str, stage: int) -> Tuple[bool, str]:
        if self.name != HOST:
            return False, "a guest's setstage is a request; the story only uses the host's"
        self.w.host_set(self._qk(quest), int(stage), HOST)
        return True, ""

    def state(self) -> Dict[str, Any]:
        quests = {}
        for edid, ref in self.w.script.quests.items():
            qu = self.s.q(self._qk(ref))
            quests[D.snap_key(ref)] = {"edid": edid, "running": qu.running, "completed": qu.completed, "failed": False, "currentStage": qu.stage, "completedStages": sorted(qu.done),
                                       "objectives": {str(o): {"displayed": f[0], "completed": f[1]} for o, f in qu.obj.items()}}
        reps = {D.snap_key(ref): {"fame": self.s.rep[n][0], "infamy": self.s.rep[n][1]} for n, ref in self.w.script.reps.items()}
        return {"shared": {"quests": quests, "reputations": reps, "karma": self.s.karma},
                "perPlayer": {"local": {"actorValues": {"Karma": {"base": self.s.karma, "current": self.s.karma}, "XP": {"base": self.s.xp, "current": self.s.xp}},
                                        "inventory": {k: {"count": v} for k, v in self.s.inv.items()}, "caps": self.s.caps, "position": {"cell": self.s.cell}}}}

    # ---- moving
    def walk_ref(self, ref: str, dist: float, timeout_s: float, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]:
        self.w.clock.sleep(1.5)
        self.s.near = ref
        self.s.dist3d = 247.0 if self.w.has("walk_stops_far") else min(dist, 100.0)
        if self.w.has("boxes_queued"):
            self.w.noise.add(self.name)
        if self.w.has("dlc_boxes"):
            self.w.noise.update({HOST, CLIENT})
        if ref == self.w.script.refs.get("house_door") and self.w.sides[HOST].q(self.w.qk("VCG01")).stage >= 110:
            self.w.box[self.name] = True                             # the exit trigger fired: Doc's farewell raises the casual / hardcore box
        return True, ""

    # ---- dialogue
    def talk(self, npc: str, timeout_s: float) -> Tuple[bool, str]:
        if self.name in self.w.noise:
            return False, "the dialogue menu did not open; a message box is up (4 queued) and eats the input"
        if not self._near(npc):
            return False, f"the dialogue menu did not open; the NPC is far away ({self.s.near} is not {npc})"
        if self.name == HOST:
            self.s.dialogue = {"npc": npc}
            self.w.greeting(npc, HOST)
            return True, ""
        if self.w.has("guest_talk_vetoed"):
            self.w.clock.sleep(min(timeout_s, 5.0))
            return False, "the dialogue menu did not open; the host never granted the conversation"

        def grant(c: Side, npc=npc) -> None:
            c.dialogue = {"npc": npc}
            self.w.greeting(npc, CLIENT)
        self.w.send(CLIENT, grant, self.w.latency)
        t0 = self.w.t
        while self.s.dialogue is None and self.w.t - t0 < timeout_s:
            self.w.clock.sleep(0.1)
        return self.s.dialogue is not None, "" if self.s.dialogue else "the dialogue menu did not open"

    def in_dialogue(self) -> bool:
        return self.s.dialogue is not None

    def _topics(self, npc_ref: str) -> List[Dict[str, Any]]:
        host = self.w.sides[HOST]
        out = []
        for n in self.w.script.npcs.values():
            if n["ref"] != npc_ref:
                continue
            fillers = [t for t in n["topics"] if not t.get("sets") and not t.get("last") and not t.get("keep")]
            for t in n["topics"]:
                if t["info"] in self.w.said:
                    continue
                if any(host.q(self.w.qk(q)).stage < int(st) for q, st in (t.get("needs") or {}).items()):
                    continue
                if t.get("sets") and host.q(self.w.qk(t["sets"][0])).stage >= int(t["sets"][1]):
                    continue
                if t.get("last") and any(f["info"] not in self.w.said for f in fillers):
                    continue
                out.append(t)
        return out

    def choices(self) -> List[Dict[str, Any]]:
        if self.s.dialogue is None or (self.name == CLIENT and self.w.has("guest_lines_missing")):
            return []
        return [{"idx": i, "info": t["info"]} for i, t in enumerate(self._topics(self.s.dialogue["npc"]))]

    def say(self, info: str, timeout_s: float) -> Tuple[bool, str, Dict[str, Any]]:
        offered = [c["info"] for c in self.choices()]
        if info not in offered:
            return False, f"INFO not offered in this dialogue (offered {offered[:4]})", {}
        t = self.w.script.topic(info) or {}
        npc = self.s.dialogue["npc"]
        me = self.name

        def host_runs(h: Side = None, times: int = 1) -> None:
            self.w.said.add(info)
            for _ in range(times):
                if t.get("sets"):
                    self.w.host_event(t["sets"][0], int(t["sets"][1]), me)
                if t.get("obj"):
                    self.w.host_obj(t["obj"][0], int(t["obj"][1]), t["obj"][2], me)       # fake's own: a result script that only shows an objective (10A1EB)
        if me == HOST:
            host_runs()
        elif not self.w.has("guest_pick_lost"):
            self.w.send(HOST, lambda h: host_runs())
        if self.w.has("digest_noise") and me == CLIENT:
            self.w.sides[HOST].repairs += 1
        self.w.clock.sleep(0.3)
        if not self.choices():
            self.s.dialogue = self.s.dialogue            # the menu stays open until the player ends it
        return True, "", {}

    def end_dialogue(self) -> Tuple[bool, str]:
        self.s.dialogue = None
        return True, ""

    def activate(self, ref: str) -> Tuple[bool, str]:
        if ref == self.w.script.refs.get("house_door"):
            def go():
                self.w.host_event("VCG01", 200, self.name)                 # the exit scene asks for the end of the quest: a barrier stage, held until both passed
                self.w.passed(self.name)
            if self.name == HOST:
                go()
            else:
                self.w.send(HOST, lambda h: go())
        return True, ""

    # ---- world
    def pickup(self, ref: str) -> Tuple[bool, str]:
        if self.harvested(ref):
            return False, "the plant is not in the world"
        if not self._near(ref):
            return False, "too far away"
        name = next((n for n, r in self.w.script.refs.items() if r == ref), "")
        item = D.snap_key(self.w.script.refs[PLANT_ITEM[name]]) if name in PLANT_ITEM else D.snap_key(ref)
        me = self.name

        def gone(s: Side) -> None:
            s.destroyed.add(ref)
        self.s.destroyed.add(ref)            # the plant's own script: it stays enabled, the pickup check must read the destroyed mark
        self.s.inv[item] = self.s.inv.get(item, 0) + 1
        if not self.w.has("pickup_unsynced"):
            self.w.send(CLIENT if me == HOST else HOST, gone)
        return True, ""

    def shoot(self, ref: str, volleys: int = 12, stand: Optional[str] = None) -> Tuple[bool, str]:
        if not self.s.armed:
            return False, "no weapon"
        if not self.s.enabled.get(ref, True):
            return False, "the target is not in the world"
        self.w.clock.sleep(1.0)
        me = self.name
        if self.w.has("dlc_boxes") and me == CLIENT and HOST in self.w.noise:
            return False, f"{volleys} volleys did not kill it (the host refused every hit: attacker in another cell)"
        if self.w.has("host_in_line") and me == CLIENT and not self.s.stood_clear:
            return False, f"{volleys} volleys did not kill it (19 hits neutralized: player-vs-player damage is off, victim = the host's puppet)"
        if self.w.has("tough_geckos") and (self.s.hp.get(ref, FULL_HP) > 10.0 or ref not in self.w.sides[HOST].held):
            return False, f"{volleys} volleys did not kill it (target health {self.s.hp.get(ref, FULL_HP):.0f}, 53 units, line of sight unknown)"
        self.s.dead.add(ref)

        def host_sees(h: Side) -> None:
            h.dead.add(ref)
            self.w.kill_rules(me)
        if me == HOST:
            self.w.kill_rules(me)
            self.w.send(CLIENT, lambda c: c.dead.add(ref))
        elif not self.w.has("kill_unseen"):
            self.w.send(HOST, host_sees)
        return True, ""


class FakeStoryEnv:
    live = False

    def __init__(self, script: D.Script, plan: Dict[str, Any], bugs=()):
        self.script, self.plan, self.bugs = script, plan, list(bugs)
        self.w = World(script, plan, bugs)
        self.entered = False

    def build_id(self) -> str:
        return "fake-story-world"

    def pairs_allowed(self, n: int) -> Tuple[int, str]:
        return 1, "the story is one pair of copies" if n > 1 else ""

    def prepare(self, pair: int):
        return None

    def machines(self, pair: int):
        return FakeMachine(self.w, HOST), FakeMachine(self.w, CLIENT)

    def clock(self, pair: int):
        return self.w.clock

    def probe(self, pair: int):
        return lambda: []

    def enter(self, pair: int):
        def go() -> None:
            self.w.reset()
            if self.script.data.get("start", {}).get("keep"):         # the start save: VCG01 at stage 110 on both copies, as the real save is
                w = self.w
                bugs, w.bugs = w.bugs, set()                      # a loaded save is the same on both copies, whatever the replication does later
                w.host_set(w.qk("VCG01"), 110, HOST)
                w.sides[HOST].q(w.qk("VCG01")).running = True
                w.sides[CLIENT].q(w.qk("VCG01")).running = True
                w.clock.sleep(2.0)
                w.bugs = bugs
            self.entered = True
        return go

    def reset(self, pair: int):
        self.w.reset()
        return None

    def finish(self) -> None:
        pass
