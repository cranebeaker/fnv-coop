"""The launcher's Steam side (docs/steam.md): friends who are hosting, and the party lobby carried over Steam instead of UDP. Standard library only.

The launcher is Python and the lobby runs before anyone's game does, so a small helper program (fnvsteam.exe, tools/steamlink) talks to the
player's Steam client for it, under the game's own AppID, using the game's own steam_api.dll. This module starts that helper and speaks its
JSON-line protocol (the command and event lists are in tools/steamlink/main.cpp). The lobby code (fnv_lobby.py) sees an ordinary datagram
socket: SteamSocket has the few socket methods it uses (sendto, recvfrom, settimeout, close), and an address is ("steam", peer): a guest
sees its host as peer 0, a host numbers its guests from 1.

Everything is behind Channel so tests run the same code against a pretend helper (FakeSteam below) or against fnvsteam.exe in its mock mode.
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


def spawn_helper(game_dir: str, exe: Optional[Path] = None, popen=subprocess.Popen) -> Channel:
    exe = exe or helper_path()
    if exe is None:
        raise FileNotFoundError(f"{HELPER_EXE} is not in the launcher folder")
    return ProcessChannel([str(exe), "--game-dir", game_dir], popen)


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
        self.link_error = ""            # why the last connect did not work
        self.state = "off"              # off | ready | error
        self.inbox: "queue.Queue[Tuple[int, bytes]]" = queue.Queue(maxsize=2048)
        self.peers: Dict[int, str] = {}   # up peers by number -> Steam ID
        self.join_requests: List[dict] = []
        self._friends: Optional[List[dict]] = None
        self._friends_event = threading.Event()
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
        """Quit the helper (before the game starts: the game talks to Steam itself)."""
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
                              "same": bool(r.get("same", True))} for r in rows]
            self._friends_event.set()
        elif kind == "hosting":
            self._hosting.set()
        elif kind == "join_request":
            sid = str(ev.get("id", ""))
            if valid_steam_id(sid):
                with self.lock:
                    self.join_requests.append({"id": sid, "connect": str(ev.get("connect", ""))[:256]})
                    del self.join_requests[:-8]
        elif kind == "link":
            self.link_error = str(ev.get("error", ""))
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

    def host(self, tag: str, mode: str = "") -> None:
        self._hosting.clear()
        self.link_error = ""
        self._send({"cmd": "host", "tag": tag, "mode": mode})
        self._hosting.wait(2.0)

    def join(self, steam_id: str) -> None:
        self.link_error = ""
        self._send({"cmd": "join", "id": steam_id})

    def send(self, peer: int, data: bytes) -> None:
        self._send({"cmd": "send", "peer": int(peer), "b64": base64.b64encode(data).decode("ascii")})

    def stop(self) -> None:
        self._send({"cmd": "stop"})
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
    """The slice of a UDP socket the lobby uses, over a SteamLink. Closing it leaves the party (it does not end the helper)."""

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
            self.link.stop()
