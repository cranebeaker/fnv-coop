//! `Havok SDK/hkpcollidablemediator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpCollidableMediator` (vtable `010d40b4`, RTTI `.?AVhkpCollidableMediator@@`) holds,
//! at `+8`, a list object (`+0` the element array, `+4` the element count) of collidables
//! (`hkpCollidable`, Xbox PDB: `m_shape` `+0`, `m_motion` `+8`, `m_ownerOffset` signed byte
//! `+0x10`, broad-phase handle `+0x14`/`+0x18`, bounding volume data `+0x20`..`+0x3F`: three
//! min words and the expansion bytes at `+0x20`, three max words at `+0x30`). The
//! collidable's owner (an entity or a phantom) lies `m_ownerOffset` bytes from the
//! collidable; the owner's world is at `owner + 8` and the world's broad phase at
//! `world + 0x78`, whose `+0x20` offset, `+0x30` offset and `+0x40` scale vectors convert
//! floats to the integer bounding volume.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, maxps, minps, zip_with, Vec4};

/// Vtable of the mediator (`.?AVhkpCollidableMediator@@`).
const MEDIATOR_VTABLE: u32 = 0x010d_40b4;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (cdecl: broad phase, entity list, count).
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpPhantom::setBoundingVolumeData` (`RET 4`).
const PHANTOM_SET_BOUNDING_VOLUME_DATA: u32 = 0x00d0_7950;
/// Copies the eight words of a bounding volume data block (cdecl: destination, source).
const COPY_BOUNDING_VOLUME_DATA: u32 = 0x00cf_8690;
/// Four floats `2147352576.0` (`0x4efffc00`): the upper clamp of the integer conversion.
const CLAMP_MAX: u32 = 0x011b_6d70;
/// Four zeros: the lower clamp of the integer conversion.
const CLAMP_MIN: u32 = 0x0126_7e30;

// Translated from 00d9df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableMediator::hkpCollidableMediator` (Xbox PDB): sets the reference count `+6`
/// to 1, the vtable and the collidable list `+8`. When `context` is not null, every
/// collidable whose first min word (`+0x20`) is above its first max word (`+0x30`) (unsigned:
/// the bounding volume was never set) gets one: an entity (broad-phase handle type byte
/// `+0x18` equal to 1) through `entityBatchRecalcAabb` on `[context + 0x78]`, anything else
/// (a phantom) by asking the owner's slot `0x18` for its AABB and passing it to
/// `hkpPhantom::setBoundingVolumeData`. Returns `this`.
pub fn hkp_collidable_mediator_ctor(
    e: &mut Engine,
    this: Ptr,
    collidables: Ptr,
    context: Ptr,
) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), MEDIATOR_VTABLE);
    e.mem.set_u32(this.addr() + 8, collidables.addr());
    if context.addr() != 0 && (e.mem.u32(collidables.addr() + 4) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let list = e.mem.u32(this.addr() + 8);
            let collidable = e.mem.u32(e.mem.u32(list) + index * 4);
            if e.mem.u32(collidable + 0x20) > e.mem.u32(collidable + 0x30) {
                let owner = collidable.wrapping_add(e.mem.u8(collidable + 0x10) as i8 as u32);
                if e.mem.u8(collidable + 0x18) == 1 {
                    let broad_phase = e.mem.u32(context.addr() + 0x78);
                    e.with_stack(4, |e, entity| {
                        e.mem.set_u32(entity.addr(), owner);
                        e.call(ENTITY_BATCH_RECALC_AABB, &args![broad_phase, entity, 1u32]);
                    });
                } else {
                    e.with_stack(0x2c, |e, aabb| {
                        e.vcall(owner, 0x18, &args![aabb]);
                        e.call(PHANTOM_SET_BOUNDING_VOLUME_DATA, &args![owner, aabb]);
                    });
                }
            }
            index += 1;
            let list = e.mem.u32(this.addr() + 8);
            if (index as i32) >= (e.mem.u32(list + 4) as i32) {
                break;
            }
        }
    }
    this
}

// Translated from 00d9e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableMediator::getNumPrimitives` (Xbox PDB): the element count of the collidable
/// list.
pub fn hkp_collidable_mediator_get_num_primitives(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.mem.u32(this.addr() + 8);
    e.mem.u32(list + 4)
}

// Translated from 00d9e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableMediator::getPrimitiveId` (Xbox PDB): element `index` of the collidable
/// list; the primitive id is the collidable's address.
pub fn hkp_collidable_mediator_get_primitive_id(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let list = e.mem.u32(this.addr() + 8);
    let data = e.mem.u32(list);
    e.mem.u32(data.wrapping_add(index.wrapping_mul(4)))
}

/// The x87 `FISTP` of a value already clamped into `0..=2147352576` to a 32-bit word, with
/// the default round-to-nearest-even control word.
fn float_to_word(value: f32) -> u32 {
    crate::x87::fistp(f64::from(value)) as u32
}

/// One bounding volume corner: `(offset + corner) * scale` per lane, clamped with `MINPS`
/// against the upper bound and `MAXPS` against the lower one, then converted to words (lanes
/// 0 to 2 only; lane 3 is returned as converted too).
fn corner_words(offset: Vec4, corner: Vec4, scale: Vec4, upper: Vec4, lower: Vec4) -> [u32; 4] {
    let sum = zip_with(offset, corner, |a, b| a + b);
    let scaled = zip_with(sum, scale, |a, b| a * b);
    let capped = zip_with(scaled, upper, minps);
    let clamped = zip_with(capped, lower, maxps);
    clamped.map(float_to_word)
}

// Translated from 00d9e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableMediator::projectPrimitive` (Xbox PDB): writes the 16-byte compressed
/// bounding volume of the collidable `primitive` to `out`: eight 16-bit words, the high
/// words of `min << 1` for the four min lanes and then of the four max lanes' `max << 1`
/// plus one (unsigned saturating). `this` (`ECX`) is not read.
///
/// When the collidable's bounding volume data is set (first min word `+0x20` not above the
/// first max word `+0x30`, unsigned) the eight words at `+0x20` are copied (`00cf8690`) and
/// used. Otherwise the shape (`[primitive]`) is asked for its AABB (slot `0x1C`, with the
/// transform `[primitive + 8]` and tolerance `0.0`), and both corners are converted with the
/// broad phase's offset (`+0x20` for the min corner, `+0x30` for the max corner) and scale
/// (`+0x40`): clamped to `0 ..= 2147352576.0` and rounded to nearest-even words. The min
/// `w` word is zero and the max `w` word keeps only its top byte. Assumes the default x87
/// rounding mode.
pub fn hkp_collidable_mediator_project_primitive(
    e: &mut Engine,
    _unused_this: Ptr,
    primitive: Ptr,
    out: Ptr,
) {
    let base = primitive.addr();
    let words: [u32; 8] = if e.mem.u32(base + 0x20) > e.mem.u32(base + 0x30) {
        let owner = base.wrapping_add(e.mem.u8(base + 0x10) as i8 as u32);
        let world = e.mem.u32(owner + 8);
        let shape = e.mem.u32(base);
        let transform = e.mem.u32(base + 8);
        let broad_phase = e.mem.u32(world + 0x78);
        e.with_stack(0x20, |e, aabb| {
            e.vcall(shape, 0x1c, &args![transform, 0.0f32, aabb]);
            let low = load_vec(e, aabb.addr());
            let high = load_vec(e, aabb.addr() + 0x10);
            let offset_low = load_vec(e, broad_phase + 0x20);
            let offset_high = load_vec(e, broad_phase + 0x30);
            let scale = load_vec(e, broad_phase + 0x40);
            let upper = load_vec(e, CLAMP_MAX);
            let lower = load_vec(e, CLAMP_MIN);
            let mut min = corner_words(offset_low, low, scale, upper, lower);
            let mut max = corner_words(offset_high, high, scale, upper, lower);
            min[3] = 0;
            max[3] &= 0xff00_0000;
            [
                min[0], min[1], min[2], min[3], max[0], max[1], max[2], max[3],
            ]
        })
    } else {
        e.with_stack(0x20, |e, copy| {
            e.call(
                COPY_BOUNDING_VOLUME_DATA,
                &args![copy, primitive.addr() + 0x20],
            );
            std::array::from_fn(|i| e.mem.u32(copy.addr() + 4 * i as u32))
        })
    };
    for (i, word) in words.into_iter().enumerate() {
        let shifted = word << 1;
        let high_word = (shifted >> 16) as u16;
        let value = if i < 4 {
            high_word
        } else {
            high_word.saturating_add(1)
        };
        e.mem.set_u16(out.addr() + 2 * i as u32, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_df50, hkp_collidable_mediator_ctor(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d9_e000, hkp_collidable_mediator_get_num_primitives(Ptr) -> u32),
        entry!(0x00d9_e010, hkp_collidable_mediator_get_primitive_id(Ptr, u32) -> u32),
        entry!(
            0x00d9_e020,
            hkp_collidable_mediator_project_primitive(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    /// A mediator object whose list (`+8`) holds the given collidables: `(this, list)`.
    fn mediator(e: &mut Engine, collidables: &[u32]) -> (u32, u32) {
        let data = e.mem.alloc(0x40);
        for (i, c) in collidables.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *c);
        }
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, collidables.len() as u32);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, list);
        (this, list)
    }

    /// A collidable `0x40` bytes after its owner block (owner offset `-0x40`), with the
    /// broad-phase handle type and the first min/max words of its bounding volume data.
    /// Returns `(collidable, owner)`.
    fn collidable(e: &mut Engine, kind: u8, min0: u32, max0: u32) -> (u32, u32) {
        let owner = e.mem.alloc(0x100);
        let collidable = owner + 0x40;
        e.mem.set_u8(collidable + 0x10, 0xc0);
        e.mem.set_u8(collidable + 0x18, kind);
        e.mem.set_u32(collidable + 0x20, min0);
        e.mem.set_u32(collidable + 0x30, max0);
        (collidable, owner)
    }

    #[test]
    fn constructor_00d9df50() {
        let mut e = engine();
        let (valid, _) = collidable(&mut e, 1, 5, 9);
        let (entity, entity_owner) = collidable(&mut e, 1, 9, 5);
        let (phantom, phantom_owner) = collidable(&mut e, 0, 6, 5);
        let (_, list) = mediator(&mut e, &[valid, entity, phantom]);
        let this = e.mem.alloc(0x10);
        let context = e.mem.alloc(0x100);
        e.mem.set_u32(context + 0x78, 0xb0ad);
        let log = new_log();
        let seen = log.clone();
        e.register_double(ENTITY_BATCH_RECALC_AABB, move |e, a| {
            let first = e.mem.u32(a[1]);
            seen.borrow_mut()
                .push((ENTITY_BATCH_RECALC_AABB, vec![a[0], first, a[2]]));
            Ret::default()
        });
        log_double(&mut e, &log, PHANTOM_SET_BOUNDING_VOLUME_DATA, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, 0x0900_0018);
        e.mem.set_u32(phantom_owner, vtable);
        log_double(&mut e, &log, 0x0900_0018, 0);

        let result = e.call(0x00d9_df50, &[this, list, context]).u32();

        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), MEDIATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 8), list);
        assert_eq!(
            calls(&log, ENTITY_BATCH_RECALC_AABB),
            vec![vec![0xb0ad, entity_owner, 1]]
        );
        let get_aabb = calls(&log, 0x0900_0018);
        assert_eq!(get_aabb.len(), 1);
        assert_eq!(get_aabb[0][0], phantom_owner);
        let set = calls(&log, PHANTOM_SET_BOUNDING_VOLUME_DATA);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0], vec![phantom_owner, get_aabb[0][1]]);
    }

    #[test]
    fn constructor_without_context_skips_the_collidables_00d9df50() {
        let mut e = engine();
        let (entity, _) = collidable(&mut e, 1, 9, 5);
        let (_, list) = mediator(&mut e, &[entity]);
        let this = e.mem.alloc(0x10);
        let log = new_log();
        log_double(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.call(0x00d9_df50, &[this, list, 0]);
        assert!(calls(&log, ENTITY_BATCH_RECALC_AABB).is_empty());
        assert_eq!(e.mem.u32(this + 8), list);
    }

    #[test]
    fn constructor_with_an_empty_list_00d9df50() {
        let mut e = engine();
        let (_, list) = mediator(&mut e, &[]);
        let this = e.mem.alloc(0x10);
        let context = e.mem.alloc(0x100);
        let log = new_log();
        log_double(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.call(0x00d9_df50, &[this, list, context]);
        assert!(calls(&log, ENTITY_BATCH_RECALC_AABB).is_empty());
    }

    #[test]
    fn counts_and_ids_00d9e000_00d9e010() {
        let mut e = engine();
        let (this, _) = mediator(&mut e, &[0x1111, 0x2222, 0x3333]);
        assert_eq!(e.call(0x00d9_e000, &[this]).u32(), 3);
        assert_eq!(e.call(0x00d9_e010, &[this, 0]).u32(), 0x1111);
        assert_eq!(e.call(0x00d9_e010, &[this, 2]).u32(), 0x3333);
    }

    /// Installs a double of `00cf8690` that copies the eight words, and returns its log.
    fn copy_double(e: &mut Engine) -> Log {
        let log = new_log();
        let seen = log.clone();
        e.register_double(COPY_BOUNDING_VOLUME_DATA, move |e, a| {
            seen.borrow_mut()
                .push((COPY_BOUNDING_VOLUME_DATA, a.to_vec()));
            for i in 0..8 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        log
    }

    #[test]
    fn project_primitive_uses_the_stored_volume_00d9e020() {
        let mut e = engine();
        let (primitive, _) = collidable(&mut e, 1, 0, 0);
        let stored = [
            0x1_0000u32,
            0x2_8000,
            0xffff_ffff,
            0x1234_5678,
            0x7fff_ffff,
            0x10,
            0xffff,
            0xffff_0000,
        ];
        for (i, word) in stored.into_iter().enumerate() {
            e.mem.set_u32(primitive + 0x20 + 4 * i as u32, word);
        }
        let log = copy_double(&mut e);
        let out = e.mem.alloc(0x10);

        e.call(0x00d9_e020, &[0, primitive, out]);

        let copies = calls(&log, COPY_BOUNDING_VOLUME_DATA);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][1], primitive + 0x20);
        let words: Vec<u16> = (0..8).map(|i| e.mem.u16(out + 2 * i)).collect();
        // Min lanes: the high word of the word shifted left once. Max lanes: the same plus
        // one, saturating at 0xffff.
        assert_eq!(words, vec![2, 5, 0xffff, 0x2468, 0xffff, 1, 2, 0xffff]);
    }

    #[test]
    fn project_primitive_converts_the_shape_aabb_00d9e020() {
        let mut e = engine();
        let (primitive, owner) = collidable(&mut e, 1, 9, 5);
        let broad_phase = e.mem.alloc(0x100);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x78, broad_phase);
        e.mem.set_u32(owner + 8, world);
        for lane in 0..4 {
            e.mem.set_f32(broad_phase + 0x20 + 4 * lane, 0.5);
            e.mem.set_f32(broad_phase + 0x30 + 4 * lane, 1.0);
            e.mem.set_f32(broad_phase + 0x40 + 4 * lane, 1_000_000.0);
            e.mem.set_u32(CLAMP_MAX + 4 * lane, 0x4eff_fc00);
            e.mem.set_u32(CLAMP_MIN + 4 * lane, 0);
        }
        let shape = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape, vtable);
        e.mem.set_u32(vtable + 0x1c, 0x0900_001c);
        e.mem.set_u32(primitive, shape);
        let transform = e.mem.alloc(0x40);
        e.mem.set_u32(primitive + 8, transform);
        let log = new_log();
        let seen = log.clone();
        e.register_double(0x0900_001c, move |e, a| {
            seen.borrow_mut().push((0x0900_001c, a.to_vec()));
            let min = [1.0f32, -5.0, f32::NAN, 5.0];
            let max = [4.0f32, 5.0, 6.0, 1e10];
            for lane in 0..4 {
                e.mem.set_f32(a[3] + 4 * lane, min[lane as usize]);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane, max[lane as usize]);
            }
            Ret::default()
        });
        let out = e.mem.alloc(0x10);

        e.call(0x00d9_e020, &[0, primitive, out]);

        let query = calls(&log, 0x0900_001c);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][0], shape);
        assert_eq!(query[0][1], transform);
        assert_eq!(query[0][2], 0.0f32.to_bits());
        let words: Vec<u16> = (0..8).map(|i| e.mem.u16(out + 2 * i)).collect();
        // Min: 1.5e6 gives 45; the negative lane clamps to 0; NaN becomes the upper bound
        // (0xfffc); w is zero. Max: 5e6 gives 153, 6e6 gives 184, 7e6 gives 214; the huge w
        // keeps only its top byte 0x7f, giving 0xfe00 + 1.
        assert_eq!(words, vec![45, 0, 0xfffc, 0, 153, 184, 214, 0xfe01]);
    }
}
