//! `BSHavok/bhkshapecollection.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkShapeCollection` (vtable `010c44e4`, RTTI-confirmed) is a `bhkShape` whose Havok object
//! (`+8`) carries an interface at its own `+0x10` that enumerates the child shapes by key: its
//! virtual slot `+8` answers the first key, `+0xc` the key after a given one (both answer
//! `0xffffffff` at the end) and `+0x14` the child for a key, given a 0x200-byte scratch buffer;
//! the child's `+8` is the `bhkShape` wrapper (the identity of the interface is unconfirmed).
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame or
//! a `/GS` stack-cookie check (`00ec408c`), neither of which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `~bhkShape` (`00c84c50`), `bhkSerializable::Kill` (`00c876a0`), `bhkShape::CanShare`
/// (`00c84ec0`) and `bhkShape::GetViewerStrings` (`00c851c0`), engine map names.
const SHAPE_DESTRUCT: u32 = 0x00c8_4c50;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c8_51c0;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable `~bhkShapeCollection` stores and the object counter it lowers.
const COLLECTION_VTABLE: u32 = 0x010c_44e4;
const COLLECTION_COUNT: u32 = 0x0126_86d0;
/// The class name word (`012686d4`) `GetViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_86d4;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

/// The key that ends the enumeration.
const END_KEY: u32 = 0xffff_ffff;
/// The scratch buffer the child lookup is given.
const CHILD_BUFFER_SIZE: u32 = 0x200;
// Virtual slots (byte offsets) of the enumeration interface.
const VSLOT_FIRST_KEY: u32 = 0x8;
const VSLOT_NEXT_KEY: u32 = 0xc;
const VSLOT_CHILD: u32 = 0x14;
// Virtual slots of the child `bhkShape`.
/// `CanShare` of the child (unconfirmed which class names it; the engine map names the
/// `bhkShape` one `00c84ec0`).
const VSLOT_CHILD_CAN_SHARE: u32 = 0xd0;
/// Builds the child's display geometry (`BuildDisplayGeometry`, engine map name of the
/// collection's own override).
const VSLOT_CHILD_BUILD_DISPLAY: u32 = 0xd4;

/// The enumeration interface of the Havok object (`+8`) of `this`: its address plus 0x10, with
/// no null checks (the exe computes `0 + 0x10` for a missing object).
fn container_of(e: &Engine, this: u32) -> u32 {
    let object = if this != 0 { e.mem.u32(this + 8) } else { 0 };
    object.wrapping_add(0x10)
}

/// The child for `key` through the interface (virtual `+0x14`, with the key and the caller's
/// 0x200-byte `buffer`), answering the `bhkShape` wrapper at its `+8`, or 0. The child can be
/// built in `buffer`, so `buffer` must still be there when its `+8` is read.
fn child_shape(e: &mut Engine, container: u32, key: u32, buffer: Ptr) -> u32 {
    let child = e.vcall(container, VSLOT_CHILD, &args![key, buffer]).u32();
    if child != 0 {
        e.mem.u32(child + 8)
    } else {
        0
    }
}

// Translated from 00cd2de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd2de0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShapeCollection::~bhkShapeCollection` (engine map name): stores the vtable, runs `Kill`
/// (`00c876a0`), lowers the object counter (`012686d0`) and runs `~bhkShape` (`00c84c50`).
pub fn bhk_shape_collection_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COLLECTION_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COLLECTION_COUNT);
    e.mem.set_u32(COLLECTION_COUNT, count.wrapping_sub(1));
    e.call(SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00cd2e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd2e50 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkShapeCollection::BuildDisplayGeometry` (engine map name), `(first_argument,
/// second_argument)`: walks the keys of the Havok object's interface (virtual `+8` for the
/// first, `+0xc` for the next, `0xffffffff` ending it) and, for each, fetches the child
/// (virtual `+0x14`, with a 0x200-byte buffer), takes the next key and runs the child's
/// virtual `+0xd4` with the two arguments; the walk ends at a key without a child.
pub fn bhk_shape_collection_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    first_argument: u32,
    second_argument: u32,
) {
    let container = container_of(e, this.addr());
    let mut key = e.vcall(container, VSLOT_FIRST_KEY, &args![]).u32();
    // The 0x200-byte buffer is a local of the whole frame (stack offset -0x220, used for every key):
    // the child slot 0x14 answers can be built in it and its `+8` is read after the call, so
    // the buffer lives for the whole walk. (Scoped to the slot call alone, the `+8` was read
    // from a freed block.)
    e.with_stack(CHILD_BUFFER_SIZE, |e, buffer| {
        while key != END_KEY {
            let mut shape = 0;
            if this.addr() != 0 {
                let object = e.mem.u32(this.addr() + 8);
                if object != 0 {
                    shape = child_shape(e, object.wrapping_add(0x10), key, buffer);
                }
            }
            let container = container_of(e, this.addr());
            key = e.vcall(container, VSLOT_NEXT_KEY, &args![key]).u32();
            if shape == 0 {
                break;
            }
            e.vcall(
                shape,
                VSLOT_CHILD_BUILD_DISPLAY,
                &args![first_argument, second_argument],
            );
        }
    });
}

// Translated from 00cd2f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd2f10 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkShapeCollection::CanShare` (engine map name), `(other)`: false if `bhkShape::CanShare`
/// (`00c84ec0`) says so; otherwise walks the keys of the Havok object's interface as
/// `BuildDisplayGeometry` does, replacing the answer by the child's virtual `+0xd0` with
/// `other` for each child found (a key without a child leaves it), and stops when it turns
/// false. Answers the last answer.
pub fn bhk_shape_collection_can_share(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mut result = e.call(SHAPE_CAN_SHARE, &args![this, other]).bool();
    if !result {
        return false;
    }
    let container = container_of(e, this.addr());
    let mut key = e.vcall(container, VSLOT_FIRST_KEY, &args![]).u32();
    // The frame's 0x200-byte buffer, live for the whole walk as in `BuildDisplayGeometry`.
    e.with_stack(CHILD_BUFFER_SIZE, |e, buffer| {
        while key != END_KEY && result {
            if this.addr() != 0 {
                let object = e.mem.u32(this.addr() + 8);
                if object != 0 {
                    let shape = child_shape(e, object.wrapping_add(0x10), key, buffer);
                    if shape != 0 {
                        result = e.vcall(shape, VSLOT_CHILD_CAN_SHARE, &args![other]).bool();
                    }
                }
            }
            let container = container_of(e, this.addr());
            key = e.vcall(container, VSLOT_NEXT_KEY, &args![key]).u32();
        }
    });
    result
}

// Translated from 00cd2fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd2fd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkShapeCollection::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the shape's strings (`00c851c0`) and the text
/// `NiGetViewerString` makes of the word at `012686d4`, stored in the string array (grown by
/// `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`; stored by `0096ae90`).
pub fn bhk_shape_collection_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    let array = strings.addr();
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
    e.mem.set_u32(thread + SCOPE_WORD, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd2de0, bhk_shape_collection_destructor(Ptr)),
        entry!(
            0x00cd2e50,
            bhk_shape_collection_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00cd2f10, bhk_shape_collection_can_share(Ptr, Ptr) -> bool),
        entry!(
            0x00cd2fd0,
            bhk_shape_collection_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_KEY_SLOT: u32 = 0x7000_0008;
    const NEXT_KEY_SLOT: u32 = 0x7000_000c;
    const CHILD_SLOT: u32 = 0x7000_0014;
    const CAN_SHARE_SLOT: u32 = 0x7000_00d0;
    const BUILD_DISPLAY_SLOT: u32 = 0x7000_00d4;
    const CONTAINER_VTABLE: u32 = 0x7001_1000;
    const CHILD_VTABLE: u32 = 0x7001_2000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// A collection whose Havok object enumerates `keys` and has a child for each key with
    /// `Some(wrapper)` in `children` (the wrapper's `+8` is a child shape with the slots
    /// `+0xd0` and `+0xd4`). Answers the collection, the Havok object's interface and the
    /// child shapes.
    fn collection(e: &mut Engine, keys: &[u32], children: &[bool]) -> (u32, u32, Vec<u32>) {
        let mut container_slots = vec![0u32; 6];
        container_slots[VSLOT_FIRST_KEY as usize / 4] = FIRST_KEY_SLOT;
        container_slots[VSLOT_NEXT_KEY as usize / 4] = NEXT_KEY_SLOT;
        container_slots[VSLOT_CHILD as usize / 4] = CHILD_SLOT;
        e.put_vtable(CONTAINER_VTABLE, &container_slots);
        let mut child_slots = vec![0u32; 0x36];
        child_slots[VSLOT_CHILD_CAN_SHARE as usize / 4] = CAN_SHARE_SLOT;
        child_slots[VSLOT_CHILD_BUILD_DISPLAY as usize / 4] = BUILD_DISPLAY_SLOT;
        e.put_vtable(CHILD_VTABLE, &child_slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x10, CONTAINER_VTABLE);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, object);
        let mut shapes = vec![];
        let mut wrappers = vec![];
        for has_child in children {
            let shape = e.mem.alloc(0x10);
            e.mem.set_u32(shape, CHILD_VTABLE);
            shapes.push(shape);
            let wrapper = if *has_child {
                let wrapper = e.mem.alloc(0x10);
                e.mem.set_u32(wrapper + 8, shape);
                wrapper
            } else {
                0
            };
            wrappers.push(wrapper);
        }
        let first = keys.first().copied().unwrap_or(END_KEY);
        e.register_double(FIRST_KEY_SLOT, move |_, _| first.into_ret());
        let order = keys.to_vec();
        e.register_double(NEXT_KEY_SLOT, move |_, a| {
            let at = order.iter().position(|k| *k == a[1]).unwrap();
            order.get(at + 1).copied().unwrap_or(END_KEY).into_ret()
        });
        let order = keys.to_vec();
        e.register_double(CHILD_SLOT, move |_, a| {
            let at = order.iter().position(|k| *k == a[1]).unwrap();
            wrappers[at].into_ret()
        });
        (this, object + 0x10, shapes)
    }

    #[test]
    fn bhk_shape_collection_destructor_runs_kill_lowers_the_counter_and_runs_the_shape() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SHAPE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(COLLECTION_COUNT, 4);
        e.call(0x00cd_2de0, &args![this]);
        assert_eq!(e.mem.u32(this), COLLECTION_VTABLE);
        assert_eq!(e.mem.u32(COLLECTION_COUNT), 3);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SHAPE_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, SHAPE_DESTRUCT]),
            vec![SERIALIZABLE_KILL, SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn bhk_shape_collection_build_display_geometry_visits_every_child() {
        let mut e = logged_engine();
        e.register(BUILD_DISPLAY_SLOT, |_, _| Ret::default());
        let (this, container, shapes) = collection(&mut e, &[5, 7, 9], &[true, true, true]);
        e.call(0x00cd_2e50, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(
            calls_to(&e, BUILD_DISPLAY_SLOT),
            shapes
                .iter()
                .map(|shape| vec![*shape, 0x1111, 0x2222])
                .collect::<Vec<_>>()
        );
        assert_eq!(calls_to(&e, FIRST_KEY_SLOT), vec![vec![container]]);
        assert_eq!(
            calls_to(&e, NEXT_KEY_SLOT),
            vec![vec![container, 5], vec![container, 7], vec![container, 9]]
        );
        let children = calls_to(&e, CHILD_SLOT);
        assert_eq!(
            children.iter().map(|c| (c[0], c[1])).collect::<Vec<_>>(),
            vec![(container, 5), (container, 7), (container, 9)]
        );
    }

    #[test]
    fn bhk_shape_collection_build_display_geometry_stops_at_a_key_without_a_child() {
        let mut e = logged_engine();
        e.register(BUILD_DISPLAY_SLOT, |_, _| Ret::default());
        let (this, _, shapes) = collection(&mut e, &[5, 7, 9], &[true, false, true]);
        e.call(0x00cd_2e50, &args![this, 1u32, 2u32]);
        assert_eq!(
            calls_to(&e, BUILD_DISPLAY_SLOT),
            vec![vec![shapes[0], 1, 2]]
        );
        // The next key is taken before the missing child ends the walk.
        assert_eq!(calls_to(&e, NEXT_KEY_SLOT).len(), 2);
        assert_eq!(calls_to(&e, CHILD_SLOT).len(), 2);
    }

    #[test]
    fn bhk_shape_collection_build_display_geometry_with_no_keys_does_nothing() {
        let mut e = logged_engine();
        e.register(BUILD_DISPLAY_SLOT, |_, _| Ret::default());
        let (this, _, _) = collection(&mut e, &[], &[]);
        e.call(0x00cd_2e50, &args![this, 1u32, 2u32]);
        assert!(calls_to(&e, CHILD_SLOT).is_empty());
        assert!(calls_to(&e, BUILD_DISPLAY_SLOT).is_empty());
    }

    #[test]
    fn bhk_shape_collection_can_share_asks_every_child() {
        let mut e = logged_engine();
        e.register(SHAPE_CAN_SHARE, |_, _| 1u32.into_ret());
        e.register(CAN_SHARE_SLOT, |_, _| 1u32.into_ret());
        let (this, _, shapes) = collection(&mut e, &[5, 7], &[true, true]);
        assert!(e.call(0x00cd_2f10, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, SHAPE_CAN_SHARE), vec![vec![this, 0x4444]]);
        assert_eq!(
            calls_to(&e, CAN_SHARE_SLOT),
            vec![vec![shapes[0], 0x4444], vec![shapes[1], 0x4444]]
        );
    }

    #[test]
    fn bhk_shape_collection_can_share_stops_at_the_first_child_that_refuses() {
        let mut e = logged_engine();
        e.register(SHAPE_CAN_SHARE, |_, _| 1u32.into_ret());
        let (this, _, shapes) = collection(&mut e, &[5, 7, 9], &[true, true, true]);
        let refusing = shapes[1];
        e.register_double(CAN_SHARE_SLOT, move |_, a| {
            u32::from(a[0] != refusing).into_ret()
        });
        assert!(!e.call(0x00cd_2f10, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, CAN_SHARE_SLOT).len(), 2);
    }

    #[test]
    fn bhk_shape_collection_can_share_skips_a_key_without_a_child() {
        let mut e = logged_engine();
        e.register(SHAPE_CAN_SHARE, |_, _| 1u32.into_ret());
        e.register(CAN_SHARE_SLOT, |_, _| 1u32.into_ret());
        let (this, _, shapes) = collection(&mut e, &[5, 7], &[false, true]);
        assert!(e.call(0x00cd_2f10, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, CAN_SHARE_SLOT), vec![vec![shapes[1], 0x4444]]);
        assert_eq!(calls_to(&e, NEXT_KEY_SLOT).len(), 2);
    }

    #[test]
    fn bhk_shape_collection_can_share_answers_false_when_the_shape_refuses() {
        let mut e = logged_engine();
        e.register(SHAPE_CAN_SHARE, |_, _| Ret::default());
        let (this, _, _) = collection(&mut e, &[5], &[true]);
        assert!(!e.call(0x00cd_2f10, &args![this, 0x4444u32]).bool());
        assert!(calls_to(&e, FIRST_KEY_SLOT).is_empty());
    }

    /// A child the interface builds in the 0x200-byte buffer (virtual `+0x14` answers the
    /// buffer itself, its `+8` the child shape) is read while the buffer is still there: the
    /// buffer is the frame's (stack offset -0x220), not the slot call's. With the freed-block check a
    /// buffer freed early reads back as the fill word, and the virtual call through it faults.
    #[test]
    fn bhk_shape_collection_reads_a_child_built_in_its_buffer() {
        let mut e = logged_engine();
        e.mem.enable_freed_check(true);
        e.register(BUILD_DISPLAY_SLOT, |_, _| Ret::default());
        e.register(SHAPE_CAN_SHARE, |_, _| 1u32.into_ret());
        e.register(CAN_SHARE_SLOT, |_, _| 1u32.into_ret());
        let (this, _, shapes) = collection(&mut e, &[5, 7], &[true, true]);
        let in_place = shapes.clone();
        e.register_double(CHILD_SLOT, move |e, a| {
            let at = usize::from(a[1] == 7);
            e.mem.set_u32(a[2] + 8, in_place[at]);
            a[2].into_ret()
        });
        e.call(0x00cd_2e50, &args![this, 1u32, 2u32]);
        assert_eq!(
            calls_to(&e, BUILD_DISPLAY_SLOT),
            vec![vec![shapes[0], 1, 2], vec![shapes[1], 1, 2]]
        );
        assert!(e.call(0x00cd_2f10, &args![this, 0x4444u32]).bool());
        assert_eq!(
            calls_to(&e, CAN_SHARE_SLOT),
            vec![vec![shapes[0], 0x4444], vec![shapes[1], 0x4444]]
        );
    }

    #[test]
    fn bhk_shape_collection_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cd_2fd0, &args![this, array]);
        assert_eq!(
            calls_to(&e, SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
