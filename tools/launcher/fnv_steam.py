"""The launcher's Steam side (docs/steam.md): friends who are hosting, the party lobby carried over Steam instead of UDP, and the game's tunnel.
Standard library only.

The launcher is Python, so a small helper program (fnvsteam.exe, tools/steamlink) talks to the player's Steam client for it, as Valve's
sample app "Spacewar" (AppID 480). This module starts that helper and speaks its JSON-line protocol (the command and event lists are at the top
of tools/steamlink/main.cpp). The helper stays alive for the whole session: before the game starts it carries the lobby, afterwards it is the
UDP tunnel the game's own ENet connection runs through (a guest's game connects to 127.0.0.1:tunnel_port).

The lobby code (fnv_lobby.py) sees an ordinary datagram socket: SteamSocket has the few socket methods it uses (sendto, recvfrom, settimeout,
close), and an address is ("steam", peer): a guest sees its host as peer 0, a host numbers its guests from 1.

Everything is behind Channel so tests run the same code against a pretend helper (fnv_steam_fake.py) or against fnvsteam.exe in its mock mode.
"""
import base64
import json
import os
import queue
import re
import socket
import subprocess
import sys
import threading
import time
from pathlib import Path
from typing import Callable, Dict, List, Optional, Tuple

HELPER_EXE = "fnvsteam.exe"
READY_TIMEOUT = 20.0       # the helper has to be up and logged in to Steam by then
FRIENDS_TIMEOUT = 3.0
HOST_TIMEOUT = 8.0         # Steam has to open the lobby by then
JOIN_TIMEOUT = 15.0        # Steam has to let the guest into the host's lobby by then
PEER = "steam"             # the "host" half of an address of the lobby's socket
STEAM_ID_RE = re.compile(r"^7656119\d{10}$")   # a SteamID64 of an ordinary account: 17 digits


def valid_steam_id(text: str) -> bool:
    return bool(STEAM_ID_RE.match((text or "").strip()))


def helper_path(launcher_dir: Optional[Path] = None) -> Optional[Path]:
    """fnvsteam.exe next to the launcher (a release), or in out/steamlink (the repo)."""
    here = Path(launcher_dir) if launcher_dir else Path(__file__).resolve().parent
    for p in (here / HELPER_EXE, here.parent.parent / "out" / "steamlink" / HELPER_EXE):
        if p.is_file():
            return p
    return None


def protocol_tag(launcher_dir: Optional[Path] = None) -> str:
    """The session protocol version as a string: a friend's launcher lists a host only when it matches. From the release manifest, else wire.h."""
    here = Path(launcher_dir) if launcher_dir else Path(__file__).resolve().parent
    try:
        v = json.loads((here.parent / "manifest.json").read_text(encoding="utf-8")).get("protocol")
        if v:
            return str(v)
    except (OSError, ValueError):
        pass
    try:
        t = (here.parent.parent / "synccore" / "include" / "fnvcoop" / "wire.h").read_text(encoding="utf-8")
        m = re.search(r"kProtocolVersion\s*=\s*(\d+)", t)
        if m:
            return m.group(1)
    except OSError:
        pass
    return ""


# ---- channels: a line of JSON each way ------------------------------------------------------------------------------------------------

class Channel:
    """What SteamLink talks to: the helper process, or a pretend one. send() never blocks for long; recv() returns None after the timeout and
    raises EOFError when the other end is gone."""

    def send(self, obj: dict) -> None:
        raise NotImplementedError

    def recv(self, timeout: float) -> Optional[dict]:
        raise NotImplementedError

    def close(self) -> None:
        raise NotImplementedError


class ProcessChannel(Channel):
    def __init__(self, argv: List[str], popen=subprocess.Popen):
        flags = getattr(subprocess, "CREATE_NO_WINDOW", 0)
        self.proc = popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, creationflags=flags, close_fds=True)
        self.q: "queue.Queue[Optional[dict]]" = queue.Queue()
        self.lock = threading.Lock()
        threading.Thread(target=self._read, name="fnvcoop-steam-read", daemon=True).start()

    def _read(self) -> None:
        try:
            for raw in self.proc.stdout:
                try:
                    obj = json.loads(raw.decode("utf-8", errors="replace"))
                except ValueError:
                    continue
                if isinstance(obj, dict):
                    self.q.put(obj)
        except (OSError, ValueError):
            pass
        self.q.put(None)

    def send(self, obj: dict) -> None:
        data = (json.dumps(obj, separators=(",", ":")) + "\n").encode("utf-8")
        with self.lock:
            try:
                self.proc.stdin.write(data)
                self.proc.stdin.flush()
            except (OSError, ValueError):
                pass

    def recv(self, timeout: float) -> Optional[dict]:
        try:
            obj = self.q.get(timeout=timeout)
        except queue.Empty:
            return None
        if obj is None:
            self.q.put(None)
            raise EOFError("the Steam helper ended")
        return obj

    def close(self) -> None:
        try:
            self.send({"cmd": "quit"})
            self.proc.stdin.close()
        except (OSError, ValueError):
            pass
        try:
            self.proc.wait(timeout=2)
        except Exception:  # noqa: BLE001
            try:
                self.proc.kill()
            except OSError:
                pass


def spawn_helper(exe: Optional[Path] = None, popen=subprocess.Popen) -> Channel:
    """Start fnvsteam.exe. It looks for steam_api.dll and steam_appid.txt (480) in its own folder."""
    exe = exe or helper_path()
    if exe is None:
        raise FileNotFoundError(f"{HELPER_EXE} is not in the launcher folder")
    return ProcessChannel([str(exe)], popen)


# ---- the link ---------------------------------------------------------------------------------------------------------------------------

class SteamLink:
    """One helper, started on demand. Thread-safe: the lobby's thread sends and receives, the page's thread asks for friends."""

    def __init__(self, spawn: Callable[[], Channel], clock: Callable[[], float] = time.monotonic):
        self.spawn, self.clock = spawn, clock
        self.chan: Optional[Channel] = None
        self.lock = threading.RLock()
        self.me = ""
        self.name = ""
        self.error = ""                 # why Steam is not usable ("" = fine or not started)
        self.link_error = ""            # why the last host / join did not work, or that the host left
        self.tunnel_port = 0            # guest: the local UDP port the game connects to (0 = not joined)
        self.hosting = False            # host: the Steam lobby is open
        self._tunnel_event = threading.Event()
        self.state = "off"              # off | ready | error
        self.inbox: "queue.Queue[Tuple[int, bytes]]" = queue.Queue(maxsize=2048)
        self.peers: Dict[int, str] = {}   # up peers by number -> Steam ID
        self.join_requests: List[dict] = []
        self._friends: Optional[List[dict]] = None
        self._friends_event = threading.Event()
        self._online: Optional[List[dict]] = None
        self._online_event = threading.Event()
        self._invited: Optional[dict] = None
        self._invited_event = threading.Event()
        self._hosting = threading.Event()
        self._reader: Optional[threading.Thread] = None

    # ---- start / stop
    def start(self) -> Tuple[bool, str]:
        with self.lock:
            if self.state == "ready":
                return True, ""
            try:
                self.chan = self.spawn()
            except (OSError, FileNotFoundError) as e:
                self.state, self.error = "error", f"Cannot start the Steam helper: {e}"
                return False, self.error
            end = self.clock() + READY_TIMEOUT
            while self.clock() < end:
                try:
                    ev = self.chan.recv(0.2)
                except EOFError:
                    break
                if ev is None:
                    continue
                if ev.get("ev") == "ready":
                    self.me, self.name = str(ev.get("me", "")), str(ev.get("name", ""))
                    self.state, self.error = "ready", ""
                    self._reader = threading.Thread(target=self._read, name="fnvcoop-steam-link", daemon=True)
                    self._reader.start()
                    return True, ""
                if ev.get("ev") == "error":
                    self.state, self.error = "error", str(ev.get("why") or "Steam is not available.")
                    self._close_chan()
                    return False, self.error
            if self.state != "ready":
                self.state, self.error = "error", self.error or "Steam did not answer. Is Steam running and are you logged in?"
                self._close_chan()
            return False, self.error

    def _close_chan(self) -> None:
        chan, self.chan = self.chan, None
        if chan:
            chan.close()

    def close(self) -> None:
        """Quit the helper: the party is over (it ends the lobby and the game's tunnel)."""
        with self.lock:
            if self.chan:
                try:
                    self.chan.send({"cmd": "stop"})
                except Exception:  # noqa: BLE001
                    pass
            self._close_chan()
            self.state = "off"
            self.peers.clear()

    # ---- events
    def _read(self) -> None:
        chan = self.chan
        while chan is not None and self.chan is chan:
            try:
                ev = chan.recv(0.2)
            except EOFError:
                with self.lock:
                    if self.chan is chan:
                        self.state, self.error = "error", "The Steam helper stopped."
                return
            if ev is not None:
                self.on_event(ev)

    def on_event(self, ev: dict) -> None:
        kind = ev.get("ev")
        if kind == "recv":
            try:
                data = base64.b64decode(str(ev.get("b64", "")), validate=True)
            except ValueError:
                return
            try:
                self.inbox.put_nowait((int(ev.get("peer", 0)), data))
            except (queue.Full, ValueError):
                pass
        elif kind == "peer":
            try:
                peer = int(ev.get("peer", 0))
            except ValueError:
                return
            with self.lock:
                if ev.get("state") == "up":
                    self.peers[peer] = str(ev.get("id", ""))
                else:
                    self.peers.pop(peer, None)
        elif kind == "friends":
            rows = [r for r in ev.get("list", []) if isinstance(r, dict) and valid_steam_id(str(r.get("id", "")))]
            self._friends = [{"id": str(r["id"]), "name": str(r.get("name") or r["id"])[:40], "mode": str(r.get("mode") or "")[:64],
                              "same": bool(r.get("same", True)), "lobby": str(r.get("lobby") or "")[:24]} for r in rows]
            self._friends_event.set()
        elif kind == "online":
            rows = [r for r in ev.get("list", []) if isinstance(r, dict) and valid_steam_id(str(r.get("id", "")))]
            self._online = sorted(({"id": str(r["id"]), "name": str(r.get("name") or r["id"])[:40]} for r in rows), key=lambda r: r["name"].lower())
            self._online_event.set()
        elif kind == "invited":
            self._invited = {"id": str(ev.get("id", ""))[:24], "error": str(ev.get("error", ""))[:200]}
            self._invited_event.set()
        elif kind == "hosting":
            self.hosting = not ev.get("error")
            self.link_error = str(ev.get("error", ""))
            self._hosting.set()
        elif kind == "tunnel":
            try:
                self.tunnel_port = int(ev.get("port", 0))
            except (TypeError, ValueError):
                self.tunnel_port = 0
            self._tunnel_event.set()
        elif kind == "join_request":
            sid = str(ev.get("id", ""))
            if valid_steam_id(sid):
                with self.lock:
                    self.join_requests.append({"id": sid, "connect": str(ev.get("connect", ""))[:256], "lobby": str(ev.get("lobby", ""))[:24]})
                    del self.join_requests[:-8]
        elif kind == "link":
            self.link_error = str(ev.get("error", ""))
            self._tunnel_event.set()          # a join that is waiting for its tunnel learns it will not come
        elif kind == "error":
            with self.lock:
                self.state, self.error = "error", str(ev.get("why", "Steam stopped."))

    # ---- commands
    def _send(self, obj: dict) -> None:
        chan = self.chan
        if chan is not None:
            chan.send(obj)

    def friends(self, tag: str = "", timeout: float = FRIENDS_TIMEOUT) -> List[dict]:
        """Steam friends in the game who are hosting this release's co-op, with their names."""
        self._friends_event.clear()
        self._send({"cmd": "friends", "tag": tag})
        if not self._friends_event.wait(timeout):
            return list(self._friends or [])
        return list(self._friends or [])

    def online(self, timeout: float = FRIENDS_TIMEOUT) -> List[dict]:
        """Steam friends who are online now, by name: the ones an invite can reach."""
        self._online_event.clear()
        self._send({"cmd": "online"})
        self._online_event.wait(timeout)
        return list(self._online or [])

    def invite(self, steam_id: str, timeout: float = FRIENDS_TIMEOUT) -> str:
        """Send this friend Steam's invite into our lobby (host only). Returns "" or why not."""
        if not valid_steam_id(steam_id):
            return "That is not a Steam ID."
        self._invited_event.clear()
        self._invited = None
        self._send({"cmd": "invite", "id": steam_id})
        if not self._invited_event.wait(timeout):
            return "Steam did not answer. Is Steam running?"
        return (self._invited or {}).get("error", "")

    def host(self, tag: str, mode: str = "lobby", port: int = 0, max_players: int = 4) -> str:
        """Open the friends-only Steam lobby. The game will host on 127.0.0.1:port; the helper forwards the friends' traffic there.
        Returns "" or why Steam could not open it."""
        self._hosting.clear()
        self.link_error = ""
        self.hosting = False
        self._send({"cmd": "host", "tag": tag, "mode": mode, "port": int(port), "max": int(max_players)})
        if not self._hosting.wait(HOST_TIMEOUT):
            return "Steam did not open the lobby. Is Steam running and are you logged in?"
        return self.link_error

    def set_mode(self, mode: str) -> None:
        """"lobby" (waiting for the party) or "game" (in the world: friends may still join)."""
        self._send({"cmd": "mode", "mode": mode})

    def join(self, steam_id: str, lobby: str = "", timeout: float = JOIN_TIMEOUT) -> str:
        """Go into the lobby of this hosting friend and open the tunnel. Returns "" (then tunnel_port is the port the game connects to) or why not."""
        self._tunnel_event.clear()
        self.link_error = ""
        self.tunnel_port = 0
        self._send({"cmd": "join", "id": steam_id, "lobby": lobby})
        if not self._tunnel_event.wait(timeout):
            return "Steam did not let you into the host's lobby in time. Is the host still hosting?"
        if self.tunnel_port <= 0:
            return self.link_error or "Steam would not let you into the host's lobby."
        return ""

    def send(self, peer: int, data: bytes) -> None:
        self._send({"cmd": "send", "peer": int(peer), "b64": base64.b64encode(data).decode("ascii")})

    def stop(self) -> None:
        """Leave the lobby and close the tunnels; the helper keeps running."""
        self._send({"cmd": "stop"})
        self.tunnel_port = 0
        self.hosting = False
        with self.lock:
            self.peers.clear()
        while True:
            try:
                self.inbox.get_nowait()
            except queue.Empty:
                break

    def take_join_request(self) -> Optional[dict]:
        with self.lock:
            return self.join_requests.pop(0) if self.join_requests else None

    def socket(self) -> "SteamSocket":
        return SteamSocket(self)


class SteamSocket:
    """The slice of a UDP socket the lobby uses, over a SteamLink. Closing it only ends the lobby's datagrams: the Steam link (the party and
    the game's tunnel) lives on until the launcher ends the session (SteamLink.close)."""

    def __init__(self, link: SteamLink):
        self.link = link
        self.timeout: Optional[float] = None
        self.closed = False

    def settimeout(self, t: Optional[float]) -> None:
        self.timeout = t

    def getsockname(self) -> Tuple[str, int]:
        return (PEER, 0)

    def sendto(self, data: bytes, addr: Tuple[str, int]) -> int:
        if self.closed:
            raise OSError("closed")
        self.link.send(int(addr[1]), data)
        return len(data)

    def recvfrom(self, n: int) -> Tuple[bytes, Tuple[str, int]]:
        if self.closed:
            raise OSError("closed")
        try:
            peer, data = self.link.inbox.get(timeout=self.timeout if self.timeout is not None else 3600.0)
        except queue.Empty:
            raise socket.timeout("timed out") from None
        return data[:n], (PEER, peer)

    def close(self) -> None:
        if not self.closed:
            self.closed = True
            while True:
                try:
                    self.link.inbox.get_nowait()
                except queue.Empty:
                    break
