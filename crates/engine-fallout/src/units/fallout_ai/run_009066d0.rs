//! `fallout/ai/run_009066d0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00906dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906dc0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructor of the object `0092c950` extends; `ECX` is the object and the answer is the
/// object. It runs the base constructor (`008d0270`), stores the vtable pointer `0x010886e4`,
/// runs the component constructors (`00906f60` on `+0x38`, `0096a2d0` on `+0x5c`, `+0x6c`,
/// `+0x74` and `+0x7c`, `00937000` on `+0x94`), sets the float at `+0x20` to the float at
/// `0x01012054`, runs `008d03a0`, clears the other fields, runs `00906f80` with 0, 0xff and 0
/// and finishes with `0061cc40` with 3. The engine map gives it no name. The
/// exception-unwinding frame is not translated.
pub fn fn_00906dc0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x008d_0270, &args![this]);
    e.mem.set_u32(base, 0x0108_86e4);
    e.call(0x0090_6f60, &args![base + 0x38]);
    e.mem.set_u32(base + 0x40, 0);
    e.call(0x0096_a2d0, &args![base + 0x5c]);
    e.call(0x0096_a2d0, &args![base + 0x6c]);
    e.call(0x0096_a2d0, &args![base + 0x74]);
    e.call(0x0096_a2d0, &args![base + 0x7c]);
    e.call(0x0093_7000, &args![base + 0x94]);
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x20, value);
    e.call(0x008d_03a0, &args![this]);
    e.mem.set_f32(base + 0x1c, 0.0);
    for offset in [0x24, 0x64, 0x68, 0x44, 0x84, 0x88, 0x8c, 0x34, 0x58] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_f32(base + 0xa8, 0.0);
    e.mem.set_u32(base + 0x54, 0);
    e.call(0x0090_6f80, &args![this, 0u32, 0xffu32, 0u32]);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_f32(base + 0xac, 0.0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u8(base + 0xb0, 0);
    e.call(0x0061_cc40, &args![this, 3u32]);
    this
}

// Translated from 00906f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructor of a pair of floats (`ECX` = the
/// pair, both set to 0.0 with `FLDZ`/`FSTP`). Returns `this`.
pub fn fn_00906f60(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_f32(this.addr(), 0.0);
    e.mem.set_f32(this.addr() + 4, 0.0);
    this
}

// Translated from 00906f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; `value`, `mask`, `shift`
/// are bytes): replaces the bits of the byte at `+0x30` selected by `mask`
/// with `value << (shift & 31)` (the shifted value is not masked: bits of it
/// outside `mask` are set in the byte, as the code ORs it in; only the low byte
/// is stored).
pub fn fn_00906f80(e: &mut Engine, this: Ptr, value: u8, mask: u8, shift: u8) {
    let old = u32::from(e.mem.u8(this.addr() + 0x30));
    let new = (old & !u32::from(mask)) | (u32::from(value) << (u32::from(shift) & 0x1f));
    e.mem.set_u8(this.addr() + 0x30, new as u8);
}

/// `0044dec0` (unnamed in the engine map): `thiscall`, no stack words; on an iterator, its
/// current node (0 at the end).
const ITERATOR_NODE: u32 = 0x0044_dec0;
/// `0044df00` (unnamed in the engine map): `thiscall`, no stack words; on an iterator, the
/// address of its current element.
const ITERATOR_ELEMENT: u32 = 0x0044_df00;
/// `0044df20` (unnamed in the engine map): `thiscall`, no stack words; on an element, true
/// (`AL`) when it is to be skipped.
const ELEMENT_SKIPPED: u32 = 0x0044_df20;
/// `0092c820` (unnamed in the engine map): `thiscall`, `RET 4`; stores the found entry (the
/// element's address plus 4) into the result object.
const STORE_RESULT: u32 = 0x0092_c820;

/// One found element of the iterator at `iterator` (`this + 0x14` or `+0x18`) for
/// `00906af0`/`00906c10`: the owner's (`[this]`) vtable slot `0x30` gets the element's word
/// and `second`, the result gets the element (`0092c820(result, element + 4)`), and `0040fbe0`
/// runs. Returns whether the new current element is to be skipped (`0044df20`).
fn take_element(e: &mut Engine, this: Ptr, iterator: u32, second: u32, result: u32) -> bool {
    let element = e.call(ITERATOR_ELEMENT, &args![iterator]).u32();
    let word = e.mem.u32(element);
    let owner = e.mem.u32(this.addr());
    e.vcall(owner, 0x30, &args![word, second]);
    let element = e.call(ITERATOR_ELEMENT, &args![iterator]).u32();
    e.call(STORE_RESULT, &args![result, element.wrapping_add(4)]);
    e.call(0x0040_fbe0, &args![]);
    let element = e.call(ITERATOR_ELEMENT, &args![iterator]).u32();
    e.call(ELEMENT_SKIPPED, &args![element]).bool()
}

// Translated from 00906af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xc`: `key`, `second`, `result`; answer in `AL`). Repeats
/// until it settles: the node for `key` (`0044ee20` on the owner `[this]`) is stored at `+0x10`
/// and its first word at `+0x14` (an iterator), the iterator's node is written through the
/// pointer at `+8`, `0040fbe0` runs, and a temporary (`006ecba0(temp, 0, node)`) is compared
/// with the stored node through `00559450`; a mismatch starts again. With no node the answer is
/// 0; otherwise the element is taken (`take_element`, answer 1) and, when the next one is to be
/// skipped, it starts again. `00529000(this)` runs at the end.
pub fn fn_00906af0(e: &mut Engine, this: Ptr, key: u32, second: u32, result: u32) -> u8 {
    let base = this.addr();
    let iterator = base.wrapping_add(0x14);
    let answer = e.with_stack(8, |e, temp| loop {
        let owner = e.mem.u32(base);
        let node = e.call(0x0044_ee20, &args![owner, key]).u32();
        e.mem.set_u32(base.wrapping_add(0x10), node);
        let first = e.mem.u32(e.mem.u32(base.wrapping_add(0x10)));
        e.mem.set_u32(iterator, first);
        let current = e.call(ITERATOR_NODE, &args![iterator]).u32();
        let slot = e.mem.u32(base.wrapping_add(8));
        e.mem.set_u32(slot, current);
        e.call(0x0040_fbe0, &args![]);
        let current = e.call(ITERATOR_NODE, &args![iterator]).u32();
        e.call(0x006e_cba0, &args![temp, 0u32, current]);
        let stored = e.mem.u32(base.wrapping_add(0x10));
        let stored = e.call(0x0055_9450, &args![stored]).u32();
        let made = e.call(0x0055_9450, &args![temp]).u32();
        if stored != made {
            continue;
        }
        if e.call(ITERATOR_NODE, &args![iterator]).u32() == 0 {
            break 0u8;
        }
        if !take_element(e, this, iterator, second, result) {
            break 1;
        }
    });
    e.call(0x0052_9000, &args![this]);
    answer
}

// Translated from 00906c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xc`: `key`, `second` (a pointer whose word is passed to
/// `00529030`), `result`; answer in `AL`). Repeats until it settles: `00529030(this, key,
/// *second)` picks the iterator (`+0x18` when true, `+0x14` otherwise); an iterator without a
/// node gives 0; a current element to be skipped (`0044df20`) starts again; otherwise the
/// element is taken (`take_element` with `second`, answer 1) and, when the next one is to be
/// skipped, the result is reset (`006f74f0(result, 0)`) and it starts again. `00529000(this)`
/// runs at the end.
pub fn fn_00906c10(e: &mut Engine, this: Ptr, key: u32, second: u32, result: u32) -> u8 {
    let base = this.addr();
    let answer = loop {
        let word = e.mem.u32(second);
        let picked = e.call(0x0052_9030, &args![this, key, word]).u8();
        let iterator = if picked != 0 {
            base.wrapping_add(0x18)
        } else {
            base.wrapping_add(0x14)
        };
        if e.call(ITERATOR_NODE, &args![iterator]).u32() == 0 {
            break 0u8;
        }
        let element = e.call(ITERATOR_ELEMENT, &args![iterator]).u32();
        if e.call(ELEMENT_SKIPPED, &args![element]).bool() {
            continue;
        }
        if take_element(e, this, iterator, second, result) {
            e.call(0x006f_74f0, &args![result, 0u32]);
            continue;
        }
        break 1;
    };
    e.call(0x0052_9000, &args![this]);
    answer
}

/// The lock-free list's calls (`thiscall`): `0044dec0` the node a cursor
/// word holds (no stack words), `0044df00` the node's record (no stack
/// words), `006ecba0` builds a tagged word at `this` from (tag, node),
/// `00559450` reads a tagged word's value, `004491c0` (cdecl, three words)
/// swaps a word when it still holds the expected value, `005292a0` (one word)
/// retires a node, `00529000` ends the operation, `0044ee20` (one word) the
/// bucket of a key, `0044e370` (one word) / `0044cbf0` construct / release an
/// `NiPointer` local, `0044dff0` tests a node's mark, `004b0460` (one word)
/// compares the local with the wanted value. `0040fbe0` (no stack words, an
/// empty body the engine map names `Error`) separates the stores from the
/// reads that check them.
const CURSOR_NODE: u32 = 0x0044_dec0;
const NODE_RECORD: u32 = 0x0044_df00;
const MAKE_TAGGED: u32 = 0x006e_cba0;
const TAGGED_VALUE: u32 = 0x0055_9450;
const COMPARE_SWAP: u32 = 0x0044_91c0;
const RETIRE_NODE: u32 = 0x0052_92a0;
const END_OPERATION: u32 = 0x0052_9000;
const BUCKET_OF: u32 = 0x0044_ee20;
const POINTER_CONSTRUCT: u32 = 0x0044_e370;
const POINTER_RELEASE: u32 = 0x0044_cbf0;
const NODE_MARKED: u32 = 0x0044_dff0;
const POINTER_MATCHES: u32 = 0x004b_0460;
const BARRIER: u32 = 0x0040_fbe0;

/// The lock-free list's working words in the inner object: the cursor
/// record (+0x10), the current (+0x14) and next (+0x18) cursor words, and
/// the three hazard slots the found nodes are published to (+0x4, +0x8,
/// +0xC, each a pointer to a word).
const CURSOR_RECORD: u32 = 0x10;
const CURRENT: u32 = 0x14;
const NEXT: u32 = 0x18;

// Translated from 009066d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unlinks the node of `key` from the inner object's lock-free list: while
/// `fn_00906820` finds it, the node's next word is marked (`004491c0` on its
/// record +8 with tag 1) until that succeeds, then it is unlinked from the
/// previous record (`004491c0` on +0x10), retired on success (`005292a0`) or
/// searched again (`fn_00906820`). Answers whether the key was found;
/// `00529000` ends the operation either way.
pub fn fn_009066d0(e: &mut Engine, this: Ptr, key: u32, value: u32) -> bool {
    let at = this.addr();
    let found = e.with_stack(0xc, |e, frame| {
        // The tagged words at EBP-0x10, -0xC and -0x8.
        let unmarked = frame.addr();
        let marked = frame.addr() + 4;
        let current = frame.addr() + 8;
        loop {
            if !fn_00906820(e, this, key, value) {
                return false;
            }
            let node = e.call(CURSOR_NODE, &args![at + NEXT]).u32();
            e.call(MAKE_TAGGED, &args![unmarked, 0u32, node]);
            let node = e.call(CURSOR_NODE, &args![at + NEXT]).u32();
            e.call(MAKE_TAGGED, &args![marked, 1u32, node]);
            let expected = e.call(TAGGED_VALUE, &args![unmarked]).u32();
            let wanted = e.call(TAGGED_VALUE, &args![marked]).u32();
            let record = e.call(NODE_RECORD, &args![at + CURRENT]).u32();
            if e.call(COMPARE_SWAP, &args![record + 8, wanted, expected])
                .bool()
            {
                break;
            }
        }
        let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
        e.call(MAKE_TAGGED, &args![current, 0u32, node]);
        let expected = e.call(TAGGED_VALUE, &args![current]).u32();
        let wanted = e.call(TAGGED_VALUE, &args![unmarked]).u32();
        let slot = e.mem.u32(at + CURSOR_RECORD);
        if e.call(COMPARE_SWAP, &args![slot, wanted, expected]).bool() {
            let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
            e.call(RETIRE_NODE, &args![this, node]);
        } else {
            fn_00906820(e, this, key, value);
        }
        true
    });
    e.call(END_OPERATION, &args![this]);
    found
}

// Translated from 009067f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LockFreeMap` scalar deleting destructor: the destructor body
/// (`009064e0`), then `operator delete(this)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_009067f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0090_64e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00906820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906820 - the compiler's exception frame (FS:[0]) that releases the pointer local when a step throws is not translated
/// Finds `key`'s node in the inner object's lock-free list (bucket from
/// `0044ee20` on the object at `this`): it walks the list, publishing each
/// current and next node in the hazard slots and starting over from the
/// bucket whenever a re-read shows the list changed. Marked nodes on the way
/// are unlinked (`004491c0`, then retired with `005292a0`); a node whose
/// pointer matches `value` (`004b0460`) answers true, the end of the list
/// false.
pub fn fn_00906820(e: &mut Engine, this: Ptr, key: u32, value: u32) -> bool {
    let at = this.addr();
    let owner = e.mem.u32(at);
    let bucket = e.call(BUCKET_OF, &args![owner, key]).u32();
    e.with_stack(0x10, |e, frame| {
        // The tagged words at EBP-0x18, -0x1C and -0x24 and the pointer
        // local at EBP-0x20.
        let first = frame.addr();
        let check = frame.addr() + 4;
        let pointer = frame.addr() + 8;
        let unmarked = frame.addr() + 0xc;
        let publish = |e: &mut Engine, cursor: u32, slot: u32| {
            let node = e.call(CURSOR_NODE, &args![at + cursor]).u32();
            let hazard = e.mem.u32(at + slot);
            e.mem.set_u32(hazard, node);
            e.call(BARRIER, &[]);
        };
        'restart: loop {
            e.mem.set_u32(at + CURSOR_RECORD, bucket);
            let head = e.mem.u32(e.mem.u32(at + CURSOR_RECORD));
            e.mem.set_u32(at + CURRENT, head);
            publish(e, CURRENT, 8);
            let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
            e.call(MAKE_TAGGED, &args![first, 0u32, node]);
            let record = e.mem.u32(at + CURSOR_RECORD);
            let stored = e.call(TAGGED_VALUE, &args![record]).u32();
            if stored != e.call(TAGGED_VALUE, &args![first]).u32() {
                continue 'restart;
            }
            loop {
                if e.call(CURSOR_NODE, &args![at + CURRENT]).u32() == 0 {
                    return false;
                }
                let record = e.call(NODE_RECORD, &args![at + CURRENT]).u32();
                let next = e.mem.u32(record + 8);
                e.mem.set_u32(at + NEXT, next);
                publish(e, NEXT, 4);
                let read = e.call(TAGGED_VALUE, &args![at + NEXT]).u32();
                let record = e.call(NODE_RECORD, &args![at + CURRENT]).u32();
                if read != e.call(TAGGED_VALUE, &args![record + 8]).u32() {
                    continue 'restart;
                }
                let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
                e.call(POINTER_CONSTRUCT, &args![pointer, node + 4]);
                let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
                e.call(MAKE_TAGGED, &args![check, 0u32, node]);
                let record = e.mem.u32(at + CURSOR_RECORD);
                let stored = e.call(TAGGED_VALUE, &args![record]).u32();
                if stored != e.call(TAGGED_VALUE, &args![check]).u32() {
                    e.call(POINTER_RELEASE, &args![pointer]);
                    continue 'restart;
                }
                if !e.call(NODE_MARKED, &args![at + NEXT]).bool() {
                    if e.call(POINTER_MATCHES, &args![pointer, value]).bool() {
                        e.call(POINTER_RELEASE, &args![pointer]);
                        return true;
                    }
                    let record = e.call(NODE_RECORD, &args![at + CURRENT]).u32();
                    e.mem.set_u32(at + CURSOR_RECORD, record + 8);
                    publish(e, CURRENT, 0xc);
                } else {
                    let node = e.call(CURSOR_NODE, &args![at + NEXT]).u32();
                    e.call(MAKE_TAGGED, &args![unmarked, 0u32, node]);
                    let expected = e.call(TAGGED_VALUE, &args![check]).u32();
                    let wanted = e.call(TAGGED_VALUE, &args![unmarked]).u32();
                    let slot = e.mem.u32(at + CURSOR_RECORD);
                    if !e.call(COMPARE_SWAP, &args![slot, wanted, expected]).bool() {
                        e.call(POINTER_RELEASE, &args![pointer]);
                        continue 'restart;
                    }
                    let node = e.call(CURSOR_NODE, &args![at + CURRENT]).u32();
                    e.call(RETIRE_NODE, &args![this, node]);
                }
                let next = e.mem.u32(at + NEXT);
                e.mem.set_u32(at + CURRENT, next);
                publish(e, NEXT, 8);
                e.call(POINTER_RELEASE, &args![pointer]);
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0090_66d0, fn_009066d0(Ptr, u32, u32) -> bool),
        entry!(0x0090_67f0, fn_009067f0(Ptr, u32) -> Ptr),
        entry!(0x0090_6820, fn_00906820(Ptr, u32, u32) -> bool),
        entry!(0x0090_6dc0, fn_00906dc0(Ptr) -> Ptr),
        entry!(0x0090_6f60, fn_00906f60(Ptr) -> Ptr),
        entry!(0x0090_6f80, fn_00906f80(Ptr, u8, u8, u8)),
        entry!(0x0090_6af0, fn_00906af0(Ptr, u32, u32, u32) -> u8),
        entry!(0x0090_6c10, fn_00906c10(Ptr, u32, u32, u32) -> u8),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00906dc0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0090_6dc0,
            size: 0xb4,
            globals: &[(0x0101_2054, 0x3f80_0000)],
            returns: &[],
            calls: &[
                (0x008d_0270, &[A::T(0)]),
                (0x0090_6f60, &[A::T(0x38)]),
                (0x0096_a2d0, &[A::T(0x5c)]),
                (0x0096_a2d0, &[A::T(0x6c)]),
                (0x0096_a2d0, &[A::T(0x74)]),
                (0x0096_a2d0, &[A::T(0x7c)]),
                (0x0093_7000, &[A::T(0x94)]),
                (0x008d_03a0, &[A::T(0)]),
                (0x0090_6f80, &[A::T(0), A::V(0), A::V(0xff), A::V(0)]),
                (0x0061_cc40, &[A::T(0), A::V(3)]),
            ],
            stores: &[
                (0x00, 4, 0x0108_86e4),
                (0x1c, 4, 0),
                (0x20, 4, 0x3f80_0000),
                (0x24, 4, 0),
                (0x34, 4, 0),
                (0x40, 4, 0),
                (0x44, 4, 0),
                (0x48, 4, 0),
                (0x4c, 4, 0),
                (0x54, 4, 0),
                (0x58, 4, 0),
                (0x64, 4, 0),
                (0x68, 4, 0),
                (0x84, 4, 0),
                (0x88, 4, 0),
                (0x8c, 4, 0),
                (0x90, 4, 0),
                (0xa8, 4, 0),
                (0xac, 4, 0),
                (0xb0, 1, 0),
            ],
        });
    }
}

#[cfg(test)]
mod helper_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00906f60_zeroes_both_floats_and_returns_this() {
        let mut e = Engine::new();
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0xdead_beef);
        e.mem.set_u32(pair + 4, 0xdead_beef);
        assert_eq!(e.call(0x0090_6f60, &args![pair]).u32(), pair);
        assert_eq!(e.mem.u32(pair), 0);
        assert_eq!(e.mem.u32(pair + 4), 0);
    }

    #[test]
    fn fn_00906f80_replaces_the_masked_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 0x30, 0b1111_0000);
        // value 0b11 shifted by 1, mask 0b0000_0110.
        e.call(0x0090_6f80, &args![object, 3u32, 6u32, 1u32]);
        assert_eq!(e.mem.u8(object + 0x30), 0b1111_0110);
        // The constructor's call: value 0, mask 0xff, shift 0 clears the byte.
        e.call(0x0090_6f80, &args![object, 0u32, 0xffu32, 0u32]);
        assert_eq!(e.mem.u8(object + 0x30), 0);
        // The shift count is taken modulo 32 and bits shifted past the mask are
        // still ORed in; bits shifted out of the low byte are dropped.
        e.mem.set_u8(object + 0x30, 0);
        e.call(0x0090_6f80, &args![object, 1u32, 0u32, 33u32]);
        assert_eq!(e.mem.u8(object + 0x30), 2);
        e.call(0x0090_6f80, &args![object, 1u32, 0u32, 8u32]);
        assert_eq!(e.mem.u8(object + 0x30), 2);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The object (`this`), its owner with vtable slot `0x30`, and doubles: the iterator's node
    /// is the word at the iterator, its element the node itself; `0044df20` answers the
    /// element's `+8` word.
    fn world(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20);
        let owner = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x7000_0030);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(this, owner);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(this + 8, slot);
        e.register(ITERATOR_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ITERATOR_ELEMENT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ELEMENT_SKIPPED, |e, a| {
            (e.mem.u32(a[0] + 8) != 0).into_ret()
        });
        for address in [
            0x7000_0030,
            STORE_RESULT,
            0x0040_fbe0,
            0x0052_9000,
            0x006f_74f0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        this
    }

    #[test]
    fn fn_00906af0_takes_the_element_of_the_found_node() {
        let mut e = Engine::new();
        let this = world(&mut e);
        let element = e.mem.alloc(0x10);
        e.mem.set_u32(element, 0x99);
        let node = e.mem.alloc(4);
        e.mem.set_u32(node, element);
        e.register_double(0x0044_ee20, move |_, _| node.into_ret());
        e.register(0x006e_cba0, |_, _| Ret::default());
        // The stored node and the temporary agree only on the second try.
        let tries = Rc::new(RefCell::new(0u32));
        let count = tries.clone();
        e.register_double(0x0055_9450, move |_, _| {
            *count.borrow_mut() += 1;
            let n = *count.borrow();
            (if n <= 2 { n } else { 5 }).into_ret()
        });
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x0090_6af0, &args![this, 0x55u32, 3u32, 0x4000u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 1);
        assert_eq!(calls(&log, 0x0044_ee20).len(), 2);
        assert_eq!(e.mem.u32(this + 0x10), node);
        assert_eq!(e.mem.u32(this + 0x14), element);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), element);
        let owner = e.mem.u32(this);
        assert_eq!(calls(&log, 0x7000_0030), vec![vec![owner, 0x99, 3]]);
        assert_eq!(calls(&log, STORE_RESULT), vec![vec![0x4000, element + 4]]);
        assert_eq!(calls(&log, 0x0052_9000), vec![vec![this]]);
        // No element: 0.
        e.mem.set_u32(node, 0);
        e.register(0x0055_9450, |_, _| 1u32.into_ret());
        assert_eq!(
            e.call(0x0090_6af0, &args![this, 0x55u32, 3u32, 0x4000u32])
                .u8(),
            0
        );
    }

    #[test]
    fn fn_00906c10_picks_the_iterator_and_takes_its_element() {
        let mut e = Engine::new();
        let this = world(&mut e);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, 0x11);
        let second_element = e.mem.alloc(0x10);
        e.mem.set_u32(second_element, 0x22);
        e.mem.set_u32(this + 0x14, first);
        e.mem.set_u32(this + 0x18, second_element);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, 0x66);
        e.register(0x0052_9030, |_, a| u32::from(a[2] == 0x66).into_ret());
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x0090_6c10, &args![this, 0x55u32, pointer, 0x4000u32])
            .u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(answer, 1);
        assert_eq!(calls(&log, 0x0052_9030), vec![vec![this, 0x55, 0x66]]);
        let owner = e.mem.u32(this);
        assert_eq!(calls(&log, 0x7000_0030), vec![vec![owner, 0x22, pointer]]);
        assert_eq!(
            calls(&log, STORE_RESULT),
            vec![vec![0x4000, second_element + 4]]
        );
        // The other iterator, empty: 0.
        e.mem.set_u32(pointer, 0);
        e.mem.set_u32(this + 0x14, 0);
        assert_eq!(
            e.call(0x0090_6c10, &args![this, 0x55u32, pointer, 0x4000u32])
                .u8(),
            0
        );
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A list model for the doubles: a cursor word holds a node with bit 0
    /// as the mark; a node's pointer is at +4 and its next word at +8.
    fn list_doubles(e: &mut Engine, bucket: u32) {
        e.register(CURSOR_NODE, |e, a| ret(e.mem.u32(a[0]) & !1));
        e.register(NODE_RECORD, |e, a| ret(e.mem.u32(a[0]) & !1));
        e.register(MAKE_TAGGED, |e, a| {
            e.mem.set_u32(a[0], a[2] | a[1]);
            ret(a[0])
        });
        e.register(TAGGED_VALUE, |e, a| ret(e.mem.u32(a[0])));
        e.register(COMPARE_SWAP, |e, a| {
            if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                ret(1)
            } else {
                ret(0)
            }
        });
        e.register(NODE_MARKED, |e, a| ret(e.mem.u32(a[0]) & 1));
        e.register(POINTER_CONSTRUCT, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(POINTER_MATCHES, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])))
        });
        e.register_double(BUCKET_OF, move |_, _| ret(bucket));
        for callee in [POINTER_RELEASE, BARRIER, RETIRE_NODE, END_OPERATION] {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// The inner object (owner, three hazard slots, the cursor words), a
    /// bucket word and the list `values` in order.
    fn list(e: &mut Engine, values: &[u32]) -> (u32, u32, Vec<u32>) {
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, 0x0a00);
        for slot in [4, 8, 0xc] {
            let hazard = e.mem.alloc(4);
            e.mem.set_u32(this + slot, hazard);
        }
        let bucket = e.mem.alloc(4);
        let nodes: Vec<u32> = values.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, &value) in values.iter().enumerate() {
            e.mem.set_u32(nodes[i] + 4, value);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[i] + 8, next);
        }
        e.mem.set_u32(bucket, nodes.first().copied().unwrap_or(0));
        list_doubles(e, bucket);
        (this, bucket, nodes)
    }

    fn wanted(e: &mut Engine, value: u32) -> u32 {
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, value);
        word
    }

    #[test]
    fn fn_00906820_finds_the_node_with_the_value() {
        let mut e = Engine::new();
        let (this, bucket, nodes) = list(&mut e, &[0x111, 0x222]);
        let value = wanted(&mut e, 0x222);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_6820, &[this, 7, value]).bool());
        assert_eq!(calls_to(&e, BUCKET_OF), vec![vec![0x0a00, 7]]);
        assert_eq!(e.mem.u32(this + CURRENT), nodes[1]);
        assert_eq!(e.mem.u32(this + CURSOR_RECORD), nodes[0] + 8);
        assert_eq!(e.mem.u32(bucket), nodes[0]);
        let missing = wanted(&mut e, 0x333);
        assert!(!e.call(0x0090_6820, &[this, 7, missing]).bool());
    }

    #[test]
    fn fn_00906820_unlinks_a_marked_node_on_the_way() {
        let mut e = Engine::new();
        let (this, bucket, nodes) = list(&mut e, &[0x111, 0x222]);
        e.mem.set_u32(nodes[0] + 8, nodes[1] | 1);
        let value = wanted(&mut e, 0x222);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_6820, &[this, 7, value]).bool());
        assert_eq!(e.mem.u32(bucket), nodes[1]);
        assert_eq!(calls_to(&e, RETIRE_NODE), vec![vec![this, nodes[0]]]);
    }

    #[test]
    fn fn_009066d0_marks_and_unlinks_the_found_node() {
        let mut e = Engine::new();
        let (this, _bucket, nodes) = list(&mut e, &[0x111, 0x222, 0x333]);
        let value = wanted(&mut e, 0x222);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_66d0, &[this, 7, value]).bool());
        assert_eq!(e.mem.u32(nodes[1] + 8), nodes[2] | 1);
        assert_eq!(e.mem.u32(nodes[0] + 8), nodes[2]);
        assert_eq!(calls_to(&e, RETIRE_NODE), vec![vec![this, nodes[1]]]);
        assert_eq!(calls_to(&e, END_OPERATION), vec![vec![this]]);
        let missing = wanted(&mut e, 0x444);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_66d0, &[this, 7, missing]).bool());
        assert_eq!(calls_to(&e, END_OPERATION), vec![vec![this]]);
    }

    #[test]
    fn fn_009067f0_destroys_then_maybe_frees() {
        let mut e = Engine::new();
        e.register(0x0090_64e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_67f0, &[0x9000, 0]).u32(), 0x9000);
        assert!(calls_to(&e, 0x0040_1030).is_empty());
        e.call(0x0090_67f0, &[0x9000, 1]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, 0x0090_64e0).len(), 2);
    }
}
