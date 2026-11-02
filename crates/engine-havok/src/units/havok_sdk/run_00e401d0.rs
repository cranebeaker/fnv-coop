//! `Havok SDK/run_00e401d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, load_vec, mulps, shufps, store_vec, subps, Vec4,
};

/// `hkQuaternion::setMul` (Xbox PDB, `00c66320`): thiscall, `this` is the result, then the two
/// operands, `RET 8`.
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The 16-byte vector added to the squared quaternion scalar (`010c71e0`).
const ROTATION_CONSTANT: u32 = 0x010c_71e0;
/// `hkaAnimation::hkaAnimation` copy constructor (Xbox PDB, `00e2f500`): thiscall, one stack
/// word (the source).
const BASE_ANIMATION_COPY: u32 = 0x00e2_f500;
/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Virtual slots of the heap allocator: allocate (`this`, pointer to the size, which the
/// allocator may raise) and free (`this`, pointer, size).
const ALLOCATE_SLOT: u32 = 0xc;
const FREE_SLOT: u32 = 0x10;

/// `v` rotated by the quaternion `q` and doubled the way the loop of `00e401d0` computes it:
/// `2 * (cross(q, v) * q.w + (q.w * q.w + c) * v + dot(q, v) * q) + t`, `c` being the vector at
/// `010c71e0` and `t` the translation, with every lane operation in the exe's operand order.
fn rotate_and_translate(e: &Engine, rotation: Vec4, v: Vec4, translation: Vec4) -> Vec4 {
    let product = mulps(rotation, v);
    let partial = addss(shufps(product, product, 0x55), product);
    let dot = addss(shufps(product, product, 0xaa), partial);
    let first = mulps(shufps(rotation, rotation, 0xc9), shufps(v, v, 0xd2));
    let second = mulps(shufps(rotation, rotation, 0xd2), shufps(v, v, 0xc9));
    let cross = subps(first, second);
    let w = shufps(rotation, rotation, 0xff);
    let cross_w = mulps(cross, w);
    let w_squared = mulps(w, w);
    let constant = load_vec(e, ROTATION_CONSTANT);
    let adjusted = mulps(addps(w_squared, constant), v);
    let along = mulps(shufps(dot, dot, 0x00), rotation);
    let total = addps(cross_w, addps(adjusted, along));
    addps(addps(total, total), translation)
}

// Translated from 00e401d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: moves the elements of `0x30` bytes (three vectors: a position, a
/// quaternion, a scale) at `*(this + 0x28)` by the transform at `transform` (a translation
/// vector at `+0`, a quaternion at `+0x10` and a scale vector at `+0x20`). There are
/// `[+0x2c] / [+0x10]` rounds (a zero stride divides by zero, a fault in the exe) and round `i`
/// handles element `[+0x10] * i + offset`: its position is rotated (see
/// [`rotate_and_translate`]), its quaternion becomes the product `hkQuaternion::setMul`
/// (`00c66320`) makes of the transform's quaternion and the old one, and its scale is multiplied
/// by the transform's scale. The exe also copies the position into a stack slot that nothing
/// reads again; that store is left out.
pub fn fn_00e401d0(e: &mut Engine, this: Ptr, offset: i32, transform: u32) {
    let this = this.addr();
    let rounds = e.mem.i32(this.wrapping_add(0x2c)) / e.mem.i32(this.wrapping_add(0x10));
    if rounds <= 0 {
        return;
    }
    e.with_stack(0x10, |e, copy| {
        let copy = copy.addr();
        for round in 0..rounds {
            let stride = e.mem.i32(this.wrapping_add(0x10));
            let array = e.mem.u32(this.wrapping_add(0x28));
            let index = stride.wrapping_mul(round).wrapping_add(offset);
            let element = (index as u32).wrapping_mul(0x30).wrapping_add(array);
            let position = load_vec(e, element);
            let quaternion = load_vec(e, element.wrapping_add(0x10));
            store_vec(e, copy, quaternion);
            let scale = load_vec(e, element.wrapping_add(0x20));
            let rotation = load_vec(e, transform.wrapping_add(0x10));
            let translation = load_vec(e, transform);
            let moved = rotate_and_translate(e, rotation, position, translation);
            store_vec(e, element, moved);
            e.call(
                QUATERNION_SET_MUL,
                &args![
                    element.wrapping_add(0x10),
                    transform.wrapping_add(0x10),
                    copy
                ],
            );
            let factor = load_vec(e, transform.wrapping_add(0x20));
            store_vec(e, element.wrapping_add(0x20), mulps(factor, scale));
        }
    });
}

/// Makes room for `count` elements of `size` bytes in the `hkArray` at `array` (`data`, `size`,
/// `capacity | flags`), the way `00e402e0` does: nothing when the capacity already holds them,
/// otherwise the old block is freed (unless the array has the "not owned" flag, the sign bit)
/// and a new one allocated; the allocator may return a larger size, which sets the capacity.
/// The capacity word is the granted size divided by `size`, rounded toward zero.
pub(crate) fn reserve_block(e: &mut Engine, allocator: u32, array: u32, count: i32, size: i32) {
    let flags = e.mem.u32(array.wrapping_add(8));
    let capacity = flags & 0x3fff_ffff;
    if (capacity as i32) >= count {
        return;
    }
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            FREE_SLOT,
            &args![data, (capacity as i32).wrapping_mul(size)],
        );
    }
    let bytes = count.wrapping_mul(size);
    let granted = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_i32(slot, bytes);
        let block = e.vcall(allocator, ALLOCATE_SLOT, &args![slot]).u32();
        e.mem.set_u32(array, block);
        e.mem.i32(slot)
    });
    e.mem.set_i32(array.wrapping_add(8), granted / size);
}

// Translated from 00e402e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: copy constructor of an object made of the base copy (`00e2f500`, run on
/// `this` with `source`) and two arrays. The vtable `010e5474` is stored and both `hkArray`s
/// (`+0x28` of 0x30-byte elements, `+0x34` of floats) start empty and unowned
/// (`capacity | 0x80000000`); each is then given room for the source's elements (counts at
/// `source + 0x2c` and `source + 0x38`) through the allocator at `+0x10` of the router
/// (`00c85750`; the second array asks the router again) and filled from the source. The floats
/// go through the x87 stack one by one (a signalling NaN is quieted), the exe copying four at a
/// time and then the rest; the result is the same as one loop. Returns `this`.
pub fn fn_00e402e0(e: &mut Engine, this: Ptr, source: u32) -> u32 {
    let this = this.addr();
    e.call(BASE_ANIMATION_COPY, &args![this, source]);
    e.mem.set_u32(this, 0x010e_5474);
    e.mem.set_u32(this.wrapping_add(0x28), 0);
    e.mem.set_u32(this.wrapping_add(0x2c), 0);
    e.mem.set_u32(this.wrapping_add(0x30), 0x8000_0000);
    e.mem.set_u32(this.wrapping_add(0x34), 0);
    e.mem.set_u32(this.wrapping_add(0x38), 0);
    e.mem.set_u32(this.wrapping_add(0x3c), 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let count = e.mem.i32(source.wrapping_add(0x2c));
    reserve_block(e, allocator, this.wrapping_add(0x28), count, 0x30);
    // The count is read again after the allocator ran.
    let count = e.mem.i32(source.wrapping_add(0x2c));
    let target = e.mem.u32(this.wrapping_add(0x28));
    e.mem.set_i32(this.wrapping_add(0x2c), count);
    let origin = e.mem.u32(source.wrapping_add(0x28));
    for element in 0..count.max(0) as u32 {
        let from = origin.wrapping_add(element.wrapping_mul(0x30));
        let to = target.wrapping_add(element.wrapping_mul(0x30));
        for row in 0..3u32 {
            let value = load_vec(e, from.wrapping_add(16 * row));
            store_vec(e, to.wrapping_add(16 * row), value);
        }
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let count = e.mem.i32(source.wrapping_add(0x38));
    reserve_block(e, allocator, this.wrapping_add(0x34), count, 4);
    let count = e.mem.i32(source.wrapping_add(0x38));
    let target = e.mem.u32(this.wrapping_add(0x34));
    e.mem.set_i32(this.wrapping_add(0x38), count);
    let origin = e.mem.u32(source.wrapping_add(0x34));
    for element in 0..count.max(0) as u32 {
        let bits = e.mem.u32(origin.wrapping_add(4 * element));
        e.mem
            .set_u32(target.wrapping_add(4 * element), x87_quiet_bits(bits));
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_01d0, fn_00e401d0(Ptr, i32, u32)),
        entry!(0x00e4_02e0, fn_00e402e0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The setMul double saw: `this`, the two operand pointers and the second operand's vector.
    type MulCalls = Rc<RefCell<Vec<(Vec<u32>, Vec4)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn mul_double(e: &mut Engine) -> MulCalls {
        let seen: MulCalls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            log.borrow_mut().push((a.to_vec(), load_vec(e, a[2])));
            store_vec(e, a[0], [7.0, 7.0, 7.0, 7.0]);
            Ret::default()
        });
        seen
    }

    /// An object with `elements` elements at stride `stride`: the element index `i` has the
    /// position `(i, 2, 3, 4)`, a quaternion `(0, 0, i, 1)` and the scale `(2, 2, 2, 2)`.
    fn element_array(e: &mut Engine, stride: i32, total: i32, elements: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x30 * elements);
        for index in 0..elements {
            let at = array + 0x30 * index;
            store_vec(e, at, [index as f32, 2.0, 3.0, 4.0]);
            store_vec(e, at + 0x10, [0.0, 0.0, index as f32, 1.0]);
            store_vec(e, at + 0x20, [2.0; 4]);
        }
        e.mem.set_i32(this + 0x10, stride);
        e.mem.set_i32(this + 0x2c, total);
        e.mem.set_u32(this + 0x28, array);
        this
    }

    fn transform(e: &mut Engine, rotation: Vec4) -> u32 {
        let transform = e.mem.alloc(0x30);
        store_vec(e, transform, [10.0, 20.0, 30.0, 40.0]);
        store_vec(e, transform + 0x10, rotation);
        store_vec(e, transform + 0x20, [3.0, 4.0, 5.0, 6.0]);
        transform
    }

    #[test]
    fn identity_rotation_only_translates_the_position() {
        let mut e = engine();
        store_vec(&mut e, ROTATION_CONSTANT, [-0.5; 4]);
        let seen = mul_double(&mut e);
        let this = element_array(&mut e, 2, 6, 6);
        let transform = transform(&mut e, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00e4_01d0, &args![this, 1i32, transform]);
        let array = e.mem.u32(this + 0x28);
        // Elements 1, 3 and 5 are handled (stride 2, offset 1, 3 rounds).
        for (index, handled) in [(0u32, false), (1, true), (2, false), (3, true), (5, true)] {
            let at = array + 0x30 * index;
            if handled {
                let expected = [index as f32 + 10.0, 22.0, 33.0, 44.0];
                assert_eq!(load_vec(&e, at), expected);
                assert_eq!(load_vec(&e, at + 0x10), [7.0; 4]);
                assert_eq!(load_vec(&e, at + 0x20), [6.0, 8.0, 10.0, 12.0]);
            } else {
                assert_eq!(load_vec(&e, at), [index as f32, 2.0, 3.0, 4.0]);
            }
        }
        let calls = seen.borrow();
        assert_eq!(calls.len(), 3);
        assert_eq!(
            calls[0].0,
            vec![array + 0x30 + 0x10, transform + 0x10, calls[0].0[2]]
        );
        // The second operand is a copy of the element's old quaternion.
        assert_eq!(calls[1].1, [0.0, 0.0, 3.0, 1.0]);
    }

    #[test]
    fn half_turn_about_z_negates_x_y_and_w() {
        let mut e = engine();
        store_vec(&mut e, ROTATION_CONSTANT, [-0.5; 4]);
        mul_double(&mut e);
        let this = element_array(&mut e, 1, 1, 1);
        let transform = transform(&mut e, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00e4_01d0, &args![this, 0i32, transform]);
        let array = e.mem.u32(this + 0x28);
        assert_eq!(load_vec(&e, array), [10.0, 18.0, 33.0, 36.0]);
    }

    #[test]
    fn no_rounds_leaves_the_elements_alone() {
        let mut e = engine();
        let seen = mul_double(&mut e);
        let this = element_array(&mut e, 4, 3, 4);
        let transform = transform(&mut e, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00e4_01d0, &args![this, 0i32, transform]);
        assert!(seen.borrow().is_empty());
        let array = e.mem.u32(this + 0x28);
        assert_eq!(load_vec(&e, array), [0.0, 2.0, 3.0, 4.0]);
    }

    /// What the allocator double saw: the slot offset and the argument words.
    type AllocatorLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A router with an allocator whose allocate returns a fresh block and grants the sizes given
    /// (first allocation, then the others).
    fn allocator(e: &mut Engine, granted: [i32; 2]) -> AllocatorLog {
        let log: AllocatorLog = Rc::new(RefCell::new(Vec::new()));
        e.mem.set_u32(0x0126_0010, 0x0126_0100);
        e.mem.set_u32(0x0126_0100, 0x0126_0200);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.put_vtable(0x0126_0200, &[0, 0, 0, 0x0ff0_0001, 0x0ff0_0002]);
        let allocations = Rc::new(RefCell::new(0usize));
        let seen = Rc::clone(&log);
        e.register_double(0x0ff0_0001, move |e, a| {
            let mut count = allocations.borrow_mut();
            let asked = e.mem.i32(a[1]);
            seen.borrow_mut().push((0xc, vec![asked as u32]));
            e.mem.set_i32(a[1], granted[(*count).min(1)]);
            *count += 1;
            Ret {
                eax: e.mem.alloc(asked as u32 + 64),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(0x0ff0_0002, move |_, a| {
            seen.borrow_mut().push((0x10, a[1..].to_vec()));
            Ret::default()
        });
        log
    }

    /// A source object: two elements and the floats given.
    fn source(e: &mut Engine, floats: &[u32]) -> u32 {
        let source = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0x60);
        for index in 0..2u32 {
            for row in 0..3u32 {
                let value = (index * 3 + row) as f32;
                store_vec(e, elements + 0x30 * index + 16 * row, [value; 4]);
            }
        }
        let data = e.mem.alloc(4 * floats.len() as u32 + 4);
        for (index, bits) in floats.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *bits);
        }
        e.mem.set_u32(source + 0x28, elements);
        e.mem.set_u32(source + 0x2c, 2);
        e.mem.set_u32(source + 0x34, data);
        e.mem.set_u32(source + 0x38, floats.len() as u32);
        source
    }

    #[test]
    fn copy_constructor_allocates_and_copies_both_arrays() {
        let mut e = engine();
        let log = allocator(&mut e, [0x70, 32]);
        let base: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&base);
        e.register_double(BASE_ANIMATION_COPY, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let floats: Vec<u32> = (0..6).map(|index| (index as f32).to_bits()).collect();
        let source = source(&mut e, &floats);
        let this = e.mem.alloc(0x40);
        let result = e.call(0x00e4_02e0, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(*base.borrow(), vec![vec![this, source]]);
        assert_eq!(e.mem.u32(this), 0x010e_5474);
        // Elements: asked for 0x60 bytes, granted 0x70, capacity 0x70 / 0x30 = 2.
        assert_eq!(e.mem.u32(this + 0x2c), 2);
        assert_eq!(e.mem.u32(this + 0x30), 2);
        let block = e.mem.u32(this + 0x28);
        assert_eq!(load_vec(&e, block + 0x30 + 0x20), [5.0; 4]);
        assert_eq!(load_vec(&e, block), [0.0; 4]);
        // Floats: asked for 24 bytes, granted 32, capacity 8.
        assert_eq!(e.mem.u32(this + 0x38), 6);
        assert_eq!(e.mem.u32(this + 0x3c), 8);
        let data = e.mem.u32(this + 0x34);
        let copied: Vec<u32> = (0..6).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(copied, floats);
        assert_eq!(*log.borrow(), vec![(0xc, vec![0x60]), (0xc, vec![24])]);
    }

    #[test]
    fn floats_are_quieted_and_empty_arrays_allocate_nothing() {
        let mut e = engine();
        let log = allocator(&mut e, [20, 0]);
        e.register(BASE_ANIMATION_COPY, |_, _| Ret::default());
        let floats = [0x7f80_0001, 0x3f80_0000, 0xffc0_0000, 7, 8];
        let source = source(&mut e, &floats);
        e.mem.set_u32(source + 0x2c, 0);
        let this = e.mem.alloc(0x40);
        e.call(0x00e4_02e0, &args![this, source]);
        // No elements: capacity stays 0 | flag, nothing allocated for them.
        assert_eq!(e.mem.u32(this + 0x30), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        let data = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(data), 0x7fc0_0001);
        assert_eq!(e.mem.u32(data + 8), 0xffc0_0000);
        assert_eq!(e.mem.u32(data + 16), 8);
        assert_eq!(e.mem.u32(this + 0x3c), 5);
        assert_eq!(*log.borrow(), vec![(0xc, vec![20])]);
    }

    #[test]
    fn nothing_to_copy_allocates_nothing() {
        let mut e = engine();
        let log = allocator(&mut e, [0, 0]);
        e.register(BASE_ANIMATION_COPY, |_, _| Ret::default());
        let source = source(&mut e, &[]);
        e.mem.set_u32(source + 0x2c, 0);
        let this = e.mem.alloc(0x40);
        e.call(0x00e4_02e0, &args![this, source]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
    }
}
