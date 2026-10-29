"""A pretend Steam for the launcher tests (not shipped): FakeSteam hands out Channels that behave like fnvsteam.exe, so the real lobby code
runs over a "Steam" that is just a few Python objects. See fnv_steam.py."""
import queue
from typing import Dict, List, Optional, Tuple

from fnv_steam import Channel


class FakeSteam:
    """Players on one virtual Steam. Each player() is a Channel that behaves like fnvsteam.exe: friends only, a host numbers its guests from 1,
    a guest sees its host as peer 0, datagrams are delivered at once (and a player can be cut off). Tests drive the real lobby code over it."""

    def __init__(self):
        self.players: Dict[str, "FakeSteam.Player"] = {}
        self.friend_pairs: set = set()
        self.cut: set = set()
        self.log: List[Tuple[str, dict]] = []

    def befriend(self, a: str, b: str) -> None:
        self.friend_pairs.add(frozenset((a, b)))

    def are_friends(self, a: str, b: str) -> bool:
        return frozenset((a, b)) in self.friend_pairs

    def player(self, steam_id: str, name: str = "Player", error: str = "") -> "FakeSteam.Player":
        p = FakeSteam.Player(self, steam_id, name, error)
        self.players[steam_id] = p
        return p

    class Player(Channel):
        def __init__(self, world: "FakeSteam", steam_id: str, name: str, error: str):
            self.w, self.id, self.name, self.error = world, steam_id, name, error
            self.q: "queue.Queue[Optional[dict]]" = queue.Queue()
            self.hosting: Optional[dict] = None       # {"tag", "mode"}
            self.joined: Optional[str] = None         # host id (a guest)
            self.guests: Dict[int, str] = {}          # host: peer number -> guest id
            self.closed = False
            if error:
                self.q.put({"ev": "error", "why": error})
            else:
                self.q.put({"ev": "ready", "me": steam_id, "name": name})

        # Channel
        def send(self, obj: dict) -> None:
            self.w.log.append((self.id, obj))
            c = obj.get("cmd")
            if c == "friends":
                tag = str(obj.get("tag", ""))
                rows = []
                for p in self.w.players.values():
                    if p is not self and p.hosting is not None and not p.closed and self.w.are_friends(self.id, p.id):
                        rows.append({"id": p.id, "name": p.name, "mode": p.hosting["mode"], "same": not tag or tag == p.hosting["tag"]})
                self.q.put({"ev": "friends", "list": rows})
            elif c == "host":
                self.hosting = {"tag": str(obj.get("tag", "")), "mode": str(obj.get("mode", ""))}
                self.guests.clear()
                self.joined = None
                self.q.put({"ev": "hosting"})
            elif c == "join":
                self._join(str(obj.get("id", "")))
            elif c == "send":
                self._send_to(int(obj.get("peer", 0)), obj.get("b64", ""))
            elif c == "stop":
                self._leave()
            elif c == "quit":
                self._leave()
                self.closed = True
                self.q.put(None)

        def recv(self, timeout: float) -> Optional[dict]:
            try:
                ev = self.q.get(timeout=timeout)
            except queue.Empty:
                return None
            if ev is None:
                self.q.put(None)
                raise EOFError("closed")
            return ev

        def close(self) -> None:
            self.send({"cmd": "quit"})

        # the pretend network
        def _join(self, host_id: str) -> None:
            self._leave()
            host = self.w.players.get(host_id)
            if host is None or host.hosting is None or host.closed:
                self.q.put({"ev": "link", "error": "The host's game is not running (yet)."})
                return
            if not self.w.are_friends(self.id, host_id):
                return            # a stranger gets no answer at all
            peer = 1
            while peer in host.guests:
                peer += 1
            host.guests[peer] = self.id
            self.joined = host_id
            host.q.put({"ev": "peer", "peer": peer, "state": "up", "id": self.id})
            self.q.put({"ev": "peer", "peer": 0, "state": "up", "id": host_id})

        def _send_to(self, peer: int, b64: str) -> None:
            if self.id in self.w.cut:
                return
            if self.joined is not None and peer == 0:
                target = self.w.players.get(self.joined)
                back = next((n for n, g in (target.guests.items() if target else []) if g == self.id), None)
                if target and back is not None and target.id not in self.w.cut:
                    target.q.put({"ev": "recv", "peer": back, "b64": b64})
            elif self.hosting is not None and peer in self.guests:
                target = self.w.players.get(self.guests[peer])
                if target and target.id not in self.w.cut:
                    target.q.put({"ev": "recv", "peer": 0, "b64": b64})

        def _leave(self) -> None:
            if self.joined is not None:
                host = self.w.players.get(self.joined)
                if host:
                    for n, g in list(host.guests.items()):
                        if g == self.id:
                            del host.guests[n]
                            host.q.put({"ev": "peer", "peer": n, "state": "down", "id": self.id})
                self.joined = None
            if self.hosting is not None:
                for n, g in list(self.guests.items()):
                    guest = self.w.players.get(g)
                    if guest and guest.joined == self.id:
                        guest.joined = None
                        guest.q.put({"ev": "peer", "peer": 0, "state": "down", "id": self.id})
                self.guests.clear()
                self.hosting = None

        def click_join_game(self, host_id: str) -> None:
            """The player clicked Join Game on a hosting friend while the helper runs."""
            self.q.put({"ev": "join_request", "id": host_id, "connect": "+fnvcoop_join " + host_id})
