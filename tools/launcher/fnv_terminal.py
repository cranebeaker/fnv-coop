#!/usr/bin/env python3
"""The co-op launcher, terminal style: a small web page (terminal.html) served on this PC only and shown in an app window.

    py -3 launcher\\fnv_terminal.py            opens the launcher window (Edge app mode, else the default browser)
    py -3 launcher\\fnv_terminal.py --no-window --port 8765    serve only (tests, or open the printed address yourself)

The page talks to this process through /api/* on 127.0.0.1 with a random key, so no other program or web page can drive it. All the
game logic is fnv_launcher_core.py (settings, nvcoop.ini, starting the game) and fnv_lobby.py (the party lobby). The process ends a
little after the window is closed. Nothing outside the party is contacted.
"""
import argparse
import json
import os
import secrets
import socket
import subprocess
import sys
import threading
import time
import webbrowser
from dataclasses import asdict
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import List, Optional, Tuple

_HERE = os.path.dirname(os.path.abspath(__file__))
for _d in (_HERE, os.path.join(os.path.dirname(_HERE), "installer")):
    if _d not in sys.path:
        sys.path.insert(0, _d)
import fnv_find  # noqa: E402
import fnv_launcher_core as core  # noqa: E402
import fnv_mut  # noqa: E402
import fnv_lobby  # noqa: E402
import fnv_party_saves  # noqa: E402
import fnv_pe  # noqa: E402
import fnv_portcheck  # noqa: E402
import fnv_steam  # noqa: E402

HERE = Path(_HERE)
PAGE = HERE / "terminal.html"
FONT = HERE / "VT323-Regular.ttf"
IDLE_EXIT = 20.0          # seconds without the page asking for state, after it first did: the window was closed


def release_version() -> str:
    """The release this launcher came with (manifest.json next to launcher\\); "dev" in the repo."""
    try:
        return str(json.loads((HERE.parent / "manifest.json").read_text(encoding="utf-8"))["version"])
    except (OSError, ValueError, KeyError):
        return "dev"


def join_key(connection: str, address: str) -> str:
    """Who a guest joins, as one string: the same host gives the same key (the port counts, the default port is written out)."""
    a = (address or "").strip().lower()
    if connection == "steam":
        return "steam:" + a
    addr, _ = core.parse_address(a, core.DEFAULT_PORT)
    return "ip:%s:%d" % addr if addr else ""


class App:
    def __init__(self, settings_path: Optional[Path] = None, version: Optional[str] = None):
        self.settings_path = settings_path
        self.s = core.load_settings(settings_path)
        self.version = version or release_version()
        self.lock = threading.RLock()
        self.lines: List[str] = []
        self.lobby = None
        self.launched = ""
        self.documents: Optional[str] = None     # tests: stands in for the Documents folder
        self.steam: Optional[fnv_steam.SteamLink] = None
        self.steam_spawn = None                  # tests: makes a pretend Steam helper (fnv_steam_fake) instead of starting fnvsteam.exe
        self.steam_lock = threading.Lock()       # one helper start at a time (it can take seconds; the page's polling must not wait for it)
        self.steam_friends: List[dict] = []
        self.steam_online: List[dict] = []         # friends online now (the invite list)
        self.steam_note = ""
        self.steam_tag = fnv_steam.protocol_tag()
        self.last_poll: Optional[float] = None
        if not self.s.game_dir:
            self.find_game()

    # ---- helpers --------------------------------------------------------------------------------------------------------
    def log(self, msg: str) -> None:
        with self.lock:
            self.lines.append(msg)
            del self.lines[:-60]

    def remember(self) -> None:
        try:
            core.save_settings(self.s, self.settings_path)
        except OSError as e:
            self.log(f"Could not remember the settings: {e}")

    def game_line(self) -> dict:
        g = self.s.game_dir
        if not g:
            return {"dir": "", "ok": False, "line": "GAME NOT FOUND. OPEN SETTINGS AND PICK THE FOLDER WITH FalloutNV.exe."}
        problems, warnings = core.preflight(g, running=lambda: False)
        info = fnv_find.inspect_game(g)
        nv = ".".join(map(str, fnv_find.nvse_triple(info.nvse_version) or ())) or "MISSING"
        head = f"GAME {fnv_pe.format_version(info.exe_version)}, xNVSE {nv}"
        return {"dir": g, "ok": not problems, "line": head + (": " + " ".join(problems) if problems else ": OK"),
                "warnings": warnings}

    def find_game(self) -> None:
        cands = fnv_find.candidates()
        if cands:
            self.s.game_dir = str(cands[0].path)
            self.s = core.read_settings_from_ini(self.s.game_dir, self.s)
            self.log(f"Game found: {self.s.game_dir}")
        else:
            self.log("Could not find the game. Pick its folder in SETTINGS.")

    def host_saves(self) -> List[fnv_party_saves.HostSave]:
        """The co-op saves this PC can continue as host, newest first (nothing when the game folder is not set)."""
        if not self.s.game_dir:
            return []
        return fnv_party_saves.list_host_saves(fnv_party_saves.game_save_dir(self.s.game_dir, self.documents))

    # ---- state for the page -----------------------------------------------------------------------------------------------
    def state(self) -> dict:
        with self.lock:
            self.last_poll = time.monotonic()
            lob = asdict(self.lobby.view()) if self.lobby else None
            s = asdict(self.s)
            s.pop("token", None)
            s.pop("last_campaign", None)
            saves = [{"name": h.name, "campaign": fnv_party_saves.short_campaign(h.campaign), "when": time.strftime("%Y-%m-%d %H:%M", time.localtime(h.mtime))}
                     for h in self.host_saves()[:8]]
            return {"version": self.version, "settings": s, "game": self.game_line(), "lobby": lob, "launched": self.launched,
                    "log": list(self.lines), "local_ips": fnv_portcheck.local_ips(), "saves": saves, "steam": self.steam_view()}

    # ---- actions ----------------------------------------------------------------------------------------------------------
    def apply_settings(self, d: dict) -> dict:
        with self.lock:
            for k, v in d.items():
                if k in ("token", "last_join_key", "last_campaign") or not hasattr(self.s, k):
                    continue
                cur = getattr(self.s, k)
                if isinstance(cur, bool):
                    setattr(self.s, k, bool(v))
                elif k == "port":
                    p, why = core.parse_port(v)
                    if p:
                        self.s.port = p
                    else:
                        self.log(why)
                elif k == "max_players":
                    if str(v).isdigit() and 2 <= int(v) <= 4:
                        self.s.max_players = int(v)
                elif k == "chat_key":
                    key, why = core.parse_chat_key(v)
                    if key:
                        self.s.chat_key = key
                    else:
                        self.log(why)
                elif k == "join_mode":
                    if v in core.JOIN_MODES:
                        self.s.join_mode = v
                elif k == "connection":
                    if v in ("ip", "steam") and not self.lobby:
                        self.s.connection = v
                elif isinstance(cur, str):
                    setattr(self.s, k, str(v)[:260])
            if "game_dir" in d and self.s.game_dir:
                self.s = core.read_settings_from_ini(self.s.game_dir, self.s)
            self.remember()
            return {"ok": True}

    # ---- Steam (docs/steam.md) ---------------------------------------------------------------------------------------------------
    def steam_available(self) -> bool:
        return self.steam_spawn is not None or fnv_steam.helper_path() is not None

    def steam_view(self) -> dict:
        link = self.steam
        req = link.join_requests[-1] if link and link.join_requests else None
        if req:
            known = next((f for f in self.steam_friends if f["id"] == req["id"]), None)
            online = next((f for f in self.steam_online if f["id"] == req["id"]), None)
            req = {"id": req["id"], "name": (known or online or {}).get("name") or req["id"], "mode": known["mode"] if known else "",
                   "lobby": req.get("lobby", ""), "invite": req.get("connect") == "invite"}
        return {"available": self.steam_available(), "state": link.state if link else "off", "name": link.name if link else "",
                "error": (link.error if link else "") or self.steam_note, "friends": list(self.steam_friends), "join_request": req,
                "online": list(self.steam_online)}

    def steam_ready(self) -> Tuple[Optional[fnv_steam.SteamLink], str]:
        """The running helper, starting it if needed (outside self.lock: it can take seconds)."""
        with self.steam_lock:
            if self.steam is not None and self.steam.state == "ready":
                return self.steam, ""
            if self.steam is not None:
                self.steam.close()
                self.steam = None
            if not self.steam_available():
                self.steam_note = "Steam joining needs fnvsteam.exe next to the launcher. This copy does not have it; use an IP address instead."
                return None, self.steam_note
            spawn = self.steam_spawn or fnv_steam.spawn_helper
            link = fnv_steam.SteamLink(spawn)
            ok, why = link.start()
            if not ok:
                self.steam_note = why
                return None, why
            self.steam, self.steam_note = link, ""
            return link, ""

    def steam_close(self) -> None:
        """Quit the helper: the party is over. It stays alive from the lobby through the whole game, because it carries the game's traffic."""
        with self.steam_lock:
            link, self.steam = self.steam, None
        if link is not None:
            link.close()

    def steam_refresh(self) -> dict:
        """Ask Steam which friends are hosting; the page lists them."""
        link, why = self.steam_ready()
        if link is None:
            return {"ok": False, "why": why}
        rows = link.friends(self.steam_tag)
        out = []
        for r in rows:
            joinable = r.get("same", True) or fnv_mut.on("steam_ignore_release")
            out.append({"id": r["id"], "name": r["name"], "mode": r["mode"], "joinable": joinable, "lobby": r.get("lobby", "")})
        with self.lock:
            self.steam_friends = out
        return {"ok": True}

    def steam_online_refresh(self) -> dict:
        """Ask Steam which friends are online: the host's invite list."""
        link, why = self.steam_ready()
        if link is None:
            return {"ok": False, "why": why}
        rows = link.online()
        with self.lock:
            self.steam_online = rows
        return {"ok": True}

    def steam_invite(self, steam_id: str) -> dict:
        """Host over Steam: send this friend Steam's invite into our lobby. If their launcher is open it asks them to join at once;
        otherwise the invite waits in their Steam chat (they open the launcher first, then accept it)."""
        with self.lock:
            hosting = isinstance(self.lobby, fnv_lobby.HostLobby) and self.lobby.sock_factory is not None   # a lobby held over Steam
        if not hosting:
            return {"ok": False, "why": "Host a game with the connection set to STEAM first, then invite."}
        link, why = self.steam_ready()
        if link is None:
            return {"ok": False, "why": why}
        why = link.invite(steam_id)
        if why:
            return {"ok": False, "why": why}
        who = next((f["name"] for f in self.steam_online if f["id"] == steam_id), steam_id)
        self.log(f"INVITE SENT TO {who}. IF THEIR LAUNCHER IS OPEN THEY CAN JOIN NOW; IF NOT, THEY OPEN IT AND ACCEPT YOUR INVITE IN STEAM.")
        return {"ok": True}

    def autostart_for(self, mode: str, lobby, camp: str = "") -> str:
        """[coop] autostart for this machine when the lobby starts: a new game loads the start save; a continued campaign loads the host's
        save (host) or this player's own save for that host save (guest). A guest with no save for the campaign joins with the start save.
        No lobby (a guest joining a game in progress): see autostart_in_progress."""
        if lobby is None:
            return self.autostart_in_progress(camp) if mode == "join" else ""
        if mode == "host":
            return "load:" + lobby.host_save if lobby.campaign and lobby.host_save else "start"
        camp, host_save = lobby.start_info
        if camp and host_save:
            mine = fnv_party_saves.guest_pick(self.s.game_dir, camp, host_save, self.documents)
            if mine:
                self.log(f"CONTINUING WITH YOUR SAVE {mine.upper()}.")
                return "load:" + mine
            self.log("NO SAVE OF YOURS FOR THIS CAMPAIGN. YOU JOIN THE HOST AS A NEW CHARACTER.")
        return "start"

    def autostart_in_progress(self, camp: str = "") -> str:
        """[coop] autostart for a guest that joins a game already in the world, so nobody has to click through the game's menus:
        * the host's campaign (`camp`) is known when this PC joined the same host before (the plugin noted it in nvcoop.ini, _launch reads it):
          load this player's newest save of that campaign, the way CONTINUE does for a lobby;
        * otherwise the shipped start save. The plugin then skips the opening movie (the host says the campaign is under way), the guest makes its
          character at Doc's and is moved to the host (docs/party-save.md, "A guest with no save for the campaign").
        Never "" and never a wait for a party: a joined guest has nothing to wait for."""
        if fnv_mut.on("inprogress_loads_nothing"):
            return ""
        camp = fnv_party_saves.norm_campaign(camp)
        if camp and not fnv_mut.on("inprogress_ignores_campaign"):
            mine = fnv_party_saves.guest_pick(self.s.game_dir, camp, "", self.documents)
            if mine:
                self.log(f"JOINING IN PROGRESS WITH YOUR SAVE {mine.upper()}.")
                return "load:" + mine
        self.log("JOINING IN PROGRESS AS A NEW CHARACTER: YOUR GAME SKIPS THE OPENING, YOU MAKE YOUR CHARACTER AND ARE MOVED TO THE HOST.")
        return "start"

    def _lobby_start(self, mode: str, lobby, address: str = "") -> None:
        """A lobby decided to start the game. Over Steam the helper stays: the host's friends keep arriving through it, and every game's
        traffic runs through it. The host's lobby changes to "in the world" so friends can join in progress."""
        if getattr(lobby, "sock_factory", None) is not None:
            if fnv_mut.on("steam_close_at_start"):
                self.steam_close()
            elif mode == "host" and self.steam is not None:
                self.steam.set_mode("game")
        self._launch(mode, address, lobby=lobby)

    def _launch(self, mode: str, address: str = "", lobby=None) -> bool:
        lobby = lobby if lobby is not None else self.lobby
        s = core.Settings(**asdict(self.s))
        s.mode, s.address = mode, address or s.address
        if lobby is not None:
            s.connection = "ip" if getattr(lobby, "sock_factory", None) is None else "steam"   # how the lobby was held is how the game joins
        if s.connection == "steam":
            link = self.steam
            if mode == "join":
                s.tunnel_port = link.tunnel_port if link is not None else 0
            elif link is not None and not link.hosting:
                link.host(self.steam_tag, "game", s.port, s.max_players)    # no lobby was held: open the Steam side now, friends join in progress
        key = join_key(s.connection, address or s.address) if mode == "join" else ""
        s.last_campaign = ""
        if mode == "join" and key and key == self.s.last_join_key and self.s.game_dir:
            s.last_campaign = fnv_party_saves.last_campaign(self.s.game_dir)      # the same host as last time: the plugin's note is about its campaign
        s.autostart = self.autostart_for(mode, lobby, s.last_campaign)
        if mode == "join" and lobby is not None:
            s.last_campaign = lobby.start_info[0]                                 # the lobby said it ("" in a new game: the plugin notes the new campaign)
        s.party = len(lobby.view().players) if lobby else 0
        r = core.start_session(s, resolver=socket.gethostbyname)
        for m in r.messages:
            self.log(m)
        if r.ok:
            self.launched = "host" if mode == "host" else "guest"
            self.s.mode = mode
            if mode == "join":
                self.s.address = s.address
                self.s.last_join_key = key
            self.remember()
        return r.ok

    def host(self, cont: bool = False, save: str = "") -> dict:
        """Open the lobby. `cont`: continue a campaign from the host's co-op save `save` (empty = the newest one)."""
        link = None
        if self.s.connection == "steam" and not self.lobby:
            link, why = self.steam_ready()
            if link is None:
                return {"ok": False, "why": why}
        with self.lock:
            if self.lobby:
                return {"ok": False, "why": "Leave the current lobby first."}
            pick = None
            if cont:
                saves = self.host_saves()
                pick = next((h for h in saves if h.name == save), None) if save else (saves[0] if saves else None)
                if pick is None:
                    return {"ok": False, "why": "No co-op save to continue." if not save else f"No co-op save named {save}."}
            name, why = core.validate_name(self.s.name)
            if name is None:
                return {"ok": False, "why": why}
            gl = self.game_line()
            if not gl["ok"]:
                return {"ok": False, "why": gl["line"]}
            if link is not None:
                why = link.host(self.steam_tag, "lobby", self.s.port, self.s.max_players)   # friends see the lobby in Steam and in their launcher's list
                if why:
                    return {"ok": False, "why": why}
                lob = fnv_lobby.HostLobby(name, 0, self.version, max_players=self.s.max_players, sock_factory=link.socket)
            else:
                lob = fnv_lobby.HostLobby(name, self.s.port, self.version, max_players=self.s.max_players)
            ok, why = lob.open()
            if not ok:
                return {"ok": False, "why": why}
            if pick:
                lob.campaign, lob.host_save = pick.campaign, pick.name
            lob.on_start = lambda: self._lobby_start("host", lob)
            self.lobby = lob
            self.s.mode = "host"
            self.remember()
            if pick:
                self.log(f"CONTINUING {pick.name.upper()} (CAMPAIGN {fnv_party_saves.short_campaign(pick.campaign).upper()}).")
            if link is not None:
                self.log("LOBBY OPEN OVER STEAM. YOUR STEAM FRIENDS PICK YOU IN JOIN A FRIEND, OR CLICK JOIN GAME NEXT TO YOUR NAME. NO ADDRESS, NO PORT FORWARDING.")
            else:
                ips = ", ".join(f"{ip}:{lob.port}" for ip in fnv_portcheck.local_ips()) or "this PC's address"
                self.log(f"LOBBY OPEN ON UDP {lob.port}. HOME NETWORK: {ips}. INTERNET: YOUR PUBLIC ADDRESS, UDP {lob.port} FORWARDED.")
            return {"ok": True}

    def join(self, address: str, mode: str = "") -> dict:
        """JOIN with an IP address. `mode` (default: the setting): "auto" asks the host's lobby first and, when nothing answers within a few
        seconds, starts the game to join the host in the world; "lobby" only the lobby; "game" only the game in progress."""
        mode = mode if mode in core.JOIN_MODES else self.s.join_mode
        if mode == "game":
            return self.join_direct(address)
        with self.lock:
            if self.lobby:
                return {"ok": False, "why": "Leave the current lobby first."}
            name, why = core.validate_name(self.s.name)
            if name is None:
                return {"ok": False, "why": why}
            addr, why = core.parse_address(address, core.DEFAULT_PORT, socket.gethostbyname)
            if addr is None:
                return {"ok": False, "why": why}
            gl = self.game_line()
            if not gl["ok"]:
                return {"ok": False, "why": gl["line"]}
            if not self.s.token:
                self.s.token = core.new_token()
            self.s.address = address.strip()
            self.remember()
            lob = fnv_lobby.GuestLobby(name, self.s.token, addr[0], addr[1], self.version, probe_s=fnv_lobby.PROBE_FOR if mode == "auto" and not fnv_mut.on("join_no_probe") else 0.0)
            lob.on_start = lambda: self._launch("join", address.strip(), lobby=lob)
            lob.on_no_lobby = lambda: self._no_lobby(lob, address.strip())
            lob.open()
            self.lobby = lob
            if mode == "auto":
                self.log("LOOKING FOR THE HOST'S LOBBY. IF THE HOST IS ALREADY IN THE WORLD, YOUR GAME STARTS AND JOINS THEM.")
            return {"ok": True}

    def _no_lobby(self, lob, address: str) -> None:
        """Nothing answered the lobby hello: the host is in the world (or not there). Start the game and join it in progress."""
        with self.lock:
            if self.lobby is not lob:
                return                      # the player left in the meantime
            self.lobby = None
            self.log("NO LOBBY ANSWERED, SO THE HOST IS PROBABLY IN THE WORLD ALREADY. STARTING YOUR GAME TO JOIN THEM.")
        r = self.join_direct(address)
        if not r.get("ok"):
            self.log("COULD NOT START YOUR GAME. SEE THE MESSAGES ABOVE.")

    def join_steam(self, steam_id: str, in_game: bool = False, lobby: str = "") -> dict:
        """Join a Steam friend who is hosting: into their lobby, or straight into their game when they are already in the world (in_game)."""
        sid, why = core.parse_steam_host(steam_id)
        if sid is None:
            return {"ok": False, "why": why}
        known = next((f for f in self.steam_friends if f["id"] == sid), None)
        lobby = lobby if lobby.isdigit() else (known["lobby"] if known else "")   # an invite names its lobby; a listed friend has one
        if known is not None and not known["joinable"]:
            return {"ok": False, "why": f"{known['name']} runs a different release of FNV Co-Op. Everyone needs the same one."}
        if in_game:
            return self.join_direct(sid, steam=True, lobby=lobby)
        link, why = self.steam_ready()
        if link is None:
            return {"ok": False, "why": why}
        with self.lock:
            if self.lobby:
                return {"ok": False, "why": "Leave the current lobby first."}
            name, why = core.validate_name(self.s.name)
            if name is None:
                return {"ok": False, "why": why}
            gl = self.game_line()
            if not gl["ok"]:
                return {"ok": False, "why": gl["line"]}
            if not self.s.token:
                self.s.token = core.new_token()
            self.s.connection, self.s.address = "steam", sid
            self.remember()
            why = link.join(sid, lobby)
            if why:
                return {"ok": False, "why": why}
            lob = fnv_lobby.GuestLobby(name, self.s.token, fnv_steam.PEER, 0, self.version, sock_factory=link.socket)
            lob.on_start = lambda: self._lobby_start("join", lob, sid)
            lob.open()
            self.lobby = lob
            link.join_requests.clear()
            return {"ok": True}

    def join_direct(self, address: str, steam: bool = False, lobby: str = "") -> dict:
        """The host is already in the world: no lobby, start the game and join it. Over Steam the helper first goes into the host's Steam
        lobby (that opens the tunnel), then the game connects to the tunnel."""
        over_steam = steam or self.s.connection == "steam"
        if over_steam:
            sid, why = core.parse_steam_host(address)
            if sid is None:
                return {"ok": False, "why": why}
            address = sid
            link, why = self.steam_ready()
            if link is None:
                return {"ok": False, "why": why}
            why = link.join(sid, lobby)
            if why:
                return {"ok": False, "why": why}
        with self.lock:
            if self.lobby:
                return {"ok": False, "why": "Leave the current lobby first."}
            if over_steam:
                self.s.connection = "steam"
            ok = self._launch("join", address.strip())
            return {"ok": ok, "why": "" if ok else "See the messages below."}

    def ready(self, on: bool) -> dict:
        with self.lock:
            if isinstance(self.lobby, fnv_lobby.GuestLobby):
                self.lobby.set_ready(on)
                return {"ok": True}
            return {"ok": False, "why": "Only a guest has a ready switch."}

    def start(self) -> dict:
        with self.lock:
            if not isinstance(self.lobby, fnv_lobby.HostLobby):
                return {"ok": False, "why": "Only the host starts the game."}
            ok, why = self.lobby.start()
            if ok:
                self.log("BEGIN TRANSMISSION. EVERY GAME IS STARTING.")
            return {"ok": ok, "why": why}

    def leave(self) -> dict:
        with self.lock:
            if self.lobby:
                self.lobby.close()
                self.lobby = None
            self.launched = ""
        self.steam_close()
        return {"ok": True}

    def coop_off(self) -> dict:
        with self.lock:
            done = bool(self.s.game_dir) and core.switch_off(self.s.game_dir)
            self.log("CO-OP IS OFF. THE GAME STARTS AS NORMAL SINGLE-PLAYER." if done else "NOTHING TO SWITCH OFF.")
            return {"ok": done}

    def browse(self) -> dict:
        try:
            import tkinter as tk
            from tkinter import filedialog
            root = tk.Tk()
            root.withdraw()
            root.attributes("-topmost", True)
            d = filedialog.askdirectory(title="Pick the folder that holds FalloutNV.exe", parent=root)
            root.destroy()
        except Exception as e:  # noqa: BLE001  tkinter missing or no display
            return {"ok": False, "why": f"Cannot open a folder picker ({e}). Type the folder instead."}
        if d:
            self.apply_settings({"game_dir": str(Path(d))})
        return {"ok": bool(d)}


def make_handler(app: App, key: str):
    def page() -> bytes:
        return PAGE.read_bytes() if PAGE.is_file() else b"terminal.html missing"
    font = FONT.read_bytes() if FONT.is_file() else b""

    class H(BaseHTTPRequestHandler):
        def log_message(self, *a):   # quiet
            pass

        def _send(self, code: int, body: bytes, ctype: str) -> None:
            self.send_response(code)
            self.send_header("Content-Type", ctype)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def _json(self, obj) -> None:
            self._send(200, json.dumps(obj).encode("utf-8"), "application/json")

        def _authed(self) -> bool:
            return secrets.compare_digest(self.headers.get("X-Key", ""), key)

        def do_GET(self):
            path = self.path.split("?", 1)[0]
            if path == "/":
                self._send(200, page(), "text/html; charset=utf-8")
            elif path == "/font.ttf":
                self._send(200, font, "font/ttf")
            elif path == "/api/state" and self._authed():
                self._json(app.state())
            else:
                self._send(404, b"not found", "text/plain")

        def do_POST(self):
            if not self._authed():
                self._send(403, b"forbidden", "text/plain")
                return
            n = int(self.headers.get("Content-Length") or 0)
            try:
                body = json.loads(self.rfile.read(min(n, 65536)) or b"{}")
            except ValueError:
                body = {}
            if not isinstance(body, dict):
                body = {}
            routes = {
                "/api/settings": lambda: app.apply_settings(body),
                "/api/find": lambda: (app.find_game(), app.remember(), {"ok": bool(app.s.game_dir)})[-1],
                "/api/browse": app.browse,
                "/api/host": lambda: app.host(bool(body.get("continue")), str(body.get("save", ""))),
                "/api/join": lambda: app.join(str(body.get("address", "")), str(body.get("mode", ""))),
                "/api/join_direct": lambda: app.join_direct(str(body.get("address", ""))),
                "/api/steam_refresh": app.steam_refresh,
                "/api/join_steam": lambda: app.join_steam(str(body.get("id", "")), bool(body.get("in_game")), str(body.get("lobby", ""))[:24]),
                "/api/steam_online": app.steam_online_refresh,
                "/api/steam_invite": lambda: app.steam_invite(str(body.get("id", ""))[:24]),
                "/api/ready": lambda: app.ready(bool(body.get("ready"))),
                "/api/start": app.start,
                "/api/leave": app.leave,
                "/api/coop_off": app.coop_off,
                "/api/quit": lambda: (threading.Timer(0.3, lambda: os._exit(0)).start(), {"ok": True})[-1],
            }
            fn = routes.get(self.path)
            if not fn:
                self._send(404, b"not found", "text/plain")
                return
            self._json(fn())

    return H


def edge_path() -> Optional[str]:
    for base in (os.environ.get("ProgramFiles(x86)"), os.environ.get("ProgramFiles"), os.environ.get("LOCALAPPDATA")):
        if base:
            p = Path(base) / "Microsoft" / "Edge" / "Application" / "msedge.exe"
            if p.is_file():
                return str(p)
    return None


def open_window(url: str) -> None:
    edge = edge_path()
    if edge:
        prof = Path(os.environ.get("LOCALAPPDATA") or Path.home()) / "FNVCoOp" / "window"
        subprocess.Popen([edge, f"--app={url}", "--window-size=1000,760", f"--user-data-dir={prof}", "--no-first-run",
                          "--no-default-browser-check"], close_fds=True)
    else:
        webbrowser.open(url)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--port", type=int, default=0, help="local web port (default: any free one)")
    ap.add_argument("--no-window", action="store_true", help="do not open a window; print the address")
    ap.add_argument("--key", help="fixed access key (tests); default random")
    a = ap.parse_args(argv)
    app = App()
    key = a.key or secrets.token_urlsafe(18)
    srv = ThreadingHTTPServer(("127.0.0.1", a.port), make_handler(app, key))
    url = f"http://127.0.0.1:{srv.server_address[1]}/?k={key}"
    print(f"FNV Co-Op launcher: {url}", flush=True)
    if not a.no_window:
        open_window(url)

    def watchdog():
        while True:
            time.sleep(2)
            lp = app.last_poll
            lob = app.lobby
            handing_over = isinstance(lob, fnv_lobby.HostLobby) and lob.view().state == "starting"   # the host's game is not started yet
            if lp is not None and time.monotonic() - lp > IDLE_EXIT and not handing_over:
                if lob:
                    lob.close()
                os._exit(0)
    threading.Thread(target=watchdog, daemon=True).start()
    srv.serve_forever()
    return 0


if __name__ == "__main__":
    sys.exit(main())
