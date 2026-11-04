//! `(unplaced)/run_00ec3020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The counter incremented (`.data`).
const COUNTER_011C15E4: u32 = 0x011c_15e4;
/// `__Mtxinit` (`00ec3acf`, cdecl, one word: the critical section to initialise; Ghidra's
/// library match for the body is `InitializeCriticalSection`).
const MUTEX_INIT: u32 = 0x00ec_3acf;
/// First address after the last 0x18-byte object the loop initialises, and the first object.
const OBJECTS_END: u32 = 0x0126_fb90;
const OBJECTS_START: u32 = 0x0126_fb30;

// Translated from 00ec3289 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a runtime-library one-time initialisation): calls
/// `InterlockedIncrement(&011c15e4)`; when the result is 0, calls `00ec3acf` once for each
/// 0x18-byte object from `0126fb30` while the address is below `0126fb90` (four objects).
/// Returns `this`, which it does not otherwise read.
pub fn fn_00ec3289(e: &mut Engine, this: Ptr) -> Ptr {
    if e.call(INTERLOCKED_INCREMENT, &args![COUNTER_011C15E4])
        .u32()
        == 0
    {
        let mut object = OBJECTS_START;
        loop {
            e.call(MUTEX_INIT, &args![object]);
            object += 0x18;
            if (object as i32) >= OBJECTS_END as i32 {
                break;
            }
        }
    }
    this
}

/// `00aa1070` (cdecl, one word: the size; the allocator wrapper of `run_00aa1010`), `_memset`
/// (`00ec61c0`, cdecl: destination, byte, count).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
/// The vftable `00ec3020` stores (RTTI
/// `NiTMapBase<DFALL<NiTMapItem<const char *,char *>>,...>`, the decorated name cut at 48
/// characters).
const VFTABLE_010F1CA4: u32 = 0x010f_1ca4;

// Translated from 00ec3020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the shape of an `NiTMapBase` constructor (thiscall, one word: the hash
/// size; `RET 4`). Stores the vftable `010f1ca4` at `+0`, the hash size at `+4` and 0 (the
/// count) at `+0xc`, allocates `size * 4` bytes with `00aa1070` for the bucket array at `+8`
/// and clears it with `_memset` (both reading `+4` and `+8` back). Returns `this`.
pub fn fn_00ec3020(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VFTABLE_010F1CA4);
    e.mem.set_u32(base + 4, hash_size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.call(ALLOCATE_00AA1070, &args![bytes]).u32();
    e.mem.set_u32(base + 8, buckets);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.mem.u32(base + 8);
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 00ec3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ios_base::_Callfns` (Ghidra library match; thiscall, one word: the event). For each
/// callback node from `+0x20` (next at `+0`), calls the node's function (`+8`, cdecl) with the
/// event, `this` and the node's index word (`+4`).
pub fn ios_base_callfns(e: &mut Engine, this: Ptr, event: u32) {
    let mut node = e.mem.u32(this.addr() + 0x20);
    while node != 0 {
        let index = e.mem.u32(node + 4);
        let function = e.mem.u32(node + 8);
        e.call(function, &args![event, this, index]);
        node = e.mem.u32(node);
    }
}

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00ec3147 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ios_base::_Tidy` (Ghidra library match; the code takes the stream as its one stack
/// word, cdecl). Calls `_Callfns(ios, 0)`, then deletes every node of the list at `+0x1c` and
/// clears it, then every node of the callback list at `+0x20` and clears it (next at `+0`).
pub fn ios_base_tidy(e: &mut Engine, ios: Ptr) {
    ios_base_callfns(e, ios, 0);
    let base = ios.addr();
    let mut node = e.mem.u32(base + 0x1c);
    while node != 0 {
        let next = e.mem.u32(node);
        e.call(OPERATOR_DELETE, &args![node]);
        node = next;
    }
    e.mem.set_u32(base + 0x1c, 0);
    let mut node = e.mem.u32(base + 0x20);
    while node != 0 {
        let next = e.mem.u32(node);
        e.call(OPERATOR_DELETE, &args![node]);
        node = next;
    }
    e.mem.set_u32(base + 0x20, 0);
}

/// `00acb720` (thiscall; unnamed in the map): the destructor `00ec3192` runs first.
const DESTRUCTOR_00ACB720: u32 = 0x00ac_b720;

// Translated from 00ec3192 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor (thiscall, one word: the flags; `RET 4`). Calls
/// `00acb720(this)` and, when bit 0 of the flags is set, `operator delete(this)`. Returns
/// `this`.
pub fn fn_00ec3192(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00ACB720, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The per-index reference counts of `ios_base` (`.data`, one signed byte per index).
const INDEX_COUNTS_0126FB24: u32 = 0x0126_fb24;

// Translated from 00ec3205 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ios_base::_Ios_base_dtor` (Ghidra library match; cdecl, one word). When the stream's
/// index (`+4`) is not 0 it decrements that index's byte at `0126fb24 + index` and returns
/// while the byte is still above 0 (signed). Otherwise it runs `_Tidy` and, when the word at
/// `+0x24` is not null, that object's scalar deleting destructor (`00ec3192`) with 1.
pub fn ios_base_dtor(e: &mut Engine, ios: Ptr) {
    let base = ios.addr();
    if e.mem.u32(base + 4) != 0 {
        let index = e.mem.u32(base + 4);
        let count = e.mem.u8(index.wrapping_add(INDEX_COUNTS_0126FB24));
        e.mem.set_u8(
            index.wrapping_add(INDEX_COUNTS_0126FB24),
            count.wrapping_sub(1),
        );
        let index = e.mem.u32(base + 4);
        if e.mem.i8(index.wrapping_add(INDEX_COUNTS_0126FB24)) > 0 {
            return;
        }
    }
    ios_base_tidy(e, ios);
    let owned = e.mem.u32(base + 0x24);
    if owned != 0 {
        fn_00ec3192(e, Ptr::new(owned), 1);
    }
}

// Translated from 00ec3246 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Mutex::_Mutex` (Ghidra library match; thiscall): allocates the 0x18-byte critical
/// section with `operator new`, keeps it at `+0` and initialises it with `__Mtxinit`
/// (`00ec3acf`). Returns `this`.
pub fn mutex_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    let section = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    e.mem.set_u32(this.addr(), section);
    e.call(MUTEX_INIT, &args![section]);
    this
}

/// `operator new` (`00401000`, cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00ec3adf` (the `__Mtxdst` shape: `DeleteCriticalSection`), `__Mtxlock` (`00ec3aef`) and
/// `__Mtxunlock` (`00ec3aff`), cdecl, one word each.
const MUTEX_DESTROY: u32 = 0x00ec_3adf;
const MUTEX_LOCK: u32 = 0x00ec_3aef;
const MUTEX_UNLOCK: u32 = 0x00ec_3aff;

// Translated from 00ec3260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `std::_Mutex` destructor shape; thiscall): destroys the critical section at
/// `+0` (`00ec3adf`) and deletes it (`+0` read again).
pub fn fn_00ec3260(e: &mut Engine, this: Ptr) {
    let section = e.mem.u32(this.addr());
    e.call(MUTEX_DESTROY, &args![section]);
    let section = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![section]);
}

/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00ec32ba (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the counterpart of `00ec3289`): `InterlockedDecrement(&011c15e4)`;
/// when the result is below 0 (signed), destroys the four 0x18-byte critical sections from
/// `0126fb30` with `00ec3adf`.
pub fn fn_00ec32ba(e: &mut Engine) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![COUNTER_011C15E4])
        .i32();
    if count >= 0 {
        return;
    }
    let mut object = OBJECTS_START;
    loop {
        e.call(MUTEX_DESTROY, &args![object]);
        object += 0x18;
        if (object as i32) >= OBJECTS_END as i32 {
            break;
        }
    }
}

// Translated from 00ec32e3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Lockit::_Lockit(int)` (Ghidra library match; thiscall, one word): keeps the kind at
/// `+0` and, when it is below 4 (signed), locks the kind's critical section
/// (`0126fb30 + kind * 0x18`, `__Mtxlock`). Returns `this`.
pub fn lockit_ctor(e: &mut Engine, this: Ptr, kind: i32) -> Ptr {
    e.mem.set_i32(this.addr(), kind);
    if kind < 4 {
        let section = (kind.wrapping_mul(0x18) as u32).wrapping_add(OBJECTS_START);
        e.call(MUTEX_LOCK, &args![section]);
    }
    this
}

// Translated from 00ec330b (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `std::_Lockit` destructor shape; thiscall): when the kind at `+0` is below 4
/// (signed), unlocks its critical section (`__Mtxunlock`).
pub fn fn_00ec330b(e: &mut Engine, this: Ptr) {
    let kind = e.mem.i32(this.addr());
    if kind < 4 {
        let section = (kind.wrapping_mul(0x18) as u32).wrapping_add(OBJECTS_START);
        e.call(MUTEX_UNLOCK, &args![section]);
    }
}

/// `_atexit` (`00ec658f`, cdecl, one word) and `_abort` (`00ecf335`).
const ATEXIT: u32 = 0x00ec_658f;
const ABORT: u32 = 0x00ec_f335;

// Translated from 00ec3437 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_AtModuleExit` (Ghidra library match; cdecl, one word): `_atexit(function)`, and a tail jump
/// to `_abort` when that returns non-zero.
pub fn at_module_exit(e: &mut Engine, function: u32) {
    if e.call(ATEXIT, &args![function]).u32() != 0 {
        e.call(ABORT, &args![]);
    }
}

/// `00acb770` (thiscall; unnamed in the map): what a `_Fac_node` and the global locale give
/// before their facet is deleted (`std::locale::facet::_Decref` shape, unconfirmed).
const DECREF_00ACB770: u32 = 0x00ac_b770;

/// The shared tail of `~_Fac_node`, `__Deletegloballocale` and `_Locimp_dtor`: `00acb770` on
/// `facet`, and the scalar deleting destructor (vtable slot 0) with 1 on what it returns when
/// that is not null.
fn release_facet(e: &mut Engine, facet: u32) {
    let deleted = e.call(DECREF_00ACB770, &args![facet]).u32();
    if deleted != 0 {
        e.vcall(deleted, 0, &args![1u32]);
    }
}

// Translated from 00ec3451 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Fac_node::~_Fac_node` (Ghidra library match; thiscall): releases the facet at `+4`
/// (`00acb770`, then its slot-0 destructor with 1 when that returns an object).
pub fn fac_node_dtor(e: &mut Engine, this: Ptr) {
    let facet = e.mem.u32(this.addr() + 4);
    release_facet(e, facet);
}

// Translated from 00ec3466 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Fac_node`'s scalar deleting destructor (Ghidra library match; thiscall, one word:
/// the flags): `~_Fac_node`, then `operator delete(this)` when bit 0 is set. Returns `this`.
pub fn fac_node_scalar_deleting_dtor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fac_node_dtor(e, this);
    if flags as u8 & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ec3487 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Deletegloballocale` (Ghidra library match; cdecl, one word: the address of a locale
/// pointer): when the pointer is not null, releases it as `~_Fac_node` does.
pub fn delete_global_locale(e: &mut Engine, holder: Ptr) {
    let locale = e.mem.u32(holder.addr());
    if locale != 0 {
        release_facet(e, locale);
    }
}

/// The global `_Locimp` (`.data`), the facet list head, and the "registered" byte of
/// `_Setgloballocale`.
const GLOBAL_LOCALE_0126FB98: u32 = 0x0126_fb98;
const FACET_LIST_0126FB94: u32 = 0x0126_fb94;
const REGISTERED_0126FBBC: u32 = 0x0126_fbbc;
/// `std::_Atexit` (`00ec3b0f`, cdecl, one word).
const STD_ATEXIT: u32 = 0x00ec_3b0f;

// Translated from 00ec34a8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_tidy_global` (Ghidra library match; no arguments): under a `_Lockit(0)` (4 bytes on the
/// stack), `__Deletegloballocale(&0126fb98)` and clears `0126fb98`.
pub fn tidy_global(e: &mut Engine) {
    e.with_stack(4, |e, lock| {
        lockit_ctor(e, lock, 0);
        delete_global_locale(e, Ptr::new(GLOBAL_LOCALE_0126FB98));
        e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0);
        fn_00ec330b(e, lock);
    });
}

// Translated from 00ec34d4 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the `std::locale::_Getgloballocale` shape): the word at `0126fb98`.
pub fn fn_00ec34d4(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_LOCALE_0126FB98)
}

// Translated from 00ec34da (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::_Setgloballocale` (Ghidra library match; cdecl, one word): the first time
/// (byte at `0126fbbc` still 0) sets that byte and registers `_tidy_global` (`00ec34a8`) with
/// `std::_Atexit`; then stores the locale at `0126fb98`.
pub fn set_global_locale(e: &mut Engine, locale: u32) {
    if e.mem.u8(REGISTERED_0126FBBC) == 0 {
        e.mem.set_u8(REGISTERED_0126FBBC, 1);
        e.call(STD_ATEXIT, &args![0x00ec_34a8u32]);
    }
    e.mem.set_u32(GLOBAL_LOCALE_0126FB98, locale);
}

/// `_free` (`00ecd291`, cdecl, one word).
const FREE: u32 = 0x00ec_d291;

// Translated from 00ec3504 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3504 - the compiler's exception frame (`__EH_prolog3` at 00ece255, `__EH_epilog3` at 00ece2be: FS:[0] chain) is not translated
/// `std::locale::_Locimp::_Locimp_dtor` (Ghidra library match; cdecl, one word). Under a
/// `_Lockit(0)` (4 bytes on the stack), walks the facet array (`+8`, count at `+0xc`) from the
/// last entry down and releases each non-null entry as `~_Fac_node` does, then frees the array
/// with `_free`.
pub fn locimp_dtor(e: &mut Engine, locimp: Ptr) {
    e.with_stack(4, |e, lock| {
        lockit_ctor(e, lock, 0);
        let base = locimp.addr();
        let mut index = e.mem.u32(base + 0xc);
        while index > 0 {
            index -= 1;
            let slot = e.mem.u32(base + 8).wrapping_add(index.wrapping_mul(4));
            if e.mem.u32(slot) != 0 {
                let facet = e.mem.u32(slot);
                release_facet(e, facet);
            }
        }
        let facets = e.mem.u32(base + 8);
        e.call(FREE, &args![facets]);
        fn_00ec330b(e, lock);
    });
}

// Translated from 00ec3564 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__Fac_tidy` (Ghidra library match; no arguments): under a `_Lockit(0)` (4 bytes on the
/// stack), unlinks each node of the facet list at `0126fb94` (next at `+0`) and runs its scalar
/// deleting destructor with 1.
pub fn fac_tidy(e: &mut Engine) {
    e.with_stack(4, |e, lock| {
        lockit_ctor(e, lock, 0);
        loop {
            let node = e.mem.u32(FACET_LIST_0126FB94);
            if node == 0 {
                break;
            }
            let next = e.mem.u32(node);
            e.mem.set_u32(FACET_LIST_0126FB94, next);
            fac_node_scalar_deleting_dtor(e, Ptr::new(node), 1);
        }
        fn_00ec330b(e, lock);
    });
}

// Translated from 00ec3599 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet::facet_Register` (Ghidra library match; cdecl, one word). With an empty
/// facet list it first registers `__Fac_tidy` (`00ec3564`) with `_AtModuleExit`. Then it pushes
/// a new 8-byte `_Fac_node` (next, facet) on the list at `0126fb94`; when the allocation fails
/// the list becomes null.
pub fn facet_register(e: &mut Engine, facet: u32) {
    if e.mem.u32(FACET_LIST_0126FB94) == 0 {
        at_module_exit(e, 0x00ec_3564);
    }
    let node = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if node != 0 {
        let head = e.mem.u32(FACET_LIST_0126FB94);
        e.mem.set_u32(node, head);
        e.mem.set_u32(node + 4, facet);
    }
    e.mem.set_u32(FACET_LIST_0126FB94, node);
}

/// `_setlocale` (`00ed036e`, cdecl: category, locale string).
const SETLOCALE: u32 = 0x00ed_036e;

// Translated from 00ec35d7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Locinfo::_Locinfo_dtor` (Ghidra library match; cdecl, one word). When the saved
/// locale name (the string at `+0x3c`; length `+0x50`, capacity `+0x54`, characters inline at
/// `+0x40` below capacity 0x10, else at the pointer there) is not empty, restores it with
/// `setlocale(0, name)`. The result is `EAX` as the code leaves it: `setlocale`'s result, or the
/// argument when the name is empty.
pub fn locinfo_dtor(e: &mut Engine, locinfo: Ptr) -> u32 {
    let base = locinfo.addr();
    if e.mem.u32(base + 0x50) == 0 {
        return base;
    }
    let name = if e.mem.u32(base + 0x54) >= 0x10 {
        e.mem.u32(base + 0x40)
    } else {
        base + 0x40
    };
    e.call(SETLOCALE, &args![0u32, name]).u32()
}

/// `basic_string` from a C string (`0044a4c0`, thiscall, one word; the map's name).
const STRING_CTOR_0044A4C0: u32 = 0x0044_a4c0;
/// `"*"` (`.rdata`).
const STAR_0101FFBC: u32 = 0x0101_ffbc;
/// The vftable of `std::locale::_Locimp` (RTTI `_Locimp@locale@std`).
const LOCIMP_VFTABLE_010F1ED8: u32 = 0x010f_1ed8;

// Translated from 00ec35ff (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec35ff - the compiler's exception frame (`__EH_prolog3` at 00ece255, `__EH_epilog3` at 00ece2be: FS:[0] chain) is not translated
/// `std::locale::_Locimp::_Locimp(bool)` (Ghidra library match; thiscall, one word): reference
/// count (`+4`) 1, `+8`, `+0xc` and `+0x10` 0, the vftable `010f1ed8`, the flag's byte at
/// `+0x14`, and the name string at `+0x18` constructed from `"*"`. Returns `this`.
pub fn locimp_ctor(e: &mut Engine, this: Ptr, transparent: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 4, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base, LOCIMP_VFTABLE_010F1ED8);
    e.mem.set_u8(base + 0x14, transparent as u8);
    e.call(STRING_CTOR_0044A4C0, &args![base + 0x18, STAR_0101FFBC]);
    this
}

/// `std::out_of_range::out_of_range(const std::string &)` (Ghidra library match of
/// `00ec3093`, which stores `.?AVout_of_range@std@@`'s vtable `010a57fc`; thiscall, one word).
const OUT_OF_RANGE_CONSTRUCT: u32 = 0x00ec_3093;
/// `std::length_error::length_error(const std::string &)` (Ghidra library match of
/// `0044a2e0`, which stores `.?AVlength_error@std@@`'s vtable `010173dc`; thiscall, one word).
const LENGTH_ERROR_CONSTRUCT: u32 = 0x0044_a2e0;
/// `_CxxThrowException(object, throw_info)` (`00ec7894`, stdcall).
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `"invalid string position"` and `"string too long"`.
const INVALID_STRING_POSITION: u32 = 0x010f_1e28;
const STRING_TOO_LONG: u32 = 0x010f_1e18;
/// The throw information of `std::out_of_range` and of `std::length_error`.
const OUT_OF_RANGE_THROW_INFO: u32 = 0x0117_327c;
const LENGTH_ERROR_THROW_INFO: u32 = 0x0114_0cb0;
/// The frame's slots for the message string (`EBP-0x28`) and the exception (`EBP-0x50`).
const STRING_SLOT_SIZE: u32 = 0x28;
const EXCEPTION_SLOT_SIZE: u32 = 0x28;

/// Builds the message string from `text`, the exception from the string (`construct`) and
/// throws it with `throw_info`. Does not return.
fn throw_string_error(e: &mut Engine, text: u32, construct: u32, throw_info: u32) {
    e.with_stack(STRING_SLOT_SIZE, |e, message| {
        e.call(STRING_CTOR_0044A4C0, &args![message, text]);
        e.with_stack(EXCEPTION_SLOT_SIZE, |e, exception| {
            e.call(construct, &args![exception, message]);
            e.call(CXX_THROW_EXCEPTION, &args![exception, throw_info]);
        });
    });
    unreachable!("_CxxThrowException does not return (00ec7894)")
}

// Translated from 00ec30e8 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec30e8 - the compiler's exception frame (`__EH_prolog3` at 00ece255: FS:[0] chain) is not translated
/// `std::_Xran` shape (no map name; no arguments): throws `std::out_of_range("invalid string
/// position")` (the string `0044a4c0`, the exception `00ec3093`, `_CxxThrowException` with
/// `0117327c`). Does not return.
pub fn fn_00ec30e8(e: &mut Engine) {
    throw_string_error(
        e,
        INVALID_STRING_POSITION,
        OUT_OF_RANGE_CONSTRUCT,
        OUT_OF_RANGE_THROW_INFO,
    );
}

// Translated from 00ec30b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec30b0 - the compiler's exception frame (`__EH_prolog3` at 00ece255: FS:[0] chain) is not translated
/// `std::_Xlen` shape (no map name; no arguments): throws `std::length_error("string too
/// long")` (the string `0044a4c0`, the exception `0044a2e0`, `_CxxThrowException` with
/// `01140cb0`). Does not return.
pub fn fn_00ec30b0(e: &mut Engine) {
    throw_string_error(
        e,
        STRING_TOO_LONG,
        LENGTH_ERROR_CONSTRUCT,
        LENGTH_ERROR_THROW_INFO,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_30e8, fn_00ec30e8()),
        entry!(0x00ec_30b0, fn_00ec30b0()),
        entry!(0x00ec_3289, fn_00ec3289(Ptr) -> Ptr),
        entry!(0x00ec_3020, fn_00ec3020(Ptr, u32) -> Ptr),
        entry!(0x00ec_3120, ios_base_callfns(Ptr, u32)),
        entry!(0x00ec_3147, ios_base_tidy(Ptr)),
        entry!(0x00ec_3192, fn_00ec3192(Ptr, u32) -> Ptr),
        entry!(0x00ec_3205, ios_base_dtor(Ptr)),
        entry!(0x00ec_3246, mutex_ctor(Ptr) -> Ptr),
        entry!(0x00ec_3260, fn_00ec3260(Ptr)),
        entry!(0x00ec_32ba, fn_00ec32ba()),
        entry!(0x00ec_32e3, lockit_ctor(Ptr, i32) -> Ptr),
        entry!(0x00ec_330b, fn_00ec330b(Ptr)),
        entry!(0x00ec_3437, at_module_exit(u32)),
        entry!(0x00ec_3451, fac_node_dtor(Ptr)),
        entry!(0x00ec_3466, fac_node_scalar_deleting_dtor(Ptr, u32) -> Ptr),
        entry!(0x00ec_3487, delete_global_locale(Ptr)),
        entry!(0x00ec_34a8, tidy_global()),
        entry!(0x00ec_34d4, fn_00ec34d4() -> u32),
        entry!(0x00ec_34da, set_global_locale(u32)),
        entry!(0x00ec_3504, locimp_dtor(Ptr)),
        entry!(0x00ec_3564, fac_tidy()),
        entry!(0x00ec_3599, facet_register(u32)),
        entry!(0x00ec_35d7, locinfo_dtor(Ptr) -> u32),
        entry!(0x00ec_35ff, locimp_ctor(Ptr, u32) -> Ptr),
        entry!(0x00ec_3093, out_of_range_ctor(Ptr, u32) -> Ptr),
        entry!(0x00ec_31b3, ios_base_addstd(Ptr)),
        entry!(0x00ec_3241, fn_00ec3241() -> u32),
        entry!(0x00ec_3277, fn_00ec3277(Ptr) -> u32),
        entry!(0x00ec_3280, fn_00ec3280(Ptr) -> u32),
        entry!(0x00ec_3322, fiopen_wide(Ptr, u32, u32) -> u32),
        entry!(0x00ec_33da, fiopen(Ptr, u32, u32) -> u32),
    ]
}

/// The base constructor `00ec3093` runs first (`0044a310`; thiscall: the message).
const LOGIC_ERROR_BASE_CTOR: u32 = 0x0044_a310;
/// `std::out_of_range`'s vtable (`010a57fc`; RTTI `.?AVout_of_range@std@@`).
const OUT_OF_RANGE_VTABLE: u32 = 0x010a_57fc;
/// The standard streams' table (`0126fafc`, eight words indexed by the stream's index).
const STANDARD_STREAMS_0126FAFC: u32 = 0x0126_fafc;
/// `00ece56e` (unnamed; the jump target of `00ec3241`).
const PROCESSING_THROW_00ECE56E: u32 = 0x00ec_e56e;
/// The open modes `fiopen_wide` accepts (`010f1e48`, zero-terminated words; the search
/// starts with the value 1 and then reads from `010f1e4c`), the `fopen` mode texts for
/// them (`011c15ec`, one pointer per mode) and the read-only mode text (`010f1e44`).
const VALID_MODES_010F1E48: u32 = 0x010f_1e48;
const VALID_MODES_010F1E4C: u32 = 0x010f_1e4c;
const MODE_TEXTS_011C15EC: u32 = 0x011c_15ec;
const READ_MODE_TEXT_010F1E44: u32 = 0x010f_1e44;
/// The CRT calls of the two `_Fiopen`s: the wide open `00ecef9e` (cdecl: name, mode text,
/// protection), `fclose` (`00ec9907`), `fseek` (`00ecb65a`: stream, 0, 2 = end) and
/// `mbstowcs_s` (`00ecf315`: 0, buffer, 0x104, text, 0x103).
const WIDE_OPEN: u32 = 0x00ec_ef9e;
const FCLOSE: u32 = 0x00ec_9907;
const FSEEK: u32 = 0x00ec_b65a;
const MBSTOWCS_S: u32 = 0x00ec_f315;

// Translated from 00ec3093 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `std::out_of_range` constructor (by the vtable it sets; this file's string range
/// check builds its exception with it): the base constructor `0044a310` with `message`,
/// then the vtable `010a57fc`. Returns `this`.
pub fn out_of_range_ctor(e: &mut Engine, this: Ptr, message: u32) -> Ptr {
    e.call(LOGIC_ERROR_BASE_CTOR, &args![this, message]);
    e.mem.set_u32(this.addr(), OUT_OF_RANGE_VTABLE);
    this
}

// Translated from 00ec31b3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ios_base::_Addstd` (Ghidra library match; cdecl, one word): under the lock of kind
/// 2 (`_Lockit`), the stream's index (+4) starts at 1 and moves up while the table slot
/// (`0126fafc + 4 * index`) holds another stream, at most to 8; the stream is stored there
/// and the index's byte count (`0126fb24 + index`) incremented.
pub fn ios_base_addstd(e: &mut Engine, ios: Ptr) {
    let ios = ios.addr();
    e.with_stack(4, |e, lock| {
        lockit_ctor(e, lock, 2);
        e.mem.set_u32(ios + 4, 1);
        loop {
            let index = e.mem.u32(ios + 4);
            let slot = e.mem.u32(
                index
                    .wrapping_mul(4)
                    .wrapping_add(STANDARD_STREAMS_0126FAFC),
            );
            if slot == 0 || slot == ios {
                break;
            }
            e.mem.set_u32(ios + 4, index.wrapping_add(1));
            if e.mem.u32(ios + 4) >= 8 {
                break;
            }
        }
        let index = e.mem.u32(ios + 4);
        e.mem.set_u32(
            index
                .wrapping_mul(4)
                .wrapping_add(STANDARD_STREAMS_0126FAFC),
            ios,
        );
        let index = e.mem.u32(ios + 4);
        let at = index.wrapping_add(INDEX_COUNTS_0126FB24);
        let count = e.mem.u8(at);
        e.mem.set_u8(at, count.wrapping_add(1));
        fn_00ec330b(e, lock);
    });
}

// Translated from 00ec3241 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump to `00ece56e`, passing on its answer.
pub fn fn_00ec3241(e: &mut Engine) -> u32 {
    e.call(PROCESSING_THROW_00ECE56E, &args![]).u32()
}

// Translated from 00ec3277 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `std::_Mutex::_Lock` shape; thiscall): `__Mtxlock` on the
/// critical section at +0, passing on its `EAX`.
pub fn fn_00ec3277(e: &mut Engine, this: Ptr) -> u32 {
    let section = e.mem.u32(this.addr());
    e.call(MUTEX_LOCK, &args![section]).u32()
}

// Translated from 00ec3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `std::_Mutex::_Unlock` shape; thiscall): `__Mtxunlock` on
/// the critical section at +0, passing on its `EAX`.
pub fn fn_00ec3280(e: &mut Engine, this: Ptr) -> u32 {
    let section = e.mem.u32(this.addr());
    e.call(MUTEX_UNLOCK, &args![section]).u32()
}

// Translated from 00ec3322 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Fiopen(const wchar_t *, int, int)` (Ghidra library match; cdecl): `ate` (4) and
/// `noreplace` (0x80) are kept aside, 0x40 sets bit 1 and 8
/// sets bit 2; the mode without 4, 0x40, 0x80 must be one of the table
/// (`010f1e48`, searched from the value 1), else null. With `noreplace` and bit 1 an
/// existing file (opened with the text at `010f1e44`) is closed and null answered. The file
/// is opened with the mode's text (`011c15ec`); with `ate` it is moved to its end, a failed
/// move closing it and answering null.
pub fn fiopen_wide(e: &mut Engine, name: Ptr, mode: u32, protection: u32) -> u32 {
    let ate = mode & 4;
    let no_replace = mode & 0x80;
    let mut mode = mode;
    if mode & 0x40 != 0 {
        mode |= 1;
    }
    if mode & 8 != 0 {
        mode |= 2;
    }
    let wanted = mode & 0xffff_ff3b;
    let mut index = 0u32;
    let mut candidate = 1u32;
    while candidate != wanted {
        candidate = e.mem.u32(VALID_MODES_010F1E4C + 4 * index);
        index += 1;
        if candidate == 0 {
            break;
        }
    }
    let slot = index << 2;
    if e.mem.u32(VALID_MODES_010F1E48 + slot) == 0 {
        return 0;
    }
    if no_replace != 0 && mode & 1 != 0 {
        let existing = e
            .call(WIDE_OPEN, &args![name, READ_MODE_TEXT_010F1E44, protection])
            .u32();
        if existing != 0 {
            e.call(FCLOSE, &args![existing]);
            return 0;
        }
    }
    let text = e.mem.u32(MODE_TEXTS_011C15EC + slot);
    let file = e.call(WIDE_OPEN, &args![name, text, protection]).u32();
    if file == 0 {
        return 0;
    }
    if ate != 0 && e.call(FSEEK, &args![file, 0u32, 2u32]).u32() != 0 {
        e.call(FCLOSE, &args![file]);
        return 0;
    }
    file
}

// Translated from 00ec33da (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec33da - the /GS cookie check (00ec408c on the cookie at 011c16bc XOR the frame) is not translated
/// `std::_Fiopen(const char *, int, int)` (Ghidra library match; cdecl): the name widened
/// into a 0x104-character local (`mbstowcs_s`, at most 0x103); a failure answers null,
/// otherwise the wide `_Fiopen`'s answer.
pub fn fiopen(e: &mut Engine, name: Ptr, mode: u32, protection: u32) -> u32 {
    e.with_stack(0x20c, |e, buffer| {
        let failed = e
            .call(MBSTOWCS_S, &args![0u32, buffer, 0x104u32, name, 0x103u32])
            .u32();
        if failed != 0 {
            return 0;
        }
        fiopen_wide(e, buffer, mode, protection)
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
    fn out_of_range_runs_the_base_then_sets_its_vtable() {
        let mut e = Engine::new();
        e.register(LOGIC_ERROR_BASE_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3093, &args![this, 0x77u32]).u32(), this);
        assert_eq!(calls_to(&e, LOGIC_ERROR_BASE_CTOR), vec![vec![this, 0x77]]);
        assert_eq!(e.mem.u32(this), OUT_OF_RANGE_VTABLE);
    }

    #[test]
    fn addstd_takes_the_first_free_or_own_slot() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MUTEX_LOCK, |_, _| Ret::default());
        e.register(MUTEX_UNLOCK, |_, _| Ret::default());
        let ios = e.mem.alloc(0x30);
        e.mem.set_u32(STANDARD_STREAMS_0126FAFC + 4, 0x5000);
        e.mem.set_u32(STANDARD_STREAMS_0126FAFC + 8, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00ec_31b3, &args![ios]);
        assert_eq!(e.mem.u32(ios + 4), 3);
        assert_eq!(e.mem.u32(STANDARD_STREAMS_0126FAFC + 12), ios);
        assert_eq!(e.mem.u8(INDEX_COUNTS_0126FB24 + 3), 1);
        assert_eq!(calls_to(&e, MUTEX_LOCK).len(), 1);
        assert_eq!(calls_to(&e, MUTEX_UNLOCK).len(), 1);
        // Its own slot again: the same index, the count 2.
        e.call(0x00ec_31b3, &args![ios]);
        assert_eq!(e.mem.u32(ios + 4), 3);
        assert_eq!(e.mem.u8(INDEX_COUNTS_0126FB24 + 3), 2);
    }

    #[test]
    fn thin_wrappers_pass_on_their_callees() {
        let mut e = Engine::new();
        e.register(PROCESSING_THROW_00ECE56E, |_, _| 0x101u32.into_ret());
        e.register(MUTEX_LOCK, |_, a| (a[0] + 1).into_ret());
        e.register(MUTEX_UNLOCK, |_, a| (a[0] + 2).into_ret());
        assert_eq!(e.call(0x00ec_3241, &args![]).u32(), 0x101);
        let mutex = e.mem.alloc(4);
        e.mem.set_u32(mutex, 0x40);
        assert_eq!(e.call(0x00ec_3277, &args![mutex]).u32(), 0x41);
        assert_eq!(e.call(0x00ec_3280, &args![mutex]).u32(), 0x42);
    }

    fn modes(e: &mut Engine) {
        e.map(0x010f_1000, 0x1000);
        e.map(0x011c_1000, 0x1000);
        // Valid modes 1 (in), 2 (out), 0x12 (out | trunc).
        for (i, m) in [1u32, 2, 0x12, 0].iter().enumerate() {
            e.mem.set_u32(VALID_MODES_010F1E48 + 4 * i as u32, *m);
        }
        for i in 0..3u32 {
            e.mem.set_u32(MODE_TEXTS_011C15EC + 4 * i, 0x9000 + i);
        }
    }

    #[test]
    fn wide_fiopen_checks_the_mode_and_opens() {
        let mut e = Engine::new();
        modes(&mut e);
        e.register(FCLOSE, |_, _| Ret::default());
        e.register(FSEEK, |_, _| 0u32.into_ret());
        e.register(WIDE_OPEN, |_, a| {
            (if a[1] == READ_MODE_TEXT_010F1E44 {
                0
            } else {
                0x7000
            })
            .into_ret()
        });
        e.call_log = Some(vec![]);
        // out | trunc (0x12): slot 2.
        assert_eq!(
            e.call(0x00ec_3322, &args![0x100u32, 0x12u32, 0x40u32])
                .u32(),
            0x7000
        );
        assert_eq!(calls_to(&e, WIDE_OPEN), vec![vec![0x100, 0x9002, 0x40]]);
        // An unknown mode answers null without opening.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3322, &args![0x100u32, 0x30u32, 0x40u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, WIDE_OPEN).is_empty());
        // in | ate (5): moved to the end; a failed move closes and answers null.
        assert_eq!(
            e.call(0x00ec_3322, &args![0x100u32, 5u32, 0x40u32]).u32(),
            0x7000
        );
        e.register(FSEEK, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3322, &args![0x100u32, 5u32, 0x40u32]).u32(),
            0
        );
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7000]]);
        // in | noreplace with an existing file: closed, null.
        e.register(WIDE_OPEN, |_, _| 0x7100u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3322, &args![0x100u32, 0x81u32, 0x40u32])
                .u32(),
            0
        );
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7100]]);
    }

    #[test]
    fn narrow_fiopen_widens_first() {
        let mut e = Engine::new();
        modes(&mut e);
        e.register(WIDE_OPEN, |_, _| 0x7000u32.into_ret());
        e.register_double(MBSTOWCS_S, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_33da, &args![0x100u32, 2u32, 0x40u32]).u32(),
            0x7000
        );
        let widen = calls_to(&e, MBSTOWCS_S);
        assert_eq!(
            (widen[0][0], widen[0][2], widen[0][3], widen[0][4]),
            (0, 0x104, 0x100, 0x103)
        );
        assert_eq!(calls_to(&e, WIDE_OPEN)[0][0], widen[0][1]);
        e.register(MBSTOWCS_S, |_, _| 22u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_33da, &args![0x100u32, 2u32, 0x40u32]).u32(),
            0
        );
        assert!(calls_to(&e, WIDE_OPEN).is_empty());
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
    fn initialises_four_objects_when_the_increment_gives_zero() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MUTEX_INIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3289, &args![0x55u32]).u32(), 0x55);
        assert_eq!(
            calls_to(&mut e, INTERLOCKED_INCREMENT),
            vec![vec![COUNTER_011C15E4]]
        );
        assert_eq!(
            calls_to(&mut e, MUTEX_INIT),
            vec![
                vec![0x0126_fb30],
                vec![0x0126_fb48],
                vec![0x0126_fb60],
                vec![0x0126_fb78]
            ]
        );
    }

    #[test]
    fn does_nothing_more_when_the_increment_is_not_zero() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(MUTEX_INIT, |_, _| panic!("not initialised"));
        assert_eq!(e.call(0x00ec_3289, &args![0x55u32]).u32(), 0x55);
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

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, a| a.first().copied().unwrap_or(0).into_ret());
        }
    }

    #[test]
    fn nitmapbase_ctor_allocates_and_clears_the_buckets() {
        let mut e = Engine::new();
        let buckets = e.mem.alloc(0x40);
        e.register_double(ALLOCATE_00AA1070, move |_, _| buckets.into_ret());
        quiet(&mut e, &[MEMSET]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3020, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VFTABLE_010F1CA4);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 8), buckets);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, ALLOCATE_00AA1070), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
    }

    #[test]
    fn callfns_calls_each_callback_with_event_stream_and_index() {
        let mut e = Engine::new();
        e.register(0x0050_0000, |_, _| Ret::default());
        e.register(0x0050_0010, |_, _| Ret::default());
        let second = e.mem.alloc(0xc);
        e.mem.write(second, &[0; 12]);
        e.mem.set_u32(second + 4, 7);
        e.mem.set_u32(second + 8, 0x0050_0010);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 3);
        e.mem.set_u32(first + 8, 0x0050_0000);
        let ios = e.mem.alloc(0x28);
        e.mem.set_u32(ios + 0x20, first);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3120, &args![ios, 2u32]);
        assert_eq!(calls_to(&e, 0x0050_0000), vec![vec![2, ios, 3]]);
        assert_eq!(calls_to(&e, 0x0050_0010), vec![vec![2, ios, 7]]);
    }

    fn two_node_list(e: &mut Engine) -> (u32, u32) {
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second, 0);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        (first, second)
    }

    #[test]
    fn tidy_deletes_both_lists_after_the_erase_event() {
        let mut e = Engine::new();
        quiet(&mut e, &[OPERATOR_DELETE, 0x0050_0000]);
        let ios = e.mem.alloc(0x28);
        let (a, b) = two_node_list(&mut e);
        e.mem.set_u32(ios + 0x1c, a);
        let callback = e.mem.alloc(0xc);
        e.mem.set_u32(callback, 0);
        e.mem.set_u32(callback + 8, 0x0050_0000);
        e.mem.set_u32(ios + 0x20, callback);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3147, &args![ios]);
        assert_eq!(calls_to(&e, 0x0050_0000), vec![vec![0, ios, 0]]);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![a], vec![b], vec![callback]]
        );
        assert_eq!(e.mem.u32(ios + 0x1c), 0);
        assert_eq!(e.mem.u32(ios + 0x20), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_bit_zero() {
        for (flags, deleted) in [(1u32, true), (0, false), (0x100, false), (3, true)] {
            let mut e = Engine::new();
            quiet(&mut e, &[DESTRUCTOR_00ACB720, OPERATOR_DELETE]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ec_3192, &args![0x4000u32, flags]).u32(), 0x4000);
            assert_eq!(calls_to(&e, DESTRUCTOR_00ACB720), vec![vec![0x4000]]);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), deleted as usize);
        }
    }

    #[test]
    fn ios_base_dtor_keeps_a_shared_index_and_tidies_the_last() {
        let mut e = data_engine();
        quiet(&mut e, &[DESTRUCTOR_00ACB720, OPERATOR_DELETE]);
        let ios = e.mem.alloc(0x28);
        e.mem.write(ios, &[0; 0x28]);
        e.mem.set_u32(ios + 4, 2);
        e.mem.set_u8(INDEX_COUNTS_0126FB24 + 2, 2);
        e.mem.set_u32(ios + 0x24, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3205, &args![ios]);
        assert_eq!(e.mem.u8(INDEX_COUNTS_0126FB24 + 2), 1);
        assert!(calls_to(&e, DESTRUCTOR_00ACB720).is_empty(), "still shared");
        e.call_log = Some(vec![]);
        e.call(0x00ec_3205, &args![ios]);
        assert_eq!(e.mem.u8(INDEX_COUNTS_0126FB24 + 2), 0);
        assert_eq!(calls_to(&e, DESTRUCTOR_00ACB720), vec![vec![0x6000]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x6000]]);
        // Index 0 tidies at once; no owned object, no destructor.
        e.mem.set_u32(ios + 4, 0);
        e.mem.set_u32(ios + 0x24, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3205, &args![ios]);
        assert!(calls_to(&e, DESTRUCTOR_00ACB720).is_empty());
    }

    #[test]
    fn mutex_ctor_and_dtor_create_and_destroy_the_section() {
        let mut e = Engine::new();
        quiet(&mut e, &[MUTEX_INIT, MUTEX_DESTROY, OPERATOR_DELETE]);
        e.register(OPERATOR_NEW, |_, _| 0x7000u32.into_ret());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3246, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x18]]);
        assert_eq!(e.mem.u32(this), 0x7000);
        assert_eq!(calls_to(&e, MUTEX_INIT), vec![vec![0x7000]]);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3260, &args![this]);
        assert_eq!(calls_to(&e, MUTEX_DESTROY), vec![vec![0x7000]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x7000]]);
    }

    #[test]
    fn mutex_table_teardown_runs_only_below_zero() {
        for (count, destroyed) in [(-1i32, 4usize), (0, 0)] {
            let mut e = Engine::new();
            e.register_double(INTERLOCKED_DECREMENT, move |_, _| count.into_ret());
            quiet(&mut e, &[MUTEX_DESTROY]);
            e.call_log = Some(vec![]);
            e.call(0x00ec_32ba, &args![]);
            assert_eq!(
                calls_to(&e, INTERLOCKED_DECREMENT),
                vec![vec![COUNTER_011C15E4]]
            );
            let sections = calls_to(&e, MUTEX_DESTROY);
            assert_eq!(sections.len(), destroyed);
            if destroyed > 0 {
                assert_eq!(sections[3], vec![0x0126_fb78]);
            }
        }
    }

    #[test]
    fn lockit_locks_and_unlocks_only_the_four_kinds() {
        for (kind, locked) in [
            (0i32, Some(0x0126_fb30u32)),
            (3, Some(0x0126_fb78)),
            (4, None),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[MUTEX_LOCK, MUTEX_UNLOCK]);
            let lock = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ec_32e3, &args![lock, kind]).u32(), lock);
            assert_eq!(e.mem.i32(lock), kind);
            e.call(0x00ec_330b, &args![lock]);
            let expected: Vec<Vec<u32>> = locked.into_iter().map(|s| vec![s]).collect();
            assert_eq!(calls_to(&e, MUTEX_LOCK), expected);
            assert_eq!(calls_to(&e, MUTEX_UNLOCK), expected);
        }
    }

    #[test]
    fn at_module_exit_aborts_when_atexit_fails() {
        for (result, aborted) in [(0u32, false), (1, true)] {
            let mut e = Engine::new();
            e.register_double(ATEXIT, move |_, _| result.into_ret());
            quiet(&mut e, &[ABORT]);
            e.call_log = Some(vec![]);
            e.call(0x00ec_3437, &args![0x1234u32]);
            assert_eq!(calls_to(&e, ATEXIT), vec![vec![0x1234]]);
            assert_eq!(calls_to(&e, ABORT).len(), aborted as usize);
        }
    }

    /// A facet object whose slot-0 destructor is a recorded double at `0x0050_0100`.
    fn deletable(e: &mut Engine) -> u32 {
        e.register(0x0050_0100, |_, _| Ret::default());
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0050_0100);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fac_node_dtor_deletes_the_released_facet() {
        for deleted in [true, false] {
            let mut e = Engine::new();
            let object = deletable(&mut e);
            let answer = if deleted { object } else { 0 };
            e.register_double(DECREF_00ACB770, move |_, _| answer.into_ret());
            let node = e.mem.alloc(8);
            e.mem.set_u32(node + 4, 0x5555);
            e.call_log = Some(vec![]);
            e.call(0x00ec_3451, &args![node]);
            assert_eq!(calls_to(&e, DECREF_00ACB770), vec![vec![0x5555]]);
            let expected = if deleted {
                vec![vec![object, 1]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&e, 0x0050_0100), expected);
        }
    }

    #[test]
    fn fac_node_deleting_dtor_frees_with_bit_zero() {
        let mut e = Engine::new();
        quiet(&mut e, &[OPERATOR_DELETE]);
        e.register(DECREF_00ACB770, |_, _| Ret::default());
        let node = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_3466, &args![node, 1u32]).u32(), node);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![node]]);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3466, &args![node, 2u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, DECREF_00ACB770).len(), 1);
    }

    #[test]
    fn delete_global_locale_skips_a_null_pointer() {
        let mut e = Engine::new();
        let object = deletable(&mut e);
        e.register_double(DECREF_00ACB770, move |_, _| object.into_ret());
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3487, &args![holder]);
        assert!(calls_to(&e, DECREF_00ACB770).is_empty());
        e.mem.set_u32(holder, 0x9999);
        e.call(0x00ec_3487, &args![holder]);
        assert_eq!(calls_to(&e, DECREF_00ACB770), vec![vec![0x9999]]);
        assert_eq!(calls_to(&e, 0x0050_0100), vec![vec![object, 1]]);
    }

    #[test]
    fn tidy_global_releases_and_clears_under_lock_zero() {
        let mut e = data_engine();
        quiet(&mut e, &[MUTEX_LOCK, MUTEX_UNLOCK]);
        e.register(DECREF_00ACB770, |_, _| Ret::default());
        e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x8888);
        e.call_log = Some(vec![]);
        e.call(0x00ec_34a8, &args![]);
        assert_eq!(calls_to(&e, MUTEX_LOCK), vec![vec![0x0126_fb30]]);
        assert_eq!(calls_to(&e, DECREF_00ACB770), vec![vec![0x8888]]);
        assert_eq!(calls_to(&e, MUTEX_UNLOCK), vec![vec![0x0126_fb30]]);
        assert_eq!(e.mem.u32(GLOBAL_LOCALE_0126FB98), 0);
        assert_eq!(e.call(0x00ec_34d4, &args![]).u32(), 0);
    }

    #[test]
    fn global_locale_getter_reads_the_word() {
        let mut e = data_engine();
        e.mem.set_u32(GLOBAL_LOCALE_0126FB98, 0x1357);
        assert_eq!(e.call(0x00ec_34d4, &args![]).u32(), 0x1357);
    }

    #[test]
    fn set_global_locale_registers_the_tidy_once() {
        let mut e = data_engine();
        quiet(&mut e, &[STD_ATEXIT]);
        e.call_log = Some(vec![]);
        e.call(0x00ec_34da, &args![0x1111u32]);
        e.call(0x00ec_34da, &args![0x2222u32]);
        assert_eq!(calls_to(&e, STD_ATEXIT), vec![vec![0x00ec_34a8]]);
        assert_eq!(e.mem.u8(REGISTERED_0126FBBC), 1);
        assert_eq!(e.mem.u32(GLOBAL_LOCALE_0126FB98), 0x2222);
    }

    #[test]
    fn locimp_dtor_releases_the_facets_from_the_last_and_frees_the_array() {
        let mut e = Engine::new();
        quiet(&mut e, &[MUTEX_LOCK, MUTEX_UNLOCK, FREE]);
        e.register(DECREF_00ACB770, |_, _| Ret::default());
        let facets = e.mem.alloc(12);
        e.mem.set_u32(facets, 0xa);
        e.mem.set_u32(facets + 4, 0);
        e.mem.set_u32(facets + 8, 0xc);
        let locimp = e.mem.alloc(0x34);
        e.mem.set_u32(locimp + 8, facets);
        e.mem.set_u32(locimp + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3504, &args![locimp]);
        assert_eq!(calls_to(&e, DECREF_00ACB770), vec![vec![0xc], vec![0xa]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![facets]]);
        assert_eq!(calls_to(&e, MUTEX_LOCK), vec![vec![0x0126_fb30]]);
        assert_eq!(calls_to(&e, MUTEX_UNLOCK), vec![vec![0x0126_fb30]]);
    }

    #[test]
    fn fac_tidy_unlinks_and_deletes_every_node() {
        let mut e = data_engine();
        quiet(&mut e, &[MUTEX_LOCK, MUTEX_UNLOCK, OPERATOR_DELETE]);
        e.register(DECREF_00ACB770, |_, _| Ret::default());
        let (a, b) = two_node_list(&mut e);
        e.mem.set_u32(a + 4, 0x1a);
        e.mem.set_u32(b + 4, 0x1b);
        e.mem.set_u32(FACET_LIST_0126FB94, a);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3564, &args![]);
        assert_eq!(calls_to(&e, DECREF_00ACB770), vec![vec![0x1a], vec![0x1b]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![a], vec![b]]);
        assert_eq!(e.mem.u32(FACET_LIST_0126FB94), 0);
    }

    #[test]
    fn facet_register_pushes_a_node_and_registers_the_tidy_first() {
        let mut e = data_engine();
        e.register(ATEXIT, |_, _| Ret::default());
        let node = e.mem.alloc(8);
        e.register_double(OPERATOR_NEW, move |_, _| node.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3599, &args![0x4242u32]);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![0x00ec_3564]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 4), 0x4242);
        assert_eq!(e.mem.u32(FACET_LIST_0126FB94), node);
        // A non-empty list: no registration; a failed allocation empties the list.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_3599, &args![0x4343u32]);
        assert!(calls_to(&e, ATEXIT).is_empty());
        assert_eq!(e.mem.u32(FACET_LIST_0126FB94), 0);
    }

    #[test]
    fn locinfo_dtor_restores_a_saved_name() {
        let mut e = Engine::new();
        e.register(SETLOCALE, |_, _| 0x3333u32.into_ret());
        let locinfo = e.mem.alloc(0x70);
        e.mem.write(locinfo, &[0; 0x70]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_35d7, &args![locinfo]).u32(), locinfo);
        assert!(calls_to(&e, SETLOCALE).is_empty());
        e.mem.set_u32(locinfo + 0x50, 3);
        e.mem.set_u32(locinfo + 0x54, 0xf);
        assert_eq!(e.call(0x00ec_35d7, &args![locinfo]).u32(), 0x3333);
        e.mem.set_u32(locinfo + 0x54, 0x10);
        e.mem.set_u32(locinfo + 0x40, 0xbeef);
        e.call(0x00ec_35d7, &args![locinfo]);
        assert_eq!(
            calls_to(&e, SETLOCALE),
            vec![vec![0, locinfo + 0x40], vec![0, 0xbeef]]
        );
    }

    #[test]
    fn locimp_ctor_initialises_the_fields_and_the_name() {
        let mut e = Engine::new();
        quiet(&mut e, &[STRING_CTOR_0044A4C0]);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_35ff, &args![this, 0x101u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LOCIMP_VFTABLE_010F1ED8);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x14), 1);
        assert_eq!(
            calls_to(&e, STRING_CTOR_0044A4C0),
            vec![vec![this + 0x18, STAR_0101FFBC]]
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// Runs `address` with doubles for the string, both exception constructors and a
    /// `_CxxThrowException` that panics with "thrown"; answers the logged calls.
    fn throw_calls(address: u32) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        for callee in [
            STRING_CTOR_0044A4C0,
            OUT_OF_RANGE_CONSTRUCT,
            LENGTH_ERROR_CONSTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        e.call_log = Some(vec![]);
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(address, &args![]);
        }));
        let payload = thrown.expect_err("the function does not return");
        assert_eq!(payload.downcast_ref::<&str>(), Some(&"thrown"));
        e.call_log.take().unwrap()
    }

    #[test]
    fn fn_00ec30e8_throws_out_of_range() {
        let log = throw_calls(0x00ec_30e8);
        let string = log
            .iter()
            .find(|(a, _)| *a == STRING_CTOR_0044A4C0)
            .unwrap();
        assert_eq!(string.1[1], INVALID_STRING_POSITION);
        let built = log
            .iter()
            .find(|(a, _)| *a == OUT_OF_RANGE_CONSTRUCT)
            .unwrap();
        assert_eq!(built.1[1], string.1[0]);
        let thrown = log.iter().find(|(a, _)| *a == CXX_THROW_EXCEPTION).unwrap();
        assert_eq!(thrown.1, vec![built.1[0], OUT_OF_RANGE_THROW_INFO]);
        assert!(log.iter().all(|(a, _)| *a != LENGTH_ERROR_CONSTRUCT));
    }

    #[test]
    fn fn_00ec30b0_throws_length_error() {
        let log = throw_calls(0x00ec_30b0);
        let string = log
            .iter()
            .find(|(a, _)| *a == STRING_CTOR_0044A4C0)
            .unwrap();
        assert_eq!(string.1[1], STRING_TOO_LONG);
        let built = log
            .iter()
            .find(|(a, _)| *a == LENGTH_ERROR_CONSTRUCT)
            .unwrap();
        assert_eq!(built.1[1], string.1[0]);
        let thrown = log.iter().find(|(a, _)| *a == CXX_THROW_EXCEPTION).unwrap();
        assert_eq!(thrown.1, vec![built.1[0], LENGTH_ERROR_THROW_INFO]);
        assert!(log.iter().all(|(a, _)| *a != OUT_OF_RANGE_CONSTRUCT));
    }
}
