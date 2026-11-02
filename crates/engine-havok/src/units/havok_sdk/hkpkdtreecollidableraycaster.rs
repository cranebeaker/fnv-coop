//! `Havok SDK/hkpkdtreecollidableraycaster.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, rows_times, splat, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, shufps, store_bits, store_vec, Vec4, LANE_BITS,
};

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable both deleting destructors leave in the object (`010d4050`).
const BASE_VTABLE: u32 = 0x010d_4050;
/// Size of the caster object the second deleting destructor frees.
const CASTER_SIZE: u32 = 0x14;
/// `-1.0` and `1.0`, the lane values of a rejected and an accepted ray.
const MINUS_ONE_F32: u32 = 0x0101_2054;
const ONE_F32: u32 = 0x0101_2068;

/// Offsets inside a 0x50-byte ray hit record (the `hkpShapeRayCastOutput` layout used by the
/// shape code in this crate: normal +0, hit fraction +0x10, extra info +0x14, shape keys
/// +0x20, shape key index +0x40) and the 0x60-byte records the caster fills.
const HIT_FRACTION: u32 = 0x10;
const HIT_COLLIDABLE: u32 = 0x50;

// Translated from 00d979a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBundleCastCollector::_scalar_deleting_destructor_` (Xbox PDB): leaves the vtable
/// `010d4050` in the object and, when bit 0 of `flags` is set, calls `operator delete` on it.
/// Returns `this`.
pub fn hk_kd_tree_bundle_cast_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00d979c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeCollidableRaycaster::hasHit` (Xbox PDB): writes to the byte at `out` whether the
/// hit record at `this + 0x10` has a collidable (its word at `+0x50`) set.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_kd_tree_collidable_raycaster_has_hit(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let hit = e.mem.u32(this.addr() + 0x10);
    let found = e.mem.u32(hit + HIT_COLLIDABLE) != 0;
    e.mem.set_u8(out.addr(), u8::from(found));
    out
}

// Translated from 00d979e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeCollidableRaycaster::getHitFraction` (Xbox PDB): the hit fraction of the hit
/// record at `this + 0x10`, loaded on the x87 stack.
pub fn hkp_kd_tree_collidable_raycaster_get_hit_fraction(e: &mut Engine, this: Ptr) -> f32 {
    let hit = e.mem.u32(this.addr() + 0x10);
    x87_quiet(e.mem.f32(hit + HIT_FRACTION))
}

// Translated from 00d97a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCollidableRayBundleCaster::getHitFractions` (Xbox PDB): the hit fractions of the
/// four hit records (0x60 bytes apart) at `this + 0x10`, copied to the four floats at `out`.
pub fn hk_kd_tree_collidable_ray_bundle_caster_get_hit_fractions(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) {
    let hits = e.mem.u32(this.addr() + 0x10);
    for lane in 0..4u32 {
        let fraction = x87_quiet(e.mem.f32(hits + HIT_FRACTION + 0x60 * lane));
        e.mem.set_f32(out.addr() + 4 * lane, fraction);
    }
}

/// The transform of a collidable (`+8` of the candidate): three rows at `+0`, `+0x10`, `+0x20`
/// and a translation at `+0x30`.
struct Frame {
    rows: [Vec4; 3],
    translation: Vec4,
}

fn load_frame(e: &Engine, transform: u32) -> Frame {
    Frame {
        rows: [
            load_vec(e, transform),
            load_vec(e, transform + 0x10),
            load_vec(e, transform + 0x20),
        ],
        translation: load_vec(e, transform + 0x30),
    }
}

impl Frame {
    /// `(point - translation)` taken through the rows: the `UNPCK`/`SHUFPS` chain.
    fn to_local(&self, point: Vec4) -> Vec4 {
        let [r0, r1, r2] = self.rows;
        rows_times(sub(point, self.translation), r0, r1, r2)
    }
}

// Translated from 00d97a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeCollidableRaycaster::addMidPhaseHit` (Xbox PDB): `result` is the byte this writes
/// (1 when the candidate's object reported a hit that was recorded, else 0). `this + 4` is the
/// ray (from at `+0`, to at `+0x10`, the filter word at `+0x24`), `+8` an optional filter
/// object, `+0xC` a word copied into the local ray, `+0x10` the hit record, `+0x14` an
/// optional second object. The candidate's object is `[candidate]` and its transform
/// `[candidate + 8]`. Without an object nothing is cast. A filter (slot `+4`, returning a
/// pointer to a byte) can reject the candidate. The ray is taken into the candidate's
/// space (a 0x28-byte local: from at `+0`, to at `+0x10`, filter word `+0x20`, the word of
/// `this + 0xC` at `+0x24`). With `this + 0x14` set the object's slot `0x24` is called
/// with `(ray, candidate, that word)` and the result is 0; otherwise its slot `0x20` is
/// called with `(byte, ray, hit record)` and when the byte it sets is non-zero the candidate
/// is stored at `hit + 0x50`, the normal at `hit` is rotated back through the rows and
/// the result is 1.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00d1b726).
pub fn hkp_kd_tree_collidable_raycaster_add_mid_phase_hit(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    candidate: Ptr,
) -> Ptr {
    let this = this.addr();
    let candidate = candidate.addr();
    let object = e.mem.u32(candidate);
    let reject = e.with_stack(0x50, |e, frame| {
        let flag = frame.addr() + 0x1b;
        let ray_local = frame.addr() + 0x20;
        if object == 0 {
            return true;
        }
        let filter = e.mem.u32(this + 8);
        let ray = e.mem.u32(this + 4);
        if filter != 0 {
            let verdict = e.vcall(filter, 4, &args![flag, ray, candidate]).u32();
            if e.mem.u8(verdict) == 0 {
                return true;
            }
        }
        let transform = e.mem.u32(candidate + 8);
        let local = load_frame(e, transform);
        let from = local.to_local(load_vec(e, ray));
        store_vec(e, ray_local, from);
        let to = local.to_local(load_vec(e, ray + 0x10));
        store_vec(e, ray_local + 0x10, to);
        let filter_word = e.mem.u32(ray + 0x24);
        let user_word = e.mem.u32(this + 0xc);
        e.mem.set_u32(ray_local + 0x20, filter_word);
        e.mem.set_u32(ray_local + 0x24, user_word);
        let other = e.mem.u32(this + 0x14);
        if other != 0 {
            e.vcall(object, 0x24, &args![ray_local, candidate, other]);
            return true;
        }
        let hit = e.mem.u32(this + 0x10);
        e.vcall(object, 0x20, &args![flag, ray_local, hit]);
        if e.mem.u8(flag) == 0 {
            return true;
        }
        e.mem.set_u32(hit + HIT_COLLIDABLE, candidate);
        let normal = load_vec(e, hit);
        let [r0, r1, r2] = local.rows;
        let rotated = add(
            add(mul(splat(normal, 0), r0), mul(splat(normal, 1), r1)),
            mul(splat(normal, 2), r2),
        );
        store_vec(e, hit, rotated);
        false
    });
    e.mem.set_u8(result.addr(), u8::from(!reject));
    result
}

/// The four 0x30-byte rays of a bundle: ray `i` has its start at `+0x30 * i` and its end at
/// `+0x30 * i + 0x10`.
fn bundle_ray(index: u32) -> u32 {
    0x30 * index
}

/// Lane `lane` of four vectors as one vector (the `SHUFPS` transpose).
fn transpose(vectors: [Vec4; 4]) -> [Vec4; 3] {
    let low01 = shufps(vectors[0], vectors[1], 0x44);
    let high01 = shufps(vectors[0], vectors[1], 0xee);
    let low23 = shufps(vectors[2], vectors[3], 0x44);
    let high23 = shufps(vectors[2], vectors[3], 0xee);
    [
        shufps(low01, low23, 0x88),
        shufps(low01, low23, 0xdd),
        shufps(high01, high23, 0x88),
    ]
}

// Translated from 00d97c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCollidableRayBundleCaster::addMidPhaseHit` (Xbox PDB): casts the four rays at
/// `this + 4` (0x30 bytes apart: start, end) against the candidate's object (`[candidate]`,
/// transform `[candidate + 8]`). A lane is live when the sign bit of its dword in the vector
/// at `mask` is set and the optional filter at `this + 8` (slot `+4`: `(byte, ray, candidate)`,
/// returning a pointer to a byte) accepts it; without a filter all four are live (the
/// exe tests `0.0 <= lane` on lanes holding `1.0` or `-1.0`). The rays are taken into the
/// candidate's space and laid out as x, y and z vectors of the four starts and four ends
/// (a 0x70-byte input: starts at `+0`, ends at `+0x30`, the filter word of `this + 4`'s
/// `+0x24` at `+0x60` and the word of `this + 0xC` at `+0x64`) with the lane mask after it,
/// and four 0x50-byte hit records (fraction `1.0`, `-1` at `+0x14` and `+0x20`, `0` at
/// `+0x40`) for the object's slot `0x28` (`(scratch, input, hits, lane mask)`). Every hit
/// whose fraction is below `1.0` and below the fraction already held in the 0x60-byte
/// record `i` of `this + 0x10` replaces it: the normal is rotated through the candidate's
/// rows, the fraction is loaded on the x87 stack, the words `+0x14` to `+0x40` are copied and
/// the candidate is stored at `+0x50`. (The exe also copies the word at `+0x50` of the hit
/// record, which it overwrites with the candidate straight away.)
pub fn hk_kd_tree_collidable_ray_bundle_caster_add_mid_phase_hit(
    e: &mut Engine,
    this: Ptr,
    candidate: Ptr,
    mask: Ptr,
) {
    let this = this.addr();
    let candidate = candidate.addr();
    let mask = mask.addr();
    let object = e.mem.u32(candidate);
    if object == 0 {
        return;
    }
    let transform = e.mem.u32(candidate + 8);
    let filter = e.mem.u32(this + 8);
    let rays = e.mem.u32(this + 4);
    e.with_stack(0x240, |e, frame| {
        let base = frame.addr();
        let flag = base + 0x1f;
        let mut lanes: Vec4 = [e.mem.f32(ONE_F32); 4];
        if filter != 0 {
            for lane in 0..4u32 {
                let bits = load_bits(e, mask);
                let signs = bits
                    .iter()
                    .enumerate()
                    .fold(0u32, |sum, (n, word)| sum | ((word >> 31) << n));
                let live = e.mem.u32(LANE_BITS + 4 * lane) & signs != 0;
                lanes[lane as usize] = if live {
                    let ray = rays + bundle_ray(lane);
                    let verdict = e.vcall(filter, 4, &args![flag, ray, candidate]).u32();
                    if e.mem.u8(verdict) == 0 {
                        e.mem.f32(MINUS_ONE_F32)
                    } else {
                        e.mem.f32(ONE_F32)
                    }
                } else {
                    e.mem.f32(MINUS_ONE_F32)
                };
            }
        }
        let lane_mask: [u32; 4] = lanes.map(|v| if 0.0 <= v { u32::MAX } else { 0 });
        store_bits(e, base + 0xf0, lane_mask);
        let local = load_frame(e, transform);
        let ray_vectors = |e: &Engine, offset: u32| -> [Vec4; 4] {
            std::array::from_fn(|lane| {
                local.to_local(load_vec(e, rays + bundle_ray(lane as u32) + offset))
            })
        };
        let starts = transpose(ray_vectors(e, 0));
        let ends = transpose(ray_vectors(e, 0x10));
        for (n, vector) in starts.into_iter().chain(ends).enumerate() {
            store_vec(e, base + 0x80 + 0x10 * n as u32, vector);
        }
        let filter_word = e.mem.u32(rays + 0x24);
        let user_word = e.mem.u32(this + 0xc);
        e.mem.set_u32(base + 0xe0, filter_word);
        e.mem.set_u32(base + 0xe4, user_word);
        let one = e.mem.f32(ONE_F32);
        for hit in 0..4u32 {
            let record = base + 0x100 + 0x50 * hit;
            e.mem.set_f32(record + HIT_FRACTION, one);
            e.mem.set_u32(record + 0x14, u32::MAX);
            e.mem.set_u32(record + 0x20, u32::MAX);
            e.mem.set_u32(record + 0x40, 0);
        }
        e.vcall(
            object,
            0x28,
            &args![base + 0x60, base + 0x80, base + 0x100, base + 0xf0],
        );
        let best = e.mem.u32(this + 0x10);
        let [r0, r1, r2] = local.rows;
        for hit in 0..4u32 {
            let record = base + 0x100 + 0x50 * hit;
            let target = best + 0x60 * hit;
            let fraction = e.mem.f32(record + HIT_FRACTION);
            let held = e.mem.f32(target + HIT_FRACTION);
            if one > fraction && held > fraction {
                copy_vec(e, record, target);
                e.mem.set_f32(target + HIT_FRACTION, x87_quiet(fraction));
                for word in 0..12u32 {
                    let value = e.mem.u32(record + 0x14 + 4 * word);
                    e.mem.set_u32(target + 0x14 + 4 * word, value);
                }
                e.mem.set_u32(target + HIT_COLLIDABLE, candidate);
                let normal = load_vec(e, record);
                let rotated = add(
                    add(mul(splat(normal, 1), r1), mul(splat(normal, 0), r0)),
                    mul(splat(normal, 2), r2),
                );
                store_vec(e, target, rotated);
            }
        }
    });
}

// Translated from 00d98300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initialises four 0x50-byte hit records at `this` (+0x10 of each
/// being a hit fraction of `1.0`, `-1` at `+0x14` and `+0x20`, `0` at `+0x40`), the layout the
/// bundle caster hands to the object's slot `0x28`. Which class it belongs to is
/// unconfirmed.
pub fn fn_00d98300(e: &mut Engine, this: Ptr) {
    let one = e.mem.f32(ONE_F32);
    for hit in 0..4u32 {
        let record = this.addr() + 0x50 * hit;
        e.mem.set_f32(record + HIT_FRACTION, one);
        e.mem.set_u32(record + 0x14, u32::MAX);
        e.mem.set_u32(record + 0x20, u32::MAX);
        e.mem.set_u32(record + 0x40, 0);
    }
}

// Translated from 00d98360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeCollidableRayBundleCaster::_scalar_deleting_destructor_` (Xbox PDB): leaves the
/// vtable `010d4050` in the object and, when bit 0 of `flags` is set, frees its 0x14 bytes
/// through the memory router's allocator (slot `8`). Returns `this`.
pub fn hk_kd_tree_collidable_ray_bundle_caster_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, CASTER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_79a0,
            hk_kd_tree_bundle_cast_collector_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00d9_79c0,
            hkp_kd_tree_collidable_raycaster_has_hit(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d9_79e0,
            hkp_kd_tree_collidable_raycaster_get_hit_fraction(Ptr) -> f32
        ),
        entry!(
            0x00d9_7a10,
            hk_kd_tree_collidable_ray_bundle_caster_get_hit_fractions(Ptr, Ptr)
        ),
        entry!(
            0x00d9_7a40,
            hkp_kd_tree_collidable_raycaster_add_mid_phase_hit(Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d9_7c90,
            hk_kd_tree_collidable_ray_bundle_caster_add_mid_phase_hit(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d9_8300, fn_00d98300(Ptr)),
        entry!(
            0x00d9_8360,
            hk_kd_tree_collidable_ray_bundle_caster_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The double base of the candidate's object: slot `s` is the double at `OBJECT + s`.
    const OBJECT: u32 = 0x0900_0000;
    /// The double base of the filter object.
    const FILTER: u32 = 0x0900_1000;
    /// A hit record the candidate object may fill: (words, ray start, ray end, filter, user).
    type Seen = Rc<RefCell<Vec<(Vec<u32>, [f32; 3], [f32; 3], u32, u32)>>>;

    /// Rows that send `(x, y, z)` to `(y, z, x)` through `to_local` and `(1, 2, 3)` to
    /// `(3, 1, 2)` through the row sum, and a translation.
    const ROWS: [Vec4; 3] = [
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    ];
    const TRANSLATION: Vec4 = [1.0, 2.0, 3.0, 0.0];

    fn xyz(v: Vec4) -> [f32; 3] {
        [v[0], v[1], v[2]]
    }

    fn constants(e: &mut Engine) {
        e.mem.set_f32(ONE_F32, 1.0);
        e.mem.set_f32(MINUS_ONE_F32, -1.0);
        for lane in 0..4 {
            e.mem.set_u32(LANE_BITS + 4 * lane, 1 << lane);
        }
    }

    /// A candidate whose object is a logging object (doubles at `OBJECT + slot`) and whose
    /// transform holds `ROWS` and `TRANSLATION`; returns `(candidate, object)`.
    fn candidate(e: &mut Engine, log: &Log) -> (u32, u32) {
        let object = logging_object(e, log, OBJECT);
        let transform = e.mem.alloc(0x40);
        for (n, row) in ROWS.iter().enumerate() {
            store_vec(e, transform + 0x10 * n as u32, *row);
        }
        store_vec(e, transform + 0x30, TRANSLATION);
        let candidate = e.mem.alloc(0x10);
        e.mem.set_u32(candidate, object);
        e.mem.set_u32(candidate + 8, transform);
        (candidate, object)
    }

    /// A filter object (doubles at `FILTER + slot`) whose slot `4` answers a pointer to a
    /// byte holding `verdict`.
    fn filter(e: &mut Engine, log: &Log, verdict: u8) -> u32 {
        let object = logging_object(e, log, FILTER);
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, verdict);
        log_double(e, log, FILTER + 4, byte);
        object
    }

    struct Setup {
        this: u32,
        candidate: u32,
        hit: u32,
        result: u32,
    }

    /// A single-ray caster: the ray from `(11, 22, 33)` to `(21, 32, 43)` with filter word
    /// `0xf11f`, user word `0x77`, a hit record with normal `(1, 2, 3)` and fraction `0.5`.
    fn setup(e: &mut Engine, log: &Log, filter_object: u32, other: u32) -> Setup {
        constants(e);
        let (candidate, _) = candidate(e, log);
        let ray = e.mem.alloc(0x30);
        store_vec(e, ray, [11.0, 22.0, 33.0, 0.0]);
        store_vec(e, ray + 0x10, [21.0, 32.0, 43.0, 0.0]);
        e.mem.set_u32(ray + 0x24, 0xf11f);
        let hit = e.mem.alloc(0x60);
        store_vec(e, hit, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(hit + 0x10, 0.5);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, ray);
        e.mem.set_u32(this + 8, filter_object);
        e.mem.set_u32(this + 0xc, 0x77);
        e.mem.set_u32(this + 0x10, hit);
        e.mem.set_u32(this + 0x14, other);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        Setup {
            this,
            candidate,
            hit,
            result,
        }
    }

    /// Records the local ray of a call to the object's slot `slot` (its ray is argument
    /// `ray_argument` of the double) and sets the byte at argument 1 when `hit`.
    fn recording_double(e: &mut Engine, slot: u32, ray_argument: usize, hit: bool) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(OBJECT + slot, move |e, a| {
            let ray = a[ray_argument];
            sink.borrow_mut().push((
                a.to_vec(),
                xyz(load_vec(e, ray)),
                xyz(load_vec(e, ray + 0x10)),
                e.mem.u32(ray + 0x20),
                e.mem.u32(ray + 0x24),
            ));
            if hit {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        seen
    }

    #[test]
    fn deleting_destructor_00d979a0_frees_only_when_bit_zero_is_set() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, OPERATOR_DELETE, 0);
        let this = e.mem.alloc(0x14);
        let got = e.call(0x00d9_79a0, &args![this, 0xfeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert!(log.borrow().is_empty());
        let got = e.call(0x00d9_79a0, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn has_hit_00d979c0_reports_the_collidable_word() {
        let mut e = engine();
        let hit = e.mem.alloc(0x60);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, hit);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xee);
        e.call(0x00d9_79c0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.set_u32(hit + 0x50, 0x1234);
        e.call(0x00d9_79c0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u8(out + 1), 0);
    }

    #[test]
    fn fn_00d979c0_answers_its_result_pointer() {
        let mut e = engine();
        let hit = e.mem.alloc(0x60);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, hit);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xee);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(e.call(0x00d9_79c0, &args![this, out]).u32(), args![out][0]);
    }

    #[test]
    fn get_hit_fraction_00d979e0_loads_the_fraction_quietly() {
        let mut e = engine();
        let hit = e.mem.alloc(0x60);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, hit);
        e.mem.set_f32(hit + 0x10, 0.375);
        assert_eq!(e.call(0x00d9_79e0, &args![this]).f32(), 0.375);
        e.mem.set_u32(hit + 0x10, 0x7f80_0001);
        let quiet = e.call(0x00d9_79e0, &args![this]).f32();
        assert_eq!(quiet.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn get_hit_fractions_00d97a10_gathers_four_fractions() {
        let mut e = engine();
        let hits = e.mem.alloc(0x180);
        for lane in 0..4u32 {
            e.mem
                .set_f32(hits + 0x10 + 0x60 * lane, 0.25 * (lane + 1) as f32);
        }
        e.mem.set_u32(hits + 0x10 + 0x60 * 2, 0x7f80_0001);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, hits);
        let out = e.mem.alloc(0x10);
        e.call(0x00d9_7a10, &args![this, out]);
        assert_eq!(e.mem.f32(out), 0.25);
        assert_eq!(e.mem.f32(out + 4), 0.5);
        assert_eq!(e.mem.u32(out + 8), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 0xc), 1.0);
    }

    #[test]
    fn add_mid_phase_hit_00d97a40_does_nothing_without_an_object() {
        let mut e = engine();
        let log = new_log();
        let s = setup(&mut e, &log, 0, 0);
        e.mem.set_u32(s.candidate, 0);
        e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate]);
        assert_eq!(e.mem.u8(s.result), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d97a40_answers_its_result_pointer() {
        let mut e = engine();
        let log = new_log();
        let s = setup(&mut e, &log, 0, 0);
        e.mem.set_u32(s.candidate, 0);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate])
                .u32(),
            args![s.result][0]
        );
    }

    #[test]
    fn add_mid_phase_hit_00d97a40_stops_at_a_rejecting_filter() {
        let mut e = engine();
        let log = new_log();
        let filter_object = filter(&mut e, &log, 0);
        let s = setup(&mut e, &log, filter_object, 0);
        e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate]);
        assert_eq!(e.mem.u8(s.result), 0);
        let ray = e.mem.u32(s.this + 4);
        let asked = calls(&log, FILTER + 4);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], filter_object);
        assert_eq!(asked[0][2..], [ray, s.candidate]);
        assert!(calls(&log, OBJECT + 0x20).is_empty());
        assert!(calls(&log, OBJECT + 0x24).is_empty());
    }

    #[test]
    fn add_mid_phase_hit_00d97a40_hands_the_local_ray_to_the_second_object() {
        let mut e = engine();
        let log = new_log();
        let s = setup(&mut e, &log, 0, 0x4321);
        let seen = recording_double(&mut e, 0x24, 1, false);
        e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate]);
        assert_eq!(e.mem.u8(s.result), 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (words, from, to, filter_word, user) = &seen[0];
        assert_eq!(words[2..], [s.candidate, 0x4321]);
        assert_eq!(*from, [20.0, 30.0, 10.0]);
        assert_eq!(*to, [30.0, 40.0, 20.0]);
        assert_eq!((*filter_word, *user), (0xf11f, 0x77));
        assert!(calls(&log, OBJECT + 0x20).is_empty());
        assert_eq!(e.mem.u32(s.hit + 0x50), 0);
    }

    #[test]
    fn add_mid_phase_hit_00d97a40_records_an_accepted_hit() {
        let mut e = engine();
        let log = new_log();
        let filter_object = filter(&mut e, &log, 1);
        let s = setup(&mut e, &log, filter_object, 0);
        let seen = recording_double(&mut e, 0x20, 2, true);
        e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate]);
        assert_eq!(e.mem.u8(s.result), 1);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0].0[3], s.hit);
        assert_eq!(seen.borrow()[0].1, [20.0, 30.0, 10.0]);
        assert_eq!(e.mem.u32(s.hit + 0x50), s.candidate);
        assert_eq!(xyz(load_vec(&e, s.hit)), [3.0, 1.0, 2.0]);
        assert!(calls(&log, OBJECT + 0x24).is_empty());
    }

    #[test]
    fn add_mid_phase_hit_00d97a40_leaves_the_record_after_a_miss() {
        let mut e = engine();
        let log = new_log();
        let s = setup(&mut e, &log, 0, 0);
        let seen = recording_double(&mut e, 0x20, 2, false);
        e.call(0x00d9_7a40, &args![s.this, s.result, s.candidate]);
        assert_eq!(e.mem.u8(s.result), 0);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(e.mem.u32(s.hit + 0x50), 0);
        assert_eq!(xyz(load_vec(&e, s.hit)), [1.0, 2.0, 3.0]);
    }

    /// What the bundle caster handed to the object's slot `0x28`.
    #[derive(Default)]
    struct BundleCall {
        words: Vec<u32>,
        input: Vec<Vec4>,
        filter_and_user: (u32, u32),
        lane_mask: [u32; 4],
        initial: Vec<(f32, u32, u32, u32)>,
    }

    /// The hit fractions the double reports, and the pattern word at `+0x14 + 4 * n`.
    const FRACTIONS: [f32; 4] = [0.5, 0.25, 0.75, 1.0];

    /// Installs the slot-`0x28` double: it records the call and fills the four hit
    /// records with `FRACTIONS`, the normal `(1, 2, 3)` and the words `0x100 * hit + n`.
    fn bundle_double(e: &mut Engine) -> Rc<RefCell<BundleCall>> {
        let seen = Rc::new(RefCell::new(BundleCall::default()));
        let sink = Rc::clone(&seen);
        e.register_double(OBJECT + 0x28, move |e, a| {
            let mut call = sink.borrow_mut();
            call.words = a.to_vec();
            call.input = (0..6).map(|n| load_vec(e, a[2] + 0x10 * n)).collect();
            call.filter_and_user = (e.mem.u32(a[2] + 0x60), e.mem.u32(a[2] + 0x64));
            call.lane_mask = load_bits(e, a[4]);
            for hit in 0..4u32 {
                let record = a[3] + 0x50 * hit;
                call.initial.push((
                    e.mem.f32(record + 0x10),
                    e.mem.u32(record + 0x14),
                    e.mem.u32(record + 0x20),
                    e.mem.u32(record + 0x40),
                ));
                e.mem.set_f32(record + 0x10, FRACTIONS[hit as usize]);
                store_vec(e, record, [1.0, 2.0, 3.0, 0.0]);
                for word in 0..12u32 {
                    e.mem.set_u32(record + 0x14 + 4 * word, 0x100 * hit + word);
                }
            }
            Ret::default()
        });
        seen
    }

    /// A bundle caster: the rays `i` from `(i + 1, 10 + i, 20 + i)` to
    /// `(100 + i, 200 + i, 300 + i)` with filter word `0xf11f`, user word `0x77`, four hit
    /// records whose fractions are `held`, and the lane mask `lanes` (sign bits).
    fn bundle_setup(e: &mut Engine, log: &Log, filter_object: u32, held: [f32; 4]) -> Setup {
        constants(e);
        let (candidate, _) = candidate(e, log);
        let rays = e.mem.alloc(0xc0);
        for i in 0..4u32 {
            let f = i as f32;
            store_vec(e, rays + 0x30 * i, [f + 1.0, f + 10.0, f + 20.0, 0.0]);
            store_vec(
                e,
                rays + 0x30 * i + 0x10,
                [f + 100.0, f + 200.0, f + 300.0, 0.0],
            );
        }
        e.mem.set_u32(rays + 0x24, 0xf11f);
        let hits = e.mem.alloc(0x180);
        for (i, fraction) in held.into_iter().enumerate() {
            e.mem.set_f32(hits + 0x60 * i as u32 + 0x10, fraction);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, rays);
        e.mem.set_u32(this + 8, filter_object);
        e.mem.set_u32(this + 0xc, 0x77);
        e.mem.set_u32(this + 0x10, hits);
        Setup {
            this,
            candidate,
            hit: hits,
            result: 0,
        }
    }

    fn lane_vector(e: &mut Engine, signs: [u32; 4]) -> u32 {
        let mask = e.mem.alloc(0x10);
        store_bits(e, mask, signs);
        mask
    }

    #[test]
    fn bundle_add_mid_phase_hit_00d97c90_does_nothing_without_an_object() {
        let mut e = engine();
        let log = new_log();
        let s = bundle_setup(&mut e, &log, 0, [1.0; 4]);
        e.mem.set_u32(s.candidate, 0);
        let mask = lane_vector(&mut e, [u32::MAX; 4]);
        e.call(0x00d9_7c90, &args![s.this, s.candidate, mask]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn bundle_add_mid_phase_hit_00d97c90_casts_four_rays_and_keeps_the_nearer_hits() {
        let mut e = engine();
        let log = new_log();
        let s = bundle_setup(&mut e, &log, 0, [1.0, 0.3, 0.5, 1.0]);
        let seen = bundle_double(&mut e);
        let mask = lane_vector(&mut e, [0; 4]);
        e.call(0x00d9_7c90, &args![s.this, s.candidate, mask]);
        let call = seen.borrow();
        assert_eq!(call.words.len(), 5);
        assert_eq!(call.lane_mask, [u32::MAX; 4]);
        assert_eq!(call.filter_and_user, (0xf11f, 0x77));
        assert_eq!(call.words[4], call.words[2] + 0x70);
        assert_eq!(call.words[3], call.words[2] + 0x80);
        // The rays through the rows: (x, y, z) - (1, 2, 3) becomes (y, z, x).
        let starts: [[f32; 4]; 3] = [
            [8.0, 9.0, 10.0, 11.0],
            [17.0, 18.0, 19.0, 20.0],
            [0.0, 1.0, 2.0, 3.0],
        ];
        let ends: [[f32; 4]; 3] = [
            [198.0, 199.0, 200.0, 201.0],
            [297.0, 298.0, 299.0, 300.0],
            [99.0, 100.0, 101.0, 102.0],
        ];
        for (n, expected) in starts.iter().chain(ends.iter()).enumerate() {
            assert_eq!(call.input[n], *expected, "input vector {n}");
        }
        for initial in &call.initial {
            assert_eq!(*initial, (1.0, u32::MAX, u32::MAX, 0));
        }
        // Hit 0 (0.5 < 1.0) and hit 1 (0.25 < 0.3) are taken, hit 2 (0.75 > 0.5) and hit 3
        // (1.0) are not.
        let hits = s.hit;
        assert_eq!(e.mem.f32(hits + 0x10), 0.5);
        assert_eq!(e.mem.f32(hits + 0x60 + 0x10), 0.25);
        assert_eq!(e.mem.f32(hits + 0xc0 + 0x10), 0.5);
        assert_eq!(e.mem.f32(hits + 0x120 + 0x10), 1.0);
        for hit in 0..2u32 {
            let target = hits + 0x60 * hit;
            assert_eq!(e.mem.u32(target + 0x50), s.candidate);
            assert_eq!(xyz(load_vec(&e, target)), [3.0, 1.0, 2.0]);
            for word in 0..12u32 {
                assert_eq!(e.mem.u32(target + 0x14 + 4 * word), 0x100 * hit + word);
            }
        }
        for hit in 2..4u32 {
            let target = hits + 0x60 * hit;
            assert_eq!(e.mem.u32(target + 0x50), 0);
            assert_eq!(e.mem.u32(target + 0x14), 0);
        }
    }

    #[test]
    fn bundle_add_mid_phase_hit_00d97c90_asks_the_filter_for_the_masked_lanes_only() {
        let mut e = engine();
        let log = new_log();
        let filter_object = filter(&mut e, &log, 0);
        let s = bundle_setup(&mut e, &log, filter_object, [1.0; 4]);
        let rays = e.mem.u32(s.this + 4);
        // The filter accepts lane 1's ray and rejects the others.
        let accept = e.mem.alloc(4);
        e.mem.set_u8(accept, 1);
        let reject = e.mem.alloc(4);
        let sink = Rc::clone(&log);
        e.register_double(FILTER + 4, move |_, a| {
            sink.borrow_mut().push((FILTER + 4, a.to_vec()));
            Ret {
                eax: if a[2] == rays + 0x30 { accept } else { reject },
                ..Ret::default()
            }
        });
        let seen = bundle_double(&mut e);
        // Lanes 0, 1 and 3 are masked in.
        let mask = lane_vector(&mut e, [u32::MAX, u32::MAX, 0, u32::MAX]);
        e.call(0x00d9_7c90, &args![s.this, s.candidate, mask]);
        let asked = calls(&log, FILTER + 4);
        let rays_asked: Vec<u32> = asked.iter().map(|a| a[2]).collect();
        assert_eq!(rays_asked, vec![rays, rays + 0x30, rays + 0x90]);
        assert!(asked
            .iter()
            .all(|a| a[0] == filter_object && a[3] == s.candidate));
        assert_eq!(seen.borrow().lane_mask, [0, u32::MAX, 0, 0]);
    }

    #[test]
    fn fn_00d98300_initialises_four_hit_records() {
        let mut e = engine();
        constants(&mut e);
        let this = e.mem.alloc(0x140);
        e.mem.write(this, &[0x5a; 0x140]);
        e.call(0x00d9_8300, &args![this]);
        for hit in 0..4u32 {
            let record = this + 0x50 * hit;
            assert_eq!(e.mem.f32(record + 0x10), 1.0);
            assert_eq!(e.mem.u32(record + 0x14), u32::MAX);
            assert_eq!(e.mem.u32(record + 0x20), u32::MAX);
            assert_eq!(e.mem.u32(record + 0x40), 0);
            assert_eq!(e.mem.u32(record + 0x18), 0x5a5a_5a5a);
            assert_eq!(e.mem.u32(record + 0x44), 0x5a5a_5a5a);
        }
    }

    #[test]
    fn bundle_deleting_destructor_00d98360_frees_only_when_bit_zero_is_set() {
        let mut e = engine();
        let log = new_log();
        let allocator = logging_object(&mut e, &log, 0x0900_2000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let got = e.call(0x00d9_8360, &args![this, 0x10u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert!(log.borrow().is_empty());
        let got = e.call(0x00d9_8360, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(calls(&log, 0x0900_2008), vec![vec![allocator, this, 0x14]]);
    }
}
