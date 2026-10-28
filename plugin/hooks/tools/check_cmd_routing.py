#!/usr/bin/env python3
"""The script-command forms of GetItemCount and GetActorValue reach the party reducer through the condition evaluators (W3d item 2, W3b request 7).

W3b read h_GetItemCountCmd / h_GetAVCmd (engine_hooks.cpp), saw that they answer from PlayerFactsScope and never call the reducer, and asked for the commands to be
routed through reduceCondition. They already are, one level down: the engine's own command handler calls the condition evaluator with the calling reference as the
first operand, and the evaluator is what nvhooks detours (Cond_GetItemCount 0x59D8E0, Cond_GetActorValue 0x59C4F0, entry detours, so a direct `call` from a command
handler hits them too):

    Cmd_GetItemCount 0x5C6A70:  ... mov edx,[ebp+0x10] ; push edx ; call 0x59D8E0          ([ebp+0x10] = thisObj of the command)
    Cmd_GetActorValue 0x5BD8A0: ... mov edx,[ebp+0x10] ; push edx ; call 0x59C4F0
    Cmd_GetLevel      0x5C84D0: the same shape into 0x5A00C0 (docs/engine-addresses.md section 12)

So `Player.GetItemCount X` / `Player.GetAV InventoryWeight` in a quest script evaluates the party (condEval) and reduces under the quest of the capture scope that
surrounds the script. This check reads the reference image and proves the call edge and the operand, and that engine_addrs.h detours the evaluator at exactly that
target. It is skipped (with a line saying so) when <reference exe> is not on this machine.

    python -I plugin/hooks/tools/check_cmd_routing.py              check the image
    python -I plugin/hooks/tools/check_cmd_routing.py --self-test  the check on a fake image, and the twins (each defect must be reported)
"""
import pathlib
import re
import struct
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
EXE = ROOT / 'out' / 'L5' / 're' / 'fnv_reference.exe'
ADDRS = HERE.parent / 'engine_addrs.h'

# (command handler, command VA, evaluator it must call, evaluator name in engine_addrs.h)
EDGES = [
    ('Cmd_GetItemCount', 0x5C6A70, 0x59D8E0, 'Cond_GetItemCount'),
    ('Cmd_GetActorValue', 0x5BD8A0, 0x59C4F0, 'Cond_GetActorValue'),
    ('Cmd_GetLevel', 0x5C84D0, 0x5A00C0, 'Cond_GetLevel'),
]
# `mov reg,[ebp+0x10]` ; `push reg` right before the call: the command's thisObj is the evaluator's first operand
THIS_OBJ = (b'\x8b\x55\x10\x52', b'\x8b\x4d\x10\x51', b'\x8b\x45\x10\x50')


class Image:
    """Reads bytes at virtual addresses of a PE file (no third-party module)."""
    def __init__(self, data):
        self.d = data
        pe = struct.unpack_from('<I', data, 0x3C)[0]
        nsec = struct.unpack_from('<H', data, pe + 6)[0]
        opt = struct.unpack_from('<H', data, pe + 20)[0]
        self.base = struct.unpack_from('<I', data, pe + 24 + 28)[0]
        self.secs = []
        off = pe + 24 + opt
        for i in range(nsec):
            _, vsize, va, rsize, raw = struct.unpack_from('<8sIIII', data, off + 40 * i)[0:5]
            self.secs.append((va, max(vsize, rsize), raw))

    def read(self, va, n):
        rva = va - self.base
        for sva, size, raw in self.secs:
            if sva <= rva < sva + size:
                return self.d[raw + rva - sva: raw + rva - sva + n]
        return b''


class FakeImage:
    def __init__(self, blobs):
        self.blobs = blobs   # {va: bytes}

    def read(self, va, n):
        for a, b in self.blobs.items():
            if a <= va < a + len(b):
                return b[va - a: va - a + n]
        return b''


def body(img, va):
    """The bytes of an ebp-framed function up to and including its epilogue (`pop ebp ; ret`)."""
    raw = img.read(va, 0x80)
    m = re.search(rb'\x5d(\xc3|\xc2..)', raw)
    return raw[:m.end()] if m else raw


def calls(code, va):
    """[(address of the call, target, the 4 bytes before it)] of every `call rel32` in `code`."""
    out = []
    for i in range(len(code) - 4):
        if code[i] == 0xE8:
            rel = struct.unpack_from('<i', code, i + 1)[0]
            out.append((va + i, (va + i + 5 + rel) & 0xFFFFFFFF, code[max(0, i - 4):i]))
    return out


def detoured_targets():
    text = ADDRS.read_text(encoding='utf-8')
    return {m.group(1): int(m.group(2), 16) for m in re.finditer(r'HookSpec (\w+) = \{"[^"]+", (0x[0-9a-f]+)u', text)}


def check(img, edges=EDGES, targets=None):
    problems = []
    targets = detoured_targets() if targets is None else targets
    for cmd, va, ev, evname in edges:
        code = body(img, va)
        hits = [c for c in calls(code, va) if c[1] == ev]
        if not hits:
            problems.append('%s (0x%X) does not call the evaluator 0x%X' % (cmd, va, ev))
        elif not any(c[2] in THIS_OBJ for c in hits):
            problems.append('%s (0x%X): the call to 0x%X is not made with the command\'s thisObj ([ebp+0x10]) as the first operand' % (cmd, va, ev))
        if targets.get(evname) != ev:
            problems.append('engine_addrs.h detours %s at %s, the command handler calls 0x%X' % (evname, targets.get(evname) and hex(targets[evname]), ev))
    return problems


def fake_image():
    """Two command handlers shaped like the real ones: ... 8B 55 10 52 E8 <rel> ... 5D C3."""
    def handler(va, ev, reg=b'\x8b\x55\x10\x52'):
        pre = b'\x55\x8b\xec\x51'
        call_at = va + len(pre) + len(reg)
        rel = struct.pack('<i', ev - (call_at + 5))
        return pre + reg + b'\xe8' + rel + b'\x83\xc4\x10\x8b\xe5\x5d\xc3'
    return {0x5C6A70: handler(0x5C6A70, 0x59D8E0), 0x5BD8A0: handler(0x5BD8A0, 0x59C4F0), 0x5C84D0: handler(0x5C84D0, 0x5A00C0)}, handler


def self_test():
    blobs, handler = fake_image()
    targets = {e[3]: e[2] for e in EDGES}
    bad = []
    if check(FakeImage(blobs), targets=targets):
        bad.append('the clean fake image was reported: %s' % check(FakeImage(blobs), targets=targets))
    twins = {
        'the handler calls another function': {**blobs, 0x5C6A70: handler(0x5C6A70, 0x59D8E1)},
        'the call is made with another operand': {**blobs, 0x5BD8A0: handler(0x5BD8A0, 0x59C4F0, reg=b'\x8b\x55\x0c\x52')},
        'the handler is gone': {k: v for k, v in blobs.items() if k != 0x5C84D0},
    }
    for name, b in twins.items():
        if not check(FakeImage(b), targets=targets):
            bad.append('twin NOT caught: ' + name)
    moved = dict(targets, Cond_GetItemCount=0x59D8E1)
    if not check(FakeImage(blobs), targets=moved):
        bad.append('twin NOT caught: engine_addrs.h detours the evaluator at another address')
    for b in bad:
        print('FAIL ', b)
    print('check_cmd_routing self-test: ' + ('FAILED' if bad else 'ALL OK (%d twins caught)' % (len(twins) + 1)))
    return 1 if bad else 0


def main(argv):
    rc = self_test() if '--self-test' in argv else 0
    if not EXE.exists():
        print('skip: %s is not on this machine (the check reads the reference image)' % EXE.relative_to(ROOT))
        return rc
    problems = check(Image(EXE.read_bytes()))
    for p in problems:
        print('FAIL ', p)
    print('script commands route through the party evaluators: ' + ('FAILED' if problems else 'ALL OK'))
    return 1 if problems or rc else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
