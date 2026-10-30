#!/usr/bin/env python
"""Writes tools/parity/vectors.json: the answers both comparers (plugin/parity C++ and tools/parity/compare.py) must give to the same inputs.

    python -I tools/parity/gen_vectors.py            rewrite vectors.json
    python -I tools/parity/gen_vectors.py --check    exit 1 when the checked-in file differs from what this script writes

The C++ test (plugin/parity/tests/test_parity.cpp) and tools/parity/tests/test_parity.py both read the file, so a change to a rule in one language that
is not made in the other fails one of the two suites. Samples use the same JSON shape as a line of nvcoop_parity.jsonl (plugin/parity/parity_json.h).
Default tolerances: posTol 210, posHard 1400, lag 0.9 s, skew 0.15 s, persist 3000 ms, health gap 2 buckets, attack gap 2.
"""
import argparse
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import compare  # noqa: E402  (the offline comparer: the group classes of all 256 ids come from it, so the C++ table is pinned to it)

OUT = HERE / "vectors.json"


def npc(ref, x=0, y=0, z=0, sp=0, se=0, d=0, dis=0, hp=5, g=0):
    return {"t": "n", "r": ref, "x": x, "y": y, "z": z, "sp": sp, "se": se, "d": d, "dis": dis, "hp": hp, "g": g}


def player(kind, slot, x=0, y=0, z=0, sp=0, se=0, d=0, hp=10, g=0, ac=0, look=None):
    e = {"t": kind, "sl": slot, "r": "0x00000014" if kind == "s" else "0x0020%04X" % slot, "x": x, "y": y, "z": z, "sp": sp, "se": se, "d": d, "dis": 0, "hp": hp, "g": g, "ac": ac}
    if look is not None:
        e["look"] = look
    return e


LOOK = {"race": "0x00000019", "f": 0, "hair": "0x00001000", "eyes": "0x00002000", "fh": "0xAABBCCDD"}


def sample(slot, ents, cov=6000, cell="0x00001234", ws="0x00000000", menu=0, seq=1):
    return {"slot": slot, "seq": seq, "ms": 0, "cell": cell, "ws": ws, "rad": 6000, "cov": cov, "menu": menu, "e": ents}


def pair(host_npcs=(), client_npcs=(), host_players=None, client_players=None, **kw):
    """host = slot 0, client = slot 1; each side has its own player as 's' and the other as 'p'."""
    hp = host_players if host_players is not None else [player("p", 1, x=100, look=LOOK), player("s", 0, look=LOOK)]
    cp = client_players if client_players is not None else [player("s", 1, x=100, look=LOOK), player("p", 0, look=LOOK)]
    return (sample(0, list(host_npcs) + hp, **kw), sample(1, list(client_npcs) + cp, **kw))


def case(name, steps):
    return {"name": name, "steps": [{"t": t, "host": h, "client": c, "expect": sorted(e), **({"skipped": True} if sk else {})} for (t, h, c, e, sk) in steps]}


def step(t, hc, expect=(), skipped=False):
    return (t, hc[0], hc[1], list(expect), skipped)


def cases():
    out = []
    A = "0x00000100"
    out.append(case("identical", [step(0, pair([npc(A)], [npc(A)])), step(3000, pair([npc(A)], [npc(A)]))]))
    out.append(case("pos_needs_persistence", [step(0, pair([npc(A, x=0)], [npc(A, x=500)])), step(3000, pair([npc(A, x=0)], [npc(A, x=500)]), ["pos|" + A])]))
    out.append(case("pos_hard_is_immediate", [step(0, pair([npc(A, x=0)], [npc(A, x=2000)]), ["pos|" + A])]))
    out.append(case("pos_moving_is_excused", [step(0, pair([npc(A, x=0, sp=304)], [npc(A, x=500, sp=304)])), step(3000, pair([npc(A, x=0, sp=304)], [npc(A, x=500, sp=304)]))]))
    out.append(case("seated_persists", [step(0, pair([npc(A, se=0)], [npc(A, se=1)])), step(3000, pair([npc(A, se=0)], [npc(A, se=1)]), ["seated|" + A])]))
    out.append(case("a_difference_that_stops_starts_over", [
        step(0, pair([npc(A, se=0)], [npc(A, se=1)])),
        step(1500, pair([npc(A, se=0)], [npc(A, se=0)])),
        step(3000, pair([npc(A, se=0)], [npc(A, se=1)])),
        step(6000, pair([npc(A, se=0)], [npc(A, se=1)]), ["seated|" + A])]))
    out.append(case("dead_differs", [step(0, pair([npc(A, d=0)], [npc(A, d=1)])), step(3000, pair([npc(A, d=0)], [npc(A, d=1)]), ["dead|" + A])]))
    out.append(case("dead_actors_are_not_compared_for_position", [step(0, pair([npc(A, d=1, x=0)], [npc(A, d=1, x=900)])), step(3000, pair([npc(A, d=1, x=0)], [npc(A, d=1, x=900)]))]))
    out.append(case("anim_class_differs", [step(0, pair([npc(A, g=0)], [npc(A, g=3)])), step(3000, pair([npc(A, g=0)], [npc(A, g=3)]), ["anim|" + A])]))
    out.append(case("walk_directions_are_one_class", [step(0, pair([npc(A, g=3)], [npc(A, g=5)])), step(3000, pair([npc(A, g=3)], [npc(A, g=5)]))]))
    out.append(case("transient_groups_are_not_compared", [step(0, pair([npc(A, g=26)], [npc(A, g=0)])), step(3000, pair([npc(A, g=26)], [npc(A, g=0)]))]))
    out.append(case("hp_gap_of_two_buckets", [step(0, pair([npc(A, hp=10)], [npc(A, hp=12)])), step(3000, pair([npc(A, hp=10)], [npc(A, hp=12)]), ["hp|" + A])]))
    out.append(case("hp_gap_of_one_bucket_is_fine", [step(0, pair([npc(A, hp=10)], [npc(A, hp=11)])), step(3000, pair([npc(A, hp=10)], [npc(A, hp=11)]))]))
    out.append(case("disabled_differs", [step(0, pair([npc(A, dis=0)], [npc(A, dis=1)])), step(3000, pair([npc(A, dis=0)], [npc(A, dis=1)]), ["disabled|" + A])]))
    out.append(case("absent_inside_coverage", [step(0, pair([npc(A, x=1000)], [])), step(3000, pair([npc(A, x=1000)], []), ["absent|" + A])]))
    out.append(case("absent_outside_coverage_is_not_a_difference", [step(0, pair([npc(A, x=1000)], [], cov=500)), step(3000, pair([npc(A, x=1000)], [], cov=500))]))
    out.append(case("absent_on_the_host_side", [step(0, pair([], [npc(A, x=1000)])), step(3000, pair([], [npc(A, x=1000)]), ["absent|" + A])]))
    out.append(case("created_actors_are_not_compared", [step(0, pair([npc("0xFF000123", x=0)], [])), step(3000, pair([npc("0xFF000123", x=0)], []))]))
    look2 = dict(LOOK, hair="0x00001001")
    face2 = dict(LOOK, fh="0x11111111")
    out.append(case("puppet_look_hair", [
        step(0, pair(host_players=[player("p", 1, x=100, look=look2), player("s", 0, look=LOOK)])),
        step(3000, pair(host_players=[player("p", 1, x=100, look=look2), player("s", 0, look=LOOK)]), ["look|player1"])]))
    out.append(case("puppet_look_face_hash_only", [
        step(0, pair(host_players=[player("p", 1, x=100, look=face2), player("s", 0, look=LOOK)])),
        step(3000, pair(host_players=[player("p", 1, x=100, look=face2), player("s", 0, look=LOOK)]), ["look|player1"])]))
    out.append(case("puppet_without_a_look", [
        step(0, pair(host_players=[player("p", 1, x=100), player("s", 0, look=LOOK)])),
        step(3000, pair(host_players=[player("p", 1, x=100), player("s", 0, look=LOOK)]), ["look|player1"])]))
    out.append(case("player_position", [
        step(0, pair(host_players=[player("p", 1, x=900, look=LOOK), player("s", 0, look=LOOK)])),
        step(3000, pair(host_players=[player("p", 1, x=900, look=LOOK), player("s", 0, look=LOOK)]), ["pos|player1"])]))
    out.append(case("attacks_the_copy_never_showed", [
        step(0, pair(host_players=[player("p", 1, x=100, ac=0, look=LOOK), player("s", 0, look=LOOK)], client_players=[player("s", 1, x=100, ac=5, look=LOOK), player("p", 0, look=LOOK)])),
        step(1000, pair(host_players=[player("p", 1, x=100, ac=0, look=LOOK), player("s", 0, look=LOOK)], client_players=[player("s", 1, x=100, ac=7, look=LOOK), player("p", 0, look=LOOK)])),
        step(4000, pair(host_players=[player("p", 1, x=100, ac=0, look=LOOK), player("s", 0, look=LOOK)], client_players=[player("s", 1, x=100, ac=7, look=LOOK), player("p", 0, look=LOOK)]), ["action|player1"])]))
    out.append(case("a_copy_that_started_late_is_not_an_attack_gap", [
        step(0, pair(host_players=[player("p", 1, x=100, ac=0, look=LOOK), player("s", 0, look=LOOK)], client_players=[player("s", 1, x=100, ac=9, look=LOOK), player("p", 0, look=LOOK)])),
        step(4000, pair(host_players=[player("p", 1, x=100, ac=1, look=LOOK), player("s", 0, look=LOOK)], client_players=[player("s", 1, x=100, ac=10, look=LOOK), player("p", 0, look=LOOK)]))]))
    out.append(case("a_menu_skips_the_round", [step(0, pair([npc(A, x=0)], [npc(A, x=5000)], menu=1), skipped=True)]))
    a, b = pair([npc(A)], [npc(A)])
    b["cell"] = "0x00009999"
    out.append(case("different_cells_skip_the_round", [(0, a, b, [], True)]))
    return out


PLUGIN_LINE_SAMPLE = sample(0, [npc("0x00000100", x=-12, y=34, z=5, sp=48, se=1, hp=7, g=3), player("p", 1, x=100, ac=2, look=LOOK), player("s", 0, look=LOOK)], seq=4)
PLUGIN_LINE_META = {"role": "host", "why": "cmp", "cmpSlot": 1, "cmpSeq": 9}
# the exact text plugin/parity/parity_json.cpp writes for that sample (the C++ test fails and prints the real text when it changes)
PLUGIN_LINE = r'''{"v":1,"k":"s","role":"host","slot":0,"seq":4,"ms":0,"wall":0,"gt":0.0000,"cell":"0x00001234","ws":"0x00000000","rad":6000,"cov":6000,"menu":0,"why":"cmp","cmp":{"slot":1,"seq":9},"e":[{"t":"n","r":"0x00000100","b":"0x00000000","x":-12,"y":34,"z":5,"sp":48,"se":1,"d":0,"dis":0,"w":0,"hp":7,"g":3,"gn":"Forward","gc":"walk"},{"t":"p","sl":1,"r":"0x00200001","b":"0x00000000","x":100,"y":0,"z":0,"sp":0,"se":0,"d":0,"dis":0,"w":0,"hp":10,"g":0,"gn":"Idle","gc":"idle","ac":2,"look":{"race":"0x00000019","f":0,"hair":"0x00001000","eyes":"0x00002000","fh":"0xAABBCCDD"}},{"t":"s","sl":0,"r":"0x00000014","b":"0x00000000","x":0,"y":0,"z":0,"sp":0,"se":0,"d":0,"dis":0,"w":0,"hp":10,"g":0,"gn":"Idle","gc":"idle","ac":0,"look":{"race":"0x00000019","f":0,"hair":"0x00001000","eyes":"0x00002000","fh":"0xAABBCCDD"}}]}'''


def vectors():
    return {
        "plugin_line": {"sample": PLUGIN_LINE_SAMPLE, "meta": PLUGIN_LINE_META, "line": PLUGIN_LINE},
        "group_class": [[3, "walk"], [6, "walk"], [7, "run"], [10, "run"], [0, "idle"], [1, "idle"], [2, "idle"], [17, "idle"], [225, "idle"], [224, "death"], [11, "transient"],
                        [15, "transient"], [23, "transient"], [26, "transient"], [167, "transient"], [173, "transient"], [223, "transient"], [227, "transient"], [255, "unknown"]],
        "primary_group": [[[0, 7, 3], 7], [[0, 3], 3], [[0, -1], 0], [[], 255], [[-1, -1], 255], [[0, 224, 7], 224], [[0, 3, 26], 26], [[17, 0], 17], [[3, 10], 10]],
        "group_class_all": [compare.group_class(g) for g in range(256)],
        "hp_bucket": [[0, 0], [-5, 0], [1, 1], [24, 1], [25, 2], [49, 2], [50, 3], [499, 20], [9999, 20]],
        "pos_tol": [{"a": 0, "b": 0, "tol": 210}, {"a": 10, "b": 20, "tol": 462}, {"a": 19, "b": 19, "tol": 529.2}],
        "cases": cases(),
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()
    text = json.dumps(vectors(), indent=1, sort_keys=True) + "\n"
    if a.check:
        if not OUT.is_file() or OUT.read_text(encoding="utf-8") != text:
            print("vectors.json is out of date: run python -I tools/parity/gen_vectors.py")
            return 1
        print("vectors.json is current")
        return 0
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print("wrote", OUT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
