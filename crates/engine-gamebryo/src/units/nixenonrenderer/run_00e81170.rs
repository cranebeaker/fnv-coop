//! `NiXenonRenderer/run_00e81170` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a8f440` (unnamed; `thiscall`, no stack words) and
/// `NiD3DShaderInterface`'s vtable (`010ef39c`, RTTI `.?AVNiD3DShaderInterface@@`).
const BASE_CONSTRUCT_00A8F440: u32 = 0x00a8_f440;
const SHADER_INTERFACE_VTABLE_010EF39C: u32 = 0x010e_f39c;

// Translated from 00e818a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): the `NiD3DShaderInterface` constructor (the
/// vtable it stores has that class's RTTI), `thiscall`, no stack words: the base constructor
/// `00a8f440`, the words +0x14 to +0x1c and the byte +0x20 cleared, the vtable `010ef39c`.
/// Answers `this`.
pub fn fn_00e818a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00A8F440, &args![base]);
    for offset in [0x14u32, 0x18, 0x1c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x20, 0);
    e.mem.set_u32(base, SHADER_INTERFACE_VTABLE_010EF39C);
    this
}

/// What `00e812f0` calls: `00e6d760` (unnamed; `thiscall` on the renderer, one word: a
/// declaration's word, answers the vertex declaration), the renderer's packers
/// `NiXenonRenderer::PackGeometryBuffer` (`00e71fe0`, engine map; `thiscall`, five words) and
/// `00e6d780` (unnamed; `thiscall`, six words, the skinned form).
const DECLARATION_00E6D760: u32 = 0x00e6_d760;
const PACK_GEOMETRY_BUFFER_00E71FE0: u32 = 0x00e7_1fe0;
const PACK_SKINNED_00E6D780: u32 = 0x00e6_d780;

// Translated from 00e812f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): a shader's geometry packing and stream set-up,
/// `thiscall` with four words: the geometry, the skin partition, the buffer data and a word
/// never read. With a geometry and no word at +0x38: its data (+0xb8; the words below read
/// through it) and second word (+0xbc); the vertex declaration is set on the object at +0x1c
/// (its slot 0xac, `thiscall`: the declaration and 0) from the data's shader declaration (+0x30)
/// when that one's slot 0x8c answers set (`00e6d760` on the renderer +0x18 with its +0x2c),
/// else from the shader's own declaration (+0x28, its slot 0xb4) when there is one (nothing set
/// otherwise). Then the buffer data comes from the packer: with a skin partition its +0x28
/// (`00e6d780` with it, the data, the second word, the partition, +0x28 and 0), else the data's
/// +0x34 (`00e71fe0` with it, the data, the second word, +0x28 and 0). Without a geometry, or
/// with +0x38 set, the third word is the buffer data. When there is buffer data, each of its
/// streams (count +0x20, strides +0x24, chips +0x28; the chip's buffer +8) is set on the device
/// (+0x14; `SetStreamSource`, slot 0x190: stream, buffer, 0, stride) and its declaration (+0x34)
/// too (slot 0x1a0). Answers the buffer data.
pub fn fn_00e812f0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    partition: u32,
    buffer_data: u32,
    _unused_4: u32,
) -> u32 {
    let base = this.addr();
    let mut result = buffer_data;
    // Diagnostic only (`NV_HEAD_CHECK`, not exe behaviour): notes the geometry's skin and data.
    if super::run_00e6fbb0::head_check_enabled() {
        super::run_00e6fbb0::head_check_note_geometry(e, geometry);
    }
    if geometry != 0 && e.mem.u32(base.wrapping_add(0x38)) == 0 {
        let data = e.mem.u32(geometry.wrapping_add(0xb8));
        let second = e.mem.u32(geometry.wrapping_add(0xbc));
        let mut declaration = None;
        if data != 0 {
            let shader_declaration = e.mem.u32(data.wrapping_add(0x30));
            if shader_declaration != 0 && e.vcall(shader_declaration, 0x8c, &args![]).bool() {
                let word = e.mem.u32(shader_declaration.wrapping_add(0x2c));
                let renderer = e.mem.u32(base.wrapping_add(0x18));
                declaration = Some(e.call(DECLARATION_00E6D760, &args![renderer, word]).u32());
            }
        }
        if declaration.is_none() {
            let own = e.mem.u32(base.wrapping_add(0x28));
            if own != 0 {
                declaration = Some(e.vcall(own, 0xb4, &args![]).u32());
            }
        }
        if let Some(declaration) = declaration {
            let target = e.mem.u32(base.wrapping_add(0x1c));
            e.vcall(target, 0xac, &args![declaration, 0u32]);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        let own = e.mem.u32(base.wrapping_add(0x28));
        if partition != 0 {
            result = e.mem.u32(partition.wrapping_add(0x28));
            e.call(
                PACK_SKINNED_00E6D780,
                &args![renderer, result, data, second, partition, own, 0u32],
            );
        } else {
            result = e.mem.u32(data.wrapping_add(0x34));
            e.call(
                PACK_GEOMETRY_BUFFER_00E71FE0,
                &args![renderer, result, data, second, own, 0u32],
            );
        }
    }
    if result == 0 {
        return 0;
    }
    let mut stream = 0u32;
    let mut count = e.mem.u32(result.wrapping_add(0x20));
    while stream < count {
        let strides = e.mem.u32(result.wrapping_add(0x24));
        let stride = e.mem.u32(strides.wrapping_add(stream.wrapping_mul(4)));
        let chips = e.mem.u32(result.wrapping_add(0x28));
        let chip = e.mem.u32(chips.wrapping_add(stream.wrapping_mul(4)));
        let buffer = e.mem.u32(chip.wrapping_add(8));
        let device = e.mem.u32(base.wrapping_add(0x14));
        e.vcall(device, 0x190, &args![stream, buffer, 0u32, stride]);
        count = e.mem.u32(result.wrapping_add(0x20));
        stream += 1;
    }
    let device = e.mem.u32(base.wrapping_add(0x14));
    let declaration = e.mem.u32(result.wrapping_add(0x34));
    e.vcall(device, 0x1a0, &args![declaration]);
    result
}

/// The interlocked imports.
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e81420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): the smart-pointer assignment of the shader's
/// object at +0x28, `thiscall` with the new object. Nothing when it is already there;
/// otherwise the old one's count (+4) goes down (`InterlockedDecrement`; at 0 it is deleted
/// through its slot 4), the new one is stored and its count goes up when not null.
pub fn fn_00e81420(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    let old = e.mem.u32(base.wrapping_add(0x28));
    if old == object {
        return;
    }
    if old != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x28), object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// The current shader global (`0126f74c`).
const CURRENT_0126F74C: u32 = 0x0126_f74c;

// Translated from 00e81470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`), `thiscall`, no stack words. With a current
/// shader at `0126f74c`: its object at +0x44 gets its slot 0x84 and its object at +0x5c its slot
/// 0x9c (both `thiscall` without words) when set. Then `this`'s byte +0x10 is cleared.
pub fn fn_00e81470(e: &mut Engine, this: Ptr) {
    let current = e.mem.u32(CURRENT_0126F74C);
    if current != 0 {
        let first = e.mem.u32(current.wrapping_add(0x44));
        if first != 0 {
            e.vcall(first, 0x84, &args![]);
        }
        let second = e.mem.u32(current.wrapping_add(0x5c));
        if second != 0 {
            e.vcall(second, 0x9c, &args![]);
        }
    }
    e.mem.set_u8(this.addr().wrapping_add(0x10), 0);
}

/// `00e8bb70` (unnamed; `thiscall` on the renderer's +0x8c8, three words) and the render
/// state block's set-up `00e7eed0` (unnamed; `thiscall` on the block, no stack words).
const RENDERER_8C8_00E8BB70: u32 = 0x00e8_bb70;
const APPLY_RENDER_STATES_00E7EED0: u32 = 0x00e7_eed0;

// Translated from 00e81170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, seven stack words of which the fourth and fifth are
/// read; the shaders' `PreProcessPipeline` slot, +0x54 in the Xbox PDB's `NiD3DShader`):
/// -1 when the shader is not initialized (byte +0x10). Otherwise the render state (+0x1c)
/// gets its slot 8 with `a4` when the byte +0x24 is set; with the byte +0x25 set and the
/// renderer's (+0x18) +0x8c8 not null, that object gets `00e8bb70` with `a5`, the word at
/// `a4` +0x14 and 0; the render-state block (+0x2c), when there is one, is applied
/// (`00e7eed0`). Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e81170(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    a4: u32,
    a5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(0x10)) == 0 {
        return 0xffff_ffff;
    }
    if e.mem.u8(base.wrapping_add(0x24)) != 0 {
        let state = e.mem.u32(base.wrapping_add(0x1c));
        e.vcall(state, 8, &args![a4]);
    }
    if e.mem.u8(base.wrapping_add(0x25)) != 0 {
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        let object = e.mem.u32(renderer.wrapping_add(0x8c8));
        if object != 0 {
            let word = e.mem.u32(a4.wrapping_add(0x14));
            e.call(RENDERER_8C8_00E8BB70, &args![object, a5, word, 0u32]);
        }
    }
    let block = e.mem.u32(base.wrapping_add(0x2c));
    if block != 0 {
        e.call(APPLY_RENDER_STATES_00E7EED0, &args![block]);
    }
    0
}

/// `NiObjectNET::GetExtraData` (`00a5bdd0`, `thiscall`, the name), `RemoveExtraData`
/// (`00a5be90`, `thiscall`, the name) and `AddExtraData` (`00a5bca0`, engine map
/// `AddExtraData_ov2`; `thiscall`, the extra data), and the name `0126f750` they are given.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const REMOVE_EXTRA_DATA: u32 = 0x00a5_be90;
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
const EXTRA_DATA_NAME_0126F750: u32 = 0x0126_f750;

// Translated from 00e811d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, eight stack words; the shaders'
/// `SetupShaderPrograms` slot, +0x68 in the Xbox PDB): with a geometry, its extra data named
/// by `0126f750` (when there is one) gets +0x18 and +0x20 cleared. The current pass
/// (`0126f74c`) gets its slot 4 with the eight words and the shader's +0x38, and its answer is
/// the result. The shader's constant maps then set their constants (slot 0x78) when the pass
/// has the matching shader: the map at +0x30 with the pass's +0x44, the map at +0x34 with the
/// pass's +0x5c, each with that shader, the eight words, +0x38 and 1. Last, on the first pass
/// (+0x38 zero) with a declaration (+0x28), the render state (+0x1c) gets its slot 0xac with
/// the declaration's Direct3D declaration (its slot 0xb4) and 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e811d0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    let base = this.addr();
    if geometry != 0 {
        let extra = e
            .call(GET_EXTRA_DATA, &args![geometry, EXTRA_DATA_NAME_0126F750])
            .u32();
        if extra != 0 {
            e.mem.set_u32(extra.wrapping_add(0x18), 0);
            e.mem.set_u32(extra.wrapping_add(0x20), 0);
        }
    }
    let pass_word = e.mem.u32(base.wrapping_add(0x38));
    let pass = e.mem.u32(CURRENT_0126F74C);
    let result = e
        .vcall(
            pass,
            4,
            &args![geometry, a2, a3, a4, a5, a6, a7, a8, pass_word],
        )
        .u32();
    for (map_at, shader_at) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base.wrapping_add(map_at));
        if map == 0 {
            continue;
        }
        let pass = e.mem.u32(CURRENT_0126F74C);
        let shader = e.mem.u32(pass.wrapping_add(shader_at));
        if shader != 0 {
            let pass_word = e.mem.u32(base.wrapping_add(0x38));
            e.vcall(
                map,
                0x78,
                &args![shader, geometry, a2, a3, a4, a5, a6, a7, a8, pass_word, 1u32],
            );
        }
    }
    if e.mem.u32(base.wrapping_add(0x38)) == 0 {
        let own = e.mem.u32(base.wrapping_add(0x28));
        if own != 0 {
            // The render state's vtable is read before the declaration is asked for, its
            // slot after.
            let state = e.mem.u32(base.wrapping_add(0x1c));
            let vtable = e.mem.u32(state);
            let declaration = e.vcall(own, 0xb4, &args![]).u32();
            let state = e.mem.u32(base.wrapping_add(0x1c));
            let set = e.mem.u32(vtable.wrapping_add(0xac));
            e.call(set, &args![state, declaration, 0u32]);
        }
    }
    result
}

// Translated from 00e814b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; reached through several shader
/// vtables and the jump `00be08b0`): with the word +0x18 set, the current pass's (`0126f74c`)
/// shader at +0x44 gets its slot 0x88 and the one at +0x5c its slot 0xa0 (each when set),
/// and the byte +0x10 is set (also without a current pass).
pub fn fn_00e814b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x18)) == 0 {
        return;
    }
    let pass = e.mem.u32(CURRENT_0126F74C);
    if pass != 0 {
        let first = e.mem.u32(pass.wrapping_add(0x44));
        if first != 0 {
            e.vcall(first, 0x88, &args![]);
        }
        let second = e.mem.u32(pass.wrapping_add(0x5c));
        if second != 0 {
            e.vcall(second, 0xa0, &args![]);
        }
    }
    e.mem.set_u8(base.wrapping_add(0x10), 1);
}

// Translated from 00e814f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a shader vtable slot, `this` not read): whether the current
/// pass (`0126f74c`) has a shader at +0x5c, in AL; the rest of EAX is the pass pointer's.
pub fn fn_00e814f0(e: &mut Engine, _unused_this: Ptr) -> u32 {
    let pass = e.mem.u32(CURRENT_0126F74C);
    (pass & 0xffff_ff00) | u32::from(e.mem.u32(pass.wrapping_add(0x5c)) != 0)
}

// Translated from 00e81500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: as [`fn_00e814f0`] for the pass's shader at +0x44.
pub fn fn_00e81500(e: &mut Engine, _unused_this: Ptr) -> u32 {
    let pass = e.mem.u32(CURRENT_0126F74C);
    (pass & 0xffff_ff00) | u32::from(e.mem.u32(pass.wrapping_add(0x44)) != 0)
}

/// `00ee8910` (unnamed; `thiscall` on the new 0x30-byte extra data: the name and the two
/// counts), the shift `01101294` applied to 0xff for the shader's own constants, and the Ni
/// allocation `00aa13e0` (`cdecl`, the size).
const EXTRA_DATA_CONSTRUCT_00EE8910: u32 = 0x00ee_8910;
const SHIFT_01101294: u32 = 0x0110_1294;
const NI_NEW_00AA13E0: u32 = 0x00aa_13e0;

/// Whether a constant map entry (slot 0x74 answer) counts: not null and its type (the top
/// four bits of +0x14) 3.
fn counted_entry(e: &Engine, entry: u32) -> bool {
    entry != 0 && e.mem.u32(entry.wrapping_add(0x14)) & 0xf000_0000 == 0x3000_0000
}

/// The counted entries of the map at `map` (read again for each entry when `reread` holds
/// the address it is read from).
fn count_entries(e: &mut Engine, map: u32, reread: Option<u32>) -> u32 {
    let mut found = 0u32;
    if map == 0 {
        return found;
    }
    let count = u32::from(e.mem.u16(map.wrapping_add(0x18)));
    for index in 0..count {
        let map = reread.map_or(map, |at| e.mem.u32(at));
        let entry = e.vcall(map, 0x74, &args![index]).u32();
        if counted_entry(e, entry) {
            found += 1;
        }
    }
    found
}

/// The entries of the map at `map` whose extra data the geometry has (by the entry's name at
/// +0xc) appended to the 8-byte records of the new extra data (`records` +0, `count` +0): the
/// entry's +0x1c ored with `0xff << shift` (or with 0 for `shifted` false) and the extra data.
#[allow(clippy::too_many_arguments)]
fn record_entries(
    e: &mut Engine,
    map: u32,
    reread: Option<u32>,
    count: u32,
    geometry: u32,
    extra: u32,
    records: u32,
    tally: u32,
    shifted: bool,
) {
    for index in 0..count {
        let map = reread.map_or(map, |at| e.mem.u32(at));
        let entry = e.vcall(map, 0x74, &args![index]).u32();
        if !counted_entry(e, entry) {
            continue;
        }
        let found = e
            .call(GET_EXTRA_DATA, &args![geometry, entry.wrapping_add(0xc)])
            .u32();
        if found == 0 {
            continue;
        }
        let shift = e.mem.u32(SHIFT_01101294) & 0x1f;
        let high = if shifted { 0xffu32 << shift } else { 0 };
        let value = high | e.mem.u32(entry.wrapping_add(0x1c));
        let index = e.mem.u32(extra.wrapping_add(tally));
        let array = e.mem.u32(extra.wrapping_add(records));
        e.mem
            .set_u32(array.wrapping_add(index.wrapping_mul(8)), value);
        let index = e.mem.u32(extra.wrapping_add(tally));
        let array = e.mem.u32(extra.wrapping_add(records));
        e.mem.set_u32(
            array.wrapping_add(index.wrapping_mul(8)).wrapping_add(4),
            found,
        );
        let next = e.mem.u32(extra.wrapping_add(tally)).wrapping_add(1);
        e.mem.set_u32(extra.wrapping_add(tally), next);
    }
}

// Translated from 00e81510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the shader and the geometry; called by `fn_00e81880`):
/// rebuilds the geometry's constant extra data (named by `0126f750`). The old one is removed
/// (`RemoveExtraData`). The type-3 entries (`(+0x14 >> 28) == 3`) of the shader's constant
/// maps (+0x34 counted first, +0x30 second; each map read again for each entry) and of the
/// current pass's (`0126f74c`) maps (+0x34 adding to the second count, +0x48 to the first)
/// are counted. With none, nothing more. Otherwise a 0x30-byte extra data is made (`00ee8910`
/// with the name and the two counts; null when the allocation fails, the stores still go to
/// it) and, with a current pass, filled with the entries the geometry has extra data for
/// (by the entry's name at +0xc): the shader's +0x30 map into the records at +0x2c (count
/// +0x20) and its +0x34 map into those at +0x24 (count +0x18), each value being the entry's
/// +0x1c ored with `0xff << [01101294]`; then the pass's +0x34 map into +0x2c and its +0x48
/// map into +0x24 with the entry's +0x1c alone. The extra data is added to the geometry
/// (`AddExtraData`).
pub fn fn_00e81510(e: &mut Engine, shader: Ptr, geometry: u32) {
    let shader = shader.addr();
    e.call(
        REMOVE_EXTRA_DATA,
        &args![geometry, EXTRA_DATA_NAME_0126F750],
    );
    let vertex_map_at = shader.wrapping_add(0x34);
    let pixel_map_at = shader.wrapping_add(0x30);
    let vertex_map = e.mem.u32(vertex_map_at);
    let mut first = count_entries(e, vertex_map, Some(vertex_map_at));
    let pixel_map = e.mem.u32(pixel_map_at);
    let mut second = count_entries(e, pixel_map, Some(pixel_map_at));
    let pass = e.mem.u32(CURRENT_0126F74C);
    if pass != 0 {
        let map = e.mem.u32(pass.wrapping_add(0x34));
        second = second.wrapping_add(count_entries(e, map, None));
        let map = e.mem.u32(pass.wrapping_add(0x48));
        first = first.wrapping_add(count_entries(e, map, None));
    }
    if first == 0 && second == 0 {
        return;
    }
    let block = e.call(NI_NEW_00AA13E0, &args![0x30u32]).u32();
    let extra = if block != 0 {
        e.call(
            EXTRA_DATA_CONSTRUCT_00EE8910,
            &args![block, EXTRA_DATA_NAME_0126F750, first, second],
        )
        .u32()
    } else {
        0
    };
    let pass = e.mem.u32(CURRENT_0126F74C);
    if pass != 0 {
        for (map_at, records, tally) in [
            (pixel_map_at, 0x2cu32, 0x20u32),
            (vertex_map_at, 0x24, 0x18),
        ] {
            let map = e.mem.u32(map_at);
            let count = if map != 0 {
                u32::from(e.mem.u16(map.wrapping_add(0x18)))
            } else {
                0
            };
            record_entries(
                e,
                map,
                Some(map_at),
                count,
                geometry,
                extra,
                records,
                tally,
                true,
            );
        }
        for (map_at, records, tally) in [(0x34u32, 0x2cu32, 0x20u32), (0x48, 0x24, 0x18)] {
            let map = e.mem.u32(pass.wrapping_add(map_at));
            if map == 0 {
                continue;
            }
            let count = u32::from(e.mem.u16(map.wrapping_add(0x18)));
            record_entries(e, map, None, count, geometry, extra, records, tally, false);
        }
    }
    e.call(ADD_EXTRA_DATA, &args![geometry, extra]);
}

// Translated from 00e81880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the geometry; the shaders'
/// `SetupGeometry` slot, +0x50 in the Xbox PDB): rebuilds the geometry's constant extra data
/// ([`fn_00e81510`]) and answers `true`.
pub fn fn_00e81880(e: &mut Engine, this: Ptr, geometry: u32) -> bool {
    fn_00e81510(e, this, geometry);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e8_1170,
            fn_00e81170(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_11d0,
            fn_00e811d0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_14b0, fn_00e814b0(Ptr)),
        entry!(0x00e8_14f0, fn_00e814f0(Ptr) -> u32),
        entry!(0x00e8_1500, fn_00e81500(Ptr) -> u32),
        entry!(0x00e8_1510, fn_00e81510(Ptr, u32)),
        entry!(0x00e8_1880, fn_00e81880(Ptr, u32) -> bool),
        entry!(0x00e8_18a0, fn_00e818a0(Ptr) -> Ptr),
        entry!(0x00e8_12f0, fn_00e812f0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_1420, fn_00e81420(Ptr, u32)),
        entry!(0x00e8_1470, fn_00e81470(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e818a0_builds_the_shader_interface() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00A8F440, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xcc; 0x24]);
        assert_eq!(e.call(0x00e8_18a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_INTERFACE_VTABLE_010EF39C);
        assert_eq!(e.mem.bytes(this + 0x14, 0xd), vec![0; 0xd]);
        assert_eq!(e.mem.u8(this + 0x21), 0xcc);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCT_00A8F440, vec![this]));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const SET_DECLARATION: u32 = 0x0e70_00ac;
    const OWN_DECLARATION: u32 = 0x0e70_00b4;
    const IS_READY: u32 = 0x0e70_008c;
    const SET_STREAM: u32 = 0x0e70_0190;
    const SET_VERTEX_DECLARATION: u32 = 0x0e70_01a0;
    const TABLE: u32 = 0x0e71_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// One table for every object the shader touches (slots 0x8c, 0xac, 0xb4, 0x190, 0x1a0).
    fn engine(ready: bool) -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x69];
        slots[0x8c / 4] = IS_READY;
        slots[0xac / 4] = SET_DECLARATION;
        slots[0xb4 / 4] = OWN_DECLARATION;
        slots[0x190 / 4] = SET_STREAM;
        slots[0x1a0 / 4] = SET_VERTEX_DECLARATION;
        e.put_vtable(TABLE, &slots);
        for no_op in [
            SET_DECLARATION,
            SET_STREAM,
            SET_VERTEX_DECLARATION,
            PACK_GEOMETRY_BUFFER_00E71FE0,
            PACK_SKINNED_00E6D780,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(OWN_DECLARATION, |_, _| 0xdd01u32.into_ret());
        e.register(DECLARATION_00E6D760, |_, _| 0xdd02u32.into_ret());
        if ready {
            e.register(IS_READY, |_, _| 1u32.into_ret());
        } else {
            e.register(IS_READY, |_, _| 0u32.into_ret());
        }
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, TABLE);
        object
    }

    /// A shader (+0x14 device, +0x18 renderer, +0x1c target, +0x28 own object), a geometry
    /// whose data has a shader declaration and buffer data with two streams.
    fn rig(e: &mut Engine) -> (u32, u32, u32, u32) {
        let this = object(e, 0x40);
        let device = object(e, 4);
        let target = object(e, 4);
        let own = object(e, 8);
        e.mem.set_u32(this + 0x14, device);
        e.mem.set_u32(this + 0x18, 0x7100);
        e.mem.set_u32(this + 0x1c, target);
        e.mem.set_u32(this + 0x28, own);
        let declaration = object(e, 0x30);
        e.mem.set_u32(declaration + 0x2c, 0x2c2c);
        let buffer_data = e.mem.alloc(0x38);
        let strides = e.mem.alloc(8);
        e.mem.set_u32(strides, 0x10);
        e.mem.set_u32(strides + 4, 0x20);
        let chips = e.mem.alloc(8);
        for index in 0..2u32 {
            let chip = e.mem.alloc(0xc);
            e.mem.set_u32(chip + 8, 0xb0 + index);
            e.mem.set_u32(chips + 4 * index, chip);
        }
        e.mem.set_u32(buffer_data + 0x20, 2);
        e.mem.set_u32(buffer_data + 0x24, strides);
        e.mem.set_u32(buffer_data + 0x28, chips);
        e.mem.set_u32(buffer_data + 0x34, 0xdec1);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x30, declaration);
        e.mem.set_u32(data + 0x34, buffer_data);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xbc, 0x5ec0);
        (this, geometry, data, buffer_data)
    }

    #[test]
    fn fn_00e812f0_packs_and_sets_the_streams() {
        let mut e = engine(true);
        let (this, geometry, data, buffer_data) = rig(&mut e);
        let device = e.mem.u32(this + 0x14);
        let target = e.mem.u32(this + 0x1c);
        let own = e.mem.u32(this + 0x28);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(answered, buffer_data);
        assert_eq!(calls(&e, DECLARATION_00E6D760), vec![vec![0x7100, 0x2c2c]]);
        assert_eq!(calls(&e, SET_DECLARATION), vec![vec![target, 0xdd02, 0]]);
        assert_eq!(
            calls(&e, PACK_GEOMETRY_BUFFER_00E71FE0),
            vec![vec![0x7100, buffer_data, data, 0x5ec0, own, 0]]
        );
        assert_eq!(
            calls(&e, SET_STREAM),
            vec![
                vec![device, 0, 0xb0, 0, 0x10],
                vec![device, 1, 0xb1, 0, 0x20]
            ]
        );
        assert_eq!(
            calls(&e, SET_VERTEX_DECLARATION),
            vec![vec![device, 0xdec1]]
        );
    }

    #[test]
    fn fn_00e812f0_falls_back_to_its_own_declaration_and_the_skinned_packer() {
        let mut e = engine(false);
        let (this, geometry, data, _) = rig(&mut e);
        let target = e.mem.u32(this + 0x1c);
        let own = e.mem.u32(this + 0x28);
        let partition_data = e.mem.alloc(0x38);
        e.mem.set_u32(partition_data + 0x20, 0);
        let partition = e.mem.alloc(0x2c);
        e.mem.set_u32(partition + 0x28, partition_data);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, partition, 0u32, 0u32])
            .u32();
        assert_eq!(answered, partition_data);
        assert_eq!(calls(&e, OWN_DECLARATION), vec![vec![own]]);
        assert_eq!(calls(&e, SET_DECLARATION), vec![vec![target, 0xdd01, 0]]);
        assert_eq!(
            calls(&e, PACK_SKINNED_00E6D780),
            vec![vec![
                0x7100,
                partition_data,
                data,
                0x5ec0,
                partition,
                own,
                0
            ]]
        );
        assert!(calls(&e, SET_STREAM).is_empty());
        assert_eq!(calls(&e, SET_VERTEX_DECLARATION).len(), 1);
    }

    #[test]
    fn fn_00e812f0_uses_the_given_buffer_data_when_locked() {
        let mut e = engine(true);
        let (this, geometry, _, buffer_data) = rig(&mut e);
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        let answered = e
            .call(0x00e8_12f0, &args![this, geometry, 0u32, buffer_data, 0u32])
            .u32();
        assert_eq!(answered, buffer_data);
        assert!(calls(&e, SET_DECLARATION).is_empty());
        assert_eq!(calls(&e, SET_STREAM).len(), 2);
        // No geometry and no buffer data: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_12f0, &args![this, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e81420_assigns_the_counted_object() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(0x0e72_0004, |_, _| Ret::default());
        e.put_vtable(0x0e73_0000, &[0, 0x0e72_0004]);
        let (old, new) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(old, 0x0e73_0000);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, old);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, new]);
        assert_eq!(e.mem.u32(this + 0x28), new);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![old + 4]]);
        assert_eq!(calls(&e, 0x0e72_0004), vec![vec![old]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT), vec![vec![new + 4]]);
        // The same object: nothing; a count left above 0: not deleted.
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, new]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.mem.set_u32(new, 0x0e73_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1420, &args![this, 0u32]);
        assert!(calls(&e, 0x0e72_0004).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn fn_00e81470_tells_the_current_shader() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let mut slots = vec![0u32; 0x28];
        slots[0x84 / 4] = 0x0e74_0084;
        slots[0x9c / 4] = 0x0e74_009c;
        e.put_vtable(0x0e75_0000, &slots);
        e.register(0x0e74_0084, |_, _| Ret::default());
        e.register(0x0e74_009c, |_, _| Ret::default());
        let (first, second) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(first, 0x0e75_0000);
        e.mem.set_u32(second, 0x0e75_0000);
        let current = e.mem.alloc(0x60);
        e.mem.set_u32(current + 0x44, first);
        e.mem.set_u32(current + 0x5c, second);
        e.mem.set_u32(CURRENT_0126F74C, current);
        let this = e.mem.alloc(0x14);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1470, &args![this]);
        assert_eq!(calls(&e, 0x0e74_0084), vec![vec![first]]);
        assert_eq!(calls(&e, 0x0e74_009c), vec![vec![second]]);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        // No current shader: only the byte.
        e.mem.set_u32(CURRENT_0126F74C, 0);
        e.mem.set_u8(this + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1470, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    const SLOTS: u32 = 0x0e50_0000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slots point at `slots + slot`, each answering 0.
    fn object(e: &mut Engine, slots: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(vtable + slot, slots + slot);
            e.register(slots + slot, |_, _| Ret::default());
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0110_1000, 0x1000);
        e.mem.set_u32(SHIFT_01101294, 16);
        for callee in [
            RENDERER_8C8_00E8BB70,
            APPLY_RENDER_STATES_00E7EED0,
            REMOVE_EXTRA_DATA,
            ADD_EXTRA_DATA,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e81170_refuses_an_uninitialized_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_1170, &words).u32(), 0xffff_ffff);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e81170_sets_up_state_and_applies_the_block() {
        let mut e = engine();
        let state = object(&mut e, SLOTS, 8);
        let renderer = e.mem.alloc(0x8cc);
        e.mem.set_u32(renderer + 0x8c8, 0x0e60_0000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x10, 1);
        e.mem.set_u8(this + 0x24, 1);
        e.mem.set_u8(this + 0x25, 1);
        e.mem.set_u32(this + 0x18, renderer);
        e.mem.set_u32(this + 0x1c, state);
        e.mem.set_u32(this + 0x2c, 0x0e70_0000);
        let property = e.mem.alloc(0x18);
        e.mem.set_u32(property + 0x14, 0x99);
        let words = args![this, 1u32, 2u32, 3u32, property, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00e8_1170, &words).u32(), 0);
        assert_eq!(calls(&e, SLOTS + 8), vec![vec![state, property]]);
        assert_eq!(
            calls(&e, RENDERER_8C8_00E8BB70),
            vec![vec![0x0e60_0000, 5, 0x99, 0]]
        );
        assert_eq!(
            calls(&e, APPLY_RENDER_STATES_00E7EED0),
            vec![vec![0x0e70_0000]]
        );
        // Without the flags and the block: only the answer.
        e.mem.set_u8(this + 0x24, 0);
        e.mem.set_u8(this + 0x25, 0);
        e.mem.set_u32(this + 0x2c, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_1170, &words).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e811d0_runs_the_pass_and_the_constant_maps() {
        let mut e = engine();
        let pass = object(&mut e, SLOTS, 0x60);
        e.register(SLOTS + 4, |_, _| 0x55u32.into_ret());
        e.mem.set_u32(CURRENT_0126F74C, pass);
        e.mem.set_u32(pass + 0x44, 0x0e61_0000);
        let pixel_map = object(&mut e, SLOTS + 0x1000, 8);
        let vertex_map = object(&mut e, SLOTS + 0x2000, 8);
        let declaration = object(&mut e, SLOTS + 0x3000, 8);
        e.register(SLOTS + 0x3000 + 0xb4, |_, _| 0x0e62_0000u32.into_ret());
        let state = object(&mut e, SLOTS + 0x4000, 8);
        let extra = e.mem.alloc(0x24);
        e.mem.write(extra, &[0xff; 0x24]);
        e.register_double(GET_EXTRA_DATA, move |_, _| extra.into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x1c, state);
        e.mem.set_u32(this + 0x28, declaration);
        e.mem.set_u32(this + 0x30, pixel_map);
        e.mem.set_u32(this + 0x34, vertex_map);
        let words = args![
            this,
            0x0e10_0000u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            7u32,
            8u32
        ];
        assert_eq!(e.call(0x00e8_11d0, &words).u32(), 0x55);
        assert_eq!(e.mem.u32(extra + 0x18), 0);
        assert_eq!(e.mem.u32(extra + 0x20), 0);
        assert_eq!(
            calls(&e, GET_EXTRA_DATA),
            vec![vec![0x0e10_0000, EXTRA_DATA_NAME_0126F750]]
        );
        assert_eq!(
            calls(&e, SLOTS + 4),
            vec![vec![pass, 0x0e10_0000, 2, 3, 4, 5, 6, 7, 8, 0]]
        );
        // The pixel map runs with the pass shader at +0x44; the vertex map finds none at +0x5c.
        assert_eq!(
            calls(&e, SLOTS + 0x1000 + 0x78),
            vec![vec![
                pixel_map,
                0x0e61_0000,
                0x0e10_0000,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                0,
                1
            ]]
        );
        assert!(calls(&e, SLOTS + 0x2000 + 0x78).is_empty());
        assert_eq!(
            calls(&e, SLOTS + 0x4000 + 0xac),
            vec![vec![state, 0x0e62_0000, 0]]
        );
        // A later pass (+0x38 set) sets no declaration.
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_11d0, &words);
        assert!(calls(&e, SLOTS + 0x4000 + 0xac).is_empty());
        assert_eq!(calls(&e, SLOTS + 4)[0][9], 1);
    }

    #[test]
    fn fn_00e814b0_tells_the_pass_shaders() {
        let mut e = engine();
        let pass = e.mem.alloc(0x60);
        let first = object(&mut e, SLOTS, 8);
        let second = object(&mut e, SLOTS + 0x1000, 8);
        e.mem.set_u32(pass + 0x44, first);
        e.mem.set_u32(pass + 0x5c, second);
        e.mem.set_u32(CURRENT_0126F74C, pass);
        let this = e.mem.alloc(0x20);
        e.call(0x00e8_14b0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        e.mem.set_u32(this + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_14b0, &args![this]);
        assert_eq!(calls(&e, SLOTS + 0x88), vec![vec![first]]);
        assert_eq!(calls(&e, SLOTS + 0x1000 + 0xa0), vec![vec![second]]);
        assert_eq!(e.mem.u8(this + 0x10), 1);
    }

    #[test]
    fn fn_00e814f0_answers_whether_the_pass_has_its_second_shader() {
        let mut e = engine();
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(CURRENT_0126F74C, pass);
        assert_eq!(e.call(0x00e8_14f0, &args![0u32]).u32(), pass & 0xffff_ff00);
        e.mem.set_u32(pass + 0x5c, 7);
        assert_eq!(
            e.call(0x00e8_14f0, &args![0u32]).u32(),
            (pass & 0xffff_ff00) | 1
        );
    }

    #[test]
    fn fn_00e81500_answers_whether_the_pass_has_its_first_shader() {
        let mut e = engine();
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(CURRENT_0126F74C, pass);
        assert_eq!(e.call(0x00e8_1500, &args![0u32]).u32() & 0xff, 0);
        e.mem.set_u32(pass + 0x44, 7);
        assert_eq!(e.call(0x00e8_1500, &args![0u32]).u32() & 0xff, 1);
    }

    /// A constant map whose slot 0x74 answers the entries of `kinds` (null for kind 0): each
    /// entry has the type bits `kind << 28` and the value `0x100 + index` at +0x1c.
    fn map(e: &mut Engine, slots: u32, kinds: &'static [u32]) -> u32 {
        let map = object(e, slots, 0x1c);
        e.mem.set_u16(map + 0x18, kinds.len() as u16);
        let entries = e.mem.alloc(0x20 * kinds.len() as u32);
        for (i, kind) in kinds.iter().enumerate() {
            let entry = entries + 0x20 * i as u32;
            e.mem.set_u32(entry + 0x14, kind << 28);
            e.mem.set_u32(entry + 0x1c, 0x100 + i as u32);
        }
        e.register_double(slots + 0x74, move |_, a| {
            if kinds[a[1] as usize] == 0 {
                Ret::default()
            } else {
                (entries + 0x20 * a[1]).into_ret()
            }
        });
        map
    }

    #[test]
    fn fn_00e81510_without_constants_only_removes_the_old_data() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        let pixel = map(&mut e, SLOTS, &[1, 0]);
        e.mem.set_u32(shader + 0x30, pixel);
        e.call(0x00e8_1510, &args![shader, 0x0e10_0000u32]);
        assert_eq!(
            calls(&e, REMOVE_EXTRA_DATA),
            vec![vec![0x0e10_0000, EXTRA_DATA_NAME_0126F750]]
        );
        assert!(calls(&e, ADD_EXTRA_DATA).is_empty());
    }

    #[test]
    fn fn_00e81510_builds_the_records() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        let pixel = map(&mut e, SLOTS, &[3, 1, 3]);
        let vertex = map(&mut e, SLOTS + 0x1000, &[3]);
        e.mem.set_u32(shader + 0x30, pixel);
        e.mem.set_u32(shader + 0x34, vertex);
        let pass = e.mem.alloc(0x60);
        let pass_pixel = map(&mut e, SLOTS + 0x2000, &[3]);
        e.mem.set_u32(pass + 0x34, pass_pixel);
        e.mem.set_u32(CURRENT_0126F74C, pass);
        let block = e.mem.alloc(0x30);
        e.register_double(NI_NEW_00AA13E0, move |_, _| block.into_ret());
        let records_a = e.mem.alloc(0x40);
        let records_b = e.mem.alloc(0x40);
        e.register_double(EXTRA_DATA_CONSTRUCT_00EE8910, move |e, a| {
            e.mem.set_u32(a[0] + 0x24, records_b);
            e.mem.set_u32(a[0] + 0x2c, records_a);
            a[0].into_ret()
        });
        // The geometry has extra data for every entry: the double answers the name address.
        e.register(GET_EXTRA_DATA, |_, a| a[1].into_ret());
        e.call(0x00e8_1510, &args![shader, 0x0e10_0000u32]);
        // Counted: first 1 (the vertex map), second 2 (the pixel map) + 1 (the pass map).
        assert_eq!(
            calls(&e, EXTRA_DATA_CONSTRUCT_00EE8910),
            vec![vec![block, EXTRA_DATA_NAME_0126F750, 1, 3]]
        );
        // The pixel map entries 0 and 2 shifted, then the pass map entry unshifted.
        assert_eq!(e.mem.u32(block + 0x20), 3);
        assert_eq!(e.mem.u32(records_a), 0x00ff_0100);
        assert_eq!(e.mem.u32(records_a + 8), 0x00ff_0102);
        assert_eq!(e.mem.u32(records_a + 0x10), 0x100);
        assert_eq!(e.mem.u32(block + 0x18), 1);
        assert_eq!(e.mem.u32(records_b), 0x00ff_0100);
        assert_eq!(calls(&e, ADD_EXTRA_DATA), vec![vec![0x0e10_0000, block]]);
    }

    #[test]
    fn fn_00e81880_rebuilds_and_answers_true() {
        let mut e = engine();
        let shader = e.mem.alloc(0x40);
        assert!(e.call(0x00e8_1880, &args![shader, 0x0e10_0000u32]).bool());
        assert_eq!(
            calls(&e, REMOVE_EXTRA_DATA),
            vec![vec![0x0e10_0000, EXTRA_DATA_NAME_0126F750]]
        );
    }
}
