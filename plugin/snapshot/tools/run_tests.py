#!/usr/bin/env python3
"""One-command test run for the snapshot lane (L7):

    python -I plugin/snapshot/tools/run_tests.py [--no-cpp] [--no-engine]

 1. regenerate-and-verify goldens against MANIFEST.sha256
 2. python unit tests (snapdiff / converge / guard / mutation twins)
 3. C++ core json test (x86, MSVC) + its mutant, which MUST fail
 4. compile-only check of the engine dumper against third_party/NVSE headers (x86)
Build products go to a temp dir that is deleted afterwards (disk is tight).
"""
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
SNAP = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.path.abspath(os.path.join(SNAP, "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "tools", "build"))
import vsroot  # noqa: E402
VCVARS = os.path.join(vsroot.find(), r"VC\Auxiliary\Build\vcvarsall.bat")


def run(cmd, **kw):
    return subprocess.run(cmd, **kw).returncode


def cl(tmp, args, run_exe=None):
    bat = os.path.join(tmp, "b.bat")
    lines = ['@echo off', 'call "%s" x86 >nul 2>&1' % VCVARS, 'cd /d "%s"' % SNAP,
             "cl /nologo %s" % args]
    if run_exe:
        lines.append('if errorlevel 1 exit /b 1')
        lines.append('"%s"' % run_exe)
    with open(bat, "w") as f:
        f.write("\r\n".join(lines) + "\r\n")
    return run(["cmd", "/c", bat])


def main():
    failures = []
    py = [sys.executable, "-I"]
    if run(py + [os.path.join(ROOT, "tests", "goldens", "snapshot", "make_goldens.py")]) != 0:
        failures.append("goldens differ from MANIFEST.sha256")
    if run(py + [os.path.join(HERE, "test_snapshot_tools.py")]) != 0:
        failures.append("python tests")
    if run(py + [os.path.join(HERE, "test_volatile.py")]) != 0:
        failures.append("volatile probe tests")
    if run(py + [os.path.join(HERE, "test_sweep.py")]) != 0:
        failures.append("sweep analysis tests")
    if run(py + [os.path.join(HERE, "test_sweep_driver.py")]) != 0:
        failures.append("sweep driver/config tests")
    if run(py + [os.path.join(HERE, "test_effects.py")]) != 0:
        failures.append("effect oracle / check_stage tests")
    if run(py + [os.path.join(HERE, "test_build_flags.py")]) != 0:
        failures.append("synccore build flag parity")
    if "--no-cpp" not in sys.argv and os.path.exists(VCVARS):
        tmp = tempfile.mkdtemp(prefix="snaptest_")
        try:
            srcs = "tests/json_test.cpp src/json.cpp src/snapshot_util.cpp"
            common = "/std:c++17 /EHsc /Iinclude"
            fw = tmp.replace("\\", "/")
            ok, bad = os.path.join(tmp, "ok.exe"), os.path.join(tmp, "mutant.exe")
            os.mkdir(os.path.join(tmp, "o")); os.mkdir(os.path.join(tmp, "m"))
            if cl(tmp, '%s %s /Fo"%s/o/" /Fe"%s"' % (common, srcs, fw, ok)) != 0 or run([ok]) != 0:
                failures.append("C++ json test")
            if cl(tmp, '%s /DMUTANT %s /Fo"%s/m/" /Fe"%s"' % (common, srcs, fw, bad)) != 0:
                failures.append("C++ mutant failed to BUILD")
            else:
                rc = run([bad])
                if rc == 0:
                    failures.append("C++ MUTANT PASSED (test is not sensitive)")
                else:
                    print("C++ mutant correctly failed (rc=%d)" % rc)
            sc = os.path.join(ROOT, "synccore")
            if os.path.isdir(os.path.join(sc, "src")):
                core = " ".join("%s/src/%s.cpp" % (sc.replace("\\", "/"), n) for n in (
                    "wire shadow policy storylog protocol baseline replicator transport transport_util session").split())
                bsrc = "tests/baseline_test.cpp src/baseline_build.cpp " + core
                # synccore's own conformance flags (synccore/CMakeLists.txt); test_build_flags.py keeps the two in step
                binc = '/std:c++17 /EHsc /permissive- /utf-8 /Iinclude /I"%s/include"' % sc.replace("\\", "/")
                for tag, extra in (("b", ""), ("bm", " /DBASELINE_MUTANT")):
                    os.mkdir(os.path.join(tmp, tag))
                    exe = os.path.join(tmp, tag + ".exe")
                    if cl(tmp, '%s%s %s /Fo"%s/%s/" /Fe"%s"' % (binc, extra, bsrc, fw, tag, exe)) != 0:
                        failures.append("baseline test build (%s)" % tag)
                    elif tag == "b" and run([exe]) != 0:
                        failures.append("baseline test")
                    elif tag == "bm":
                        rc = run([exe])
                        if rc == 0:
                            failures.append("baseline MUTANT PASSED")
                        else:
                            print("baseline mutant correctly failed (rc=%d)" % rc)
            if os.path.isdir(os.path.join(sc, "src")):
                cross = os.path.join(tmp, "cross")
                if run(py + [os.path.join(HERE, "volatile_cross.py"), cross]) != 0:
                    failures.append("volatile cross fixture")
                for tag, extra in (("v", ""), ("vm", " /DVOL_MUTANT")):
                    os.mkdir(os.path.join(tmp, tag))
                    exe = os.path.join(tmp, tag + ".exe")
                    if cl(tmp, '%s%s tests/volatile_cross_test.cpp %s /Fo"%s/%s/" /Fe"%s"' % (binc, extra, core, fw, tag, exe)) != 0:
                        failures.append("volatile cross test build (%s)" % tag)
                    elif tag == "v" and run([exe, os.path.join(cross, "keys.txt"), os.path.join(cross, "queries.txt")]) != 0:
                        failures.append("volatile cross test (shared loader disagrees with the probe grammar)")
                    elif tag == "vm":
                        rc = run([exe, os.path.join(cross, "keys.txt"), os.path.join(cross, "queries.txt")])
                        if rc == 0:
                            failures.append("volatile cross MUTANT PASSED")
                        else:
                            print("volatile cross mutant correctly failed (rc=%d)" % rc)
            if "--no-engine" not in sys.argv:
                nvse = os.path.join(ROOT, "third_party", "NVSE")
                if os.path.isdir(os.path.join(nvse, "nvse", "nvse")):
                    os.mkdir(os.path.join(tmp, "e"))
                    a = ('/c /std:c++latest /EHsc /Zc:__cplusplus /DRUNTIME=1 /DRUNTIME_VERSION=0x040020D0 '
                         '/FI"%s\\nvse\\nvse\\prefix.h" /Iinclude /I"%s" /I"%s\\nvse" /I"%s\\nvse\\nvse" '
                         "src/snapshot_nvse.cpp src/baseline_nvse.cpp /I\"%s/synccore/include\" /Fo\"%s/e/\"" % (nvse, nvse, nvse, nvse, ROOT.replace("\\", "/"), fw))
                    if cl(tmp, a) != 0:
                        failures.append("engine dumper does not compile against xNVSE")
                    else:
                        print("engine dumper compiled (x86)")
                else:
                    print("SKIP engine compile: third_party/NVSE missing")
        finally:
            shutil.rmtree(tmp, ignore_errors=True)
    print("\nRESULT:", "ALL OK" if not failures else "FAILED: " + "; ".join(failures))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
