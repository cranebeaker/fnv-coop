"""Guards for the hard safety rules, checked on the source files themselves."""
import glob
import os
import re
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))

# Forbidden in anything we ship (comments in .ps1 files starting with # are skipped).
FORBIDDEN_PS = [
    r"\bdel\b", r"\berase\b", r"\brd\b", r"\brmdir\b", r"\brm\b", r"Remove-Item", r"taskkill", r"Stop-Process",
    r"\bkill\b", r"SendKeys", r"SendInput", r"mouse_event", r"keybd_event", r"SetCursorPos", r"Windows\.Forms",
    r"Invoke-Expression", r"\biex\b",
]
FORBIDDEN_PY = [r"rmtree", r"os\.remove", r"os\.unlink", r"\.unlink\(", r"os\.rmdir", r"taskkill", r"pkill",
                r"killall", r"pyautogui", r"SendInput", r"keybd_event", r"mouse_event"]
# the owner's real name or Windows username must never appear (built without writing the literals)
IDENT = [re.compile("mthw" + "j", re.I), re.compile("mthw" + "jsmith", re.I)]


def read(p):
    with open(p, encoding="utf-8", errors="replace") as f:
        return f.read()


def ps_files(sub=None):
    subs = [sub] if sub else ["remote", "controller"]
    return [p for d in subs for p in glob.glob(os.path.join(HERE, d, "*.ps1"))]


def code_lines(path):
    with open(path, encoding="utf-8") as f:
        for n, line in enumerate(f, 1):
            if path.endswith(".ps1") and line.lstrip().startswith("#"):
                continue
            yield n, line


class Safety(unittest.TestCase):
    def test_powershell_helpers_have_no_destructive_or_input_calls(self):
        files = ps_files()
        self.assertGreaterEqual(len(files), 8)
        for p in files:
            for n, line in code_lines(p):
                for pat in FORBIDDEN_PS:
                    self.assertIsNone(re.search(pat, line, re.I), "%s:%d matches %s: %s" % (p, n, pat, line.strip()))

    def test_python_sources_never_delete_or_kill_by_name(self):
        for p in glob.glob(os.path.join(HERE, "*.py")):
            if os.path.basename(p).startswith("test_"):
                continue
            for n, line in code_lines(p):
                for pat in FORBIDDEN_PY:
                    self.assertIsNone(re.search(pat, line), "%s:%d matches %s" % (p, n, pat))

    def test_remote_scripts_stay_under_c_fnv(self):
        for p in ps_files():
            text = "".join(line for _, line in code_lines(p))
            for m in re.finditer(r"[A-Za-z]:\\[^\s\"'`]*", text):
                path = m.group(0).replace("\\\\", "\\").lower()
                ok = path.startswith("c:\\fnv") or path.startswith("c:\\program")  # the python install
                self.assertTrue(ok, "%s mentions %s" % (p, m.group(0)))
        for p in ps_files("remote"):
            text = read(p)
            self.assertIn("Set-Location C:\\fnv", text, p + " must cd to C:\\fnv first")

    def test_no_owner_identifiers_anywhere(self):
        for root, _, names in os.walk(HERE):
            if "__pycache__" in root or os.sep + "out" in root:
                continue
            for nme in names:
                if nme.endswith((".pyc", ".local.json")):
                    continue
                p = os.path.join(root, nme)
                text = read(p)
                for rx in IDENT:
                    self.assertIsNone(rx.search(text), "%s contains a private identifier" % p)
                self.assertNotIn("\u2014", text, "%s has an em dash" % p)

    def test_no_emoji(self):
        for p in glob.glob(os.path.join(HERE, "**", "*"), recursive=True):
            if os.path.isfile(p) and p.endswith((".py", ".md", ".ps1", ".json")):
                text = read(p)
                self.assertIsNone(re.search("[\U0001F300-\U0001FAFF\u2600-\u27BF]", text), p)


if __name__ == "__main__":
    unittest.main()
