//! `BSHavok/bhkacousticspacephantomcallbackshape.obj` (Xbox PDB source unit), subsystem `BSHavok`:
//! its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two phantom callbacks find an object through the second collidable they receive and
//! forward the event to it. Both functions keep a 0x80-byte local block in which the exe stores
//! their first argument at `+0x20` and the second at `+0x48` and hands the block's address to
//! the callee; the other bytes of that block are uninitialised stack in the exe, and the
//! translation passes zeros there. Both carry a `/GS` cookie check (`00ec408c`), which is not
//! modeled, and are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Size of the local block whose address the callbacks pass on.
const EVENT_BLOCK_SIZE: u32 = 0x80;

/// The object the event is forwarded to: from the second collidable (`other`) take the signed
/// byte at `+0x10` (its owner offset), read the word `+8` of the object at `other + offset`;
/// when that is not 0, the word at `+0x350` of it. Answers 0 when the word is 0.
fn event_world(e: &mut Engine, other: u32) -> u32 {
    let owner_offset = e.mem.i8(other.wrapping_add(0x10)) as i32 as u32;
    let owner_word = e.mem.u32(other.wrapping_add(owner_offset).wrapping_add(8));
    if owner_word == 0 {
        0
    } else {
        e.mem.u32(owner_word.wrapping_add(0x350))
    }
}

/// Runs virtual slot `slot` of the object at `+0x28` of `world` on the event block built from
/// the two collidables.
fn forward_event(e: &mut Engine, world: u32, slot: u32, first: u32, other: u32) {
    let target = e.mem.u32(world.wrapping_add(0x28));
    e.with_stack(EVENT_BLOCK_SIZE, |e, block| {
        e.mem.set_u32(block.addr() + 0x20, first);
        e.mem.set_u32(block.addr() + 0x48, other);
        e.vcall(target, slot, &args![block]);
    });
}

// Translated from 00ca0b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0b70 - the /GS cookie check (00ec408c) is not modeled, and the event block's uninitialised bytes are passed as zeros.
/// `bhkAcousticSpacePhantomCallbackShape::phantomEnterEvent` (Xbox PDB): a virtual method
/// (`this` in `ECX`, never read) with three stack words (the third is unused). Finds the world
/// object through the second collidable ([`event_world`]) and runs virtual slot 0x1C of the
/// object at `world + 0x28` on the event block. The exe does not test the world object for 0
/// here: with no world object it reads address 0x28 and faults, which the translation
/// reproduces as a memory panic.
pub fn bhk_acoustic_space_phantom_callback_shape_phantom_enter_event(
    e: &mut Engine,
    _this: u32,
    first: u32,
    other: u32,
    _unused_3: u32,
) {
    let world = event_world(e, other);
    forward_event(e, world, 0x1c, first, other);
}

// Translated from 00ca0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0bd0 - the /GS cookie check (00ec408c) is not modeled, and the event block's uninitialised bytes are passed as zeros.
/// `bhkAcousticSpacePhantomCallbackShape::phantomLeaveEvent` (Xbox PDB): a virtual method
/// (`this` in `ECX`, never read) with two stack words. Finds the world object through the second
/// collidable ([`event_world`]) and, when there is one, runs virtual slot 0x20 of the object at
/// `world + 0x28` on the event block.
pub fn bhk_acoustic_space_phantom_callback_shape_phantom_leave_event(
    e: &mut Engine,
    _this: u32,
    first: u32,
    other: u32,
) {
    let world = event_world(e, other);
    if world != 0 {
        forward_event(e, world, 0x20, first, other);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ca0b70,
            bhk_acoustic_space_phantom_callback_shape_phantom_enter_event(u32, u32, u32, u32)
        ),
        entry!(
            0x00ca0bd0,
            bhk_acoustic_space_phantom_callback_shape_phantom_leave_event(u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: the exe never reads `ECX` in these functions.
    const THIS: u32 = 0x7777_0000;

    /// Records of the sink callbacks: `(slot, this, first word, second word)`.
    type Seen = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    /// A second collidable whose owner offset is -0x20 (the owner sits 0x20 bytes before it),
    /// the owner's `+8` word pointing at a Havok world whose `+0x350` is a world object with
    /// an event sink at `+0x28`; the sink's vtable slots 0x1C and 0x20 are doubles.
    fn setup() -> (Engine, u32, Seen) {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        for (slot, id) in [(0x1cu32, 0x0900_001cu32), (0x20, 0x0900_0020)] {
            let log = seen.clone();
            e.register_double(id, move |e, a| {
                let first = e.mem.u32(a[1] + 0x20);
                let other = e.mem.u32(a[1] + 0x48);
                log.borrow_mut().push((slot, a[0], first, other));
                Ret::default()
            });
        }
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, 0x0900_001c);
        e.mem.set_u32(table + 0x20, 0x0900_0020);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, table);
        let world_object = e.mem.alloc(0x400);
        e.mem.set_u32(world_object + 0x28, sink);
        let havok_world = e.mem.alloc(0x400);
        e.mem.set_u32(havok_world + 0x350, world_object);
        let region = e.mem.alloc(0x40);
        let owner = region;
        let other = region + 0x20;
        e.mem.set_u32(owner + 8, havok_world);
        e.mem.set_i8(other + 0x10, -0x20);
        (e, other, seen)
    }

    #[test]
    fn enter_event_forwards_to_slot_0x1c_of_the_sink() {
        let (mut e, other, seen) = setup();
        e.call(0x00ca0b70, &args![THIS, 0x1111u32, other, 0x9999u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0].0, seen[0].2, seen[0].3), (0x1c, 0x1111, other));
    }

    #[test]
    #[should_panic]
    fn enter_event_without_a_world_object_faults_like_the_exe() {
        let (mut e, other, _) = setup();
        let region = other - 0x20;
        e.mem.set_u32(region + 8, 0);
        e.call(0x00ca0b70, &args![THIS, 0x1111u32, other, 0u32]);
    }

    #[test]
    fn leave_event_forwards_to_slot_0x20_only_with_a_world_object() {
        let (mut e, other, seen) = setup();
        e.call(0x00ca0bd0, &args![THIS, 0x2222u32, other]);
        {
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!((seen[0].0, seen[0].2, seen[0].3), (0x20, 0x2222, other));
        }
        let region = other - 0x20;
        e.mem.set_u32(region + 8, 0);
        e.call(0x00ca0bd0, &args![THIS, 0x2222u32, other]);
        assert_eq!(seen.borrow().len(), 1);
    }
}
