"""Merge the questline rule patches into docs/quest-policies/rules.json (+ pending, provenance, report, proposals).

  python -I tools/coverage/build_rules.py            # writes under docs/quest-policies/
Outputs
  rules.json            applied rules: only the vocabulary L4's RulesEngine implements, validated
  rules.pending.json    everything outside it (new kinds, new params/values/reductions, tethers without a cell)
  rules.provenance.json {quest: {kind: [lanes]}}
  merge_report.md       counts, conflicts, cross-lane moves, dropped tethers
  proposals.md          the new kinds / params / engine changes the lanes asked for, grouped, for L4
"""
import collections
import datetime
import gzip
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import model                       # noqa: E402
import policies as pol             # noqa: E402
import merge_rules as M            # noqa: E402
import rules_final as F            # noqa: E402

PROPOSAL_KEYS = {
    'new_kinds': 'kind', 'proposed_kinds': 'kind', 'kinds': 'kind', 'proposed': 'param',
    'param_extensions': 'param', 'new_params': 'param', 'proposed_params': 'param', 'new_param_values': 'param',
    'engine_proposals': 'engine', 'engine_changes': 'engine', 'effect_proposals': 'engine',
    'capabilities_needed': 'engine', 'proposals': None, 'funcs_vocabulary_extension': 'param',
    'owner_questions': 'decision', 'open_owner_questions': 'decision', 'open_decisions': 'decision',
}


def base_rules(data):
    pols = pol.build_policies(data)
    return {'schema': 1, 'quests': {p['edid']: {'tier': p['tier'], 'questline': p['questline'],
                                               'curated': bool(p['curated']), 'rules': p['machine_rules']}
                                    for p in pols}}


def lookup_meta_factory(data):
    def f(q):
        qq = data['by_edid'].get(q)
        if not qq:
            return None, None
        return model.tier_of(data, qq), model.questline_of(data, qq)
    return f


def _learn(loader):
    names = collections.Counter()

    def walk(x):
        if isinstance(x, dict):
            for k, v in x.items():
                if k == 'funcs' and isinstance(v, dict):
                    names.update(v.keys())
                walk(v)
        elif isinstance(x, list):
            for i in x:
                walk(i)
    for s in M.ADAPTERS:
        try:
            walk(loader(s))
        except FileNotFoundError:
            pass
    best = {}
    for n, c in names.items():                 # most frequent spelling wins (ties: mixed case over lowercase)
        k = n.lower()
        if k not in best or (c, n != n.lower()) > (names[best[k]], best[k] != best[k].lower()):
            best[k] = n
    M.FUNC_CANON.clear()
    M.FUNC_CANON.update(best)


def merge(data, resolver, loader=M.L, base=None):
    """-> (rules_json, merger, final_report, errors)"""
    base = base or base_rules(data)
    _learn(loader)
    for e in base['quests'].values():                     # normalise func casing in the base too
        for r in e['rules']:
            if r['kind'] == 'PartyAwareCond':
                r['params']['funcs'] = {M._canon_func(k): v for k, v in r['params'].get('funcs', {}).items()}
    ops = M.collect_ops(loader=loader)
    m = M.run_merge(base, ops, lookup_meta_factory(data))
    rep = F.finalize(m, data, resolver)
    for q, e in m.entries.items():
        if not e.get('tier') or not e.get('questline'):
            t, ql = lookup_meta_factory(data)(q)
            e['tier'] = e.get('tier') or t
            e['questline'] = e.get('questline') or ql
    quests = {q: {'tier': e['tier'], 'questline': e['questline'], 'curated': bool(e.get('curated', True)),
                  'rules': e['rules']} for q, e in sorted(m.entries.items())}
    rules = {'schema': 1, 'quests': quests}
    return rules, m, rep, F.validate(rules, resolver, data), len(ops)


def _txt(x, n=700):
    s = x if isinstance(x, str) else json.dumps(x, ensure_ascii=False)
    s = re.sub(r'\s+', ' ', s)
    return s[:n] + ('...' if len(s) > n else '')


def extract_proposals(loader=M.L):
    """-> list of {name, category, lane, text, raw}"""
    out = []
    for lane in sorted(M.ADAPTERS):
        try:
            d = loader(lane)
        except FileNotFoundError:
            continue

        def add(cat, item, key):
            if isinstance(item, dict):
                name = item.get('kind') or item.get('name') or item.get('type') or item.get('param') or \
                    item.get('id') or item.get('topic') or key
                if item.get('kind') and item.get('param'):
                    name = f"{item['kind']}.{item['param']}"
                text = item.get('summary') or item.get('justification') or item.get('why') or \
                    item.get('what') or item.get('text') or item.get('question') or ''
                out.append({'name': str(name), 'category': cat, 'lane': lane, 'text': _txt(text),
                            'raw': _txt({k: v for k, v in item.items() if k not in ('why', 'justification',
                                                                                    'summary')}, 500)})
            elif isinstance(item, str):
                out.append({'name': (key or 'decision') if cat != 'decision' else item[:60], 'category': cat,
                            'lane': lane, 'text': _txt(item), 'raw': ''})
        for key, cat in PROPOSAL_KEYS.items():
            v = d.get(key)
            if v is None:
                continue
            if key == 'proposals':
                if isinstance(v, list):
                    for it in v:
                        add('kind' if str(it.get('kind', '')).find('param') < 0 and it.get('kind') else 'param',
                            it, key)
                elif isinstance(v, dict):
                    for sub, items in v.items():
                        c = {'new_kinds': 'kind', 'kinds': 'kind', 'new_param_values': 'param',
                             'open_decisions': 'decision', 'engine_changes': 'engine',
                             'kind_uses': None}.get(sub, 'engine')
                        if c and isinstance(items, list):
                            for it in items:
                                add(c, it, sub)
                continue
            if isinstance(v, list):
                for it in v:
                    add(cat, it, key)
            elif isinstance(v, dict):
                for k2, it in v.items():
                    if k2 == 'doc':
                        continue
                    if isinstance(it, str):
                        out.append({'name': k2, 'category': cat, 'lane': lane, 'text': _txt(it), 'raw': ''})
                    else:
                        add(cat, dict(it, name=k2) if isinstance(it, dict) else it, k2)
        for q, rules in []:
            pass
    return out


def render_proposals(props, pending_rules, pending_params, rep):
    L = ['# Proposals for L4 (not in rules.json until implemented)', '',
         'Generated by `tools/coverage/build_rules.py` from the `proposals`/`new_kinds`/`param_extensions`/... sections of '
         'every `questlines/*/rules.patch.json`. Nothing here is in `docs/quest-policies/rules.json`; the full-fidelity rules '
         'that need them are in `rules.pending.json` and are marked pending. Applied vocabulary: see `tools/coverage/merge_rules.py` '
         '(`SCHEMA`, `REDUCTIONS`).', '']
    kinds_used = collections.Counter(r['kind'] for rs in pending_rules.values() for r in rs)
    L += ['## Pending rule kinds in use', '', '| Kind | rules | quests |', '|---|---|---|']
    for k, n in sorted(kinds_used.items(), key=lambda kv: -kv[1]):
        qs = sorted(q for q, rs in pending_rules.items() if any(r['kind'] == k for r in rs))
        L.append(f"| {k} | {n} | {', '.join(qs[:8])}{'...' if len(qs) > 8 else ''} |")
    L += ['', '## Pending parameters / values / reductions (counts of rules affected)', '',
          '| Kind.param | rules | example value | reason |', '|---|---|---|---|']
    agg = collections.defaultdict(list)
    for p in pending_params:
        agg[(p['kind'], p['param'].split('.')[0] if p['param'].startswith('funcs.') else p['param'])].append(p)
    for (k, prm), ps in sorted(agg.items(), key=lambda kv: -len(kv[1])):
        L.append(f"| {k}.{prm} | {len(ps)} | `{_txt(ps[0]['value'], 60)}` | {ps[0]['reason']} |")
    L += ['', f"Tethers dropped for lack of a resolvable CELL (need `PartyTether.anchor/radius`): "
              f"{len(rep['tether_dropped'])} quests; PartyWarp dropped for lack of a marker: {len(rep['warp_dropped'])}.", '']
    for cat, title in (('kind', 'New rule kinds'), ('param', 'Parameter, value and reduction extensions'),
                       ('engine', 'Engine / capture / effect proposals'),
                       ('decision', 'Open owner decisions raised by the lanes')):
        L += [f'## {title}', '']
        by = collections.defaultdict(list)
        for p in props:
            if p['category'] == cat:
                by[p['name']].append(p)
        for name, ps in sorted(by.items()):
            L.append(f"### {name}")
            for p in ps:
                L.append(f"- **{p['lane']}**: {p['text']}" + (f"  \n  `{p['raw']}`" if p['raw'] and cat != 'decision' else ''))
            L.append('')
    L += ['## Named in the integration brief but not found in any patch', '',
          '- `ForkGate`: no lane proposed it; if it exists it was raised elsewhere. `ModalChoice` is in act2; '
          '`PerPlayerVars` in strip; `PerPlayerState` in deadmoney; `LocalQuestScript` in owb and `GuestLocalRun` in systems.', '']
    return '\n'.join(L)


def render_report(rules, m, rep, errs, nops, base):
    kinds = collections.Counter(r['kind'] for e in rules['quests'].values() for r in e['rules'])
    L = ['# rules.json merge report', '', f"Generated {datetime.datetime.now().isoformat(timespec='seconds')}", '',
         f"- base quests: {len(base['quests'])} -> merged: {len(rules['quests'])}; ops applied: {nops}",
         f"- validation errors: {len(errs)}",
         f"- pending parameters/values: {len(m.pending)}; quests with pending-kind rules: {len(m.pending_rules)}",
         f"- tethers: {len(rep['tether_fixed'])} fixed from data, {len(rep['tether_dropped'])} dropped (no CELL); "
         f"PartyWarp dropped (no marker): {len(rep['warp_dropped'])}", '', '## Rule kinds in rules.json', '',
         '| kind | rules |', '|---|---|']
    L += [f'| {k} | {n} |' for k, n in kinds.most_common()]
    L += ['', '## Cross-lane moves (Fort / Colonel Moore / Yes Man confirmation scripts)', '']
    L += [f"- {x['quest']}: {'added' if x.get('added') else 'removed'} {x['kind']}" for x in rep['moved']]
    L += ['', '## Conflicts (same quest+kind touched by several lanes with remove/replace; later lane wins)', '']
    L += [f"- {c['quest']}.{c['kind']}: {c['lanes']}" for c in m.conflicts] or ['- none']
    L += ['', '## Tethers dropped (no resolvable CELL; listed in rules.pending.json)', '', ', '.join(rep['tether_dropped']), '']
    L += ['## Warps dropped (no marker)', '', ', '.join(rep['warp_dropped']) or '-', '']
    return '\n'.join(L)


def load_resolver(data):
    qd = data['qdir']
    rn = json.load(gzip.open(os.path.join(qd, 'refnames.json.gz'), 'rt', encoding='utf-8'))
    fm = json.load(gzip.open(os.path.join(qd, 'forms.json.gz'), 'rt', encoding='utf-8'))
    return F.Resolver(data, rn, fm)


def write_all(data, outdir, loader=M.L, resolver=None):
    resolver = resolver or load_resolver(data)
    base = base_rules(data)
    rules, m, rep, errs, nops = merge(data, resolver, loader, base=json.loads(json.dumps(base)))
    os.makedirs(outdir, exist_ok=True)
    json.dump(rules, open(os.path.join(outdir, 'rules.json'), 'w', encoding='utf-8'), indent=1)
    json.dump({'schema': 1, 'note': 'NOT applied: outside the vocabulary RulesEngine implements (see proposals.md)',
               'pending_rules': m.pending_rules, 'pending_params': m.pending,
               'tethers_dropped': rep['tether_dropped'], 'warps_dropped': rep['warp_dropped']},
              open(os.path.join(outdir, 'rules.pending.json'), 'w', encoding='utf-8'), indent=1)
    prov = {}
    for (q, k), lanes in sorted(m.prov.items()):
        prov.setdefault(q, {})[k] = lanes
    json.dump(prov, open(os.path.join(outdir, 'rules.provenance.json'), 'w', encoding='utf-8'), indent=1)
    props = extract_proposals(loader)
    open(os.path.join(outdir, 'proposals.md'), 'w', encoding='utf-8').write(
        render_proposals(props, m.pending_rules, m.pending, rep))
    open(os.path.join(outdir, 'merge_report.md'), 'w', encoding='utf-8').write(
        render_report(rules, m, rep, errs, nops, base))
    return rules, errs, rep, m


if __name__ == '__main__':
    d = model.load_quests()
    r, e, rep, m = write_all(d, os.path.join(model.ROOT, 'docs', 'quest-policies'))
    print(f"rules.json: {len(r['quests'])} quests, {sum(len(x['rules']) for x in r['quests'].values())} rules, "
          f"{len(e)} validation errors; pending params {len(m.pending)}, pending-kind quests {len(m.pending_rules)}; "
          f"tethers dropped {len(rep['tether_dropped'])}")
    for x in e[:20]:
        print('  ERR', x)
    # The merge only knows the base vocabulary: put the questsweep overlay (wave 1 kinds, the top-level "policy" object) back, so a regeneration keeps them.
    ov = os.path.join(model.ROOT, 'tools', 'questsweep', 'rules_overlay.py')
    if not e and os.path.exists(ov):
        import subprocess
        rc = subprocess.call([sys.executable, '-I', ov, '--apply'])
        if rc:
            print('  ERR the questsweep overlay does not apply (python -I tools/questsweep/rules_overlay.py --apply)')
            e = ['overlay']
    sys.exit(1 if e else 0)
