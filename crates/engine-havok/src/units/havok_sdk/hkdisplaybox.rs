//! `Havok SDK/hkdisplaybox.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays and the
/// geometry object come from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = the result, then `(transform,
/// vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The exe's float constant (0.0) that the fourth lane of every vertex is read from.
const FOURTH_LANE: u32 = 0x0101_1d78;
/// The exe's double constant (-1.0) a corner's half extents are multiplied by.
const EXTENT_SCALE: u32 = 0x0101_a6b0;
/// The exe's float constant (-0.0) the negated half extents are subtracted from.
const NEGATIVE_ZERO: u32 = 0x010c_4990;
/// The vtable of `hkDisplayBox`.
const VTABLE: u32 = 0x010d_8100;

/// The exe moves 16-byte vectors with `MOVAPS`, which faults on an address that is not 16-byte
/// aligned.
fn assert_aligned(address: u32) {
    assert_eq!(
        address & 0xf,
        0,
        "MOVAPS address {address:#010x} is not 16-byte aligned"
    );
}

/// A `MOVAPS` store of four words.
fn store_vector(e: &mut Engine, address: u32, lanes: [u32; 4]) {
    assert_aligned(address);
    for (index, lane) in lanes.iter().enumerate() {
        e.mem.set_u32(address + 4 * index as u32, *lane);
    }
}

/// A `MOVAPS` copy of 16 bytes.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    assert_aligned(from);
    assert_aligned(to);
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The start of all the constructors: the word `0` at `+8`, 2 at `+0x50`, 1 at `+6` (a word),
/// the 4x4 transform at `+0x10` as the identity (zero vectors with 1.0 stored at `+0x10`,
/// `+0x24` and `+0x38`), a zero vector at `+0x40` and the vtable.
fn initialise(e: &mut Engine, this: u32) {
    assert_aligned(this);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0x50, 2);
    e.mem.set_u16(this + 6, 1);
    for offset in [0x10, 0x20, 0x30] {
        store_vector(e, this + offset, [0; 4]);
    }
    for offset in [0x10, 0x24, 0x38] {
        e.mem.set_u32(this + offset, 1.0f32.to_bits());
    }
    store_vector(e, this + 0x40, [0; 4]);
    e.mem.set_u32(this, VTABLE);
}

// Translated from 00de1f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayBox::hkDisplayBox` (Xbox PDB; `this` = the display box, then the half extents
/// vector, `RET 4`): the common start (word `0` at `+8`, 2 at `+0x50`, 1 at `+6`, the identity
/// transform at `+0x10`, a zero vector at `+0x40`, the vtable `0x010D8100`), then the half
/// extents are copied to `+0x60`, first as four words and then again as one `MOVAPS` (so the
/// source must be 16-byte aligned). Answers `this`.
pub fn hk_display_box_construct(e: &mut Engine, this: Ptr, half_extents: Ptr) -> Ptr {
    initialise(e, this.addr());
    for word in 0..4 {
        let value = e.mem.u32(half_extents.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0x60 + 4 * word, value);
    }
    copy_vector(e, half_extents.addr(), this.addr() + 0x60);
    this
}

// Translated from 00de1fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayBox::hkDisplayBox_ov2` (Xbox PDB): the common start of `hkDisplayBox` followed by
/// a zero vector for the half extents (`+0x60`). Answers `this`.
pub fn hk_display_box_construct_ov2(e: &mut Engine, this: Ptr) -> Ptr {
    initialise(e, this.addr());
    store_vector(e, this.addr() + 0x60, [0; 4]);
    this
}

// Translated from 00de2010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the half extents and the transform (`this` = the display box, then `(half extents,
/// transform)`, `RET 8`; not named in the map): copies the vector to `+0x60` and the four
/// vectors of the transform to `+0x10`, `+0x20`, `+0x30` and `+0x40`. All the addresses must
/// be 16-byte aligned (`MOVAPS`).
pub fn fn_00de2010(e: &mut Engine, this: Ptr, half_extents: Ptr, transform: Ptr) {
    copy_vector(e, half_extents.addr(), this.addr() + 0x60);
    for row in 0..4 {
        copy_vector(
            e,
            transform.addr() + 0x10 * row,
            this.addr() + 0x10 * (row + 1),
        );
    }
}

/// One component of a corner, multiplied by the exe's double constant like the x87 does it:
/// loaded from a float (a signalling NaN is quieted), multiplied and stored back as a float.
fn scaled_component(bits: u32, scale: f64) -> u32 {
    ((f64::from(x87_quiet(f32::from_bits(bits))) * scale) as f32).to_bits()
}

// Translated from 00de2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayBox::getWireframeGeometry` (Xbox PDB; `this` = the display box, then `(array,
/// allocator)`, `RET 8`): makes sure the `hkArray` of 16-byte vectors (data, size, capacity
/// word) has room for 24 elements (when its capacity, the low 30 bits of the word, is below
/// 0x18 it is reserved through `hkArrayUtil::_reserve` (`00c908c0`) with the allocator, twice
/// the capacity or 0x18, whichever is larger, and the element size 0x10) and sets the size to
/// 0x18. For each of the eight corners it copies the half extents (`+0x60`) to a local
/// vector, multiplies the x component by the double at `0101a6b0` (-1.0) when bit 0 of the
/// corner number is set, the y component when bit 1 is set and the z component when bit 2 is
/// set (loaded from and stored back as floats; the others keep their bits) and puts the
/// result through `hkVector4::setTransformedPos` (`00c7f640`) with the transform at `+0x10`
/// into a local array. Then, for every corner `i` and each bit `b` of 1, 2, 4 in turn, when
/// `i < i ^ b` the vertices `i` and `i ^ b` are stored as the next two elements of the
/// array: the 12 edges. The stores are `MOVAPS`: the array data must be 16-byte aligned.
pub fn hk_display_box_get_wireframe_geometry(
    e: &mut Engine,
    this: Ptr,
    array: Ptr,
    allocator: u32,
) {
    let display = this.addr();
    let array = array.addr();
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < 0x18 {
        let wanted = capacity.wrapping_add(capacity).max(0x18);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, wanted as u32, 0x10u32],
        );
    }
    e.mem.set_u32(array + 4, 0x18);
    // The exe's frame: a working vector, then the eight transformed corners, 16-byte aligned.
    e.with_stack(0xb0, |e, frame| {
        let vector = (frame.addr() + 15) & !15;
        let corners = vector + 0x10;
        for corner in 0..8u32 {
            for word in 0..4 {
                let value = e.mem.u32(display + 0x60 + 4 * word);
                e.mem.set_u32(vector + 4 * word, value);
            }
            let scale: f64 = e.global(EXTENT_SCALE);
            for axis in 0..3 {
                if corner & (1 << axis) != 0 {
                    let bits = e.mem.u32(vector + 4 * axis);
                    e.mem
                        .set_u32(vector + 4 * axis, scaled_component(bits, scale));
                }
            }
            e.call(
                SET_TRANSFORMED_POS,
                &args![corners + 0x10 * corner, display + 0x10, vector],
            );
        }
        let mut element = 0u32;
        for first in 0..8u32 {
            let mut bit = 1u32;
            while bit < 8 {
                let second = bit ^ first;
                if first < second {
                    let data = e.mem.u32(array);
                    copy_vector(e, corners + 0x10 * first, data + 0x10 * element);
                    let data = e.mem.u32(array);
                    copy_vector(e, corners + 0x10 * second, data + 0x10 * (element + 1));
                    element += 2;
                }
                bit *= 2;
            }
        }
    });
}

/// Appends one element slot to the `hkArray` at `array` the way the exe's inlined push does:
/// asks the memory router for its allocator, and when the capacity (the low 30 bits of the
/// word at `+8`) is below the size plus one reserves twice the capacity or size plus one,
/// whichever is larger, with element size 0x10 (`00c908c0`); raises the size and answers the
/// address of the new element in the data.
fn append_slot(e: &mut Engine, array: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.u32(array + 4);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    let wanted = (size as i32).wrapping_add(1);
    if capacity < wanted {
        let new_capacity = wanted.max(capacity.wrapping_add(capacity));
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity as u32, 0x10u32],
        );
    }
    let current = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, current.wrapping_add(1));
    size.wrapping_shl(4).wrapping_add(e.mem.u32(array))
}

/// Which half extents are negated (1) for each of the eight vertices `buildGeometry`
/// appends, as `(x, y, z)`.
const BOX_VERTEX_NEGATED: [[u32; 3]; 8] = [
    [1, 0, 0],
    [0, 0, 0],
    [0, 1, 0],
    [1, 1, 0],
    [1, 0, 1],
    [0, 0, 1],
    [0, 1, 1],
    [1, 1, 1],
];

/// The 12 triangles over those vertices, each with the word -1 after the three indices.
const BOX_TRIANGLES: [[u32; 3]; 12] = [
    [3, 2, 1],
    [3, 1, 0],
    [6, 7, 4],
    [6, 4, 5],
    [4, 7, 3],
    [4, 3, 0],
    [2, 6, 5],
    [2, 5, 1],
    [7, 6, 2],
    [7, 2, 3],
    [1, 5, 4],
    [1, 4, 0],
];

// Translated from 00de2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayBox::buildGeometry` (Xbox PDB; `this` = the display box, nothing else): gets a
/// 0x18-byte geometry object from the memory router's allocator (virtual call at `+4` with
/// 0x18; when it answers null the exe goes on with a null object and faults reading its size),
/// gives it two empty arrays (vertices at `+0`, triangles at `+0xC`: data 0, size 0, capacity
/// word `0x80000000`), stores it at `this + 8`, appends the eight corner vertices (the half
/// extents at `+0x60`, a component negated as `-0.0 - value` in single precision where the
/// corner says so; fourth lane read from the float at `0101_1d78`; the stores are `MOVAPS`,
/// so the data must be 16-byte aligned), transforms every vertex in place with the transform
/// at `+0x10` (`hkVector4::setTransformedPos`, `00c7f640`) and appends the twelve triangles
/// (three vertex indices and -1).
pub fn hk_display_box_build_geometry(e: &mut Engine, this: Ptr) {
    let display = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let geometry = if block != 0 {
        e.mem.set_u32(block, 0);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0x8000_0000);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x10, 0);
        e.mem.set_u32(block + 0x14, 0x8000_0000);
        block
    } else {
        0
    };
    e.mem.set_u32(display + 8, geometry);
    for negated in BOX_VERTEX_NEGATED {
        let geometry = e.mem.u32(display + 8);
        let slot = append_slot(e, geometry);
        let mut lanes = [0u32; 4];
        for (axis, lane) in lanes.iter_mut().take(3).enumerate() {
            let value = e.mem.u32(display + 0x60 + 4 * axis as u32);
            *lane = if negated[axis] != 0 {
                let zero = f32::from_bits(e.mem.u32(NEGATIVE_ZERO));
                (zero - f32::from_bits(value)).to_bits()
            } else {
                value
            };
        }
        lanes[3] = e.mem.u32(FOURTH_LANE);
        store_vector(e, slot, lanes);
    }
    let geometry = e.mem.u32(display + 8);
    if e.mem.i32(geometry + 4) > 0 {
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let geometry = e.mem.u32(display + 8);
            let vertex = e.mem.u32(geometry) + offset;
            e.call(SET_TRANSFORMED_POS, &args![vertex, display + 0x10, vertex]);
            let geometry = e.mem.u32(display + 8);
            index += 1;
            offset += 0x10;
            if index >= e.mem.i32(geometry + 4) {
                break;
            }
        }
    }
    for triangle in BOX_TRIANGLES {
        let geometry = e.mem.u32(display + 8);
        let slot = append_slot(e, geometry + 0xc);
        e.mem.set_u32(slot, triangle[0]);
        e.mem.set_u32(slot + 4, triangle[1]);
        e.mem.set_u32(slot + 8, triangle[2]);
        e.mem.set_u32(slot + 0xc, 0xffff_ffff);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_1f50, hk_display_box_construct(Ptr, Ptr) -> Ptr),
        entry!(0x00de_1fc0, hk_display_box_construct_ov2(Ptr) -> Ptr),
        entry!(0x00de_2010, fn_00de2010(Ptr, Ptr, Ptr)),
        entry!(
            0x00de_2050,
            hk_display_box_get_wireframe_geometry(Ptr, Ptr, u32)
        ),
        entry!(0x00de_2190, hk_display_box_build_geometry(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A 16-byte aligned block of `size` zeroed bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// An engine whose constants are mapped: the fourth lane `fourth`, the scale -1.0 and
    /// the negative zero.
    fn engine(fourth: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_a000, 0x1000);
        e.map(0x010c_4000, 0x1000);
        e.mem.set_u32(FOURTH_LANE, fourth);
        e.set_global(EXTENT_SCALE, -1.0f64);
        e.mem.set_u32(NEGATIVE_ZERO, 0x8000_0000);
        e
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    fn floats(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// A box at an aligned address, its words dirtied, so the constructors' stores show.
    fn dirty_box(e: &mut Engine) -> u32 {
        let this = aligned_block(e, 0x80);
        for offset in (0..0x80).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        this
    }

    fn check_start(e: &Engine, this: u32) {
        assert_eq!(e.mem.u32(this), 0x010d_8100);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 2);
        assert_eq!(words(e, this + 0x10, 4), floats([1.0, 0.0, 0.0, 0.0]));
        assert_eq!(words(e, this + 0x20, 4), floats([0.0, 1.0, 0.0, 0.0]));
        assert_eq!(words(e, this + 0x30, 4), floats([0.0, 0.0, 1.0, 0.0]));
        assert_eq!(words(e, this + 0x40, 4), vec![0; 4]);
        // The words in between are left alone.
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
        assert_eq!(words(e, this + 0x70, 4), vec![0xdead_beef; 4]);
    }

    #[test]
    fn construct_ov2_zeroes_the_half_extents() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        assert_eq!(e.call(0x00de_1fc0, &args![this]).u32(), this);
        check_start(&e, this);
        assert_eq!(words(&e, this + 0x60, 4), vec![0; 4]);
    }

    #[test]
    fn construct_copies_the_half_extents() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let source = aligned_block(&mut e, 16);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        assert_eq!(e.call(0x00de_1f50, &args![this, source]).u32(), this);
        check_start(&e, this);
        assert_eq!(words(&e, this + 0x60, 4), vec![0x100, 0x101, 0x102, 0x103]);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn construct_faults_on_unaligned_half_extents() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let source = aligned_block(&mut e, 32) + 4;
        e.call(0x00de_1f50, &args![this, source]);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn construct_ov2_faults_on_an_unaligned_box() {
        let mut e = Engine::new();
        let this = aligned_block(&mut e, 0x90) + 8;
        e.call(0x00de_1fc0, &args![this]);
    }

    #[test]
    fn set_half_extents_and_transform_copies_the_five_vectors() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let extents = aligned_block(&mut e, 16);
        let transform = aligned_block(&mut e, 0x40);
        for i in 0..4 {
            e.mem.set_u32(extents + 4 * i, 0x10 + i);
        }
        for i in 0..16 {
            e.mem.set_u32(transform + 4 * i, 0x100 + i);
        }
        e.call(0x00de_2010, &args![this, extents, transform]);
        assert_eq!(words(&e, this + 0x60, 4), vec![0x10, 0x11, 0x12, 0x13]);
        assert_eq!(
            words(&e, this + 0x10, 16),
            (0..16).map(|i| 0x100 + i).collect::<Vec<_>>()
        );
        assert_eq!(e.mem.u32(this), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x50), 0xdead_beef);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn set_half_extents_and_transform_faults_on_an_unaligned_transform() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let extents = aligned_block(&mut e, 16);
        let transform = aligned_block(&mut e, 0x50) + 4;
        e.call(0x00de_2010, &args![this, extents, transform]);
    }

    /// A box at an aligned address with the half extents (1, 2, 3, 9).
    fn extents_box(e: &mut Engine) -> u32 {
        let this = dirty_box(e);
        for (offset, value) in [(0x60, 1.0f32), (0x64, 2.0), (0x68, 3.0), (0x6c, 9.0)] {
            e.mem.set_f32(this + offset, value);
        }
        this
    }

    /// An array header (data, size, capacity word) with `capacity` and the data an aligned
    /// block of 0x200 bytes.
    fn array_with(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = aligned_block(e, 0x200);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    /// The reserve double that swaps in a larger aligned block with the new capacity
    /// (element size 0x10), keeping the old elements.
    fn install_reserve(e: &mut Engine) {
        e.register(ARRAY_RESERVE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let grown = (e.mem.alloc(a[2] * a[3] + 16) + 15) & !15;
            for word in 0..size * 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(grown + 4 * word, value);
            }
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    fn reserved(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == ARRAY_RESERVE)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// `setTransformedPos` double: copies the vector to the result with the fourth lane
    /// replaced by `marker + call number`, and logs `(result, transform, vector, the
    /// vector's words)`.
    type TransformCall = (u32, u32, u32, Vec<u32>);
    type TransformLog = Rc<RefCell<Vec<TransformCall>>>;
    fn install_transform(e: &mut Engine, marker: u32) -> TransformLog {
        let log = TransformLog::default();
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            let vector = words(e, a[2], 4);
            let call = sink.borrow().len() as u32;
            sink.borrow_mut().push((a[0], a[1], a[2], vector.clone()));
            for (i, word) in vector.iter().take(3).enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, *word);
            }
            e.mem.set_u32(a[0] + 12, marker + call);
            Ret::default()
        });
        log
    }

    #[test]
    fn wireframe_transforms_the_eight_corners_and_stores_the_12_edges() {
        let mut e = engine(0);
        install_reserve(&mut e);
        let transforms = install_transform(&mut e, 0x1000);
        e.call_log = Some(Vec::new());
        let this = extents_box(&mut e);
        let array = array_with(&mut e, 0x8000_0000);
        e.call(0x00de_2050, &args![this, array, 0x5000u32]);
        assert_eq!(reserved(&e), vec![vec![0x5000, array, 0x18, 0x10]]);
        assert_eq!(e.mem.u32(array + 4), 0x18);
        let transforms = transforms.borrow();
        assert_eq!(transforms.len(), 8);
        for (corner, (out, matrix, vector, input)) in transforms.iter().enumerate() {
            assert_eq!(*matrix, this + 0x10);
            let x = if corner & 1 != 0 { -1.0f32 } else { 1.0 };
            let y = if corner & 2 != 0 { -2.0f32 } else { 2.0 };
            let z = if corner & 4 != 0 { -3.0f32 } else { 3.0 };
            assert_eq!(*input, floats([x, y, z, 9.0]), "corner {corner}");
            // The working vector is one block, the results are consecutive 16-byte slots.
            assert_eq!(*out, transforms[0].0 + 0x10 * corner as u32);
            assert_eq!(*vector, transforms[0].2);
            assert_eq!(*out & 0xf, 0);
        }
        // The 12 edges in the order the bit loops find them; the marker in the fourth lane
        // says which corner each vertex is.
        let edges = [
            (0, 1),
            (0, 2),
            (0, 4),
            (1, 3),
            (1, 5),
            (2, 3),
            (2, 6),
            (3, 7),
            (4, 5),
            (4, 6),
            (5, 7),
            (6, 7),
        ];
        let data = e.mem.u32(array);
        for (index, (first, second)) in edges.iter().enumerate() {
            for (side, corner) in [(0u32, *first), (1, *second)] {
                let at = data + 0x10 * (2 * index as u32 + side);
                assert_eq!(e.mem.u32(at + 12), 0x1000 + corner, "edge {index}");
                assert_eq!(
                    words(&e, at, 3),
                    transforms[corner as usize].3[..3].to_vec()
                );
            }
        }
    }

    #[test]
    fn wireframe_negates_only_flagged_components_and_quiets_them() {
        let mut e = engine(0);
        install_reserve(&mut e);
        let transforms = install_transform(&mut e, 0);
        let this = extents_box(&mut e);
        e.mem.set_u32(this + 0x60, 0x7f80_0001);
        e.mem.set_u32(this + 0x64, 0xffc0_0005);
        e.mem.set_u32(this + 0x68, 0x7fa0_0000);
        e.mem.set_u32(this + 0x6c, 0x7f80_0002);
        let array = array_with(&mut e, 0x18);
        e.call(0x00de_2050, &args![this, array, 0x5000u32]);
        let transforms = transforms.borrow();
        // Corner 0 keeps every bit, even the signalling NaNs.
        assert_eq!(
            transforms[0].3,
            vec![0x7f80_0001, 0xffc0_0005, 0x7fa0_0000, 0x7f80_0002]
        );
        // Corner 7 multiplied x, y and z: signalling NaNs come out quiet, the sign stays, the
        // fourth lane is copied untouched.
        assert_eq!(
            transforms[7].3,
            vec![0x7fc0_0001, 0xffc0_0005, 0x7fe0_0000, 0x7f80_0002]
        );
    }

    #[test]
    fn wireframe_reserves_only_when_the_capacity_is_below_24() {
        for (capacity, wanted) in [
            (0x8000_0000u32, Some(0x18u32)),
            (0x0c, Some(0x18)),
            (0x0d, Some(0x1a)),
            (0x17, Some(0x2e)),
            (0x18, None),
            (0xc000_0018, None),
        ] {
            let mut e = engine(0);
            install_reserve(&mut e);
            install_transform(&mut e, 0);
            e.call_log = Some(Vec::new());
            let this = extents_box(&mut e);
            let array = array_with(&mut e, capacity);
            e.call(0x00de_2050, &args![this, array, 0x5000u32]);
            let expected: Vec<Vec<u32>> = wanted
                .map(|w| vec![vec![0x5000, array, w, 0x10]])
                .unwrap_or_default();
            assert_eq!(reserved(&e), expected, "capacity word {capacity:#x}");
            assert_eq!(e.mem.u32(array + 4), 0x18);
        }
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn wireframe_faults_on_unaligned_data() {
        let mut e = engine(0);
        install_transform(&mut e, 0);
        let this = extents_box(&mut e);
        let array = array_with(&mut e, 0x18);
        let data = e.mem.u32(array);
        e.mem.set_u32(array, data + 4);
        e.call(0x00de_2050, &args![this, array, 0x5000u32]);
    }

    /// An engine for `buildGeometry`: a memory router whose allocator's virtual `+4` hands out
    /// blocks (or answers `allocation`), the reserve double above and the transform double.
    fn geometry_engine(allocation: Option<u32>) -> (Engine, TransformLog) {
        let mut e = engine(0);
        install_reserve(&mut e);
        let transforms = install_transform(&mut e, 0x2000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0044);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0044, move |e, _| Ret {
            eax: allocation.unwrap_or_else(|| e.mem.alloc(0x18)),
            ..Ret::default()
        });
        e.call_log = Some(Vec::new());
        (e, transforms)
    }

    #[test]
    fn build_geometry_makes_the_vertices_transforms_them_and_adds_the_triangles() {
        let (mut e, transforms) = geometry_engine(None);
        let this = extents_box(&mut e);
        e.call(0x00de_2190, &args![this]);
        let geometry = e.mem.u32(this + 8);
        assert_eq!(e.mem.block_size(geometry), Some(0x18));
        assert_eq!(e.mem.u32(geometry + 4), 8);
        assert_eq!(e.mem.u32(geometry + 0x10), 12);
        let vertices = e.mem.u32(geometry);
        // The signs per vertex, then the transform double's marker in the fourth lane (it
        // runs on every vertex in place, in order).
        let signs: [[f32; 3]; 8] = [
            [-1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
            [1.0, -2.0, 3.0],
            [-1.0, -2.0, 3.0],
            [-1.0, 2.0, -3.0],
            [1.0, 2.0, -3.0],
            [1.0, -2.0, -3.0],
            [-1.0, -2.0, -3.0],
        ];
        let transforms = transforms.borrow();
        assert_eq!(transforms.len(), 8);
        for (index, sign) in signs.iter().enumerate() {
            let at = vertices + 0x10 * index as u32;
            let (out, matrix, vector, input) = &transforms[index];
            assert_eq!((*out, *matrix, *vector), (at, this + 0x10, at));
            assert_eq!(*input, floats([sign[0], sign[1], sign[2], 0.0]));
            let mut expected = floats([sign[0], sign[1], sign[2], 0.0]);
            expected[3] = 0x2000 + index as u32;
            assert_eq!(words(&e, at, 4), expected);
        }
        let triangles = e.mem.u32(geometry + 0xc);
        let expected_triangles: [[u32; 3]; 12] = [
            [3, 2, 1],
            [3, 1, 0],
            [6, 7, 4],
            [6, 4, 5],
            [4, 7, 3],
            [4, 3, 0],
            [2, 6, 5],
            [2, 5, 1],
            [7, 6, 2],
            [7, 2, 3],
            [1, 5, 4],
            [1, 4, 0],
        ];
        for (index, want) in expected_triangles.iter().enumerate() {
            let got = words(&e, triangles + 0x10 * index as u32, 4);
            assert_eq!(got[..3], *want);
            assert_eq!(got[3], 0xffff_ffff);
        }
        let grown: Vec<(u32, u32)> = reserved(&e).iter().map(|w| (w[1], w[2])).collect();
        let mut want = Vec::new();
        for capacity in [1, 2, 4, 8] {
            want.push((geometry, capacity));
        }
        for capacity in [1, 2, 4, 8, 16] {
            want.push((geometry + 0xc, capacity));
        }
        assert_eq!(grown, want);
        assert!(reserved(&e).iter().all(|w| w[3] == 0x10));
    }

    #[test]
    fn build_geometry_negates_by_subtracting_from_negative_zero() {
        let (mut e, transforms) = geometry_engine(None);
        e.mem.set_u32(FOURTH_LANE, 0x7f80_0001);
        let this = extents_box(&mut e);
        e.mem.set_u32(this + 0x60, 0);
        e.mem.set_u32(this + 0x64, 0x7f80_0001);
        e.mem.set_u32(this + 0x68, 0xffc0_0000);
        e.call(0x00de_2190, &args![this]);
        let transforms = transforms.borrow();
        // Vertex 0 negates x: -0.0 - 0.0 is -0.0.
        assert_eq!(
            transforms[0].3,
            vec![0x8000_0000, 0x7f80_0001, 0xffc0_0000, 0x7f80_0001]
        );
        // Vertex 2 negates y: a NaN stays as it is (quieted), vertex 4 negates z: a negative
        // quiet NaN stays negative.
        assert_eq!(transforms[2].3[1], 0x7fc0_0001);
        assert_eq!(transforms[4].3[2], 0xffc0_0000);
        assert_eq!(transforms[1].3[0], 0);
    }

    #[test]
    #[should_panic]
    fn build_geometry_with_a_null_allocation_faults_like_the_exe() {
        let (mut e, _) = geometry_engine(Some(0));
        let this = extents_box(&mut e);
        e.call(0x00de_2190, &args![this]);
    }
}
