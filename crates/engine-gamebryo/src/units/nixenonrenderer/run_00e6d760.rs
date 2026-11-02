//! `NiXenonRenderer/run_00e6d760` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e8eeb0` (`thiscall` on the buffer data; in `nigeometrybufferdata.cpp`): a `bool` in
/// `AL`.
const BUFFER_DATA_00E8EEB0: u32 = 0x00e8_eeb0;
/// `00e8ef10` (`thiscall`, `RET 4`; in `nigeometrybufferdata.cpp`): given the stream count.
const BUFFER_DATA_SET_COUNT_00E8EF10: u32 = 0x00e8_ef10;
/// `NiGeometryBufferData::RemoveIB` (`00e8eee0`, `thiscall`).
const BUFFER_DATA_REMOVE_IB: u32 = 0x00e8_eee0;
/// `00e8c820` (`thiscall`, six words) on the object at `this + 0x8bc`.
const VERTEX_PACK_00E8C820: u32 = 0x00e8_c820;
/// `00e8f200` (`thiscall`, seven words) on the object at `this + 0x8c0`.
const INDEX_PACK_00E8F200: u32 = 0x00e8_f200;
/// The word `00e6d780` clamps the low bit of the geometry's flags to.
const LIMIT_011BF558: u32 = 0x011b_f558;
/// The byte whose zero test is passed to `00e8f200`.
const FLAG_0126F0D0: u32 = 0x0126_f0d0;

// Translated from 00e6d780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the renderer-side object with the packers at +0x8bc and +0x8c0;
/// `RET 0x18`, six words: buffer data, geometry data, a word passed through, a source
/// description, an optional object with a vtable, and a byte flag). Returns `false` at once
/// for a null buffer data or geometry data; with the flag clear, `00e8eeb0` true and the
/// geometry's word +0xe masked with 0x7000 equal to 0x4000, `true` at once.
///
/// Otherwise it writes the buffer data's word +4 (the geometry's +0xc low bit, clamped to
/// the word at `011bf558`, in the top byte; 0x400000 when +0x28 is set, 0x800000 when +0x24
/// is set, 0x200000 when +0x24 is set and +0xc has a bit of 0xf000), takes the stream count
/// from the object (+0x20) or 1, and the update mask (+0xe & 0xfff, or 0xfff with the
/// flag). When `00e8eeb0` was false, the buffer data's object at +8 gets its virtual slot
/// +0x1c once per stream (buffer data, stream), then `00e8ef10(count)` and +0x38 cleared;
/// when it was true and the count differs from +0x20, `00e8ef10(count)`. In both of those
/// cases the mask becomes 0xfff. The source description's words +0x14, +0x18 and `u16`s
/// +0x1c, +0x1e, +0x22 go to +0x50, +0x4c, +0x18/+0x1c, +0x40/+0x44 and +0x48 (+0x22 being
/// replaced by 1 when zero). When the mask has a bit other than 0x10, each stream is packed
/// by the object's slot +0xac (nine words) or, failing that and with one stream only,
/// `00e8c820`; a failure returns `false`. When the mask has 0x30, the index buffer is
/// requested from `00e8f200`; a null result returns `false`, a new one replaces the old
/// (`RemoveIB`, then +0x34, +0x2c and +0x30 set).
pub fn fn_00e6d780(
    e: &mut Engine,
    this: Ptr,
    buffer_data: Ptr,
    geometry: Ptr,
    passed: u32,
    source: Ptr,
    packer: Ptr,
    force: u8,
) -> bool {
    let b = buffer_data.addr();
    let g = geometry.addr();
    if b == 0 || g == 0 {
        return false;
    }
    let has_buffer = e.call(BUFFER_DATA_00E8EEB0, &args![b]).bool();
    if force == 0 && has_buffer && e.mem.u16(g + 0xe) & 0x7000 == 0x4000 {
        return true;
    }
    let has_28 = e.mem.u32(g + 0x28) != 0;
    let has_24 = e.mem.u32(g + 0x24) != 0;
    let has_f000 = has_24 && e.mem.u16(g + 0xc) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(g + 0xc)) & 1;
    let limit: u32 = e.global(LIMIT_011BF558);
    if low > limit {
        low = limit;
    }
    let mut bits = if has_28 { 0x40_0000 } else { 0 };
    if has_24 {
        bits |= 0x80_0000;
    }
    if has_f000 {
        bits |= 0x20_0000;
    }
    e.mem.set_u32(b + 4, (low << 24) | bits);

    let s = source.addr();
    let word_14 = e.mem.u32(s + 0x14);
    let half_1c = u32::from(e.mem.u16(s + 0x1c));
    let half_1e = u32::from(e.mem.u16(s + 0x1e));
    let half_22 = e.mem.u16(s + 0x22);
    let word_18 = e.mem.u32(s + 0x18);
    let (index_count, half_22) = if half_22 == 0 {
        (half_1e.wrapping_mul(3), 1u32)
    } else {
        (
            half_1e.wrapping_add(u32::from(half_22) * 2),
            u32::from(half_22),
        )
    };
    let p = packer.addr();
    let count = if p != 0 { e.mem.u32(p + 0x20) } else { 1 };
    let mut mask: u32 = if force != 0 {
        0xfff
    } else {
        u32::from(e.mem.u16(g + 0xe) & 0xfff)
    };
    if has_buffer {
        if count != e.mem.u32(b + 0x20) {
            e.call(BUFFER_DATA_SET_COUNT_00E8EF10, &args![b, count]);
            mask = 0xfff;
        }
    } else {
        let streams = e.mem.u32(b + 8);
        let mut stream = 0u32;
        while stream < count {
            e.vcall(streams, 0x1c, &args![b, stream]);
            stream += 1;
        }
        e.call(BUFFER_DATA_SET_COUNT_00E8EF10, &args![b, count]);
        e.mem.set_u32(b + 0x38, 0);
        mask = 0xfff;
    }
    e.mem.set_u32(b + 0x18, half_1c);
    e.mem.set_u32(b + 0x1c, half_1c);
    e.mem.set_u32(b + 0x4c, word_18);
    e.mem.set_u32(b + 0x50, word_14);
    e.mem.set_u32(b + 0x40, half_1e);
    e.mem.set_u32(b + 0x44, half_1e);
    e.mem.set_u32(b + 0x48, half_22);
    if mask & 0xffff_ffef != 0 {
        let mut stream = 0u32;
        while stream < count {
            let target = if stream < e.mem.u32(b + 0x20) {
                let list = e.mem.u32(b + 0x28);
                e.mem.u32(list.wrapping_add(stream * 4))
            } else {
                0
            };
            let packed = p != 0
                && e.vcall(
                    p,
                    0xac,
                    &args![g, passed, s, mask, target, stream, 0u32, 0u32, 0u32],
                )
                .u32()
                    != 0;
            if !packed {
                if count != 1 {
                    return false;
                }
                let vertex_packer = e.mem.u32(this.addr().wrapping_add(0x8bc));
                if e.call(
                    VERTEX_PACK_00E8C820,
                    &args![vertex_packer, g, passed, s, mask, target, 0u32],
                )
                .u32()
                    == 0
                {
                    return false;
                }
            }
            stream += 1;
        }
    }
    if mask & 0x30 != 0 {
        let flag_10: u32 = if e.mem.u8(b + 0x14) != 0 { 0x10 } else { 0 };
        let old = e.mem.u32(b + 0x34);
        let unset = u32::from(e.global::<u8>(FLAG_0126F0D0) == 0);
        let index_packer = e.mem.u32(this.addr().wrapping_add(0x8c0));
        let start = e.mem.u32(b + 0x30);
        let (result, slot) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), start);
            let result = e
                .call(
                    INDEX_PACK_00E8F200,
                    &args![
                        index_packer,
                        word_14,
                        index_count,
                        index_count,
                        old,
                        slot,
                        unset,
                        flag_10
                    ],
                )
                .u32();
            (result, e.mem.u32(slot.addr()))
        });
        if result == 0 {
            return false;
        }
        if old != result {
            e.call(BUFFER_DATA_REMOVE_IB, &args![b]);
            e.mem.set_u32(b + 0x34, result);
            e.mem.set_u32(b + 0x2c, index_count);
            e.mem.set_u32(b + 0x30, slot);
        }
    }
    true
}

/// The object table at `0126f108` that `fn_00e6d760` indexes (one word per entry).
const OBJECT_TABLE_0126F108: u32 = 0x0126_f108;

// Translated from 00e6d760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, which it does not read, one word;
/// `00e812f0` calls it with a shader declaration's word +0x2c): 0 for index 0, otherwise the
/// answer of the virtual slot +0xb4 (no stack words) of the object at entry `index` of the
/// table at `0126f108`.
pub fn fn_00e6d760(e: &mut Engine, _unused_this: Ptr, index: u32) -> u32 {
    if index == 0 {
        return index;
    }
    let object = e
        .mem
        .u32(OBJECT_TABLE_0126F108.wrapping_add(index.wrapping_mul(4)));
    e.vcall(object, 0xb4, &args![]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e6_d780,
            fn_00e6d780(Ptr, Ptr, Ptr, u32, Ptr, Ptr, u8) -> bool
        ),
        entry!(0x00e6_d760, fn_00e6d760(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    #[test]
    fn fn_00e6d760_asks_the_table_entry() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(0x0e50_0000, |_, a| (a[0] + 1).into_ret());
        let vtable = e.mem.alloc(0xb8);
        e.mem.set_u32(vtable + 0xb4, 0x0e50_0000);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(OBJECT_TABLE_0126F108 + 3 * 4, object);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e6_d760, &args![0x0e10_0000u32, 3u32]).u32(),
            object + 1
        );
        assert_eq!(e.call(0x00e6_d760, &args![0x0e10_0000u32, 0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let slot_calls: Vec<_> = log.iter().filter(|c| c.0 == 0x0e50_0000).collect();
        assert_eq!(slot_calls, vec![&(0x0e50_0000, vec![object])]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    struct Rig {
        e: Engine,
        this: u32,
        b: u32,
        g: u32,
        s: u32,
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(LIMIT_011BF558 & !0xfff, 0x1000);
        e.map(FLAG_0126F0D0 & !0xfff, 0x1000);
        e.set_global(LIMIT_011BF558, 1u32);
        e.register(BUFFER_DATA_00E8EEB0, |_, _| Ret::default());
        e.register(BUFFER_DATA_SET_COUNT_00E8EF10, |_, _| Ret::default());
        e.register(BUFFER_DATA_REMOVE_IB, |_, _| Ret::default());
        e.register(VERTEX_PACK_00E8C820, |_, _| 1u32.into_ret());
        e.register(INDEX_PACK_00E8F200, |e, a| {
            e.mem.set_u32(a[5], 0x77);
            0x9900u32.into_ret()
        });
        let this = e.mem.alloc(0x900);
        e.mem.set_u32(this + 0x8bc, 0x1100);
        e.mem.set_u32(this + 0x8c0, 0x2200);
        let b = e.mem.alloc(0x60);
        let streams_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(streams_vtable + 0x1c, 0x00ab_0100);
        e.register(0x00ab_0100, |_, _| Ret::default());
        let streams = e.mem.alloc(8);
        e.mem.set_u32(streams, streams_vtable);
        e.mem.set_u32(b + 8, streams);
        let g = e.mem.alloc(0x30);
        e.mem.set_u16(g + 0xc, 0x1001);
        e.mem.set_u16(g + 0xe, 0x0030);
        e.mem.set_u32(g + 0x24, 1);
        let s = e.mem.alloc(0x30);
        e.mem.set_u32(s + 0x14, 0x14);
        e.mem.set_u32(s + 0x18, 0x18);
        e.mem.set_u16(s + 0x1c, 0x1c);
        e.mem.set_u16(s + 0x1e, 0x1e);
        e.call_log = Some(vec![]);
        Rig { e, this, b, g, s }
    }

    #[test]
    fn null_inputs_fail_and_an_unchanged_static_buffer_succeeds() {
        let Rig {
            mut e,
            this,
            b,
            g,
            s,
        } = rig();
        assert!(!e
            .call(0x00e6_d780, &args![this, 0u32, g, 0u32, s, 0u32, 0u32])
            .bool());
        assert!(!e
            .call(0x00e6_d780, &args![this, b, 0u32, 0u32, s, 0u32, 0u32])
            .bool());
        e.register(BUFFER_DATA_00E8EEB0, |_, _| 1u32.into_ret());
        e.mem.set_u16(g + 0xe, 0x4000);
        assert!(e
            .call(0x00e6_d780, &args![this, b, g, 0u32, s, 0u32, 0u32])
            .bool());
        assert_eq!(e.mem.u32(b + 4), 0);
    }

    #[test]
    fn a_new_buffer_resets_its_streams_packs_and_requests_the_index_buffer() {
        let Rig {
            mut e,
            this,
            b,
            g,
            s,
        } = rig();
        let streams = e.mem.u32(b + 8);
        assert!(e
            .call(0x00e6_d780, &args![this, b, g, 0x33u32, s, 0u32, 0u32])
            .bool());
        assert_eq!(e.mem.u32(b + 4), 0x0180_0000 | 0x20_0000);
        assert_eq!(calls_to(&e, 0x00ab_0100), vec![vec![streams, b, 0]]);
        assert_eq!(
            calls_to(&e, BUFFER_DATA_SET_COUNT_00E8EF10),
            vec![vec![b, 1]]
        );
        assert_eq!(e.mem.u32(b + 0x18), 0x1c);
        assert_eq!(e.mem.u32(b + 0x44), 0x1e);
        assert_eq!(e.mem.u32(b + 0x48), 1);
        assert_eq!(e.mem.u32(b + 0x4c), 0x18);
        assert_eq!(e.mem.u32(b + 0x50), 0x14);
        assert_eq!(
            calls_to(&e, VERTEX_PACK_00E8C820),
            vec![vec![0x1100, g, 0x33, s, 0xfff, 0, 0]]
        );
        let index = calls_to(&e, INDEX_PACK_00E8F200);
        assert_eq!(index.len(), 1);
        assert_eq!(index[0][..5], [0x2200, 0x14, 0x5a, 0x5a, 0]);
        assert_eq!(index[0][6..], [1, 0]);
        assert_eq!(calls_to(&e, BUFFER_DATA_REMOVE_IB), vec![vec![b]]);
        assert_eq!(e.mem.u32(b + 0x34), 0x9900);
        assert_eq!(e.mem.u32(b + 0x2c), 0x5a);
        assert_eq!(e.mem.u32(b + 0x30), 0x77);
    }

    #[test]
    fn the_packer_object_is_tried_first_and_failures_return_false() {
        let Rig {
            mut e,
            this,
            b,
            g,
            s,
        } = rig();
        let vtable = e.mem.alloc(0xb0);
        e.mem.set_u32(vtable + 0xac, 0x00ab_0200);
        e.register(0x00ab_0200, |_, _| Ret::default());
        let packer = e.mem.alloc(0x24);
        e.mem.set_u32(packer, vtable);
        e.mem.set_u32(packer + 0x20, 2);
        e.register(BUFFER_DATA_00E8EEB0, |_, _| 1u32.into_ret());
        e.mem.set_u32(b + 0x20, 2);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0xaaaa);
        e.mem.set_u32(b + 0x28, list);
        // Two streams, the packer fails: no fallback for more than one stream.
        assert!(!e
            .call(0x00e6_d780, &args![this, b, g, 5u32, s, packer, 0u32])
            .bool());
        assert_eq!(
            calls_to(&e, 0x00ab_0200),
            vec![vec![packer, g, 5, s, 0x30, 0xaaaa, 0, 0, 0, 0]]
        );
        assert!(calls_to(&e, BUFFER_DATA_SET_COUNT_00E8EF10).is_empty());
        e.register(0x00ab_0200, |_, _| 1u32.into_ret());
        e.register(INDEX_PACK_00E8F200, |_, _| Ret::default());
        assert!(!e
            .call(0x00e6_d780, &args![this, b, g, 5u32, s, packer, 0u32])
            .bool());
        // An empty mask returns true without packing or the index buffer.
        e.mem.set_u16(g + 0xe, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e6_d780, &args![this, b, g, 5u32, s, packer, 0u32])
            .bool());
        assert!(calls_to(&e, 0x00ab_0200).is_empty());
        assert!(calls_to(&e, INDEX_PACK_00E8F200).is_empty());
    }
}
