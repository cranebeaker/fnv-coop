//! `NiXenonRenderer/run_00e6d310` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e6d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSBatchRenderer::RenderPassImmediately_Skinned` calls it
/// for each skin partition (`thiscall`, `RET 0xc`: the partition, a second object, an
/// unread word). Draws the partition's strips through the device at `this + 0x288`
/// (its slot +0x148, six words): for each of the partition's `+0x48` strips, the primitive
/// type (5 when the second object's `u16` +0x22 is non-zero, else 4), the partition's
/// word +0x38, 0, the second object's `u16` +0x1c, the strip's first index and its length.
/// A strip's length is its 16-bit entry in the partition's array +0x4c minus 2, or the
/// partition's word +0x40 when there is no array; the first index advances by the length
/// plus 2.
pub fn fn_00e6d310(e: &mut Engine, this: Ptr, partition: u32, object: u32, _unused_3: u32) {
    let mut start = 0u32;
    let mut strip = 0u32;
    if e.mem.u32(partition + 0x48) == 0 {
        return;
    }
    loop {
        let lengths = e.mem.u32(partition + 0x4c);
        let length = if lengths != 0 {
            u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
        } else {
            e.mem.u32(partition + 0x40)
        };
        let primitive = if e.mem.u16(object + 0x22) != 0 {
            5u32
        } else {
            4
        };
        let vertex_count = u32::from(e.mem.u16(object + 0x1c));
        let device = e.mem.u32(this.addr() + 0x288);
        let word_38 = e.mem.u32(partition + 0x38);
        e.vcall(
            device,
            0x148,
            &args![primitive, word_38, 0u32, vertex_count, start, length],
        );
        strip += 1;
        start = start.wrapping_add(length).wrapping_add(2);
        if strip >= e.mem.u32(partition + 0x48) {
            break;
        }
    }
}

/// The strip data's methods (unit `NiParticle/bsstrippsysdata.cpp`, all `thiscall` on it):
/// the vertex count of a strip (`00c25db0`, one word, a 16-bit answer), `00c25dd0` (no
/// stack words, its answer unused), the strip's texture step (`00c25eb0`, one word, a
/// float in ST0) and, for a strip and a vertex (two words), the vertex's two position
/// pairs (`00c25e70`), its two normals (`00c25e90`) and its colour (`00c26350`, four floats:
/// red, green, blue, alpha).
const STRIP_VERTEX_COUNT: u32 = 0x00c2_5db0;
const STRIP_DATA_00C25DD0: u32 = 0x00c2_5dd0;
const STRIP_TEXTURE_STEP: u32 = 0x00c2_5eb0;
const STRIP_POSITIONS: u32 = 0x00c2_5e70;
const STRIP_NORMALS: u32 = 0x00c2_5e90;
const STRIP_COLOR: u32 = 0x00c2_6350;
/// The vertex writer `00e69b60` (`thiscall`, seven stack words: the write position, a
/// position, a normal, the packed colour, the texture coordinates `u` and `v` and the
/// float `w`; returns the next write position).
const WRITE_VERTEX: u32 = 0x00e6_9b60;
/// `255.0` (`double`, `0101e568`) and `-1.0` (`float`, `01012054`).
const COLOR_SCALE: u32 = 0x0101_e568;
const NO_STRIP_BYTE: u32 = 0x0101_2054;

/// One colour channel as the exe packs it: the float times 255.0 (`double`), stored as a
/// float, loaded again and stored as an integer (`FISTP`, nearest).
fn color_channel(e: &mut Engine, color: u32, offset: u32) -> u32 {
    let scale = f64::from_bits(e.mem.u64(COLOR_SCALE));
    let scaled = x87_mul(x87_load32(e.mem.u32(color + offset)), scale);
    let stored = x87_store32(scaled);
    crate::x87::fistp(x87_load32(stored)) as u32
}

// Translated from 00e6d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, five stack words; called by `00e70720`): builds the
/// vertices and 16-bit indices of a strip particle system's strips. `this` is the vertex
/// writer's `this` (`00e69b60`), `vertices` its first write position, `indices` where the
/// indices go and `strips` the strip data (`BSStripPSysData`: the strip count `u16` +0x50, a
/// byte per strip at +0x64 or none, 0x20-byte strip records at +0x7c with a vertex modulus
/// +0x10 and a first vertex `u16` +0x14).
///
/// For each strip with at least two vertices: after the first such strip two indices join
/// it to the last (the running index less one, and the running index; two more counted);
/// `00c25dd0` runs and the texture step is read twice (the first answer dropped). From the
/// strip's first vertex, for each of its vertices (the next one the vertex plus one modulo
/// the modulus, signed), the writer is called four times (the two positions, each with the
/// two normals as the exe pairs them, `v` 0 and 1) with the colour packed as
/// `a << 24 | r << 16 | g << 8 | b` (each channel × 255 rounded), `u` (from 0, plus the
/// step after each vertex, as a float) and `w` (the strip's byte as a float, or -1.0 without
/// the bytes); each vertex adds the indices running, running + 1 and the running index
/// moves on by 4. The strip ends with two indices (running − 3 and the strip's base, two
/// counted) and, for each vertex, base and base + 1 (base moving by 4). The vertex total
/// (four per vertex) goes to `vertex_count` and the vertex total plus the counted joins to
/// `index_count`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e6d3a0(
    e: &mut Engine,
    this: Ptr,
    vertices: u32,
    indices: u32,
    strips: u32,
    vertex_count: u32,
    index_count: u32,
) {
    let strip_bytes = e.mem.u32(strips + 0x64);
    let strip_total = e.mem.u16(strips + 0x50);
    let mut vertex_total = 0u32;
    let mut joins = 0u32;
    let mut running = 0u32;
    let mut started = false;
    let mut write = vertices;
    let mut out = indices;
    let mut put_index = |e: &mut Engine, value: u32| {
        e.mem.set_u16(out, value as u16);
        out = out.wrapping_add(2);
    };
    let mut strip = 0u32;
    while (strip as u16) < strip_total {
        let count = e.call(STRIP_VERTEX_COUNT, &args![strips, strip]).u16();
        if count >= 2 {
            if started {
                put_index(e, running.wrapping_sub(1));
                put_index(e, running);
                joins = joins.wrapping_add(2);
            } else {
                started = true;
            }
            e.call(STRIP_DATA_00C25DD0, &args![strips]);
            e.call(STRIP_TEXTURE_STEP, &args![strips, strip]);
            let mut u_bits = x87_store32(0.0);
            let step_bits = x87_store32(e.call(STRIP_TEXTURE_STEP, &args![strips, strip]).f64());
            let base = u32::from(running.wrapping_add(2) as u16);
            let first = e
                .mem
                .u32(strips + 0x7c)
                .wrapping_add(strip.wrapping_mul(0x20));
            let mut vertex = u32::from(e.mem.u16(first + 0x14));
            vertex_total = vertex_total.wrapping_add(u32::from(count) * 4);
            for _ in 0..count {
                let positions = e.call(STRIP_POSITIONS, &args![strips, strip, vertex]).u32();
                let normals = e.call(STRIP_NORMALS, &args![strips, strip, vertex]).u32();
                let color = e.call(STRIP_COLOR, &args![strips, strip, vertex]).u32();
                let alpha = color_channel(e, color, 0xc);
                let color = e.call(STRIP_COLOR, &args![strips, strip, vertex]).u32();
                let red = color_channel(e, color, 0);
                let color = e.call(STRIP_COLOR, &args![strips, strip, vertex]).u32();
                let green = color_channel(e, color, 4);
                let color = e.call(STRIP_COLOR, &args![strips, strip, vertex]).u32();
                let blue = color_channel(e, color, 8);
                let packed = (((((alpha << 8) | red) << 8) | green) << 8) | blue;
                let w = if strip_bytes != 0 {
                    f64::from(e.mem.u8(strip_bytes.wrapping_add(strip)))
                } else {
                    x87_load32(e.mem.u32(NO_STRIP_BYTE))
                };
                let w_bits = x87_quiet_bits(x87_store32(w));
                let u_push = x87_quiet_bits(u_bits);
                for (position, normal, v) in [
                    (positions, normals, 0u32),
                    (positions + 0xc, normals, 0x3f80_0000),
                    (positions + 0x18, normals + 0xc, 0),
                    (positions + 0x24, normals + 0xc, 0x3f80_0000),
                ] {
                    write = e
                        .call(
                            WRITE_VERTEX,
                            &args![this, write, position, normal, packed, u_push, v, w_bits],
                        )
                        .u32();
                }
                u_bits = x87_store32(x87_add(x87_load32(step_bits), x87_m32(u_bits)));
                put_index(e, running);
                put_index(e, running.wrapping_add(1));
                running = running.wrapping_add(4);
                // The record is found again through +0x7c for each vertex.
                let record = e
                    .mem
                    .u32(strips + 0x7c)
                    .wrapping_add(strip.wrapping_mul(0x20));
                let modulus = e.mem.u32(record + 0x10) as i32;
                let mut next = (vertex as i32 + 1) % modulus;
                if next < 0 {
                    next += modulus;
                }
                vertex = u32::from(next as u16);
            }
            joins = joins.wrapping_add(2);
            put_index(e, running.wrapping_sub(3));
            put_index(e, base);
            let mut base = base;
            for _ in 0..count {
                put_index(e, base);
                put_index(e, base.wrapping_add(1));
                base = base.wrapping_add(4);
            }
        }
        strip = strip.wrapping_add(1);
    }
    e.mem.set_u32(vertex_count, vertex_total);
    e.mem.set_u32(index_count, joins.wrapping_add(vertex_total));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_d310, fn_00e6d310(Ptr, u32, u32, u32)),
        entry!(0x00e6_d3a0, fn_00e6d3a0(Ptr, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRAW: u32 = 0x0000_5148;

    fn setup(lengths: Option<&[u16]>, strips: u32, type_word: u16) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(DRAW, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x14c);
        e.mem.set_u32(vtable + 0x148, DRAW);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let this = e.mem.alloc(0x28c);
        e.mem.set_u32(this + 0x288, device);
        let partition = e.mem.alloc(0x50);
        e.mem.set_u32(partition + 0x48, strips);
        e.mem.set_u32(partition + 0x38, 0x38);
        e.mem.set_u32(partition + 0x40, 9);
        if let Some(lengths) = lengths {
            let array = e.mem.alloc(2 * lengths.len() as u32);
            for (i, l) in lengths.iter().enumerate() {
                e.mem.set_u16(array + 2 * i as u32, *l);
            }
            e.mem.set_u32(partition + 0x4c, array);
        }
        let object = e.mem.alloc(0x24);
        e.mem.set_u16(object + 0x1c, 0x30);
        e.mem.set_u16(object + 0x22, type_word);
        e.call_log = Some(vec![]);
        (e, this, partition, object)
    }

    fn draws(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == DRAW)
            .map(|(_, w)| w[1..].to_vec())
            .collect()
    }

    #[test]
    fn draws_each_strip_from_the_length_array() {
        let (mut e, this, partition, object) = setup(Some(&[5, 7]), 2, 1);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert_eq!(
            draws(&mut e),
            vec![vec![5, 0x38, 0, 0x30, 0, 3], vec![5, 0x38, 0, 0x30, 5, 5]]
        );
    }

    /// Strip data with three strips of `counts` vertices (first vertices 1, 0, 0, moduli 3,
    /// 3, 2) and per-strip bytes 10, 20, 30 when `with_bytes`.
    fn strip_engine(with_bytes: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_u64(COLOR_SCALE, 255.0f64.to_bits());
        e.mem.set_u32(NO_STRIP_BYTE, (-1.0f32).to_bits());
        let strips = e.mem.alloc(0x80);
        e.mem.set_u16(strips + 0x50, 3);
        if with_bytes {
            let bytes = e.mem.alloc(4);
            e.mem.write(bytes, &[10, 20, 30]);
            e.mem.set_u32(strips + 0x64, bytes);
        }
        let records = e.mem.alloc(0x60);
        for (i, (first, modulus)) in [(1u16, 3u32), (0, 3), (0, 2)].iter().enumerate() {
            let record = records + 0x20 * i as u32;
            e.mem.set_u32(record + 0x10, *modulus);
            e.mem.set_u16(record + 0x14, *first);
        }
        e.mem.set_u32(strips + 0x7c, records);
        let color = e.mem.alloc(0x10);
        for (i, value) in [1.0f32, 0.5, 0.0, 0.25].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *value);
        }
        e.register(STRIP_VERTEX_COUNT, |_, a| {
            [2u32, 1, 2][a[1] as usize].into_ret()
        });
        e.register(STRIP_DATA_00C25DD0, |_, _| Ret::default());
        e.register(STRIP_TEXTURE_STEP, |_, _| 0.5f32.into_ret());
        e.register(STRIP_POSITIONS, |_, a| (0x1000 + 0x100 * a[2]).into_ret());
        e.register(STRIP_NORMALS, |_, a| (0x2000 + 0x100 * a[2]).into_ret());
        e.register_double(STRIP_COLOR, move |_, _| color.into_ret());
        e.register(WRITE_VERTEX, |_, a| (a[1] + 0x28).into_ret());
        e.call_log = Some(vec![]);
        (e, strips)
    }

    #[test]
    fn fn_00e6d3a0_builds_the_strip_vertices_and_indices() {
        let (mut e, strips) = strip_engine(true);
        let indices = e.mem.alloc(0x40);
        let counts = e.mem.alloc(8);
        e.call(
            0x00e6_d3a0,
            &args![
                0x0e10_0000u32,
                0x0e20_0000u32,
                indices,
                strips,
                counts,
                counts + 4
            ],
        );
        let written: Vec<u16> = (0..22).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(
            written,
            vec![
                0, 1, 4, 5, 5, 2, 2, 3, 6, 7, // strip 0
                7, 8, 8, 9, 12, 13, 13, 10, 10, 11, 14, 15, // strip 2, joined
            ]
        );
        assert_eq!(e.mem.u32(counts), 16);
        assert_eq!(e.mem.u32(counts + 4), 22);
        let log = e.call_log.take().unwrap();
        let writes: Vec<_> = log.iter().filter(|c| c.0 == WRITE_VERTEX).collect();
        assert_eq!(writes.len(), 16);
        let half = 0.5f32.to_bits();
        let one = 1.0f32.to_bits();
        let ten = 10.0f32.to_bits();
        let thirty = 30.0f32.to_bits();
        // Strip 0, vertex 1: the four corners chained from the first write position.
        assert_eq!(
            writes[0].1,
            vec![
                0x0e10_0000,
                0x0e20_0000,
                0x1100,
                0x2100,
                0x40ff_8000,
                0,
                0,
                ten
            ]
        );
        assert_eq!(
            writes[1].1,
            vec![
                0x0e10_0000,
                0x0e20_0028,
                0x110c,
                0x2100,
                0x40ff_8000,
                0,
                one,
                ten
            ]
        );
        assert_eq!(writes[2].1[2..4], [0x1118, 0x210c]);
        assert_eq!(writes[3].1[2..4], [0x1124, 0x210c]);
        // Vertex 2 of strip 0 has `u` 0.5; strip 2 starts again at `u` 0 with vertex 0.
        assert_eq!(writes[4].1[2], 0x1200);
        assert_eq!(writes[4].1[5], half);
        assert_eq!(writes[8].1[1], 0x0e20_0000 + 8 * 0x28);
        assert_eq!(writes[8].1[2], 0x1000);
        assert_eq!(writes[8].1[5], 0);
        assert_eq!(writes[8].1[7], thirty);
        assert_eq!(writes[12].1[2], 0x1100);
        // The strip with one vertex is skipped but counted for the byte and record index.
        assert!(!log.iter().any(|c| c.0 == STRIP_POSITIONS && c.1[1] == 1));
    }

    #[test]
    fn fn_00e6d3a0_uses_minus_one_without_strip_bytes() {
        let (mut e, strips) = strip_engine(false);
        let indices = e.mem.alloc(0x40);
        let counts = e.mem.alloc(8);
        e.call(
            0x00e6_d3a0,
            &args![
                0x0e10_0000u32,
                0x0e20_0000u32,
                indices,
                strips,
                counts,
                counts + 4
            ],
        );
        let log = e.call_log.take().unwrap();
        let minus_one = (-1.0f32).to_bits();
        assert!(log
            .iter()
            .filter(|c| c.0 == WRITE_VERTEX)
            .all(|c| c.1[7] == minus_one));
        // No strips: nothing is written, both counts 0.
        e.mem.set_u16(strips + 0x50, 0);
        e.mem.set_u32(counts, 9);
        e.mem.set_u32(counts + 4, 9);
        e.call(
            0x00e6_d3a0,
            &args![
                0x0e10_0000u32,
                0x0e20_0000u32,
                indices,
                strips,
                counts,
                counts + 4
            ],
        );
        assert_eq!(e.mem.u32(counts), 0);
        assert_eq!(e.mem.u32(counts + 4), 0);
    }

    #[test]
    fn draws_with_the_partition_length_and_type_4() {
        let (mut e, this, partition, object) = setup(None, 2, 0);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert_eq!(
            draws(&mut e),
            vec![vec![4, 0x38, 0, 0x30, 0, 9], vec![4, 0x38, 0, 0x30, 11, 9]]
        );
        let (mut e, this, partition, object) = setup(None, 0, 0);
        e.call(0x00e6_d310, &[this, partition, object, 0]);
        assert!(draws(&mut e).is_empty());
    }
}
