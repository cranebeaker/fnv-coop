//! `NiXenonRenderer/nixenonvbmanager.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vertex buffer manager's packing of untransformed geometry into a vertex buffer. The
//! manager's fields are read at their offsets: the pending copy of a locked buffer (source
//! +0x40, destination +0x48, size +0x4c) and its lock (`CRITICAL_SECTION` at +0x80).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_memcpy` (`00ec44d0`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00e8c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the manager, which it does not read; five
/// words): copies `count` elements of `size` bytes from consecutive source elements into
/// destination elements `stride` bytes apart (`memcpy` each).
pub fn fn_00e8c090(
    e: &mut Engine,
    _unused_this: Ptr,
    destination: u32,
    source: u32,
    stride: u32,
    size: u32,
    count: u32,
) {
    let (mut to, mut from) = (destination, source);
    for _ in 0..count {
        e.call(MEMCPY, &args![to, from, size]);
        to = to.wrapping_add(stride);
        from = from.wrapping_add(size);
    }
}

/// What `PackUntransformedVB` calls: the manager's vertex layout `00e8c120` (unnamed;
/// `thiscall`, nine words: the geometry data, 0 and the addresses of the format, stride and
/// the position, unused, normal, colour and texture offsets it fills), the buffer data's
/// format set `00e8f010` (unnamed; `thiscall`, one word), the manager's new vertex buffer
/// `00e8bfa0` (`thiscall`: the buffer data and 0; a `bool`) and lock `00e8bff0` (`thiscall`,
/// four words of a buffer chip: its +8, +0xc, +0x14, +0x10; answers the write pointer), the
/// position and normal packer `00687d40` (unnamed, unit
/// `fallout shared/tempeffects/bstempeffectgeometrydecal.cpp`; `thiscall` on the second word:
/// the geometry data, the position and normal destinations, 0, 0, the stride),
/// `NiGeometryData::GetTextureSet` (`00a66960`, `thiscall`, the set index), the imported
/// `LeaveCriticalSection` (import slot `00fdf100`) and the colour scale `255.0` (`double`,
/// `0101e568`). The vertex buffer's `GetDesc` (slot 0x34) and `Unlock` (slot 0x30) are named by
/// `IDirect3DVertexBuffer9`'s slot order (the arguments match).
const VERTEX_LAYOUT_00E8C120: u32 = 0x00e8_c120;
const SET_FORMAT_00E8F010: u32 = 0x00e8_f010;
const NEW_VERTEX_BUFFER_00E8BFA0: u32 = 0x00e8_bfa0;
const LOCK_00E8BFF0: u32 = 0x00e8_bff0;
const PACK_POSITIONS_00687D40: u32 = 0x0068_7d40;
const GET_TEXTURE_SET: u32 = 0x00a6_6960;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const COLOR_SCALE: u32 = 0x0101_e568;
const VERTEX_BUFFER_GET_DESC: u32 = 0x34;
const VERTEX_BUFFER_UNLOCK: u32 = 0x30;
/// The highest texture set count (`011bf558`).
const TEXTURE_SET_LIMIT_011BF558: u32 = 0x011b_f558;
/// `D3DFMT_VERTEXDATA` (100), the format a reusable buffer must have.
const VERTEX_DATA_FORMAT: u32 = 100;

/// One colour channel as the exe packs it: the float times 255.0 (`double`), stored as a
/// float, loaded again and stored as an integer (`FISTP`, nearest).
fn color_channel(e: &mut Engine, at: u32) -> u32 {
    let scale = f64::from_bits(e.mem.u64(COLOR_SCALE));
    let scaled = x87_store32(x87_mul(x87_load32(e.mem.u32(at)), scale));
    crate::x87::fistp(x87_load32(scaled)) as u32
}

// Translated from 00e8c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonVertexBufferManager::PackUntransformedVB` (engine map name; `thiscall`, five
/// words: the geometry data, a skin object or null, the parts to pack (1 positions, 2
/// normals, 4 colours, 8 texture coordinates), a buffer chip to reuse, and a write pointer
/// when the caller already locked).
///
/// Without active vertices (the data's slot 0x90, a `u16`) the chip is answered at once.
/// Otherwise the layout (`00e8c120`) gives the format, stride and offsets; the buffer data
/// (+0x34) gets the format (`00e8f010`) and, when it has streams (+0x20), the stride as its
/// first one. With a write pointer the chip is used as it is and nothing is unlocked.
/// Otherwise the chip is reused when its vertex buffer (+8) describes itself (`GetDesc`) as
/// vertex data of the same format and at least count × stride bytes; if not, a new buffer is
/// made (`00e8bfa0`, 0 answered on failure) and its first chip (or null) used, with every
/// part packed (0xfff); the chip is locked (`00e8bff0`).
///
/// Then, from the write pointer: with a skin object, positions (and normals when the data has
/// them) go through `00687d40`; without one, positions (+0x20) and normals (+0x24) are copied
/// as 12-byte elements (`fn_00e8c090`). Colours (+0x28, four floats each) are stored as
/// `a << 24 | r << 16 | g << 8 | b` (each × 255 rounded) and the texture sets (the data's
/// `u16` +0xc bit 0, at most `011bf558`) as 8-byte pairs from `GetTextureSet`, each at its
/// offset in every vertex. Last, when the manager locked: its pending copy (+0x48 from +0x40,
/// +0x4c bytes) is made and cleared, its lock (+0x80) left and the vertex buffer unlocked
/// (0 answered on failure). Answers the chip.
pub fn ni_xenon_vertex_buffer_manager_pack_untransformed_vb(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    skin: u32,
    parts: u32,
    chip: u32,
    write: u32,
) -> u32 {
    let manager = this.addr();
    let data = data.addr();
    let count = u32::from(e.vcall(data, 0x90, &args![]).u16());
    if count == 0 {
        return chip;
    }
    let normals = e.mem.u32(data.wrapping_add(0x24));
    let vertices = e.mem.u32(data.wrapping_add(0x20));
    let mut texture_sets = u32::from(e.mem.u16(data.wrapping_add(0xc))) & 1;
    let limit = e.mem.u32(TEXTURE_SET_LIMIT_011BF558);
    let colors = e.mem.u32(data.wrapping_add(0x28));
    if texture_sets > limit {
        texture_sets = limit;
    }
    // The layout's outputs: format, stride, position, unused, normal, colour and texture
    // offsets (the exe's locals), then the vertex buffer description.
    e.with_stack(0x1c + 0x18, |e, frame| {
        let frame = frame.addr();
        let [format_at, stride_at, position_at, unused_at, normal_at, color_at, texture_at] =
            [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18].map(|offset| frame + offset);
        let desc = frame + 0x1c;
        e.mem.set_u32(format_at, 0);
        e.mem.set_u32(stride_at, 0);
        e.call(
            VERTEX_LAYOUT_00E8C120,
            &args![
                manager,
                data,
                0u32,
                format_at,
                stride_at,
                position_at,
                unused_at,
                normal_at,
                color_at,
                texture_at
            ],
        );
        let buffer_data = e.mem.u32(data.wrapping_add(0x34));
        let format = e.mem.u32(format_at);
        e.call(SET_FORMAT_00E8F010, &args![buffer_data, format]);
        if e.mem.u32(buffer_data.wrapping_add(0x20)) > 0 {
            let strides = e.mem.u32(buffer_data.wrapping_add(0x24));
            let stride = e.mem.u32(stride_at);
            e.mem.set_u32(strides, stride);
        }
        let mut parts = parts;
        let mut locked_here = true;
        let mut used = chip;
        let mut out = write;
        if write != 0 {
            locked_here = false;
        } else {
            let mut reuse = false;
            if chip != 0 {
                let buffer = e.mem.u32(chip.wrapping_add(8));
                if buffer != 0 {
                    e.vcall(buffer, VERTEX_BUFFER_GET_DESC, &args![desc]);
                    reuse = e.mem.u32(desc) == VERTEX_DATA_FORMAT
                        && e.mem.u32(desc + 0x14) == e.mem.u32(format_at)
                        && e.mem.u32(desc + 0x10)
                            >= (count & 0xffff).wrapping_mul(e.mem.u32(stride_at));
                }
            }
            if !reuse {
                if !e
                    .call(
                        NEW_VERTEX_BUFFER_00E8BFA0,
                        &args![manager, buffer_data, 0u32],
                    )
                    .bool()
                {
                    return 0;
                }
                used = if e.mem.u32(buffer_data.wrapping_add(0x20)) > 0 {
                    e.mem.u32(e.mem.u32(buffer_data.wrapping_add(0x28)))
                } else {
                    0
                };
                parts = 0xfff;
            }
            let words = args![
                manager,
                e.mem.u32(used.wrapping_add(8)),
                e.mem.u32(used.wrapping_add(0xc)),
                e.mem.u32(used.wrapping_add(0x14)),
                e.mem.u32(used.wrapping_add(0x10))
            ];
            out = e.call(LOCK_00E8BFF0, &words).u32();
        }
        let stride = |e: &Engine| e.mem.u32(stride_at);
        if skin != 0 {
            if vertices != 0 && parts & 1 != 0 {
                let positions = e.mem.u32(position_at).wrapping_add(out);
                let normal_out = if normals != 0 {
                    e.mem.u32(normal_at).wrapping_add(out)
                } else {
                    0
                };
                let words = args![skin, data, positions, normal_out, 0u32, 0u32, stride(e)];
                e.call(PACK_POSITIONS_00687D40, &words);
            }
        } else {
            if vertices != 0 && parts & 1 != 0 {
                let to = e.mem.u32(position_at).wrapping_add(out);
                fn_00e8c090(e, this, to, vertices, stride(e), 0xc, count);
            }
            if normals != 0 && parts & 2 != 0 {
                let to = e.mem.u32(normal_at).wrapping_add(out);
                fn_00e8c090(e, this, to, normals, stride(e), 0xc, count);
            }
        }
        if colors != 0 && parts & 4 != 0 {
            let mut to = e.mem.u32(color_at).wrapping_add(out);
            let mut from = colors;
            for _ in 0..count {
                let alpha = color_channel(e, from + 0xc);
                let red = color_channel(e, from);
                let green = color_channel(e, from + 4);
                let blue = color_channel(e, from + 8);
                let packed = (((((alpha << 8) | red) << 8) | green) << 8) | blue;
                from = from.wrapping_add(0x10);
                e.mem.set_u32(to, packed);
                to = to.wrapping_add(stride(e));
            }
        }
        if parts & 8 != 0 && texture_sets > 0 {
            let mut base = out.wrapping_add(e.mem.u32(texture_at));
            for set in 0..texture_sets {
                let mut from = e.call(GET_TEXTURE_SET, &args![data, set]).u32();
                let mut to = base;
                for _ in 0..count {
                    let low = e.mem.u32(from);
                    e.mem.set_u32(to, low);
                    let high = e.mem.u32(from.wrapping_add(4));
                    e.mem.set_u32(to.wrapping_add(4), high);
                    to = to.wrapping_add(stride(e));
                    from = from.wrapping_add(8);
                }
                base = base.wrapping_add(8);
            }
        }
        if locked_here {
            let size = e.mem.u32(manager.wrapping_add(0x4c));
            let source = e.mem.u32(manager.wrapping_add(0x40));
            let buffer = e.mem.u32(used.wrapping_add(8));
            let destination = e.mem.u32(manager.wrapping_add(0x48));
            e.call(MEMCPY, &args![destination, source, size]);
            e.mem.set_u32(manager.wrapping_add(0x48), 0);
            e.mem.set_u32(manager.wrapping_add(0x4c), 0);
            e.call(
                IMPORT_LEAVE_CRITICAL_SECTION,
                &args![manager.wrapping_add(0x80)],
            );
            if e.vcall(buffer, VERTEX_BUFFER_UNLOCK, &args![]).i32() < 0 {
                return 0;
            }
        }
        used
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_c090, fn_00e8c090(Ptr, u32, u32, u32, u32, u32)),
        entry!(
            0x00e8_c1e0,
            ni_xenon_vertex_buffer_manager_pack_untransformed_vb(Ptr, Ptr, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_SLOTS: u32 = 0x0e50_0000;
    const BUFFER_SLOTS: u32 = 0x0e51_0000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

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

    #[test]
    fn fn_00e8c090_copies_each_element() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let words = args![0u32, 0x1000u32, 0x2000u32, 0x20u32, 0xcu32, 3u32];
        e.call(0x00e8_c090, &words);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![
                vec![0x1000, 0x2000, 0xc],
                vec![0x1020, 0x200c, 0xc],
                vec![0x1040, 0x2018, 0xc]
            ]
        );
        e.call_log = Some(vec![]);
        let words = args![0u32, 0x1000u32, 0x2000u32, 0x20u32, 0xcu32, 0u32];
        e.call(0x00e8_c090, &words);
        assert!(calls(&e, MEMCPY).is_empty());
    }

    struct Rig {
        e: Engine,
        manager: u32,
        data: u32,
        chip: u32,
        buffer_data: u32,
    }

    /// Two vertices with positions, normals, colours and one texture set; the layout gives
    /// format 0x152, stride 0x24 and the offsets 0, 0xc, 0x18 and 0x1c.
    fn rig(count: u16) -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_e000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        e.mem.set_u64(COLOR_SCALE, 255.0f64.to_bits());
        e.mem.set_u32(TEXTURE_SET_LIMIT_011BF558, 4);
        e.register(MEMCPY, |e, a| {
            for k in 0..a[2] {
                let b = e.mem.u8(a[1] + k);
                e.mem.set_u8(a[0] + k, b);
            }
            Ret::default()
        });
        for callee in [
            SET_FORMAT_00E8F010,
            IMPORT_LEAVE_CRITICAL_SECTION,
            PACK_POSITIONS_00687D40,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(VERTEX_LAYOUT_00E8C120, |e, a| {
            for (at, value) in [
                (3, 0x152u32),
                (4, 0x24),
                (5, 0),
                (7, 0xc),
                (8, 0x18),
                (9, 0x1c),
            ] {
                e.mem.set_u32(a[at], value);
            }
            Ret::default()
        });
        let data = object(&mut e, DATA_SLOTS, 0x38);
        e.register_double(DATA_SLOTS + 0x90, move |_, _| u32::from(count).into_ret());
        e.mem.set_u16(data + 0xc, 1);
        let vertices = e.mem.alloc(0x18);
        let normals = e.mem.alloc(0x18);
        for k in 0..6u32 {
            e.mem.set_f32(vertices + 4 * k, k as f32);
            e.mem.set_f32(normals + 4 * k, 10.0 + k as f32);
        }
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(data + 0x24, normals);
        let colors = e.mem.alloc(0x20);
        for (k, value) in [1.0f32, 0.5, 0.0, 0.25, 0.0, 0.0, 1.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(colors + 4 * k as u32, *value);
        }
        e.mem.set_u32(data + 0x28, colors);
        let uvs = e.mem.alloc(0x10);
        for k in 0..4u32 {
            e.mem.set_u32(uvs + 4 * k, 0x7700 + k);
        }
        e.register_double(GET_TEXTURE_SET, move |_, _| uvs.into_ret());
        let buffer_data = e.mem.alloc(0x2c);
        let strides = e.mem.alloc(4);
        e.mem.set_u32(buffer_data + 0x20, 1);
        e.mem.set_u32(buffer_data + 0x24, strides);
        e.mem.set_u32(data + 0x34, buffer_data);
        let buffer = object(&mut e, BUFFER_SLOTS, 8);
        let chip = e.mem.alloc(0x18);
        e.mem.set_u32(chip + 8, buffer);
        let chips = e.mem.alloc(4);
        e.mem.set_u32(chips, chip);
        e.mem.set_u32(buffer_data + 0x28, chips);
        let manager = e.mem.alloc(0x98);
        e.call_log = Some(vec![]);
        Rig {
            e,
            manager,
            data,
            chip,
            buffer_data,
        }
    }

    fn pack(r: &mut Rig, skin: u32, parts: u32, chip: u32, write: u32) -> u32 {
        let words = args![r.manager, r.data, skin, parts, chip, write];
        r.e.call(0x00e8_c1e0, &words).u32()
    }

    #[test]
    fn pack_untransformed_vb_without_vertices_answers_the_chip() {
        let mut r = rig(0);
        assert_eq!(pack(&mut r, 0, 0xf, 0x1234, 0), 0x1234);
        assert!(calls(&r.e, VERTEX_LAYOUT_00E8C120).is_empty());
    }

    #[test]
    fn pack_untransformed_vb_into_a_given_pointer() {
        let mut r = rig(2);
        let out = r.e.mem.alloc(0x48);
        let chip = r.chip;
        assert_eq!(pack(&mut r, 0, 0xf, chip, out), chip);
        let e = &r.e;
        assert_eq!(
            calls(e, SET_FORMAT_00E8F010),
            vec![vec![r.buffer_data, 0x152]]
        );
        let strides = e.mem.u32(r.buffer_data + 0x24);
        assert_eq!(e.mem.u32(strides), 0x24);
        // Positions and normals copied per vertex.
        assert_eq!(e.mem.f32(out + 4), 1.0);
        assert_eq!(e.mem.f32(out + 0x24 + 8), 5.0);
        assert_eq!(e.mem.f32(out + 0xc), 10.0);
        assert_eq!(e.mem.f32(out + 0x24 + 0xc + 8), 15.0);
        // Colours: a 0.25 to 64, r 255, g 128 (127.5 to even), b 0; then a 255, r 0, g 0, b 255.
        assert_eq!(e.mem.u32(out + 0x18), 0x40ff_8000);
        assert_eq!(e.mem.u32(out + 0x24 + 0x18), 0xff00_00ff);
        // The texture set.
        assert_eq!(e.mem.u32(out + 0x1c), 0x7700);
        assert_eq!(e.mem.u32(out + 0x20), 0x7701);
        assert_eq!(e.mem.u32(out + 0x24 + 0x1c), 0x7702);
        assert!(calls(e, LOCK_00E8BFF0).is_empty());
        assert!(calls(e, IMPORT_LEAVE_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn pack_untransformed_vb_reuses_a_matching_buffer_and_unlocks() {
        let mut r = rig(2);
        let out = r.e.mem.alloc(0x48);
        r.e.register_double(LOCK_00E8BFF0, move |_, _| out.into_ret());
        r.e.register(BUFFER_SLOTS + VERTEX_BUFFER_GET_DESC, |e, a| {
            e.mem.set_u32(a[1], VERTEX_DATA_FORMAT);
            e.mem.set_u32(a[1] + 0x10, 0x48);
            e.mem.set_u32(a[1] + 0x14, 0x152);
            Ret::default()
        });
        let pending = r.e.mem.alloc(4);
        r.e.mem.set_u32(pending, 0xabcd);
        let target = r.e.mem.alloc(4);
        r.e.mem.set_u32(r.manager + 0x40, pending);
        r.e.mem.set_u32(r.manager + 0x48, target);
        r.e.mem.set_u32(r.manager + 0x4c, 4);
        let chip = r.chip;
        assert_eq!(pack(&mut r, 0, 1, chip, 0), chip);
        let e = &r.e;
        assert!(calls(e, NEW_VERTEX_BUFFER_00E8BFA0).is_empty());
        assert_eq!(calls(e, LOCK_00E8BFF0).len(), 1);
        // Only the positions (part 1).
        assert_eq!(e.mem.f32(out + 4), 1.0);
        assert_eq!(e.mem.u32(target), 0xabcd);
        assert_eq!(e.mem.u32(r.manager + 0x48), 0);
        assert_eq!(e.mem.u32(r.manager + 0x4c), 0);
        assert_eq!(
            calls(e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![r.manager + 0x80]]
        );
        assert_eq!(calls(e, BUFFER_SLOTS + VERTEX_BUFFER_UNLOCK).len(), 1);
    }

    #[test]
    fn pack_untransformed_vb_makes_a_new_buffer_when_the_chip_does_not_fit() {
        let mut r = rig(2);
        r.e.register(NEW_VERTEX_BUFFER_00E8BFA0, |_, _| 0u32.into_ret());
        let chip = r.chip;
        assert_eq!(pack(&mut r, 0, 1, chip, 0), 0);
        // A buffer that is made: its first chip, every part packed, through the skin packer.
        let mut r = rig(2);
        r.e.register(NEW_VERTEX_BUFFER_00E8BFA0, |_, _| 1u32.into_ret());
        let out = r.e.mem.alloc(0x48);
        r.e.register_double(LOCK_00E8BFF0, move |_, _| out.into_ret());
        let made = r.chip;
        assert_eq!(pack(&mut r, 0x0e70_0000, 1, 0, 0), made);
        let e = &r.e;
        assert_eq!(
            calls(e, PACK_POSITIONS_00687D40),
            vec![vec![0x0e70_0000, r.data, out, out + 0xc, 0, 0, 0x24]]
        );
        // All parts: colours and the texture set too.
        assert_eq!(e.mem.u32(out + 0x18), 0x40ff_8000);
        assert_eq!(e.mem.u32(out + 0x1c), 0x7700);
    }
}
