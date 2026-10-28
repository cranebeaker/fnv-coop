#!/usr/bin/env python3
"""Form id resolver for rule data: "never invent FormIDs" made checkable.

A rule may name a form as Plugin.esm:HEX or as an EditorID. `Refs.resolve(text)` answers (ref, signature, editor id) from out/questdata (forms, refnames, quests, scripts,
globals, factions, INFOs) and falls back to a scan of the game's .esm files (a read-only copy; set FNV_ESM_DIR, default <repo>/../fnv-instances/0/Data) for the ids the
extraction does not carry (message boxes, a few perks). Nothing here writes anywhere and nothing touches the game install.

    python -I tools/questsweep/refcheck.py Plugin.esm:HEX | EDID ...
"""
from __future__ import annotations

import gzip
import json
import os
import re
import sys
from typing import Dict, List, Optional, Tuple

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from questsweep import common  # noqa: E402

REF_RX = re.compile(r"^([A-Za-z0-9_.]+\.es[mp]):([0-9A-Fa-f]{1,8})$")
DLC_PREFIX = {"NVDLC01": "DeadMoney.esm", "NVDLC02": "HonestHearts.esm", "NVDLC03": "OldWorldBlues.esm", "NVDLC04": "LonesomeRoad.esm", "NVDLC05": "GunRunnersArsenal.esm"}


def norm_ref(ref: str) -> str:
    m = REF_RX.match(ref)
    if not m:
        raise ValueError("not a Plugin.esm:HEX reference: %r" % ref)
    return "%s:%06X" % (m.group(1).lower(), int(m.group(2), 16) & 0xFFFFFF)


def esm_dir() -> str:
    return os.environ.get("FNV_ESM_DIR") or os.path.join(os.path.dirname(common.REPO), "fnv-instances", "0", "Data")


class Refs:
    def __init__(self) -> None:
        self.by_ref: Dict[str, Tuple[str, str]] = {}        # normalised ref -> (signature, edid)
        self.by_edid: Dict[str, List[str]] = {}             # lower edid -> [ref, ...]
        self.display: Dict[str, str] = {}                   # normalised ref -> the ref as the data spells it (Plugin.esm:HEX)
        self._esm: Dict[Tuple[str, int], Tuple[str, str]] = {}
        qd = common.QD

        def add(ref: str, sig: str, edid: str) -> None:
            n = norm_ref(ref)
            if n not in self.by_ref or (not self.by_ref[n][1] and edid):
                self.by_ref[n] = (sig, edid)
            self.display.setdefault(n, "%s:%06X" % (ref.rsplit(":", 1)[0], int(ref.rsplit(":", 1)[1], 16) & 0xFFFFFF))
            if edid:
                self.by_edid.setdefault(edid.lower(), [])
                if n not in self.by_edid[edid.lower()]:
                    self.by_edid[edid.lower()].append(n)

        for path in ("forms.json.gz", "refnames.json.gz"):
            d = common.load_json(os.path.join(qd, path))
            for ref, v in d.items():
                add(ref, v.get("sig", "?"), v.get("edid", ""))
        q = common.load_json(os.path.join(qd, "quests.json"))
        for e in q["quests"]:
            add(e["ref"], "QUST", e["edid"])
        for key in ("globals", "factions", "reputations"):
            for e in q.get(key, []):
                add(e["ref"], {"globals": "GLOB", "factions": "FACT", "reputations": "REPU"}[key], e["edid"])
        for s in common.load_json(os.path.join(qd, "scripts.json.gz")):
            add(s["ref"], "SCPT", s["edid"])
        for r in common.load_jsonl_gz(os.path.join(qd, "infos.jsonl.gz")):
            add(r["ref"], "INFO", "")

    PLUGINS = ("FalloutNV.esm", "DeadMoney.esm", "HonestHearts.esm", "OldWorldBlues.esm", "LonesomeRoad.esm")

    def _scan_all(self) -> Dict[Tuple[str, int], Tuple[str, str]]:
        """{(owner plugin lower, local id): (signature, edid)} over the game's masters. An override of a master's record is filed under the master (the id is the master's)."""
        if self._esm:
            return self._esm
        sys.path.insert(0, os.path.join(common.REPO, "tools"))
        from questdata import esm  # noqa: E402
        out: Dict[Tuple[str, int], Tuple[str, str]] = {}
        for plugin in self.PLUGINS:
            path = os.path.join(esm_dir(), plugin)
            if not os.path.exists(path):
                continue
            f = esm.EsmFile.open(path)
            try:
                masters = f.masters
                for rec in f.records():
                    idx = rec.formid >> 24
                    owner = masters[idx] if idx < len(masters) else plugin
                    edid = ""
                    try:
                        for sig, data in rec.subrecords():
                            if sig == "EDID":
                                edid = data.split(b"\x00")[0].decode("latin1")
                                break
                    except esm.ParseError:
                        pass
                    k = (owner.lower(), rec.formid & 0xFFFFFF)
                    if k not in out or (edid and not out[k][1]):
                        out[k] = (rec.sig, edid)
            finally:
                f.close()
        self._esm = out
        return out

    def resolve(self, text: str, allow_esm: bool = True) -> Optional[Tuple[str, str, str]]:
        """(Plugin.esm:HEX, signature, editor id) or None."""
        m = REF_RX.match(text)
        if m:
            n = norm_ref(text)
            if n in self.by_ref:
                sig, edid = self.by_ref[n]
                return self.display[n], sig, edid
            if allow_esm:
                hit = self._scan_all().get((m.group(1).lower(), int(m.group(2), 16) & 0xFFFFFF))
                if hit:
                    return "%s:%06X" % (m.group(1), int(m.group(2), 16) & 0xFFFFFF), hit[0], hit[1]
            return None
        lst = self.by_edid.get(text.lower())
        if lst:
            n = lst[0]
            return self.display[n], self.by_ref[n][0], self.by_ref[n][1]
        if allow_esm:
            low = text.lower()
            hint = DLC_PREFIX.get(text[:7], "").lower()
            best = None
            for (owner, local), (sig, edid) in self._scan_all().items():
                if edid.lower() == low and (best is None or owner == hint):
                    plug = next((p for p in self.PLUGINS if p.lower() == owner), owner)
                    best = ("%s:%06X" % (plug, local), sig, edid)
            return best
        return None

    def ref_of(self, edid: str) -> str:
        r = self.resolve(edid)
        if not r:
            raise KeyError("EditorID %r does not resolve in out/questdata or the game data" % edid)
        return r[0]


def main(argv: List[str]) -> int:
    if not common.have_questdata():
        print("SKIP: " + common.questdata_missing_reason())
        return 0
    refs = Refs()
    rc = 0
    for a in argv:
        r = refs.resolve(a)
        print("%s -> %s" % (a, ("%s %s %s" % r) if r else "UNRESOLVED"))
        rc |= 0 if r else 1
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
