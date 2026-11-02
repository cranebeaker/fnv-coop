//! `BSHavok/bhkcharacterpointcollector.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCharacterPointCollector` is the `hkpAllCdPointCollector` a character
//! controller casts with. Besides the base class's hits (`m_hits`, an
//! `hkInplaceArray<hkpRootCdPoint, 8>`: data +0x10, size +0x14, capacity
//! and flags +0x18, storage +0x20) it remembers the objects it touched
//! (+0x3A4), a state per object (+0x3B0: 0 new, 1 seen last time, 2 seen
//! again) and a root point per object (+0x3BC), and tells the touched
//! object's listener when a contact starts or ends. Its vtable is
//! `010cb300`; the base `hkpAllCdPointCollector`'s is `0104da28`.
//!
//! `hkArray` (Havok): data, size, capacity in the low 30 bits of the third
//! word with bit 31 set when the array does not own its storage. Growing
//! goes through `00c90940` (`hkarrayutil.cpp`, cdecl `(allocator, array,
//! element size)`) when the size reaches the capacity; freeing through the
//! allocator at +0x10 of the memory router (`00c85750`), slot 0x10
//! `(data, bytes)`.
//!
//! An `hkpRootCdPoint` (Xbox PDB) is 0x70 bytes: the contact position
//! (+0x00) and separating normal (+0x10), `m_rootCollidableA` (+0x20),
//! `m_shapeKeyA` (+0x24, which this collector uses as a state: 1, 2 or 0),
//! `m_rootCollidableB` (+0x48), `m_shapeKeyB` (+0x4C). An `hkpCdPoint` has
//! the contact at +0x00 and the bodies A and B at +0x20 and +0x24; an
//! `hkpCdBody` its shape at +0x00, shape key +0x04, motion +0x08 and parent
//! +0x0C; an `hkpCollidable` the signed byte offset to its owner at +0x10.
//!
//! SSE code computes lane by lane in `f32` (dot products summed as
//! `z + (y + x)`), `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`);
//! x87 comparisons follow the flag tests (NaN included), and a float copied
//! through `FLD`/`FSTP` goes through [`x87_quiet`].

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

layout! {
    /// `bhkCharacterPointCollector` (Xbox PDB), 0x3C8 bytes.
    pub struct BhkCharacterPointCollector: 0x3C8 {
        /// `hkpCdPointCollector::m_earlyOutDistance` (Xbox PDB).
        0x004 m_earlyOutDistance: f32,
        /// `m_hits` data (Xbox PDB `hkpAllCdPointCollector::m_hits`).
        0x010 m_hits: u32,
        /// `m_hits` size.
        0x014 m_hitsSize: i32,
        /// `pStartCollector` (Xbox PDB): another collector whose points
        /// remove this one's.
        0x3A0 pStartCollector: Ptr<BhkCharacterPointCollector>,
        /// `m_collidingObjects` (Xbox PDB): `hkArray<hkpWorldObject*>`.
        0x3A4 m_collidingObjects: u32,
        /// `m_collisionStates` (Xbox PDB): `hkArray` of states.
        0x3B0 m_collisionStates: u32,
        /// `m_collisionPoints` (Xbox PDB): `hkArray<hkpRootCdPoint>`.
        0x3BC m_collisionPoints: u32,
    }
}

const POINT_SIZE: u32 = 0x70;
const HITS: u32 = 0x10;
const OBJECTS: u32 = 0x3a4;
const STATES: u32 = 0x3b0;
const POINTS: u32 = 0x3bc;

/// `bhkCharacterPointCollector`'s vtable.
const VTABLE: u32 = 0x010c_b300;
/// `hkpAllCdPointCollector`'s vtable.
const BASE_VTABLE: u32 = 0x0104_da28;
/// The reset early-out distance (`float`, `0x7F7FFFEE`).
const EARLY_OUT: u32 = 0x0104_da48;
/// A `double` constant `addCdPoint` hands to `00ec9f70`.
const SLOPE_CONSTANT: u32 = 0x010c_b310;
/// The four-lane sign mask (`0x80000000`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// `0.0`, `3.0` and `0.5` (`float`): the `RSQRTSS` refinement.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The per-thread word the Havok code sets to 0x10 for its duration.
const SCOPE_WORD: u32 = 0x2b4;

/// The memory router (`00c85750`); its +0x10 is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkarrayutil.cpp` growth routine (cdecl `(allocator, array, size)`).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkreferencedobject.cpp` routines called on an object when it is added
/// to (`00c90510`) and removed from (`00c905b0`) the touched objects.
const REFERENCE_ADD: u32 = 0x00c9_0510;
const REFERENCE_REMOVE: u32 = 0x00c9_05b0;
/// `hkVector4::setRotatedDir` (Xbox PDB): `this = rotation * vector`.
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkpAllCdPointCollector::~hkpAllCdPointCollector` (Xbox PDB).
const ALL_COLLECTOR_DESTRUCT: u32 = 0x0062_11b0;
/// A CRT routine (`LIBCMT`, unnamed in the map): argument and result in
/// ST0 (the argument is a leading `f64`).
const CRT_00EC9F70: u32 = 0x00ec_9f70;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `(a - b) . normal` over four lanes, summed `z + (y + x)` (`SUBPS`,
/// `MULPS`, `ADDSS`).
fn facing(a: [f32; 4], b: [f32; 4], normal: [f32; 4]) -> f32 {
    let product = [0, 1, 2, 3].map(|i| (a[i] - b[i]) * normal[i]);
    product[2] + (product[1] + product[0])
}

fn dot(a: [f32; 4], b: [f32; 4]) -> f32 {
    let product = [0, 1, 2, 3].map(|i| a[i] * b[i]);
    product[2] + (product[1] + product[0])
}

/// The root of an `hkpCdBody` (parents at +0x0C until null).
fn root(e: &Engine, mut body: u32) -> u32 {
    loop {
        let parent = e.mem.u32(body + 0xc);
        if parent == 0 {
            return body;
        }
        body = parent;
    }
}

/// The memory router's allocator (`*(00c85750() + 0x10)`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router + 0x10)
}

/// Grows the `hkArray` at `array` when it is full (the router is asked
/// first either way).
fn reserve(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        let allocator = e.mem.u32(router + 0x10);
        e.call(RESERVE_MORE, &args![allocator, array, element]);
    }
}

/// Removes element `index` of a 0x70-byte `hkArray` by moving the last
/// one into its place.
fn remove_point(e: &mut Engine, array: u32, index: u32) {
    let size = e.mem.u32(array + 4).wrapping_sub(1);
    e.mem.set_u32(array + 4, size);
    if size != index {
        let data = e.mem.u32(array);
        copy_words(e, data + size * POINT_SIZE, data + index * POINT_SIZE, 28);
    }
}

/// Removes element `index` of a word `hkArray` the same way.
fn remove_word(e: &mut Engine, array: u32, index: u32) {
    let size = e.mem.u32(array + 4).wrapping_sub(1);
    e.mem.set_u32(array + 4, size);
    if size != index {
        let data = e.mem.u32(array);
        let last = e.mem.u32(data + 4 * size);
        e.mem.set_u32(data + 4 * index, last);
    }
}

/// Empties an `hkArray` and frees its storage if it owns it.
fn clear_array(e: &mut Engine, array: u32, element: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00cd3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops hits that lie behind another: going down from the last hit `i`
/// (to 1), it compares it with the hits `i - 2` down to 0 (the hit just
/// before `i` is not compared) and removes `i` (last hit moved into its
/// place) at the first `j` whose state (+0x24) is 1 while `i`'s is not and
/// `0 < (j.position - i.position) . j.normal`. Then, with a
/// `pStartCollector` that has hits, every one of its hits `a` (last to
/// first) removes each of this collector's hits `b` (last to first) under
/// the same test with `a` in `j`'s place. The thread's scope word (+0x2B4)
/// is 0x10 for the duration.
pub fn fn_00cd3080(e: &mut Engine, this: Ptr<BhkCharacterPointCollector>) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + SCOPE_WORD);
    e.mem.set_u32(tls + SCOPE_WORD, 0x10);
    let t = this.addr();
    let hits = t + HITS;

    let mut i = e.mem.i32(hits + 4).wrapping_sub(1);
    while i > 0 {
        if i != 1 {
            let data = e.mem.u32(hits);
            let later = data + i as u32 * POINT_SIZE;
            let mut j = i - 1;
            loop {
                j -= 1;
                let earlier = data + j as u32 * POINT_SIZE;
                if e.mem.u32(earlier + 0x24) == 1
                    && e.mem.u32(later + 0x24) != 1
                    && 0.0 < facing(lanes(e, earlier), lanes(e, later), lanes(e, earlier + 0x10))
                {
                    remove_point(e, hits, i as u32);
                    break;
                }
                if j == 0 {
                    break;
                }
            }
        }
        i -= 1;
    }

    let start = e.get(this, BhkCharacterPointCollector::pStartCollector);
    if !start.is_null() {
        let mut a = e.mem.u32(start.addr() + 0x14);
        while a != 0 {
            a -= 1;
            let mut b = e.mem.u32(hits + 4);
            while b != 0 {
                b -= 1;
                let theirs = e.mem.u32(start.addr() + 0x10) + a * POINT_SIZE;
                let ours = e.mem.u32(hits) + b * POINT_SIZE;
                if e.mem.u32(theirs + 0x24) == 1
                    && e.mem.u32(ours + 0x24) != 1
                    && 0.0 < facing(lanes(e, theirs), lanes(e, ours), lanes(e, theirs + 0x10))
                {
                    remove_point(e, hits, b);
                }
            }
        }
    }
    e.mem.set_u32(tls + SCOPE_WORD, saved);
}

// Translated from 00cd3290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterPointCollector::reset` (Xbox PDB): empties the hits and
/// sets the early-out distance back (`0104da48`).
pub fn bhk_character_point_collector_reset(e: &mut Engine, this: Ptr<BhkCharacterPointCollector>) {
    let distance = x87_quiet(e.global::<f32>(EARLY_OUT));
    e.set(this, BhkCharacterPointCollector::m_hitsSize, 0);
    e.set(
        this,
        BhkCharacterPointCollector::m_earlyOutDistance,
        distance,
    );
}

// Translated from 00cd32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterPointCollector::handleCollisions` (Xbox PDB): resets the
/// hits, then tells each touched object's listener about its contact: for
/// an object whose word +0x2C has 0x0E or 0x10 in its low 7 bits, whose
/// point's root collidable B has an owner (its +0x08) with an object at
/// +0x350 that has a listener at +0x20, the listener's slot 0x1C (state 0,
/// new) or 0x20 (state 1, gone) gets the point. Then, last to first, an
/// object still in state 1 is released (`00c905b0`) and removed with its
/// point and state (last element moved into its place); any other state
/// becomes 1.
pub fn bhk_character_point_collector_handle_collisions(
    e: &mut Engine,
    this: Ptr<BhkCharacterPointCollector>,
) {
    let t = this.addr();
    bhk_character_point_collector_reset(e, this);
    let mut k = 0i32;
    while k < e.mem.i32(t + STATES + 4) {
        let index = k as u32;
        let object = e.mem.u32(e.mem.u32(t + OBJECTS) + 4 * index);
        let point = e.mem.u32(t + POINTS) + index * POINT_SIZE;
        let kind = e.mem.u32(object + 0x2c) & 0x7f;
        let collidable = e.mem.u32(point + 0x48);
        let offset = e.mem.i8(collidable + 0x10) as i32 as u32;
        let owner_at = offset.wrapping_add(collidable);
        let owner = if owner_at == 0 {
            0
        } else {
            e.mem.u32(owner_at + 8)
        };
        if owner != 0 {
            let holder = e.mem.u32(owner + 0x350);
            if holder != 0 && (kind == 0x0e || kind == 0x10) {
                let listener = e.mem.u32(holder + 0x20);
                if listener != 0 {
                    let state = e.mem.u32(e.mem.u32(t + STATES) + 4 * index);
                    if state == 0 {
                        e.vcall(listener, 0x1c, &args![point]);
                    } else if state == 1 {
                        e.vcall(listener, 0x20, &args![point]);
                    }
                }
            }
        }
        k += 1;
    }

    let mut k = e.mem.i32(t + STATES + 4).wrapping_sub(1);
    while k >= 0 {
        let index = k as u32;
        let state_at = e.mem.u32(t + STATES) + 4 * index;
        if e.mem.u32(state_at) == 1 {
            let object = e.mem.u32(e.mem.u32(t + OBJECTS) + 4 * index);
            e.call(REFERENCE_REMOVE, &args![object]);
            remove_word(e, t + OBJECTS, index);
            remove_point(e, t + POINTS, index);
            remove_word(e, t + STATES, index);
        } else {
            e.mem.set_u32(state_at, 1);
        }
        k -= 1;
    }
}

// Translated from 00cd3410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterPointCollector::bhkCharacterPointCollector` (Xbox PDB):
/// the base collector (in-place storage of 8 hits at +0x20, early-out
/// distance `0104da48`), `pStartCollector` = `start`, the three arrays
/// empty. Returns `this`.
pub fn bhk_character_point_collector_ctor(
    e: &mut Engine,
    this: Ptr<BhkCharacterPointCollector>,
    start: Ptr<BhkCharacterPointCollector>,
) -> Ptr<BhkCharacterPointCollector> {
    let t = this.addr();
    let distance = x87_quiet(e.global::<f32>(EARLY_OUT));
    e.set(
        this,
        BhkCharacterPointCollector::m_earlyOutDistance,
        distance,
    );
    e.mem.set_u32(t, BASE_VTABLE);
    e.mem.set_u32(t + 0x18, 0x8000_0008);
    e.mem.set_u32(t + HITS, t + 0x20);
    e.set(this, BhkCharacterPointCollector::pStartCollector, start);
    e.mem.set_u32(t, VTABLE);
    for array in [OBJECTS, STATES, POINTS] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
    e.set(this, BhkCharacterPointCollector::m_hitsSize, 0);
    this
}

// Translated from 00cd3490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd3490 - the FS:[0] exception frame (its unwind states clear the arrays) is not translated
/// `bhkCharacterPointCollector::~bhkCharacterPointCollector` (Xbox PDB):
/// releases every touched object (`00c905b0`), empties and frees the
/// objects, points and states arrays (then the points, states and objects
/// again, as the inlined member destructors do), and runs the base
/// destructor.
pub fn bhk_character_point_collector_dtor(e: &mut Engine, this: Ptr<BhkCharacterPointCollector>) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    let mut i = 0i32;
    while i < e.mem.i32(t + OBJECTS + 4) {
        let object = e.mem.u32(e.mem.u32(t + OBJECTS) + 4 * i as u32);
        e.call(REFERENCE_REMOVE, &args![object]);
        i += 1;
    }
    clear_array(e, t + OBJECTS, 4);
    clear_array(e, t + POINTS, POINT_SIZE);
    clear_array(e, t + STATES, 4);
    clear_array(e, t + POINTS, POINT_SIZE);
    clear_array(e, t + STATES, 4);
    clear_array(e, t + OBJECTS, 4);
    e.call(ALL_COLLECTOR_DESTRUCT, &args![this]);
}

// Translated from 00cd36a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd36a0 - the /GS cookie check 00ec408c is not modeled, and the 16 uninitialized stack words the exe copies into a new root point's m_shapeKeysA/B (+0x28, +0x50) are zeros here
/// `bhkCharacterPointCollector::addCdPoint` (Xbox PDB): records the object
/// of body B (the owner of its root collidable) and keeps the contact.
///
/// A known object in state 1 goes to state 2. A new one is added
/// (`00c90510` on it) with its root point (contact, root collidables and
/// shape keys of A and B) and state 0. Then, if the object's byte +0x28 is
/// 1 or its word +0x2C has 0x1E in the low 7 bits, the contact is appended
/// to the hits with state 0 (+0x24); when body B's shape has type 3 (its
/// word +0x0C) the state becomes 2, or 1 when the triangle's normal
/// (shape +0x20/+0x30/+0x40, cross product normalized with the refined
/// `RSQRTSS`, rotated by body B's motion with `setRotatedDir`, turned to
/// face the contact normal) has a dot product with the contact normal above
/// `00ec9f70(010cb310)`; then the hits are filtered (`00cd3080`).
pub fn bhk_character_point_collector_add_cd_point(
    e: &mut Engine,
    this: Ptr<BhkCharacterPointCollector>,
    event: u32,
) {
    let t = this.addr();
    let body_b = e.mem.u32(event + 0x24);
    let root_b = root(e, body_b);
    let offset = e.mem.i8(root_b + 0x10) as i32 as u32;
    let object = offset.wrapping_add(root_b);
    if object == 0 {
        return;
    }

    let count = e.mem.i32(t + OBJECTS + 4);
    let data = e.mem.u32(t + OBJECTS);
    let known = (0..count.max(0) as u32).find(|&i| e.mem.u32(data + 4 * i) == object);
    if let Some(index) = known {
        let state = e.mem.u32(t + STATES) + 4 * index;
        if e.mem.u32(state) == 1 {
            e.mem.set_u32(state, 2);
        }
    } else {
        e.call(REFERENCE_ADD, &args![object]);
        reserve(e, t + OBJECTS, 4);
        let size = e.mem.u32(t + OBJECTS + 4);
        let data = e.mem.u32(t + OBJECTS);
        e.mem.set_u32(data + 4 * size, object);
        e.mem.set_u32(t + OBJECTS + 4, size.wrapping_add(1));

        // The contact is copied to the stack before the array grows.
        let contact = [0u32, 1, 2, 3, 4, 5, 6, 7].map(|i| e.mem.u32(event + 4 * i));
        let body_a = e.mem.u32(event + 0x20);
        let root_a = root(e, body_a);
        let key_a = e.mem.u32(body_a + 4);
        let body_b = e.mem.u32(event + 0x24);
        let root_b = root(e, body_b);
        let key_b = e.mem.u32(body_b + 4);
        reserve(e, t + POINTS, POINT_SIZE);
        let size = e.mem.u32(t + POINTS + 4);
        let point = (size.wrapping_mul(POINT_SIZE)).wrapping_add(e.mem.u32(t + POINTS));
        if point != 0 {
            for (i, word) in (0u32..).zip(contact) {
                e.mem.set_u32(point + 4 * i, word);
            }
            e.mem.set_u32(point + 0x20, root_a);
            e.mem.set_u32(point + 0x24, key_a);
            e.with_stack(0x40, |e, keys| {
                copy_words(e, keys.addr(), point + 0x28, 8);
                e.mem.set_u32(point + 0x48, root_b);
                e.mem.set_u32(point + 0x4c, key_b);
                copy_words(e, keys.addr() + 0x20, point + 0x50, 8);
            });
        }
        let size = e.mem.u32(t + POINTS + 4);
        e.mem.set_u32(t + POINTS + 4, size.wrapping_add(1));

        reserve(e, t + STATES, 4);
        let size = e.mem.u32(t + STATES + 4);
        let state = e.mem.u32(t + STATES).wrapping_add(4 * size);
        if state != 0 {
            e.mem.set_u32(state, 0);
        }
        e.mem.set_u32(t + STATES + 4, size.wrapping_add(1));
    }

    if e.mem.u8(object + 0x28) != 1 && e.mem.u32(object + 0x2c) & 0x7f != 0x1e {
        return;
    }
    reserve(e, t + HITS, POINT_SIZE);
    let size = e.mem.u32(t + HITS + 4);
    let hit = size
        .wrapping_mul(POINT_SIZE)
        .wrapping_add(e.mem.u32(t + HITS));
    e.mem.set_u32(t + HITS + 4, size.wrapping_add(1));
    copy_words(e, event, hit, 8);
    let root_a = root(e, e.mem.u32(event + 0x20));
    e.mem.set_u32(hit + 0x20, root_a);
    let body_b = e.mem.u32(event + 0x24);
    let root_b = root(e, body_b);
    let constant: f64 = e.global(SLOPE_CONSTANT);
    e.mem.set_u32(hit + 0x48, root_b);
    e.mem.set_u32(hit + 0x24, 0);
    let key_b = e.mem.u32(e.mem.u32(event + 0x24) + 4);
    e.mem.set_u32(hit + 0x4c, key_b);
    let limit = e.call(CRT_00EC9F70, &args![constant]).f64() as f32;
    let limit = x87_quiet(limit);

    let body_b = e.mem.u32(event + 0x24);
    let shape = e.mem.u32(body_b);
    if e.mem.u32(shape + 0xc) != 3 {
        return;
    }
    e.mem.set_u32(hit + 0x24, 2);
    let v0 = lanes(e, shape + 0x20);
    let v1 = lanes(e, shape + 0x30);
    let v2 = lanes(e, shape + 0x40);
    let a = [0, 1, 2, 3].map(|i| v1[i] - v0[i]);
    let b = [0, 1, 2, 3].map(|i| v2[i] - v1[i]);
    // `SHUFPS 0xC9` is (y, z, x, w), `0xD2` is (z, x, y, w). The w lane is
    // `w*w - w*w` as the SSE code computes it (NaN, not 0, for an infinite w).
    #[allow(clippy::eq_op)]
    let normal = [
        b[2] * a[1] - b[1] * a[2],
        b[0] * a[2] - b[2] * a[0],
        b[1] * a[0] - b[0] * a[1],
        b[3] * a[3] - b[3] * a[3],
    ];
    let sum = normal[2] * normal[2] + (normal[1] * normal[1] + normal[0] * normal[0]);
    let estimate = rsqrt_estimate(sum);
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let refined = (half * estimate) * (three - (sum * estimate) * estimate);
    let scale = if sum == zero { 0.0 } else { refined };
    let scaled = normal.map(|lane| scale * lane);
    let motion = e.mem.u32(body_b + 8);
    let contact_normal = lanes(e, hit + 0x10);
    let rotated = e.with_stack(0x10, |e, local| {
        for (i, lane) in scaled.into_iter().enumerate() {
            e.mem.set_f32(local.addr() + 4 * i as u32, lane);
        }
        e.call(SET_ROTATED_DIR, &args![local, motion, local]);
        lanes(e, local.addr())
    });
    // `FLDZ`, `FCOMP`, `TEST AH,0x41`: negated only when `0 > dot`.
    let mut facing_normal = rotated;
    if 0.0 > dot(rotated, contact_normal) {
        let sign = lanes(e, SIGN_MASK);
        facing_normal =
            [0, 1, 2, 3].map(|i| f32::from_bits(rotated[i].to_bits() ^ sign[i].to_bits()));
    }
    let alignment = dot(facing_normal, contact_normal);
    // `TEST AH,0x5` / `JP`: only a strict `limit < alignment`.
    if limit < alignment {
        e.mem.set_u32(hit + 0x24, 1);
    }
    fn_00cd3080(e, this);
}

// Translated from 00cd3ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterPointCollector::`scalar deleting destructor'` (Xbox PDB):
/// destroys the collector and, when bit 0 of `flags` is set and `this` is
/// not null, frees 0x3A0 bytes through the allocator's slot 8. Returns
/// `this`.
pub fn bhk_character_point_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkCharacterPointCollector>,
    flags: u8,
) -> Ptr<BhkCharacterPointCollector> {
    bhk_character_point_collector_dtor(e, this);
    if flags & 1 != 0 && !this.is_null() {
        let allocator = allocator(e);
        e.vcall(allocator, 8, &args![this, 0x3a0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd3080, fn_00cd3080(Ptr<BhkCharacterPointCollector>)),
        entry!(
            0x00cd3290,
            bhk_character_point_collector_reset(Ptr<BhkCharacterPointCollector>)
        ),
        entry!(
            0x00cd32b0,
            bhk_character_point_collector_handle_collisions(Ptr<BhkCharacterPointCollector>)
        ),
        entry!(
            0x00cd3410,
            bhk_character_point_collector_ctor(
                Ptr<BhkCharacterPointCollector>,
                Ptr<BhkCharacterPointCollector>,
            ) -> Ptr<BhkCharacterPointCollector>
        ),
        entry!(
            0x00cd3490,
            bhk_character_point_collector_dtor(Ptr<BhkCharacterPointCollector>)
        ),
        entry!(
            0x00cd36a0,
            bhk_character_point_collector_add_cd_point(Ptr<BhkCharacterPointCollector>, u32)
        ),
        entry!(
            0x00cd3ad0,
            bhk_character_point_collector_scalar_deleting_destructor(
                Ptr<BhkCharacterPointCollector>,
                u8,
            ) -> Ptr<
                BhkCharacterPointCollector,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SCRATCH: u32 = 0x0600_0000;
    const ROUTER: u32 = 0x0610_0000;
    const ALLOCATOR: u32 = ROUTER + 0x100;
    const ALLOCATOR_TABLE: u32 = ROUTER + 0x200;
    const ALLOCATOR_FREE_SIZED: u32 = 0x0700_0008;
    const ALLOCATOR_FREE: u32 = 0x0700_0010;
    const LISTENER: u32 = 0x0620_0000;
    const LISTENER_TABLE: u32 = 0x0620_0100;

    fn engine() -> (Engine, Ptr<BhkCharacterPointCollector>) {
        let mut e = Engine::new();
        for a in [
            RESERVE_MORE,
            REFERENCE_ADD,
            REFERENCE_REMOVE,
            SET_ROTATED_DIR,
            ALL_COLLECTOR_DESTRUCT,
            CRT_00EC9F70,
            ALLOCATOR_FREE_SIZED,
            ALLOCATOR_FREE,
            0x0700_001c,
            0x0700_0020,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        for page in [
            0x0104_d000,
            0x010c_b000,
            0x010c_4000,
            0x0101_1000,
            0x0101_7000,
            0x0101_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(SCRATCH, 0x10000);
        e.map(ROUTER, 0x1000);
        e.map(LISTENER, 0x1000);
        e.mem.set_u32(EARLY_OUT, 0x7f7f_ffee);
        e.set_global(SLOPE_CONSTANT, 0.25f64);
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
        }
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.register(MEMORY_ROUTER, |_, _| ROUTER.into_ret());
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_TABLE);
        for slot in 0..0x10 {
            e.mem
                .set_u32(ALLOCATOR_TABLE + 4 * slot, 0x0700_0000 + 4 * slot);
            e.mem
                .set_u32(LISTENER_TABLE + 4 * slot, 0x0700_0000 + 4 * slot);
        }
        e.mem.set_u32(LISTENER, LISTENER_TABLE);
        let collector = e.new_object::<BhkCharacterPointCollector>();
        (e, collector)
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An `hkArray` at `array` with its own storage of `capacity` elements.
    fn array(e: &mut Engine, array: u32, element: u32, capacity: u32, size: u32) -> u32 {
        let data = e.mem.alloc(element * capacity);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
        data
    }

    /// A hit with a state, an x position and the normal (1, 0, 0).
    fn hit(e: &mut Engine, at: u32, state: u32, x: f32) {
        e.mem.set_f32(at, x);
        e.mem.set_f32(at + 0x10, 1.0);
        e.mem.set_u32(at + 0x24, state);
        e.mem.set_u32(at + 0x30, 0xabc0 + state);
    }

    #[test]
    fn reset_empties_the_hits() {
        let (mut e, c) = engine();
        e.set(c, BhkCharacterPointCollector::m_hitsSize, 5);
        e.call(0x00cd_3290, &args![c]);
        assert_eq!(e.get(c, BhkCharacterPointCollector::m_hitsSize), 0);
        assert_eq!(e.mem.u32(c.addr() + 4), 0x7f7f_ffee);
    }

    #[test]
    fn constructor_sets_up_empty_arrays() {
        let (mut e, c) = engine();
        let r = e.call(0x00cd_3410, &args![c, 0x1234u32]).u32();
        let t = c.addr();
        assert_eq!(r, t);
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(e.mem.u32(t + 4), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(t + 0x10), t + 0x20);
        assert_eq!(e.mem.u32(t + 0x18), 0x8000_0008);
        assert_eq!(e.mem.u32(t + 0x3a0), 0x1234);
        for a in [OBJECTS, STATES, POINTS] {
            assert_eq!(
                [e.mem.u32(t + a), e.mem.u32(t + a + 4), e.mem.u32(t + a + 8)],
                [0, 0, 0x8000_0000]
            );
        }
    }

    #[test]
    fn filter_removes_hits_behind_others() {
        let (mut e, c) = engine();
        let t = c.addr();
        let data = array(&mut e, t + HITS, POINT_SIZE, 8, 4);
        hit(&mut e, data, 1, 0.0);
        hit(&mut e, data + 0x70, 0, -1.0);
        hit(&mut e, data + 0xe0, 0, -5.0);
        hit(&mut e, data + 0x150, 0, 9.0);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 0x55);
        e.call(0x00cd_3080, &args![c]);
        // Hit 2 lies behind hit 0 and is replaced by hit 3; hit 1 is never
        // compared with hit 0.
        assert_eq!(e.mem.i32(t + HITS + 4), 3);
        assert_eq!(e.mem.f32(data + 0xe0), 9.0);
        assert_eq!(e.mem.f32(data + 0x70), -1.0);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 0x55);

        // The start collector's hit at x = 10 removes hits 2 and 1.
        let start = e.new_object::<BhkCharacterPointCollector>();
        let theirs = array(&mut e, start.addr() + HITS, POINT_SIZE, 1, 1);
        hit(&mut e, theirs, 1, 10.0);
        e.set(c, BhkCharacterPointCollector::pStartCollector, start);
        e.call(0x00cd_3080, &args![c]);
        assert_eq!(e.mem.i32(t + HITS + 4), 1);
        assert_eq!(e.mem.f32(data), 0.0);

        // A NaN dot product removes nothing.
        e.mem.set_u32(t + HITS + 4, 2);
        hit(&mut e, data + 0x70, 0, f32::NAN);
        e.call(0x00cd_3080, &args![c]);
        assert_eq!(e.mem.i32(t + HITS + 4), 2);
    }

    #[test]
    fn handle_collisions_reports_and_drops_departed_objects() {
        let (mut e, c) = engine();
        let t = c.addr();
        e.set(c, BhkCharacterPointCollector::m_hitsSize, 3);
        let objects = array(&mut e, t + OBJECTS, 4, 2, 2);
        let states = array(&mut e, t + STATES, 4, 2, 2);
        let points = array(&mut e, t + POINTS, POINT_SIZE, 2, 2);
        let (first, second) = (SCRATCH + 0x100, SCRATCH + 0x200);
        e.mem.set_u32(objects, first);
        e.mem.set_u32(objects + 4, second);
        e.mem.set_u32(first + 0x2c, 0x0e);
        e.mem.set_u32(second + 0x2c, 0x90);
        e.mem.set_u32(states, 1);
        e.mem.set_u32(states + 4, 0);
        let collidable = SCRATCH + 0x300;
        e.mem.set_u8(collidable + 0x10, 0x20);
        e.mem.set_u32(collidable + 0x28, SCRATCH + 0x400);
        e.mem.set_u32(SCRATCH + 0x750, SCRATCH + 0x800);
        e.mem.set_u32(SCRATCH + 0x820, LISTENER);
        for i in 0..2 {
            e.mem.set_u32(points + POINT_SIZE * i + 0x48, collidable);
            e.mem.set_u32(points + POINT_SIZE * i + 0x60, 0x100 + i);
        }
        let calls: Calls = Rc::default();
        for slot in [0x1c, 0x20] {
            let calls = calls.clone();
            e.register_double(0x0700_0000 + slot, move |_, a| {
                calls.borrow_mut().push((slot, a.to_vec()));
                Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        e.call(0x00cd_32b0, &args![c]);
        assert_eq!(e.get(c, BhkCharacterPointCollector::m_hitsSize), 0);
        assert_eq!(
            *calls.borrow(),
            vec![
                (0x20, vec![LISTENER, points]),
                (0x1c, vec![LISTENER, points + POINT_SIZE])
            ]
        );
        // Object 1 becomes state 1; object 0 (state 1) is released and
        // replaced by object 1.
        assert_eq!(logged(&e, REFERENCE_REMOVE), vec![vec![first]]);
        assert_eq!(e.mem.u32(t + OBJECTS + 4), 1);
        assert_eq!(e.mem.u32(objects), second);
        assert_eq!(e.mem.u32(t + POINTS + 4), 1);
        assert_eq!(e.mem.u32(points + 0x60), 0x101);
        assert_eq!(e.mem.u32(t + STATES + 4), 1);
        assert_eq!(e.mem.u32(states), 1);
    }

    #[test]
    fn destructor_releases_and_frees_the_arrays() {
        let (mut e, c) = engine();
        let t = c.addr();
        let objects = array(&mut e, t + OBJECTS, 4, 2, 2);
        e.mem.set_u32(objects, 0x11);
        e.mem.set_u32(objects + 4, 0x22);
        let points = array(&mut e, t + POINTS, POINT_SIZE, 3, 1);
        e.mem.set_u32(t + STATES + 8, 0x8000_0004);
        e.call_log = Some(vec![]);
        e.call(0x00cd_3490, &args![c]);
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(logged(&e, REFERENCE_REMOVE), vec![vec![0x11], vec![0x22]]);
        assert_eq!(
            logged(&e, ALLOCATOR_FREE),
            vec![
                vec![ALLOCATOR, objects, 8],
                vec![ALLOCATOR, points, 3 * 0x70]
            ]
        );
        for a in [OBJECTS, STATES, POINTS] {
            assert_eq!(
                [e.mem.u32(t + a), e.mem.u32(t + a + 4), e.mem.u32(t + a + 8)],
                [0, 0, 0x8000_0000]
            );
        }
        assert_eq!(logged(&e, ALL_COLLECTOR_DESTRUCT), vec![vec![t]]);
        assert_eq!(logged(&e, MEMORY_ROUTER).len(), 6);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_0() {
        let (mut e, c) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00cd_3ad0, &args![c, 0u32]);
        assert!(logged(&e, ALLOCATOR_FREE_SIZED).is_empty());
        let r = e.call(0x00cd_3ad0, &args![c, 1u32]).u32();
        assert_eq!(r, c.addr());
        assert_eq!(
            logged(&e, ALLOCATOR_FREE_SIZED),
            vec![vec![ALLOCATOR, c.addr(), 0x3a0]]
        );
        assert_eq!(logged(&e, ALL_COLLECTOR_DESTRUCT).len(), 2);
    }

    /// An `addCdPoint` event: body A (no parent, key 0x11), body B (key
    /// 0x22) under a root whose owner offset is 0x40, on a triangle shape.
    const EVENT: u32 = SCRATCH + 0x1000;
    const BODY_A: u32 = SCRATCH + 0x1100;
    const BODY_B: u32 = SCRATCH + 0x1200;
    const ROOT_B: u32 = SCRATCH + 0x1300;
    const OBJECT: u32 = ROOT_B + 0x40;
    const SHAPE: u32 = SCRATCH + 0x1400;

    fn add_engine() -> (Engine, Ptr<BhkCharacterPointCollector>) {
        let (mut e, c) = engine();
        let t = c.addr();
        for (i, v) in [1.0f32, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 0.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(EVENT + 4 * i as u32, v);
        }
        e.mem.set_u32(EVENT + 0x20, BODY_A);
        e.mem.set_u32(EVENT + 0x24, BODY_B);
        e.mem.set_u32(BODY_A + 4, 0x11);
        e.mem.set_u32(BODY_B, SHAPE);
        e.mem.set_u32(BODY_B + 4, 0x22);
        e.mem.set_u32(BODY_B + 8, 0x5150);
        e.mem.set_u32(BODY_B + 0xc, ROOT_B);
        e.mem.set_u8(ROOT_B + 0x10, 0x40);
        e.mem.set_u8(OBJECT + 0x28, 1);
        e.mem.set_u32(SHAPE + 0xc, 3);
        e.mem.set_f32(SHAPE + 0x30, 1.0);
        e.mem.set_f32(SHAPE + 0x40, 1.0);
        e.mem.set_f32(SHAPE + 0x44, 1.0);
        array(&mut e, t + OBJECTS, 4, 4, 0);
        array(&mut e, t + STATES, 4, 4, 0);
        array(&mut e, t + POINTS, POINT_SIZE, 4, 0);
        e.mem.set_u32(t + HITS, t + 0x20);
        e.mem.set_u32(t + HITS + 8, 0x8000_0008);
        e.register(CRT_00EC9F70, |_, _| 0.5f64.into_ret());
        (e, c)
    }

    #[test]
    fn add_cd_point_records_a_new_object_and_a_steep_hit() {
        let (mut e, c) = add_engine();
        let t = c.addr();
        e.call_log = Some(vec![]);
        e.call(0x00cd_36a0, &args![c, EVENT]);
        assert_eq!(logged(&e, REFERENCE_ADD), vec![vec![OBJECT]]);
        assert!(logged(&e, RESERVE_MORE).is_empty());
        assert_eq!(e.mem.u32(t + OBJECTS + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(t + OBJECTS)), OBJECT);
        assert_eq!(e.mem.u32(t + STATES + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(t + STATES)), 0);
        let point = e.mem.u32(t + POINTS);
        assert_eq!(e.mem.u32(t + POINTS + 4), 1);
        assert_eq!(e.mem.f32(point + 8), 3.0);
        assert_eq!(
            [0x20, 0x24, 0x48, 0x4c].map(|o| e.mem.u32(point + o)),
            [BODY_A, 0x11, ROOT_B, 0x22]
        );
        let hit = t + 0x20;
        assert_eq!(e.get(c, BhkCharacterPointCollector::m_hitsSize), 1);
        assert_eq!(e.mem.f32(hit + 0x18), 1.0);
        assert_eq!(
            [0x20, 0x48, 0x4c].map(|o| e.mem.u32(hit + o)),
            [BODY_A, ROOT_B, 0x22]
        );
        let crt = logged(&e, CRT_00EC9F70);
        assert_eq!(crt, vec![args![0.25f64]]);
        // The triangle normal (0, 0, 1) faces the contact normal past 0.5.
        let rotated = logged(&e, SET_ROTATED_DIR);
        assert_eq!(rotated[0][1], 0x5150);
        assert_eq!(rotated[0][0], rotated[0][2]);
        assert_eq!(e.mem.u32(hit + 0x24), 1);
    }

    #[test]
    fn add_cd_point_marks_a_known_object_and_keeps_a_flat_hit_at_2() {
        let (mut e, c) = add_engine();
        let t = c.addr();
        e.mem.set_u32(t + OBJECTS + 4, 1);
        e.mem.set_u32(e.mem.u32(t + OBJECTS), OBJECT);
        e.mem.set_u32(t + STATES + 4, 1);
        e.mem.set_u32(e.mem.u32(t + STATES), 1);
        e.register(CRT_00EC9F70, |_, _| 2.0f64.into_ret());
        // The rotation flips the normal; it is turned back to the contact.
        e.register(SET_ROTATED_DIR, |e, a| {
            let z = e.mem.f32(a[0] + 8);
            e.mem.set_f32(a[0] + 8, -z);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cd_36a0, &args![c, EVENT]);
        assert!(logged(&e, REFERENCE_ADD).is_empty());
        assert_eq!(e.mem.u32(t + OBJECTS + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(t + STATES)), 2);
        assert_eq!(e.mem.u32(t + 0x20 + 0x24), 2);

        // Without the object flags no hit is kept.
        e.mem.set_u8(OBJECT + 0x28, 0);
        e.mem.set_u32(OBJECT + 0x2c, 0x1f);
        e.call(0x00cd_36a0, &args![c, EVENT]);
        assert_eq!(e.get(c, BhkCharacterPointCollector::m_hitsSize), 1);
        // With 0x1E in the low bits it is.
        e.mem.set_u32(OBJECT + 0x2c, 0x9e);
        e.mem.set_u32(SHAPE + 0xc, 2);
        e.call(0x00cd_36a0, &args![c, EVENT]);
        assert_eq!(e.get(c, BhkCharacterPointCollector::m_hitsSize), 2);
        assert_eq!(e.mem.u32(t + 0x20 + 0x70 + 0x24), 0);
    }

    #[test]
    fn add_cd_point_grows_a_full_array() {
        let (mut e, c) = add_engine();
        let t = c.addr();
        e.mem.set_u32(t + OBJECTS + 8, 0);
        e.register(RESERVE_MORE, |e, a| {
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cd_36a0, &args![c, EVENT]);
        assert_eq!(
            logged(&e, RESERVE_MORE),
            vec![vec![ALLOCATOR, t + OBJECTS, 4]]
        );
        assert_eq!(e.mem.u32(e.mem.u32(t + OBJECTS)), OBJECT);
    }
}
