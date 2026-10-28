"""The real game behind the story steps: StoryLiveMachine (the lab's LiveMachine plus the story verbs) and StoryLiveEnv (one pair, boot, the save, the start state).

Reuse, not a rewrite: LiveMachine (shooting, pickup, menus, quest stage), prove.py's LiveEnv (slot lease owner `story`, RAM guard, provisioning, boot, saves,
crash watch) and the scenario runner's jump (load the save, wait for the session, reset the Goodsprings quests, rebaseline, settle, move the players). The lab
plugin is NOT installed: the story runs in the real Goodsprings.
"""
from __future__ import annotations

import math
import sys
import threading
import time
from pathlib import Path
from types import SimpleNamespace
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (ROOT / "tools", ROOT / "tools" / "runner", ROOT / "tools" / "proof", ROOT / "tools" / "scenarios", ROOT / "tools" / "lab", HERE):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_core as K                                  # noqa: E402
import lab_live as LL                                 # noqa: E402
from lab_core import CLIENT, HOST, Blocked           # noqa: E402
from story_core import StoryMachine                   # noqa: E402
import story_data as D                                # noqa: E402

WHY = LL.WHY

# the switches the story needs in nvcoop.ini: the lab's, plus the guest-dialogue and story ones that docs/handover/P6-requests.md proposes (G46)
STORY_INI = LL.LAB_INI + ["intents.dialogue=1", "intents.dialogue_results=1", "intents.stage=1", "hooks.story_capture=1"]
GAME_ENV = {"NVCOOP_TEST_HELPERS": "1"}               # test.quest.reset and the dev bridge methods


def _hex(form: str) -> str:
    return D.hexid(form)


class StoryLiveMachine(LL.LiveMachine, StoryMachine):
    """One game copy through the bridge and the bot layer."""

    def __init__(self, nv, name: str):
        LL.LiveMachine.__init__(self, nv, name)

    def _q(self, quest: str) -> str:
        return self.bot._norm(quest)[1]          # noqa: SLF001 (the runtime id with this copy's load-order index)

    def quest_running(self, quest: str) -> bool:
        return bool(self.nv.quest_running(self._q(quest)))

    def stage_done(self, quest: str, stage: int) -> bool:
        return bool(self.bot.stage_done(quest, stage))

    def objective(self, quest: str, obj: int) -> Tuple[bool, bool]:
        q = self._q(quest)
        return bool(self.nv.objective_displayed(q, obj)), bool(self.nv.objective_completed(q, obj))

    def start_quest(self, quest: str) -> Tuple[bool, str]:
        r = self.nv.exec(f"startquest {self._q(quest)}")
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def state(self) -> Dict[str, Any]:
        return self.nv.state(["shared", "perPlayer"])

    def walk_ref(self, ref: str, dist: float, timeout_s: float, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]:
        res: Dict[str, Any] = {}
        try:
            p = tuple(self.bot.pos(ref))                 # a walk to a position: goto(ref) refuses another exterior grid cell ("other_cell"), goto(position) does not
        except Exception as e:   # noqa: BLE001 - the ref is not loaded on this copy
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, f"the ref is not loaded here: {str(e)[:120]}"
        try:
            far = math.dist(tuple(self.bot.pos()), p)
        except Exception:   # noqa: BLE001
            far = 0.0
        if far > 6000.0:                      # another space (an interior's coordinates against the exterior's): no walk can get there
            return False, f"{far:.0f} units away, another space: no walk goes there"

        def work():
            try:
                res["r"] = self.bot.goto(p, radius=dist, timeout_s=timeout_s, mode="walk")
            except BaseException as e:   # noqa: BLE001 - handed to the caller's thread below
                res["exc"] = e
        th = threading.Thread(target=work, daemon=True)
        th.start()
        while th.is_alive():
            if tick:
                tick()
            time.sleep(0.25)
        th.join(1.0)
        if "exc" in res:
            if type(res["exc"]).__name__ in ("ConnectionLost", "GameCrashed"):
                raise res["exc"]
            return False, f"the walk raised {type(res['exc']).__name__}: {str(res['exc'])[:WHY]}"
        r = res.get("r")
        if r is None:
            return False, "walk thread gave no result"
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def distance_to(self, ref: str) -> Optional[float]:
        try:
            return float(math.dist(tuple(self.bot.pos()), tuple(self.bot.pos(ref))))
        except Exception as e:   # noqa: BLE001 - the ref is not loaded here, or the other space
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return None

    def goto_ref(self, ref: str, dist: float = 0.0) -> Tuple[bool, str]:
        """The lab's teleport lands inside a radius of 100 and the 3D distance of the second story run was still 250 (floor of the house, the NPC moved): when it is more than
        150 away the player hops onto the NPC's own position, 60 units to the side."""
        ok, why = LL.LiveMachine.goto_ref(self, ref, dist)
        if ok and not dist:
            d3 = self.distance_to(ref)
            if d3 is not None and d3 > 150.0:
                try:
                    x, y, z = self.bot.pos(ref)
                    return self.goto_pos(x + 60.0, y, z)
                except Exception as e:   # noqa: BLE001
                    if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                        raise
        return ok, why

    def harvested(self, ref: str) -> bool:
        """A harvest plant is not deleted or disabled when it is picked: its script (`XanderRootScript`, `BrocFlowerScript`) ends in `setdestroyed 1` and leaves the reference
        enabled, so the lab's flags check (FLAGS_GONE) never sees it, whoever picked it. The plant's own mark is GetDestroyed."""
        if not self.enabled(ref):
            return True                                      # disabled (its quest has not enabled it yet) or deleted
        from bots import primitives as P
        try:
            return bool(self.bot._optional_eval(P.Line("GetDestroyed", self.bot._ref_of(ref))))   # noqa: SLF001
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False

    def pickup(self, ref: str) -> Tuple[bool, str]:
        """Activate the plant (the host with the console line, a guest with the bridge call: the lab's reasons are in LiveMachine.pickup) and wait for the plant's own mark.
        The bot's pickup() asks the flags for "gone" and fails with "world ref still present" for a plant that was harvested; the item itself is checked by the step's expect.items."""
        from bots import primitives as P
        try:
            if self.role == HOST:
                self.bot._exec(P.on(self.bot._ref_of(ref), "Activate player 1"), "activate")   # noqa: SLF001
            else:
                r = self.bot.activate(ref)
                if not r.ok:
                    return False, str(r)[:WHY]
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, f"the activation raised {type(e).__name__}: {str(e)[:WHY]}"
        end = time.monotonic() + 6.0
        while time.monotonic() < end:
            if self.harvested(ref):
                return True, ""
            time.sleep(0.25)
        return False, "the plant is not marked destroyed 6 s after the activation (GetDestroyed is 0, it is still enabled)"

    def talk(self, npc: str, timeout_s: float) -> Tuple[bool, str]:
        r = self.bot.talk(npc, timeout_s=timeout_s)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def in_dialogue(self) -> bool:
        return bool(self.bot.dialogue_open())

    def choices(self) -> List[Dict[str, Any]]:
        try:
            if not self.bot.dialogue_open():
                return []
            return [{"idx": c.get("idx"), "info": c.get("info") or ""} for c in self.bot.dialog_choices()]
        except Exception as e:   # noqa: BLE001 - "no dialogue open" while the menu closes
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return []

    def say(self, info: str, timeout_s: float) -> Tuple[bool, str, Dict[str, Any]]:
        r = self.bot.choose_topic(info, timeout_s=timeout_s)
        return bool(r.ok), "" if r.ok else str(r)[:WHY], dict(getattr(r, "data", None) or {})

    def end_dialogue(self) -> Tuple[bool, str]:
        r = self.bot.end_dialogue()
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def puppets_follow(self, player_cell: int) -> Tuple[bool, str]:
        """avatar.status of this copy: the puppet it shows of the other player is Driving and in the cell that player is in (`player_cell`, read on the other copy).
        Off: a puppet in "Loading3D" in the cell it was spawned in."""
        try:
            av = self.nv.call("avatar.status", {}, timeout_s=10)
        except Exception as e:   # noqa: BLE001 - a build without the avatar layer: nothing to wait for
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return True, ""
        mine = int(player_cell)
        off = []
        for p in (av or {}).get("puppets", []) or []:
            try:
                cell = int(str(p.get("cell", "0")), 16)
            except ValueError:
                cell = 0
            if p.get("phase") != "Driving" or cell != mine:
                off.append(f"slot {p.get('slot')} {p.get('phase')} in cell {cell} (its player is in {mine})")
        return (not off), "; ".join(off)

    def activate(self, ref: str) -> Tuple[bool, str]:
        r = self.bot.activate(ref)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def intents_counters(self) -> Dict[str, Any]:
        """intents.status numbers about dialogue, stages, denials (the service counters of the guest-intents module), flattened."""
        flat: Dict[str, float] = {}
        K.C.flatten(self.nv.call("intents.status", {}, timeout_s=10), "", flat)
        return {k: v for k, v in sorted(flat.items()) if v and any(w in k.lower() for w in ("dialogue", "stage", "denied", "unsupported", "failed", "ratelimited", "notloaded", "outofrange"))}

    def item_count(self, edid: str) -> int:
        """The count of an item in this copy's player inventory (evidence for a reward that pays more than once). Goes through the bots' bridge helper, like the lab: it turns
        "Plugin.esm:HEX" into the hex runtime id with this copy's load-order index, which is all the console parser takes (a Plugin:ID there is a syntax error)."""
        if D.MUTANT == "item_console_plugin_id":      # the bug the twin guards: the Plugin:ID reaches the console parser as it is
            return int(self.nv.eval(f"player.GetItemCount {edid}"))
        return int(self.bot.inventory_count(edid))

    def dialogue_status(self) -> Dict[str, Any]:
        """coop.status "dialogue": the glue's counters (offers, picks, pickLatched, lineDones), the dialogue host's (scriptRuns, repeatPicks, rejected) and the said-once byte of
        the story's INFOs on this copy (-1 unknown, 0 not said, 1 said)."""
        return dict((self.nv.call("coop.status", {}, timeout_s=10) or {}).get("dialogue") or {})

    def scene_pass(self, quest: str) -> Tuple[bool, str]:
        from bots.playbooks import probe as PB
        r = PB.scene_pass(self.bot, quest)
        return bool(r.ok), "" if r.ok else str(r.reason)[:WHY]

    def message_box(self) -> bool:
        try:
            return (self.bot.menu_info() or {}).get("kind") == "message"
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False

    def press_box(self, button: int) -> Tuple[bool, str]:
        r = self.bot.press_box(button)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def enable_controls(self) -> Tuple[bool, str]:
        try:
            r = self.nv.exec("EnablePlayerControls")
            return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, str(e)[:WHY]

    def give_item(self, item: str, n: int) -> Tuple[bool, str]:
        r = self.bot.give(item, n)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]


def _scenario_modules():
    import scenlib                                    # noqa: WPS433
    return scenlib


class StoryLiveEnv(LL.LiveEnv):
    """One pair of real copies (instances 0 and 1), booted by prove.py's LiveEnv under the lease owner `story`. Not a LiveEnv.__init__ call: that one names the owner
    `lab`, and a lab queue running beside a story run must keep its own lease and its own copies."""
    live = True

    def __init__(self, script: D.Script, args, log: Callable[[str], None] = lambda s: None, out_dir: Optional[Path] = None):
        import prove                                  # tools/proof/prove.py
        self.script, self.args, self.log = script, args, log
        self.layout = SimpleNamespace(plugin="", cell_edid="", cell_alt="", path=None)
        self.lab_dir = Path(out_dir or ROOT / "out" / "story")
        prove.OWNER = "story"                          # its own slot lease and pid registry, set BEFORE the env adopts anything
        prove.OUT = self.lab_dir / "state"
        prove.OUT.mkdir(parents=True, exist_ok=True)
        self.prove = prove
        self.pv = prove.LiveEnv(SimpleNamespace(provision=True, boot=True, pairs=1, reserve_gb=float(getattr(args, "reserve_gb", 1.5))))
        self.nvs: Dict[int, Any] = {}
        self.held: set = set()
        self.t_start = time.time()
        self.entered: Dict[int, bool] = {}
        self.net_prof: Dict[int, Any] = {}
        self.chaos: Dict[int, Any] = {}
        self.install_script = None
        self.esp = None

    def install(self, i: int) -> Optional[str]:
        return None                                    # no lab plugin: the story runs in the real world

    def stale_copies(self, instances: List[int]) -> List[int]:
        return []

    def _have_save(self, i: int, save: str) -> bool:
        return (self.pv.idir(i) / "Saves" / (save + ".fos")).is_file()

    def prepare(self, pair: int) -> Optional[Tuple[str, str]]:
        ins = [2 * pair, 2 * pair + 1]
        test = {"id": f"story{pair}", "copies": 2, "pair": pair, "n": 1, "ini": STORY_INI, "timeout_s": 3600, "snapshot": [], "commands": [], "game_env": GAME_ENV,
                "requires_methods": ["coop.status", "coop.owner", "bot.status", "bot.menu", "bot.dialog", "avatar.status"], "also_roadmap": []}
        self.pv.hold(ins)
        self.held.update(ins)
        why = self.pv.preflight(test)
        if why:
            return "ENV", why
        env_reason, build_reason, _fresh = self.pv.prepare(test, ins)
        if env_reason:
            return "ENV", env_reason
        if build_reason:
            return "BLOCKED", build_reason
        import nvbridge
        for i in ins:
            if i not in self.nvs:
                self.nvs[i] = nvbridge.NvBridge.connect(i, wait_s=30)
        save = getattr(self.args, "save", "") or self.script.data.get("start", {}).get("save") or "nvcoop_autosave_ready"
        gone = [i for i in ins if not self._have_save(i, save)]
        if gone:
            return "BLOCKED", (f"copy(s) {gone} have no save {save!r} in their Saves folder. Make it once: python -I tools/scenarios/prepare_save.py autosave "
                               f"(it needs the pool; the proof harness copies the Saves folder of copy 0 and 1 to later copies)")
        return None

    def machines(self, pair: int):
        return (StoryLiveMachine(self.nvs[2 * pair], HOST), StoryLiveMachine(self.nvs[2 * pair + 1], CLIENT))

    LOG_KEEP = ("dialogue:", "intents: guest", "intents: dialogue", "not run", "veto", "grant", "denied", "barrier", "rules:", "stage")
    LOG_SKIP = ("node attached", "tables:", "enabled (", "observers installed")

    def _log_path(self, i: int) -> Path:
        return self.pv.idir(i) / "Data" / "NVSE" / "Plugins" / "nvcoop.log"

    def log_mark(self, pair: int) -> Dict[int, int]:
        out = {}
        for i in (2 * pair, 2 * pair + 1):
            try:
                out[i] = self._log_path(i).stat().st_size
            except OSError:
                out[i] = 0
        return out

    def log_since(self, pair: int, mark: Dict[int, int], n: int = 6) -> Dict[str, List[str]]:
        """The last n plugin log lines about dialogue, guest intents, stages and barriers that each copy wrote since the mark (what the plugin said while the step failed)."""
        out: Dict[str, List[str]] = {}
        for i, name in ((2 * pair, HOST), (2 * pair + 1, CLIENT)):
            try:
                with open(self._log_path(i), "rb") as f:
                    f.seek(mark.get(i, 0))
                    text = f.read().decode("utf-8", "replace")
            except OSError:
                continue
            lines = [l.strip()[:170] for l in text.splitlines() if any(k in l for k in self.LOG_KEEP) and not any(k in l for k in self.LOG_SKIP)]
            seen: List[str] = []
            for l in lines:
                if l not in seen[-3:]:
                    seen.append(l)
            if seen:
                out[name] = seen[-n:]
        return out

    def load_keep(self, pair: int, save: str) -> Dict[str, Any]:
        """Load `save` on both copies as it is (no quest reset, no move): the lab's way (Runner load, the load boxes pressed away, the session synced)."""
        scen_runner = LL._scenario_runner()        # noqa: SLF001
        nvs = [self.nvs[2 * pair], self.nvs[2 * pair + 1]]
        r = scen_runner.Runner(nvs, prepared=False, log=self.log)
        r._load_start_save({"start": {"save": save}})                  # noqa: SLF001
        r.sleep(r.frames / 60.0)
        boxes = r._clear_load_messages()                               # noqa: SLF001
        sync = r._wait_coop_synced(digest_timeout_s=LL.ENTER_DIGEST_S, digest_stall_s=LL.ENTER_DIGEST_S / 2)   # noqa: SLF001
        if sync and sync.get("synced") is False:
            raise Blocked(f"the pair did not sync after loading {save!r}: {sync.get('reason')}", "ENV")
        return {"boxes": boxes, "sync": sync}

    def save_as(self, pair: int, name: str) -> Dict[str, Any]:
        """The bridge's saveGame on both copies (the way tools/scenarios/prepare_save.py makes a ready save)."""
        out = {}
        for i in (2 * pair, 2 * pair + 1):
            try:
                out[i] = self.nvs[i].save_game(name, overwrite=True)
            except Exception as e:   # noqa: BLE001
                out[i] = {"error": f"{type(e).__name__}: {str(e)[:160]}"}
        return out

    def enter(self, pair: int) -> Callable[[], None]:
        """S00: load the save on both copies, wait until the session is synced, reset the Goodsprings quests, rebaseline, settle, stand both players at Doc Mitchell."""
        def go():
            if self.script.data.get("start", {}).get("keep"):      # a save that is already at the story's start: nothing is reset or moved
                self.load_keep(pair, getattr(self.args, "save", "") or self.script.data["start"]["save"])
                self.entered[pair] = True
                return
            scen_runner = LL._scenario_runner()        # noqa: SLF001
            scenlib = _scenario_modules()
            nvs = [self.nvs[2 * pair], self.nvs[2 * pair + 1]]
            sc = scenlib.load_scenario(ROOT / "scenarios" / "story_goodsprings.json")
            sc["start"] = dict(sc["start"], save=getattr(self.args, "save", "") or "nvcoop_autosave_ready")
            r = scen_runner.Runner(nvs, prepared=False, log=self.log)
            out = r.jump(sc, force_reload=True)
            self.log(f"story start: jump {out.get('seconds')} s, reload {out.get('reload')}, setup {[s.get('status') for s in out.get('setup', [])]}")
            bad = [s for s in out.get("setup", []) if s.get("status") in ("error", "blocked")]
            if bad:
                raise Blocked(f"the start state could not be reached: {bad[0].get('detail') or bad[0]}", "ENV")
            sync = (out.get("coopSync") or {})
            if sync.get("synced") is False:
                raise Blocked(f"the pair did not sync after loading the save: {sync.get('reason')}", "ENV")
            self.entered[pair] = True
        return go
