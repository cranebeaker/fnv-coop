"""Generates plugin/hooks/data/guest_local_run.json: the quest scripts that run locally on EVERY machine (GuestLocalRun allow-list).

Sources: questlines/systems/types.json quests with policy "per-player" (achievement trackers, timers, casino, perk effects,
rumble, DLC setup, chargen helpers, radio) plus EXTRA_QUESTS (the DLC setup MQ00 quests, classified shared in types.json but whose
scripts only initialise per-machine state). Quests without a script of their own (radio) are listed under "no_script" for the record.

Run: python -I plugin/hooks/tools/gen_guest_local_run.py
"""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
EXTRA_QUESTS = ['NVDLC01MQ00', 'NVDLC02MQ00', 'NVDLC03MQ00', 'NVDLC04MQ00', 'NVDLC05MQ00', 'VDLCPackQuest',
                'VStreetLighting', 'NVDLC02RainQuest', 'NVDLC02RadioMessage', 'NVDLC03RadioBroadcast']
# Why each EXTRA quest is per-machine (the two-game diff, out/L10/twogame/live/snapdiff.txt). Written into the json "reason" field.
# Deliberately NOT here: VERNovac01 / VERStalkersSouth01 (random-encounter spawners: a guest copy would spawn a second set of creatures;
# their iSpawnMonsters / iCount are host-private counters that must not count as shared divergence, see internal lane notes round 12)
# and every quest whose state is story.
REASONS = {
    'VStreetLighting': 'street lights follow each machine own game clock (LightsOn is a function of GameHour), a pure presentation toggle',
    'NVDLC02RainQuest': 'Honest Hearts rain: pDepth tracks the local player height/camera every frame, presentation only',
    'NVDLC02RadioMessage': 'Happy Trails radio broadcast: playback state of the local radio (running flag), stub script, never gates story',
    'VDLCPackQuest': 'DLC pack item grants run once per machine on each save (bClassicItemsGiven, nGiven)',
    'NVDLC03RadioBroadcast': 'Old World Blues radio broadcast: playback state of the local radio (running flag), like NVDLC02RadioMessage; '
                             'seen in the two-copy fight test 2026-10-21 (running on the host only)',
}
# Quests out/questdata (the vanilla dump) does not have, with their script: TTW's pre-order pack menu asks each player which packs to take
# and starts the vanilla VPreorder* quests, whose scripts add the items to the local player. Per machine like VDLCPackQuest: a guest got no
# menu and no pack items at all (TTW, 2026-10-27).
PREORDER_WHY = 'pre-order pack choice and item grant for each machine own player (TTW stash pack menu -> VPreorder* item scripts)'
EXPLICIT = {
    'TTWPreorderMenuQuest': ('TaleOfTwoWastelands.esm:001BFA', PREORDER_WHY),
    'VPreorderCaravan': ('FalloutNV.esm:172A25', PREORDER_WHY),
    'VPreorderClassic': ('FalloutNV.esm:172A27', PREORDER_WHY),
    'VPreorderMercenary': ('FalloutNV.esm:172A24', PREORDER_WHY),
    'VPreorderTribal': ('FalloutNV.esm:172A26', PREORDER_WHY),
}
# Faction ranks that per-machine DLC setup scripts give the local player; their membership is per-machine by definition, so the
# shared snapshot compare leaves them out (plugin/snapshot/tools/snaplib.py load_local_allowlist reads "factionRanks").
FACTION_RANKS = {
    'HonestHearts.esm:00B42E': 'NVDLC02PlayerFaction: the Honest Hearts setup (NVDLC02MQ00, GuestLocalRun) ranks each machine\'s own '
                               'player; seen in the two-copy fight test 2026-10-21 (rank 1 on the guest only)',
    'LonesomeRoad.esm:003E41': 'NVDLC04PlayerFaction: the Lonesome Road setup (NVDLC04MQ00, GuestLocalRun) ranks each machine\'s own '
                               'player; seen in the two-copy fight test 2026-10-21 (rank 1 on the guest only)',
    'OldWorldBlues.esm:016154': 'NVDLC03PlayerFaction: the Old World Blues setup (NVDLC03MQ00, GuestLocalRun) ranks each machine\'s '
                                'own player; seen in the two-copy fight test 2026-10-21 (rank 1 on the host only)',
}


def main():
    types = json.load(open(os.path.join(ROOT, 'questlines', 'systems', 'types.json'), encoding='utf-8'))
    quests = {q['edid']: q for q in json.load(open(os.path.join(ROOT, 'out', 'questdata', 'quests.json'), encoding='utf-8'))['quests']}
    want = [q['edid'] for q in types['quests'] if q['policy'] == 'per-player']
    for e in EXTRA_QUESTS:
        if e not in want:
            want.append(e)
    for e in EXPLICIT:
        if e not in want:
            want.append(e)
    scripts, no_script = [], []
    for edid in sorted(want):
        q = quests.get(edid)
        ref = q.get('script') if q else None
        if not ref and edid in EXPLICIT:
            ref = EXPLICIT[edid][0]
        if not ref:
            no_script.append(edid)
            continue
        entry = {'quest': edid, 'script': ref}
        if edid in REASONS:
            entry['reason'] = REASONS[edid]
        elif edid in EXPLICIT:
            entry['reason'] = EXPLICIT[edid][1]
        scripts.append(entry)
    out = {'schema': 1, 'note': 'GuestLocalRun allow-list: quest scripts that keep running on guests. script = "plugin:hexLocalId".',
           'scripts': scripts, 'no_script': no_script,
           'factionRanks': sorted(FACTION_RANKS), 'factionReasons': FACTION_RANKS}
    path = os.path.join(ROOT, 'plugin', 'hooks', 'data', 'guest_local_run.json')
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(out, f, indent=1)
    print('wrote', path, len(scripts), 'scripts', len(no_script), 'without script')


main()
