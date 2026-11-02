#!/usr/bin/env bash
# Build the nv-oracle tools for 32-bit Windows and test them under Wine
# against a synthetic program (tests/fixture/target.c).
#
# Needs: cargo with the i686-pc-windows-gnu target, the MinGW i686 cross
# compiler (i686-w64-mingw32-gcc, -nm, -objdump, -strip), a 32-bit capable
# Wine, and python3. Nothing from any game is used.
#
# Everything is written under research/nv-oracle/target/wine-tests, which is
# ignored by git. Set WINEPREFIX to reuse a Wine prefix; otherwise a private
# one is created there (the first run takes a while).
set -uo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/.." && pwd)
OUT=$ROOT/target/wine-tests
# PROFILE=debug builds without optimisation and with overflow checks, to
# catch arithmetic mistakes the release build would hide.
PROFILE=${PROFILE:-release}
REL=$ROOT/target/i686-pc-windows-gnu/$PROFILE
BUILD_FLAGS=()
[ "$PROFILE" = release ] && BUILD_FLAGS=(--release)
PY="python3 $HERE/oracle_tests.py"

export WINEDEBUG=-all
export WINEARCH=${WINEARCH:-win32}
export WINEPREFIX=${WINEPREFIX:-$OUT/wineprefix}
CC=${CC:-i686-w64-mingw32-gcc}
NM=${NM:-i686-w64-mingw32-nm}
STRIP=${STRIP:-i686-w64-mingw32-strip}
export OBJDUMP=${OBJDUMP:-i686-w64-mingw32-objdump}
CFLAGS="-O1 -fno-omit-frame-pointer -msse2"

for tool in cargo wine python3 "$CC" "$NM" "$STRIP" "$OBJDUMP"; do
    command -v "$tool" >/dev/null || { echo "missing tool: $tool" >&2; exit 2; }
done

PASSED=()
FAILED=()
group() { # group <name> <command...>
    local name=$1
    shift
    echo
    echo "== $name"
    if "$@"; then PASSED+=("$name"); else FAILED+=("$name"); echo "   -> group failed: $name"; fi
}

# ---------------------------------------------------------------- build
echo "== build"
mkdir -p "$OUT/work" "$OUT/probe"
(cd "$ROOT" && cargo build "${BUILD_FLAGS[@]}" --target i686-pc-windows-gnu) || { echo "cargo build failed" >&2; exit 1; }

F=$HERE/fixture/target.c
$CC $CFLAGS -o "$OUT/fixture_unstripped.exe" "$F" || exit 1
$STRIP -o "$OUT/fixture.exe" "$OUT/fixture_unstripped.exe"
# The same program linked far from 0x00400000 (outside nv-call's placeholder),
# and one linked where the engine DLL lives, to test the failure message.
$CC $CFLAGS -Wl,--image-base=0x04000000 -o "$OUT/fixture_far_unstripped.exe" "$F" || exit 1
$CC $CFLAGS -Wl,--image-base=0x10000000 -o "$OUT/fixture_collide.exe" "$F" || exit 1
$NM "$OUT/fixture_unstripped.exe" > "$OUT/nm.txt"
$NM "$OUT/fixture_far_unstripped.exe" > "$OUT/nm_far.txt"
# The thread fixture: a 4 MiB .bss, a TLS callback, threads (see threadfx.c).
$CC $CFLAGS -o "$OUT/threadfx.exe" "$HERE/fixture/threadfx.c" || exit 1
$NM "$OUT/threadfx.exe" > "$OUT/nm_threadfx.txt"

cp "$REL/nv-call.exe" "$REL/nv_call_engine.dll" "$OUT/fixture.exe" "$OUT/fixture_far_unstripped.exe" "$OUT/fixture_collide.exe" "$OUT/threadfx.exe" "$OUT/work/"
cp "$REL/nv-inject.exe" "$REL/nv_probe.dll" "$OUT/fixture.exe" "$OUT/probe/"

# Ground truth: the program calls its own functions on this CPU.
(cd "$OUT" && wine fixture_unstripped.exe selftest > selftest.txt) || { echo "selftest failed" >&2; exit 1; }
(cd "$OUT" && wine fixture_far_unstripped.exe selftest > selftest_far.txt) || exit 1
echo "selftest: $(grep -c '^CASE ' "$OUT/selftest.txt") cases"

# ---------------------------------------------------------- unit tests
unit_host() {
    local out
    out=$(cd "$ROOT" && cargo test -p nv-oracle-core 2>&1) || { echo "$out" | tail -30; return 1; }
    echo "$out" | grep '^test result'
}
group "core unit tests (host)" unit_host

unit_wine() {
    local out
    out=$(cd "$ROOT" && CARGO_TARGET_I686_PC_WINDOWS_GNU_RUNNER=wine \
        cargo test --release --target i686-pc-windows-gnu -p nv-oracle-core -p nv-inject -p nv-probe 2>&1) ||
        { echo "$out" | tail -30; return 1; }
    echo "$out" | grep '^test result' | grep -v ' 0 passed'
}
group "unit tests (32-bit, under Wine)" unit_wine

decode_crosscheck() {
    # The fixture's own code, and a few larger 32-bit programs if Wine's
    # builtin libraries are installed (any 32-bit PE with a .text works).
    local files=("$OUT/fixture_unstripped.exe") d
    for d in /usr/lib/i386-linux-gnu/wine/i386-windows /usr/lib/wine/i386-windows; do
        for f in kernelbase msvcrt user32; do [ -f "$d/$f.dll" ] && files+=("$d/$f.dll"); done
    done
    $PY decode-crosscheck "$OUT/decode" "${files[@]}"
}
group "decoder: instruction lengths agree with objdump on compiled code" decode_crosscheck

# ------------------------------------------------------------- nv-call
W="$OUT/work"
call() { (cd "$W" && wine nv-call.exe "$@"); }

nv_call_selftest() {
    $PY call-vectors "$OUT/selftest.txt" "$OUT/nm.txt" "$W/vectors.jsonl" &&
        call fixture.exe vectors.jsonl --out results.jsonl &&
        $PY call-check "$OUT/selftest.txt" "$W/results.jsonl" "nv-call vs selftest"
}
group "nv-call: every vector matches the program's own results bit for bit" nv_call_selftest

nv_call_extras() {
    $PY extra-vectors "$OUT/nm.txt" "$W/extra.jsonl" || return 1
    $PY snapshot-files "$OUT/nm.txt" "$W/snap" || return 1
    # Six deliberately bad lines make the exit status 2; each run must still finish.
    call fixture.exe extra.jsonl --out extra_results.jsonl
    [ $? -eq 2 ] || { echo "expected exit status 2 for bad vector lines"; return 1; }
    $PY extra-check "$W/extra_results.jsonl" nosnap "faults, traps, registers, bad input" || return 1
    call fixture.exe extra.jsonl --snapshot snap --out extra_snap_results.jsonl
    [ $? -eq 2 ] || return 1
    $PY extra-check "$W/extra_snap_results.jsonl" snapshot "snapshot regions applied" || return 1
    call fixture.exe extra.jsonl --snapshot snap --keep-state --out extra_snapkeep.jsonl
    [ $? -eq 2 ] || return 1
    $PY extra-check "$W/extra_snapkeep.jsonl" snapshot-keep "snapshot regions, state kept" || return 1
    call fixture.exe extra.jsonl --resolve-imports --out extra_resolve.jsonl
    [ $? -eq 2 ] || return 1
    grep -q '"imports_resolved":true' "$W/extra_resolve.jsonl" || { echo "header does not say imports were resolved"; return 1; }
    $PY extra-check "$W/extra_resolve.jsonl" resolve "resolved imports, exceptions the called code handles itself" || return 1
    call fixture.exe extra.jsonl --keep-state --out extra_keep.jsonl
    [ $? -eq 2 ] || return 1
    $PY extra-check "$W/extra_keep.jsonl" keep "--keep-state carries memory from one vector to the next"
}
group "nv-call: faults, traps, handlers, stack overflow, state restore, imports, snapshots, bad input" nv_call_extras

nv_call_far() {
    $PY call-vectors "$OUT/selftest_far.txt" "$OUT/nm_far.txt" "$W/vectors_far.jsonl" &&
        call fixture_far_unstripped.exe vectors_far.jsonl --out results_far.jsonl &&
        $PY call-check "$OUT/selftest_far.txt" "$W/results_far.jsonl" "nv-call, image outside the placeholder" &&
        grep -q '"placeholder":null' "$W/results_far.jsonl" && grep -q '"image_base":"0x04000000"' "$W/results_far.jsonl"
}
group "nv-call: an image placed outside the host placeholder" nv_call_far

nv_call_collide() {
    local msg
    msg=$(call fixture_collide.exe vectors.jsonl 2>&1 >/dev/null)
    local code=$?
    echo "   exit $code: $msg"
    [ $code -eq 1 ] && echo "$msg" | grep -q "cannot reserve" && echo "$msg" | grep -q "0x10000000"
}
group "nv-call: a taken address range fails with a clear message" nv_call_collide

# Section header fields of a PE file, as lower-case hex without the 0x.
section_va() { $OBJDUMP -h "$1" | awk -v n="$2" '$2 == n { print $4 }'; }
size_of_image() { $OBJDUMP -p "$1" | awk '$1 == "SizeOfImage" { print $2 }'; }
hexadd() { printf '%08x' $(( 16#$1 + $2 )); }

# Run nv-call with a one-file snapshot of 64 bytes at <hexaddr>; it must be
# refused with exit 1 and a message that names the address.
refuse_at() { # refuse_at <label> <hexaddr>
    local msg code
    rm -rf "$W/snap_$1"
    mkdir -p "$W/snap_$1"
    head -c 64 /dev/zero | tr '\0' '\377' > "$W/snap_$1/$2.bin"
    msg=$(call fixture.exe extra.jsonl --snapshot "snap_$1" 2>&1 >/dev/null)
    code=$?
    echo "   $1: exit $code: $msg"
    [ $code -eq 1 ] && echo "$msg" | grep -q "0x$2" && echo "$msg" | grep -q "not part of the mapped image"
}

nv_call_snapshot_refusals() {
    # A snapshot file must not overwrite the harness's own memory, or another
    # snapshot file. The engine's own data section is a place that is certainly taken.
    local va msg code
    va=$(section_va "$W/nv_call_engine.dll" .data)
    [ -n "$va" ] || { echo "cannot find the engine's .data section"; return 1; }
    rm -rf "$W/snap_bad" "$W/snap_overlap"
    mkdir -p "$W/snap_bad" "$W/snap_overlap"
    head -c 4096 /dev/zero | tr '\0' '\377' > "$W/snap_bad/$va.bin"
    msg=$(call fixture.exe extra.jsonl --snapshot snap_bad 2>&1 >/dev/null)
    code=$?
    echo "   exit $code: $msg"
    [ $code -eq 1 ] && echo "$msg" | grep -q "0x$va" && echo "$msg" | grep -q "not part of the mapped image" || return 1
    head -c 8 /dev/zero > "$W/snap_overlap/0a000100.bin"
    head -c 8 /dev/zero > "$W/snap_overlap/0a000104.bin"
    msg=$(call fixture.exe extra.jsonl --snapshot snap_overlap 2>&1 >/dev/null)
    code=$?
    echo "   exit $code: $msg"
    [ $code -eq 1 ] && echo "$msg" | grep -q "overlap" || return 1
    # The host program's own sections are inside the range it reserves, but they
    # are not the placeholder array: its import table (live data), the startup
    # tables behind it (the TLS callback array), and what is left of its code
    # in front of a small image.
    local idata crt bss remnant
    idata=$(section_va "$W/nv-call.exe" .idata)
    crt=$(section_va "$W/nv-call.exe" .CRT)
    bss=$(section_va "$W/nv-call.exe" .bss)
    [ -n "$idata" ] && [ -n "$crt" ] && [ -n "$bss" ] || { echo "cannot find the host's sections"; return 1; }
    refuse_at host_idata "$idata" || return 1
    refuse_at host_crt "$crt" || return 1
    remnant=$(hexadd 400000 $(( 16#$(size_of_image "$W/fixture.exe") + 0x10000 )))
    if [ $(( 16#$remnant )) -lt $(( 16#$bss )) ]; then
        refuse_at host_code "$remnant" || return 1
    else
        echo "   (the host's code ends before the fixture's image does; no leftover code to test)"
    fi
}
group "nv-call: a snapshot over the harness's own memory or another snapshot is refused" nv_call_snapshot_refusals

nv_call_placeholder_array() {
    # The array itself is the image's: a region in it, past the end of a small
    # image, is accepted and read back, and the header reports the range
    # that was reserved without the host's sections behind the array.
    local bss va idata
    bss=$(section_va "$W/nv-call.exe" .bss)
    idata=$(section_va "$W/nv-call.exe" .idata)
    va=$(hexadd "$bss" 0x1000000)
    rm -rf "$W/snap_array"
    mkdir -p "$W/snap_array"
    printf '\x78\x56\x34\x12' > "$W/snap_array/$va.bin"
    $PY deref-vector "$OUT/nm.txt" "$W/deref_array.jsonl" "$va" &&
        call fixture.exe deref_array.jsonl --snapshot snap_array --out deref_array_results.jsonl &&
        $PY deref-check "$W/deref_array_results.jsonl" 0x12345678 "snapshot inside the placeholder array" "$idata"
}
group "nv-call: a snapshot region inside the placeholder array is accepted, and the reserved range ends with the array" nv_call_placeholder_array

nv_call_blocks() {
    # Address space comes in 64 KiB blocks. An image the engine allocated owns
    # the whole last block, so a snapshot file in the pages after its end works.
    local size va msg code block
    size=$(size_of_image "$OUT/fixture_far_unstripped.exe")
    if [ $(( 16#$size % 65536 )) -eq 0 ]; then
        echo "   (the far image ends on a 64 KiB boundary: no tail to test)"
    else
        va=$(hexadd 04000000 $(( 16#$size )))
        rm -rf "$W/snap_tail"
        mkdir -p "$W/snap_tail"
        printf '\x78\x56\x34\x12' > "$W/snap_tail/$va.bin"
        $PY deref-vector "$OUT/nm_far.txt" "$W/deref_tail.jsonl" "$va" &&
            call fixture_far_unstripped.exe deref_tail.jsonl --snapshot snap_tail --out deref_tail_results.jsonl &&
            $PY deref-check "$W/deref_tail_results.jsonl" 0x12345678 "snapshot in the tail of the image's last 64 KiB block" || return 1
    fi
    # Pages after somebody else's allocation are free but cannot be allocated.
    # The error says so and names the block (the engine's own image is one).
    size=$(size_of_image "$W/nv_call_engine.dll")
    if [ $(( 16#$size % 65536 )) -eq 0 ]; then
        echo "   (the engine image ends on a 64 KiB boundary: no tail to test)"
        return 0
    fi
    va=$(hexadd 10000000 $(( 16#$size )))
    rm -rf "$W/snap_tail2"
    mkdir -p "$W/snap_tail2"
    printf '\1\2\3\4' > "$W/snap_tail2/$va.bin"
    $PY deref-vector "$OUT/nm.txt" "$W/deref_tail2.jsonl" "$va" || return 1
    msg=$(call fixture.exe deref_tail2.jsonl --snapshot snap_tail2 2>&1 >/dev/null)
    code=$?
    echo "   exit $code: $msg"
    # The message names the block the tail belongs to (the one the engine's image ends in).
    block=$(printf '0x%08x' $(( (16#$va) & ~0xffff )))
    [ $code -eq 1 ] && echo "$msg" | grep -q "unused tail of a 64 KiB block" && echo "$msg" | grep -q "block at $block"
}
group "nv-call: 64 KiB blocks (the tail of an allocated image's block, the unusable tail of another block)" nv_call_blocks

nv_call_threads() {
    # The thread fixture has a 4 MiB .bss (its image covers the host's startup
    # code), a TLS callback that counts thread starts, and functions that start
    # threads. Natively the callback runs once for the one thread.
    local out code
    out=$(cd "$OUT" && wine threadfx.exe tls)
    echo "   natively: $out"
    echo "$out" | grep -qE '^tls_count=[1-9]' || { echo "the TLS callback does not run natively, so this test would prove nothing"; return 1; }
    $PY thread-vectors "$OUT/nm_threadfx.txt" "$W/threads_ok.jsonl" "$W/threads_fault.jsonl" || return 1
    call threadfx.exe threads_ok.jsonl --resolve-imports --out threads_ok_results.jsonl || { echo "nv-call failed"; return 1; }
    $PY thread-check ok "$W/threads_ok_results.jsonl" "image TLS callbacks are not run in the harness" || return 1
    call threadfx.exe threads_fault.jsonl --resolve-imports --out threads_fault_results.jsonl 2> "$W/threads_fault_stderr.txt"
    code=$?
    echo "   faulting thread: exit $code: $(head -c 300 "$W/threads_fault_stderr.txt")"
    [ $code -eq 4 ] || { echo "expected exit status 4"; return 1; }
    $PY thread-check fault "$W/threads_fault_results.jsonl" "exception on a thread the called code created" "$W/threads_fault_stderr.txt"
}
group "nv-call: image TLS callbacks stay off, and an exception on a callee's thread ends the process with the engine's message" nv_call_threads

nv_call_usage() {
    local out
    out=$(call 2>&1)
    echo "$out" | grep -q "usage" || { echo "no usage text"; return 1; }
    out=$(call missing.exe vectors.jsonl 2>&1) && { echo "a missing image was accepted"; return 1; }
    echo "   $out" | head -1
    echo "$out" | grep -q "cannot read"
}
group "nv-call: usage and missing-file errors" nv_call_usage

# ------------------------------------------------------------ nv-probe
PD="$OUT/probe"
# Start the program suspended, load the probe, run to the end. Prints only the
# program's own output. Fails if nv-inject fails, or if the program's exit
# status is not reported as 0 (--wait comes before --launch; everything after
# the program name belongs to the program).
inj() {
    local raw code
    raw=$(cd "$PD" && wine nv-inject.exe --dll nv_probe.dll --wait --launch fixture.exe "$@" 2>&1)
    code=$?
    printf '%s\n' "$raw" | grep -v '^nv-inject:'
    [ $code -eq 0 ] || { echo "nv-inject exit status $code" >&2; return 1; }
    printf '%s\n' "$raw" | grep -q '^nv-inject: process [0-9]* exited with code 0$' ||
        { echo "nv-inject did not report that the program exited with code 0" >&2; return 1; }
    return 0
}
manifest() { $PY probe-manifest "$1" "$OUT/nm.txt" "$PD/fixture.exe" "$PD/nv-probe.txt" "${@:2}"; }
(cd "$PD" && wine fixture.exe loop 5 > baseline.txt)

probe_main() {
    manifest main output=probe-main.jsonl && rm -f "$PD/probe-main.jsonl" &&
        inj loop 5 > "$PD/prog-main.txt" &&
        $PY probe-check main "$PD/probe-main.jsonl" "$PD/prog-main.txt" "$PD/baseline.txt" "nv-probe hooks" "$PD/fixture.exe"
}
group "nv-probe: arguments, returns, struct dumps, x87 returns, nesting, limits, refusals" probe_main

probe_badsha() {
    manifest badsha output=probe-badsha.jsonl && rm -f "$PD/probe-badsha.jsonl" &&
        inj loop 5 > "$PD/prog-badsha.txt" &&
        $PY probe-check badsha "$PD/probe-badsha.jsonl" "$PD/prog-badsha.txt" "$PD/baseline.txt" "nv-probe wrong host hash" "$PD/fixture.exe"
}
group "nv-probe: a wrong host exe hash refuses every hook" probe_badsha

probe_rel8() {
    manifest rel8 output=probe-rel8.jsonl && rm -f "$PD/nv-probe.jsonl" &&
        inj loop 5 > "$PD/prog-rel8.txt" &&
        $PY probe-check rel8 "$PD/nv-probe.jsonl" "$PD/prog-rel8.txt" "$PD/baseline.txt" "nv-probe short branch"
}
group "nv-probe: a short branch in the stolen bytes is a reported manifest error" probe_rel8

probe_threads() {
    manifest threads output=probe-threads.jsonl flush_ms=100 && rm -f "$PD/probe-threads.jsonl" &&
        inj threads 4 200 > "$PD/prog-threads.txt" &&
        $PY probe-check threads "$PD/probe-threads.jsonl" "$PD/prog-threads.txt" "$PD/baseline.txt" "nv-probe threads"
}
group "nv-probe: four threads, per-thread return tracking" probe_threads

probe_nosuspend() {
    manifest simple 'output=probe-nosuspend-%PID%.jsonl' suspend=0 && rm -f "$PD"/probe-nosuspend-*.jsonl &&
        inj loop 5 > "$PD/prog-nosuspend.txt" &&
        $PY probe-check nosuspend "$(ls "$PD"/probe-nosuspend-*.jsonl | head -1)" "$PD/prog-nosuspend.txt" "$PD/baseline.txt" "nv-probe no suspend"
}
group "nv-probe: patching without suspending threads; %PID% in the output name" probe_nosuspend

probe_pid() {
    manifest simple output=probe-pid.jsonl mode=inject || return 1
    rm -f "$PD/probe-pid.jsonl" "$PD/pidprog.txt"
    (cd "$PD" && wine fixture.exe loop 3 5000 > pidprog.txt) &
    local bg=$! pid=""
    for _ in $(seq 1 100); do
        pid=$(sed -n 's/^waiting pid=\([0-9]*\).*/\1/p' "$PD/pidprog.txt" 2>/dev/null)
        [ -n "$pid" ] && break
        sleep 0.2
    done
    [ -n "$pid" ] || { echo "the program did not start"; return 1; }
    (cd "$PD" && wine nv-inject.exe --dll nv_probe.dll --pid "$pid") || return 1
    wait $bg
    $PY probe-check inject-pid "$PD/probe-pid.jsonl" "$PD/pidprog.txt" "$PD/baseline.txt" "nv-probe injected into a running process"
}
group "nv-probe: injected into a running process (--pid)" probe_pid

probe_auto() {
    manifest simple output=probe-auto.jsonl mode=auto grace=300 && rm -f "$PD/probe-auto.jsonl" &&
        inj loop 5 1500 > "$PD/prog-auto.txt" &&
        $PY probe-check auto "$PD/probe-auto.jsonl" "$PD/prog-auto.txt" "$PD/baseline.txt" "nv-probe auto mode"
}
group "nv-probe: auto mode starts on its own thread when NVSE never calls" probe_auto

nvse_runs() {
    manifest simple output=probe-nvse.jsonl mode=nvse || return 1
    rm -f "$PD/probe-nvse.jsonl"
    (cd "$PD" && wine fixture.exe nvse-sim nv_probe.dll > prog-nvse.txt) &&
        $PY nvse-check "$PD/probe-nvse.jsonl" "$PD/prog-nvse.txt" normal "NVSE query and load" || return 1
    # With return capture the module pins itself, so FreeLibrary leaves the hooks working.
    rm -f "$PD/probe-nvse.jsonl"
    (cd "$PD" && wine fixture.exe nvse-sim nv_probe.dll free > prog-nvse-free-pinned.txt) &&
        $PY nvse-check "$PD/probe-nvse.jsonl" "$PD/prog-nvse-free-pinned.txt" free-pinned "NVSE unload with return capture keeps the module" || return 1
    # Without it the unload puts the original bytes back.
    manifest noret output=probe-nvse.jsonl mode=nvse || return 1
    rm -f "$PD/probe-nvse.jsonl"
    (cd "$PD" && wine fixture.exe nvse-sim nv_probe.dll free > prog-nvse-free.txt) &&
        $PY nvse-check "$PD/probe-nvse.jsonl" "$PD/prog-nvse-free.txt" free "NVSE unload restores the code" || return 1
    rm -f "$PD/probe-nvse.jsonl"
    (cd "$PD" && wine fixture.exe nvse-sim nv_probe.dll editor > prog-nvse-editor.txt) &&
        $PY nvse-check "$PD/probe-nvse.jsonl" "$PD/prog-nvse-editor.txt" editor "NVSE editor refusal" || return 1
    manifest simple output=probe-auto-nvse.jsonl mode=auto grace=200 || return 1
    rm -f "$PD/probe-auto-nvse.jsonl"
    (cd "$PD" && wine fixture.exe nvse-sim nv_probe.dll wait 900 > prog-auto-nvse.txt) &&
        $PY nvse-check "$PD/probe-auto-nvse.jsonl" "$PD/prog-auto-nvse.txt" normal "auto mode defers to NVSE"
}
group "nv-probe: NVSE plugin exports, editor refusal, unload, auto mode deferring to NVSE" nvse_runs

probe_tail() {
    manifest tail output=probe-tail.jsonl && rm -f "$PD/probe-tail.jsonl" &&
        inj loop 5 > "$PD/prog-tail.txt" &&
        $PY probe-check tail "$PD/probe-tail.jsonl" "$PD/prog-tail.txt" "$PD/baseline.txt" "nv-probe tail jump between hooked functions" "$PD/fixture.exe"
}
group "nv-probe: a jump from one return-captured function into another" probe_tail

probe_nosha() {
    manifest nosha output=probe-nosha.jsonl && rm -f "$PD/probe-nosha.jsonl" &&
        inj loop 5 > "$PD/prog-nosha.txt" &&
        $PY probe-check nosha "$PD/probe-nosha.jsonl" "$PD/prog-nosha.txt" "$PD/baseline.txt" "nv-probe host hash without host_sha256" "$PD/fixture.exe"
}
group "nv-probe: the host exe hash is logged even when the manifest does not ask" probe_nosha

probe_unload_inside() {
    manifest sleepy output=probe-sleepy.jsonl mode=inject && rm -f "$PD/probe-sleepy.jsonl" &&
        (cd "$PD" && wine fixture.exe unload-inside nv_probe.dll > prog-sleepy.txt) &&
        $PY probe-check sleepy "$PD/probe-sleepy.jsonl" "$PD/prog-sleepy.txt" "$PD/baseline.txt" "nv-probe unload while a thread is inside a hooked function"
}
group "nv-probe: unloading while a thread is inside a return-captured function" probe_unload_inside

inject_wait() {
    manifest simple output=probe-inject-wait.jsonl || return 1
    local out code t0 t1
    # The program's own exit status comes through, and nv-inject passes it on as 3.
    out=$(cd "$PD" && wine nv-inject.exe --dll nv_probe.dll --wait --launch fixture.exe bogus-mode 2>&1)
    code=$?
    echo "$out" | grep -q 'exited with code 2$' || { echo "exit code 2 was not reported: $out"; return 1; }
    [ $code -eq 3 ] || { echo "nv-inject exit status $code, wanted 3"; return 1; }
    # --wait before --launch really waits: the program sleeps one second before it works.
    t0=$(date +%s%N)
    out=$(cd "$PD" && wine nv-inject.exe --dll nv_probe.dll --wait --launch fixture.exe loop 1 1000 2>&1) || { echo "$out"; return 1; }
    t1=$(date +%s%N)
    echo "$out" | grep -q 'exited with code 0$' || { echo "exit code 0 was not reported: $out"; return 1; }
    [ $(( (t1 - t0) / 1000000 )) -ge 900 ] || { echo "nv-inject returned before the program ended"; return 1; }
    # Written after the program name, --wait belongs to the program, and nv-inject says so.
    out=$(cd "$PD" && wine nv-inject.exe --dll nv_probe.dll --launch fixture.exe loop 1 --wait 2>&1) || { echo "$out"; return 1; }
    echo "$out" | grep -q 'passed to the program' || { echo "no warning about a trailing --wait: $out"; return 1; }
    sleep 1
    rm -f "$PD/probe-inject-wait.jsonl"
}
group "nv-inject: --wait before --launch waits and reports the program's exit code" inject_wait

# -------------------------------------------------------------- summary
echo
echo "== summary"
echo "passed: ${#PASSED[@]}"
for g in "${PASSED[@]}"; do echo "  ok    $g"; done
echo "failed: ${#FAILED[@]}"
for g in "${FAILED[@]}"; do echo "  FAIL  $g"; done
[ ${#FAILED[@]} -eq 0 ]
