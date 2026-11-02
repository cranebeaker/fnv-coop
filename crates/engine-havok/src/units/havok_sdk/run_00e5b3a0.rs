//! `Havok SDK/run_00e5b3a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkVector4::setTransformedInversePos` (Xbox PDB, `00c7f690`): thiscall, `this` is the result,
/// then the transform and the position, `RET 8`.
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// `00d53d30`: thiscall on the object at `world + 0xc`, `(vector, vector)`, `RET 8`.
const PASS_VECTORS: u32 = 0x00d5_3d30;
/// `00dd04d0`: thiscall on the world, one word (a body), `RET 4`.
const ADD_BODY: u32 = 0x00dd_04d0;
/// `00c74ac0`: thiscall, `this` is the result vector, then two pointers, `RET 8`.
const COMBINE_VECTORS: u32 = 0x00c7_4ac0;
/// `00e3b850`: thiscall on the object at `world + 0xc`, three vectors then three object pointers,
/// `RET 0x18`.
const PASS_VECTORS_AND_OBJECTS: u32 = 0x00e3_b850;

/// The frame an object carries at `+0x10`: when the byte at `object + 0x28` is 1, the object plus
/// `0x10` plus the signed byte at `object + 0x20`; otherwise 0.
fn object_frame(e: &Engine, object: u32) -> u32 {
    let base = object.wrapping_add(0x10);
    if e.mem.u8(base.wrapping_add(0x18)) == 1 {
        let offset = i32::from(e.mem.i8(base.wrapping_add(0x10)));
        (offset as u32).wrapping_add(base)
    } else {
        0
    }
}

/// The frames of the last body of the world's body array (`*(world + 0x38)`, `[+0x3c]` entries)
/// and of `body`, in that order.
fn frames(e: &Engine, world: u32, body: u32) -> (u32, u32) {
    let bodies = e.mem.u32(world.wrapping_add(0x38));
    let count = e.mem.u32(world.wrapping_add(0x3c));
    let last = e
        .mem
        .u32(bodies.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
    (object_frame(e, last), object_frame(e, body))
}

// Translated from 00e5b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(world, body, position)`. Takes the frame of the last body in `world`'s array and the
/// frame of `body` (see [`object_frame`]; the transform is the 64-byte block at `frame + 0xf0`),
/// transforms `position` by the inverse of each (`hkVector4::setTransformedInversePos`,
/// `00c7f690`, into two vectors on the stack), passes both to `00d53d30` on the object at
/// `world + 0xc` and then runs `00dd04d0` on `world` with `body`.
pub fn fn_00e5b3a0(e: &mut Engine, world: u32, body: u32, position: u32) {
    let object = e.mem.u32(world.wrapping_add(0xc));
    let (last_frame, body_frame) = frames(e, world, body);
    e.with_stack(0x20, |e, vectors| {
        let first = vectors.addr();
        let second = first.wrapping_add(0x10);
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![first, last_frame.wrapping_add(0xf0), position],
        );
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![second, body_frame.wrapping_add(0xf0), position],
        );
        e.call(PASS_VECTORS, &args![object, first, second]);
    });
    e.call(ADD_BODY, &args![world, body]);
}

// Translated from 00e5b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(world, body, position, first, second, third)`. As `00e5b3a0` up to the two transformed
/// vectors; then `00c74ac0` builds a third vector (on the stack) from the blocks at `frame +
/// 0x160` of `body` and of the last body, `00e3b850` runs on the object at `world + 0xc` with the
/// three vectors and the three words `first`, `second` and `third` (object pointers it takes a
/// reference on), and `00dd04d0` runs on `world` with `body`. The exe also copies the three
/// words into a stack block that nothing reads again; that is left out.
pub fn fn_00e5b440(
    e: &mut Engine,
    world: u32,
    body: u32,
    position: u32,
    first: u32,
    second: u32,
    third: u32,
) {
    let object = e.mem.u32(world.wrapping_add(0xc));
    let (last_frame, body_frame) = frames(e, world, body);
    e.with_stack(0x30, |e, vectors| {
        let first_vector = vectors.addr();
        let second_vector = first_vector.wrapping_add(0x10);
        let third_vector = first_vector.wrapping_add(0x20);
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![first_vector, last_frame.wrapping_add(0xf0), position],
        );
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![second_vector, body_frame.wrapping_add(0xf0), position],
        );
        e.call(
            COMBINE_VECTORS,
            &args![
                third_vector,
                body_frame.wrapping_add(0x160),
                last_frame.wrapping_add(0x160)
            ],
        );
        e.call(
            PASS_VECTORS_AND_OBJECTS,
            &args![
                object,
                first_vector,
                second_vector,
                third_vector,
                first,
                second,
                third
            ],
        );
    });
    e.call(ADD_BODY, &args![world, body]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5_b3a0, fn_00e5b3a0(u32, u32, u32)),
        entry!(0x00e5_b440, fn_00e5b440(u32, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::store_vec;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                // The vector builders write a marker into their result.
                if address == SET_TRANSFORMED_INVERSE_POS || address == COMBINE_VECTORS {
                    store_vec(e, a[0], [a[1] as f32, 0.0, 0.0, 0.0]);
                }
                Ret::default()
            });
        }
        calls
    }

    /// A body object whose frame sits at `+0x10 + offset` (when `framed`).
    fn body(e: &mut Engine, framed: bool, offset: i8) -> u32 {
        let body = e.mem.alloc(0x40);
        if framed {
            e.mem.set_u8(body + 0x28, 1);
            e.mem.set_i8(body + 0x20, offset);
        }
        body
    }

    fn world(e: &mut Engine, bodies: &[u32]) -> u32 {
        let world = e.mem.alloc(0x50);
        let list = e.mem.alloc(4 * bodies.len() as u32 + 4);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *body);
        }
        e.mem.set_u32(world + 0x38, list);
        e.mem.set_u32(world + 0x3c, bodies.len() as u32);
        e.mem.set_u32(world + 0xc, 0x5555);
        world
    }

    #[test]
    fn frames_follow_the_flag_and_the_signed_offset() {
        let mut e = Engine::new();
        let calls = record(
            &mut e,
            &[SET_TRANSFORMED_INVERSE_POS, PASS_VECTORS, ADD_BODY],
        );
        let last = body(&mut e, true, 8);
        let other = body(&mut e, false, 0);
        let world = world(&mut e, &[0x1111, last]);
        e.call(0x00e5_b3a0, &args![world, other, 0x4000u32]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 4);
        // The last body's frame is `last + 0x10 + 8`; the matrix is 0xf0 into it.
        assert_eq!(calls[0].1[1], last + 0x10 + 8 + 0xf0);
        assert_eq!(calls[0].1[2], 0x4000);
        // The second body is not framed: the frame is 0.
        assert_eq!(calls[1].1[1], 0xf0);
        assert_eq!(calls[2].0, PASS_VECTORS);
        assert_eq!(calls[2].1[0], 0x5555);
        assert_eq!(calls[2].1[1], calls[0].1[0]);
        assert_eq!(calls[2].1[2], calls[1].1[0]);
        assert_eq!(calls[3], (ADD_BODY, vec![world, other]));
    }

    #[test]
    fn negative_offsets_are_signed() {
        let mut e = Engine::new();
        let calls = record(
            &mut e,
            &[SET_TRANSFORMED_INVERSE_POS, PASS_VECTORS, ADD_BODY],
        );
        let last = body(&mut e, true, -8);
        let other = body(&mut e, true, 0x10);
        let world = world(&mut e, &[last]);
        e.call(0x00e5_b3a0, &args![world, other, 0u32]);
        let calls = calls.borrow();
        assert_eq!(calls[0].1[1], last + 0x10 - 8 + 0xf0);
        assert_eq!(calls[1].1[1], other + 0x10 + 0x10 + 0xf0);
    }

    #[test]
    fn the_extended_version_also_combines_the_frames() {
        let mut e = Engine::new();
        let calls = record(
            &mut e,
            &[
                SET_TRANSFORMED_INVERSE_POS,
                COMBINE_VECTORS,
                PASS_VECTORS_AND_OBJECTS,
                ADD_BODY,
            ],
        );
        let last = body(&mut e, true, 0);
        let other = body(&mut e, true, 4);
        let world = world(&mut e, &[last]);
        e.call(
            0x00e5_b440,
            &args![world, other, 0x4000u32, 0xa1u32, 0xa2u32, 0xa3u32],
        );
        let calls = calls.borrow();
        let order: Vec<u32> = calls.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![
                SET_TRANSFORMED_INVERSE_POS,
                SET_TRANSFORMED_INVERSE_POS,
                COMBINE_VECTORS,
                PASS_VECTORS_AND_OBJECTS,
                ADD_BODY
            ]
        );
        // The combine call: result, the second body's block, the last body's block.
        assert_eq!(calls[2].1[1], other + 0x10 + 4 + 0x160);
        assert_eq!(calls[2].1[2], last + 0x10 + 0x160);
        let words = &calls[3].1;
        assert_eq!(words[0], 0x5555);
        assert_eq!(&words[1..3], &[calls[0].1[0], calls[1].1[0]]);
        assert_eq!(words[3], calls[2].1[0]);
        assert_eq!(&words[4..], &[0xa1, 0xa2, 0xa3]);
        assert_eq!(calls[4], (ADD_BODY, vec![world, other]));
        // The three vectors are laid out 16 bytes apart.
        assert_eq!(calls[1].1[0], calls[0].1[0] + 0x10);
        assert_eq!(calls[2].1[0], calls[0].1[0] + 0x20);
    }
}
