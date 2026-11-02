//! `fallout/interface/run_00700480` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007004e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00700500`, which stores the vtable `0106e8dc` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00700500`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_007004e0(e: &mut Engine, this: Ptr) {
    e.call(0x0070_0500, &args![this]);
}

/// The virtual tables of `NiTPrimitiveArray<BGSMenuPacker::PackedMenu *>` and of its base
/// `NiTArray<BGSMenuPacker::PackedMenu *, ...>` (RTTI).
const PRIMITIVE_ARRAY_VTABLE: u32 = 0x0106_e8d4;
const ARRAY_VTABLE: u32 = 0x0106_e8dc;
/// `004ede70` (cdecl, one word): frees an array's block; `0096afc0` (cdecl, one word: the
/// element count): allocates one.
const FREE_BLOCK: u32 = 0x004e_de70;
const ALLOCATE_BLOCK: u32 = 0x0096_afc0;
/// Three 0x1c-byte records from `011d8830` (a byte at +0, words at +8 and +0xc, an object at
/// +0x10 reset by `0066b0d0`, `thiscall` with one word).
const RECORDS: u32 = 0x011d_8830;
const RECORD_RESET: u32 = 0x0066_b0d0;

// Translated from 00700480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTPrimitiveArray<BGSMenuPacker::PackedMenu *>`
/// constructor (`thiscall`, two words: the size and the growth): the array constructor
/// `00700560`, then its own virtual table. Answers `this`.
pub fn fn_00700480(e: &mut Engine, this: Ptr, size: u32, growth: u32) -> Ptr {
    fn_00700560(e, this, size as u16, growth as u16);
    e.mem.set_u32(this.addr(), PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00700500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTArray<BGSMenuPacker::PackedMenu *, ...>` destructor
/// body (`thiscall`, no stack words): its virtual table, then the block at +4 freed
/// (`004ede70`).
pub fn fn_00700500(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    let block = e.mem.u32(this.addr().wrapping_add(4));
    e.call(FREE_BLOCK, &args![block]);
}

// Translated from 00700560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTArray<BGSMenuPacker::PackedMenu *, ...>` constructor
/// (`thiscall`, two words, of which the low 16 bits count): its virtual table, the size at
/// +8 and the growth at +0xe, +0xa and +0xc cleared, and a block for `size` elements
/// (`0096afc0`) at +4, or 0 for size 0. Answers `this`.
pub fn fn_00700560(e: &mut Engine, this: Ptr, size: u16, growth: u16) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, ARRAY_VTABLE);
    e.mem.set_u16(base.wrapping_add(8), size);
    e.mem.set_u16(base.wrapping_add(0xe), growth);
    e.mem.set_u16(base.wrapping_add(0xa), 0);
    e.mem.set_u16(base.wrapping_add(0xc), 0);
    let size = e.mem.u16(base.wrapping_add(8));
    if size > 0 {
        let size = e.mem.u16(base.wrapping_add(8));
        let block = e.call(ALLOCATE_BLOCK, &args![u32::from(size)]).u32();
        e.mem.set_u32(base.wrapping_add(4), block);
    } else {
        e.mem.set_u32(base.wrapping_add(4), 0);
    }
    this
}

/// Clears the three records from `011d8830`: the byte at +0, the word at +8, the `float` at
/// +0xc (`FLDZ`/`FSTP`), and resets the object at +0x10 with 0.
fn clear_records(e: &mut Engine) {
    for index in 0..3u32 {
        let record = RECORDS + index * 0x1c;
        e.mem.set_u8(record, 0);
        e.mem.set_u32(record + 8, 0);
        e.mem.set_u32(record + 0xc, 0.0f32.to_bits());
        e.call(RECORD_RESET, &args![record + 0x10, 0u32]);
    }
}

// Translated from 007005d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears `this` (+0 and +4) and the three records from
/// `011d8830` (`thiscall`, no stack words). Answers `this`.
pub fn fn_007005d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    clear_records(e);
    this
}

// Translated from 00700650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the three records from `011d8830` (`thiscall`, no stack
/// words; `this` is kept but never read).
pub fn fn_00700650(e: &mut Engine, _unused_this: Ptr) {
    clear_records(e);
}

/// `MemoryManager` free (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;

// Translated from 007004b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPrimitiveArray<BGSMenuPacker::PackedMenu *>` scalar deleting destructor
/// (`thiscall`, `RET 4`): the destructor (`007004e0`), then the block freed (`00401030`)
/// when bit 0 of `flags` is set. Answers `this`.
pub fn fn_007004b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007004e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00700530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<BGSMenuPacker::PackedMenu *, ...>` scalar deleting destructor
/// (`thiscall`, `RET 4`): the destructor body (`00700500`), then the block freed when bit
/// 0 of `flags` is set. Answers `this`.
pub fn fn_00700530(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00700500(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_04e0, fn_007004e0(Ptr)),
        entry!(0x0070_0480, fn_00700480(Ptr, u32, u32) -> Ptr),
        entry!(0x0070_0500, fn_00700500(Ptr)),
        entry!(0x0070_0560, fn_00700560(Ptr, u16, u16) -> Ptr),
        entry!(0x0070_05d0, fn_007005d0(Ptr) -> Ptr),
        entry!(0x0070_0650, fn_00700650(Ptr)),
        entry!(0x0070_04b0, fn_007004b0(Ptr, u32) -> Ptr),
        entry!(0x0070_0530, fn_00700530(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
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

    #[test]
    fn fwd_a03_007004e0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0070_0500, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_04e0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0070_0500), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b049_tests {
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

    #[test]
    fn fn_00700480_constructs_the_primitive_array() {
        let mut e = Engine::new();
        e.register(ALLOCATE_BLOCK, |_, _| 0x4400u32.into_ret());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0070_0480, &args![this, 3u32, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 8), 3);
        assert_eq!(e.mem.u16(this + 0xe), 2);
        assert_eq!(e.mem.u32(this + 4), 0x4400);
    }

    #[test]
    fn fn_00700500_frees_the_block() {
        let mut e = Engine::new();
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4400);
        e.call_log = Some(vec![]);
        e.call(0x0070_0500, &args![this]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x4400]]);
    }

    #[test]
    fn fn_00700560_allocates_only_for_a_size() {
        let mut e = Engine::new();
        e.register(ALLOCATE_BLOCK, |_, _| 0x4400u32.into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0070_0560, &args![this, 0x0001_0005u32, 7u32]);
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK), vec![vec![5]]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 7);
        assert_eq!(e.mem.u32(this + 4), 0x4400);
        e.call_log = Some(vec![]);
        e.call(0x0070_0560, &args![this, 0u32, 7u32]);
        assert!(calls_to(&e, ALLOCATE_BLOCK).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    fn records_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_8000, 0x1000);
        e.mem.write(RECORDS, &[0xcd; 0x54]);
        e.register(RECORD_RESET, |_, _| Ret::default());
        e
    }

    fn check_records(e: &Engine) {
        for index in 0..3u32 {
            let record = RECORDS + index * 0x1c;
            assert_eq!(e.mem.u8(record), 0);
            assert_eq!(e.mem.u32(record + 8), 0);
            assert_eq!(e.mem.u32(record + 0xc), 0);
        }
        assert_eq!(
            calls_to(e, RECORD_RESET),
            vec![
                vec![RECORDS + 0x10, 0],
                vec![RECORDS + 0x2c, 0],
                vec![RECORDS + 0x48, 0]
            ]
        );
    }

    #[test]
    fn fn_007005d0_clears_itself_and_the_records() {
        let mut e = records_engine();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_05d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        check_records(&e);
    }

    #[test]
    fn fn_00700650_clears_the_records() {
        let mut e = records_engine();
        e.call_log = Some(vec![]);
        e.call(0x0070_0650, &args![0u32]);
        check_records(&e);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn deleting_destructors_free_on_bit_zero() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4444);
        for (address, frees) in [(0x0070_04b0u32, 1u32), (0x0070_0530, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, frees]).u32(), this);
            assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
            let log = e.call_log.take().unwrap();
            let freed: Vec<_> = log.iter().filter(|(a, _)| *a == FREE).collect();
            assert_eq!(freed.len(), frees as usize);
            assert!(log.iter().any(|(a, w)| *a == FREE_BLOCK && w[0] == 0x4444));
        }
    }
}
