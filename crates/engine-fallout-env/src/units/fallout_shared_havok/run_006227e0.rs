//! `fallout shared/havok/run_006227e0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `00622870` stores (its first slot is `00622810`).
const BASE_ARRAY_VTABLE: u32 = 0x0104_dc94;
/// The vtable `006227e0` stores (its first slot is `00622840`).
const ARRAY_VTABLE: u32 = 0x0104_dc9c;
/// `00622780` (`run_00622470.rs`): `thiscall`, plain `RET`; stores `0104dc94` and releases the
/// block at `+4`.
const BASE_ARRAY_DESTRUCT: u32 = 0x0062_2780;
/// `00621e40` (`fallout shared/havok/cellmopp.cpp` in the engine map): `thiscall`, plain `RET`.
const ARRAY_DESTRUCT: u32 = 0x0062_1e40;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete of a deleting
/// destructor.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (Xbox PDB), `005e0ba0`: `cdecl`, the
/// element count.
const ALLOCATE_POINTERS: u32 = 0x005e_0ba0;
/// `0060bef0` (unnamed in the engine map): `thiscall`, `RET 4`, the new capacity.
const ARRAY_SET_SIZE: u32 = 0x0060_bef0;
/// `00633c90` (unnamed in the engine map): `thiscall`, `RET 4`; constructs a smart pointer from
/// a word.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `_atexit`, `00ec658f`: `cdecl`, one function address.
const ATEXIT: u32 = 0x00ec_658f;
/// The destructor `00622920` gives to `_atexit` for its static null pointer.
const NULL_POINTER_DESTRUCT: u32 = 0x00fc_b910;
/// `00631820` (`fallout shared/havok/teswaterlistener.cpp` in the engine map): `thiscall` on a
/// smart pointer, `RET 4`, another smart pointer; answers a byte (whether they differ, as
/// `00622920` uses it).
const POINTER_DIFFERS: u32 = 0x0063_1820;
/// `004b0460` (unnamed in the engine map): `thiscall` on a smart pointer, `RET 4`, another smart
/// pointer; answers a byte.
const POINTER_EQUALS: u32 = 0x004b_0460;
/// `NiPointer<PathingDebugGeometryData>::operator=` (Xbox PDB), `006e5cc0`: `thiscall`, `RET 4`.
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// The static null smart pointer `00622920` compares with.
const NULL_POINTER: u32 = 0x011c_c1f4;
/// The guard word of [`NULL_POINTER`]'s construction (bit 0).
const NULL_POINTER_GUARD: u32 = 0x011c_c1f8;
/// `00553300` (`fallout shared/tesobjectcell.cpp` in the engine map): `cdecl`, the size; the
/// allocation.
const ALLOCATE: u32 = 0x0055_3300;
/// `005dbe10` (`fallout shared/tesscriptfunctions.cpp` in the engine map): no arguments.
const CASTER_SOURCE: u32 = 0x005d_be10;
/// `0087e9b0` (`fallout/ai/actor.cpp` in the engine map): `cdecl`, one word; the delete of
/// `00622c50`.
const CASTER_DELETE: u32 = 0x0087_e9b0;
/// `00620ff0` (`fallout shared/havok/cameracaster.cpp` in the engine map): `thiscall`, plain
/// `RET`; the base constructor.
const CASTER_BASE_CONSTRUCT: u32 = 0x0062_0ff0;
/// `DecalCaster::Init` (Xbox PDB), `00622d60`: `thiscall`, `RET 8`, a `float` and a word.
const DECAL_CASTER_INIT: u32 = 0x0062_2d60;
/// `0045cec0` (unnamed in the engine map): `thiscall`, plain `RET`; the smart pointer's
/// destructor.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `006211b0` (Xbox PDB name `hkpAllCdPointCollector::~hkpAllCdPointCollector`, possibly a
/// folded body): `thiscall`, plain `RET`; the base destructor.
const CASTER_BASE_DESTRUCT: u32 = 0x0062_11b0;
/// The cached caster for sizes up to the `00622ba0` size.
const SMALL_CASTER: u32 = 0x011c_c1fc;
/// The cached caster for larger sizes.
const LARGE_CASTER: u32 = 0x011c_c200;
/// The `float` `00622ba0` answers (`32.0` in the exe).
const SMALL_SIZE: u32 = 0x0101_e340;
/// The `float` `00622bb0` answers (`256.0` in the exe).
const LARGE_SIZE: u32 = 0x0103_2adc;
/// Offset of the caster's smart pointer.
const CASTER_POINTER_OFFSET: u32 = 0x3a0;

fn add_u16(e: &mut Engine, addr: u32, delta: u16) {
    let value = e.mem.u16(addr);
    e.mem.set_u16(addr, value.wrapping_add(delta));
}

// Translated from 00622870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8` (the low halves of two words). The pointer
/// array's base constructor: stores the vtable `0104dc94`, the capacity at `+8` and the growth
/// step at `+0xe`, clears the used size at `+0xa` and the count at `+0xc`, and allocates the
/// block at `+4` for the capacity (`005e0ba0`), or stores 0 for none.
pub fn fn_00622870(e: &mut Engine, this: Ptr, capacity: u16, grow_by: u16) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_ARRAY_VTABLE);
    e.mem.set_u16(base.wrapping_add(8), capacity);
    e.mem.set_u16(base.wrapping_add(0xe), grow_by);
    e.mem.set_u16(base.wrapping_add(0xa), 0);
    e.mem.set_u16(base.wrapping_add(0xc), 0);
    let stored = e.mem.u16(base.wrapping_add(8));
    if stored == 0 {
        e.mem.set_u32(base.wrapping_add(4), 0);
    } else {
        let block = e.call(ALLOCATE_POINTERS, &args![u32::from(stored)]).u32();
        e.mem.set_u32(base.wrapping_add(4), block);
    }
    this
}

// Translated from 006227e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. Runs the base constructor `00622870` with
/// both words and stores the vtable `0104dc9c`. Answers `this`.
pub fn fn_006227e0(e: &mut Engine, this: Ptr, capacity: u32, grow_by: u32) -> Ptr {
    fn_00622870(e, this, capacity as u16, grow_by as u16);
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    this
}

// Translated from 00622810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the first slot of the vtable `0104dc94`. `thiscall`, `RET 4`. The
/// deleting destructor: runs `00622780` on `this` and, with bit 0 of `flags`, deletes it
/// (`00401030`). Answers `this`.
pub fn fn_00622810(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BASE_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00622840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the first slot of the vtable `0104dc9c`. `thiscall`, `RET 4`. The
/// deleting destructor: runs `00621e40` on `this` and, with bit 0 of `flags`, deletes it
/// (`00401030`). Answers `this`.
pub fn fn_00622840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00622920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622920 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`: sets the smart pointer at `index` to the one
/// at `value`. On first use constructs the static null pointer `011cc1f4` (`00633c90` with 0)
/// and registers its destructor `00fcb910` (`_atexit`). At or past the used size (`+0xa`) the
/// used size becomes `index + 1` and the count (`+0xc`) grows when `value` is not null
/// (`00631820`). Below it, a non-null `value` grows the count when the slot is null (`004b0460`);
/// a null `value` shrinks it when the slot is not null (`00631820`). Then assigns (`006e5cc0`).
pub fn fn_00622920(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let guard = e.mem.u32(NULL_POINTER_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(NULL_POINTER_GUARD, guard | 1);
        e.call(POINTER_CONSTRUCT, &args![NULL_POINTER, 0u32]);
        e.call(ATEXIT, &args![NULL_POINTER_DESTRUCT]);
    }
    let base = this.addr();
    let count = base.wrapping_add(0xc);
    let slot_of = |e: &Engine| {
        e.mem
            .u32(base.wrapping_add(4))
            .wrapping_add(index.wrapping_mul(4))
    };
    if index < u32::from(e.mem.u16(base.wrapping_add(0xa))) {
        if e.call(POINTER_DIFFERS, &args![value, NULL_POINTER]).u8() != 0 {
            let slot = slot_of(e);
            if e.call(POINTER_EQUALS, &args![slot, NULL_POINTER]).u8() != 0 {
                add_u16(e, count, 1);
            }
        } else {
            let slot = slot_of(e);
            if e.call(POINTER_DIFFERS, &args![slot, NULL_POINTER]).u8() != 0 {
                add_u16(e, count, 0xffff);
            }
        }
    } else {
        e.mem
            .set_u16(base.wrapping_add(0xa), index.wrapping_add(1) as u16);
        if e.call(POINTER_DIFFERS, &args![value, NULL_POINTER]).u8() != 0 {
            add_u16(e, count, 1);
        }
    }
    let slot = slot_of(e);
    e.call(POINTER_ASSIGN, &args![slot, value]);
}

// Translated from 006228e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. When `index` is at or past the capacity
/// (`+8`), grows the array to `index` plus the growth step (`+0xe`, `0060bef0`); then sets the
/// slot (`00622920`). Answers `index`.
pub fn fn_006228e0(e: &mut Engine, this: Ptr, index: u32, value: u32) -> u32 {
    if index >= u32::from(e.mem.u16(this.addr().wrapping_add(8))) {
        let grow_by = u32::from(e.mem.u16(this.addr().wrapping_add(0xe)));
        e.call(ARRAY_SET_SIZE, &args![this, grow_by.wrapping_add(index)]);
    }
    fn_00622920(e, this, index, value);
    index
}

// Translated from 00622ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments: answers the `float` at `0101e340` in `ST0`.
pub fn fn_00622ba0(e: &mut Engine) -> f64 {
    x87_load32(e.mem.u32(SMALL_SIZE))
}

// Translated from 00622bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments: answers the `float` at `01032adc` in `ST0`.
pub fn fn_00622bb0(e: &mut Engine) -> f64 {
    x87_load32(e.mem.u32(LARGE_SIZE))
}

// Translated from 00622c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622c80 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`, a `float` and a word. The decal caster's
/// constructor: the base (`00620ff0`), the smart pointer at `+0x3a0` from 0 (`00633c90`), then
/// `DecalCaster::Init` with `radius` and `source`. Answers `this`.
pub fn fn_00622c80(e: &mut Engine, this: Ptr, radius: f32, source: u32) -> Ptr {
    e.call(CASTER_BASE_CONSTRUCT, &args![this]);
    let pointer = this.addr().wrapping_add(CASTER_POINTER_OFFSET);
    e.call(POINTER_CONSTRUCT, &args![pointer, 0u32]);
    e.call(
        DECAL_CASTER_INIT,
        &args![this, x87_quiet_bits(radius.to_bits()), source],
    );
    this
}

// Translated from 00622a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622a70 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `cdecl`, a `float` and a byte. Answers the cached decal caster
/// for `size`: above the `00622ba0` size (`FCOMPP`, `TEST AH,0x41`: not below, equal or
/// unordered) the large one at `011cc200`, otherwise (NaN included) the small one at
/// `011cc1fc`. When the cache is empty and `create` is set, allocates 0x3b0 bytes (`00553300`)
/// and constructs a caster in them (`00622c80`, with the `00622bb0` or `00622ba0` size rounded to
/// a `float` and what `005dbe10` answers), or stores 0 when the allocation fails.
pub fn fn_00622a70(e: &mut Engine, size: f32, create: u8) -> u32 {
    let limit = fn_00622ba0(e);
    let value = f64::from(x87_quiet(size));
    let (cache, large) = if value > limit {
        (LARGE_CASTER, true)
    } else {
        (SMALL_CASTER, false)
    };
    if e.mem.u32(cache) == 0 && create != 0 {
        let block = e.call(ALLOCATE, &args![0x3b0u32]).u32();
        let caster = if block == 0 {
            0
        } else {
            let source = e.call(CASTER_SOURCE, &args![]).u32();
            let radius = if large {
                fn_00622bb0(e)
            } else {
                fn_00622ba0(e)
            };
            let radius = f32::from_bits(x87_store32(radius));
            fn_00622c80(e, Ptr::new(block), radius, source).addr()
        };
        e.mem.set_u32(cache, caster);
    }
    e.mem.u32(cache)
}

// Translated from 00622d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00622d00 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `fastcall`, plain `RET`. The decal caster's destructor: the smart
/// pointer at `+0x3a0` (`0045cec0`), then the base (`006211b0`).
pub fn fn_00622d00(e: &mut Engine, this: Ptr) {
    let pointer = this.addr().wrapping_add(CASTER_POINTER_OFFSET);
    e.call(POINTER_DESTRUCT, &args![pointer]);
    e.call(CASTER_BASE_DESTRUCT, &args![this]);
}

// Translated from 00622c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. The decal caster's deleting destructor:
/// `00622d00`, then with bit 0 of `flags` the delete `0087e9b0`. Answers `this`.
pub fn fn_00622c50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00622d00(e, this);
    if flags & 1 != 0 {
        e.call(CASTER_DELETE, &args![this]);
    }
    this
}

// Translated from 00622bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments. Deletes the cached large caster (`011cc200`), then
/// the small one (`011cc1fc`), each through `00622c50` with 1 when set, and clears both.
pub fn fn_00622bc0(e: &mut Engine) {
    for cache in [LARGE_CASTER, SMALL_CASTER] {
        let caster = e.mem.u32(cache);
        if caster != 0 {
            fn_00622c50(e, Ptr::new(caster), 1);
        }
        e.mem.set_u32(cache, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_27e0, fn_006227e0(Ptr, u32, u32) -> Ptr),
        entry!(0x0062_2810, fn_00622810(Ptr, u32) -> Ptr),
        entry!(0x0062_2840, fn_00622840(Ptr, u32) -> Ptr),
        entry!(0x0062_2870, fn_00622870(Ptr, u16, u16) -> Ptr),
        entry!(0x0062_28e0, fn_006228e0(Ptr, u32, u32) -> u32),
        entry!(0x0062_2920, fn_00622920(Ptr, u32, u32)),
        entry!(0x0062_2a70, fn_00622a70(f32, u8) -> u32),
        entry!(0x0062_2ba0, fn_00622ba0() -> f64),
        entry!(0x0062_2bb0, fn_00622bb0() -> f64),
        entry!(0x0062_2bc0, fn_00622bc0()),
        entry!(0x0062_2c50, fn_00622c50(Ptr, u32) -> Ptr),
        entry!(0x0062_2c80, fn_00622c80(Ptr, f32, u32) -> Ptr),
        entry!(0x0062_2d00, fn_00622d00(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(0x011c_c000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0103_2000, 0x1000);
        e.set_global(SMALL_SIZE, 32.0f32);
        e.set_global(LARGE_SIZE, 256.0f32);
        e
    }

    #[test]
    fn at_00622870_allocates_the_block_for_the_capacity() {
        let mut e = engine();
        e.register(ALLOCATE_POINTERS, |_, _| 0x9000u32.into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xa, 0xffff_ffff);
        // Only the low halves of the words are kept.
        let answer = e.call(0x0062_2870, &args![this, 0x1_0005u32, 0x2_0003u32]);
        assert_eq!(answer.u32(), this);
        assert_eq!(e.mem.u32(this), BASE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x9000);
        assert_eq!(e.mem.u16(this + 8), 5);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 3);
        assert_eq!(calls(&e, ALLOCATE_POINTERS), vec![vec![5]]);
    }

    #[test]
    fn at_006227e0_without_capacity_allocates_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x1234);
        e.call(0x0062_27e0, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
        assert!(calls(&e, ALLOCATE_POINTERS).is_empty());
    }

    #[test]
    fn at_00622810_and_00622840_delete_only_with_bit_0() {
        let mut e = engine();
        e.register(BASE_ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_2810, &args![0x4000u32, 2u32]).u32(), 0x4000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0062_2810, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, BASE_ARRAY_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
        assert_eq!(e.call(0x0062_2840, &args![0x5000u32, 3u32]).u32(), 0x5000);
        assert_eq!(calls(&e, ARRAY_DESTRUCT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, OPERATOR_DELETE)[1], vec![0x5000]);
        assert!(calls(&e, BASE_ARRAY_DESTRUCT)
            .iter()
            .all(|a| a[0] == 0x4000));
    }

    /// An array whose block is `0x9000`, with `used` slots in use and a count of 5; smart
    /// pointers are null when their word is 0.
    fn array(e: &mut Engine, used: u16) -> u32 {
        e.register(POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(POINTER_DIFFERS, |e, a| (e.mem.u32(a[0]) != 0).into_ret());
        e.register(POINTER_EQUALS, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.map(0x9000, 0x100);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x9000);
        e.mem.set_u16(this + 8, 8);
        e.mem.set_u16(this + 0xa, used);
        e.mem.set_u16(this + 0xc, 5);
        this
    }

    fn pointer(e: &mut Engine, word: u32) -> u32 {
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, word);
        p
    }

    #[test]
    fn at_00622920_constructs_the_null_pointer_once() {
        let mut e = engine();
        let this = array(&mut e, 4);
        let value = pointer(&mut e, 0);
        e.call(0x0062_2920, &args![this, 1u32, value]);
        e.call(0x0062_2920, &args![this, 1u32, value]);
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![NULL_POINTER, 0]]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![NULL_POINTER_DESTRUCT]]);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD), 1);
    }

    #[test]
    fn at_00622920_past_the_used_size_extends_it_and_counts_a_value() {
        let mut e = engine();
        let this = array(&mut e, 2);
        let value = pointer(&mut e, 0x77);
        e.call(0x0062_2920, &args![this, 5u32, value]);
        assert_eq!(e.mem.u16(this + 0xa), 6);
        assert_eq!(e.mem.u16(this + 0xc), 6);
        assert_eq!(e.mem.u32(0x9000 + 5 * 4), 0x77);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![0x9014, value]]);
        let null = pointer(&mut e, 0);
        e.call(0x0062_2920, &args![this, 6u32, null]);
        assert_eq!(e.mem.u16(this + 0xa), 7);
        assert_eq!(e.mem.u16(this + 0xc), 6, "a null value is not counted");
    }

    #[test]
    fn at_00622920_below_the_used_size_counts_filled_and_emptied_slots() {
        let mut e = engine();
        let this = array(&mut e, 4);
        let value = pointer(&mut e, 0x77);
        let null = pointer(&mut e, 0);
        // Null slot, value: the count grows.
        e.call(0x0062_2920, &args![this, 1u32, value]);
        assert_eq!(e.mem.u16(this + 0xc), 6);
        // Filled slot, value: no change.
        e.call(0x0062_2920, &args![this, 1u32, value]);
        assert_eq!(e.mem.u16(this + 0xc), 6);
        // Filled slot, null: the count shrinks.
        e.call(0x0062_2920, &args![this, 1u32, null]);
        assert_eq!(e.mem.u16(this + 0xc), 5);
        // Null slot, null: no change.
        e.call(0x0062_2920, &args![this, 1u32, null]);
        assert_eq!(e.mem.u16(this + 0xc), 5);
        assert_eq!(e.mem.u16(this + 0xa), 4, "the used size is kept");
        assert_eq!(
            calls(&e, POINTER_EQUALS),
            vec![vec![0x9004, NULL_POINTER]; 2]
        );
    }

    #[test]
    fn at_006228e0_grows_only_at_or_past_the_capacity() {
        let mut e = engine();
        let this = array(&mut e, 0);
        e.mem.set_u16(this + 0xe, 4);
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        let value = pointer(&mut e, 0);
        assert_eq!(e.call(0x0062_28e0, &args![this, 7u32, value]).u32(), 7);
        assert!(calls(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(e.call(0x0062_28e0, &args![this, 8u32, value]).u32(), 8);
        assert_eq!(calls(&e, ARRAY_SET_SIZE), vec![vec![this, 12]]);
        assert_eq!(calls(&e, POINTER_ASSIGN).len(), 2);
    }

    #[test]
    fn at_00622ba0_and_00622bb0_answer_their_constants() {
        let mut e = engine();
        assert_eq!(e.call(0x0062_2ba0, &args![]).f64(), 32.0);
        assert_eq!(e.call(0x0062_2bb0, &args![]).f64(), 256.0);
    }

    fn caster_doubles(e: &mut Engine, block: u32) {
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(CASTER_SOURCE, |_, _| 0x6600u32.into_ret());
        e.register(CASTER_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(DECAL_CASTER_INIT, |_, _| Ret::default());
    }

    #[test]
    fn at_00622a70_creates_the_small_caster_up_to_the_limit_and_for_nan() {
        let mut e = engine();
        caster_doubles(&mut e, 0x8000);
        assert_eq!(e.call(0x0062_2a70, &args![10.0f32, 0u32]).u32(), 0);
        assert!(calls(&e, ALLOCATE).is_empty(), "no create: nothing made");
        assert_eq!(e.call(0x0062_2a70, &args![32.0f32, 1u32]).u32(), 0x8000);
        assert_eq!(e.mem.u32(SMALL_CASTER), 0x8000);
        assert_eq!(e.mem.u32(LARGE_CASTER), 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x3b0]]);
        assert_eq!(
            calls(&e, DECAL_CASTER_INIT),
            vec![vec![0x8000, 32.0f32.to_bits(), 0x6600]]
        );
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![0x83a0, 0]]);
        // NaN takes the small cache, already filled.
        assert_eq!(e.call(0x0062_2a70, &args![f32::NAN, 1u32]).u32(), 0x8000);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn at_00622a70_creates_the_large_caster_above_the_limit() {
        let mut e = engine();
        caster_doubles(&mut e, 0x8000);
        assert_eq!(e.call(0x0062_2a70, &args![33.0f32, 1u32]).u32(), 0x8000);
        assert_eq!(e.mem.u32(LARGE_CASTER), 0x8000);
        assert_eq!(e.mem.u32(SMALL_CASTER), 0);
        assert_eq!(
            calls(&e, DECAL_CASTER_INIT),
            vec![vec![0x8000, 256.0f32.to_bits(), 0x6600]]
        );
    }

    #[test]
    fn at_00622a70_stores_null_when_the_allocation_fails() {
        let mut e = engine();
        caster_doubles(&mut e, 0);
        assert_eq!(e.call(0x0062_2a70, &args![100.0f32, 1u32]).u32(), 0);
        assert!(calls(&e, CASTER_SOURCE).is_empty());
        // Still empty: the next call allocates again.
        e.call(0x0062_2a70, &args![100.0f32, 1u32]);
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
    }

    #[test]
    fn at_00622c80_quiets_a_signalling_radius() {
        let mut e = engine();
        caster_doubles(&mut e, 0);
        let radius = f32::from_bits(0x7f80_0001);
        let answer = e.call(0x0062_2c80, &args![0x8000u32, radius, 7u32]);
        assert_eq!(answer.u32(), 0x8000);
        assert_eq!(calls(&e, CASTER_BASE_CONSTRUCT), vec![vec![0x8000]]);
        assert_eq!(
            calls(&e, DECAL_CASTER_INIT),
            vec![vec![0x8000, 0x7fc0_0001, 7]]
        );
    }

    #[test]
    fn at_00622bc0_deletes_and_clears_both_casters() {
        let mut e = engine();
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_DELETE, |_, _| Ret::default());
        e.mem.set_u32(LARGE_CASTER, 0x8000);
        e.call(0x0062_2bc0, &args![]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![0x83a0]]);
        assert_eq!(calls(&e, CASTER_BASE_DESTRUCT), vec![vec![0x8000]]);
        assert_eq!(calls(&e, CASTER_DELETE), vec![vec![0x8000]]);
        e.mem.set_u32(SMALL_CASTER, 0x9000);
        e.call(0x0062_2bc0, &args![]);
        assert_eq!(calls(&e, CASTER_DELETE)[1], vec![0x9000]);
        assert_eq!(e.mem.u32(LARGE_CASTER), 0);
        assert_eq!(e.mem.u32(SMALL_CASTER), 0);
    }

    #[test]
    fn at_00622c50_deletes_only_with_bit_0() {
        let mut e = engine();
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_2c50, &args![0x8000u32, 0u32]).u32(), 0x8000);
        assert_eq!(calls(&e, CASTER_BASE_DESTRUCT), vec![vec![0x8000]]);
        assert!(calls(&e, CASTER_DELETE).is_empty());
    }

    #[test]
    fn at_00622d00_destroys_the_pointer_then_the_base() {
        let mut e = engine();
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_BASE_DESTRUCT, |_, _| Ret::default());
        e.call(0x0062_2d00, &args![0x8000u32]);
        let log: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            log,
            vec![0x0062_2d00, POINTER_DESTRUCT, CASTER_BASE_DESTRUCT]
        );
    }
}
