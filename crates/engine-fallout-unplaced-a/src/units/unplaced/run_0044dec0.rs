//! `(unplaced)/run_0044dec0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0044e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the function whose address is the word at `+0xc` (`cdecl`, five
/// words: the object, then the four stack words of this function) and adds
/// its answer to the word at `+4` (unnamed in the map; `ECX`, four stack
/// words, `RET 0x10`; answers the callee's result). `TESFile` writes a
/// `BSFile` through it (`00473180`: data, size, the address of a 1, 1), the
/// word at `+4` then being the file position.
pub fn fn_0044e120(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    let function = e.mem.u32(this.addr() + 0xc);
    let done = e
        .call(function, &args![this, first, second, third, fourth])
        .u32();
    let position = e.mem.u32(this.addr() + 4).wrapping_add(done);
    e.mem.set_u32(this.addr() + 4, position);
    done
}

/// The function `fn_0044df20` calls on the member (`0044dff0`).
const CALLEE_0044DFF0: u32 = 0x0044_dff0;

// Translated from 0044df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044dff0` with `ECX` = `this + 0x8` and passes on its `bool` result (`AL`).
pub fn fn_0044df20(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_0044DFF0, &args![this.byte_add(0x8)]).bool()
}

// Translated from 0044ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044ee40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044ed20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_ee40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The empty function `0040fbe0` (no arguments; the engine map's `Error`).
const EMPTY_HOOK: u32 = 0x0040_fbe0;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(ptr)` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `thiscall` getter on an object (no arguments), compared by the
/// lookup loops of `0044e550` and `0044ea60` (and read by `0044e170`); the
/// engine map names it nothing.
const OBJECT_KEY: u32 = 0x0055_9450;
/// The release that ends `0044e550`, `0044ea60` and `0044eb80`
/// (`thiscall` on `this`, no arguments).
const LOOP_RELEASE: u32 = 0x0052_9000;
/// `thiscall(this, word)` assigning a word into a pointer holder
/// (`0044df40`, `0044e550`).
const HOLDER_ASSIGN: u32 = 0x0092_c820;
/// `thiscall(this, word)` initializer of a pointer holder (`0044df40`,
/// `0044dfc0`).
const HOLDER_INIT: u32 = 0x0052_8cb0;
/// `thiscall(this, word)` assigning a pointer (the engine map's
/// `NiPointer<QueuedFile>::operator=`).
const POINTER_ASSIGN: u32 = 0x006f_74f0;
/// Lookup through the object at `this+0` (`thiscall(object, key)`).
const LOOKUP: u32 = 0x0044_ee20;
/// Two-word constructor of a one-word local (`thiscall`).
const LOCAL_CONSTRUCT: u32 = 0x006e_cba0;
/// One-word assignment into that local (`thiscall`; the engine map's
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer`).
const LOCAL_ASSIGN: u32 = 0x0053_7e90;
/// `TlsAlloc`, `TlsSetValue`, `TlsFree` (import slots).
const TLS_ALLOC: u32 = 0x00fd_f048;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
const TLS_FREE: u32 = 0x00fd_f050;
/// `memcpy_s(dest, destsize, src, count)` (cdecl).
const MEMCPY_S: u32 = 0x00ec_7c66;
/// `memmove_s(dest, destsize, src, count)` (cdecl, `00ec7ce3`).
const MEMMOVE_S: u32 = 0x00ec_7ce3;

// Translated from 0044dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this+0` with bit 0 cleared (a pointer whose low bit is a
/// flag).
pub fn fn_0044dec0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr()) & 0xffff_fffe
}

// Translated from 0044df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same as `0044dec0`: the pointer at `this+0` without its flag bit.
pub fn fn_0044df00(e: &mut Engine, this: Ptr) -> u32 {
    fn_0044dec0(e, this)
}

// Translated from 0044df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (class unconfirmed): initializes the holder at `+4`
/// (`00528cb0` with 0) and the member at `+8` (`0044dee0`), stores `first`
/// at `+0`, then assigns `second` to the holder at `+4` (`0092c820`).
/// Returns `this`.
// Translation incomplete: 0044df40 - the FS:[0] exception frame (unwinds the +4 holder if a later step throws) is not translated
pub fn fn_0044df40(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(HOLDER_INIT, &args![this.addr() + 4, 0u32]);
    e.call(0x0044_dee0, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), first);
    e.call(HOLDER_ASSIGN, &args![this.addr() + 4, second]);
    this
}

// Translated from 0044dfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an 8-byte node (class unconfirmed): `+0` = 0, then the
/// holder at `+4` initialized with 0 (`00528cb0`). Returns `this`.
pub fn fn_0044dfc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.call(HOLDER_INIT, &args![this.addr() + 4, 0u32]);
    this
}

// Translated from 0044dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the word at `this+0` is set.
pub fn fn_0044dff0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) & 1 != 0
}

// Translated from 0044e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` true) or clears bit 0 of the word at `this+0`.
pub fn fn_0044e010(e: &mut Engine, this: Ptr, flag: bool) {
    let word = e.mem.u32(this.addr());
    if flag {
        e.mem.set_u32(this.addr(), word | 1);
    } else {
        e.mem.set_u32(this.addr(), word & 0xffff_fffe);
    }
}

// Translated from 0044e050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the 64-bit key at `item+0x10` (`0044e0d0`), asks `this`'s vtable
/// slot `+0x24` for its index and slot `+0x14` whether it applies; when it
/// does, calls `004019a0` on the counter at `this[+0x54] + index * 4` and
/// on `this+0x58`, and answers true.
pub fn fn_0044e050(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let key = fn_0044e0d0(e, item);
    let index = e
        .vcall(this.addr(), 0x24, &args![key as u32, (key >> 32) as u32])
        .u32();
    let key = fn_0044e0d0(e, item);
    if e.vcall(this.addr(), 0x14, &args![key as u32, (key >> 32) as u32])
        .bool()
    {
        let counters = e.mem.u32(this.addr() + 0x54);
        e.call(
            0x0040_19a0,
            &args![counters.wrapping_add(index.wrapping_mul(4))],
        );
        e.call(0x0040_19a0, &args![this.addr() + 0x58]);
        true
    } else {
        false
    }
}

// Translated from 0044e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 64-bit value at `this+0x10` (low word) and `+0x14` (high word),
/// returned in `EDX:EAX`.
pub fn fn_0044e0d0(e: &mut Engine, this: Ptr) -> u64 {
    let low = e.mem.u32(this.addr() + 0x10) as u64;
    let high = e.mem.u32(this.addr() + 0x14) as u64;
    low | high << 32
}

// Translated from 0044e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `count` 4-byte items from `data` through the stream (cdecl):
/// `0044e120(stream, data, count * 4, &4, 1)`; answers its result.
pub fn fn_0044e0f0(e: &mut Engine, stream: Ptr, data: u32, count: u32) -> u32 {
    e.with_stack(4, |e, item_size| {
        e.mem.set_u32(item_size.addr(), 4);
        let bytes = e.mem.u32(item_size.addr()).wrapping_mul(count);
        fn_0044e120(e, stream, data, bytes, item_size.addr(), 1)
    })
}

// Translated from 0044e170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a queue (class unconfirmed; the guard local names
/// `ThreadSafeStructures.inl` line 0x115): under the lock at `+0x20`
/// (`0040fbf0`/`0040fba0`) and a scope local (`00404eb0`/`00404ee0`),
/// deletes the object at `+0x14` (`0044ed20`), zeroes `+0x14`/`+0x18`, and
/// deletes every node of the list at `+4` (each node's first word is the
/// next one; its `+4` pointer is released first). Unless `keep_empty` is
/// set it then makes a fresh 8-byte head node (`0044dfc0`, at `+4` and
/// `+8`) and a fresh 0x10-byte object (`006c73b0`, given the old object's
/// `00559450` value) at `+0x14`.
// Translation incomplete: 0044e170 - the FS:[0] exception frame (frees the new blocks and runs the scope local's destructor when a constructor throws) is not translated
pub fn fn_0044e170(e: &mut Engine, this: Ptr, keep_empty: bool) {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    e.with_stack(4, |e, scope| {
        e.call(
            0x0040_4eb0,
            &args![scope, 6u32, 1u32, 0x0101_73f8u32, 0x115u32],
        );
        let old = e.mem.u32(this.addr() + 0x14);
        let key = e.call(OBJECT_KEY, &args![old]).u32();
        let object = e.mem.u32(this.addr() + 0x14);
        if object != 0 {
            fn_0044ed20(e, Ptr::new(object), 1);
        }
        e.mem.set_u32(this.addr() + 0x14, 0);
        e.mem.set_u32(this.addr() + 0x18, 0);
        while e.mem.u32(this.addr() + 4) != 0 {
            let head = e.mem.u32(this.addr() + 4);
            let next = e.mem.u32(head);
            e.call(POINTER_ASSIGN, &args![head + 4, 0u32]);
            let node = e.mem.u32(this.addr() + 4);
            if node != 0 {
                fn_0044ed50(e, Ptr::new(node), 1);
            }
            e.mem.set_u32(this.addr() + 4, next);
        }
        if !keep_empty {
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let node = if block == 0 {
                0
            } else {
                fn_0044dfc0(e, Ptr::new(block)).addr()
            };
            e.mem.set_u32(this.addr() + 4, node);
            let head = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(this.addr() + 8, head);
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let object = if block == 0 {
                0
            } else {
                e.call(0x006c_73b0, &args![block, key]).u32()
            };
            e.mem.set_u32(this.addr() + 0x14, object);
        }
        e.call(0x0040_fba0, &args![this.addr() + 0x20]);
        e.call(0x0040_4ee0, &args![scope]);
    });
}

// Translated from 0044e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TlsAlloc()`; answers its index.
pub fn fn_0044e330(e: &mut Engine) -> u32 {
    e.call(TLS_ALLOC, &[]).u32()
}

// Translated from 0044e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TlsSetValue(index, value)` (cdecl); answers its result.
pub fn fn_0044e340(e: &mut Engine, index: u32, value: u32) -> u32 {
    e.call(TLS_SET_VALUE, &args![index, value]).u32()
}

// Translated from 0044e360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TlsFree(index)` (cdecl, the import Ghidra names at `00fdf050`);
/// answers its result.
pub fn fn_0044e360(e: &mut Engine, index: u32) -> u32 {
    e.call(TLS_FREE, &args![index]).u32()
}

// Translated from 0044e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands out the next per-thread slot (class unconfirmed): increments the
/// counter at `+0xc` (`0040b460`); when the new index less one is below
/// the limit at `+0`, makes the slot's object through `factory`'s vtable
/// slot `+4` (given the index), records it and the current thread's id
/// (`0040fc90`) in the 8-byte entry at `this[+8] + index * 8`, and stores it
/// in the TLS index at `+4` (`0044e340`). Answers the object, or 0.
pub fn fn_0044e3a0(e: &mut Engine, this: Ptr, factory: Ptr) -> u32 {
    let mut object = 0;
    let index = e
        .call(0x0040_b460, &args![this.addr() + 0xc])
        .u32()
        .wrapping_sub(1);
    if index < e.mem.u32(this.addr()) {
        object = e.vcall(factory.addr(), 4, &args![index]).u32();
        let entries = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(
            entries.wrapping_add(index.wrapping_mul(8)).wrapping_add(4),
            object,
        );
        let thread = e.call(0x0040_fc90, &[]).u32();
        let entries = e.mem.u32(this.addr() + 8);
        e.mem
            .set_u32(entries.wrapping_add(index.wrapping_mul(8)), thread);
        let tls_index = e.mem.u32(this.addr() + 4);
        fn_0044e340(e, tls_index, object);
    }
    object
}

// Translated from 0044e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Traits_helper::copy_s`/`move_s` with the secure-traits tag (cdecl,
/// five words; the tag is not read): forwards to `0044e440`.
pub fn fn_0044e420(
    e: &mut Engine,
    destination: u32,
    destination_size: u32,
    source: u32,
    count: u32,
    _unused_4: u32,
) -> u32 {
    fn_0044e440(e, destination, destination_size, source, count)
}

// Translated from 0044e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `char_traits::_Copy_s` (cdecl): `memcpy_s(dest, destsize, src, count)`,
/// answers `dest`.
pub fn fn_0044e440(
    e: &mut Engine,
    destination: u32,
    destination_size: u32,
    source: u32,
    count: u32,
) -> u32 {
    e.call(
        MEMCPY_S,
        &args![destination, destination_size, source, count],
    );
    destination
}

// Translated from 0044e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 4-byte elements `[first, last)` to `destination` with
/// `memmove_s` (cdecl; the count is the signed difference over 4, nothing
/// is moved when it is not positive) and answers the end of the copy.
pub fn fn_0044e4a0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let count = (last.wrapping_sub(first) as i32) >> 2;
    let end = destination.wrapping_add((count as u32).wrapping_mul(4));
    if count > 0 {
        let bytes = (count as u32) << 2;
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 0044e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes `node` on the free list at `+0x10` (class unconfirmed): releases
/// the node's `+4` pointer (`006f74f0` with 0), links it to the old head
/// (`006ecd40`), makes it the head and counts it at `+0xc`; when the count
/// reaches the value `0084e3a0` reads from the object at `+0`, runs
/// `0044eed0` on `this`.
pub fn fn_0044e4f0(e: &mut Engine, this: Ptr, node: Ptr) {
    e.call(POINTER_ASSIGN, &args![node.addr() + 4, 0u32]);
    let head = e.mem.u32(this.addr() + 0x10);
    e.call(0x006e_cd40, &args![node, head]);
    e.mem.set_u32(this.addr() + 0x10, node.addr());
    let count = e.mem.u32(this.addr() + 0xc).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0xc, count);
    let owner = e.mem.u32(this.addr());
    let limit = e.call(0x0084_e3a0, &args![owner]).u32();
    if e.mem.u32(this.addr() + 0xc) == limit {
        e.call(0x0044_eed0, &args![this]);
    }
}

// Translated from 0044e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `key` up through the object at `+0` (`0044ee20`) until the found
/// entry is stable (its `00559450` value matches that of a local copy of
/// the entry's pointer at `+0x14`), recording the entry at `+0x10`, its
/// first word at `+0x14` and the unflagged pointer at `*this[+8]`; with a
/// non-null pointer, passes its first word and `argument` to vtable slot
/// `+0x30` of the object at `+0`, assigns the pointer's `+4` word into
/// `holder` (`0092c820`) and answers true, starting over while the
/// pointer's flag (`0044df20`) is set. A null pointer answers false. Ends
/// with `00529000` on `this`.
pub fn fn_0044e550(e: &mut Engine, this: Ptr, key: u32, argument: u32, holder: Ptr) -> bool {
    let mut found = false;
    e.call(EMPTY_HOOK, &[]);
    e.with_stack(4, |e, local| {
        e.call(LOCAL_CONSTRUCT, &args![local, 0u32, 0u32]);
        loop {
            let owner = e.mem.u32(this.addr());
            let entry = e.call(LOOKUP, &args![owner, key]).u32();
            e.mem.set_u32(this.addr() + 0x10, entry);
            let first = e.mem.u32(entry);
            e.mem.set_u32(this.addr() + 0x14, first);
            let link = Ptr::new(this.addr() + 0x14);
            let pointer = fn_0044dec0(e, link);
            let slot = e.mem.u32(this.addr() + 8);
            e.mem.set_u32(slot, pointer);
            let pointer = fn_0044dec0(e, link);
            e.call(LOCAL_ASSIGN, &args![local, pointer]);
            fn_0044e010(e, local, false);
            let entry = e.mem.u32(this.addr() + 0x10);
            let entry_key = e.call(OBJECT_KEY, &args![entry]).u32();
            let local_key = e.call(OBJECT_KEY, &args![local]).u32();
            if entry_key != local_key {
                continue;
            }
            if fn_0044dec0(e, link) == 0 {
                found = false;
                break;
            }
            let pointer = fn_0044df00(e, link);
            let word = e.mem.u32(pointer);
            let owner = e.mem.u32(this.addr());
            e.vcall(owner, 0x30, &args![word, argument]);
            let pointer = fn_0044df00(e, link);
            e.call(HOLDER_ASSIGN, &args![holder, pointer + 4]);
            found = true;
            let pointer = fn_0044df00(e, link);
            if !fn_0044df20(e, Ptr::new(pointer)) {
                break;
            }
        }
    });
    e.call(LOOP_RELEASE, &args![this]);
    found
}

// Translated from 0044ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The variant of `0044e550` that copies the pointer's `+4` word into
/// `*out` instead of assigning it to a holder, and makes its local copy
/// with the two-word constructor (`006ecba0(0, pointer)`) on each pass.
pub fn fn_0044ea60(e: &mut Engine, this: Ptr, key: u32, argument: u32, out: Ptr) -> bool {
    let mut found = false;
    e.with_stack(4, |e, local| loop {
        let owner = e.mem.u32(this.addr());
        let entry = e.call(LOOKUP, &args![owner, key]).u32();
        e.mem.set_u32(this.addr() + 0x10, entry);
        let first = e.mem.u32(entry);
        e.mem.set_u32(this.addr() + 0x14, first);
        let link = Ptr::new(this.addr() + 0x14);
        let pointer = fn_0044dec0(e, link);
        let slot = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(slot, pointer);
        e.call(EMPTY_HOOK, &[]);
        let pointer = fn_0044dec0(e, link);
        e.call(LOCAL_CONSTRUCT, &args![local, 0u32, pointer]);
        let entry = e.mem.u32(this.addr() + 0x10);
        let entry_key = e.call(OBJECT_KEY, &args![entry]).u32();
        let local_key = e.call(OBJECT_KEY, &args![local]).u32();
        if entry_key != local_key {
            continue;
        }
        if fn_0044dec0(e, link) == 0 {
            found = false;
            break;
        }
        let pointer = fn_0044df00(e, link);
        let word = e.mem.u32(pointer);
        let owner = e.mem.u32(this.addr());
        e.vcall(owner, 0x30, &args![word, argument]);
        let pointer = fn_0044df00(e, link);
        let value = e.mem.u32(pointer + 4);
        e.mem.set_u32(out.addr(), value);
        found = true;
        e.call(EMPTY_HOOK, &[]);
        let pointer = fn_0044df00(e, link);
        if !fn_0044df20(e, Ptr::new(pointer)) {
            break;
        }
    });
    e.call(LOOP_RELEASE, &args![this]);
    found
}

/// One side of `0044eb80`: the flagged pointer at `this + offset`.
/// `None` continues the outer loop, `Some(result)` ends it.
fn fn_0044eb80_side(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    argument: Ptr,
    out: Ptr,
) -> Option<bool> {
    let link = Ptr::new(this.addr() + offset);
    if fn_0044dec0(e, link) == 0 {
        return Some(false);
    }
    let pointer = fn_0044df00(e, link);
    if fn_0044df20(e, Ptr::new(pointer)) {
        return None;
    }
    let pointer = fn_0044df00(e, link);
    let word = e.mem.u32(pointer);
    let owner = e.mem.u32(this.addr());
    e.vcall(owner, 0x30, &args![word, argument]);
    let pointer = fn_0044df00(e, link);
    let value = e.mem.u32(pointer + 4);
    e.mem.set_u32(out.addr(), value);
    e.call(EMPTY_HOOK, &[]);
    let pointer = fn_0044df00(e, link);
    if fn_0044df20(e, Ptr::new(pointer)) {
        e.mem.set_u32(out.addr(), 0);
        return None;
    }
    Some(true)
}

// Translated from 0044eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repeats `006662a0(this, key, *argument)` and, by its answer, takes the
/// flagged pointer at `+0x18` (true) or `+0x14` (false): a null pointer
/// ends with false; a flagged one (`0044df20`) starts over; otherwise its
/// first word and `argument` go to vtable slot `+0x30` of the object at
/// `+0`, its `+4` word to `*out`, and the answer is true unless the flag
/// got set meanwhile (then `*out` is cleared and it starts over). Ends with
/// `00529000` on `this`.
pub fn fn_0044eb80(e: &mut Engine, this: Ptr, key: u32, argument: Ptr, out: Ptr) -> bool {
    let result = loop {
        let wanted = e.mem.u32(argument.addr());
        let first_side = e.call(0x0066_62a0, &args![this, key, wanted]).bool();
        let offset = if first_side { 0x18 } else { 0x14 };
        if let Some(result) = fn_0044eb80_side(e, this, offset, argument, out) {
            break result;
        }
    };
    e.call(LOOP_RELEASE, &args![this]);
    result
}

// Translated from 0044ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (class unconfirmed): runs `0044dd40`
/// on `this`, then frees it (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0044ed50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_dd40, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0044ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (class unconfirmed): runs `0044f200`
/// on `this`, then frees it (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0044ed80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_f200, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// `0092c870` (`ECX` = the pointed-to object): takes a reference.
const ADD_REFERENCE: u32 = 0x0092_c870;
/// `00529030` (`ECX` = `this`, two stack words: the key and a word), `AL`.
const LOOP_BEGIN: u32 = 0x0052_9030;
/// `004491c0` (`cdecl`, three words), `AL`.
const UNLINK_TEST_004491C0: u32 = 0x0044_91c0;
/// `00666510` (`ECX` = `this`, one stack word).
const UNLINK_00666510: u32 = 0x0066_6510;

// Translated from 0044e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of a reference-counted pointer word (unnamed in the engine map; `ECX`
/// and one stack word, `RET 4`): copies the word at `source` into `this` and, when it is not
/// null, takes a reference on the object (`0092c870`). Answers `this`.
pub fn fn_0044e370(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let object = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), object);
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    this
}

// Translated from 0044e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Traits_helper::move_s` with the secure-traits tag (the decompiler's library match;
/// `cdecl`, five words, the tag not read): forwards to [`fn_0044e480`] and answers its
/// result.
pub fn fn_0044e460(
    e: &mut Engine,
    destination: u32,
    destination_size: u32,
    source: u32,
    count: u32,
    _unused_4: u32,
) -> u32 {
    fn_0044e480(e, destination, destination_size, source, count)
}

// Translated from 0044e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `char_traits::_Move_s` (the decompiler's library match; `cdecl`, four words):
/// `memmove_s(dest, destsize, src, count)`, answers `dest`.
pub fn fn_0044e480(
    e: &mut Engine,
    destination: u32,
    destination_size: u32,
    source: u32,
    count: u32,
) -> u32 {
    e.call(
        MEMMOVE_S,
        &args![destination, destination_size, source, count],
    );
    destination
}

// Translated from 0044e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A lookup of the lock-free map family of [`fn_0044e550`] (unnamed in the engine map;
/// `ECX` and three stack words, `RET 0xc`; called by `0044b8d0`): after the empty hook,
/// repeats `00529030(this, key, *argument)`; its answer picks the link to use, `+0x18`
/// when true and `+0x14` when false. A null link answers false. While the linked node's
/// flag (`0044df20`) is set, it starts over; otherwise it passes the node's first word and
/// `argument` to vtable slot +0x30 of the object at `+0`, assigns the node's `+4` word into
/// `holder` (`0092c820`) and answers true, unless the node's flag is set by then, in which
/// case it clears `holder` (`006f74f0` with 0) and starts over. Ends with `00529000`.
pub fn fn_0044e680(e: &mut Engine, this: Ptr, key: u32, argument: Ptr, holder: Ptr) -> bool {
    e.call(EMPTY_HOOK, &[]);
    let found = loop {
        let word = e.mem.u32(argument.addr());
        let began = e.call(LOOP_BEGIN, &args![this, key, word]).u8() != 0;
        let link = Ptr::new(this.addr() + if began { 0x18 } else { 0x14 });
        if fn_0044dec0(e, link) == 0 {
            break false;
        }
        let node = fn_0044df00(e, link);
        if fn_0044df20(e, Ptr::new(node)) {
            continue;
        }
        let node = fn_0044df00(e, link);
        let first = e.mem.u32(node);
        let owner = e.mem.u32(this.addr());
        e.vcall(owner, 0x30, &args![first, argument]);
        let node = fn_0044df00(e, link);
        e.call(HOLDER_ASSIGN, &args![holder, node.wrapping_add(4)]);
        let node = fn_0044df00(e, link);
        if fn_0044df20(e, Ptr::new(node)) {
            e.call(POINTER_ASSIGN, &args![holder, 0u32]);
            continue;
        }
        break true;
    };
    e.call(LOOP_RELEASE, &args![this]);
    found
}

/// A local copy of a node pointer (`006ecba0(0, pointer)` on the local at `local`).
fn copy_local(e: &mut Engine, local: u32, pointer: u32) {
    e.call(LOCAL_CONSTRUCT, &args![local, 0u32, pointer]);
}

// Translated from 0044e820 (decompiled, FalloutNV.exe 1.4.0.525)
/// A search of the lock-free map family of [`fn_0044e550`] (unnamed in the engine map;
/// `ECX` and two stack words, `RET 8`; called by `0044d3f0`): finds the bucket for `key`
/// once (`0044ee20` on the object at `+0`), then walks its chain with hazard words: the
/// current link at `+0x10`, the node at `+0x14` (published at `*this[+8]`), the next at
/// `+0x18` (published at `*this[+4]`), each publication followed by the empty hook. The walk
/// restarts from the bucket whenever a key (`00559450`) no longer matches that of a local
/// copy (`006ecba0`) of the pointer. A null node answers false. An unflagged next node
/// (`0044dff0`): the node's `+4` word equal to `*value` answers true, otherwise the link
/// moves to the node's `+8` (the node published at `*this[+0xc]`). A flagged next node is
/// unlinked (`00666510` with the node) when `004491c0(link, key of the next copy, key of
/// the node copy)` answers true, else the walk restarts. Then the next node becomes the
/// current one, published at `*this[+8]`.
pub fn fn_0044e820(e: &mut Engine, this: Ptr, key: u32, value: Ptr) -> bool {
    let base = this.addr();
    let owner = e.mem.u32(base);
    let bucket = e.call(LOOKUP, &args![owner, key]).u32();
    let node_link = Ptr::new(base + 0x14);
    let next_link = Ptr::new(base + 0x18);
    // The frame's three local copies (`EBP-0xc`, `EBP-0x10`, `EBP-0x18`).
    e.with_stack(12, |e, frame| {
        let (start_copy, node_copy, next_copy) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        loop {
            e.mem.set_u32(base + 0x10, bucket);
            let first = e.mem.u32(e.mem.u32(base + 0x10));
            e.mem.set_u32(base + 0x14, first);
            let node = fn_0044dec0(e, node_link);
            let hazard = e.mem.u32(base + 8);
            e.mem.set_u32(hazard, node);
            e.call(EMPTY_HOOK, &[]);
            let node = fn_0044dec0(e, node_link);
            copy_local(e, start_copy, node);
            let link = e.mem.u32(base + 0x10);
            let link_key = e.call(OBJECT_KEY, &args![link]).u32();
            if link_key != e.call(OBJECT_KEY, &args![start_copy]).u32() {
                continue;
            }
            loop {
                if fn_0044dec0(e, node_link) == 0 {
                    return false;
                }
                let node = fn_0044df00(e, node_link);
                let next = e.mem.u32(node.wrapping_add(8));
                e.mem.set_u32(base + 0x18, next);
                let next = fn_0044dec0(e, next_link);
                let hazard = e.mem.u32(base + 4);
                e.mem.set_u32(hazard, next);
                e.call(EMPTY_HOOK, &[]);
                let next_key = e.call(OBJECT_KEY, &args![next_link]).u32();
                let node = fn_0044df00(e, node_link);
                if next_key != e.call(OBJECT_KEY, &args![node.wrapping_add(8)]).u32() {
                    break;
                }
                let node = fn_0044dec0(e, node_link);
                let node_value = e.mem.u32(node.wrapping_add(4));
                let node = fn_0044dec0(e, node_link);
                copy_local(e, node_copy, node);
                let link = e.mem.u32(base + 0x10);
                let link_key = e.call(OBJECT_KEY, &args![link]).u32();
                if link_key != e.call(OBJECT_KEY, &args![node_copy]).u32() {
                    break;
                }
                if !fn_0044dff0(e, next_link) {
                    if node_value == e.mem.u32(value.addr()) {
                        return true;
                    }
                    let node = fn_0044df00(e, node_link);
                    e.mem.set_u32(base + 0x10, node.wrapping_add(8));
                    let node = fn_0044dec0(e, node_link);
                    let hazard = e.mem.u32(base + 0xc);
                    e.mem.set_u32(hazard, node);
                    e.call(EMPTY_HOOK, &[]);
                } else {
                    let next = fn_0044dec0(e, next_link);
                    copy_local(e, next_copy, next);
                    let node_copy_key = e.call(OBJECT_KEY, &args![node_copy]).u32();
                    let next_copy_key = e.call(OBJECT_KEY, &args![next_copy]).u32();
                    let link = e.mem.u32(base + 0x10);
                    let unlinked = e
                        .call(
                            UNLINK_TEST_004491C0,
                            &args![link, next_copy_key, node_copy_key],
                        )
                        .u8();
                    if unlinked == 0 {
                        break;
                    }
                    let node = fn_0044dec0(e, node_link);
                    e.call(UNLINK_00666510, &args![this, node]);
                }
                let next = e.mem.u32(base + 0x18);
                e.mem.set_u32(base + 0x14, next);
                let next = fn_0044dec0(e, next_link);
                let hazard = e.mem.u32(base + 8);
                e.mem.set_u32(hazard, next);
                e.call(EMPTY_HOOK, &[]);
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_e370, fn_0044e370(Ptr, Ptr) -> Ptr),
        entry!(0x0044_e460, fn_0044e460(u32, u32, u32, u32, u32) -> u32),
        entry!(0x0044_e480, fn_0044e480(u32, u32, u32, u32) -> u32),
        entry!(0x0044_e680, fn_0044e680(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x0044_e820, fn_0044e820(Ptr, u32, Ptr) -> bool),
        entry!(0x0044dec0, fn_0044dec0(Ptr) -> u32),
        entry!(0x0044df00, fn_0044df00(Ptr) -> u32),
        entry!(0x0044df40, fn_0044df40(Ptr, u32, u32) -> Ptr),
        entry!(0x0044dfc0, fn_0044dfc0(Ptr) -> Ptr),
        entry!(0x0044dff0, fn_0044dff0(Ptr) -> bool),
        entry!(0x0044e010, fn_0044e010(Ptr, bool)),
        entry!(0x0044e050, fn_0044e050(Ptr, Ptr) -> bool),
        entry!(0x0044e0d0, fn_0044e0d0(Ptr) -> u64),
        entry!(0x0044e0f0, fn_0044e0f0(Ptr, u32, u32) -> u32),
        entry!(0x0044e170, fn_0044e170(Ptr, bool)),
        entry!(0x0044e330, fn_0044e330() -> u32),
        entry!(0x0044e340, fn_0044e340(u32, u32) -> u32),
        entry!(0x0044e360, fn_0044e360(u32) -> u32),
        entry!(0x0044e3a0, fn_0044e3a0(Ptr, Ptr) -> u32),
        entry!(
            0x0044e420,
            fn_0044e420(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0044e440, fn_0044e440(u32, u32, u32, u32) -> u32),
        entry!(0x0044e4a0, fn_0044e4a0(u32, u32, u32) -> u32),
        entry!(0x0044e4f0, fn_0044e4f0(Ptr, Ptr)),
        entry!(0x0044e550, fn_0044e550(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0044ea60, fn_0044ea60(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0044eb80, fn_0044eb80(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x0044ed50, fn_0044ed50(Ptr, u32) -> Ptr),
        entry!(0x0044ed80, fn_0044ed80(Ptr, u32) -> Ptr),
        entry!(
            0x0044e120,
            fn_0044e120(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0044_df20, fn_0044df20(Ptr) -> bool),
        entry!(0x0044_ed20, fn_0044ed20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const WRITER: u32 = 0x0f00_0000;

    #[test]
    fn calls_the_function_at_plus_0xc_and_advances_plus_4() {
        let mut e = Engine::new();
        e.register(WRITER, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, 0x100);
        e.mem.set_u32(stream + 0xc, WRITER);
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(stream);
        let back = e.call(0x0044_e120, &args![this, 0xaau32, 0x30u32, 0xccu32, 1u32]);
        assert_eq!(back.u32(), 0x30);
        assert_eq!(e.mem.u32(stream + 4), 0x130);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == WRITER).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![stream, 0xaa, 0x30, 0xcc, 1]);
        e.mem.set_u32(stream + 4, 0xffff_fff0);
        e.call(0x0044_e120, &args![this, 0u32, 0x20u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(stream + 4), 0x10);
    }

    #[test]
    fn l68d_0044df20_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DFF0, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_df20, &args![0x4100u32]).bool());
        assert!(e.call(0x0044_df20, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DFF0).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
    #[test]
    fn at_0044ed20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_ee40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_ed20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_ed20)
                .collect();
            let mut expected = vec![(0x0044_ee40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0044dec0_and_0044df00_clear_the_flag_bit() {
        let mut e = Engine::new();
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, 0x1235);
        assert_eq!(e.call(0x0044_dec0, &args![p]).u32(), 0x1234);
        assert_eq!(e.call(0x0044_df00, &args![p]).u32(), 0x1234);
        e.mem.set_u32(p, 0x1234);
        assert_eq!(e.call(0x0044_dec0, &args![p]).u32(), 0x1234);
    }

    #[test]
    fn at_0044df40_initializes_and_assigns() {
        let mut e = Engine::new();
        e.register(HOLDER_INIT, no_op);
        e.register(0x0044_dee0, no_op);
        e.register(HOLDER_ASSIGN, no_op);
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_df40, &args![this, 0x11u32, 0x22u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), 0x11);
        assert_eq!(calls_to(&mut e, HOLDER_INIT), vec![vec![this + 4, 0]]);
        assert_eq!(calls_to(&mut e, 0x0044_dee0), vec![vec![this + 8]]);
        assert_eq!(calls_to(&mut e, HOLDER_ASSIGN), vec![vec![this + 4, 0x22]]);
    }

    #[test]
    fn at_0044dfc0_zeroes_and_initializes() {
        let mut e = Engine::new();
        e.register(HOLDER_INIT, no_op);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_dfc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(calls_to(&mut e, HOLDER_INIT), vec![vec![this + 4, 0]]);
    }

    #[test]
    fn at_0044dff0_and_0044e010_read_and_write_bit_zero() {
        let mut e = Engine::new();
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, 0x100);
        assert!(!e.call(0x0044_dff0, &args![p]).bool());
        e.call(0x0044_e010, &args![p, true]);
        assert_eq!(e.mem.u32(p), 0x101);
        assert!(e.call(0x0044_dff0, &args![p]).bool());
        e.call(0x0044_e010, &args![p, false]);
        assert_eq!(e.mem.u32(p), 0x100);
    }

    #[test]
    fn at_0044e050_counts_when_the_slot_applies() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x24, 0x0f10_0024);
        e.mem.set_u32(vtable + 0x14, 0x0f10_0014);
        e.register(0x0f10_0024, |_, _| 3u32.into_ret());
        e.register(0x0040_19a0, no_op);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x54, 0x1000);
        let item = e.mem.alloc(0x18);
        e.mem.set_u32(item + 0x10, 0xaaaa);
        e.mem.set_u32(item + 0x14, 0xbbbb);
        e.register(0x0f10_0014, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_e050, &args![this, item]).bool());
        assert_eq!(
            calls_to(&mut e, 0x0f10_0024),
            vec![vec![this, 0xaaaa, 0xbbbb]]
        );
        assert_eq!(
            calls_to(&mut e, 0x0040_19a0),
            vec![vec![0x1000 + 12], vec![this + 0x58]]
        );
        e.register(0x0f10_0014, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_e050, &args![this, item]).bool());
        assert!(calls_to(&mut e, 0x0040_19a0).is_empty());
    }

    #[test]
    fn at_0044e0d0_returns_the_64_bit_value() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x18);
        e.mem.set_u32(p + 0x10, 0x1111_2222);
        e.mem.set_u32(p + 0x14, 0x3333_4444);
        let r = e.call(0x0044_e0d0, &args![p]);
        assert_eq!(r.u64(), 0x3333_4444_1111_2222);
    }

    #[test]
    fn at_0044e0f0_writes_count_times_four_bytes() {
        let mut e = Engine::new();
        e.register_double(WRITER, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0xc, WRITER);
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_e0f0, &args![stream, 0x500u32, 3u32]).u32();
        assert_eq!(r, 12);
        let writes = calls_to(&mut e, WRITER);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][..3], [stream, 0x500, 12]);
        assert_eq!(writes[0][4], 1);
        assert_eq!(e.mem.u32(stream + 4), 12);
    }

    #[test]
    fn at_0044e170_clears_the_list_and_rebuilds_unless_kept_empty() {
        for keep_empty in [true, false] {
            let mut e = Engine::new();
            for addr in [
                0x0040_fbf0,
                0x0040_4eb0,
                0x0040_fba0,
                0x0040_4ee0,
                POINTER_ASSIGN,
                0x0044_ee40,
                OPERATOR_DELETE,
                0x0044_dd40,
                HOLDER_INIT,
            ] {
                e.register(addr, no_op);
            }
            e.register(OBJECT_KEY, |_, _| 0x77u32.into_ret());
            e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(0x006c_73b0, |_, a| a[0].into_ret());
            let this = e.mem.alloc(0x24);
            let object = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0x14, object);
            e.mem.set_u32(this + 0x18, 9);
            let second = e.mem.alloc(8);
            let first = e.mem.alloc(8);
            e.mem.set_u32(first, second);
            e.mem.set_u32(this + 4, first);
            e.call_log = Some(vec![]);
            e.call(0x0044_e170, &args![this, keep_empty]);
            assert_eq!(calls_to(&mut e, 0x0040_fbf0), vec![vec![this + 0x20, 0]]);
            let scope = calls_to(&mut e, 0x0040_4eb0);
            assert_eq!(scope[0][1..], [6, 1, 0x0101_73f8, 0x115]);
            assert_eq!(calls_to(&mut e, 0x0040_4ee0), vec![vec![scope[0][0]]]);
            assert_eq!(calls_to(&mut e, 0x0044_ee40), vec![vec![object]]);
            assert_eq!(
                calls_to(&mut e, POINTER_ASSIGN),
                vec![vec![first + 4, 0], vec![second + 4, 0]]
            );
            assert_eq!(
                calls_to(&mut e, 0x0044_dd40),
                vec![vec![first], vec![second]]
            );
            assert_eq!(calls_to(&mut e, 0x0040_fba0), vec![vec![this + 0x20]]);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            if keep_empty {
                assert_eq!(e.mem.u32(this + 4), 0);
                assert_eq!(e.mem.u32(this + 0x14), 0);
                assert!(calls_to(&mut e, OPERATOR_NEW).is_empty());
            } else {
                let head = e.mem.u32(this + 4);
                assert_ne!(head, 0);
                assert_eq!(e.mem.u32(this + 8), head);
                let made = calls_to(&mut e, 0x006c_73b0);
                assert_eq!(made.len(), 1);
                assert_eq!(made[0][1], 0x77);
                assert_eq!(e.mem.u32(this + 0x14), made[0][0]);
            }
        }
    }

    #[test]
    fn at_0044e330_e340_e360_call_the_tls_imports() {
        let mut e = Engine::new();
        e.register(TLS_ALLOC, |_, _| 5u32.into_ret());
        e.register(TLS_SET_VALUE, |_, _| 1u32.into_ret());
        e.register(TLS_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_e330, &[]).u32(), 5);
        assert_eq!(e.call(0x0044_e340, &args![5u32, 0x99u32]).u32(), 1);
        assert_eq!(e.call(0x0044_e360, &args![5u32]).u32(), 1);
        assert_eq!(calls_to(&mut e, TLS_SET_VALUE), vec![vec![5, 0x99]]);
        assert_eq!(calls_to(&mut e, TLS_FREE), vec![vec![5]]);
    }

    #[test]
    fn at_0044e3a0_makes_a_slot_object_below_the_limit() {
        let mut e = Engine::new();
        e.register(0x0040_b460, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(0x0040_fc90, |_, _| 0x4321u32.into_ret());
        e.register(TLS_SET_VALUE, no_op);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0f10_0004);
        e.register(0x0f10_0004, |_, a| (0x8000 + a[1]).into_ret());
        let factory = e.mem.alloc(4);
        e.mem.set_u32(factory, vtable);
        let entries = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 7);
        e.mem.set_u32(this + 8, entries);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_e3a0, &args![this, factory]).u32(), 0x8000);
        assert_eq!(e.call(0x0044_e3a0, &args![this, factory]).u32(), 0x8001);
        assert_eq!(e.mem.u32(entries + 8), 0x4321);
        assert_eq!(e.mem.u32(entries + 12), 0x8001);
        assert_eq!(
            calls_to(&mut e, TLS_SET_VALUE),
            vec![vec![7, 0x8000], vec![7, 0x8001]]
        );
        // Index 2 is not below the limit 2: nothing is made.
        assert_eq!(e.call(0x0044_e3a0, &args![this, factory]).u32(), 0);
        assert_eq!(calls_to(&mut e, 0x0f10_0004).len(), 2);
    }

    #[test]
    fn at_0044e420_and_0044e440_copy_with_memcpy_s() {
        let mut e = Engine::new();
        e.register(MEMCPY_S, |_, _| 0xffu32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_e420, &args![0x10u32, 0x20u32, 0x30u32, 4u32, 1u32]);
        assert_eq!(r.u32(), 0x10);
        assert_eq!(e.call(0x0044_e440, &args![1u32, 2u32, 3u32, 4u32]).u32(), 1);
        assert_eq!(
            calls_to(&mut e, MEMCPY_S),
            vec![vec![0x10, 0x20, 0x30, 4], vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn at_0044e4a0_moves_whole_elements() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, no_op);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_e4a0, &args![0x100u32, 0x10cu32, 0x200u32])
                .u32(),
            0x20c
        );
        assert_eq!(
            calls_to(&mut e, MEMMOVE_S),
            vec![vec![0x200, 12, 0x100, 12]]
        );
        e.call_log = Some(vec![]);
        // An empty and a reversed range move nothing.
        assert_eq!(
            e.call(0x0044_e4a0, &args![0x100u32, 0x100u32, 0x200u32])
                .u32(),
            0x200
        );
        assert_eq!(
            e.call(0x0044_e4a0, &args![0x108u32, 0x100u32, 0x200u32])
                .u32(),
            0x1f8
        );
        assert!(calls_to(&mut e, MEMMOVE_S).is_empty());
    }

    #[test]
    fn at_0044e4f0_pushes_and_trims_at_the_limit() {
        let mut e = Engine::new();
        e.register(POINTER_ASSIGN, no_op);
        e.register(0x006e_cd40, no_op);
        e.register(0x0084_e3a0, |_, _| 2u32.into_ret());
        e.register(0x0044_eed0, no_op);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, 0x4444);
        e.mem.set_u32(this + 0x10, 0x9990);
        let node = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0044_e4f0, &args![this, node]);
        assert_eq!(e.mem.u32(this + 0x10), node);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(calls_to(&mut e, POINTER_ASSIGN), vec![vec![node + 4, 0]]);
        assert_eq!(calls_to(&mut e, 0x006e_cd40), vec![vec![node, 0x9990]]);
        assert_eq!(calls_to(&mut e, 0x0084_e3a0), vec![vec![0x4444]]);
        assert!(calls_to(&mut e, 0x0044_eed0).is_empty());
        e.call(0x0044_e4f0, &args![this, node]);
        assert_eq!(calls_to(&mut e, 0x0044_eed0), vec![vec![this]]);
    }

    /// An object at `+0` with vtable slot `+0x30`, `+8` pointing at a slot,
    /// and entries the lookup double hands out in turn.
    fn lookup_engine(entries: Vec<u32>) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(EMPTY_HOOK, no_op);
        e.register(LOCAL_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        e.register(LOCAL_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(OBJECT_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LOOP_RELEASE, no_op);
        e.register(HOLDER_ASSIGN, no_op);
        e.register(0x0f10_0030, no_op);
        let mut queue = entries.into_iter();
        e.register_double(LOOKUP, move |_, _| queue.next().unwrap().into_ret());
        let vtable = e.mem.alloc(0x34);
        e.mem.set_u32(vtable + 0x30, 0x0f10_0030);
        let owner = e.mem.alloc(4);
        e.mem.set_u32(owner, vtable);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, owner);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(this + 8, slot);
        (e, this, owner)
    }

    /// A pointed-to record: first word, the `+4` word, the flag word at `+8`.
    fn record(e: &mut Engine, first: u32, value: u32) -> u32 {
        let p = e.mem.alloc(0xc);
        e.mem.set_u32(p, first);
        e.mem.set_u32(p + 4, value);
        p
    }

    fn entry(e: &mut Engine, pointer: u32) -> u32 {
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, pointer);
        p
    }

    #[test]
    fn at_0044e550_looks_up_until_stable_and_assigns() {
        // Null pointer: false.
        let (mut e, this, _) = lookup_engine(vec![]);
        let null_entry = entry(&mut e, 0);
        e.register_double(LOOKUP, move |_, _| null_entry.into_ret());
        let holder = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_e550, &args![this, 1u32, 2u32, holder]).bool());
        assert_eq!(calls_to(&mut e, LOOP_RELEASE), vec![vec![this]]);

        // A flagged entry first (unstable), then a stable one; the pointer's
        // flag word asks for one more pass.
        let (mut e, this, owner) = lookup_engine(vec![]);
        let target = record(&mut e, 0x5150, 0x6160);
        let unstable = entry(&mut e, target | 1);
        let stable = entry(&mut e, target);
        let mut queue = vec![unstable, stable, stable].into_iter();
        e.register_double(LOOKUP, move |_, _| queue.next().unwrap().into_ret());
        let mut flags = vec![true, false].into_iter();
        e.register_double(0x0044_dff0, move |_, _| flags.next().unwrap().into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_e550, &args![this, 0x42u32, 0x43u32, holder])
            .bool());
        assert_eq!(calls_to(&mut e, LOOKUP).len(), 3);
        assert_eq!(calls_to(&mut e, LOOKUP)[0], vec![owner, 0x42]);
        assert_eq!(
            calls_to(&mut e, 0x0f10_0030),
            vec![vec![owner, 0x5150, 0x43], vec![owner, 0x5150, 0x43]]
        );
        assert_eq!(
            calls_to(&mut e, HOLDER_ASSIGN),
            vec![vec![holder, target + 4], vec![holder, target + 4]]
        );
        let slot = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(slot), target);
        assert_eq!(e.mem.u32(this + 0x10), stable);
        assert_eq!(calls_to(&mut e, EMPTY_HOOK).len(), 1);
    }

    #[test]
    fn at_0044ea60_copies_the_value_out() {
        let (mut e, this, owner) = lookup_engine(vec![]);
        let target = record(&mut e, 0x5150, 0x6160);
        let unstable = entry(&mut e, target | 1);
        let stable = entry(&mut e, target);
        let mut queue = vec![unstable, stable].into_iter();
        e.register_double(LOOKUP, move |_, _| queue.next().unwrap().into_ret());
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_ea60, &args![this, 0x42u32, 0x43u32, out])
            .bool());
        assert_eq!(e.mem.u32(out), 0x6160);
        assert_eq!(
            calls_to(&mut e, 0x0f10_0030),
            vec![vec![owner, 0x5150, 0x43]]
        );
        assert_eq!(calls_to(&mut e, LOCAL_CONSTRUCT).len(), 2);
        assert_eq!(calls_to(&mut e, EMPTY_HOOK).len(), 3);
        assert_eq!(calls_to(&mut e, LOOP_RELEASE), vec![vec![this]]);

        // Null pointer: false.
        let (mut e, this, _) = lookup_engine(vec![]);
        let null_entry = entry(&mut e, 0);
        e.register_double(LOOKUP, move |_, _| null_entry.into_ret());
        assert!(!e.call(0x0044_ea60, &args![this, 0u32, 0u32, out]).bool());
    }

    #[test]
    fn at_0044eb80_takes_the_side_the_check_picks() {
        for (side, check) in [(0x18u32, true), (0x14, false)] {
            let (mut e, this, owner) = lookup_engine(vec![]);
            if check {
                e.register(0x0066_62a0, |_, _| true.into_ret());
            } else {
                e.register(0x0066_62a0, |_, _| false.into_ret());
            }
            let target = record(&mut e, 0x5150, 0x6160);
            e.mem.set_u32(this + side, target);
            let argument = e.mem.alloc(4);
            e.mem.set_u32(argument, 0x99);
            let out = e.mem.alloc(4);
            // Flag set at the first check (start over), then clear twice.
            let mut flags = vec![true, false, false].into_iter();
            e.register_double(0x0044_dff0, move |_, _| flags.next().unwrap().into_ret());
            e.call_log = Some(vec![]);
            assert!(e
                .call(0x0044_eb80, &args![this, 7u32, argument, out])
                .bool());
            assert_eq!(
                calls_to(&mut e, 0x0066_62a0),
                vec![vec![this, 7, 0x99], vec![this, 7, 0x99]]
            );
            assert_eq!(
                calls_to(&mut e, 0x0f10_0030),
                vec![vec![owner, 0x5150, argument]]
            );
            assert_eq!(e.mem.u32(out), 0x6160);
            assert_eq!(calls_to(&mut e, LOOP_RELEASE), vec![vec![this]]);
        }
        // The flag set after the call: out cleared, start over; then a null
        // pointer ends with false.
        let (mut e, this, _) = lookup_engine(vec![]);
        let target = record(&mut e, 0x5150, 0x6160);
        e.mem.set_u32(this + 0x14, target);
        let mut picks = vec![false, true].into_iter();
        e.register_double(0x0066_62a0, move |_, _| picks.next().unwrap().into_ret());
        let mut flags = vec![false, true].into_iter();
        e.register_double(0x0044_dff0, move |_, _| flags.next().unwrap().into_ret());
        let argument = e.mem.alloc(4);
        let out = e.mem.alloc(4);
        assert!(!e
            .call(0x0044_eb80, &args![this, 7u32, argument, out])
            .bool());
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn at_0044ed50_and_0044ed80_destroy_then_free_on_bit_zero() {
        for (addr, body) in [(0x0044_ed50u32, 0x0044_dd40u32), (0x0044_ed80, 0x0044_f200)] {
            let mut e = Engine::new();
            e.register(body, no_op);
            e.register(OPERATOR_DELETE, no_op);
            let this = e.mem.alloc(8);
            for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
                assert_eq!(calls_to(&mut e, body), vec![vec![this]]);
                assert_eq!(calls_to(&mut e, OPERATOR_DELETE).len(), frees as usize);
            }
        }
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0044e370_copies_the_pointer_and_takes_a_reference() {
        let mut e = Engine::new();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        let words = e.mem.alloc(8);
        e.mem.set_u32(words + 4, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_e370, &args![words, words + 4]).u32(), words);
        assert_eq!(e.mem.u32(words), 0x1234);
        e.mem.set_u32(words + 4, 0);
        e.call(0x0044_e370, &args![words, words + 4]);
        assert_eq!(e.mem.u32(words), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x1234]]);
    }

    #[test]
    fn fn_0044e460_and_0044e480_move_through_memmove_s() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| 0x22u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_e460, &args![0x10u32, 0x20u32, 0x30u32, 4u32, 9u32])
                .u32(),
            0x10
        );
        assert_eq!(
            e.call(0x0044_e480, &args![0x11u32, 0x21u32, 0x31u32, 5u32])
                .u32(),
            0x11
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMMOVE_S),
            vec![vec![0x10, 0x20, 0x30, 4], vec![0x11, 0x21, 0x31, 5]]
        );
    }

    /// The map object for 0044e680: owner with a vtable whose slot +0x30 is a double.
    fn map_engine(began: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(EMPTY_HOOK, |_, _| Ret::default());
        e.register(LOOP_RELEASE, |_, _| Ret::default());
        e.register(HOLDER_ASSIGN, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        e.register(0x0060_0030, |_, _| Ret::default());
        e.register(
            LOOP_BEGIN,
            if began {
                |_, _| 1u32.into_ret()
            } else {
                |_, _| 0x100u32.into_ret()
            },
        );
        let mut slots = vec![0u32; 13];
        slots[12] = 0x0060_0030;
        e.put_vtable(0x0070_0000, &slots);
        let owner = e.mem.alloc(4);
        e.mem.set_u32(owner, 0x0070_0000);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, owner);
        e.call_log = Some(vec![]);
        (e, this, owner)
    }

    #[test]
    fn fn_0044e680_reports_the_node_of_the_chosen_link() {
        for began in [true, false] {
            let (mut e, this, owner) = map_engine(began);
            let node = e.mem.alloc(0x10);
            e.mem.set_u32(node, 0x5151);
            e.mem
                .set_u32(this + if began { 0x18 } else { 0x14 }, node | 1);
            let argument = e.mem.alloc(4);
            e.mem.set_u32(argument, 0x77);
            let found = e
                .call(0x0044_e680, &args![this, 0x99u32, argument, 0x4000u32])
                .bool();
            assert!(found);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, LOOP_BEGIN), vec![vec![this, 0x99, 0x77]]);
            assert_eq!(
                calls(&log, 0x0060_0030),
                vec![vec![owner, 0x5151, argument]]
            );
            assert_eq!(calls(&log, HOLDER_ASSIGN), vec![vec![0x4000, node + 4]]);
            assert_eq!(calls(&log, LOOP_RELEASE), vec![vec![this]]);
        }
    }

    #[test]
    fn fn_0044e680_answers_false_for_a_null_link() {
        let (mut e, this, _) = map_engine(true);
        let argument = e.mem.alloc(4);
        let found = e
            .call(0x0044_e680, &args![this, 0x99u32, argument, 0x4000u32])
            .bool();
        assert!(!found);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, HOLDER_ASSIGN).is_empty());
        assert_eq!(calls(&log, LOOP_RELEASE).len(), 1);
    }

    /// The chain for 0044e820: `this` with its hazard words, the bucket holding `first`.
    fn chain_engine(first: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(EMPTY_HOOK, |_, _| Ret::default());
        e.register(OBJECT_KEY, |_, _| 0u32.into_ret());
        e.register(LOCAL_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        e.register(UNLINK_TEST_004491C0, |_, _| 1u32.into_ret());
        e.register(UNLINK_00666510, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let hazards = e.mem.alloc(0x10);
        for (i, offset) in [4u32, 8, 0xc].iter().enumerate() {
            e.mem.set_u32(this + offset, hazards + 4 * i as u32);
        }
        let bucket = e.mem.alloc(4);
        e.mem.set_u32(bucket, first);
        e.register_double(LOOKUP, move |_, _| bucket.into_ret());
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn node(e: &mut Engine, value: u32, next: u32) -> u32 {
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node + 4, value);
        e.mem.set_u32(node + 8, next);
        node
    }

    #[test]
    fn fn_0044e820_finds_a_value_or_reaches_the_end() {
        let (mut e, this) = chain_engine(0);
        let a = node(&mut e, 5, 0);
        // Point the bucket at the node (the bucket word is the chain's first link).
        let bucket = e.call(LOOKUP, &args![0u32, 0u32]).u32();
        e.mem.set_u32(bucket, a);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 5);
        assert!(e.call(0x0044_e820, &args![this, 1u32, value]).bool());
        e.mem.set_u32(value, 6);
        assert!(!e.call(0x0044_e820, &args![this, 1u32, value]).bool());
        // The link moved to the node's +8 before the end was reached.
        assert_eq!(e.mem.u32(this + 0x10), a + 8);
    }

    #[test]
    fn fn_0044e820_unlinks_a_flagged_next_node() {
        let (mut e, this) = chain_engine(0);
        let b = node(&mut e, 6, 0);
        let a = node(&mut e, 5, b | 1);
        let bucket = e.call(LOOKUP, &args![0u32, 0u32]).u32();
        e.mem.set_u32(bucket, a);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 6);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_e820, &args![this, 1u32, value]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, UNLINK_00666510), vec![vec![this, a]]);
        assert_eq!(calls(&log, UNLINK_TEST_004491C0), vec![vec![bucket, 0, 0]]);
    }
}
