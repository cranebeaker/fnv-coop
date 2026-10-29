"""Static check of the BaseProcess virtual functions the NPC animation backend may CALL (plugin/actors/anim/anim_game.cpp).

Reads <reference exe> (FalloutNV.exe 1.4.0.525) READ ONLY, like tools/animcheck/verify_engine.py, and compares the
bytes of each function with what the xNVSE header (nvse/GameProcess.h, class BaseProcess) says it is. No disassembler needed. The slot numbers are
the position in the HighProcess vtable (the process class of every loaded actor); verify_engine.py proves that vtable's address and the getter slots
next to the setters checked here.

    python -I plugin/actors/anim/verify_slots.py            table, exit 0 if every check holds (or the exe is absent: SKIP)
    python -I plugin/actors/anim/verify_slots.py --mutant   self test: one expectation is wrong, so this MUST fail

What this does NOT prove: what the functions do to the running game (SetWeaponOut also notifies the actor, PushActorAway's force units).
Those are the in-game proofs in docs/tools/enemy-anim.md.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import List, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from animcheck.pe_static import DEFAULT_EXE, PeImage  # noqa: E402

HIGH_PROCESS_VTABLE = 0x01087864   # verified by tools/animcheck/verify_engine.py (RTTI walk)
PRO = bytes.fromhex("558bec51894dfc8b45fc")   # push ebp; mov ebp,esp; push ecx; mov [ebp-4],ecx; mov eax,[ebp-4]

Result = Tuple[str, bool, str]


def run(exe: Path = DEFAULT_EXE, mutant: bool = False) -> List[Result]:
    img = PeImage(exe)
    out: List[Result] = []

    def ok(name: str, cond: bool, detail: str = "") -> None:
        out.append((name, bool(cond), detail))

    def fn(slot: int, n: int = 0x100) -> bytes:
        va = img.vslot(HIGH_PROCESS_VTABLE, slot)
        return (img.read(va, n) or b"") if va else b""

    flag_aim = 0x228 if not mutant else 0x229
    # SetIsAiming(bool) = slot 0x100: writes the byte at +0x228; the getter IsAiming (slot 0x101, proven in verify_engine.py) reads the same byte
    b = fn(0x100, 0x20)
    want = PRO + bytes.fromhex("8a4d08") + bytes.fromhex("8888") + flag_aim.to_bytes(4, "little") + bytes.fromhex("8be55dc20400")
    ok("HighProcess slot 0x100 SetIsAiming(bool) writes byte +0x228 and pops 4", b.startswith(want), b[:0x20].hex())

    # SetKnockedState(char) = slot 0x104: writes the byte at +0x13C; the getter GetKnockedState (slot 0x103) reads +0x13C
    b = fn(0x104, 0x20)
    want = PRO + bytes.fromhex("8a4d08") + bytes.fromhex("8888") + (0x13C).to_bytes(4, "little") + bytes.fromhex("8be55dc20400")
    ok("HighProcess slot 0x104 SetKnockedState(char) writes byte +0x13C and pops 4", b.startswith(want), b[:0x20].hex())

    # SetWeaponOut(Actor*, bool) = slot 0x116: compares the byte at +0x135 with arg2, stores it, and when an actor was passed notifies it
    # through two of the actor's virtual functions (offsets 0x1E4 and 0x1E8). The getter IsWeaponOut (slot 0x115) reads +0x135.
    b = fn(0x116, 0x80)
    flag = (0x135).to_bytes(4, "little")
    ok("HighProcess slot 0x116 SetWeaponOut starts with the standard prologue", b.startswith(PRO), b[:10].hex())
    ok("SetWeaponOut reads byte +0x135 (movzx ecx,[eax+135h]) and compares it with arg2", bytes.fromhex("0fb688") + flag in b and bytes.fromhex("0fb6550c3bca") in b, "")
    ok("SetWeaponOut stores arg2 into byte +0x135 (mov [eax+135h],cl)", bytes.fromhex("8a4d0c8888") + flag in b, "")
    ok("SetWeaponOut notifies the actor through its vtable +0x1E4 and +0x1E8", (bytes.fromhex("8b90") + (0x1E4).to_bytes(4, "little")) in b and (bytes.fromhex("8b82") + (0x1E8).to_bytes(4, "little")) in b, "")
    g = fn(0x115, 0x20)
    ok("HighProcess slot 0x115 IsWeaponOut reads byte +0x135 (the flag SetWeaponOut writes)", g.startswith(PRO + bytes.fromhex("8a80") + flag), g[:0x14].hex())

    # GetSitSleepState() = slot 0x12F: returns the byte at +0x13D of the process (xNVSE BaseProcess::kSitSleepState_*: 4 = sitting, 9 = sleeping); when a
    # virtual of the process (vtable +0x618) reports bit 3 the values 4 and 9 come back as 5 and 10 (want to stand / wake). Read only, no side effect.
    b = fn(0x12F, 0x50)
    ok("HighProcess slot 0x12F GetSitSleepState reads byte +0x13D and special-cases 4 and 9", b.startswith(PRO[:3]) and bytes.fromhex("0fb6913d010000") in b and bytes.fromhex("83fa04") in b and bytes.fromhex("83f909") in b, b[:0x40].hex())

    # PushActorAway(Actor* pushed, float x, y, z, float force) = slot 0x106: exists, has a real frame, takes the actor first (never called
    # unless nvcoop.ini [anim] knockdown=1; its force units and effect are an in-game proof)
    b = fn(0x106, 0x40)
    ok("HighProcess slot 0x106 PushActorAway is a function with a stack frame", b[:1] == b"\x53" or b.startswith(PRO[:3]), b[:12].hex())
    # The pins header the game backend compiles must hold exactly these slots and function addresses (a typo there would call the wrong function)
    pins = (HERE / "anim_pins.h").read_text(encoding="utf-8")

    def const(name: str) -> int:
        m = re.search(r"\b" + name + r"\s*=\s*(0[xX][0-9A-Fa-f]+)u?", pins)
        return int(m.group(1), 16) if m else -1

    ok("anim_pins.h kHighProcessVtable is the HighProcess vtable", const("kHighProcessVtable") == HIGH_PROCESS_VTABLE, hex(const("kHighProcessVtable")))
    for slot_name, fn_name in (("kSlotSetIsAiming", "kFnSetIsAiming"), ("kSlotSetKnockedState", "kFnSetKnockedState"), ("kSlotPushActorAway", "kFnPushActorAway"),
                               ("kSlotIsWeaponOut", "kFnIsWeaponOut"), ("kSlotSetWeaponOut", "kFnSetWeaponOut"),
                               ("kSlotGetSitSleepState", "kFnGetSitSleepState")):
        slot, want = const(slot_name), const(fn_name) + (4 if mutant and fn_name == "kFnSetWeaponOut" else 0)
        got = img.vslot(HIGH_PROCESS_VTABLE, slot) if slot >= 0 else None
        ok(f"anim_pins.h {slot_name}={slot:#x} holds {fn_name}={want:#x} in the exe", got == want, f"exe has {got and hex(got)}")
    mm = const("kMenuModeAny")
    b8 = img.read(mm, 8) if mm > 0 else None
    pinned = re.search(r"kMenuModeBytes\[8\]\s*=\s*\{([^}]*)\}", pins)
    want_b = bytes(int(x, 16) for x in re.findall(r"0[xX][0-9A-Fa-f]{2}", pinned.group(1))) if pinned else b""
    ok("anim_pins.h kMenuModeAny: the first 8 bytes in the exe equal kMenuModeBytes", b8 is not None and b8 == want_b, (b8 or b"").hex())
    return out


def main(argv) -> int:
    if not DEFAULT_EXE.exists():
        print(f"SKIP verify_slots: {DEFAULT_EXE} not present (regenerate per plugin/hooks/tools/re/README.md)")
        return 0
    res = run(mutant="--mutant" in argv)
    bad = 0
    for name, good, detail in res:
        print(f"[{'ok' if good else 'FAIL'}] {name}" + ("" if good else f"   ({detail})"))
        bad += 0 if good else 1
    print(f"verify_slots: {len(res)} checks, {bad} failed")
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
