//! `NiMain/niparticlemeshes.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleMeshes::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_MESHES_RTTI: u32 = 0x011f_5f14;

// Translated from 00a98ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_meshes_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_MESHES_RTTI)
}

/// `00a9b1a0`: the jump target of `00a71d10`.
const JUMP_TARGET_00A71D10: u32 = 0x00a9_b1a0;

/// `00a9b190`: the jump target of `00a74500`.
const JUMP_TARGET_00A74500: u32 = 0x00a9_b190;

/// `00a89770`: the jump target of `00a74510`.
const JUMP_TARGET_00A74510: u32 = 0x00a8_9770;

/// `00a9b180`: the jump target of `00a98ea0`.
const JUMP_TARGET_00A98EA0: u32 = 0x00a9_b180;

/// `00a74780`: the jump target of `00a98eb0`.
const JUMP_TARGET_00A98EB0: u32 = 0x00a7_4780;

/// `00a80400`: the jump target of `00a9b140`.
const JUMP_TARGET_00A9B140: u32 = 0x00a8_0400;

// Translated from 00a71d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::SaveBinary` (Xbox PDB): a 5-byte jump to `00a9b1a0` (engine map: `NiLines::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A71D10, &args![this, stream]);
}

// Translated from 00a74500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a9b190` (engine map: `NiLines::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiParticleMeshes@@UAA_NAAVNiStream@@@Z`.
pub fn ni_particle_meshes_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A74500, &args![this, stream]).bool()
}

// Translated from 00a74510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::IsEqual` (Xbox PDB): a 5-byte jump to `00a89770` (engine map: `NiLines::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiParticleMeshes@@UAA_NPAVNiObject@@@Z`.
pub fn ni_particle_meshes_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A74510, &args![this, other]).bool()
}

// Translated from 00a98ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::LoadBinary` (Xbox PDB): a 5-byte jump to `00a9b180` (engine map: `NiLines::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A98EA0, &args![this, stream]);
}

// Translated from 00a98eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::LinkObject` (Xbox PDB): a 5-byte jump to `00a74780` (engine map: `NiLines::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A98EB0, &args![this, stream]);
}

// Translated from 00a9b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::CopyMembers` (Xbox PDB): a 5-byte jump to `00a80400` (engine map: `NiLines::CopyMembers`), passing on `this` and its two arguments. Return type (void): Xbox PDB public name `?CopyMembers@NiParticleMeshes@@IAAXPAV1@AAVNiCloningProcess@@@Z`.
pub fn ni_particle_meshes_copy_members(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00A9B140, &args![this, first, second]);
}

/// `NiParticleMeshes`'s vtable (`010a0a6c`, RTTI `.?AVNiParticleMeshes@@`).
const NI_PARTICLE_MESHES_VTABLE: u32 = 0x010a_0a6c;
/// The Ni allocator `operator new` (`00aa13e0`, `cdecl`) and the sized delete (`00aa1460`).
const NI_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a9b120` (`thiscall`, no stack words): the constructor body the creation functions run.
const BASE_CONSTRUCT: u32 = 0x00a9_b120;
/// `NiParticleMeshes` fields as the PC code reads them: the time copied from the update
/// data (+0xc4), the byte copied with it (+0xc8, set to 1 by the creation functions), the
/// model data (+0xb8; the object it holds at +0x70 has an `NiAVObject`-style local
/// transform), the world rotation (+0x68, nine floats), translation (+0x8c) and scale
/// (+0x98), the property state (+0x9c).
const UPDATE_TIME: u32 = 0xc4;
const UPDATE_FLAG: u32 = 0xc8;
const MODEL_DATA: u32 = 0xb8;
const MESH_DATA: u32 = 0x70;
/// Base functions (`thiscall`): `NiAVObject::UpdateDownwardPass` (`00a59f90`), `00a59fd0`
/// (the same unit), `NiAVObject::UpdateWorldData` (`00a68c60`), `NiAVObject::UpdateProperties`
/// (`00a5a040`), `00a29680`, `NiGeometry`'s destructor (`00a80580`),
/// `SetSelectiveUpdateFlags` (`00a808f0`), `UpdatePropertiesDownward` (`00a80820`) and
/// `PurgeRendererData` (`00a809b0`), `NiParticles::GetViewerStrings` (`00a9b2e0`), and the
/// model data's `00a98c90` (unit `niparticlemeshesdata.cpp`, no stack words).
const AV_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_9f90;
const AV_00A59FD0: u32 = 0x00a5_9fd0;
const AV_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const AV_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const FN_00A29680: u32 = 0x00a2_9680;
const GEOMETRY_DESTRUCT: u32 = 0x00a8_0580;
const GEOMETRY_SET_SELECTIVE_UPDATE_FLAGS: u32 = 0x00a8_08f0;
const GEOMETRY_UPDATE_PROPERTIES_DOWNWARD: u32 = 0x00a8_0820;
const GEOMETRY_PURGE_RENDERER_DATA: u32 = 0x00a8_09b0;
const PARTICLES_GET_VIEWER_STRINGS: u32 = 0x00a9_b2e0;
const MODEL_DATA_00A98C90: u32 = 0x00a9_8c90;
/// The slots of the object at model data +0x70 the meshes call, by the methods that call
/// them: +0xa0 (selective update flags), +0xa4 (world data), +0xb0 (properties
/// downward), +0xd4 (on visible), +0xd8 (purge).
const SLOT_SELECTIVE_UPDATE_FLAGS: u32 = 0xa0;
const SLOT_UPDATE_WORLD_DATA: u32 = 0xa4;
const SLOT_UPDATE_PROPERTIES_DOWNWARD: u32 = 0xb0;
const SLOT_ON_VISIBLE: u32 = 0xd4;
const SLOT_PURGE_RENDERER_DATA: u32 = 0xd8;
/// The RTTI record whose class name the viewer strings add (`011f5f64`, the record
/// `00a9b1b0` answers; `NiParticles::GetViewerStrings` adds the same name).
const VIEWER_NAME_RTTI: u32 = 0x011f_5f64;
/// `NiGetViewerString` (`00a7d330`), and the `NiTArray<char *>` resize (`0096ad30`) and
/// `SetAt` (`0096ae90`).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

/// The object the meshes forward to (model data +0xb8, then its +0x70).
fn mesh_data(e: &Engine, this: Ptr) -> u32 {
    let model = e.mem.u32(this.addr() + MODEL_DATA);
    e.mem.u32(model + MESH_DATA)
}

// Translated from 00a98e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::UpdateDownwardPass` (Xbox PDB; the update data): the data's time
/// (through the x87 stack) and its byte +4 copied to +0xc4 and +0xc8, then a jump to
/// `NiAVObject::UpdateDownwardPass` with the same data. The jump passes on both stack
/// words (the callee's `RET 8`), the second as it came.
pub fn ni_particle_meshes_update_downward_pass(e: &mut Engine, this: Ptr, data: Ptr, word: u32) {
    let time = x87_quiet_bits(e.mem.u32(data.addr()));
    e.mem.set_u32(this.addr() + UPDATE_TIME, time);
    let flag = e.mem.u8(data.addr() + 4);
    e.mem.set_u8(this.addr() + UPDATE_FLAG, flag);
    // 00a98e49: a JMP, so `00a59f90` (RET 8) gets this function's two stack words.
    e.call(AV_UPDATE_DOWNWARD_PASS, &args![this, data, word]);
}

// Translated from 00a98e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niparticlemeshes.cpp`; the update data): the data's
/// time (through the x87 stack) copied to +0xc4, then a jump to `00a59fd0` with the same
/// data; the jump passes on both stack words (the callee's `RET 8`).
pub fn fn_00a98e50(e: &mut Engine, this: Ptr, data: Ptr, word: u32) {
    let time = x87_quiet_bits(e.mem.u32(data.addr()));
    e.mem.set_u32(this.addr() + UPDATE_TIME, time);
    // 00a98e60: a JMP, so `00a59fd0` (RET 8) gets this function's two stack words.
    e.call(AV_00A59FD0, &args![this, data, word]);
}

// Translated from 00a98e70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiParticleMeshes`): allocates 0xcc bytes (`00aa13e0`) and, when that
/// succeeds, runs `00a9b120`, sets the vtable and the byte at +0xc8 to 1. Answers the
/// object, or null.
pub fn fn_00a98e70(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0xccu32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_PARTICLE_MESHES_VTABLE);
    e.mem.set_u8(object + UPDATE_FLAG, 1);
    Ptr::new(object)
}

// Translated from 00a98ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::_scalar_deleting_destructor_` (Xbox PDB): `NiGeometry`'s destructor
/// (`00a80580`), then the sized delete of 0xcc bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn ni_particle_meshes_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(GEOMETRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xccu32]);
    }
    this
}

// Translated from 00a98f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::CreateClone` (Xbox PDB): a new object as `00a98e70` makes it, then
/// `CopyMembers` (`00a9b140`) from `this` into it; when the allocation failed `CopyMembers`
/// still runs with a null clone. Answers the clone.
pub fn ni_particle_meshes_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(NI_NEW, &args![0xccu32]).u32();
    if clone != 0 {
        e.call(BASE_CONSTRUCT, &args![clone]);
        e.mem.set_u32(clone, NI_PARTICLE_MESHES_VTABLE);
        e.mem.set_u8(clone + UPDATE_FLAG, 1);
    }
    ni_particle_meshes_copy_members(e, this, Ptr::new(clone), cloning);
    Ptr::new(clone)
}

// Translated from 00a98f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::UpdateWorldData` (Xbox PDB; the update data):
/// `NiAVObject::UpdateWorldData`, the model data's `00a98c90`, then the world transform
/// copied into the local transform of the object at model data +0x70 (rotation to +0x34, translation to +0x58, the scale's magnitude
/// through a float to +0x64) and that object's slot +0xa4 with (data, 0); when the
/// model data's byte +0x74 is set, `NiAVObject::UpdateProperties` and `00a29680` run and
/// the byte is cleared.
pub fn ni_particle_meshes_update_world_data(e: &mut Engine, this: Ptr, data: Ptr) {
    let meshes = this.addr();
    e.call(AV_UPDATE_WORLD_DATA, &args![this, data]);
    let model = e.mem.u32(meshes + MODEL_DATA);
    let target = e.mem.u32(model + MESH_DATA);
    e.call(MODEL_DATA_00A98C90, &args![model]);
    for word in 0..9 {
        let value = e.mem.u32(meshes + 0x68 + 4 * word);
        e.mem.set_u32(target + 0x34 + 4 * word, value);
    }
    for (from, to) in [(0x8cu32, 0x58u32), (0x90, 0x5c), (0x94, 0x60)] {
        let value = e.mem.u32(meshes + from);
        e.mem.set_u32(target + to, value);
    }
    let scale = x87_load32(e.mem.u32(meshes + 0x98)).abs();
    let scale = x87_store32(x87_load32(x87_store32(scale)));
    e.mem.set_u32(target + 0x64, scale);
    e.vcall(target, SLOT_UPDATE_WORLD_DATA, &args![data, 0u32]);
    if e.mem.u8(model + 0x74) != 0 {
        e.call(AV_UPDATE_PROPERTIES, &args![this]);
        e.call(FN_00A29680, &args![this]);
        e.mem.set_u8(model + 0x74, 0);
    }
}

// Translated from 00a99000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::SetSelectiveUpdateFlags` (Xbox PDB; (selective update, a second word
/// it never reads, the rigid flag's address)): `NiGeometry::SetSelectiveUpdateFlags` and the
/// slot +0xa0 of the object at model data +0x70, both with (the first word, 1, the address); then the flag byte
/// is cleared and bit 4 of that object's flags (+0x30) too.
pub fn ni_particle_meshes_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr,
    selective: u32,
    _unused_2: u32,
    rigid: Ptr,
) {
    e.call(
        GEOMETRY_SET_SELECTIVE_UPDATE_FLAGS,
        &args![this, selective, 1u32, rigid],
    );
    let target = mesh_data(e, this);
    e.vcall(
        target,
        SLOT_SELECTIVE_UPDATE_FLAGS,
        &args![selective, 1u32, rigid],
    );
    e.mem.set_u8(rigid.addr(), 0);
    let flags = e.mem.u32(target + 0x30);
    e.mem.set_u32(target + 0x30, flags & 0xffff_ffef);
}

// Translated from 00a99040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::UpdatePropertiesDownward` (Xbox PDB; the parent's property state):
/// `NiGeometry::UpdatePropertiesDownward`, then slot +0xb0 of the object at model data +0x70, with this
/// object's property state (+0x9c).
pub fn ni_particle_meshes_update_properties_downward(e: &mut Engine, this: Ptr, state: Ptr) {
    e.call(GEOMETRY_UPDATE_PROPERTIES_DOWNWARD, &args![this, state]);
    let target = mesh_data(e, this);
    e.vcall(
        target,
        SLOT_UPDATE_PROPERTIES_DOWNWARD,
        &args![this.addr() + 0x9c],
    );
}

// Translated from 00a99070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::OnVisible` (Xbox PDB; the culling process): a jump to the mesh
/// data's slot +0xd4 with the same argument.
pub fn ni_particle_meshes_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let target = mesh_data(e, this);
    e.vcall(target, SLOT_ON_VISIBLE, &args![culler]);
}

// Translated from 00a99090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::PurgeRendererData` (Xbox PDB; the renderer):
/// `NiGeometry::PurgeRendererData`, then, when there is a model data with an object at +0x70, that
/// object's slot +0xd8 with the renderer.
pub fn ni_particle_meshes_purge_renderer_data(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(GEOMETRY_PURGE_RENDERER_DATA, &args![this, renderer]);
    let model = e.mem.u32(this.addr() + MODEL_DATA);
    if model == 0 {
        return;
    }
    let target = e.mem.u32(model + MESH_DATA);
    if target != 0 {
        e.vcall(target, SLOT_PURGE_RENDERER_DATA, &args![renderer]);
    }
}

// Translated from 00a990c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::GetViewerStrings` (Xbox PDB): after `NiParticles::GetViewerStrings`,
/// adds the class name of the RTTI record at `011f5f64` (grown first when full; `SetAt`
/// at the old size).
pub fn ni_particle_meshes_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PARTICLES_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(VIEWER_NAME_RTTI);
    let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
    let list = strings.addr();
    let index = u32::from(e.mem.u16(list + 0xa));
    if index >= u32::from(e.mem.u16(list + 8)) {
        let grow = u32::from(e.mem.u16(list + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_8e30,
            ni_particle_meshes_update_downward_pass(Ptr, Ptr, u32)
        ),
        entry!(0x00a9_8e50, fn_00a98e50(Ptr, Ptr, u32)),
        entry!(0x00a9_8e70, fn_00a98e70() -> Ptr),
        entry!(
            0x00a9_8ed0,
            ni_particle_meshes_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_8f00, ni_particle_meshes_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_8f60, ni_particle_meshes_update_world_data(Ptr, Ptr)),
        entry!(
            0x00a9_9000,
            ni_particle_meshes_set_selective_update_flags(Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00a9_9040,
            ni_particle_meshes_update_properties_downward(Ptr, Ptr)
        ),
        entry!(0x00a9_9070, ni_particle_meshes_on_visible(Ptr, Ptr)),
        entry!(
            0x00a9_9090,
            ni_particle_meshes_purge_renderer_data(Ptr, Ptr)
        ),
        entry!(0x00a9_90c0, ni_particle_meshes_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a98ec0, ni_particle_meshes_get_rtti() -> Ptr),
        entry!(0x00a7_1d10, ni_particle_meshes_save_binary(Ptr, Ptr)),
        entry!(0x00a7_4500, ni_particle_meshes_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_4510, ni_particle_meshes_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_8ea0, ni_particle_meshes_load_binary(Ptr, Ptr)),
        entry!(0x00a9_8eb0, ni_particle_meshes_link_object(Ptr, Ptr)),
        entry!(0x00a9_b140, ni_particle_meshes_copy_members(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_meshes_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a98ec0, &args![]).u32(), NI_PARTICLE_MESHES_RTTI);
    }

    #[test]
    fn thunk_00a71d10_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_1d10;
        const TARGET: u32 = 0x00a9_b1a0;
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
    fn thunk_00a74500_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_4500;
        const TARGET: u32 = 0x00a9_b190;
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
    fn thunk_00a74510_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_4510;
        const TARGET: u32 = 0x00a8_9770;
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
    fn thunk_00a98ea0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_8ea0;
        const TARGET: u32 = 0x00a9_b180;
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
    fn thunk_00a98eb0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_8eb0;
        const TARGET: u32 = 0x00a7_4780;
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
    fn thunk_00a9b140_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b140;
        const TARGET: u32 = 0x00a8_0400;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    const TARGETS: u32 = 0x0330_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Meshes whose model data holds an object at +0x70 with the test vtable.
    fn engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x100);
        for slot in 0..0x40 {
            e.mem.set_u32(vtable + 4 * slot, TARGETS + 4 * slot);
        }
        let meshes = e.mem.alloc(0xcc);
        let model = e.mem.alloc(0x80);
        let target = e.mem.alloc(0x70);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(model + MESH_DATA, target);
        e.mem.set_u32(meshes + MODEL_DATA, model);
        e.call_log = Some(vec![]);
        (e, meshes, model, target)
    }

    #[test]
    fn update_downward_pass_copies_time_and_flag_then_jumps() {
        let (mut e, meshes, _, _) = engine();
        quiet(&mut e, &[AV_UPDATE_DOWNWARD_PASS]);
        let data = e.mem.alloc(0xc);
        e.mem.set_u32(data, 0x7f80_0001);
        e.mem.set_u8(data + 4, 1);
        e.call(0x00a9_8e30, &args![meshes, data, 7u32]);
        assert_eq!(e.mem.u32(meshes + UPDATE_TIME), 0x7fc0_0001);
        assert_eq!(e.mem.u8(meshes + UPDATE_FLAG), 1);
        assert_eq!(
            calls(&e, AV_UPDATE_DOWNWARD_PASS),
            vec![vec![meshes, data, 7]]
        );
    }

    #[test]
    fn selected_downward_pass_copies_the_time_then_jumps() {
        let (mut e, meshes, _, _) = engine();
        quiet(&mut e, &[AV_00A59FD0]);
        let data = e.mem.alloc(0xc);
        e.mem.set_f32(data, 2.5);
        e.mem.set_u8(data + 4, 1);
        e.call(0x00a9_8e50, &args![meshes, data, 7u32]);
        assert_eq!(e.mem.f32(meshes + UPDATE_TIME), 2.5);
        assert_eq!(e.mem.u8(meshes + UPDATE_FLAG), 0);
        assert_eq!(calls(&e, AV_00A59FD0), vec![vec![meshes, data, 7]]);
    }

    #[test]
    fn create_constructs_or_answers_null() {
        let (mut e, _, _, _) = engine();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let p = e.call(0x00a9_8e70, &args![]).u32();
        assert_eq!(e.mem.u32(p), NI_PARTICLE_MESHES_VTABLE);
        assert_eq!(e.mem.u8(p + UPDATE_FLAG), 1);
        assert_eq!(calls(&e, BASE_CONSTRUCT), vec![vec![p]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_8e70, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let (mut e, meshes, _, _) = engine();
        quiet(&mut e, &[GEOMETRY_DESTRUCT, SIZED_DELETE]);
        assert_eq!(e.call(0x00a9_8ed0, &args![meshes, 0u32]).u32(), meshes);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_8ed0, &args![meshes, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![meshes, 0xcc]]);
        assert_eq!(calls(&e, GEOMETRY_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_copies_members_even_without_a_clone() {
        let (mut e, meshes, _, _) = engine();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[BASE_CONSTRUCT, JUMP_TARGET_00A9B140]);
        let clone = e.call(0x00a9_8f00, &args![meshes, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_PARTICLE_MESHES_VTABLE);
        assert_eq!(e.mem.u8(clone + UPDATE_FLAG), 1);
        assert_eq!(
            calls(&e, JUMP_TARGET_00A9B140),
            vec![vec![meshes, clone, 0x55]]
        );
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_8f00, &args![meshes, 0x55u32]).u32(), 0);
        assert_eq!(calls(&e, JUMP_TARGET_00A9B140)[1], vec![meshes, 0, 0x55]);
    }

    #[test]
    fn update_world_data_copies_the_transform_and_updates_properties_once() {
        let (mut e, meshes, model, target) = engine();
        quiet(
            &mut e,
            &[
                AV_UPDATE_WORLD_DATA,
                MODEL_DATA_00A98C90,
                AV_UPDATE_PROPERTIES,
                FN_00A29680,
                TARGETS + SLOT_UPDATE_WORLD_DATA,
            ],
        );
        for word in 0..9 {
            e.mem.set_u32(meshes + 0x68 + 4 * word, 0x100 + word);
        }
        for (i, offset) in [0x8cu32, 0x90, 0x94].iter().enumerate() {
            e.mem.set_u32(meshes + offset, 0x200 + i as u32);
        }
        e.mem.set_f32(meshes + 0x98, -2.0);
        e.mem.set_u8(model + 0x74, 1);
        e.call(0x00a9_8f60, &args![meshes, 0x77u32]);
        let rotation: Vec<u32> = (0..9).map(|w| e.mem.u32(target + 0x34 + 4 * w)).collect();
        assert_eq!(rotation, (0x100..0x109).collect::<Vec<u32>>());
        assert_eq!(
            (
                e.mem.u32(target + 0x58),
                e.mem.u32(target + 0x5c),
                e.mem.u32(target + 0x60)
            ),
            (0x200, 0x201, 0x202)
        );
        assert_eq!(e.mem.f32(target + 0x64), 2.0);
        assert_eq!(calls(&e, MODEL_DATA_00A98C90), vec![vec![model]]);
        assert_eq!(
            calls(&e, TARGETS + SLOT_UPDATE_WORLD_DATA),
            vec![vec![target, 0x77, 0]]
        );
        assert_eq!(calls(&e, AV_UPDATE_PROPERTIES), vec![vec![meshes]]);
        assert_eq!(calls(&e, FN_00A29680), vec![vec![meshes]]);
        assert_eq!(e.mem.u8(model + 0x74), 0);
        // The byte is clear now: no property update the second time.
        e.call(0x00a9_8f60, &args![meshes, 0x77u32]);
        assert_eq!(calls(&e, AV_UPDATE_PROPERTIES).len(), 1);
    }

    #[test]
    fn selective_update_flags_pass_1_and_clear_the_flags() {
        let (mut e, meshes, _, target) = engine();
        quiet(
            &mut e,
            &[
                GEOMETRY_SET_SELECTIVE_UPDATE_FLAGS,
                TARGETS + SLOT_SELECTIVE_UPDATE_FLAGS,
            ],
        );
        let rigid = e.mem.alloc(4);
        e.mem.set_u32(rigid, 0x0101_0101);
        e.mem.set_u32(target + 0x30, 0xff);
        e.call(0x00a9_9000, &args![meshes, 1u32, 0u32, rigid]);
        assert_eq!(
            calls(&e, GEOMETRY_SET_SELECTIVE_UPDATE_FLAGS),
            vec![vec![meshes, 1, 1, rigid]]
        );
        assert_eq!(
            calls(&e, TARGETS + SLOT_SELECTIVE_UPDATE_FLAGS),
            vec![vec![target, 1, 1, rigid]]
        );
        assert_eq!(e.mem.u32(rigid), 0x0101_0100);
        assert_eq!(e.mem.u32(target + 0x30), 0xef);
    }

    #[test]
    fn properties_downward_pass_the_own_state() {
        let (mut e, meshes, _, target) = engine();
        quiet(
            &mut e,
            &[
                GEOMETRY_UPDATE_PROPERTIES_DOWNWARD,
                TARGETS + SLOT_UPDATE_PROPERTIES_DOWNWARD,
            ],
        );
        e.call(0x00a9_9040, &args![meshes, 0x44u32]);
        assert_eq!(
            calls(&e, GEOMETRY_UPDATE_PROPERTIES_DOWNWARD),
            vec![vec![meshes, 0x44]]
        );
        assert_eq!(
            calls(&e, TARGETS + SLOT_UPDATE_PROPERTIES_DOWNWARD),
            vec![vec![target, meshes + 0x9c]]
        );
    }

    #[test]
    fn on_visible_forwards_the_culler() {
        let (mut e, meshes, _, target) = engine();
        quiet(&mut e, &[TARGETS + SLOT_ON_VISIBLE]);
        e.call(0x00a9_9070, &args![meshes, 0x33u32]);
        assert_eq!(
            calls(&e, TARGETS + SLOT_ON_VISIBLE),
            vec![vec![target, 0x33]]
        );
    }

    #[test]
    fn purge_renderer_data_forwards_when_there_is_an_object() {
        let (mut e, meshes, model, target) = engine();
        quiet(
            &mut e,
            &[
                GEOMETRY_PURGE_RENDERER_DATA,
                TARGETS + SLOT_PURGE_RENDERER_DATA,
            ],
        );
        e.call(0x00a9_9090, &args![meshes, 0x22u32]);
        assert_eq!(
            calls(&e, TARGETS + SLOT_PURGE_RENDERER_DATA),
            vec![vec![target, 0x22]]
        );
        e.mem.set_u32(model + MESH_DATA, 0);
        e.call(0x00a9_9090, &args![meshes, 0x22u32]);
        e.mem.set_u32(meshes + MODEL_DATA, 0);
        e.call(0x00a9_9090, &args![meshes, 0x22u32]);
        assert_eq!(calls(&e, TARGETS + SLOT_PURGE_RENDERER_DATA).len(), 1);
        assert_eq!(calls(&e, GEOMETRY_PURGE_RENDERER_DATA).len(), 3);
    }

    #[test]
    fn viewer_strings_add_the_particles_class_name() {
        let (mut e, meshes, _, _) = engine();
        e.map(VIEWER_NAME_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(VIEWER_NAME_RTTI, 0x0abc_0000);
        quiet(
            &mut e,
            &[PARTICLES_GET_VIEWER_STRINGS, STRING_ARRAY_SET_SIZE],
        );
        e.register(VIEWER_STRING_NAME, |_, _| 0x61u32.into_ret());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x61);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a9_90c0, &args![meshes, strings]);
        assert_eq!(
            calls(&e, PARTICLES_GET_VIEWER_STRINGS),
            vec![vec![meshes, strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0abc_0000]]);
        assert_eq!(calls(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 5]]);
        assert_eq!(calls(&e, STRING_ARRAY_SET_AT)[0][..2], [strings, 3]);
    }
}
