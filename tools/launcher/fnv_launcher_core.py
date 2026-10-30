"""Logic of the co-op launcher, without any window: settings, address checks, writing nvcoop.ini, starting the game.

The window (launcher_gui.py) only shows these functions. Everything here is tested without a game and without a screen.

What the launcher writes into Data\\NVSE\\Plugins\\nvcoop.ini (only these keys, every other line of the file is left alone):

  [coop]    enable=1  role=host|client  listen=0.0.0.0 (host)  host=<address> (join)  port=<n>  maxplayers=<2..4>  token=<n> (join)
            transport=enet                          always written (an older release wrote transport=steam, which the plugin no longer accepts)
            via=ip|steam                            only for the launcher: how the session was set up (the plugin ignores it). Over Steam
                                                    (docs/steam.md) the game is a normal ENet session: the host listens on 127.0.0.1:port and
                                                    a guest joins host=127.0.0.1 port=<the local port of fnvsteam.exe's tunnel>
            handshake=1 baseline=1 dialogue=1       session defaults, see SESSION_DEFAULTS
            name=<player name>                      the plugin ignores keys it does not know; no screen shows the name yet
  [combat]  xpmode=1|0        1 = kill XP is shared by the party, 0 = the player who lands the killing blow gets it (read by plugin/combat)
  [party]   slowmo=0|1        host option: everyone slows down together while a player is in VATS      (NOT READ BY THE PLUGIN YET)
            tether=1|0        story moments wait for the whole party                                   (NOT READ BY THE PLUGIN YET)
  [live]    actors avatars combat spawns   the sync layers (docs/tools/dll-wiring.md), all on unless unticked
  [rebalance] mode=normal|balanced  friendlyfire=0|1   the pick for a NEW campaign (docs/rebalance.md); a saved campaign keeps its own
  [chat]    enabled=1|0  name=<player name>  key=<letter, digit or F1..F24>   text chat (docs/chat.md)

The two [party] keys are the agreed names; the code that reads them lands with the combat and world work (docs/tools/release.md).
"""
import json
import os
import random
import re
import subprocess
import sys
from dataclasses import dataclass, asdict, field
from pathlib import Path
from typing import Callable, Dict, List, Optional, Tuple

_HERE = os.path.dirname(os.path.abspath(__file__))
for _d in (_HERE, os.path.join(os.path.dirname(_HERE), "installer")):
    if _d not in sys.path:
        sys.path.insert(0, _d)
import fnv_find  # noqa: E402
import fnv_ini  # noqa: E402
import fnv_install  # noqa: E402
import fnv_mut  # noqa: E402
import fnv_party_saves  # noqa: E402
import fnv_portcheck  # noqa: E402
import fnv_steam  # noqa: E402

DEFAULT_PORT = fnv_portcheck.DEFAULT_PORT
MAX_NAME = 24
PLUGIN_INI_REL = ("Data", "NVSE", "Plugins", "nvcoop.ini")
PLUGIN_DLL_REL = ("Data", "NVSE", "Plugins", "nvcoop.dll")
SESSION_DEFAULTS = {"coop": {"handshake": "1", "baseline": "1", "dialogue": "1"}}


@dataclass
class Settings:
    name: str = "Courier"
    mode: str = "host"                 # "host" | "join"
    connection: str = "ip"             # "ip" (UDP, the host's address and a forwarded port) | "steam" (Steam friends, nothing to forward)
    address: str = ""                  # join: the host's address, "1.2.3.4" or "1.2.3.4:27960" or a name; over Steam the host's Steam ID
    port: int = DEFAULT_PORT           # host: the port to listen on (over Steam: on this PC only, for the Steam helper)
    tunnel_port: int = 0               # join over Steam: the local UDP port of the Steam helper's tunnel, where the game connects (set by the launcher)
    max_players: int = 4
    xp_share: bool = False
    slowmo: bool = False
    tether: bool = True
    live_actors: bool = True
    live_avatars: bool = True
    live_combat: bool = True
    live_spawns: bool = True
    game_dir: str = ""
    token: int = 0
    second_copy: bool = False          # a second game copy on this PC (split screen, or two copies for testing): its own folder
    autostart: str = ""                # "start" = the plugin loads the party's start save by itself (lobby start); "load:<save>" = it loads that save (Continue)
    party: int = 0                     # lobby start: how many players the start save waits for before the opening movie
    balanced: bool = False             # host: a new campaign plays Co-op balanced (tougher enemies, more of them, scaled by party size)
    friendly_fire: bool = False        # host: a new campaign lets players hurt each other
    chat: bool = True                  # text chat in the game
    chat_key: str = "Y"                # the key that opens the chat line

    def normalized(self) -> "Settings":
        """Same dependency rule as the plugin (live_switches.cpp resolveSwitches): combat and spawns need actors."""
        s = Settings(**asdict(self))
        if (s.live_combat or s.live_spawns) and not s.live_actors and not fnv_mut.on("no_live_dependency"):
            s.live_actors = True
        return s


def parse_chat_key(key) -> Tuple[Optional[str], str]:
    """The chat key as plugin/chat reads it (chat_text.cpp parseKeyName): one letter or digit, or F1 to F24."""
    k = str(key or "").strip().upper()
    if len(k) == 1 and k.isascii() and k.isalnum():
        return k, ""
    if len(k) in (2, 3) and k[0] == "F" and k[1:].isdigit() and 1 <= int(k[1:]) <= 24 and not fnv_mut.on("chat_key_any"):
        return k, ""
    return None, "Chat key: one letter or digit, or F1 to F24."


def validate_name(name: str) -> Tuple[Optional[str], str]:
    n = (name or "").strip()
    if not n:
        return None, "Type a player name."
    if len(n) > MAX_NAME and not fnv_mut.on("name_no_limit"):
        return None, f"The name can have at most {MAX_NAME} characters."
    if any(ord(c) < 32 or ord(c) == 127 for c in n):
        return None, "The name cannot contain control characters."
    try:
        n.encode(fnv_ini.ini_encoding())
    except UnicodeEncodeError:
        return None, "The name has a character the game's text settings cannot hold. Use letters and digits."
    return n, ""


_HOSTNAME = re.compile(r"^(?=.{1,253}$)([A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?)(\.[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?)*$")
_IPV4 = re.compile(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$")


def parse_port(text, default: Optional[int] = None) -> Tuple[Optional[int], str]:
    t = str(text).strip()
    if not t and default is not None:
        return default, ""
    if not t.isdigit() or not 1 <= int(t) <= 65535:
        return None, "The port must be a whole number from 1 to 65535."
    return int(t), ""


def parse_address(text: str, default_port: int = DEFAULT_PORT,
                  resolver: Optional[Callable[[str], str]] = None) -> Tuple[Optional[Tuple[str, int]], str]:
    """'1.2.3.4', '1.2.3.4:27960', 'friend.example.net:30000' -> (host, port) or (None, a message for the player)."""
    t = (text or "").strip()
    if not t:
        return None, "Type the host's address, for example 203.0.113.5 or 203.0.113.5:27960."
    if "://" in t or "/" in t or " " in t:
        return None, "Type just the address and optional port, like 203.0.113.5:27960 (no http://, no spaces)."
    if t.startswith("[") or t.count(":") > 1:
        return None, "IPv6 addresses are not supported. Ask the host for an IPv4 address (four numbers with dots) or a host name."
    host, port = t, default_port
    if ":" in t:
        host, _, ptxt = t.partition(":")
        p, why = parse_port(ptxt)
        if p is None:
            return None, why
        port = p
    host = host.strip()
    m = _IPV4.match(host)
    if m:
        if any(int(g) > 255 for g in m.groups()) and not fnv_mut.on("ip_no_range"):
            return None, f"{host} is not a valid address: each of the four numbers must be 0 to 255."
        if host in ("0.0.0.0", "255.255.255.255"):
            return None, f"{host} cannot be used to join a game."
    elif not _HOSTNAME.match(host):
        return None, f"'{host}' is not a valid address or host name."
    elif resolver is not None:
        try:
            resolver(host)
        except OSError:
            return None, f"Cannot find '{host}'. Check the spelling and your internet connection."
    return (host, port), ""


def parse_steam_host(text: str) -> Tuple[Optional[str], str]:
    """The host's Steam ID (17 digits) -> (id, "") or (None, a message for the player)."""
    t = (text or "").strip()
    if not fnv_steam.valid_steam_id(t) and not fnv_mut.on("steam_id_any"):
        return None, "That is not a Steam ID. Pick your host from the list of Steam friends who are hosting."
    return t, ""


def new_token() -> int:
    """A random 63-bit identity for this PC; the host uses it to give you the same player slot when you rejoin."""
    return random.SystemRandom().randrange(1, 1 << 63)


# ---- the launcher's own settings file ------------------------------------------------------------------------------------------

def config_path(env: Optional[Dict[str, str]] = None) -> Path:
    env = os.environ if env is None else env
    base = env.get("FNVCOOP_LAUNCHER_DIR") or os.path.join(env.get("APPDATA") or os.path.expanduser("~"), "FNVCoOp")
    return Path(base) / "launcher.json"


def load_settings(path: Optional[Path] = None) -> Settings:
    path = path or config_path()
    s = Settings()
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
        if isinstance(raw, dict):
            for k, v in raw.items():
                if k in asdict(s) and type(v) is type(getattr(s, k)):
                    setattr(s, k, v)
    except (OSError, ValueError):
        pass
    if not s.token:
        s.token = new_token()
    if s.mode not in ("host", "join"):
        s.mode = "host"
    return s


def save_settings(s: Settings, path: Optional[Path] = None) -> None:
    path = path or config_path()
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(".tmp")
    tmp.write_text(json.dumps(asdict(s), indent=1), encoding="utf-8")
    os.replace(tmp, path)


# ---- nvcoop.ini ----------------------------------------------------------------------------------------------------------------

@dataclass
class Plan:
    ok: bool
    errors: List[str] = field(default_factory=list)
    values: Dict[str, Dict[str, str]] = field(default_factory=dict)
    host: str = ""
    port: int = 0


def b(v: bool) -> str:
    return "1" if v else "0"


def autostart_value(text: str) -> str:
    """The [coop] autostart value to write: "start", "load:<save name>" for a save name the plugin can load, else "" (the main menu stays)."""
    if text == "start":
        return text
    if text.startswith("load:") and fnv_party_saves.valid_save_name(text[5:]):
        return text
    return ""


def build_plan(s: Settings, resolver: Optional[Callable[[str], str]] = None) -> Plan:
    """Check the settings and work out the nvcoop.ini keys. No file is touched."""
    s = s.normalized()
    plan = Plan(ok=True)
    name, why = validate_name(s.name)
    if name is None:
        plan.errors.append(why)
    if s.mode not in ("host", "join"):
        plan.errors.append("Pick Host or Join.")
    if not 2 <= s.max_players <= 4:
        plan.errors.append("A session holds 2 to 4 players.")
    coop: Dict[str, str] = {"enable": "1", "name": name or ""}
    coop.update(SESSION_DEFAULTS["coop"])
    if s.connection not in ("ip", "steam"):
        plan.errors.append("Pick IP address or Steam.")
    if not fnv_mut.on("steam_transport_sticky"):
        coop["transport"] = "enet"   # always written: a Steam session is plain ENet too, and an old transport=steam would stop the plugin
    coop["via"] = s.connection
    if s.mode == "host":
        port, why = parse_port(s.port)
        if port is None:
            plan.errors.append(why)
        else:
            plan.port = port
            # over Steam the game listens on this PC only: the Steam helper forwards the friends to it; nothing is opened to the internet
            coop.update({"role": "host", "listen": "127.0.0.1" if s.connection == "steam" and not fnv_mut.on("steam_listen_all") else "0.0.0.0", "port": str(port),
                         "maxplayers": str(s.max_players)})
    elif s.connection == "steam":
        sid, why = parse_steam_host(s.address)
        if sid is None:
            plan.errors.append(why)
        elif not 1 <= s.tunnel_port <= 65535:
            plan.errors.append("Steam is not connected to your host yet. Pick your host from the list of Steam friends who are hosting.")
        else:
            plan.host, plan.port = "127.0.0.1", s.tunnel_port
            coop.update({"role": "client", "host": "127.0.0.1", "port": str(s.tunnel_port), "token": str(s.token or new_token())})
    else:
        addr, why = parse_address(s.address, DEFAULT_PORT, resolver)
        if addr is None:
            plan.errors.append(why)
        else:
            plan.host, plan.port = addr
            coop.update({"role": "client", "host": addr[0], "port": str(addr[1]), "token": str(s.token or new_token())})
    plan.values = {
        "coop": coop,
        "combat": {"xpmode": b(s.xp_share)},
        "party": {"slowmo": b(s.slowmo), "tether": b(s.tether)},
        "live": {"actors": b(s.live_actors), "avatars": b(s.live_avatars), "combat": b(s.live_combat), "spawns": b(s.live_spawns)},
        "rebalance": {"mode": "balanced" if s.balanced and not fnv_mut.on("balanced_ignored") else "normal", "friendlyfire": b(s.friendly_fire)},
        "chat": {"enabled": b(s.chat), "name": "" if fnv_mut.on("chat_no_name") else (name or "")},
    }
    key, why = parse_chat_key(s.chat_key)
    if key is None:
        plan.errors.append(why)
    else:
        plan.values["chat"]["key"] = key
    if s.mode == "join":
        plan.values["coop"].pop("maxplayers", None)
    plan.values["coop"]["autostart"] = autostart_value(s.autostart)
    plan.values["coop"]["party"] = str(s.party) if plan.values["coop"]["autostart"] == "start" and 1 <= s.party <= 8 else ""
    # The game lets one copy run per PC; the plugin hides that check in a second copy ([bridge] multiInstance, the same keys
    # tools/splitscreen/plan.py writes), and the second copy gets its own bridge pipe number.
    # Host and guest on one PC each get their own bridge pipe (two copies on pipe 1 left the second without one).
    plan.values["bridge"] = {"multiInstance": b(s.second_copy), "instance": "1" if s.second_copy and s.mode == "join" else "0"}
    # Two copies on one PC: the one without the focus pauses (the engine opens the escape menu when its window is deactivated; the focus guard,
    # plugin/splitscreen/focus_guard.h, keeps that from it), and they would open on top of each other (host on the left, the friend on the right).
    plan.values["bridge"]["focusGuard"] = b(s.second_copy)
    plan.values["bridge"]["windowSide"] = ("left" if s.mode == "host" else "right") if s.second_copy else ""
    plan.ok = not plan.errors
    return plan


def ini_path(game: Path) -> Path:
    return Path(game).joinpath(*PLUGIN_INI_REL)


def write_ini(game, plan: Plan) -> Path:
    """Apply the plan to the game's nvcoop.ini. Keeps every other line; keeps one copy of the previous file as nvcoop.ini.prev."""
    if not plan.ok:
        raise ValueError("; ".join(plan.errors))
    path = ini_path(game)
    old = fnv_ini.read_text(path)
    if path.is_file() and not fnv_mut.on("no_prev_copy"):
        try:
            Path(str(path) + ".prev").write_bytes(path.read_bytes())
        except OSError:
            pass
    text = fnv_ini.apply(old, plan.values)
    fnv_ini.write_atomic(path, text)
    return path


def read_settings_from_ini(game, base: Optional[Settings] = None) -> Settings:
    """Fill Settings from an existing nvcoop.ini so the window shows what the game will actually use."""
    s = Settings(**asdict(base)) if base else Settings()
    t = fnv_ini.read_text(ini_path(game))
    if not t:
        return s

    def g(sec, key):
        return fnv_ini.get(t, sec, key)

    def flag(v, default):
        return default if v is None else v.strip().lower() in ("1", "true", "on", "yes")
    role = (g("coop", "role") or "").lower()
    if role in ("host",):
        s.mode = "host"
    elif role in ("client", "guest"):
        s.mode = "join"
    p, _ = parse_port(g("coop", "port") or "")
    if p:
        s.port = p
    s.connection = "steam" if (g("coop", "via") or "").strip().lower() == "steam" else "ip"
    h = g("coop", "host")
    if h and role in ("client", "guest") and s.connection == "ip" and h != "127.0.0.1":
        s.address = h if s.port == DEFAULT_PORT else f"{h}:{s.port}"
    s.xp_share = flag(g("combat", "xpmode"), s.xp_share)
    s.slowmo = flag(g("party", "slowmo"), s.slowmo)
    s.tether = flag(g("party", "tether"), s.tether)
    s.live_actors = flag(g("live", "actors"), s.live_actors)
    s.live_avatars = flag(g("live", "avatars"), s.live_avatars)
    s.live_combat = flag(g("live", "combat"), s.live_combat)
    s.live_spawns = flag(g("live", "spawns"), s.live_spawns)
    m = (g("rebalance", "mode") or "").strip().lower()
    if m in ("normal", "balanced"):
        s.balanced = m == "balanced"
    s.friendly_fire = flag(g("rebalance", "friendlyfire"), s.friendly_fire)
    s.chat = flag(g("chat", "enabled"), s.chat)
    k, _ = parse_chat_key(g("chat", "key") or "")
    if k:
        s.chat_key = k
    return s


def switch_off(game) -> bool:
    """Set [coop] enable=0 (used by the Stop co-op button so a normal launch from Steam plays single-player)."""
    path = ini_path(game)
    if not path.is_file():
        return False
    fnv_ini.write_atomic(path, fnv_ini.set_value(fnv_ini.read_text(path), "coop", "enable", "0"))
    return True


# ---- starting the game --------------------------------------------------------------------------------------------------------

def preflight(game, running: Optional[Callable[[], bool]] = None) -> Tuple[List[str], List[str]]:
    """(problems, warnings) before the game is started."""
    running = running or fnv_find.game_is_running
    game = Path(game)
    problems: List[str] = []
    warnings: List[str] = []
    info = fnv_find.inspect_game(game)
    problems += info.problems
    problems += info.overridable
    warnings += info.warnings
    if not game.joinpath(*PLUGIN_DLL_REL).is_file() and not fnv_mut.on("skip_plugin_check"):
        problems.append("The co-op plugin is not installed in this game folder. Run Install.cmd first.")
    elif fnv_install.load_record(game) is None:
        warnings.append("The plugin files are there but were not put there by this installer (no FNVCoOp\\install.json).")
    if running():
        problems.append("Fallout: New Vegas is already running. Close it first; a second copy cannot be started.")
    return problems, warnings


def launch_game(game, popen=subprocess.Popen) -> int:
    """Start xNVSE's loader (which starts FalloutNV.exe with the plugins). Returns the loader's process id."""
    game = Path(game)
    loader = game / fnv_find.NVSE_LOADER
    if not loader.is_file():
        raise FileNotFoundError(f"{fnv_find.NVSE_LOADER} not found in {game}")
    p = popen([str(loader)], cwd=str(game), close_fds=True)
    return int(getattr(p, "pid", 0))


@dataclass
class StartResult:
    ok: bool
    messages: List[str] = field(default_factory=list)
    pid: int = 0
    plan: Optional[Plan] = None


def start_session(s: Settings, *, resolver: Optional[Callable[[str], str]] = None, running: Optional[Callable[[], bool]] = None,
                  popen=subprocess.Popen, port_free: Callable[[int], Tuple[bool, str]] = fnv_portcheck.bind_check) -> StartResult:
    """Host / Join pressed: check everything, write nvcoop.ini, start the game. Nothing is written when a check fails."""
    r = StartResult(ok=False)
    if not s.game_dir:
        r.messages.append("Pick your Fallout: New Vegas folder first.")
        return r
    problems, warnings = preflight(s.game_dir, (lambda: False) if s.second_copy else running)
    r.messages += ["warning: " + w for w in warnings]
    if problems:
        r.messages += ["PROBLEM: " + p for p in problems]
        return r
    plan = build_plan(s, resolver)
    r.plan = plan
    if not plan.ok:
        r.messages += ["PROBLEM: " + e for e in plan.errors]
        return r
    if s.mode == "host":
        ok, why = port_free(plan.port)
        if not ok and not fnv_mut.on("skip_port_check"):
            r.messages.append("PROBLEM: " + why)
            return r
    write_ini(s.game_dir, plan)
    r.messages.append(f"Saved the settings to {ini_path(s.game_dir)}")
    try:
        r.pid = launch_game(s.game_dir, popen)
    except OSError as e:
        r.messages.append(f"PROBLEM: could not start the game: {e}")
        return r
    r.ok = True
    if s.mode == "host" and s.connection == "steam":
        r.messages.append("Starting the game as host over Steam. When you are in the world, your Steam friends can join. Keep the launcher open while you play.")
    elif s.mode == "host":
        r.messages.append(f"Starting the game as host on UDP port {plan.port}. When you are in the world, friends can join.")
    elif s.connection == "steam":
        r.messages.append("Starting the game and joining your host over Steam. Keep the launcher open while you play.")
    else:
        r.messages.append(f"Starting the game and joining {plan.host}:{plan.port}.")
    return r
