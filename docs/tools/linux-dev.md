# Working on Linux (no game)

What a Linux machine can build and test, and how. Everything here is game-free. The game, xNVSE, MSVC and the
in-game tests need Windows (see `docs/INGAME-TESTS.md`).

## What runs and what does not
| Runs on Linux | Needs Windows |
|---|---|
| `python -I tools/test_all.py --quick`: every Python suite that needs no game data, MSVC or a game copy | `nvcoop.dll` and every MSVC/CMake suite (`build_l1.cmd`, the `*.cmd` runners) |
| synccore: build, `ctest`, the loopback and chaos-relay tests, simfuzz | the in-game tests, `prove.py queue`, the runner and pool |
| synccore under ASan, UBSan and TSan (g++ or clang) | the installer's real Steam lookup (its tests run on Linux with a fake game folder) |

Suites that need something Linux does not have print `SKIP <name>: <reason>`. On Linux nothing should print `FAIL`; a
`FAIL` that does not happen on Windows is a portability bug worth an issue.

## Python suites
Python **3.11** (the version CI and Windows use). One way to get it without root:
```
uv python install 3.11
python3.11 -I tools/test_all.py --quick
```
The quick suite takes several minutes; the slowest parts are the proof harness and the T7 animation checks.

## synccore
Needs CMake, Ninja, a C++20 compiler (g++ or clang), Python 3 (some tests call it) and the **system** ENet with its
headers. `third_party/enet` holds only the Windows backend, so it is not used here. Package names:
`cmake ninja-build g++ libenet-dev python3` (Debian/Ubuntu) or `cmake ninja gcc clang enet python` (Arch).
```
cmake -S synccore -B build -G Ninja
cmake --build build
ctest --test-dir build -j 4 --output-on-failure
```

### Sanitizers (as CI runs them)
```
F="-fsanitize=address,undefined -fno-sanitize-recover=undefined -fno-omit-frame-pointer -O1 -g"
cmake -S synccore -B build-san -G Ninja -DCMAKE_BUILD_TYPE=Debug "-DCMAKE_C_FLAGS=$F" "-DCMAKE_CXX_FLAGS=$F" "-DCMAKE_EXE_LINKER_FLAGS=$F"
cmake --build build-san
ASAN_OPTIONS=detect_leaks=1 ctest --test-dir build-san -j 2 -E '^(unit_perf|twin_perf_regression_still_fails|flight_perf|twin_unit_async_completion_leak)$'
```
Left out: the perf tests, whose wall-clock budgets mean nothing at sanitizer speed, and `twin_unit_async_completion_leak`,
which puts a reference cycle back on purpose, so LeakSanitizer rightly fails it.

ThreadSanitizer, in a separate build folder:
```
F="-fsanitize=thread -fno-omit-frame-pointer -O1 -g"
cmake -S synccore -B build-tsan -G Ninja -DCMAKE_BUILD_TYPE=Debug "-DCMAKE_C_FLAGS=$F" "-DCMAKE_CXX_FLAGS=$F" "-DCMAKE_EXE_LINKER_FLAGS=$F"
cmake --build build-tsan
TSAN_OPTIONS="second_deadlock_stack=1 log_path=$PWD/tsan-reports/tsan" ctest --test-dir build-tsan -j 2 -E '^(unit_perf|twin_perf_regression_still_fails|flight_perf)$'
ls tsan-reports   # must be missing or empty: a report inside a WILL_FAIL test does not fail ctest
```
Only the perf tests are left out. `twin_unit_async_completion_leak` stays in: it passes when its own check catches the
reference cycle, and only LeakSanitizer, which a TSan build does not have, fails it for the leak it plants. On kernels with 32-bit mmap randomization (the GitHub Ubuntu runners, for
example) TSan can stop with "unexpected memory mapping"; `sudo sysctl -w vm.mmap_rnd_bits=28` fixes that.

**Memory:** a sanitizer compile takes about 1 GB per compiler process. On a machine with 16 GB or less, build with
`--parallel 3` (or `ninja -j3`) and run one sanitizer build or test run at a time.

## SteamOS (Steam Deck)
The root file system is read-only and a system update wipes packages installed with pacman, so do not install the
toolchain on the host (and do not run `steamos-readonly disable`). Use a container instead:
```
distrobox create --name fnvbuild --image docker.io/library/archlinux:latest
distrobox enter fnvbuild -- sudo pacman -Syu --needed cmake ninja gcc clang enet python
distrobox enter fnvbuild -- bash -c 'cmake -S synccore -B /tmp/sc -G Ninja && ninja -C /tmp/sc && ctest --test-dir /tmp/sc -j 4'
```
The container shares your home folder, so the checkout is the same one. Python 3.11 from `uv` (above) works on the
host as well.

## Known gaps
- `loopback_faults` and `loopback_unreliable` time out on the host in a few percent of runs (#52); rerun before you
  read a single red run as a regression.
- The chaos relay's `hell` profile is too slow for its 90 s budget and is not gated (#48).
