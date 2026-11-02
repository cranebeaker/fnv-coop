//! `NiMain/nicullingproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCullingProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_CULLING_PROPERTY_RTTI: u32 = 0x011f_5d20;

// Translated from 00a8f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_culling_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CULLING_PROPERTY_RTTI)
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5d28`: the pointer `00a8f5b0` stores the object it builds in.
const CULLING_PROPERTY_SLOT: u32 = 0x011f_5d28;

// Translated from 00a8f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nicullingproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): builds a 0x1c-byte object (allocation `00aa13e0`, base
/// constructor `00a5c200`, bit 0 of the word at `+0x18` cleared, vtable `0109f68c`) and
/// stores it, counted, in `011f5d28`, releasing the object held there before. A failed
/// allocation stores null.
pub fn fn_00a8f5b0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        let flags = e.mem.u16(block + 0x18);
        e.mem.set_u16(block + 0x18, flags & 0xfffe);
        e.mem.set_u32(block, 0x0109_f68c);
        block
    };
    assign_ref_counted(e, CULLING_PROPERTY_SLOT, object);
}

/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a8f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): releases the object that `00a8f5b0` stores in `011f5d28`. When it is not null, drops one reference
/// (`InterlockedDecrement` of the count at +4; at zero it
/// calls the object's vtable slot +4) and clears the pointer.
pub fn fn_00a8f620(e: &mut Engine) {
    let object = e.mem.u32(CULLING_PROPERTY_SLOT);
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(CULLING_PROPERTY_SLOT, 0);
    }
}

/// `NiCullingProperty`'s vtable (the one `00a8f5b0` stores).
const CULLING_PROPERTY_VTABLE: u32 = 0x0109_f68c;
/// The property base's `SaveBinary` (`00a5d7b0`), member copy (`00a5d7a0`, `thiscall`, (clone,
/// cloning process)), `NiProperty::LoadBinary` (`00a5d7f0`), `NiProperty::GetViewerStrings`
/// (`00a5d850`) and `IsEqual` (`00a62140`; the engine map names the folded bodies after
/// `NiSequenceStreamHelper`).
const PROPERTY_SAVE_BINARY: u32 = 0x00a5_d7b0;
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
const PROPERTY_IS_EQUAL: u32 = 0x00a6_2140;
/// `NiGetViewerString` of a class name (`00a7d330`) and of a `bool` (`00a7d370`, name and
/// the word holding the byte); the member name `0109f720`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const NAME_CULL_FLAG: u32 = 0x0109_f720;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

/// One 2-byte transfer of the flag word (+0x18) through the stream's input (`+0x24c`, read
/// function +8) or output (`+0x250`, write function +0xc); the stream argument's stack word is
/// the byte-count word (2).
fn transfer_flags(e: &mut Engine, this: Ptr, stream: Ptr, write: bool) {
    let binary = e.mem.u32(stream.addr() + if write { 0x250 } else { 0x24c });
    let function = e.mem.u32(binary + if write { 0xc } else { 8 });
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 2);
        e.call(
            function,
            &args![binary, this.addr() + 0x18, 2u32, count, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(binary + 4).wrapping_add(done);
    e.mem.set_u32(binary + 4, position);
}

// Translated from 00a815c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::SaveBinary` (Xbox PDB): the property base's `SaveBinary` (`00a5d7b0`),
/// then the flag word (+0x18, 2 bytes).
pub fn ni_culling_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_SAVE_BINARY, &args![this, stream]);
    transfer_flags(e, this, stream, true);
}

/// A new culling property (0x1c bytes from `00aa13e0`, the base constructor `00a5c200`, bit 0
/// of the flag word +0x18 cleared, the vtable `0109f68c`); null when the allocation fails.
fn new_culling_property(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
    let flags = e.mem.u16(block + 0x18);
    e.mem.set_u16(block + 0x18, flags & 0xfffe);
    e.mem.set_u32(block, CULLING_PROPERTY_VTABLE);
    block
}

// Translated from 00a8f530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::CreateClone` (Xbox PDB): a new culling property (null when the
/// allocation fails; the stores below still go to it), the property base's member copy
/// (`00a5d7a0`), then this one's flag word (+0x18). Answers the clone.
pub fn ni_culling_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_culling_property(e);
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    Ptr::new(clone)
}

// Translated from 00a8f580 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiCullingProperty`: a new culling property, or null.
pub fn fn_00a8f580(e: &mut Engine) -> Ptr {
    Ptr::new(new_culling_property(e))
}

// Translated from 00a8f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary` (`00a5d7f0`), then the
/// flag word (+0x18, 2 bytes).
pub fn ni_culling_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    transfer_flags(e, this, stream, false);
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array (grown to
/// count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a8f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::GetViewerStrings` (Xbox PDB): `NiProperty::GetViewerStrings`, the class
/// name (first word of the `NiRTTI` at `011f5d20`), then bit 0 of the flag word as a `bool`
/// under the name at `0109f720` (`00a7d370`). Both strings are kept in the `strings`
/// argument's stack word; the bool goes in its low byte over the class name's upper bytes,
/// and that whole word is pushed.
pub fn ni_culling_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_CULLING_PROPERTY_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let flag = e.mem.u8(this.addr() + 0x18) & 1;
        e.mem.set_u8(slot, flag);
        let word = e.mem.u32(slot);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_CULL_FLAG, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
    });
}

// Translated from 00a9a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProperty::IsEqual` (Xbox PDB): the property base's comparison (`00a62140`), then
/// whether bit 0 of both flag words (+0x18) is the same.
pub fn ni_culling_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(PROPERTY_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    (!e.mem.u8(other.addr() + 0x18) ^ e.mem.u8(this.addr() + 0x18)) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8f520, ni_culling_property_get_rtti() -> Ptr),
        entry!(0x00a8_f5b0, fn_00a8f5b0()),
        entry!(0x00a8_f620, fn_00a8f620()),
        entry!(0x00a8_15c0, ni_culling_property_save_binary(Ptr, Ptr)),
        entry!(0x00a8_f530, ni_culling_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_f580, fn_00a8f580() -> Ptr),
        entry!(0x00a8_f650, ni_culling_property_load_binary(Ptr, Ptr)),
        entry!(
            0x00a8_f690,
            ni_culling_property_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a9_a5b0, ni_culling_property_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_culling_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f520, &args![]).u32(), NI_CULLING_PROPERTY_RTTI);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn clears_bit_zero_of_the_word_the_base_constructor_left() {
        let run = run_default_instance(0x00a8_f5b0, CULLING_PROPERTY_SLOT, |e| {
            e.register(OBJECT_BASE_CONSTRUCTOR, |e, a| {
                e.mem.set_u16(a[0] + 0x18, 0x0003);
                Ret::default()
            });
        });
        let e = &run.engine;
        assert_eq!(e.mem.u16(run.object + 0x18), 0x0002);
        assert_eq!(e.mem.u32(run.object), 0x0109_f68c);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a8_f5b0, CULLING_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a8f620_releases_and_clears_the_held_object() {
        let mut e = Engine::new();
        e.map(CULLING_PROPERTY_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        let remaining = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let counter = remaining.clone();
        e.register_double(INTERLOCKED_DECREMENT, move |_, _| {
            counter.set(counter.get() - 1);
            counter.get().into_ret()
        });
        // Still referenced elsewhere: no destructor.
        remaining.set(2);
        e.mem.set_u32(CULLING_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![object + 4])));
        assert!(log.iter().all(|call| call.0 != 0x0150_1000));
        assert_eq!(e.mem.u32(CULLING_PROPERTY_SLOT), 0);
        // Last reference: vtable slot +4 runs on the object.
        e.mem.set_u32(CULLING_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0150_1000, vec![object])));
        assert_eq!(e.mem.u32(CULLING_PROPERTY_SLOT), 0);
        // Null: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a8_f620, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for double in [
            PROPERTY_SAVE_BINARY,
            PROPERTY_COPY_MEMBERS,
            PROPERTY_LOAD_BINARY,
            PROPERTY_GET_VIEWER_STRINGS,
            ARRAY_SET_SIZE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn stream(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        e.mem.set_u32(binary + 8, 0x0300_0000);
        e.mem.set_u32(binary + 0xc, 0x0300_0004);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            e.mem.set_u16(a[1], 0x1235);
            2u32.into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            2u32.into_ret()
        });
        (stream, binary)
    }

    #[test]
    fn save_and_load_move_the_flag_word() {
        let mut e = engine();
        let (stream, binary) = stream(&mut e);
        let this = e.mem.alloc(0x1c);
        e.call(0x00a8_15c0, &args![this, stream]);
        assert_eq!(calls_to(&e, PROPERTY_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(calls_to(&e, 0x0300_0004)[0][1], this + 0x18);
        e.call(0x00a8_f650, &args![this, stream]);
        assert_eq!(calls_to(&e, PROPERTY_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u16(this + 0x18), 0x1235);
        assert_eq!(e.mem.u32(binary + 4), 4);
    }

    #[test]
    fn creation_and_clone() {
        let mut e = engine();
        let made = e.call(0x00a8_f580, &args![]).u32();
        assert_eq!(e.mem.u32(made), CULLING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(made + 0x18) & 1, 0);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x4321);
        let clone = e.call(0x00a8_f530, &args![this, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), CULLING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x18), 0x4321);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_f580, &args![]).u32(), 0);
    }

    #[test]
    fn viewer_strings_add_class_and_flag() {
        let mut e = engine();
        e.mem.set_u32(NI_CULLING_PROPERTY_RTTI, 0x0102_0000);
        e.register(GET_VIEWER_STRING, |_, _| 0x1234_5600u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, _| 0x8000u32.into_ret());
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_4f00 + 4 * a[1], value);
            let count = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, count);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u8(this + 0x18, 0x81);
        e.call(0x00a8_f690, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_BOOL),
            vec![vec![NAME_CULL_FLAG, 0x1234_5601]]
        );
        assert_eq!(
            (e.mem.u32(0x011f_4f00), e.mem.u32(0x011f_4f04)),
            (0x1234_5600, 0x8000)
        );
    }

    #[test]
    fn is_equal_compares_bit_0() {
        let mut e = engine();
        e.register(PROPERTY_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u8(this + 0x18, 0xfe);
        e.mem.set_u8(other + 0x18, 0x02);
        assert!(e.call(0x00a9_a5b0, &args![this, other]).bool());
        e.mem.set_u8(other + 0x18, 0x03);
        assert!(!e.call(0x00a9_a5b0, &args![this, other]).bool());
        e.register(PROPERTY_IS_EQUAL, |_, _| 0u32.into_ret());
        e.mem.set_u8(other + 0x18, 0x02);
        assert!(!e.call(0x00a9_a5b0, &args![this, other]).bool());
    }
}
