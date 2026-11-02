//! `BSHavok/bhkwaterphantomcallbackshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The three callbacks find an object through the second collidable they receive and forward
//! the event to the object kept at `+0x24` of it. `phantomEnterEvent` and `phantomLeaveEvent`
//! keep a local block in which the exe stores their first argument at `+0x20` and the second at
//! `+0x48` and hand the block's address on; the other bytes of that block are uninitialised
//! stack in the exe, and the translation passes zeros there. All three carry a `/GS` cookie
//! check (`00ec408c`) or an `FS:[0]` exception frame, which are not modeled, and are marked
//! `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

/// Size of the local block `phantomEnterEvent` and `phantomLeaveEvent` pass on.
const EVENT_BLOCK_SIZE: u32 = 0x80;
/// Per-thread word (`TLS + 0x2b4`) that `castRayWithCollector` sets to 0x10 around its work.
const SCOPE_WORD: u32 = 0x2b4;
/// The shape-type value (at `+0xC` of the first word of the list entry) that
/// `castRayWithCollector` accepts.
const ACCEPTED_SHAPE_KIND: u32 = 0x1b;
/// Reads the hit flag out of the ray-cast result buffer (`00632c60`, `this` = the 0x4C-byte
/// result block, one argument: the address of the byte the cast wrote).
const RESULT_FLAG_ADDRESS: u32 = 0x0063_2c60;
/// Size of the ray-cast result block `castRayWithCollector` builds.
const RAY_RESULT_SIZE: u32 = 0x4c;

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

/// Runs virtual slot `slot` of the object at `+0x24` of `world` on the event block built from
/// the two collidables.
fn forward_event(e: &mut Engine, world: u32, slot: u32, first: u32, other: u32) {
    let target = e.mem.u32(world.wrapping_add(0x24));
    e.with_stack(EVENT_BLOCK_SIZE, |e, block| {
        e.mem.set_u32(block.addr() + 0x20, first);
        e.mem.set_u32(block.addr() + 0x48, other);
        e.vcall(target, slot, &args![block]);
    });
}

// Translated from 00c9e610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9e610 - the /GS cookie check (00ec408c) is not modeled, and the event block's uninitialised bytes are passed as zeros.
/// `bhkWaterPhantomCallbackShape::phantomEnterEvent` (Xbox PDB): takes three stack words (the third
/// is unused), finds the world object through the second collidable ([`event_world`]) and
/// runs virtual slot 0x1C of the object at `world + 0x24` on the event block. The exe does
/// not test the world object for 0 here: with no world object it reads address 0x24 and
/// faults, which the translation reproduces as a memory panic.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn bhk_water_phantom_callback_shape_phantom_enter_event(
    e: &mut Engine,
    _this: u32,
    first: u32,
    other: u32,
    _unused_3: u32,
) {
    let world = event_world(e, other);
    forward_event(e, world, 0x1c, first, other);
}

// Translated from 00c9e670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9e670 - the /GS cookie check (00ec408c) is not modeled, and the event block's uninitialised bytes are passed as zeros.
/// `bhkWaterPhantomCallbackShape::phantomLeaveEvent` (Xbox PDB): takes two stack words, finds the
/// world object through the second collidable ([`event_world`]) and, when there is one, runs
/// virtual slot 0x20 of the object at `world + 0x24` on the event block. Then, when the byte
/// at `other + 0x18` is 1 and `other + offset` (the signed byte at `other + 0x10`) is not 0
/// and the word at `+0xC` of it is not 0, clears bit 3 of the word at `+0x10` of that.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn bhk_water_phantom_callback_shape_phantom_leave_event(
    e: &mut Engine,
    _this: u32,
    first: u32,
    other: u32,
) {
    let world = event_world(e, other);
    if world != 0 {
        forward_event(e, world, 0x20, first, other);
    }
    if e.mem.u8(other.wrapping_add(0x18)) == 1 {
        let owner_offset = e.mem.i8(other.wrapping_add(0x10)) as i32 as u32;
        let owner = other.wrapping_add(owner_offset);
        if owner != 0 {
            let target = e.mem.u32(owner.wrapping_add(0xc));
            if target != 0 {
                let flags = e.mem.u32(target.wrapping_add(0x10));
                e.mem
                    .set_u32(target.wrapping_add(0x10), flags & 0xffff_fff7);
            }
        }
    }
}

// Translated from 00c9e700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9e700 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled, and the ray-cast result block's uninitialised bytes are passed as zeros.
/// `bhkWaterPhantomCallbackShape::castRayWithCollector` (Xbox PDB): takes the stack words `(first, other,
/// collector)`. With the allocation-scope word set to 0x10 it reads the list `other + 0xC`;
/// when the list exists, its word at `+8` is not 0, its first word (the entry) is not 0, the
/// entry's word at `+0xC` is 0x1B and its word at `+0x10` (the shape) is not 0, it builds a
/// result block (`+0x10` = 1.0f, `+0x14` = -1, `+0x20` = -1, `+0x40` = 0), runs virtual slot
/// 0x20 of the shape on `(flag byte, first, block)`, reads the flag with `00632c60` on the
/// block and, when it is not 0, runs virtual slot 0 of the collector on `(other, block)`.
/// The scope word is restored at the end.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn bhk_water_phantom_callback_shape_cast_ray_with_collector(
    e: &mut Engine,
    _this: u32,
    first: u32,
    other: u32,
    collector: u32,
) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    let list = e.mem.u32(other.wrapping_add(0xc));
    if list != 0 {
        let entry = e.mem.u32(list);
        let counted = e.mem.u32(list.wrapping_add(8));
        if counted != 0 && entry != 0 && e.mem.u32(entry.wrapping_add(0xc)) == ACCEPTED_SHAPE_KIND {
            let shape = e.mem.u32(entry.wrapping_add(0x10));
            if shape != 0 {
                e.with_stack(RAY_RESULT_SIZE, |e, block| {
                    e.with_stack(4, |e, flag| {
                        e.mem.set_f32(block.addr() + 0x10, 1.0);
                        e.mem.set_u32(block.addr() + 0x14, 0xffff_ffff);
                        e.mem.set_u32(block.addr() + 0x20, 0xffff_ffff);
                        e.mem.set_u32(block.addr() + 0x40, 0);
                        e.vcall(shape, 0x20, &args![flag, first, block]);
                        let hit = e.call(RESULT_FLAG_ADDRESS, &args![block, flag]).u32();
                        if e.mem.u8(hit) != 0 {
                            e.vcall(collector, 0, &args![other, block]);
                        }
                    });
                });
            }
        }
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9e610,
            bhk_water_phantom_callback_shape_phantom_enter_event(u32, u32, u32, u32)
        ),
        entry!(
            0x00c9e670,
            bhk_water_phantom_callback_shape_phantom_leave_event(u32, u32, u32)
        ),
        entry!(
            0x00c9e700,
            bhk_water_phantom_callback_shape_cast_ray_with_collector(u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    /// Records of the sink callbacks: `(slot, this, first word, second word)`.
    type Seen = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    /// A second collidable whose owner offset is -0x20 (the owner sits 0x20 bytes before it),
    /// the owner's `+8` word pointing at a Havok world whose `+0x350` is a world object with
    /// an event sink at `+0x24`; the sink's vtable slots 0x1C and 0x20 are doubles.
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
        e.mem.set_u32(world_object + 0x24, sink);
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
        e.call(0x00c9e610, &args![VCALL_THIS, 0x1111u32, other, 0x9999u32]);
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
        e.call(0x00c9e610, &args![VCALL_THIS, 0x1111u32, other, 0u32]);
    }

    #[test]
    fn leave_event_forwards_to_slot_0x20_only_with_a_world_object() {
        let (mut e, other, seen) = setup();
        e.call(0x00c9e670, &args![VCALL_THIS, 0x2222u32, other]);
        {
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!((seen[0].0, seen[0].2, seen[0].3), (0x20, 0x2222, other));
        }
        let region = other - 0x20;
        e.mem.set_u32(region + 8, 0);
        e.call(0x00c9e670, &args![VCALL_THIS, 0x2222u32, other]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn leave_event_clears_bit_3_of_the_flags_when_the_byte_at_0x18_is_1() {
        let (mut e, other, _) = setup();
        let region = other - 0x20;
        e.mem.set_u32(region + 8, 0);
        // other + (-0x20) is the owner; its word at +0xC points at the flagged object.
        let flagged = e.mem.alloc(0x20);
        e.mem.set_u32(flagged + 0x10, 0xffff_ffff);
        e.mem.set_u32(region + 0xc, flagged);
        e.mem.set_u8(other + 0x18, 2);
        e.call(0x00c9e670, &args![VCALL_THIS, 0u32, other]);
        assert_eq!(e.mem.u32(flagged + 0x10), 0xffff_ffff);
        e.mem.set_u8(other + 0x18, 1);
        e.call(0x00c9e670, &args![VCALL_THIS, 0u32, other]);
        assert_eq!(e.mem.u32(flagged + 0x10), 0xffff_fff7);
    }

    type Casts = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A ray-cast list for the collidable `other`: its word at `+0xC` is a list whose first
    /// word is an entry (kind 0x1B, shape at `+0x10`) and whose `+8` is 1. The shape's slot
    /// 0x20 is a double that records `(flag, first, block, scope word, block words)`; the
    /// collector's slot 0 records its arguments. Returns the engine, `other`, the collector,
    /// the scope word's address and the shared logs.
    fn cast_setup(hit: u8) -> (Engine, u32, u32, u32, Casts, Casts) {
        let mut e = Engine::new();
        let casts: Casts = Rc::new(RefCell::new(vec![]));
        let reports: Casts = Rc::new(RefCell::new(vec![]));
        let log = casts.clone();
        e.register_double(0x0900_0020, move |e, a| {
            // The cast sees the scope word set to 0x10 and the result block prepared.
            let tls = e.tls();
            let scope = e.mem.u32(tls + 0x2b4);
            log.borrow_mut().push(vec![
                a[1],
                a[2],
                a[3],
                scope,
                e.mem.u32(a[3] + 0x10),
                e.mem.u32(a[3] + 0x14),
                e.mem.u32(a[3] + 0x20),
                e.mem.u32(a[3] + 0x40),
            ]);
            e.mem.set_u8(a[1], hit);
            Ret::default()
        });
        e.register(0x0063_2c60, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let log = reports.clone();
        e.register_double(0x0900_0100, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let shape_table = e.mem.alloc(0x40);
        e.mem.set_u32(shape_table + 0x20, 0x0900_0020);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, shape_table);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0xc, 0x1b);
        e.mem.set_u32(entry + 0x10, shape);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, entry);
        e.mem.set_u32(list + 8, 1);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, list);
        let collector_table = e.mem.alloc(0x10);
        e.mem.set_u32(collector_table, 0x0900_0100);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, collector_table);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 0x33);
        (e, other, collector, tls + 0x2b4, casts, reports)
    }

    #[test]
    fn cast_ray_builds_the_result_block_and_reports_a_hit_to_the_collector() {
        let (mut e, other, collector, scope, casts, reports) = cast_setup(1);
        e.call(0x00c9e700, &args![VCALL_THIS, 0x7777u32, other, collector]);
        let casts = casts.borrow();
        assert_eq!(casts.len(), 1);
        let cast = &casts[0];
        assert_eq!(cast[1], 0x7777);
        assert_eq!(
            (cast[3], cast[4], cast[5], cast[6], cast[7]),
            (0x10, 1.0f32.to_bits(), 0xffff_ffff, 0xffff_ffff, 0)
        );
        let reports = reports.borrow();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0][0], collector);
        assert_eq!(reports[0][1], other);
        assert_eq!(reports[0][2], cast[2]);
        assert_eq!(e.mem.u32(scope), 0x33);
    }

    #[test]
    fn cast_ray_without_a_hit_does_not_call_the_collector() {
        let (mut e, other, collector, scope, casts, reports) = cast_setup(0);
        e.call(0x00c9e700, &args![VCALL_THIS, 1u32, other, collector]);
        assert_eq!(casts.borrow().len(), 1);
        assert!(reports.borrow().is_empty());
        assert_eq!(e.mem.u32(scope), 0x33);
    }

    #[test]
    fn cast_ray_needs_the_list_the_count_and_a_shape_of_the_accepted_kind() {
        for broken in 0..4 {
            let (mut e, other, collector, scope, casts, _) = cast_setup(1);
            let list = e.mem.u32(other + 0xc);
            let entry = e.mem.u32(list);
            match broken {
                0 => e.mem.set_u32(other + 0xc, 0),
                1 => e.mem.set_u32(list + 8, 0),
                2 => e.mem.set_u32(entry + 0xc, 0x1a),
                _ => e.mem.set_u32(entry + 0x10, 0),
            }
            e.call(0x00c9e700, &args![VCALL_THIS, 1u32, other, collector]);
            assert!(casts.borrow().is_empty());
            assert_eq!(e.mem.u32(scope), 0x33);
        }
    }
}
