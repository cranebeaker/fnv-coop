#!/usr/bin/env python3
"""Helpers for run-wine-tests.sh: build vectors from the fixture's selftest
output, and check nv-call and nv-probe results against it.

Subcommands:
  call-vectors   selftest text + nm output  ->  vectors file
  call-check     selftest text + results    ->  compare bit for bit
  decode-crosscheck workdir exe...          ->  length decoder vs objdump
  extra-vectors  nm output                  ->  vectors for faults, imports,
                                                snapshots and register presets
  extra-check    results mode label         ->  check those (mode: nosnap,
                                                snapshot, snapshot-keep, resolve
                                                or keep)
  snapshot-files nm output + folder         ->  write snapshot region files
  deref-vector   nm out addr                ->  one vector that reads the dword at addr
  deref-check    results value label        ->  check what it read (and the header)
  thread-vectors nm ok-out fault-out        ->  vectors for the thread fixture
  thread-check   ok|fault results label [stderr] -> check the thread fixture runs
  probe-manifest variant nm exe out [k=v..] ->  write nv-probe.txt
  probe-check    variant log out base label [exe] ->  check a probe log
  nvse-check     log out mode label         ->  check the NVSE simulation
Run with no arguments for details. Standard library only.
"""
import hashlib
import json
import os
import re
import struct
import subprocess
import sys

FAILURES = []
CHECKS = 0


def check(cond, message):
    global CHECKS
    CHECKS += 1
    if not cond:
        FAILURES.append(message)
        print(f"  FAIL: {message}")


def finish(label):
    print(f"{label}: {CHECKS - len(FAILURES)}/{CHECKS} checks passed")
    return 1 if FAILURES else 0


# ---- symbols and selftest -------------------------------------------------

def parse_nm(text):
    """Map plain function/variable names to addresses."""
    syms = {}
    for line in text.splitlines():
        m = re.match(r"^([0-9a-fA-F]{8}) ([TtDdBbRr]) (\S+)$", line.strip())
        if not m:
            continue
        name = m.group(3)
        name = name.lstrip("_@").split("@")[0]
        syms[name] = int(m.group(1), 16)
    return syms


def parse_selftest(text):
    cases = []
    for line in text.splitlines():
        if not line.startswith("CASE "):
            continue
        fields = {}
        for tok in line[5:].split(" "):
            k, _, v = tok.partition("=")
            fields[k] = v
        cases.append(fields)
    return cases


def read_jsonl(path):
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


# ---- call vectors ---------------------------------------------------------

def vector_for(case, syms):
    v = {
        "id": case["id"],
        "fn": "0x%08x" % syms[case["sym"]],
        "cc": case["cc"],
        "args": [a for a in case["args"].split(",") if a],
        "ret": case["ret"],
        "fpcw": case["fpcw"],
    }
    buffers = {}
    for k, val in case.items():
        if k.startswith("in."):
            buffers[k[3:]] = {"size": len(val) // 2, "init": val}
    v["buffers"] = buffers
    return v


def cmd_call_vectors(selftest, nm, out):
    cases = parse_selftest(open(selftest).read())
    syms = parse_nm(open(nm).read())
    with open(out, "w") as f:
        for c in cases:
            f.write(json.dumps(vector_for(c, syms)) + "\n")
    print(f"wrote {len(cases)} vectors to {out}")
    return 0


def cmd_call_check(selftest, results, label):
    cases = parse_selftest(open(selftest).read())
    rows = read_jsonl(results)
    header = rows[0]
    by_id = {r["id"]: r for r in rows if r.get("type") == "result"}
    check(header.get("type") == "header", "first line is the header")
    check(bool(header.get("cpu_vendor")) and bool(header.get("cpu_brand")), "header has the CPU vendor and brand")
    check(len(header.get("image_sha256", "")) == 64, "header has the image SHA-256")
    check(header.get("fpcw") == "0x0000027F" and header.get("mxcsr") == "0x00001F80", "header has the default control words")
    check(len(cases) > 40, f"selftest produced many cases ({len(cases)})")
    for c in cases:
        cid = c["id"]
        r = by_id.get(cid)
        if r is None:
            check(False, f"{cid}: no result")
            continue
        check(r["fault"] is None, f"{cid}: no fault ({r['fault']})")
        regs = r["regs"]
        if regs is None:
            continue
        cw = int(c["fpcw"], 16)
        check(int(regs["fcw"], 16) == cw, f"{cid}: control word after the call is {regs['fcw']}")
        # The low six bits are sticky exception flags that SSE code may set.
        check(int(regs["mxcsr"], 16) & ~0x3F == 0x1F80, f"{cid}: MXCSR control bits after the call ({regs['mxcsr']})")
        if "eax" in c:
            check(regs["eax"].lower() == "0x" + c["eax"], f"{cid}: eax {regs['eax']} vs {c['eax']}")
        if "f32" in c:
            check(r["value"]["f32_bits"].lower() == "0x" + c["f32"], f"{cid}: f32 bits {r['value']['f32_bits']} vs {c['f32']}")
        if "f64" in c:
            check(r["value"]["f64_bits"].lower() == "0x" + c["f64"], f"{cid}: f64 bits {r['value']['f64_bits']} vs {c['f64']}")
        if "st0" in c:
            st0 = regs["st0"]
            check(st0 is not None and st0["raw"] == c["st0"], f"{cid}: raw ST0 {st0 and st0['raw']} vs {c['st0']}")
        if "xmm0" in c:
            check(regs["xmm0"] == c["xmm0"], f"{cid}: xmm0 {regs['xmm0']} vs {c['xmm0']}")
        for k, val in c.items():
            if k.startswith("out."):
                check(r["buffers"].get(k[4:]) == val, f"{cid}: buffer {k[4:]} {r['buffers'].get(k[4:])} vs {val}")
        x87 = c["ret"] in ("f32_x87", "f64_x87")
        check(regs["fpu_depth"] == (1 if x87 else 0), f"{cid}: fpu depth {regs['fpu_depth']}")
        stack_args = r["stack_bytes"]
        want = "none" if stack_args == 0 else ("caller" if c["cc"] == "cdecl" else "callee")
        check(regs["cleanup"] == want, f"{cid}: stack cleanup {regs['cleanup']} (cc {c['cc']}, {stack_args} bytes)")
    return finish(label)


# ---- extra vectors --------------------------------------------------------

def extra_vectors(syms):
    def v(i, sym, cc="cdecl", args=(), ret="i32", **kw):
        d = {"id": i, "fn": "0x%08x" % syms[sym], "cc": cc, "args": list(args), "ret": ret}
        d.update(kw)
        return d
    return [
        v("fault_null_read", "fx_deref", args=["ptr:null"]),
        v("fault_ud2", "fx_ud2"),
        v("fault_div0", "fx_divide", args=["i32:5", "i32:0"]),
        v("fault_write", "fx_fill", args=["ptr:0x00000010", "i32:1"], ret="void"),
        v("import_trap", "fx_calls_import", args=["u32:5"]),
        v("after_faults", "fx_add3", args=["i32:1", "i32:2", "i32:3"]),
        v("global_5", "fx_use_global", args=["i32:5"]),
        v("read_ptr", "fx_read_ptr"),
        # A write into a snapshot region is undone before the next vector.
        v("snap_write", "fx_fill", args=["ptr:0x0a000100", "i32:7"], ret="void"),
        v("read_ptr_after_write", "fx_read_ptr"),
        v("snap_same_page", "fx_deref", args=["ptr:0x0a000200"]),
        v("snap_same_block", "fx_deref", args=["ptr:0x0a003000"]),
        v("regs_ecx_5", "fx_tiny_raw", regs={"ecx": "0x5"}),
        v("regs_ecx_0", "fx_tiny_raw", regs={"ecx": "0x0"}),
        v("mix_declared_cdecl", "fx_mix", args=["i32:3", "u32:16"]),
        v("add3_declared_stdcall", "fx_add3", cc="stdcall", args=["i32:1", "i32:2", "i32:3"]),
        v("preset_conflict", "fx_this_sum", cc="thiscall", args=["ptr:0x1000", "i32:1"], regs={"ecx": "0x5"}),
        v("buffer_offset", "fx_this_sum", cc="thiscall", args=["ptr:obj+4", "i32:2"],
          buffers={"obj": {"size": 16, "init": "ffffffff" + "05000000" + "03000000" + "00000000"}}),
        # The image's writable memory is put back between vectors.
        v("bump_a", "fx_bump"),
        v("bump_b", "fx_bump"),
        # An SSE exception that the vector's MXCSR unmasks.
        v("sse_trap", "fx_quat_normalize", args=["ptr:q"], ret="void", mxcsr="0x1F00",
          buffers={"q": {"size": 16, "init": ""}}),
        v("after_sse_trap", "fx_add3", args=["i32:4", "i32:5", "i32:6"]),
        # Stack overflow twice: the guard page must be armed again.
        v("overflow_a", "fx_recurse", args=["i32:0"]),
        v("overflow_b", "fx_recurse", args=["i32:0"]),
        v("after_overflow", "fx_add3", args=["i32:7", "i32:8", "i32:9"]),
        # The caller's handler chain must be intact after an abandoned call
        # that had registered its own handler record.
        v("seh_next_a", "fx_seh_next", ret="u32"),
        v("seh_fault", "fx_seh_fault"),
        v("seh_import", "fx_seh_import"),
        v("seh_next_b", "fx_seh_next", ret="u32"),
        # Exceptions that the called code (or the system library it calls)
        # handles itself are not faults.
        v("badread_null", "fx_bad_read", args=["ptr:0x10"]),
        v("badread_ok", "fx_bad_read", args=["ptr:buf"], buffers={"buf": {"size": 16, "init": ""}}),
        v("raise_noncont", "fx_raise_noncont"),
        v("after_raise", "fx_add3", args=["i32:1", "i32:1", "i32:1"]),
        # Exceptions that nothing in the called code handles are faults, whatever
        # their code, and the batch goes on. A single step needs no import.
        v("raise_cpp", "fx_raise_cpp"),
        v("after_raise_cpp", "fx_add3", args=["i32:2", "i32:3", "i32:4"]),
        v("raise_custom", "fx_raise_custom"),
        v("raise_info", "fx_raise_info"),
        v("single_step", "fx_single_step"),
        v("after_single_step", "fx_add3", args=["i32:3", "i32:3", "i32:3"]),
    ]


def cmd_extra_vectors(nm, out):
    syms = parse_nm(open(nm).read())
    with open(out, "w") as f:
        for d in extra_vectors(syms):
            f.write(json.dumps(d) + "\n")
        # Lines that must be reported as errors without stopping the run.
        f.write("this is not json\n")
        f.write(json.dumps({"id": "bad_buffer", "fn": "0x00401000", "args": ["ptr:nope"]}) + "\n")
        f.write(json.dumps({"id": "bad_cc", "fn": "0x00401000", "cc": "pascal"}) + "\n")
        f.write(json.dumps({"id": "bad_this", "fn": "0x00401000", "cc": "thiscall", "args": ["f32:1.0"]}) + "\n")
        f.write(json.dumps({"id": "bad_mxcsr", "fn": "0x%08x" % syms["fx_add3"], "mxcsr": "0x10000"}) + "\n")
        f.write(json.dumps({"id": "last_good", "fn": "0x%08x" % syms["fx_add3"], "args": ["i32:2", "i32:2", "i32:2"]}) + "\n")
    return 0


def cmd_snapshot_files(nm, folder):
    syms = parse_nm(open(nm).read())
    import os
    os.makedirs(folder, exist_ok=True)
    # fx_global_k = 1000, and fx_ptr_global -> a heap-like region at 0x0A000000.
    with open(os.path.join(folder, "%08x.bin" % syms["fx_global_k"]), "wb") as f:
        f.write(struct.pack("<i", 1000))
    with open(os.path.join(folder, "%08x.bin" % syms["fx_ptr_global"]), "wb") as f:
        f.write(struct.pack("<I", 0x0A000100))
    with open(os.path.join(folder, "0a000100.bin"), "wb") as f:
        f.write(struct.pack("<i", 51966))
    # A second file in a page that the first one allocated, and a third one in
    # another page of the same 64 KiB block.
    with open(os.path.join(folder, "0a000200.bin"), "wb") as f:
        f.write(struct.pack("<i", 4660))
    with open(os.path.join(folder, "0a003000.bin"), "wb") as f:
        f.write(struct.pack("<i", 22136))
    return 0


def cmd_deref_vector(nm, out, addr):
    """A vector that reads the dword at `addr` (hex), to see what a snapshot
    file put there."""
    syms = parse_nm(open(nm).read())
    v = {"id": "deref", "fn": "0x%08x" % syms["fx_deref"], "args": ["ptr:0x%08x" % int(addr, 16)], "ret": "i32"}
    with open(out, "w") as f:
        f.write(json.dumps(v) + "\n")
    return 0


def cmd_deref_check(results, value, label, host_idata=None):
    """`host_idata` (hex VA of nv-call.exe's .idata): the placeholder range
    in the header must end before it."""
    rows = read_jsonl(results)
    header = rows[0]
    r = {x["id"]: x for x in rows if x.get("type") == "result"}.get("deref")
    check(header.get("type") == "header" and r is not None, "header and one result")
    if r is not None:
        check(r["fault"] is None and r["value"]["u32"] == int(value, 0), f"the snapshot region was read back: {r['fault'] or r['value']}")
    if host_idata:
        ph = header.get("placeholder")
        check(ph is not None, "the header reports the placeholder")
        if ph:
            base, size, array = int(ph["base"], 16), ph["size"], int(ph["array_start"], 16)
            check(base == 0x400000 and array > base, f"placeholder starts at the host's base, its array later: {ph}")
            check(base + size <= int(host_idata, 16), f"the reserved range ends before the host's .idata ({base + size:#x} vs {host_idata})")
            check(base + size > array, f"the range reaches past the start of the array: {ph}")
    return finish(label)


def cmd_thread_vectors(nm, ok_out, fault_out):
    syms = parse_nm(open(nm).read())

    def v(i, sym, args=()):
        return {"id": i, "fn": "0x%08x" % syms[sym], "args": list(args), "ret": "i32"}
    ok = [v("add_a", "tx_add", ["i32:1", "i32:2"]), v("spawn", "tx_spawn"),
          v("add_b", "tx_add", ["i32:3", "i32:4"]), v("spawn_again", "tx_spawn")]
    fault = [v("add_a", "tx_add", ["i32:1", "i32:2"]), v("spawn_fault", "tx_spawn_fault"),
             v("add_b", "tx_add", ["i32:3", "i32:4"])]
    for path, vectors in ((ok_out, ok), (fault_out, fault)):
        with open(path, "w") as f:
            for d in vectors:
                f.write(json.dumps(d) + "\n")
    return 0


# The code of nv-call's host program ends well below this; the thread fixture's
# image must reach past the function the host registers as the exception filter.
HOST_CODE_LIMIT = 0x100000


def cmd_thread_check(mode, results, label, stderr_path=None):
    rows = read_jsonl(results)
    header = rows[0]
    by_id = {r["id"]: r for r in rows if r.get("type") == "result"}

    def eax_of(i):
        r = by_id.get(i)
        return int(r["regs"]["eax"], 16) if r and r["regs"] else None
    check(header["size_of_image"] > HOST_CODE_LIMIT, f"the image is larger than the host's code ({header['size_of_image']:#x})")
    check(eax_of("add_a") == 3, f"the first vector ran: {eax_of('add_a')}")
    if mode == "ok":
        # The image has a TLS directory with a callback. The loader reads that
        # directory from the headers in memory on every thread start, so unless
        # the entry is cleared the callback runs inside the harness (natively
        # it runs once for the one thread the function starts).
        check(header["tls_directory_cleared"] is True, f"the header says the TLS directory was cleared: {header.get('tls_directory_cleared')}")
        check(all(r["fault"] is None for r in by_id.values()), "no vector faulted")
        check(eax_of("spawn") == 0 and eax_of("spawn_again") == 0, f"the image's TLS callback did not run: {eax_of('spawn')}, {eax_of('spawn_again')}")
        check(eax_of("add_b") == 7, "the vectors between ran")
    else:
        # A fault on a thread the called code created is outside the harness's
        # reach. The engine's own filter says so and ends the process; the
        # image's bytes at the host's old filter address are not run.
        check("spawn_fault" not in by_id and "add_b" not in by_id, f"the process ended inside the faulting vector: {sorted(by_id)}")
        text = open(stderr_path).read() if stderr_path else ""
        check("nv-call: unhandled exception 0xC0000005" in text and "outside the harness's reach" in text,
              f"the engine's own message is on stderr: {text.strip()[:200]}")
    return finish(label)


def cmd_extra_check(results, mode, label):
    """`mode` is nosnap, snapshot, snapshot-keep, resolve (imports resolved)
    or keep (--keep-state)."""
    snapshot = mode in ("snapshot", "snapshot-keep")
    resolve = mode == "resolve"
    keep = mode in ("keep", "snapshot-keep")
    rows = read_jsonl(results)
    header = rows[0]
    by_id = {r["id"]: r for r in rows if r.get("type") == "result"}
    errors = [r for r in rows if r.get("type") == "error"]

    def fault_of(i):
        return by_id[i]["fault"]

    def eax_of(i):
        r = by_id[i]
        return int(r["regs"]["eax"], 16) if r["regs"] else None

    check(header["state_restored"] is (not keep), f"header says the state is {'kept' if keep else 'restored'}: {header.get('state_restored')}")
    f = fault_of("fault_null_read")
    check(f and f["name"] == "access_violation" and f["access"] == "read" and f["address"] == "0x00000000", f"null read fault: {f}")
    check(by_id["fault_null_read"]["regs"] is None, "no registers after a fault")
    f = fault_of("fault_ud2")
    check(f and f["name"] == "illegal_instruction", f"ud2 fault: {f}")
    f = fault_of("fault_div0")
    check(f and f["name"] == "integer_divide_by_zero", f"divide fault: {f}")
    f = fault_of("fault_write")
    check(f and f["access"] == "write" and 0x10 <= int(f["address"], 16) < 0x20, f"write fault: {f}")
    if not resolve:
        f = fault_of("import_trap")
        check(f and f["kind"] == "import" and f["name"] == "KERNEL32.dll!GetCurrentProcessId", f"import trap: {f}")
        check(f and int(f["caller"], 16) != 0, "import trap reports the caller")
    r = by_id["after_faults"]
    check(r["fault"] is None and r["regs"]["eax"] == "0x00000002", "the batch survives faults")
    check(r["regs"]["fcw"] == "0x0000027F" and r["regs"]["fpu_depth"] == 0, "FPU is clean after recovery")

    r = by_id["global_5"]
    want = 1005 if snapshot else 12
    check(r["fault"] is None and r["value"]["i32"] == want, f"global value {r['value']} (snapshot={snapshot})")
    r = by_id["read_ptr"]
    want = 51966 if snapshot else -1
    check(r["fault"] is None and r["value"]["i32"] == want, f"pointer read {r['value']} (snapshot={snapshot})")

    r = by_id["read_ptr_after_write"]
    if snapshot:
        want = 7 if keep else 51966
        check(r["fault"] is None and r["value"]["i32"] == want, f"snapshot region after a write into it: {r['value']}, wanted {want} (keep-state={keep})")
    else:
        check(by_id["snap_write"]["fault"] is not None and r["value"]["i32"] == -1, "no snapshot: the write faults, the pointer stays null")
    for name, want, what in (("snap_same_page", 4660, "in a page the first file allocated"),
                             ("snap_same_block", 22136, "in another page of the same 64 KiB block")):
        r = by_id[name]
        if snapshot:
            check(r["fault"] is None and r["value"]["i32"] == want, f"a later snapshot file {what}: {r['value']}")
        else:
            check(r["fault"] is not None and r["fault"]["name"] == "access_violation", f"nothing is mapped there without the snapshot: {r['fault']}")

    check(by_id["regs_ecx_5"]["regs"]["eax"] == "0x00000001", "preset ECX nonzero")
    check(by_id["regs_ecx_0"]["regs"]["eax"] == "0x00000000", "preset ECX zero")
    check(by_id["mix_declared_cdecl"]["regs"]["cleanup"] == "callee", "a wrongly declared cdecl is detected as callee-cleaned")
    check(by_id["add3_declared_stdcall"]["regs"]["cleanup"] == "caller", "a wrongly declared stdcall is detected as caller-cleaned")
    check(by_id["buffer_offset"]["regs"]["eax"] == "0x0000000B" or by_id["buffer_offset"]["regs"]["eax"] == "0x0000000b",
          f"buffer offset pointer: {by_id['buffer_offset']['regs']['eax']}")

    # A result must not depend on the vectors that ran before it.
    check(eax_of("bump_a") == 1, f"first bump returns {eax_of('bump_a')}")
    want = 2 if keep else 1
    check(eax_of("bump_b") == want, f"second identical bump returns {eax_of('bump_b')}, wanted {want} (keep-state={keep})")

    # An SSE exception raised because the vector unmasked it is a fault, and the
    # batch goes on.
    f = fault_of("sse_trap")
    check(f and f["kind"] == "exception" and f["name"].startswith("float_"), f"unmasked SSE exception is a fault: {f}")
    check(eax_of("after_sse_trap") == 4 + 10 - 6, "the vector after an SSE trap runs")

    # Two stack overflows in a row, then a normal vector.
    for i in ("overflow_a", "overflow_b"):
        f = fault_of(i)
        check(f and f["name"] == "stack_overflow", f"{i}: {f}")
    check(eax_of("after_overflow") == 7 + 16 - 9, "the vector after two stack overflows runs")

    # The thread's handler chain is intact after abandoned calls.
    a, b = eax_of("seh_next_a"), eax_of("seh_next_b")
    check(a is not None and a != 0 and a == b, f"handler chain after abandoned calls: {a} then {b}")
    f = fault_of("seh_fault")
    check(f and f["name"] == "access_violation" and f["address"] == "0x00000000", f"fault under a declining handler: {f}")
    if resolve:
        check(fault_of("seh_import") is None, "import under a declining handler runs when resolved")
    else:
        f = fault_of("seh_import")
        check(f and f["kind"] == "import", f"import under a declining handler is trapped: {f}")

    if resolve:
        # The system library handles the access violation itself, so the answer
        # is the same as without the harness.
        check(fault_of("import_trap") is None, "resolved import ran")
        check(fault_of("badread_null") is None and eax_of("badread_null") == 1, f"IsBadReadPtr(null page) handles its own fault: {by_id['badread_null']['fault']} eax={eax_of('badread_null')}")
        check(fault_of("badread_ok") is None and eax_of("badread_ok") == 0, "IsBadReadPtr(valid) returns 0")
        f = fault_of("raise_noncont")
        check(f and f["kind"] == "exception" and f["name"] == "integer_divide_by_zero",
              f"a non-continuable software exception is recovered, not escalated: {f}")
        check(eax_of("after_raise") == 2, "the vector after it runs")
        # No handler of the called code, so the harness records each one. Without
        # this the system's unhandled-exception path decides, and a vector that
        # never returned normally could come back with fault null and the value
        # of the code after the raise.
        f = fault_of("raise_cpp")
        check(f and f["kind"] == "exception" and f["name"] == "cpp_exception" and f["code"] == "0xE06D7363",
              f"an unhandled C++-style exception is a fault: {f}")
        check(by_id["raise_cpp"]["regs"] is None, "no registers after the C++-style exception")
        check(eax_of("after_raise_cpp") == 2 + 6 - 4, "the vector after it runs")
        f = fault_of("raise_custom")
        check(f and f["kind"] == "exception" and f["name"] == "exception" and f["code"] == "0xE0000001",
              f"an unhandled exception with a code of its own is a fault: {f}")
        check(fault_of("raise_info") is None and eax_of("raise_info") == 8,
              f"a thread-name exception is continued, not a fault: {by_id['raise_info']['fault']} eax={eax_of('raise_info')}")
    else:
        for name in ("raise_cpp", "raise_custom", "raise_info"):
            f = fault_of(name)
            check(f and f["kind"] == "import" and f["name"] == "KERNEL32.dll!RaiseException", f"{name}: unresolved RaiseException traps: {f}")
        check(eax_of("after_raise_cpp") == 2 + 6 - 4, "the vector after the trapped raise runs")
        f = fault_of("badread_null")
        check(f and f["kind"] == "import" and f["name"] == "KERNEL32.dll!IsBadReadPtr", f"unresolved IsBadReadPtr traps: {f}")

    # A single step (the code set the trap flag itself) is a fault in every mode,
    # and the trap flag must not follow the harness into its recovery code.
    f = fault_of("single_step")
    check(f and f["kind"] == "exception" and f["name"] == "single_step" and f["code"] == "0x80000004", f"a single step is a fault: {f}")
    check(eax_of("after_single_step") == 3 + 6 - 3, "the vector after a single step runs")

    msgs = " | ".join(e["error"] for e in errors)
    check(len(errors) == 6, f"six bad lines reported as errors ({len(errors)}): {msgs}")
    check("bad_buffer" in {e.get("id") for e in errors} or "unknown buffer" in msgs, "unknown buffer reported")
    check("preset_conflict" in {e.get("id") for e in errors}, "register preset conflict reported")
    check(by_id["last_good"]["regs"]["eax"] == "0x00000004" and by_id["last_good"]["fault"] is None, "the run continues after bad lines")
    return finish(label)


# ---- probe ----------------------------------------------------------------

OBJDUMP = os.environ.get("OBJDUMP", "i686-w64-mingw32-objdump")


def function_bytes(exe, addr, count=12):
    """The first `count` bytes of the function at `addr`, read with objdump."""
    out = subprocess.run(
        [OBJDUMP, "-d", "-M", "intel", "--start-address=0x%x" % addr,
         "--stop-address=0x%x" % (addr + count + 8), "--insn-width=16", exe],
        check=True, capture_output=True, text=True).stdout
    data = bytearray()
    for line in out.splitlines():
        m = re.match(r"^\s*([0-9a-f]+):\t((?:[0-9a-f]{2} )+)", line)
        if m and int(m.group(1), 16) >= addr and len(data) < count:
            data += bytes.fromhex(m.group(2).replace(" ", ""))
    return bytes(data[:count])


def hex_bytes(b):
    return " ".join("%02X" % x for x in b)


def hook_line(syms, exe, name, sym, cc="cdecl", args="", ret=None, dump=None, max_calls=None, tamper=False, regs=False):
    addr = syms[sym]
    b = bytearray(function_bytes(exe, addr))
    if tamper:
        b[4] ^= 0xFF
    parts = [f"hook {name} addr=0x{addr:08x} cc={cc} bytes=\"{hex_bytes(b)}\""]
    if args:
        parts.append(f"args={args}")
    if ret:
        parts.append(f"ret={ret}")
    if dump:
        parts.append(f"dump={dump}")
    if max_calls is not None:
        parts.append(f"max={max_calls}")
    if regs:
        parts.append("regs=1")
    return " ".join(parts)


def file_sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        h.update(f.read())
    return h.hexdigest()


def cmd_probe_manifest(variant, nm, exe, out, *opts):
    syms = parse_nm(open(nm).read())
    o = dict(x.split("=", 1) for x in opts)
    lines = ["# generated by oracle_tests.py"]
    mode = o.get("mode", "inject")
    lines.append(f"output = {o.get('output', 'probe.jsonl')}")
    lines.append(f"mode = {mode}")
    lines.append(f"flush_ms = {o.get('flush_ms', '250')}")
    if "suspend" in o:
        lines.append(f"suspend_threads = {o['suspend']}")
    if "grace" in o:
        lines.append(f"nvse_grace_ms = {o['grace']}")
    sha = file_sha256(exe)
    if variant == "badsha":
        sha = "0" * 63 + "1"
    if variant != "nosha":
        lines.append(f"host_sha256 = {sha}")
    H = lambda *a, **k: hook_line(syms, exe, *a, **k)
    if variant in ("main", "badsha", "nosha"):
        lines += [
            H("add3", "fx_add3", args="i32,i32,i32", ret="i32", regs=True),
            H("mix", "fx_mix", cc="stdcall", args="i32,u32", ret="i32", max_calls=2),
            H("this_sum", "fx_this_sum", cc="thiscall", args="ptr,i32", ret="i32", dump="ecx+0:16,arg0+4:8", regs=True),
            H("obj_set", "fx_obj_set", cc="thiscall", args="ptr,i32,i32", ret="i32", dump="ecx+0:16,esp+4:8,[esp+4]:4"),
            H("lerp", "fx_lerp", args="f32,f32,f32", ret="f32_x87"),
            H("fact", "fx_fact", args="i32", ret="i32"),
            H("reloc", "fx_reloc", args="i32", ret="i32"),
            H("thunk", "fx_thunk_sub3", args="i32,i32,i32", ret="i32"),
            H("fill_wrong_bytes", "fx_fill", args="ptr,i32", tamper=True),
        ]
    elif variant == "simple":
        lines += [H("add3", "fx_add3", args="i32,i32,i32", ret="i32")]
    elif variant == "noret":
        lines += [H("add3", "fx_add3", args="i32,i32,i32")]
    elif variant == "tail":
        # A jump thunk and the function it jumps to, both with return capture:
        # the second entry arrives at the same stack position as the first.
        lines += [
            H("thunk", "fx_thunk_sub3", args="i32,i32,i32", ret="i32"),
            H("sub3", "fx_sub3", args="i32,i32,i32", ret="i32"),
        ]
    elif variant == "sleepy":
        lines += [H("sleepy", "fx_sleepy", args="i32", ret="i32")]
    elif variant == "threads":
        lines += [H("fact", "fx_fact", args="i32", ret="i32")]
    elif variant == "rel8":
        lines += [
            H("add3", "fx_add3", args="i32,i32,i32", ret="i32"),
            "hook tiny addr=0x%08x bytes=\"85 C9 74 06 B8 01 00 00 00\"" % syms["fx_tiny_raw"],
        ]
    else:
        print(f"unknown variant {variant}")
        return 2
    with open(out, "w") as f:
        f.write("\n".join(lines) + "\n")
    return 0


def load_log(path):
    rows = read_jsonl(path)
    header = rows[0]
    body = rows[1:]
    return header, [r for r in body if r["type"] == "call"], [r for r in body if r["type"] == "ret"], body


def argvals(call):
    return [a.get("v") for a in call["args"]]


def f32_bits(x):
    return "0x%08X" % struct.unpack("<I", struct.pack("<f", x))[0]


def obj_state(a, b):
    return struct.pack("<iifI", a, b, (a + b) * 0.5, 0xC0DE0000 | (a & 0xFFFF)).hex()


def check_common(header, body, n_expected_tid=1):
    check(header["type"] == "header" and header["tool"] == "nv-probe", "header line first")
    seqs = [r["seq"] for r in body]
    check(seqs == sorted(seqs) and len(set(seqs)) == len(seqs), "sequence numbers strictly increase")
    check(len({r["tid"] for r in body}) == n_expected_tid, f"{n_expected_tid} thread id(s) seen ({len({r['tid'] for r in body})})")
    check(all(isinstance(r["tick"], int) for r in body), "every record has a tick count")


def cmd_probe_check(variant, log, program_out, baseline_out, label, exe=None):
    n = 5
    # Lines about waiting for an injector are not program results.
    prog = [l for l in open(program_out).read().strip().splitlines() if not l.startswith("waiting pid=")]
    base = open(baseline_out).read().strip().splitlines()
    if variant in ("main", "simple", "auto", "inject-pid", "nosuspend", "tail", "nosha"):
        check(prog == base or variant == "inject-pid", f"program output unchanged by the probe: {prog} vs {base}")
    header, calls, rets, body = load_log(log)
    if exe is not None:
        # The log always says which binary it measured, asked for or not.
        check(header.get("host_sha256", "").lower() == file_sha256(exe), f"header carries the host exe hash: {header.get('host_sha256')}")
    if variant == "main":
        check_common(header, body)
        check(header["installed"] is True and header["host_sha256_check"] == "match", "installed, host hash matched")
        hooks = {h["name"]: h for h in header["hooks"]}
        for name in ["add3", "mix", "this_sum", "obj_set", "lerp", "fact", "reloc", "thunk"]:
            check(hooks[name]["status"] == "installed", f"{name} installed")
        check(hooks["fill_wrong_bytes"]["status"] == "refused" and "expected bytes" in hooks["fill_wrong_bytes"]["reason"],
              f"wrong expected bytes refused: {hooks['fill_wrong_bytes']}")
        by = lambda name, kind: [r for r in (calls if kind == "call" else rets) if r["hook"] == name]
        # add3
        c, r = by("add3", "call"), by("add3", "ret")
        check(len(c) == n and len(r) == n, f"add3 called {len(c)} times, {len(r)} returns")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check(argvals(cc_) == [i, i + 1, 100] and [a["t"] for a in cc_["args"]] == ["i32"] * 3, f"add3[{i}] args {argvals(cc_)}")
            want = i + 2 * (i + 1) - 100
            check(rr["ret"] == want and rr["eax"] == "0x%08X" % (want & 0xFFFFFFFF) and rr["call"] == cc_["seq"], f"add3[{i}] ret {rr['ret']} want {want}")
            check("regs" in cc_ and "regs" in rr and "regs" not in by("mix", "call")[0], "regs only where asked")
        # mix: max=2
        c, r = by("mix", "call"), by("mix", "ret")
        check(len(c) == 2 and len(r) == 2, f"mix limited to 2 records ({len(c)}, {len(r)})")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check(argvals(cc_) == [i, 16], f"mix[{i}] args {argvals(cc_)}")
            want = ((i * 31) ^ 16)
            check(rr["ret"] == want, f"mix[{i}] ret {rr['ret']} want {want}")
        # this_sum
        c, r = by("this_sum", "call"), by("this_sum", "ret")
        check(len(c) == n and len(r) == n, f"this_sum called {len(c)} times")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check(cc_["args"][0]["t"] == "ptr" and cc_["args"][1]["v"] == 3, f"this_sum[{i}] args {cc_['args']}")
            check(cc_["dumps"][0].get("hex") == obj_state(i, 2 * i), f"this_sum[{i}] struct dump {cc_['dumps'][0]}")
            ptr = int(cc_["args"][0]["v"], 16)
            check(cc_["dumps"][0]["addr"] == "0x%08X" % ptr, "dump address is the this pointer")
            check(cc_["dumps"][1].get("hex") == struct.pack("<if", 2 * i, (3 * i) * 0.5).hex(), f"this_sum[{i}] arg0+4 dump {cc_['dumps'][1]}")
            check(rr["ret"] == i + 2 * i * 3, f"this_sum[{i}] ret {rr['ret']}")
            check(rr["dumps_after"][0]["hex"] == obj_state(i, 2 * i), "dumps_after repeats the state")
            # regs=1: ECX holds `this` at entry, and the stack pointer is word aligned.
            check(cc_["regs"]["ecx"] == cc_["args"][0]["v"], f"this_sum[{i}] ECX is the this pointer {cc_['regs']}")
            check(int(cc_["regs"]["esp"], 16) % 4 == 0 and set(cc_["regs"]) == {"eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"}, "all eight registers logged")
            check(rr["regs"]["eax"] == rr["eax"], "registers logged at return too")
        # obj_set
        c, r = by("obj_set", "call"), by("obj_set", "ret")
        check(len(c) == n and len(r) == n, f"obj_set called {len(c)} times")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check([a["t"] for a in cc_["args"]] == ["ptr", "i32", "i32"] and argvals(cc_)[1:] == [i, 2 * i], f"obj_set[{i}] args {cc_['args']}")
            before = "00" * 16 if i == 0 else obj_state(i - 1, 2 * (i - 1))
            check(cc_["dumps"][0].get("hex") == before, f"obj_set[{i}] struct before {cc_['dumps'][0]} want {before}")
            check(cc_["dumps"][1].get("hex") == struct.pack("<ii", i, 2 * i).hex(), f"obj_set[{i}] esp+4 dump {cc_['dumps'][1]}")
            check("error" in cc_["dumps"][2] and "hex" not in cc_["dumps"][2], f"obj_set[{i}] unreadable dump reported, not crashed: {cc_['dumps'][2]}")
            check(rr["dumps_after"][0].get("hex") == obj_state(i, 2 * i), f"obj_set[{i}] struct after {rr['dumps_after'][0]}")
            check("error" in rr["dumps_after"][2], f"obj_set[{i}] unreadable dump still unreadable after")
            want = 0 if i == 0 else 3 * (i - 1)
            check(rr["ret"] == want, f"obj_set[{i}] ret {rr['ret']} want {want}")
        # lerp: x87 return
        c, r = by("lerp", "call"), by("lerp", "ret")
        check(len(c) == n and len(r) == n, f"lerp called {len(c)} times")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check([a["bits"] for a in cc_["args"]] == [f32_bits(float(i)), f32_bits(10.0), f32_bits(0.5)], f"lerp[{i}] args {cc_['args']}")
            st0 = rr["st0"]
            check(st0 is not None and st0["f64"] == (i + 10) / 2 and st0["f32"] == (i + 10) / 2, f"lerp[{i}] st0 {st0}")
        # Hooks whose stolen bytes hold a relative call and a relative jump:
        # the relocated copies must still reach the right targets.
        c, r = by("reloc", "call"), by("reloc", "ret")
        check(len(c) == n and len(r) == n, f"reloc called {len(c)} times")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check(argvals(cc_) == [i] and rr["ret"] == 2 * i + 6, f"reloc[{i}] (relocated call) ret {rr['ret']}")
        check(hooks["reloc"]["steal"] == 9, f"reloc steals 9 bytes ({hooks['reloc']['steal']})")
        c, r = by("thunk", "call"), by("thunk", "ret")
        check(len(c) == n and len(r) == n, f"thunk called {len(c)} times")
        for i, (cc_, rr) in enumerate(zip(c, r)):
            check(argvals(cc_) == [i, 1, 2] and rr["ret"] == i - 3, f"thunk[{i}] (relocated jump) ret {rr['ret']}")
        check(hooks["thunk"]["steal"] == 5, f"thunk steals 5 bytes ({hooks['thunk']['steal']})")
        # fact: nested calls come back in last-in first-out order
        facts = [x for x in body if x["hook"] == "fact"]
        expect = []
        for _ in range(n):
            expect += [("call", 4), ("call", 3), ("call", 2), ("call", 1), ("ret", 1), ("ret", 2), ("ret", 6), ("ret", 24)]
        got = [("call", argvals(x)[0]) if x["type"] == "call" else ("ret", x["ret"]) for x in facts]
        check(got == expect, f"fact nesting order: {got[:10]}...")
        seq_of_call = {}
        ok = True
        stack = []
        for x in facts:
            if x["type"] == "call":
                stack.append(x["seq"])
            else:
                ok = ok and stack and stack.pop() == x["call"]
        check(ok and not stack, "every fact return refers to its own call")
    elif variant == "nosha":
        check_common(header, body)
        check(header["host_sha256_check"] == "not_requested", f"hash not requested: {header['host_sha256_check']}")
        check(header["installed"] is True, "hooks installed without a host hash in the manifest")
    elif variant == "tail":
        check_common(header, body)
        check(header["installed"] is True and all(h["status"] == "installed" for h in header["hooks"]), f"both hooks installed: {header['hooks']}")
        check(not [x for x in body if x["type"] == "fatal"], f"no fatal record: {[x for x in body if x['type'] == 'fatal']}")
        got = [(x["type"], x["hook"]) for x in body]
        want = [("call", "thunk"), ("call", "sub3"), ("ret", "sub3"), ("ret", "thunk")] * n
        check(got == want, f"calls and returns pair up through the tail jump: {got[:8]}...")
        by_seq = {x["seq"]: x for x in body}
        for x in body:
            if x["type"] == "ret":
                c = by_seq.get(x["call"])
                check(c is not None and c["type"] == "call" and c["hook"] == x["hook"], f"{x['hook']} return {x['seq']} refers to its own call")
        for i, x in enumerate([r for r in rets if r["hook"] == "thunk"]):
            check(x["ret"] == i - 3, f"thunk[{i}] ret {x['ret']}")
        for i, x in enumerate([r for r in rets if r["hook"] == "sub3"]):
            check(x["ret"] == i - 3, f"sub3[{i}] ret {x['ret']}")
    elif variant == "sleepy":
        check_common(header, body)
        check(header["installed"] is True and header.get("pinned") is True, f"installed and pinned in memory: pinned={header.get('pinned')}")
        text = " ".join(prog)
        check("unload-inside: freed" in text and "unload-inside: sleepy returned 401" in text,
              f"the thread returned through the return stub after the unload: {prog}")
        check(len(calls) == 1 and len(rets) == 1 and rets[0]["ret"] == 401 and rets[0]["call"] == calls[0]["seq"], f"one call and its return were logged: {body}")
    elif variant in ("simple", "auto", "inject-pid", "nosuspend"):
        check_common(header, body)
        check(header["installed"] is True, "hooks installed")
        if variant == "nosuspend":
            check(header["suspend_threads"] is False and header["threads_suspended"] == 0, "no thread was suspended")
        want_origin = {"simple": "dllmain", "auto": "auto-thread", "inject-pid": "dllmain", "nosuspend": "dllmain"}[variant]
        check(header["origin"] == want_origin, f"origin {header['origin']}")
        want = n if variant != "inject-pid" else 3
        check(len(calls) == want and len(rets) == want, f"add3 calls {len(calls)} returns {len(rets)}, wanted {want}")
        for i, (cc_, rr) in enumerate(zip(calls, rets)):
            check(argvals(cc_) == [i, i + 1, 100] and rr["ret"] == i + 2 * (i + 1) - 100, f"call {i}")
    elif variant == "badsha":
        check(header["host_sha256_check"] == "mismatch" and header["installed"] is False, "host hash mismatch refuses everything")
        check(all(h["status"] == "refused" and "SHA-256" in h["reason"] for h in header["hooks"]) and len(header["hooks"]) == 9, "every hook refused with the reason")
        check(not calls and not rets, "no calls were logged")
        check(prog == base, f"program output unchanged: {prog} vs {base}")
    elif variant == "rel8":
        check("manifest_error" in header and "short branch" in header["manifest_error"], f"manifest error reported: {header.get('manifest_error')}")
        check(header["installed"] is False and not calls, "nothing installed after a manifest error")
        check(prog == base, "program output unchanged")
    elif variant == "threads":
        # 4 threads x 200 iterations x fact(5) recursion depth 5
        check_common(header, body, n_expected_tid=4)
        check(len(calls) == 4 * 200 * 5 and len(rets) == 4 * 200 * 5, f"threads: {len(calls)} calls, {len(rets)} returns")
        by_tid = {}
        for x in body:
            by_tid.setdefault(x["tid"], []).append(x)
        want = [("call", 5), ("call", 4), ("call", 3), ("call", 2), ("call", 1), ("ret", 1), ("ret", 2), ("ret", 6), ("ret", 24), ("ret", 120)]
        for tid, rows in by_tid.items():
            got = [("call", argvals(x)[0]) if x["type"] == "call" else ("ret", x["ret"]) for x in rows]
            check(got == want * 200, f"thread {tid}: nesting order intact over {len(rows)} records")
        check("bad=0" in " ".join(prog), f"program saw correct results: {prog}")
    return finish(label)


def cmd_nvse_check(log, program_out, mode, label):
    prog = open(program_out).read()
    if mode == "editor":
        check("query=0" in prog and "load=0" in prog, f"editor: plugin refuses to load: {prog.strip()}")
        check(not os.path.exists(log), "editor: no log written")
        return finish(label)
    check("query=1 info_version=1 name=nv-probe version=1" in prog, f"query filled PluginInfo: {prog.strip()}")
    check("load=1" in prog, "load returned true")
    rows = read_jsonl(log)
    check(sum(1 for r in rows if r["type"] == "header") == 1, "installed once")
    header = rows[0]
    check(header["origin"] == "nvse" and header["installed"] is True, f"origin {header.get('origin')}")
    calls = [r for r in rows if r["type"] == "call"]
    # nvse-sim runs the loop 3 times, unloads the DLL (with "free") and runs it
    # twice more. Without return capture the unload restores the code, so the
    # second run is not logged. With return capture the module stays pinned in
    # memory and the hooks keep working.
    if mode == "free":
        check(header.get("pinned") is False, f"no return capture, so the module is not pinned: {header.get('pinned')}")
        check(len(calls) == 3, f"unload restored the code: three loop iterations logged ({len(calls)})")
        check("unloaded" in prog and prog.count("loop n=") == 2, f"program ran after unload: {prog.strip()}")
    elif mode == "free-pinned":
        check(header.get("pinned") is True, f"return capture pins the module: {header.get('pinned')}")
        check(len(calls) == 5, f"hooks kept working after FreeLibrary: five loop iterations logged ({len(calls)})")
        check("unloaded" in prog and prog.count("loop n=") == 2, f"program ran after unload: {prog.strip()}")
    else:
        check(len(calls) == 3, f"three loop iterations logged ({len(calls)})")
    return finish(label)


def cmd_decode_crosscheck(workdir, *exes):
    """Compare the length decoder with objdump on the .text of compiled programs."""
    os.makedirs(workdir, exist_ok=True)
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    for exe in exes:
        data = open(exe, "rb").read()
        pe = struct.unpack_from("<I", data, 0x3C)[0]
        nsec = struct.unpack_from("<H", data, pe + 6)[0]
        optsz = struct.unpack_from("<H", data, pe + 20)[0]
        base = struct.unpack_from("<I", data, pe + 24 + 28)[0]
        text = None
        for i in range(nsec):
            o = pe + 24 + optsz + i * 40
            if data[o:o + 8].rstrip(b"\0") == b".text":
                vsize, va, rsize, rptr = struct.unpack_from("<IIII", data, o + 8)
                text = (va, data[rptr:rptr + min(rsize, vsize)])
        if text is None:
            print(f"{exe}: no .text section")
            return 1
        va, raw = text
        listing = subprocess.run([OBJDUMP, "-d", "-M", "intel", "--insn-width=16", exe],
                                 check=True, capture_output=True, text=True).stdout
        starts = set()
        cut = len(raw)
        for line in listing.splitlines():
            m = re.match(r"^\s*([0-9a-f]+):\t(?:[0-9a-f]{2} )+", line)
            parts = line.split("\t")
            if m and len(parts) >= 3 and parts[2].strip():
                off = int(m.group(1), 16) - (base + va)
                if not 0 <= off < len(raw):
                    continue
                if parts[2].strip().startswith(("(bad)", ".byte")):
                    # Data at the end of .text (constructor lists); stop here.
                    cut = min(cut, off)
                    continue
                starts.add(off)
        raw = raw[:cut]
        bounds = sorted(o for o in starts if o < cut) + [cut]
        code_path = os.path.join(workdir, "code.bin")
        bounds_path = os.path.join(workdir, "boundaries.txt")
        open(code_path, "wb").write(raw)
        open(bounds_path, "w").write("\n".join(map(str, bounds)) + "\n")
        run = subprocess.run(["cargo", "run", "--quiet", "--release", "-p", "nv-oracle-core", "--example", "decode_check",
                              "--", code_path, bounds_path], cwd=root, capture_output=True, text=True)
        print(f"{os.path.basename(exe)}: {run.stdout.strip()}")
        check(run.returncode == 0, f"{exe}: decoder agrees with objdump {run.stderr.strip()[:200]}")
        check(len(bounds) > 1000, f"{exe}: enough instructions to mean something ({len(bounds)})")
    return finish("decoder cross-check")


def main(argv):
    if len(argv) < 2:
        print(__doc__)
        return 2
    cmd, args = argv[1], argv[2:]
    if cmd == "call-vectors":
        return cmd_call_vectors(*args)
    if cmd == "call-check":
        return cmd_call_check(*args)
    if cmd == "extra-vectors":
        return cmd_extra_vectors(*args)
    if cmd == "snapshot-files":
        return cmd_snapshot_files(*args)
    if cmd == "extra-check":
        return cmd_extra_check(*args)
    if cmd == "deref-vector":
        return cmd_deref_vector(*args)
    if cmd == "deref-check":
        return cmd_deref_check(*args)
    if cmd == "thread-vectors":
        return cmd_thread_vectors(*args)
    if cmd == "thread-check":
        return cmd_thread_check(*args)
    if cmd == "decode-crosscheck":
        return cmd_decode_crosscheck(*args)
    if cmd == "probe-manifest":
        return cmd_probe_manifest(*args)
    if cmd == "probe-check":
        return cmd_probe_check(*args)
    if cmd == "nvse-check":
        return cmd_nvse_check(*args)
    print(f"unknown subcommand {cmd}")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
