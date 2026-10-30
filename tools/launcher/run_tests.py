#!/usr/bin/env python3
"""Game-free tests for the co-op launcher (settings, address checks, nvcoop.ini writing, port checks, starting the game) with twins.

    python -I tools/launcher/run_tests.py          prints ALL OK when every check passes and every twin is caught

The game is never started: launch_game gets a fake popen. The window is only built (hidden) to prove it assembles. Sockets used by
the port checks are on 127.0.0.1 only.
"""
import json
import os
import shutil
import socket
import sys
import tempfile
import threading
import time
import traceback
from contextlib import contextmanager
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(REPO / "tools" / "installer"))
sys.path.insert(0, str(REPO / "tools" / "installer" / "tests"))
import fakegame as fg  # noqa: E402
import fnv_find  # noqa: E402
import fnv_ini  # noqa: E402
import fnv_install as inst  # noqa: E402
import fnv_launcher_core as core  # noqa: E402
import fnv_mut  # noqa: E402
import fnv_lobby  # noqa: E402
import fnv_party_saves as ps  # noqa: E402
import fnv_terminal as term  # noqa: E402
import fnv_portcheck as pc  # noqa: E402
import fnv_steam  # noqa: E402
import fnv_steam_fake as steamfake  # noqa: E402
fnv_portcheck = pc

SCRATCH = REPO / "out" / "release" / "tmp"


@contextmanager
def tmp():
    SCRATCH.mkdir(parents=True, exist_ok=True)
    d = tempfile.mkdtemp(prefix="l-", dir=str(SCRATCH))
    try:
        yield Path(d)
    finally:
        shutil.rmtree(d, ignore_errors=True)


@contextmanager
def ansi(name="cp1252"):
    old = fnv_ini.ini_encoding
    fnv_ini.ini_encoding = lambda: name
    try:
        yield
    finally:
        fnv_ini.ini_encoding = old


def not_running():
    return False


def rd(p):
    try:
        return Path(p).read_bytes()
    except OSError:
        return None


def installed_game(td):
    """A fake game with the plugin installed by the real installer."""
    g = fg.make_game(td / "game")
    rel = fg.make_release(td / "release")
    r = inst.install(g, rel, running=not_running, mygames=fg.make_mygames(td / "docs"))
    assert r.ok, r.lines
    return g


class FakePopen:
    def __init__(self):
        self.calls = []

    def __call__(self, argv, **kw):
        self.calls.append((argv, kw))
        return type("P", (), {"pid": 4242})()


def free_udp_port():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


# ---- input checks ----------------------------------------------------------------------------------------------------------

def check_parse_address():
    ok = {"1.2.3.4": ("1.2.3.4", 27960), " 1.2.3.4:30000 ": ("1.2.3.4", 30000), "friend.example.net:1234": ("friend.example.net", 1234),
          "localhost": ("localhost", 27960), "My-PC": ("My-PC", 27960), "10.0.0.255:65535": ("10.0.0.255", 65535)}
    for text, want in ok.items():
        got, why = core.parse_address(text)
        assert got == want and why == "", (text, got, why)
    bad = ["", "   ", "999.1.1.1", "1.2.3.256", "1.2.3.4:0", "1.2.3.4:70000", "1.2.3.4:abc", "1.2.3.4:", "http://1.2.3.4", "1.2.3.4/x", "a b",
           "[::1]:5", "::1", "fe80::1", "0.0.0.0", "255.255.255.255", "bad_host!", "-bad.com", "bad-.com", ".com", "a..b", "x" * 300]
    for text in bad:
        got, why = core.parse_address(text)
        assert got is None and why, (text, got)
    got, why = core.parse_address("nosuchhost.invalid", resolver=lambda h: (_ for _ in ()).throw(OSError("nope")))
    assert got is None and "Cannot find" in why
    got, why = core.parse_address("1.2.3.4", resolver=lambda h: (_ for _ in ()).throw(OSError("never called for a number")))
    assert got == ("1.2.3.4", 27960)
    assert core.parse_port("27960") == (27960, "") and core.parse_port("", 5) == (5, "") and core.parse_port("0")[0] is None and core.parse_port("65536")[0] is None


def check_validate_name():
    with ansi("cp1252"):
        assert core.validate_name("  Courier Six ") == ("Courier Six", "")
        assert core.validate_name("Zoë")[0] == "Zoë"
        for bad in ("", "   ", "x" * 25, "a\nb", "a\x00b", "你好"):
            n, why = core.validate_name(bad)
            assert n is None and why, bad
        assert core.validate_name("x" * 24)[0] == "x" * 24


# ---- nvcoop.ini ------------------------------------------------------------------------------------------------------------

def check_build_plan():
    with ansi():
        h = core.build_plan(core.Settings(name="Ann", mode="host", port=30000, max_players=3, xp_share=True, slowmo=True, tether=False))
        assert h.ok and h.port == 30000
        assert h.values["coop"] == {"enable": "1", "name": "Ann", "handshake": "1", "baseline": "1", "dialogue": "1", "role": "host",
                                    "transport": "enet", "via": "ip", "listen": "0.0.0.0", "port": "30000", "maxplayers": "3", "autostart": "", "party": ""}, h.values["coop"]
        assert h.values["bridge"] == {"multiInstance": "0", "instance": "0", "focusGuard": "0", "windowSide": ""}, "one copy per PC unless asked"
        sc = core.build_plan(core.Settings(name="Ann", mode="join", address="127.0.0.1", second_copy=True, autostart="start", party=2))
        assert sc.values["bridge"] == {"multiInstance": "1", "instance": "1", "focusGuard": "1", "windowSide": "right"} and sc.values["coop"]["autostart"] == "start", sc.values
        hc = core.build_plan(core.Settings(name="Ann", mode="host", second_copy=True))
        assert hc.values["bridge"]["windowSide"] == "left" and hc.values["bridge"]["focusGuard"] == "1", "two copies on one PC: host left, no pause on focus loss"
        assert hc.values["bridge"]["instance"] == "0" and hc.values["bridge"]["multiInstance"] == "1", "host and guest on one PC use different pipes"
        assert sc.values["coop"]["party"] == "2", "the lobby's size goes with the start save"
        assert core.build_plan(core.Settings(name="Ann", autostart="new")).values["coop"]["autostart"] == "", "the old New-button value is gone"
        assert core.build_plan(core.Settings(name="Ann", autostart="rm -rf")).values["coop"]["autostart"] == "", "only known autostart values"
        assert h.values["combat"] == {"xpmode": "1"} and h.values["party"] == {"slowmo": "1", "tether": "0"}
        assert h.values["live"] == {"actors": "1", "avatars": "1", "combat": "1", "spawns": "1"}, "the sync layers are on by default"
        j = core.build_plan(core.Settings(name="Bob", mode="join", address="203.0.113.5:31000", token=777))
        assert j.ok and (j.host, j.port) == ("203.0.113.5", 31000)
        assert j.values["coop"]["role"] == "client" and j.values["coop"]["host"] == "203.0.113.5" and j.values["coop"]["port"] == "31000"
        assert j.values["coop"]["token"] == "777" and "maxplayers" not in j.values["coop"] and "listen" not in j.values["coop"]
        assert j.values["combat"] == {"xpmode": "0"} and j.values["party"]["tether"] == "1"
        j2 = core.build_plan(core.Settings(name="Bob", mode="join", address="1.2.3.4", token=0))
        assert j2.values["coop"]["token"] not in ("", "0")
        for s in (core.Settings(name="", mode="host"), core.Settings(name="A", mode="join", address="nope!"), core.Settings(name="A", mode="host", port=0),
                  core.Settings(name="A", mode="host", max_players=5), core.Settings(name="A", mode="spectate")):
            p = core.build_plan(s)
            assert not p.ok and p.errors, s
        # the live layers follow the plugin's dependency rule: combat and spawns switch actors on
        off = dict(live_actors=False, live_avatars=False, live_combat=False, live_spawns=False)
        p = core.build_plan(core.Settings(name="A", **dict(off, live_combat=True)))
        assert p.values["live"] == {"actors": "1", "avatars": "0", "combat": "1", "spawns": "0"}, p.values["live"]
        p = core.build_plan(core.Settings(name="A", **dict(off, live_spawns=True, live_avatars=True)))
        assert p.values["live"]["actors"] == "1" and p.values["live"]["avatars"] == "1"


def check_write_ini():
    with tmp() as td, ansi():
        g = installed_game(td)
        path = core.ini_path(g)
        path.write_bytes(b"; keep this comment\r\n[bridge]\r\nautoload=quicksave\r\n\r\n[coop]\r\nenable=0\r\nrole=guest\r\nbaseline=0\r\n[dev]\r\nbots=1\r\n")
        before = path.read_bytes()
        s = core.Settings(name="Ann", mode="host", port=28000, xp_share=True)
        core.write_ini(g, core.build_plan(s))
        text = fnv_ini.read_text(path)
        assert "; keep this comment" in text and "autoload=quicksave" in text and "bots=1" in text, text
        assert fnv_ini.get(text, "coop", "enable") == "1" and fnv_ini.get(text, "coop", "role") == "host" and fnv_ini.get(text, "coop", "baseline") == "1"
        assert fnv_ini.get(text, "combat", "xpmode") == "1" and fnv_ini.get(text, "coop", "port") == "28000"
        assert rd(str(path) + ".prev") == before, "the previous nvcoop.ini is kept as nvcoop.ini.prev"
        assert not list(path.parent.glob("*.fnvcoop-tmp"))
        back = core.read_settings_from_ini(g, core.Settings())
        assert back.mode == "host" and back.port == 28000 and back.xp_share is True and back.tether is True and back.slowmo is False
        j = core.Settings(name="Bob", mode="join", address="1.2.3.4:31000", token=5, slowmo=True, tether=False, live_actors=True)
        core.write_ini(g, core.build_plan(j))
        back = core.read_settings_from_ini(g, core.Settings())
        assert (back.mode, back.address, back.slowmo, back.tether, back.live_actors) == ("join", "1.2.3.4:31000", True, False, True), back
        must_fail = core.build_plan(core.Settings(name="", mode="host"))
        try:
            core.write_ini(g, must_fail)
        except ValueError:
            pass
        else:
            raise AssertionError("an invalid plan must not be written")
        assert core.switch_off(g) and fnv_ini.get(fnv_ini.read_text(path), "coop", "enable") == "0"
        assert not core.switch_off(td / "nowhere")


def check_settings_file():
    with tmp() as td:
        p = td / "sub" / "launcher.json"
        s = core.load_settings(p)
        assert s.token > 0 and s.mode == "host" and s.port == 27960
        s.name, s.address, s.port, s.xp_share, s.mode = "Ann", "1.2.3.4", 29000, True, "join"
        core.save_settings(s, p)
        t = core.load_settings(p)
        assert (t.name, t.address, t.port, t.xp_share, t.mode, t.token) == ("Ann", "1.2.3.4", 29000, True, "join", s.token), t
        p.write_text("{not json")
        assert core.load_settings(p).token > 0
        p.write_text(json.dumps({"port": "abc", "name": 5, "mode": "weird", "token": 12, "unknown": 1, "xp_share": 1}))
        u = core.load_settings(p)
        assert u.port == 27960 and u.name == "Courier" and u.mode == "host" and u.token == 12 and u.xp_share is False
        assert core.config_path({"FNVCOOP_LAUNCHER_DIR": str(td)}).parent == td
        assert core.config_path({"APPDATA": str(td)}) == td / "FNVCoOp" / "launcher.json"
        assert len({core.new_token() for _ in range(20)}) == 20


# ---- ports ------------------------------------------------------------------------------------------------------------------

def check_port_checks():
    port = free_udp_port()
    ok, why = pc.bind_check(port, "127.0.0.1")
    assert ok and why == ""
    holder = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    holder.bind(("0.0.0.0", 0))
    busy = holder.getsockname()[1]
    try:
        ok, why = pc.bind_check(busy)
        assert not ok and "already in use" in why and str(busy) in why
        rep = pc.host_report(busy)
        assert rep["free"] is False and any("already in use" in ln for ln in rep["lines"]), rep
    finally:
        holder.close()
    for bad in (0, -1, 65536, "27960"):
        assert pc.bind_check(bad)[0] is False
    rep = pc.host_report(27961)
    assert any("UDP" in ln and "forward" in ln.lower() for ln in rep["lines"]) and rep["port"] == 27961
    assert any("below 1024" in ln for ln in pc.host_report(80)["lines"])
    assert isinstance(pc.local_ips(), list)


def check_probe_round_trip():
    lst = pc.TestListener(0, seconds=20, address="127.0.0.1")
    ok, why = lst.start()
    assert ok, why
    try:
        ok, msg = pc.probe("127.0.0.1", lst.port, timeout=1.0, tries=2)
        assert ok and "answered" in msg and lst.answered >= 1, msg
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)       # a stranger's packet gets no answer
        s.settimeout(0.5)
        s.sendto(b"hello", ("127.0.0.1", lst.port))
        try:
            s.recvfrom(64)
            raise AssertionError("the listener answered something that is not the test message")
        except socket.timeout:
            pass
        finally:
            s.close()
    finally:
        lst.stop()
    ok, msg = pc.probe("127.0.0.1", free_udp_port(), timeout=0.3, tries=1)
    assert not ok and msg, "nobody listens: the test must say so"
    ok, msg = pc.probe("no such host.invalid", 5)
    assert not ok and "Cannot find" in msg
    # a second listener on a taken port reports it
    a = pc.TestListener(0, address="127.0.0.1")
    assert a.start()[0]
    try:
        b = pc.TestListener(a.port, address="127.0.0.1")
        ok, why = b.start()
        assert not ok and why
    finally:
        a.stop()


def check_probe_rejects_wrong_reply():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("127.0.0.1", 0))
    sock.settimeout(0.2)
    stop = threading.Event()

    def junk():
        while not stop.is_set():
            try:
                _d, addr = sock.recvfrom(64)
                sock.sendto(b"some other service", addr)
            except OSError:
                continue
    t = threading.Thread(target=junk, daemon=True)
    t.start()
    try:
        ok, msg = pc.probe("127.0.0.1", sock.getsockname()[1], timeout=0.4, tries=1)
        assert not ok, "a reply that is not ours must not count as a working host"
    finally:
        stop.set()
        t.join(timeout=2)
        sock.close()


# ---- starting the game ------------------------------------------------------------------------------------------------------

def check_start_session():
    with tmp() as td, ansi():
        g = installed_game(td)
        path = core.ini_path(g)
        pop = FakePopen()
        free = lambda port: (True, "")   # noqa: E731
        host = core.Settings(name="Ann", mode="host", port=28123, game_dir=str(g))
        r = core.start_session(host, running=not_running, popen=pop, port_free=free)
        assert r.ok and r.pid == 4242, r.messages
        assert len(pop.calls) == 1 and pop.calls[0][0] == [str(g / "nvse_loader.exe")] and pop.calls[0][1]["cwd"] == str(g)
        text = fnv_ini.read_text(path)
        assert fnv_ini.get(text, "coop", "role") == "host" and fnv_ini.get(text, "coop", "port") == "28123" and fnv_ini.get(text, "coop", "enable") == "1"
        assert any("host" in m for m in r.messages)
        join = core.Settings(name="Bob", mode="join", address="203.0.113.9:28123", game_dir=str(g), token=99)
        r = core.start_session(join, resolver=lambda h: h, running=not_running, popen=pop, port_free=free)
        assert r.ok and len(pop.calls) == 2
        text = fnv_ini.read_text(path)
        assert fnv_ini.get(text, "coop", "role") == "client" and fnv_ini.get(text, "coop", "host") == "203.0.113.9" and fnv_ini.get(text, "coop", "token") == "99"
        # a second copy on this PC starts although the game already runs, and asks the plugin to allow it
        r = core.start_session(core.Settings(name="Cy", mode="join", address="127.0.0.1", game_dir=str(g), second_copy=True),
                               resolver=lambda h: h, running=lambda: True, popen=pop, port_free=free)
        assert r.ok and len(pop.calls) == 3, r.messages
        assert fnv_ini.get(fnv_ini.read_text(path), "bridge", "multiInstance") == "1"
        # every refusal leaves nvcoop.ini and the process list alone
        snap = rd(path)
        calls = len(pop.calls)
        cases = {
            "bad address": (core.Settings(name="Bob", mode="join", address="not an address", game_dir=str(g)), dict(running=not_running, port_free=free)),
            "no name": (core.Settings(name="", mode="host", game_dir=str(g)), dict(running=not_running, port_free=free)),
            "game running": (host, dict(running=lambda: True, port_free=free)),
            "port busy": (host, dict(running=not_running, port_free=lambda p: (False, "in use"))),
            "no folder": (core.Settings(name="Ann", mode="host"), dict(running=not_running, port_free=free)),
        }
        for label, (s, kw) in cases.items():
            r = core.start_session(s, popen=pop, **kw)
            assert not r.ok and r.messages, label
            assert rd(path) == snap and len(pop.calls) == calls, f"{label}: a refused start must change nothing"
        # a game folder without the plugin, and one with the wrong game version
        bare = fg.make_game(td / "bare")
        r = core.start_session(core.Settings(name="Ann", mode="host", game_dir=str(bare)), running=not_running, popen=pop, port_free=free)
        assert not r.ok and "Install.cmd" in "\n".join(r.messages) and not core.ini_path(bare).exists() and len(pop.calls) == calls
        old = fg.make_game(td / "old", exe_version=(1, 3, 0, 452))
        (old / "Data" / "NVSE" / "Plugins").mkdir(parents=True)
        (old / "Data" / "NVSE" / "Plugins" / "nvcoop.dll").write_bytes(b"x")
        r = core.start_session(core.Settings(name="Ann", mode="host", game_dir=str(old)), running=not_running, popen=pop, port_free=free)
        assert not r.ok and "1.3.0.452" in "\n".join(r.messages) and len(pop.calls) == calls
        # a loader that cannot be started is reported, not raised
        def boom(*a, **k):
            raise OSError("access denied")
        r = core.start_session(host, running=not_running, popen=boom, port_free=free)
        assert not r.ok and "could not start" in "\n".join(r.messages)


def check_gui_builds():
    try:
        import tkinter as tk
        root = tk.Tk()
    except Exception:   # noqa: BLE001  (no tkinter or no display: nothing to build; the logic above is what matters)
        print("      (tkinter or a display is not available; window not built)")
        return
    try:
        root.withdraw()
        import launcher_gui as gui
        with tmp() as td, ansi():
            g = installed_game(td)
            app = gui.LauncherApp(root, core.Settings(name="Ann", game_dir=str(g)), save_path=td / "l.json")
            s = app.collect()
            assert s.name == "Ann" and s.game_dir == str(g) and s.mode == "host" and s.port == 27960 and s.tether is True
            app.v["name"].set("Zed")
            app.v["mode"].set("join")
            app.v["address"].set("1.2.3.4:5000")
            app.v["xp"].set(True)
            app.save_only()
            t = fnv_ini.read_text(core.ini_path(g))
            assert fnv_ini.get(t, "coop", "name") == "Zed" and fnv_ini.get(t, "coop", "role") == "client" and fnv_ini.get(t, "combat", "xpmode") == "1"
            assert core.load_settings(td / "l.json").name == "Zed"
            app.coop_off()
            assert fnv_ini.get(fnv_ini.read_text(core.ini_path(g)), "coop", "enable") == "0"
    finally:
        root.destroy()


# ---- lobby -------------------------------------------------------------------------------------------------------------------

def wait_for(cond, seconds=8.0):
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        if cond():
            return True
        time.sleep(0.05)
    return False


def check_lobby_party_starts_together():
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", address="127.0.0.1")
    ok, why = host.open()
    assert ok, why
    started = []
    host.on_start = lambda: started.append("host")
    a = fnv_lobby.GuestLobby("Ann", 11, "127.0.0.1", host.port, "0.1.1-alpha")
    b = fnv_lobby.GuestLobby("Bob", 22, "127.0.0.1", host.port, "0.1.1-alpha")
    for gl in (a, b):
        gl.on_start = (lambda n: (lambda: started.append(n)))(gl.name)
        gl.open()
    try:
        assert wait_for(lambda: len(host.view().players) == 3), host.view()
        assert wait_for(lambda: a.view().state == "lobby" and len(a.view().players) == 3), a.view()
        assert {p["name"] for p in a.view().players} == {"Six", "Ann", "Bob"} and a.view().you in (1, 2)
        assert not host.view().can_start and not host.start()[0], "nobody starts while a guest is not ready"
        a.set_ready(True)
        b.set_ready(True)
        assert wait_for(lambda: host.view().can_start), host.view()
        assert host.start()[0]
        assert wait_for(lambda: sorted(started) == ["Ann", "Bob", "host"]), started
        assert fnv_portcheck.bind_check(host.port, "127.0.0.1")[0], "the host gives the port back to its game"
        assert a.view().state == "started" and b.view().state == "started"
    finally:
        for x in (a, b, host):
            x.close()


def check_lobby_refusals():
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", max_players=2, address="127.0.0.1")
    assert host.open()[0]
    old = fnv_lobby.GuestLobby("Old", 1, "127.0.0.1", host.port, "0.1.0-alpha")
    a = fnv_lobby.GuestLobby("Ann", 2, "127.0.0.1", host.port, "0.1.1-alpha")
    c = fnv_lobby.GuestLobby("Cy", 3, "127.0.0.1", host.port, "0.1.1-alpha")
    try:
        old.open()
        assert wait_for(lambda: old.view().state == "closed"), old.view()
        assert "0.1.0-alpha" in old.view().message and len(host.view().players) == 1, "a different release is refused"
        a.open()
        assert wait_for(lambda: a.view().state == "lobby")
        c.open()
        assert wait_for(lambda: c.view().state == "closed") and "full" in c.view().message, c.view()
        a.close()
        assert wait_for(lambda: len(host.view().players) == 1), "a guest that leaves is dropped at once"
    finally:
        for x in (old, a, c, host):
            x.close()
    lone = fnv_lobby.GuestLobby("Ann", 4, "127.0.0.1", free_udp_port(), "0.1.1-alpha")
    lone.open()
    try:
        assert wait_for(lambda: lone.view().state == "closed", fnv_lobby.GIVE_UP_AFTER + 4), lone.view()
        assert "IN-PROGRESS" in lone.view().message, "a plain lobby join that finds no lobby points at the other join mode"
    finally:
        lone.close()
    assert fnv_lobby.unpack(b"junk") is None and fnv_lobby.unpack(fnv_lobby.MAGIC + b"[1]") is None



# ---- co-op over Steam (docs/steam.md) ------------------------------------------------------------------------------------------------

HOST_ID = "76561198000000001"
ANN_ID = "76561198000000002"
BOB_ID = "76561198000000003"
EVE_ID = "76561198000000009"       # not anybody's Steam friend


def check_steam_plan():
    with tmp() as td, ansi():
        # a Steam host is a normal ENet host that listens on this PC only; the Steam helper forwards the friends to it
        h = core.build_plan(core.Settings(name="Ann", mode="host", connection="steam", port=40123, max_players=3))
        assert h.ok, h.errors
        c = h.values["coop"]
        assert c.get("transport") == "enet" and c["via"] == "steam" and c["role"] == "host" and c["maxplayers"] == "3", c
        assert c["listen"] == "127.0.0.1" and c["port"] == "40123", c
        ipc = core.build_plan(core.Settings(name="Ann", mode="host", connection="ip")).values["coop"]
        assert ipc["listen"] == "0.0.0.0" and ipc["via"] == "ip" and ipc.get("transport") == "enet", ipc
        # a Steam guest connects to the local port of the helper's tunnel, with the same ENet settings as any guest
        j = core.build_plan(core.Settings(name="Bob", mode="join", connection="steam", address=HOST_ID, token=9, tunnel_port=41234))
        c = j.values["coop"]
        assert j.ok and j.host == "127.0.0.1" and j.port == 41234, j
        assert c["host"] == "127.0.0.1" and c["port"] == "41234" and c.get("transport") == "enet" and c["via"] == "steam", c
        assert c["role"] == "client" and c["token"] == "9" and "maxplayers" not in c and "listen" not in c, c
        for bad in ("", "127.0.0.1", "7656119", HOST_ID + "1", "abc", " 12345678901234567 "):
            p = core.build_plan(core.Settings(name="Bob", mode="join", connection="steam", address=bad, tunnel_port=41234))
            assert not p.ok and "Steam ID" in " ".join(p.errors), bad
        for port in (0, 70000):
            p = core.build_plan(core.Settings(name="Bob", mode="join", connection="steam", address=HOST_ID, tunnel_port=port))
            assert not p.ok and "not connected" in " ".join(p.errors), port
        assert not core.build_plan(core.Settings(name="A", connection="carrier pigeon")).ok
        # the ini round trip: a Steam run is shown as Steam in the launcher next time, an IP run overwrites it
        g = installed_game(td)
        core.write_ini(g, j)
        s = core.read_settings_from_ini(g)
        assert s.connection == "steam" and s.mode == "join", s
        core.write_ini(g, core.build_plan(core.Settings(name="Bob", mode="join", address="203.0.113.5", token=9)))
        t = fnv_ini.read_text(core.ini_path(g))
        assert fnv_ini.get(t, "coop", "transport") == "enet" and core.read_settings_from_ini(g).connection == "ip"
        # an ini from an older release says transport=steam, which the plugin no longer accepts: the next run must overwrite it
        core.ini_path(g).write_text(fnv_ini.set_value(fnv_ini.read_text(core.ini_path(g)), "coop", "transport", "steam"), encoding="utf-8")
        core.write_ini(g, h)
        assert fnv_ini.get(fnv_ini.read_text(core.ini_path(g)), "coop", "transport") == "enet"
        # the game's port is a real local port now: a busy one stops a Steam host like an IP host
        pop = FakePopen()
        r = core.start_session(core.Settings(name="Ann", mode="host", connection="steam", game_dir=str(g)), running=not_running, popen=pop,
                               port_free=lambda port: (True, ""))
        assert r.ok and any("Steam" in m for m in r.messages), r.messages
        r = core.start_session(core.Settings(name="Ann", mode="host", connection="steam", game_dir=str(g)), running=not_running, popen=pop,
                               port_free=lambda port: (False, "in use"))
        assert not r.ok, "a Steam host needs its local port too"


def make_world():
    w = steamfake.FakeSteam()
    w.befriend(HOST_ID, ANN_ID)
    w.befriend(HOST_ID, BOB_ID)
    return w


def check_steam_link():
    w = make_world()
    host = fnv_steam.SteamLink(lambda: w.player(HOST_ID, "Host"))
    assert host.start() == (True, "") and host.me == HOST_ID and host.name == "Host" and host.state == "ready"
    ann = fnv_steam.SteamLink(lambda: w.player(ANN_ID, "Ann"))
    assert ann.start()[0]
    assert ann.friends("6") == [], "nobody is hosting yet"
    assert host.host("6", "lobby", 40123, 2) == "" and host.hosting
    rows = ann.friends("6")
    assert len(rows) == 1 and {k: rows[0][k] for k in ("id", "name", "mode", "same")} == {"id": HOST_ID, "name": "Host", "mode": "lobby", "same": True}, rows
    assert rows[0]["lobby"], "the row carries the lobby, so a join does not have to search again"
    assert ann.friends("7")[0]["same"] is False, "a host on another release is listed but marked"
    bob = fnv_steam.SteamLink(lambda: w.player(BOB_ID, "Bob"))
    eve = fnv_steam.SteamLink(lambda: w.player(EVE_ID, "Eve"))
    assert bob.start()[0] and eve.start()[0]
    assert [r["id"] for r in bob.friends("6")] == [HOST_ID]
    assert eve.friends("6") == [], "only the Steam friends of a host see it"
    # joining opens the guest's tunnel: the port the game connects to
    assert ann.tunnel_port == 0
    assert ann.join(HOST_ID, rows[0]["lobby"]) == "" and ann.tunnel_port > 0
    # datagrams: a guest is peer 0 for its host, a host numbers its guests from 1
    sa, sh = ann.socket(), host.socket()
    sa.settimeout(1.0)
    sh.settimeout(1.0)
    sa.sendto(b"hello", (fnv_steam.PEER, 0))
    data, addr = sh.recvfrom(100)
    assert data == b"hello" and addr == (fnv_steam.PEER, 1), (data, addr)
    sh.sendto(b"roster", addr)
    assert sa.recvfrom(100) == (b"roster", (fnv_steam.PEER, 0))
    sh.settimeout(0.05)
    try:
        sh.recvfrom(10)
        assert False, "nothing more to read"
    except socket.timeout:
        pass
    # a stranger who says "join" gets nowhere, and is told so; a full party says so too
    why = eve.join(HOST_ID)
    assert "not found" in why and eve.tunnel_port == 0, why
    time.sleep(0.2)
    assert eve.peers == {} and sorted(host.peers) == [1], (eve.peers, host.peers)
    assert "full" in bob.join(HOST_ID), "this host was opened for two players"
    # closing the lobby's socket does not end the Steam link: the game's tunnel still needs it
    sa.close()
    time.sleep(0.2)
    assert ann.state == "ready" and ann.tunnel_port > 0 and sorted(host.peers) == [1]
    ann.stop()
    assert wait_for(lambda: host.peers == {}, 2.0), "a guest that leaves is gone for the host"
    # a Steam that is not there says why instead of hanging
    w2 = steamfake.FakeSteam()
    bad = fnv_steam.SteamLink(lambda: w2.player(HOST_ID, "Host", error="Steam is not running or you are not logged in. Start Steam, then try again."))
    ok, why = bad.start()
    assert not ok and "Steam is not running" in why and bad.state == "error"
    assert not fnv_steam.SteamLink(lambda: (_ for _ in ()).throw(FileNotFoundError("fnvsteam.exe is not in the launcher folder"))).start()[0]
    for x in (host, ann, bob, eve):
        x.close()
    assert fnv_steam.valid_steam_id(HOST_ID) and not fnv_steam.valid_steam_id("1234") and not fnv_steam.valid_steam_id("7656119800000000x")


def steam_lobbies(w, names=("Ann", "Bob")):
    """A host lobby and two guest lobbies, all over the pretend Steam, with the links they use."""
    hl = fnv_steam.SteamLink(lambda: w.player(HOST_ID, "Six"))
    assert hl.start()[0]
    assert hl.host("6", "lobby", 40123, 4) == ""
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", sock_factory=hl.socket)
    assert host.open()[0]
    guests = []
    for name, sid, token in (("Ann", ANN_ID, 11), ("Bob", BOB_ID, 22))[:len(names)]:
        gl = fnv_steam.SteamLink((lambda i, n: lambda: w.player(i, n))(sid, name))
        assert gl.start()[0]
        assert gl.join(HOST_ID) == ""
        lob = fnv_lobby.GuestLobby(name, token, fnv_steam.PEER, 0, "0.1.1-alpha", sock_factory=gl.socket)
        guests.append((gl, lob))
    return hl, host, guests


def check_steam_lobby_party_starts_together():
    w = make_world()
    hl, host, guests = steam_lobbies(w)
    started = []
    assert "Join Game" in host.view().message and "Steam" in host.view().message, host.view().message
    host.on_start = lambda: started.append("host")
    for gl, lob in guests:
        lob.on_start = (lambda n: (lambda: started.append(n)))(lob.name)
        lob.open()
    a, b = guests[0][1], guests[1][1]
    try:
        assert wait_for(lambda: len(host.view().players) == 3), host.view()
        assert host.view().via == "steam" and a.view().via == "steam"
        assert wait_for(lambda: a.view().state == "lobby" and len(a.view().players) == 3), a.view()
        assert {p["name"] for p in a.view().players} == {"Six", "Ann", "Bob"}
        assert not host.view().can_start and not host.start()[0], "nobody starts while a guest is not ready"
        a.set_ready(True)
        b.set_ready(True)
        assert wait_for(lambda: host.view().can_start), host.view()
        assert host.start()[0]
        assert wait_for(lambda: sorted(started) == ["Ann", "Bob", "host"]), started
        assert a.view().state == "started" and b.view().state == "started"
        # the lobby is over but the Steam link is not: the game's traffic still runs through it
        assert all(g[0].state == "ready" and g[0].tunnel_port > 0 for g in guests) and hl.state == "ready" and hl.hosting
    finally:
        for x in (a, b, host):
            x.close()
        for l in [hl] + [g[0] for g in guests]:
            l.close()


def check_steam_lobby_refusals():
    w = make_world()
    hl, host, guests = steam_lobbies(w, names=("Ann",))
    ann_link, ann = guests[0]
    eve_link = fnv_steam.SteamLink(lambda: w.player(EVE_ID, "Eve"))
    assert eve_link.start()[0]
    assert "not found" in eve_link.join(HOST_ID), "a stranger is not let into the lobby"
    eve = fnv_lobby.GuestLobby("Eve", 5, fnv_steam.PEER, 0, "0.1.1-alpha", sock_factory=eve_link.socket)
    try:
        ann.open()
        assert wait_for(lambda: ann.view().state == "lobby"), ann.view()
        eve.open()
        assert wait_for(lambda: eve.view().state == "closed", fnv_lobby.GIVE_UP_AFTER + 4), eve.view()
        assert "did not answer" in eve.view().message and "Steam" in eve.view().message, eve.view().message
        assert [p["name"] for p in host.view().players] == ["Six", "Ann"], "a stranger never reaches the roster"
        # a different release is refused, over Steam like over UDP
        bob_link = fnv_steam.SteamLink(lambda: w.player(BOB_ID, "Bob"))
        assert bob_link.start()[0]
        assert bob_link.join(HOST_ID) == ""
        bob = fnv_lobby.GuestLobby("Bob", 7, fnv_steam.PEER, 0, "0.1.0-alpha", sock_factory=bob_link.socket)
        bob.open()
        assert wait_for(lambda: bob.view().state == "closed"), bob.view()
        assert "0.1.0-alpha" in bob.view().message
        bob.close()
        bob_link.stop()
        ann_link.stop()
        assert wait_for(lambda: len(host.view().players) == 1), "a guest that leaves is dropped at once"
    finally:
        for x in (ann, eve, host):
            x.close()
        for l in (hl, ann_link, eve_link):
            l.close()


def make_steam_app(td, world, who, name, docs_sub):
    g = installed_game(td / docs_sub)
    cfg = td / (docs_sub + ".json")
    core.save_settings(core.Settings(name=name, game_dir=str(g), token=int(who[-3:]), connection="steam", second_copy=True, port=free_udp_port()), cfg)
    app = term.App(settings_path=cfg, version="0.1.1-alpha")
    app.steam_spawn = lambda: world.player(who, name)
    app.documents = str(td / (docs_sub + "-docs"))
    return app, g


def check_steam_in_the_launcher():
    with tmp() as td, ansi():
        w = make_world()
        host, hg = make_steam_app(td, w, HOST_ID, "Six", "h")
        ann, ag = make_steam_app(td, w, ANN_ID, "Ann", "a")
        launched = {}

        def fake_launch(game, popen=None):
            key = "host" if str(game) == str(hg) else "ann"
            launched[key] = (host if key == "host" else ann).steam is not None      # was the helper still there when the game was started?
            return 4242
        orig_launch = core.launch_game
        core.launch_game = fake_launch
        try:
            assert host.state()["steam"]["available"] and host.state()["steam"]["state"] == "off"
            r = host.host()
            assert r["ok"], r
            assert host.state()["steam"]["state"] == "ready" and host.lobby.view().via == "steam"
            hosting = w.players[HOST_ID].hosting
            assert hosting == {"tag": host.steam_tag, "mode": "lobby", "port": host.s.port, "max": host.s.max_players}, \
                "friends see the host as joinable, with the lobby's tag; the helper learns the game's port"
            assert any("STEAM" in l for l in host.state()["log"]), host.state()["log"]
            # Ann's launcher lists the host, joins the lobby, readies
            assert ann.steam_refresh()["ok"]
            st = ann.state()["steam"]
            assert [(f["id"], f["name"], f["mode"], f["joinable"]) for f in st["friends"]] == [(HOST_ID, "Six", "lobby", True)], st
            r = ann.join_steam(HOST_ID)
            assert r["ok"], r
            assert wait_for(lambda: ann.lobby.view().state == "lobby"), ann.lobby.view()
            assert ann.ready(True)["ok"]
            assert wait_for(lambda: host.lobby.view().can_start), host.lobby.view()
            assert host.start()["ok"]
            assert wait_for(lambda: len(launched) == 2), "both games start"
            # the games run a normal ENet session: the host listens on this PC only, the guest connects to the helper's tunnel port
            ht = fnv_ini.read_text(core.ini_path(hg))
            at = fnv_ini.read_text(core.ini_path(ag))
            assert fnv_ini.get(ht, "coop", "transport") == "enet" and fnv_ini.get(ht, "coop", "role") == "host", ht
            assert fnv_ini.get(ht, "coop", "listen") == "127.0.0.1" and fnv_ini.get(ht, "coop", "port") == str(host.s.port), ht
            assert fnv_ini.get(at, "coop", "transport") == "enet" and fnv_ini.get(at, "coop", "role") == "client", at
            assert fnv_ini.get(at, "coop", "host") == "127.0.0.1" and fnv_ini.get(at, "coop", "port") == str(w.players[ANN_ID].tunnel_port) != "0", at
            assert fnv_ini.get(ht, "coop", "autostart") == "start" and fnv_ini.get(at, "coop", "autostart") == "start"
            # the helper is still there when the games start, and stays: it carries their traffic. The host's lobby now says "in the world".
            assert launched == {"host": True, "ann": True}, launched
            assert not w.players[HOST_ID].closed and not w.players[ANN_ID].closed
            assert w.players[HOST_ID].hosting["mode"] == "game"
            # ending the session ends the helper
            ann.leave()
            assert w.players[ANN_ID].closed
        finally:
            core.launch_game = orig_launch
            for app in (host, ann):
                app.leave()
        assert w.players[HOST_ID].closed


def check_steam_invite_in_the_launcher():
    """The host invites a Steam friend from the lobby screen; the friend's launcher shows the invite and one press joins that lobby."""
    with tmp() as td, ansi():
        w = make_world()
        host, hg = make_steam_app(td, w, HOST_ID, "Six", "h")
        ann, ag = make_steam_app(td, w, ANN_ID, "Ann", "a")
        try:
            assert not host.steam_invite(ANN_ID)["ok"], "no invite before the lobby is open"
            assert host.host()["ok"]
            assert ann.steam_ready()[0] is not None          # Ann's launcher is open (her helper runs)
            assert host.steam_online_refresh()["ok"]
            assert [f["id"] for f in host.state()["steam"]["online"]] == [ANN_ID], host.state()["steam"]["online"]
            r = host.steam_invite(ANN_ID)
            assert r["ok"], r
            assert any("INVITE SENT" in l for l in host.state()["log"])
            assert wait_for(lambda: (ann.state()["steam"]["join_request"] or {}).get("invite")), ann.state()["steam"]
            req = ann.state()["steam"]["join_request"]
            assert req["id"] == HOST_ID and req["lobby"], req
            r = ann.join_steam(req["id"], False, req["lobby"])
            assert r["ok"], r
            assert wait_for(lambda: ann.lobby.view().state == "lobby" and len(host.lobby.view().players) == 2), (ann.lobby.view(), host.lobby.view())
            assert not ann.state()["steam"]["join_request"], "a used invite is gone"
            assert not host.steam_invite("not-an-id")["ok"]
        finally:
            for app in (host, ann):
                app.leave()


def check_steam_join_rules():
    with tmp() as td, ansi():
        w = make_world()
        ann, ag = make_steam_app(td, w, ANN_ID, "Ann", "a")
        ann.steam_tag = "6"
        hl = fnv_steam.SteamLink(lambda: w.player(HOST_ID, "Six"))
        assert hl.start()[0]
        assert hl.host("5", "game", 40123, 4) == ""     # another release, already in the world
        try:
            assert ann.steam_refresh()["ok"]
            f = ann.state()["steam"]["friends"]
            assert len(f) == 1 and f[0]["joinable"] is False, f
            r = ann.join_steam(HOST_ID)
            assert not r["ok"] and "different release" in r["why"] and ann.lobby is None, r
            for bad in ("", "10.0.0.5", "nope"):
                assert not ann.join_steam(bad)["ok"], bad
            # the host is in the world already (no lobby): the helper joins the Steam lobby, the game starts and connects to the tunnel
            assert hl.host("6", "game", 40123, 4) == ""
            assert ann.steam_refresh()["ok"]
            pop = FakePopen()
            orig = core.launch_game
            core.launch_game = lambda game, popen=None: orig(game, pop)
            try:
                r = ann.join_steam(HOST_ID, in_game=True)
            finally:
                core.launch_game = orig
            assert r["ok"] and len(pop.calls) == 1 and ann.steam is not None and ann.steam.tunnel_port > 0, r
            at = fnv_ini.read_text(core.ini_path(ag))
            assert fnv_ini.get(at, "coop", "host") == "127.0.0.1" and fnv_ini.get(at, "coop", "port") == str(ann.steam.tunnel_port), at
            assert fnv_ini.get(at, "coop", "transport") == "enet" and fnv_ini.get(at, "coop", "via") == "steam"
            assert ann.launched == "guest"
            # nobody hosting any more: a clear message, no game started
            hl.stop()
            r = ann.join_steam(HOST_ID, in_game=True)
            assert not r["ok"] and "not found" in r["why"] and len(pop.calls) == 1, r
            # a click on Join Game next to a hosting friend shows up as a request the page offers
            assert hl.host("6", "lobby", 40123, 4) == ""
            ann2, _ = make_steam_app(td, w, ANN_ID, "Ann", "a2")
            ann2.steam_tag = "6"
            assert ann2.steam_refresh()["ok"]
            assert ann2.state()["steam"]["join_request"] is None
            ann2.steam.chan.click_join_game(HOST_ID)
            assert wait_for(lambda: ann2.state()["steam"]["join_request"] is not None)
            req = ann2.state()["steam"]["join_request"]
            assert req["id"] == HOST_ID and req["name"] == "Six"
            ann2.steam_close()
        finally:
            hl.close()
            ann.leave()


def check_steam_unavailable():
    with tmp() as td, ansi():
        w = steamfake.FakeSteam()
        app, g = make_steam_app(td, w, HOST_ID, "Six", "h")
        app.steam_spawn = lambda: w.player(HOST_ID, "Six", error="Steam is not running or you are not logged in. Start Steam, then try again.")
        r = app.host()
        assert not r["ok"] and "Steam is not running" in r["why"] and app.lobby is None, r
        assert "Steam is not running" in app.state()["steam"]["error"]
        r = app.join_steam(ANN_ID)
        assert not r["ok"] and "Steam is not running" in r["why"] and app.lobby is None
        # no helper in this copy of the launcher: a clear message, and IP still works
        app.steam_spawn = None
        orig = fnv_steam.helper_path
        fnv_steam.helper_path = lambda *a, **k: None
        try:
            assert not app.state()["steam"]["available"]
            r = app.host()
            assert not r["ok"] and "fnvsteam.exe" in r["why"], r
            app.apply_settings({"connection": "ip", "port": free_udp_port()})
            r = app.host()
            try:
                assert r["ok"] and app.lobby.view().via == "ip"
            finally:
                app.leave()
        finally:
            fnv_steam.helper_path = orig
        assert app.apply_settings({"connection": "carrier pigeon"}) and app.s.connection == "ip", "only ip or steam"
        # Steam cannot open the lobby (the helper says so): the host is told why and no lobby is left half open
        w2 = make_world()
        app2, _ = make_steam_app(td, w2, HOST_ID, "Six", "h2")
        orig_host = fnv_steam.SteamLink.host
        fnv_steam.SteamLink.host = lambda self, *a, **k: "Steam could not create the lobby (result 2)."
        try:
            r = app2.host()
            assert not r["ok"] and "could not create the lobby" in r["why"] and app2.lobby is None, r
        finally:
            fnv_steam.SteamLink.host = orig_host
            app2.leave()


def udp_socket(port=0):
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(("127.0.0.1", port))
    s.settimeout(0.05)
    return s


def udp_read(s, seconds=3.0):
    """The next datagram on s, or None."""
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        try:
            return s.recvfrom(4096)
        except socket.timeout:
            continue
        except ConnectionResetError:
            continue
    return None


def check_steam_real_helper():
    """fnvsteam.exe in its mock mode (several helpers on one PC: lobbies in a shared folder, messages over localhost UDP) carries the real lobby
    code and the game's UDP tunnel, the way it does over real Steam. Twins: the helper's FNVSTEAM_MUT switches."""
    exe = fnv_steam.helper_path()
    if exe is None:
        print("      (fnvsteam.exe is not built in out/steamlink: the full run builds it, see the Steam helper suite in tools/test_all.py)")
        return
    base = 43000 + (os.getpid() % 400) * 8
    with tmp() as td:
        mock_dir = td / "steam"
        mock_dir.mkdir()

        def mock(sid, name, friends):
            return lambda: fnv_steam.ProcessChannel([str(exe), "--mock-id", sid, "--mock-base", str(base), "--mock-dir", str(mock_dir),
                                                     "--mock-name", name, "--mock-friends", ",".join(friends)])
        hl = fnv_steam.SteamLink(mock(HOST_ID, "Six", [ANN_ID, BOB_ID]))
        al = fnv_steam.SteamLink(mock(ANN_ID, "Ann", [HOST_ID]))
        bl = fnv_steam.SteamLink(mock(BOB_ID, "Bob", [HOST_ID]))
        el = fnv_steam.SteamLink(mock(EVE_ID, "Eve", [HOST_ID]))      # Eve calls the host a friend, but the host does not
        ok = [x.start() for x in (hl, al, bl, el)]
        assert all(o[0] for o in ok), ok
        assert al.me == ANN_ID and al.name == "Ann"
        game = udp_socket()                    # the host's game: ENet would listen here
        host = ann = eve = None
        guests = {}
        try:
            assert al.friends("6") == [], "nobody is hosting yet"
            assert "lobby first" in al.invite(BOB_ID), "only a host invites"
            assert hl.host("6", "lobby", game.getsockname()[1], 3) == "" and hl.hosting
            # invites: the host's online list is its (mutual) friends; an invite reaches a friend's launcher at once, never a stranger
            assert sorted(f["id"] for f in hl.online()) == sorted([ANN_ID, BOB_ID]), hl.online()
            assert hl.invite(ANN_ID) == ""
            assert wait_for(lambda: any(r["connect"] == "invite" and r["id"] == HOST_ID for r in al.join_requests)), al.join_requests
            inv = next(r for r in al.join_requests if r["connect"] == "invite")
            assert inv["lobby"].isdigit() and inv["lobby"] != "0", inv
            assert "friends" in hl.invite(EVE_ID) and not el.join_requests, "a stranger gets no invite"
            al.join_requests.clear()
            rows = al.friends("6")
            assert [(f["id"], f["name"], f["mode"], f["same"]) for f in rows] == [(HOST_ID, "Six", "lobby", True)], rows
            assert al.friends("7")[0]["same"] is False
            assert el.friends("6") == [] and bl.friends("6")[0]["id"] == HOST_ID, "only the host's friends see the lobby"
            # a stranger is turned away, a friend gets a tunnel
            assert "not found" in el.join(HOST_ID) and el.tunnel_port == 0
            assert al.join(HOST_ID, rows[0]["lobby"]) == "" and al.tunnel_port > 0
            assert bl.join(HOST_ID) == "" and bl.tunnel_port > 0 and bl.tunnel_port != al.tunnel_port    # no lobby id given: the helper finds it
            # the party lobby runs over the same helpers
            host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", sock_factory=hl.socket)
            host.open()
            ann = fnv_lobby.GuestLobby("Ann", 11, fnv_steam.PEER, 0, "0.1.1-alpha", sock_factory=al.socket)
            ann.open()
            started = []
            host.on_start = lambda: started.append("host")
            ann.on_start = lambda: started.append("ann")
            assert wait_for(lambda: ann.view().state == "lobby" and len(host.view().players) == 2), (ann.view(), host.view())
            ann.set_ready(True)
            assert wait_for(lambda: host.view().can_start)
            assert host.start()[0]
            assert wait_for(lambda: sorted(started) == ["ann", "host"]), started
            assert [p["name"] for p in host.view().players] == ["Six", "Ann"]
            # the game starts: the host says it is in the world; the helpers carry the game's UDP
            hl.set_mode("game")
            assert wait_for(lambda: al.friends("6") and al.friends("6")[0]["mode"] == "game"), "friends can join in progress"
            ag, bg = udp_socket(), udp_socket()           # the guests' games
            guests = {"ann": ag, "bob": bg}
            ag.sendto(b"ENET-ann-1", ("127.0.0.1", al.tunnel_port))
            bg.sendto(b"ENET-bob-1", ("127.0.0.1", bl.tunnel_port))
            got = {}
            for _ in range(2):
                r = udp_read(game)
                assert r is not None, "the host game got nothing through the tunnel"
                got[r[0]] = r[1]
            assert set(got) == {b"ENET-ann-1", b"ENET-bob-1"}, got
            assert got[b"ENET-ann-1"] != got[b"ENET-bob-1"], "ENet must see each friend as its own peer (own source port)"
            assert all(a[0] == "127.0.0.1" for a in got.values())
            game.sendto(b"reply-to-ann", got[b"ENET-ann-1"])
            game.sendto(b"reply-to-bob", got[b"ENET-bob-1"])
            assert udp_read(ag)[0] == b"reply-to-ann" and udp_read(bg)[0] == b"reply-to-bob"
            # a bigger datagram and a burst both come through, in the right direction
            big = bytes(range(256)) * 5
            ag.sendto(big, ("127.0.0.1", al.tunnel_port))
            assert udp_read(game)[0] == big
            for i in range(40):
                ag.sendto(b"burst%02d" % i, ("127.0.0.1", al.tunnel_port))
            seen = set()
            for _ in range(40):
                r = udp_read(game, 2.0)
                if r is None:
                    break
                seen.add(r[0])
            assert len(seen) >= 38, f"{len(seen)} of 40 datagrams arrived"
            # nothing else reaches the host's game: the stranger never got a tunnel
            assert udp_read(game, 0.3) is None
            # a guest that leaves is gone for the host; the host leaving is told to its guests
            al.stop()
            assert wait_for(lambda: al.tunnel_port == 0)
            hl.stop()
            assert wait_for(lambda: "left" in bl.link_error), bl.link_error
        finally:
            for x in (ann, eve, host):
                if x:
                    x.close()
            for s in list(guests.values()) + [game]:
                s.close()
            for l in (hl, al, bl, el):
                l.close()


def check_steam_helper_twins():
    """The helper's own twins (FNVSTEAM_MUT): each breaks one rule and the tunnel check has to notice."""
    if fnv_steam.helper_path() is None:
        return
    for twin in ("merge", "noaccept"):
        os.environ["FNVSTEAM_MUT"] = twin
        try:
            check_steam_real_helper()
        except AssertionError:
            continue
        finally:
            os.environ.pop("FNVSTEAM_MUT", None)
        raise RuntimeError(f"helper twin {twin} was NOT caught")


# ---- party saves (docs/party-save.md) ---------------------------------------------------------------------------------------------

CAMP = "00ab12cd34ef5678"
OTHER = "ffee00112233aabb"


def write_party_save(saves, name, campaign, role="host", host_save=None, age=0):
    """A save file plus the sidecar the plugin writes next to it; `age` seconds old."""
    saves.mkdir(parents=True, exist_ok=True)
    fos = saves / (name + ".fos")
    fos.write_bytes(b"FO3SAVEGAME")
    side = saves / (name + ".fos" + ps.SIDECAR_EXT)
    side.write_text(f"{ps.SIDECAR_HEADER}\ncampaign={campaign}\nrole={role}\nhostSave={host_save or name}\n", encoding="utf-8")
    t = time.time() - age
    os.utime(fos, (t, t))
    return fos


def write_index(game, rows):
    p = Path(game).joinpath(*ps.INDEX_REL)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(ps.INDEX_HEADER + "\n" + "".join("\t".join(str(c) for c in r) + "\n" for r in rows), encoding="utf-8")


def check_party_names_and_folder():
    assert ps.norm_campaign(CAMP.upper()) == CAMP and ps.norm_campaign("0" * 16) == "" and ps.norm_campaign("xyz") == "" and ps.norm_campaign(None) == ""
    for good in ("Save 12 - Courier Goodsprings 00 04 11", "Co-op 00ab12 Quicksave", "Autosave"):
        assert ps.valid_save_name(good), good
    for bad in ("", " x", "x ", 'a"b', "a\\b", "a/b", "a:b", "a\tb", "x" * 151, "name.fos", None, "中文"):
        assert not ps.valid_save_name(bad), repr(bad)
    with tmp() as td:
        game = td / "game"
        game.mkdir()
        docs = td / "docs"
        # no Fallout.ini anywhere: the defaults, My Games\FalloutNV\Saves
        assert ps.game_save_dir(game, str(docs)) == docs / "My Games" / "FalloutNV" / "Saves"
        mg = fg.make_mygames(docs)
        assert ps.game_save_dir(game, str(docs)) == mg / "Saves"
        (mg / "Fallout.ini").write_text("[General]\nbUseMyGamesDirectory=0\nSLocalSavePath=Slots\\\n", encoding="latin-1")
        assert ps.game_save_dir(game, str(docs)) == game / "Slots", "bUseMyGamesDirectory=0 saves next to the exe, in SLocalSavePath"


def check_party_host_saves():
    with tmp() as td:
        saves = td / "Saves"
        write_party_save(saves, "Save 3 - Ann Goodsprings", CAMP, age=300)
        write_party_save(saves, "Save 4 - Ann Primm", CAMP, age=100)
        write_party_save(saves, "Save 1 - Old campaign", OTHER, age=900)
        write_party_save(saves, "Co-op 00ab12 Save 4 - Ann Primm", CAMP, role="guest", host_save="Save 4 - Ann Primm", age=50)   # a guest save is never offered
        (saves / "Plain single player save.fos").write_bytes(b"x")                                   # no sidecar: not a co-op save
        (saves / "Gone.fos.fnvparty").write_text(f"{ps.SIDECAR_HEADER}\ncampaign={CAMP}\nrole=host\n", encoding="utf-8")   # sidecar without its save
        (saves / "Broken.fos").write_bytes(b"x")
        (saves / "Broken.fos.fnvparty").write_text("garbage", encoding="utf-8")
        got = [(h.name, h.campaign) for h in ps.list_host_saves(saves)]
        assert got == [("Save 4 - Ann Primm", CAMP), ("Save 3 - Ann Goodsprings", CAMP), ("Save 1 - Old campaign", OTHER)], got
        assert ps.latest_host_save(saves).name == "Save 4 - Ann Primm"
        assert ps.latest_host_save(saves, OTHER).name == "Save 1 - Old campaign"
        assert ps.latest_host_save(saves, "1" * 16) is None and ps.latest_host_save(td / "nothing") is None


def check_party_guest_pick():
    entries = ps.parse_index(ps.INDEX_HEADER + "\n" + "\n".join([
        f"{CAMP}\tSave 3\tCo-op 00ab12 Save 3\t1000",
        f"{CAMP}\tSave 4\tCo-op 00ab12 Save 4\t2000",
        f"{CAMP}\tSave 5\tCo-op 00ab12 Save 5\t3000",
        f"{OTHER}\tSave 3\tCo-op ffee00 Save 3\t9000",
        "not an entry", f"{CAMP}\tx\ty\tnotanumber", ""]))
    assert len(entries) == 4 and ps.parse_index("junk") == [] and ps.parse_index("") == []

    def pick(camp, host, exists=lambda n: True):
        e = ps.pick_guest_save(entries, camp, host, exists)
        return e.guest_save if e else None
    assert pick(CAMP, "Save 4") == "Co-op 00ab12 Save 4", "the save made for exactly that host save"
    assert pick(CAMP, "save 3") == "Co-op 00ab12 Save 3", "names compare without case, the way Windows does"
    assert pick(CAMP, "Save 9") == "Co-op 00ab12 Save 5", "no save for that host save: the newest of the campaign"
    assert pick(OTHER, "Save 9") == "Co-op ffee00 Save 3" and pick(OTHER, "Save 3") == "Co-op ffee00 Save 3", "another campaign has its own saves"
    assert pick("1" * 16, "Save 4") is None, "no save of that campaign: none (the guest joins with the start save)"
    assert pick("", "Save 4") is None
    assert pick(CAMP, "Save 5", lambda n: n != "Co-op 00ab12 Save 5") == "Co-op 00ab12 Save 4", "a save file that is gone is skipped"
    assert pick(CAMP, "Save 4", lambda n: False) is None
    # the same through a real folder: index file in the game folder, saves in the (fake) My Games folder
    with tmp() as td:
        game, docs = td / "game", td / "docs"
        game.mkdir()
        saves = fg.make_mygames(docs) / "Saves"
        write_index(game, [(CAMP, "Save 4", "Co-op 00ab12 Save 4", 2000), (CAMP, "Save 5", "Co-op 00ab12 Save 5", 3000)])
        assert ps.guest_pick(game, CAMP, "Save 4", str(docs)) is None, "the index names saves that are not on disk"
        write_party_save(saves, "Co-op 00ab12 Save 4", CAMP, role="guest", host_save="Save 4")
        assert ps.guest_pick(game, CAMP, "Save 4", str(docs)) == "Co-op 00ab12 Save 4"
        assert ps.guest_pick(game, CAMP, "Save 5", str(docs)) == "Co-op 00ab12 Save 4", "Save 5 is gone: the newest that exists"
        assert ps.guest_pick(game, OTHER, "Save 4", str(docs)) is None


def check_lobby_continue_message():
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", address="127.0.0.1")
    host.campaign, host.host_save = CAMP, "Save 4 - Ann Primm"
    assert host.open()[0]
    a = fnv_lobby.GuestLobby("Ann", 11, "127.0.0.1", host.port, "0.1.1-alpha")
    started = []
    a.on_start = lambda: started.append(a.start_info)
    a.open()
    try:
        assert wait_for(lambda: a.view().state == "lobby"), a.view()
        assert a.view().resume == "Save 4 - Ann Primm" and host.view().resume == "Save 4 - Ann Primm", "the lobby shows what is continued"
        a.set_ready(True)
        assert wait_for(lambda: host.view().can_start) and host.start()[0]
        assert wait_for(lambda: started), started
        assert started == [(CAMP, "Save 4 - Ann Primm")], "the start message carries the campaign id and the host save"
    finally:
        a.close()
        host.close()
    # a new game carries neither, and a guest ignores a campaign it cannot use
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", address="127.0.0.1")
    assert host.open()[0]
    b = fnv_lobby.GuestLobby("Bob", 22, "127.0.0.1", host.port, "0.1.1-alpha")
    got = []
    b.on_start = lambda: got.append(b.start_info)
    b.open()
    try:
        b.set_ready(True)
        assert wait_for(lambda: host.view().can_start and len(host.view().players) == 2) and host.start()[0]
        assert wait_for(lambda: got) and got == [("", "")], got
    finally:
        b.close()
        host.close()
    g = fnv_lobby.GuestLobby("Cy", 3, "127.0.0.1", 1, "0.1.1-alpha")
    g.handle({"t": "start", "players": [], "you": 1, "campaign": "nope", "hostSave": "Save 4"})
    assert g.start_info == ("", ""), "a bad campaign id is ignored"
    g2 = fnv_lobby.GuestLobby("Cy", 3, "127.0.0.1", 1, "0.1.1-alpha")
    g2.handle({"t": "start", "players": [], "you": 1, "campaign": CAMP, "hostSave": "..\\evil/save"})
    assert g2.start_info == ("", ""), "a host save name with a path in it is ignored"


def check_continue_in_the_launcher():
    with tmp() as td, ansi():
        g = installed_game(td)
        docs = td / "docs2"
        saves = fg.make_mygames(docs) / "Saves"
        write_party_save(saves, "Save 3 - Ann Goodsprings", CAMP, age=300)
        write_party_save(saves, "Save 4 - Ann Primm", CAMP, age=100)
        write_party_save(saves, "Save 1 - Other", OTHER, age=900)
        cfg = td / "launcher.json"
        core.save_settings(core.Settings(name="Ann", game_dir=str(g), port=free_udp_port(), token=5), cfg)
        app = term.App(settings_path=cfg, version="0.1.1-alpha")
        app.documents = str(docs)
        assert [h["name"] for h in app.state()["saves"]] == ["Save 4 - Ann Primm", "Save 3 - Ann Goodsprings", "Save 1 - Other"]
        r = app.host(cont=True, save="No such save")
        assert not r["ok"] and app.lobby is None, "a save that is not on the list is refused and no lobby opens"
        r = app.host(cont=True)
        try:
            assert r["ok"], r
            assert (app.lobby.campaign, app.lobby.host_save) == (CAMP, "Save 4 - Ann Primm"), "Continue picks the newest co-op save"
            assert app.autostart_for("host", app.lobby) == "load:Save 4 - Ann Primm"
            assert app.lobby.view().resume == "Save 4 - Ann Primm"
        finally:
            app.leave()
        r = app.host(cont=True, save="Save 3 - Ann Goodsprings")
        try:
            assert r["ok"] and app.lobby.host_save == "Save 3 - Ann Goodsprings", "the host can choose from the list"
        finally:
            app.leave()
        r = app.host()
        try:
            assert r["ok"] and app.autostart_for("host", app.lobby) == "start", "HOST is still a new game from the start save"
        finally:
            app.leave()
        # the guest's side: its index decides
        guest_lobby = fnv_lobby.GuestLobby("Bob", 7, "127.0.0.1", 1, "0.1.1-alpha")
        guest_lobby.campaign, guest_lobby.host_save = CAMP, "Save 4 - Ann Primm"
        assert app.autostart_for("join", guest_lobby) == "start", "no index: the guest joins with the start save"
        write_index(g, [(CAMP, "Save 4 - Ann Primm", "Co-op 00ab12 Save 4 - Ann Primm", 2000)])
        write_party_save(saves, "Co-op 00ab12 Save 4 - Ann Primm", CAMP, role="guest", host_save="Save 4 - Ann Primm")
        assert app.autostart_for("join", guest_lobby) == "load:Co-op 00ab12 Save 4 - Ann Primm"
        guest_lobby.campaign, guest_lobby.host_save = OTHER, "Save 1 - Other"
        assert app.autostart_for("join", guest_lobby) == "start", "no save of that campaign: the start save"
        assert app.autostart_for("join", fnv_lobby.GuestLobby("Bob", 7, "127.0.0.1", 1, "x")) == "start", "a new game"
        assert app.autostart_for("join", None) == "start", "joining a game in progress with no known campaign loads the start save (never nothing)"
        assert app.autostart_for("join", None, CAMP) == "load:Co-op 00ab12 Save 4 - Ann Primm", "a known campaign: the guest's own newest save of it"
        assert app.autostart_for("join", None, OTHER) == "start", "no save of that campaign: the start save"
        assert app.autostart_for("host", None) == "", "only a guest has an in-progress start"
        # the plan writes it, one-shot like "start", and refuses a name the plugin could not load
        pop = FakePopen()
        s = core.Settings(name="Ann", mode="host", port=28123, game_dir=str(g), autostart="load:Save 4 - Ann Primm", party=3)
        r = core.start_session(s, running=not_running, popen=pop, port_free=lambda p: (True, ""))
        assert r.ok, r.messages
        t = fnv_ini.read_text(core.ini_path(g))
        assert fnv_ini.get(t, "coop", "autostart") == "load:Save 4 - Ann Primm" and not fnv_ini.get(t, "coop", "party"), "no party count for a load: nobody waits for a movie"
        s.autostart = "load:evil\\..\\path"
        assert core.build_plan(s).values["coop"]["autostart"] == ""
        s.autostart = "start"
        p = core.build_plan(s)
        assert p.values["coop"]["autostart"] == "start" and p.values["coop"]["party"] == "3"


def fake_launcher_app(td, name="Ann", sub="g"):
    """A launcher with a fake game folder: (app, game folder, list the fake game start appends to)."""
    g = installed_game(td / sub)
    cfg = td / (sub + ".json")
    core.save_settings(core.Settings(name=name, game_dir=str(g), token=77, port=free_udp_port(), second_copy=True), cfg)
    app = term.App(settings_path=cfg, version="0.1.1-alpha")
    app.documents = str(td / (sub + "-docs"))
    return app, g, []


def check_join_one_button():
    """JOIN with an IP address is one action: the lobby when there is one, else the game in progress, and either way nobody clicks in the game."""
    with tmp() as td, ansi():
        orig = core.launch_game
        try:
            # 1. the host is in its lobby: the guest lands in the lobby and no game is started
            app, g, launches = fake_launcher_app(td)
            core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
            host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", address="127.0.0.1")
            assert host.open()[0]
            try:
                assert app.join(f"127.0.0.1:{host.port}")["ok"]
                assert wait_for(lambda: app.lobby is not None and app.lobby.view().state == "lobby"), app.lobby.view()
                time.sleep(fnv_lobby.PROBE_FOR + 0.8)
                assert app.lobby is not None and app.lobby.view().state == "lobby" and not launches, "an answering lobby is never abandoned for a game start"
            finally:
                app.leave()
                host.close()
            # 2. nothing answers (the host is in the world; its game holds the port): the game starts and joins in progress, with the start save
            app, g, launches = fake_launcher_app(td, sub="g2")
            core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
            port = free_udp_port()
            t0 = time.monotonic()
            assert app.join(f"127.0.0.1:{port}")["ok"]
            assert app.lobby is not None and app.lobby.view().state == "joining" and "lobby" in app.lobby.view().message.lower()
            assert wait_for(lambda: len(launches) == 1, fnv_lobby.PROBE_FOR + 4), "no lobby answered: the game starts to join in progress"
            assert time.monotonic() - t0 >= fnv_lobby.PROBE_FOR - 0.2, "the launcher waits for the lobby before it gives up on it"
            assert app.lobby is None and app.launched == "guest"
            t = fnv_ini.read_text(core.ini_path(g))
            assert fnv_ini.get(t, "coop", "role") == "client" and fnv_ini.get(t, "coop", "port") == str(port), t
            assert fnv_ini.get(t, "coop", "autostart") == "start", "the game loads the start save by itself: no menu clicks"
            assert not fnv_ini.get(t, "coop", "party"), "a guest in progress waits for no party"
            assert any("NO LOBBY ANSWERED" in l for l in app.state()["log"]), app.state()["log"]
            assert app.s.last_join_key == f"ip:127.0.0.1:{port}"
            # 3. forced modes: "game" starts at once, "lobby" does not fall back
            app, g, launches = fake_launcher_app(td, sub="g3")
            core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
            assert app.join(f"127.0.0.1:{free_udp_port()}", "game")["ok"] and len(launches) == 1 and app.lobby is None, "forced in-progress join: no lobby is asked"
            app, g, launches = fake_launcher_app(td, sub="g4")
            core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
            assert app.join(f"127.0.0.1:{free_udp_port()}", "lobby")["ok"] and app.lobby.probe_s == 0
            time.sleep(fnv_lobby.PROBE_FOR + 1.0)
            assert not launches and app.lobby.view().state == "joining", "forced lobby join keeps waiting for the lobby"
            app.leave()
            # the player can leave while it is still looking
            app, g, launches = fake_launcher_app(td, sub="g5")
            core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
            assert app.join(f"127.0.0.1:{free_udp_port()}")["ok"]
            app.leave()
            time.sleep(fnv_lobby.PROBE_FOR + 1.0)
            assert not launches and app.lobby is None, "leaving cancels the in-progress fallback"
            # the mode setting is validated and remembered; the launcher's own bookkeeping is not settable from the page
            assert app.apply_settings({"join_mode": "lobby"})["ok"] and app.s.join_mode == "lobby"
            app.apply_settings({"join_mode": "bogus"})
            assert app.s.join_mode == "lobby"
            app.apply_settings({"last_join_key": "x"})
            assert app.s.last_join_key == "" and "last_campaign" not in app.state()["settings"]
        finally:
            core.launch_game = orig


def check_join_probe_rules():
    """The lobby side of the probe: a host whose game is starting is waited for, a real refusal is final."""
    g = fnv_lobby.GuestLobby("Ann", 1, "127.0.0.1", 1, "0.1.1-alpha", probe_s=2.5)
    g.opened = -100.0
    g.handle({"t": "refuse", "why": "The game is already starting.", "code": "starting"})
    assert g.state == "joining" and g.opened > -100.0, "the host's lobby is closing for its game: keep waiting, then join in progress"
    g.handle({"t": "refuse", "why": "The party is full (4 players)."})
    assert g.state == "closed", "a refusal with a reason ends the join"
    plain = fnv_lobby.GuestLobby("Ann", 1, "127.0.0.1", 1, "0.1.1-alpha")
    plain.handle({"t": "refuse", "why": "x", "code": "starting"})
    assert plain.state == "closed", "a plain lobby join keeps the old behaviour"
    host = fnv_lobby.HostLobby("Six", 0, "0.1.1-alpha", address="127.0.0.1")
    host.started_at = 1.0
    r = host.handle({"t": "hello", "token": 9, "ver": "0.1.1-alpha", "name": "Cy"}, ("127.0.0.1", 2))
    assert r["t"] == "refuse" and r.get("code") == "starting", r
    # no answer at all ends in "no_lobby" and calls the launcher back once
    calls = []
    q = fnv_lobby.GuestLobby("Ann", 2, "127.0.0.1", free_udp_port(), "0.1.1-alpha", probe_s=0.6)
    q.on_no_lobby = lambda: calls.append(1)
    q.open()
    try:
        assert wait_for(lambda: calls, 4) and q.view().state == "no_lobby", q.view()
        time.sleep(0.4)
        assert calls == [1]
    finally:
        q.close()


def check_join_in_progress_loads_by_itself():
    """A guest joining a game in progress loads its own save of the host's campaign when it knows it, else the start save: never the main menu."""
    with tmp() as td, ansi():
        app, g, launches = fake_launcher_app(td)
        saves = fg.make_mygames(Path(app.documents)) / "Saves"
        orig = core.launch_game
        core.launch_game = lambda game, popen=None: launches.append(str(game)) or 4242
        try:
            assert ps.last_campaign(g) == "", "nothing noted yet"
            r = app.join_direct("127.0.0.1")
            assert r["ok"] and app.s.last_join_key == "ip:127.0.0.1:27960" and launches, r
            ini = core.ini_path(g)
            t = fnv_ini.read_text(ini)
            assert fnv_ini.get(t, "coop", "autostart") == "start" and not fnv_ini.get(t, "coop", "lastcampaign"), t
            # the plugin notes the host's campaign in nvcoop.ini when the host's order arrives; the guest's party save is in its index
            fnv_ini.write_atomic(ini, fnv_ini.set_value(fnv_ini.read_text(ini), "coop", "lastcampaign", CAMP))
            assert ps.last_campaign(g) == CAMP
            write_index(g, [(CAMP, "Save 4 - Ann Primm", "Co-op 00ab12 Save 4 - Ann Primm", 2000)])
            write_party_save(saves, "Co-op 00ab12 Save 4 - Ann Primm", CAMP, role="guest", host_save="Save 4 - Ann Primm")
            app.launched = ""
            assert app.join_direct("127.0.0.1")["ok"]
            t = fnv_ini.read_text(ini)
            assert fnv_ini.get(t, "coop", "autostart") == "load:Co-op 00ab12 Save 4 - Ann Primm", "the same host again: the guest's own newest save of the campaign"
            assert fnv_ini.get(t, "coop", "lastcampaign") == CAMP, "the note survives the launch, for the next time"
            assert not fnv_ini.get(t, "coop", "party")
            # another host: the note is about the first one and is not used
            app.launched = ""
            assert app.join_direct("127.0.0.2")["ok"]
            t = fnv_ini.read_text(ini)
            assert fnv_ini.get(t, "coop", "autostart") == "start" and not fnv_ini.get(t, "coop", "lastcampaign"), t
            # the lobby route hands the campaign on for next time too, and a new game in a lobby has none yet
            lob = fnv_lobby.GuestLobby("Ann", 7, "127.0.0.1", 1, "0.1.1-alpha")
            lob.campaign, lob.host_save = CAMP, "Save 4 - Ann Primm"
            app.launched = ""
            assert app._launch("join", "127.0.0.3", lobby=lob)
            t = fnv_ini.read_text(ini)
            assert fnv_ini.get(t, "coop", "lastcampaign") == CAMP and fnv_ini.get(t, "coop", "autostart") == "load:Co-op 00ab12 Save 4 - Ann Primm", t
            lob.campaign, lob.host_save = "", ""
            app.launched = ""
            assert app._launch("join", "127.0.0.3", lobby=lob)
            t = fnv_ini.read_text(ini)
            assert not fnv_ini.get(t, "coop", "lastcampaign") and fnv_ini.get(t, "coop", "autostart") == "start", t
        finally:
            core.launch_game = orig


def check_campaign_and_chat_options():
    """The host's pick for a new campaign ([rebalance], docs/rebalance.md) and the text chat keys ([chat], docs/chat.md)."""
    with tmp() as td, ansi():
        d = core.build_plan(core.Settings(name="Ann", mode="host"))
        assert d.values["rebalance"] == {"mode": "normal", "friendlyfire": "0"}, d.values["rebalance"]
        assert d.values["chat"] == {"enabled": "1", "name": "Ann", "key": "Y"}, d.values["chat"]
        h = core.build_plan(core.Settings(name="Ann", mode="host", balanced=True, friendly_fire=True, chat=False, chat_key="f2"))
        assert h.ok and h.values["rebalance"] == {"mode": "balanced", "friendlyfire": "1"}, h.values["rebalance"]
        assert h.values["chat"] == {"enabled": "0", "name": "Ann", "key": "F2"}, h.values["chat"]
        for bad in ("", "YY", "F0", "F25", "Esc!", "\u00e9"):
            p = core.build_plan(core.Settings(name="Ann", chat_key=bad))
            assert not p.ok and any("Chat key" in e for e in p.errors), bad
        for good, want in (("t", "T"), ("7", "7"), (" F12 ", "F12"), ("f24", "F24")):
            assert core.parse_chat_key(good) == (want, ""), good
        g = installed_game(td)
        core.write_ini(g, h)
        text = fnv_ini.read_text(core.ini_path(g))
        assert fnv_ini.get(text, "rebalance", "mode") == "balanced" and fnv_ini.get(text, "chat", "key") == "F2" and fnv_ini.get(text, "chat", "name") == "Ann"
        back = core.read_settings_from_ini(g, core.Settings())
        assert (back.balanced, back.friendly_fire, back.chat, back.chat_key) == (True, True, False, "F2"), back
        core.write_ini(g, d)
        back = core.read_settings_from_ini(g, core.Settings(balanced=True, chat_key="Q"))
        assert (back.balanced, back.friendly_fire, back.chat, back.chat_key) == (False, False, True, "Y"), "the file wins over the remembered settings"


REAL = [check_parse_address, check_validate_name, check_build_plan, check_write_ini, check_settings_file, check_port_checks,
        check_probe_round_trip, check_probe_rejects_wrong_reply, check_start_session, check_gui_builds,
        check_lobby_party_starts_together, check_lobby_refusals,
        check_steam_plan, check_steam_link, check_steam_lobby_party_starts_together, check_steam_lobby_refusals, check_steam_in_the_launcher, check_steam_invite_in_the_launcher,
        check_steam_join_rules, check_steam_unavailable, check_steam_real_helper, check_steam_helper_twins,
        check_party_names_and_folder, check_party_host_saves, check_party_guest_pick, check_lobby_continue_message, check_continue_in_the_launcher,
        check_join_one_button, check_join_probe_rules, check_join_in_progress_loads_by_itself,
        check_campaign_and_chat_options]

TWINS = [
    (check_lobby_party_starts_together, "lobby_start_unready"),
    (check_lobby_refusals, "lobby_ignore_version"),
    (check_parse_address, "ip_no_range"),
    (check_validate_name, "name_no_limit"),
    (check_build_plan, "no_live_dependency"),
    (check_write_ini, "no_prev_copy"),
    (check_port_checks, "port_always_ok"),
    (check_probe_round_trip, "answer_anything"),
    (check_probe_rejects_wrong_reply, "accept_any_reply"),
    (check_party_guest_pick, "party_pick_newest_only"), (check_party_guest_pick, "party_pick_any_campaign"),
    (check_lobby_continue_message, "lobby_drop_campaign"),
    (check_start_session, "skip_plugin_check"), (check_start_session, "skip_port_check"),
    (check_steam_plan, "steam_id_any"), (check_steam_plan, "steam_transport_sticky"), (check_steam_plan, "steam_listen_all"),
    (check_steam_in_the_launcher, "steam_close_at_start"), (check_steam_join_rules, "steam_ignore_release"),
    (check_campaign_and_chat_options, "balanced_ignored"), (check_campaign_and_chat_options, "chat_no_name"),
    (check_campaign_and_chat_options, "chat_key_any"),
    (check_join_one_button, "join_no_probe"), (check_join_in_progress_loads_by_itself, "inprogress_loads_nothing"),
    (check_join_in_progress_loads_by_itself, "inprogress_ignores_campaign"),
]


def real_game_fingerprint():
    """Names, sizes and times in the plugin folder of every real game folder on this PC, plus whether FNVCoOp exists. The tests must not change it."""
    out = {}
    try:
        for c in fnv_find.candidates():
            pl = c.path / "Data" / "NVSE" / "Plugins"
            out[str(c.path)] = (sorted((p.name, p.stat().st_size, p.stat().st_mtime_ns) for p in pl.glob("*")) if pl.is_dir() else None,
                                (c.path / "FNVCoOp").exists())
    except OSError:
        pass
    return out


def main() -> int:
    bad = []
    n = 0
    fingerprint = real_game_fingerprint()
    for fn in REAL:
        n += 1
        try:
            fn()
            print("ok   ", fn.__name__)
        except Exception:  # noqa: BLE001
            bad.append(fn.__name__)
            print("FAIL ", fn.__name__)
            traceback.print_exc()
    for fn, mut in TWINS:
        n += 1
        try:
            with fnv_mut.use(mut):
                fn()
        except AssertionError:
            print(f"ok    twin {fn.__name__}[{mut}] caught")
        except Exception:  # noqa: BLE001
            bad.append(f"{fn.__name__}[{mut}]")
            print(f"FAIL  twin {fn.__name__}[{mut}] crashed instead of failing an assertion")
            traceback.print_exc()
        else:
            bad.append(f"{fn.__name__}[{mut}]")
            print(f"FAIL  twin {fn.__name__}[{mut}] NOT caught")
    if real_game_fingerprint() != fingerprint:
        bad.append("REAL GAME FOLDER CHANGED")
        print("FAIL  the tests changed a real game folder on this PC (plugin folder or FNVCoOp)")
    print(f"\n{n} checks ({len(REAL)} real, {len(TWINS)} twins), {len(bad)} failed")
    if bad:
        print("FAILED:", ", ".join(bad))
        return 1
    print("ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
