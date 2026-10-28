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
import fnv_portcheck as pc  # noqa: E402

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
                                    "listen": "0.0.0.0", "port": "30000", "maxplayers": "3"}, h.values["coop"]
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


REAL = [check_parse_address, check_validate_name, check_build_plan, check_write_ini, check_settings_file, check_port_checks,
        check_probe_round_trip, check_probe_rejects_wrong_reply, check_start_session, check_gui_builds]

TWINS = [
    (check_parse_address, "ip_no_range"),
    (check_validate_name, "name_no_limit"),
    (check_build_plan, "no_live_dependency"),
    (check_write_ini, "no_prev_copy"),
    (check_port_checks, "port_always_ok"),
    (check_probe_round_trip, "answer_anything"),
    (check_probe_rejects_wrong_reply, "accept_any_reply"),
    (check_start_session, "skip_plugin_check"), (check_start_session, "skip_port_check"),
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
