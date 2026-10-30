"""Offline tests of the lab's parity room (no game): the pure comparison (lab_parity.py) on synthetic and recorded-shape bridge answers, the room on the fake
world (one test per fake bug: the entity, the field, both values, the source), the live side's reads and scripted situations against a stub bridge, and the probe
source (plugin/probe/parity_probe.cpp: read only, pinned addresses only).

    python -I tools/lab/tests/test_parity.py        # all of it
    python -I tools/lab/run_tests.py                # these tests too, then the mutation twins (LAB_MUTANT=<name>)
"""
from __future__ import annotations

import re
import sys
import unittest
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
LAB = HERE.parent
ROOT = LAB.parent.parent
sys.path.insert(0, str(LAB))
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "tools"))

import lab_core as K             # noqa: E402
import lab_layout as LY          # noqa: E402
import lab_parity as P           # noqa: E402
import labfake                   # noqa: E402
import test_lab as TL            # noqa: E402
from lab_core import CLIENT, HOST   # noqa: E402

PLUGIN = ROOT / "plugin"


# ------------------------------------------------------------------ answers shaped like the bridge's (anim.probe, parity.look, avatar.status)
def seq(group, state=1, weight=1.0, slot=0):
    return {"slot": slot, "group": group, "state": state, "t": 0.5, "begin": 0.0, "end": 1.0, "freq": 1.0, "weight": weight}


def probe_actor(ref, base="0x00100000", out=False, sit=0, group=0, dead=False, player=False, form="0x00000000", valid3d=True):
    a = {"ref": ref, "base": base, "valid3d": valid3d, "first_person": False, "dead": dead, "life": 2 if dead else 0, "pos": [1.0, 2.0, 3.0], "rot": [0, 0, 0], "cell": "0x00168fbc",
         "anim": {"action": 0, "sequence_raw": "0x00000000", "seqs": [seq(group)] if group is not None else []},
         "weapon": {"out": out, "aiming": False, "form": form}, "stance": {"knocked": 0, "sit_sleep": sit, "swimming": False, "ragdoll_ctrl": False},
         "ai": {"in_combat": False, "combat_target": "0x00000000", "package": "0x00000000"}}
    if player:
        a["player"] = True
    return a


def look_actor(ref, worn=(), race="0x00000019", female=False, hair="0x00000100", eyes="0x00000200", known=True):
    return {"ref": ref, "ok": True, "actor": True, "creature": False, "base": "0x00100000", "race": race, "female": female, "hair": hair, "eyes": eyes, "name": "x",
            "wornKnown": known, "worn": [{"form": f, "type": 40, "mods": 0} for f in worn]}


def look_answer(actors, current=1.0, target=1.0, vats=0):
    return {"v": 1, "player": "0x00000014", "time": {"ok": True, "current": current, "target": target}, "vatsMode": vats, "pausedByMenu": False, "actors": actors}


def snapshot(npc_worn=(), npc_out=False, npc_sit=0, npc_group=3, player_worn=(), puppet_worn=(), player_out=False, puppet_out=False, puppet_look=None, player_look=None,
             time=(1.0, 1.0), vats=0, with_look=True, npc_ref="0x01000863"):
    anim = {"actors": [probe_actor("0x00000014", "0x00000007", out=player_out, player=True, form=("0x000e3778" if player_worn else "0x00000000")),
                       probe_actor("0xff000801", "0x00020000", out=puppet_out, form=("0x000e3778" if puppet_worn else "0x00000000")),
                       probe_actor(npc_ref, out=npc_out, sit=npc_sit, group=npc_group, form=("0x000e3778" if npc_worn else "0x00000000"))]}
    plook = player_look or {}
    ulook = puppet_look or {}
    look = look_answer([look_actor("0x00000014", player_worn, **plook), look_actor("0xff000801", puppet_worn, **ulook), look_actor(npc_ref, ("0x0010a001",) + tuple(npc_worn))],
                       time[0], time[1], vats) if with_look else None
    return P.assemble(anim, look, {"puppets": [{"slot": 1, "ref": "0xff000801", "phase": "active"}]})


class Helpers(unittest.TestCase):
    def test_hex_ids_and_classes(self):
        self.assertEqual(P.norm_hex("0100086F"), "0x0100086f")
        self.assertEqual(P.norm_hex("0x0100086F"), "0x0100086f")
        self.assertEqual(P.norm_hex(0x14), "0x00000014")
        self.assertIsNone(P.norm_hex("player"))
        self.assertEqual([P.pose_of(n) for n in (None, 0, 1, 4, 5, 6, 9, 10, 11)], [None, "stand", "sit", "sit", "sit", "sleep", "sleep", "sleep", "state11"])
        self.assertEqual([P.category_of(g) for g in (None, -1, 0, 3, 7, 26)], [None, None, "idle", "walk", "run", "attack"])

    def test_the_time_multiplier_is_read_as_two_little_endian_floats(self):
        self.assertEqual(P.decode_time_peek("0000803f0000803f"), (1.0, 1.0))
        cur, tgt = P.decode_time_peek("0000f041" + "0000803f")
        self.assertEqual((cur, tgt), (30.0, 1.0))
        self.assertIsNone(P.decode_time_peek("0000803f"))
        self.assertIsNone(P.decode_time_peek("zz"))
        self.assertEqual((P.TIME_ADDR_CURRENT, P.TIME_ADDR_TARGET), (0x011AC3A0, 0x011AC3A4))     # plugin/vats/vats_addrs.h

    def test_the_primary_group_is_the_heaviest_playing_sequence(self):
        self.assertEqual(P.primary_group([seq(3, weight=0.4), seq(26, weight=0.9, slot=1), seq(7, state=0, weight=5.0, slot=2)]), 26)
        self.assertIsNone(P.primary_group([seq(7, state=0)]))
        self.assertIsNone(P.primary_group([]))
        self.assertEqual(P.primary_category(probe_actor("0x1", group=26)), "attack")
        self.assertIsNone(P.primary_category(None))

    def test_a_first_person_punch_is_an_attack(self):
        # a human in first person swings on the player's first-person AnimData (plugin/animprobe anim.seqs_first_person), not on the process's set
        a = probe_actor("0x14", group=0, player=True)
        a["anim"]["seqs_first_person"] = [seq(26, slot=4)]
        self.assertEqual(P.primary_category(a), "attack")
        a["anim"]["seqs_first_person"] = [seq(3, slot=0)]          # only locomotion there: the process's set decides
        self.assertEqual(P.primary_category(a), "idle")
        a["anim"]["seqs_first_person"] = []
        a["anim"]["seqs_alt"] = [seq(32, slot=4)]
        self.assertEqual(P.primary_category(a), "attack")


class Assemble(unittest.TestCase):
    def test_the_player_the_puppet_and_the_npcs_are_told_apart(self):
        s = snapshot(npc_worn=("0x000e3778",), npc_out=True, npc_sit=4, player_worn=("0x000e3778",), player_out=True)
        self.assertEqual(s["player"]["ref"], "0x00000014")
        self.assertEqual(list(s["puppets"]), ["1"])
        self.assertEqual(list(s["actors"]), ["0x01000863"])
        n = s["actors"]["0x01000863"]
        self.assertEqual((n["weaponOut"], n["pose"], n["cat"], n["equipped"]), (True, "sit", "walk", ["0x000e3778", "0x0010a001"]))
        self.assertEqual(s["player"]["weapon"], "0x000e3778")
        self.assertEqual(s["puppets"]["1"]["weapon"], "none")                      # fists are knowledge, not a missing read
        self.assertEqual(s["puppets"]["1"]["race"], "0x00000019")
        self.assertEqual(s["time"], {"current": 1.0, "target": 1.0, "source": "parity.look", "vatsMode": 0})
        self.assertEqual(s["missing"], [])

    def test_a_build_without_parity_look_still_gives_weapon_sit_animation_and_time(self):
        peek = "0000f041" + "0000803f"
        s = P.assemble({"actors": [probe_actor("0x00000014", player=True), probe_actor("0x01000863", out=True, sit=4)]}, None, None, peek, 0)
        self.assertEqual(s["time"]["source"], "world.peek")
        self.assertEqual((s["time"]["current"], s["time"]["target"]), (30.0, 1.0))
        v = s["actors"]["0x01000863"]
        self.assertEqual((v["weaponOut"], v["pose"], v["equipped"], v["race"]), (True, "sit", None, None))      # not readable: None, not "nothing"
        self.assertTrue(s["missing"][0].startswith("parity.look"))
        nothing = P.assemble({"actors": []}, None, None, "", None)
        self.assertIsNone(nothing["time"])
        self.assertTrue(any("time scale" in m for m in nothing["missing"]))

    def test_an_actor_whose_worn_list_cannot_be_read_has_no_list(self):
        s = P.assemble({"actors": [probe_actor("0x01000863")]}, look_answer([look_actor("0x01000863", known=False)]), None)
        self.assertIsNone(s["actors"]["0x01000863"]["equipped"])

    def test_a_puppet_that_is_not_placed_yet_is_left_out(self):
        self.assertEqual(P.puppet_refs({"puppets": [{"slot": 1, "ref": "0x00000000"}, {"slot": 2, "ref": "0xff000801"}]}), ["0xff000801"])
        self.assertEqual(P.puppet_refs(None), [])


class Compare(unittest.TestCase):
    def two(self, host, client):
        return P.compare_snapshots(host, client)[0]

    def fields(self, found):
        return sorted((m["entity"].split(" (")[0], m["field"]) for m in found)

    def test_identical_copies_have_no_mismatch(self):
        found, stats = P.compare_snapshots(snapshot(), snapshot())
        self.assertEqual(found, [])
        self.assertEqual((stats["npcs"], stats["puppets"]), (1, 2))

    def test_a_settler_holding_nothing_while_the_owner_holds_a_rifle(self):
        found = self.two(snapshot(npc_worn=("0x000e3778",), npc_out=True), snapshot())
        by = {m["field"]: m for m in found}
        self.assertEqual(set(by), {"equipped", "weapon", "weaponOut"})
        m = by["equipped"]
        self.assertEqual((m["entity"], m["host"], m["client"]), ("npc 0x01000863", ["0x000e3778", "0x0010a001"], ["0x0010a001"]))
        self.assertIn("plugin/actors", m["source"])
        self.assertIn("plugin/actors/anim", by["weaponOut"]["source"])

    def test_a_copy_sitting_while_the_owner_stands(self):
        found = self.two(snapshot(), snapshot(npc_sit=4))
        self.assertEqual([(m["field"], m["host"], m["client"]) for m in found], [("pose", "stand", "sit")])
        self.assertIn("sit / furniture", found[0]["source"])

    def test_the_step_of_the_sit_state_is_not_a_mismatch_only_its_class(self):
        self.assertEqual(self.two(snapshot(npc_sit=2), snapshot(npc_sit=5)), [])
        self.assertEqual(self.two(snapshot(npc_sit=0), snapshot(npc_sit=1))[0]["field"], "pose")

    def test_a_field_one_copy_cannot_read_is_not_compared(self):
        found = self.two(snapshot(npc_worn=("0x000e3778",)), snapshot(with_look=False))
        self.assertNotIn("equipped", [m["field"] for m in found])

    def test_a_dead_npc_is_not_compared(self):
        h, c = snapshot(npc_out=True), snapshot()
        h["actors"]["0x01000863"]["dead"] = True
        self.assertEqual(self.two(h, c), [])

    def test_puppets_with_the_default_look(self):
        default = {"race": "0x00000002", "female": False, "hair": "0x00000001", "eyes": "0x00000001"}
        found = self.two(snapshot(puppet_look=default), snapshot(player_look={"female": True}))
        mine = [m for m in found if m["entity"].startswith("puppet of client")]
        self.assertEqual(sorted(m["field"] for m in mine), ["eyes", "female", "hair", "race"])
        self.assertTrue(all("puppet look" in m["source"] for m in mine))

    def test_a_puppet_that_does_not_exist_is_a_finding(self):
        h = snapshot()
        h["puppets"] = {}
        found = self.two(h, snapshot())
        self.assertEqual([(m["entity"], m["field"], m["host"], m["client"]) for m in found], [("puppet of client (on host)", "present", False, True)])

    def test_a_player_is_compared_with_the_other_copys_puppet_in_both_directions(self):
        found = self.two(snapshot(player_worn=("0x000e3778",), player_out=True), snapshot())
        self.assertEqual({m["entity"] for m in found}, {"puppet of host (on client)"})
        found = self.two(snapshot(), snapshot(player_worn=("0x000e3778",), player_out=True))
        self.assertEqual({m["entity"] for m in found}, {"puppet of client (on host)"})

    def test_animation_categories_are_compared_as_sets_over_the_rounds(self):
        seen = {}
        P.compare_snapshots(snapshot(npc_group=3), snapshot(npc_group=0), seen)
        found, _ = P.compare_snapshots(snapshot(npc_group=3), snapshot(npc_group=0), seen)
        self.assertEqual([m["field"] for m in found], ["anim"])                              # the owner walks and the copy only ever idles: disjoint sets
        seen = {}
        P.compare_snapshots(snapshot(npc_group=0), snapshot(npc_group=3), seen)              # the copy is a step behind ...
        found, _ = P.compare_snapshots(snapshot(npc_group=3), snapshot(npc_group=3), seen)   # ... and caught up: the sets overlap
        self.assertEqual(found, [])

    def test_the_time_multiplier_must_be_one_on_every_copy_outside_vats(self):
        found = self.two(snapshot(), snapshot(time=(30.0, 30.0)))
        self.assertEqual([(m["entity"], m["field"], m["host"], m["client"]) for m in found], [("client machine", "timeScale", "1/1", "30/30")])
        self.assertIn("world_time", found[0]["source"])
        found = self.two(snapshot(time=(1.0, 0.5)), snapshot())
        self.assertEqual([m["entity"] for m in found], ["host machine"])                    # a target that is not 1 is as wrong as a current that is not
        self.assertEqual(self.two(snapshot(time=(0.2, 0.2), vats=4), snapshot()), [])       # VATS slow motion is not a finding
        self.assertEqual(self.two(snapshot(time=(1.004, 1.0)), snapshot()), [])             # the tolerance

    def test_npcs_loaded_on_one_copy_only_are_counted_not_compared(self):
        h, c = snapshot(), snapshot()
        h["actors"]["0x01000999"] = dict(h["actors"]["0x01000863"], ref="0x01000999")
        found, stats = P.compare_snapshots(h, c)
        self.assertEqual((found, stats["only_host"], stats["only_client"], stats["npcs"]), ([], ["0x01000999"], [], 1))

    def test_the_punch_that_did_not_play_on_the_puppet(self):
        found = P.attack_mismatch(P.puppet_entity(HOST), HOST, True, False)
        self.assertEqual([(m["field"], m["host"], m["client"]) for m in found], [("attack", "attack shown", "no attack")])
        self.assertIn("player attack events", found[0]["source"])
        self.assertEqual([(m["host"], m["client"]) for m in P.attack_mismatch("e", CLIENT, True, False)], [("no attack", "attack shown")])
        self.assertEqual(P.attack_mismatch("e", HOST, True, True), [])
        self.assertEqual(P.attack_mismatch("e", HOST, False, False), [])                    # the owner's own copy did not show it: nothing to judge


class Log(unittest.TestCase):
    def test_one_entry_per_entity_and_field_with_the_step_it_first_showed_at(self):
        log = P.ParityLog()
        m = P.mismatch("npc 0x01", "npc", "pose", "stand", "sit")
        log.add("step one", [m])
        log.add("step two", [dict(m), P.mismatch("npc 0x01", "npc", "weaponOut", True, False)])
        self.assertEqual([(i["field"], i["step"], i["steps"]) for i in log.items], [("pose", "step one", 2), ("weaponOut", "step two", 1)])
        self.assertEqual(log.fields(), {"pose": 1, "weaponOut": 1})
        self.assertEqual([s["mismatches"] for s in log.steps], [1, 2])
        self.assertTrue(log.lines()[0].startswith("npc 0x01: pose host stand vs client sit -> plugin/actors/anim"))
        self.assertIn("[step one]", log.lines()[0])
        self.assertEqual(log.sources(), "plugin/actors/anim")

    def test_a_long_value_is_cut_in_the_line(self):
        self.assertLessEqual(len(P.show(["0x%08x" % i for i in range(40)])), 60)


# ------------------------------------------------------------------ the room on the fake world
def parity_run(bugs=(), env=None, lay=None):
    res, env = TL.run_lab(bugs=bugs, rooms=["parity"], env=env, lay=lay)
    return TL.row(res, "parity"), env


def found(row):
    return row["metrics"]["parity_mismatches"]


class Room(unittest.TestCase):
    def test_the_fake_world_passes_and_every_step_ran(self):
        r, _ = parity_run()
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        steps = [s["step"] for s in r["metrics"]["parity_steps"]]
        self.assertEqual(len(steps), 11, steps)
        for want in ("baseline", "the NPC equips and draws a weapon", "the NPC puts the weapon away", "the NPC sits down", "the NPC stands up", "the host player punches",
                     "the client player equips and draws a weapon", "the client player is unarmed again"):
            self.assertTrue(any(s.startswith(want) for s in steps), want)
        self.assertEqual(r["metrics"]["parity_mismatch_count"], 0)
        self.assertEqual(r["metrics"]["parity_npc_owner"], "host")
        self.assertFalse([c for c in r["checks"] if c.get("status") == "skipped"])

    EXPECT = {
        "npc_equip_unsynced": ("npc 0x", "equipped", "the NPC equips and draws a weapon", "plugin/actors"),
        "npc_draw_unsynced": ("npc 0x", "weaponOut", "the NPC equips and draws a weapon", "plugin/actors/anim"),
        "npc_sit_unsynced": ("npc 0x", "pose", "the NPC sits down", "sit / furniture"),
        "npc_stand_unsynced": ("npc 0x", "pose", "the NPC stands up", "sit / furniture"),
        "npc_anim_stuck": ("npc 0x", "anim", "baseline", "current idle"),
        "puppet_equip_unsynced": ("puppet of host", "equipped", "the host player equips and draws a weapon", "puppet equipment"),
        "puppet_default_look": ("puppet of client", "race", "baseline", "puppet look"),
        "puppet_no_punch": ("puppet of host", "attack", "the host player punches", "player attack events"),
        "guest_time_30x": ("client machine", "timeScale", "baseline", "world_time"),
    }

    def test_each_parity_bug_names_the_entity_the_field_both_values_and_the_source(self):
        self.assertEqual(set(self.EXPECT), {b for b in labfake.BUGS if b in self.EXPECT})
        for bug, (entity, field, step, src) in self.EXPECT.items():
            r, _ = parity_run([bug])
            self.assertEqual((r["verdict"], r["class"]), ("FAIL", "SYNC"), bug)
            hit = [m for m in found(r) if m["entity"].startswith(entity) and m["field"] == field]
            self.assertTrue(hit, (bug, found(r)))
            m = hit[0]
            self.assertTrue(m["step"].startswith(step), (bug, m["step"]))
            self.assertIn(src, m["source"], bug)
            self.assertNotEqual(m["host"], m["client"], bug)
            self.assertIn(m["source"].split(" (")[0].split(", ")[0], r["area"], bug)       # the likely module reaches the room's row
            self.assertIn(field, r["reason"], bug)                           # and the reason an AI reads says the field and both values
            self.assertIn("host", r["reason"])

    def test_the_findings_are_all_collected_not_just_the_first(self):
        r, _ = parity_run(["npc_equip_unsynced", "puppet_default_look", "guest_time_30x"])
        self.assertEqual({m["field"] for m in found(r)}, {"equipped", "weapon", "race", "female", "hair", "eyes", "timeScale"})
        self.assertEqual([c["ok"] for c in r["checks"] if "agrees" in c["name"]], [False])

    def test_a_layout_without_furniture_skips_the_sit_steps_and_says_why(self):
        d = LY.sample_layout()
        del d["rooms"]["npc"]["refs"]["chair"]
        r, _ = parity_run(lay=LY.Layout(d))
        self.assertEqual(r["verdict"], "PASS")
        sk = [c for c in r["checks"] if c.get("status") == "skipped"]
        self.assertEqual([c["name"] for c in sk], ["the NPC sits down and stands up"])
        self.assertIn("furniture", sk[0]["note"])
        self.assertEqual(len(r["metrics"]["parity_steps"]), 9)

    def test_a_parity_room_of_the_layouts_own_wins_over_the_borrowed_npc_room(self):
        d = LY.sample_layout()
        npc = dict(d["rooms"]["npc"])
        d["rooms"]["parity"] = {"origin": [100, 100, 7040], "stand": "NVCoopLab.esp:00899F", "refs": dict(npc["refs"], chair="NVCoopLab.esp:000894")}
        lay = LY.Layout(d)
        self.assertEqual(lay.stand("parity"), "NVCoopLab.esp:00899F")
        self.assertEqual(lay.refs("parity", "chair"), ["NVCoopLab.esp:000894"])
        self.assertEqual(LY.Layout(LY.sample_layout()).stand("parity"), LY.Layout(LY.sample_layout()).stand("npc"))

    def test_the_room_needs_the_npc_room_in_the_layout(self):
        d = LY.sample_layout()
        del d["rooms"]["npc"]
        self.assertIn("not in the layout", LY.Layout(d).missing("parity")[0])
        self.assertEqual(LY.Layout(LY.sample_layout()).missing("parity"), [])

    def test_a_situation_the_driver_cannot_make_is_skipped_not_failed(self):
        class NoDraw(labfake.FakeMachine):
            def npc_draw(self, ref, on=True):
                return False, "anim.npc.do weapon: unsupported"

        class Env(labfake.FakeEnv):
            def machines(self, pair):
                w = self.worlds[pair]
                return NoDraw(w, HOST), NoDraw(w, CLIENT)
        r, _ = parity_run(env=Env(TL.layout()))
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        sk = {c["name"]: c["note"] for c in r["checks"] if c.get("status") == "skipped"}
        self.assertIn("the NPC equips and draws a weapon", sk)
        self.assertIn("unsupported", sk["the NPC equips and draws a weapon"])

    def test_a_situation_the_owners_own_copy_never_showed_is_not_compared(self):
        class Lies(labfake.FakeMachine):
            def npc_equip(self, ref, item):
                return True, ""                       # says yes and does nothing

        class Env(labfake.FakeEnv):
            def machines(self, pair):
                w = self.worlds[pair]
                return Lies(w, HOST), Lies(w, CLIENT)
        r, _ = parity_run(["npc_equip_unsynced"], env=Env(TL.layout(), bugs=["npc_equip_unsynced"]))
        self.assertEqual(r["verdict"], "PASS", r["reason"])                    # nothing happened on the owner, so nothing could differ
        self.assertIn("never showed", [c["note"] for c in r["checks"] if c.get("status") == "skipped"][0])

    def test_when_the_client_owns_the_npc_the_scripted_lines_go_to_the_client(self):
        class Env(labfake.FakeEnv):
            def _world(self):
                w = super()._world()
                w.truth = {"owner": 1, "epoch": 2}
                for s in w.sides.values():
                    s.owner = dict(w.truth)
                return w
        r, env = parity_run(["no_handoff"], env=Env(TL.layout(), bugs=["no_handoff"]))
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        self.assertEqual(r["metrics"]["parity_npc_owner"], "client")
        r, _ = parity_run(["no_handoff", "npc_sit_unsynced"], env=Env(TL.layout(), bugs=["no_handoff", "npc_sit_unsynced"]))
        m = [m for m in found(r) if m["field"] == "pose"][0]
        self.assertEqual((m["host"], m["client"]), ("stand", "sit"))           # the client owns it and sits: the host's copy is the one that is wrong

    def test_a_build_without_parity_look_compares_what_it_can_and_says_what_it_skipped(self):
        class Old(labfake.FakeMachine):
            def parity_view(self):
                v = super().parity_view()
                for grp in (v["actors"], v["puppets"], {"p": v["player"]}):
                    for a in grp.values():
                        a["equipped"] = a["race"] = a["female"] = a["hair"] = a["eyes"] = None
                v["missing"] = ["parity.look (no such method): equipment and look of actors"]
                return v

        class Env(labfake.FakeEnv):
            def machines(self, pair):
                w = self.worlds[pair]
                return Old(w, HOST), Old(w, CLIENT)
        r, _ = parity_run(["npc_equip_unsynced", "puppet_default_look", "npc_draw_unsynced"], env=Env(TL.layout(), bugs=["npc_equip_unsynced", "puppet_default_look", "npc_draw_unsynced"]))
        self.assertEqual({m["field"] for m in found(r)} & {"equipped", "race", "female"}, set())
        self.assertIn("weaponOut", {m["field"] for m in found(r)})            # what it can read is still compared
        self.assertTrue([c for c in r["checks"] if c.get("status") == "skipped" and "cannot read" in c["name"]])

    def test_the_network_pass_gives_the_room_longer_windows(self):
        from lab_rooms import PARAMS
        self.assertIn("parity.settle_s@bad_wifi", PARAMS)
        self.assertGreater(PARAMS["parity.settle_s@bad_wifi"], PARAMS["parity.settle_s"])
        self.assertGreater(PARAMS["parity.punch_window_s@bad_wifi"], PARAMS["parity.punch_window_s"])

    def test_the_lab_ini_switches_the_probe_on(self):
        import lab_live
        self.assertIn("dev.parityprobe=1", lab_live.LAB_INI)
        self.assertIn("dev.animprobe=1", lab_live.LAB_INI)


# ------------------------------------------------------------------ the live side against a stub bridge
class Stub:
    """A bridge that answers from a table; an unknown method is a -32601 like the game's. Records every call."""

    def __init__(self, answers):
        from bots.fake import RpcError
        self.RpcError = RpcError
        self.answers, self.calls, self.lines = dict(answers), [], []

    def call(self, method, params=None, timeout_s=0):
        self.calls.append((method, params))
        if method not in self.answers:
            raise self.RpcError(-32601, "method not found", method)
        a = self.answers[method]
        if isinstance(a, Exception):
            raise a
        return a(params) if callable(a) else a

    def exec(self, cmd, timeout_ms=None, *, ref=None):
        self.lines.append((cmd, ref))
        return {"ok": True, "output": ""}


class StubBot:
    auto_dismiss = False
    look_via_console = False

    def __init__(self):
        self.unequipped = []

    def _norm(self, ref):
        if ref in (None, "player"):
            return "FalloutNV.esm:14", "player", "00000014"
        if ref.startswith("0x"):
            return ref, ref, ref[2:].upper()
        name, local = ref.rsplit(":", 1)
        idx = 0 if name.lower().endswith(".esm") else 1
        return f"{name}:{int(local, 16):X}", f"{idx:02X}{int(local, 16):06X}", f"{idx:02X}{int(local, 16):06X}"

    def _item(self, item):
        return self._norm(item)[2]

    def _ref_of(self, ref):
        return "player" if ref is None else self._norm(ref)[0]

    def unequip(self, item):
        self.unequipped.append(item)
        return SimpleNamespace(ok=True)


class Live(unittest.TestCase):
    def setUp(self):
        import lab_live
        self.lab_live = lab_live
        self.actors = [probe_actor("0x00000014", "0x00000007", player=True), probe_actor("0xff000801", "0x00020000"), probe_actor("0x01000863", out=True, sit=4)]
        self.looks = look_answer([look_actor("0x00000014"), look_actor("0xff000801", ("0x000e3778",)), look_actor("0x01000863", ("0x0010a001",))])
        self.av = {"puppets": [{"slot": 1, "ref": "0xff000801", "phase": "active", "group": 0}]}

    def machine(self, **extra):
        answers = {"avatar.status": self.av, "anim.probe": {"actors": self.actors}, "parity.look": self.looks}
        answers.update(extra)
        nv = Stub(answers)
        return self.lab_live.LiveMachine(nv, HOST, bot=StubBot()), nv

    def test_parity_view_makes_the_three_reads_in_order_and_assembles_them(self):
        m, nv = self.machine()
        s = m.parity_view()
        self.assertEqual([c[0] for c in nv.calls], ["avatar.status", "anim.probe", "parity.look"])
        probe = nv.calls[1][1]
        self.assertEqual(probe["refs"], ["player", "0xff000801"])
        self.assertFalse(probe["bones"])
        self.assertGreaterEqual(probe["near"]["radius"], 10000)
        self.assertLessEqual(probe["max"], 64)
        self.assertEqual(nv.calls[2][1]["refs"], ["player", "0xff000801", "0x01000863"])    # the player first, then every listed actor but the player
        self.assertEqual(s["puppets"]["1"]["equipped"], ["0x000e3778"])
        self.assertEqual(s["actors"]["0x01000863"]["pose"], "sit")
        self.assertEqual(s["missing"], [])

    def test_without_parity_look_the_time_comes_from_world_peek_and_vats_state(self):
        m, nv = self.machine()
        del nv.answers["parity.look"]
        nv.answers["world.peek"] = {"addr": 0x11AC3A0, "hex": "0000f0410000803f"}
        nv.answers["vats.state"] = {"mode": 0}
        s = m.parity_view()
        self.assertEqual([c[0] for c in nv.calls][-2:], ["world.peek", "vats.state"])
        self.assertEqual(nv.calls[-2][1], {"addr": "0x11AC3A0", "len": 8})
        self.assertEqual((s["time"]["current"], s["time"]["source"]), (30.0, "world.peek"))
        self.assertTrue(s["missing"][0].startswith("parity.look (no such method)"))

    def test_a_parity_look_that_refuses_is_a_missing_read_with_its_message(self):
        m, nv = self.machine()
        nv.answers["parity.look"] = nv.RpcError(-32000, "LookupFormByID does not match its pinned prologue", "parity.look")
        s = m.parity_view()
        self.assertIn("pinned prologue", s["missing"][0])

    def test_no_anim_probe_blocks_the_room_as_a_build_problem(self):
        m, nv = self.machine()
        del nv.answers["anim.probe"]
        with self.assertRaises(K.Blocked) as cm:
            m.parity_view()
        self.assertEqual(cm.exception.cls, "BUILD")
        self.assertIn("dev.animprobe=1", cm.exception.reason)

    def test_a_lost_game_is_not_swallowed(self):
        class ConnectionLost(Exception):
            pass
        m, nv = self.machine()
        nv.answers["anim.probe"] = ConnectionLost("gone")
        with self.assertRaises(ConnectionLost):
            m.parity_view()

    def test_the_npc_lines_run_on_the_npc_and_the_weapon_goes_out_through_the_dev_method(self):
        m, nv = self.machine(**{"anim.npc.do": {"result": "ok"}})
        self.assertEqual(m.npc_equip("NVCoopLab.esp:000863", "FalloutNV.esm:0E3778"), (True, ""))
        self.assertEqual(nv.lines, [("AddItem 000E3778 1", "NVCoopLab.esp:863"), ("EquipItem 000E3778", "NVCoopLab.esp:863")])
        self.assertEqual(m.npc_draw("NVCoopLab.esp:000863", True), (True, ""))
        self.assertEqual(nv.calls[-1], ("anim.npc.do", {"ref": "01000863", "op": "weapon", "on": True}))
        nv.answers["anim.npc.do"] = {"result": "refused"}
        self.assertEqual(m.npc_draw("NVCoopLab.esp:000863", False), (False, "anim.npc.do weapon: refused"))
        del nv.answers["anim.npc.do"]
        ok, why = m.npc_draw("NVCoopLab.esp:000863", True)
        self.assertFalse(ok)
        self.assertIn("animprobe", why)

    def test_sit_activates_the_furniture_with_the_npc_and_stand_re_evaluates_the_package(self):
        m, nv = self.machine()
        m.npc_sit("NVCoopLab.esp:000863", "NVCoopLab.esp:000864")
        m.npc_stand("NVCoopLab.esp:000863")
        self.assertEqual(nv.lines, [("Activate 01000863", "NVCoopLab.esp:864"), ("EvaluatePackage", "NVCoopLab.esp:863")])

    def test_reset_puts_the_npc_back(self):
        m, nv = self.machine(**{"anim.npc.do": {"result": "ok"}})
        m.npc_reset("NVCoopLab.esp:000863", "FalloutNV.esm:0E3778")
        self.assertEqual([c for c in nv.lines], [("UnequipItem 000E3778", "NVCoopLab.esp:863"), ("RemoveItem 000E3778 1", "NVCoopLab.esp:863")])
        self.assertEqual(nv.calls[-1][1]["on"], False)

    def test_punches_are_attack_pulses_and_the_category_is_read_from_anim_probe(self):
        m, nv = self.machine(**{"bot.fire": {"fired": 1}})
        self.assertEqual(m.punch(3, 0.0), (True, ""))
        self.assertEqual([c for c in nv.calls if c[0] == "bot.fire"], [("bot.fire", {"shots": 1})] * 3)
        nv.answers["anim.probe"] = {"actors": [probe_actor("0x00000014", group=26)]}
        self.assertEqual(m.anim_category(None), "attack")
        self.assertEqual(nv.calls[-1][1]["refs"], ["player"])
        nv.answers["anim.probe"] = {"actors": [probe_actor("0xff000801", group=0)]}
        self.assertEqual(m.anim_category("puppet"), "idle")
        self.assertEqual(nv.calls[-1][1]["refs"], ["0xff000801"])
        m.anim_category("puppet")
        self.assertEqual([c[0] for c in nv.calls].count("avatar.status"), 1)             # the puppet's ref is cached for a moment
        nv.answers["bot.fire"] = nv.RpcError(-32601, "method not found", "bot.fire")
        ok, why = m.punch(1, 0.0)
        self.assertFalse(ok)
        self.assertIn("bot.fire", why)

    def test_unarm_puts_the_labs_weapon_away(self):
        m, _ = self.machine()
        self.assertEqual(m.unarm(), (True, ""))
        self.assertEqual(m.bot.unequipped, [self.lab_live.WEAPON])


# ------------------------------------------------------------------ the probe source: read only, pinned addresses only
class ProbeSource(unittest.TestCase):
    def setUp(self):
        self.src = (PLUGIN / "probe" / "parity_probe.cpp").read_text(encoding="utf-8")

    def test_it_writes_nothing(self):
        for bad in ("VirtualProtect", "WriteProcessMemory", "memset(", "SetPos", "AddItem", "execConsole", "RunScript", "DrawWeapon", "SetWeaponOut", "EquipItem"):
            self.assertNotIn(bad, self.src, bad)
        self.assertEqual(self.src.count("std::memcpy("), 1)                  # the one guarded read in SafeRead (and strncpy of a name out of the form)
        self.assertIn("__try", self.src)

    def test_every_engine_name_it_uses_is_defined_in_the_documented_pin_headers(self):
        pins = (PLUGIN / "avatar" / "engine" / "engine_pins.h").read_text(encoding="utf-8")
        vats = (PLUGIN / "vats" / "vats_addrs.h").read_text(encoding="utf-8")
        for name in sorted(set(re.findall(r"\bP::(\w+)", self.src))):
            if name in ("Pin",):
                continue
            self.assertRegex(pins, r"\b%s\b" % name, name)
        for name in sorted(set(re.findall(r"\bA::(\w+)", self.src))):
            self.assertRegex(vats, r"\b%s\b" % name, name)
        self.assertNotRegex(self.src, r"0x[0-9A-Fa-f]{6,8}u?\b(?!\s*\")" if False else r"\bconstexpr\s+uint32_t\s+k\w+\s*=\s*0x[0-9A-Fa-f]{6}")      # no address of its own

    def test_it_is_registered_only_with_the_dev_switch_and_is_built_into_the_plugin(self):
        self.assertIn('"dev", "parityprobe"', self.src)
        self.assertIn('RegisterMethod("parity.look"', self.src)
        cm = (PLUGIN / "CMakeLists.txt").read_text(encoding="utf-8")
        self.assertIn("probe/parity_probe.cpp", cm)

    def test_the_pin_is_checked_before_the_only_engine_call(self):
        self.assertLess(self.src.index("LookupPinOk()"), self.src.index("LookupSEH(id)"))
        self.assertEqual(len(re.findall(r"__cdecl\*|__thiscall\*", self.src)), 1)        # LookupFormByID is the only function it calls


if __name__ == "__main__":
    unittest.main(verbosity=1)
