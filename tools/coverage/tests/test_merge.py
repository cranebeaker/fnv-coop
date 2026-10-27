"""Tests for the questline rules merge (merge_rules / rules_final / build_rules)."""
import copy
import json
import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..'))
import model                      # noqa: E402
import merge_rules as M           # noqa: E402
import rules_final as F           # noqa: E402
import build_rules as B           # noqa: E402

REAL = os.path.exists(os.path.join(model.QDIR, 'risk.json'))


class FakeData:
    """Minimal data dict: quests A..D, cells/markers via a Resolver."""
    @staticmethod
    def make():
        qs = [{'edid': e, 'ref': f'T.esm:{i:06X}', 'plugin': 'T.esm', 'stages': [], 'objectives': [],
               'conditions': [], 'script_data': None} for i, e in enumerate(['QA', 'QB', 'QC', 'VMS35', 'VMQ05',
                                                                            'VHDLegionBattle',
                                                                            'VHDIndependentBattle'], 1)]
        return {'quests': qs, 'by_edid': {q['edid']: q for q in qs}, 'by_ref': {q['ref']: q for q in qs},
                'infos': {}, 'risk': {}}

    @staticmethod
    def resolver(data):
        refn = {'a': {'sig': 'CELL', 'edid': 'CellOne'}, 'b': {'sig': 'REFR', 'edid': 'MarkerOne'},
                'c': {'sig': 'WRLD', 'edid': 'WorldOne'}}
        return F.Resolver(data, refn, {})


def base():
    return {'quests': {
        'QA': {'tier': 'T3', 'questline': 'x', 'curated': True, 'rules': [
            {'kind': 'PartyTether', 'params': {'cells': ['CellOne'], 'gather': 'auto-warp'}, 'origin': 'curated'},
            {'kind': 'PartyAwareCond', 'params': {'funcs': {'GetInCell': 'initiator'}}, 'origin': 'derived'}]}}}


def patches(d):
    return lambda lane: d[lane]


class AdapterAndMergeTests(unittest.TestCase):
    def merge(self, lanes):
        M.FUNC_CANON.clear()
        M.FUNC_CANON.update({'getincell': 'GetInCell', 'getav': 'GetAV'})
        data = FakeData.make()
        ops = []
        for lane in sorted(lanes, key=M.LANE_PRIORITY.index):
            ops += M.ADAPTERS[lane](lanes[lane], lane)
        m = M.run_merge(base(), ops, lambda q: ('T1', 'q'))
        return m, data

    def rules(self, m, q):
        return {r['kind']: r['params'] for r in m.entries[q]['rules']}

    def test_ops_roundtrip(self):
        m, _ = self.merge({
            'towns': {'quests': {'QA': {'add': [{'kind': 'RewardAll', 'params': {}}],
                                        'set_params': {'PartyAwareCond': {'funcs': {'getav': 'initiator'}}}}},
                      'new_quests': {'QB': {'add': [{'kind': 'ExcludeFromGate', 'params': {}}], 'tier': 'T0',
                                            'questline': 'z'}}},
            'house': {'ops': [{'op': 'remove_rule', 'quest': 'QA', 'kind': 'PartyTether'},
                              {'op': 'set_param', 'quest': 'QA', 'kind': 'PartyAwareCond', 'param': 'funcs',
                               'value': {'GetInCell': 'entry:all-in,exit:any-in'}}]}})
        qa = self.rules(m, 'QA')
        self.assertIn('RewardAll', qa)
        self.assertNotIn('PartyTether', qa)
        self.assertEqual(qa['PartyAwareCond']['funcs'], {'GetInCell': 'entry:all-in,exit:any-in'})
        self.assertEqual(self.rules(m, 'QB'), {'ExcludeFromGate': {}})
        self.assertEqual(m.entries['QB']['tier'], 'T0')

    def test_specialist_lane_wins_replace(self):
        m, _ = self.merge({
            'systems': {'add': {}, 'modify': {'QA': {'update_params': {}, 'add_rules': [], 'remove_kinds': []}}},
            'act2': {'quests': {'QA': {'add': [{'kind': 'PartyTether',
                                                'params': {'cells': ['CellOne', 'WorldOne']}}]}}},
            'yesman': {'patch': [{'op': 'replace', 'quest': 'QA', 'kind': 'PartyTether',
                                  'params': {'cells': ['WorldOne'], 'gather': 'wait'}}], 'cross_lane': []}})
        self.assertEqual(self.rules(m, 'QA')['PartyTether'], {'cells': ['WorldOne'], 'gather': 'wait'})

    def test_vocabulary_gate(self):
        m, _ = self.merge({'owb': {'quests': {'QC': {'op': 'add', 'rules': [
            {'kind': 'ReadyCheck', 'params': {'timeout_s': 5}},
            {'kind': 'RewardAll', 'params': {'include': ['AddPerk']}},
            {'kind': 'PartyAwareCond', 'params': {'funcs': {'GetLevel': 'max-over-party', 'GetAV': 'any'}}},
            {'kind': 'CompanionOwner', 'params': {'owner': 'party', 'party_total': 4}}]}}}})
        qc = self.rules(m, 'QC')
        self.assertNotIn('ReadyCheck', qc)
        self.assertEqual([r['kind'] for r in m.pending_rules['QC']], ['ReadyCheck'])
        self.assertEqual(qc['RewardAll'], {})
        self.assertEqual(qc['PartyAwareCond']['funcs'], {'GetAV': 'any'})
        self.assertEqual(qc['CompanionOwner'], {})
        pend = {(p['kind'], p['param']) for p in m.pending}
        self.assertIn(('RewardAll', 'include'), pend)
        self.assertIn(('PartyAwareCond', 'funcs.GetLevel'), pend)
        self.assertIn(('CompanionOwner', 'owner'), pend)

    def test_func_case_dedupe(self):
        m, data = self.merge({'towns': {'quests': {'QA': {'add': [], 'set_params': {
            'PartyAwareCond': {'funcs': {'getincell': 'entry:all-in,exit:any-in'}}}}}, 'new_quests': {}}})
        f = self.rules(m, 'QA')['PartyAwareCond']['funcs']
        self.assertEqual(f, {'GetInCell': 'entry:all-in,exit:any-in'})

    def test_finalize_moves_and_empty_tether(self):
        data = FakeData.make()
        res = FakeData.resolver(data)
        M.FUNC_CANON.clear()
        b = base()
        b['quests']['VMS35'] = {'tier': 'T4', 'questline': 'q', 'curated': True, 'rules': [
            {'kind': 'SetPlayerControls', 'params': {'scope': 'all'}, 'origin': 'derived'},
            {'kind': 'FastTravelLock', 'params': {'scope': 'party'}, 'origin': 'derived'},
            {'kind': 'PartyTether', 'params': {'cells': ['CellOne']}, 'origin': 'derived'}]}
        b['quests']['QC'] = {'tier': 'T2', 'questline': 'q', 'curated': True, 'rules': [
            {'kind': 'PartyTether', 'params': {'cells': []}, 'origin': 'derived'},
            {'kind': 'PartyWarp', 'params': {'markers': ['MarkerOne', 'NoSuchMarker']}, 'origin': 'derived'}]}
        m = M.run_merge(b, [], lambda q: ('T1', 'q'))
        rep = F.finalize(m, data, res)
        self.assertEqual({r['kind'] for r in m.entries['VMS35']['rules']}, {'PartyTether'})
        self.assertEqual({r['kind'] for r in m.entries['VHDIndependentBattle']['rules']},
                         {'SetPlayerControls', 'FastTravelLock', 'SaveSuppress'})
        self.assertEqual(rep['tether_dropped'], ['QC'])
        self.assertEqual(m.entries['QC']['rules'][0]['params']['markers'], ['MarkerOne'])
        self.assertEqual(rep['unresolved_dropped'][0]['ids'], ['NoSuchMarker'])
        errs = F.validate({'schema': 1, 'quests': m.entries}, res, data)
        self.assertEqual(errs, [])

    # ---- validator twins: each corruption must be caught
    def check_validator_catches(self, mutate, needle):
        data = FakeData.make()
        res = FakeData.resolver(data)
        rules = {'schema': 1, 'quests': copy.deepcopy(base()['quests'])}
        self.assertEqual(F.validate(rules, res, data), [])
        mutate(rules['quests'])
        errs = F.validate(rules, res, data)
        self.assertTrue(any(needle in e for e in errs), (needle, errs))

    def test_validator_twins(self):
        self.check_validator_catches(lambda q: q['QA']['rules'].append({'kind': 'ReadyCheck', 'params': {}}),
                                     'unknown kind')
        self.check_validator_catches(lambda q: q['QA']['rules'][0]['params'].update(cells=[]), 'empty cells')
        self.check_validator_catches(lambda q: q['QA']['rules'][0]['params'].update(cells=['Nope']), 'not a resolvable')
        self.check_validator_catches(lambda q: q.update(Ghost={'tier': 'T0', 'questline': 'x', 'rules': []}),
                                     'does not resolve')
        self.check_validator_catches(lambda q: q['QA']['rules'][1]['params']['funcs'].update(X='max-over-party'),
                                     'not implemented')
        self.check_validator_catches(lambda q: q['QA']['rules'][1]['params']['funcs'].update(getincell='any'),
                                     'case-duplicate')
        self.check_validator_catches(lambda q: q['QA']['rules'][0]['params'].update(radius=5), 'not in vocabulary')
        self.check_validator_catches(lambda q: q['QA']['rules'].append(dict(q['QA']['rules'][0])), 'duplicate kind')

    def test_mutation_twin_sanitizer_off(self):
        """A merger that skips sanitising lets a new kind/param into rules: the validator must flag it."""
        real = M.sanitize_params
        M.sanitize_params = lambda kind, params, pend, ctx: dict(params or {})
        try:
            data = FakeData.make()
            res = FakeData.resolver(data)
            m = M.run_merge(base(), M.adapt_towns({'quests': {'QA': {'add': [
                {'kind': 'RewardAll', 'params': {'include': ['AddPerk']}}]}}, 'new_quests': {}}, 'towns'),
                lambda q: ('T1', 'q'))
            errs = F.validate({'schema': 1, 'quests': m.entries}, res, data)
            self.assertTrue(any('not in vocabulary' in e for e in errs))
        finally:
            M.sanitize_params = real

    def test_ttw_lane_only_when_named(self):
        seen = []

        def loader(lane):
            seen.append(lane)
            raise FileNotFoundError(lane)

        M.collect_ops(loader=loader)
        self.assertTrue(seen)
        self.assertFalse(set(seen) & set(M.TTW_LANES), 'plain New Vegas data must not load the TTW lane')
        seen.clear()
        M.collect_ops(lanes=list(M.ADAPTERS), loader=loader)
        self.assertTrue(set(M.TTW_LANES) <= set(seen))
        self.assertEqual(seen[-1], 'ttw', 'the TTW lane is applied last')

    def test_ttw_lane_file_is_actionlist_shaped(self):
        d = M.L('ttw')
        ops = M.adapt_actionlist(d, 'ttw')
        self.assertTrue(ops)
        for o in ops:
            self.assertTrue(all(r['kind'] in M.SCHEMA for r in o['entry']['rules']), o)


@unittest.skipUnless(REAL, 'no real questdata')
class RealMergeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = model.load_quests()
        cls.res = B.load_resolver(cls.data)
        cls.rules, cls.m, cls.rep, cls.errs, cls.nops = B.merge(cls.data, cls.res)

    def test_valid(self):
        self.assertEqual(self.errs, [])
        self.assertGreater(len(self.rules['quests']), 400)

    def test_every_lane_adapts(self):
        lanes = {op['lane'] for op in M.collect_ops()}
        have = {s for s in M.ADAPTERS if os.path.exists(os.path.join(model.ROOT, 'questlines', s, 'rules.patch.json'))}
        self.assertEqual(lanes, have - set(M.TTW_LANES))   # TTW lanes load only for TTW data (test_ttw_lane_only_when_named)

    def test_every_op_quest_resolves(self):
        for op in M.collect_ops():
            self.assertIn(op['quest'], self.data['by_edid'], op)

    def test_no_empty_tethers_and_cross_moves(self):
        for q, e in self.rules['quests'].items():
            for r in e['rules']:
                if r['kind'] == 'PartyTether':
                    self.assertTrue(r['params'].get('cells'), q)
        for q in ('VMS29', 'VMS32', 'VMS35', 'VMS45'):
            kinds = {r['kind'] for r in self.rules['quests'][q]['rules']}
            self.assertFalse(kinds & {'PartyWarp', 'SetPlayerControls', 'FastTravelLock', 'SaveSuppress'}, q)
        for q in ('VHDLegionBattle', 'VMQ05', 'VHDIndependentBattle'):
            kinds = {r['kind'] for r in self.rules['quests'][q]['rules']}
            self.assertTrue({'SetPlayerControls', 'SaveSuppress', 'FastTravelLock'} <= kinds, q)

    def test_pending_kinds_stay_out(self):
        allowed = set(M.ALLOWED_KINDS)
        for e in self.rules['quests'].values():
            for r in e['rules']:
                self.assertIn(r['kind'], allowed)
        self.assertTrue({'ReadyCheck', 'ExclusiveStages', 'ShareMapMarkers', 'LocalQuestScript'}
                        <= {r['kind'] for rs in self.m.pending_rules.values() for r in rs})

    def test_l4_contract_still_holds(self):
        q = self.rules['quests']
        self.assertGreater(len(q), 100)
        vcg01 = {r['kind']: r for r in q['VCG01']['rules']}
        self.assertEqual(vcg01['Barrier']['params']['step'], 'all-players-pass')
        self.assertEqual(len(vcg01['SetPlayerControls']['params']['masks']), 3)
        f = {r['kind']: r for r in q['VMQTops']['rules']}['PartyAwareCond']['params']['funcs']
        self.assertEqual({k.lower(): v for k, v in f.items()}['getincell'], 'entry:all-in,exit:any-in')

    def test_proposals_cover_requested_names(self):
        props = B.extract_proposals()
        txt = B.render_proposals(props, self.m.pending_rules, self.m.pending, self.rep)
        for name in ('ReadyCheck', 'GuestEventRelay', 'ModalChoice', 'ExclusiveStages', 'PerPlayerVars',
                     'PerPlayerState', 'LocalQuestScript', 'GuestLocalRun', 'ShareMapMarkers', 'max-over-party',
                     'sum-over-party', 'PartyTether.anchor', 'stages'):
            self.assertIn(name, txt, name)

    def test_idempotent(self):
        rules2 = B.merge(self.data, self.res)[0]
        self.assertEqual(json.dumps(self.rules, sort_keys=True), json.dumps(rules2, sort_keys=True))


if __name__ == '__main__':
    unittest.main()
