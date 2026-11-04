//! `(unplaced)/run_00ec36a6` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec3bda (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the two bytes of the low half (unnamed in the map; a runtime
/// library function, `cdecl`, one word; the half-word swap `00407a90`
/// calls it). The whole of `EAX` is the result: the low half is the swapped
/// half (`(value << 8) + (value >> 8)` in 16 bits), the high half is bits 8
/// to 23 of the argument, which callers ignore.
pub fn fn_00ec3bda(_e: &mut Engine, value: u32) -> u32 {
    let shifted = value << 8;
    let low = (shifted as u16).wrapping_add((value as u16) >> 8);
    (shifted & 0xffff_0000) | low as u32
}

/// `___report_gsfailure` (`00ed159c`): the stack-cookie failure handler; it
/// does not return.
const REPORT_GS_FAILURE: u32 = 0x00ed_159c;
/// The security cookie (`.data`).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;

// Translated from 00ec408c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__security_check_cookie` (CRT, `__fastcall`): returns when `cookie`
/// equals the security cookie at `011c16bc`, otherwise jumps to
/// `___report_gsfailure` (`00ed159c`), which never returns.
pub fn security_check_cookie(e: &mut Engine, cookie: u32) {
    if cookie == e.global::<u32>(SECURITY_COOKIE_011C16BC) {
        return;
    }
    e.call(REPORT_GS_FAILURE, &args![]);
}

/// The call at the end (`00ad0470`: the stream base at `this + offset` plus two stack words,
/// `RET 8`). The engine map puts it in `FaceGen/algs.obj`; its identity is unconfirmed.
const STREAM_BASE_INIT_00AD0470: u32 = 0x00ad_0470;
/// Vbtable stored at `+0` and at `+4` when the most-derived flag is set; the objects they
/// point at have RTTI names `basic_ostream` (`010a318c`) and `basic_ios` (`010a2fa4`).
const VBTABLE_010A3190: u32 = 0x010a_3190;
const VBTABLE_010A2FA4: u32 = 0x010a_2fa4;
/// The vftable stored in the virtual base subobject (RTTI class
/// `basic_ostream<char,std::char_traits<char>>`).
const VFTABLE_010A318C: u32 = 0x010a_318c;

// Translated from 00ec3a75 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3a75 - the compiler's exception frame (`__EH_prolog3` at
// `00ece255`, `__EH_epilog3` at `00ece2be`: FS:[0] chain) is not translated.
/// Unnamed in the engine map: the shape of a `std::basic_ostream` constructor with a virtual
/// base. When `most_derived` is non-zero it stores the vbtables `010a3190` at `+0` and
/// `010a2fa4` at `+4`; it then reads the virtual-base offset from `[[this] + 4]`, stores the
/// vftable `010a318c` at `this + offset` and calls `00ad0470(this + offset, first, second)`.
/// Returns `this`.
pub fn fn_00ec3a75(e: &mut Engine, this: Ptr, first: u32, second: u32, most_derived: u32) -> Ptr {
    let base = this.addr();
    if most_derived != 0 {
        e.mem.set_u32(base, VBTABLE_010A3190);
        e.mem.set_u32(base + 4, VBTABLE_010A2FA4);
    }
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.mem.set_u32(base.wrapping_add(offset), VFTABLE_010A318C);
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.call(
        STREAM_BASE_INIT_00AD0470,
        &args![base.wrapping_add(offset), first, second],
    );
    this
}

/// `operator new` (`00401000`, cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `std::_Lockit::_Lockit` (`00ec32e3`, thiscall, one word: the lock kind) and its destructor
/// (`00ec330b`, thiscall).
const LOCKIT_CTOR: u32 = 0x00ec_32e3;
const LOCKIT_DTOR: u32 = 0x00ec_330b;
/// `std::locale::_Locimp::_Locimp(bool)` (`00ec35ff`, thiscall, one word).
const LOCIMP_CTOR: u32 = 0x00ec_35ff;
/// `std::locale::_Setgloballocale` (`00ec34da`, cdecl, one word).
const SET_GLOBAL_LOCALE: u32 = 0x00ec_34da;
/// `0044a640` (thiscall, one word: a C string; Ghidra's library match is a
/// `basic_string::assign`, a name it shares with other bodies, so unconfirmed).
const STRING_ASSIGN_0044A640: u32 = 0x0044_a640;
/// `std::locale::facet::_Incref` (`00ace790`, thiscall; Ghidra library match).
const FACET_INCREF: u32 = 0x00ac_e790;
/// The global locale implementation (`.data`), the classic one, and its second copy.
const GLOBAL_LOCALE_0126FB98: u32 = 0x0126_fb98;
const CLASSIC_LOCALE_0126FB9C: u32 = 0x0126_fb9c;
const CLASSIC_LOCALE_0126FBB4: u32 = 0x0126_fbb4;
/// The C string `_Init` gives the new `_Locimp`'s name (`.rdata`).
const NAME_01063D10: u32 = 0x0106_3d10;

// Translated from 00ec36a6 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec36a6 - the compiler's exception frame (`__EH_prolog3` at 00ece255, `__EH_epilog3` at 00ece2be: FS:[0] chain) is not translated
/// `std::locale::_Init` (Ghidra library match; cdecl, no arguments): returns the global
/// `_Locimp` at `0126fb98` when there is one. Otherwise, under a `_Lockit(0)` (4 bytes on the
/// stack), checks again and, when still none, allocates 0x34 bytes, constructs a
/// `_Locimp(false)` in them (0 when the allocation failed), makes it the global locale, sets
/// its word at +0x10 to `0x3f`, assigns the name `01063d10` to the string at +0x18, stores it
/// at `0126fb9c`, `_Incref`s it and copies `0126fb9c` to `0126fbb4`. Returns the `_Locimp`.
pub fn locale_init(e: &mut Engine) -> Ptr {
    let current = e.mem.u32(GLOBAL_LOCALE_0126FB98);
    if current != 0 {
        return Ptr::new(current);
    }
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let mut locimp = e.mem.u32(GLOBAL_LOCALE_0126FB98);
        if locimp == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
            locimp = if block != 0 {
                e.call(LOCIMP_CTOR, &args![block, 0u32]).u32()
            } else {
                0
            };
            e.call(SET_GLOBAL_LOCALE, &args![locimp]);
            e.mem.set_u32(locimp.wrapping_add(0x10), 0x3f);
            e.call(
                STRING_ASSIGN_0044A640,
                &args![locimp.wrapping_add(0x18), NAME_01063D10],
            );
            e.mem.set_u32(CLASSIC_LOCALE_0126FB9C, locimp);
            e.call(FACET_INCREF, &args![locimp]);
            let classic = e.mem.u32(CLASSIC_LOCALE_0126FB9C);
            e.mem.set_u32(CLASSIC_LOCALE_0126FBB4, classic);
        }
        e.call(LOCKIT_DTOR, &args![lock]);
        Ptr::new(locimp)
    })
}

/// `_setlocale` (`00ed036e`, cdecl: category, locale string).
const SETLOCALE: u32 = 0x00ed_036e;
/// The empty C string (`.rdata`) and `"*"` (`.rdata`), the defaults `_Locinfo_ctor` assigns.
const EMPTY_01011584: u32 = 0x0101_1584;
const STAR_0101FFBC: u32 = 0x0101_ffbc;

// Translated from 00ec3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Locinfo::_Locinfo_ctor(_Locinfo *, const std::string &)` (Ghidra library match;
/// cdecl, two words). The name's characters are at `+4` of the string when its capacity
/// (`+0x18`) is below 0x10, else at the pointer at `+4`. Assigns `setlocale(0, NULL)` (or `""`
/// when that is null) to the string at `+0x3c`, then `setlocale(0, name)` (or `"*"` when the
/// name pointer or the result is null) to the string at `+0x58`.
pub fn locinfo_ctor(e: &mut Engine, locinfo: Ptr, name: Ptr) {
    let name = name.addr();
    let characters = if e.mem.u32(name + 0x18) >= 0x10 {
        e.mem.u32(name + 4)
    } else {
        name + 4
    };
    let mut current = e.call(SETLOCALE, &args![0u32, 0u32]).u32();
    if current == 0 {
        current = EMPTY_01011584;
    }
    let locinfo = locinfo.addr();
    e.call(STRING_ASSIGN_0044A640, &args![locinfo + 0x3c, current]);
    let mut chosen = STAR_0101FFBC;
    if characters != 0 {
        let result = e.call(SETLOCALE, &args![0u32, characters]).u32();
        if result != 0 {
            chosen = result;
        }
    }
    e.call(STRING_ASSIGN_0044A640, &args![locinfo + 0x58, chosen]);
}

/// `____lc_handle_func` (`00ed0cb9`), `____lc_codepage_func` (`00ed0c93`), `__calloc_crt`
/// (`00ed0d24`, cdecl: count, size), `___pctype_func` (`00ed08c8`) and `_memcpy` (`00ec44d0`).
const LC_HANDLE_FUNC: u32 = 0x00ed_0cb9;
const LC_CODEPAGE_FUNC: u32 = 0x00ed_0c93;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const PCTYPE_FUNC: u32 = 0x00ed_08c8;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00ec38a9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Getctype` (Ghidra library match; cdecl, one word: the `_Ctypevec` it fills and returns).
/// `+0` is the word at `+4` of `____lc_handle_func()`'s result, `+4` the code page. It
/// allocates 0x100 entries of 2 bytes (`__calloc_crt`); when that succeeds it copies the 0x200
/// bytes of `___pctype_func()` into them, keeps them at `+8` and sets `+0xc` (delete it) to 1;
/// otherwise `+0xc` is 0 and `+8` the `___pctype_func()` table itself.
pub fn getctype(e: &mut Engine, result: Ptr) -> Ptr {
    let vector = result.addr();
    let handles = e.call(LC_HANDLE_FUNC, &args![]).u32();
    let handle = e.mem.u32(handles + 4);
    e.mem.set_u32(vector, handle);
    let codepage = e.call(LC_CODEPAGE_FUNC, &args![]).u32();
    e.mem.set_u32(vector + 4, codepage);
    let table = e.call(CALLOC_CRT, &args![0x100u32, 2u32]).u32();
    e.mem.set_u32(vector + 8, table);
    if table != 0 {
        let source = e.call(PCTYPE_FUNC, &args![]).u32();
        let destination = e.mem.u32(vector + 8);
        e.call(MEMCPY, &args![destination, source, 0x200u32]);
        e.mem.set_u32(vector + 0xc, 1);
    } else {
        let source = e.call(PCTYPE_FUNC, &args![]).u32();
        e.mem.set_u32(vector + 0xc, 0);
        e.mem.set_u32(vector + 8, source);
    }
    result
}

/// The stream object at `0126fc10` (its vbtable pointer is its first word) and the words
/// `00ec3a1b` reads and writes beside it (`.data`).
const STREAM_0126FC10: u32 = 0x0126_fc10;
const SOURCE_0126FC50: u32 = 0x0126_fc50;
const STREAM_POINTER_0126FC54: u32 = 0x0126_fc54;
const FIELD_BASE_0126FC3C: u32 = 0x0126_fc3c;

// Translated from 00ec3a1b (disassembled; Ghidra has no function there, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (reached by the static initializer at `00fc791c`: `MOV ECX,0126fc48;
/// JMP 00ec3a1b`); thiscall, no stack words. Stores `0126fc10` at `0126fc54`, then with `off`
/// the virtual-base offset at `+4` of the vbtable `[0126fc10]` (read each time): stores the
/// word at `0126fc50` at `0126fc3c + off` and sets bit 1 of the word at `0126fc10 + off +
/// 0x10`. Returns `this` (`MOV EAX,ECX`).
pub fn fn_00ec3a1b(e: &mut Engine, this: u32) -> u32 {
    let value = e.mem.u32(SOURCE_0126FC50);
    let vbtable = e.mem.u32(STREAM_0126FC10);
    e.mem.set_u32(STREAM_POINTER_0126FC54, STREAM_0126FC10);
    let offset = e.mem.u32(vbtable + 4);
    e.mem
        .set_u32(offset.wrapping_add(FIELD_BASE_0126FC3C), value);
    let vbtable = e.mem.u32(STREAM_0126FC10);
    let offset = e.mem.u32(vbtable + 4);
    let state = offset.wrapping_add(STREAM_0126FC10).wrapping_add(0x10);
    let flags = e.mem.u32(state);
    e.mem.set_u32(state, flags | 2);
    this
}

/// `00ad0630` (thiscall: the `basic_streambuf` part's constructor, unnamed in the map) and
/// `00acfc10` (thiscall, two words: the file and a flag; unnamed in the map).
const STREAMBUF_CTOR_00AD0630: u32 = 0x00ad_0630;
const FILEBUF_INIT_00ACFC10: u32 = 0x00ac_fc10;
/// The vftable `00ec3a4f` stores (RTTI `basic_filebuf<char,std::char_traits<char>>`).
const VFTABLE_010A3138: u32 = 0x010a_3138;

// Translated from 00ec3a4f (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the shape of a `basic_filebuf<char>` constructor (thiscall, one word,
/// `RET 4`). Calls `00ad0630(this)`, stores the vftable `010a3138` at `+0`, then calls
/// `00acfc10(this, file, 0)`. Returns `this`.
pub fn fn_00ec3a4f(e: &mut Engine, this: Ptr, file: u32) -> Ptr {
    e.call(STREAMBUF_CTOR_00AD0630, &args![this]);
    e.mem.set_u32(this.addr(), VFTABLE_010A3138);
    e.call(FILEBUF_INIT_00ACFC10, &args![this, file, 0u32]);
    this
}

/// KERNEL32 import slots (one word each): `InitializeCriticalSection` (`00fdf054`),
/// `DeleteCriticalSection` (`00fdf058`), `EnterCriticalSection` (`00fdf05c`),
/// `LeaveCriticalSection` (`00fdf100`), `InterlockedExchange` (`00fdf0e4`, two words).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const INTERLOCKED_EXCHANGE: u32 = 0x00fd_f0e4;

// Translated from 00ec3acf (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxinit` (Ghidra library match; cdecl, one word): `InitializeCriticalSection(mutex)`.
pub fn mtxinit(e: &mut Engine, mutex: u32) {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3adf (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `__Mtxdst` shape; cdecl, one word): `DeleteCriticalSection(mutex)`.
pub fn fn_00ec3adf(e: &mut Engine, mutex: u32) {
    e.call(DELETE_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3aef (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxlock` (Ghidra library match; cdecl, one word): `EnterCriticalSection(mutex)`.
pub fn mtxlock(e: &mut Engine, mutex: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![mutex]);
}

// Translated from 00ec3aff (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Mtxunlock` (Ghidra library match; cdecl, one word): `LeaveCriticalSection(mutex)`.
pub fn mtxunlock(e: &mut Engine, mutex: u32) {
    e.call(LEAVE_CRITICAL_SECTION, &args![mutex]);
}

/// `_abort` (`00ecf335`).
const ABORT: u32 = 0x00ec_f335;
/// The number of free slots of `_Atexit`'s table (`.data`, 10 at start) and the table (10
/// function pointers, filled from the end).
const ATEXIT_FREE_011C165C: u32 = 0x011c_165c;
const ATEXIT_TABLE_0126FC6C: u32 = 0x0126_fc6c;

// Translated from 00ec3b0f (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Atexit` (Ghidra library match `_Atexit`; cdecl, one word): with no free slot left
/// it ends in `_abort` (a tail jump); otherwise it takes the next free slot from the end of the
/// table at `0126fc6c` and stores the function there.
pub fn std_atexit(e: &mut Engine, function: u32) {
    let free = e.mem.u32(ATEXIT_FREE_011C165C);
    if free == 0 {
        e.call(ABORT, &args![]);
        return;
    }
    let free = free - 1;
    e.mem.set_u32(ATEXIT_FREE_011C165C, free);
    e.mem.set_u32(ATEXIT_TABLE_0126FC6C + free * 4, function);
}

// Translated from 00ec3b35 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00ec3b4f` (the `atexit` destructor row at `00fde581` jumps here with `ECX`
/// `0126fc95`, which neither reads).
pub fn fn_00ec3b35(e: &mut Engine) {
    fn_00ec3b4f(e);
}

// Translated from 00ec3b4f (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): runs the functions `std::_Atexit` registered, from the most recent:
/// while the free count at `011c165c` is below 10, takes the entry at that index, increments
/// the count and calls the entry when it is not null.
pub fn fn_00ec3b4f(e: &mut Engine) {
    loop {
        let index = e.mem.u32(ATEXIT_FREE_011C165C);
        if index >= 10 {
            return;
        }
        let function = e.mem.u32(ATEXIT_TABLE_0126FC6C + index * 4);
        e.mem.set_u32(ATEXIT_FREE_011C165C, index + 1);
        if function != 0 {
            e.call(function, &args![]);
        }
    }
}

/// `00ecf80b` (cdecl, one word; unnamed in the map) and `_free` (`00ecd291`).
const RELEASE_00ECF80B: u32 = 0x00ec_f80b;
const FREE: u32 = 0x00ec_d291;
/// The list heads `00ec3b5a` empties: one word each from `0126fc98` to before `0126fd90`.
const LISTS_0126FC98: u32 = 0x0126_fc98;
const LISTS_END_0126FD90: u32 = 0x0126_fd90;

// Translated from 00ec3b5a (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; `00ec3b9a` gives it to `_atexit`): for each list head from
/// `0126fc98` to before `0126fd90`, takes the list with `InterlockedExchange(head, 0)` and, for
/// each node (next at `+0`), calls `00ecf80b` on the node's word at `+8` and frees the node.
pub fn fn_00ec3b5a(e: &mut Engine) {
    let mut head = LISTS_0126FC98;
    loop {
        let mut node = e.call(INTERLOCKED_EXCHANGE, &args![head, 0u32]).u32();
        while node != 0 {
            let next = e.mem.u32(node);
            let word = e.mem.u32(node + 8);
            e.call(RELEASE_00ECF80B, &args![word]);
            e.call(FREE, &args![node]);
            node = next;
        }
        head += 4;
        if (head as i32) >= LISTS_END_0126FD90 as i32 {
            return;
        }
    }
}

/// `_atexit` (`00ec658f`, cdecl, one word).
const ATEXIT: u32 = 0x00ec_658f;

// Translated from 00ec3b9a (disassembled; Ghidra has no function there, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (a C initializer of `_initterm_e`'s table, entry `010113e0`; no
/// arguments): `_atexit(00ec3b5a)`, then 0 when that returned 0 and `0x18` otherwise (`NEG;
/// SBB; AND 0x18`).
pub fn fn_00ec3b9a(e: &mut Engine) -> u32 {
    let result = e.call(ATEXIT, &args![0x00ec_3b5au32]).u32();
    if result != 0 {
        0x18
    } else {
        0
    }
}

// Translated from 00ec3e73 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3e73 - a structured-exception handler: the dispatcher context is not modelled, and its last step (resuming 00ec3e47 on the guard node's saved ESP/EBP, a stack switch) stops with a panic
/// `TranslatorGuardHandler` (Ghidra library match; cdecl, four words: record, guard node,
/// context, dispatcher). Checks the node's cookie (+8 XOR the node, `__security_check_cookie`);
/// a record whose flags (+4) have a bit of 0x66 set (unwinding) sets the node's unwind flag
/// (+0x24) and answers 1. Otherwise `__InternalCxxFrameHandler` (`00eceeb8`) with the record,
/// the node's frame (+0x10), the context, 0, its function info (+0xc), nesting level (+0x14),
/// catch frame (+0x18) and 1; when the unwind flag is still 0, [`unwind_nested_frames`] of the
/// node; then [`call_se_translator`] with the code 0x123 into a local gives the continuation
/// `00ec3e47`, which the exe jumps to with the stack and base pointers saved in the node
/// (+0x1c, +0x20). That jump is not a call in this model.
pub fn translator_guard_handler(
    e: &mut Engine,
    record: Ptr,
    guard: Ptr,
    context: Ptr,
    _unused_3: u32,
) -> u32 {
    let node = guard.addr();
    let cookie = e.mem.u32(node + 8) ^ node;
    security_check_cookie(e, cookie);
    if e.mem.u32(record.addr() + 4) & 0x66 != 0 {
        e.mem.set_u32(node + 0x24, 1);
        return 1;
    }
    let frame = e.mem.u32(node + 0x10);
    let info = e.mem.u32(node + 0xc);
    let level = e.mem.u32(node + 0x14);
    let catch_frame = e.mem.u32(node + 0x18);
    e.call(
        INTERNAL_CXX_FRAME_HANDLER,
        &args![record, frame, context, 0u32, info, level, catch_frame, 1u32],
    );
    if e.mem.u32(node + 0x24) == 0 {
        unwind_nested_frames(e, guard, record);
    }
    let continuation = e.with_stack(4, |e, local| {
        call_se_translator(
            e,
            Ptr::new(GET_CONTINUATION_CODE),
            local,
            Ptr::new(0),
            0,
            0,
            0,
            0,
        );
        e.mem.u32(local.addr())
    });
    panic!(
        "00ec3e73: resuming {continuation:08x} with ESP {:08x} and EBP {:08x} (a stack switch) is not modelled",
        e.mem.u32(node + 0x1c),
        e.mem.u32(node + 0x20)
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec408c, security_check_cookie(u32)),
        entry!(0x00ec3bda, fn_00ec3bda(u32) -> u32),
        entry!(0x00ec_3a75, fn_00ec3a75(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ec_36a6, locale_init() -> Ptr),
        entry!(0x00ec_3740, locinfo_ctor(Ptr, Ptr)),
        entry!(0x00ec_38a9, getctype(Ptr) -> Ptr),
        entry!(0x00ec_3a1b, fn_00ec3a1b(u32) -> u32),
        entry!(0x00ec_3a4f, fn_00ec3a4f(Ptr, u32) -> Ptr),
        entry!(0x00ec_3acf, mtxinit(u32)),
        entry!(0x00ec_3adf, fn_00ec3adf(u32)),
        entry!(0x00ec_3aef, mtxlock(u32)),
        entry!(0x00ec_3aff, mtxunlock(u32)),
        entry!(0x00ec_3b0f, std_atexit(u32)),
        entry!(0x00ec_3b35, fn_00ec3b35()),
        entry!(0x00ec_3b4f, fn_00ec3b4f()),
        entry!(0x00ec_3b5a, fn_00ec3b5a()),
        entry!(0x00ec_3b9a, fn_00ec3b9a() -> u32),
        entry!(0x00ec_379c, tolower_ctype(i32, Ptr) -> i32),
        entry!(0x00ec_3909, toupper_ctype(i32, Ptr) -> i32),
        entry!(0x00ec_3ca6, jump_to_continuation(u32, u32)),
        (0x00ec_3cd8, call_member_function_entry as AbiFn),
        entry!(0x00ec_3cdf, unwind_nested_frames(Ptr, Ptr)),
        entry!(0x00ec_3d33, cxx_frame_handler(u32, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00ec_3d69, catch_guard_handler(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00ec_3d9c, call_se_translator(Ptr, Ptr, Ptr, u32, u32, u32, u32) -> u32),
        entry!(
            0x00ec_3e73,
            translator_guard_handler(Ptr, Ptr, Ptr, u32) -> u32
        ),
        entry!(0x00ec_3f12, get_range_of_trys_to_check(Ptr, i32, i32, Ptr, Ptr) -> u32),
        entry!(0x00ec_3f87, create_frame_info(Ptr, u32) -> Ptr),
        entry!(0x00ec_3fb3, is_exception_object_to_be_destroyed(u32) -> u32),
        entry!(0x00ec_3fda, find_and_unlink_frame(Ptr)),
        entry!(0x00ec_402c, call_catch_block2(Ptr, u32, u32, u32, u32) -> u32),
    ]
}

/// `_isupper` (`00ec8762`) and `_islower` (`00ec87e1`), cdecl, one word.
const ISUPPER: u32 = 0x00ec_8762;
const ISLOWER: u32 = 0x00ec_87e1;
/// `__crtLCMapStringA` (`00ed0883`, cdecl, nine words).
const LCMAP_STRING: u32 = 0x00ed_0883;
/// `_getptd` (`00ed11de`): the thread's CRT data; `+0x80` holds the structured-exception
/// translator, `+0x98` the list of frame infos.
const GETPTD: u32 = 0x00ed_11de;
const PTD_TRANSLATOR: u32 = 0x80;
const PTD_FRAME_INFOS: u32 = 0x98;
/// `_inconsistency` (`00ed1500`, no arguments; terminates).
const INCONSISTENCY: u32 = 0x00ed_1500;
/// `__InternalCxxFrameHandler` (`00eceeb8`, cdecl, eight words).
const INTERNAL_CXX_FRAME_HANDLER: u32 = 0x00ec_eeb8;
/// The `RtlUnwind` thunk (`00ee8210`, stdcall: frame, target address, record, value).
const RTL_UNWIND: u32 = 0x00ee_8210;
/// `_UnwindNestedFrames`' continuation address (the instruction after its `RtlUnwind`).
const UNWIND_CONTINUATION: u32 = 0x00ec_3d0a;
/// `_CallSETranslator`'s guard handler and continuation (`00ec3e73`, `00ec3e47`), and the
/// code that asks for that continuation (0x123).
const TRANSLATOR_GUARD_HANDLER: u32 = 0x00ec_3e73;
const TRANSLATOR_CONTINUATION: u32 = 0x00ec_3e47;
const GET_CONTINUATION_CODE: u32 = 0x123;
/// `_CallCatchBlock2`'s guard handler (`CatchGuardHandler`, `00ec3d69`) and the catch call
/// (`00ed1550`, cdecl: handler address, frame, nesting).
const CATCH_GUARD_HANDLER: u32 = 0x00ec_3d69;
const CALL_SETTING_FRAME: u32 = 0x00ed_1550;

/// The shared body of `__Tolower` / `__Toupper` (one per address below, both inlined in the
/// exe): `kind_bit` is the ctype bit tested (1 upper, 2 lower), `flags` the map flags (0x100
/// lowercase, 0x200 uppercase), `classify` `_isupper` / `_islower`, `ascii_first` the first
/// letter changed by the plain-ASCII fallback and `ascii_shift` its offset.
#[allow(clippy::too_many_arguments)]
fn map_case(
    e: &mut Engine,
    c: i32,
    ctype: Ptr,
    kind_bit: u8,
    flags: u32,
    classify: u32,
    ascii_first: i32,
    ascii_shift: i32,
) -> i32 {
    let ctype = ctype.addr();
    let (handle, codepage) = if ctype == 0 {
        let locale = e.call(LC_HANDLE_FUNC, &args![]).u32();
        let handle = e.mem.u32(locale + 8);
        (handle, e.call(LC_CODEPAGE_FUNC, &args![]).u32())
    } else {
        (e.mem.u32(ctype), e.mem.u32(ctype + 4))
    };
    if handle == 0 {
        return if (c.wrapping_sub(ascii_first) as u32) <= 0x19 {
            c.wrapping_add(ascii_shift)
        } else {
            c
        };
    }
    if (c as u32) < 0x100 {
        if ctype == 0 {
            if e.call(classify, &args![c]).u32() == 0 {
                return c;
            }
        } else {
            let table = e.mem.u32(ctype + 8);
            if e.mem.u8(table.wrapping_add((c as u32).wrapping_mul(2))) & kind_bit == 0 {
                return c;
            }
        }
    }
    let high = c >> 8;
    let lead = if ctype == 0 {
        let table = e.call(PCTYPE_FUNC, &args![]).u32();
        u32::from(e.mem.u16(table.wrapping_add(u32::from(high as u8) * 2))) & 0x8000
    } else {
        let table = e.mem.u32(ctype + 8);
        let entry = e.mem.i16(table.wrapping_add((high as u32 & 0xff) * 2)) as i32 as u32;
        (entry >> 15) & 1
    };
    e.with_stack(8, |e, locals| {
        // +0 the exe's argument slot reused as the source bytes (it still holds the ctype
        // pointer under them), +4 the result bytes.
        let source = locals.addr();
        let result = source + 4;
        e.mem.set_u32(source, ctype);
        let count = if lead != 0 {
            e.mem.set_u8(source, high as u8);
            e.mem.set_u8(source + 1, c as u8);
            e.mem.set_u8(source + 2, 0);
            2u32
        } else {
            e.mem.set_u8(source, c as u8);
            e.mem.set_u8(source + 1, 0);
            1
        };
        let mapped = e
            .call(
                LCMAP_STRING,
                &args![0u32, handle, flags, source, count, result, 3u32, codepage, 1u32],
            )
            .u32();
        match mapped {
            0 => c,
            1 => i32::from(e.mem.u8(result)),
            _ => (i32::from(e.mem.u8(result)) << 8) | i32::from(e.mem.u8(result + 1)),
        }
    })
}

// Translated from 00ec379c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Tolower` (Ghidra library match; cdecl: the character, a `_Ctypevec` or null for the
/// current locale: locale handle +0, code page +4, ctype table +8). Without a locale handle
/// `A`..`Z` become lowercase by ASCII. Otherwise a character below 0x100 that is not
/// uppercase (`_isupper` / table bit 1) is answered as it is; a lead byte (bit 0x8000 of the
/// high byte's entry) makes a two-byte source; `__crtLCMapStringA` (flags 0x100) maps it, 0
/// answering the character, 1 the first result byte, 2 both bytes.
pub fn tolower_ctype(e: &mut Engine, c: i32, ctype: Ptr) -> i32 {
    map_case(e, c, ctype, 1, 0x100, ISUPPER, 0x41, 0x20)
}

// Translated from 00ec3909 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Toupper` (Ghidra library match): `__Tolower`'s body with `a`..`z`, the lowercase test
/// (`_islower` / table bit 2) and map flags 0x200.
pub fn toupper_ctype(e: &mut Engine, c: i32, ctype: Ptr) -> i32 {
    map_case(e, c, ctype, 2, 0x200, ISLOWER, 0x61, -0x20)
}

// Translated from 00ec3ca6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3ca6 - the structured-exception chain at FS:[0] (one node unlinked) and the stack switch (ESP = [node-4], EBX = the node) are not modelled; the jump to the continuation is a call with its frame as its one word, and in the exe it never comes back
/// `_JumpToContinuation` (Ghidra library match; stdcall: the continuation address, the
/// registration node): `FS:[0]` takes the next node of the chain's head, then a jump to the
/// continuation (the code after a catch block) with `EBP` = the node + 0xc (the frame of the
/// function the catch belongs to), `EBX` = the node and `ESP` = the stack pointer the frame
/// kept just below the node (`[node-4]`).
pub fn jump_to_continuation(e: &mut Engine, continuation: u32, node: u32) {
    let frame = node.wrapping_add(0xc);
    e.call(continuation, &args![frame]);
}

// Translated from 00ec3cd8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_CallMemberFunction1` / `2` (Ghidra library match): pops its return address and `this`,
/// puts the return address in place of the function pointer and jumps to that function: a
/// `thiscall` of `function` on `this` with the rest of the caller's words, its result passed
/// back.
pub fn call_member_function(e: &mut Engine, this: u32, function: u32, rest: &[u32]) -> Ret {
    let mut words = vec![this];
    words.extend_from_slice(rest);
    e.call(function, &words)
}

/// The registered form of `00ec3cd8`: `this`, the function, then every other word.
fn call_member_function_entry(e: &mut Engine, words: &[u32]) -> Ret {
    call_member_function(e, words[0], words[1], &words[2..])
}

// Translated from 00ec3cdf (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3cdf - the structured-exception chain at FS:[0] (saved before the unwind and relinked after) is not modelled
/// `_UnwindNestedFrames` (Ghidra library match; stdcall: frame, exception record):
/// `RtlUnwind` (`00ee8210`) of `frame` with the continuation `00ec3d0a`, the record and 0,
/// then bit 1 of the record's flags (+4) cleared.
pub fn unwind_nested_frames(e: &mut Engine, frame: Ptr, record: Ptr) {
    e.call(RTL_UNWIND, &args![frame, UNWIND_CONTINUATION, record, 0u32]);
    let flags = e.mem.u32(record.addr() + 4);
    e.mem.set_u32(record.addr() + 4, flags & 0xffff_fffd);
}

// Translated from 00ec3d33 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___CxxFrameHandler` (Ghidra library match; cdecl with the function info in `EAX`, the
/// compiler's handler stubs load it there): `__InternalCxxFrameHandler` (`00eceeb8`) with
/// the four words, the function info and three zeros, its answer passed on. The leading
/// `function_info` word stands for `EAX`.
pub fn cxx_frame_handler(
    e: &mut Engine,
    function_info: u32,
    record: Ptr,
    frame: Ptr,
    context: Ptr,
    dispatcher: Ptr,
) -> u32 {
    e.call(
        INTERNAL_CXX_FRAME_HANDLER,
        &args![
            record,
            frame,
            context,
            dispatcher,
            function_info,
            0u32,
            0u32,
            0u32
        ],
    )
    .u32()
}

// Translated from 00ec3d69 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3d69 - a structured-exception handler: the dispatcher context it runs under is not modelled (its guard-node cookie check through 00ec408c runs as security_check_cookie)
/// `CatchGuardHandler` (Ghidra library match; cdecl, four words): checks the guard node's
/// cookie (+8 XOR the node's address, `__security_check_cookie`), then
/// `__InternalCxxFrameHandler` with the record, the node's frame (+0x10), the context, 0,
/// its function info (+0xc), nesting level (+0x14), the node and 0; its answer passed on.
pub fn catch_guard_handler(
    e: &mut Engine,
    record: Ptr,
    guard: Ptr,
    context: Ptr,
    _unused_3: u32,
) -> u32 {
    let node = guard.addr();
    let cookie = e.mem.u32(node + 8) ^ node;
    security_check_cookie(e, cookie);
    let frame = e.mem.u32(node + 0x10);
    let info = e.mem.u32(node + 0xc);
    let level = e.mem.u32(node + 0x14);
    e.call(
        INTERNAL_CXX_FRAME_HANDLER,
        &args![record, frame, context, 0u32, info, level, node, 0u32],
    )
    .u32()
}

// Translated from 00ec3d9c (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3d9c - the guard node is not linked into the structured-exception chain at FS:[0] (nor unlinked), so a translator that throws cannot come back through 00ec3e47 with the answer 1
/// `_CallSETranslator` (Ghidra library match; cdecl, seven words). The code 0x123 is the
/// request for the continuation: `*frame` = `00ec3e47`, answer 1. Otherwise a translator
/// guard node (handler `00ec3e73`, cookie = `011c16bc` XOR the node, function info, frame,
/// nesting level, catch frame, the frame's stack and base pointers, a 0 unwind flag) and
/// the exception pointers (record, context) are built in the frame, the thread's translator
/// (`_getptd` +0x80) is called with the record's code and the pointers, and the answer is 0.
#[allow(clippy::too_many_arguments)]
pub fn call_se_translator(
    e: &mut Engine,
    record: Ptr,
    frame: Ptr,
    context: Ptr,
    _unused_3: u32,
    function_info: u32,
    nesting: u32,
    catch_frame: u32,
) -> u32 {
    if record.addr() == GET_CONTINUATION_CODE {
        e.mem.set_u32(frame.addr(), TRANSLATOR_CONTINUATION);
        return 1;
    }
    e.with_stack(0x38, |e, base| {
        // The exe's frame from EBP-0x38: +0 the answer, +4 the exception pointers (record,
        // context), +0xc the translator, +0x10 the guard node (next, handler, cookie, function
        // info, frame, nesting, catch frame, ESP, EBP, unwind flag).
        let base = base.addr();
        let node = base + 0x10;
        let frame_pointer = base + 0x38;
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, TRANSLATOR_GUARD_HANDLER);
        let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC) ^ node;
        e.mem.set_u32(node + 8, cookie);
        e.mem.set_u32(node + 0xc, function_info);
        e.mem.set_u32(node + 0x10, frame.addr());
        e.mem.set_u32(node + 0x14, nesting);
        e.mem.set_u32(node + 0x18, catch_frame);
        e.mem.set_u32(node + 0x1c, 0);
        e.mem.set_u32(node + 0x20, 0);
        e.mem.set_u32(node + 0x24, 0);
        e.mem.set_u32(node + 0x1c, frame_pointer.wrapping_sub(0x3c));
        e.mem.set_u32(node + 0x20, frame_pointer);
        e.mem.set_u32(base, 1);
        e.mem.set_u32(base + 4, record.addr());
        e.mem.set_u32(base + 8, context.addr());
        let ptd = e.call(GETPTD, &args![]).u32();
        let translator = e.mem.u32(ptd + PTD_TRANSLATOR);
        e.mem.set_u32(base + 0xc, translator);
        let code = e.mem.u32(record.addr());
        e.call(translator, &args![code, base + 4]);
        e.mem.set_u32(base, 0);
        e.mem.u32(base)
    })
}

// Translated from 00ec3f12 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_GetRangeOfTrysToCheck` (Ghidra library match; cdecl): walks the function info's try
/// map (+0x10, 0x14-byte entries, count +0xc) down from the end for `level` enclosing try
/// blocks around `state` (an entry whose `tryLow < state <= tryHigh` (+4, +8) counts one
/// level, as does running out at index -1, after which `_inconsistency` is called on the
/// next pass); `*start` and `*end` get the first index after the last match and the index
/// of the previous match; out-of-range results call `_inconsistency`. Answers the start
/// entry's address.
pub fn get_range_of_trys_to_check(
    e: &mut Engine,
    info: Ptr,
    level: i32,
    state: i32,
    start: Ptr,
    end: Ptr,
) -> u32 {
    let info = info.addr();
    let map = e.mem.u32(info + 0x10);
    let count = e.mem.u32(info + 0xc);
    let mut index = count as i32;
    let mut last = count;
    let mut previous = index;
    let mut level = level;
    while level >= 0 {
        if index == -1 {
            e.call(INCONSISTENCY, &args![]);
        }
        index -= 1;
        let entry = (index as u32).wrapping_mul(0x14).wrapping_add(map);
        let inside = e.mem.i32(entry + 4) < state && state <= e.mem.i32(entry + 8);
        if inside || index == -1 {
            level -= 1;
            last = previous as u32;
            previous = index;
        }
    }
    let first = index.wrapping_add(1) as u32;
    e.mem.set_u32(start.addr(), first);
    e.mem.set_u32(end.addr(), last);
    if last > e.mem.u32(info + 0xc) || first > last {
        e.call(INCONSISTENCY, &args![]);
    }
    first.wrapping_mul(0x14).wrapping_add(map)
}

// Translated from 00ec3f87 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__CreateFrameInfo` (Ghidra library match; cdecl): fills `frame` with `object` and the
/// thread's current frame-info list head (`_getptd` +0x98), makes it the new head and
/// answers it.
pub fn create_frame_info(e: &mut Engine, frame: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(frame.addr(), object);
    let ptd = e.call(GETPTD, &args![]).u32();
    let head = e.mem.u32(ptd + PTD_FRAME_INFOS);
    e.mem.set_u32(frame.addr() + 4, head);
    let ptd = e.call(GETPTD, &args![]).u32();
    e.mem.set_u32(ptd + PTD_FRAME_INFOS, frame.addr());
    frame
}

// Translated from 00ec3fb3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__IsExceptionObjectToBeDestroyed` (Ghidra library match; cdecl): 0 when `object` is the
/// object of a frame info in the thread's list (`_getptd` +0x98, linked through +4), else 1.
pub fn is_exception_object_to_be_destroyed(e: &mut Engine, object: u32) -> u32 {
    let ptd = e.call(GETPTD, &args![]).u32();
    let mut info = e.mem.u32(ptd + PTD_FRAME_INFOS);
    while info != 0 {
        if e.mem.u32(info) == object {
            return 0;
        }
        info = e.mem.u32(info + 4);
    }
    1
}

// Translated from 00ec3fda (decompiled, FalloutNV.exe 1.4.0.525)
/// `__FindAndUnlinkFrame` (Ghidra library match; cdecl): unlinks `frame` from the thread's
/// frame-info list (the head, or the entry after which it follows); not finding it ends in
/// `_inconsistency` (a tail jump).
pub fn find_and_unlink_frame(e: &mut Engine, frame: Ptr) {
    let frame = frame.addr();
    let ptd = e.call(GETPTD, &args![]).u32();
    if frame == e.mem.u32(ptd + PTD_FRAME_INFOS) {
        let ptd = e.call(GETPTD, &args![]).u32();
        let next = e.mem.u32(frame + 4);
        e.mem.set_u32(ptd + PTD_FRAME_INFOS, next);
        return;
    }
    let ptd = e.call(GETPTD, &args![]).u32();
    let mut info = e.mem.u32(ptd + PTD_FRAME_INFOS);
    while e.mem.u32(info + 4) != 0 {
        let next = e.mem.u32(info + 4);
        if frame == next {
            let after = e.mem.u32(frame + 4);
            e.mem.set_u32(info + 4, after);
            return;
        }
        info = next;
    }
    e.call(INCONSISTENCY, &args![]);
}

// Translated from 00ec402c (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec402c - the catch guard node is not linked into the structured-exception chain at FS:[0] (nor restored after)
/// `_CallCatchBlock2` (Ghidra library match; cdecl, five words): builds a catch guard node
/// in the frame (handler `CatchGuardHandler` `00ec3d69`, cookie = `011c16bc` XOR the node,
/// the function info, the frame, `nesting + 1`) and calls the catch block through
/// `00ed1550` (handler address, frame, the fifth word), answering its result.
pub fn call_catch_block2(
    e: &mut Engine,
    frame: Ptr,
    function_info: u32,
    handler: u32,
    nesting: u32,
    trylevel: u32,
) -> u32 {
    e.with_stack(0x18, |e, node| {
        let node = node.addr();
        e.mem.set_u32(node, 0);
        let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC) ^ node;
        e.mem.set_u32(node + 8, cookie);
        e.mem.set_u32(node + 0xc, function_info);
        e.mem.set_u32(node + 4, CATCH_GUARD_HANDLER);
        e.mem.set_u32(node + 0x10, frame.addr());
        e.mem.set_u32(node + 0x14, nesting.wrapping_add(1));
        e.call(CALL_SETTING_FRAME, &args![handler, frame, trylevel])
            .u32()
    })
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn case_mapping_with_and_without_a_locale() {
        let mut e = Engine::new();
        // No locale handle: ASCII only.
        let ctype = e.mem.alloc(12);
        assert_eq!(e.call(0x00ec_379c, &args![0x41u32, ctype]).i32(), 0x61);
        assert_eq!(e.call(0x00ec_379c, &args![0x5bu32, ctype]).i32(), 0x5b);
        assert_eq!(e.call(0x00ec_3909, &args![0x7au32, ctype]).i32(), 0x5a);
        assert_eq!(e.call(0x00ec_3909, &args![0x40u32, ctype]).i32(), 0x40);
        // A locale: not uppercase by the table answers the character.
        let table = e.mem.alloc(0x200);
        e.mem.set_u16(table + 2 * 0x41, 1);
        e.mem.set_u16(table + 2 * 0x81, 0x8000);
        e.mem.set_u32(ctype, 0x409);
        e.mem.set_u32(ctype + 4, 1252);
        e.mem.set_u32(ctype + 8, table);
        e.register(LCMAP_STRING, |e, a| {
            assert_eq!((a[1], a[6], a[7], a[8]), (0x409, 3, 1252, 1));
            let first = e.mem.u8(a[3]);
            if a[4] == 1 {
                e.mem.set_u8(a[5], first + 0x20);
                1u32.into_ret()
            } else {
                e.mem.set_u8(a[5], 0x12);
                e.mem.set_u8(a[5] + 1, 0x34);
                2u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00ec_379c, &args![0x42u32, ctype]).i32(), 0x42);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_379c, &args![0x41u32, ctype]).i32(), 0x61);
        assert_eq!(calls_to(&e, LCMAP_STRING)[0][2], 0x100);
        // A wide character whose high byte is a lead byte: two source bytes, two results.
        assert_eq!(e.call(0x00ec_379c, &args![0x8141u32, ctype]).i32(), 0x1234);
        // The mapper answering 0 gives the character back.
        e.register(LCMAP_STRING, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_3909, &args![0x8141u32, ctype]).i32(), 0x8141);
    }

    #[test]
    fn case_mapping_through_the_current_locale() {
        let mut e = Engine::new();
        let locale = e.mem.alloc(12);
        e.mem.set_u32(locale + 8, 0x409);
        e.register_double(LC_HANDLE_FUNC, move |_, _| locale.into_ret());
        e.register(LC_CODEPAGE_FUNC, |_, _| 1252u32.into_ret());
        e.register(ISLOWER, |_, a| u32::from(a[0] == 0x61).into_ret());
        let ctype_table = e.mem.alloc(0x200);
        e.register_double(PCTYPE_FUNC, move |_, _| ctype_table.into_ret());
        e.register(LCMAP_STRING, |e, a| {
            let first = e.mem.u8(a[3]);
            e.mem.set_u8(a[5], first - 0x20);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3909, &args![0x61u32, 0u32]).i32(), 0x41);
        assert_eq!(calls_to(&e, LCMAP_STRING)[0][2], 0x200);
        assert_eq!(e.call(0x00ec_3909, &args![0x62u32, 0u32]).i32(), 0x62);
    }

    #[test]
    fn member_call_thunk_and_handlers_forward_their_words() {
        let mut e = Engine::new();
        e.register(0x0079_0000, |_, a| a.iter().sum::<u32>().into_ret());
        let r = e.call(0x00ec_3cd8, &args![0x100u32, 0x0079_0000u32, 2u32, 3u32]);
        assert_eq!(r.u32(), 0x105);
        e.register(INTERNAL_CXX_FRAME_HANDLER, |_, a| a[4].into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x00ec_3d33, &args![0x77u32, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(r.u32(), 0x77);
        assert_eq!(
            calls_to(&e, INTERNAL_CXX_FRAME_HANDLER),
            vec![vec![1, 2, 3, 4, 0x77, 0, 0, 0]]
        );
    }

    #[test]
    fn catch_guard_checks_the_node_cookie() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.mem.set_u32(SECURITY_COOKIE_011C16BC, 0xabcd);
        e.register(REPORT_GS_FAILURE, |_, _| Ret::default());
        e.register(INTERNAL_CXX_FRAME_HANDLER, |_, _| 9u32.into_ret());
        let node = e.mem.alloc(0x18);
        e.mem.set_u32(node + 8, 0xabcd ^ node);
        e.mem.set_u32(node + 0xc, 0x11);
        e.mem.set_u32(node + 0x10, 0x22);
        e.mem.set_u32(node + 0x14, 0x33);
        e.call_log = Some(vec![]);
        let r = e.call(0x00ec_3d69, &args![0x1000u32, node, 0x2000u32, 0x3000u32]);
        assert_eq!(r.u32(), 9);
        assert!(calls_to(&e, REPORT_GS_FAILURE).is_empty());
        assert_eq!(
            calls_to(&e, INTERNAL_CXX_FRAME_HANDLER),
            vec![vec![0x1000, 0x22, 0x2000, 0, 0x11, 0x33, node, 0]]
        );
        e.mem.set_u32(node + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3d69, &args![0x1000u32, node, 0x2000u32, 0x3000u32]);
        assert_eq!(calls_to(&e, REPORT_GS_FAILURE).len(), 1);
    }

    #[test]
    fn jump_to_continuation_enters_the_continuation_on_the_node_s_frame() {
        let mut e = Engine::new();
        e.register(0x0090_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3ca6, &args![0x0090_0000u32, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0090_0000), vec![vec![0x500c]]);
    }

    #[test]
    fn unwind_nested_frames_clears_the_unwinding_bit() {
        let mut e = Engine::new();
        e.register(RTL_UNWIND, |_, _| Ret::default());
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, 0x7);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3cdf, &args![0x5000u32, record]);
        assert_eq!(
            calls_to(&e, RTL_UNWIND),
            vec![vec![0x5000, UNWIND_CONTINUATION, record, 0]]
        );
        assert_eq!(e.mem.u32(record + 4), 5);
    }

    #[test]
    fn se_translator_answers_the_continuation_or_calls_the_translator() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        let r = e.call(
            0x00ec_3d9c,
            &args![GET_CONTINUATION_CODE, slot, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), 1);
        assert_eq!(e.mem.u32(slot), TRANSLATOR_CONTINUATION);
        e.map(SECURITY_COOKIE_011C16BC, 4);
        let ptd = e.mem.alloc(0x100);
        e.mem.set_u32(ptd + PTD_TRANSLATOR, 0x0079_0000);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register(0x0079_0000, |e, a| {
            assert_eq!(e.mem.u32(a[1] - 4), 1);
            assert_eq!(e.mem.u32(a[1] + 0x10), TRANSLATOR_GUARD_HANDLER);
            Ret::default()
        });
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0xc000_0005);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00ec_3d9c,
            &args![record, 0x5000u32, 0x6000u32, 0u32, 0x11u32, 0x22u32, 0x33u32],
        );
        assert_eq!(r.u32(), 0);
        let call = &calls_to(&e, 0x0079_0000)[0];
        assert_eq!(call[0], 0xc000_0005);
    }

    #[test]
    fn range_of_trys_finds_the_enclosing_blocks() {
        let mut e = Engine::new();
        e.register(INCONSISTENCY, |_, _| Ret::default());
        // Three try blocks: (0, 2], (3, 5], (6, 8] as tryLow < state <= tryHigh.
        let map = e.mem.alloc(0x14 * 3);
        for (i, (low, high)) in [(0i32, 2i32), (3, 5), (6, 8)].iter().enumerate() {
            e.mem.set_i32(map + 0x14 * i as u32 + 4, *low);
            e.mem.set_i32(map + 0x14 * i as u32 + 8, *high);
        }
        let info = e.mem.alloc(0x14);
        e.mem.set_u32(info + 0xc, 3);
        e.mem.set_u32(info + 0x10, map);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // Level 0 around state 4: block 1; start 2, end 3.
        let r = e
            .call(0x00ec_3f12, &args![info, 0u32, 4u32, out, out + 4])
            .u32();
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (2, 3));
        assert_eq!(r, map + 0x28);
        assert!(calls_to(&e, INCONSISTENCY).is_empty());
    }

    #[test]
    fn frame_info_list_create_test_and_unlink() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x100);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register(INCONSISTENCY, |_, _| Ret::default());
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        assert_eq!(e.call(0x00ec_3f87, &args![a, 0x11u32]).u32(), a);
        assert_eq!(e.call(0x00ec_3f87, &args![b, 0x22u32]).u32(), b);
        assert_eq!(e.mem.u32(ptd + 0x98), b);
        assert_eq!(e.mem.u32(b + 4), a);
        assert_eq!(e.call(0x00ec_3fb3, &args![0x11u32]).u32(), 0);
        assert_eq!(e.call(0x00ec_3fb3, &args![0x33u32]).u32(), 1);
        // Unlink the inner one, then the head; an unknown frame ends in _inconsistency.
        e.call(0x00ec_3fda, &args![a]);
        assert_eq!(e.mem.u32(b + 4), 0);
        e.call(0x00ec_3fda, &args![b]);
        assert_eq!(e.mem.u32(ptd + 0x98), 0);
        let c = e.mem.alloc(8);
        e.mem.set_u32(ptd + 0x98, c);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3fda, &args![a]);
        assert_eq!(calls_to(&e, INCONSISTENCY).len(), 1);
    }

    #[test]
    fn catch_block_builds_its_guard_and_calls() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.register(CALL_SETTING_FRAME, |_, a| (a[0] + a[2]).into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00ec_402c,
            &args![0x5000u32, 0x11u32, 0x700u32, 2u32, 0x9u32],
        );
        assert_eq!(r.u32(), 0x709);
        assert_eq!(
            calls_to(&e, CALL_SETTING_FRAME),
            vec![vec![0x700, 0x5000, 9]]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_low_half_and_keeps_the_shifted_high_half() {
        let mut e = Engine::new();
        let mut swap = |value: u32| e.call(0x00ec_3bda, &args![value]).u32();
        assert_eq!(swap(0x0000_1234), 0x0012_3412);
        assert_eq!(swap(0x0000_ff01), 0x00ff_01ff);
        assert_eq!(swap(0xabcd_1234), 0xcd12_3412);
        assert_eq!(swap(0), 0);
        assert_eq!(swap(0x0000_00ff) as u16, 0xff00);
    }

    #[test]
    fn security_check_cookie_returns_on_a_match_and_reports_otherwise() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xdead_beefu32);
        e.register_double(REPORT_GS_FAILURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beefu32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beeeu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == REPORT_GS_FAILURE)
                .count(),
            1
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vbtables_and_initialises_the_virtual_base() {
        let mut e = Engine::new();
        e.put_vtable(VBTABLE_010A3190, &[0, 0x10]);
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3a75, &args![this, 0x11u32, 0x22u32, 1u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VBTABLE_010A3190);
        assert_eq!(e.mem.u32(this + 4), VBTABLE_010A2FA4);
        assert_eq!(e.mem.u32(this + 0x10), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x10, 0x11, 0x22]]
        );
    }

    #[test]
    fn keeps_the_existing_vbtable_when_not_most_derived() {
        let mut e = Engine::new();
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x18);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vbtable);
        e.mem.set_u32(this + 4, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3a75, &args![this, 1u32, 2u32, 0u32]);
        assert_eq!(e.mem.u32(this), vbtable);
        assert_eq!(e.mem.u32(this + 4), 0x77);
        assert_eq!(e.mem.u32(this + 0x18), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x18, 1, 2]]
        );
    }
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn data_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e
    }

    #[test]
    fn locale_init_returns_the_existing_global_locale() {
        let mut e = data_engine();
        e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), 0x4444);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn locale_init_builds_the_classic_locale_under_the_lock() {
        let mut e = data_engine();
        let block = e.mem.alloc(0x34);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(LOCKIT_CTOR, |_, a| a[0].into_ret());
        e.register(LOCKIT_DTOR, |_, _| Ret::default());
        e.register(LOCIMP_CTOR, |_, a| a[0].into_ret());
        e.register(SET_GLOBAL_LOCALE, |_, _| Ret::default());
        e.register(STRING_ASSIGN_0044A640, |_, a| a[0].into_ret());
        e.register(FACET_INCREF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), block);
        let lock = calls_to(&e, LOCKIT_CTOR);
        assert_eq!(lock.len(), 1);
        assert_eq!(lock[0][1], 0);
        assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![lock[0][0]]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, LOCIMP_CTOR), vec![vec![block, 0]]);
        assert_eq!(calls_to(&e, SET_GLOBAL_LOCALE), vec![vec![block]]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![vec![block + 0x18, NAME_01063D10]]
        );
        assert_eq!(calls_to(&e, FACET_INCREF), vec![vec![block]]);
        assert_eq!(e.mem.u32(block + 0x10), 0x3f);
        assert_eq!(e.mem.u32(CLASSIC_LOCALE_0126FB9C), block);
        assert_eq!(e.mem.u32(CLASSIC_LOCALE_0126FBB4), block);
    }

    #[test]
    fn locale_init_keeps_a_locale_set_while_waiting_for_the_lock() {
        let mut e = data_engine();
        e.register(LOCKIT_CTOR, |e, a| {
            e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x7777);
            a[0].into_ret()
        });
        e.register(LOCKIT_DTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_36a6, &args![]).u32(), 0x7777);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&e, LOCKIT_DTOR).len(), 1);
    }

    #[test]
    fn locinfo_ctor_assigns_the_current_and_the_named_locale() {
        let mut e = Engine::new();
        let locinfo = e.mem.alloc(0x80);
        let short = e.mem.alloc(0x20);
        e.mem.set_u32(short + 0x18, 0xf);
        let long = e.mem.alloc(0x20);
        e.mem.set_u32(long + 0x18, 0x10);
        e.mem.set_u32(long + 4, 0x9000);
        e.register(STRING_ASSIGN_0044A640, |_, a| a[0].into_ret());
        // setlocale(0, NULL) answers 0x1111; setlocale(0, name) answers name + 1.
        e.register(SETLOCALE, |_, a| {
            if a[1] == 0 {
                0x1111u32.into_ret()
            } else {
                (a[1] + 1).into_ret()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, short]);
        assert_eq!(
            calls_to(&e, SETLOCALE),
            vec![vec![0, 0], vec![0, short + 4]]
        );
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![
                vec![locinfo + 0x3c, 0x1111],
                vec![locinfo + 0x58, short + 5]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(calls_to(&e, SETLOCALE)[1], vec![0, 0x9000]);
        // Null results: the empty string and the star.
        e.register(SETLOCALE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A640),
            vec![
                vec![locinfo + 0x3c, EMPTY_01011584],
                vec![locinfo + 0x58, STAR_0101FFBC]
            ]
        );
        // A null name pointer: the star without a second setlocale.
        e.mem.set_u32(long + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3740, &args![locinfo, long]);
        assert_eq!(calls_to(&e, SETLOCALE).len(), 1);
        assert_eq!(calls_to(&e, STRING_ASSIGN_0044A640)[1][1], STAR_0101FFBC);
    }

    #[test]
    fn getctype_copies_the_table_or_keeps_the_global_one() {
        for allocated in [true, false] {
            let mut e = Engine::new();
            let handles = e.mem.alloc(0x20);
            e.mem.set_u32(handles + 4, 0x0409);
            e.register_double(LC_HANDLE_FUNC, move |_, _| handles.into_ret());
            e.register(LC_CODEPAGE_FUNC, |_, _| 1252u32.into_ret());
            let table = e.mem.alloc(0x200);
            let answer = if allocated { table } else { 0 };
            e.register_double(CALLOC_CRT, move |_, _| answer.into_ret());
            e.register(PCTYPE_FUNC, |_, _| 0x00ab_0000u32.into_ret());
            e.register(MEMCPY, |_, a| a[0].into_ret());
            let vector = e.mem.alloc(0x10);
            e.mem.set_u32(vector + 0xc, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ec_38a9, &args![vector]).u32(), vector);
            assert_eq!(e.mem.u32(vector), 0x0409);
            assert_eq!(e.mem.u32(vector + 4), 1252);
            assert_eq!(calls_to(&e, CALLOC_CRT), vec![vec![0x100, 2]]);
            if allocated {
                assert_eq!(calls_to(&e, MEMCPY), vec![vec![table, 0x00ab_0000, 0x200]]);
                assert_eq!(e.mem.u32(vector + 8), table);
                assert_eq!(e.mem.u32(vector + 0xc), 1);
            } else {
                assert!(calls_to(&e, MEMCPY).is_empty());
                assert_eq!(e.mem.u32(vector + 8), 0x00ab_0000);
                assert_eq!(e.mem.u32(vector + 0xc), 0);
            }
        }
    }

    #[test]
    fn stream_tail_writes_through_the_virtual_base_offset() {
        let mut e = data_engine();
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x8);
        e.mem.set_u32(STREAM_0126FC10, vbtable);
        e.mem.set_u32(SOURCE_0126FC50, 0xabcd);
        e.mem.set_u32(STREAM_0126FC10 + 8 + 0x10, 0x10);
        assert_eq!(
            e.call(0x00ec_3a1b, &args![0x0126_fc48u32]).u32(),
            0x0126_fc48
        );
        assert_eq!(e.mem.u32(STREAM_POINTER_0126FC54), STREAM_0126FC10);
        assert_eq!(e.mem.u32(FIELD_BASE_0126FC3C + 8), 0xabcd);
        assert_eq!(e.mem.u32(STREAM_0126FC10 + 8 + 0x10), 0x12);
    }

    #[test]
    fn filebuf_ctor_sets_the_vftable_between_its_calls() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.register(STREAMBUF_CTOR_00AD0630, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(FILEBUF_INIT_00ACFC10, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VFTABLE_010A3138);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3a4f, &args![this, 0x77u32]).u32(), this);
        assert_eq!(
            calls_to(&e, FILEBUF_INIT_00ACFC10),
            vec![vec![this, 0x77, 0]]
        );
        assert_eq!(calls_to(&e, STREAMBUF_CTOR_00AD0630), vec![vec![this]]);
    }

    #[test]
    fn mutex_wrappers_pass_the_mutex_to_their_import() {
        for (wrapper, import) in [
            (0x00ec_3acfu32, INITIALIZE_CRITICAL_SECTION),
            (0x00ec_3adf, DELETE_CRITICAL_SECTION),
            (0x00ec_3aef, ENTER_CRITICAL_SECTION),
            (0x00ec_3aff, LEAVE_CRITICAL_SECTION),
        ] {
            let mut e = Engine::new();
            e.register(import, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(wrapper, &args![0x0126_fb30u32]);
            assert_eq!(calls_to(&e, import), vec![vec![0x0126_fb30]]);
        }
    }

    #[test]
    fn std_atexit_fills_from_the_end_and_aborts_when_full() {
        let mut e = data_engine();
        e.register(ABORT, |_, _| Ret::default());
        e.mem.set_u32(ATEXIT_FREE_011C165C, 10);
        e.call(0x00ec_3b0f, &args![0xaaaau32]);
        e.call(0x00ec_3b0f, &args![0xbbbbu32]);
        assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 8);
        assert_eq!(e.mem.u32(ATEXIT_TABLE_0126FC6C + 9 * 4), 0xaaaa);
        assert_eq!(e.mem.u32(ATEXIT_TABLE_0126FC6C + 8 * 4), 0xbbbb);
        e.mem.set_u32(ATEXIT_FREE_011C165C, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3b0f, &args![0xccccu32]);
        assert_eq!(calls_to(&e, ABORT).len(), 1);
        assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 0);
    }

    #[test]
    fn atexit_runner_calls_the_entries_from_the_free_index_on() {
        for entry in [0x00ec_3b4fu32, 0x00ec_3b35] {
            let mut e = data_engine();
            e.register(0x0050_0000, |_, _| Ret::default());
            e.register(0x0050_0010, |_, _| Ret::default());
            e.mem.set_u32(ATEXIT_FREE_011C165C, 7);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 7 * 4, 0x0050_0000);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 8 * 4, 0);
            e.mem.set_u32(ATEXIT_TABLE_0126FC6C + 9 * 4, 0x0050_0010);
            e.call_log = Some(vec![]);
            e.call(entry, &args![0x0126_fc95u32]);
            let order: Vec<u32> = e
                .call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .filter(|a| *a == 0x0050_0000 || *a == 0x0050_0010)
                .collect();
            assert_eq!(order, vec![0x0050_0000, 0x0050_0010]);
            assert_eq!(e.mem.u32(ATEXIT_FREE_011C165C), 10);
        }
    }

    #[test]
    fn list_release_empties_every_head_and_frees_each_node() {
        let mut e = data_engine();
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 8, 0x22);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(LISTS_0126FC98 + 4, first);
        e.register(INTERLOCKED_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], a[1]);
            old.into_ret()
        });
        e.register(RELEASE_00ECF80B, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3b5a, &args![]);
        let exchanges = calls_to(&e, INTERLOCKED_EXCHANGE);
        assert_eq!(exchanges.len(), 62);
        assert_eq!(exchanges[0], vec![LISTS_0126FC98, 0]);
        assert_eq!(exchanges[61][0], LISTS_END_0126FD90 - 4);
        assert_eq!(calls_to(&e, RELEASE_00ECF80B), vec![vec![0x11], vec![0x22]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![first], vec![second]]);
        assert_eq!(e.mem.u32(LISTS_0126FC98 + 4), 0);
    }

    #[test]
    fn initializer_registers_the_list_release_and_maps_the_result() {
        let mut e = Engine::new();
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3b9a, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![0x00ec_3b5a]]);
        e.register(ATEXIT, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ec_3b9a, &args![]).u32(), 0x18);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    #[test]
    fn n089_00ec3e73_marks_unwinding_or_handles_and_resumes() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.set_global(SECURITY_COOKIE_011C16BC, 0x00c0_0c1eu32);
        e.register(INTERNAL_CXX_FRAME_HANDLER, |_, _| Ret::default());
        e.register(RTL_UNWIND, |_, _| Ret::default());
        let node = e.mem.alloc(0x28);
        e.mem.set_u32(node + 8, 0x00c0_0c1e ^ node);
        for (offset, value) in [(0xcu32, 0xf1u32), (0x10, 0xf2), (0x14, 0xf3), (0x18, 0xf4)] {
            e.mem.set_u32(node + offset, value);
        }
        e.mem.set_u32(node + 0x1c, 0x5500);
        e.mem.set_u32(node + 0x20, 0x5600);
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, 2);
        // Unwinding: the flag is set and the answer is 1.
        assert_eq!(
            e.call(0x00ec_3e73, &args![record, node, 0x0c0du32, 0u32])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(node + 0x24), 1);
        // Dispatching: the frame handler runs; with the flag clear the frames are unwound; the
        // resume on the node's stack stops.
        e.mem.set_u32(record + 4, 0x81);
        e.mem.set_u32(node + 0x24, 0);
        e.call_log = Some(vec![]);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00ec_3e73, &args![record, node, 0x0c0du32, 0u32]);
        }));
        assert!(stopped.is_err());
        let log = e.call_log.take().unwrap();
        let handler: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INTERNAL_CXX_FRAME_HANDLER)
            .collect();
        assert_eq!(
            handler[0].1,
            vec![record, 0xf2, 0x0c0d, 0, 0xf1, 0xf3, 0xf4, 1]
        );
        assert!(log.iter().any(|c| c.0 == RTL_UNWIND && c.1[0] == node));
    }
}
