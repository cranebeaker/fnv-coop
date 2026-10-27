"""Data loading + shared helpers for the coverage lane (L9). stdlib only."""
import gzip
import json
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
# FNVCOOP_QUESTDATA points at another run_all.py output, e.g. one generated from a TTW Data folder
QDIR = os.environ.get('FNVCOOP_QUESTDATA') or os.path.join(ROOT, 'out', 'questdata')

TIERS = ['T0', 'T1', 'T2', 'T3', 'T4']
# Ordered coverage ladder. 'failing' is not a rung; it is a flag on top of the highest rung proven.
LADDER = ['unknown', 'spec', 'sim-pass', 'game-pass', 'playtested']


def load_quests(qdir=QDIR, with_infos=True):
    """Return dict(quests, by_ref, by_edid, risk, infos, qdir)."""
    qj = json.load(open(os.path.join(qdir, 'quests.json'), encoding='utf-8'))
    rj = json.load(open(os.path.join(qdir, 'risk.json'), encoding='utf-8'))['quests']
    quests = qj['quests']
    by_ref = {q['ref']: q for q in quests}
    by_edid = {q['edid']: q for q in quests}
    infos = {}
    if with_infos:
        p = os.path.join(qdir, 'infos.jsonl.gz')
        if os.path.exists(p):
            with gzip.open(p, 'rt', encoding='utf-8') as f:
                for line in f:
                    d = json.loads(line)
                    infos.setdefault(d.get('quest'), []).append(d)
    return dict(quests=quests, by_ref=by_ref, by_edid=by_edid, risk=rj, infos=infos, qdir=qdir)


def tier_of(data, q):
    r = data['risk'].get(q['ref'])
    return r['tier'] if r else 'T0'


def kind_of(data, q):
    r = data['risk'].get(q['ref'])
    return r['kind'] if r else 'container'


def story_quests(data):
    return [q for q in data['quests'] if kind_of(data, q) == 'story']


def scenario_exists(data, edid):
    return os.path.exists(os.path.join(data['qdir'], 'scenarios', edid + '.json'))


# ---------------------------------------------------------------- questlines
# Hand-curated membership. Everything else falls back to plugin / prefix rules (see questline_of).
MAIN_PROLOGUE = ['VCG00', 'VCG01', 'VCG02', 'VCG03', 'PressDemoQuest', 'CGTutorial']
MAIN_SHARED = ['VMQ01', 'VMQTops', 'VDialogueMrHouse', 'VDialogueCaesar', 'VDialogueLucky38Entrance',
               'VDialogueVegasNorth', 'VMQ02', 'VMQ03a', 'VMQ03b', 'VMQ04', 'VMQCaesarFail', 'VMQNCRFail',
               'VMS29a', 'VMS18', 'VMS46', 'VMS40']
MAIN_PATHS = {
    'Main: House': ['VMQHouse', 'VMQHouse1', 'VMQHouse2', 'VMQHouse3', 'VMQHouse4', 'VMQHouse5', 'VMQHouse6',
                    'VMQHouse7', 'VMQHouse8', 'VMQHouseFail', 'VMQHouseLockdown', 'VHDHouseBattle'],
    'Main: Yes Man': ['VMQYesMan01', 'VMQYesMan01a', 'VMQYesMan02', 'VMQYesMan03', 'VMQYesManFailsafe',
                      'VHDIndependentBattle'],
    'Main: NCR': ['VMQ05', 'vHDBattleController', 'VMQ06', 'VEFR00NCRMixed2LimitedAccess',
                  'VEFR01NCRGood2EmergencyRadio', 'VEFR02NCRBad2MostWanted'],
    'Main: Legion': ['VHDLegionBattle', 'VDeadSea', 'VEFR00LegionMix2CaesarsHire',
                     'VEFR01LegionGood2CaesarsFavor', 'VEFR02LegionBad2CaesarsFoe'],
}
# Ending quest(s) = the quest whose completion finishes that path (seeds for critical-path closure)
ENDINGS = {
    'House': ['VHDHouseBattle'],
    'Yes Man': ['VHDIndependentBattle'],
    'NCR': ['vHDBattleController'],
    'Legion': ['VHDLegionBattle'],
}
ENTRY = ['VCG00']

DLC_NAMES = {
    'DeadMoney.esm': 'DLC: Dead Money',
    'HonestHearts.esm': 'DLC: Honest Hearts',
    'OldWorldBlues.esm': 'DLC: Old World Blues',
    'LonesomeRoad.esm': 'DLC: Lonesome Road',
    'GunRunnersArsenal.esm': "DLC: Gun Runners' Arsenal",
    # Tale of Two Wastelands: the Fallout 3 DLCs and TTW's own quests
    'Anchorage.esm': 'TTW: Operation Anchorage',
    'ThePitt.esm': 'TTW: The Pitt',
    'BrokenSteel.esm': 'TTW: Broken Steel',
    'PointLookout.esm': 'TTW: Point Lookout',
    'Zeta.esm': 'TTW: Mothership Zeta',
    'TaleOfTwoWastelands.esm': 'TTW: Tale of Two Wastelands',
}


def fallout3_questline(edid):
    """Fallout3.esm quests under TTW: main quest MQ01-MQ12, the Vault 101 prologue CG00-CG04, side quests MS##."""
    if re.match(r'^MQ\d\d$', edid):
        return 'TTW: Fallout 3 main quest'
    if re.match(r'^CG0\d$', edid):
        return 'TTW: Fallout 3 prologue (Vault 101)'
    if re.match(r'^MS\d', edid):
        return 'TTW: Fallout 3 side quests'
    return 'TTW: Fallout 3 misc'

# Side-quest location buckets (names from quests.json; best-effort, unlisted VMS## fall into "Mojave (other)")
LOCATIONS = {
    'Goodsprings': ['VMS01', 'VMS16', 'VMS16b', 'VMS03', 'VMS02', 'VMS06', 'VMS05', 'VMS04',
                    'VMS13', 'VMS12', 'VMS09', 'VMS09a'],
    'Primm': ['nVPrimmDeputyConv', 'nVPrimmVanceGun', 'VMS08', 'VES04FactReactPrimmArea01PowderGangers'],
    'Novac': ['VMS11', 'VMS10', 'VMS15', 'VES01aVictorInNovac', 'VMS19', 'VMS23'],
    'Nipton': ['VNipton', 'vNiptonVulpes'],
    'Boulder City': ['VMS20', 'VMS21', 'VMS21a', 'VES01bVictorInBoulder', 'VES01VictorGSMyBodyguard'],
    'Freeside / Strip': ['VMS22', 'VMS24', 'VMS25', 'VMS26', 'VMS29', 'VMS30', 'VMS31', 'VMS32', 'VMS33',
                         'VMS35', 'VMS36', 'VMS37', 'VMS38', 'VMS41', 'VMS42', 'VMS43', 'VMS43a', 'VMS44',
                         'VMS45', 'VMS47', 'VMSTopsTalentPool', 'vfreeformlucky38', 'vCasinoCompsTheTops',
                         'VFreeformTheStreet01', 'vCCEyeforaneye', 'vCCLeftMyHeart', 'VFSAtomicPimp',
                         'VFSDebtCollector', 'VFSHighTimes', 'VNellisInfestation', 'VNellisJack'],
    'Nellis / Camp Golf / NCR': ['VFreeformCampGolf', 'NellisSunshineBoogie', 'vFreeformForlornHope02',
                                 'vFreeformForlornHope03', 'VMS50', 'VMS51', 'VMS52', 'VMS53', 'VMS54',
                                 'VMS55', 'VMS55a', 'VMS55b', 'VMS56', 'VMS57', 'VMSRSGlobalCommBreakdown'],
}
PROLOGUE_EXTRA = ['VCG02', 'VCG03']


def questline_of(data, q):
    e, plugin = q['edid'], q['plugin']
    if plugin in DLC_NAMES:
        return DLC_NAMES[plugin]
    if plugin == 'Fallout3.esm':
        return fallout3_questline(e)
    for path, lst in MAIN_PATHS.items():
        if e in lst:
            return path
    if e in MAIN_PROLOGUE:
        return 'Main: Prologue'
    if e in MAIN_SHARED:
        return 'Main: Shared (Ring-a-Ding-Ding and Act 2)'
    for loc, lst in LOCATIONS.items():
        if e in lst:
            return 'Side: ' + loc
    if re.match(r'^VMS\d', e):
        return 'Side: Mojave (other)'
    if e.startswith(('VES', 'VT74', 'VVault', 'VNelson', 'vDialogue', 'vMojave')):
        return 'Side: Misc / events'
    return 'Systems & misc'


def is_main(line):
    return line.startswith('Main:')
