//! `Havok SDK/hkpmoppshapemediator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppShapeMediator` (vtable `010d7bb8`): the shape container it mediates for is the word
//! at +8 and the number of primitives the word at +0xC. A primitive is five words (0x14 bytes):
//! the key, a word set to 0, the lower and the upper extent along a direction (floats) and one
//! more word the code here does not use.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor stores (`010d7bb8`).
const MEDIATOR_VTABLE: u32 = 0x010d_7bb8;
/// The 16-byte sign mask (four words `0x80000000`) at `010c44d0`.
const SIGN_MASK: u32 = 0x010c_44d0;
/// Size of one primitive in bytes.
const PRIMITIVE_SIZE: u32 = 0x14;
/// Scratch for the extent search: the negated direction (16 bytes) and the 524-byte buffer
/// the container's lookup (slot `+0x14`) builds a shape in.
const EXTENT_FRAME: u32 = 0x10 + 524;

// Translated from 00dd51e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::setSplittingPlaneDirections` (Xbox PDB; `RET 8`): does nothing.
pub fn hkp_mopp_shape_mediator_set_splitting_plane_directions(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00dd51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::getPrimitiveProperties` (Xbox PDB; `RET 8`): stores the word at
/// `primitive + 4` in `properties` and returns 1.
pub fn hkp_mopp_shape_mediator_get_primitive_properties(
    e: &mut Engine,
    _unused_0: Ptr,
    primitive: Ptr,
    properties: Ptr,
) -> u32 {
    let value = e.mem.u32(primitive.addr() + 4);
    e.mem.set_u32(properties.addr(), value);
    1
}

// Translated from 00dd5210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::getPrimitives` (Xbox PDB; `RET 4`): asks the container at +8 for
/// its first key (slot `+8`) and then, as long as the count at +0xC lasts, writes the key and a
/// zero word to the next primitive of `primitives` and asks for the key after it (slot `+0xC`).
pub fn hkp_mopp_shape_mediator_get_primitives(e: &mut Engine, this: Ptr, primitives: Ptr) {
    let mediator = this.addr();
    let container = e.mem.u32(mediator + 8);
    let mut count = e.mem.u32(mediator + 0xc) as i32;
    let mut key = e.vcall(container, 8, &args![]).u32();
    let mut primitive = primitives.addr();
    while count > 0 {
        e.mem.set_u32(primitive, key);
        e.mem.set_u32(primitive + 4, 0);
        primitive = primitive.wrapping_add(PRIMITIVE_SIZE);
        let container = e.mem.u32(mediator + 8);
        key = e.vcall(container, 0xc, &args![key]).u32();
        count -= 1;
    }
}

// Translated from 00dd5260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::splitPrimitive` (Xbox PDB; `RET 0x14`): copies the five words of
/// `primitive` to `copy`. The other three arguments are not used.
pub fn hkp_mopp_shape_mediator_split_primitive(
    e: &mut Engine,
    _unused_0: Ptr,
    primitive: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    copy: Ptr,
) {
    for word in 0..5 {
        let value = e.mem.u32(primitive.addr() + 4 * word);
        e.mem.set_u32(copy.addr() + 4 * word, value);
    }
}

// Translated from 00dd5290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::getNumPrimitives` (Xbox PDB; fastcall): the count at +0xC.
pub fn hkp_mopp_shape_mediator_get_num_primitives(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc)
}

// Translated from 00dd52a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`; the constructor): sets the word at +4 to 1 (a 16-bit store at +6),
/// stores the vtable `010d7bb8` and the container, and takes the primitive count from the
/// container's slot `+4`. Returns `this`.
pub fn fn_00dd52a0(e: &mut Engine, this: Ptr, container: u32) -> Ptr {
    let mediator = this.addr();
    e.mem.set_u16(mediator + 6, 1);
    e.mem.set_u32(mediator, MEDIATOR_VTABLE);
    e.mem.set_u32(mediator + 8, container);
    let count = e.vcall(container, 4, &args![]).u32();
    e.mem.set_u32(mediator + 0xc, count);
    this
}

/// The extent of one primitive along `direction`, as `(lower, upper)`: looks the key up in the
/// container (slot `+0x14`, with the buffer in `frame`) and, when it finds a shape, takes the
/// shape's support (slot `+0xC`) along the direction as the upper extent and minus its
/// support along the negated direction (stored in `frame`) as the lower one, both rounded to
/// float. A key without a shape gives `(0.0, 0.0)`.
fn primitive_extent(
    e: &mut Engine,
    mediator: u32,
    key: u32,
    direction: u32,
    frame: u32,
) -> (f32, f32) {
    let container = e.mem.u32(mediator + 8);
    let shape = e.vcall(container, 0x14, &args![key, frame + 0x10]).u32();
    if shape == 0 {
        return (0.0, 0.0);
    }
    let upper = e.vcall(shape, 0xc, &args![direction]).f32();
    for lane in 0..4 {
        let negated = e.mem.u32(direction + 4 * lane) ^ e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(frame + 4 * lane, negated);
    }
    let lower = -e.vcall(shape, 0xc, &args![frame]).f32();
    (lower, upper)
}

// Translated from 00dd52d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::projectPrimitives` (Xbox PDB; `RET 0x18`): for each of the `count`
/// primitives (at least the first is always done) finds its extent along `direction`, stores
/// the lower and the upper extent in the primitive's third and fourth word and keeps the
/// smallest lower and the largest upper extent (a comparison with NaN changes nothing, the
/// first primitive's extents are taken as they are) for `lowest` and `highest`. The second
/// argument is not used.
pub fn hkp_mopp_shape_mediator_project_primitives(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    _unused_0: u32,
    primitives: Ptr,
    count: i32,
    lowest: Ptr,
    highest: Ptr,
) {
    let mediator = this.addr();
    let direction = direction.addr();
    let first = primitives.addr();
    e.with_stack(EXTENT_FRAME, |e, frame| {
        let frame = frame.addr();
        let key = e.mem.u32(first);
        let (lower, upper) = primitive_extent(e, mediator, key, direction, frame);
        e.mem.set_f32(first + 8, lower);
        e.mem.set_f32(first + 0xc, upper);
        let mut lowest_so_far = lower;
        let mut highest_so_far = upper;
        if count > 1 {
            let mut primitive = first;
            for _ in 1..count {
                primitive = primitive.wrapping_add(PRIMITIVE_SIZE);
                let key = e.mem.u32(primitive);
                let (lower, upper) = primitive_extent(e, mediator, key, direction, frame);
                e.mem.set_f32(primitive + 8, lower);
                e.mem.set_f32(primitive + 0xc, upper);
                if lower < lowest_so_far {
                    lowest_so_far = lower;
                }
                if highest_so_far < upper {
                    highest_so_far = upper;
                }
            }
        }
        e.mem.set_f32(lowest.addr(), lowest_so_far);
        e.mem.set_f32(highest.addr(), highest_so_far);
    });
}

// Translated from 00dd5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppShapeMediator::findExtents` (Xbox PDB; `RET 0x18`): like `projectPrimitives` but the
/// primitives are only read, not written. The second argument is not used.
pub fn hkp_mopp_shape_mediator_find_extents(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    _unused_0: u32,
    primitives: Ptr,
    count: i32,
    lowest: Ptr,
    highest: Ptr,
) {
    let mediator = this.addr();
    let direction = direction.addr();
    let first = primitives.addr();
    e.with_stack(EXTENT_FRAME, |e, frame| {
        let frame = frame.addr();
        let key = e.mem.u32(first);
        let (mut lowest_so_far, mut highest_so_far) =
            primitive_extent(e, mediator, key, direction, frame);
        if count > 1 {
            let mut primitive = first;
            for _ in 1..count {
                primitive = primitive.wrapping_add(PRIMITIVE_SIZE);
                let key = e.mem.u32(primitive);
                let (lower, upper) = primitive_extent(e, mediator, key, direction, frame);
                if lower < lowest_so_far {
                    lowest_so_far = lower;
                }
                if highest_so_far < upper {
                    highest_so_far = upper;
                }
            }
        }
        e.mem.set_f32(lowest.addr(), lowest_so_far);
        e.mem.set_f32(highest.addr(), highest_so_far);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd_51e0,
            hkp_mopp_shape_mediator_set_splitting_plane_directions(Ptr, u32, u32)
        ),
        entry!(
            0x00dd_51f0,
            hkp_mopp_shape_mediator_get_primitive_properties(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dd_5210,
            hkp_mopp_shape_mediator_get_primitives(Ptr, Ptr)
        ),
        entry!(
            0x00dd_5260,
            hkp_mopp_shape_mediator_split_primitive(Ptr, Ptr, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00dd_5290,
            hkp_mopp_shape_mediator_get_num_primitives(Ptr) -> u32
        ),
        entry!(0x00dd_52a0, fn_00dd52a0(Ptr, u32) -> Ptr),
        entry!(
            0x00dd_52d0,
            hkp_mopp_shape_mediator_project_primitives(Ptr, Ptr, u32, Ptr, i32, Ptr, Ptr)
        ),
        entry!(
            0x00dd_5440,
            hkp_mopp_shape_mediator_find_extents(Ptr, Ptr, u32, Ptr, i32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The container double: its vtable (slots `+4` count, `+8` first key, `+0xC` next key,
    /// `+0x14` lookup, which treats the key as the address of the shape).
    const COUNT_DOUBLE: u32 = 0x0900_0010;
    const FIRST_KEY_DOUBLE: u32 = 0x0900_0020;
    const NEXT_KEY_DOUBLE: u32 = 0x0900_0030;
    const LOOKUP_DOUBLE: u32 = 0x0900_0040;
    /// The shape double: its vtable slot `+0xC` gives the largest of `direction.x * low` and
    /// `direction.x * high`, the low and high at +4 and +8 of the shape.
    const SUPPORT_DOUBLE: u32 = 0x0900_0050;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.register(COUNT_DOUBLE, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register(FIRST_KEY_DOUBLE, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.register(NEXT_KEY_DOUBLE, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(LOOKUP_DOUBLE, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(SUPPORT_DOUBLE, |e, a| {
            let x = e.mem.f32(a[1]);
            let low = e.mem.f32(a[0] + 4);
            let high = e.mem.f32(a[0] + 8);
            Ret {
                st0: f64::from((x * low).max(x * high)),
                ..Ret::default()
            }
        });
        e
    }

    /// An object whose vtable has `slots` as `(offset, address)` pairs.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x20);
        for &(offset, address) in slots {
            e.mem.set_u32(vtable + offset, address);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    fn container(e: &mut Engine) -> u32 {
        object_with(
            e,
            &[
                (4, COUNT_DOUBLE),
                (8, FIRST_KEY_DOUBLE),
                (0xc, NEXT_KEY_DOUBLE),
                (0x14, LOOKUP_DOUBLE),
            ],
        )
    }

    fn shape(e: &mut Engine, low: f32, high: f32) -> u32 {
        let shape = object_with(e, &[(0xc, SUPPORT_DOUBLE)]);
        e.mem.set_f32(shape + 4, low);
        e.mem.set_f32(shape + 8, high);
        shape
    }

    /// A mediator over a container, with primitives for the given shapes (a key of 0 is a
    /// key without a shape), a direction along x and two result slots.
    fn setup(shapes: &[Option<(f32, f32)>]) -> (Engine, u32, u32, u32, u32, u32) {
        let mut e = engine();
        let container = container(&mut e);
        let mediator = e.mem.alloc(0x20);
        e.mem.set_u32(mediator + 8, container);
        let primitives = e.mem.alloc(0x14 * shapes.len() as u32);
        for (index, entry) in shapes.iter().enumerate() {
            let key = match *entry {
                Some((low, high)) => shape(&mut e, low, high),
                None => 0,
            };
            e.mem.set_u32(primitives + 0x14 * index as u32, key);
        }
        let direction = e.mem.alloc(0x10);
        e.mem.set_f32(direction, 1.0);
        let results = e.mem.alloc(8);
        (e, mediator, primitives, direction, results, container)
    }

    #[test]
    fn set_splitting_plane_directions_does_nothing() {
        let mut e = engine();
        let before = e.mem.bytes(0x0100_0000, 64);
        e.call(0x00dd_51e0, &args![0x1234u32, 1u32, 2u32]);
        assert_eq!(e.mem.bytes(0x0100_0000, 64), before);
    }

    #[test]
    fn get_primitive_properties_copies_the_second_word() {
        let mut e = engine();
        let primitive = e.mem.alloc(0x14);
        e.mem.set_u32(primitive + 4, 0xcafe);
        let properties = e.mem.alloc(4);
        let result = e.call(0x00dd_51f0, &args![0u32, primitive, properties]);
        assert_eq!(result.u32(), 1);
        assert_eq!(e.mem.u32(properties), 0xcafe);
    }

    #[test]
    fn get_primitives_walks_the_keys_of_the_container() {
        let (mut e, mediator, _, _, _, _) = setup(&[]);
        e.mem.set_u32(mediator + 0xc, 3);
        let out = e.mem.alloc(0x14 * 4);
        for word in 0..20 {
            e.mem.set_u32(out + 4 * word, 0xffff_ffff);
        }
        e.call_log = Some(Vec::new());
        e.call(0x00dd_5210, &args![mediator, out]);
        for index in 0..3 {
            assert_eq!(e.mem.u32(out + 0x14 * index), 100 + index);
            assert_eq!(e.mem.u32(out + 0x14 * index + 4), 0);
            assert_eq!(e.mem.u32(out + 0x14 * index + 8), 0xffff_ffff);
        }
        assert_eq!(e.mem.u32(out + 0x3c), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        let next: Vec<_> = log
            .iter()
            .filter(|call| call.0 == NEXT_KEY_DOUBLE)
            .map(|call| call.1[1])
            .collect();
        assert_eq!(next, [100, 101, 102]);
    }

    #[test]
    fn get_primitives_with_no_primitives_only_asks_for_the_first_key() {
        let (mut e, mediator, _, _, _, _) = setup(&[]);
        e.mem.set_u32(mediator + 0xc, 0);
        let out = e.mem.alloc(0x14);
        e.call_log = Some(Vec::new());
        e.call(0x00dd_5210, &args![mediator, out]);
        assert_eq!(e.mem.u32(out), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|call| call.0 == FIRST_KEY_DOUBLE).count(),
            1
        );
        assert!(log.iter().all(|call| call.0 != NEXT_KEY_DOUBLE));
    }

    #[test]
    fn split_primitive_copies_five_words() {
        let mut e = engine();
        let primitive = e.mem.alloc(0x14);
        let copy = e.mem.alloc(0x18);
        for word in 0..5 {
            e.mem.set_u32(primitive + 4 * word, 0x10 + word);
        }
        e.mem.set_u32(copy + 0x14, 0x77);
        e.call(0x00dd_5260, &args![0u32, primitive, 1u32, 2u32, 3u32, copy]);
        for word in 0..5 {
            assert_eq!(e.mem.u32(copy + 4 * word), 0x10 + word);
        }
        assert_eq!(e.mem.u32(copy + 0x14), 0x77);
    }

    #[test]
    fn get_num_primitives_is_the_count_word() {
        let mut e = engine();
        let mediator = e.mem.alloc(0x20);
        e.mem.set_u32(mediator + 0xc, 42);
        assert_eq!(e.call(0x00dd_5290, &args![mediator]).u32(), 42);
    }

    #[test]
    fn the_constructor_takes_the_count_from_the_container() {
        let mut e = engine();
        let container = container(&mut e);
        let mediator = e.mem.alloc(0x20);
        e.mem.set_u32(mediator + 4, 0xaaaa_bbbb);
        let result = e.call(0x00dd_52a0, &args![mediator, container]);
        assert_eq!(result.u32(), mediator);
        assert_eq!(e.mem.u32(mediator), MEDIATOR_VTABLE);
        assert_eq!(e.mem.u16(mediator + 6), 1);
        assert_eq!(e.mem.u16(mediator + 4), 0xbbbb);
        assert_eq!(e.mem.u32(mediator + 8), container);
        assert_eq!(e.mem.u32(mediator + 0xc), 5);
    }

    #[test]
    fn project_primitives_stores_the_extents_and_returns_the_range() {
        let shapes = [Some((1.0, 3.0)), Some((-2.0, 0.5)), Some((0.0, 4.0))];
        let (mut e, mediator, primitives, direction, results, _) = setup(&shapes);
        e.call(
            0x00dd_52d0,
            &args![
                mediator,
                direction,
                0u32,
                primitives,
                3i32,
                results,
                results + 4
            ],
        );
        assert_eq!(e.mem.f32(results), -2.0);
        assert_eq!(e.mem.f32(results + 4), 4.0);
        let expected = [(1.0, 3.0), (-2.0, 0.5), (0.0, 4.0)];
        for (index, (low, high)) in expected.into_iter().enumerate() {
            assert_eq!(e.mem.f32(primitives + 0x14 * index as u32 + 8), low);
            assert_eq!(e.mem.f32(primitives + 0x14 * index as u32 + 12), high);
        }
    }

    #[test]
    fn project_primitives_of_a_key_without_a_shape_is_zero() {
        let (mut e, mediator, primitives, direction, results, _) = setup(&[None, Some((2.0, 3.0))]);
        e.mem.set_f32(primitives + 8, 9.0);
        e.call(
            0x00dd_52d0,
            &args![
                mediator,
                direction,
                0u32,
                primitives,
                1i32,
                results,
                results + 4
            ],
        );
        assert_eq!(e.mem.f32(primitives + 8), 0.0);
        assert_eq!(e.mem.f32(primitives + 12), 0.0);
        assert_eq!(e.mem.f32(results), 0.0);
        assert_eq!(e.mem.f32(results + 4), 0.0);
        // The second primitive was not visited.
        assert_eq!(e.mem.f32(primitives + 0x14 + 8), 0.0);
    }

    #[test]
    fn project_primitives_ignores_a_nan_after_the_first_primitive() {
        let shapes = [Some((1.0, 2.0)), Some((f32::NAN, f32::NAN))];
        let (mut e, mediator, primitives, direction, results, _) = setup(&shapes);
        e.call(
            0x00dd_52d0,
            &args![
                mediator,
                direction,
                0u32,
                primitives,
                2i32,
                results,
                results + 4
            ],
        );
        assert_eq!(e.mem.f32(results), 1.0);
        assert_eq!(e.mem.f32(results + 4), 2.0);
        assert!(e.mem.f32(primitives + 0x14 + 8).is_nan());
    }

    #[test]
    fn find_extents_returns_the_range_without_writing_the_primitives() {
        let shapes = [Some((1.0, 3.0)), Some((-2.0, 0.5)), Some((0.0, 4.0))];
        let (mut e, mediator, primitives, direction, results, _) = setup(&shapes);
        e.call(
            0x00dd_5440,
            &args![
                mediator,
                direction,
                0u32,
                primitives,
                3i32,
                results,
                results + 4
            ],
        );
        assert_eq!(e.mem.f32(results), -2.0);
        assert_eq!(e.mem.f32(results + 4), 4.0);
        for index in 0..3 {
            assert_eq!(e.mem.u32(primitives + 0x14 * index + 8), 0);
            assert_eq!(e.mem.u32(primitives + 0x14 * index + 12), 0);
        }
    }

    #[test]
    fn find_extents_of_a_negative_direction_swaps_the_ends() {
        let shapes = [Some((1.0, 3.0)), Some((-2.0, 0.5))];
        let (mut e, mediator, primitives, direction, results, _) = setup(&shapes);
        e.mem.set_f32(direction, -1.0);
        e.call(
            0x00dd_5440,
            &args![
                mediator,
                direction,
                0u32,
                primitives,
                2i32,
                results,
                results + 4
            ],
        );
        assert_eq!(e.mem.f32(results), -3.0);
        assert_eq!(e.mem.f32(results + 4), 2.0);
    }
}
