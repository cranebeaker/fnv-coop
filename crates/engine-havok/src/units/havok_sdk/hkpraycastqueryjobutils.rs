//! `Havok SDK/hkpraycastqueryjobutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The stack frames of `00e2abf0` and `00e2acf0` are modelled as one zeroed block each (the
//! exe leaves what it does not write uninitialised; the callees get pointers into the block
//! at the same offsets from the frame's low end `L` (the stack pointer after the prologue)).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, shufps, store_vec};

/// The kd-tree single ray cast (`00d98ca0`, cdecl `(out byte, tree, ray, ray + 0x10,
/// collector)`) and the kd-tree bundle cast (`00d99570`, cdecl `(out byte, tree, rows, rows,
/// collector, table entry)`).
const KD_TREE_CAST_RAY: u32 = 0x00d9_8ca0;
const KD_TREE_CAST_BUNDLE: u32 = 0x00d9_9570;
/// The bundle cast collector's constructor (`00e4dde0`, `this` = the collector) and its
/// `getHitFractions` (`00e4dd90`, `this` = the collector, one stack argument: the out array).
const BUNDLE_COLLECTOR_CONSTRUCT: u32 = 0x00e4_dde0;
const BUNDLE_COLLECTOR_GET_HIT_FRACTIONS: u32 = 0x00e4_dd90;
/// Vtables the single ray cast's stack object holds, and the ray hit collector's.
const CAST_VTABLES: [u32; 4] = [0x010d_baf8, 0x010d_bae4, 0x010d_bad8, 0x010d_bacc];
const RAY_HIT_COLLECTOR_VTABLE: u32 = 0x010d_bb04;
/// The global the collector set-up stores the address of `00e2b9b0` in (a function pointer).
const COLLECTOR_CALLBACK_SLOT: u32 = 0x0126_ec04;
const COLLECTOR_CALLBACK: u32 = 0x00e2_b9b0;
/// Table of words indexed by the number of rays in the bundle (`011b3830`).
const BUNDLE_TABLE: u32 = 0x011b_3830;

// Translated from 00e2aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeRayCastJobUtil::initializeBuildSetupJobs` (Xbox PDB), cdecl: `count` is first
/// limited to `total` (signed compare), then `total / count` (signed; `IDIV` faults when
/// `count` is 0, which panics here) is the share of each job. The first job (`0x70` bytes at
/// `jobs`) gets `p4` at `+0x10`, `first` at `+0x50`, `p3` at `+0x54`, 0 at `+0x58` and the share
/// at `+0x5c`; the others are copies of it with their own `+0x58` (the running start) and
/// `+0x5c` (the start of the next), and the last job's `+0x5c` is `total`. Parameters in
/// stack order: `first`, `total`, `p3`, `p4`, `count`, `jobs`.
pub fn hk_kd_tree_ray_cast_job_util_initialize_build_setup_jobs(
    e: &mut Engine,
    first: u32,
    total: i32,
    p3: u32,
    p4: u32,
    count: i32,
    jobs: Ptr,
) {
    let count = count.min(total);
    let share = total
        .checked_div(count)
        .unwrap_or_else(|| panic!("00e2aa70: IDIV of {total} by {count} faults"));
    let base = jobs.addr();
    e.mem.set_u32(base + 0x10, p4);
    e.mem.set_u32(base + 0x50, first);
    e.mem.set_u32(base + 0x54, p3);
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, share as u32);
    if count > 1 {
        let mut start = share;
        let mut job = base + 0x70;
        for _ in 1..count {
            let copy = e.mem.bytes(base, 0x70);
            e.mem.write(job, &copy);
            e.mem.set_u32(job + 0x58, start as u32);
            start = start.wrapping_add(share);
            e.mem.set_u32(job + 0x5c, start as u32);
            job += 0x70;
        }
    }
    let last = (count.wrapping_mul(0x70) as u32)
        .wrapping_add(base)
        .wrapping_sub(0x14);
    e.mem.set_u32(last, total as u32);
}

// Translated from 00e2ab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (unconfirmed meaning): copies the rows at `+0x00`, `+0x30`, `+0x60` of
/// `source` to the first output and the rows at `+0x10`, `+0x40`, `+0x70` to the second, then
/// transposes each output's three rows together with a fourth row (`source + 0x90` for the first,
/// `source + 0xA0` for the second) into three columns, in place. Cdecl: `(source, first, second)`.
pub fn fn_00e2ab00(e: &mut Engine, source: Ptr, first: Ptr, second: Ptr) {
    let (src, a, b) = (source.addr(), first.addr(), second.addr());
    for (out, from) in [(a, 0x00), (b, 0x10)] {
        for row in 0..3 {
            let v = load_vec(e, src + from + 0x30 * row);
            store_vec(e, out + 0x10 * row, v);
        }
    }
    let fourth_a = load_vec(e, src + 0x90);
    let fourth_b = load_vec(e, src + 0xa0);
    for (out, fourth) in [(a, fourth_a), (b, fourth_b)] {
        let row0 = load_vec(e, out);
        let row1 = load_vec(e, out + 0x10);
        let row2 = load_vec(e, out + 0x20);
        let low01 = shufps(row0, row1, 0x44);
        let high01 = shufps(row0, row1, 0xee);
        let low23 = shufps(row2, fourth, 0x44);
        let high23 = shufps(row2, fourth, 0xee);
        store_vec(e, out + 0x10, shufps(low01, low23, 0xdd));
        store_vec(e, out + 0x20, shufps(high01, high23, 0x88));
        store_vec(e, out, shufps(low01, low23, 0x88));
    }
}

// Translated from 00e2abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (unconfirmed): casts one ray through the kd-tree `tree` (nothing when it
/// is 0). The ray is the record `ray` (its `+0x10` is the end point, `+0x20` a flag byte,
/// `+0x24` and `+0x30` floats, `+0x3C` a flag byte telling whether hits are collected); the
/// stack object handed to the cast (`00d98ca0`) holds the four vtables, `limit` (`+0x20`), 1.0
/// (`+0x24`), the ray twice (`+0x2C`, `+0x30`), `ray + 0x30` twice (`+0x34`, `+0x38`),
/// `ray + 0x24` (`+0x60`), `limit + 0x10` when the flag byte at `ray + 0x20` is set and `limit`
/// is not 0 (`+0x64`) and the collector (`+0x70`). The collector is `collector` (when the hit
/// flag is set and `collector` is not 0: 1.0 at `+4`, the vtable `010dbb04`, `ray + 0x30` at
/// `+8` and `+0xC`, `ray + 0x34` at `+0x10`, zeros at `+0x14` and `+0x18`); when the hit flag is
/// set the global at `0126ec04` is set to `00e2b9b0`. Cdecl: `(tree, ray, limit, collector)`.
///
/// The cast's first argument is the stack byte at `L + 0xF`; the object is not initialised
/// beyond the words above, so the callee sees whatever else the frame holds (zeros here).
pub fn fn_00e2abf0(e: &mut Engine, tree: u32, ray: Ptr, limit: u32, collector: Ptr) {
    if tree == 0 {
        return;
    }
    let ray_at = ray.addr();
    e.with_stack(0x90, |e, frame| {
        let l = frame.addr();
        let ray_flag = e.mem.u8(ray_at + 0x20);
        let float_30 = e.mem.u32(ray_at + 0x30);
        e.mem.set_f32(l + 0x24, 1.0);
        e.mem.set_u32(l + 0x34, float_30);
        e.mem.set_u32(l + 0x38, float_30);
        let float_24 = e.mem.u32(ray_at + 0x24);
        e.mem.set_u32(l + 0x60, float_24);
        let masked = if ray_flag != 0 { limit } else { 0 };
        for (slot, vtable) in CAST_VTABLES.into_iter().enumerate() {
            e.mem.set_u32(l + 0x10 + 4 * slot as u32, vtable);
        }
        e.mem.set_u8(l + 0x28, 0);
        e.mem.set_u32(l + 0x20, limit);
        e.mem.set_u32(l + 0x30, ray_at);
        e.mem.set_u32(l + 0x2c, ray_at);
        if masked == 0 {
            e.mem.set_u32(l + 0x64, 0);
        } else {
            e.mem.set_u32(l + 0x64, masked.wrapping_add(0x10));
        }
        if e.mem.u8(ray_at + 0x3c) == 0 {
            e.mem.set_u32(l + 0x70, 0);
        } else if collector.addr() != 0 {
            let at = collector.addr();
            let float_34 = e.mem.u32(ray_at + 0x34);
            e.mem.set_f32(at + 4, 1.0);
            e.mem.set_u32(at, RAY_HIT_COLLECTOR_VTABLE);
            e.mem.set_u32(at + 8, float_30);
            e.mem.set_u32(at + 0xc, float_30);
            e.mem.set_u32(at + 0x10, float_34);
            e.mem.set_u32(at + 0x14, 0);
            e.mem.set_u32(at + 0x18, 0);
            e.mem.set_u32(l + 0x70, at);
            e.set_global(COLLECTOR_CALLBACK_SLOT, COLLECTOR_CALLBACK);
        } else {
            e.mem.set_u32(l + 0x70, 0);
            e.set_global(COLLECTOR_CALLBACK_SLOT, COLLECTOR_CALLBACK);
        }
        e.call(
            KD_TREE_CAST_RAY,
            &args![l + 0xf, tree, ray_at, ray_at + 0x10, l + 0x14],
        );
    });
}

/// One ray record of the bundle cast, copied between the caller's rays and the frame (the
/// same offsets on both sides): the vector at `+0x00`, the float at `+0x10` through the x87
/// stack (a signalling NaN is quieted), the words from `+0x14` to `+0x40`, and the word at
/// `+0x50`.
fn copy_ray_record(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let v = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, v);
    }
    let quiet = x87_quiet_bits(e.mem.u32(from + 0x10));
    e.mem.set_u32(to + 0x10, quiet);
    for offset in (0x14..=0x40).step_by(4) {
        let v = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, v);
    }
    let v = e.mem.u32(from + 0x50);
    e.mem.set_u32(to + 0x50, v);
}

// Translated from 00e2acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTreeRayCastJobUtil::castRayBundleKdTree` (Xbox PDB), cdecl `(tree, bundle, p3, hits)`:
/// nothing when `tree` is 0. The `bundle` holds the number of rays at `+0xC0` (at most four
/// are expected), a flag byte at `+0xC4`, one pointer per ray at `+0xC8` and one word per ray
/// at `+0xE0`; its rows at `+0x00` to `+0x70` and `+0x90`/`+0xA0` are what `00e2ab00` turns
/// into the two column sets. The frame gets: four ray records of `0x60` bytes from `L + 0x190`
/// (initialised with 1.0 at `+0x10`, -1 at `+0x14` and `+0x20`, 0 at `+0x40` and `+0x50`, then
/// the rays' own records copied in), four `0x30`-byte ray slots from `L + 0x20`, the bundle's
/// words saved at `L + 0xF0` while the hit fractions `hits` stand in their place in the
/// records' `+0x10`, the collector (constructed by `00e4dde0` at `L + 0x100`, with `p3` at
/// `+4`, the slots at `+0xC` and the bundle's fields at `+0x70`, `+0x84`, `+0x88`, `+0x8C`),
/// and the two column sets (`L + 0x310`, `L + 0x340`). `00d99570` casts the bundle (its last
/// argument is the word of `011b3830` for the ray count), the records and words are copied back
/// out and the collector's `getHitFractions` (`00e4dd90`) fills `hits`.
pub fn hk_tree_ray_cast_job_util_cast_ray_bundle_kd_tree(
    e: &mut Engine,
    tree: u32,
    bundle: Ptr,
    p3: u32,
    hits: Ptr,
) {
    if tree == 0 {
        return;
    }
    let esi = bundle.addr();
    e.with_stack(0x380, |e, frame| {
        let l = frame.addr();
        let count = e.mem.i32(esi + 0xc0);
        e.mem.set_f32(l + 0x1a0, 1.0);
        e.mem.set_f32(l + 0x200, 1.0);
        e.mem.set_f32(l + 0x260, 1.0);
        e.mem.set_f32(l + 0x2c0, 1.0);
        for (minus_one, zero) in [
            (0x1a4, 0x1d0),
            (0x1b0, 0x1e0),
            (0x204, 0x230),
            (0x210, 0x240),
            (0x264, 0x290),
            (0x270, 0x2a0),
            (0x2c4, 0x2f0),
            (0x2d0, 0x300),
        ] {
            e.mem.set_u32(l + minus_one, u32::MAX);
            e.mem.set_u32(l + zero, 0);
        }
        // The rays' records into the frame.
        if count > 0 {
            for ray in 0..count as u32 {
                let record = e.mem.u32(esi + 0xc8 + 4 * ray);
                copy_ray_record(e, record, l + 0x190 + 0x60 * ray);
            }
        }
        let flag = e.mem.u8(esi + 0xc4);
        for slot in [0x40, 0x70, 0xa0, 0xd0] {
            e.mem.set_u8(l + slot, 0);
            e.mem.set_u32(l + slot + 4, 0);
        }
        e.mem.set_u8(l + 0xe0, flag);
        e.mem.set_u32(l + 0xe4, l + 0x190);
        if count > 0 {
            let mut ray = 0u32;
            loop {
                let slot = 0x30 * ray;
                let flag_byte = e.mem.u8(esi + 0x20 + slot);
                e.mem.set_u8(l + 0x40 + slot, flag_byte);
                let word = e.mem.u32(esi + 0x24 + slot);
                e.mem.set_u32(l + 0x44 + slot, word);
                let low = load_vec(e, esi + slot);
                store_vec(e, l + 0x20 + slot, low);
                let high = load_vec(e, esi + 0x10 + slot);
                let fraction = x87_quiet_bits(e.mem.u32(hits.addr() + 4 * ray));
                store_vec(e, l + 0x30 + slot, high);
                e.mem.set_u32(l + 0x190 + 0x60 * ray + 0x10, fraction);
                let saved = e.mem.u32(esi + 0xe0 + 4 * ray);
                e.mem.set_u32(l + 0xf0 + 4 * ray, saved);
                ray += 1;
                if ray as i32 >= e.mem.i32(esi + 0xc0) {
                    break;
                }
            }
        }
        e.call(BUNDLE_COLLECTOR_CONSTRUCT, &args![l + 0x100]);
        e.mem.set_u32(l + 0x10c, l + 0x20);
        let word_24 = e.mem.u32(esi + 0x24);
        e.mem.set_u32(l + 0x170, word_24);
        let word_e4 = e.mem.u32(esi + 0xe4);
        e.mem.set_u32(l + 0x184, word_e4);
        e.mem.set_u32(l + 0x104, p3);
        let word_e8 = e.mem.u32(esi + 0xe8);
        e.mem.set_u32(l + 0x188, word_e8);
        let word_e0 = e.mem.u32(esi + 0xe0);
        e.mem.set_u32(l + 0x180, word_e0);
        let word_ec = e.mem.u32(esi + 0xec);
        e.mem.set_u32(l + 0x108, esi);
        e.mem.set_u32(l + 0x18c, word_ec);
        fn_00e2ab00(e, bundle, Ptr::new(l + 0x310), Ptr::new(l + 0x340));
        let entry = e
            .mem
            .u32(BUNDLE_TABLE.wrapping_add((e.mem.u32(esi + 0xc0)).wrapping_mul(4)));
        e.call(
            KD_TREE_CAST_BUNDLE,
            &args![l + 0x13, tree, l + 0x310, l + 0x340, l + 0x100, entry],
        );
        // The rays' records and words back out of the frame.
        let mut ray = 0u32;
        while (ray as i32) < e.mem.i32(esi + 0xc0) {
            let record = e.mem.u32(esi + 0xc8 + 4 * ray);
            copy_ray_record(e, l + 0x190 + 0x60 * ray, record);
            let saved = e.mem.u32(l + 0xf0 + 4 * ray);
            e.mem.set_u32(esi + 0xe0 + 4 * ray, saved);
            ray += 1;
        }
        e.call(BUNDLE_COLLECTOR_GET_HIT_FRACTIONS, &args![l + 0x100, hits]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e2_aa70,
            hk_kd_tree_ray_cast_job_util_initialize_build_setup_jobs(u32, i32, u32, u32, i32, Ptr)
        ),
        entry!(0x00e2_ab00, fn_00e2ab00(Ptr, Ptr, Ptr)),
        entry!(0x00e2_abf0, fn_00e2abf0(u32, Ptr, u32, Ptr)),
        entry!(
            0x00e2_acf0,
            hk_tree_ray_cast_job_util_cast_ray_bundle_kd_tree(u32, Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a cast double saw: its arguments and the bytes of the frame at the time.
    type Snapshots = Rc<RefCell<Vec<(Vec<u32>, Vec<u8>)>>>;

    fn word(frame: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(frame[offset..offset + 4].try_into().unwrap())
    }

    #[test]
    fn jobs_split_the_total_and_the_last_one_ends_at_it_00e2aa70() {
        let mut e = engine();
        let jobs = e.mem.alloc(0x70 * 4);
        e.mem.write(jobs, &[0xcd; 0x70 * 4]);
        e.call(
            0x00e2_aa70,
            &args![0x11u32, 10u32, 0x33u32, 0x44u32, 3u32, jobs],
        );
        for (job, start, end) in [(0u32, 0u32, 3u32), (1, 3, 6), (2, 6, 10)] {
            let at = jobs + 0x70 * job;
            assert_eq!(e.mem.u32(at + 0x10), 0x44, "job {job}");
            assert_eq!(e.mem.u32(at + 0x50), 0x11);
            assert_eq!(e.mem.u32(at + 0x54), 0x33);
            assert_eq!(e.mem.u32(at + 0x58), start);
            assert_eq!(e.mem.u32(at + 0x5c), end);
        }
        // Only three jobs are touched.
        assert_eq!(e.mem.u8(jobs + 0x70 * 3), 0xcd);
    }

    #[test]
    fn the_count_is_limited_to_the_total_00e2aa70() {
        let mut e = engine();
        let jobs = e.mem.alloc(0x70 * 4);
        e.call(0x00e2_aa70, &args![1u32, 2u32, 3u32, 4u32, 9u32, jobs]);
        // count 2, share 1.
        assert_eq!((e.mem.u32(jobs + 0x58), e.mem.u32(jobs + 0x5c)), (0, 1));
        let second = jobs + 0x70;
        assert_eq!((e.mem.u32(second + 0x58), e.mem.u32(second + 0x5c)), (1, 2));
        // A single job: the share is the total.
        let single = e.mem.alloc(0x70 * 2);
        e.call(0x00e2_aa70, &args![1u32, 7u32, 3u32, 4u32, 1u32, single]);
        assert_eq!((e.mem.u32(single + 0x58), e.mem.u32(single + 0x5c)), (0, 7));
    }

    #[test]
    fn a_negative_count_writes_the_total_before_the_buffer_00e2aa70() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x400);
        let jobs = buffer + 0x200;
        e.call(0x00e2_aa70, &args![1u32, 4u32, 3u32, 4u32, u32::MAX, jobs]);
        // count -1, share -4; no copies; the total goes to jobs - 0x70 - 0x14.
        assert_eq!(e.mem.u32(jobs + 0x5c), -4i32 as u32);
        assert_eq!(e.mem.u32(jobs - 0x70 - 0x14), 4);
    }

    #[test]
    #[should_panic(expected = "00e2aa70")]
    fn a_zero_count_faults_00e2aa70() {
        let mut e = engine();
        let jobs = e.mem.alloc(0x100);
        e.call(0x00e2_aa70, &args![1u32, 4u32, 3u32, 4u32, 0u32, jobs]);
    }

    #[test]
    fn the_rows_are_transposed_with_the_fourth_row_00e2ab00() {
        let mut e = engine();
        let source = e.mem.alloc(0xc0);
        for index in 0..0x30u32 {
            e.mem.set_f32(source + 4 * index, index as f32);
        }
        let [first, second] = [0; 2].map(|_| e.mem.alloc(0x30));
        e.call(0x00e2_ab00, &args![source, first, second]);
        // The source row at byte `byte`: its words are numbered by their index.
        let row =
            |byte: u32| -> [f32; 4] { std::array::from_fn(|lane| (byte / 4 + lane as u32) as f32) };
        for (out, from, fourth) in [(first, 0x00u32, 0x90u32), (second, 0x10, 0xa0)] {
            let rows = [row(from), row(from + 0x30), row(from + 0x60), row(fourth)];
            for column in 0..3 {
                let expected: [f32; 4] = std::array::from_fn(|r| rows[r][column]);
                assert_eq!(load_vec(&e, out + 0x10 * column as u32), expected);
            }
        }
    }

    /// A double for the single-ray cast that records the arguments and the frame from its
    /// low end `L = argument 4 - 0x14`.
    fn cast_ray_double(e: &mut Engine) -> Snapshots {
        let seen: Snapshots = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(KD_TREE_CAST_RAY, move |e, a| {
            let frame = e.mem.bytes(a[4] - 0x14, 0x90);
            log.borrow_mut().push((a.to_vec(), frame));
            Ret::default()
        });
        seen
    }

    #[test]
    fn nothing_happens_without_a_tree_00e2abf0() {
        let mut e = engine();
        let seen = cast_ray_double(&mut e);
        let ray = e.mem.alloc(0x40);
        e.call(0x00e2_abf0, &args![0u32, ray, 0x100u32, 0u32]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn the_cast_object_is_built_from_the_ray_00e2abf0() {
        let mut e = engine();
        let seen = cast_ray_double(&mut e);
        let ray = e.mem.alloc(0x40);
        e.mem.set_u8(ray + 0x20, 1);
        e.mem.set_u32(ray + 0x24, 0x2424);
        e.mem.set_u32(ray + 0x30, 0x3030);
        e.mem.set_u32(ray + 0x34, 0x3434);
        e.mem.set_u8(ray + 0x3c, 1);
        let collector = e.mem.alloc(0x20);
        e.call(0x00e2_abf0, &args![0x77u32, ray, 0x100u32, collector]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (a, frame) = &seen[0];
        let l = a[4] - 0x14;
        assert_eq!(*a, [l + 0xf, 0x77, ray, ray + 0x10, l + 0x14]);
        assert_eq!(
            [0x10, 0x14, 0x18, 0x1c].map(|at| word(frame, at)),
            CAST_VTABLES
        );
        assert_eq!(word(frame, 0x20), 0x100);
        assert_eq!(word(frame, 0x24), 1.0f32.to_bits());
        assert_eq!(frame[0x28], 0);
        assert_eq!([0x2c, 0x30].map(|at| word(frame, at)), [ray, ray]);
        assert_eq!([0x34, 0x38].map(|at| word(frame, at)), [0x3030, 0x3030]);
        assert_eq!(word(frame, 0x60), 0x2424);
        assert_eq!(word(frame, 0x64), 0x110);
        assert_eq!(word(frame, 0x70), collector);
        assert_eq!(e.mem.u32(collector), RAY_HIT_COLLECTOR_VTABLE);
        assert_eq!(e.mem.f32(collector + 4), 1.0);
        assert_eq!(e.mem.u32(collector + 8), 0x3030);
        assert_eq!(e.mem.u32(collector + 0xc), 0x3030);
        assert_eq!(e.mem.u32(collector + 0x10), 0x3434);
        assert_eq!(e.mem.u32(collector + 0x14), 0);
        assert_eq!(e.mem.u32(collector + 0x18), 0);
        assert_eq!(e.global::<u32>(COLLECTOR_CALLBACK_SLOT), COLLECTOR_CALLBACK);
    }

    #[test]
    fn the_limit_only_counts_when_the_ray_flag_is_set_00e2abf0() {
        for (flag, limit, expected) in [(1u8, 0x100u32, 0x110u32), (0, 0x100, 0), (1, 0, 0)] {
            let mut e = engine();
            let seen = cast_ray_double(&mut e);
            let ray = e.mem.alloc(0x40);
            e.mem.set_u8(ray + 0x20, flag);
            e.call(0x00e2_abf0, &args![1u32, ray, limit, 0u32]);
            let seen = seen.borrow();
            let frame = &seen[0].1;
            assert_eq!(word(frame, 0x64), expected, "flag {flag} limit {limit}");
            assert_eq!(word(frame, 0x20), limit);
            // Without the hit flag there is no collector and the callback slot stays as it was.
            assert_eq!(word(frame, 0x70), 0);
            assert_eq!(e.global::<u32>(COLLECTOR_CALLBACK_SLOT), 0);
        }
    }

    #[test]
    fn a_missing_collector_still_sets_the_callback_slot_00e2abf0() {
        let mut e = engine();
        let seen = cast_ray_double(&mut e);
        let ray = e.mem.alloc(0x40);
        e.mem.set_u8(ray + 0x3c, 1);
        e.call(0x00e2_abf0, &args![1u32, ray, 0u32, 0u32]);
        assert_eq!(word(&seen.borrow()[0].1, 0x70), 0);
        assert_eq!(e.global::<u32>(COLLECTOR_CALLBACK_SLOT), COLLECTOR_CALLBACK);
    }

    fn bundle_cast_double(e: &mut Engine) -> Snapshots {
        let seen: Snapshots = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(KD_TREE_CAST_BUNDLE, move |e, a| {
            let frame = e.mem.bytes(a[2] - 0x310, 0x380);
            log.borrow_mut().push((a.to_vec(), frame));
            Ret::default()
        });
        seen
    }

    /// A bundle of two rays: recognisable rows, per-ray records and words.
    fn bundle(e: &mut Engine) -> (u32, [u32; 2]) {
        let bundle = e.mem.alloc(0x100);
        for offset in (0..0xc0).step_by(4) {
            e.mem.set_u32(bundle + offset, 0x1000_0000 + offset);
        }
        e.mem.set_u32(bundle + 0xc0, 2);
        e.mem.set_u8(bundle + 0xc4, 0x5a);
        let records = [0; 2].map(|_| e.mem.alloc(0x60));
        for (index, record) in (0u32..).zip(records) {
            for offset in (0..0x60).step_by(4) {
                e.mem
                    .set_u32(record + offset, 0x2000_0000 + 0x100 * index + offset);
            }
            e.mem.set_u32(bundle + 0xc8 + 4 * index, record);
            e.mem
                .set_u32(bundle + 0xe0 + 4 * index, 0x3000_0000 + index);
        }
        e.mem.set_u32(bundle + 0xe8, 0x3e8);
        e.mem.set_u32(bundle + 0xec, 0x3ec);
        (bundle, records)
    }

    #[test]
    fn nothing_happens_without_a_tree_in_the_bundle_cast_00e2acf0() {
        let mut e = engine();
        let seen = bundle_cast_double(&mut e);
        let (bundle, _) = bundle(&mut e);
        e.call(0x00e2_acf0, &args![0u32, bundle, 0u32, 0u32]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn the_bundle_cast_builds_the_frame_and_gives_it_back_00e2acf0() {
        let mut e = engine();
        e.set_global(BUNDLE_TABLE + 8, 0x7e57u32);
        let log = new_log();
        log_double(&mut e, &log, BUNDLE_COLLECTOR_CONSTRUCT, 0);
        log_double(&mut e, &log, BUNDLE_COLLECTOR_GET_HIT_FRACTIONS, 0);
        let seen = bundle_cast_double(&mut e);
        let (bundle, records) = bundle(&mut e);
        // A signalling NaN in the first ray's own float: replaced by its hit fraction.
        e.mem.set_u32(records[0] + 0x10, 0x7f80_0001);
        let hits = e.mem.alloc(0x10);
        e.mem.set_f32(hits, 0.25);
        e.mem.set_u32(hits + 4, 0x7f80_0002);
        e.call(0x00e2_acf0, &args![0x55u32, bundle, 0x1234u32, hits]);

        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (a, frame) = &seen[0];
        let l = a[2] - 0x310;
        assert_eq!(
            *a,
            [l + 0x13, 0x55, l + 0x310, l + 0x340, l + 0x100, 0x7e57]
        );
        // The collector: constructed, then filled in.
        assert_eq!(
            calls(&log, BUNDLE_COLLECTOR_CONSTRUCT),
            vec![vec![l + 0x100]]
        );
        assert_eq!(word(frame, 0x104), 0x1234);
        assert_eq!(word(frame, 0x108), bundle);
        assert_eq!(word(frame, 0x10c), l + 0x20);
        assert_eq!(word(frame, 0x170), 0x1000_0024);
        assert_eq!(word(frame, 0x180), 0x3000_0000);
        assert_eq!(word(frame, 0x184), 0x3000_0001);
        assert_eq!(word(frame, 0x188), 0x3e8);
        assert_eq!(word(frame, 0x18c), 0x3ec);
        assert_eq!(frame[0xe0], 0x5a);
        assert_eq!(word(frame, 0xe4), l + 0x190);
        // The records: copies of the rays' own, with the hit fractions in their +0x10.
        for (index, base) in [(0usize, 0x2000_0000u32), (1, 0x2000_0100)] {
            let at = 0x190 + 0x60 * index;
            assert_eq!(word(frame, at), base);
            assert_eq!(word(frame, at + 0x0c), base + 0x0c);
            assert_eq!(word(frame, at + 0x14), base + 0x14);
            assert_eq!(word(frame, at + 0x20), base + 0x20);
            assert_eq!(word(frame, at + 0x40), base + 0x40);
            assert_eq!(word(frame, at + 0x50), base + 0x50);
            // The words between those are not copied.
            assert_eq!(word(frame, at + 0x44), 0);
            assert_eq!(word(frame, at + 0x54), 0);
        }
        assert_eq!(word(frame, 0x1a0), 0.25f32.to_bits());
        assert_eq!(word(frame, 0x200), 0x7fc0_0002);
        // The unused third and fourth records keep their initial values.
        assert_eq!(word(frame, 0x260), 1.0f32.to_bits());
        assert_eq!(word(frame, 0x2c0), 1.0f32.to_bits());
        for at in [0x264, 0x270, 0x2c4, 0x2d0] {
            assert_eq!(word(frame, at), u32::MAX);
        }
        for at in [0x290, 0x2a0, 0x2f0, 0x300] {
            assert_eq!(word(frame, at), 0);
        }
        // The ray slots: flag byte, word, two vectors; the saved words.
        assert_eq!(frame[0x40], 0x20);
        assert_eq!(word(frame, 0x44), 0x1000_0024);
        assert_eq!(word(frame, 0x20), 0x1000_0000);
        assert_eq!(word(frame, 0x30), 0x1000_0010);
        assert_eq!(frame[0x70], 0x50);
        assert_eq!(word(frame, 0x74), 0x1000_0054);
        assert_eq!(word(frame, 0x50), 0x1000_0030);
        assert_eq!(word(frame, 0x60), 0x1000_0040);
        assert_eq!(word(frame, 0xf0), 0x3000_0000);
        assert_eq!(word(frame, 0xf4), 0x3000_0001);
        assert_eq!(frame[0xa0], 0);
        assert_eq!(word(frame, 0xd4), 0);
        // The column sets from `00e2ab00`.
        assert_eq!(word(frame, 0x310), 0x1000_0000);
        assert_eq!(word(frame, 0x314), 0x1000_0030);
        assert_eq!(word(frame, 0x318), 0x1000_0060);
        assert_eq!(word(frame, 0x31c), 0x1000_0090);
        // Afterwards the bundle's words are back and the records hold the frame's floats.
        assert_eq!(e.mem.u32(bundle + 0xe0), 0x3000_0000);
        assert_eq!(e.mem.u32(bundle + 0xe4), 0x3000_0001);
        assert_eq!(e.mem.u32(records[0] + 0x10), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(records[1] + 0x10), 0x7fc0_0002);
        assert_eq!(
            calls(&log, BUNDLE_COLLECTOR_GET_HIT_FRACTIONS),
            vec![vec![l + 0x100, hits]]
        );
    }

    #[test]
    fn a_bundle_without_rays_still_casts_00e2acf0() {
        let mut e = engine();
        e.set_global(BUNDLE_TABLE, 0x70u32);
        let seen = bundle_cast_double(&mut e);
        let log = new_log();
        log_double(&mut e, &log, BUNDLE_COLLECTOR_CONSTRUCT, 0);
        log_double(&mut e, &log, BUNDLE_COLLECTOR_GET_HIT_FRACTIONS, 0);
        let bundle = e.mem.alloc(0x100);
        let hits = e.mem.alloc(0x10);
        e.call(0x00e2_acf0, &args![9u32, bundle, 1u32, hits]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[5], 0x70);
        assert_eq!(calls(&log, BUNDLE_COLLECTOR_GET_HIT_FRACTIONS).len(), 1);
    }
}
