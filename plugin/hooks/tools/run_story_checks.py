#!/usr/bin/env python3
"""Python-side checks of the story-capture work (no compiler, no game): python -I plugin/hooks/tools/run_story_checks.py   (prints ALL OK)

  1. story_addrs.h is exactly what tools/gen_story_addrs.py makes from tests/story_prologues_dump.txt;
  2. data/story_cmd_params.json + story_animgroups.h are exactly what tools/gen_story_params.py reads from a reference exe (skipped, with a line saying so,
     when <reference exe> is not on this machine: it is not in git);
  3. every function the op table hooks has a prologue line in the dump, and the dump has no line without an op (plus the one dedicated ForceActiveQuest hook);
  4. the in-game proof checker passes against its scripted fake and catches every injected defect;
  5. W3d: the script commands GetItemCount / GetActorValue call the condition evaluators the party hooks detour (tools/check_cmd_routing.py, twins on a fake image);
  6. twin: a header with one byte changed is reported by check 1 (so a hand edit of the generated file cannot slip through).
"""
import pathlib
import re
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PY = [sys.executable, '-I']


def run(args):
    r = subprocess.run(PY + args, capture_output=True, text=True, cwd=str(ROOT))
    return r.returncode, (r.stdout + r.stderr).strip()


def main():
    bad = []
    rc, out = run([str(HERE / 'gen_story_addrs.py'), '--check'])
    print('%s story_addrs.h matches its generator' % ('ok  ' if rc == 0 else 'FAIL'))
    if rc:
        bad.append('story_addrs.h is stale: run python -I plugin/hooks/tools/gen_story_addrs.py')
    exe = ROOT / 'out' / 'L5' / 're' / 'fnv_reference.exe'
    if exe.exists():
        rc, out = run([str(HERE / 'gen_story_params.py'), '--check'])
        print('%s engine parameter data + animation group names match the exe' % ('ok  ' if rc == 0 else 'FAIL'))
        if rc:
            bad.append('data/story_cmd_params.json or story_animgroups.h is stale: run python -I plugin/hooks/tools/gen_story_params.py')
    else:
        print('skip engine parameter data check: %s is not on this machine' % exe.relative_to(ROOT))
    ops = (ROOT / 'plugin' / 'hooks' / 'ref_ops.cpp').read_text(encoding='utf-8')
    wanted = set(re.findall(r'\{fnvcoop::kOp\w+,\s*"\w+",\s*S_\((Cmd_\w+)\)', ops)) | {'Cmd_ForceActiveQuest'}
    dumped = {ln.split()[0] for ln in (HERE.parent / 'tests' / 'story_prologues_dump.txt').read_text().splitlines() if ln.strip()}
    miss, extra = sorted(wanted - dumped), sorted(dumped - wanted)
    print('%s every hooked command has a prologue line, and every line is used' % ('ok  ' if not miss and not extra else 'FAIL'))
    if miss or extra:
        bad.append('prologue dump vs op table: missing %s, unused %s' % (miss, extra))
    rc, out = run([str(HERE / 'proof_story_capture.py'), '--selftest'])
    print('%s in-game proof checker: selftest + injected defects' % ('ok  ' if rc == 0 and 'ALL OK' in out else 'FAIL'))
    if rc or 'ALL OK' not in out:
        bad.append('proof_story_capture --selftest failed:\n' + out)
    rc, out = run([str(HERE / 'check_cmd_routing.py'), '--self-test'])   # W3d: Player.GetItemCount / GetAV reach the party evaluators (call edges read from the exe, twins on a fake image)
    print('%s script commands GetItemCount / GetActorValue / GetLevel route through the party evaluators (+ twins)' % ('ok  ' if rc == 0 and 'ALL OK' in out else 'FAIL'))
    if rc or 'ALL OK' not in out:
        bad.append('check_cmd_routing failed: ' + out)
    # twin: one flipped byte in a copy of the header must be noticed by the generator's comparison
    hdr = (HERE.parent / 'story_addrs.h').read_text(encoding='utf-8')
    flipped = hdr.replace('0x55, 0x8b, 0xec', '0x55, 0x8b, 0xed', 1)
    import importlib.util
    spec = importlib.util.spec_from_file_location('gen_story_addrs', str(HERE / 'gen_story_addrs.py'))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    caught = flipped.replace('\r\n', '\n') != mod.render()
    print('%s twin: a hand-edited header is not equal to the generated one' % ('ok  ' if caught else 'FAIL'))
    if not caught:
        bad.append('twin survived')
    if bad:
        print('\n'.join(bad))
        return 1
    print('ALL OK')
    return 0


if __name__ == '__main__':
    sys.exit(main())
