"""Resolve editor ids (EDIDs) to 'plugin.esm:LOCALID' by sweeping the master files (read-only; the data folder is never written).

    find(data_dir, names)  -> (found {name: ref}, ambiguous {name: [refs]})

The game matches editor ids without regard to case (the rules name `HooverDamIntCheckpoint`, the cell is `HooverDamIntCheckPoint`), so the
match is case-insensitive and the result is keyed by the name the caller asked for. The ref is the plugin that DEFINES the form (a DLC
override of a FalloutNV.esm record resolves to FalloutNV.esm, the same as runtime ids do). Plugins are swept in loadorder.json order; when
two different forms carry one EDID the first in load order wins and the name is also returned in `ambiguous`.
EDID is the first subrecord of every record that has one, so a compressed record is only inflated for its first bytes.
"""
import os
import struct
import sys
import zlib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loadorder  # noqa: E402
from esm import COMPRESSED, EsmFile  # noqa: E402

_HEAD = 4 + 2 + 256  # sig, size, longest EDID the engine accepts


def record_edid(rec):
    """The EDID of a record (cp1252 text) or None. Reads at most the first bytes of the record."""
    raw = bytes(rec._buf[rec._off:rec._off + min(rec._size, _HEAD + 8)])
    if rec.flags & COMPRESSED:
        if len(raw) < 5:
            return None
        raw = zlib.decompressobj().decompress(raw[4:], _HEAD)
    if raw[:4] != b'EDID' or len(raw) < 6:
        return None
    n = struct.unpack_from('<H', raw, 4)[0]
    return raw[6:6 + n].split(b'\0')[0].decode('cp1252')


def sweep(path, wanted_lower):
    """[(edid, ref, own)] for every record of the plugin at `path` whose EDID (lower-cased) is in `wanted_lower`."""
    e = EsmFile.open(path)
    try:
        ref = e.resolver()
        out = []
        for rec in e.records():
            if rec.deleted:
                continue
            ed = record_edid(rec)
            if ed is not None and ed.lower() in wanted_lower:
                out.append((ed, ref(rec.formid), (rec.formid >> 24) >= len(e.masters)))
        return out
    finally:
        e.close()


def find(data_dir, names, plugins=None):
    plugins = plugins or loadorder.load()
    wanted = {n.lower(): n for n in names}
    # a name can differ only by case between two asked names; keep every spelling
    spellings = {}
    for n in names:
        spellings.setdefault(n.lower(), []).append(n)
    refs = {}
    for p in plugins:
        path = os.path.join(data_dir, p)
        if not os.path.isfile(path):
            continue
        for ed, r, _own in sweep(path, wanted):
            lst = refs.setdefault(ed.lower(), [])
            if r.lower() not in [x.lower() for x in lst]:
                lst.append(r)
    found, ambiguous = {}, {}
    for low, lst in refs.items():
        for n in spellings[low]:
            found[n] = lst[0]
            if len(lst) > 1:
                ambiguous[n] = lst
    return found, ambiguous


def default_data_dir(repo):
    """FNVCOOP_GAME_DATA, else the read-only game copy beside the repo (games/fnv-instances/0/Data). None when neither exists."""
    env = os.environ.get('FNVCOOP_GAME_DATA')
    if env:
        return env if os.path.isdir(env) else None
    p = os.path.join(os.path.dirname(os.path.abspath(repo)), 'fnv-instances', '0', 'Data')
    return p if os.path.isdir(p) else None


def mesg_buttons(data_dir, ref):
    """The button texts (ITXT, in order) of the MESG record `ref` ('Plugin.esm:LOCALID'), or None when the plugin or the record is missing.
    Button 0 is the first ITXT. The three Lonesome Road Temple confirm boxes are ['No.', 'Yes.']."""
    plugin, local = ref.rsplit(':', 1)
    path = os.path.join(data_dir, plugin)
    if not os.path.isfile(path):
        return None
    want = int(local, 16)
    e = EsmFile.open(path)
    try:
        for rec in e.records(sigs={'MESG'}):
            if (rec.formid & 0xFFFFFF) == want and (rec.formid >> 24) >= len(e.masters) and not rec.deleted:
                return [d.split(b'\0')[0].decode('cp1252') for s, d in rec.subrecords() if s == 'ITXT']
        return None
    finally:
        e.close()
