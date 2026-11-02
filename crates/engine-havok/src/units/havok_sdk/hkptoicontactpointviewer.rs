//! `Havok SDK/hkptoicontactpointviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_vec, profile_record, store_vec, sum3, PROFILE_END,
};

// Translated from 00d68710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d68740`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68710(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8740, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d68720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d68740`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68720(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8740, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The profile label of the arrow display (the string `TtToiDisplayArrow`).
const LABEL_DISPLAY_ARROW: u32 = 0x010d_1f18;
/// The process tag (`01269004`) that `registerViewer` stores and `getProcessTag` returns.
const PROCESS_TAG: u32 = 0x0126_9004;
/// The process factory pointer (`01269cd0`) and `registerProcess` (`00dd9380`, `this` = the
/// factory, `(name, creator)`).
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
/// `hkpWorldViewerBase`'s constructor (`00ddccb0`, `this`, `(world)`).
const VIEWER_BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
/// Constants of the arrow: the smallest squared length that is drawn (`0104da4c`), the fraction
/// of the shaft at which the head starts (`01019de0`), the head's two half-widths
/// (`010c69c4`, `010c69c0`) and the vector of ones (`010ec7f0`).
const ARROW_MINIMUM: u32 = 0x0104_da4c;
const ARROW_HEAD_START: u32 = 0x0101_9de0;
const ARROW_HEAD_PLUS: u32 = 0x010c_69c4;
const ARROW_HEAD_MINUS: u32 = 0x010c_69c0;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// The color of the arrow `00d68670` draws.
const ARROW_COLOR: u32 = 0xffff_0000;

// Translated from 00d68260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiContactPointViewer::getProcessTag` (engine map; a static function): the tag word at
/// `01269004`.
pub fn hkp_toi_contact_point_viewer_get_process_tag(e: &mut Engine) -> u32 {
    e.mem.u32(PROCESS_TAG)
}

// Translated from 00d68270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00c93a20` (`this` = `list`, `(listener)`). `this` is the address of the
/// sub-object at `+0x20` of a viewer `B`; the listener is the sub-object at `B + 0x28`, that is
/// `this + 8`, or 0 when `B` is null (`this - 0x20` is 0). The caller gets the callee's `EAX`;
/// the class is unconfirmed.
pub fn fn_00d68270(e: &mut Engine, this: Ptr, list: Ptr) -> u32 {
    let listener = if this.addr().wrapping_sub(0x20) != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    };
    e.call(0x00c9_3a20, &args![list, listener]).u32()
}

// Translated from 00d682a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As [`fn_00d68270`] but forwards to `00c91a20` (`this` = `list`, `(listener)`); the listener
/// is `this + 8`, or 0 when `this - 0x20` is 0. The caller gets the callee's `EAX`; the class is
/// unconfirmed.
pub fn fn_00d682a0(e: &mut Engine, this: Ptr, list: Ptr) -> u32 {
    let listener = if this.addr().wrapping_sub(0x20) != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    };
    e.call(0x00c9_1a20, &args![list, listener]).u32()
}

// Translated from 00d682d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiContactPointViewer` constructor (engine map), `(this, world)`: the base constructor
/// (`00ddccb0`) runs, then the vtables are stored: first `0x0104dd7c` at `+0x28` (overwritten at
/// once), then `0x010d1f0c` at `+0`, `0x010d1eec` at `+8`, `0x010d1edc` at `+0x20` and
/// `0x010d1ebc` at `+0x28`. Returns `this`.
pub fn hkp_toi_contact_point_viewer_construct(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let object = this.addr();
    e.call(VIEWER_BASE_CONSTRUCT, &args![this, world]);
    e.mem.set_u32(object + 0x28, 0x0104_dd7c);
    e.mem.set_u32(object, 0x010d_1f0c);
    e.mem.set_u32(object + 8, 0x010d_1eec);
    e.mem.set_u32(object + 0x20, 0x010d_1edc);
    e.mem.set_u32(object + 0x28, 0x010d_1ebc);
    this
}

// Translated from 00d68310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiContactPointViewer::init` (engine map): while the pointer at `this + 0x1c` (the
/// world) is not zero and the index is below the signed count at `[world + 0x70]`, the object at
/// `this + 0x18` is called (virtual `4`, `(item)`) with the item `[world + 0x6c] + 4 * index`
/// (the world is read again after every call).
pub fn hkp_toi_contact_point_viewer_init(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    let mut index = 0i32;
    let mut world = e.mem.u32(object + 0x1c);
    while world != 0 && index < e.mem.u32(world + 0x70) as i32 {
        let items = e.mem.u32(world + 0x6c);
        let item = e
            .mem
            .u32(items.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(object + 0x18, 4, &args![item]);
        world = e.mem.u32(object + 0x1c);
        index = index.wrapping_add(1);
    }
}

// Translated from 00d68350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiContactPointViewer` destructor body (no engine-map name): the vtables are stored
/// (`0x010d1f0c` at `+0`, `0x010d1eec` at `+8`, `0x010d1edc` at `+0x20`, `0x010d1ebc` at
/// `+0x28`), then while the pointer at `+0x24` (the world) is not zero and the index is below
/// the signed count at `[world + 0x70]`, `00d682a0` runs on `this + 0x20` with the item
/// `[world + 0x6c] + 4 * index`; `0x0104dd7c` is stored at `+0x28` and the base destructor
/// (`00ddcc60`, `this`) is run.
pub fn fn_00d68350(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, 0x010d_1f0c);
    e.mem.set_u32(object + 8, 0x010d_1eec);
    e.mem.set_u32(object + 0x20, 0x010d_1edc);
    e.mem.set_u32(object + 0x28, 0x010d_1ebc);
    let mut index = 0i32;
    if e.mem.u32(object + 0x24) != 0 {
        loop {
            let world = e.mem.u32(object + 0x24);
            if index >= e.mem.u32(world + 0x70) as i32 {
                break;
            }
            let items = e.mem.u32(world + 0x6c);
            let item = e
                .mem
                .u32(items.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(0x00d6_82a0, &args![object + 0x20, item]);
            index = index.wrapping_add(1);
            if e.mem.u32(object + 0x24) == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(object + 0x28, 0x0104_dd7c);
    e.call(0x00dd_cc60, &args![this]);
}

// Translated from 00d683b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkToiContactPointViewer_displayArrow` (engine map). Register inputs first: `handler` in `ECX`
/// (the display handler) and `direction` in `EAX` (a pointer to the arrow's vector); the stack
/// words follow (cdecl, the caller removes them): `start` (a pointer to the start position),
/// `color`, `id` and `tag`. Nothing happens unless `0x34000000` (`0104da4c`) is less than the
/// squared length of the direction (summed as `z + (y + x)`; NaN fails). Otherwise a profile
/// record (`TtToiDisplayArrow`) is written and the arrow is drawn as three lines through the
/// handler's virtual `0x24` (`(from, to, color, id, tag)`): the shaft from the start to the end
/// `start + direction`, and the two barbs from the end to the head points
/// `m + 0.15 * q` and `m - 0.15 * q`, where `m` is the point `0.85` along the shaft and `q`
/// is the vector perpendicular to the direction (built from its two largest components, the
/// larger one's slot taking the other's value and the other's slot the negated first, then
/// normalised with the refined `RSQRTSS` and scaled to the direction's length). The end record
/// is written afterwards.
pub fn hk_toi_contact_point_viewer_display_arrow(
    e: &mut Engine,
    handler: Ptr,
    direction: Ptr,
    start: Ptr,
    color: u32,
    id: u32,
    tag: u32,
) {
    let handler = handler.addr();
    let start = start.addr();
    let direction = direction.addr();
    let vector = load_vec(e, direction);
    let length_squared = sum3(mul(vector, vector));
    let minimum = e.mem.f32(ARROW_MINIMUM);
    let draws = minimum < length_squared;
    if !draws {
        return;
    }
    profile_record(e, LABEL_DISPLAY_ARROW);
    let origin = load_vec(e, start);
    let end = add(origin, vector);
    // The two largest components: `larger` is the slot of the larger one of x and y; the third
    // is the z slot, unless z is smaller than both, when it is the smaller one of x and y.
    let (smallest_of_xy, larger, smaller) = if vector[0].abs() > vector[1].abs() {
        (vector[1].abs(), 0usize, 1usize)
    } else {
        (vector[0].abs(), 1usize, 0usize)
    };
    let other = if smallest_of_xy > vector[2].abs() {
        smaller
    } else {
        2
    };
    let mut perpendicular = [0.0f32; 4];
    perpendicular[larger] = x87_quiet(vector[other]);
    perpendicular[other] = -x87_quiet(vector[larger]);
    let inverse = inverse_length(e, sum3(mul(perpendicular, perpendicular)));
    let length = sum3(mul(vector, vector)).sqrt();
    let head_start = e.mem.f32(ARROW_HEAD_START);
    let head_plus = e.mem.f32(ARROW_HEAD_PLUS);
    let head_minus = e.mem.f32(ARROW_HEAD_MINUS);
    let side = mul(mul([inverse; 4], perpendicular), [length; 4]);
    let ones = load_vec(e, ONES_VECTOR);
    let head_base = add(
        mul(sub(ones, [head_start; 4]), origin),
        mul([head_start; 4], end),
    );
    let first_barb = add(mul([head_plus; 4], side), head_base);
    let second_barb = add(mul([head_minus; 4], side), head_base);
    e.with_stack(0x30, |e, block| {
        let block = block.addr();
        store_vec(e, block, end);
        store_vec(e, block + 0x10, first_barb);
        store_vec(e, block + 0x20, second_barb);
        e.vcall(handler, 0x24, &args![start, block, color, id, tag]);
        e.vcall(handler, 0x24, &args![block, block + 0x10, color, id, tag]);
        e.vcall(handler, 0x24, &args![block, block + 0x20, color, id, tag]);
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d68670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws the arrow of a time-of-impact event (no engine-map name), `(this, event)`: when the
/// signed word at `[event + 0x10]` is not above 0, the arrow of `[event + 0x14]` is drawn
/// (`00d683b0`) through the handler at `[this - 0x10]`: the start is the pointer
/// `[event + 0x14]`, the direction the vector 0x10 bytes after it, the color `0xffff0000`, the id
/// `[event + 4] + 0x10` and the tag the word at `01269004`.
pub fn fn_00d68670(e: &mut Engine, this: Ptr, event: Ptr) {
    let event = event.addr();
    if (e.mem.u32(event + 0x10) as i32) > 0 {
        return;
    }
    let contact = e.mem.u32(event + 0x14);
    let id = e.mem.u32(event + 4).wrapping_add(0x10);
    let handler = e.mem.u32(this.addr().wrapping_sub(0x10));
    let tag = e.mem.u32(PROCESS_TAG);
    e.call(
        0x00d6_83b0,
        &args![
            handler,
            contact.wrapping_add(0x10),
            contact,
            ARROW_COLOR,
            id,
            tag
        ],
    );
}

// Translated from 00d686f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpToiContactPointViewer::registerViewer` (engine map): registers the process
/// `Contact Points (TOI)` with the factory (`01269cd0`, creator `00d686b0`) and stores the
/// id it returns at `01269004`.
pub fn hkp_toi_contact_point_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, 0x010d_1ea0u32, 0x00d6_86b0u32],
        )
        .u32();
    e.mem.set_u32(PROCESS_TAG, id);
}

// Translated from 00d686b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x2c bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d682d0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d686f6.
pub fn fn_00d686b0(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x2c]).u32();
    e.mem.set_u16(block + 4, 0x2c);
    let object = e.call(0x00d6_82d0, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_8260, hkp_toi_contact_point_viewer_get_process_tag() -> u32),
        entry!(0x00d6_8270, fn_00d68270(Ptr, Ptr) -> u32),
        entry!(0x00d6_82a0, fn_00d682a0(Ptr, Ptr) -> u32),
        entry!(0x00d6_82d0, hkp_toi_contact_point_viewer_construct(Ptr, u32) -> Ptr),
        entry!(0x00d6_8310, hkp_toi_contact_point_viewer_init(Ptr)),
        entry!(0x00d6_8350, fn_00d68350(Ptr)),
        entry!(
            0x00d6_83b0,
            hk_toi_contact_point_viewer_display_arrow(Ptr, Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x00d6_8670, fn_00d68670(Ptr, Ptr)),
        entry!(0x00d6_86f0, hkp_toi_contact_point_viewer_register_viewer()),
        entry!(0x00d6_8710, fn_00d68710(Ptr, u32) -> u32),
        entry!(0x00d6_8720, fn_00d68720(Ptr, u32) -> u32),
        entry!(0x00d6_86b0, fn_00d686b0(u32) -> u32),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d68710() {
        assert_eq!(check(0x00d6_8710, 0x00d6_8740, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d68720() {
        assert_eq!(check(0x00d6_8720, 0x00d6_8740, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use std::cell::RefCell;
    use std::rc::Rc;

    const HANDLER_BASE: u32 = 0x0900_0000;
    const TAG: u32 = 0x66;

    /// The line the handler was asked to draw: both ends and (color, id, tag).
    type Line = ([f32; 4], [f32; 4], [u32; 3]);

    #[test]
    fn the_process_tag_is_the_stored_word() {
        let mut e = engine();
        e.mem.set_u32(PROCESS_TAG, 0x1357);
        assert_eq!(e.call(0x00d6_8260, &args![]).u32(), 0x1357);
    }

    #[test]
    fn the_listener_calls_forward_this_plus_eight_or_zero_for_a_null_base() {
        for (address, callee) in [(0x00d6_8270, 0x00c9_3a20), (0x00d6_82a0, 0x00c9_1a20)] {
            let mut e = engine();
            let log = new_log();
            log_double(&mut e, &log, callee, 5);
            assert_eq!(e.call(address, &args![0x120u32, 0x5000u32]).u32(), 5);
            assert_eq!(e.call(address, &args![0x20u32, 0x6000u32]).u32(), 5);
            assert_eq!(
                calls(&log, callee),
                vec![vec![0x5000, 0x128], vec![0x6000, 0]]
            );
        }
    }

    #[test]
    fn the_constructor_runs_the_base_constructor_and_stores_the_vtables() {
        let mut e = engine();
        let base = new_log();
        log_double(&mut e, &base, VIEWER_BASE_CONSTRUCT, 0);
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00d6_82d0, &args![this, 0x4242u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&base, VIEWER_BASE_CONSTRUCT),
            vec![vec![this, 0x4242]]
        );
        let words: Vec<u32> = [0, 8, 0x20, 0x28]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![0x010d_1f0c, 0x010d_1eec, 0x010d_1edc, 0x010d_1ebc]
        );
    }

    /// A world whose item array (at `+0x6c`, count at `+0x70`) holds `items`.
    fn world_with_items(e: &mut Engine, items: &[u32]) -> u32 {
        let world = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x20);
        for (position, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * position as u32, *item);
        }
        e.mem.set_u32(world + 0x6c, data);
        e.mem.set_u32(world + 0x70, items.len() as u32);
        world
    }

    #[test]
    fn init_calls_the_object_for_each_item_while_the_world_stays_set() {
        let mut e = engine();
        let log = new_log();
        let object = logging_object(&mut e, &log, HANDLER_BASE);
        let world = world_with_items(&mut e, &[0xa1, 0xa2, 0xa3]);
        // The object at `this + 0x18` has the logging object's vtable as its first word.
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this + 0x18, vtable);
        e.mem.set_u32(this + 0x1c, world);
        e.call(0x00d6_8310, &args![this]);
        assert_eq!(
            calls(&log, HANDLER_BASE + 4),
            vec![
                vec![this + 0x18, 0xa1],
                vec![this + 0x18, 0xa2],
                vec![this + 0x18, 0xa3]
            ]
        );
    }

    #[test]
    fn init_stops_when_a_call_clears_the_world() {
        let mut e = engine();
        let log = new_log();
        let object = logging_object(&mut e, &log, HANDLER_BASE);
        let world = world_with_items(&mut e, &[0xa1, 0xa2, 0xa3]);
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this + 0x18, vtable);
        e.mem.set_u32(this + 0x1c, world);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(HANDLER_BASE + 4, move |e, a| {
            sink.borrow_mut().push(a[1]);
            if a[1] == 0xa2 {
                e.mem.set_u32(this + 0x1c, 0);
            }
            Ret::default()
        });
        e.call(0x00d6_8310, &args![this]);
        assert_eq!(*seen.borrow(), vec![0xa1, 0xa2]);
        // With no world nothing is called.
        e.call(0x00d6_8310, &args![this]);
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn the_destructor_removes_each_item_and_runs_the_base_destructor() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00d6_82a0, 0);
        log_double(&mut e, &log, 0x00dd_cc60, 0);
        let world = world_with_items(&mut e, &[0xa1, 0xa2]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, world);
        e.call(0x00d6_8350, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x00d6_82a0, vec![this + 0x20, 0xa1]),
                (0x00d6_82a0, vec![this + 0x20, 0xa2]),
                (0x00dd_cc60, vec![this]),
            ]
        );
        let words: Vec<u32> = [0, 8, 0x20, 0x28]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![0x010d_1f0c, 0x010d_1eec, 0x010d_1edc, 0x0104_dd7c]
        );
        // Without a world only the base destructor runs.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00d6_8350, &args![this]);
        assert_eq!(*log.borrow(), vec![(0x00dd_cc60, vec![this])]);
    }

    /// Sets the constants of the arrow and the normalising code.
    fn arrow_engine() -> Engine {
        let mut e = engine();
        e.set_global(ARROW_MINIMUM, 1.1920929e-7f32);
        e.set_global(ARROW_HEAD_START, 0.85f32);
        e.set_global(ARROW_HEAD_PLUS, 0.15f32);
        e.set_global(ARROW_HEAD_MINUS, -0.15f32);
        for lane in 0..4 {
            e.set_global(ONES_VECTOR + 4 * lane, 1.0f32);
        }
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e
    }

    /// Replaces the handler's line slot with a double that keeps both ends.
    fn record_lines(e: &mut Engine) -> Rc<RefCell<Vec<Line>>> {
        let lines = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&lines);
        e.register_double(HANDLER_BASE + 0x24, move |e, a| {
            let from = load_vec(e, a[1]);
            let to = load_vec(e, a[2]);
            sink.borrow_mut().push((from, to, [a[3], a[4], a[5]]));
            Ret::default()
        });
        lines
    }

    fn vector_at(e: &mut Engine, value: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        store_vec(e, at, value);
        at
    }

    fn near(actual: [f32; 4], expected: [f32; 4]) {
        for (got, want) in actual.iter().zip(expected) {
            assert!((got - want).abs() < 1e-4, "{actual:?} != {expected:?}");
        }
    }

    /// Draws the arrow of `direction` from `start` and returns the lines and the profile
    /// labels.
    fn draw(direction: [f32; 4], start: [f32; 4]) -> (Vec<Line>, Vec<u32>) {
        let mut e = arrow_engine();
        let log = new_log();
        let handler = logging_object(&mut e, &log, HANDLER_BASE);
        let lines = record_lines(&mut e);
        let (stream, records) = monitor(&mut e, 0x60);
        let direction = vector_at(&mut e, direction);
        let start = vector_at(&mut e, start);
        e.call(
            0x00d6_83b0,
            &args![handler, direction, start, 0xff00_ff00u32, 0x31u32, TAG],
        );
        let drawn = lines.borrow().clone();
        (drawn, profile_labels(&e, stream, records))
    }

    /// The shaft runs from the start to the end and both barbs from the end; the barbs are
    /// symmetric about the point 0.85 along the shaft, perpendicular to the direction and as
    /// long as it is, scaled by 0.15.
    fn check_arrow(direction: [f32; 4], start: [f32; 4]) {
        let (lines, labels) = draw(direction, start);
        assert_eq!(labels, vec![LABEL_DISPLAY_ARROW, PROFILE_END]);
        assert_eq!(lines.len(), 3);
        let end: [f32; 4] = std::array::from_fn(|lane| start[lane] + direction[lane]);
        assert_eq!(lines[0].0, start);
        near(lines[0].1, end);
        for line in &lines {
            assert_eq!(line.2, [0xff00_ff00, 0x31, TAG]);
        }
        near(lines[1].0, end);
        near(lines[2].0, end);
        let base: [f32; 4] = std::array::from_fn(|lane| 0.15 * start[lane] + 0.85 * end[lane]);
        let length = (direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2])
            .sqrt();
        let side: [f32; 4] = std::array::from_fn(|lane| (lines[1].1[lane] - base[lane]) / 0.15);
        let opposite: [f32; 4] = std::array::from_fn(|lane| (base[lane] - lines[2].1[lane]) / 0.15);
        near(side, opposite);
        let dot: f32 = (0..3).map(|lane| side[lane] * direction[lane]).sum();
        assert!(dot.abs() < 1e-3, "{dot}");
        let side_length = (side[0] * side[0] + side[1] * side[1] + side[2] * side[2]).sqrt();
        assert!(
            (side_length - length).abs() < 1e-3,
            "{side_length} {length}"
        );
    }

    #[test]
    fn the_arrow_has_a_shaft_and_two_symmetric_barbs() {
        // The perpendicular is (-4, 3, 0) scaled by the length 5, so the barbs are
        // 0.15 * (-4, 3, 0) off the point 0.85 along the shaft.
        let (lines, _) = draw([3.0, 4.0, 0.0, 0.0], [1.0, 1.0, 1.0, 0.0]);
        near(lines[0].1, [4.0, 5.0, 1.0, 0.0]);
        near(lines[1].1, [3.55 - 0.6, 4.4 + 0.45, 1.0, 0.0]);
        near(lines[2].1, [3.55 + 0.6, 4.4 - 0.45, 1.0, 0.0]);
    }

    #[test]
    fn the_perpendicular_uses_the_two_largest_components() {
        // z smallest, y larger than x; z largest; x largest.
        check_arrow([3.0, 4.0, 0.0, 0.0], [1.0, 2.0, 3.0, 0.0]);
        check_arrow([1.0, 2.0, 5.0, 0.0], [0.0, -1.0, 2.0, 0.0]);
        check_arrow([5.0, 2.0, 1.0, 0.0], [-3.0, 0.5, 0.0, 0.0]);
        // Equal components take the other branches of the comparisons.
        check_arrow([2.0, 2.0, 2.0, 0.0], [0.0; 4]);
        check_arrow([-2.0, 4.0, -4.0, 0.0], [1.0; 4]);
    }

    #[test]
    fn a_tiny_or_nan_direction_draws_nothing() {
        for direction in [[1.0e-4, 0.0, 0.0, 0.0], [0.0; 4], [f32::NAN, 1.0, 1.0, 0.0]] {
            let (lines, labels) = draw(direction, [0.0; 4]);
            assert!(lines.is_empty());
            assert!(labels.is_empty());
        }
    }

    /// The event: `[+0x10]` is `level`, `[+0x14]` the contact (start, direction at `+0x10`)
    /// and `[+4]` an object whose address plus 0x10 is the id.
    fn make_event(e: &mut Engine, level: u32) -> (u32, u32, u32) {
        let contact = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x20);
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 4, owner);
        e.mem.set_u32(event + 0x10, level);
        e.mem.set_u32(event + 0x14, contact);
        (event, contact, owner)
    }

    #[test]
    fn the_event_arrow_is_drawn_only_when_the_level_is_not_above_zero() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00d6_83b0, 0);
        e.mem.set_u32(PROCESS_TAG, TAG);
        let this = e.mem.alloc(0x40);
        let viewer = this + 0x10;
        e.mem.set_u32(this, 0x7000);
        for (level, drawn) in [(0u32, true), (0xffff_fffe, true), (1, false), (7, false)] {
            log.borrow_mut().clear();
            let (event, contact, owner) = make_event(&mut e, level);
            e.call(0x00d6_8670, &args![viewer, event]);
            let want: Vec<Vec<u32>> = if drawn {
                vec![vec![
                    0x7000,
                    contact + 0x10,
                    contact,
                    ARROW_COLOR,
                    owner + 0x10,
                    TAG,
                ]]
            } else {
                vec![]
            };
            assert_eq!(calls(&log, 0x00d6_83b0), want);
        }
    }

    #[test]
    fn register_viewer_stores_the_id_the_factory_returns() {
        let mut e = engine();
        let factory_calls: Log = new_log();
        log_double(&mut e, &factory_calls, REGISTER_PROCESS, 0x4d);
        e.mem.set_u32(PROCESS_FACTORY, 0x5000);
        e.call(0x00d6_86f0, &args![]);
        assert_eq!(
            calls(&factory_calls, REGISTER_PROCESS),
            vec![vec![0x5000, 0x010d_1ea0, 0x00d6_86b0]]
        );
        assert_eq!(e.mem.u32(PROCESS_TAG), 0x4d);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00d686b0_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_82d0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_86b0, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x2c]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x2c);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d686b0_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_82d0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_86b0, &args![0x1234u32]).u32(), 0);
    }
}
