"""Off Windows, provision finds the game and its Proton prefix by itself (steamlib.py), so a Steam Deck needs neither
FNV_GAME_DIR nor FNV_DOCUMENTS_DIR. The fake home has the layout of a Deck with the game on an SD card library: the
Steam root lists the card in libraryfolders.vdf, the card has the game's appmanifest, the game and its prefix. A stale
prefix in the Steam root (the game was moved) must not be picked.
Twins: libraryfolders.vdf ignored, the prefix taken from the first library instead of the game's, the Windows
Program Files default used off Windows, an old folder in an earlier library beating the manifest, FNV_GAME_DIR
without a prefix falling back to the found install's INIs, and `steamlib.py documents` ignoring FNV_DOCUMENTS_DIR
must each fail the check.
Run: python -I tools/runner/tests/test_steamlib.py
"""
from __future__ import annotations

import contextlib
import io
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import instances  # noqa: E402
import mygames  # noqa: E402
import steamlib  # noqa: E402

DOCS = ("pfx", "drive_c", "users", "steamuser", "Documents")


def vdf_path(p: Path) -> str:
    return str(p).replace("\\", "\\\\")   # Steam escapes backslashes; a Linux path has none


class Deck:
    """home/.local/share/Steam (root, a stale prefix) and card/ (the game, its appmanifest and its prefix)."""

    def __init__(self, base: Path, appmanifest: bool = True):
        self.home = base / "home"
        self.root = self.home / ".local" / "share" / "Steam"
        self.card = base / "card"
        (self.root / "steamapps").mkdir(parents=True)
        (self.root / "steamapps" / "libraryfolders.vdf").write_text(
            '"libraryfolders"\n{\n\t"0"\n\t{\n\t\t"path"\t\t"%s"\n\t}\n\t"1"\n\t{\n\t\t"path"\t\t"%s"\n\t}\n}\n'
            % (vdf_path(self.root), vdf_path(self.card)), encoding="utf-8")
        self.stale_docs = self.root / "steamapps" / "compatdata" / "22380" / Path(*DOCS)
        self.stale_docs.mkdir(parents=True)
        self.game = self.card / "steamapps" / "common" / "FNV Custom"
        self.game.mkdir(parents=True)
        (self.game / "FalloutNV.exe").write_bytes(b"MZ")
        if appmanifest:
            (self.card / "steamapps" / "appmanifest_22380.acf").write_text(
                '"AppState"\n{\n\t"appid"\t\t"22380"\n\t"installdir"\t\t"FNV Custom"\n}\n', encoding="utf-8")
        self.docs = self.card / "steamapps" / "compatdata" / "22380" / Path(*DOCS)
        (self.docs / "My Games" / "FalloutNV").mkdir(parents=True)


def deck_errors(deck: Deck) -> list:
    """What the runner gets wrong on the fake Deck with no FNV_GAME_DIR / FNV_DOCUMENTS_DIR set."""
    errs = []
    with mock.patch.object(Path, "home", lambda *a: deck.home), mock.patch.object(steamlib, "OFF_WINDOWS", True), \
            mock.patch.dict(os.environ):
        os.environ.pop("FNV_GAME_DIR", None)
        os.environ.pop("FNV_DOCUMENTS_DIR", None)
        game, docs = instances.game_dir(), mygames.documents_dir()
    if game != deck.game:
        errs.append(f"game dir {game}, want the card's {deck.game}")
    if docs != deck.docs:
        errs.append(f"Documents {docs}, want the game's prefix {deck.docs}" + (" (the stale one)" if docs == deck.stale_docs else ""))
    return errs


def add_old_root_game(deck: Deck) -> None:
    """What a moved game can leave behind: "Fallout New Vegas" with FalloutNV.exe in the Steam root, which has no manifest."""
    old = deck.root / "steamapps" / "common" / "Fallout New Vegas"
    old.mkdir(parents=True)
    (old / "FalloutNV.exe").write_bytes(b"MZ")


def env_game_documents(deck: Deck) -> Path:
    """mygames.documents_dir() on the fake Deck with FNV_GAME_DIR set to another install that has no prefix."""
    other = deck.home.parent / "elsewhere" / "steamapps" / "common" / "Fallout New Vegas"
    other.mkdir(parents=True, exist_ok=True)
    with mock.patch.object(Path, "home", lambda *a: deck.home), mock.patch.object(steamlib, "OFF_WINDOWS", True), \
            mock.patch.dict(os.environ, {"FNV_GAME_DIR": str(other)}):
        os.environ.pop("FNV_DOCUMENTS_DIR", None)
        return mygames.documents_dir()


def run_cli(home: Path, argv: list, env: dict = None, main=None):
    """steamlib.main(argv) (or `main`) with only the FNV_* variables in `env` set: (exit code, stdout)."""
    out = io.StringIO()
    with mock.patch.object(Path, "home", lambda *a: home), mock.patch.dict(os.environ), \
            contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
        os.environ.pop("FNV_GAME_DIR", None)
        os.environ.pop("FNV_DOCUMENTS_DIR", None)
        os.environ.update(env or {})
        code = (main or steamlib.main)(argv)
    return code, out.getvalue()


def cli_documents_override_errors(deck: Deck, main=None) -> list:
    """What `steamlib.py documents` gets wrong with FNV_DOCUMENTS_DIR set: it must print the folder provision reads."""
    own = deck.home.parent / "own docs"
    (own / "My Games" / "FalloutNV").mkdir(parents=True, exist_ok=True)
    got = run_cli(deck.home, ["documents"], {"FNV_DOCUMENTS_DIR": str(own)}, main)
    with mock.patch.object(Path, "home", lambda *a: deck.home), mock.patch.dict(os.environ, {"FNV_DOCUMENTS_DIR": str(own)}):
        want = mygames.documents_dir()
    return [] if got == (0, f"{want}\n") else [f"documents printed {got}, provision reads {want}"]


class SteamLib(unittest.TestCase):
    def setUp(self):
        td = tempfile.TemporaryDirectory()
        self.addCleanup(td.cleanup)
        self.base = Path(td.name)

    def test_deck_finds_game_and_prefix(self):
        self.assertEqual(deck_errors(Deck(self.base)), [])

    def test_without_appmanifest_the_common_folder_names_still_count(self):
        d = Deck(self.base, appmanifest=False)
        (d.game.parent / "Fallout New Vegas").mkdir()
        (d.game.parent / "Fallout New Vegas" / "FalloutNV.exe").write_bytes(b"MZ")
        self.assertEqual(steamlib.find_game(d.home), d.game.parent / "Fallout New Vegas")

    def test_linked_root_is_listed_once(self):
        d = Deck(self.base)
        (d.home / ".steam").mkdir()
        try:
            os.symlink(d.root, d.home / ".steam" / "steam", target_is_directory=True)
        except (OSError, NotImplementedError):
            self.skipTest("no symlinks here")
        self.assertEqual(steamlib.steam_roots(d.home), [d.home / ".steam" / "steam"])
        self.assertEqual(len(steamlib.libraries(d.home)), 2)

    def test_flatpak_root(self):
        home = self.base / "home"
        root = home / ".var" / "app" / "com.valvesoftware.Steam" / ".local" / "share" / "Steam"
        game = root / "steamapps" / "common" / "Fallout New Vegas"
        game.mkdir(parents=True)
        (game / "FalloutNV.exe").write_bytes(b"MZ")
        self.assertEqual(steamlib.find_game(home), game)

    def test_env_overrides_still_win(self):
        d = Deck(self.base)
        other = self.base / "other"
        with mock.patch.object(Path, "home", lambda *a: d.home), mock.patch.object(steamlib, "OFF_WINDOWS", True), \
                mock.patch.dict(os.environ, {"FNV_GAME_DIR": str(other), "FNV_DOCUMENTS_DIR": str(other / "Docs")}):
            self.assertEqual(instances.game_dir(), other)
            self.assertEqual(mygames.documents_dir(), (other / "Docs").resolve())

    def test_game_dir_env_points_at_its_own_prefix(self):
        """FNV_GAME_DIR set, FNV_DOCUMENTS_DIR not: the Documents of that install's library."""
        d = Deck(self.base)
        with mock.patch.object(Path, "home", lambda *a: self.base / "nobody"), mock.patch.object(steamlib, "OFF_WINDOWS", True), \
                mock.patch.dict(os.environ, {"FNV_GAME_DIR": str(d.game)}):
            os.environ.pop("FNV_DOCUMENTS_DIR", None)
            self.assertEqual(mygames.documents_dir(), d.docs)

    def test_no_steam_keeps_the_old_fallbacks(self):
        home = self.base / "empty"
        home.mkdir()
        with mock.patch.object(Path, "home", lambda *a: home), mock.patch.object(steamlib, "OFF_WINDOWS", True), \
                mock.patch.dict(os.environ):
            os.environ.pop("FNV_GAME_DIR", None)
            os.environ.pop("FNV_DOCUMENTS_DIR", None)
            self.assertEqual(instances.game_dir(), Path(instances.DEFAULT_GAME))
            self.assertEqual(mygames.documents_dir(), home / "Documents")

    def test_windows_does_not_look_in_steam_libraries(self):
        d = Deck(self.base)
        with mock.patch.object(Path, "home", lambda *a: d.home), mock.patch.object(steamlib, "OFF_WINDOWS", False), \
                mock.patch.object(steamlib, "find_game", lambda *a, **k: self.fail("Steam libraries searched on Windows")), \
                mock.patch.object(mygames, "shell_documents", lambda: Path("W:/Docs")), mock.patch.dict(os.environ):
            os.environ.pop("FNV_GAME_DIR", None)
            os.environ.pop("FNV_DOCUMENTS_DIR", None)
            self.assertEqual(instances.game_dir(), Path(instances.DEFAULT_GAME))
            self.assertEqual(mygames.documents_dir(), Path("W:/Docs"))

    def test_manifest_beats_an_old_folder_in_an_earlier_library(self):
        """The game moved to the card and left "Fallout New Vegas/FalloutNV.exe" in the root, which is listed first (review)."""
        d = Deck(self.base)
        add_old_root_game(d)
        self.assertEqual(deck_errors(d), [])

    def test_game_dir_env_without_a_prefix_takes_no_other_installs_inis(self):
        """FNV_GAME_DIR names an install that never ran under Proton: the old home Documents fallback, not the Deck's (review)."""
        d = Deck(self.base)
        docs = env_game_documents(d)
        self.assertNotIn(docs, (d.docs, d.stale_docs))
        self.assertEqual(docs, d.home / "Documents")

    def test_cli_prints_the_paths(self):
        d = Deck(self.base)
        for what, want in (("game", d.game), ("prefix", d.docs.parents[3]), ("documents", d.docs)):
            with self.subTest(what=what):
                self.assertEqual(run_cli(d.home, [what]), (0, f"{want}\n"))
        self.assertEqual(run_cli(self.base / "nobody", ["game"])[0], 1)

    def test_cli_documents_follows_fnv_documents_dir(self):
        """The doc's save copy reads from `documents`: with FNV_DOCUMENTS_DIR set, that folder, as provision (review)."""
        d = Deck(self.base)
        self.assertEqual(cli_documents_override_errors(d), [])
        self.assertEqual(run_cli(d.home, ["documents"], {"FNV_DOCUMENTS_DIR": "relative/docs"})[0], 1)
        self.assertEqual(run_cli(d.home, ["documents"], {"FNV_DOCUMENTS_DIR": str(self.base / "missing")})[0], 1)

    def test_cli_runs_as_the_doc_runs_it(self):
        """`python -I tools/runner/steamlib.py ...` as a real process: -I leaves tools/runner off sys.path."""
        d = Deck(self.base)
        env = {k: v for k, v in os.environ.items() if not k.startswith("FNV_")}
        home = {"HOME": str(d.home), "USERPROFILE": str(d.home)}   # Path.home(): HOME off Windows, USERPROFILE on it
        for what, extra, want in (("game", home, d.game),
                                  ("documents", {"FNV_DOCUMENTS_DIR": str(d.docs)}, d.docs.resolve())):
            with self.subTest(what=what):
                r = subprocess.run([sys.executable, "-I", str(HERE.parent / "steamlib.py"), what], env={**env, **extra},
                                   capture_output=True, text=True, timeout=60)
                self.assertEqual((r.returncode, r.stdout, r.stderr), (0, f"{want}\n", ""))

    # ---- twins
    def test_twin_cli_documents_ignoring_the_override_is_caught(self):
        def prefix_only(argv):   # the first version: always the prefix's Documents
            print(steamlib.prefix_documents(steamlib.find_game()))
            return 0
        self.assertNotEqual(cli_documents_override_errors(Deck(self.base), prefix_only), [])

    def test_twin_first_library_folder_name_before_manifests_is_caught(self):
        def one_pass(home=None):   # the first version: each library's manifest, then its folder names, library by library
            for lib in steamlib.libraries(home):
                inst = steamlib.fnv_find.parse_appmanifest_installdir(steamlib._read(lib / "steamapps" / "appmanifest_22380.acf"))
                for name in ([inst] if inst else []) + list(steamlib.fnv_find.STEAM_NAMES):
                    if (lib / "steamapps" / "common" / name / "FalloutNV.exe").is_file():
                        return lib / "steamapps" / "common" / name
            return None
        d = Deck(self.base)
        add_old_root_game(d)
        with mock.patch.object(steamlib, "find_game", one_pass):
            self.assertNotEqual(deck_errors(d), [])

    def test_twin_env_game_falling_back_to_the_found_install_is_caught(self):
        def with_fallback(home=None, game=None):   # the first version: FNV_GAME_DIR's prefix, then the found install's
            for g in ([Path(game)] if game else []) + [steamlib.find_game(home)]:
                if g and steamlib.prefix_documents(g).is_dir():
                    return steamlib.prefix_documents(g)
            return None
        d = Deck(self.base)
        with mock.patch.object(steamlib, "find_documents", with_fallback):
            self.assertIn(env_game_documents(d), (d.docs, d.stale_docs))   # what the check above rejects

    def test_twin_libraryfolders_ignored_is_caught(self):
        with mock.patch.object(steamlib.fnv_find, "parse_libraryfolders", lambda text: []):
            self.assertNotEqual(deck_errors(Deck(self.base)), [])

    def test_twin_prefix_from_the_first_library_is_caught(self):
        d = Deck(self.base)
        with mock.patch.object(steamlib, "prefix_documents", lambda game: d.stale_docs):
            errs = deck_errors(d)
        self.assertTrue(any("stale" in e for e in errs), errs)

    def test_twin_windows_default_off_windows_is_caught(self):
        with mock.patch.object(steamlib, "find_game", lambda *a, **k: None):
            self.assertNotEqual(deck_errors(Deck(self.base)), [])


if __name__ == "__main__":
    unittest.main()
