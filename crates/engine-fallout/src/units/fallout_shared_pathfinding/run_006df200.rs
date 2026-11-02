//! `fallout shared/pathfinding/run_006df200` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`): the word at `this`.
const FIRST_VALUE_00559450: u32 = 0x0055_9450;
/// `006e1610` (`thiscall`, `RET 4`): a byte map of the object, 0 when there
/// is none.
const BYTE_MAP_006E1610: u32 = 0x006e_1610;
/// `0042fb00` and `0042fa40` (`thiscall`): construct and destroy the
/// 4-byte local `0069ad00` fills.
const LOCAL_CONSTRUCT_0042FB00: u32 = 0x0042_fb00;
const LOCAL_DESTRUCT_0042FA40: u32 = 0x0042_fa40;
/// `0069ad00` (`thiscall` on the word at `+0x14`, `RET 4`: the local): a
/// byte; fills the local.
const FILL_LOCAL_0069AD00: u32 = 0x0069_ad00;
/// `006911f0` (`thiscall` on the local's object, `RET 4`: the word at
/// `+0x18`): an object with an array of 16-bit indices, 0 when there is
/// none.
const INDEX_ARRAY_006911F0: u32 = 0x0069_11f0;
/// `0076b610` (`thiscall` on the array at `+4`): a byte.
const ARRAY_CHECK_0076B610: u32 = 0x0076_b610;
/// `0044ddc0` (`thiscall`): an array's element count.
const COUNT_0044DDC0: u32 = 0x0044_ddc0;
/// `0042f5a0` (cdecl: two words): the smaller, unsigned.
const SMALLER_0042F5A0: u32 = 0x0042_f5a0;
/// `006a62f0` (`thiscall` on the array at `+4`, `RET 4`: index): the
/// address of byte `index`.
const BYTE_ELEMENT_006A62F0: u32 = 0x006a_62f0;
/// `0069dec0` (`thiscall`, `RET 4`: index): the address of 16-bit
/// element `index` of the index array.
const INDEX_ELEMENT_0069DEC0: u32 = 0x0069_dec0;
/// `008454f0` (`thiscall` on the array at `+4`, `RET 4`: a byte).
const ARRAY_RESET_008454F0: u32 = 0x0084_54f0;
/// `006e10c0` (`thiscall` on the array at `+4`, `RET 8`: count, 1): sizes
/// the array.
const ARRAY_SIZE_006E10C0: u32 = 0x006e_10c0;

// Translated from 006df200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the object (`thiscall`; two words; `RET 8`; no name in the
/// engine map): the byte at `+0` is 0, the words go to `+0x14` and
/// `+0x18`.
pub fn fn_006df200(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let base = this.addr();
    e.mem.set_u8(base, 0);
    e.mem.set_u32(base + 0x14, first);
    e.mem.set_u32(base + 0x18, second);
}

/// The local the four functions below keep at `[EBP-0x14]`: constructed,
/// filled by `0069ad00` from the word at `+0x14`; `None` (after
/// destroying it) when that fails, else the index array `006911f0` gives
/// for the word at `+0x18`. `body` runs with it and its answer is
/// returned after the local is destroyed. `check` (`006df230` and
/// `006df440` make it) answers false, after destroying the local, for a
/// null index array.
fn with_index_array(
    e: &mut Engine,
    this: u32,
    check: bool,
    body: impl FnOnce(&mut Engine, u32) -> bool,
) -> bool {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(LOCAL_CONSTRUCT_0042FB00, &args![local]);
        let source = e.mem.u32(this + 0x14);
        if !e.call(FILL_LOCAL_0069AD00, &args![source, local]).bool() {
            e.call(LOCAL_DESTRUCT_0042FA40, &args![local]);
            return false;
        }
        let key = e.mem.u32(this + 0x18);
        let object = e.call(FIRST_VALUE_00559450, &args![local]).u32();
        let indices = e.call(INDEX_ARRAY_006911F0, &args![object, key]).u32();
        if check && indices == 0 {
            e.call(LOCAL_DESTRUCT_0042FA40, &args![local]);
            return false;
        }
        let answer = body(e, indices);
        e.call(LOCAL_DESTRUCT_0042FA40, &args![local]);
        answer
    })
}

/// The byte map `006e1610` gives for `other`'s first word with the first
/// word of the object at `+0x14`.
fn byte_map(e: &mut Engine, this: u32, other: Ptr) -> u32 {
    let source = e.mem.u32(this + 0x14);
    let key = e.call(FIRST_VALUE_00559450, &args![source]).u32();
    let object = e.call(FIRST_VALUE_00559450, &args![other]).u32();
    e.call(BYTE_MAP_006E1610, &args![object, key]).u32()
}

/// The 16-bit index at element `index` of the index array.
fn index_at(e: &mut Engine, indices: u32, index: u32) -> u32 {
    let slot = e.call(INDEX_ELEMENT_0069DEC0, &args![indices, index]).u32();
    u32::from(e.mem.u16(slot))
}

/// The address of byte `index` of the array at `+4`.
fn byte_at(e: &mut Engine, this: u32, index: u32) -> u32 {
    e.call(BYTE_ELEMENT_006A62F0, &args![this + 4, index]).u32()
}

// Translated from 006df230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006df230 - the exception-unwinding frame (FS:[0]) that destroys the local on unwind is not translated.
/// Writes this object's bytes into `other`'s byte map (`thiscall`; one
/// word; `RET 4`; no name in the engine map). False without a byte map
/// (`006e1610`), when the local cannot be filled or there is no index
/// array; else true after: when `0076b610` of the array at `+4` is clear,
/// for the smaller of the two counts, the map byte at each index gets the
/// array's byte; when it is set and the byte at `+0` is set, the map byte
/// at every index is 3; when it is set and the byte at `+0` is clear, the
/// map bytes at the indices that hold 0x7F are cleared.
pub fn fn_006df230(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let base = this.addr();
    let map = byte_map(e, base, other);
    if map == 0 {
        return false;
    }
    with_index_array(e, base, true, |e, indices| {
        if !e.call(ARRAY_CHECK_0076B610, &args![base + 4]).bool() {
            let theirs = e.call(COUNT_0044DDC0, &args![indices]).u32();
            let ours = e.call(COUNT_0044DDC0, &args![base + 4]).u32();
            let count = e.call(SMALLER_0042F5A0, &args![ours, theirs]).u32();
            let mut index = 0u32;
            while index < count {
                let source = byte_at(e, base, index);
                let at = index_at(e, indices, index);
                let value = e.mem.u8(source);
                e.mem.set_u8(map.wrapping_add(at), value);
                index += 1;
            }
        } else if e.mem.u8(base) != 0 {
            let mut index = 0u32;
            while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
                let at = index_at(e, indices, index);
                e.mem.set_u8(map.wrapping_add(at), 3);
                index += 1;
            }
        } else {
            let mut index = 0u32;
            while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
                let at = index_at(e, indices, index);
                if e.mem.i8(map.wrapping_add(at)) == 0x7f {
                    let at = index_at(e, indices, index);
                    e.mem.set_u8(map.wrapping_add(at), 0);
                }
                index += 1;
            }
        }
        true
    })
}

// Translated from 006df440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006df440 - the exception-unwinding frame (FS:[0]) that destroys the local on unwind is not translated.
/// Reads this object's bytes from `other`'s byte map (`thiscall`; one word;
/// `RET 4`; no name in the engine map), the counterpart of
/// [`fn_006df230`]. False as there; else the array at `+4` is reset
/// (`008454f0` with 0) and sized to the index count (`006e10c0` with 1),
/// the byte at `+0` is set, and each array byte gets the map byte at its
/// index; the byte at `+0` is cleared when any of them is 0 or below
/// (signed) or 0x7F. When it stays set, `008454f0` runs with 1. True.
pub fn fn_006df440(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let base = this.addr();
    let map = byte_map(e, base, other);
    if map == 0 {
        return false;
    }
    with_index_array(e, base, true, |e, indices| {
        e.call(ARRAY_RESET_008454F0, &args![base + 4, 0u32]);
        let count = e.call(COUNT_0044DDC0, &args![indices]).u32();
        e.call(ARRAY_SIZE_006E10C0, &args![base + 4, count, 1u32]);
        e.mem.set_u8(base, 1);
        let mut index = 0u32;
        while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
            let at = index_at(e, indices, index);
            let target = byte_at(e, base, index);
            let value = e.mem.u8(map.wrapping_add(at));
            e.mem.set_u8(target, value);
            let value = byte_at(e, base, index);
            if e.mem.i8(value) <= 0 {
                e.mem.set_u8(base, 0);
            } else {
                let value = byte_at(e, base, index);
                if e.mem.i8(value) == 0x7f {
                    e.mem.set_u8(base, 0);
                }
            }
            index += 1;
        }
        if e.mem.u8(base) != 0 {
            e.call(ARRAY_RESET_008454F0, &args![base + 4, 1u32]);
        }
        true
    })
}

// Translated from 006df5f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006df5f0 - the exception-unwinding frame (FS:[0]) that destroys the local on unwind is not translated.
/// Whether `other` (its 16-bit index `006df760`) is marked (`thiscall`;
/// one word; `RET 4`; no name in the engine map): true when the byte at
/// `+0` is set; else false when the local cannot be filled or the index
/// array's count differs from the array at `+4`'s; else whether the array
/// byte at the first position holding `other`'s index is above 0
/// (signed), false when no position does.
pub fn fn_006df5f0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let base = this.addr();
    if e.mem.u8(base) != 0 {
        return true;
    }
    with_index_array(e, base, false, |e, indices| {
        let theirs = e.call(COUNT_0044DDC0, &args![indices]).u32();
        let ours = e.call(COUNT_0044DDC0, &args![base + 4]).u32();
        if theirs != ours {
            return false;
        }
        let mut index = 0u32;
        while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
            let at = index_at(e, indices, index);
            if at == u32::from(fn_006df760(e, other)) {
                let value = byte_at(e, base, index);
                return e.mem.i8(value) > 0;
            }
            index += 1;
        }
        false
    })
}

// Translated from 006df760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `+0x24` (`thiscall`, plain `RET`; no name in the
/// engine map).
pub fn fn_006df760(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x24)
}

// Translated from 006df780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006df780 - the exception-unwinding frame (FS:[0]) that destroys the local on unwind is not translated.
/// Marks `other` (its 16-bit index `006df760`) as `on` (`thiscall`; a word
/// and a byte; `RET 8`; no name in the engine map). False when the local
/// cannot be filled. With the byte at `+0` set: true at once when `on`;
/// else the byte is cleared, the array at `+4` sized to the index count
/// (`006e10c0` with 1) and each array byte becomes 0xFF where the index is
/// `other`'s, 1 elsewhere; true. With it clear: false when the counts
/// differ; else the array byte at the first position holding `other`'s
/// index becomes 1 (`on`) or 0xFF and the answer is true; false when no
/// position does.
pub fn fn_006df780(e: &mut Engine, this: Ptr, other: Ptr, on: u8) -> bool {
    let base = this.addr();
    with_index_array(e, base, false, |e, indices| {
        if e.mem.u8(base) != 0 {
            if on != 0 {
                return true;
            }
            e.mem.set_u8(base, 0);
            let count = e.call(COUNT_0044DDC0, &args![indices]).u32();
            e.call(ARRAY_SIZE_006E10C0, &args![base + 4, count, 1u32]);
            let mut index = 0u32;
            while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
                let at = index_at(e, indices, index);
                let value = if at == u32::from(fn_006df760(e, other)) {
                    0xff
                } else {
                    1
                };
                let target = byte_at(e, base, index);
                e.mem.set_u8(target, value);
                index += 1;
            }
            return true;
        }
        let theirs = e.call(COUNT_0044DDC0, &args![indices]).u32();
        let ours = e.call(COUNT_0044DDC0, &args![base + 4]).u32();
        if theirs != ours {
            return false;
        }
        let mut index = 0u32;
        while index < e.call(COUNT_0044DDC0, &args![indices]).u32() {
            let at = index_at(e, indices, index);
            if at == u32::from(fn_006df760(e, other)) {
                // `NEG; SBB; AND 2; ADD -1`: 1 when set, 0xFF when clear.
                let value = if on != 0 { 1 } else { 0xff };
                let target = byte_at(e, base, index);
                e.mem.set_u8(target, value);
                return true;
            }
            index += 1;
        }
        false
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006d_f200, fn_006df200(Ptr, u32, u32)),
        entry!(0x006d_f230, fn_006df230(Ptr, Ptr) -> bool),
        entry!(0x006d_f440, fn_006df440(Ptr, Ptr) -> bool),
        entry!(0x006d_f5f0, fn_006df5f0(Ptr, Ptr) -> bool),
        entry!(0x006d_f760, fn_006df760(Ptr) -> u16),
        entry!(0x006d_f780, fn_006df780(Ptr, Ptr, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The object (byte at `+0`, byte array at `+4` with storage at `+8`
    /// and count at `+0xc`, words at `+0x14`, `+0x18`), an index array
    /// (16-bit indices, storage at `+4`, count at `+8`), a byte map and
    /// doubles over them. `fills` answers `0069ad00`, `with_indices` makes
    /// `006911f0` answer the index array (else 0), `flagged` answers
    /// `0076b610`.
    struct MarkCase {
        e: Engine,
        this: u32,
        other: u32,
        map: u32,
    }

    fn mark_case(
        bytes: &[u8],
        indices: &[u16],
        fills: bool,
        with_indices: bool,
        flagged: bool,
    ) -> MarkCase {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        let storage = e.mem.alloc(0x40);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(storage + i as u32, *b);
        }
        // The array at +4: the doubles read its storage at +4 and count at
        // +8 of the array.
        e.mem.set_u32(this + 8, storage);
        e.mem.set_u32(this + 0xc, bytes.len() as u32);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x5151);
        e.mem.set_u32(this + 0x14, source);
        e.mem.set_u32(this + 0x18, 0x1818);
        let index_array = e.mem.alloc(0xc);
        let index_storage = e.mem.alloc(0x40);
        for (i, at) in indices.iter().enumerate() {
            e.mem.set_u16(index_storage + 2 * i as u32, *at);
        }
        e.mem.set_u32(index_array + 4, index_storage);
        e.mem.set_u32(index_array + 8, indices.len() as u32);
        let map = e.mem.alloc(0x100);
        let other = e.mem.alloc(0x28);
        e.mem.set_u32(other, 0x0707);
        e.mem.set_u16(other + 0x24, 5);
        e.register(FIRST_VALUE_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(BYTE_MAP_006E1610, move |_, a| {
            assert_eq!(a, &[0x0707, 0x5151]);
            map.into_ret()
        });
        e.register(LOCAL_CONSTRUCT_0042FB00, |_, _| Ret::default());
        e.register(LOCAL_DESTRUCT_0042FA40, |_, _| Ret::default());
        e.register_double(FILL_LOCAL_0069AD00, move |e, a| {
            e.mem.set_u32(a[1], 0x4242);
            u32::from(fills).into_ret()
        });
        e.register_double(INDEX_ARRAY_006911F0, move |_, a| {
            assert_eq!(a, &[0x4242, 0x1818]);
            (if with_indices { index_array } else { 0 }).into_ret()
        });
        e.register_double(ARRAY_CHECK_0076B610, move |_, _| {
            u32::from(flagged).into_ret()
        });
        e.register(COUNT_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SMALLER_0042F5A0, |_, a| a[0].min(a[1]).into_ret());
        e.register(BYTE_ELEMENT_006A62F0, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1]).into_ret()
        });
        e.register(INDEX_ELEMENT_0069DEC0, |e, a| {
            (e.mem.u32(a[0] + 4) + 2 * a[1]).into_ret()
        });
        e.register(ARRAY_RESET_008454F0, |_, _| Ret::default());
        e.register(ARRAY_SIZE_006E10C0, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        MarkCase {
            e,
            this,
            other,
            map,
        }
    }

    fn array_bytes(e: &Engine, this: u32) -> Vec<u8> {
        let storage = e.mem.u32(this + 8);
        (0..e.mem.u32(this + 0xc))
            .map(|i| e.mem.u8(storage + i))
            .collect()
    }

    #[test]
    fn fn_006df200_initialises() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, 0xffff_ffff);
        e.call(0x006d_f200, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u32(this), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 0x14), 0x11);
        assert_eq!(e.mem.u32(this + 0x18), 0x22);
    }

    #[test]
    fn fn_006df760_reads_the_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x24, 0xaaaa_1234);
        assert_eq!(e.call(0x006d_f760, &args![this]).u16(), 0x1234);
    }

    #[test]
    fn fn_006df230_writes_the_bytes_into_the_map() {
        // Unflagged: the smaller count (2 of 3 indices) copies the bytes.
        let MarkCase {
            mut e,
            this,
            other,
            map,
        } = mark_case(&[7, 8], &[10, 20, 30], true, true, false);
        assert!(e.call(0x006d_f230, &args![this, other]).bool());
        assert_eq!(
            [e.mem.u8(map + 10), e.mem.u8(map + 20), e.mem.u8(map + 30)],
            [7, 8, 0]
        );
        // Flagged with the byte at +0 set: 3 at every index.
        let MarkCase {
            mut e,
            this,
            other,
            map,
        } = mark_case(&[7, 8], &[10, 20, 30], true, true, true);
        e.mem.set_u8(this, 1);
        assert!(e.call(0x006d_f230, &args![this, other]).bool());
        assert_eq!(
            [e.mem.u8(map + 10), e.mem.u8(map + 20), e.mem.u8(map + 30)],
            [3, 3, 3]
        );
        // Flagged with it clear: 0x7F bytes cleared, others kept.
        let MarkCase {
            mut e,
            this,
            other,
            map,
        } = mark_case(&[], &[10, 20], true, true, true);
        e.mem.set_u8(map + 10, 0x7f);
        e.mem.set_u8(map + 20, 0x7e);
        assert!(e.call(0x006d_f230, &args![this, other]).bool());
        assert_eq!([e.mem.u8(map + 10), e.mem.u8(map + 20)], [0, 0x7e]);
    }

    #[test]
    fn fn_006df230_and_006df440_fail_without_the_parts() {
        for addr in [0x006d_f230u32, 0x006d_f440] {
            // No byte map.
            let MarkCase {
                mut e, this, other, ..
            } = mark_case(&[1], &[1], true, true, false);
            e.register(BYTE_MAP_006E1610, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert!(!e.call(addr, &args![this, other]).bool());
            let log = e.call_log.take().unwrap();
            assert!(!log.iter().any(|(a, _)| *a == LOCAL_CONSTRUCT_0042FB00));
            // The fill fails, or there is no index array: the local is
            // destroyed.
            for (fills, with_indices) in [(false, true), (true, false)] {
                let MarkCase {
                    mut e, this, other, ..
                } = mark_case(&[1], &[1], fills, with_indices, false);
                e.call_log = Some(vec![]);
                assert!(!e.call(addr, &args![this, other]).bool());
                let log = e.call_log.take().unwrap();
                assert_eq!(
                    log.iter()
                        .filter(|(a, _)| *a == LOCAL_DESTRUCT_0042FA40)
                        .count(),
                    1
                );
            }
        }
    }

    #[test]
    fn fn_006df440_reads_the_bytes_from_the_map() {
        let MarkCase {
            mut e,
            this,
            other,
            map,
        } = mark_case(&[0, 0], &[10, 20], true, true, false);
        e.mem.set_u8(map + 10, 4);
        e.mem.set_u8(map + 20, 9);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_f440, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(array_bytes(&e, this), vec![4, 9]);
        // All above 0 and none 0x7F: the byte at +0 stays set and the
        // array is reset with 1.
        assert_eq!(e.mem.u8(this), 1);
        let resets: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ARRAY_RESET_008454F0)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(resets, vec![0, 1]);
        let sizes: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ARRAY_SIZE_006E10C0)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(sizes, vec![vec![this + 4, 2, 1]]);
        // A 0x7F, a 0 or a negative byte clears the byte at +0.
        for bad in [0x7fu8, 0, 0x80] {
            let MarkCase {
                mut e,
                this,
                other,
                map,
            } = mark_case(&[0, 0], &[10, 20], true, true, false);
            e.mem.set_u8(map + 10, 4);
            e.mem.set_u8(map + 20, bad);
            e.call_log = Some(vec![]);
            assert!(e.call(0x006d_f440, &args![this, other]).bool());
            assert_eq!(e.mem.u8(this), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log.iter()
                    .filter(|(a, _)| *a == ARRAY_RESET_008454F0)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn fn_006df5f0_reads_the_mark_of_the_other_index() {
        // The byte at +0 set: true without looking.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[0, 2], &[3, 5], true, true, false);
        e.mem.set_u8(this, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_f5f0, &args![this, other]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // The other's index 5 is at position 1, whose byte is 2.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[0, 2], &[3, 5], true, true, false);
        assert!(e.call(0x006d_f5f0, &args![this, other]).bool());
        // A byte of 0x80 (negative) is not a mark.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[0, 0x80], &[3, 5], true, true, false);
        assert!(!e.call(0x006d_f5f0, &args![this, other]).bool());
        // Counts differ, the index is missing, or the fill fails.
        for (bytes, indices, fills) in [
            (&[1u8][..], &[3u16, 5][..], true),
            (&[1, 1][..], &[3, 4][..], true),
            (&[1, 1][..], &[3, 5][..], false),
        ] {
            let MarkCase {
                mut e, this, other, ..
            } = mark_case(bytes, indices, fills, true, false);
            assert!(!e.call(0x006d_f5f0, &args![this, other]).bool());
        }
    }

    #[test]
    fn fn_006df780_marks_the_other_index() {
        // Clear byte at +0: the first matching position gets 1 or 0xFF.
        for (on, expected) in [(1u32, 1u8), (0x100, 0xff)] {
            let MarkCase {
                mut e, this, other, ..
            } = mark_case(&[9, 9, 9], &[3, 5, 5], true, true, false);
            assert!(e.call(0x006d_f780, &args![this, other, on]).bool());
            assert_eq!(array_bytes(&e, this), vec![9, expected, 9]);
        }
        // Counts differ or the index is missing: false.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[9], &[3, 5], true, true, false);
        assert!(!e.call(0x006d_f780, &args![this, other, 1u32]).bool());
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[9, 9], &[3, 4], true, true, false);
        assert!(!e.call(0x006d_f780, &args![this, other, 1u32]).bool());
        // Set byte at +0 and `on`: true, nothing changes.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[9, 9], &[3, 5], true, true, false);
        e.mem.set_u8(this, 1);
        assert!(e.call(0x006d_f780, &args![this, other, 1u32]).bool());
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(array_bytes(&e, this), vec![9, 9]);
        // Set byte at +0 and not `on`: cleared, every byte rewritten.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[9, 9, 9], &[5, 3, 5], true, true, false);
        e.mem.set_u8(this, 1);
        assert!(e.call(0x006d_f780, &args![this, other, 0u32]).bool());
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(array_bytes(&e, this), vec![0xff, 1, 0xff]);
        // The fill fails: false.
        let MarkCase {
            mut e, this, other, ..
        } = mark_case(&[9], &[5], false, true, false);
        assert!(!e.call(0x006d_f780, &args![this, other, 1u32]).bool());
    }
}
