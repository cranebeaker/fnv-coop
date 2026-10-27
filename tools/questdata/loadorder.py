"""Plugin:LOCALID <-> runtime FormID using loadorder.json (single shared load-order file)."""
import json
import os

# FNVCOOP_LOADORDER picks another order file, e.g. loadorder.ttw.json for Tale of Two Wastelands
PATH = os.environ.get('FNVCOOP_LOADORDER') or os.path.join(os.path.dirname(os.path.abspath(__file__)), 'loadorder.json')


def load(path=PATH):
    with open(path, encoding='utf-8') as f:
        return json.load(f)['plugins']


def runtime_formid(ref, plugins=None):
    """'DeadMoney.esm:00ABCD' -> int 0x0100ABCD. Raises KeyError for a plugin not in the load order."""
    plugins = plugins or load()
    name, local = ref.rsplit(':', 1)
    idx = {p.lower(): i for i, p in enumerate(plugins)}[name.lower()]
    return (idx << 24) | int(local, 16)


def console_id(ref, plugins=None):
    """Hex FormID string as the console wants it: 6 digits for the base game, 8 for DLC (e.g. 104C1C, 0100ABCD)."""
    v = runtime_formid(ref, plugins)
    return f'{v:06X}' if v < 0x1000000 else f'{v:08X}'


class LoadOrderMismatch(Exception):
    pass


def verify(observed, plugins=None):
    """Fail fast when the game's real plugin order differs from loadorder.json (runtime FormIDs would be wrong).

    `observed` = plugin file names in load-index order as reported by the bridge `plugins` method (may be a prefix of
    the shared list when fewer DLCs are loaded). Raises LoadOrderMismatch; returns True otherwise."""
    plugins = plugins or load()
    obs = [str(p).lower() for p in observed]
    exp = [p.lower() for p in plugins]
    if not obs:
        raise LoadOrderMismatch('bridge reported no plugins')
    if len(obs) > len(exp):
        raise LoadOrderMismatch('game loads %d plugins, loadorder.json lists %d (extra: %s)'
                                % (len(obs), len(exp), ', '.join(observed[len(exp):])))
    bad = [(i, o, e) for i, (o, e) in enumerate(zip(obs, exp)) if o != e]
    if bad:
        raise LoadOrderMismatch('load order differs from tools/questdata/loadorder.json: '
                                + '; '.join(f'index {i:02X} is {o} but loadorder.json says {e}' for i, o, e in bad))
    return True


if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        verify(sys.argv[1].split(','))
        print('load order OK')
    else:
        for i, p in enumerate(load()):
            print(f'{i:02X} {p}')
