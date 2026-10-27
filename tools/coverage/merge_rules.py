"""Merge questlines/*/rules.patch.json into docs/quest-policies/rules.json.

Pipeline: base (rules.py-generated, hand-curated) -> adapters turn each lane's patch (15 different shapes) into
canonical ops -> ops applied in lane-priority order (specialist lanes win over general ones) -> every rule is
sanitised against SCHEMA (the vocabulary L4's RulesEngine implements). Anything outside it (new kinds, new params,
new param values, new reductions) is NOT put in rules.json; it goes to rules.pending.json and proposals.md.

Canonical ops (dict): {lane, op, quest, ...}
  set_entry   quest, entry{tier,questline,rules[]}, mode 'replace'|'new'   (new on an existing quest = add_rules)
  add_rule    quest, rule{kind,params}           (kind present -> params merged)
  replace_params quest, kind, params             (wholesale; missing kind -> add)
  merge_params   quest, kind, params             (lists unioned, funcs merged)
  set_param      quest, kind, param, value
  remove_kind    quest, kind
"""
import copy
import json
import os
import re

ALLOWED_KINDS = ['PartyTether', 'PartyWarp', 'SetPlayerControls', 'Barrier', 'PerPlayerScene', 'SlideshowEnd',
                 'CompanionOwner', 'SaveSuppress', 'FastTravelLock', 'PartyAwareCond', 'QuestItemDup',
                 'RewardAll', 'RemoveItemPerOwner', 'ShareKarmaRep', 'InitiatorDialogue', 'ExcludeFromGate']
REDUCTIONS = {'initiator', 'any-party-member', 'any', 'min-over-party', 'entry:all-in,exit:any-in'}
# kind -> {param: ('list'|'int'|set-of-values)}.  Anything else is pending.
SCHEMA = {
    'PartyTether': {'cells': 'list', 'gather': {'auto-warp', 'wait'}},
    'PartyWarp': {'markers': 'list', 'mode': {'slot-offset'}, 'absent': {'hold', 'warp'}},
    'SetPlayerControls': {'scope': {'all'}, 'release': {'host-effect'}, 'timeout_s': 'int', 'masks': 'list'},
    'Barrier': {'step': {'all-players-pass'}},
    'SlideshowEnd': {'run': {'host-once'}, 'lock': {'all'}, 'release': {'together'}},
    'CompanionOwner': {'owner': {'recruiter'}},
    'FastTravelLock': {'scope': {'party'}},
    'QuestItemDup': {'recipient': {'all'}},
    'PartyAwareCond': {'funcs': 'funcs'},
}
for _k in ALLOWED_KINDS:
    SCHEMA.setdefault(_k, {})
# lane priority: later wins a conflict. Specialist lanes beat the broad ones.
LANE_PRIORITY = ['systems', 'act2', 'leftovers', 'towns', 'factions', 'strip', 'prologue', 'deadmoney',
                 'honesthearts', 'owb', 'lonesomeroad', 'legion', 'ncr', 'yesman', 'house', 'ttw']
# lanes for quests that only exist in a Tale of Two Wastelands install; build_rules.merge skips them for plain New Vegas data
TTW_LANES = ('ttw',)
FUNC_CANON = {}


def _canon_func(name):
    low = name.lower()
    if low in FUNC_CANON:
        return FUNC_CANON[low]
    return name


def learn_funcs(names):
    for n in names:
        if n != n.lower():                 # a mixed-case spelling is the canonical one
            FUNC_CANON.setdefault(n.lower(), n)


def L(slug):
    return json.load(open(os.path.join(ROOT, 'questlines', slug, 'rules.patch.json'), encoding='utf-8'))


ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))


# ---------------------------------------------------------------- adapters
def _r(rule):
    return {'kind': rule['kind'], 'params': copy.deepcopy(rule.get('params') or {})}


def _rules(lst):
    return [_r(x) for x in lst or [] if isinstance(x, dict) and 'kind' in x]


def _mk(lane, op, quest, **kw):
    d = {'lane': lane, 'op': op, 'quest': quest}
    d.update(kw)
    return d


def _entry(lane, quest, e, mode):
    return _mk(lane, 'set_entry', quest, mode=mode, entry={
        'tier': e.get('tier'), 'questline': e.get('questline'), 'rules': _rules(e.get('rules'))})


def adapt_act2(d, lane):
    ops = []
    for q, e in d['quests'].items():
        for r in e.get('add', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
        for m in e.get('modify', []):
            if m.get('add'):
                ops.append(_mk(lane, 'merge_params', q, kind=m['kind'], params={m['param']: m['add']}))
    return ops


def adapt_deadmoney(d, lane):
    ops = []
    for q, e in d['quests'].items():
        for r in e.get('replace', []):
            ops.append(_mk(lane, 'replace_params', q, kind=r['kind'], params=r['params']))
        for r in e.get('add', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
        for k in e.get('remove', []):
            ops.append(_mk(lane, 'remove_kind', q, kind=k if isinstance(k, str) else k['kind']))
    for q, e in d.get('new_quests', {}).items():
        ops.append(_entry(lane, q, e, 'new'))
    return ops


def adapt_actionlist(d, lane):             # factions, leftovers, ttw
    ops = []
    for q, e in d['quests'].items():
        if e.get('action') == 'add-rules':
            ops += [_mk(lane, 'add_rule', q, rule=_r(r)) for r in e['rules']]
        elif e.get('action') == 'replace-quest-entry':   # curated full rule set: drops the derived rules it does not repeat
            ops.append(_entry(lane, q, e, 'replace'))
        else:
            ops.append(_entry(lane, q, e, 'new'))
    return ops


def adapt_honesthearts(d, lane):
    ops = []
    for p in d['patches']:
        if p['op'] == 'replace_params':
            ops.append(_mk(lane, 'replace_params', p['quest'], kind=p['kind'], params=p['params']))
        elif p['op'] == 'add_rule':
            ops.append(_mk(lane, 'add_rule', p['quest'], rule=_r(p)))
        elif p['op'] == 'add_quest':
            ops.append(_entry(lane, p['quest'], p, 'new'))
    return ops


def adapt_house(d, lane):
    ops = []
    for o in d['ops']:
        k, q = o['op'], o['quest']
        if k == 'replace_rules':
            ops.append(_mk(lane, 'set_entry', q, mode='replace', entry={'tier': None, 'questline': None,
                                                                      'rules': _rules(o['rules'])}))
        elif k == 'remove_rule':
            ops.append(_mk(lane, 'remove_kind', q, kind=o['kind']))
        elif k == 'set_param':
            ops.append(_mk(lane, 'set_param', q, kind=o['kind'], param=o['param'], value=o['value']))
        elif k == 'add_rule':
            ops.append(_mk(lane, 'add_rule', q, rule=_r(o['rule'])))
        elif k == 'add_quest':
            ops.append(_entry(lane, q, o['entry'], 'new'))
    return ops


def adapt_legion(d, lane):
    ops = []
    for q, e in d['quests'].items():
        if e['op'] == 'add':
            ops.append(_entry(lane, q, {'tier': e.get('tier'), 'questline': e.get('questline'),
                                        'rules': e.get('add_rules')}, 'new'))
            for r in e.get('update_rules', []):
                ops.append(_mk(lane, 'replace_params', q, kind=r['kind'], params=r.get('params', {})))
            continue
        for r in e.get('add_rules', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
        for r in e.get('update_rules', []):
            ops.append(_mk(lane, 'replace_params', q, kind=r['kind'], params=r.get('params', {})))
        for r in e.get('remove_rules', []):
            ops.append(_mk(lane, 'remove_kind', q, kind=r if isinstance(r, str) else r['kind']))
    return ops


def adapt_lonesomeroad(d, lane):
    return [_entry(lane, q, e, 'replace' if e['action'] == 'replace' else 'new') for q, e in d['quests'].items()]


def adapt_ncr(d, lane):
    ops = []
    for o in d['ops']:
        k, q = o['op'], o['quest']
        if k == 'add_rule':
            ops.append(_mk(lane, 'add_rule', q, rule=_r(o['rule'])))
        elif k == 'update_params':
            ops.append(_mk(lane, 'merge_params', q, kind=o['kind'], params=o['params_merge']))
        elif k == 'remove_rule':
            ops.append(_mk(lane, 'remove_kind', q, kind=o['kind']))
        elif k == 'add_quest':
            ops.append(_entry(lane, q, o['entry'], 'new'))
    return ops


def adapt_owb(d, lane):
    ops = []
    for q, e in d['quests'].items():
        ops.append(_entry(lane, q, e, 'replace' if e['op'] == 'replace' else 'new'))
    return ops


def adapt_prologue(d, lane):
    ops = []
    for o in d['ops']:
        k, q = o['op'], o['quest']
        if k == 'merge_params':
            ops.append(_mk(lane, 'merge_params', q, kind=o['kind'], params=o['params']))
        elif k == 'add_rule':
            ops.append(_mk(lane, 'add_rule', q, rule=_r(o)))
        elif k == 'add_quest':
            ops.append(_entry(lane, q, {'tier': o.get('tier'), 'questline': o.get('questline'),
                                        'rules': [o] if o.get('kind') else []}, 'new'))
    return ops


def adapt_strip(d, lane):
    ops = []
    for q, e in d['amend'].items():
        for k in e.get('remove', []):
            ops.append(_mk(lane, 'remove_kind', q, kind=k))
        for k, p in (e.get('set_params') or {}).items():
            ops.append(_mk(lane, 'replace_params', q, kind=k, params=p))
        for r in e.get('add', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
    for q, e in d['new_quests'].items():
        ops.append(_entry(lane, q, e, 'new'))
    return ops


def adapt_systems(d, lane):
    ops = []
    for q, e in d['add'].items():
        ops.append(_entry(lane, q, e, 'new'))
    for q, e in d['modify'].items():
        for k in e.get('remove_kinds', []):
            ops.append(_mk(lane, 'remove_kind', q, kind=k))
        for r in e.get('add_rules', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
        for k, p in (e.get('update_params') or {}).items():
            ops.append(_mk(lane, 'merge_params', q, kind=k, params=p))
    return ops


def adapt_towns(d, lane):
    ops = []
    for q, e in d['quests'].items():
        for r in e.get('add', []):
            ops.append(_mk(lane, 'add_rule', q, rule=_r(r)))
        for k, p in (e.get('set_params') or {}).items():
            ops.append(_mk(lane, 'merge_params', q, kind=k, params=p))
    for q, e in d.get('new_quests', {}).items():
        ops.append(_entry(lane, q, {'tier': e.get('tier'), 'questline': e.get('questline'), 'rules': e['add']}, 'new'))
    return ops


def adapt_yesman(d, lane):
    ops = []
    for p in d['patch']:
        k, q = p['op'], p['quest']
        if k == 'replace':
            ops.append(_mk(lane, 'replace_params', q, kind=p['kind'], params=p['params']))
        elif k == 'remove':
            ops.append(_mk(lane, 'remove_kind', q, kind=p['kind']))
        elif k == 'add':
            if p.get('questline'):
                ops.append(_entry(lane, q, {'tier': p.get('tier'), 'questline': p.get('questline'),
                                            'rules': [p]}, 'new'))
            else:
                ops.append(_mk(lane, 'add_rule', q, rule=_r(p)))
    for x in d.get('cross_lane', []):
        prm = x.get('params', {})
        for kind in x['add']:
            kp = prm.get(kind, {}) if prm and all(k in ALLOWED_KINDS for k in prm) else prm
            ops.append(_mk(lane, 'add_rule', x['quest'], rule={'kind': kind, 'params': kp}))
    return ops


ADAPTERS = {'act2': adapt_act2, 'deadmoney': adapt_deadmoney, 'factions': adapt_actionlist,
            'leftovers': adapt_actionlist, 'honesthearts': adapt_honesthearts, 'house': adapt_house,
            'legion': adapt_legion, 'lonesomeroad': adapt_lonesomeroad, 'ncr': adapt_ncr, 'owb': adapt_owb,
            'prologue': adapt_prologue, 'strip': adapt_strip, 'systems': adapt_systems, 'towns': adapt_towns,
            'yesman': adapt_yesman, 'ttw': adapt_actionlist}


def collect_ops(lanes=None, loader=L):
    """Ops of `lanes` (default: every New Vegas lane; TTW_LANES only when named, see build_rules.merge)."""
    ops = []
    lanes = lanes or [s for s in ADAPTERS if s not in TTW_LANES]
    for lane in sorted(lanes, key=lambda s: LANE_PRIORITY.index(s) if s in LANE_PRIORITY else -1):
        try:
            d = loader(lane)
        except FileNotFoundError:
            continue
        ops += ADAPTERS[lane](d, lane)
    return ops


# ---------------------------------------------------------------- sanitise
def sanitize_params(kind, params, pend, ctx):
    """-> clean params; everything outside SCHEMA appended to pend (dicts)."""
    sch = SCHEMA[kind]
    out = {}
    for p, v in (params or {}).items():
        spec = sch.get(p)
        if spec is None or p in ('why', 'note'):
            if p not in ('why', 'note'):
                pend.append(dict(ctx, kind=kind, param=p, value=v, reason='param not in applied vocabulary'))
            continue
        if spec == 'list':
            if not isinstance(v, list):
                pend.append(dict(ctx, kind=kind, param=p, value=v, reason='expected list'))
                continue
            out[p] = [x for x in dict.fromkeys(v)]
        elif spec == 'int':
            if isinstance(v, int) and not isinstance(v, bool):
                out[p] = v
            else:
                pend.append(dict(ctx, kind=kind, param=p, value=v, reason='expected int'))
        elif spec == 'funcs':
            funcs = {}
            for fn, red in (v or {}).items():
                if red in REDUCTIONS:
                    key = _canon_func(fn)
                    funcs[key] = red
                else:
                    pend.append(dict(ctx, kind=kind, param='funcs.' + fn, value=red,
                                     reason='reduction not implemented by RulesEngine'))
            out[p] = funcs
        else:
            if v in spec:
                out[p] = v
            else:
                pend.append(dict(ctx, kind=kind, param=p, value=v, reason='value not in applied vocabulary'))
    return out


def _merge(kind, cur, new):
    out = copy.deepcopy(cur)
    for p, v in new.items():
        if isinstance(v, list):
            out[p] = list(dict.fromkeys(list(out.get(p, [])) + v))
        elif isinstance(v, dict):
            d = {k.lower(): (k, x) for k, x in out.get(p, {}).items()}
            for fk, red in v.items():
                d[fk.lower()] = (_canon_func(fk), red)
            out[p] = {k: x for k, x in d.values()}
        else:
            out[p] = v
    return out


class Merger:
    def __init__(self, base, lookup_meta=None):
        self.entries = copy.deepcopy(base['quests'])
        self.pending = []            # parameter/kind level items outside the vocabulary
        self.pending_rules = {}      # quest -> [rule] with pending kinds (kept verbatim)
        self.conflicts = []
        self.prov = {}               # (quest, kind) -> [lanes]
        self.lookup_meta = lookup_meta or (lambda q: (None, None))
        self.touch = {}              # (quest, kind) -> {lane: op}

    def _entry(self, q, tier=None, questline=None):
        if q not in self.entries:
            t, ql = self.lookup_meta(q)
            self.entries[q] = {'tier': tier or t, 'questline': questline or ql, 'curated': True, 'rules': []}
        e = self.entries[q]
        e['tier'] = e.get('tier') or tier
        e['questline'] = e.get('questline') or questline
        return e

    def _find(self, e, kind):
        for r in e['rules']:
            if r['kind'] == kind:
                return r
        return None

    def _note(self, q, kind, lane, op, params=None):
        self.prov.setdefault((q, kind), [])
        if lane not in self.prov[(q, kind)]:
            self.prov[(q, kind)].append(lane)
        t = self.touch.setdefault((q, kind), {})
        t.setdefault(lane, []).append(op)

    def _pending_kind(self, q, rule, lane):
        self.pending_rules.setdefault(q, []).append({'kind': rule['kind'], 'params': rule.get('params', {}),
                                                     'lane': lane})

    def add_rule(self, q, rule, lane, mode='merge'):
        if rule['kind'] not in ALLOWED_KINDS:
            return self._pending_kind(q, rule, lane)
        e = self._entry(q)
        ctx = {'quest': q, 'lane': lane}
        clean = sanitize_params(rule['kind'], rule.get('params'), self.pending, ctx)
        cur = self._find(e, rule['kind'])
        if cur is None:
            e['rules'].append({'kind': rule['kind'], 'params': clean, 'origin': 'curated'})
        elif mode == 'replace':
            cur['params'] = clean
            cur['origin'] = 'curated'
        else:
            cur['params'] = _merge(rule['kind'], cur['params'], clean)
        self._note(q, rule['kind'], lane, 'add' if mode == 'merge' else 'replace')

    def apply(self, op):
        k, q, lane = op['op'], op['quest'], op['lane']
        if k == 'add_rule':
            self.add_rule(q, op['rule'], lane)
        elif k == 'replace_params':
            self.add_rule(q, {'kind': op['kind'], 'params': op['params']}, lane, mode='replace')
        elif k == 'merge_params':
            self.add_rule(q, {'kind': op['kind'], 'params': op['params']}, lane)
        elif k == 'set_param':
            e = self._entry(q)
            cur = self._find(e, op['kind'])
            if cur is None:
                self.add_rule(q, {'kind': op['kind'], 'params': {op['param']: op['value']}}, lane)
            else:
                cur['params'] = _merge(op['kind'], {k2: v for k2, v in cur['params'].items() if k2 != op['param']},
                                       sanitize_params(op['kind'], {op['param']: op['value']}, self.pending,
                                                       {'quest': q, 'lane': lane}))
                if op['param'] == 'funcs':
                    cur['params']['funcs'] = sanitize_params(op['kind'], {'funcs': op['value']}, [], {})['funcs']
                self._note(q, op['kind'], lane, 'replace')
        elif k == 'remove_kind':
            e = self.entries.get(q)
            if e:
                e['rules'] = [r for r in e['rules'] if r['kind'] != op['kind']]
            self._note(q, op['kind'], lane, 'remove')
        elif k == 'set_entry':
            ent = op['entry']
            if op['mode'] == 'replace' and q in self.entries:
                e = self._entry(q, ent.get('tier'), ent.get('questline'))
                gone = [r['kind'] for r in e['rules']]
                e['rules'] = []
                for kind in gone:
                    self._note(q, kind, lane, 'replace')
                for r in ent['rules']:
                    self.add_rule(q, r, lane, mode='replace')
            else:
                self._entry(q, ent.get('tier'), ent.get('questline'))
                for r in ent['rules']:
                    self.add_rule(q, r, lane)
            self.entries[q]['curated'] = True

    def detect_conflicts(self):
        for (q, kind), by_lane in sorted(self.touch.items()):
            kinds_of_op = {l: set(ops) for l, ops in by_lane.items()}
            if len(by_lane) > 1 and any('remove' in o for o in kinds_of_op.values()) and \
                    any(('add' in o or 'replace' in o) for o in kinds_of_op.values()):
                self.conflicts.append({'quest': q, 'kind': kind, 'lanes': {l: sorted(o) for l, o in kinds_of_op.items()},
                                       'resolved_by': 'lane priority (later lane wins)'})
            elif sum('replace' in o for o in kinds_of_op.values()) > 1:
                self.conflicts.append({'quest': q, 'kind': kind, 'lanes': {l: sorted(o) for l, o in kinds_of_op.items()},
                                       'resolved_by': 'lane priority (later lane wins)'})


def run_merge(base, ops, lookup_meta=None):
    m = Merger(base, lookup_meta)
    for op in ops:
        m.apply(op)
    m.detect_conflicts()
    return m
