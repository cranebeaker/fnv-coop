//! A shader's geometry packing and stream set-up (`00e812f0`, the census's `PrepareShaderAndStreams`)
//! and the renderer's packing of a geometry's buffer (`NiXenonRenderer::PackGeometryBuffer`), what the
//! batch renderer's draw (a shader's slot 0x6c) runs for each plain geometry.
//!
//! The translation of `00e812f0` also notes the geometry for the `NV_HEAD_CHECK` diagnostic (not exe
//! behaviour); with this family on that diagnostic's notes are not taken (`NV_TYPED=render:off`), and
//! [`note_head_check`] says so once.

use super::FAMILY;
use crate::shader::state;
use engine_core::ptr::Ptr;
use engine_core::{args, typed_call, Engine};

/// With `NV_HEAD_CHECK` set, one line saying this family's stream set-up takes none of that
/// diagnostic's notes (the translation of `00e812f0` does): `NV_TYPED=render:off` for them.
pub fn note_head_check() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    if std::env::var_os("NV_HEAD_CHECK").is_some() {
        ONCE.call_once(|| {
            println!("head check: the render family's stream set-up takes no notes; NV_TYPED=render:off for them");
        });
    }
}

/// `00e6d760` (`thiscall` on the renderer, a declaration word): the vertex declaration.
const DECLARATION: u32 = 0x00e6_d760;
/// `00e6d780` (`thiscall` on the renderer, six words): the skinned packer.
const PACK_SKINNED: u32 = 0x00e6_d780;
/// The stream limit (`011bf558`).
const STREAM_LIMIT: u32 = 0x011b_f558;
/// The RTTI records the packer tells the data's kind by: the triangle list data (walked up the data's
/// own RTTI chain) and the strip data (`IsKindOf`, `0043b300`).
const TRI_LIST_DATA_RTTI: u32 = 0x011f_4a88;
const STRIP_DATA_RTTI: u32 = 0x011f_4a70;
const IS_KIND_OF: u32 = 0x0043_b300;
/// `NiTriShapeData::GetTriList` (`00a7b850`) and `NiTriStripsData::GetStripLists` (`00a75f10`).
const GET_TRI_LIST: u32 = 0x00a7_b850;
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// `NiD3DHLSLVertexShader::GetEntryPoint` (`00e8bf90`, on the renderer's +0x8bc), the buffer data's
/// stream count set (`00e8ef10`), `PackUntransformedVB` (`00e8c1e0`), the index buffer maker
/// (`00e8f200`, on the renderer's +0x8c0) and `NiGeometryBufferData::RemoveIB` (`00e8eee0`).
const GET_ENTRY_POINT: u32 = 0x00e8_bf90;
const SET_STREAM_COUNT: u32 = 0x00e8_ef10;
const PACK_UNTRANSFORMED_VB: u32 = 0x00e8_c1e0;
const INDEX_BUFFER: u32 = 0x00e8_f200;
const REMOVE_IB: u32 = 0x00e8_eee0;
/// Whether the device is Direct3D 9Ex (`0126f0d0`, a byte).
const DIRECT3D_IS_EX: u32 = 0x0126_f0d0;
/// The render state cache's `SetVertexDeclaration` (slot 0xac, `00e90fc0`).
const SLOT_SET_DECLARATION: u32 = 0xac;
const SET_DECLARATION: u32 = 0x00e9_0fc0;

// Typed from 00e812f0
/// `thiscall(shader, geometry, partition, buffer data, unused)`: with a geometry and no word at +0x38,
/// its data (+0xb8) and second word (+0xbc); the vertex declaration is set on the object at +0x1c (slot
/// 0xac with the declaration and 0) from the data's shader declaration (+0x30) when that one's slot 0x8c
/// answers set (`00e6d760` on the renderer +0x18 with its +0x2c), else from the shader's own (+0x28, its
/// slot 0xb4) when there is one. The buffer data comes from the packer: with a skin partition its
/// +0x28 (`00e6d780`), else the data's +0x34 ([`pack_geometry_buffer`]). Without a geometry, or with
/// +0x38 set, the fourth word is the buffer data. With buffer data, each stream is set on the device
/// (+0x14; `SetStreamSource`, slot 0x190) and its declaration (+0x34; slot 0x1a0). Answers the buffer
/// data.
pub fn prepare_streams(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    partition: u32,
    buffer_data: u32,
    _unused: u32,
) -> u32 {
    let base = this.addr();
    let mut result = buffer_data;
    if geometry != 0 && e.mem.u32(base.wrapping_add(0x38)) == 0 {
        let data = e.mem.u32(geometry.wrapping_add(0xb8));
        let second = e.mem.u32(geometry.wrapping_add(0xbc));
        let mut declaration = None;
        if data != 0 {
            let shader_declaration = e.mem.u32(data.wrapping_add(0x30));
            if shader_declaration != 0 && e.vcall(shader_declaration, 0x8c, &args![]).bool() {
                let word = e.mem.u32(shader_declaration.wrapping_add(0x2c));
                let renderer = e.mem.u32(base.wrapping_add(0x18));
                declaration = Some(e.call(DECLARATION, &args![renderer, word]).u32());
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
            if crate::slot_target(e, target, SLOT_SET_DECLARATION) == SET_DECLARATION {
                typed_call!(
                    crate::shader::FAMILY,
                    0x00e9_0fc0,
                    state::set_declaration(e, Ptr::new(target), declaration, 0u8)
                );
            } else {
                e.vcall(target, SLOT_SET_DECLARATION, &args![declaration, 0u32]);
            }
        }
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        let own = e.mem.u32(base.wrapping_add(0x28));
        if partition != 0 {
            result = e.mem.u32(partition.wrapping_add(0x28));
            e.call(
                PACK_SKINNED,
                &args![renderer, result, data, second, partition, own, 0u32],
            );
        } else {
            result = e.mem.u32(data.wrapping_add(0x34));
            let _: bool = typed_call!(
                FAMILY,
                0x00e7_1fe0,
                pack_geometry_buffer(e, Ptr::new(renderer), result, data, second, own, 0u8)
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

// Typed from 00e71fe0
/// `NiXenonRenderer::PackGeometryBuffer` (Xbox PDB): `thiscall(renderer, buffer data, data, object,
/// source, force)`; answers whether it packed. False without buffer data or data; true at once when not
/// forced and the buffer data is packed (`00e8eeb0`) with the data's +0xe at 0x4000 in its bits 0x7000.
/// The buffer's +4 gets the stream flags; the vertex count (data slot 0x90) and +8 are kept, and the
/// triangle data's (RTTI `011f4a88` in its chain) triangle count (slot 0xa8), index count and list
/// (`00a7b850`), or the strip data's (`IsKindOf` `011f4a70`) strip count, lengths and index count
/// (`00a75f10`). The streams (the source's +0x20, else 1) are reset unless packed with that count, the
/// dirty bits (+0xe, low 12 bits, with 3 when the object's +0x18 isn't the entry point `00e8bf90`, which
/// then replaces it; all when forced) decide which are repacked (the source's slot 0xac, else
/// `00e8c1e0` for a single stream) and whether the index buffer is remade (`00e8f200`, the old one
/// removed). The data's dirty bits are cleared.
pub fn pack_geometry_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    data: u32,
    object: u32,
    source: u32,
    force: u8,
) -> bool {
    let (b, d) = (buffer, data);
    if b == 0 || d == 0 {
        return false;
    }
    let packed: bool = typed_call!(
        crate::shader::FAMILY,
        0x00e8_eeb0,
        state::has_streams(e, Ptr::new(buffer))
    );
    if force == 0 && packed && e.mem.u16(d.wrapping_add(0xe)) & 0x7000 == 0x4000 {
        return true;
    }
    let has_28 = e.mem.u32(d.wrapping_add(0x28)) != 0;
    let has_24 = e.mem.u32(d.wrapping_add(0x24)) != 0;
    let top_bits = has_24 && e.mem.u16(d.wrapping_add(0xc)) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(d.wrapping_add(0xc))) & 1;
    let limit = e.global::<u32>(STREAM_LIMIT);
    if low > limit {
        low = limit;
    }
    let mut stream_flags = 0u32;
    if has_28 {
        stream_flags = 0x40_0000;
    }
    if has_24 {
        stream_flags |= 0x80_0000;
    }
    if top_bits {
        stream_flags |= 0x20_0000;
    }
    e.mem.set_u32(b.wrapping_add(4), (low << 24) | stream_flags);
    let vertices = u32::from(e.vcall(d, 0x90, &args![]).u16());
    let second = u32::from(e.mem.u16(d.wrapping_add(8)));
    let mut triangles = 0u32;
    let mut base_count = 0u32;
    let mut index_count = 0u32;
    let mut list = 0u32;
    let mut lengths = 0u32;
    let mut strips = 0u32;
    let mut rtti = e.vcall(d, 8, &args![]).u32();
    let mut tri_list = false;
    while rtti != 0 {
        if rtti == TRI_LIST_DATA_RTTI {
            tri_list = true;
            break;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    if tri_list {
        triangles = u32::from(e.vcall(d, 0xa8, &args![]).u16());
        base_count = u32::from(e.mem.u16(d.wrapping_add(0x40)));
        list = e.call(GET_TRI_LIST, &args![data]).u32();
        lengths = 0;
        strips = 1;
        index_count = base_count.wrapping_mul(3);
    } else if e.call(IS_KIND_OF, &args![STRIP_DATA_RTTI, data]).u8() != 0 {
        triangles = u32::from(e.vcall(d, 0xa8, &args![]).u16());
        base_count = u32::from(e.mem.u16(d.wrapping_add(0x40)));
        list = e.call(GET_STRIP_LISTS, &args![data]).u32();
        lengths = e.mem.u32(d.wrapping_add(0x48));
        strips = u32::from(e.mem.u16(d.wrapping_add(0x44)));
        index_count = base_count.wrapping_add(strips.wrapping_mul(2));
    }
    let count = if source != 0 {
        e.mem.u32(source.wrapping_add(0x20))
    } else {
        1
    };
    let mut dirty = u32::from(e.mem.u16(d.wrapping_add(0xe)) & 0xfff);
    if object != 0 {
        let previous = e.mem.u32(object.wrapping_add(0x18));
        let group = e.mem.u32(this.addr().wrapping_add(0x8bc));
        if previous != e.call(GET_ENTRY_POINT, &args![group]).u32() {
            dirty |= 3;
            let group = e.mem.u32(this.addr().wrapping_add(0x8bc));
            let entry = e.call(GET_ENTRY_POINT, &args![group]).u32();
            e.mem.set_u32(object.wrapping_add(0x18), entry);
        }
    }
    if force != 0 {
        dirty = 0xfff;
    }
    if packed {
        if count != e.mem.u32(b.wrapping_add(0x20)) {
            e.call(SET_STREAM_COUNT, &args![buffer, count]);
            dirty = 0xfff;
        }
    } else {
        let owner = e.mem.u32(b.wrapping_add(8));
        let mut index = 0u32;
        if count > 0 {
            loop {
                e.vcall(owner, 0x1c, &args![buffer, index]);
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
        e.call(SET_STREAM_COUNT, &args![buffer, count]);
        e.mem.set_u32(b.wrapping_add(0x38), 0);
        dirty = 0xfff;
    }
    e.mem.set_u32(b.wrapping_add(0x18), vertices);
    e.mem.set_u32(b.wrapping_add(0x1c), second);
    e.mem.set_u32(b.wrapping_add(0x4c), lengths);
    e.mem.set_u32(b.wrapping_add(0x44), base_count);
    e.mem.set_u32(b.wrapping_add(0x40), triangles);
    e.mem.set_u32(b.wrapping_add(0x50), list);
    e.mem.set_u32(b.wrapping_add(0x48), strips);
    if dirty & 0xffff_ffef != 0 {
        let mut index = 0u32;
        while index < count {
            let stream = if index < e.mem.u32(b.wrapping_add(0x20)) {
                let slots = e.mem.u32(b.wrapping_add(0x28));
                e.mem.u32(slots.wrapping_add(index.wrapping_mul(4)))
            } else {
                0
            };
            let done = source != 0
                && e.vcall(
                    source,
                    0xac,
                    &args![data, object, 0u32, dirty, stream, index, 0u32, 0u32, 0u32],
                )
                .u32()
                    != 0;
            if !done {
                if count != 1 {
                    return false;
                }
                let group = e.mem.u32(this.addr().wrapping_add(0x8bc));
                let vb = e
                    .call(
                        PACK_UNTRANSFORMED_VB,
                        &args![group, data, object, dirty, stream, 0u32],
                    )
                    .u32();
                if vb == 0 {
                    return false;
                }
            }
            index += 1;
        }
    }
    if list != 0 && dirty & 0x30 != 0 {
        let dynamic = if e.mem.u8(b.wrapping_add(0x14)) != 0 {
            0x10u32
        } else {
            0
        };
        let old = e.mem.u32(b.wrapping_add(0x34));
        let not_ex = u32::from(e.mem.u8(DIRECT3D_IS_EX) == 0);
        let (index_buffer, offset) = e.with_stack(4, |e, slot| {
            let s = slot.addr();
            let word = e.mem.u32(b.wrapping_add(0x30));
            e.mem.set_u32(s, word);
            let manager = e.mem.u32(this.addr().wrapping_add(0x8c0));
            let made = e
                .call(
                    INDEX_BUFFER,
                    &args![
                        manager,
                        list,
                        index_count,
                        index_count,
                        old,
                        s,
                        not_ex,
                        dynamic
                    ],
                )
                .u32();
            (made, e.mem.u32(s))
        });
        if index_buffer == 0 {
            return false;
        }
        if old != index_buffer {
            e.call(REMOVE_IB, &args![buffer]);
            e.mem.set_u32(b.wrapping_add(0x34), index_buffer);
            e.mem.set_u32(b.wrapping_add(0x2c), index_count);
            e.mem.set_u32(b.wrapping_add(0x30), offset);
        }
    }
    let flags = e.mem.u16(d.wrapping_add(0xe));
    e.mem.set_u16(d.wrapping_add(0xe), flags & 0xf000);
    true
}
