//! `NiParticle/nimeshparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1ad80`: the jump target of `00c21900`.
const JUMP_TARGET_00C21900: u32 = 0x00c1_ad80;

// Translated from 00c21900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::SetModelData` (Xbox PDB): a 5-byte jump to `00c1ad80` (engine map: `NiParticleSystem::SetModelData`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetModelData@NiMeshParticleSystem@@UAAXPAVNiGeometryData@@@Z`.
pub fn ni_mesh_particle_system_set_model_data(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C21900, &args![this, arg]);
}

// Translated from 00c21e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2538.
pub fn ni_mesh_particle_system_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2538
}

layout! {
    /// `NiMeshParticleSystem` (Xbox PDB, 0x160 bytes; 0x118 on PC, whose `NiParticleSystem`
    /// base is 0x110 bytes): the fields its own functions use.
    pub struct NiMeshParticleSystem: 0x118 {
        /// `m_fTime` (Xbox PDB).
        0x110 m_fTime: u32,
        /// `m_bUpdateControllers` (Xbox PDB).
        0x114 m_bUpdateControllers: u8,
    }
}

/// `NiGeometry::m_spModelData` (Xbox PDB `+0xdc`, `+0xb8` on PC) and the particle data's
/// object at `+0x7c` (the mesh node the mesh system's calls go to).
const MODEL_DATA: u32 = 0xb8;
const DATA_MESHES: u32 = 0x7c;
/// `NiParticleSystem::m_kModifierList`'s head (`+0xc8` on PC).
const MODIFIER_HEAD: u32 = 0xc8;
/// `NiMeshParticleSystem`'s vtable (RTTI `.?AVNiMeshParticleSystem@@`) and size.
const NI_MESH_PARTICLE_SYSTEM_VTABLE: u32 = 0x010b_dd84;
const NI_MESH_PARTICLE_SYSTEM_SIZE: u32 = 0x118;
/// `NiParticleSystem`'s functions it builds on (in `niparticlesystem.cpp`): `LoadBinary`
/// (`00c1b3a0`), `UpdateWorldData` (`00c1add0`), `UpdateWorldBound` (`00c1af60`),
/// `SetSelectiveUpdateFlags` (`00c1b020`), the constructor (`00c1b5c0`), `CopyMembers`
/// (`00c1b6f0`), `GetViewerStrings` (`00c1b850`) and the modifier list's `RemoveAll`
/// (`00e74d40`).
const SYSTEM_LOAD_BINARY_00C1B3A0: u32 = 0x00c1_b3a0;
const SYSTEM_UPDATE_WORLD_DATA_00C1ADD0: u32 = 0x00c1_add0;
const SYSTEM_UPDATE_WORLD_BOUND_00C1AF60: u32 = 0x00c1_af60;
const SYSTEM_SET_SELECTIVE_UPDATE_FLAGS_00C1B020: u32 = 0x00c1_b020;
const SYSTEM_CONSTRUCT_00C1B5C0: u32 = 0x00c1_b5c0;
const SYSTEM_COPY_MEMBERS_00C1B6F0: u32 = 0x00c1_b6f0;
const SYSTEM_GET_VIEWER_STRINGS_00C1B850: u32 = 0x00c1_b850;
const LIST_REMOVE_ALL_00E74D40: u32 = 0x00e7_4d40;
/// `NiGeometry`'s functions (`thiscall`): `UpdatePropertiesDownward` (`00a80820`, one word),
/// `PurgeRendererData` (`00a809b0`, one word) and the destructor (`00a80580`).
const GEOMETRY_UPDATE_PROPERTIES_DOWNWARD_00A80820: u32 = 0x00a8_0820;
const GEOMETRY_PURGE_RENDERER_DATA_00A809B0: u32 = 0x00a8_09b0;
const GEOMETRY_DESTRUCT_00A80580: u32 = 0x00a8_0580;
/// The Ni allocation (`00aa13e0`, `cdecl`: size) and free (`00aa1460`, `cdecl`: block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE: u32 = 0x00aa_1460;
/// The RTTI records the mesh system's passes look for: the controller kept for last in the
/// downward pass (`01202690`) and the two modifier kinds whose objects are purged
/// (`01202500`: an array at `+0x58`, its 16-bit size at `+0x60`; `01202530`: an array at
/// `+0x1c`, its size at `+0x22`).
const LAST_CONTROLLER_RTTI_01202690: u32 = 0x0120_2690;
const PURGE_MODIFIER_RTTI_01202500: u32 = 0x0120_2500;
const PURGE_MODIFIER_RTTI_01202530: u32 = 0x0120_2530;
/// The bound used when the mesh node has none (`011f4288`).
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
/// `NiMeshParticleSystem`'s RTTI record (its first word the class name).
const NI_MESH_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_2538;
/// `NiGetViewerString` (`00a7d330`, `cdecl`) and the string array's grow (`0096ad30`) and
/// set-at (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The load warning: `00aa1360` (`cdecl`: the stream's file name at `+0xe0` and a model
/// path; 0 skips the warning), the two rain models it is skipped for, the stream's name
/// (`+8`) or `Please` when that is empty, the format `File %s contains a mesh particle
/// system!` (`sprintf`, `00ec623a`) and the warning callback (`011ac2ac`, `cdecl`: the text,
/// `Unsupported Object`, 0).
const PATH_COMPARE_00AA1360: u32 = 0x00aa_1360;
const RAIN_HEAVY_MODEL: u32 = 0x010b_dd60;
const RAIN_LIGHT_MODEL: u32 = 0x010b_dd44;
const NO_NAME_TEXT: u32 = 0x010b_dd3c;
const MESH_PARTICLE_WARNING_FORMAT: u32 = 0x010b_dd04;
const SPRINTF: u32 = 0x00ec_623a;
const WARNING_CALLBACK_011AC2AC: u32 = 0x011a_c2ac;
const UNSUPPORTED_OBJECT_TEXT: u32 = 0x010b_dcf0;

/// The particle data's mesh object (`[m_spModelData + 0x7c]`).
fn data_meshes(e: &Engine, this: u32) -> u32 {
    let data = e.mem.u32(this.wrapping_add(MODEL_DATA));
    e.mem.u32(data.wrapping_add(DATA_MESHES))
}

/// Whether `object`'s RTTI chain (virtual slot +8, the base at +4) holds `rtti`.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record + 4);
    }
    false
}

/// Calls virtual slot +0xd8 with `renderer` on each set entry of the array at `array`
/// (`count` the 16-bit size word, re-read for each entry against the size read first).
fn purge_entries(e: &mut Engine, array: u32, count: u32, renderer: u32) {
    let size = u32::from(e.mem.u16(count));
    let mut index = 0u32;
    while index < size {
        if index < u32::from(e.mem.u16(count)) {
            let entries = e.mem.u32(array);
            let object = e.mem.u32(entries.wrapping_add(index.wrapping_mul(4)));
            if object != 0 {
                e.vcall(object, 0xd8, &args![renderer]);
            }
        }
        index += 1;
    }
}

// Translated from 00c21960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::LoadBinary` (Xbox PDB, `thiscall`, one word: the stream): unless the
/// stream's file name (`+0xe0`) is one of the two rain models (`00aa1360` answering 0), it
/// formats the warning `File %s contains a mesh particle system!` with the file name (and the
/// stream's name at `+8`, or `Please`, which the format does not use) into a 0x100-byte
/// stack buffer and passes it to the warning callback (`011ac2ac`, when set) with
/// `Unsupported Object` and 0. Then `NiParticleSystem::LoadBinary`.
// Translation incomplete: 00c21960 - the /GS stack cookie check (00ec408c) is not translated
pub fn ni_mesh_particle_system_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let path = stream.addr().wrapping_add(0xe0);
    let mut warn = true;
    if path != 0 {
        let heavy = e
            .call(PATH_COMPARE_00AA1360, &args![path, RAIN_HEAVY_MODEL])
            .u32();
        warn = heavy != 0
            && e.call(PATH_COMPARE_00AA1360, &args![path, RAIN_LIGHT_MODEL])
                .u32()
                != 0;
    }
    if warn {
        e.with_stack(0x108, |e, buffer| {
            let name_at = stream.addr().wrapping_add(8);
            let name = if e.mem.u32(name_at) != 0 {
                name_at
            } else {
                NO_NAME_TEXT
            };
            e.call(
                SPRINTF,
                &args![buffer, MESH_PARTICLE_WARNING_FORMAT, path, name],
            );
            let callback = e.global::<u32>(WARNING_CALLBACK_011AC2AC);
            if callback != 0 {
                e.call(callback, &args![buffer, UNSUPPORTED_OBJECT_TEXT, 0u32]);
            }
        });
    }
    e.call(SYSTEM_LOAD_BINARY_00C1B3A0, &args![this, stream]);
}

// Translated from 00c21a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::PurgeRendererDataForPools` (engine map, `thiscall`, one word: the
/// renderer): for each of the data's pools (the array at `+0x8c`, its 16-bit count at
/// `+0x92`) that is set, virtual slot +0xd8 with the renderer on each set object of the pool
/// (the array at `+4`, its 16-bit count at `+0xa`, read once per pool).
pub fn ni_mesh_p_sys_data_purge_renderer_data_for_pools(e: &mut Engine, this: Ptr, renderer: u32) {
    let pools = u32::from(e.mem.u16(this.addr() + 0x92));
    let mut pool_index = 0u32;
    while pool_index < pools {
        let table = e.mem.u32(this.addr() + 0x8c);
        let pool = e.mem.u32(table.wrapping_add(pool_index.wrapping_mul(4)));
        if pool != 0 {
            let count = u32::from(e.mem.u16(pool + 0xa));
            let mut index = 0u32;
            while index < count {
                let objects = e.mem.u32(pool + 4);
                let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
                if object != 0 {
                    e.vcall(object, 0xd8, &args![renderer]);
                }
                index += 1;
            }
        }
        pool_index += 1;
    }
}

// Translated from 00c21a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::UpdateDownwardPass` (Xbox PDB, `thiscall`, two words: the update
/// data and one not read). With the data's byte +5 set it only sets its byte +6. Otherwise,
/// when the data's byte +4 is set, each property (`m_kPropertyList`, `+0x24`) with a word +0xc
/// gets its virtual slot +0x90 and each controller (`m_spControllers`, next at `+0x30`) its
/// slot +0x94 with the data, except the last one of the kind `01202690`, which runs after the
/// system's slot +0xb8; without byte +4 only slot +0xb8 runs. Then slot +0xbc, `m_fTime` from
/// the data's float +0 and `m_bUpdateControllers` from its byte +4.
pub fn ni_mesh_particle_system_update_downward_pass(
    e: &mut Engine,
    this: Ptr<NiMeshParticleSystem>,
    data: Ptr,
    _unused_2: u32,
) {
    let (base, data) = (this.addr(), data.addr());
    if e.mem.u8(data + 5) != 0 {
        e.mem.set_u8(data + 6, 1);
        return;
    }
    let mut last = 0u32;
    if e.mem.u8(data + 4) != 0 {
        let mut item = e.mem.u32(base + 0x24);
        while item != 0 {
            let property = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if property != 0 && e.mem.u32(property + 0xc) != 0 {
                e.vcall(property, 0x90, &args![data]);
            }
        }
        let mut controller = e.mem.u32(base + 0xc);
        while controller != 0 {
            if is_kind_of(e, controller, LAST_CONTROLLER_RTTI_01202690) {
                last = controller;
            } else {
                e.vcall(controller, 0x94, &args![data]);
            }
            controller = e.mem.u32(controller + 0x30);
        }
        e.vcall(base, 0xb8, &args![data]);
        if last != 0 {
            e.vcall(last, 0x94, &args![data]);
        }
    } else {
        e.vcall(base, 0xb8, &args![data]);
    }
    e.vcall(base, 0xbc, &args![]);
    let time = x87_quiet_bits(e.mem.u32(data));
    e.set(this, NiMeshParticleSystem::m_fTime, time);
    let update = e.mem.u8(data + 4);
    e.set(this, NiMeshParticleSystem::m_bUpdateControllers, update);
}

// Translated from 00c21b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::UpdatePropertiesDownward` (Xbox PDB, `thiscall`, one word: the
/// property state): `NiGeometry::UpdatePropertiesDownward`, then the mesh object's virtual slot
/// +0xb0 with the system's property state (`+0x9c`).
pub fn ni_mesh_particle_system_update_properties_downward(e: &mut Engine, this: Ptr, state: u32) {
    e.call(
        GEOMETRY_UPDATE_PROPERTIES_DOWNWARD_00A80820,
        &args![this, state],
    );
    let meshes = data_meshes(e, this.addr());
    e.vcall(meshes, 0xb0, &args![this.addr() + 0x9c]);
}

// Translated from 00c21b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::OnVisible` (Xbox PDB, `thiscall`, one word: the culling process): a
/// jump to the mesh object's virtual slot +0xd4 with the same word.
pub fn ni_mesh_particle_system_on_visible(e: &mut Engine, this: Ptr, culler: u32) {
    let meshes = data_meshes(e, this.addr());
    e.vcall(meshes, 0xd4, &args![culler]);
}

// Translated from 00c21bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::SetSelectiveUpdateFlags` (Xbox PDB, `thiscall`, three words: the
/// selective-update flag, the transforms flag (not read; 1 is passed on) and the address of
/// the rigid flag): `NiParticleSystem::SetSelectiveUpdateFlags` and the mesh object's virtual
/// slot +0xa0 with the first flag, 1 and the address; then the rigid flag is cleared and bit
/// 4 of the mesh object's flags (`+0x30`) cleared.
pub fn ni_mesh_particle_system_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr,
    selective: u32,
    _unused_2: u32,
    rigid: u32,
) {
    e.call(
        SYSTEM_SET_SELECTIVE_UPDATE_FLAGS_00C1B020,
        &args![this, selective, 1u32, rigid],
    );
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    let meshes = e.mem.u32(data + DATA_MESHES);
    e.vcall(meshes, 0xa0, &args![selective, 1u32, rigid]);
    e.mem.set_u8(rigid, 0);
    let meshes = e.mem.u32(data + DATA_MESHES);
    let flags = e.mem.u32(meshes + 0x30) & 0xffff_ffef;
    e.mem.set_u32(meshes + 0x30, flags);
}

// Translated from 00c21bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::UpdateWorldData` (Xbox PDB, `thiscall`, one word: the update data):
/// `NiParticleSystem::UpdateWorldData`, then the mesh object's local transform takes the
/// system's world rotation (nine words from `+0x68` to its `+0x34`), translation (`+0x8c` to
/// its `+0x58`) and the magnitude of the world scale (`FABS`, to its `+0x64`), and its virtual
/// slot +0xb8 runs with the data.
pub fn ni_mesh_particle_system_update_world_data(e: &mut Engine, this: Ptr, update: u32) {
    let base = this.addr();
    e.call(SYSTEM_UPDATE_WORLD_DATA_00C1ADD0, &args![this, update]);
    let data = e.mem.u32(base + MODEL_DATA);
    let meshes = e.mem.u32(data + DATA_MESHES);
    for word in 0..9u32 {
        let value = e.mem.u32(base + 0x68 + 4 * word);
        e.mem.set_u32(meshes.wrapping_add(0x34 + 4 * word), value);
    }
    let meshes = e.mem.u32(data + DATA_MESHES);
    for word in 0..3u32 {
        let value = e.mem.u32(base + 0x8c + 4 * word);
        e.mem.set_u32(meshes.wrapping_add(0x58 + 4 * word), value);
    }
    let meshes = e.mem.u32(data + DATA_MESHES);
    let scale = x87_store32(x87_load32(e.mem.u32(base + 0x98)).abs());
    e.mem
        .set_u32(meshes.wrapping_add(0x64), x87_quiet_bits(scale));
    e.vcall(meshes, 0xb8, &args![update]);
}

// Translated from 00c21c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::UpdateWorldBound` (Xbox PDB, `thiscall`):
/// `NiParticleSystem::UpdateWorldBound`, then the system's world bound (`m_kWorldBound`,
/// `+0x20`) takes the mesh object's (its `+0x20`, or the default at `011f4288` when null):
/// the centre's three words and the radius (`FLD`/`FSTP` twice).
pub fn ni_mesh_particle_system_update_world_bound(e: &mut Engine, this: Ptr) {
    e.call(SYSTEM_UPDATE_WORLD_BOUND_00C1AF60, &args![this]);
    let meshes = data_meshes(e, this.addr());
    let mut source = e.mem.u32(meshes + 0x20);
    if source == 0 {
        source = DEFAULT_BOUND_011F4288;
    }
    let first = e.mem.u32(source);
    let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
    let bound = e.mem.u32(this.addr() + 0x20);
    e.mem.set_u32(bound, first);
    let second = e.mem.u32(source + 4);
    e.mem.set_u32(bound + 4, second);
    let third = e.mem.u32(source + 8);
    e.mem.set_u32(bound + 0xc, radius);
    e.mem.set_u32(bound + 8, third);
}

// Translated from 00c21cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::PurgeRendererData` (Xbox PDB, `thiscall`, one word: the renderer):
/// `NiGeometry::PurgeRendererData`; with particle data, its mesh object's virtual slot +0xd8
/// (when set) and [`ni_mesh_p_sys_data_purge_renderer_data_for_pools`]; then each modifier of
/// the kind `01202500` purges the objects of its array at `+0x58` (size `+0x60`), and each
/// of the kind `01202530` those of its array at `+0x1c` (size `+0x22`), with slot +0xd8.
pub fn ni_mesh_particle_system_purge_renderer_data(e: &mut Engine, this: Ptr, renderer: u32) {
    e.call(
        GEOMETRY_PURGE_RENDERER_DATA_00A809B0,
        &args![this, renderer],
    );
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    if data != 0 {
        let meshes = e.mem.u32(data + DATA_MESHES);
        if meshes != 0 {
            e.vcall(meshes, 0xd8, &args![renderer]);
        }
        ni_mesh_p_sys_data_purge_renderer_data_for_pools(e, Ptr::new(data), renderer);
    }
    let mut item = e.mem.u32(this.addr() + MODIFIER_HEAD);
    while item != 0 {
        let modifier = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if modifier == 0 {
            continue;
        }
        if is_kind_of(e, modifier, PURGE_MODIFIER_RTTI_01202500) {
            purge_entries(e, modifier + 0x58, modifier + 0x60, renderer);
        } else if is_kind_of(e, modifier, PURGE_MODIFIER_RTTI_01202530) {
            purge_entries(e, modifier + 0x1c, modifier + 0x22, renderer);
        }
    }
}

// Translated from 00c21dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::GetViewerStrings` (Xbox PDB, `thiscall`, one word: the string
/// array): `NiParticleSystem::GetViewerStrings`, then the class name (the first word of the
/// RTTI record `01202538`) added to the array (grown to growth + size when full; the string
/// kept in the argument's stack word).
pub fn ni_mesh_particle_system_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(SYSTEM_GET_VIEWER_STRINGS_00C1B850, &args![this, strings]);
    let strings = strings.addr();
    let name = e.global::<u32>(NI_MESH_PARTICLE_SYSTEM_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// A new `NiMeshParticleSystem` (0x118 bytes from `00aa13e0`; 0 when that fails): the
/// `NiParticleSystem` constructor, `m_fTime` 0.0, the vtable, `m_bUpdateControllers` set.
fn new_mesh_particle_system(e: &mut Engine) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_MESH_PARTICLE_SYSTEM_SIZE])
        .u32();
    if object == 0 {
        return 0;
    }
    e.call(SYSTEM_CONSTRUCT_00C1B5C0, &args![object]);
    e.mem.set_u32(object + 0x110, 0);
    e.mem.set_u32(object, NI_MESH_PARTICLE_SYSTEM_VTABLE);
    e.mem.set_u8(object + 0x114, 1);
    object
}

// Translated from 00c21e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::CreateClone` (Xbox PDB, `thiscall`, one word: the cloning
/// process): a new system (see [`fn_00c21e90`]'s steps), `NiParticleSystem::CopyMembers`
/// into it, then `m_fTime` (`FLD`/`FSTP`) and `m_bUpdateControllers` copied. Answers the
/// clone (a failed allocation goes on with 0, as the exe does).
pub fn ni_mesh_particle_system_create_clone(
    e: &mut Engine,
    this: Ptr<NiMeshParticleSystem>,
    cloning: u32,
) -> u32 {
    let clone = new_mesh_particle_system(e);
    e.call(SYSTEM_COPY_MEMBERS_00C1B6F0, &args![this, clone, cloning]);
    let time = x87_quiet_bits(e.get(this, NiMeshParticleSystem::m_fTime));
    e.mem.set_u32(clone.wrapping_add(0x110), time);
    let update = e.get(this, NiMeshParticleSystem::m_bUpdateControllers);
    e.mem.set_u8(clone.wrapping_add(0x114), update);
    clone
}

// Translated from 00c21e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The streaming factory of `NiMeshParticleSystem` (no function in the Ghidra project; read
/// with the byte disassembler; no arguments): 0x118 bytes from `00aa13e0`, the
/// `NiParticleSystem` constructor (`00c1b5c0`), `m_fTime` 0.0, the vtable `010bdd84`,
/// `m_bUpdateControllers` set. Answers the object (0 when the allocation fails).
pub fn fn_00c21e90(e: &mut Engine) -> u32 {
    new_mesh_particle_system(e)
}

// Translated from 00c21ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem`'s scalar deleting destructor (Xbox PDB `_scalar_deleting_destructor_`,
/// `thiscall`, one word: the flags): the modifier list emptied (`00e74d40`), the `NiGeometry`
/// destructor (`00a80580`), and with bit 0 of the flags the block freed (`00aa1460`, 0x118
/// bytes). Answers `this`.
pub fn ni_mesh_particle_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(
        LIST_REMOVE_ALL_00E74D40,
        &args![this.addr() + MODIFIER_HEAD],
    );
    e.call(GEOMETRY_DESTRUCT_00A80580, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, NI_MESH_PARTICLE_SYSTEM_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c21e10, ni_mesh_particle_system_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_1900,
            ni_mesh_particle_system_set_model_data(Ptr, Ptr)
        ),
        entry!(0x00c2_1960, ni_mesh_particle_system_load_binary(Ptr, Ptr)),
        entry!(
            0x00c2_1a10,
            ni_mesh_p_sys_data_purge_renderer_data_for_pools(Ptr, u32)
        ),
        entry!(
            0x00c2_1a80,
            ni_mesh_particle_system_update_downward_pass(Ptr<NiMeshParticleSystem>, Ptr, u32)
        ),
        entry!(
            0x00c2_1b60,
            ni_mesh_particle_system_update_properties_downward(Ptr, u32)
        ),
        entry!(0x00c2_1b90, ni_mesh_particle_system_on_visible(Ptr, u32)),
        entry!(
            0x00c2_1bb0,
            ni_mesh_particle_system_set_selective_update_flags(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00c2_1bf0,
            ni_mesh_particle_system_update_world_data(Ptr, u32)
        ),
        entry!(0x00c2_1c60, ni_mesh_particle_system_update_world_bound(Ptr)),
        entry!(
            0x00c2_1cb0,
            ni_mesh_particle_system_purge_renderer_data(Ptr, u32)
        ),
        entry!(
            0x00c2_1dc0,
            ni_mesh_particle_system_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c2_1e20,
            ni_mesh_particle_system_create_clone(Ptr<NiMeshParticleSystem>, u32) -> u32
        ),
        entry!(0x00c2_1e90, fn_00c21e90() -> u32),
        entry!(
            0x00c2_1ed0,
            ni_mesh_particle_system_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c21900_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_1900;
        const TARGET: u32 = 0x00c1_ad80;
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
    fn ni_mesh_particle_system_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_1e10, &args![]).u32(), 0x0120_2538);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    /// Slot byte offset `o` of the vtable made with base `b` calls `b + o`.
    const SYSTEM_SLOTS: u32 = 0x0e10_0000;
    const MESH_SLOTS: u32 = 0x0e20_0000;
    const OTHER_SLOTS: u32 = 0x0e30_0000;
    const SYSTEM_VTABLE: u32 = 0x0e50_0000;
    const MESH_VTABLE: u32 = 0x0e51_0000;
    const OTHER_VTABLE: u32 = 0x0e52_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for (at, slots) in [
            (SYSTEM_VTABLE, SYSTEM_SLOTS),
            (MESH_VTABLE, MESH_SLOTS),
            (OTHER_VTABLE, OTHER_SLOTS),
        ] {
            let targets: Vec<u32> = (0..0x40u32).map(|i| slots + 4 * i).collect();
            e.put_vtable(at, &targets);
        }
        e
    }

    fn object(e: &mut Engine, vtable: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A system with particle data whose mesh object (`+0x7c`) has the mesh vtable.
    fn system(e: &mut Engine) -> (u32, u32, u32) {
        let this = object(e, SYSTEM_VTABLE, 0x118);
        let data = object(e, OTHER_VTABLE, 0x98);
        let meshes = object(e, MESH_VTABLE, 0x70);
        e.mem.set_u32(this + MODEL_DATA, data);
        e.mem.set_u32(data + DATA_MESHES, meshes);
        (this, data, meshes)
    }

    #[test]
    fn load_binary_warns_except_for_the_rain_models() {
        let mut e = rig();
        e.map(0x011a_c000, 0x1000);
        e.set_global(WARNING_CALLBACK_011AC2AC, 0x0e70_0000u32);
        noop(&mut e, &[SPRINTF, 0x0e70_0000, SYSTEM_LOAD_BINARY_00C1B3A0]);
        e.register(PATH_COMPARE_00AA1360, |_, _| 1u32.into_ret());
        let stream = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1960, &args![0x5000u32, stream]);
        let printed = calls(&e, SPRINTF);
        assert_eq!(printed.len(), 1);
        assert_eq!(
            &printed[0][1..],
            &[MESH_PARTICLE_WARNING_FORMAT, stream + 0xe0, NO_NAME_TEXT]
        );
        assert_eq!(
            calls(&e, 0x0e70_0000),
            vec![vec![printed[0][0], UNSUPPORTED_OBJECT_TEXT, 0]]
        );
        assert_eq!(
            calls(&e, SYSTEM_LOAD_BINARY_00C1B3A0),
            vec![vec![0x5000, stream]]
        );
        // A named stream; the light rain model skips the warning.
        e.mem.set_u32(stream + 8, 0x41);
        e.register(PATH_COMPARE_00AA1360, |_, a| {
            u32::from(a[1] != RAIN_LIGHT_MODEL).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c2_1960, &args![0x5000u32, stream]);
        assert!(calls(&e, SPRINTF).is_empty());
        assert_eq!(calls(&e, PATH_COMPARE_00AA1360).len(), 2);
        assert_eq!(calls(&e, SYSTEM_LOAD_BINARY_00C1B3A0).len(), 1);
        e.register(PATH_COMPARE_00AA1360, |_, _| 1u32.into_ret());
        e.set_global(WARNING_CALLBACK_011AC2AC, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1960, &args![0x5000u32, stream]);
        assert_eq!(calls(&e, SPRINTF)[0][3], stream + 8);
    }

    #[test]
    fn purge_for_pools_visits_each_set_object() {
        let mut e = rig();
        noop(&mut e, &[MESH_SLOTS + 0xd8]);
        let data = e.mem.alloc(0x98);
        let pools = e.mem.alloc(8);
        let pool = e.mem.alloc(0x10);
        let objects = e.mem.alloc(8);
        let first = object(&mut e, MESH_VTABLE, 8);
        e.mem.set_u32(objects, first);
        e.mem.set_u32(pool + 4, objects);
        e.mem.set_u16(pool + 0xa, 2);
        e.mem.set_u32(pools + 4, pool);
        e.mem.set_u32(data + 0x8c, pools);
        e.mem.set_u16(data + 0x92, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1a10, &args![data, 0x77u32]);
        assert_eq!(calls(&e, MESH_SLOTS + 0xd8), vec![vec![first, 0x77]]);
    }

    #[test]
    fn update_downward_pass_runs_properties_controllers_then_the_kept_one() {
        let mut e = rig();
        let (this, _, _) = system(&mut e);
        let update = e.mem.alloc(8);
        e.mem.set_u32(update, 0x3f00_0000);
        // Skipped: only the skipped byte.
        e.mem.set_u8(update + 5, 1);
        e.call(0x00c2_1a80, &args![this, update, 0u32]);
        assert_eq!(e.mem.u8(update + 6), 1);
        e.mem.set_u8(update + 5, 0);
        e.mem.set_u8(update + 4, 1);
        // One property with a word +0xc, one without.
        let property = object(&mut e, OTHER_VTABLE, 0x10);
        e.mem.set_u32(property + 0xc, 1);
        let idle = object(&mut e, OTHER_VTABLE, 0x10);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 8, idle);
        e.mem.set_u32(items[1] + 8, property);
        e.mem.set_u32(this + 0x24, items[0]);
        // Two controllers: the second of the kept kind.
        let plain = object(&mut e, MESH_VTABLE, 0x40);
        let kept = object(&mut e, MESH_VTABLE, 0x40);
        e.mem.set_u32(this + 0xc, plain);
        e.mem.set_u32(plain + 0x30, kept);
        let record = e.mem.alloc(8);
        e.register_double(MESH_SLOTS + 8, move |_, a| {
            if a[0] == kept {
                LAST_CONTROLLER_RTTI_01202690.into_ret()
            } else {
                record.into_ret()
            }
        });
        e.map(0x0120_2000, 0x1000);
        noop(
            &mut e,
            &[
                OTHER_SLOTS + 0x90,
                MESH_SLOTS + 0x94,
                SYSTEM_SLOTS + 0xb8,
                SYSTEM_SLOTS + 0xbc,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c2_1a80, &args![this, update, 0u32]);
        assert_eq!(calls(&e, OTHER_SLOTS + 0x90), vec![vec![property, update]]);
        assert_eq!(
            calls(&e, MESH_SLOTS + 0x94),
            vec![vec![plain, update], vec![kept, update]]
        );
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == SYSTEM_SLOTS + 0xb8 || *a == MESH_SLOTS + 0x94)
            .collect();
        assert_eq!(
            order,
            vec![MESH_SLOTS + 0x94, SYSTEM_SLOTS + 0xb8, MESH_SLOTS + 0x94]
        );
        assert_eq!(calls(&e, SYSTEM_SLOTS + 0xbc), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x110), 0x3f00_0000);
        assert_eq!(e.mem.u8(this + 0x114), 1);
        // Without byte +4: only slots +0xb8 and +0xbc.
        e.mem.set_u8(update + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1a80, &args![this, update, 0u32]);
        assert!(calls(&e, MESH_SLOTS + 0x94).is_empty());
        assert_eq!(calls(&e, SYSTEM_SLOTS + 0xb8).len(), 1);
        assert_eq!(e.mem.u8(this + 0x114), 0);
    }

    #[test]
    fn mesh_object_forwarders() {
        let mut e = rig();
        let (this, _, meshes) = system(&mut e);
        noop(
            &mut e,
            &[
                GEOMETRY_UPDATE_PROPERTIES_DOWNWARD_00A80820,
                SYSTEM_SET_SELECTIVE_UPDATE_FLAGS_00C1B020,
                MESH_SLOTS + 0xb0,
                MESH_SLOTS + 0xd4,
                MESH_SLOTS + 0xa0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c2_1b60, &args![this, 0x33u32]);
        assert_eq!(
            calls(&e, GEOMETRY_UPDATE_PROPERTIES_DOWNWARD_00A80820),
            vec![vec![this, 0x33]]
        );
        assert_eq!(
            calls(&e, MESH_SLOTS + 0xb0),
            vec![vec![meshes, this + 0x9c]]
        );
        e.call(0x00c2_1b90, &args![this, 0x44u32]);
        assert_eq!(calls(&e, MESH_SLOTS + 0xd4), vec![vec![meshes, 0x44]]);
        let rigid = e.mem.alloc(4);
        e.mem.set_u32(rigid, 0x0101_0101);
        e.mem.set_u32(meshes + 0x30, 0x31);
        e.call(0x00c2_1bb0, &args![this, 0x55u32, 0u32, rigid]);
        assert_eq!(
            calls(&e, SYSTEM_SET_SELECTIVE_UPDATE_FLAGS_00C1B020),
            vec![vec![this, 0x55, 1, rigid]]
        );
        assert_eq!(
            calls(&e, MESH_SLOTS + 0xa0),
            vec![vec![meshes, 0x55, 1, rigid]]
        );
        assert_eq!(e.mem.u32(rigid), 0x0101_0100);
        assert_eq!(e.mem.u32(meshes + 0x30), 0x21);
    }

    #[test]
    fn world_data_and_bound_reach_the_mesh_object() {
        let mut e = rig();
        let (this, _, meshes) = system(&mut e);
        for word in 0..0xdu32 {
            e.mem.set_u32(this + 0x68 + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(this + 0x98, (-2.5f32).to_bits());
        noop(
            &mut e,
            &[
                SYSTEM_UPDATE_WORLD_DATA_00C1ADD0,
                MESH_SLOTS + 0xb8,
                SYSTEM_UPDATE_WORLD_BOUND_00C1AF60,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c2_1bf0, &args![this, 0x66u32]);
        assert_eq!(e.mem.u32(meshes + 0x34), 0x100);
        assert_eq!(e.mem.u32(meshes + 0x54), 0x108);
        assert_eq!(e.mem.u32(meshes + 0x58), 0x109);
        assert_eq!(e.mem.u32(meshes + 0x60), 0x10b);
        assert_eq!(e.mem.u32(meshes + 0x64), 2.5f32.to_bits());
        assert_eq!(calls(&e, MESH_SLOTS + 0xb8), vec![vec![meshes, 0x66]]);
        // The bound: the mesh object's, or the default.
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x20, bound);
        let source = e.mem.alloc(0x10);
        for word in 0..4u32 {
            e.mem.set_u32(source + 4 * word, 0x200 + word);
        }
        e.mem.set_u32(source + 0xc, 0x7f80_0001);
        e.mem.set_u32(meshes + 0x20, source);
        e.call(0x00c2_1c60, &args![this]);
        assert_eq!(e.mem.u32(bound + 8), 0x202);
        assert_eq!(e.mem.u32(bound + 0xc), 0x7fc0_0001);
        e.mem.set_u32(meshes + 0x20, 0);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(DEFAULT_BOUND_011F4288, 0x300);
        e.call(0x00c2_1c60, &args![this]);
        assert_eq!(e.mem.u32(bound), 0x300);
        assert_eq!(e.mem.u32(bound + 0xc), 0);
    }

    #[test]
    fn purge_renderer_data_reaches_meshes_pools_and_modifier_arrays() {
        let mut e = rig();
        let (this, data, meshes) = system(&mut e);
        e.mem.set_u16(data + 0x92, 0);
        noop(
            &mut e,
            &[GEOMETRY_PURGE_RENDERER_DATA_00A809B0, MESH_SLOTS + 0xd8],
        );
        // Modifiers: one of each purged kind, one of neither.
        let first = object(&mut e, OTHER_VTABLE, 0x70);
        let second = object(&mut e, OTHER_VTABLE, 0x70);
        let neither = object(&mut e, OTHER_VTABLE, 0x70);
        let records = e.mem.alloc(0x10);
        e.mem.set_u32(records + 4, PURGE_MODIFIER_RTTI_01202530);
        e.register_double(OTHER_SLOTS + 8, move |_, a| {
            if a[0] == first {
                PURGE_MODIFIER_RTTI_01202500.into_ret()
            } else if a[0] == second {
                records.into_ret()
            } else {
                (records + 8).into_ret()
            }
        });
        e.map(0x0120_2000, 0x1000);
        let entry = object(&mut e, MESH_VTABLE, 8);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, entry);
        e.mem.set_u32(first + 0x58, array);
        e.mem.set_u16(first + 0x60, 2);
        e.mem.set_u32(second + 0x1c, array);
        e.mem.set_u16(second + 0x22, 2);
        let items: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc)).collect();
        for (index, modifier) in [first, 0, second, neither].iter().enumerate() {
            e.mem.set_u32(items[index] + 8, *modifier);
            e.mem
                .set_u32(items[index], items.get(index + 1).copied().unwrap_or(0));
        }
        e.mem.set_u32(this + MODIFIER_HEAD, items[0]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1cb0, &args![this, 0x99u32]);
        assert_eq!(
            calls(&e, MESH_SLOTS + 0xd8),
            vec![vec![meshes, 0x99], vec![entry, 0x99], vec![entry, 0x99]]
        );
        // `neither` is asked twice, `first` once.
        assert_eq!(calls(&e, OTHER_SLOTS + 8).len(), 5);
    }

    #[test]
    fn viewer_strings_clone_factory_and_destructor() {
        let mut e = rig();
        e.map(0x0120_2000, 0x1000);
        e.set_global(NI_MESH_PARTICLE_SYSTEM_RTTI, 0x0abc_0000u32);
        noop(
            &mut e,
            &[
                SYSTEM_GET_VIEWER_STRINGS_00C1B850,
                STRINGS_GROW,
                STRINGS_SET_AT,
                SYSTEM_CONSTRUCT_00C1B5C0,
                SYSTEM_COPY_MEMBERS_00C1B6F0,
                LIST_REMOVE_ALL_00E74D40,
                GEOMETRY_DESTRUCT_00A80580,
                NI_FREE,
            ],
        );
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 3);
        let (this, _, _) = system(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1dc0, &args![this, strings]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        let set = calls(&e, STRINGS_SET_AT);
        assert_eq!(set[0][1], 2);
        assert_eq!(e.mem.u32(set[0][2]), 0x0abc_0001);
        // The clone: allocated, constructed, members copied, time and flag copied.
        let block = e.mem.alloc(0x118);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.mem.set_u32(this + 0x110, 0x4000_0000);
        e.mem.set_u8(this + 0x114, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_1e20, &args![this, 0x55u32]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x118]]);
        assert_eq!(calls(&e, SYSTEM_CONSTRUCT_00C1B5C0), vec![vec![block]]);
        assert_eq!(
            calls(&e, SYSTEM_COPY_MEMBERS_00C1B6F0),
            vec![vec![this, block, 0x55]]
        );
        assert_eq!(e.mem.u32(block), NI_MESH_PARTICLE_SYSTEM_VTABLE);
        assert_eq!(e.mem.u32(block + 0x110), 0x4000_0000);
        assert_eq!(e.mem.u8(block + 0x114), 0);
        // The factory leaves the flag set; a failed allocation answers 0.
        assert_eq!(e.call(0x00c2_1e90, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block + 0x110), 0);
        assert_eq!(e.mem.u8(block + 0x114), 1);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_1e90, &args![]).u32(), 0);
        // The deleting destructor frees only with bit 0.
        for (flags, freed) in [(1u32, 1usize), (0, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c2_1ed0, &args![this, flags]).u32(), this);
            assert_eq!(calls(&e, LIST_REMOVE_ALL_00E74D40), vec![vec![this + 0xc8]]);
            assert_eq!(calls(&e, GEOMETRY_DESTRUCT_00A80580), vec![vec![this]]);
            assert_eq!(calls(&e, NI_FREE).len(), freed);
        }
    }
}
