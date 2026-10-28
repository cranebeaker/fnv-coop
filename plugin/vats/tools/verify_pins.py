"""Check the engine pins of plugin/vats (vats_addrs.h) against the reference FalloutNV.exe 1.4.0.525 image.

    python -I plugin/vats/tools/verify_pins.py               # against <reference exe> (or $FNV_REF_EXE); SKIP (exit 0) when the image is absent
    python -I plugin/vats/tools/verify_pins.py --require     # a missing image is a failure
    python -I plugin/vats/tools/verify_pins.py --selftest    # game-free: the checker's own logic on a synthetic image, with twins

What is checked (all read-only, nothing is executed):
  * every pinned function starts with the recorded 16 bytes;
  * every call site in every table is `E8 rel32` and reaches the function the table is for;
  * the VATSMenu vtable slot 3 holds the pinned HandleClick;
  * each pinned GameSetting object is the one the exe's static initialiser constructs for that name (the `push name` / `mov ecx, object` pair).
The reference image is FalloutNV.exe 1.4.0.525.
"""
from __future__ import annotations

import argparse
import os
import re
import struct
import sys
from pathlib import Path
from typing import Callable, Dict, List, Optional, Tuple

ROOT = Path(__file__).resolve().parents[3]
HEADER = ROOT / "plugin" / "vats" / "vats_addrs.h"
IMAGE = Path(os.environ.get("FNV_REF_EXE") or ROOT / "out" / "L5" / "re" / "fnv_reference.exe")

# function constant -> its prologue array
FUNCTIONS: List[Tuple[str, str]] = [
    ("kSetTimeScale", "kSetTimeScalePrologue"),
    ("kGetPlayerTimeMult", "kGetTimeMultPrologue"),
    ("kGetTargetTimeMult", "kGetTimeMultPrologue"),
    ("kMenuMode", "kMenuModePrologue"),
    ("kMenuModeOf", "kMenuModeOfPrologue"),
    ("kAccept", "kAcceptPrologue"),
    ("kUnclick", "kUnclickPrologue"),
    ("kLimbOfPart", "kLimbOfPartPrologue"),
    ("kMenuHandleClickFn", "kMenuHandleClickPrologue"),
    ("kGetMode", "kGetModePrologue"),
    ("kInterfaceUpdate", "kInterfaceUpdatePrologue"),
    ("kWorkerKick", "kWorkerKickPrologue"),
]
# site table -> the function its sites call
SITE_TABLES: List[Tuple[str, str]] = [
    ("kTimeScaleSites", "kSetTimeScale"),
    ("kPlayerMultSites", "kGetPlayerTimeMult"),
    ("kTargetMultSites", "kGetTargetTimeMult"),
    ("kPauseSitesMain", "kMenuMode"),
    ("kPauseSitesExtended", "kMenuMode"),
    ("kWorkerInterfaceSites", "kInterfaceUpdate"),
    ("kWorkerKickSites", "kWorkerKick"),
]
Reader = Callable[[int, int], Optional[bytes]]


def parse_header(text: str) -> Dict[str, object]:
    consts: Dict[str, int] = {}
    for m in re.finditer(r"constexpr\s+uint32_t\s+(k\w+)\s*=\s*(0x[0-9A-Fa-f]+)\s*;", text):
        consts[m.group(1)] = int(m.group(2), 16)
    # several constants on one line: `constexpr uint32_t kA = 0x1, kB = 0x2;`
    for line in text.splitlines():
        if "constexpr uint32_t" in line:
            for m in re.finditer(r"\b(k\w+)\s*=\s*(0x[0-9A-Fa-f]+)", line):
                consts.setdefault(m.group(1), int(m.group(2), 16))
    prologues: Dict[str, bytes] = {}
    for m in re.finditer(r"uint8_t\s+(k\w+)\[16\]\s*=\s*\{([^}]*)\}", text):
        prologues[m.group(1)] = bytes(int(x, 16) for x in re.findall(r"0x[0-9A-Fa-f]{2}", m.group(2)))
    tables: Dict[str, List[Tuple[str, int]]] = {}
    for m in re.finditer(r"Site\s+(k\w+)\[\]\s*=\s*\{(.*?)\};", text, re.S):
        tables[m.group(1)] = [(n, int(a, 16)) for n, a in re.findall(r'\{\s*"([^"]+)"\s*,\s*(0x[0-9A-Fa-f]+)\s*\}', m.group(2))]
    settings: Dict[str, Tuple[str, int]] = {}
    for m in re.finditer(r'GameSetting\s+(kSetting\w+)\s*=\s*\{\s*"([^"]+)"\s*,\s*(0x[0-9A-Fa-f]+)', text):
        settings[m.group(1)] = (m.group(2), int(m.group(3), 16))
    return {"consts": consts, "prologues": prologues, "tables": tables, "settings": settings}


def verify(read: Reader, h: Dict[str, object], find_string: Optional[Callable[[str], List[int]]] = None, code_refs: Optional[Callable[[int], List[int]]] = None) -> List[Tuple[str, bool, str]]:
    consts: Dict[str, int] = h["consts"]  # type: ignore[assignment]
    prologues: Dict[str, bytes] = h["prologues"]  # type: ignore[assignment]
    tables: Dict[str, List[Tuple[str, int]]] = h["tables"]  # type: ignore[assignment]
    settings: Dict[str, Tuple[str, int]] = h["settings"]  # type: ignore[assignment]
    res: List[Tuple[str, bool, str]] = []
    for fn, pro in FUNCTIONS:
        if fn not in consts or pro not in prologues:
            res.append((f"{fn}: pinned in the header", False, "missing constant or prologue array"))
            continue
        got = read(consts[fn], 16)
        want = prologues[pro]
        res.append((f"{fn} @{consts[fn]:#x} starts with the pinned 16 bytes", got == want and len(want) == 16, f"got {got.hex(' ') if got else None}"))
    for tbl, fn in SITE_TABLES:
        sites = tables.get(tbl, [])
        target = consts.get(fn)
        if not sites or target is None:
            res.append((f"{tbl}: table present", False, "missing"))
            continue
        bad = []
        for name, addr in sites:
            b = read(addr, 5)
            ok = b is not None and b[0] == 0xE8 and ((addr + 5 + struct.unpack("<i", b[1:5])[0]) & 0xFFFFFFFF) == target
            if not ok:
                bad.append(f"{name}@{addr:#x}")
        res.append((f"{tbl}: all {len(sites)} sites are `call {fn}`", not bad, ",".join(bad) or "ok"))
    if "kMenuVtable" in consts and "kMenuHandleClickFn" in consts:
        slot = read(consts["kMenuVtable"] + 4 * 3, 4)
        v = struct.unpack("<I", slot)[0] if slot else None
        res.append(("VATSMenu vtable slot 3 is the pinned HandleClick", v == consts["kMenuHandleClickFn"], f"slot3={v if v is None else hex(v)}"))
    if find_string and code_refs:
        for key, (name, obj) in settings.items():
            found = False
            for s in find_string(name):
                for ref in code_refs(s):
                    # static initialiser: push <default>; push <name>; mov ecx, <object>; call ctor
                    window = read(ref - 8, 40) or b""
                    for i in range(len(window) - 4):
                        if window[i] == 0xB9 and struct.unpack("<I", window[i + 1:i + 5])[0] == obj:
                            found = True
            res.append((f"{key}: {name} is constructed at {obj:#x}", found, "initialiser pair found" if found else "no `push name ... mov ecx, object`"))
    return res


# ---- the image ----------------------------------------------------------------------------------------------------------------------


class Image:
    def __init__(self, path: Path):
        self.data = path.read_bytes()
        pe = struct.unpack_from("<I", self.data, 0x3C)[0]
        nsec = struct.unpack_from("<H", self.data, pe + 6)[0]
        optsz = struct.unpack_from("<H", self.data, pe + 20)[0]
        self.base = struct.unpack_from("<I", self.data, pe + 24 + 28)[0]
        so = pe + 24 + optsz
        self.secs = []
        for i in range(nsec):
            name = self.data[so + i * 40: so + i * 40 + 8].rstrip(b"\0").decode()
            vsz, va, rsz, rptr = struct.unpack_from("<IIII", self.data, so + i * 40 + 8)
            self.secs.append((name, va + self.base, vsz, rptr, rsz))

    def read(self, addr: int, n: int) -> Optional[bytes]:
        for _name, va, vsz, rptr, rsz in self.secs:
            if va <= addr < va + max(vsz, rsz):
                off = rptr + (addr - va)
                b = self.data[off: off + n]
                return b if len(b) == n else None
        return None

    def find_string(self, s: str) -> List[int]:
        pat = s.encode() + b"\0"
        out, i = [], 0
        while True:
            i = self.data.find(pat, i)
            if i < 0:
                return out
            for _n, va, _vsz, rptr, rsz in self.secs:
                if rptr <= i < rptr + rsz:
                    out.append(va + (i - rptr))
            i += 1

    def code_refs(self, target: int) -> List[int]:
        pat = struct.pack("<I", target)
        out = []
        for name, va, _vsz, rptr, rsz in self.secs:
            if name != ".text":
                continue
            blob = self.data[rptr: rptr + rsz]
            i = 0
            while True:
                i = blob.find(pat, i)
                if i < 0:
                    break
                out.append(va + i)
                i += 1
        return out


# ---- self test ----------------------------------------------------------------------------------------------------------------------


def synthetic_image(h: Dict[str, object]) -> Dict[int, int]:
    mem: Dict[int, int] = {}
    consts: Dict[str, int] = h["consts"]  # type: ignore[assignment]
    prologues: Dict[str, bytes] = h["prologues"]  # type: ignore[assignment]
    tables: Dict[str, List[Tuple[str, int]]] = h["tables"]  # type: ignore[assignment]

    def put(a: int, b: bytes) -> None:
        for i, v in enumerate(b):
            mem[a + i] = v
    for fn, pro in FUNCTIONS:
        put(consts[fn], prologues[pro])
    for tbl, fn in SITE_TABLES:
        for _n, a in tables[tbl]:
            put(a, b"\xE8" + struct.pack("<i", (consts[fn] - (a + 5)) & 0xFFFFFFFF if False else consts[fn] - (a + 5)))
    put(consts["kMenuVtable"] + 12, struct.pack("<I", consts["kMenuHandleClickFn"]))
    return mem


def selftest() -> int:
    text = HEADER.read_text(encoding="utf-8")
    h = parse_header(text)
    failures = 0

    def expect(name: str, cond: bool) -> None:
        nonlocal failures
        if not cond:
            failures += 1
            print(f"  FAIL {name}")

    consts = h["consts"]  # type: ignore[assignment]
    expect("the header parses: 12 functions, 7 site tables, the vtable", all(f in consts for f, _ in FUNCTIONS) and len(h["tables"]) == 7 and "kMenuVtable" in consts)  # type: ignore[arg-type]
    expect("the table sizes are the documented ones (11, 5, 9, 2, 9, 2, 1)", [len(h["tables"][t]) for t, _ in SITE_TABLES] == [11, 5, 9, 2, 9, 2, 1])  # type: ignore[index]
    expect("five game settings are pinned at least", len(h["settings"]) >= 5)  # type: ignore[arg-type]

    def reader_of(mem: Dict[int, int]) -> Reader:
        def r(a: int, n: int) -> Optional[bytes]:
            try:
                return bytes(mem[a + i] for i in range(n))
            except KeyError:
                return None
        return r

    mem = synthetic_image(h)
    res = verify(reader_of(mem), h)
    expect("a faithful image passes every rule", all(ok for _r, ok, _d in res))
    # twins: each defect must be caught by at least one rule
    twins: Dict[str, Callable[[Dict[int, int]], None]] = {
        "a prologue byte changed": lambda m: m.__setitem__(consts["kSetTimeScale"] + 5, m[consts["kSetTimeScale"] + 5] ^ 1),  # type: ignore[index]
        "a site is not an E8 call": lambda m: m.__setitem__(h["tables"]["kTimeScaleSites"][3][1], 0xE9),  # type: ignore[index]
        "a site calls another function": lambda m: m.__setitem__(h["tables"]["kPlayerMultSites"][0][1] + 1, m[h["tables"]["kPlayerMultSites"][0][1] + 1] ^ 4),  # type: ignore[index]
        "the vtable slot differs": lambda m: m.__setitem__(consts["kMenuVtable"] + 12, m[consts["kMenuVtable"] + 12] ^ 8),  # type: ignore[index]
        "a site is unreadable": lambda m: [m.pop(h["tables"]["kPauseSitesMain"][0][1] + i) for i in range(5)],  # type: ignore[index]
        "an extended pause site moved": lambda m: m.__setitem__(h["tables"]["kPauseSitesExtended"][8][1] + 2, m[h["tables"]["kPauseSitesExtended"][8][1] + 2] ^ 0x10),  # type: ignore[index]
    }
    for name, brk in twins.items():
        m2 = dict(mem)
        brk(m2)
        expect(f"twin '{name}' is caught", not all(ok for _r, ok, _d in verify(reader_of(m2), h)))
    if failures:
        print(f"verify_pins selftest: {failures} failed")
        return 1
    print("verify_pins selftest: ALL OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--require", action="store_true", help="a missing image is a failure")
    ap.add_argument("--image", default=None)
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    path = Path(args.image) if args.image else IMAGE
    if not path.is_file():
        print(f"verify_pins: SKIP, no image at {path} (set FNV_REF_EXE or use --image)")
        return 1 if args.require else 0
    h = parse_header(HEADER.read_text(encoding="utf-8"))
    img = Image(path)
    res = verify(img.read, h, img.find_string, img.code_refs)
    for rule, ok, detail in res:
        print(f"{'PASS' if ok else 'FAIL'}  {rule}   [{detail}]")
    bad = [r for r in res if not r[1]]
    print(f"verify_pins: {len(res) - len(bad)} of {len(res)} rules hold")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
