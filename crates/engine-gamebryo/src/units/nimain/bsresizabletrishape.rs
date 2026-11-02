//! `NiMain/bsresizabletrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSResizableTriShape::ms_RTTI` (the class's `NiRTTI` record).
const BS_RESIZABLE_TRI_SHAPE_RTTI: u32 = 0x011f_5b34;

// Translated from 00a86d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_resizable_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_RESIZABLE_TRI_SHAPE_RTTI)
}

/// `00a98eb0`: the jump target of `00a86d40`.
const JUMP_TARGET_00A86D40: u32 = 0x00a9_8eb0;

/// `00a74500`: the jump target of `00a87190`.
const JUMP_TARGET_00A87190: u32 = 0x00a7_4500;

/// `00a74510`: the jump target of `00a877f0`.
const JUMP_TARGET_00A877F0: u32 = 0x00a7_4510;

// Translated from 00a86d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::LinkObject` (Xbox PDB): a 5-byte jump to `00a98eb0` (engine map: `NiParticleMeshes::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSResizableTriShape@@UAAXAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A86D40, &args![this, stream]);
}

// Translated from 00a87190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a74500` (engine map: `NiParticleMeshes::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSResizableTriShape@@UAA_NAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A87190, &args![this, stream]).bool()
}

// Translated from 00a877f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::IsEqual` (Xbox PDB): a 5-byte jump to `00a74510` (engine map: `NiParticleMeshes::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSResizableTriShape@@UAA_NPAVNiObject@@@Z`.
pub fn bs_resizable_tri_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A877F0, &args![this, other]).bool()
}

/// The `MemoryManager` singleton and its `Allocate` (`thiscall` on it, `(size)`),
/// `NiBound::Update` (`thiscall` on the bound, `(&local bound, &transform)`) and the
/// default bound's centre (`011f426c`, three floats).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
const DEFAULT_BOUND_CENTER: u32 = 0x011f_426c;

// Translated from 00c1a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::UpdateWorldBound` (Xbox PDB): without a world bound (+0x20), a
/// 0x10-byte block (`MemoryManager::Allocate`) is given to the object's vtable slot +0x78
/// and the bound it then holds gets the centre at `011f426c` and radius 0. Then the world
/// bound is updated (`NiBound::Update`) from the local bound (`LocalBound`, +0xC8) and the
/// world transform (+0x68).
pub fn bs_resizable_tri_shape_update_world_bound(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x20) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        e.vcall(this.addr(), 0x78, &args![block]);
        let bound = e.mem.u32(this.addr() + 0x20);
        for word in 0..3u32 {
            let value = e.global::<u32>(DEFAULT_BOUND_CENTER + 4 * word);
            e.mem.set_u32(bound + 4 * word, value);
        }
        e.mem.set_u32(bound + 0xc, x87_store32(0.0));
    }
    let bound = e.mem.u32(this.addr() + 0x20);
    e.call(
        BOUND_UPDATE,
        &args![bound, this.addr() + 0xc8, this.addr() + 0x68],
    );
}

/// `BSResizableTriShape` fields (Xbox PDB names; PC offsets, `NiTriShape` being 0xc4
/// bytes): `uiNumPrimitives` +0xc4 and `LocalBound` +0xc8 (an `NiBound`: centre +0xc8,
/// +0xcc, +0xd0, radius +0xd4). `NiAVObject`'s world bound pointer is at +0x20.
const NUM_PRIMITIVES: u32 = 0xc4;
const LOCAL_BOUND: u32 = 0xc8;
const WORLD_BOUND: u32 = 0x20;
/// The class's size and vtable (`0109e704`, the scalar deleting destructor's).
const BS_RESIZABLE_TRI_SHAPE_SIZE: u32 = 0xd8;
const BS_RESIZABLE_TRI_SHAPE_VTABLE: u32 = 0x0109_e704;
/// `NiTriShape::RenderImmediate` (`00a745c0`) and `OnlyRenderImmediate` (`00a74600`),
/// thiscall with the renderer (`RET 4`).
const TRI_SHAPE_RENDER_IMMEDIATE: u32 = 0x00a7_45c0;
const TRI_SHAPE_ONLY_RENDER_IMMEDIATE: u32 = 0x00a7_4600;
/// `NiTriShape`'s destructor body (`00a89760`, thiscall), its constructor with the data
/// (`00a74480`, `NiTriShape_ov2`, thiscall: data), its default constructor (`00a744a0`) and
/// the member copy (`00a744c0`, thiscall on the source: clone, cloning process).
const TRI_SHAPE_DESTRUCT: u32 = 0x00a8_9760;
const TRI_SHAPE_CONSTRUCT_WITH_DATA: u32 = 0x00a7_4480;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_44a0;
const TRI_SHAPE_COPY_MEMBERS: u32 = 0x00a7_44c0;
/// The base `SaveBinary` (`00a71d10`) and `LoadBinary` (`00a98ea0`), thiscall: stream (the
/// engine map names both `NiParticleMeshes`, folded code).
const BASE_SAVE_BINARY: u32 = 0x00a7_1d10;
const BASE_LOAD_BINARY: u32 = 0x00a9_8ea0;
/// `NiTriShape::GetViewerStrings` (`00a74690`, thiscall: string array).
const TRI_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
/// `NiNew`'s allocator (`00aa13e0`, cdecl: size) and the sized free (`00aa1460`, cdecl:
/// block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), `NiGetViewerString`
/// (`00a7d330`) and `NiGetViewerString_ov8` (`00a7d510`), and `"Num Active Primitives"`.
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const TEXT_NUM_ACTIVE_PRIMITIVES: u32 = 0x0109_e800;

/// The local bound copied into the world bound: the centre words as they are, the radius
/// through `FLD`/`FSTP` (a signalling NaN comes out quiet).
fn copy_bound_to_world(e: &mut Engine, this: u32) {
    let x = e.mem.u32(this + LOCAL_BOUND);
    let radius = x87_quiet_bits(e.mem.u32(this + LOCAL_BOUND + 0xc));
    let bound = e.mem.u32(this + WORLD_BOUND);
    e.mem.set_u32(bound, x);
    let y = e.mem.u32(this + LOCAL_BOUND + 4);
    e.mem.set_u32(bound + 4, y);
    let z = e.mem.u32(this + LOCAL_BOUND + 8);
    e.mem.set_u32(bound + 0xc, radius);
    e.mem.set_u32(bound + 8, z);
}

// Translated from 00a86d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::GetWorldBound` (engine map; the body answers the address of
/// `LocalBound`, +0xc8).
pub fn bs_resizable_tri_shape_get_world_bound(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(LOCAL_BOUND))
}

// Translated from 00a86d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::SetLocalBound` (Xbox PDB; thiscall, `RET 0x10`: centre x, y, z,
/// radius): sets `LocalBound` and the world bound (+0x20) to it. The radius goes through
/// `FLD`/`FST` (quieting a signalling NaN); the centre words are copied as they are.
pub fn bs_resizable_tri_shape_set_localbound(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    radius: u32,
) {
    let base = this.addr();
    let radius = x87_quiet_bits(radius);
    e.mem.set_u32(base + LOCAL_BOUND + 0xc, radius);
    e.mem.set_u32(base + LOCAL_BOUND, x);
    e.mem.set_u32(base + LOCAL_BOUND + 4, y);
    e.mem.set_u32(base + LOCAL_BOUND + 8, z);
    let bound = e.mem.u32(base + WORLD_BOUND);
    e.mem.set_u32(bound, x);
    e.mem.set_u32(bound + 4, y);
    e.mem.set_u32(bound + 0xc, radius);
    e.mem.set_u32(bound + 8, z);
}

// Translated from 00a86da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape`'s scalar deleting destructor (Xbox PDB; thiscall, `RET 4`): the
/// class's vtable, `NiTriShape`'s destructor body (`00a89760`) and, with bit 0 of `flags`,
/// the 0xd8 bytes given back (`00aa1460`). Answers `this`.
pub fn bs_resizable_tri_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), BS_RESIZABLE_TRI_SHAPE_VTABLE);
    e.call(TRI_SHAPE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, BS_RESIZABLE_TRI_SHAPE_SIZE]);
    }
    this
}

// Translated from 00a86dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape`'s constructor with the geometry data (by the vtable it stores;
/// unnamed in the engine map; thiscall, `RET 4`): `NiTriShape`'s constructor with the data
/// (`00a74480`), the class's vtable and no active primitives. Answers `this`.
pub fn fn_00a86dd0(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(TRI_SHAPE_CONSTRUCT_WITH_DATA, &args![this, data]);
    e.mem.set_u32(this.addr(), BS_RESIZABLE_TRI_SHAPE_VTABLE);
    e.mem.set_u32(this.addr() + NUM_PRIMITIVES, 0);
    this
}

// Translated from 00a86e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::RenderImmediate` (Xbox PDB; thiscall, the renderer on the stack):
/// the local bound copied into the world bound (the radius through `FLD`/`FSTP`), then a
/// jump to `NiTriShape::RenderImmediate` (`00a745c0`) with the same stack word.
pub fn bs_resizable_tri_shape_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    copy_bound_to_world(e, this.addr());
    e.call(TRI_SHAPE_RENDER_IMMEDIATE, &args![this, renderer]);
}

// Translated from 00a86e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::OnlyRenderImmediate` (Xbox PDB): as
/// [`bs_resizable_tri_shape_render_immediate`], then a jump to
/// `NiTriShape::OnlyRenderImmediate` (`00a74600`).
pub fn bs_resizable_tri_shape_only_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    copy_bound_to_world(e, this.addr());
    e.call(TRI_SHAPE_ONLY_RENDER_IMMEDIATE, &args![this, renderer]);
}

// Translated from 00a86e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::CreateClone` (Xbox PDB; thiscall, `RET 4`: cloning process): 0xd8
/// bytes (`00aa13e0`) constructed as an `NiTriShape` (`00a744a0`) with the class's vtable
/// (null when the allocation fails), the members copied into it (`00a744c0`) and the
/// active primitive count copied (written through the null clone too, as the exe does).
pub fn bs_resizable_tri_shape_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![BS_RESIZABLE_TRI_SHAPE_SIZE])
        .u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(TRI_SHAPE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, BS_RESIZABLE_TRI_SHAPE_VTABLE);
        block
    };
    e.call(TRI_SHAPE_COPY_MEMBERS, &args![this, clone, cloning]);
    let count = e.mem.u32(this.addr() + NUM_PRIMITIVES);
    e.mem.set_u32(clone.wrapping_add(NUM_PRIMITIVES), count);
    Ptr::new(clone)
}

// Translated from 00a86ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::SaveBinary` (Xbox PDB; thiscall, `RET 4`): the base `SaveBinary`
/// (`00a71d10`), then the active primitive count (4 bytes) through the stream's output
/// (`NiStream` +0x250, write function at +0xc; the element size in the argument's stack
/// word).
pub fn bs_resizable_tri_shape_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let wrote = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(
            write,
            &args![output, this.addr() + NUM_PRIMITIVES, 4u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(output + 4).wrapping_add(wrote);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a86f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::LoadBinary` (Xbox PDB; thiscall, `RET 4`): the base `LoadBinary`
/// (`00a98ea0`), then the active primitive count (4 bytes) through the stream's input
/// (`NiStream` +0x24c, read function at +8).
pub fn bs_resizable_tri_shape_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let got = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(
            read,
            &args![input, this.addr() + NUM_PRIMITIVES, 4u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(input + 4).wrapping_add(got);
    e.mem.set_u32(input + 4, position);
}

/// Adds the string word at `slot` at the array's end (`SetSize` when full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![strings, grow.wrapping_add(index)],
        );
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a86f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriShape::GetViewerStrings` (`00a74690`), then the class name (the first word of
/// `ms_RTTI`, `011f5b34`) and `"Num Active Primitives"` with the count, each held in the
/// argument word and added through `SetSize`/`SetAt`.
pub fn bs_resizable_tri_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TRI_SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(BS_RESIZABLE_TRI_SHAPE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings.addr(), slot.addr());
        let count = e.mem.u32(this.addr() + NUM_PRIMITIVES);
        let text = e
            .call(
                VIEWER_STRING_UNSIGNED,
                &args![TEXT_NUM_ACTIVE_PRIMITIVES, count],
            )
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings.addr(), slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_6d30, bs_resizable_tri_shape_get_world_bound(Ptr) -> Ptr),
        entry!(
            0x00a8_6d50,
            bs_resizable_tri_shape_set_localbound(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00a8_6da0,
            bs_resizable_tri_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00a8_6dd0, fn_00a86dd0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a8_6e00,
            bs_resizable_tri_shape_render_immediate(Ptr, Ptr)
        ),
        entry!(
            0x00a8_6e40,
            bs_resizable_tri_shape_only_render_immediate(Ptr, Ptr)
        ),
        entry!(
            0x00a8_6e80,
            bs_resizable_tri_shape_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a8_6ed0, bs_resizable_tri_shape_save_binary(Ptr, Ptr)),
        entry!(0x00a8_6f10, bs_resizable_tri_shape_load_binary(Ptr, Ptr)),
        entry!(
            0x00a8_6f50,
            bs_resizable_tri_shape_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a86d20, bs_resizable_tri_shape_get_rtti() -> Ptr),
        entry!(0x00a8_6d40, bs_resizable_tri_shape_link_object(Ptr, Ptr)),
        entry!(0x00a8_7190, bs_resizable_tri_shape_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a8_77f0, bs_resizable_tri_shape_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c1a4a0, bs_resizable_tri_shape_update_world_bound(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_resizable_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a86d20, &args![]).u32(),
            BS_RESIZABLE_TRI_SHAPE_RTTI
        );
    }

    #[test]
    fn thunk_00a86d40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_6d40;
        const TARGET: u32 = 0x00a9_8eb0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a87190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_7190;
        const TARGET: u32 = 0x00a7_4500;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a877f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_77f0;
        const TARGET: u32 = 0x00a7_4510;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn update_world_bound_makes_a_bound_when_missing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(BOUND_UPDATE, |_, _| Ret::default());
        e.register(0x0090_0078, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x1f];
        slots[0x1e] = 0x0090_0078;
        e.put_vtable(0x0090_1000, &slots);
        e.map(DEFAULT_BOUND_CENTER, 12);
        e.set_global::<f32>(DEFAULT_BOUND_CENTER + 4, 2.5);
        let this = e.mem.alloc(0xd8);
        e.mem.set_u32(this, 0x0090_1000);
        e.mem.set_f32(this + 0x20, 0.0);
        e.call(0x00c1a4a0, &args![this]);
        let bound = e.mem.u32(this + 0x20);
        assert_ne!(bound, 0);
        assert_eq!(e.mem.f32(bound + 4), 2.5);
        assert_eq!(e.mem.f32(bound + 0xc), 0.0);
        let updates: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BOUND_UPDATE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(updates, vec![vec![bound, this + 0xc8, this + 0x68]]);
        // With a bound: only the update.
        e.mem.set_f32(bound + 4, 7.0);
        e.call(0x00c1a4a0, &args![this]);
        assert_eq!(e.mem.f32(bound + 4), 7.0);
        let allocations = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMORY_ALLOCATE)
            .count();
        assert_eq!(allocations, 1);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    /// A signalling NaN (quiet bit clear).
    const SIGNALLING_NAN: u32 = 0x7f80_0001;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn shape(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xd8);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(this + WORLD_BOUND, bound);
        this
    }

    #[test]
    fn get_world_bound_answers_the_bound_at_0xc8() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8_6d30, &args![0x1000u32]).u32(), 0x10c8);
    }

    #[test]
    fn set_localbound_sets_both_bounds_quieting_the_radius() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        e.call(
            0x00a8_6d50,
            &args![this, 1u32, 2u32, SIGNALLING_NAN, SIGNALLING_NAN],
        );
        let bound = e.mem.u32(this + WORLD_BOUND);
        let quiet = SIGNALLING_NAN | 0x0040_0000;
        for base in [this + LOCAL_BOUND, bound] {
            assert_eq!(e.mem.u32(base), 1);
            assert_eq!(e.mem.u32(base + 4), 2);
            assert_eq!(e.mem.u32(base + 8), SIGNALLING_NAN);
            assert_eq!(e.mem.u32(base + 0xc), quiet);
        }
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_on_request() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = shape(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_6da0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_RESIZABLE_TRI_SHAPE_VTABLE);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00a8_6da0, &args![this, 0x101u32]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, 0xd8]]);
        assert_eq!(calls_to(&e, TRI_SHAPE_DESTRUCT).len(), 2);
    }

    #[test]
    fn constructor_with_data_clears_the_primitive_count() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_CONSTRUCT_WITH_DATA, |_, _| Ret::default());
        let this = shape(&mut e);
        e.mem.set_u32(this + NUM_PRIMITIVES, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_6dd0, &args![this, 0x4000u32]).u32(), this);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_CONSTRUCT_WITH_DATA),
            vec![vec![this, 0x4000]]
        );
        assert_eq!(e.mem.u32(this), BS_RESIZABLE_TRI_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + NUM_PRIMITIVES), 0);
    }

    fn render_case(address: u32, target: u32) {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret::default());
        let this = shape(&mut e);
        for (i, value) in [3u32, 4, 5, SIGNALLING_NAN].iter().enumerate() {
            e.mem.set_u32(this + LOCAL_BOUND + 4 * i as u32, *value);
        }
        e.call_log = Some(vec![]);
        e.call(address, &args![this, 0x5000u32]);
        assert_eq!(calls_to(&e, target), vec![vec![this, 0x5000]]);
        let bound = e.mem.u32(this + WORLD_BOUND);
        assert_eq!(
            (0..4).map(|i| e.mem.u32(bound + 4 * i)).collect::<Vec<_>>(),
            vec![3, 4, 5, SIGNALLING_NAN | 0x0040_0000]
        );
    }

    #[test]
    fn render_immediate_copies_the_bound_first() {
        render_case(0x00a8_6e00, TRI_SHAPE_RENDER_IMMEDIATE);
    }

    #[test]
    fn only_render_immediate_copies_the_bound_first() {
        render_case(0x00a8_6e40, TRI_SHAPE_ONLY_RENDER_IMMEDIATE);
    }

    #[test]
    fn create_clone_copies_the_members_and_the_count() {
        let mut e = Engine::new();
        e.register(TRI_SHAPE_CONSTRUCT, |_, _| Ret::default());
        e.register(TRI_SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = shape(&mut e);
        e.mem.set_u32(this + NUM_PRIMITIVES, 12);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a8_6e80, &args![this, 0x6000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0xd8]]);
        assert_eq!(calls_to(&e, TRI_SHAPE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_COPY_MEMBERS),
            vec![vec![this, clone, 0x6000]]
        );
        assert_eq!(e.mem.u32(clone), BS_RESIZABLE_TRI_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(clone + NUM_PRIMITIVES), 12);
    }

    #[test]
    fn save_binary_writes_the_primitive_count() {
        const WRITE: u32 = 0x0370_0000;
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, output);
        let this = shape(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6ed0, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls_to(&e, WRITE);
        assert_eq!((writes[0][1], writes[0][2]), (this + NUM_PRIMITIVES, 4));
        assert_eq!(e.mem.u32(output + 4), 4);
    }

    #[test]
    fn load_binary_reads_the_primitive_count() {
        const READ: u32 = 0x0371_0000;
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            e.mem.set_u32(a[1], 33);
            a[2].into_ret()
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x24c, input);
        let this = shape(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6f10, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + NUM_PRIMITIVES), 33);
        assert_eq!(e.mem.u32(input + 4), 4);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_count() {
        let mut e = Engine::new();
        e.mem.map(BS_RESIZABLE_TRI_SHAPE_RTTI, 4);
        e.mem.set_u32(BS_RESIZABLE_TRI_SHAPE_RTTI, 0x0109_0000);
        e.register(TRI_SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        let this = shape(&mut e);
        e.mem.set_u32(this + NUM_PRIMITIVES, 6);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6f50, &args![this, strings]);
        assert_eq!(
            calls_to(&e, TRI_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let base = e.mem.u32(strings + 4);
        assert_eq!(
            (e.mem.u32(base), e.mem.u32(base + 4)),
            (0x0109_0001, TEXT_NUM_ACTIVE_PRIMITIVES + 6)
        );
    }
}
