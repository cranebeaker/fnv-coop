"""Shared build driver: one place that knows how to set up MSVC, take the build lock, configure and build.

    python -I tools/build/build.py [--tree all] [--arch x86|x64] [--target T [T ...]] [--source DIR] [--build DIR]
                                   [--cmake-arg ARG ...] [--no-lock] [--label NAME] [--quiet]

Defaults: source = the repo root (the superbuild CMakeLists.txt), build tree = build/all, arch = x86 (the game's arch),
Release, Ninja, CMAKE_BUILD_PARALLEL_LEVEL=2 unless already set.

* Lock: out/locks/build.lock is created with mkdir (atomic). If it exists we retry every 15 s. It is held only while cmake
  configures/builds and is removed on success, failure and Ctrl-C. Other agents use the same lock directory.
* Compiler cache: if `sccache` is on PATH it is used as the compiler launcher (and precompiled headers are switched off,
  because sccache cannot cache /Yu compiles). Install it with `scoop install sccache`; nothing here installs software.
* Prints "BUILD-TIME <label> <seconds>" so the speed-up can be measured.
"""
import argparse
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
import vsroot  # noqa: E402

VSROOT = vsroot.find()
VSINSTALLER = r"C:\Program Files (x86)\Microsoft Visual Studio\Installer"
NINJA_DIR = VSROOT + r"\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja"
LOCK = ROOT / "out" / "locks" / "build.lock"

DROP_ENV = ("CXX", "CC", "CMAKE_TOOLCHAIN_FILE", "ANDROID_NDK_HOME", "ANDROID_NDK", "CMAKE_ANDROID_NDK", "CMAKE_GENERATOR")


def msvc_env(arch: str) -> dict:
    bat = VSROOT + (r"\VC\Auxiliary\Build\vcvars32.bat" if arch == "x86" else r"\VC\Auxiliary\Build\vcvars64.bat")
    env = dict(os.environ)
    env["PATH"] = VSINSTALLER + os.pathsep + env.get("PATH", "")
    for k in DROP_ENV:
        env.pop(k, None)
    p = subprocess.run(f'cmd /c ""{bat}" >nul && set"', shell=True, capture_output=True, text=True, env=env, errors="replace")
    if p.returncode != 0:
        raise SystemExit("vcvars failed: " + (p.stderr or p.stdout)[-400:])
    out = {}
    for line in p.stdout.splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            out[k] = v
    for k in DROP_ENV:
        out.pop(k, None)
    out["PATH"] = NINJA_DIR + os.pathsep + out.get("PATH", "")
    out.setdefault("CMAKE_BUILD_PARALLEL_LEVEL", "2")
    return out


def acquire_lock(quiet: bool, label: str = ""):
    LOCK.parent.mkdir(parents=True, exist_ok=True)
    waited = 0
    while True:
        try:
            os.mkdir(LOCK)
            try:  # who holds it, for humans debugging a stuck lock (other agents' scripts only care that the directory exists)
                (LOCK / "owner.txt").write_text(f"build.py pid={os.getpid()} label={label} started={time.strftime('%H:%M:%S')}" + chr(10))
            except OSError:
                pass
            return
        except FileExistsError:
            if waited % 60 == 0 and not quiet:
                print(f"[build] lock {LOCK} is held, retrying every 15 s (waited {waited}s)", flush=True)
            time.sleep(15)
            waited += 15


def release_lock():
    try:
        os.rmdir(LOCK)
    except OSError:
        shutil.rmtree(LOCK, ignore_errors=True)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--tree", default="all", help="build/<tree> under the repo (default: all)")
    ap.add_argument("--source", default=str(ROOT), help="CMake source dir (default: repo root superbuild)")
    ap.add_argument("--build", default=None, help="explicit build dir (overrides --tree)")
    ap.add_argument("--arch", default="x86", choices=("x86", "x64"))
    ap.add_argument("--target", nargs="*", default=[])
    ap.add_argument("--cmake-arg", action="append", default=[])
    ap.add_argument("--config", default="Release")
    ap.add_argument("--no-lock", action="store_true")
    ap.add_argument("--no-sccache", action="store_true")
    ap.add_argument("--clean", action="store_true")
    ap.add_argument("--keep-going", "-k", action="store_true", help="ninja -k 0: report every compile error, not just the first")
    ap.add_argument("--configure-only", action="store_true", help="run cmake configure only (no lock needed, nothing is compiled)")
    ap.add_argument("--label", default="build")
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()

    # Repair string literals split by a real line break (agent / heredoc damage, MSVC C2001) in changed files before compiling:
    # one bad "\n" otherwise costs every agent a full failed compile behind the lock.
    lint = ROOT / "tools" / "lint" / "fix_newline_literals.py"
    if lint.exists():
        subprocess.run([sys.executable, "-I", str(lint), "--changed"], cwd=ROOT, check=False,
                       stdout=subprocess.DEVNULL if a.quiet else None)

    build = Path(a.build) if a.build else ROOT / "build" / a.tree
    env = msvc_env(a.arch)
    cmake_args = ["-G", "Ninja", f"-DCMAKE_BUILD_TYPE={a.config}", "-DCMAKE_C_COMPILER=cl", "-DCMAKE_CXX_COMPILER=cl"]
    sccache = None if a.no_sccache else shutil.which("sccache")
    if sccache:
        cmake_args += [f"-DCMAKE_C_COMPILER_LAUNCHER={sccache}", f"-DCMAKE_CXX_COMPILER_LAUNCHER={sccache}", "-DFNVCOOP_PCH=OFF"]
    cmake_args += a.cmake_arg

    if a.configure_only:
        a.no_lock = True
    # Nothing to compile? Then do not queue for the lock at all (a dry run only reads the tree). This is what makes the 2nd..Nth suite
    # of a test_all run start immediately instead of waiting behind other agents' compiles.
    if not a.no_lock and not a.clean and not a.cmake_arg and (build / "build.ninja").exists():
        dry = ["cmake", "--build", str(build)] + (["--target"] + a.target if a.target else []) + ["--", "-n"]
        p = subprocess.run(dry, env=env, capture_output=True, text=True, errors="replace")
        if p.returncode == 0 and "no work to do" in (p.stdout + p.stderr):
            print("ninja: no work to do. (checked without taking the build lock)")
            print(f"BUILD-TIME {a.label} 0.0s (tree={build}, arch={a.arch}, up to date)", flush=True)
            return 0
    if not a.no_lock:
        acquire_lock(a.quiet, a.label)
    t0 = time.time()
    rc = 1
    try:
        if a.clean and build.exists():
            shutil.rmtree(build, ignore_errors=True)
        cache = build / "CMakeCache.txt"
        # (re)configure when the tree is new, or when extra -D args are given; otherwise ninja re-runs cmake itself when needed
        if not (build / "build.ninja").exists() or not cache.exists() or a.cmake_arg:
            p = subprocess.run(["cmake", "-S", a.source, "-B", str(build)] + cmake_args, env=env)
            if p.returncode != 0:
                print("[build] CONFIGURE FAILED")
                return p.returncode
        if a.configure_only:
            rc = 0
            return rc
        cmd = ["cmake", "--build", str(build)]
        if a.target:
            cmd += ["--target"] + a.target
        if a.keep_going:
            cmd += ["--", "-k", "0"]
        p = subprocess.run(cmd, env=env)
        rc = p.returncode
        if rc != 0:
            print("[build] BUILD FAILED")
    finally:
        dt = time.time() - t0
        if not a.no_lock:
            release_lock()
        print(f"BUILD-TIME {a.label} {dt:.1f}s (tree={build}, arch={a.arch}, sccache={'yes' if sccache else 'no'})", flush=True)
    return rc


if __name__ == "__main__":
    sys.exit(main())
