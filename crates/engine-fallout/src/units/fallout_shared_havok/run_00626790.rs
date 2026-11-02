//! `fallout shared/havok/run_00626790` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00626790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(unused, count)`: a loop over `0..count` with an
/// empty body; it does nothing.
pub fn fn_00626790(_e: &mut Engine, _unused_0: u32, _count: i32) {}

// Translated from 006267c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__cdecl(destination, source)`: copies two words from
/// `source` to `destination`.
pub fn fn_006267c0(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..2 {
        let word = e.mem.u32(source + i * 4);
        e.mem.set_u32(destination + i * 4, word);
    }
}

/// `004ae700` (`run_004ae6a0.rs`): `__fastcall`; the object's character proxy field
/// (`bhkCharacterProxy::operatorP` plus `+0x10`), or zero without a proxy.
const CHARACTER_PROXY_FIELD: u32 = 0x004a_e700;
/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `00403550` (unnamed in the engine map): `thiscall`, `RET 4`; stores its word at `+8` of `this`.
const STORE_WORD_AT_8: u32 = 0x0040_3550;
/// Vtable slot (byte offset) `this` is called through at the end of `00626cb0`.
const SLOT_C4: u32 = 0xc4;

// Translated from 00626d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout shared/havok`). `thiscall`, `RET 4`: stores its word in
/// the member at `this + 0x14` through `00403550` (which writes `+8` of that member).
pub fn fn_00626d00(e: &mut Engine, this: Ptr, value: u32) {
    e.call(
        STORE_WORD_AT_8,
        &args![this.addr().wrapping_add(0x14), value],
    );
}

// Translated from 00626cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`, `RET 4`: when `004ae700` finds a proxy field for
/// `this`, reads the stack argument back through `00559450` (the exe passes the address of the
/// argument word) and hands it to `00626d00` for that field; then calls virtual slot `+0xc4` of
/// `this` with no arguments.
pub fn fn_00626cb0(e: &mut Engine, this: Ptr, held: u32) {
    let proxy_field = e.call(CHARACTER_PROXY_FIELD, &args![this]).u32();
    if proxy_field != 0 {
        let value = e.with_stack(4, |e, word| {
            e.mem.set_u32(word.addr(), held);
            e.call(READ_FIRST_WORD, &args![word]).u32()
        });
        fn_00626d00(e, Ptr::new(proxy_field), value);
    }
    e.vcall(this.addr(), SLOT_C4, &args![]);
}

/// `006e6da0` (unnamed in the engine map): `cdecl`, the size and the address; the placement
/// `new` of an element (answers the address, or 0).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00620ff0` (`fallout shared/havok/cameracaster.cpp` in the engine map): `thiscall`, plain
/// `RET`; the caster base constructor.
const CASTER_BASE_CONSTRUCT: u32 = 0x0062_0ff0;
/// `00633c90` (unnamed in the engine map): `thiscall`, `RET 4`; constructs a smart pointer from
/// a word.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0066b0d0` (unnamed in the engine map): `thiscall` on a smart pointer, `RET 4`; assigns a
/// word.
const POINTER_SET: u32 = 0x0066_b0d0;
/// `0045cec0` (unnamed in the engine map): `thiscall`, plain `RET`; the smart pointer's
/// destructor.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `006211b0` (Xbox PDB name `hkpAllCdPointCollector::~hkpAllCdPointCollector`, possibly a
/// folded body): `thiscall`, plain `RET`; the caster base destructor.
const CASTER_BASE_DESTRUCT: u32 = 0x0062_11b0;
/// `00aa13e0` (unnamed in the engine map): `cdecl`, the size; the allocation.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `bhkSphereShape::bhkSphereShape` (engine map), `0056e950`: `thiscall`, `RET 8`, a `float`
/// and a word.
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0056_e950;
/// `0056e090` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`;
/// the phantom record's constructor.
const PHANTOM_INFO_CONSTRUCT: u32 = 0x0056_e090;
/// `008c71b0` (engine map name `BGSChangeFlags::IsForcedChange`, a folded body: `thiscall`,
/// `RET 4`, a word; fills a 4-byte filter record).
const RECORD_FROM_WORD: u32 = 0x008c_71b0;
/// `004a39f0` (`fallout shared/bgsdecalmanager.cpp` in the engine map): `thiscall`, `RET 4`; the
/// layer of the filter record.
const RECORD_SET_LAYER: u32 = 0x004a_39f0;
/// `0059ce80` (`fallout shared/tesconditionfunctions.cpp` in the engine map): `thiscall`,
/// `RET 4`.
const RECORD_SET_WORD: u32 = 0x0059_ce80;
/// `bhkCharacterProxy::operator->` (engine map name of `004ae750`, possibly a folded body):
/// `thiscall`, plain `RET`.
const HELD_OBJECT: u32 = 0x004a_e750;
/// `0056e120` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`.
const TRANSFORM_CONSTRUCT: u32 = 0x0056_e120;
/// `bhkSimpleShapePhantom::bhkSimpleShapePhantom` (engine map), `0056e2d0`: `thiscall`,
/// `RET 4`, the address of the phantom record.
const PHANTOM_CONSTRUCT: u32 = 0x0056_e2d0;
/// `0056ea90` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`;
/// the phantom record's destructor.
const PHANTOM_INFO_DESTRUCT: u32 = 0x0056_ea90;
/// `0045bb20` (`fallout shared/tes.cpp` in the engine map): `thiscall`, `RET 8`, an out vector
/// and a `float`.
const VECTOR_SCALED: u32 = 0x0045_bb20;
/// `00439e90` (`fallout shared/multiboundmarkerdata.cpp` in the engine map): `thiscall`,
/// `RET 8`, an out vector and a vector.
const VECTOR_SUM: u32 = 0x0043_9e90;
/// `00626da0` (`run_00626da0.rs`): `thiscall`, plain `RET`.
const CASTER_RESET: u32 = 0x0062_6da0;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`.
const RECORD_CONSTRUCT: u32 = 0x0068_15c0;
/// `00621290` (`fallout shared/havok/cameracaster.cpp` in the engine map): `thiscall`, plain
/// `RET`.
const SECOND_RECORD_CONSTRUCT: u32 = 0x0062_1290;
/// The `float` `00626b30` stores twice in the second record (`0.01` in the exe).
const RECORD_FLOAT: u32 = 0x0101_3ea4;
/// `004a3e00` (unnamed in the engine map): `cdecl`, destination and source; a vector copy.
const COPY_VECTOR: u32 = 0x004a_3e00;
/// `00c9eba0` (Xbox PDB name `hkpAllCdPointCollector::hkpAllCdPointCollector`, a folded body
/// as `00626b30` uses it): `thiscall`, `RET 0x10`; the cast.
const CAST: u32 = 0x00c9_eba0;
/// `00621210` (`fallout shared/havok/cameracaster.cpp` in the engine map): `thiscall`, `RET 4`,
/// the address of a stack byte.
const RESULT_HOLDER: u32 = 0x0062_1210;
/// `005609f0` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`;
/// answers a byte.
const RESULT_HAS_HITS: u32 = 0x0056_09f0;
/// `hkpAllCdPointCollector::sortHits` (Xbox PDB), `00cabad0`: `thiscall`, plain `RET`.
const SORT_HITS: u32 = 0x00ca_bad0;
/// `0043b4f0` (`fallout shared/modelloader.cpp` in the engine map): `thiscall`, `RET 4`; fills a
/// 4-byte filter record.
const FILTER_RECORD: u32 = 0x0043_b4f0;
/// Offset of the caster's smart pointer.
const POINTER_OFFSET: u32 = 0x3a0;

// Translated from 00626800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, three words: for each of the `count` (signed) 8-byte
/// elements at `block`, a placement `new` (`006e6da0`) and, when it answers an address, a copy
/// of the two words at `value`.
pub fn fn_00626800(e: &mut Engine, block: u32, count: i32, value: u32) {
    let mut index = 0i32;
    while index < count {
        let slot = block.wrapping_add((index as u32).wrapping_mul(8));
        let element = e.call(PLACEMENT_NEW, &args![8u32, slot]).u32();
        if element != 0 {
            let first = e.mem.u32(value);
            let second = e.mem.u32(value.wrapping_add(4));
            e.mem.set_u32(element, first);
            e.mem.set_u32(element.wrapping_add(4), second);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00626870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: hands the block (`+0`), the count (`+4`) and a zero
/// byte to `00626790`, then clears the count.
pub fn fn_00626870(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    let count = e.mem.u32(this.addr().wrapping_add(4));
    // The third pushed word (a zero byte) is one `00626790` never reads.
    fn_00626790(e, block, count as i32);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
}

// Translated from 006269a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006269a0 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`, a `float` and a word. Sets the caster up: a
/// sphere shape of `radius` (0x14 bytes from `00aa13e0`, `bhkSphereShape` with 1; 0 when the
/// allocation fails), a phantom record on the stack (`0056e090`) whose filter (`008c71b0` with
/// 0, layer 0x27 `004a39f0`, `0059ce80` with `filter`) and shape (`004ae750`) it fills, the
/// filter word also at `+0x3a8`; the record's transform (`0056e120`), then a phantom (0x18
/// bytes, `bhkSimpleShapePhantom`; 0 when the allocation fails) set into the smart pointer at
/// `+0x3a0` (`0066b0d0`), the byte at `+0x3a4` set to 1, and the record destroyed (`0056ea90`).
pub fn fn_006269a0(e: &mut Engine, this: Ptr, radius: f32, filter: u32) {
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let sphere = if block == 0 {
        0
    } else {
        e.call(
            SPHERE_SHAPE_CONSTRUCT,
            &args![block, x87_quiet_bits(radius.to_bits()), 1u32],
        )
        .u32()
    };
    // The frame below `EBP`: a local at `EBP - x` is at `frame + 0xa0 - x`.
    e.with_stack(0xa0, |e, frame| {
        let at = |x: u32| frame.addr().wrapping_add(0xa0).wrapping_sub(x);
        let info = at(0x90);
        let record = at(0x94);
        e.call(PHANTOM_INFO_CONSTRUCT, &args![info]);
        e.call(RECORD_FROM_WORD, &args![record, 0u32]);
        e.call(RECORD_SET_LAYER, &args![record, 0x27u32]);
        e.call(RECORD_SET_WORD, &args![record, filter]);
        e.mem.set_u32(this.addr().wrapping_add(0x3a8), filter);
        let word = e.call(READ_FIRST_WORD, &args![record]).u32();
        e.mem.set_u32(info, word);
        let shape = e.call(HELD_OBJECT, &args![sphere]).u32();
        e.mem.set_u32(at(0x8c), shape);
        e.call(TRANSFORM_CONSTRUCT, &args![at(0x70)]);
        let block = e.call(ALLOCATE, &args![0x18u32]).u32();
        let phantom = if block == 0 {
            0
        } else {
            e.call(PHANTOM_CONSTRUCT, &args![block, info]).u32()
        };
        e.call(
            POINTER_SET,
            &args![this.addr().wrapping_add(POINTER_OFFSET), phantom],
        );
        e.mem.set_u8(this.addr().wrapping_add(0x3a4), 1);
        e.call(PHANTOM_INFO_DESTRUCT, &args![info]);
    });
}

// Translated from 006268b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006268b0 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`, a `float` and a word. A caster's
/// constructor: the base (`00620ff0`), the smart pointer at `+0x3a0` from 0 (`00633c90`), then
/// `006269a0` with `radius` and `filter`. Answers `this`.
pub fn fn_006268b0(e: &mut Engine, this: Ptr, radius: f32, filter: u32) -> Ptr {
    e.call(CASTER_BASE_CONSTRUCT, &args![this]);
    e.call(
        POINTER_CONSTRUCT,
        &args![this.addr().wrapping_add(POINTER_OFFSET), 0u32],
    );
    fn_006269a0(e, this, x87_quiet(radius), filter);
    this
}

// Translated from 00626930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626930 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `fastcall`, plain `RET`. A caster's destructor: clears the smart
/// pointer at `+0x3a0` (`0066b0d0` with 0), destroys it (`0045cec0`), then the base
/// (`006211b0`).
pub fn fn_00626930(e: &mut Engine, this: Ptr) {
    let pointer = this.addr().wrapping_add(POINTER_OFFSET);
    e.call(POINTER_SET, &args![pointer, 0u32]);
    e.call(POINTER_DESTRUCT, &args![pointer]);
    e.call(CASTER_BASE_DESTRUCT, &args![this]);
}

// Translated from 00626b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626b30 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 0xc` (`from`, `direction`, `length`): casts from
/// `from` to `from + direction * length` (`0045bb20`, `00439e90`). Resets the caster
/// (`00626da0`); without a phantom (`00559450` on `+0x3a0`) runs its vtable slot `+8` and
/// answers 0. Otherwise builds the two cast records (`006815c0`, `00621290`, the second with the
/// `float` at `01013ea4` at `+0x10` and `+0x14`), copies the start and the end into them
/// (`004a3e00`), runs the slot `+8`, casts through the phantom (`00c9eba0` with both records,
/// `this` and 0) and, when the result (`00621210` on a stack byte, `005609f0`) has hits, sorts
/// them (`00cabad0`) and answers 1; otherwise 0.
pub fn fn_00626b30(e: &mut Engine, this: Ptr, from: u32, direction: u32, length: f32) -> bool {
    e.with_stack(0x60, |e, frame| {
        let at = |x: u32| frame.addr().wrapping_add(0x60).wrapping_sub(x);
        let scaled = at(0x14);
        let end = at(0x20);
        let start_record = at(0x30);
        let end_record = at(0x50);
        let length_bits = x87_quiet_bits(length.to_bits());
        let offset = e
            .call(VECTOR_SCALED, &args![direction, scaled, length_bits])
            .u32();
        e.call(VECTOR_SUM, &args![from, end, offset]);
        e.call(CASTER_RESET, &args![this]);
        let pointer = this.addr().wrapping_add(POINTER_OFFSET);
        if e.call(READ_FIRST_WORD, &args![pointer]).u32() == 0 {
            e.vcall(this.addr(), 8, &args![]);
            return false;
        }
        e.call(RECORD_CONSTRUCT, &args![start_record]);
        e.call(SECOND_RECORD_CONSTRUCT, &args![end_record]);
        let value = x87_quiet_bits(e.mem.u32(RECORD_FLOAT));
        e.mem.set_u32(at(0x3c), value);
        e.mem.set_u32(at(0x40), value);
        e.call(COPY_VECTOR, &args![start_record, from]);
        e.call(COPY_VECTOR, &args![end_record, end]);
        e.vcall(this.addr(), 8, &args![]);
        let phantom = e.call(READ_FIRST_WORD, &args![pointer]).u32();
        e.call(CAST, &args![phantom, start_record, end_record, this, 0u32]);
        let result = e.call(RESULT_HOLDER, &args![this, at(0x51)]).u32();
        if e.call(RESULT_HAS_HITS, &args![result]).u8() != 0 {
            e.call(SORT_HITS, &args![this]);
            true
        } else {
            false
        }
    })
}

// Translated from 00626c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Sets the phantom's filter word: reads its
/// filter record (`0043b4f0` on the phantom `00559450` answers for `+0x3a0`), sets `filter` in
/// it (`0059ce80`), keeps `filter` at `+0x3a8` and hands the record's word to `00626cb0` on the
/// phantom.
pub fn fn_00626c50(e: &mut Engine, this: Ptr, filter: u32) {
    let pointer = this.addr().wrapping_add(POINTER_OFFSET);
    e.with_stack(4, |e, record| {
        let phantom = e.call(READ_FIRST_WORD, &args![pointer]).u32();
        e.call(FILTER_RECORD, &args![phantom, record]);
        e.call(RECORD_SET_WORD, &args![record, filter]);
        e.mem.set_u32(this.addr().wrapping_add(0x3a8), filter);
        let word = e.mem.u32(record.addr());
        let phantom = e.call(READ_FIRST_WORD, &args![pointer]).u32();
        fn_00626cb0(e, Ptr::new(phantom), word);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_6800, fn_00626800(u32, i32, u32)),
        entry!(0x0062_6870, fn_00626870(Ptr)),
        entry!(0x0062_68b0, fn_006268b0(Ptr, f32, u32) -> Ptr),
        entry!(0x0062_6930, fn_00626930(Ptr)),
        entry!(0x0062_69a0, fn_006269a0(Ptr, f32, u32)),
        entry!(0x0062_6b30, fn_00626b30(Ptr, u32, u32, f32) -> bool),
        entry!(0x0062_6c50, fn_00626c50(Ptr, u32)),
        entry!(0x0062_6cb0, fn_00626cb0(Ptr, u32)),
        entry!(0x0062_6d00, fn_00626d00(Ptr, u32)),
        entry!(0x00626790, fn_00626790(u32, i32)),
        entry!(0x006267c0, fn_006267c0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00626790_has_no_effect() {
        let mut e = Engine::new();
        let block = e.mem.alloc(4);
        e.mem.set_u32(block, 5);
        e.call(0x0062_6790, &args![block, 10i32]);
        assert_eq!(e.mem.u32(block), 5);
    }

    #[test]
    fn fn_006267c0_copies_two_words() {
        let mut e = Engine::new();
        let source = e.mem.alloc(12);
        let destination = e.mem.alloc(12);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, *word);
        }
        e.call(0x0062_67c0, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 1);
        assert_eq!(e.mem.u32(destination + 4), 2);
        assert_eq!(e.mem.u32(destination + 8), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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
    fn at_00626d00_stores_the_word_in_the_member_at_offset_0x14() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0040_3550, |_, _| Ret::default());
        e.call(0x0062_6d00, &args![0x4000u32, 0x77u32]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![0x4014, 0x77]]);
    }

    fn rig_for_626cb0(proxy_field: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        let mut slots = [0u32; 0x32];
        slots[0xc4 / 4] = 0x0f00_6000;
        let table = 0x0f00_5100;
        e.put_vtable(table, &slots);
        e.mem.set_u32(this, table);
        e.register(0x0f00_6000, |_, _| Ret::default());
        e.register_double(CHARACTER_PROXY_FIELD, move |_, _| proxy_field.into_ret());
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0040_3550, |_, _| Ret::default());
        (e, this)
    }

    #[test]
    fn at_00626cb0_stores_the_argument_when_there_is_a_proxy_field() {
        let (mut e, this) = rig_for_626cb0(0x6000);
        e.call(0x0062_6cb0, &args![this, 0xfeed_0001u32]);
        assert_eq!(calls(&e, CHARACTER_PROXY_FIELD), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![0x6014, 0xfeed_0001]]);
        // The call through slot 0xc4 comes last, with only `this`.
        assert_eq!(calls(&e, 0x0f00_6000), vec![vec![this]]);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.last().unwrap().0, 0x0f00_6000);
    }

    #[test]
    fn at_00626cb0_only_calls_the_slot_without_a_proxy_field() {
        let (mut e, this) = rig_for_626cb0(0);
        e.call(0x0062_6cb0, &args![this, 0xfeed_0001u32]);
        assert!(calls(&e, READ_FIRST_WORD).is_empty());
        assert!(calls(&e, 0x0040_3550).is_empty());
        assert_eq!(calls(&e, 0x0f00_6000), vec![vec![this]]);
    }
}

#[cfg(test)]
mod caster_tests {
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

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn at_00626800_constructs_each_element_from_the_value() {
        let mut e = engine();
        let block = e.mem.alloc(0x18);
        // The second placement answers 0: that element is left alone.
        e.register_double(PLACEMENT_NEW, move |_, a| {
            if a[1] == block + 8 { 0 } else { a[1] }.into_ret()
        });
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x11);
        e.mem.set_u32(value + 4, 0x22);
        e.call(0x0062_6800, &args![block, 3i32, value]);
        assert_eq!(
            calls(&e, PLACEMENT_NEW),
            vec![vec![8, block], vec![8, block + 8], vec![8, block + 16]]
        );
        assert_eq!(e.mem.u32(block), 0x11);
        assert_eq!(e.mem.u32(block + 4), 0x22);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0x14), 0x22);
        e.call(0x0062_6800, &args![block, -1i32, value]);
        assert_eq!(calls(&e, PLACEMENT_NEW).len(), 3);
    }

    #[test]
    fn at_00626870_clears_the_count() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 4, 5);
        e.call(0x0062_6870, &args![this]);
        assert_eq!(e.mem.u32(this), 0x9000);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    fn setup_doubles(e: &mut Engine, block: u32) {
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PHANTOM_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(RECORD_FROM_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(HELD_OBJECT, |_, a| (a[0] + 0x10).into_ret());
        for addr in [
            PHANTOM_INFO_CONSTRUCT,
            RECORD_SET_LAYER,
            RECORD_SET_WORD,
            TRANSFORM_CONSTRUCT,
            POINTER_SET,
            PHANTOM_INFO_DESTRUCT,
            CASTER_BASE_CONSTRUCT,
            POINTER_CONSTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn at_006268b0_constructs_and_sets_up_the_phantom() {
        let mut e = engine();
        setup_doubles(&mut e, 0x8000);
        let this = e.mem.alloc(0x3b0);
        let radius = f32::from_bits(0x7f80_0001);
        let answer = e.call(0x0062_68b0, &args![this, radius, 0x55u32]);
        assert_eq!(answer.u32(), this);
        assert_eq!(calls(&e, CASTER_BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![this + 0x3a0, 0]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x14], vec![0x18]]);
        assert_eq!(
            calls(&e, SPHERE_SHAPE_CONSTRUCT),
            vec![vec![0x8000, 0x7fc0_0001, 1]]
        );
        let info = calls(&e, PHANTOM_INFO_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, RECORD_SET_LAYER), vec![vec![info - 4, 0x27]]);
        assert_eq!(calls(&e, RECORD_SET_WORD), vec![vec![info - 4, 0x55]]);
        assert_eq!(calls(&e, HELD_OBJECT), vec![vec![0x8000]]);
        assert_eq!(calls(&e, PHANTOM_CONSTRUCT), vec![vec![0x8000, info]]);
        assert_eq!(calls(&e, POINTER_SET), vec![vec![this + 0x3a0, 0x8001]]);
        assert_eq!(calls(&e, PHANTOM_INFO_DESTRUCT), vec![vec![info]]);
        assert_eq!(e.mem.u32(info), 0, "the record word read back");
        assert_eq!(e.mem.u32(info + 4), 0x8010, "the shape");
        assert_eq!(e.mem.u32(this + 0x3a8), 0x55);
        assert_eq!(e.mem.u8(this + 0x3a4), 1);
    }

    #[test]
    fn at_006269a0_sets_nulls_when_the_allocations_fail() {
        let mut e = engine();
        setup_doubles(&mut e, 0);
        let this = e.mem.alloc(0x3b0);
        e.call(0x0062_69a0, &args![this, 1.0f32, 0x55u32]);
        assert!(calls(&e, SPHERE_SHAPE_CONSTRUCT).is_empty());
        assert!(calls(&e, PHANTOM_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, HELD_OBJECT), vec![vec![0]]);
        assert_eq!(calls(&e, POINTER_SET), vec![vec![this + 0x3a0, 0]]);
    }

    #[test]
    fn at_00626930_releases_the_pointer_then_the_base() {
        let mut e = engine();
        for addr in [POINTER_SET, POINTER_DESTRUCT, CASTER_BASE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call(0x0062_6930, &args![0x4000u32]);
        assert_eq!(
            order(&e),
            vec![
                0x0062_6930,
                POINTER_SET,
                POINTER_DESTRUCT,
                CASTER_BASE_DESTRUCT
            ]
        );
        assert_eq!(calls(&e, POINTER_SET), vec![vec![0x43a0, 0]]);
    }

    /// A caster with a vtable whose slot `+8` is a double; the phantom pointer at `+0x3a0`
    /// holds `phantom`.
    fn cast_doubles(e: &mut Engine, phantom: u32, hits: u32) -> u32 {
        e.map(0x0101_3000, 0x1000);
        e.set_global(RECORD_FLOAT, 0.01f32);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0700_0008);
        e.register(0x0700_0008, |_, _| Ret::default());
        let this = e.mem.alloc(0x3b0);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x3a0, phantom);
        e.register(VECTOR_SCALED, |_, a| a[1].into_ret());
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RESULT_HOLDER, |_, _| 0x6600u32.into_ret());
        e.register_double(RESULT_HAS_HITS, move |_, _| hits.into_ret());
        for addr in [
            VECTOR_SUM,
            CASTER_RESET,
            RECORD_CONSTRUCT,
            SECOND_RECORD_CONSTRUCT,
            COPY_VECTOR,
            CAST,
            SORT_HITS,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    #[test]
    fn at_00626b30_without_a_phantom_answers_zero() {
        let mut e = engine();
        let this = cast_doubles(&mut e, 0, 1);
        let answer = e.call(0x0062_6b30, &args![this, 0x100u32, 0x200u32, 2.0f32]);
        assert!(!answer.bool());
        let scaled = calls(&e, VECTOR_SCALED);
        assert_eq!(scaled[0][0], 0x200);
        assert_eq!(scaled[0][2], 2.0f32.to_bits());
        assert_eq!(calls(&e, VECTOR_SUM)[0][0], 0x100);
        assert_eq!(calls(&e, VECTOR_SUM)[0][2], scaled[0][1]);
        assert_eq!(calls(&e, CASTER_RESET), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0700_0008), vec![vec![this]]);
        assert!(calls(&e, CAST).is_empty());
    }

    #[test]
    fn at_00626b30_casts_and_sorts_the_hits() {
        let mut e = engine();
        let this = cast_doubles(&mut e, 0x7000, 1);
        let answer = e.call(0x0062_6b30, &args![this, 0x100u32, 0x200u32, 2.0f32]);
        assert!(answer.bool());
        let start = calls(&e, RECORD_CONSTRUCT)[0][0];
        let end_record = calls(&e, SECOND_RECORD_CONSTRUCT)[0][0];
        assert_eq!(start, end_record + 0x20);
        let end = calls(&e, VECTOR_SUM)[0][1];
        assert_eq!(
            calls(&e, COPY_VECTOR),
            vec![vec![start, 0x100], vec![end_record, end]]
        );
        assert_eq!(
            calls(&e, CAST),
            vec![vec![0x7000, start, end_record, this, 0]]
        );
        assert_eq!(calls(&e, RESULT_HOLDER)[0][0], this);
        assert_eq!(calls(&e, RESULT_HAS_HITS), vec![vec![0x6600]]);
        assert_eq!(calls(&e, SORT_HITS), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0700_0008).len(), 1);
    }

    #[test]
    fn at_00626b30_answers_zero_without_hits() {
        let mut e = engine();
        let this = cast_doubles(&mut e, 0x7000, 0);
        let answer = e.call(0x0062_6b30, &args![this, 0x100u32, 0x200u32, 2.0f32]);
        assert!(!answer.bool());
        assert_eq!(calls(&e, CAST).len(), 1);
        assert!(calls(&e, SORT_HITS).is_empty());
    }

    #[test]
    fn at_00626c50_sets_the_filter_and_hands_the_word_on() {
        let mut e = engine();
        let this = e.mem.alloc(0x3b0);
        e.mem.set_u32(this + 0x3a0, 0x7000);
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FILTER_RECORD, |e, a| {
            e.mem.set_u32(a[1], 0x1200);
            Ret::default()
        });
        e.register(RECORD_SET_WORD, |e, a| {
            let word = e.mem.u32(a[0]) | a[1];
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        // `00626cb0` on the phantom: a proxy field, then its slot `+0xc4`.
        e.map(0x7000, 0x10);
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xc4, 0x0700_00c4);
        e.mem.set_u32(0x7000, vtable);
        e.register(0x0700_00c4, |_, _| Ret::default());
        e.register(CHARACTER_PROXY_FIELD, |_, _| 0x5000u32.into_ret());
        e.register(STORE_WORD_AT_8, |_, _| Ret::default());
        e.call(0x0062_6c50, &args![this, 0x34u32]);
        assert_eq!(calls(&e, FILTER_RECORD)[0][0], 0x7000);
        assert_eq!(e.mem.u32(this + 0x3a8), 0x34);
        assert_eq!(calls(&e, STORE_WORD_AT_8), vec![vec![0x5014, 0x1234]]);
        assert_eq!(calls(&e, 0x0700_00c4), vec![vec![0x7000]]);
    }
}
