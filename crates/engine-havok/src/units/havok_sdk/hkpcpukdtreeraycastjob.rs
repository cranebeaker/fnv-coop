//! `Havok SDK/hkpcpukdtreeraycastjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code computes in `f32`; floats the code moves through `FLD` / `FSTP` go through
//! `x87_quiet_bits`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuworldraycastjob::into_transform_frame;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};

/// The memory router (`00c85750`): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Unnamed in the engine map (`ECX` = the destination, one stack word: the source; copies a ray
/// cast output block and returns the destination).
const COPY_HIT_OUTPUT: u32 = 0x009c_1aa0;
/// Unnamed in the engine map (`ECX` = an array of four 0x50-byte output blocks; sets 1.0 at
/// `+0x10`, `0xFFFFFFFF` at `+0x14` and `+0x20` and 0 at `+0x40` in each).
const OUTPUT_ARRAY_INIT: u32 = 0x00d9_8390;
/// Unnamed in the engine map (`ECX` = an array of four output blocks; does what `00d98390` does).
const OUTPUT_ARRAY_RESET: u32 = 0x00d9_8300;
/// The vtable `00e4dde0` gives the collector.
const COLLECTOR_VTABLE: u32 = 0x010e_c614;
/// The vtable the deleting destructor leaves in the object.
const DESTROYED_VTABLE: u32 = 0x010d_4050;

// Translated from 00e4dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCpuBroadphaseBundleCastCollector::getHitFractions` (Xbox PDB): `ECX` = `this`, one
/// stack word: four floats `out`. When `[this + 0xC]` is not null, it copies the fractions
/// (`+0x10`) of the four hit entries (0x60 bytes each at `[[this + 0xC] + 0xC4]`) there, each
/// through `FLD` / `FSTP`.
pub fn hk_kd_tree_cpu_broadphase_bundle_cast_collector_get_hit_fractions(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        return;
    }
    for k in 0..4 {
        let context = e.mem.u32(this.addr() + 0xc);
        let hits = e.mem.u32(context + 0xc4);
        let fraction = x87_quiet_bits(e.mem.u32(hits + 0x60 * k + 0x10));
        e.mem.set_u32(out.addr() + 4 * k, fraction);
    }
}

// Translated from 00e4dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it stores a vtable and zeroes fields, the shape of a constructor of
/// the collector class; not confirmed): `ECX` = `this`. Stores the vtable `010ec614` and 0 in
/// `+0xC`, `+0x70`, `+0x74` and `+0x80` to `+0x8C`; returns `this`.
pub fn fn_00e4dde0(e: &mut Engine, this: Ptr) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u32(this_addr, COLLECTOR_VTABLE);
    for offset in [0xc, 0x70, 0x74, 0x80, 0x84, 0x88, 0x8c] {
        e.mem.set_u32(this_addr + offset, 0);
    }
    this
}

// Translated from 00e4e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCpuBroadphaseBundleCastCollector::_scalar_deleting_destructor_` (Xbox PDB): stores
/// the vtable `010d4050` in the object and, when bit 0 of `flags` is set, gives the 0x90-byte
/// block back to the memory router's allocator (slot 8). Returns `this`.
pub fn deleting_destructor_00e4e3c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), DESTROYED_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, 0x90u32]);
    }
    this
}

// Translated from 00e4de10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCpuBroadphaseBundleCastCollector::addMidPhaseHit` (Xbox PDB): `ECX` = `this`, two
/// stack words: the `hit` entry (`[hit]` the object, `[hit + 8]` the transform) and a word passed
/// on. Does nothing for a null object. When `[this + 4]` is set it asks the filter
/// `[this + 4] + 0x14` (slot 4: a byte cell, `[this + 8]`, the entry) and stops on a 0 answer.
/// It then puts the eight points at `[this + 8] + 0x00, 0x10, 0x30, 0x40, 0x60, 0x70, 0x90, 0xA0`
/// into the frame of the transform (`into_transform_frame`) and stores them transposed in the six
/// vectors `this + 0x10` to `+0x60`: x of the points 0, 2, 4, 6 at `+0x10`, y at `+0x20`, z at
/// `+0x30`, and x, y, z of the points 1, 3, 5, 7 at `+0x40`, `+0x50`, `+0x60`. Point 3 stays in a
/// 16-byte frame slot the object is given. With four output blocks initialised by `00d98390`
/// and `00d98300` it runs slot `0x28` of the object (the point 3 slot, `this + 0x10`, the output
/// blocks, the word). Then for each block `k` with a fraction (`+0x10`) below 1.0, where either
/// the flag word `this + 0x80 + 4k` is 0 or the hit entry `k` (0x60 bytes at
/// `[[this + 0xC] + 0xC4]`) has a greater fraction, the block is copied into the hit entry
/// (`00009c1aa0`), the entry pointer goes to its `+0x50`, the normal rotated back by the
/// transform rows goes to its `+0`, its fraction becomes 0 when the byte at `[this + 0xC] + 0xC0`
/// is set (else it is passed through `FLD` / `FSTP`), and the flag word is set to 1. The words of
/// the output blocks the exe's frame leaves unwritten are zero here.
pub fn hkp_kd_tree_cpu_broadphase_bundle_cast_collector_add_mid_phase_hit(
    e: &mut Engine,
    this: Ptr,
    hit: Ptr,
    word: u32,
) {
    let (this, hit) = (this.addr(), hit.addr());
    let object = e.mem.u32(hit);
    if object == 0 {
        return;
    }
    e.with_stack(0x200, |e, frame| {
        let b = frame.addr();
        let matrix = e.mem.u32(hit + 8);
        let filter = e.mem.u32(this + 4);
        if filter != 0 {
            let bundle = e.mem.u32(this + 8);
            let verdict = e
                .vcall(filter + 0x14, 4, &args![b + 0x3f, bundle, hit])
                .u32();
            if e.mem.u8(verdict) == 0 {
                return;
            }
        }
        let bundle = e.mem.u32(this + 8);
        let points: [Vec4; 8] = [0x00, 0x10, 0x30, 0x40, 0x60, 0x70, 0x90, 0xa0]
            .map(|offset| into_transform_frame(e, matrix, load_vec(e, bundle + offset)));
        for (half, indices) in [[0, 2, 4, 6], [1, 3, 5, 7]].iter().enumerate() {
            for lane in 0..3 {
                let vector: Vec4 = std::array::from_fn(|i| points[indices[i]][lane]);
                store_vec(
                    e,
                    this + 0x10 + 0x30 * half as u32 + 0x10 * lane as u32,
                    vector,
                );
            }
        }
        store_vec(e, b + 0x90, points[3]);
        e.call(OUTPUT_ARRAY_INIT, &args![b + 0xa0]);
        e.call(OUTPUT_ARRAY_RESET, &args![b + 0xa0]);
        e.vcall(object, 0x28, &args![b + 0x90, this + 0x10, b + 0xa0, word]);
        for k in 0..4u32 {
            let block = b + 0xa0 + 0x50 * k;
            let offset = 0x60 * k;
            let flag = this + 0x80 + 4 * k;
            let fraction = e.mem.f32(block + 0x10);
            let below_one = fraction < 1.0;
            if !below_one {
                continue;
            }
            if e.mem.u32(flag) != 0 {
                let context = e.mem.u32(this + 0xc);
                let hits = e.mem.u32(context + 0xc4);
                let closer = e.mem.f32(hits + offset + 0x10) > fraction;
                if !closer {
                    continue;
                }
            }
            let context = e.mem.u32(this + 0xc);
            let slot = e.mem.u32(context + 0xc4).wrapping_add(offset);
            e.call(COPY_HIT_OUTPUT, &args![slot, block]);
            let tail = e.mem.u32(block + 0x50);
            e.mem.set_u32(slot + 0x50, tail);
            let context = e.mem.u32(this + 0xc);
            let slot = e.mem.u32(context + 0xc4).wrapping_add(offset);
            e.mem.set_u32(slot + 0x50, hit);
            let normal = load_vec(e, block);
            let rows = [
                load_vec(e, matrix),
                load_vec(e, matrix + 0x10),
                load_vec(e, matrix + 0x20),
            ];
            let rotated: Vec4 = std::array::from_fn(|lane| {
                (normal[0] * rows[0][lane] + normal[1] * rows[1][lane]) + normal[2] * rows[2][lane]
            });
            let context = e.mem.u32(this + 0xc);
            let slot = e.mem.u32(context + 0xc4).wrapping_add(offset);
            store_vec(e, slot, rotated);
            let context = e.mem.u32(this + 0xc);
            let slot = e.mem.u32(context + 0xc4).wrapping_add(offset);
            let kept = if e.mem.u8(context + 0xc0) != 0 {
                0
            } else {
                x87_quiet_bits(e.mem.u32(slot + 0x10))
            };
            e.mem.set_u32(slot + 0x10, x87_quiet_bits(kept));
            e.mem.set_u32(flag, 1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4dd90,
            hk_kd_tree_cpu_broadphase_bundle_cast_collector_get_hit_fractions(Ptr, Ptr)
        ),
        entry!(0x00e4dde0, fn_00e4dde0(Ptr) -> Ptr),
        entry!(
            0x00e4de10,
            hkp_kd_tree_cpu_broadphase_bundle_cast_collector_add_mid_phase_hit(Ptr, Ptr, u32)
        ),
        entry!(0x00e4e3c0, deleting_destructor_00e4e3c0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use crate::units::havok_sdk::hkpcpuworldraycastjob::test_support::{
        get_floats, object_with_slots, set_floats,
    };

    const ALLOCATOR: u32 = 0x0900_0000;
    const FILTER_SLOT: u32 = 0x0900_1004;
    const CAST_SLOT: u32 = 0x0900_1028;

    #[test]
    fn hit_fractions_are_copied_from_the_four_hit_entries() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out, 0xdead_beef);
        e.call(0x00e4_dd90, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0xdead_beef);
        let context = e.mem.alloc(0x100);
        let hits = e.mem.alloc(0x180);
        e.mem.set_u32(context + 0xc4, hits);
        e.mem.set_u32(this + 0xc, context);
        for (k, bits) in [0x3f00_0000u32, 0x3e80_0000, 0x7fa0_0000, 0x0000_0001]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(hits + 0x60 * k as u32 + 0x10, bits);
        }
        e.call(0x00e4_dd90, &args![this, out]);
        // Each one went through FLD / FSTP: the signalling NaN comes back quiet.
        let got: Vec<u32> = (0..4).map(|k| e.mem.u32(out + 4 * k)).collect();
        assert_eq!(got, vec![0x3f00_0000, 0x3e80_0000, 0x7fe0_0000, 1]);
    }

    #[test]
    fn the_constructor_stores_the_vtable_and_zeroes_its_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x90);
        for offset in (0..0x90).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e4_dde0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), COLLECTOR_VTABLE);
        for offset in [0xc, 0x70, 0x74, 0x80, 0x84, 0x88, 0x8c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x78), 0xffff_ffff);
    }

    #[test]
    fn the_deleting_destructor_stores_the_vtable_and_frees_only_with_bit_zero() {
        let mut e = engine();
        let log = new_log();
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00e4_e3c0, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), DESTROYED_VTABLE);
        assert!(calls(&log, ALLOCATOR + 8).is_empty());
        assert_eq!(e.call(0x00e4_e3c0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&log, ALLOCATOR + 8),
            vec![vec![allocator, this, 0x90]]
        );
    }

    /// The collector, its context with the four hit entries and the flag byte `zero_flag`, a
    /// bundle of eight points `(i + 1, 2i + 1, 3i + 1)`, and a hit entry whose transform scales
    /// by 2, 3 and 4 and moves by 1 (so point `i` becomes `(2i, 6i, 12i)`). Slot `0x28` of the
    /// entry object logs its words and the vectors it was given and fills the four output
    /// blocks.
    struct Bundle {
        e: Engine,
        log: Log,
        this: u32,
        hit: u32,
        hits: u32,
    }

    fn bundle_rig(zero_flag: u8, filter_verdict: Option<u8>) -> Bundle {
        let mut e = engine();
        let log = new_log();
        let matrix = e.mem.alloc(0x40);
        set_floats(&mut e, matrix, [2.0, 0.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x10, [0.0, 3.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x20, [0.0, 0.0, 4.0, 0.0]);
        set_floats(&mut e, matrix + 0x30, [1.0, 1.0, 1.0, 0.0]);
        let sink = log.clone();
        e.register_double(CAST_SLOT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            words.extend((0..24).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            sink.borrow_mut().push((CAST_SLOT, words));
            // Fractions 0.5, 1.0, 0.25 and 0.1 with the normal (1, 2, 3) and a marker word.
            for (k, fraction) in [0.5f32, 1.0, 0.25, 0.1].into_iter().enumerate() {
                let block = a[3] + 0x50 * k as u32;
                set_floats(e, block, [1.0, 2.0, 3.0, 0.0]);
                e.mem.set_f32(block + 0x10, fraction);
                e.mem.set_u32(block + 0x14, 0x100 + k as u32);
            }
            Ret::default()
        });
        log_double(&mut e, &log, OUTPUT_ARRAY_INIT, 0);
        log_double(&mut e, &log, OUTPUT_ARRAY_RESET, 0);
        // The copy of an output block: its first 0x50 bytes.
        let sink = log.clone();
        e.register_double(COPY_HIT_OUTPUT, move |e, a| {
            sink.borrow_mut().push((COPY_HIT_OUTPUT, a.to_vec()));
            for word in 0..0x14 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = object_with_slots(&mut e, &[(0x28, CAST_SLOT)]);
        let hit = e.mem.alloc(0x10);
        e.mem.set_u32(hit, object);
        e.mem.set_u32(hit + 8, matrix);
        let bundle = e.mem.alloc(0xb0);
        for (i, offset) in [0x00, 0x10, 0x30, 0x40, 0x60, 0x70, 0x90, 0xa0]
            .into_iter()
            .enumerate()
        {
            let i = i as f32;
            set_floats(
                &mut e,
                bundle + offset,
                [i + 1.0, 2.0 * i + 1.0, 3.0 * i + 1.0, 0.0],
            );
        }
        let context = e.mem.alloc(0x100);
        let hits = e.mem.alloc(0x180);
        e.mem.set_u32(context + 0xc4, hits);
        e.mem.set_u8(context + 0xc0, zero_flag);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 8, bundle);
        e.mem.set_u32(this + 0xc, context);
        if let Some(verdict) = filter_verdict {
            let answer = e.mem.alloc(4);
            e.mem.set_u8(answer, verdict);
            log_double(&mut e, &log, FILTER_SLOT, answer);
            let filter_vtable = e.mem.alloc(0x10);
            e.mem.set_u32(filter_vtable + 4, FILTER_SLOT);
            let filter = e.mem.alloc(0x40);
            e.mem.set_u32(filter + 0x14, filter_vtable);
            e.mem.set_u32(this + 4, filter);
        }
        Bundle {
            e,
            log,
            this,
            hit,
            hits,
        }
    }

    #[test]
    fn a_null_object_does_nothing() {
        let mut b = bundle_rig(0, Some(1));
        b.e.mem.set_u32(b.hit, 0);
        b.e.call(0x00e4_de10, &args![b.this, b.hit, 7u32]);
        assert!(b.log.borrow().is_empty());
    }

    #[test]
    fn a_filter_that_refuses_stops_before_the_bundle_is_transformed() {
        let mut b = bundle_rig(0, Some(0));
        b.e.call(0x00e4_de10, &args![b.this, b.hit, 7u32]);
        let filter_calls = calls(&b.log, FILTER_SLOT);
        assert_eq!(filter_calls.len(), 1);
        assert_eq!(filter_calls[0][2], b.e.mem.u32(b.this + 8));
        assert_eq!(filter_calls[0][3], b.hit);
        assert_eq!(b.log.borrow().len(), 1);
        assert_eq!(b.e.mem.u32(b.this + 0x10), 0);
    }

    #[test]
    fn the_bundle_is_transposed_and_the_closer_blocks_replace_the_hit_entries() {
        let mut b = bundle_rig(0, Some(1));
        // Entry 2 already has a closer hit (0.2 < 0.25) and entry 3 a further one (0.9).
        b.e.mem.set_u32(b.this + 0x88, 1);
        b.e.mem.set_u32(b.this + 0x8c, 1);
        b.e.mem.set_f32(b.hits + 0x60 * 2 + 0x10, 0.2);
        b.e.mem.set_f32(b.hits + 0x60 * 3 + 0x10, 0.9);
        b.e.call(0x00e4_de10, &args![b.this, b.hit, 7u32]);
        let casts = calls(&b.log, CAST_SLOT);
        assert_eq!(casts.len(), 1);
        let cast = &casts[0];
        assert_eq!(cast[2], b.this + 0x10);
        assert_eq!(cast[4], 7);
        // The fourth point (index 3) is handed over: (6, 18, 36, 0).
        let floats: Vec<f32> = cast[5..].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(floats[..4], [6.0, 18.0, 36.0, 0.0]);
        // x, y, z of the points 0, 2, 4, 6 and then of 1, 3, 5, 7.
        assert_eq!(floats[4..8], [0.0, 4.0, 8.0, 12.0]);
        assert_eq!(floats[8..12], [0.0, 12.0, 24.0, 36.0]);
        assert_eq!(floats[12..16], [0.0, 24.0, 48.0, 72.0]);
        assert_eq!(floats[16..20], [2.0, 6.0, 10.0, 14.0]);
        assert_eq!(floats[20..24], [6.0, 18.0, 30.0, 42.0]);
        assert_eq!(floats[24..28], [12.0, 36.0, 60.0, 84.0]);
        let order: Vec<u32> = b.log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order[..4],
            [
                FILTER_SLOT,
                OUTPUT_ARRAY_INIT,
                OUTPUT_ARRAY_RESET,
                CAST_SLOT
            ]
        );
        // Entry 0 and entry 3 were replaced (copy, entry pointer, rotated normal, flag),
        // entry 1 (fraction 1.0) and entry 2 (not closer) were not.
        let copies = calls(&b.log, COPY_HIT_OUTPUT);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][0], b.hits);
        assert_eq!(copies[1][0], b.hits + 0x60 * 3);
        let e = &b.e;
        assert_eq!(get_floats(e, b.hits), [2.0, 6.0, 12.0, 0.0]);
        assert_eq!(e.mem.f32(b.hits + 0x10), 0.5);
        assert_eq!(e.mem.u32(b.hits + 0x14), 0x100);
        assert_eq!(e.mem.u32(b.hits + 0x50), b.hit);
        assert_eq!(e.mem.f32(b.hits + 0x60 * 3 + 0x10), 0.1);
        assert_eq!(e.mem.u32(b.hits + 0x60 * 3 + 0x50), b.hit);
        assert_eq!(e.mem.u32(b.hits + 0x60 + 0x50), 0);
        assert_eq!(e.mem.f32(b.hits + 0x60 * 2 + 0x10), 0.2);
        assert_eq!(e.mem.u32(b.hits + 0x60 * 2 + 0x50), 0);
        let flags: Vec<u32> = (0..4).map(|k| e.mem.u32(b.this + 0x80 + 4 * k)).collect();
        assert_eq!(flags, vec![1, 0, 1, 1]);
    }

    #[test]
    fn the_zero_flag_sets_the_stored_fractions_to_zero() {
        let mut b = bundle_rig(1, None);
        b.e.call(0x00e4_de10, &args![b.this, b.hit, 7u32]);
        assert_eq!(b.e.mem.f32(b.hits + 0x10), 0.0);
        assert_eq!(b.e.mem.f32(b.hits + 0x60 * 3 + 0x10), 0.0);
        assert_eq!(b.e.mem.u32(b.hits + 0x50), b.hit);
        // Without a filter the first call is the output block setup.
        assert!(calls(&b.log, FILTER_SLOT).is_empty());
    }
}
