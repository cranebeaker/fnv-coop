#!/usr/bin/env python3
"""Find the point-of-no-return message boxes (VEndGameWarningMsg*) in the ESMs and write
plugin/dialogue/data/consent_messages.txt for nvui::Bridge::consentFor.

Line format:  <plugin>:<HEXLOCALID> <edid> confirm=<button index|*> decline=<button index> [mode=vote|actor] [kind=info] buttons=<a|b|...>

Owner decision D3 (who decides an irreversible fork). Besides the five end-game boxes (mode unanimous: a point of no return, every player
must say Ready, 120 s window), FORKS below lists the other decision points of the campaign:
  vote   story fork: the party votes, the origin counts as yes, the majority wins, the HOST breaks a tie
  actor  personal action (Caesar's surgery, a trooper's treatment, picking the nuke target): the player who acts decides alone
  kind=info rows are INFOs of a conversation (Joshua or Daniel, Elijah's offer): the pick is held until the party voted.
Every form is looked up in the ESMs; a missing one aborts the generator.
The runtime has no EDIDs for MESG, so the bridge matches on (plugin, local id).
A box is a consent box when its EDID starts with VEndGameWarningMsg (the four ending-branch confirms: Indy/Yes Man, Legion,
NCR, House), found in the scripts of YesManDead, FortLucius, FortCaesar, VHDColonelMoore and MrHouse.
Confirm = the button that continues (text "Yes"/"Continue"/...); decline = the other one.
Usage: python -I plugin/dialogue/tools/gen_consent_messages.py [--out file] [--data DIR]
"""
import argparse, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'tools', 'questdata'))
import esm  # noqa: E402
import extract  # noqa: E402

DEFAULT_DATA = extract.DEFAULT_DATA      # found by install discovery, so regional/PCR installs work too


def find_data():
    """The Data folder with the ESMs: FNV_ESM_DIR (the same setting tools/questsweep/refcheck.py reads), the discovered install, else the
    test instances next to the repo (fnv-instances/<n>/Data)."""
    env = os.environ.get('FNV_ESM_DIR')
    if env and os.path.exists(os.path.join(env, 'FalloutNV.esm')):
        return env
    if os.path.exists(os.path.join(DEFAULT_DATA, 'FalloutNV.esm')):
        return DEFAULT_DATA
    inst = os.path.join(os.path.dirname(ROOT), 'fnv-instances')
    if os.path.isdir(inst):
        for n in sorted(os.listdir(inst)):
            d = os.path.join(inst, n, 'Data')
            if os.path.exists(os.path.join(d, 'FalloutNV.esm')):
                return d
    return DEFAULT_DATA
PLUGINS = ['FalloutNV.esm', 'DeadMoney.esm', 'HonestHearts.esm', 'OldWorldBlues.esm', 'LonesomeRoad.esm']
PREFIX = 'VEndGameWarningMsg'
YES = re.compile(r"(?i)^(yes|continue|proceed|begin|go|ok|okay|i am sure|sure|start|let'?s do this)")
NO = re.compile(r"(?i)(not ready|not yet|^no|cancel|wait|never mind)")


def scan(data_dir):
    rows = []
    for name in PLUGINS:
        path = os.path.join(data_dir, name)
        if not os.path.exists(path):
            continue
        f = esm.EsmFile.open(path)
        try:
            for r in f.records({'MESG'}):
                subs = r.subrecords()
                edid = next((d.split(b'\0')[0].decode('cp1252') for s, d in subs if s == 'EDID'), '')
                if not edid.startswith(PREFIX):
                    continue
                buttons = [d.split(b'\0')[0].decode('cp1252') for s, d in subs if s == 'ITXT']
                decline = next((i for i, b in enumerate(buttons) if NO.search(b)), -1)
                if decline >= 0:
                    confirm = next((i for i in range(len(buttons)) if i != decline), 0)
                else:
                    confirm = next((i for i, b in enumerate(buttons) if YES.match(b.strip())), 0)
                    decline = next((i for i in range(len(buttons)) if i != confirm), -1)
                n_masters = len(f.masters)
                local = r.formid & 0xFFFFFF
                rows.append((name, local, edid, confirm, decline, buttons, (r.formid >> 24) == n_masters))
        finally:
            f.close()
    return sorted(rows, key=lambda x: (x[0], x[1]))


# (plugin, kind, edid or INFO local id, mode, confirm, decline, why)
FORKS = [
    ('FalloutNV.esm', 'mesg', 'FortCaesarComaMsg', 'actor', '*', 0, 'legion: Caesar surgery, the activating player decides and their Medicine counts'),
    ('LonesomeRoad.esm', 'mesg', 'NVDLC04TempleLaunchMSG', 'actor', '*', 0, 'lonesomeroad: the player at the terminal picks the nuke target'),
    ('LonesomeRoad.esm', 'mesg', 'NVDLC04TempleCaesarConfirmMSG', 'vote', 1, 0, 'lonesomeroad: launch at Caesar'),
    ('LonesomeRoad.esm', 'mesg', 'NVDLC04TempleNCRConfirmMSG', 'vote', 1, 0, 'lonesomeroad: launch at the NCR'),
    ('LonesomeRoad.esm', 'mesg', 'NVDLC04TempleNukeAllConfirmMSG', 'vote', 1, 0, 'lonesomeroad: launch at both'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper01Msg', 'actor', '*', 0, 'strip VMS30: treating the first trooper'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper01LowSkillMsg', 'actor', '*', 0, 'strip VMS30'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper02Msg', 'actor', '*', 0, 'strip VMS30: the amputation'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper02LowSkillMsg', 'actor', '*', 0, 'strip VMS30'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper03Msg', 'actor', '*', 0, 'strip VMS30'),
    ('FalloutNV.esm', 'mesg', 'VMS30InjuredTrooper03LowSkillMsg', 'actor', '*', 0, 'strip VMS30'),
    ('FalloutNV.esm', 'mesg', 'FortHowitzerMsg', 'vote', 1, 0, 'strip VMS32: replace the howitzer firing mechanism'),
    ('FalloutNV.esm', 'mesg', 'VMS42MicroclineMsg', 'vote', '*', 0, 'strip VMS42: blow up the caves (and the vault)'),
    ('HonestHearts.esm', 'info', 0x010718, 'vote', 0, -1, 'honesthearts: Daniel, the Joshua path (MQ04)'),
    ('HonestHearts.esm', 'info', 0x010723, 'vote', 0, -1, 'honesthearts: Daniel, the Joshua path (MQ04)'),
    ('HonestHearts.esm', 'info', 0x0093AA, 'vote', 0, -1, 'honesthearts: Daniel, his own path (MQ05)'),
    ('DeadMoney.esm', 'info', 0x011C68, 'vote', 0, -1, 'deadmoney: Elijah teams up with the courier'),
]


def scan_forks(data_dir):
    """Resolve FORKS against the ESMs. Returns rows (plugin, local, edid, confirm, decline, buttons, mode, kind, why) and a list of problems."""
    mesg, infos, problems, rows = {}, set(), [], []
    for name in sorted({f[0] for f in FORKS}):
        path = os.path.join(data_dir, name)
        if not os.path.exists(path):
            problems.append('missing plugin ' + name)
            continue
        f = esm.EsmFile.open(path)
        try:
            for r in f.records({'MESG'}):
                subs = r.subrecords()
                edid = next((d.split(b'\0')[0].decode('cp1252') for s, d in subs if s == 'EDID'), '')
                buttons = [d.split(b'\0')[0].decode('cp1252') for s, d in subs if s == 'ITXT']
                mesg[(name, edid)] = (r.formid & 0xFFFFFF, buttons)
            wanted = {fk[2] for fk in FORKS if fk[0] == name and fk[1] == 'info'}
            if wanted:
                for r in f.records({'INFO'}):
                    if (r.formid & 0xFFFFFF) in wanted:
                        infos.add((name, r.formid & 0xFFFFFF))
        finally:
            f.close()
    for plugin, kind, key, mode, confirm, decline, why in FORKS:
        if kind == 'mesg':
            hit = mesg.get((plugin, key))
            if not hit:
                problems.append('%s: no MESG %s' % (plugin, key))
                continue
            local, buttons = hit
            if confirm != '*' and confirm >= len(buttons):
                problems.append('%s: confirm=%s but only %d buttons' % (key, confirm, len(buttons)))
            rows.append((plugin, local, key, confirm, decline, buttons, mode, 'mesg', why))
        else:
            if (plugin, key) not in infos:
                problems.append('%s: no INFO %06X' % (plugin, key))
                continue
            rows.append((plugin, key, 'INFO_%06X' % key, confirm, decline, [], mode, 'info', why))
    return rows, problems


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', default=os.path.join(ROOT, 'plugin', 'dialogue', 'data', 'consent_messages.txt'))
    ap.add_argument('--data', default=None)
    a = ap.parse_args()
    a.data = a.data or find_data()
    rows = scan(a.data)
    if not rows:
        print('no VEndGameWarningMsg* records found (data dir %s)' % a.data)
        return 1
    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    forks, problems = scan_forks(a.data)
    for pr in problems:
        print('PROBLEM', pr)
    if problems:
        return 1
    with open(a.out, 'w', newline='\n') as f:
        f.write('# generated by tools/gen_consent_messages.py from the ESMs; do not edit\n')
        f.write('# D3: unanimous (default) = point of no return, vote = story fork (host breaks ties), actor = the acting player decides; kind=info = fork INFO\n')
        for name, local, edid, c, d, buttons, own in rows:
            f.write('%s:%06X %s confirm=%d decline=%d buttons=%s\n' % (name, local, edid, c, d, '|'.join(buttons)))
        for name, local, edid, c, d, buttons, mode, kind, why in forks:
            extra = ' mode=%s%s' % (mode, ' kind=info' if kind == 'info' else '')
            f.write('%s:%06X %s confirm=%s decline=%d%s buttons=%s\n' % (name, local, edid, c, d, extra, '|'.join(buttons)))
    print('wrote %d consent messages to %s' % (len(rows), a.out))
    for r in rows:
        print('  %s:%06X %s buttons=%s%s' % (r[0], r[1], r[2], r[5], '' if r[6] else '  (override of a master record)'))
    return 0


if __name__ == '__main__':
    sys.exit(main())
