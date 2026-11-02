#!/usr/bin/env python3
"""Checks the outputs of the Ghidra script tests (stdlib only).

Usage: check.py <stage> [<stage> ...] --out DIR --nm FILE --exe FILE

run-tests.sh calls this after each group of Ghidra runs. DIR holds the
script outputs, FILE is `nm` output for the unstripped fixture (the only
place real symbol addresses are known) and --exe is the analyzed (stripped)
executable. Every check prints a line; any failure makes the exit status 1.
"""

import argparse
import csv
import hashlib
import json
import os
import re
import struct
import sys
import uuid

FAILURES = []


def check(cond, message):
    if cond:
        print("  ok   " + message)
    else:
        print("  FAIL " + message)
        FAILURES.append(message)
    return cond


def hx(n):
    return "%08x" % n


def load_json(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def load_lines(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def load_csv(path):
    with open(path, newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def md5_file(path):
    h = hashlib.md5()
    with open(path, "rb") as f:
        h.update(f.read())
    return h.hexdigest()


class Ctx:
    def __init__(self, out, nm, exe):
        self.out = out
        self.exe = exe
        self.syms = {}
        with open(nm) as f:
            for line in f:
                parts = line.split()
                if len(parts) == 3:
                    name = parts[2][1:] if parts[2].startswith("_") else parts[2]
                    self.syms.setdefault(name, int(parts[0], 16))

    def sym(self, name):
        return self.syms[name]

    def path(self, *p):
        return os.path.join(self.out, *p)


# ---------------------------------------------------------------------
# Independent PE parser (the identity JSON is compared against this)
# ---------------------------------------------------------------------


def parse_pe(path):
    with open(path, "rb") as f:
        d = f.read()
    pe = struct.unpack_from("<I", d, 0x3C)[0]
    assert d[pe:pe + 4] == b"PE\0\0"
    machine, nsec, stamp, _symptr, _nsyms, optsz, chars = struct.unpack_from("<HHIIIHH", d, pe + 4)
    opt = pe + 24
    assert struct.unpack_from("<H", d, opt)[0] == 0x10B, "fixture must be PE32"
    info = {
        "machine": machine,
        "stamp": stamp,
        "chars": chars,
        "size_of_image": struct.unpack_from("<I", d, opt + 56)[0],
        "image_base": struct.unpack_from("<I", d, opt + 28)[0],
    }
    dbg_rva, dbg_size = struct.unpack_from("<II", d, opt + 96 + 6 * 8)
    secs = []
    info["sections"] = []
    so = opt + optsz
    for i in range(nsec):
        name, vsize, va, rawsz, rawptr = struct.unpack_from("<8sIIII", d, so + 40 * i)
        schars = struct.unpack_from("<I", d, so + 40 * i + 36)[0]
        secs.append((va, max(vsize, rawsz), rawptr))
        info["sections"].append({
            "name": name.split(b"\0")[0].decode("latin-1"),
            "virtual_size": vsize,
            "virtual_address": "0x%08x" % va,
            "size_of_raw_data": rawsz,
            "pointer_to_raw_data": "0x%08x" % rawptr,
            "characteristics": "0x%08x" % schars,
            "raw_in_file": rawptr + rawsz <= len(d),
            "raw_sha256": hashlib.sha256(d[rawptr:rawptr + rawsz]).hexdigest(),
        })
    info["sections_va"] = secs

    def rva_to_off(rva):
        for va, size, raw in secs:
            if va <= rva < va + size:
                return raw + (rva - va)
        raise ValueError("rva not in a section")

    info["codeview"] = None
    if dbg_rva and dbg_size:
        base = rva_to_off(dbg_rva)
        for i in range(dbg_size // 28):
            e = base + 28 * i
            typ = struct.unpack_from("<I", d, e + 12)[0]
            size = struct.unpack_from("<I", d, e + 16)[0]
            raw = struct.unpack_from("<I", d, e + 24)[0]
            if typ == 2 and d[raw:raw + 4] == b"RSDS":
                guid = str(uuid.UUID(bytes_le=d[raw + 4:raw + 20]))
                age = struct.unpack_from("<I", d, raw + 20)[0]
                pdb = d[raw + 24:raw + size].split(b"\0")[0].decode()
                info["codeview"] = {"guid": guid, "age": age, "pdb": pdb}
    return info


def read_va(exe, va, n):
    """Reads n bytes at a virtual address straight from the file (independent of Ghidra)."""
    pe = parse_pe(exe)
    rva = va - pe["image_base"]
    with open(exe, "rb") as f:
        d = f.read()
    for sva, size, raw in pe["sections_va"]:
        if sva <= rva < sva + size:
            off = raw + (rva - sva)
            return d[off:off + n]
    raise ValueError("address %08x is not in a section" % va)


# ---------------------------------------------------------------------
# Stages
# ---------------------------------------------------------------------


def stage_identity(c):
    print("identity")
    ident = load_json(c.path("identity.json"))
    pe = parse_pe(c.exe)
    for key in ("program_name", "executable_path", "executable_format", "sha256", "md5", "image_base",
                "ghidra_version", "function_count", "memory_blocks", "pe_header", "language_id"):
        check(key in ident and ident[key] not in (None, ""), "identity has " + key)
    check(ident["sha256"] == sha256_file(c.exe), "identity SHA-256 equals sha256 of the analyzed file")
    check(ident["md5"] == md5_file(c.exe), "identity MD5 equals md5 of the analyzed file")
    check(ident["image_base"] == hx(pe["image_base"]), "image base %s" % ident["image_base"])
    check(ident["function_count"] == ident["function_count_internal"] + ident["function_count_external"],
          "function count splits into internal + external")
    names = {b["name"]: b for b in ident["memory_blocks"]}
    text = names.get(".text")
    check(text is not None and text["flags"] == "r-x" and text["size"] > 0 and text["initialized"],
          ".text block is r-x and initialized")
    check(names.get(".data", {}).get("flags") == "rw-", ".data block is rw-")
    check(".bss" in names and names[".bss"]["initialized"] is False, ".bss block is uninitialized")
    h = ident["pe_header"]
    check(h.get("present") is True, "PE header found in program memory")
    check(h["time_date_stamp"] == pe["stamp"], "TimeDateStamp matches the file (%d)" % pe["stamp"])
    check(h["large_address_aware"] is bool(pe["chars"] & 0x20) and h["large_address_aware"] is True,
          "LARGE_ADDRESS_AWARE bit read as set, as in the file")
    check(h["characteristics"] == "0x%04x" % pe["chars"], "Characteristics matches the file")
    check(h["size_of_image"] == pe["size_of_image"], "SizeOfImage matches the file")
    check(h["machine"] == "0x%04x" % pe["machine"] and h["format"] == "PE32", "machine and PE32 format")
    cv = pe["codeview"]
    check(cv is not None, "fixture carries a CodeView record (built with ld --pdb)")
    hdr = h.get("codeview_from_headers", {})
    check(hdr.get("guid") == cv["guid"] and hdr.get("age") == cv["age"] and hdr.get("pdb_path") == cv["pdb"],
          "CodeView GUID, age and path read from the headers match the file (%s)" % cv["guid"])
    opt = ident["codeview_from_options"]
    check(opt.get("present") is True and opt.get("pdb_file") == cv["pdb"] and
          opt.get("pdb_guid") == cv["guid"] and opt.get("pdb_age_decimal") == cv["age"],
          "CodeView info from Ghidra's program options matches the file")
    check(opt.get("pdb_loaded") in (None, False), "no PDB was loaded (names must come from the exe only)")
    # The recorded path must not name the person who ran the script.
    path = ident["executable_path"]
    home = os.path.expanduser("~")
    profile_roots = ("/home/", "/Us" + "ers/", "/root")
    check(ident["executable_file_name"] == os.path.basename(c.exe) and path.endswith("/" + os.path.basename(c.exe)),
          "identity keeps the executable file name (%s)" % ident["executable_file_name"])
    check((home == "/" or home not in path) and not path.startswith(profile_roots),
          "identity executable_path has no user-profile folder (%s)" % path)
    # Section table and raw-section hashes against an independent parse of the file.
    check(h.get("section_hash_status", "").startswith("ok"), "section hashes were computed from the file")
    check(h.get("section_table") == pe["sections"] and len(pe["sections"]) >= 5,
          "section table (%d sections: names, addresses, sizes, characteristics, raw SHA-256) equals an independent parse" %
          len(pe["sections"]))
    check(h.get("rich_header") == {"present": False}, "a MinGW exe has no Rich header, and none is reported")


def load_export(c, name):
    d = c.path(name)
    man = load_json(os.path.join(d, "manifest.json"))
    recs = load_lines(os.path.join(d, "functions.jsonl"))
    return man, recs


def check_export_common(c, name, man, recs):
    check(man["exe_sha256"] == sha256_file(c.exe), name + ": manifest carries the exe SHA-256")
    check(bool(man["ghidra_version"]) and man["program_name"] == "nvfixture.exe", name + ": Ghidra version and program name")
    check(man["records_written"] == len(recs), name + ": records_written == lines in functions.jsonl (%d)" % len(recs))
    if not man["partial"]:
        check(man["records_written"] == man["function_count_api"] - man["function_count_external_excluded"],
              name + ": records_written == function_count_api (%d) - external (%d)" %
              (man["function_count_api"], man["function_count_external_excluded"]))
    check(man["gate"]["passed"] is True and man["records_written"] == man["records_expected"],
          name + ": completeness gate passed")
    check(man["functions_jsonl_sha256"] == sha256_file(c.path(name, "functions.jsonl")),
          name + ": functions_jsonl_sha256 equals the file hash")
    entries = [r["entry"] for r in recs]
    check(entries == sorted(entries) and len(set(entries)) == len(entries),
          name + ": records are sorted by entry and unique")
    required = ["entry", "name", "namespace", "name_source", "tags", "body", "size", "prototype",
                "calling_convention", "param_count", "stack_purge", "is_thunk", "thunk_target", "callers",
                "callees", "data_refs", "strings", "decompile_status", "decompile_error", "class", "vtable_slots",
                "disasm"]
    missing = [k for r in recs for k in required if k not in r]
    check(not missing, name + ": every record has all required keys")
    check(all("provenance" in d for r in recs for d in r["data_refs"]),
          name + ": every data ref has a provenance field")
    check(sum(man["status_counts"].values()) == len(recs), name + ": status counts add up to the records")
    for k in ("analysis_options", "uncovered_executable", "vtable_checks"):
        check(k in man, name + ": manifest has " + k)


def by_entry(recs):
    return {r["entry"]: r for r in recs}


def stage_export1(c):
    print("export1 (after analysis, before any labeling)")
    man, recs = load_export(c, "export1")
    check_export_common(c, "export1", man, recs)
    sc = man["status_counts"]
    check(sc["ok"] == len(recs) and sc["error"] == 0 and sc["timeout"] == 0,
          "export1: all %d functions decompiled (ok=%d)" % (len(recs), sc["ok"]))
    check(any(r.get("c") and "{" in r["c"] for r in recs), "export1: decompiled C text present")
    e = by_entry(recs)
    angle = e.get(hx(c.sym("fx_angle_to_unit")))
    check(angle is not None and angle["name"].startswith("FUN_") and angle["name_source"] == "DEFAULT",
          "export1: analyzed copy is stripped (fixture functions have default names)")
    # The constants' addresses come from the card and are checked against the file's own bytes,
    # so no address depends on the toolchain that built the fixture.
    card = load_json(c.path("cards", hx(c.sym("fx_angle_to_unit")) + ".json"))
    consts = {k["address"]: k for k in card["float_constants"]}
    check(len(consts) == 2 and all(read_va(c.exe, int(a, 16), 4).hex() == k["bytes"] for a, k in consts.items()),
          "export1: the card's two float constants hold the bytes the file has at those addresses")
    check(angle is not None and set(consts) <= {d["address"] for d in angle["data_refs"]},
          "export1: data refs list the two float constants (%s)" % ", ".join(sorted(consts)))
    check(all(d["provenance"] == "" for r in recs for d in r["data_refs"]),
          "export1: no provenance before any name import")
    check(all(r["class"] == "" and r["vtable_slots"] == [] for r in recs),
          "export1: no class and no vtable slots before labeling")
    th = e.get(hx(c.sym("Obj_Compute")))
    check(th is not None and th["calling_convention"] == "__thiscall", "export1: thiscall function exported as __thiscall")
    main = e.get(hx(c.sym("main")))
    check(main is not None and any(s["value"].startswith("fixture ") for s in main["strings"]),
          "export1: referenced string captured")
    callee_set = set(main["callees"]) if main else set()
    check({hx(c.sym(n)) for n in ("fx_sin", "fx_rsqrt", "fx_gain", "Obj_Compute")} <= callee_set,
          "export1: main's callees include the fixture functions")
    check(main is not None and hx(c.sym("main")) in e[hx(c.sym("fx_gain"))]["callers"],
          "export1: callers and callees agree")
    # Completeness: functions reachable only through data pointers are missed by the
    # automatic analysis. The walk cannot see them, so the uncovered-bytes report must.
    fp = c.sym("fp_only_target")
    check(hx(fp) not in e, "export1: function-pointer-only function is not found by analysis (as expected)")
    ranges = man["uncovered_executable"]["ranges"]
    check(any(int(r["start"], 16) <= fp <= int(r["end"], 16) for r in ranges),
          "export1: its address is inside a reported uncovered executable range")
    for n in ("cmd_alpha_execute", "cmd_beta_execute", "cmd_gamma_execute", "cmd_alpha_eval"):
        a = c.sym(n)
        check(any(int(r["start"], 16) <= a <= int(r["end"], 16) for r in ranges),
              "export1: handler %s is inside a reported uncovered range" % n)
    u = man["uncovered_executable"]
    check(u["uncovered_bytes_total"] == u["uncovered_padding_only_bytes"] + u["uncovered_non_padding_bytes"],
          "export1: uncovered totals add up")
    check(u["uncovered_padding_only_bytes"] > 0 and u["uncovered_non_padding_runs"] > 0,
          "export1: padding runs counted separately from real gaps")
    check(man["vtable_checks"]["vtable_count"] == 0, "export1: no vftable symbols before labeling")
    return man


def stage_cards(c):
    print("cards")
    d = c.path("cards")

    def card(name):
        return load_json(os.path.join(d, hx(c.sym(name)) + ".json"))

    def f32_of(hexbytes):
        return struct.unpack("<f", bytes.fromhex(hexbytes))[0]

    a = card("fx_angle_to_unit")
    fc = {k["address"]: k for k in a["float_constants"]}
    vals = sorted(k["f32"] for k in fc.values() if "f32" in k)
    check(len(vals) == 2 and vals[1] == 3.5, "angle card: 3.5 decoded from 00006040")
    check(abs(vals[0] - 0.0174533) < 1e-9 and all(abs(f32_of(k["bytes"]) - k["f32"]) < 1e-9 for k in fc.values()),
          "angle card: 0.0174533 decoded from 39fa8e3c")
    check(all(k["access_size"] == 4 and k["read_as"] == "f32" for k in fc.values()), "angle card: both read as f32 at size 4")
    check(a["tier"] == "B" and a["stateful"] is False, "angle card: x87 arithmetic is tier B, not stateful")
    md = open(os.path.join(d, hx(c.sym("fx_angle_to_unit")) + ".md"), encoding="utf-8").read()
    check("f32 = 3.5" in md and "f32 = 0.0174533" in md and "CPU-fidelity tier: B" in md,
          "angle card: Markdown shows the decoded values and the tier")

    s = card("fx_double_scale")
    f64 = [k["f64"] for k in s["float_constants"] if "f64" in k]
    check(2.718281828459045 in f64 and 0.125 in f64, "double card: f64 constants 2.718281828459045 and 0.125 decoded")
    check(any(k["writable_block"] and k["f64"] == 0.125 for k in s["float_constants"]),
          "double card: global double flagged as living in a writable block")
    check(any(g["interpretations"].get("f64") == 0.125 for g in s["globals"]), "double card: global interpretations include f64")

    e = card("fx_extended")
    f80 = [k for k in e["float_constants"] if k["read_as"] == "f80"]
    check(len(f80) == 1 and f80[0]["access_size"] == 10 and f80[0]["f80"].startswith("0.33333333333333333"),
          "extended card: 80-bit constant decoded at access size 10")
    check(f80[0]["hex"] == "0x3ffdaaaaaaaaaaaaaaa9", "extended card: raw bytes shown as the 80-bit value")

    p = card("fx_possible_float_immediate")
    imm = p["possible_float_immediates"]
    check(len(imm) == 1 and imm[0]["hex"] == "0x40490fdb" and abs(imm[0]["f32"] - 3.14159274) < 1e-6 and
          imm[0]["flag"] == "possible float", "immediate card: 0x40490fdb flagged as possible float 3.14159")
    check(p["float_constants"] == [] and p["tier"] == "A", "immediate card: no memory constants, tier A")

    r = card("fx_rsqrt")
    check(r["tier"] == "C" and r["instruction_counts"]["sse_approximation"] == 1, "rsqrt card: tier C")
    check(card("fx_rcp")["tier"] == "C", "rcp card: tier C")
    t = card("fx_sin")
    check(t["tier"] == "D" and t["instruction_counts"]["x87_transcendental"] == 1 and "FSIN" in t["fp_mnemonics"],
          "fsin card: tier D")
    sc = card("fx_sse_scalar")
    check(sc["tier"] == "A" and sc["instruction_counts"]["sse_arithmetic"] >= 3, "sse scalar card: tier A (SSE arithmetic and sqrt)")
    check(any(k["f32"] == 0.5 for k in sc["float_constants"]), "sse scalar card: ADDSS constant 0.5 decoded")
    su = card("fx_sum")
    check(su["tier"] == "A" and su["stateful"] is False, "integer card: tier A, not stateful")
    g = card("fx_gain")
    check(any(k["f32"] == 2.5 and k["writable_block"] for k in g["float_constants"]) and
          g["stateful"] is False, "gain card: reads writable global g_gain = 2.5, not stateful")
    cw = card("fx_set_cw")
    check(cw["changes_fpu_control"] is True and cw["instruction_counts"]["x87_control"] == 1 and
          cw["instruction_counts"]["fldcw"] == 1 and cw["instruction_counts"]["ldmxcsr"] == 0,
          "set_cw card: FLDCW counted")
    k = card("fx_load_cw_const")["float_constants"]
    check(len(k) == 1 and k[0]["read_as"] == "x87_cw" and k[0]["access_size"] == 2 and k[0]["bytes"] == "7f02" and
          k[0]["precision_bits"] == 53 and k[0]["rounding"] == "nearest" and k[0]["exception_masks"] == "0x3f",
          "FLDCW from a constant: x87 control word 0x027f decoded (53-bit precision, round to nearest, all masked)")
    km = card("fx_load_mxcsr_const")
    k = km["float_constants"]
    check(len(k) == 1 and k[0]["read_as"] == "mxcsr" and k[0]["access_size"] == 4 and k[0]["bytes"] == "c09f0000" and
          k[0]["flush_to_zero"] is True and k[0]["denormals_are_zero"] is True and k[0]["rounding"] == "nearest" and
          k[0]["exception_masks"] == "0x3f" and km["instruction_counts"]["ldmxcsr"] == 1 and
          km["changes_fpu_control"] is True,
          "LDMXCSR from a constant: MXCSR 0x9fc0 decoded (flush to zero, denormals are zero, round to nearest)")
    th = card("Obj_Compute")
    check(th["calling_convention"] == "__thiscall", "thiscall card: calling convention __thiscall")
    m = card("main")
    check(m["stateful"] is True and any("calls other functions" in x for x in m["stateful_reasons"]),
          "main card: stateful because it calls functions")
    check(m["tier_with_callees"] == "D" and m["tier"] in ("A", "B"),
          "main card: tier with callees is D")
    check(len(m["callees"]) >= 10 and all("name" in x and "entry" in x for x in m["callees"]),
          "main card: callees listed with entry and name")
    check(m["tier_is_lower_bound"] is True and any("indirect call" in r for r in m["tier_lower_bound_reasons"]),
          "main card: its indirect calls make the tier a lower bound, and say so")
    check(su["tier_is_lower_bound"] is False and su["tier_callee_functions_scanned"] == 0 and
          su["tier_lower_bound_reasons"] == [], "integer card: nothing was left unscanned, so the tier is not a lower bound")

    # SSE conversions: a memory operand is read as the SOURCE type (the first part of the name).
    def only_const(name):
        k = card(name)["float_constants"]
        return (k[0] if len(k) == 1 else None), card(name)

    k, cv = only_const("fx_cvt_sd2ss")
    check(k is not None and k["read_as"] == "f64" and k["access_size"] == 8 and k.get("f64") == 1.25,
          "CVTSD2SS card: the double 1.25 is decoded as f64 (the destination is f32)")
    k, cv = only_const("fx_cvt_ss2sd")
    check(k is not None and k["read_as"] == "f32" and k["access_size"] == 4 and k.get("f32") == 0.75,
          "CVTSS2SD card: the float 0.75 is decoded as f32 (the destination is f64)")
    k, cv = only_const("fx_cvt_tss2si")
    check(k is not None and k.get("f32") == 0.75 and cv["instruction_counts"]["sse_arithmetic"] == 1,
          "CVTTSS2SI eax,[m32] card: counted as SSE and its constant 0.75 decoded")
    k, cv = only_const("fx_cvt_tsd2si")
    check(k is not None and k["read_as"] == "f64" and k.get("f64") == 2.75 and
          cv["instruction_counts"]["sse_arithmetic"] == 1,
          "CVTTSD2SI eax,[m64] card: counted as SSE and its constant 2.75 decoded")
    k, cv = only_const("fx_cvt_ps2pd")
    check(k is not None and k["read_as"] == "f32" and k["access_size"] == 8 and k.get("f32_pair") == [1.5, -2.0],
          "CVTPS2PD card: two floats {1.5, -2.0} (not one double)")
    k, cv = only_const("fx_cvt_pd2ps")
    check(k is not None and k["read_as"] == "f64" and k["access_size"] == 16 and k.get("f64x2") == [3.5, -1.25],
          "CVTPD2PS card: two doubles {3.5, -1.25}")
    k, cv = only_const("fx_cvt_dq2pd")
    check(k is not None and k["read_as"] == "int_packed" and k["access_size"] == 8 and k.get("i32x2") == [7, -9],
          "CVTDQ2PD card: two 32-bit integers {7, -9}")
    k, cv = only_const("fx_cvt_si2sd")
    check(k is not None and k["read_as"] == "int" and k.get("signed") == 11, "CVTSI2SD card: integer 11")
    md = open(os.path.join(d, hx(c.sym("fx_cvt_ps2pd")) + ".md"), encoding="utf-8").read()
    check("f32_pair = [1.5, -2.0]" in md, "CVTPS2PD card: Markdown shows the decoded pair")

    # Tier through callees: transitive, by instruction class and by name.
    st = card("fx_sin_top")
    check(st["tier"] == "A" and st["tier_with_callees"] == "D" and st["tier_callee_functions_scanned"] == 2 and
          any("depth 2" in r and "FSIN" in r for r in st["tier_with_callees_reasons"]),
          "wrapper card: integer-only caller of a wrapper around FSIN is tier D two levels down")
    ch8 = card("fx_chain8")
    check(ch8["tier"] == "A" and ch8["tier_with_callees"] == "D" and
          any("depth 3" in r and "FSIN" in r for r in ch8["tier_with_callees_reasons"]),
          "chain card: FSIN found three levels down")
    ch0 = card("fx_chain0")
    check(ch0["tier_with_callees"] == "A" and ch0["tier_is_lower_bound"] is True and
          ch0["tier_callee_functions_scanned"] == 8 and ch0["tier_callee_depth_limit"] == 8 and
          any("depth limit 8" in r for r in ch0["tier_lower_bound_reasons"]),
          "chain card: FSIN is eleven levels down, past the depth limit of 8, so the tier is a lower bound")
    deep = load_json(os.path.join(c.path("cards_deep"), hx(c.sym("fx_chain0")) + ".json"))
    check(deep["tier"] == "A" and deep["tier_with_callees"] == "D" and deep["tier_is_lower_bound"] is False and
          deep["tier_callee_depth_limit"] == 12 and deep["tier_callee_functions_scanned"] == 11,
          "chain card with depth=12: all 11 functions below are scanned and the tier is D")
    ping = card("fx_ping")
    check(ping["tier_with_callees"] == "A" and ping["tier_callee_functions_scanned"] == 1 and
          ping["tier_is_lower_bound"] is False, "mutual recursion: the cycle is walked once and ends")
    ct = card("fx_crt_top")
    check(ct["tier"] == "A" and ct["tier_with_callees"] == "A",
          "crt-stub chain without names: integer-only code is tier A all the way down")

    # Calls through the import table have a known target (an external function).
    ic = card("fx_import_cos")
    n = ic["instruction_counts"]
    check(n["calls_indirect"] == 1 and n["calls_indirect_resolved"] == 1,
          "import card: the call through the import table is a computed call that resolves to a function")
    check(len(ic["crt_math_callees_transcendental"]) == 1 and
          re.fullmatch(r"ext:[^:]+::cos", ic["crt_math_callees_transcendental"][0]) and
          any(x["entry"].startswith("ext:") and x["name"] == "cos" for x in ic["callees"]),
          "import card: cos is found by name as an external callee (%s)" % ic["crt_math_callees_transcendental"])
    check(ic["tier"] == "D" and ic["tier_with_callees"] == "D" and ic["tier_is_lower_bound"] is False and
          ic["tier_lower_bound_reasons"] == [] and not any("targets unknown" in r for r in ic["tier_reasons"]),
          "import card: CRT math called through the import table is tier D, and not a lower bound")
    top = card("fx_import_cos_top")
    check(top["tier"] in ("A", "B") and top["tier_with_callees"] == "D" and top["tier_is_lower_bound"] is False,
          "import card: the caller of that function is tier D through its callee")
    tk = card("fx_import_tick")
    check(tk["instruction_counts"]["calls_indirect_resolved"] == 1 and tk["tier"] == "A" and
          tk["tier_with_callees"] == "A" and tk["tier_is_lower_bound"] is False and
          tk["tier_lower_bound_reasons"] == [] and
          any(x["entry"].startswith("ext:") and x["name"] == "GetTickCount" for x in tk["callees"]),
          "import card: a function that only calls an imported API (GetTickCount) is tier A and not a lower bound")
    tc = card("fx_table_call")
    check(tc["instruction_counts"]["calls_indirect"] == 1 and tc["instruction_counts"]["calls_indirect_resolved"] == 0 and
          tc["tier_is_lower_bound"] is True and any("1 indirect call(s), targets unknown" in r
                                                    for r in tc["tier_lower_bound_reasons"]),
          "table-call card: a call through an indexed table is not resolved to its first entry, and stays a lower bound")

    # MOVQ and MOVD load constants into XMM registers without a float-named instruction.
    for name, mn, key, val in (("fx_movq_const", "MOVQ", "f64", 6.5), ("fx_movd_const", "MOVD", "f32", 9.25)):
        k, mv = only_const(name)
        check(k is not None and k.get(key) == val and mn in mv["fp_mnemonics"] and
              mv["instruction_counts"]["sse_move_logic"] >= 1,
              "%s card: the constant %s is decoded (as %s) and the instruction is counted as an SSE move" %
              (mn, val, key))


def action_text(row):
    return row["action"]


def as_real(text):
    """Turns a dry-run action text into the text of the real run."""
    return text.replace("would create function", "created function").replace("would rename to", "renamed to")


def stage_label_dry_and_real(c):
    print("command-table labeling")
    dry = load_csv(c.path("label_dry.csv"))
    real = load_csv(c.path("label_real.csv"))
    again = load_csv(c.path("label_again.csv"))
    check(len(real) == 8 and len(dry) == 8, "report has one row per table entry (8)")
    check(list(real[0].keys()) == ["index", "address", "opcode", "long", "short", "params", "execute",
                                   "parse", "eval", "flags", "action"], "report columns")
    check(real[0]["address"] == hx(c.sym("g_commands")) and real[1]["address"] == hx(c.sym("g_commands") + 40),
          "entry addresses advance by the 40-byte stride")
    check(real[0]["opcode"] == "0x1000" and real[1]["opcode"] == "0x1001" and real[2]["opcode"] == "0x1002",
          "opcodes read from +8")
    check(real[0]["long"] == "FixtureAlpha" and real[0]["short"] == "FxA" and real[0]["params"] == "1",
          "names and parameter count read from +0, +4, +18")
    check(real[0]["execute"] == hx(c.sym("cmd_alpha_execute")) and real[0]["eval"] == hx(c.sym("cmd_alpha_eval")),
          "execute (+24) and eval (+32) pointers read")
    check(real[1]["parse"] == hx(c.sym("cmd_shared_parse")) and real[1]["flags"] == "0x1", "parse (+28) and flags (+36) read")
    s48 = load_csv(c.path("label_stride48.csv"))
    check(len(s48) == 6 and sum(r["action"].startswith("skipped") for r in s48) >= 4 and
          not s48[0]["action"].startswith("skipped"),
          "a wrong stride (48) shows up as mostly invalid entries (%d of 6 skipped)" %
          sum(r["action"].startswith("skipped") for r in s48))
    check(all("would" in r["action"] for r in dry[:3]) and "created function" not in " ".join(r["action"] for r in dry),
          "dry run only says what it would do")
    a0 = real[0]["action"]
    check("created function" in a0 and "renamed to Cmd_FixtureAlpha_Execute" in a0 and "renamed to Cmd_FixtureAlpha_Eval" in a0,
          "real run: alpha execute and eval created and named (so the dry run changed nothing)")
    check("renamed to Cmd_FixtureBeta_Execute" in real[1]["action"] and "shared by 2 entries, not renamed" in real[1]["action"],
          "real run: beta execute named, shared parse pointer not renamed")
    check("renamed to Cmd_Fixture_Gamma_Odd_Execute" in real[2]["action"] and "shared by 2 entries" in real[2]["action"],
          "real run: odd characters in the long name are sanitized to '_'")
    check(real[3]["action"].startswith("nothing to do"), "entry without handler pointers does nothing")
    check(real[4]["action"].startswith("skipped:") and "null" in real[4]["action"], "null long-name pointer reported and skipped")
    check(real[5]["action"].startswith("skipped:") and "printable ASCII" in real[5]["action"],
          "non-printable long name reported and skipped")
    both = real[6]["action"]
    check("created function" in both and "renamed to Cmd_FixtureBoth_Execute" in both and
          "eval " + real[6]["eval"] + ": also used as execute, keeps Cmd_FixtureBoth_Execute" in both,
          "real run: a pointer that is both execute and eval is created once, named for the first role, and keeps it")
    check(real[7]["action"].startswith("skipped:"), "terminator entry skipped")
    check("created function" not in " ".join(r["action"] for r in again) and
          "already named Cmd_FixtureAlpha_Execute" in again[0]["action"] and
          "already named Cmd_FixtureBoth_Execute" in again[6]["action"] and
          "also used as execute, keeps Cmd_FixtureBoth_Execute" in again[6]["action"],
          "second run is idempotent")
    mismatch = [i for i in range(8) if as_real(dry[i]["action"]) != real[i]["action"]]
    check(not mismatch, "dry run predicts the real run exactly, row by row (mismatching rows: %s)" % mismatch)
    shared_created = [r["index"] for r in dry if "would create function, shared" in r["action"]]
    check(len(shared_created) == 1, "dry run counts the shared parse function as created once, not once per entry")


def stage_names(c):
    print("name map import")
    main = {r["row"]: r for r in load_csv(c.path("names_main.report.csv"))}
    check(list(next(iter(main.values())).keys()) == ["row", "address", "requested_name", "applied_name",
                                                    "kind", "source", "pin", "action", "detail"], "report columns")
    r1 = main["1"]
    check(r1["action"] == "applied" and r1["kind"] == "function" and r1["applied_name"] == "Fixture::AngleToUnit",
          "row 1: stripped function named, in namespace Fixture")
    r2 = main["2"]
    check(r2["action"] == "applied" and r2["kind"] == "function" and "created function" in r2["detail"],
          "row 2: function created at a function-pointer-only address, then named")
    r3 = main["3"]
    check(r3["action"] == "applied" and r3["kind"] == "label" and r3["applied_name"] == "Fixture::vftable",
          "row 3: explicit label created")
    r4 = main["4"]
    check(r4["action"] == "conflict" and r4["applied_name"] == "Cmd_FixtureAlpha_Execute",
          "row 4: existing user name is not overwritten without force")
    r5 = main["5"]
    check(r5["action"] == "applied" and r5["kind"] == "label" and r5["applied_name"] == "g_Gain",
          "row 5: data address becomes a label (kind chosen automatically)")
    r6 = main["6"]
    check(r6["action"] == "applied" and r6["applied_name"] == "Fixture::Ext_Math",
          "row 6: whitespace in the name is sanitized to '_'")
    dry = load_csv(c.path("names_dry.report.csv"))
    check(dry[0]["action"] == "would-apply", "dry run: reported as would-apply")
    bom = load_csv(c.path("names_bom.report.csv"))
    check(len(bom) == 1 and bom[0]["action"] == "would-apply" and bom[0]["applied_name"] == "BomName",
          "a CSV with a byte-order mark and CRLF line ends is read")
    again = {r["row"]: r for r in load_csv(c.path("names_again.report.csv"))}
    check(again["1"]["action"] == "unchanged" and again["3"]["action"] == "unchanged" and
          again["4"]["action"] == "conflict", "second import is idempotent and still reports the conflict")
    force = load_csv(c.path("names_force.report.csv"))
    check(force[0]["action"] == "applied" and "force" in force[0]["detail"] and force[0]["applied_name"] == "UserAlpha",
          "force=1 replaces the existing name")
    rva = load_csv(c.path("names_rva.report.csv"))
    check(rva[0]["action"] == "applied" and rva[0]["address"] == hx(c.sym("fx_sum")) and rva[0]["applied_name"] == "FxSum",
          "rva=1 turns an RVA into the virtual address")
    bad = load_csv(c.path("names_bad.report.csv"))
    check(bad[0]["action"] == "not-applied" and bad[1]["action"] == "rejected" and
          "outside [A-Za-z0-9_:~<>]" in bad[1]["detail"], "bad name rejected with the reason; the valid row is not applied")
    check(bad[2]["action"] == "rejected" and "source" in bad[2]["detail"], "bad source rejected")
    check(bad[3]["action"] == "rejected" and "RVA" in bad[3]["detail"], "unmapped address rejected, with an RVA hint")
    rb = load_csv(c.path("names_unappliable.report.csv"))
    check(rb[0]["action"] == "not-applied" and rb[1]["action"] == "rejected" and
          "cannot create a function here" in rb[1]["detail"],
          "a row that cannot be applied is found by the simulated run; the other row is not applied")
    kept = load_csv(c.path("label_kept.csv"))
    check("kept existing name UserAlpha" in kept[0]["action"], "label script keeps a non-default name without force")
    forced = load_csv(c.path("label_force.csv"))
    check("renamed to Cmd_FixtureAlpha_Execute" in forced[0]["action"], "label script renames with force=1")
    forced_dry = load_csv(c.path("label_force_dry.csv"))
    mismatch = [i for i in range(len(forced)) if as_real(forced_dry[i]["action"]) != forced[i]["action"]]
    check(len(forced) == 8 and not mismatch,
          "force=1: the dry run predicts the real run exactly (mismatching rows: %s)" % mismatch)
    check("already named Cmd_FixtureBoth_Execute" in forced[6]["action"] and
          "also used as execute, keeps Cmd_FixtureBoth_Execute" in forced[6]["action"] and
          "Eval" not in forced[6]["action"],
          "force=1 does not flip a pointer that is both execute and eval to the eval name")


def stage_names_tags(c):
    print("provenance tags after the forced name import (before the label script runs again)")
    ins = load_json(c.path("inspect_names.json"))
    syms = {s["address"]: s for s in ins["symbols"]}
    alpha = syms[hx(c.sym("cmd_alpha_execute"))]
    check(alpha["name"] == "UserAlpha" and alpha["source"] == "USER_DEFINED",
          "alpha execute is named UserAlpha by the forced import")
    check(alpha["tags"] == ["pin:111111111111", "src:jip"],
          "its tags are those of the import only: src:cmdtable was removed with the name it belonged to (%s)" %
          alpha["tags"])
    fp = syms[hx(c.sym("fp_only_target"))]
    check(fp["tags"] == ["pin:fb0f4e95ac46", "src:xnvse"], "a function named once has exactly one source and pin")


def stage_export2(c):
    print("export2 (after labeling and name import)")
    man1 = load_json(c.path("export1", "manifest.json"))
    man, recs = load_export(c, "export2")
    check_export_common(c, "export2", man, recs)
    e = by_entry(recs)
    # Handlers and the shared parse from the label script (alpha execute and eval, beta, gamma,
    # shared parse, both), plus the name-map function.
    created = 6 + 1
    check(man["function_count_api"] - man1["function_count_api"] == created,
          "export2: %d functions were created (API count %d -> %d)" %
          (created, man1["function_count_api"], man["function_count_api"]))
    for n, want in (("cmd_alpha_execute", "Cmd_FixtureAlpha_Execute"), ("cmd_alpha_eval", "Cmd_FixtureAlpha_Eval"),
                    ("cmd_beta_execute", "Cmd_FixtureBeta_Execute"),
                    ("cmd_gamma_execute", "Cmd_Fixture_Gamma_Odd_Execute"),
                    ("cmd_both", "Cmd_FixtureBoth_Execute")):
        r = e.get(hx(c.sym(n)))
        check(r is not None and r["name"] == want and "src:cmdtable" in r["tags"] and r["decompile_status"] == "ok",
              "export2: %s exported as %s with tag src:cmdtable" % (n, want))
    alpha = e[hx(c.sym("cmd_alpha_execute"))]
    check(alpha["tags"] == ["src:cmdtable"],
          "export2: the label script's forced rename replaced the import's src and pin tags with its own")
    sp = e.get(hx(c.sym("cmd_shared_parse")))
    check(sp is not None and sp["name"].startswith("FUN_") and sp["tags"] == ["src:cmdtable"],
          "export2: shared parse function created, tagged, not renamed")
    check(hx(c.sym("cmd_delta_execute")) not in e, "export2: handler of an invalid entry was not created")
    fp = e.get(hx(c.sym("fp_only_target")))
    check(fp is not None and fp["name"] == "FpOnlyTarget" and fp["name_source"] == "USER_DEFINED",
          "export2: function-pointer-only function now present and named")
    check(fp is not None and "src:xnvse" in fp["tags"] and "pin:fb0f4e95ac46" in fp["tags"],
          "export2: tags src:xnvse and pin:<first 12 chars>")
    ang = e[hx(c.sym("fx_angle_to_unit"))]
    check(ang["name"] == "AngleToUnit" and ang["namespace"] == "Fixture" and "src:own" in ang["tags"] and
          "pin:0123456789ab" in ang["tags"], "export2: namespace, source and pin tags on a named function")
    check(e[hx(c.sym("fx_extended"))]["namespace"] == "Fixture" and
          e[hx(c.sym("fx_extended"))]["name"] == "Ext_Math", "export2: sanitized name stored")
    check(ang["class"] == "Fixture" and e[hx(c.sym("fx_extended"))]["class"] == "Fixture" and
          e[hx(c.sym("fx_sum"))]["class"] == "",
          "export2: class is the parent namespace when it is a class, empty otherwise")
    slots = e[hx(c.sym("fp_only_target"))]["vtable_slots"]
    check(slots == [{"table": "Fixture::vftable", "table_address": hx(c.sym("g_dispatch")), "table_class": "Fixture",
                     "slot": 1}], "export2: the function listed in slot 1 of Fixture::vftable carries that slot (%s)" % slots)
    check(sum(len(r["vtable_slots"]) for r in recs) == 1,
          "export2: no other function is in a vtable slot (slot 0 points at code that is not a function)")
    check(e[hx(c.sym("fx_sum"))]["name"] == "FxSum", "export2: rva import applied")
    check(e[hx(c.sym("fx_gain"))]["name"].startswith("FUN_"), "export2: dry-run name was not applied")
    check(e[hx(c.sym("fx_sse_scalar"))]["name"].startswith("FUN_") and e[hx(c.sym("fx_sse_scalar"))]["tags"] == [],
          "export2: CSVs with a rejected row applied nothing")
    gain = e[hx(c.sym("fx_gain"))]
    check(any(d["address"] == hx(c.sym("g_gain")) and d["label"] == "g_Gain" for d in gain["data_refs"]),
          "export2: data ref label g_Gain from the name import")
    check(any(d["address"] == hx(c.sym("g_gain")) and d["provenance"] == "src=own pin=none" for d in gain["data_refs"]),
          "export2: that data ref carries the source and pin the import recorded")
    fp_addr = c.sym("fp_only_target")
    check(not any(int(r["start"], 16) <= fp_addr <= int(r["end"], 16) for r in man["uncovered_executable"]["ranges"]),
          "export2: function-pointer-only function no longer in the uncovered ranges")
    check(man["uncovered_executable"]["uncovered_non_padding_bytes"] <
          man1["uncovered_executable"]["uncovered_non_padding_bytes"],
          "export2: uncovered non-padding bytes went down")
    v = man["vtable_checks"]
    check(v["vtable_count"] == 1 and v["vtables"][0]["symbol"].endswith("vftable") and
          v["vtables"][0]["address"] == hx(c.sym("g_dispatch")), "export2: vtable check found Fixture::vftable")
    t = v["vtables"][0]
    check(t["slots"] >= 2, "export2: walked %d slots" % t["slots"])
    bad = {b["slot"]: b for b in t["bad_slots"]}
    check(0 in bad and bad[0]["target"] == hx(c.sym("fp_other_target")) and bad[0]["problem"] == "code_without_function",
          "export2: slot 0 (target not a function) is listed")
    check(1 not in bad, "export2: slot 1 (target is a function entry) is not listed")
    check(v["bad_slots_total"] == len(t["bad_slots"]), "export2: bad slot total")


def stage_determinism(c):
    print("determinism")
    m4 = load_json(c.path("export2", "manifest.json"))
    m1 = load_json(c.path("export2_t1", "manifest.json"))
    check(m4["options"]["threads"] == 4 and m1["options"]["threads"] == 1, "exports used 4 and 1 threads")
    check(m4["functions_jsonl_sha256"] == m1["functions_jsonl_sha256"],
          "functions.jsonl is byte-identical for 4 threads and 1 thread (%s...)" % m4["functions_jsonl_sha256"][:16])
    check(sha256_file(c.path("export2", "functions.jsonl")) == sha256_file(c.path("export2_t1", "functions.jsonl")),
          "file hashes agree")


def stage_range(c):
    print("range export")
    man, recs = load_export(c, "export_range")
    check_export_common(c, "export_range", man, recs)
    want = [hx(c.sym("fx_angle_to_unit")), hx(c.sym("fx_double_scale")), hx(c.sym("fx_extended"))]
    check(man["partial"] is True and [r["entry"] for r in recs] == want,
          "range export holds exactly the three functions with an entry in start..end")
    check(all(r["decompile_status"] == "skipped" for r in recs) and
          man["status_counts"]["skipped"] == len(recs), "decompile=0 gives status skipped")
    check(all(isinstance(r.get("disasm"), list) and len(r["disasm"]) > 0 for r in recs),
          "disasm=1 adds listing lines")
    check(any("FMUL" in line for r in recs for line in r["disasm"]), "disassembly lines are instructions")


def stage_tx(c):
    print("nested transaction abort")
    ins = load_json(c.path("inspect_tx.json"))
    syms = {s["address"]: s for s in ins["symbols"]}
    check(all(s["source"] == "DEFAULT" and s["name"].startswith("FUN_") for s in syms.values()) and len(syms) == 2,
          "after TxProbe both renames are gone: an aborted nested transaction discards the whole script run")


def stage_gate(c):
    print("gate failure (mutated copy of the export script drops one function)")
    d = c.path("export_gate")
    check(not os.path.exists(os.path.join(d, "manifest.json")), "no manifest.json is written when the gate fails")
    failed = load_json(os.path.join(d, "manifest.failed.json"))
    check(failed["gate"]["passed"] is False and failed["records_written"] == failed["records_expected"] - 1,
          "manifest.failed.json records the shortfall (written %d, expected %d)" %
          (failed["records_written"], failed["records_expected"]))


def stage_nodecomp(c):
    print("decompiler failure")
    d = c.path("export_nodecomp")
    check(not os.path.exists(os.path.join(d, "manifest.json")), "no manifest.json when nothing decompiled")
    failed = load_json(os.path.join(d, "manifest.failed.json"))
    recs = load_lines(os.path.join(d, "functions.jsonl"))
    sc = failed["status_counts"]
    check(sc["ok"] == 0 and sc["error"] == len(recs) == failed["records_written"],
          "every function still has a record, with status error (%d)" % len(recs))
    check(all(r["decompile_error"] == "decompiler returned no result" for r in recs),
          "error message recorded per function")
    check(failed["gate"]["passed"] is True, "the count gate itself passed: the failure is the decompiler gate")


def stage_decomp_branches(c):
    print("decompile outcomes other than ok (the conditions were forced in mutated copies of the script)")
    for name, status, text in (("export_timeout", "timeout", "no result within 7 s"),
                               ("export_errmsg", "error", "decompile did not complete")):
        d = c.path(name)
        check(not os.path.exists(os.path.join(d, "manifest.json")),
              name + ": no manifest.json when no function decompiled")
        failed = load_json(os.path.join(d, "manifest.failed.json"))
        recs = load_lines(os.path.join(d, "functions.jsonl"))
        sc = failed["status_counts"]
        check(len(recs) == failed["records_written"] and sc[status] == len(recs) and sc["ok"] == 0,
              name + ": every function still has a record, with status %s (%d)" % (status, len(recs)))
        check(all(r["decompile_status"] == status and r["decompile_error"] == text and r["c"] is None for r in recs),
              name + ": record has the status, the error text '%s' and no C text" % text)
        check(failed["gate"]["passed"] is True, name + ": the count gate itself passed")


def stage_inspect(c):
    print("program inspection")
    ins = load_json(c.path("inspect.json"))
    bms = {b["address"]: b for b in ins["nv_names_bookmarks"]}
    check(set(bms) == {hx(c.sym("g_dispatch")), hx(c.sym("g_gain"))}, "bookmarks exist only for the two labels")
    check(all(b["type"] == "Info" and b["category"] == "nv-names" for b in bms.values()),
          "bookmarks are type Info, category nv-names")
    check(bms[hx(c.sym("g_dispatch"))]["comment"] == "src=own pin=none", "bookmark carries source and pin")
    tags = ins["function_tag_counts"]
    check(tags.get("src:cmdtable") == 6, "6 functions carry src:cmdtable (5 named handlers and the shared parse)")
    check(tags.get("src:own") == 3 and tags.get("src:xnvse") == 1,
          "source tags counted (rejected CSVs left no tags behind)")
    check("src:jip" not in tags and "pin:111111111111" not in tags,
          "the jip import's tags are gone: its name was replaced by the label script's forced rename")
    check("pin:abc" not in tags, "no pin tag from a rejected CSV")
    syms = {s["address"]: s for s in ins["symbols"]}
    check(syms[hx(c.sym("cmd_alpha_execute"))]["tags"] == ["src:cmdtable"],
          "alpha execute ends with exactly the tag of the source that set its final name")
    g = syms[hx(c.sym("g_dispatch"))]
    check(g["qualified_name"] == "Fixture::vftable" and g["source"] == "USER_DEFINED" and not g["is_function"],
          "label Fixture::vftable at the table")


def stage_cards_crt(c):
    print("tier through callees, by routine name")
    d = c.path("cards_crt")

    def card(name):
        return load_json(os.path.join(d, hx(c.sym(name)) + ".json"))

    wrap = card("fx_crt_wrap")
    check(wrap["tier"] == "D" and any(x.endswith("_CIsin") for x in wrap["crt_math_callees_transcendental"]),
          "wrapper card: a direct call to a routine named _CIsin is tier D by name, though its code is integer-only")
    top = card("fx_crt_top")
    check(top["tier"] == "A" and top["tier_with_callees"] == "D" and top["tier_is_lower_bound"] is False and
          any("_CIsin" in r for r in top["tier_with_callees_reasons"]),
          "caller card: caller -> wrapper -> _CIsin is tier D (the name is matched at every level)")


def stage_dups(c):
    print("duplicate names")
    dup = load_csv(c.path("names_dup.report.csv"))
    check([r["action"] for r in dup] == ["rejected"] * 4 + ["not-applied"],
          "two rows giving one name to two addresses, and two names for one address: all four rejected, the valid row not applied")
    check("name 'Dup' is given to 2 different addresses in rows 1, 2" in dup[0]["detail"] and
          "rows 3, 4" in dup[2]["detail"] and "different names" in dup[2]["detail"],
          "the report says which rows clash and why")
    taken = {r["row"]: r for r in load_csv(c.path("names_taken.report.csv"))}
    check(taken["1"]["action"] == "conflict" and "name already used at %s" % hx(c.sym("fp_only_target")) in taken["1"]["detail"],
          "a name the program already uses at another address is a conflict that names the other address")
    check(taken["2"]["action"] == "applied" and taken["3"]["action"] == "unchanged",
          "two sources that agree on a name: the first applies it, the second is 'unchanged'")
    forced = load_csv(c.path("names_takenforce.report.csv"))
    check(forced[0]["action"] == "applied" and "duplicate of the name at %s" % hx(c.sym("fp_only_target")) in forced[0]["detail"],
          "force=1 allows the duplicate and says so")
    ins = load_json(c.path("inspect_dup.json"))
    syms = {s["address"]: s for s in ins["symbols"]}
    for n in ("fx_rcp", "fx_rsqrt"):
        sy = syms[hx(c.sym(n))]
        check(sy["source"] == "DEFAULT" and sy["tags"] == [], "%s still has its default name and no tags (the rejected file applied nothing)" % n)
    gain = syms[hx(c.sym("fx_gain"))]
    check(gain["name"] == "FpOnlyTarget" and gain["same_name_symbols"] == 2 and
          syms[hx(c.sym("fp_only_target"))]["same_name_symbols"] == 2,
          "only after force=1 do two functions share the name")
    agreed = syms[hx(c.sym("fx_set_cw"))]
    check(agreed["name"] == "AgreedName" and agreed["tags"] == ["pin:1111", "pin:2222", "src:srca", "src:srcb"],
          "agreeing sources add their tags up (%s)" % agreed["tags"])


def stage_inject(c):
    print("rollback when applying fails halfway (mutated copies of the scripts throw)")
    ins = load_json(c.path("inspect_inject.json"))
    ident = load_json(c.path("identity.json"))
    check(ins["function_count"] == ident["function_count"],
          "function count is unchanged after the label script failed on entry 3 (%d)" % ins["function_count"])
    check(ins["function_tag_counts"] == {} and ins["nv_names_bookmarks"] == [],
          "no tag and no bookmark survived either failed run")
    syms = {s["address"]: s for s in ins["symbols"]}
    for n in ("fx_rsqrt", "fx_rcp"):
        sy = syms[hx(c.sym(n))]
        check(sy["source"] == "DEFAULT" and sy["tags"] == [],
              "%s: the name import's earlier row was rolled back with the failing row" % n)
    check(not syms[hx(c.sym("cmd_alpha_execute"))]["is_function"] and
          not syms[hx(c.sym("cmd_beta_execute"))]["is_function"],
          "no handler function created by the label script survived")


def stage_names_inside(c):
    print("name import: an address inside a function")
    rows = load_csv(c.path("names_inside.report.csv"))
    check(len(rows) == 3, "report has one row per CSV row")
    inside, label, notcode = rows
    check(inside["action"] == "conflict" and "address is inside function" in inside["detail"] and
          "at %s, not at its entry" % hx(c.sym("fx_sum")) in inside["detail"] and "kind=label" in inside["detail"],
          "an address inside fx_sum with no kind is a conflict that names the function's entry (%s)" % inside["detail"])
    check(label["action"] == "would-apply" and label["kind"] == "label" and label["applied_name"] == "InsideLabel",
          "the same kind of address with kind=label is accepted as a label inside the function")
    check(notcode["action"] == "would-apply" and notcode["kind"] == "label",
          "bytes that do not decode as an instruction are not made a function: with no kind the row becomes a label")


def stage_names_badcode_fn(c):
    print("name import: kind=function at bytes that are no instruction")
    rows = load_csv(c.path("names_badcode_fn.report.csv"))
    check(len(rows) == 1 and rows[0]["action"] == "rejected" and
          "bytes that do not decode as an instruction" in rows[0]["detail"],
          "the row is rejected with the reason (%s)" % (rows[0]["detail"] if rows else "no row"))


def stage_vt(c):
    print("vtable label forms (the export finds vftable_for_<Base>, `vftable' and vftable{for ...})")
    man, recs = load_export(c, "export_vt")
    check_export_common(c, "export_vt", man, recs)
    v = man["vtable_checks"]
    check("vftable_for_" in v["symbol_rule"] and "vftable_meta_ptr" in v["symbol_rule"],
          "the manifest records the rule that picks the vtable labels")
    disp, pdb, brace = (hx(c.sym(n)) for n in ("g_dispatch", "g_vt_pdb", "g_vt_brace"))
    tabs = {t["address"]: t for t in v["vtables"]}
    check(v["vtable_count"] == 3 and set(tabs) == {disp, pdb, brace},
          "three vtables found, one per label form (%s)" % sorted(t["symbol"] for t in v["vtables"]))
    only, other = hx(c.sym("fp_only_target")), hx(c.sym("fp_other_target"))

    def bad_in(table, nslots):
        return {b["slot"]: b for b in table["bad_slots"] if b["slot"] < nslots}

    d = tabs[disp]
    check(d["symbol"] == "Fixture::vftable_for_Base" and d["also_named"] == [] and d["slots"] >= 2,
          "Fixture::vftable_for_Base (the name Ghidra's RTTI class recovery gives) is walked: %d slots" % d["slots"])
    bd = bad_in(d, 2)
    check(list(bd) == [0] and bd[0]["target"] == other and bd[0]["problem"] == "code_without_function",
          "its slot 0 (code, no function) is listed and slot 1 (a function) is not")
    p = tabs[pdb]
    check(p["symbol"] == "Fixture::`vftable'" and p["also_named"] == ["Fixture::vftable"] and p["slots"] >= 3,
          "the PDB form `vftable' is walked, and a second label at its address is an alias, not a second table (%s, %s)" %
          (p["symbol"], p["also_named"]))
    bp = bad_in(p, 3)
    check(list(bp) == [2] and bp[2]["target"] == other and bp[2]["problem"] == "code_without_function",
          "its slot 2 is listed")
    b = tabs[brace]
    check(b["symbol"] == "Fixture::vftable{for_Base}" and b["slots"] >= 2 and bad_in(b, 2) == {},
          "the form vftable{for_<Base>} is walked and has no bad slot in its two slots")
    check(v["bad_slots_total"] == sum(len(t["bad_slots"]) for t in v["vtables"]), "bad slot total adds up")
    fp = by_entry(recs)[only]
    got = sorted((s["table_address"], s["slot"], s["table"], s["table_class"]) for s in fp["vtable_slots"])
    want = sorted([(disp, 1, "Fixture::vftable_for_Base", "Fixture"),
                   (pdb, 0, "Fixture::`vftable'", "Fixture"), (pdb, 1, "Fixture::`vftable'", "Fixture"),
                   (brace, 0, "Fixture::vftable{for_Base}", "Fixture"),
                   (brace, 1, "Fixture::vftable{for_Base}", "Fixture")])
    check(got == want, "the function in all those slots lists each of them once, with table and class (%s)" % got)


def stage_label_tags(c):
    print("label script: tags only for what it creates or renames")
    ins = load_json(c.path("inspect_tags.json"))
    syms = {s["address"]: s for s in ins["symbols"]}
    sh = syms[hx(c.sym("cmd_shared_parse"))]
    check(sh["name"] == "SharedStub" and sh["tags"] == ["pin:111111111111", "src:jip"],
          "a shared stub named by the name import keeps only that import's tags (%s)" % sh["tags"])
    beta = syms[hx(c.sym("cmd_beta_execute"))]
    check(beta["name"] == "Cmd_FixtureBeta_Execute" and beta["tags"] == ["pin:2222", "src:own"],
          "a function that already has the wanted name keeps its own tags (%s)" % beta["tags"])
    for n, name in (("cmd_alpha_execute", "Cmd_FixtureAlpha_Execute"), ("cmd_gamma_execute", "Cmd_Fixture_Gamma_Odd_Execute")):
        sy = syms[hx(c.sym(n))]
        check(sy["name"] == name and sy["tags"] == ["src:cmdtable"],
              "%s: created and named by the label script, so it carries src:cmdtable" % n)
    rows = load_csv(c.path("label_tags.csv"))
    check("already named Cmd_FixtureBeta_Execute" in rows[1]["action"] and
          "shared by 2 entries, not renamed" in rows[1]["action"] and "created function" not in rows[1]["action"],
          "the report says which functions were only recognized")
    names = {r["row"]: r for r in load_csv(c.path("names_tags.report.csv"))}
    check(names["1"]["action"] == "applied" and "created function" in names["1"]["detail"] and
          names["2"]["action"] == "applied", "the name import applied both rows")


def stage_badcode(c):
    print("label script: handler pointers that are not code")
    dry = load_csv(c.path("label_bad_dry.csv"))
    real = load_csv(c.path("label_bad_real.csv"))
    check(len(dry) == 9 and len(real) == 9, "report has one row per table entry (9, the last one after the terminator)")
    r8 = real[8]
    bad, mid = hx(c.sym("cmd_badcode_execute")), hx(c.sym("cmd_alpha_execute") + 4)
    check(r8["long"] == "FixtureBadCode" and r8["execute"] == bad and r8["parse"] == mid,
          "entry 8 is read: execute at the FF FF bytes, parse inside cmd_alpha_execute")
    check("execute %s: not code (the bytes do not decode as an instruction): skipped" % bad in r8["action"] and
          "parse %s: not code (inside function " % mid in r8["action"] and
          "created function" not in r8["action"] and "renamed" not in r8["action"],
          "real run: both pointers are reported as not code and skipped (%s)" % r8["action"])
    mismatch = [i for i in range(9) if as_real(dry[i]["action"]) != real[i]["action"]]
    check(not mismatch, "dry run predicts the real run exactly, row by row (mismatching rows: %s)" % mismatch)
    ins = load_json(c.path("inspect_badcode.json"))
    base = load_json(c.path("inspect.json"))
    syms = {s["address"]: s for s in ins["symbols"]}
    check(not syms[bad]["is_function"] and not syms[mid]["is_function"] and syms[hx(c.sym("cmd_alpha_execute"))]["is_function"],
          "no function was created at either pointer, and cmd_alpha_execute is still one function")
    check(ins["function_count"] == base["function_count"],
          "the function count did not change (%d)" % ins["function_count"])
    check(ins["function_tag_counts"] == base["function_tag_counts"], "no tag was added")


def stage_nofunc(c):
    print("label script: a failed createFunction aborts the whole run (mutated copy)")
    ins = load_json(c.path("inspect_nofunc.json"))
    ident = load_json(c.path("identity.json"))
    check(ins["function_count"] == ident["function_count"] and ins["function_tag_counts"] == {} and
          ins["nv_names_bookmarks"] == [],
          "the project is unchanged: the function created before the failure was rolled back too (%d functions)" %
          ins["function_count"])
    syms = {s["address"]: s for s in ins["symbols"]}
    check(not syms[hx(c.sym("cmd_alpha_execute"))]["is_function"], "no handler function survived")


def rol32(x, n):
    n &= 31
    x &= 0xFFFFFFFF
    return ((x << n) | (x >> (32 - n))) & 0xFFFFFFFF if n else x


RICH_ENTRIES = [(0x00AB, 0x521E, 17), (0x00DD, 0x521E, 3), (0x0105, 0x7809, 42)]
# A made-up path of the kind a module built by the user would record.
FAKE_PDB = "/home/dave/build/nvfixture.pdb"


def stage_richgen(c):
    """Writes a copy of the fixture whose DOS stub is replaced by a Rich header."""
    print("rich header test input")
    with open(c.exe, "rb") as f:
        d = bytearray(f.read())
    lfanew = struct.unpack_from("<I", d, 0x3C)[0]
    start = 0x40
    comps = [((pid << 16) | build, count) for pid, build, count in RICH_ENTRIES]
    key = start
    for i in range(start):
        if 0x3C <= i < 0x40:
            continue
        key += rol32(d[i], i)
    for comp, count in comps:
        key += rol32(comp, count)
    key &= 0xFFFFFFFF
    words = [0x536E6144, 0, 0, 0]  # "DanS" and three zero words
    for comp, count in comps:
        words += [comp, count]
    blob = b"".join(struct.pack("<I", w ^ key) for w in words) + b"Rich" + struct.pack("<I", key)
    assert start + len(blob) <= lfanew, "fixture DOS stub is too small for the test header"
    d[start:lfanew] = blob + bytes(lfanew - start - len(blob))
    # The same copy gets a CodeView path under a profile folder, written over the name the linker
    # was given (which is long enough; the rest of the record's string is zero-filled).
    old = parse_pe(c.exe)["codeview"]["pdb"].encode()
    new = FAKE_PDB.encode()
    assert d.count(old) == 1 and len(new) <= len(old), "cannot patch the CodeView path"
    d = d.replace(old, new + bytes(len(old) - len(new)))
    with open(c.path("nvrich.exe"), "wb") as f:
        f.write(d)
    check(parse_pe(c.path("nvrich.exe"))["codeview"]["pdb"] == FAKE_PDB,
          "the copy's CodeView record now names a path under a fake profile folder")
    check(True, "wrote nvrich.exe: the fixture with a Rich header of %d entries (key 0x%08x) and that path" %
          (len(comps), key))
    return key


def stage_rich(c):
    print("rich header")
    ident = load_json(c.path("identity_rich.json"))
    h = ident["pe_header"]
    rich = h["rich_header"]
    check(rich.get("present") is True and rich.get("offset") == 0x40, "Rich header found at 0x40")
    got = [(e["product_id"], e["build"], e["count"]) for e in rich.get("entries", [])]
    check(got == RICH_ENTRIES, "entries decoded: product id, build, count (%s)" % got)
    check(rich.get("checksum_valid") is True, "the XOR key equals the checksum computed over the DOS header and entries")
    check(h["section_hash_status"].startswith("ok") and h["section_table"][0]["raw_sha256"] ==
          parse_pe(c.exe)["sections"][0]["raw_sha256"],
          "the code section's hash is that of the original fixture's (only the header and the CodeView path differ)")
    check(ident["sha256"] == sha256_file(c.path("nvrich.exe")), "identity SHA-256 equals sha256 of the patched file")
    # The copy's CodeView path is under a profile folder (see stage_richgen); the identity must not keep it.
    base = FAKE_PDB.rsplit("/", 1)[1]
    cvh = h.get("codeview_from_headers", {})
    check(cvh.get("pdb_path") == "%USERPROFILE%/build/" + base,
          "the PDB path read from the CodeView record is redacted (%s)" % cvh.get("pdb_path"))
    check(ident["codeview_from_options"].get("pdb_file") == base,
          "Ghidra's own PDB File option holds only the file name (%s)" % ident["codeview_from_options"].get("pdb_file"))
    with open(c.path("identity_rich.json"), encoding="utf-8") as f:
        raw = f.read()
    check("/home/dave" not in raw and "dave" not in raw, "no part of the fake profile folder is left in identity_rich.json")


# Label names and whether the export treats them as a vtable. The forms are the ones Ghidra's RTTI
# class recovery (vftable, vftable_for_<Base>) and PDB names (`vftable', `vftable'{for `Base'}) use.
VTABLE_NAME_CASES = {
    "vftable": True,
    "vftable_for_Base": True,
    "vftable_for_Ns_Base": True,
    "`vftable'": True,
    "`vftable'{for_`Base'}": True,
    "vftable{for_Base}": True,
    "Foo_vftable": True,
    "vftable_meta_ptr": False,
    "vftables": False,
    "vftab": False,
    "my_table": False,
    "FUN_00401000": False,
    "v": False,
}


def stage_unitgen(c):
    """Cases for UnitProbe: 80-bit floats, including exact halfway ties, and path redaction."""
    import random
    print("unit-test inputs")
    rng = random.Random(20240601)
    cases = []

    def f80(neg, exp, mant):
        return struct.pack("<QH", mant, (0x8000 if neg else 0) | exp).hex()

    for exp in (0x3FFF, 0x3FFF - 20, 0x3FFF + 20, 0x3FFF - 60, 0x4000 + 300, 0x3FFF - 300, 0x3C00):
        for low in (0x400, 0x3FF, 0x401, 0x000, 0x7FF):
            for _ in range(8):
                top = (rng.getrandbits(52) | (1 << 52))  # 53-bit significand with the top bit set
                cases.append("f80 " + f80(rng.random() < 0.3, exp, (top << 11) | low))
    cases += ["f80 " + f80(False, 0x7FFF, 0x8000000000000000), "f80 " + f80(True, 0x7FFF, 0x8000000000000000),
              "f80 " + f80(False, 0x7FFF, 0xC000000000000000), "f80 " + f80(False, 0, 0), "f80 " + f80(True, 0, 0),
              "f80 " + f80(False, 0, 1), "f80 " + f80(False, 0x7FFE, 0xFFFFFFFFFFFFFFFF),
              "f80 " + f80(False, 0x0001, 0x8000000000000000), "f80 " + f80(False, 0x3FFF, 0x8000000000000000)]
    win = "C:" + "/Us" + "ers/"
    cases += [
        "redact /" + win + "alice/nv-re/FalloutNV.exe",
        "redact " + win.replace("/", "\\") + "bob\\nv-re\\FalloutNV.exe",
        "redact /Us" + "ers/carol/nv-re/FalloutNV.exe",
        "redact /home/dave/nv-re/FalloutNV.exe",
        "redact D:/nv-re/FalloutNV.exe",
        "redact FalloutNV.exe",
        # A CodeView path of the kind a module built by the user would carry.
        "redact /home/dave/build/plugin.pdb",
    ]
    cases += ["vtname " + n for n in VTABLE_NAME_CASES]
    with open(c.path("unit_cases.txt"), "w") as f:
        f.write("\n".join(cases) + "\n")
    check(True, "wrote %d unit cases" % len(cases))


def f80_reference(hexbytes):
    """The correctly rounded double for an 80-bit value (Fraction -> float rounds once), as raw bits."""
    from fractions import Fraction
    mant, se = struct.unpack("<QH", bytes.fromhex(hexbytes))
    neg, exp = bool(se & 0x8000), se & 0x7FFF
    if exp == 0x7FFF:
        v = float("nan") if (mant << 1) & 0xFFFFFFFFFFFFFFFF else (float("-inf") if neg else float("inf"))
    elif mant == 0:
        v = -0.0 if neg else 0.0
    else:
        e2 = (exp or 1) - 16383 - 63
        fr = Fraction(mant) * (Fraction(2) ** e2)
        try:
            v = float(fr)
        except OverflowError:
            v = float("inf")
        v = -v if neg else v
    return struct.pack(">d", v).hex().lstrip("0") or "0"


def f80_old_algorithm(hexbytes):
    """What f80ToDouble did before: round to 21 digits (half up), then parse the decimal text."""
    import decimal
    mant, se = struct.unpack("<QH", bytes.fromhex(hexbytes))
    exp = se & 0x7FFF
    if exp == 0x7FFF or mant == 0:
        return None
    ctx = decimal.Context(prec=20000)
    e2 = (exp or 1) - 16383 - 63
    exact = ctx.multiply(decimal.Decimal(mant), ctx.power(decimal.Decimal(2), e2))
    short = decimal.Context(prec=21, rounding=decimal.ROUND_HALF_UP).create_decimal(exact)
    v = float(str(short))
    v = -v if se & 0x8000 else v
    return struct.pack(">d", v).hex().lstrip("0") or "0"


def stage_unit(c):
    print("NvCommon unit cases (run inside Ghidra, compared with Python)")
    with open(c.path("unit_cases.txt")) as f:
        cases = [line.rstrip("\n") for line in f if line.strip()]
    with open(c.path("unit_out.txt")) as f:
        out = [line.rstrip("\n") for line in f if line.strip()]
    check(len(out) == len(cases), "one output line per case (%d)" % len(cases))
    bad = []
    old_wrong = 0
    n80 = 0
    for case, line in zip(cases, out):
        if case.startswith("f80 "):
            n80 += 1
            hexbytes = case[4:].strip()
            parts = line.split()
            want = f80_reference(hexbytes)
            got = parts[2].lstrip("0") or "0"
            if parts[0] != "f80" or parts[1] != hexbytes or (got != want and not want.startswith("7ff8")):
                bad.append((hexbytes, got, want))
            old = f80_old_algorithm(hexbytes)
            if old is not None and old != want:
                old_wrong += 1
    check(not bad, "f80ToDouble equals the correctly rounded double for all %d 80-bit cases (first difference: %s)" %
          (n80, bad[:1]))
    check(old_wrong > 0,
          "the cases include ones the old round-to-21-digits-then-parse method gets wrong (%d of %d), so they can tell the methods apart" %
          (old_wrong, n80))
    vt = {}
    for line in out:
        if line.startswith("vtname "):
            name, _, res = line[len("vtname "):].rpartition(" => ")
            vt[name] = res == "true"
    check(vt == VTABLE_NAME_CASES,
          "isVtableName accepts vftable, vftable_for_<Base>, `vftable', `vftable'{for ...}, vftable{for... and names "
          "ending in vftable, and refuses vftable_meta_ptr and the rest (differences: %s)" %
          {k: (vt.get(k), v) for k, v in VTABLE_NAME_CASES.items() if vt.get(k) != v})
    redact = [line for line in out if line.startswith("redact ")]
    want = {
        "/" + "C:" + "/Us" + "ers/alice/nv-re/FalloutNV.exe": "%USERPROFILE%/nv-re/FalloutNV.exe",
        "C:" + "/Us" + "ers/bob/nv-re/FalloutNV.exe": "%USERPROFILE%/nv-re/FalloutNV.exe",
        "/Us" + "ers/carol/nv-re/FalloutNV.exe": "%USERPROFILE%/nv-re/FalloutNV.exe",
        "/home/dave/nv-re/FalloutNV.exe": "%USERPROFILE%/nv-re/FalloutNV.exe",
        "D:/nv-re/FalloutNV.exe": "D:/nv-re/FalloutNV.exe",
        "FalloutNV.exe": "FalloutNV.exe",
        "/home/dave/build/plugin.pdb": "%USERPROFILE%/build/plugin.pdb",
    }
    got = {}
    for line in redact:
        src, _, res = line[len("redact "):].partition(" => ")
        got[src.replace("\\", "/")] = res
    check(len(redact) == len(want), "all %d redaction cases ran" % len(want))
    for src, res in want.items():
        key = src
        check(got.get(key) == res, "redactUserPath: %s -> %s" % (src.replace("alice", "<name>").replace("bob", "<name>")
                                                                  .replace("carol", "<name>").replace("dave", "<name>"), res))


STAGES = {
    "identity": stage_identity,
    "export1": stage_export1,
    "cards": stage_cards,
    "label": stage_label_dry_and_real,
    "names": stage_names,
    "export2": stage_export2,
    "determinism": stage_determinism,
    "range": stage_range,
    "gate": stage_gate,
    "tx": stage_tx,
    "nodecomp": stage_nodecomp,
    "decomp_branches": stage_decomp_branches,
    "inspect": stage_inspect,
    "names_tags": stage_names_tags,
    "cards_crt": stage_cards_crt,
    "dups": stage_dups,
    "inject": stage_inject,
    "names_inside": stage_names_inside,
    "names_badcode_fn": stage_names_badcode_fn,
    "vt": stage_vt,
    "label_tags": stage_label_tags,
    "badcode": stage_badcode,
    "nofunc": stage_nofunc,
    "richgen": stage_richgen,
    "rich": stage_rich,
    "unitgen": stage_unitgen,
    "unit": stage_unit,
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("stages", nargs="+", choices=sorted(STAGES))
    ap.add_argument("--out", required=True)
    ap.add_argument("--nm", required=True)
    ap.add_argument("--exe", required=True)
    args = ap.parse_args()
    c = Ctx(args.out, args.nm, args.exe)
    for s in args.stages:
        try:
            STAGES[s](c)
        except Exception as e:  # a missing file or key is a failure, not a crash
            check(False, "%s: exception %s: %s" % (s, type(e).__name__, e))
    if FAILURES:
        print("\n%d check(s) failed" % len(FAILURES))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
