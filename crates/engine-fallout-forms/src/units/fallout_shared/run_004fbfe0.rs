//! `fallout shared/run_004fbfe0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Seven functions of `BGSMovableStatic` (Xbox PDB) that sit between the
//! `bgsmovablestatic.cpp` code and the `bgsplaceablewater.cpp` code and have
//! no names in the Xbox PDB map. What identifies them: the RTTI
//! complete object locator `01109e5c` has offset 0x14 and the type
//! `.?AVBGSMovableStatic@@` (`01189d4c`), and the vtable it belongs to starts
//! at `01024cec`: its slot at +0x10 holds the adjustor thunk `004fc160`
//! (which subtracts 0x14 and jumps to the class's scalar deleting
//! destructor `004fc170`), the slot at +0x108 holds `004fbfe0` and the slot
//! at +0x10c holds `004fc080`. The Xbox PDB names the `TESForm` slots at
//! +0x108 and +0x10c `Copy` and `Compare` (the PC slot order matches the PDB
//! at +0x10, the destructor), so those two functions are the class's `Copy`
//! and `Compare`; the map gives no name for them, so all functions keep
//! `fn_<address>` names. The locator `01109e70` (offset 0x44, the
//! `TESModelTextureSwap` part) belongs to the vtable that holds the second
//! destructor thunk `004fc1a0`.
//!
//! The `this` of `004fbfe0`, `004fc080`, `004fc060`, `004fc130` and
//! `004fc160` is not the start of the object: `004fbfe0` and `004fc080` are
//! called on the `TESForm` part (+0x14), `004fc060` and `004fc130` on the
//! `MOVABLE_STATIC_DATA` byte (+0x70 of the object, +0x5c of the `TESForm`
//! part, the Xbox PDB's `data.cFlags` at +0x80 less the 0x10 the PC build
//! saves in `TESForm`). The sound loop `pSoundLoop` sits at +0x6c of the
//! object (Xbox +0x7c).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Returns the word at +0x6c of the object in `ECX`: `pSoundLoop`.
const GET_SOUND_LOOP: u32 = 0x005e_3fa0;
/// Stores its argument at +0x6c of the object in `ECX` (the setter of
/// `pSoundLoop`, in `bgsmovablestatic.cpp`).
const SET_SOUND_LOOP: u32 = 0x004f_bf00;
/// `TESForm::CopyAllComponents` (Xbox PDB): `this`, other form (or null).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (Xbox PDB): `this`, other form; true when
/// they differ (or `other` is null).
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;

/// RTTI type descriptors of the `dynamic_cast` from a `TESForm` to a
/// `BGSMovableStatic`.
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_MOVABLE_STATIC: u32 = 0x0118_9d4c;

/// Offset from the start of a `BGSMovableStatic` to its `TESForm` part, from
/// the start to its `TESModelTextureSwap` part (the second thunk), and to the
/// `MOVABLE_STATIC_DATA` byte.
const FORM_PART_OFFSET: u32 = 0x14;
const MODEL_PART_OFFSET: u32 = 0x44;
const DATA_OFFSET: u32 = 0x70;
/// Offset of the `MOVABLE_STATIC_DATA` byte from the `TESForm` part.
const DATA_OFFSET_IN_FORM_PART: u32 = 0x5c;

/// `dynamic_cast<BGSMovableStatic *>` of a `TESForm *`.
fn cast_to_movable_static(e: &mut Engine, form: Ptr) -> Ptr {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0, TYPE_FORM, TYPE_MOVABLE_STATIC, 0],
    )
    .ptr()
}

/// The `TESForm` part of a cast result: null stays null (the compiler's
/// pointer adjustment test).
fn form_part(object: Ptr) -> u32 {
    if object.is_null() {
        0
    } else {
        object.addr() + FORM_PART_OFFSET
    }
}

// Translated from 004fbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMovableStatic`'s copy function (`TESForm` slot `0x108`; `this` is the
/// `TESForm` part): casts `other` to `BGSMovableStatic` and, when it is one,
/// copies the sound loop (through the object's setter), the data byte and
/// then all form components (`TESForm::CopyAllComponents` with the other's
/// `TESForm` part). Nothing happens when the cast fails.
pub fn fn_004fbfe0(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_to_movable_static(e, other);
    if source.is_null() {
        return;
    }
    let sound_loop = e.call(GET_SOUND_LOOP, &args![source]).u32();
    e.call(
        SET_SOUND_LOOP,
        &args![this.addr().wrapping_sub(FORM_PART_OFFSET), sound_loop],
    );
    fn_004fc060(
        e,
        Ptr::new(this.addr() + DATA_OFFSET_IN_FORM_PART),
        Ptr::new(source.addr() + DATA_OFFSET),
    );
    e.call(COPY_ALL_COMPONENTS, &args![this, form_part(source)]);
}

// Translated from 004fc060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies one byte: `*this = *source` (the `MOVABLE_STATIC_DATA`
/// assignment, a one-byte structure).
pub fn fn_004fc060(e: &mut Engine, this: Ptr, source: Ptr) {
    let byte = e.mem.u8(source.addr());
    e.mem.set_u8(this.addr(), byte);
}

// Translated from 004fc080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMovableStatic`'s compare function (`TESForm` slot `0x10c`; `this` is
/// the `TESForm` part): true when `other` is not a `BGSMovableStatic`, when
/// `TESForm::CompareAllComponents` says the components differ, when the data
/// byte differs or when the sound loops differ.
pub fn fn_004fc080(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast_to_movable_static(e, other);
    if source.is_null() {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, form_part(source)])
        .bool()
    {
        return true;
    }
    if fn_004fc130(
        e,
        Ptr::new(this.addr() + DATA_OFFSET_IN_FORM_PART),
        Ptr::new(source.addr() + DATA_OFFSET),
    ) {
        return true;
    }
    let mine = e
        .call(
            GET_SOUND_LOOP,
            &args![this.addr().wrapping_sub(FORM_PART_OFFSET)],
        )
        .u32();
    let theirs = e.call(GET_SOUND_LOOP, &args![source]).u32();
    mine != theirs
}

// Translated from 004fc130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the bytes differ: `*this != *other` (the `MOVABLE_STATIC_DATA`
/// comparison).
pub fn fn_004fc130(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u8(this.addr()) != e.mem.u8(other.addr())
}

// Translated from 004fc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The adjustor thunk in the `TESForm` vtable's destructor slot: subtracts
/// 0x14 from `this` and continues in [`fn_004fc170`] with the same delete
/// flag. Returns the adjusted pointer, as that function does.
pub fn fn_004fc160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004fc170(
        e,
        Ptr::new(this.addr().wrapping_sub(FORM_PART_OFFSET)),
        flags,
    )
}

// Translated from 004fc170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BGSMovableStatic`: runs the
/// destructor body `004fbac0` and frees the object when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_004fc170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BODY_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004fc1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The adjustor thunk in the destructor slot of the vtable at offset 0x44:
/// subtracts 0x44 from `this` and continues in [`fn_004fc170`] with the same
/// delete flag. Returns the adjusted pointer, as that function does.
pub fn fn_004fc1a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004fc170(
        e,
        Ptr::new(this.addr().wrapping_sub(MODEL_PART_OFFSET)),
        flags,
    )
}

/// Destructor body of `BGSMovableStatic` (`004fbac0`, in `run_004fb970`).
const BODY_DESTRUCTOR: u32 = 0x004f_bac0;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fbfe0, fn_004fbfe0(Ptr, Ptr)),
        entry!(0x004fc060, fn_004fc060(Ptr, Ptr)),
        entry!(0x004fc080, fn_004fc080(Ptr, Ptr) -> bool),
        entry!(0x004fc130, fn_004fc130(Ptr, Ptr) -> bool),
        entry!(0x004fc160, fn_004fc160(Ptr, u32) -> Ptr),
        entry!(0x004fc170, fn_004fc170(Ptr, u32) -> Ptr),
        entry!(0x004fc1a0, fn_004fc1a0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles for the callees outside this file: the cast turns the
    /// `TESForm` part pointer into the start of the object (minus 0x14), the
    /// sound loop getter and setter work on +0x6c, the component functions
    /// are plain.
    fn engine_with_doubles() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0] - 0x14,
            ..Ret::default()
        });
        e.register(GET_SOUND_LOOP, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x6c),
            ..Ret::default()
        });
        e.register(SET_SOUND_LOOP, |e, a| {
            e.mem.set_u32(a[0] + 0x6c, a[1]);
            Ret::default()
        });
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(BODY_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    /// A fake object of 0x80 bytes (the start of a `BGSMovableStatic`).
    fn object(e: &mut Engine) -> u32 {
        e.mem.alloc(0x80)
    }

    #[test]
    fn copy_moves_the_sound_loop_the_data_byte_and_the_components() {
        let mut e = engine_with_doubles();
        let mine = object(&mut e);
        let theirs = object(&mut e);
        e.mem.set_u32(theirs + 0x6c, 0x5150);
        e.mem.set_u8(theirs + 0x70, 0x2a);
        e.call_log = Some(vec![]);
        e.call(0x004f_bfe0, &args![mine + 0x14, theirs + 0x14]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(mine + 0x6c), 0x5150);
        assert_eq!(e.mem.u8(mine + 0x70), 0x2a);
        // The setter gets the start of the object, the component copy the
        // two `TESForm` parts.
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == SET_SOUND_LOOP && a[..] == [mine, 0x5150]));
        assert!(
            log.iter()
                .any(|(addr, a)| *addr == COPY_ALL_COMPONENTS
                    && a[..] == [mine + 0x14, theirs + 0x14])
        );
    }

    #[test]
    fn copy_of_something_that_is_not_a_movable_static_does_nothing() {
        let mut e = engine_with_doubles();
        let mine = object(&mut e);
        e.mem.set_u32(mine + 0x6c, 9);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(COPY_ALL_COMPONENTS, |_, _| panic!("not a movable static"));
        e.call(0x004f_bfe0, &args![mine + 0x14, 0x1000u32]);
        assert_eq!(e.mem.u32(mine + 0x6c), 9);
    }

    #[test]
    fn data_byte_copy_and_compare() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u8(b, 0x81);
        e.mem.set_u8(a + 1, 0x33);
        e.call(0x004f_c060, &args![a, b]);
        assert_eq!(e.mem.u8(a), 0x81);
        // Only the one byte is copied.
        assert_eq!(e.mem.u8(a + 1), 0x33);
        assert!(!e.call(0x004f_c130, &args![a, b]).bool());
        e.mem.set_u8(b, 0x80);
        assert!(e.call(0x004f_c130, &args![a, b]).bool());
    }

    #[test]
    fn compare_checks_the_cast_the_components_the_byte_and_the_sound_loop() {
        let mut e = engine_with_doubles();
        let mine = object(&mut e);
        let theirs = object(&mut e);
        let compare = |e: &mut Engine| {
            e.call(0x004f_c080, &args![mine + 0x14, theirs + 0x14])
                .bool()
        };
        assert!(!compare(&mut e));
        // The component comparison gets the two `TESForm` parts.
        e.register_double(COMPARE_ALL_COMPONENTS, move |_, a| {
            assert_eq!(a, [mine + 0x14, theirs + 0x14]);
            Ret::default()
        });
        assert!(!compare(&mut e));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(compare(&mut e));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| Ret::default());
        // A different data byte.
        e.mem.set_u8(theirs + 0x70, 1);
        assert!(compare(&mut e));
        e.mem.set_u8(theirs + 0x70, 0);
        // A different sound loop (read from the start of each object).
        e.mem.set_u32(theirs + 0x6c, 4);
        assert!(compare(&mut e));
        e.mem.set_u32(mine + 0x6c, 4);
        assert!(!compare(&mut e));
        // Not a movable static.
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(compare(&mut e));
    }

    #[test]
    fn destructor_thunks_adjust_this_and_delete_with_the_flag() {
        for (addr, adjust) in [
            (0x004f_c160u32, 0x14u32),
            (0x004f_c1a0, 0x44),
            (0x004f_c170, 0),
        ] {
            let mut e = engine_with_doubles();
            let start = 0x5000u32;
            let this = start + adjust;
            e.call_log = Some(vec![]);
            let back = e.call(addr, &args![this, 0]).ptr::<()>();
            let log = e.call_log.take().unwrap();
            assert_eq!(back.addr(), start);
            assert!(log
                .iter()
                .any(|(callee, a)| *callee == BODY_DESTRUCTOR && a[..] == [start]));
            assert!(log.iter().all(|(callee, _)| *callee != OPERATOR_DELETE));

            e.call_log = Some(vec![]);
            let back = e.call(addr, &args![this, 3]).ptr::<()>();
            let log = e.call_log.take().unwrap();
            assert_eq!(back.addr(), start);
            assert!(log
                .iter()
                .any(|(callee, a)| *callee == OPERATOR_DELETE && a[..] == [start]));
        }
    }
}
