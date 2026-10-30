#!/usr/bin/env python3
"""The co-op launcher window: Host or Join, player name, settings, and one button that starts the game.

Needs Python 3 with tkinter (the python.org installer includes it). All the logic is in fnv_launcher_core.py and
fnv_portcheck.py; this file only shows it. Nothing is downloaded and no outside service is contacted.
"""
import os
import queue
import socket
import sys
import threading
import tkinter as tk
from pathlib import Path
from tkinter import filedialog, messagebox, ttk

_HERE = os.path.dirname(os.path.abspath(__file__))
for _d in (_HERE, os.path.join(os.path.dirname(_HERE), "installer")):
    if _d not in sys.path:
        sys.path.insert(0, _d)
import fnv_find  # noqa: E402
import fnv_install  # noqa: E402
import fnv_launcher_core as core  # noqa: E402
import fnv_pe  # noqa: E402
import fnv_portcheck  # noqa: E402

TITLE = "FNV Co-Op Launcher"


class LauncherApp:
    def __init__(self, root: tk.Tk, settings: core.Settings = None, save_path: Path = None):
        self.root = root
        self.save_path = save_path
        self.s = settings or core.load_settings(save_path)
        self.listener = None
        self.results: "queue.Queue" = queue.Queue()
        root.title(TITLE)
        root.minsize(560, 640)
        self.v = {
            "game": tk.StringVar(value=self.s.game_dir), "name": tk.StringVar(value=self.s.name), "mode": tk.StringVar(value=self.s.mode),
            "address": tk.StringVar(value=self.s.address), "port": tk.StringVar(value=str(self.s.port)),
            "xp": tk.BooleanVar(value=self.s.xp_share), "slowmo": tk.BooleanVar(value=self.s.slowmo), "tether": tk.BooleanVar(value=self.s.tether),
            "actors": tk.BooleanVar(value=self.s.live_actors), "avatars": tk.BooleanVar(value=self.s.live_avatars),
            "combat": tk.BooleanVar(value=self.s.live_combat), "spawns": tk.BooleanVar(value=self.s.live_spawns),
        }
        self._build()
        if not self.v["game"].get():
            self.detect_game()
        else:
            self.refresh_status()
        root.after(150, self._poll)

    # ---- layout --------------------------------------------------------------------------------------------------------
    def _build(self):
        f = ttk.Frame(self.root, padding=12)
        f.pack(fill="both", expand=True)
        f.columnconfigure(1, weight=1)
        r = 0
        ttk.Label(f, text="Fallout: New Vegas folder").grid(row=r, column=0, sticky="w")
        ttk.Entry(f, textvariable=self.v["game"]).grid(row=r, column=1, sticky="ew", padx=6)
        bf = ttk.Frame(f)
        bf.grid(row=r, column=2)
        ttk.Button(bf, text="Browse", command=self.browse).pack(side="left")
        ttk.Button(bf, text="Find", command=self.detect_game).pack(side="left", padx=(4, 0))
        r += 1
        self.status = ttk.Label(f, text="", wraplength=520, justify="left")
        self.status.grid(row=r, column=0, columnspan=3, sticky="w", pady=(2, 8))
        r += 1
        ttk.Label(f, text="Player name").grid(row=r, column=0, sticky="w")
        ttk.Entry(f, textvariable=self.v["name"], width=26).grid(row=r, column=1, sticky="w", padx=6)
        r += 1
        mf = ttk.Frame(f)
        mf.grid(row=r, column=0, columnspan=3, sticky="w", pady=(10, 2))
        ttk.Radiobutton(mf, text="Host a game", value="host", variable=self.v["mode"], command=self._mode).pack(side="left")
        ttk.Radiobutton(mf, text="Join a friend", value="join", variable=self.v["mode"], command=self._mode).pack(side="left", padx=16)
        r += 1
        self.host_box = ttk.LabelFrame(f, text="Hosting", padding=8)
        self.host_box.grid(row=r, column=0, columnspan=3, sticky="ew")
        ttk.Label(self.host_box, text="Port (UDP)").grid(row=0, column=0, sticky="w")
        ttk.Entry(self.host_box, textvariable=self.v["port"], width=8).grid(row=0, column=1, sticky="w", padx=6)
        ttk.Button(self.host_box, text="Check port", command=self.check_port).grid(row=0, column=2, padx=4)
        self.listen_btn = ttk.Button(self.host_box, text="Start test listener", command=self.toggle_listener)
        self.listen_btn.grid(row=0, column=3, padx=4)
        ttk.Label(self.host_box, text="Press the listener button BEFORE the game starts, let a friend press Test connection, then stop it.",
                  wraplength=500, justify="left").grid(row=1, column=0, columnspan=4, sticky="w", pady=(6, 0))
        self.join_box = ttk.LabelFrame(f, text="Joining", padding=8)
        self.join_box.grid(row=r, column=0, columnspan=3, sticky="ew")
        ttk.Label(self.join_box, text="Host's address").grid(row=0, column=0, sticky="w")
        ttk.Entry(self.join_box, textvariable=self.v["address"], width=30).grid(row=0, column=1, sticky="w", padx=6)
        ttk.Button(self.join_box, text="Test connection", command=self.test_connection).grid(row=0, column=2, padx=4)
        ttk.Label(self.join_box, text="Example: 203.0.113.5 or 203.0.113.5:27960. Your host tells you the address and port.",
                  wraplength=500, justify="left").grid(row=1, column=0, columnspan=3, sticky="w", pady=(6, 0))
        r += 1
        sf = ttk.LabelFrame(f, text="Party settings", padding=8)
        sf.grid(row=r, column=0, columnspan=3, sticky="ew", pady=(10, 0))
        ttk.Checkbutton(sf, text="Share kill XP with the whole party", variable=self.v["xp"]).pack(anchor="w")
        ttk.Checkbutton(sf, text="Party slow-mo: everyone slows down together while a player uses VATS (host decides)", variable=self.v["slowmo"]).pack(anchor="w")
        ttk.Checkbutton(sf, text="Tether: story moments wait until the whole party is together", variable=self.v["tether"]).pack(anchor="w")
        r += 1
        af = ttk.LabelFrame(f, text="Sync layers (on by default; untick one only to narrow down a problem)", padding=8)
        af.grid(row=r, column=0, columnspan=3, sticky="ew", pady=(10, 0))
        for key, text in (("actors", "Enemies and NPCs"), ("avatars", "See other players as characters"),
                          ("combat", "Hits, deaths and projectiles (needs enemies and NPCs)"), ("spawns", "Spawn arbitration (needs enemies and NPCs)")):
            ttk.Checkbutton(af, text=text, variable=self.v[key]).pack(anchor="w")
        r += 1
        bb = ttk.Frame(f)
        bb.grid(row=r, column=0, columnspan=3, sticky="ew", pady=12)
        ttk.Button(bb, text="Start game", command=self.start).pack(side="left")
        ttk.Button(bb, text="Save settings only", command=self.save_only).pack(side="left", padx=8)
        ttk.Button(bb, text="Turn co-op off", command=self.coop_off).pack(side="left")
        r += 1
        self.log_box = tk.Text(f, height=9, wrap="word", state="disabled")
        self.log_box.grid(row=r, column=0, columnspan=3, sticky="nsew")
        f.rowconfigure(r, weight=1)
        self._mode()

    def _mode(self):
        if self.v["mode"].get() == "host":
            self.join_box.grid_remove()
            self.host_box.grid()
        else:
            self.host_box.grid_remove()
            self.join_box.grid()

    # ---- helpers ------------------------------------------------------------------------------------------------------
    def log(self, msg: str):
        self.log_box.configure(state="normal")
        self.log_box.insert("end", msg + "\n")
        self.log_box.see("end")
        self.log_box.configure(state="disabled")

    def collect(self) -> core.Settings:
        s = core.Settings(**vars(self.s))
        s.game_dir = self.v["game"].get().strip()
        s.name = self.v["name"].get()
        s.mode = self.v["mode"].get()
        s.address = self.v["address"].get()
        p, _ = core.parse_port(self.v["port"].get())
        s.port = p if p else core.DEFAULT_PORT
        s.xp_share, s.slowmo, s.tether = self.v["xp"].get(), self.v["slowmo"].get(), self.v["tether"].get()
        s.live_actors, s.live_avatars = self.v["actors"].get(), self.v["avatars"].get()
        s.live_combat, s.live_spawns = self.v["combat"].get(), self.v["spawns"].get()
        return s

    def remember(self, s: core.Settings):
        self.s = s
        try:
            core.save_settings(s, self.save_path)
        except OSError as e:
            self.log(f"Could not remember the settings: {e}")

    # ---- actions ------------------------------------------------------------------------------------------------------
    def browse(self):
        d = filedialog.askdirectory(title="Pick the folder that holds FalloutNV.exe")
        if d:
            self.v["game"].set(d)
            self.load_from_ini()
            self.refresh_status()

    def detect_game(self):
        cands = fnv_find.candidates()
        if not cands:
            self.status.configure(text="Could not find the game. Use Browse and pick the folder that holds FalloutNV.exe.")
            return
        self.v["game"].set(str(cands[0].path))
        if len(cands) > 1:
            self.log("More than one install found; using the first. Others: " + "; ".join(str(c.path) for c in cands[1:]))
        self.load_from_ini()
        self.refresh_status()

    def load_from_ini(self):
        game = self.v["game"].get().strip()
        if game:
            s = core.read_settings_from_ini(game, self.collect())
            self.v["mode"].set(s.mode)
            self.v["port"].set(str(s.port))
            self.v["address"].set(s.address)
            for k, a in (("xp", "xp_share"), ("slowmo", "slowmo"), ("tether", "tether"), ("actors", "live_actors"), ("avatars", "live_avatars"),
                         ("combat", "live_combat"), ("spawns", "live_spawns")):
                self.v[k].set(getattr(s, a))
            self._mode()

    def refresh_status(self):
        game = self.v["game"].get().strip()
        if not game:
            self.status.configure(text="Pick your game folder.")
            return
        problems, warnings = core.preflight(game, running=lambda: False)
        info = fnv_find.inspect_game(game)
        head = f"Game {fnv_pe.format_version(info.exe_version)}, xNVSE {'.'.join(map(str, fnv_find.nvse_triple(info.nvse_version) or ())) or 'not found'}. "
        self.status.configure(text=head + (" ".join(problems) if problems else "Ready.") + ("  " + " ".join(warnings) if warnings else ""))

    def check_port(self):
        p, why = core.parse_port(self.v["port"].get())
        if p is None:
            self.log(why)
            return
        rep = fnv_portcheck.host_report(p)
        for ln in rep["lines"]:
            self.log(ln)

    def toggle_listener(self):
        if self.listener:
            self.listener.stop()
            self.listener = None
            self.listen_btn.configure(text="Start test listener")
            self.log("Test listener stopped.")
            return
        p, why = core.parse_port(self.v["port"].get())
        if p is None:
            self.log(why)
            return
        lst = fnv_portcheck.TestListener(p, seconds=300)
        ok, why = lst.start()
        if not ok:
            self.log(why)
            return
        self.listener = lst
        self.listen_btn.configure(text="Stop test listener")
        self.log(f"Listening on UDP port {p} for 5 minutes. Ask your friend to press Test connection. Stop it before you start the game.")

    def test_connection(self):
        addr, why = core.parse_address(self.v["address"].get(), core.DEFAULT_PORT, socket.gethostbyname)
        if addr is None:
            self.log(why)
            return
        self.log(f"Testing {addr[0]}:{addr[1]} ...")
        threading.Thread(target=lambda: self.results.put(fnv_portcheck.probe(addr[0], addr[1])[1]), daemon=True).start()

    def _poll(self):
        try:
            while True:
                self.log(self.results.get_nowait())
        except queue.Empty:
            pass
        self.root.after(150, self._poll)

    def start(self):
        if self.listener:
            self.toggle_listener()   # the game needs the port itself
        s = self.collect()
        if s.mode == "join":
            s.autostart = "start"   # this window has no lobby: the host is in the world, so the game loads the start save by itself and joins (docs/party-save.md)
        r = core.start_session(s, resolver=socket.gethostbyname)
        for m in r.messages:
            self.log(m)
        if r.ok:
            self.remember(s)
        else:
            messagebox.showwarning(TITLE, "Could not start:\n\n" + "\n".join(m for m in r.messages if m.startswith("PROBLEM")))

    def save_only(self):
        s = self.collect()
        if not s.game_dir:
            self.log("Pick your game folder first.")
            return
        plan = core.build_plan(s, socket.gethostbyname)
        if not plan.ok:
            for e in plan.errors:
                self.log("PROBLEM: " + e)
            return
        self.log(f"Saved to {core.write_ini(s.game_dir, plan)}")
        self.remember(s)

    def coop_off(self):
        game = self.v["game"].get().strip()
        self.log("Co-op is off; the game starts as normal single-player." if game and core.switch_off(game) else "Nothing to switch off.")

    def close(self):
        if self.listener:
            self.listener.stop()
        self.root.destroy()


def main() -> int:
    root = tk.Tk()
    app = LauncherApp(root)
    root.protocol("WM_DELETE_WINDOW", app.close)
    root.mainloop()
    return 0


if __name__ == "__main__":
    sys.exit(main())
