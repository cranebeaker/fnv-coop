//! `NiXenonRenderer/run_00e984d0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chain of 0x20-byte records `00e98610` frees (`0126fa58`, next at +0x18) and the Ni free
/// `00aa1460` (`cdecl`: block, size).
const CHAIN_0126FA58: u32 = 0x0126_fa58;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e98610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`), no arguments: frees every record of the chain
/// at `0126fa58` (next at +0x18 read first, +8 cleared, 0x20 bytes freed through `00aa1460`),
/// then clears the chain's head.
pub fn fn_00e98610(e: &mut Engine) {
    let mut record = e.mem.u32(CHAIN_0126FA58);
    while record != 0 {
        let next = e.mem.u32(record.wrapping_add(0x18));
        e.mem.set_u32(record.wrapping_add(8), 0);
        e.call(NI_FREE_00AA1460, &args![record, 0x20u32]);
        record = next;
    }
    e.mem.set_u32(CHAIN_0126FA58, 0);
}

/// The base table (`010f1644`, `_purecall` in its slots) and the `InterlockedExchange` import.
const BASE_VTABLE_010F1644: u32 = 0x010f_1644;
const IMPORT_INTERLOCKED_EXCHANGE: u32 = 0x00fd_f0e4;

// Translated from 00e986f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): a base constructor (the geometry groups'),
/// `thiscall`, no stack words: the table `010f1644`, the word +4 set to 0 through
/// `InterlockedExchange`, +8 cleared. Answers `this`.
pub fn fn_00e986f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE_010F1644);
    e.call(
        IMPORT_INTERLOCKED_EXCHANGE,
        &args![base.wrapping_add(4), 0u32],
    );
    e.mem.set_u32(base.wrapping_add(8), 0);
    this
}

// Translated from 00e98710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): the base destructor matching `00e986f0`,
/// `thiscall`, no stack words: the table `010f1644`, then the object at +8 released (its
/// slot 8, `stdcall` with the object) when set.
pub fn fn_00e98710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE_010F1644);
    let held = e.mem.u32(base.wrapping_add(8));
    if held != 0 {
        e.vcall(held, 8, &args![]);
    }
}

// Translated from 00e984d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): `cdecl` with three words, the first an object
/// whose +0xc is a function (`cdecl`, five words): it is called with the three words, the
/// address of a local holding 1, and 1; its answer is added to the object's +4 and answered.
pub fn fn_00e984d0(e: &mut Engine, object: u32, first: u32, second: u32) -> u32 {
    let function = e.mem.u32(object.wrapping_add(0xc));
    let answer = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 1);
        e.call(function, &args![object, first, second, local, 1u32])
            .u32()
    });
    let total = e.mem.u32(object.wrapping_add(4)).wrapping_add(answer);
    e.mem.set_u32(object.wrapping_add(4), total);
    answer
}

/// The array set `00ba3aa0` (`thiscall` on the block's chunk array at +0x2c: the index and the
/// address of the new element word).
const ARRAY_SET_00BA3AA0: u32 = 0x00ba_3aa0;

// Translated from 00e98500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): gives a chunk of a buffer block back
/// (`thiscall`, its index in the chunk array +0x30). A missing chunk answers 0. When the chunk
/// ends at the block's high-water mark (+0x20 minus its size +0x14 equals its offset +0xc)
/// the mark is recomputed from the chunks before it (the greatest offset plus size, from 0).
/// The free size (+0x24) becomes the capacity (+0xc) minus the mark, the chunk's size is
/// added to +0x28, the array slot is cleared (`00ba3aa0` with a null word) and the chunk goes
/// to the head of the free chunk chain `0126fa58` (next +0x18, previous +0x1c). Answers the
/// chunk's size.
///
/// The exe keeps the size and then the null word in the index's stack word.
pub fn fn_00e98500(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    let array = e.mem.u32(base.wrapping_add(0x30));
    let chunk = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
    if chunk == 0 {
        return 0;
    }
    let size = e.mem.u32(chunk.wrapping_add(0x14));
    let below = e.mem.u32(base.wrapping_add(0x20)).wrapping_sub(size);
    if below == e.mem.u32(chunk.wrapping_add(0xc)) {
        e.mem.set_u32(base.wrapping_add(0x20), 0);
        let mut slot = array;
        for _ in 0..index {
            let other = e.mem.u32(slot);
            if other != 0 {
                let end = e
                    .mem
                    .u32(other.wrapping_add(0x14))
                    .wrapping_add(e.mem.u32(other.wrapping_add(0xc)));
                if end > e.mem.u32(base.wrapping_add(0x20)) {
                    e.mem.set_u32(base.wrapping_add(0x20), end);
                }
            }
            slot = slot.wrapping_add(4);
        }
    }
    let free = e
        .mem
        .u32(base.wrapping_add(0xc))
        .wrapping_sub(e.mem.u32(base.wrapping_add(0x20)));
    let used = e.mem.u32(base.wrapping_add(0x28)).wrapping_add(size);
    e.mem.set_u32(base.wrapping_add(0x28), used);
    e.mem.set_u32(base.wrapping_add(0x24), free);
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(
            ARRAY_SET_00BA3AA0,
            &args![base.wrapping_add(0x2c), index, word],
        );
    });
    e.mem.set_u32(chunk.wrapping_add(0x1c), 0);
    let head = e.mem.u32(CHAIN_0126FA58);
    e.mem.set_u32(chunk.wrapping_add(0x18), head);
    let head = e.mem.u32(CHAIN_0126FA58);
    if head != 0 {
        e.mem.set_u32(head.wrapping_add(0x1c), chunk);
    }
    e.mem.set_u32(CHAIN_0126FA58, chunk);
    size
}

/// The allocation `00aa13e0` (`cdecl`, size) and the number of chunk records one refill makes
/// (`011c1118`).
const ALLOCATE: u32 = 0x00aa_13e0;
const REFILL_COUNT_011C1118: u32 = 0x011c_1118;

// Translated from 00e985a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): refills the free chunk chain (`cdecl`, no
/// arguments): as many times as the word at `011c1118` says, a 0x20-byte record (`00aa13e0`,
/// its eight words cleared; null when the allocation fails) goes to the head of the chain at
/// `0126fa58` (next +0x18, previous +0x1c).
pub fn fn_00e985a0(e: &mut Engine) {
    let mut made = 0u32;
    while made < e.mem.u32(REFILL_COUNT_011C1118) {
        let block = e.call(ALLOCATE, &args![0x20u32]).u32();
        if block != 0 {
            for offset in [0u32, 4, 8, 0xc, 0x14, 0x10, 0x18, 0x1c] {
                e.mem.set_u32(block.wrapping_add(offset), 0);
            }
        }
        let head = e.mem.u32(CHAIN_0126FA58);
        if head != 0 {
            e.mem.set_u32(head.wrapping_add(0x1c), block);
        }
        let head = e.mem.u32(CHAIN_0126FA58);
        e.mem.set_u32(block.wrapping_add(0x18), head);
        e.mem.set_u32(block.wrapping_add(0x1c), 0);
        made += 1;
        e.mem.set_u32(CHAIN_0126FA58, block);
    }
}

/// The `NiTArray` table the chunk array member gets back (`010f0f7c`, RTTI
/// `NiTArray<NiVBChip *, ...>`) and the free `00aa10f0` (`cdecl`, a block).
const CHIP_ARRAY_VTABLE_010F0F7C: u32 = 0x010f_0f7c;
const FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00e98640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): the destructor body of a buffer block's chunk
/// array member (`thiscall`): the member at +0x2c gets the `NiTArray` table `010f0f7c` and its
/// elements (+0x30) are freed (`00aa10f0`).
pub fn fn_00e98640(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let elements = e.mem.u32(base.wrapping_add(0x30));
    e.mem
        .set_u32(base.wrapping_add(0x2c), CHIP_ARRAY_VTABLE_010F0F7C);
    e.call(FREE_00AA10F0, &args![elements]);
}

/// The array add `00a64330` (`thiscall` on the chunk array at +0x2c, the address of the chunk
/// word; answers its index; the engine map's `NiUnionBV::AddBoundingVolume` is a folded body).
const ARRAY_ADD_00A64330: u32 = 0x00a6_4330;

// Translated from 00e98660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): takes a chunk of `size` bytes from a buffer
/// block (`thiscall`). When the free size (+0x24) is smaller it answers 0. A record comes off
/// the free chunk chain (refilled by `00e985a0` when empty) and gets the block (+4), the
/// block's buffer (+8), the offset (the block's mark +0x20, at +0xc) and the size (+0x14); the
/// mark grows by the size, the free size (+0x24) and +0x28 shrink by it, and the chunk's index
/// in the block's chunk array (`00a64330`) goes to its +0. Answers the chunk.
///
/// The exe's chunk word for the array add is the size argument's stack word.
pub fn fn_00e98660(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x24)) < size {
        return 0;
    }
    let mut chunk = e.mem.u32(CHAIN_0126FA58);
    if chunk == 0 {
        fn_00e985a0(e);
        chunk = e.mem.u32(CHAIN_0126FA58);
    }
    let next = e.mem.u32(chunk.wrapping_add(0x18));
    e.mem.set_u32(CHAIN_0126FA58, next);
    if next != 0 {
        e.mem.set_u32(next.wrapping_add(0x1c), 0);
    }
    e.mem.set_u32(chunk.wrapping_add(0x18), 0);
    e.mem.set_u32(chunk.wrapping_add(0x1c), 0);
    e.mem.set_u32(chunk.wrapping_add(4), base);
    let buffer = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(chunk.wrapping_add(8), buffer);
    let offset = e.mem.u32(base.wrapping_add(0x20));
    e.mem.set_u32(chunk.wrapping_add(0xc), offset);
    e.mem.set_u32(chunk.wrapping_add(0x14), size);
    for (at, grows) in [(0x20u32, true), (0x24, false), (0x28, false)] {
        let value = e.mem.u32(base.wrapping_add(at));
        let value = if grows {
            value.wrapping_add(size)
        } else {
            value.wrapping_sub(size)
        };
        e.mem.set_u32(base.wrapping_add(at), value);
    }
    let index = e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), chunk);
        e.call(ARRAY_ADD_00A64330, &args![base.wrapping_add(0x2c), word])
            .u32()
    });
    e.mem.set_u32(chunk, index);
    chunk
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_84d0, fn_00e984d0(u32, u32, u32) -> u32),
        entry!(0x00e9_8500, fn_00e98500(Ptr, u32) -> u32),
        entry!(0x00e9_85a0, fn_00e985a0()),
        entry!(0x00e9_8640, fn_00e98640(Ptr)),
        entry!(0x00e9_8660, fn_00e98660(Ptr, u32) -> u32),
        entry!(0x00e9_8610, fn_00e98610()),
        entry!(0x00e9_86f0, fn_00e986f0(Ptr) -> Ptr),
        entry!(0x00e9_8710, fn_00e98710(Ptr)),
    ]
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e98610_frees_the_chain() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x18, second);
        e.mem.set_u32(first + 8, 1);
        e.mem.set_u32(second + 8, 2);
        e.mem.set_u32(CHAIN_0126FA58, first);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8610, &args![]);
        assert_eq!(
            calls(&e, NI_FREE_00AA1460),
            vec![vec![first, 0x20], vec![second, 0x20]]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(e.mem.u32(CHAIN_0126FA58), 0);
    }

    #[test]
    fn fn_00e986f0_and_00e98710_build_and_release() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_EXCHANGE, |_, _| Ret::default());
        e.register(0x0e50_0008, |_, _| Ret::default());
        e.put_vtable(0x0e51_0000, &[0, 0, 0x0e50_0008]);
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xaa; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_86f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_010F1644);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_EXCHANGE),
            vec![vec![this + 4, 0]]
        );
        // Nothing held: no release.
        e.call(0x00e9_8710, &args![this]);
        assert!(calls(&e, 0x0e50_0008).is_empty());
        let held = e.mem.alloc(4);
        e.mem.set_u32(held, 0x0e51_0000);
        e.mem.set_u32(this + 8, held);
        e.mem.set_u32(this, 0);
        e.call(0x00e9_8710, &args![this]);
        assert_eq!(calls(&e, 0x0e50_0008), vec![vec![held]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_010F1644);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e984d0_calls_the_function_and_adds_its_answer() {
        let mut e = Engine::new();
        e.register(0x0e50_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            (a[1] + a[2]).into_ret()
        });
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, 10);
        e.mem.set_u32(object + 0xc, 0x0e50_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_84d0, &args![object, 3u32, 4u32]).u32(), 7);
        assert_eq!(e.mem.u32(object + 4), 17);
        let call = &calls(&e, 0x0e50_0000)[0];
        assert_eq!([call[0], call[1], call[2], call[4]], [object, 3, 4, 1]);
    }

    fn chunk(e: &mut Engine, offset: u32, size: u32) -> u32 {
        let chunk = e.mem.alloc(0x20);
        e.mem.set_u32(chunk + 0xc, offset);
        e.mem.set_u32(chunk + 0x14, size);
        chunk
    }

    #[test]
    fn fn_00e98500_returns_a_chunk_and_lowers_the_mark() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(ARRAY_SET_00BA3AA0, |_, _| Ret::default());
        let first = chunk(&mut e, 0, 0x10);
        let second = chunk(&mut e, 0x10, 0x20);
        let last = chunk(&mut e, 0x30, 0x10);
        let array = e.mem.alloc(0xc);
        for (i, c) in [first, second, last].iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0xc, 0x100);
        e.mem.set_u32(this + 0x20, 0x40);
        e.mem.set_u32(this + 0x28, 5);
        e.mem.set_u32(this + 0x30, array);
        let old = e.mem.alloc(0x20);
        e.mem.set_u32(CHAIN_0126FA58, old);
        e.call_log = Some(vec![]);
        // The last chunk ends at the mark: the mark drops to 0x30.
        assert_eq!(e.call(0x00e9_8500, &args![this, 2u32]).u32(), 0x10);
        assert_eq!(e.mem.u32(this + 0x20), 0x30);
        assert_eq!(e.mem.u32(this + 0x24), 0xd0);
        assert_eq!(e.mem.u32(this + 0x28), 0x15);
        let set = &calls(&e, ARRAY_SET_00BA3AA0)[0];
        assert_eq!(&set[..2], &[this + 0x2c, 2]);
        assert_eq!(e.mem.u32(CHAIN_0126FA58), last);
        assert_eq!(e.mem.u32(last + 0x18), old);
        assert_eq!(e.mem.u32(old + 0x1c), last);
        // A chunk in the middle keeps the mark; a missing one answers 0.
        assert_eq!(e.call(0x00e9_8500, &args![this, 0u32]).u32(), 0x10);
        assert_eq!(e.mem.u32(this + 0x20), 0x30);
        e.mem.set_u32(array + 4, 0);
        assert_eq!(e.call(0x00e9_8500, &args![this, 1u32]).u32(), 0);
    }

    #[test]
    fn fn_00e98660_takes_a_chunk_and_refills_the_chain() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(REFILL_COUNT_011C1118 & !0xfff, 0x1000);
        e.mem.set_u32(REFILL_COUNT_011C1118, 2);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ARRAY_ADD_00A64330, |_, _| 4u32.into_ret());
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 8, 0x0e0b_0000);
        e.mem.set_u32(this + 0x20, 0x40);
        e.mem.set_u32(this + 0x24, 0x80);
        e.mem.set_u32(this + 0x28, 0x90);
        e.call_log = Some(vec![]);
        // Too big: 0.
        assert_eq!(e.call(0x00e9_8660, &args![this, 0x81u32]).u32(), 0);
        let taken = e.call(0x00e9_8660, &args![this, 0x20u32]).u32();
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
        assert_eq!(e.mem.u32(taken), 4);
        assert_eq!(e.mem.u32(taken + 4), this);
        assert_eq!(e.mem.u32(taken + 8), 0x0e0b_0000);
        assert_eq!(e.mem.u32(taken + 0xc), 0x40);
        assert_eq!(e.mem.u32(taken + 0x14), 0x20);
        assert_eq!(e.mem.u32(this + 0x20), 0x60);
        assert_eq!(e.mem.u32(this + 0x24), 0x60);
        assert_eq!(e.mem.u32(this + 0x28), 0x70);
        // One record is left on the chain, with no previous link.
        let rest = e.mem.u32(CHAIN_0126FA58);
        assert_ne!(rest, 0);
        assert_eq!(e.mem.u32(rest + 0x1c), 0);
        assert_eq!(e.mem.u32(rest + 0x18), 0);
        assert_eq!(calls(&e, ARRAY_ADD_00A64330)[0][0], this + 0x2c);
    }

    #[test]
    fn fn_00e98640_frees_the_chunk_array() {
        let mut e = Engine::new();
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 0x0e0c_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8640, &args![this]);
        assert_eq!(e.mem.u32(this + 0x2c), CHIP_ARRAY_VTABLE_010F0F7C);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e0c_0000]]);
    }
}
