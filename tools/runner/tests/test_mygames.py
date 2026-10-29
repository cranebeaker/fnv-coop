"""The tools must find the owner's My Games folder where the game does: under the shell Documents folder, which Windows
may redirect (OneDrive). Twin: the old lookup (home folder + "Documents") must fail the redirect check.

Run: python -I -m unittest discover -s tools/runner/tests -p test_mygames.py
"""
from __future__ import annotations

import os
import re
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
sys.path.insert(0, str(HERE.parent))
import mygames  # noqa: E402

FORBIDDEN = re.compile(r"""Path\.home\(\)\s*/\s*["']Documents["']""")
# mygames.py itself keeps the home-folder lookup only as the last fallback (no shell folder at all).
# tools/installer/fnv_install.py ships alone in the release zip (no tools/runner next to it), so it carries its own copy of the
# same lookup: the shell Documents folder (SHGetFolderPathW CSIDL_PERSONAL) first, the home folder only as the last fallback.
# tools/launcher/fnv_party_saves.py ships in the release without tools/runner too, with the same order (FNV_DOCUMENTS_DIR, shell folder, home).
ALLOWED = {Path("tools/runner/mygames.py"), Path("tools/installer/fnv_install.py"), Path("tools/launcher/fnv_party_saves.py")}


def mutant_documents_dir(resolver=mygames.shell_documents) -> Path:
    """The pre-fix behaviour: ignores the shell folder."""
    return Path.home() / "Documents"


def redirect_errors(documents_fn) -> list:
    """Errors when documents_fn does not follow a redirected Documents folder."""
    errs = []
    redirected = Path(tempfile.gettempdir()) / "OneDrive" / "Tiedostot"
    with mock.patch.dict(os.environ, {}, clear=False):
        os.environ.pop("FNV_DOCUMENTS_DIR", None)
        got = documents_fn(lambda: redirected)
    if got != redirected:
        errs.append(f"redirected Documents ignored: got {got}, want {redirected}")
    return errs


def override_errors(documents_fn) -> list:
    """Errors when FNV_DOCUMENTS_DIR is used without expanding ~ / %VARS% or without making it absolute."""
    errs = []
    cases = {
        "~/OneDrive/Documents": Path.home() / "OneDrive" / "Documents",
        "%FNV_TEST_DOCS_ROOT%\\Docs": Path(tempfile.gettempdir()) / "Docs",
    }
    env = {"FNV_TEST_DOCS_ROOT": tempfile.gettempdir()}
    for value, want in cases.items():
        with mock.patch.dict(os.environ, dict(env, FNV_DOCUMENTS_DIR=value)):
            got = documents_fn(lambda: None)
        if got != want.resolve():
            errs.append(f"{value!r} -> {got}, want {want.resolve()}")
    for value in ("rel_docs", "..\\Docs", "Documents"):  # relative: must be refused, never resolved against the cwd
        with mock.patch.dict(os.environ, dict(env, FNV_DOCUMENTS_DIR=value)):
            try:
                got = documents_fn(lambda: None)
                errs.append(f"relative {value!r} accepted as {got}")
            except ValueError:
                pass
    return errs


def forbidden_lookups(root: Path) -> list:
    hits = []
    for p in sorted(root.rglob("*.py")):
        rel = p.relative_to(REPO) if p.is_relative_to(REPO) else p
        if rel in ALLOWED or "tests" in p.parts and p.name == "test_mygames.py":
            continue
        for i, ln in enumerate(p.read_text("utf-8", errors="replace").splitlines(), 1):
            if FORBIDDEN.search(ln):
                hits.append(f"{rel}:{i}")
    return hits


class MyGames(unittest.TestCase):
    def test_follows_redirected_documents(self):
        self.assertEqual(redirect_errors(mygames.documents_dir), [])

    def test_twin_old_lookup_is_caught(self):
        self.assertNotEqual(redirect_errors(mutant_documents_dir), [])

    def test_env_override_wins(self):
        with mock.patch.dict(os.environ, {"FNV_DOCUMENTS_DIR": r"X:\Docs"}):
            self.assertEqual(mygames.documents_dir(lambda: Path(r"Y:\Other")), Path(r"X:\Docs"))
            self.assertEqual(mygames.saves_dir(), Path(r"X:\Docs") / "My Games" / "FalloutNV" / "Saves")

    def test_env_override_is_expanded_and_absolute(self):
        self.assertEqual(override_errors(mygames.documents_dir), [])

    def test_twin_raw_env_override_is_caught(self):
        def raw(resolver=mygames.shell_documents):  # the first version: Path(env) as given
            return Path(os.environ["FNV_DOCUMENTS_DIR"])
        self.assertNotEqual(override_errors(raw), [])

    def test_twin_relative_resolved_against_cwd_is_caught(self):
        def cwd_resolve(resolver=mygames.shell_documents):  # the second version: relative resolved against the cwd
            return Path(os.path.expandvars(os.path.expanduser(os.environ["FNV_DOCUMENTS_DIR"]))).resolve()
        errs = override_errors(cwd_resolve)
        self.assertTrue(errs and all("relative" in e for e in errs), errs)

    def test_fallback_when_shell_has_no_answer(self):
        with mock.patch.dict(os.environ, {}, clear=False):
            os.environ.pop("FNV_DOCUMENTS_DIR", None)
            self.assertEqual(mygames.documents_dir(lambda: None), Path.home() / "Documents")

    @unittest.skipUnless(os.name == "nt", "Windows shell folders only")
    def test_live_shell_documents_exists(self):
        d = mygames.shell_documents()
        self.assertIsNotNone(d)
        self.assertTrue(d.is_dir(), d)

    def test_no_tool_builds_my_games_from_home(self):
        self.assertEqual(forbidden_lookups(REPO / "tools"), [])

    def test_twin_scanner_flags_the_old_pattern(self):
        with tempfile.TemporaryDirectory() as t:
            (Path(t) / "bad.py").write_text('src = Path.home() / "Documents" / "My Games"\n', encoding="utf-8")
            self.assertEqual(len(forbidden_lookups(Path(t))), 1)


if __name__ == "__main__":
    unittest.main()
