#!/usr/bin/env python3
"""One-command L8 gate: regenerate INFO stats + M1 scenarios, build (MSVC x86 via vcvars32) and run
   dialogue_tests, adapter_tests, engine_tests and the scenario-spec tests.
   python plugin/dialogue/run_tests.py"""
import os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, '..', '..'))
OUT = os.path.join(REPO, 'out', 'dialogue')
sys.path.insert(0, os.path.join(REPO, 'tools', 'build'))
import vsroot  # noqa: E402
VC = os.path.join(vsroot.find(), r'VC\Auxiliary\Build\vcvars32.bat')
SYN = os.path.join(REPO, 'synccore')
os.makedirs(OUT, exist_ok=True)
CRLF = chr(13) + chr(10)

TARGETS = [  # (exe, sources)
    ('dialogue_tests', ['tests/main.cpp']),
    ('adapter_tests', ['tests/adapter_test.cpp', os.path.join(SYN, 'src', 'transport.cpp')]),
    ('engine_tests', ['tests/engine_test.cpp', 'engine/info_reader.cpp', os.path.join(SYN, 'src', 'transport.cpp')]),
]


def build(exe, srcs):
    bat = os.path.join(OUT, 'build_%s.bat' % exe)
    full = [s if os.path.isabs(s) else os.path.join(HERE, s) for s in srcs]
    obj = os.path.join(OUT, exe + '_obj')
    os.makedirs(obj, exist_ok=True)
    lines = ['@echo off', 'call "%s" >nul' % VC,
             'cl /nologo /std:c++17 /EHsc /W4 /permissive- /utf-8 /I"%s" /I"%s" /I"%s" %s /Fo"%s/" /Fe"%s"' % (
                 os.path.join(HERE, 'include'), os.path.join(HERE, 'engine'), os.path.join(SYN, 'include'),
                 ' '.join('"%s"' % f for f in full), obj, os.path.join(OUT, exe + '.exe'))]
    with open(bat, 'w', newline='') as f:
        f.write('\r\n'.join(lines) + '\r\n')
    r = subprocess.run(['cmd', '/c', bat], capture_output=True, text=True)
    if r.returncode:
        print(r.stdout, r.stderr)
        sys.exit('build failed: ' + exe)
    warn = [l for l in r.stdout.splitlines() if 'warning' in l]
    if warn:
        print('\n'.join(warn))


def run(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True)
    print(r.stdout, r.stderr)
    return r.returncode


def compile_xnvse():
    """Compile-only check of engine/engine_xnvse.cpp against the xNVSE headers (x86, same flags as nvcoop)."""
    n = os.path.join(REPO, 'third_party', 'NVSE')
    bat = os.path.join(OUT, 'build_engine_xnvse.bat')
    lines = ['@echo off', 'call "%s" >nul' % VC,
             'cl /nologo /c /std:c++latest /EHsc /Zc:__cplusplus /permissive- /DRUNTIME=1 /DRUNTIME_VERSION=0x040020D0 '
             '/DWIN32 /D_WINDOWS /D_CRT_SECURE_NO_WARNINGS /FI"%s" /I"%s" /I"%s" /I"%s" /I"%s" /I"%s" /I"%s" /I"%s" /I"%s" "%s" /Fo"%s"' % (
                 os.path.join(n, 'nvse', 'nvse', 'prefix.h'), n, os.path.join(n, 'nvse'), os.path.join(n, 'nvse', 'nvse'),
                 os.path.join(HERE, 'include'), os.path.join(HERE, 'engine'), os.path.join(SYN, 'include'),
                 os.path.join(REPO, 'plugin', 'hooks_ui'), os.path.join(REPO, 'plugin', 'hooks'),
                 os.path.join(HERE, 'engine', 'engine_xnvse.cpp'), os.path.join(OUT, 'engine_xnvse.obj'))]
    with open(bat, 'w', newline='') as f:
        f.write(CRLF.join(lines) + CRLF)
    r = subprocess.run(['cmd', '/c', bat], capture_output=True, text=True)
    print('[%s] compile engine_xnvse.cpp (x86, xNVSE headers)' % ('PASS' if r.returncode == 0 else 'FAIL'))
    if r.returncode:
        print(r.stdout, r.stderr)
    return r.returncode


rc = 0
for tool in (['tools/info_stats.py', '--md', 'tools/info_stats.md'], ['tools/gen_m1_dialogue.py'], ['tools/gen_companion_topics.py'], ['tools/gen_consent_messages.py']):
    s = subprocess.run([sys.executable, '-I', os.path.join(HERE, tool[0])] + [
        os.path.join(HERE, a) if a.endswith('.md') else a for a in tool[1:]], capture_output=True, text=True)
    print(s.stdout.strip() or '%s ok' % tool[0], s.stderr.strip())
    rc |= s.returncode
for exe, srcs in TARGETS:
    build(exe, srcs)
    rc |= run([os.path.join(OUT, exe + '.exe')] + (['--group', 'all'] if exe == 'dialogue_tests' else []))
rc |= compile_xnvse()
rc |= run([sys.executable, '-I', os.path.join(HERE, 'tests', 'test_scenarios.py')])
sys.exit(1 if rc else 0)
