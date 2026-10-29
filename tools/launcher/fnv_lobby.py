"""The launcher lobby: the party gathers here before anyone's game starts. Standard library only.

The host's launcher listens on the session's UDP port (the game needs that port later, so the lobby gives it back the moment the
host presses start). Every guest's launcher sends a "hello" about twice a second; the host answers each one with the roster. When
everyone is ready the host starts: the host's answers turn into "start" for a few seconds, then the host closes the port and starts
its game, and each guest starts its own game and joins.

Every datagram is MAGIC + one JSON object:
  guest -> host  {"t":"hello", "name", "token", "ver", "ready", "ts"}
  host -> guest  {"t":"roster", "players":[{"slot","name","ready","ping","host"}], "you":slot, "ver", "echo":ts, "campaign"?, "hostSave"?}
                 {"t":"start", "players":[...], "you":slot, "campaign"?, "hostSave"?}
                 {"t":"refuse", "why"}
  guest -> host  {"t":"bye", "token"}
"campaign" and "hostSave" are only there when the host CONTINUES a campaign (docs/party-save.md): the campaign id and the name of the
host save it loads. A guest's launcher uses them to pick its own matching save. A new game carries neither.
A guest the host has not heard from for DROP_AFTER seconds leaves the roster. Nothing is stored; nothing outside the party is contacted.

The datagrams normally go over UDP. Over Steam (docs/steam.md) the same messages go through fnv_steam.SteamSocket, which has the few socket
methods used here: pass `sock_factory` and the lobby neither binds a port nor needs an address (a guest's host is ("steam", 0), its peer 0).
"""
import json
import os
import socket
import sys
import threading
import time
from dataclasses import dataclass, field
from typing import Callable, Dict, List, Optional, Tuple

_HERE = os.path.dirname(os.path.abspath(__file__))
for _d in (_HERE, os.path.join(os.path.dirname(_HERE), "installer")):
    if _d not in sys.path:
        sys.path.insert(0, _d)
import fnv_mut  # noqa: E402
import fnv_party_saves  # noqa: E402

MAGIC = b"FNVCOOP-LOBBY-1\n"
MAX_PLAYERS = 4
HELLO_EVERY = 0.5
DROP_AFTER = 5.0
START_FOR = 2.0          # seconds the host keeps answering "start" before it gives the port to its game
GIVE_UP_AFTER = 6.0      # a guest that hears nothing for this long reports the host as gone


def pack(msg: dict) -> bytes:
    return MAGIC + json.dumps(msg, separators=(",", ":")).encode("utf-8")


def unpack(data: bytes) -> Optional[dict]:
    if not data.startswith(MAGIC):
        return None
    try:
        msg = json.loads(data[len(MAGIC):].decode("utf-8"))
    except (ValueError, UnicodeDecodeError):
        return None
    return msg if isinstance(msg, dict) and isinstance(msg.get("t"), str) else None


@dataclass
class Member:
    slot: int
    name: str
    token: int
    addr: Tuple[str, int]
    ready: bool = False
    ping: int = 0
    seen: float = 0.0


@dataclass
class View:
    """What the launcher page shows. `state`: idle | hosting | joining | lobby | starting | started | closed."""
    role: str = ""
    state: str = "idle"
    players: List[dict] = field(default_factory=list)
    you: int = -1
    message: str = ""
    can_start: bool = False
    resume: str = ""          # Continue: the host save the party loads ("" = a new game)
    via: str = "ip"           # "ip" | "steam": how the party is connected (the page shows addresses only for ip)


class HostLobby:
    def __init__(self, name: str, port: int, version: str, max_players: int = MAX_PLAYERS, address: str = "0.0.0.0",
                 clock: Callable[[], float] = time.monotonic, sock_factory: Optional[Callable[[], object]] = None):
        self.name, self.port, self.version, self.max_players, self.address = name, port, version, max_players, address
        self.sock_factory = sock_factory           # Steam: the lobby's datagrams go through this socket-like object instead of UDP
        self.clock = clock
        self.members: Dict[int, Member] = {}       # by token
        self.ready = True                          # the host is ready by pressing start; shown as ready
        self.started_at: Optional[float] = None
        self.sock: Optional[socket.socket] = None
        self.lock = threading.Lock()
        self._stop = threading.Event()
        self.thread: Optional[threading.Thread] = None
        self.on_start: Optional[Callable[[], None]] = None   # called once, after the port is free again
        self.campaign = ""                         # Continue: the campaign id and the host save the party loads (both "" = a new game)
        self.host_save = ""

    def open(self) -> Tuple[bool, str]:
        if self.sock_factory is not None:
            self.sock = self.sock_factory()
            self.sock.settimeout(0.1)
            self.port = 0
            self.thread = threading.Thread(target=self._run, name="fnvcoop-lobby-host", daemon=True)
            self.thread.start()
            return True, ""
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        try:
            s.bind((self.address, self.port))
        except OSError as e:
            s.close()
            return False, f"Cannot open the lobby on UDP port {self.port}: {e.strerror or e}. Is the game or another launcher already hosting?"
        s.settimeout(0.1)
        self.sock = s
        self.port = s.getsockname()[1]
        self.thread = threading.Thread(target=self._run, name="fnvcoop-lobby-host", daemon=True)
        self.thread.start()
        return True, ""

    # ---- roster ---------------------------------------------------------------------------------------------------------
    def roster(self) -> List[dict]:
        rows = [{"slot": 0, "name": self.name, "ready": True, "ping": 0, "host": True}]
        for m in sorted(self.members.values(), key=lambda m: m.slot):
            rows.append({"slot": m.slot, "name": m.name, "ready": m.ready, "ping": m.ping, "host": False})
        return rows

    def all_ready(self) -> bool:
        return all(m.ready for m in self.members.values()) or fnv_mut.on("lobby_start_unready")

    def view(self) -> View:
        with self.lock:
            v = View(role="host", players=self.roster(), you=0, resume=self.host_save, via="ip" if self.sock_factory is None else "steam")
            if self.started_at is not None:
                v.state = "starting" if self.sock else "started"
                v.message = "Starting the game for the whole party."
            else:
                v.state = "hosting"
                waiting = [m.name for m in self.members.values() if not m.ready]
                v.can_start = self.all_ready()
                v.message = ("Waiting for: " + ", ".join(waiting)) if waiting else (
                    "Everyone is ready." if self.members else (
                    "Waiting for friends. Give them your address." if self.sock_factory is None else
                    "Waiting for friends. They pick you under JOIN A FRIEND (connection: Steam), or click Join Game next to your name in Steam."))
            return v

    def _free_slot(self) -> Optional[int]:
        used = {m.slot for m in self.members.values()}
        for s in range(1, self.max_players):
            if s not in used:
                return s
        return None

    def handle(self, msg: dict, addr: Tuple[str, int]) -> Optional[dict]:
        """One datagram from a guest: the answer to send back (None = no answer). Called with the lock held."""
        now = self.clock()
        t = msg.get("t")
        token = msg.get("token")
        if not isinstance(token, int):
            return None
        if t == "bye":
            self.members.pop(token, None)
            return None
        if t != "hello":
            return None
        if msg.get("ver") != self.version and not fnv_mut.on("lobby_ignore_version"):
            return {"t": "refuse", "why": f"The host runs FNV Co-Op {self.version}, you run {msg.get('ver')}. Everyone needs the same release."}
        m = self.members.get(token)
        if m is None:
            if self.started_at is not None:
                return {"t": "refuse", "why": "The game is already starting. Join it once the host is in the world."}
            slot = self._free_slot()
            if slot is None:
                return {"t": "refuse", "why": f"The party is full ({self.max_players} players)."}
            name = str(msg.get("name") or "Courier")[:24]
            m = Member(slot=slot, name=name, token=token, addr=addr)
            self.members[token] = m
        m.addr, m.seen = addr, now
        m.name = str(msg.get("name") or m.name)[:24]
        m.ready = bool(msg.get("ready"))
        out = {"t": "start" if self.started_at is not None else "roster", "players": self.roster(), "you": m.slot, "ver": self.version}
        if self.campaign and self.host_save and not fnv_mut.on("lobby_drop_campaign"):
            out["campaign"], out["hostSave"] = self.campaign, self.host_save
        if "ts" in msg:
            out["echo"] = msg["ts"]
        return out

    def start(self) -> Tuple[bool, str]:
        with self.lock:
            if self.started_at is not None:
                return False, "Already starting."
            if not self.all_ready():
                return False, "Not everyone is ready."
            self.started_at = self.clock()
        return True, ""

    def _expire(self, now: float) -> None:
        for tok in [t for t, m in self.members.items() if now - m.seen > DROP_AFTER]:
            del self.members[tok]

    def _run(self) -> None:
        assert self.sock is not None
        while not self._stop.is_set():
            try:
                data, addr = self.sock.recvfrom(2048)
            except socket.timeout:
                data, addr = None, None
            except OSError:
                data, addr = None, None   # Windows reports an ICMP "port unreachable" from a gone guest as an error here
            with self.lock:
                now = self.clock()
                if data:
                    msg = unpack(data)
                    if msg:
                        reply = self.handle(msg, addr)
                        if reply is not None:
                            try:
                                self.sock.sendto(pack(reply), addr)
                            except OSError:
                                pass
                        if msg.get("t") == "hello" and isinstance(msg.get("rtt"), int):
                            m = self.members.get(msg.get("token"))
                            if m:
                                m.ping = max(0, msg["rtt"])
                if self.started_at is None:
                    self._expire(now)
                    continue
                if now - self.started_at < START_FOR:
                    continue
                self.sock.close()       # give the port to the game
                self.sock = None
                cb, self.on_start = self.on_start, None
            if cb:
                cb()
            return

    def close(self) -> None:
        self._stop.set()
        if self.thread and self.thread is not threading.current_thread():
            self.thread.join(timeout=2)
        with self.lock:
            if self.sock:
                self.sock.close()
                self.sock = None


class GuestLobby:
    def __init__(self, name: str, token: int, host: str, port: int, version: str, clock: Callable[[], float] = time.monotonic,
                 sock_factory: Optional[Callable[[], object]] = None):
        self.name, self.token, self.host, self.port, self.version = name, token, host, port, version
        self.sock_factory = sock_factory           # Steam: see HostLobby
        self.clock = clock
        self.ready = False
        self.sock: Optional[socket.socket] = None
        self.lock = threading.Lock()
        self._stop = threading.Event()
        self.thread: Optional[threading.Thread] = None
        self.players: List[dict] = []
        self.you = -1
        self.state = "joining"
        self.message = f"Calling {host}:{port} ..." if sock_factory is None else "Calling the host through Steam ..."
        self.last_heard: Optional[float] = None
        self.opened = 0.0
        self.rtt = -1
        self.on_start: Optional[Callable[[], None]] = None
        self.campaign = ""         # what the host said about a continued campaign (see start_info)
        self.host_save = ""

    @property
    def start_info(self) -> Tuple[str, str]:
        """(campaign, host save) the host continues from; ("", "") for a new game."""
        return self.campaign, self.host_save

    def open(self) -> Tuple[bool, str]:
        s = self.sock_factory() if self.sock_factory is not None else socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.settimeout(0.1)
        self.sock = s
        self.opened = self.clock()
        self.thread = threading.Thread(target=self._run, name="fnvcoop-lobby-guest", daemon=True)
        self.thread.start()
        return True, ""

    def set_ready(self, ready: bool) -> None:
        with self.lock:
            self.ready = bool(ready)

    def view(self) -> View:
        with self.lock:
            v = View(role="guest", state=self.state, players=list(self.players), you=self.you, message=self.message, resume=self.host_save,
                     via="ip" if self.sock_factory is None else "steam")
            return v

    def handle(self, msg: dict) -> Optional[str]:
        """One datagram from the host. Returns "start" when the game should start now. Called with the lock held."""
        t = msg.get("t")
        now = self.clock()
        if t == "refuse":
            self.state, self.message = "closed", str(msg.get("why") or "The host refused.")
            return None
        if t not in ("roster", "start"):
            return None
        self.last_heard = now
        if isinstance(msg.get("echo"), (int, float)):
            self.rtt = max(0, int((now - msg["echo"]) * 1000))
        if isinstance(msg.get("players"), list):
            self.players = msg["players"]
        if isinstance(msg.get("you"), int):
            self.you = msg["you"]
        camp, save = fnv_party_saves.norm_campaign(msg.get("campaign")), msg.get("hostSave")
        if camp and fnv_party_saves.valid_save_name(save):
            self.campaign, self.host_save = camp, save
        elif t == "roster" or not self.campaign:
            self.campaign, self.host_save = "", ""
        if t == "start":
            if self.state != "started":
                self.state, self.message = "started", "The host started. Starting your game."
                return "start"
            return None
        self.state = "lobby"
        waiting = [p.get("name") for p in self.players if not p.get("ready")]
        self.message = ("Waiting for: " + ", ".join(waiting)) if waiting else "Everyone is ready. Waiting for the host to start."
        return None

    def _send_hello(self) -> None:
        msg = {"t": "hello", "name": self.name, "token": self.token, "ver": self.version, "ready": self.ready, "ts": self.clock()}
        if self.rtt >= 0:
            msg["rtt"] = self.rtt
        try:
            self.sock.sendto(pack(msg), (self.host, self.port))
        except OSError:
            pass

    def _run(self) -> None:
        next_hello = 0.0
        while not self._stop.is_set():
            now = self.clock()
            with self.lock:
                if self.state in ("closed", "started"):
                    break
                if now >= next_hello:
                    self._send_hello()
                    next_hello = now + HELLO_EVERY
                silent_for = now - (self.last_heard if self.last_heard is not None else self.opened)
                if silent_for > GIVE_UP_AFTER:
                    if self.last_heard is None:
                        self.state = "closed"
                        self.message = (f"No lobby answered at {self.host}:{self.port}. Check the address, that the host pressed HOST, "
                                        "and the router forwards UDP. If the host is already in the game, use JOIN GAME IN PROGRESS."
                                        if self.sock_factory is None else
                                        "The host's lobby did not answer. Check that the host pressed HOST (connection: Steam) and that you are Steam friends. "
                                        "If the host is already in the game, use JOIN GAME IN PROGRESS.")
                    else:
                        self.state, self.message = "closed", "Lost the host. The lobby closed."
                    break
            try:
                data, _ = self.sock.recvfrom(4096)
            except (socket.timeout, OSError):
                continue
            msg = unpack(data)
            if not msg:
                continue
            with self.lock:
                go = self.handle(msg)
                cb = self.on_start if go == "start" else None
            if cb:
                cb()
                break
        with self.lock:
            if self.sock:
                self.sock.close()
                self.sock = None

    def close(self) -> None:
        with self.lock:
            if self.sock and self.state not in ("started",):
                try:
                    self.sock.sendto(pack({"t": "bye", "token": self.token}), (self.host, self.port))
                except OSError:
                    pass
        self._stop.set()
        if self.thread and self.thread is not threading.current_thread():
            self.thread.join(timeout=2)
