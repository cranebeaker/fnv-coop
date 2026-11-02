//! `Havok SDK/run_00ddc8e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDebugDisplayProcess` (class confirmed by the RTTI of its vtables `010d7ed0`, `010d7eb0`
//! and `010d7e5c`): the thunks of its `hkDebugDisplayHandler` base (vtable `010d7e5c`, the
//! base sits at `+0x20`; the slot names are the Xbox PDB's `hkDebugDisplayHandler` slots).
//! Each takes the pointer stored at `this - 8` (the process's `+0x18`), calls the same slot of
//! that object's vtable with the caller's argument words and, where the slot has a tag
//! argument, replaces that word with the process id stored at `01269cdc` (by
//! `hkDebugDisplayProcess::registerProcess`, `00ddcbc0`). The result is the callee's `EAX`.
//! The argument counts are the ones `hkDebugDisplay`'s own loops over its handlers push
//! (`RET n` of `00c983e0` .. `00c991c0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The process id `registerProcess` stored: the tag every forwarded call carries.
const PROCESS_ID: u32 = 0x0126_9cdc;
/// The `hkDebugDisplay` singleton pointer the process adds itself to and removes itself from.
const DEBUG_DISPLAY: u32 = 0x0126_81e0;
/// `hkDebugDisplayProcess::registerProcess`.
const REGISTER_PROCESS: u32 = 0x00dd_cbc0;
/// Adds the handler (one stack word) to the `hkDebugDisplay` in `ECX`.
const ADD_HANDLER: u32 = 0x00c9_9ce0;
/// Removes the handler (one stack word) from the `hkDebugDisplay` in `ECX`.
const REMOVE_HANDLER: u32 = 0x00c9_9590;

/// The vtables the constructor and destructor store, by the class their RTTI names.
const PROCESS_VTABLE_ROOT: u32 = 0x010d_7ed0;
const PROCESS_VTABLE_OBJECT: u32 = 0x010d_7eb0;
const PROCESS_VTABLE_HANDLER: u32 = 0x010d_7e5c;
const HANDLER_VTABLE: u32 = 0x010d_7cfc;
const PROCESS_BASE_VTABLE: u32 = 0x010d_1508;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

/// The object the thunks forward to: the word at `this - 8`.
fn target_object(e: &Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_sub(8))
}

/// The process id the forwarded calls carry in place of their tag word.
fn process_id(e: &Engine) -> u32 {
    e.global::<u32>(PROCESS_ID)
}

// Translated from 00ddc8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x4` (`addGeometry`, Xbox PDB): forwards five words; word 4 is replaced by
/// the process id. Word 4 is never read, the others are passed on.
pub fn fn_00ddc8e0(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    _unused_4: u32,
    arg_5: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x4, &args![arg_1, arg_2, arg_3, tag, arg_5])
        .u32()
}

// Translated from 00ddc900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x8` (`addGeometryInstance`, Xbox PDB): forwards five words; word 4 is
/// replaced by the process id.
pub fn fn_00ddc900(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    _unused_4: u32,
    arg_5: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x8, &args![arg_1, arg_2, arg_3, tag, arg_5])
        .u32()
}

// Translated from 00ddc920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x3c` (the second `displayGeometry`, Xbox PDB): forwards five words; word 5
/// is replaced by the process id.
pub fn fn_00ddc920(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    _unused_5: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x3c, &args![arg_1, arg_2, arg_3, arg_4, tag])
        .u32()
}

// Translated from 00ddc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x38` (the first `displayGeometry`, Xbox PDB): forwards four words; word 4
/// is replaced by the process id.
pub fn fn_00ddc940(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    _unused_4: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x38, &args![arg_1, arg_2, arg_3, tag])
        .u32()
}

// Translated from 00ddc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0xc` (`setGeometryColor`, Xbox PDB): forwards three words; word 3 is
/// replaced by the process id.
pub fn fn_00ddc960(e: &mut Engine, this: Ptr, arg_1: u32, arg_2: u32, _unused_3: u32) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0xc, &args![arg_1, arg_2, tag]).u32()
}

// Translated from 00ddc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x10` (`updateGeometry`, Xbox PDB): forwards three words; word 3 is
/// replaced by the process id.
pub fn fn_00ddc980(e: &mut Engine, this: Ptr, arg_1: u32, arg_2: u32, _unused_3: u32) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x10, &args![arg_1, arg_2, tag]).u32()
}

// Translated from 00ddc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x14` (`removeGeometry`, Xbox PDB): forwards three words; word 2 is
/// replaced by the process id.
pub fn fn_00ddc9a0(e: &mut Engine, this: Ptr, arg_1: u32, _unused_2: u32, arg_3: u32) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x14, &args![arg_1, tag, arg_3]).u32()
}

// Translated from 00ddc9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x18` (`updateCamera`, Xbox PDB): forwards seven words. Words 4, 5 and 6 go
/// through the x87 stack (`FLD`/`FSTP` of a float: a signalling NaN comes out quiet); the rest
/// are passed unchanged. No process id is inserted.
// Nine words: seven of the exe's plus `e` and `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ddc9c0(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: f32,
    arg_5: f32,
    arg_6: f32,
    arg_7: u32,
) -> u32 {
    let handler = target_object(e, this);
    let words = args![
        arg_1,
        arg_2,
        arg_3,
        x87_quiet_bits(arg_4.to_bits()),
        x87_quiet_bits(arg_5.to_bits()),
        x87_quiet_bits(arg_6.to_bits()),
        arg_7
    ];
    e.vcall(handler, 0x18, &words).u32()
}

// Translated from 00ddca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x1c` (`updateBehavior`, Xbox PDB): jumps to the handler's slot with the
/// caller's words untouched (eight words: `hkDebugDisplay`'s loop pushes eight). The result
/// registers are the callee's, passed through.
pub fn fn_00ddca00(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::<()>::new(words[0]);
    let handler = target_object(e, this);
    e.vcall(handler, 0x1c, &words[1..])
}

// Translated from 00ddca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x20` (`displayPoint`, Xbox PDB): forwards four words; word 4 is replaced
/// by the process id.
pub fn fn_00ddca10(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    _unused_4: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x20, &args![arg_1, arg_2, arg_3, tag])
        .u32()
}

// Translated from 00ddca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x24` (`displayLine`, Xbox PDB): forwards five words; word 5 is replaced by
/// the process id.
pub fn fn_00ddca30(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    _unused_5: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x24, &args![arg_1, arg_2, arg_3, arg_4, tag])
        .u32()
}

// Translated from 00ddca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x28` (`displayTriangle`, Xbox PDB): forwards six words; word 6 is replaced
/// by the process id.
#[allow(clippy::too_many_arguments)] // the exe takes the six words and `this`
pub fn fn_00ddca50(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    arg_5: u32,
    _unused_6: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(
        handler,
        0x28,
        &args![arg_1, arg_2, arg_3, arg_4, arg_5, tag],
    )
    .u32()
}

// Translated from 00ddca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x2c` (`displayText`, Xbox PDB): forwards four words; word 4 is replaced by
/// the process id.
pub fn fn_00ddca70(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    _unused_4: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x2c, &args![arg_1, arg_2, arg_3, tag])
        .u32()
}

// Translated from 00ddca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x30` (`display3dText`, Xbox PDB): forwards five words; word 5 is replaced
/// by the process id.
pub fn fn_00ddca90(
    e: &mut Engine,
    this: Ptr,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    _unused_5: u32,
) -> u32 {
    let handler = target_object(e, this);
    let tag = process_id(e);
    e.vcall(handler, 0x30, &args![arg_1, arg_2, arg_3, arg_4, tag])
        .u32()
}

// Translated from 00ddcab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x34` (`displayAnnotation`, Xbox PDB): replaces word 3 of the caller's
/// words by the process id and jumps. No caller of this slot is in the exe's own handler
/// loops, so the word count is not confirmed: every word the caller pushed is passed on. The
/// result registers are the callee's.
pub fn fn_00ddcab0(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::<()>::new(words[0]);
    let handler = target_object(e, this);
    let mut forwarded = words[1..].to_vec();
    forwarded[2] = process_id(e);
    e.vcall(handler, 0x34, &forwarded)
}

// Translated from 00ddcad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x40` (`sendMemStatsDump`, Xbox PDB): jumps with the caller's words
/// untouched (the word count is not confirmed: the exe has no caller of this slot in its
/// handler loops). The result registers are the callee's.
pub fn fn_00ddcad0(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::<()>::new(words[0]);
    let handler = target_object(e, this);
    e.vcall(handler, 0x40, &words[1..])
}

// Translated from 00ddcae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of slot `0x44` (`holdImmediate`, Xbox PDB): jumps with the caller's words untouched
/// (the word count is not confirmed). The result registers are the callee's.
pub fn fn_00ddcae0(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::<()>::new(words[0]);
    let handler = target_object(e, this);
    e.vcall(handler, 0x44, &words[1..])
}

// Translated from 00ddcaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplayProcess::hkDebugDisplayProcess` (constructor; class confirmed by the RTTI
/// of the vtables it stores): sets the 16-bit word at `+6` and the byte at `+0xc` to 1,
/// stores the vtables (the first values of `+0x20` and `+8` are overwritten at once, as the
/// exe does), registers the process type with `registerProcess` if `01269cdc` is still 0 and
/// adds the handler base (`this + 0x20`) to the `hkDebugDisplay` at `012681e0`. Returns
/// `this`.
pub fn hk_debug_display_process_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let handler = base.wrapping_add(0x20);
    e.mem.set_u16(base.wrapping_add(6), 1);
    e.mem.set_u32(base.wrapping_add(8), PROCESS_BASE_VTABLE);
    e.mem.set_u8(base.wrapping_add(0xc), 1);
    e.mem.set_u32(handler, HANDLER_VTABLE);
    e.mem.set_u32(base, PROCESS_VTABLE_ROOT);
    e.mem.set_u32(base.wrapping_add(8), PROCESS_VTABLE_OBJECT);
    e.mem.set_u32(handler, PROCESS_VTABLE_HANDLER);
    if process_id(e) == 0 {
        e.call(REGISTER_PROCESS, &args![]);
    }
    let display = e.global::<u32>(DEBUG_DISPLAY);
    e.call(ADD_HANDLER, &args![display, handler]);
    this
}

// Translated from 00ddcb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplayProcess::~hkDebugDisplayProcess` (destructor; class confirmed by the RTTI):
/// stores its own vtables, removes the handler base (`this + 0x20`) from the `hkDebugDisplay`
/// at `012681e0`, then stores the base class vtables (`hkDebugDisplayHandler` at `+0x20`,
/// `hkProcess` at `+8`, `hkBaseObject` at `+0`).
pub fn hk_debug_display_process_dtor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let handler = base.wrapping_add(0x20);
    e.mem.set_u32(base, PROCESS_VTABLE_ROOT);
    e.mem.set_u32(base.wrapping_add(8), PROCESS_VTABLE_OBJECT);
    e.mem.set_u32(handler, PROCESS_VTABLE_HANDLER);
    let display = e.global::<u32>(DEBUG_DISPLAY);
    e.call(REMOVE_HANDLER, &args![display, handler]);
    e.mem.set_u32(handler, HANDLER_VTABLE);
    e.mem.set_u32(base.wrapping_add(8), PROCESS_BASE_VTABLE);
    e.mem.set_u32(base, BASE_OBJECT_VTABLE);
}

// Translated from 00ddcb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, `_unused_0`, that it never reads (cdecl, nothing popped; the other creators of this kind receive the process contexts there). Allocates 0x24 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00ddcaf0` on the block (no stack word), and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00ddcbc6.
pub fn fn_00ddcb90(e: &mut Engine, _unused_0: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x24]).u32();
    e.mem.set_u16(block + 4, 0x24);
    let object = e.call(0x00dd_caf0, &args![block]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_c8e0, fn_00ddc8e0(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_c900, fn_00ddc900(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_c920, fn_00ddc920(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_c940, fn_00ddc940(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_c960, fn_00ddc960(Ptr, u32, u32, u32) -> u32),
        entry!(0x00dd_c980, fn_00ddc980(Ptr, u32, u32, u32) -> u32),
        entry!(0x00dd_c9a0, fn_00ddc9a0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00dd_c9c0, fn_00ddc9c0(Ptr, u32, u32, u32, f32, f32, f32, u32) -> u32),
        (0x00dd_ca00, fn_00ddca00 as AbiFn),
        entry!(0x00dd_ca10, fn_00ddca10(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_ca30, fn_00ddca30(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_ca50, fn_00ddca50(Ptr, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_ca70, fn_00ddca70(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd_ca90, fn_00ddca90(Ptr, u32, u32, u32, u32, u32) -> u32),
        (0x00dd_cab0, fn_00ddcab0 as AbiFn),
        (0x00dd_cad0, fn_00ddcad0 as AbiFn),
        (0x00dd_cae0, fn_00ddcae0 as AbiFn),
        entry!(0x00dd_caf0, hk_debug_display_process_ctor(Ptr) -> Ptr),
        entry!(0x00dd_cb50, hk_debug_display_process_dtor(Ptr)),
        entry!(0x00dd_cb90, fn_00ddcb90(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const TAG: u32 = 0x7777;
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An engine with the globals mapped, a handler object whose vtable sends every slot to its
    /// own double (`0x0900_0000 + slot`, returning `0xabcd`), and a process whose `this - 8`
    /// word points at the handler. Returns the engine, `this`, the handler and the calls the
    /// doubles saw as `(slot address, words)`.
    fn setup() -> (Engine, u32, u32, Seen) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(PROCESS_ID, TAG);
        let handler = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x60);
        let process = e.mem.alloc(0x40);
        let this = process + 0x20;
        e.mem.set_u32(this - 8, handler);
        e.mem.set_u32(handler, vtable);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        for slot in (0..0x48).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
            let seen = seen.clone();
            e.register_double(0x0900_0000 + slot, move |_, words| {
                let mut entry = vec![slot];
                entry.extend_from_slice(words);
                seen.borrow_mut().push(entry);
                Ret {
                    eax: 0xabcd,
                    ..Ret::default()
                }
            });
        }
        (e, this, handler, seen)
    }

    /// Calls `thunk` with `count` words (`0x11`, `0x22`, ...) and checks it reached `slot` of the
    /// handler once with `this` replaced by the handler and word `replaced` (1-based) by the
    /// process id.
    fn check_forward(thunk: u32, slot: u32, count: u32, replaced: usize) {
        let (mut e, this, handler, seen) = setup();
        let mut words = vec![this];
        words.extend((1..=count).map(|n| n * 0x11));
        let result = e.call(thunk, &words);
        assert_eq!(result.u32(), 0xabcd);
        let mut want = vec![slot, handler];
        want.extend((1..=count).map(|n| n * 0x11));
        want[1 + replaced] = TAG;
        assert_eq!(*seen.borrow(), vec![want]);
    }

    #[test]
    fn add_geometry_replaces_word_four() {
        check_forward(0x00dd_c8e0, 0x4, 5, 4);
    }

    #[test]
    fn add_geometry_instance_replaces_word_four() {
        check_forward(0x00dd_c900, 0x8, 5, 4);
    }

    #[test]
    fn second_display_geometry_replaces_word_five() {
        check_forward(0x00dd_c920, 0x3c, 5, 5);
    }

    #[test]
    fn first_display_geometry_replaces_word_four() {
        check_forward(0x00dd_c940, 0x38, 4, 4);
    }

    #[test]
    fn set_geometry_color_replaces_word_three() {
        check_forward(0x00dd_c960, 0xc, 3, 3);
    }

    #[test]
    fn update_geometry_replaces_word_three() {
        check_forward(0x00dd_c980, 0x10, 3, 3);
    }

    #[test]
    fn remove_geometry_replaces_word_two() {
        check_forward(0x00dd_c9a0, 0x14, 3, 2);
    }

    #[test]
    fn update_camera_quiets_the_three_floats() {
        let (mut e, this, handler, seen) = setup();
        let words = [
            this,
            0x11,
            0x22,
            0x33,
            0x7f80_0001,
            0x3f80_0000,
            0xff80_0002,
            0x77,
        ];
        let result = e.call(0x00dd_c9c0, &words);
        assert_eq!(result.u32(), 0xabcd);
        assert_eq!(
            *seen.borrow(),
            vec![vec![
                0x18,
                handler,
                0x11,
                0x22,
                0x33,
                0x7fc0_0001,
                0x3f80_0000,
                0xffc0_0002,
                0x77
            ]]
        );
    }

    #[test]
    fn update_behavior_passes_every_word_through() {
        let (mut e, this, handler, seen) = setup();
        let words: Vec<u32> = std::iter::once(this)
            .chain((1..=8).map(|n| n * 3))
            .collect();
        let result = e.call(0x00dd_ca00, &words);
        assert_eq!(result.u32(), 0xabcd);
        let mut want = vec![0x1c, handler];
        want.extend((1..=8).map(|n| n * 3));
        assert_eq!(*seen.borrow(), vec![want]);
    }

    #[test]
    fn display_point_replaces_word_four() {
        check_forward(0x00dd_ca10, 0x20, 4, 4);
    }

    #[test]
    fn display_line_replaces_word_five() {
        check_forward(0x00dd_ca30, 0x24, 5, 5);
    }

    #[test]
    fn display_triangle_replaces_word_six() {
        check_forward(0x00dd_ca50, 0x28, 6, 6);
    }

    #[test]
    fn display_text_replaces_word_four() {
        check_forward(0x00dd_ca70, 0x2c, 4, 4);
    }

    #[test]
    fn display_3d_text_replaces_word_five() {
        check_forward(0x00dd_ca90, 0x30, 5, 5);
    }

    #[test]
    fn display_annotation_replaces_word_three_and_keeps_the_rest() {
        check_forward(0x00dd_cab0, 0x34, 3, 3);
        check_forward(0x00dd_cab0, 0x34, 5, 3);
    }

    #[test]
    fn send_mem_stats_dump_passes_every_word_through() {
        let (mut e, this, handler, seen) = setup();
        let result = e.call(0x00dd_cad0, &[this, 0x10, 0x20]);
        assert_eq!(result.u32(), 0xabcd);
        assert_eq!(*seen.borrow(), vec![vec![0x40, handler, 0x10, 0x20]]);
    }

    #[test]
    fn hold_immediate_passes_every_word_through() {
        let (mut e, this, handler, seen) = setup();
        let result = e.call(0x00dd_cae0, &[this]);
        assert_eq!(result.u32(), 0xabcd);
        assert_eq!(*seen.borrow(), vec![vec![0x44, handler]]);
    }

    /// An engine with the `hkDebugDisplay` pointer set and doubles over the registration
    /// functions; returns it with the calls the doubles saw as `(address, words)`.
    fn process_engine(id: u32) -> (Engine, Rc<RefCell<Vec<(u32, Vec<u32>)>>>) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(DEBUG_DISPLAY, 0x5555u32);
        e.set_global(PROCESS_ID, id);
        let seen = Rc::new(RefCell::new(Vec::new()));
        for address in [REGISTER_PROCESS, ADD_HANDLER, REMOVE_HANDLER] {
            let seen = seen.clone();
            e.register_double(address, move |_, words| {
                seen.borrow_mut().push((address, words.to_vec()));
                Ret::default()
            });
        }
        (e, seen)
    }

    #[test]
    fn constructor_registers_the_process_once_and_adds_the_handler() {
        let (mut e, seen) = process_engine(0);
        let this = e.mem.alloc(0x30);
        let result = e.call(0x00dd_caf0, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(this), PROCESS_VTABLE_ROOT);
        assert_eq!(e.mem.u32(this + 8), PROCESS_VTABLE_OBJECT);
        assert_eq!(e.mem.u32(this + 0x20), PROCESS_VTABLE_HANDLER);
        assert_eq!(
            *seen.borrow(),
            vec![
                (REGISTER_PROCESS, vec![]),
                (ADD_HANDLER, vec![0x5555, this + 0x20])
            ]
        );
    }

    #[test]
    fn constructor_skips_registration_when_the_id_exists() {
        let (mut e, seen) = process_engine(9);
        let this = e.mem.alloc(0x30);
        e.call(0x00dd_caf0, &[this]);
        assert_eq!(
            *seen.borrow(),
            vec![(ADD_HANDLER, vec![0x5555, this + 0x20])]
        );
    }

    #[test]
    fn destructor_removes_the_handler_and_leaves_the_base_vtables() {
        let (mut e, seen) = process_engine(9);
        let this = e.mem.alloc(0x30);
        e.call(0x00dd_cb50, &[this]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), PROCESS_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x20), HANDLER_VTABLE);
        assert_eq!(
            *seen.borrow(),
            vec![(REMOVE_HANDLER, vec![0x5555, this + 0x20])]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00ddcb90_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00dd_caf0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00dd_cb90, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x24]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block]);
        assert_eq!(seen[0].1, 0x24);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00ddcb90_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00dd_caf0, |_, _| Ret::default());
        assert_eq!(e.call(0x00dd_cb90, &args![0x1234u32]).u32(), 0);
    }
}
