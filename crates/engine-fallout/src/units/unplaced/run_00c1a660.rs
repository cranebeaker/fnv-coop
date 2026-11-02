//! `(unplaced)/run_00c1a660` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_012024D4: u32 = 0x0120_24d4;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00C1AA00: u32 = 0x00c1_aa00;
const EXIT_HANDLER_ARGUMENT_00C1A7D0: u32 = 0x00c1_a7d0;

// Translated from 00c1ad50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `012024d4`; when it was 0, calls
/// `00aa2080(00c1aa00, 00c1a7d0)`. Returns `this`, which it does not otherwise read.
pub fn fn_00c1ad50(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_012024D4);
    e.set_global(USE_COUNT_012024D4, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00C1AA00, EXIT_HANDLER_ARGUMENT_00C1A7D0],
        );
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// Import slot of `GetCurrentThreadId` (`stdcall`, no arguments).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// Byte `012024d0`: set to 1 when `fn_00c1aa00` has started.
const LOADERS_REGISTERED: u32 = 0x0120_24d0;
/// Word `012024dc`: the thread id `fn_00c1aa00` keeps.
const START_THREAD: u32 = 0x0120_24dc;

/// The class registrations `fn_00c1aa00` makes, in order: (class name string, creation
/// function).
const LOADERS: [(u32, u32); 53] = [
    (0x010b_d404, 0x00c2_1e90),
    (0x010b_d3f4, 0x00c3_0ca0),
    (0x010b_d3e0, 0x00c1_b7f0),
    (0x010b_d3c0, 0x00c3_0790),
    (0x010b_d39c, 0x00c3_0600),
    (0x010b_d384, 0x00c2_f630),
    (0x010b_d368, 0x00c2_f3a0),
    (0x010b_d350, 0x00c2_ef20),
    (0x010b_d33c, 0x00c2_e4c0),
    (0x010b_d320, 0x00c2_d690),
    (0x010b_d30c, 0x00c2_0350),
    (0x010b_d2f4, 0x00c2_d5b0),
    (0x010b_d2e0, 0x00c2_d180),
    (0x010b_d2c8, 0x00c1_fbf0),
    (0x010b_d2bc, 0x00c2_4da0),
    (0x010b_d2a4, 0x00c2_c7b0),
    (0x010b_d290, 0x00c2_c080),
    (0x010b_d27c, 0x00c1_c5e0),
    (0x010b_d264, 0x00c2_ba60),
    (0x010b_d244, 0x00c2_b970),
    (0x010b_d224, 0x00c2_b870),
    (0x010b_d204, 0x00c2_b720),
    (0x010b_d1e8, 0x00c2_b620),
    (0x010b_d1c8, 0x00c2_b520),
    (0x010b_d1a8, 0x00c2_b420),
    (0x010b_d190, 0x00c2_b330),
    (0x010b_d174, 0x00c2_b200),
    (0x010b_d158, 0x00c2_b110),
    (0x010b_d13c, 0x00c2_b010),
    (0x010b_d124, 0x00c2_2ad0),
    (0x010b_d108, 0x00c2_aa50),
    (0x010b_d0ec, 0x00c2_a870),
    (0x010b_d0d4, 0x00c2_a2b0),
    (0x010b_d0b8, 0x00c2_a160),
    (0x010b_d098, 0x00c2_a050),
    (0x010b_d07c, 0x00c2_9f50),
    (0x010b_d05c, 0x00c2_9e50),
    (0x010b_d048, 0x00c1_f970),
    (0x010b_d02c, 0x00c2_17f0),
    (0x010b_d010, 0x00c2_9d00),
    (0x010b_cff8, 0x00c2_9af0),
    (0x010b_cfe0, 0x00c2_8b20),
    (0x010b_cfc4, 0x00c2_8570),
    (0x010b_cfac, 0x00c2_83b0),
    (0x010b_cf94, 0x00c2_0640),
    (0x010b_cf80, 0x00c2_3b20),
    (0x010b_cf6c, 0x00c1_fff0),
    (0x010b_cf54, 0x00c2_82a0),
    (0x010b_cf34, 0x00c2_7350),
    (0x010b_cf20, 0x00c2_70f0),
    (0x010b_cf04, 0x00c2_6790),
    (0x010b_d42c, 0x00c2_5820),
    (0x010b_d41c, 0x00c2_5ca0),
];

// Translated from 00c1aa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00c1ad50` registers; no
/// arguments): once only (byte `012024d0`), registers the 53 class names of `LOADERS` with
/// their creation functions through `00a64900` and keeps the current thread id (the import
/// at `00fdf1d0`) in `012024dc`.
pub fn fn_00c1aa00(e: &mut Engine) {
    if e.mem.u8(LOADERS_REGISTERED) != 0 {
        return;
    }
    e.mem.set_u8(LOADERS_REGISTERED, 1);
    for (name, creator) in LOADERS {
        e.call(REGISTER_LOADER, &args![name, creator]);
    }
    let thread = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    e.mem.set_u32(START_THREAD, thread);
}

// Translated from 00c1a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// A 16-bit setter (Ghidra made no function here; unnamed in the map; `thiscall`,
/// `(value)`, `RET 4`): `value` stored at `this + 0xC4`.
pub fn fn_00c1a640(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr().wrapping_add(0xc4), value);
}

/// `00a63520` (`cdecl`, `(class name)`): removes a class's loader (the counterpart of
/// `NiStream::RegisterLoader`), and `00c25ac0` (no arguments), the tail call that ends
/// `fn_00c1a7d0`.
const UNREGISTER_LOADER: u32 = 0x00a6_3520;
const AFTER_UNREGISTER_00C25AC0: u32 = 0x00c2_5ac0;
/// The class names `fn_00c1a7d0` unregisters, in its order (the pushed immediates).
const UNREGISTERED_NAMES: [u32; 51] = [
    0x010b_d404,
    0x010b_d3f4,
    0x010b_d3e0,
    0x010b_d3c0,
    0x010b_d39c,
    0x010b_d384,
    0x010b_d368,
    0x010b_d350,
    0x010b_d33c,
    0x010b_d320,
    0x010b_d30c,
    0x010b_d2f4,
    0x010b_d2e0,
    0x010b_d2c8,
    0x010b_d2bc,
    0x010b_d2a4,
    0x010b_d290,
    0x010b_d27c,
    0x010b_d264,
    0x010b_d244,
    0x010b_d224,
    0x010b_d204,
    0x010b_d1e8,
    0x010b_d1c8,
    0x010b_d1a8,
    0x010b_d190,
    0x010b_d174,
    0x010b_d158,
    0x010b_d13c,
    0x010b_d124,
    0x010b_d108,
    0x010b_d0ec,
    0x010b_d0d4,
    0x010b_d0b8,
    0x010b_d098,
    0x010b_d07c,
    0x010b_d05c,
    0x010b_d048,
    0x010b_d02c,
    0x010b_d010,
    0x010b_cff8,
    0x010b_cfe0,
    0x010b_cfc4,
    0x010b_cfac,
    0x010b_cf94,
    0x010b_cf80,
    0x010b_cf6c,
    0x010b_cf54,
    0x010b_cf34,
    0x010b_cf20,
    0x010b_cf04,
];

// Translated from 00c1a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The exit handler `fn_00c1ad50` registers (Ghidra made no function here; unnamed in the
/// map; no arguments): when the loaders were registered (byte `012024d0`), the flag is
/// cleared, each of the 51 class names is unregistered (`00a63520`) and `00c25ac0` runs.
pub fn fn_00c1a7d0(e: &mut Engine) {
    if e.global::<u8>(LOADERS_REGISTERED) == 0 {
        return;
    }
    e.set_global::<u8>(LOADERS_REGISTERED, 0);
    for name in UNREGISTERED_NAMES {
        e.call(UNREGISTER_LOADER, &args![name]);
    }
    e.call(AFTER_UNREGISTER_00C25AC0, &args![]);
}

// The tall-grass geometry: `TallGrassTriStrips` (vtable `010bcc90`, `NiRTTI` `012024b8`
// built by `00fbaed0`) and `TallGrassTriShape` (vtable `010bcdc8`, `NiRTTI` `012024c0`
// built by `00fbaef0`), names from those records. On PC both keep `iInstanceCount` (Xbox
// PDB) at +0xc4 and `pLocalBound` (an `NiBound`: centre, radius) at +0xc8, and the
// world bound pointer (`NiAVObject::m_kWorldBound`) at +0x20.

/// The instance count, the local bound and the world bound pointer.
const INSTANCE_COUNT: u32 = 0xc4;
const LOCAL_BOUND: u32 = 0xc8;
const WORLD_BOUND: u32 = 0x20;
/// `NiTriStrips` and `NiTriShape` methods (engine map; `thiscall`, one word):
/// `RenderImmediate`, `OnlyRenderImmediate`, `GetViewerStrings`, and
/// `NiTriShape::CreateClone` and the `NiTriShape` constructor `NiTriShape_ov2`.
const STRIPS_RENDER_IMMEDIATE: u32 = 0x00a7_1d30;
const STRIPS_ONLY_RENDER_IMMEDIATE: u32 = 0x00a7_1d70;
const STRIPS_GET_VIEWER_STRINGS: u32 = 0x00a7_1e70;
const SHAPE_RENDER_IMMEDIATE: u32 = 0x00a7_45c0;
const SHAPE_ONLY_RENDER_IMMEDIATE: u32 = 0x00a7_4600;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
const SHAPE_CREATE_CLONE: u32 = 0x00a7_4640;
const SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
/// `NiBound::GetViewerString` (`thiscall`, one word: label), `NiGetViewerString_ov6`
/// (`cdecl`, two words: label, value), `NiBound::Update` (`thiscall`, two words) and the
/// `NiTArray` growth and element store (`thiscall`, one and two words).
const BOUND_GET_VIEWER_STRING: u32 = 0x00a7_fb20;
const NI_GET_VIEWER_STRING_OV6: u32 = 0x00a7_d490;
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// The labels `pLocalBound` (`010bcda8`) and `instance count` (`010bcd98`).
const LOCAL_BOUND_LABEL: u32 = 0x010b_cda8;
const INSTANCE_COUNT_LABEL: u32 = 0x010b_cd98;
/// `TallGrassTriShape`'s vtable and `NiRTTI` record.
const GRASS_SHAPE_VTABLE: u32 = 0x010b_cdc8;
const GRASS_SHAPE_RTTI: u32 = 0x0120_24c0;

/// The render slots' shared shape: the world bound becomes the local one (the centre's
/// three words copied, the radius through the FPU, quieted); with instances the call
/// goes on to `render` (a jump, its `EAX` answered), otherwise the world bound pointer
/// still in `EAX` is answered.
fn render_with_local_bound(e: &mut Engine, this: u32, renderer: u32, render: u32) -> u32 {
    // The exe's order: x, radius, world bound pointer read; x, y stored; z read; radius
    // stored; z stored.
    let x = e.mem.u32(this + LOCAL_BOUND);
    let radius = x87_quiet_bits(e.mem.u32(this + LOCAL_BOUND + 0xc));
    let world = e.mem.u32(this + WORLD_BOUND);
    e.mem.set_u32(world, x);
    let y = e.mem.u32(this + LOCAL_BOUND + 4);
    e.mem.set_u32(world + 4, y);
    let z = e.mem.u32(this + LOCAL_BOUND + 8);
    e.mem.set_u32(world + 0xc, radius);
    e.mem.set_u32(world + 8, z);
    if e.mem.u16(this + INSTANCE_COUNT) != 0 {
        return e.call(render, &args![this, renderer]).u32();
    }
    world
}

/// `NiTArray::Add` as the viewer-string slots inline it (grow to size plus `m_usGrowBy`
/// when full, then store at the old size through a stack word); answers the store's
/// `EAX`.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) -> u32 {
    let size = u32::from(e.mem.u16(strings + 0xa));
    let max = u32::from(e.mem.u16(strings + 0x8));
    if size >= max {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(ARRAY_SET_SIZE_0096AD30, &args![strings, grow + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, slot])
            .u32()
    })
}

/// The viewer strings' shared shape: the base's, then the local bound's string and the
/// instance count's appended.
fn grass_viewer_strings(e: &mut Engine, this: u32, strings: u32, base: u32) -> u32 {
    e.call(base, &args![this, strings]);
    let text = e
        .call(
            BOUND_GET_VIEWER_STRING,
            &args![this + LOCAL_BOUND, LOCAL_BOUND_LABEL],
        )
        .u32();
    append_viewer_string(e, strings, text);
    let count = u32::from(e.mem.u16(this + INSTANCE_COUNT));
    let text = e
        .call(
            NI_GET_VIEWER_STRING_OV6,
            &args![INSTANCE_COUNT_LABEL, count],
        )
        .u32();
    append_viewer_string(e, strings, text)
}

// Translated from 00c1a500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriStrips` vtable slot +0xf4 (`thiscall`, one word: the renderer): the
/// world bound set from the local bound, then with instances
/// `NiTriStrips::RenderImmediate`.
pub fn fn_00c1a500(e: &mut Engine, this: Ptr, renderer: u32) -> u32 {
    render_with_local_bound(e, this.addr(), renderer, STRIPS_RENDER_IMMEDIATE)
}

// Translated from 00c1a550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriStrips` vtable slot +0xf8, as `fn_00c1a500` with
/// `NiTriStrips::OnlyRenderImmediate`.
pub fn fn_00c1a550(e: &mut Engine, this: Ptr, renderer: u32) -> u32 {
    render_with_local_bound(e, this.addr(), renderer, STRIPS_ONLY_RENDER_IMMEDIATE)
}

// Translated from 00c1a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriStrips` vtable slot +0x60 (`GetViewerStrings`; `thiscall`, one word: an
/// `NiTArray` of strings): `NiTriStrips::GetViewerStrings`, then the local bound's string
/// (`pLocalBound`) and the instance count's (`instance count`) appended. Ends in the
/// element store; its `EAX` is answered.
pub fn fn_00c1a5a0(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    grass_viewer_strings(e, this.addr(), strings, STRIPS_GET_VIEWER_STRINGS)
}

// Translated from 00c1a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +0x48 (`CreateClone`; `thiscall`, one word), a jump to
/// `NiTriShape::CreateClone`; its answer.
pub fn fn_00c1a630(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    e.call(SHAPE_CREATE_CLONE, &args![this, process]).u32()
}

// Translated from 00c1a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +8 (`GetRTTI`): its `NiRTTI` record `012024c0`.
pub fn fn_00c1a650(_e: &mut Engine) -> u32 {
    GRASS_SHAPE_RTTI
}

// Translated from 00c1a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a `TallGrassTriShape` constructor (`thiscall`, one word
/// passed on): `NiTriShape::NiTriShape_ov2` with the word, then the class's vtable
/// `010bcdc8`. Answers `this`.
pub fn fn_00c1a660(e: &mut Engine, this: Ptr, data: u32) -> Ptr {
    e.call(SHAPE_CONSTRUCT, &args![this, data]);
    e.mem.set_u32(this.addr(), GRASS_SHAPE_VTABLE);
    this
}

// Translated from 00c1a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +0xbc (`thiscall`, no words): `NiBound::Update` on the
/// world bound with the local bound (+0xc8) and the transform at +0x68; ends in that
/// call, so its `EAX` is answered.
pub fn fn_00c1a680(e: &mut Engine, this: Ptr) -> u32 {
    let world = e.mem.u32(this.addr() + WORLD_BOUND);
    e.call(
        BOUND_UPDATE,
        &args![world, this.addr() + LOCAL_BOUND, this.addr() + 0x68],
    )
    .u32()
}

// Translated from 00c1a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +0xf4, as `fn_00c1a500` with
/// `NiTriShape::RenderImmediate`.
pub fn fn_00c1a6a0(e: &mut Engine, this: Ptr, renderer: u32) -> u32 {
    render_with_local_bound(e, this.addr(), renderer, SHAPE_RENDER_IMMEDIATE)
}

// Translated from 00c1a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +0xf8, as `fn_00c1a500` with
/// `NiTriShape::OnlyRenderImmediate`.
pub fn fn_00c1a6f0(e: &mut Engine, this: Ptr, renderer: u32) -> u32 {
    render_with_local_bound(e, this.addr(), renderer, SHAPE_ONLY_RENDER_IMMEDIATE)
}

// Translated from 00c1a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes):
/// `TallGrassTriShape` vtable slot +0x60, as `fn_00c1a5a0` with
/// `NiTriShape::GetViewerStrings`.
pub fn fn_00c1a740(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    grass_viewer_strings(e, this.addr(), strings, SHAPE_GET_VIEWER_STRINGS)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_a500, fn_00c1a500(Ptr, u32) -> u32),
        entry!(0x00c1_a550, fn_00c1a550(Ptr, u32) -> u32),
        entry!(0x00c1_a5a0, fn_00c1a5a0(Ptr, u32) -> u32),
        entry!(0x00c1_a630, fn_00c1a630(Ptr, u32) -> u32),
        entry!(0x00c1_a650, fn_00c1a650() -> u32),
        entry!(0x00c1_a660, fn_00c1a660(Ptr, u32) -> Ptr),
        entry!(0x00c1_a680, fn_00c1a680(Ptr) -> u32),
        entry!(0x00c1_a6a0, fn_00c1a6a0(Ptr, u32) -> u32),
        entry!(0x00c1_a6f0, fn_00c1a6f0(Ptr, u32) -> u32),
        entry!(0x00c1_a740, fn_00c1a740(Ptr, u32) -> u32),
        entry!(0x00c1_ad50, fn_00c1ad50(Ptr) -> Ptr),
        entry!(0x00c1_aa00, fn_00c1aa00()),
        entry!(0x00c1a640, fn_00c1a640(Ptr, u16)),
        entry!(0x00c1a7d0, fn_00c1a7d0()),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_ad50, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00c1_aa00, 0x00c1_a7d0]]
        );
        e.call(0x00c1_ad50, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_012024D4), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn registers_every_loader_once_and_keeps_the_thread_id() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x4321u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c1_aa00, &args![]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(log.len(), 54);
        assert_eq!(log[0], (REGISTER_LOADER, vec![0x010b_d404, 0x00c2_1e90]));
        assert_eq!(log[53].0, GET_CURRENT_THREAD_ID);
        assert_eq!(e.mem.u32(START_THREAD), 0x4321);
        assert_eq!(e.mem.u8(LOADERS_REGISTERED), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_aa00, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c1a640_stores_the_word_at_0xc4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        e.call(0x00c1a640, &args![this, 0x1_2345u32]);
        assert_eq!(e.mem.u16(this + 0xc4), 0x2345);
        assert_eq!(e.mem.u16(this + 0xc6), 0);
    }

    #[test]
    fn fn_00c1a7d0_unregisters_once() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(UNREGISTER_LOADER, |_, _| Ret::default());
        e.register(AFTER_UNREGISTER_00C25AC0, |_, _| Ret::default());
        e.map(LOADERS_REGISTERED, 1);
        e.call(0x00c1a7d0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global::<u8>(LOADERS_REGISTERED, 1);
        e.call(0x00c1a7d0, &args![]);
        assert_eq!(e.global::<u8>(LOADERS_REGISTERED), 0);
        let log = e.call_log.take().unwrap();
        let names: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == UNREGISTER_LOADER)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(names.len(), 51);
        assert_eq!(names[0], 0x010b_d404);
        assert_eq!(names[50], 0x010b_cf04);
        assert_eq!(log.last().unwrap().0, AFTER_UNREGISTER_00C25AC0);
    }
}

#[cfg(test)]
mod grass_tests {
    use super::*;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn grass(e: &mut Engine, count: u16) -> (u32, u32) {
        let this = e.mem.alloc(0x110);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(this + WORLD_BOUND, world);
        e.mem.set_u16(this + INSTANCE_COUNT, count);
        for word in 0..3 {
            e.mem.set_u32(this + LOCAL_BOUND + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(this + LOCAL_BOUND + 0xc, 0x7f80_0005);
        (this, world)
    }

    #[test]
    fn render_slots_copy_the_bound_and_render_only_with_instances() {
        let mut e = Engine::new();
        let slots = [
            (0x00c1_a500u32, STRIPS_RENDER_IMMEDIATE),
            (0x00c1_a550, STRIPS_ONLY_RENDER_IMMEDIATE),
            (0x00c1_a6a0, SHAPE_RENDER_IMMEDIATE),
            (0x00c1_a6f0, SHAPE_ONLY_RENDER_IMMEDIATE),
        ];
        for (_, target) in slots {
            e.register(target, |_, a| answer(a[1] + 1));
        }
        for (address, target) in slots {
            let (this, world) = grass(&mut e, 2);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0x40u32]).u32(), 0x41);
            assert_eq!(calls_to(&e, target), vec![vec![this, 0x40]]);
            assert_eq!(
                [0u32, 4, 8, 0xc].map(|o| e.mem.u32(world + o)),
                [0x100, 0x101, 0x102, 0x7fc0_0005]
            );
            let (this, world) = grass(&mut e, 0);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0x40u32]).u32(), world);
            assert!(calls_to(&e, target).is_empty());
        }
    }

    #[test]
    fn viewer_strings_add_the_bound_and_the_count() {
        let mut e = Engine::new();
        for base in [STRIPS_GET_VIEWER_STRINGS, SHAPE_GET_VIEWER_STRINGS] {
            e.register(base, |_, _| Ret::default());
        }
        e.register(BOUND_GET_VIEWER_STRING, |_, _| answer(0x5100));
        e.register(NI_GET_VIEWER_STRING_OV6, |_, a| answer(0x5200 + a[1]));
        e.register(ARRAY_SET_SIZE_0096AD30, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_0096AE90, |e, a| answer(e.mem.u32(a[2])));
        for (address, base) in [
            (0x00c1_a5a0u32, STRIPS_GET_VIEWER_STRINGS),
            (0x00c1_a740, SHAPE_GET_VIEWER_STRINGS),
        ] {
            let (this, _) = grass(&mut e, 3);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 0x8, 1);
            e.mem.set_u16(strings + 0xa, 1);
            e.mem.set_u16(strings + 0xe, 4);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, strings]).u32(), 0x5203);
            assert_eq!(calls_to(&e, base), vec![vec![this, strings]]);
            assert_eq!(
                calls_to(&e, BOUND_GET_VIEWER_STRING),
                vec![vec![this + LOCAL_BOUND, LOCAL_BOUND_LABEL]]
            );
            assert_eq!(
                calls_to(&e, NI_GET_VIEWER_STRING_OV6),
                vec![vec![INSTANCE_COUNT_LABEL, 3]]
            );
            // The double does not grow the array, so both appends find it full.
            assert_eq!(
                calls_to(&e, ARRAY_SET_SIZE_0096AD30),
                vec![vec![strings, 5], vec![strings, 5]]
            );
        }
    }

    #[test]
    fn shape_clone_rtti_constructor_and_bound_update() {
        let mut e = Engine::new();
        e.register(SHAPE_CREATE_CLONE, |_, a| answer(a[1] + 2));
        e.register(SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(BOUND_UPDATE, |_, _| answer(0x77));
        assert_eq!(e.call(0x00c1_a630, &args![0x9000u32, 0x30u32]).u32(), 0x32);
        assert_eq!(e.call(0x00c1_a650, &[]).u32(), GRASS_SHAPE_RTTI);
        let (this, world) = grass(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_a660, &args![this, 0x6000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), GRASS_SHAPE_VTABLE);
        assert_eq!(calls_to(&e, SHAPE_CONSTRUCT), vec![vec![this, 0x6000]]);
        assert_eq!(e.call(0x00c1_a680, &args![this]).u32(), 0x77);
        assert_eq!(
            calls_to(&e, BOUND_UPDATE),
            vec![vec![world, this + LOCAL_BOUND, this + 0x68]]
        );
    }
}
