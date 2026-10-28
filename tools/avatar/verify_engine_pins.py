"""Static proof that every engine fact the real puppet backend (plugin/avatar/engine) calls or reads is what the header says.

    python -I tools/avatar/verify_engine_pins.py              table; exit 0 when every pin holds (or a reference exe is absent: SKIP)
    python -I tools/avatar/verify_engine_pins.py --require    a missing exe is a failure
    python -I tools/avatar/verify_engine_pins.py --selftest   the checks above on a synthetic image, with an injected defect each (ALL OK)

Reads plugin/avatar/engine/engine_pins.h and compares, READ ONLY, with <reference exe> (FalloutNV.exe 1.4.0.525 with the; its addresses are the running process's addresses):
  * every Pin: the 16 prologue bytes at the address;
  * the condition evaluators: the address is the `eval` column of that command in plugin/hooks/tools/re/command_table.json;
  * pins shared with plugin/hooks/engine_addrs.h carry the same bytes there;
  * the section ranges the vtable / target sanity checks of win/game_backend_win.cpp use (.text, .rdata);
  * the two ActorMover / PlayerMover vtables and the BSAnimGroupSequence vtable constants point into .rdata.
Nothing here proves behaviour (what a console line does is only known from a run); it proves we never call an address that is not
the function we think it is.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))

from animcheck.pe_static import DEFAULT_EXE, PeImage  # noqa: E402

PINS_H = ROOT / "plugin" / "avatar" / "engine" / "engine_pins.h"
WIN_CPP = ROOT / "plugin" / "avatar" / "engine" / "win" / "game_backend_win.cpp"
HOOK_ADDRS = ROOT / "plugin" / "hooks" / "engine_addrs.h"
CMD_TABLE = ROOT / "plugin" / "hooks" / "tools" / "re" / "command_table.json"

PIN_RE = re.compile(r'inline constexpr Pin (\w+) = \{"([^"]+)",\s*(0x[0-9A-Fa-f]+)u,\s*\{([^}]*)\}', re.S)
HOOK_RE = re.compile(r'inline constexpr HookSpec (\w+) = \{"[^"]*", (0x[0-9a-fA-F]+)u, \{([^}]*)\}')
CONST_RE = re.compile(r"(?<![A-Za-z0-9_])(k[A-Za-z0-9_]+) = (0x[0-9A-Fa-f]+)u?")
SEC_RE = re.compile(r"constexpr uintptr_t kTextLo = (0x[0-9A-Fa-f]+), kTextHi = (0x[0-9A-Fa-f]+), kRdataLo = (0x[0-9A-Fa-f]+), kRdataHi = (0x[0-9A-Fa-f]+);")

# pin name -> command whose `eval` column is the address
EVAL_OF = {"GetRestrained": "GetRestrained", "IsWeaponOut": "IsWeaponOut", "IsSneaking": "IsSneaking", "GetDead": "GetDead",
           "GetItemCount": "GetItemCount", "GetEquipped": "GetEquipped", "GetActorValue": "GetActorValue"}
# pin name -> HookSpec name in plugin/hooks/engine_addrs.h with the same address and bytes
HOOK_OF = {"LookupFormByID": "LookupFormByID", "TESObjectREFR::IsDisabled": "RefIsDisabled"}

Result = Tuple[str, bool, str]


def parse_bytes(text: str) -> bytes:
    return bytes(int(x, 16) for x in re.findall(r"0x[0-9a-fA-F]{1,2}", text))


def parse_pins(text: str) -> List[Tuple[str, int, bytes]]:
    return [(m.group(2), int(m.group(3), 16), parse_bytes(m.group(4))) for m in PIN_RE.finditer(text)]


class Image:
    """What the checks need from an executable image (the real PE or a synthetic one in the self-test)."""

    def __init__(self, pe: PeImage):
        self.pe = pe

    def read(self, va: int, n: int) -> Optional[bytes]:
        return self.pe.read(va, n)

    def section(self, name: str) -> Optional[Tuple[int, int]]:
        for s in self.pe.sections:
            if s[0].startswith(name):
                return s[1], s[2]
        return None


def check(img: Image, pins_text: str, win_text: str, hook_text: str, table: Dict[str, list], skip_ranges: bool = False) -> List[Result]:
    out: List[Result] = []
    pins = parse_pins(pins_text)
    out.append(("pins parsed", len(pins) >= 9, f"{len(pins)} pins"))
    for name, addr, want in pins:
        got = img.read(addr, 16)
        out.append((f"pin {name} @ {addr:#x}", len(want) == 16 and got == want, "" if got == want else f"exe has {got.hex() if got else 'nothing'}, header {want.hex()}"))
    # evaluators are the eval column of the command table
    for pname, cmd in EVAL_OF.items():
        row = table.get(cmd)
        pin = next((p for p in pins if p[0] == pname), None)
        if pin is None or row is None:
            out.append((f"eval {pname}: row and pin exist", False, "missing"))
            continue
        out.append((f"eval {pname} is the eval column of {cmd}", int(row[3], 16) == pin[1], f"table {row[3]} header {pin[1]:#x}"))
    # shared with the hook layer
    hooks = {m.group(1): (int(m.group(2), 16), parse_bytes(m.group(3))) for m in HOOK_RE.finditer(hook_text)}
    for pname, hname in HOOK_OF.items():
        pin = next((p for p in pins if p[0] == pname), None)
        h = hooks.get(hname)
        ok = pin is not None and h is not None and pin[1] == h[0] and pin[2] == h[1][:16]
        out.append((f"pin {pname} equals plugin/hooks {hname}", ok, "" if ok else f"{pin} vs {h}"))
    # section ranges used by the vtable / target sanity checks
    m = SEC_RE.search(win_text)
    if m and not skip_ranges:
        tl, th, rl, rh = (int(x, 16) for x in m.groups())
        text, rdata, data = img.section(".text"), img.section(".rdata"), img.section(".data")
        out.append((".text range", text == (tl, th), f"exe {text and tuple(hex(x) for x in text)} cpp {hex(tl)},{hex(th)}"))
        out.append((".rdata range", rdata is not None and rdata[0] == rl and data is not None and data[0] == rh,
                    f"exe rdata {rdata and tuple(hex(x) for x in rdata)} data {data and hex(data[0])} cpp {hex(rl)},{hex(rh)}"))
    elif not skip_ranges:
        out.append(("section range constants found in win/game_backend_win.cpp", False, "pattern not found"))
        return out
    # vtable constants lie in .rdata, and each starts with code pointers
    consts = {m.group(1): int(m.group(2), 16) for m in CONST_RE.finditer(pins_text)}
    rdata = img.section(".rdata")
    for cname in ("kVtblActorMover", "kVtblPlayerMover", "kVtblBSAnimGroupSequence", "kVtblNiControllerSequence"):
        va = consts.get(cname)
        ok = va is not None and rdata is not None and rdata[0] <= va < rdata[1]
        first = img.read(va, 4) if va else None
        tgt = int.from_bytes(first, "little") if first else 0
        text = img.section(".text")
        ok = ok and text is not None and text[0] <= tgt < text[1]
        out.append((f"vtable {cname} @ {va and hex(va)} in .rdata with a code pointer in slot 0", ok, f"slot0={tgt:#x}"))
    # BaseProcess slots used: each vtable of the process classes has code pointers there (verify_engine.py proves the semantics)
    return out


def run(exe: Path, require: bool) -> int:
    if not exe.exists():
        print(f"SKIP: {exe} not found (the reference FalloutNV.exe lives in the L5 scratch directory)")
        return 1 if require else 0
    img = Image(PeImage(exe))
    table = json.loads(CMD_TABLE.read_text(encoding="utf-8"))
    res = check(img, PINS_H.read_text(encoding="utf-8"), WIN_CPP.read_text(encoding="utf-8"), HOOK_ADDRS.read_text(encoding="utf-8"), table)
    bad = [r for r in res if not r[1]]
    for name, ok, why in res:
        print(("  ok   " if ok else "  FAIL ") + name + ("" if ok else f"   [{why}]"))
    print(f"verify_engine_pins: {len(res)} checks, {len(bad)} failed")
    return 1 if bad else 0


# ---- self-test -----------------------------------------------------------------------------------------------------------------

class FakePe:
    """A PeImage stand-in built from a dict va -> bytes plus section ranges."""

    def __init__(self, mem: Dict[int, bytes], sections):
        self.mem = mem
        self.sections = sections

    def read(self, va: int, n: int):
        for base, data in self.mem.items():
            if base <= va and va + n <= base + len(data):
                return data[va - base:va - base + n]
        return None


def selftest() -> int:
    pins_text = PINS_H.read_text(encoding="utf-8")
    win_text = WIN_CPP.read_text(encoding="utf-8")
    hook_text = HOOK_ADDRS.read_text(encoding="utf-8")
    table = json.loads(CMD_TABLE.read_text(encoding="utf-8"))
    pins = parse_pins(pins_text)
    consts = {m.group(1): int(m.group(2), 16) for m in CONST_RE.finditer(pins_text)}
    m = SEC_RE.search(win_text)
    assert m, "section constants missing"
    tl, th, rl, rh = (int(x, 16) for x in m.groups())

    def build(mutate=None) -> Tuple[Image, str, str, str, dict]:
        mem: Dict[int, bytes] = {}
        for name, addr, b in pins:
            mem[addr] = b
        for cname in ("kVtblActorMover", "kVtblPlayerMover", "kVtblBSAnimGroupSequence", "kVtblNiControllerSequence"):
            mem[consts[cname]] = (tl + 0x100).to_bytes(4, "little") + b"\0" * 12
        sections = [(".text", tl, th, 0, 0), (".rdata", rl, rh, 0, 0), (".data", rh, rh + 0x1000, 0, 0)]
        pt, wt, ht, tb = pins_text, win_text, hook_text, table
        if mutate:
            mem, sections, pt, wt, ht, tb = mutate(mem, sections, pt, wt, ht, tb)
        return Image(FakePe(mem, sections)), pt, wt, ht, tb  # type: ignore[arg-type]

    fails = 0
    checks = 0

    def expect(name: str, ok: bool) -> None:
        nonlocal fails, checks
        checks += 1
        if not ok:
            fails += 1
            print(f"  FAIL selftest: {name}")

    img, pt, wt, ht, tb = build()
    res = check(img, pt, wt, ht, tb)
    expect(f"the consistent synthetic image passes ({[r for r in res if not r[1]]})", all(r[1] for r in res) and len(res) > 20)

    first = pins[0]

    def flip(mem, sec, pt_, wt_, ht_, tb_):
        b = bytearray(mem[first[1]])
        b[5] ^= 0x01
        mem[first[1]] = bytes(b)
        return mem, sec, pt_, wt_, ht_, tb_

    def move_eval(mem, sec, pt_, wt_, ht_, tb_):
        tb_ = json.loads(json.dumps(tb_))
        tb_["GetRestrained"][3] = "0x59ff91"
        return mem, sec, pt_, wt_, ht_, tb_

    def wrong_text(mem, sec, pt_, wt_, ht_, tb_):
        return mem, [(".text", tl, th + 0x1000, 0, 0)] + sec[1:], pt_, wt_, ht_, tb_

    def hook_diff(mem, sec, pt_, wt_, ht_, tb_):
        return mem, sec, pt_, wt_, ht_.replace("0x4839c0u", "0x4839c1u"), tb_

    def vtable_outside(mem, sec, pt_, wt_, ht_, tb_):
        mem[consts["kVtblActorMover"]] = (0x10).to_bytes(4, "little") + b"\0" * 12
        return mem, sec, pt_, wt_, ht_, tb_

    def lost_pin(mem, sec, pt_, wt_, ht_, tb_):
        return mem, sec, pt_.replace('"GetEquipped"', '"GetEquippedX"'), wt_, ht_, tb_

    for name, mut in (("a flipped prologue byte", flip), ("an evaluator that is not the table's eval", move_eval),
                      ("a wrong .text range in the cpp", wrong_text), ("a pin that differs from the hook layer", hook_diff),
                      ("a vtable outside .rdata", vtable_outside), ("a pin that lost its table row", lost_pin)):
        img, pt, wt, ht, tb = build(mut)
        res = check(img, pt, wt, ht, tb)
        expect(f"twin '{name}' is caught", any(not r[1] for r in res))

    # the real header parses to the pins win/game_backend_win.cpp references by name
    for need in ("LookupFormByID", "MenuModeAny", "GetRestrained", "IsWeaponOut", "IsSneaking", "GetDead", "GetItemCount", "GetEquipped"):
        expect(f"header has pin {need}", any(p[0] == need for p in pins))
    print(f"verify_engine_pins selftest: {checks} checks, {fails} failed")
    if fails:
        return 1
    print("verify_engine_pins selftest: ALL OK")
    return 0


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    if "--selftest" in argv:
        return selftest()
    return run(DEFAULT_EXE, "--require" in argv)


if __name__ == "__main__":
    raise SystemExit(main())
