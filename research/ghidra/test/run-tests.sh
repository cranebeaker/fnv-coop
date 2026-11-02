#!/usr/bin/env bash
# Tests the Ghidra scripts in research/ghidra against a synthetic 32-bit PE.
#
# What it does:
#   1. Builds test/fixture.c with the MinGW i686 cross compiler (-O1 -msse2),
#      keeps an unstripped copy only to read symbol addresses with nm, and
#      strips the copy that Ghidra analyzes.
#   2. Imports the stripped exe into a throwaway Ghidra project under
#      research/ghidra/target/ with analyzeHeadless.
#   3. Runs each script (read-only for the exports, on a project copy for the
#      labeling scripts) and checks the outputs with test/check.py.
#   4. Runs mutated copies of the scripts (made with test/mutate.py) to show
#      that the safety nets work: the export gate, the decompile outcomes
#      (no result, timeout, error) and the rollback when a name script fails
#      while it is applying. Also runs the duplicate-name cases, the vtable
#      label forms, the label script's handling of pointers that are not code,
#      the tier through callees by routine name and unit cases for NvCommon.
#
# Needs: Ghidra 12.1.x (GHIDRA_INSTALL_DIR; the Linux test container has it in
# /opt/tools/ghidra_12.1.4_PUBLIC, which is the fallback),
# a JDK on PATH, i686-w64-mingw32-gcc/nm/strip, python3. No game files and no
# game executable are used or needed. Everything is written under target/.
#
# Usage: research/ghidra/test/run-tests.sh

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)" # research/ghidra
TARGET="$ROOT/target"
OUT="$TARGET/test-out"
LOGS="$OUT/logs"
# Ghidra 12.1.x install directory: GHIDRA_INSTALL_DIR, or the location the
# Linux test container uses.
GHIDRA="${GHIDRA_INSTALL_DIR:-/opt/tools/ghidra_12.1.4_PUBLIC}"
HEADLESS="$GHIDRA/support/analyzeHeadless"
CC="${NV_TEST_CC:-i686-w64-mingw32-gcc}"
NM="${NV_TEST_NM:-i686-w64-mingw32-nm}"
STRIP="${NV_TEST_STRIP:-i686-w64-mingw32-strip}"
PROJ_NAME=nvtest

for tool in "$HEADLESS" "$(command -v "$CC")" "$(command -v "$NM")" "$(command -v "$STRIP")" "$(command -v python3)"; do
    if [ -z "$tool" ] || [ ! -e "$tool" ]; then
        echo "missing tool: $tool (see the header of this script)" >&2
        exit 2
    fi
done

now_ms() { echo $(($(date +%s%N) / 1000000)); }
SUITE_START=$(now_ms)
declare -a TIMINGS=()
note_time() { # name start_ms
    TIMINGS+=("$(printf '%-34s %6d ms' "$1" $(($(now_ms) - $2)))")
}

fail() {
    echo "FAILED: $*" >&2
    exit 1
}

# run_ghidra <name> <ok|fail> <analyzeHeadless args...>
# Runs analyzeHeadless, keeps the log, and decides success from the log:
# analyzeHeadless exits 0 even when a script throws, so the log is the evidence.
# It also exits 0 when it could not open the program at all (a locked project, a full disk), and
# then no script runs. So the run only counts when the exit status is 0, the log has none of the
# open or abort messages, and every -postScript in the arguments was started.
run_ghidra() {
    local name="$1" expect="$2"
    shift 2
    local t0 rc=0 script_error=0 wanted=0 started a
    for a in "$@"; do
        if [ "$a" = -postScript ]; then
            wanted=$((wanted + 1))
        fi
    done
    t0=$(now_ms)
    "$HEADLESS" "$@" >"$LOGS/$name.log" 2>&1 || rc=$?
    note_time "ghidra: $name" "$t0"
    if grep -q "REPORT SCRIPT ERROR" "$LOGS/$name.log"; then
        script_error=1
    fi
    if [ "$rc" -ne 0 ]; then
        tail -30 "$LOGS/$name.log" >&2
        fail "analyzeHeadless exited with $rc ($name)"
    fi
    if grep -Eq "Abort due to Headless analyzer error|Open failed|Error during analysis" "$LOGS/$name.log"; then
        grep -E "Abort due to Headless analyzer error|Open failed|Error during analysis" "$LOGS/$name.log" | head -5 >&2
        fail "analyzeHeadless could not open or analyze the program ($name)"
    fi
    started=$(grep -c '^INFO  SCRIPT: ' "$LOGS/$name.log" || true)
    if [ "$started" -ne "$wanted" ]; then
        fail "$name: $wanted script(s) were requested but $started started"
    fi
    if [ "$expect" = ok ] && [ "$script_error" -eq 1 ]; then
        grep -A6 "REPORT SCRIPT ERROR" "$LOGS/$name.log" | head -20 >&2
        fail "a script failed in $name"
    fi
    if [ "$expect" = fail ] && [ "$script_error" -eq 0 ]; then
        fail "$name should have failed with a script error but did not"
    fi
    return 0
}

check() { # stages...
    local t0
    t0=$(now_ms)
    python3 "$HERE/check.py" "$@" --out "$OUT" --nm "$TARGET/nm.txt" --exe "$TARGET/nvfixture.exe" ||
        fail "check.py $*"
    note_time "check: $*" "$t0"
}

sym() { # symbol name in the fixture -> 8 hex digits
    local v
    v=$(awk -v s="_$1" '$3 == s { print $1; exit }' "$TARGET/nm.txt")
    [ -n "$v" ] || fail "symbol $1 not found in nm output"
    echo "$v"
}

# mutate <dir> <script.java> <anchor> <replacement>: puts a copy of a script with one exact text
# replaced into $STAGE/<dir>, next to a copy of NvCommon.java. mutate.py fails if the anchor
# is not found exactly once, so a moved anchor cannot turn a test into a no-op.
mutate() {
    local dir="$STAGE/$1"
    mkdir -p "$dir"
    cp "$ROOT/NvCommon.java" "$dir/"
    python3 "$HERE/mutate.py" "$ROOT/$2" "$dir/$2" "$3" "$4" || fail "mutation of $2 did not apply"
}

# ---------------------------------------------------------------- build
rm -rf "$TARGET"
mkdir -p "$TARGET" "$LOGS"
# Modified copies of scripts and the helper scripts go in a temporary folder OUTSIDE research/ghidra.
# Ghidra compiles every .java file below a script directory, sub-folders included, and when two
# files define the same class it keeps only one. A mutated copy anywhere below research/ghidra
# (target/ too) can therefore replace the real script: a test run did exactly that to a later
# export. Nothing with a .java extension may be left under target/ (checked at the end).
STAGE="$(mktemp -d "${TMPDIR:-/tmp}/nv-ghidra-test.XXXXXX")"
trap 'rm -rf "$STAGE"' EXIT
# Run from a folder that holds no file named like a script. Ghidra reads an argument such as
# NvImportNameMap.java as a path first, so from research/ghidra it would open the real script
# and not the mutated copy that -scriptPath points to.
cd "$TARGET"
T0=$(now_ms)
(
    cd "$TARGET"
    # --pdb makes ld write a CodeView record (RSDS) into the exe so the
    # identity script has one to read. The .pdb itself is deleted straight
    # away: if it sat next to the exe, Ghidra would load names from it. The
    # name is long on purpose: check.py later writes a path under a fake
    # profile folder over it in a copy of the exe, which needs room.
    PDB_NAME=nvfixture-pdb-name-with-room-for-a-longer-path.pdb
    "$CC" -O1 -msse2 -Wall -Wl,--large-address-aware -Wl,--pdb="$PDB_NAME" \
        -o fixture_unstripped.exe "$HERE/fixture.c"
    rm -f "$PDB_NAME"
    cp fixture_unstripped.exe nvfixture.exe
    "$STRIP" nvfixture.exe
    "$NM" fixture_unstripped.exe >nm.txt
)
note_time "build fixture" "$T0"
echo "built $(wc -c <"$TARGET/nvfixture.exe") byte stripped exe; unstripped copy used only for nm"

# Optional: prove the fixture is a working PE by running the stripped exe under Wine.
if [ "${NV_TEST_RUN_FIXTURE:-0}" = 1 ] && command -v wine >/dev/null; then
    T0=$(now_ms)
    WINEDEBUG=-all WINEPREFIX="${WINEPREFIX:-$TARGET/wineprefix}" wine "$TARGET/nvfixture.exe" >"$LOGS/fixture-run.log" 2>&1 ||
        fail "the fixture exe did not run under Wine"
    grep -q '^fixture ' "$LOGS/fixture-run.log" || fail "unexpected fixture output"
    note_time "run fixture under wine" "$T0"
fi

# ---------------------------------------------------------------- import and analyze
mkdir -p "$TARGET/proj"
run_ghidra import ok "$TARGET/proj" "$PROJ_NAME" -import "$TARGET/nvfixture.exe" -overwrite
grep -q "Import succeeded" "$LOGS/import.log" || fail "import did not succeed"

SP=(-scriptPath "$ROOT")
RO=(-process nvfixture.exe -noanalysis -readOnly)
RW=(-process nvfixture.exe -noanalysis)

CARD_ADDRS=$(
    for n in fx_angle_to_unit fx_double_scale fx_extended fx_possible_float_immediate fx_sse_scalar fx_rsqrt \
        fx_rcp fx_sin fx_set_cw fx_load_cw_const fx_load_mxcsr_const fx_sum fx_gain Obj_Compute main \
        fx_cvt_sd2ss fx_cvt_ss2sd fx_cvt_tss2si fx_cvt_tsd2si fx_cvt_ps2pd fx_cvt_pd2ps fx_cvt_dq2pd fx_cvt_si2sd \
        fx_sin_top fx_chain0 fx_chain8 fx_ping fx_crt_top fx_import_cos fx_import_cos_top fx_import_tick \
        fx_table_call fx_movq_const fx_movd_const; do
        printf '%s,' "$(sym $n)"
    done
)
CARD_ADDRS="${CARD_ADDRS%,}"

# ---------------------------------------------------------------- A: identity, export, cards (read-only)
run_ghidra read_only_scripts ok "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvExeIdentity.java out="$OUT/identity.json" \
    -postScript NvExportProgram.java out="$OUT/export1" threads=3 \
    -postScript NvFunctionCard.java addr="$CARD_ADDRS" out="$OUT/cards" \
    -postScript NvFunctionCard.java addr="$(sym fx_chain0)" depth=12 out="$OUT/cards_deep"
check identity export1 cards

# A bad card address must fail loudly and write nothing.
run_ghidra card_bad_address fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvFunctionCard.java addr="$(sym fx_sum),00000001" out="$OUT/cards_bad"
grep -q "no function starts at 00000001" "$LOGS/card_bad_address.log" || fail "card error text missing"
[ ! -e "$OUT/cards_bad" ] || fail "a card was written for a bad address list"
echo "  ok   card with an address that is not a function fails and writes nothing"

# run_ghidra must not accept a run that opened nothing. A program name that is not in the project
# makes analyzeHeadless exit with 1. A project whose program database is damaged makes it exit
# with 0, log "Error during analysis" and run no script, which only the log shows. Each call runs
# in a subshell, because a failing run_ghidra ends the shell it runs in.
if (run_ghidra open_wrong_name ok "$TARGET/proj" "$PROJ_NAME" -process no-such-program.exe -noanalysis -readOnly \
    "${SP[@]}" -postScript NvExeIdentity.java out="$OUT/never_identity.json") >/dev/null 2>&1; then
    fail "run_ghidra accepted a run whose program is not in the project"
fi
cp -a "$TARGET/proj" "$TARGET/proj_damaged"
find "$TARGET/proj_damaged" -name 'db.1.gbf' -exec sh -c 'head -c 4096 /dev/zero >"$1"' _ {} \;
if (run_ghidra open_damaged ok "$TARGET/proj_damaged" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvExeIdentity.java out="$OUT/never_identity.json") >/dev/null 2>&1; then
    fail "run_ghidra accepted a run that could not open its program"
fi
grep -q "Error during analysis" "$LOGS/open_damaged.log" || fail "the damaged project did not fail the way the test assumes"
[ "$(grep -c '^INFO  SCRIPT: ' "$LOGS/open_damaged.log" || true)" -eq 0 ] || fail "a script ran on the damaged project"
[ ! -e "$OUT/never_identity.json" ] || fail "a script wrote output on a run that opened nothing"
echo "  ok   run_ghidra rejects a wrong program name (exit 1) and a damaged project (exit 0, no script ran)"
rm -rf "$TARGET/proj_damaged"

# The Rich header cannot come from MinGW, so the test writes one into a copy of the fixture's DOS
# stub, imports that copy into its own project and reads its identity.
check richgen
mkdir -p "$TARGET/proj_rich"
run_ghidra identity_rich ok "$TARGET/proj_rich" "$PROJ_NAME" -import "$OUT/nvrich.exe" -overwrite -noanalysis \
    "${SP[@]}" -postScript NvExeIdentity.java out="$OUT/identity_rich.json"
check rich

# ---------------------------------------------------------------- B: labeling on a copy of the project
cp -a "$TARGET/proj" "$TARGET/proj_named"
TABLE=$(sym g_commands)
T=0x$TABLE
run_ghidra label_1 ok "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvLabelCommandTables.java table="$T" count=6 stride=48 dry=1 report="$OUT/label_stride48.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 dry=1 report="$OUT/label_dry.csv" \
    -postScript NvLabelCommandTables.java table="$T" end=0x$(printf '%08x' $((0x$TABLE + 8 * 40))) report="$OUT/label_real.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 report="$OUT/label_again.csv"
grep -q "dry run, nothing changed. Would create" "$LOGS/label_1.log" || fail "dry run message missing"

# The stride check as the README runs it: dry runs with -readOnly on a project that is not labeled.
run_ghidra stride_check_readonly ok "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvLabelCommandTables.java table="$T" count=8 stride=40 dry=1 report="$OUT/ro_stride40.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=6 stride=48 dry=1 report="$OUT/ro_stride48.csv"
grep -q "stride 40: 8 entries, 5 valid, 3 skipped" "$LOGS/stride_check_readonly.log" || fail "stride 40 summary line missing"
grep -q "stride 48: 6 entries" "$LOGS/stride_check_readonly.log" || fail "stride 48 summary line missing"
! grep -q "stride 48: 6 entries, 6 valid" "$LOGS/stride_check_readonly.log" || fail "a wrong stride should not give all valid entries"
echo "  ok   read-only dry runs at stride 40 and 48 work, and only 40 gives the valid entries"

# Names import: main file, dry run, repeat, RVA form, force.
A_ANGLE=$(sym fx_angle_to_unit)
A_FP=$(sym fp_only_target)
A_DISPATCH=$(sym g_dispatch)
A_ALPHA=$(sym cmd_alpha_execute)
A_GAIN_VAR=$(sym g_gain)
A_EXT=$(sym fx_extended)
A_SUM=$(sym fx_sum)
A_GAIN_FN=$(sym fx_gain)
A_SSE=$(sym fx_sse_scalar)
RVA_SUM=$(printf '0x%x' $((0x$A_SUM - 0x400000)))

cat >"$OUT/names_main.csv" <<EOF
address,name,source,pin,kind
$A_ANGLE,Fixture::AngleToUnit,own,0123456789abcdef0123,
$A_FP,FpOnlyTarget,xnvse,fb0f4e95ac46c7cf49b2c68b2d222b362ea250ef,
$A_DISPATCH,Fixture::vftable,own,,label
$A_ALPHA,UserAlpha,jip,1111111111111111,
$A_GAIN_VAR,g_Gain,own,,
$A_EXT,  Fixture::Ext Math  ,own,aaaa,
EOF
cat >"$OUT/names_dry.csv" <<EOF
address,name,source,pin,kind
$A_GAIN_FN,DryRunOnly,own,deadbeef,
EOF
cat >"$OUT/names_force.csv" <<EOF
address,name,source,pin,kind
$A_ALPHA,UserAlpha,jip,1111111111111111,
EOF
cat >"$OUT/names_rva.csv" <<EOF
address,name,source,pin,kind
$RVA_SUM,FxSum,own,cafebabe1234,function
EOF
# A CSV as Windows PowerShell writes it: byte-order mark and CRLF line ends.
printf '\357\273\277address,name,source,pin,kind\r\n%s,BomName,own,deadbeef,\r\n' "$A_GAIN_FN" >"$OUT/names_bom.csv"
run_ghidra names_1 ok "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_dry.csv" dry=1 report="$OUT/names_dry.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_bom.csv" dry=1 report="$OUT/names_bom.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_main.csv" report="$OUT/names_main.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_main.csv" report="$OUT/names_again.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_rva.csv" rva=1 report="$OUT/names_rva.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_force.csv" force=1 report="$OUT/names_force.report.csv"

# A CSV with bad rows must apply nothing. The first file has rows that fail the row checks. In
# the second, a row passes those but cannot be applied (it names a function at a data address):
# the simulated run of the whole file finds that, so the earlier row is never applied. The
# rollback of a failure that only shows up while applying is tested further down with mutated
# copies of the scripts.
cat >"$OUT/names_bad.csv" <<EOF
address,name,source,pin,kind
$A_SSE,ShouldNotApply,own,abc,
$(sym fx_rcp),bad name!,own,abc,
$(sym fx_rsqrt),OkName,bad source,abc,
00001000,Unmapped,own,abc,
EOF
cat >"$OUT/names_unappliable.csv" <<EOF
address,name,source,pin,kind
$A_SSE,ShouldNotApply,own,abc,
$A_GAIN_VAR,NotAFunction,own,abc,function
EOF
run_ghidra names_bad fail "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_bad.csv" report="$OUT/names_bad.report.csv"
grep -q "row(s) rejected" "$LOGS/names_bad.log" || fail "rejected-row error text missing"
run_ghidra names_unappliable fail "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_unappliable.csv" report="$OUT/names_unappliable.report.csv"
grep -q "cannot create a function here" "$OUT/names_unappliable.report.csv" || fail "unappliable-row report text missing"

# An address inside a function (not at its entry) with no kind is a conflict, not a silent label.
# kind=label is the way to ask for a label inside the function. A dry run on the unlabeled project.
cat >"$OUT/names_inside.csv" <<EOF
address,name,source,pin,kind
$(printf '%08x' $((0x$A_SUM + 1))),InsideRow,own,aaaa,
$(printf '%08x' $((0x$A_GAIN_FN + 1))),InsideLabel,own,bbbb,label
$(sym cmd_badcode_execute),NotCode,own,cccc,
EOF
run_ghidra names_inside ok "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_inside.csv" dry=1 report="$OUT/names_inside.report.csv"
check names_inside
# The same bytes with kind=function cannot become a function: the row is rejected, so the run fails.
cat >"$OUT/names_badcode_fn.csv" <<EOF
address,name,source,pin,kind
$(sym cmd_badcode_execute),NotCodeFunction,own,dddd,function
EOF
run_ghidra names_badcode_fn fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_badcode_fn.csv" dry=1 report="$OUT/names_badcode_fn.report.csv"
check names_badcode_fn

# After the forced name import the function has the import's tags and no src:cmdtable.
mkdir -p "$STAGE/testscripts"
cp "$ROOT/NvCommon.java" "$ROOT"/test/{InspectNv,TxProbe,MakeClass,UnitProbe,SetLabels}.java "$STAGE/testscripts/"
run_ghidra inspect_names ok "$TARGET/proj_named" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_names.json" addr="$A_ALPHA,$A_FP"
check names_tags

# The label script keeps a non-default name unless force=1. Its dry run must predict the real run
# (here with force=1, which renames functions that already have names).
run_ghidra label_2 ok "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvLabelCommandTables.java table="$T" count=8 report="$OUT/label_kept.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 force=1 dry=1 report="$OUT/label_force_dry.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 force=1 report="$OUT/label_force.csv"
check label names

# NvImportNameMap creates plain namespaces, while Ghidra's RTTI analysis creates classes. Turn the
# fixture's namespace into a class so that the export's "class" and vtable fields can be tested.
run_ghidra make_class ok "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript MakeClass.java name=Fixture

# Bad arguments must fail with a clear message and change nothing.
printf 'address,name,source,pin,oops\n' >"$OUT/names_badheader.csv"
run_ghidra bad_args fail "$TARGET/proj_named" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvExeIdentity.java foo=1 out="$OUT/never.json" \
    -postScript NvExportProgram.java out="$OUT/never" start=4015a2 \
    -postScript NvLabelCommandTables.java table="$T" end=$(printf '0x%x' $((0x$TABLE + 41))) report="$OUT/never.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=3 end=$(printf '0x%x' $((0x$TABLE + 120))) report="$OUT/never.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_badheader.csv" report="$OUT/never.csv" \
    -postScript NvImportNameMap.java csv="$OUT/does-not-exist.csv" report="$OUT/never.csv" \
    -postScript NvFunctionCard.java addr="$(sym fx_sum)" depth=0 out="$OUT/never_cards"
for msg in "unknown argument 'foo'" "start and end must be given together" "not a positive multiple of stride 40" \
    "give exactly one of count=<n> or end=<hex>" "unknown CSV column 'oops'" "does-not-exist.csv" \
    "depth must be in 1..32"; do
    grep -q -- "$msg" "$LOGS/bad_args.log" || fail "bad-argument message missing: $msg"
done
[ "$(grep -c 'REPORT SCRIPT ERROR' "$LOGS/bad_args.log")" -eq 7 ] || fail "expected seven script errors in bad_args"
[ ! -e "$OUT/never.json" ] && [ ! -e "$OUT/never" ] && [ ! -e "$OUT/never.csv" ] && [ ! -e "$OUT/never_cards" ] ||
    fail "a failed script wrote output"
echo "  ok   seven bad invocations each fail with a clear message and write nothing"

# ---------------------------------------------------------------- C: export the labeled project
run_ghidra export_labeled ok "$TARGET/proj_named" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvExportProgram.java out="$OUT/export2" threads=4 \
    -postScript NvExportProgram.java out="$OUT/export2_t1" threads=1 \
    -postScript NvExportProgram.java out="$OUT/export_range" start="$A_ANGLE" end="$A_EXT" decompile=0 disasm=1
run_ghidra inspect ok "$TARGET/proj_named" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect.json" addr="$A_DISPATCH,$A_GAIN_VAR,$A_FP,$A_ALPHA"
check export2 determinism range inspect

# ---------------------------------------------------------------- C2: vtable label forms
# Ghidra's RTTI class recovery names the tables of a class with several bases vftable_for_<Base>,
# and PDB-style names carry a backtick. The export must find all of them, not only a label named
# vftable. On a copy of the labeled project: the label at g_dispatch is renamed to the first form,
# two more pointer tables get the others, and one table gets a second label.
cp -a "$TARGET/proj_named" "$TARGET/proj_vt"
A_VT_PDB=$(sym g_vt_pdb)
A_VT_BRACE=$(sym g_vt_brace)
printf '%s Fixture vftable_for_Base\n%s Fixture %s\n+%s Fixture vftable\n%s Fixture %s\n' \
    "$A_DISPATCH" "$A_VT_PDB" "\`vftable'" "$A_VT_PDB" "$A_VT_BRACE" "vftable{for_Base}" >"$OUT/vt_labels.txt"
run_ghidra vt_labels ok "$TARGET/proj_vt" "$PROJ_NAME" "${RW[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript SetLabels.java file="$OUT/vt_labels.txt"
run_ghidra export_vt ok "$TARGET/proj_vt" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvExportProgram.java out="$OUT/export_vt" threads=2 decompile=0
check vt

# ---------------------------------------------------------------- C3: provenance tags of the label script
# A copy of the unlabeled project in which the name import has already named the shared parse stub
# (source jip) and cmd_beta_execute (with exactly the name the label script wants). The label
# script then only recognizes those two, so it must not add src:cmdtable to them, while it does tag
# the functions it creates and renames.
cp -a "$TARGET/proj" "$TARGET/proj_tags"
A_BETA=$(sym cmd_beta_execute)
A_GAMMA=$(sym cmd_gamma_execute)
A_SHARED=$(sym cmd_shared_parse)
cat >"$OUT/names_tags.csv" <<EOF
address,name,source,pin,kind
$A_SHARED,SharedStub,jip,1111111111111111,
$A_BETA,Cmd_FixtureBeta_Execute,own,2222,
EOF
run_ghidra label_tags ok "$TARGET/proj_tags" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_tags.csv" report="$OUT/names_tags.report.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 report="$OUT/label_tags.csv"
run_ghidra inspect_tags ok "$TARGET/proj_tags" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_tags.json" addr="$A_SHARED,$A_BETA,$A_ALPHA,$A_GAMMA"
check label_tags

# ---------------------------------------------------------------- C4: pointers that are not code
# The ninth table entry (after the terminator) has an execute pointer that leads to FF FF bytes and
# a parse pointer into the middle of cmd_alpha_execute. Neither may become a function, in the dry
# run or the real run, and the two runs must agree. On a copy of the labeled project, where
# cmd_alpha_execute already is a function.
cp -a "$TARGET/proj_named" "$TARGET/proj_badcode"
A_BADCODE=$(sym cmd_badcode_execute)
A_ALPHA_MID=$(printf '%08x' $((0x$A_ALPHA + 4)))
run_ghidra badcode ok "$TARGET/proj_badcode" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvLabelCommandTables.java table="$T" count=9 dry=1 report="$OUT/label_bad_dry.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=9 report="$OUT/label_bad_real.csv"
grep -q "dry run, nothing changed. Would create 0 functions, rename 0; names kept [0-9]*, shared pointers [0-9]*, not code 2" \
    "$LOGS/badcode.log" || fail "dry run summary for the not-code pointers missing"
grep -q "functions created 0, renamed 0, names kept [0-9]*, shared pointers [0-9]*, not code 2" "$LOGS/badcode.log" ||
    fail "real run summary for the not-code pointers missing"
run_ghidra inspect_badcode ok "$TARGET/proj_badcode" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_badcode.json" addr="$A_BADCODE,$A_ALPHA_MID,$A_ALPHA"
check badcode

# Ghidra semantics the scripts rely on: aborting a nested transaction discards the whole run.
cp -a "$TARGET/proj" "$TARGET/proj_tx"
cp "$ROOT/test/TxProbe.java" "$STAGE/testscripts/"
run_ghidra tx_probe ok "$TARGET/proj_tx" "$PROJ_NAME" "${RW[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript TxProbe.java addr="$A_GAIN_FN,$A_SUM"
run_ghidra tx_inspect ok "$TARGET/proj_tx" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_tx.json" addr="$A_GAIN_FN,$A_SUM"
grep -q "TxProbe: renamed both, inner transaction aborted" "$LOGS/tx_probe.log" || fail "TxProbe did not run to its end"
check tx

# The original project must be untouched by the labeling steps.
run_ghidra export_original ok "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" "${SP[@]}" \
    -postScript NvExportProgram.java out="$OUT/export1_again" threads=2 decompile=0
python3 - "$OUT" <<'EOF' || fail "the original project changed"
import json, sys
a = json.load(open(sys.argv[1] + "/export1/manifest.json"))
b = json.load(open(sys.argv[1] + "/export1_again/manifest.json"))
assert a["function_count_api"] == b["function_count_api"], (a["function_count_api"], b["function_count_api"])
print("  ok   the original (read-only) project still has %d functions" % b["function_count_api"])
EOF

# ---------------------------------------------------------------- D: the export's safety nets must trip
# A copy of the export script with one function dropped from the walk. The
# gate compares the records written with Ghidra's own function count, so it
# has to refuse. The mutation is applied with sed and verified.
mkdir -p "$STAGE/gate_scripts"
cp "$ROOT/NvCommon.java" "$STAGE/gate_scripts/"
SKIP=$(sym fx_sum)
sed "s|functions.add(it.next());|Function dropped = it.next(); if (dropped.getEntryPoint().getOffset() != 0x$SKIP) { functions.add(dropped); }|" \
    "$ROOT/NvExportProgram.java" >"$STAGE/gate_scripts/NvExportProgram.java"
cmp -s "$ROOT/NvExportProgram.java" "$STAGE/gate_scripts/NvExportProgram.java" && fail "gate mutation did not change the script"
run_ghidra gate_mutation fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/gate_scripts" \
    -postScript NvExportProgram.java out="$OUT/export_gate" decompile=0
grep -q "GATE FAILED" "$LOGS/gate_mutation.log" || fail "gate failure text missing"
check gate

# Mutated copies that force each decompile outcome other than "ok". Every fixture function
# decompiles, so the branches are reached by replacing the conditions in a copy of the script.
# This tests what the script does with each outcome (the record, the status counts and the
# refusal to write manifest.json when nothing decompiled). It does not make Ghidra itself
# time out or fail.
#   nodecomp: the decompiler returns no result at all
#   timeout:  the result is reported as timed out
#   errmsg:   the result is neither complete nor timed out
mutate nodecomp_scripts NvExportProgram.java \
    'DecompileResults res = di.decompileFunction(f, timeoutSec, TaskMonitor.DUMMY);' 'DecompileResults res = null;'
run_ghidra decompile_failure fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/nodecomp_scripts" \
    -postScript NvExportProgram.java out="$OUT/export_nodecomp" threads=2
grep -q "no function decompiled successfully" "$LOGS/decompile_failure.log" || fail "decompiler-failure text missing"

mutate timeout_scripts NvExportProgram.java 'else if (res.decompileCompleted() && res.getDecompiledFunction() != null) {' 'else if (false) {'
python3 "$HERE/mutate.py" "$STAGE/timeout_scripts/NvExportProgram.java" "$STAGE/timeout_scripts/NvExportProgram.java" \
    'else if (res.isTimedOut()) {' 'else if (true) {' || fail "second timeout mutation did not apply"
run_ghidra decompile_timeout fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/timeout_scripts" \
    -postScript NvExportProgram.java out="$OUT/export_timeout" threads=2 timeout=7
mutate errmsg_scripts NvExportProgram.java 'else if (res.decompileCompleted() && res.getDecompiledFunction() != null) {' 'else if (false) {'
run_ghidra decompile_errmsg fail "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/errmsg_scripts" \
    -postScript NvExportProgram.java out="$OUT/export_errmsg" threads=2
check nodecomp decomp_branches

# ---------------------------------------------------------------- E: duplicate names
# On a copy of the labeled project (FpOnlyTarget is already named there). The first file has two
# rows that give one name to two addresses and two rows that give one address two names: all
# four are rejected and the fifth, valid, row is not applied. The second file asks for a name
# that the program already uses (a conflict), and has two sources that agree on one name. The
# third file repeats the first conflict with force=1.
cp -a "$TARGET/proj_named" "$TARGET/proj_dup"
A_RCP=$(sym fx_rcp)
A_RSQRT=$(sym fx_rsqrt)
A_CW=$(sym fx_set_cw)
cat >"$OUT/names_dup.csv" <<EOF
address,name,source,pin,kind
$A_RCP,Dup,own,aaaa,
$A_RSQRT,Dup,own,bbbb,
$(sym fx_double_scale),NameOne,own,cccc,
$(sym fx_double_scale),NameTwo,own,dddd,
$(sym fx_sse_scalar),Valid,own,eeee,
EOF
cat >"$OUT/names_taken.csv" <<EOF
address,name,source,pin,kind
$A_GAIN_FN,FpOnlyTarget,own,aaaa,
$A_CW,AgreedName,srca,1111,
$A_CW,AgreedName,srcb,2222,
EOF
cat >"$OUT/names_takenforce.csv" <<EOF
address,name,source,pin,kind
$A_GAIN_FN,FpOnlyTarget,own,bbbb,
EOF
run_ghidra names_dup fail "$TARGET/proj_dup" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_dup.csv" report="$OUT/names_dup.report.csv"
run_ghidra names_taken ok "$TARGET/proj_dup" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_taken.csv" report="$OUT/names_taken.report.csv" \
    -postScript NvImportNameMap.java csv="$OUT/names_takenforce.csv" force=1 report="$OUT/names_takenforce.report.csv"
run_ghidra inspect_dup ok "$TARGET/proj_dup" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_dup.json" \
    addr="$A_RCP,$A_RSQRT,$A_GAIN_FN,$A_FP,$A_CW"
check dups

# ---------------------------------------------------------------- F: tier through callees, by routine name
# A copy of the original project in which the integer-only stub fx_crt_stub is named _CIsin, as
# the FunctionID database would name a stripped CRT routine. fx_crt_wrap calls it and fx_crt_top
# calls the wrapper; the cards must see the name at both levels.
cp -a "$TARGET/proj" "$TARGET/proj_crt"
printf 'address,name,source,pin,kind\n%s,_CIsin,own,,\n' "$(sym fx_crt_stub)" >"$OUT/names_crt.csv"
run_ghidra crt_names_cards ok "$TARGET/proj_crt" "$PROJ_NAME" "${RW[@]}" "${SP[@]}" \
    -postScript NvImportNameMap.java csv="$OUT/names_crt.csv" report="$OUT/names_crt.report.csv" \
    -postScript NvFunctionCard.java addr="$(sym fx_crt_wrap),$(sym fx_crt_top)" out="$OUT/cards_crt"
check cards_crt

# ---------------------------------------------------------------- G: a failure while applying rolls everything back
# The name scripts check everything before they change anything, so a row that fails only while
# it is applied cannot be made with real input. These copies of the scripts throw at a chosen
# point of the real run (the dry runs are not affected), after earlier rows or entries have
# already been applied. The test then looks at the saved project: nothing may be left.
#   NvImportNameMap: throws after renaming the second function (fx_rcp); the first (fx_rsqrt) was
#                    renamed and tagged before it.
#   NvLabelCommandTables: throws when it reaches entry 3, after entries 0 to 2 created and named
#                    functions.
cp -a "$TARGET/proj" "$TARGET/proj_inject"
mutate inject_scripts NvImportNameMap.java \
    '        fn.getSymbol().setNameAndNamespace(shortName, ns, SourceType.USER_DEFINED);' \
    "        fn.getSymbol().setNameAndNamespace(shortName, ns, SourceType.USER_DEFINED);
        if (addr.getOffset() == 0x${A_RCP}L) { throw new IllegalStateException(\"injected failure\"); }"
mutate inject_scripts NvLabelCommandTables.java \
    '            List<String> actions = new ArrayList<>();' \
    '            List<String> actions = new ArrayList<>();
            if (!dry && e.index == 3) { throw new IllegalStateException("injected failure"); }'
cat >"$OUT/names_inject.csv" <<EOF
address,name,source,pin,kind
$A_RSQRT,InjectedFirst,own,abc,
$A_RCP,InjectedSecond,own,abc,
EOF
run_ghidra inject fail "$TARGET/proj_inject" "$PROJ_NAME" "${RW[@]}" -scriptPath "$STAGE/inject_scripts" \
    -postScript NvImportNameMap.java csv="$OUT/names_inject.csv" report="$OUT/names_inject.report.csv" \
    -postScript NvLabelCommandTables.java table="$T" count=8 report="$OUT/label_inject.csv"
[ "$(grep -c 'injected failure' "$LOGS/inject.log")" -ge 2 ] || fail "both injected failures should show in the log"
[ "$(grep -c 'REPORT SCRIPT ERROR' "$LOGS/inject.log")" -eq 2 ] || fail "expected exactly two script errors in inject"
run_ghidra inspect_inject ok "$TARGET/proj_inject" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_inject.json" \
    addr="$A_RSQRT,$A_RCP,$A_ALPHA,$(sym cmd_beta_execute)"
check inject

# If Ghidra cannot create a function although the checks passed, the label script aborts. A copy
# in which createFunction's result is dropped: the first handler is created, then the script
# throws, and the saved project must hold no new function and no tag.
cp -a "$TARGET/proj" "$TARGET/proj_nofunc"
mutate nofunc_scripts NvLabelCommandTables.java 'fn = createFunction(a, null);' 'createFunction(a, null); fn = null;'
run_ghidra nofunc fail "$TARGET/proj_nofunc" "$PROJ_NAME" "${RW[@]}" -scriptPath "$STAGE/nofunc_scripts" \
    -postScript NvLabelCommandTables.java table="$T" count=8 report="$OUT/label_nofunc.csv"
grep -q "Ghidra could not create a function at" "$LOGS/nofunc.log" || fail "the createFunction failure text is missing"
[ ! -e "$OUT/label_nofunc.csv" ] || fail "a report was written for the aborted run"
run_ghidra inspect_nofunc ok "$TARGET/proj_nofunc" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript InspectNv.java out="$OUT/inspect_nofunc.json" addr="$A_ALPHA,$A_BETA"
check nofunc

# ---------------------------------------------------------------- H: unit cases for NvCommon
# 80-bit float conversion (compared with an exact Python conversion) and the path redaction.
check unitgen
run_ghidra unit_probe ok "$TARGET/proj" "$PROJ_NAME" "${RO[@]}" -scriptPath "$STAGE/testscripts" \
    -postScript UnitProbe.java cases="$OUT/unit_cases.txt" out="$OUT/unit_out.txt"
check unit

# No modified or copied script may be left where Ghidra would compile it with the real ones.
stray=$(find "$TARGET" -name '*.java')
[ -z "$stray" ] || fail "stray .java files under research/ghidra/target: $stray"
echo "  ok   no copy of a script is left under research/ghidra"

echo
echo "timings:"
printf '  %s\n' "${TIMINGS[@]}"
echo "  total: $(($(now_ms) - SUITE_START)) ms"
echo "ALL TESTS PASSED"
