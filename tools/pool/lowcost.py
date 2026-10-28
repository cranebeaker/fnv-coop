"""Low-cost profile: instance-local INI edits that make a warm game instance cheap to keep running.

Applies ONLY to <instances root>/<i>/Fallout.ini and FalloutPrefs.ini (real per-instance files, see
tools/runner/instances.py PER_INSTANCE). It refuses to touch anything under Documents\\My Games, the Steam
game dir, or a file with more than one hard link (a link could write through into the owner's install).

    python -I tools/pool/lowcost.py apply --instance 0 [--grids 3] [--audio-off]
    python -I tools/pool/lowcost.py restore --instance 0   # write both INIs as provisioning does (profile off)
    python -I tools/pool/lowcost.py doc            # markdown table of every key (pasted into docs/tools/T4a-pool.md)

Every row is (file, section, key, value, why, status). status is
  believed - I expect the key in stock New Vegas INIs but have not checked; `verify` checks it against the
             owner's INIs (read only) and reports which keys are really there
  guess    - the key name comes from Fallout 3 / Oblivion / community lists; the game ignores unknown keys,
             so a wrong guess costs nothing, but the saving is unknown until measured
  opt-in   - not applied unless asked for, because it can change behaviour (see why)
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, NamedTuple, Optional

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "runner"))
import instances  # noqa: E402

FO, PR = "Fallout.ini", "FalloutPrefs.ini"


class Row(NamedTuple):
    file: str
    section: str
    key: str
    value: str
    why: str
    status: str


PROFILE: List[Row] = [
    # --- must-have for instance-local operation (same as instances.FALLOUT_INI_EDITS, repeated so the guard can assert it)
    Row(FO, "General", "bUseMyGamesDirectory", "0", "read INIs and saves from the instance dir, never from Documents\\My Games", "believed"),
    Row(FO, "General", "bAlwaysActive", "1", "keep ticking while minimized (the pool keeps windows minimized)", "believed"),
    # --- window / resolution
    Row(PR, "Display", "bFull Screen", "0", "windowed so it can stay minimized without a mode switch", "believed"),
    Row(PR, "Display", "iSize W", "640", "smallest practical resolution; fewer pixels to shade and a smaller back buffer", "believed"),
    Row(PR, "Display", "iSize H", "480", "see iSize W", "believed"),
    Row(PR, "Display", "iPresentInterval", "0", "no vsync wait", "believed"),
    Row(PR, "Display", "iMultiSample", "0", "no MSAA", "believed"),
    Row(PR, "Display", "iMaxAnisotropy", "0", "no anisotropic filtering", "guess"),
    Row(PR, "Display", "bTransparencyMultisampling", "0", "no alpha-to-coverage", "guess"),
    Row(PR, "Display", "bDoDepthOfField", "0", "no depth-of-field pass", "guess"),
    Row(PR, "Display", "bAllow30Shaders", "0", "shader model 2 paths, cheaper", "guess"),
    # --- frame cap (CPU is the real cost of a warm idle instance)
    Row(FO, "General", "iFPSClamp", "30", "cap at 30 fps: halves CPU and GPU load per instance; game time is delta-based", "believed"),
    # --- shadows
    Row(PR, "Display", "iShadowFilter", "0", "no shadow filtering", "believed"),
    Row(PR, "Display", "iActorShadowCountExt", "0", "no actor shadows outdoors", "believed"),
    Row(PR, "Display", "iActorShadowCountInt", "0", "no actor shadows indoors", "believed"),
    Row(PR, "Display", "bActorSelfShadowing", "0", "no actor self shadows", "believed"),
    Row(PR, "Display", "bShadowsOnGrass", "0", "no shadows on grass", "believed"),
    Row(PR, "Display", "bDoCanopyShadowPass", "0", "no canopy shadow pass", "believed"),
    Row(PR, "Display", "iShadowMapResolutionPrimary", "256", "smallest shadow map if shadows get re-enabled", "guess"),
    Row(PR, "Display", "iShadowMapResolutionSecondary", "256", "see above", "guess"),
    # --- grass
    Row(FO, "Grass", "bDrawShaderGrass", "0", "do not draw shader grass (large CPU saver outdoors)", "believed"),
    Row(FO, "Display", "bDoTallGrassEffect", "0", "no tall grass effect", "believed"),
    # --- water
    Row(PR, "Water", "bUseWaterReflections", "0", "no water reflection render", "believed"),
    Row(PR, "Water", "bUseWaterRefractions", "0", "no water refraction render", "believed"),
    Row(PR, "Water", "bUseWaterDepth", "0", "no water depth pass", "believed"),
    Row(PR, "Water", "bUseWaterDisplacements", "0", "no water displacement", "believed"),
    Row(PR, "Water", "bUseWaterHiRes", "0", "low-res water", "believed"),
    Row(FO, "Water", "bUseWaterReflections", "0", "same keys in Fallout.ini (stock files carry both copies)", "guess"),
    Row(FO, "Water", "bUseWaterRefractions", "0", "see above", "guess"),
    # --- effects and textures
    Row(PR, "Display", "bUseRefractionShader", "0", "no refraction shader (heat haze, glass)", "guess"),
    Row(PR, "Display", "bFXAA", "0", "no post-process AA", "guess"),
    Row(FO, "Display", "iTexMipMapMinimum", "2", "raise the minimum texture mip (default 0) to cut texture memory; whether the engine skips the big mips for this key is unknown", "guess"),
    Row(FO, "Display", "fDecalLifetime", "1", "decals expire quickly", "guess"),
    Row(FO, "Display", "bDecalsOnSkinnedGeometry", "0", "no decals on skinned meshes", "guess"),
    # --- startup speed (the warm instance boots once, but recycling boots again)
    Row(FO, "General", "SIntroSequence", "", "skip the logo movies", "believed"),
    Row(FO, "General", "SMainMenuMovieIntro", "", "skip the main menu intro movie", "guess"),
    Row(FO, "General", "uMainMenuDelayBeforeAllowSkip", "0", "no forced wait on the main menu", "guess"),
    # --- opt-in rows (see apply(grids=, audio_off=))
    Row(FO, "General", "uGridsToLoad", "3", "load 9 exterior cells instead of 25: big RAM and CPU saver. RISKY: a save made with another value can crash or corrupt on load, so it is off unless --grids is given and every save the pool loads was made at that value", "opt-in"),
    Row(FO, "Audio", "bEnableAudio", "0", "no sound device, saves CPU. Opt-in because dialogue-driven waits (voice line end) may never complete without audio", "opt-in"),
]

OPT_IN = {"uGridsToLoad": "grids", "bEnableAudio": "audio_off"}


def rows(grids: Optional[int] = None, audio_off: bool = False) -> List[Row]:
    out = []
    for r in PROFILE:
        if r.status == "opt-in":
            if r.key == "uGridsToLoad":
                if grids is None:
                    continue
                if grids < 3 or grids % 2 == 0:
                    raise ValueError("uGridsToLoad must be odd and >= 3")
                r = r._replace(value=str(grids))
            elif r.key == "bEnableAudio" and not audio_off:
                continue
        out.append(r)
    return out


class UnsafeTarget(Exception):
    pass


def guard(path: Path) -> None:
    """Refuse any INI that is not a private, single-link file of an instance dir."""
    p = Path(path).resolve()
    low = [x.lower() for x in p.parts]
    if "my games" in low:
        raise UnsafeTarget(f"{p}: under Documents\\My Games (the owner's real INIs)")
    try:
        if p.parent == instances.game_dir().resolve():
            raise UnsafeTarget(f"{p}: in the Steam game dir")
    except OSError:
        pass
    if p.exists() and os.stat(p).st_nlink > 1:
        raise UnsafeTarget(f"{p}: has {os.stat(p).st_nlink} hard links (writing would leak into another dir)")
    if p.is_symlink() or (p.exists() and Path(os.path.realpath(p)) != p):
        raise UnsafeTarget(f"{p}: is a link")


def apply(instance_dir: Path, grids: Optional[int] = None, audio_off: bool = False, check: bool = True) -> Dict[str, int]:
    """Edit the instance's two INIs in place. Returns {file: keys written}. Idempotent."""
    d = Path(instance_dir)
    texts: Dict[str, str] = {}
    for name in (FO, PR):
        f = d / name
        if check:
            guard(f)
        texts[name] = f.read_text(encoding="latin-1") if f.exists() else ""
    n = {FO: 0, PR: 0}
    for r in rows(grids, audio_off):
        texts[r.file] = instances.ini_set(texts[r.file], r.section, r.key, r.value)
        n[r.file] += 1
    for name, t in texts.items():
        (d / name).write_text(t, encoding="latin-1", newline="")
    if instances.ini_get(texts[FO], "General", "bUseMyGamesDirectory") != "0":
        raise UnsafeTarget("bUseMyGamesDirectory is not 0 after apply")
    return n


def _owner_base(name: str) -> str:
    """The base INI for a read-only report. A missing owner FalloutPrefs.ini whose fallback cannot be built (unknown display
    adapter) is an empty base here; only provisioning, which writes a file the game must start with, refuses it."""
    try:
        return instances._base_ini(name)
    except getattr(instances, "PrefsUnavailable", ()):
        return ""

def prefs_startable(text: str) -> bool:
    """True when FalloutPrefs.ini has sD3DDevice. Without it FalloutNV.exe opens its launcher instead of starting."""
    return bool((instances.ini_get(text, "Display", "sD3DDevice") or "").strip())


def provisioned_no_pad(instance_dir: Path) -> bool:
    """Whether the copy was provisioned with --no-pad (its instance.json says so). False when the copy has no instance.json.
    An instance.json that can't be read, isn't a JSON object or has a no_pad that isn't true/false raises UnsafeTarget, so
    restore() writes nothing instead of guessing the copy's controller setting."""
    p = Path(instance_dir) / "instance.json"
    try:
        text = p.read_text()
    except FileNotFoundError:
        return False
    except OSError as e:
        raise UnsafeTarget(f"{p}: can't be read ({e}), so whether the copy ignores pads (--no-pad) is unknown") from e
    try:
        meta = json.loads(text)
    except ValueError as e:
        raise UnsafeTarget(f"{p}: not valid JSON ({e}); provision the copy again") from e
    if not isinstance(meta, dict):
        raise UnsafeTarget(f"{p}: not a JSON object; provision the copy again")
    no_pad = meta.get("no_pad", False)   # provision() writes a JSON boolean; bool("false") would be True
    if not isinstance(no_pad, bool):
        raise UnsafeTarget(f"{p}: no_pad is {no_pad!r}, not true or false; provision the copy again")
    return no_pad


def restore(instance_dir: Path, check: bool = True) -> Dict[str, int]:
    """Undo apply(): write the instance's two INIs exactly as provisioning writes them (instances.render_inis()), --no-pad included.

    apply() touches only Fallout.ini and FalloutPrefs.ini, so only these two files are written. The same guard as
    apply() runs on both files before either is written: never the owner's My Games, the game dir, a hard link or a
    symlink. Returns {file: bytes written}. Idempotent. The pool calls it on every start with the profile off.
    """
    d = Path(instance_dir)
    fresh = instances.render_inis(no_pad=provisioned_no_pad(d))
    if set(fresh) != {FO, PR}:
        raise UnsafeTarget(f"render_inis() returned {sorted(fresh)}, expected {[FO, PR]}")
    if instances.ini_get(fresh[FO], "General", "bUseMyGamesDirectory") != "0":
        raise UnsafeTarget("render_inis() Fallout.ini does not set bUseMyGamesDirectory=0")
    # FalloutNV.exe starts FalloutNVLauncher.exe (a window on the owner's desktop) instead of the game when the prefs have
    # no sD3DDevice. Seen in game on 2026-10-21: render_inis() could not find the owner's prefs (redirected Documents),
    # wrote a 6-line FalloutPrefs.ini, and `pool up --no-lowcost` then failed with the launcher left open. Fail closed.
    if not prefs_startable(fresh[PR]):
        raise UnsafeTarget("render_inis() FalloutPrefs.ini has no sD3DDevice, so the game would open its launcher instead "
                           "of starting: start Fallout: New Vegas once through Steam (the launcher writes it), check that "
                           "the owner's My Games folder is found, then try again")
    if check:
        for name in (FO, PR):
            guard(d / name)
    for name in (FO, PR):
        (d / name).write_text(fresh[name], encoding="latin-1", newline="")
    return {name: len(fresh[name].encode("latin-1")) for name in (FO, PR)}


def verify() -> Dict[str, List[str]]:
    """Read-only: which profile keys exist in the owner's own INIs (the base the pool copies from)."""
    found: Dict[str, List[str]] = {"present": [], "absent": []}
    base = {FO: _owner_base(FO), PR: _owner_base(PR)}
    for r in PROFILE:
        has = instances.ini_get(base[r.file], r.section, r.key) is not None
        found["present" if has else "absent"].append(f"{r.file}:{r.section}:{r.key}")
    return found


def render_doc(grids: Optional[int] = None) -> str:
    lines = ["| File | Section | Key | Value | Why | Status |", "|---|---|---|---|---|---|"]
    ver = verify()
    for r in PROFILE:
        tag = f"{r.file}:{r.section}:{r.key}"
        st = "opt-in" if r.status == "opt-in" else ("present in owner INI" if tag in ver["present"] else "not in owner INI (added)")
        lines.append(f"| {r.file} | {r.section} | `{r.key}` | `{r.value}` | {r.why} | {st} |")
    return "\n".join(lines)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("apply")
    p.add_argument("--instance", type=int, required=True)
    p.add_argument("--grids", type=int)
    p.add_argument("--audio-off", action="store_true")
    r = sub.add_parser("restore")
    r.add_argument("--instance", type=int, required=True)
    sub.add_parser("doc")
    sub.add_parser("verify")
    a = ap.parse_args(argv)
    if a.cmd == "doc":
        print(render_doc())
        return 0
    if a.cmd == "verify":
        import json
        print(json.dumps(verify(), indent=1))
        return 0
    if a.cmd == "restore":
        print(restore(instances.instance_dir(a.instance)))
        return 0
    print(apply(instances.instance_dir(a.instance), a.grids, a.audio_off))
    return 0


if __name__ == "__main__":
    sys.exit(main())
