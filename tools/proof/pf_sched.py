"""Which tests may run at the same time (prove.py --pairs N). Pure functions: no game, no threads, no processes.

A pair is two copies: pair k = instances 2k (host) and 2k+1 (client), on the co-op port 27960+k (tools/runner/pairs.py). With --pairs 2
the owner's PC runs two independent two-copy tests at once, one per pair. The rules:

  * a test with 2 copies runs on any free pair (pair 0 first); its verdict line says which ("pair": 0 or 1)
  * a test with 1 copy uses instance 0 (pair 0) and leaves pair 1 free
  * a test with 3 or more copies (a pair plus one copy, or both pairs) runs ALONE: nothing else runs beside it; so does a test marked
    "alone" in the manifest (one whose driver starts its own game copies, such as the split-screen smoke)
  * a test with no copies (game-free) takes no pair, but never runs beside an exclusive test
  * never more tests at once than pairs
  * tests start in manifest order and the queue does not skip ahead: if the next test must wait, everything behind it waits too
"""
from __future__ import annotations

import os
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Sequence

MUTANT = os.environ.get("PROOF_MUTANT", "")   # test-only: one deliberate bug per name (tools/proof/tests/run_tests.py)
MAX_PAIRS = 2          # the owner's PC fits two pairs (docs/PROOF-PLAN.md section 6)


def copies_of(test: Dict[str, Any]) -> int:
    return int(test.get("copies", 0))


def instances_for(test: Dict[str, Any], pair: Optional[int] = None) -> List[int]:
    """The instances a test runs on. A two-copy test runs on `pair` (default: the pair written in the manifest)."""
    c = copies_of(test)
    if c <= 0:
        return []
    if c == 2:
        k = int(test.get("pair", 0)) if pair is None else pair
        return [2 * k, 2 * k + 1]
    return list(range(c))


def exclusive(test: Dict[str, Any]) -> bool:
    """True when the test needs the machine to itself: three copies or more, or "alone" in the manifest."""
    return copies_of(test) >= 3 or bool(test.get("alone"))


def claim(test: Dict[str, Any], instances: Sequence[int], pairs: int) -> List[int]:
    """What a running test blocks, as the instance set `pick` sees: its own copies, or every pair for an "alone" test whose copies
    are started by its driver (so no other test can start beside it)."""
    ins = list(instances)
    if test.get("alone"):
        return sorted(set(ins) | set(range(2 * max(1, pairs))) | set(range(3)))
    return ins


def candidates(test: Dict[str, Any], pairs: int) -> List[List[int]]:
    """Instance sets the test may run on, best first. A two-copy test whose manifest `pair` is 0 floats over the first `pairs` pairs;
    one with an explicit pair 1 or higher is pinned there."""
    c = copies_of(test)
    if c <= 0:
        return [[]]
    if c == 2:
        if int(test.get("pair", 0)) > 0:
            return [instances_for(test)]
        return [instances_for(test, k) for k in range(max(1, pairs))]
    return [instances_for(test)]


def pick(test: Dict[str, Any], running: Sequence[Sequence[int]], pairs: int) -> Optional[List[int]]:
    """The instances the next test may start on while `running` (the instance sets of the tests in flight) goes on, or None to wait.
    `running` holds an empty list for a game-free test; a set of 3 or more copies is an exclusive test."""
    if MUTANT == "exclusive_not_alone":
        return instances_for(test) if exclusive(test) else (candidates(test, pairs)[0] if len(running) < max(1, pairs) else None)
    if any(len(r) >= 3 for r in running):
        return None
    if exclusive(test):
        return instances_for(test) if not running else None
    if len(running) >= max(1, pairs):
        return None
    busy = {i for r in running for i in r}
    for cand in candidates(test, pairs):
        if MUTANT == "pair_overlap" or not (set(cand) & busy):
            return list(cand)
    return None


def pair_of(instances: Sequence[int]) -> Optional[int]:
    """The pair a run is reported on: the pair of its first copy, None for a game-free test."""
    return (int(instances[0]) // 2) if instances else None


def pairs_that_fit(requested: int, free_bytes: int, up_copies: int, reserve_gb: float, copy_gb: float) -> int:
    """How many pairs (1..requested) the free RAM can hold, copies that already run included. Never below 1: one pair is the
    old behaviour and its own check (pf_safety.preflight) decides whether even that fits."""
    gib = 1 << 30
    spare = free_bytes / gib - reserve_gb + 0.001
    more = int(spare // copy_gb) if spare > 0 else 0
    return max(1, min(requested, (max(0, up_copies) + more) // 2))


# ------------------------------------------------------------------ how the copies of a test are provisioned
def provision_args(test: Dict[str, Any], instances: Sequence[int], parallel: bool) -> List[str]:
    """Arguments for `tools/runner/run.py provision`: a pair (or both) is provisioned in pair mode, with its own co-op port and only
    its own copies (--first-pair leaves a running copy of the other pair alone); other layouts as before."""
    c = copies_of(test)
    pair = int(test.get("pair", 0))
    if c == 4:
        return ["--pairs=2"]
    if c == 2 and (parallel or pair >= 1):
        k = int(instances[0]) // 2
        return [f"--pairs={k + 1}", f"--first-pair={k}"]
    return ["--n", str(max(instances) + 1)]


def layout_problem(test: Dict[str, Any], instances: Sequence[int], parallel: bool, meta: Callable[[int], Dict[str, Any]]) -> Optional[str]:
    """Why the copies are not provisioned the way this test needs them (None when they are). `meta(i)` is instance i's instance.json.
    Pair mode writes {"pair": k, "port": ...} there; a plain `--n` provision does not."""
    c = copies_of(test)
    if c == 2 and (parallel or int(test.get("pair", 0)) >= 1) or c == 4:
        for i in instances:
            if meta(i).get("pair") != i // 2:
                return f"instance {i} is not provisioned as a member of pair {i // 2} (own co-op port)"
    elif c == 3:
        for i in instances[2:]:
            if meta(i).get("pair") is not None:
                return f"instance {i} is provisioned as the host of pair {i // 2}; a 3-copy test needs it as a plain client of pair 0"
    return None


# ---------------------------------------------------------------- one build on every copy (2026-10-22: bridge methods missing on the second pair)
_SHA: Dict[Any, str] = {}


def file_sha(p: Any) -> str:
    """sha1 of a file, remembered per (path, mtime, size) so four copies of a 3.6 MB dll cost one read each."""
    import hashlib
    p = Path(p)
    st = p.stat()
    key = (str(p), st.st_mtime_ns, st.st_size)
    if key not in _SHA:
        _SHA[key] = hashlib.sha1(p.read_bytes()).hexdigest()
    return _SHA[key]


def expected_build(root: Path) -> Dict[str, Path]:
    """The plugin files every copy must hold: the split build (loader + core) when out/L1split has both, else the monolith out/L1/nvcoop.dll.
    The same choice provision_cmd() makes, so what provisioning installs is what is checked."""
    split = Path(root) / "out" / "L1split"
    if (split / "nvcoop.dll").exists() and (split / "nvcoop_mod.dll").exists():
        return {"nvcoop.dll": split / "nvcoop.dll", "nvcoop_mod.dll": split / "nvcoop_mod.dll"}
    mono = Path(root) / "out" / "L1" / "nvcoop.dll"
    return {"nvcoop.dll": mono} if mono.exists() else {}


def build_problem(instances: Sequence[int], idir_of: Callable[[int], Path], expected: Dict[str, Path]) -> Optional[str]:
    """Why a copy does not run the build the others run (None when every copy holds exactly `expected`). A copy of pair 1 provisioned by hand, or before
    the last build, lacks modules (combat.status, vats.status ...) that pair 0 has; that must be refused as BUILD, never run and failed as a test."""
    import configparser
    if MUTANT == "build_unchecked":
        return None
    for i in instances:
        plug = Path(idir_of(i)) / "Data" / "NVSE" / "Plugins"
        for name, src in expected.items():
            dst = plug / name
            if not dst.is_file():
                return f"instance {i}: {name} is not installed"
            if file_sha(dst) != file_sha(src):
                return f"instance {i}: {name} is not the build in {Path(src).parent.name}/ ({file_sha(dst)[:8]} vs {file_sha(src)[:8]})"
        if expected and "nvcoop_mod.dll" not in expected and (plug / "nvcoop_mod.dll").exists():
            return f"instance {i}: a split core nvcoop_mod.dll is installed but the build is the monolith"
        if "nvcoop_mod.dll" in expected:
            cp = configparser.ConfigParser(strict=False, interpolation=None)
            try:
                cp.read(plug / "nvcoop.ini", encoding="mbcs" if os.name == "nt" else None)   # mbcs exists on Windows only
            except (OSError, configparser.Error):
                pass
            core = cp.get("loader", "core", fallback="") if cp.has_section("loader") else ""
            if core.split(";")[0].strip().lower() != "nvcoop_mod.dll":
                return f"instance {i}: nvcoop.ini [loader] core is {core!r}, the loader would not load nvcoop_mod.dll"
    return None
