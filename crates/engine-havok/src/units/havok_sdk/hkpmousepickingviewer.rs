//! `Havok SDK/hkpmousepickingviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMousePickingViewer` (the Xbox PDB has no record of the Havok classes, so the field
//! names below describe what the code does and are unconfirmed): the world of the picked
//! entity is the word at +0x28, the spring action holding it the word at +0x2C and the
//! maximum relative force a float at +0x30. A second interface object sits at +8 (vtable
//! `010d1cc4`) with the input stream at +0x10, and a third at +0x20 (vtable `010d1cb4`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the viewer itself (`010d1ce4`) and of its two inner interface objects
/// (`010d1cc4` at +8 and `010d1cb4` at +0x20).
const VIEWER_VTABLE: u32 = 0x010d_1ce4;
const SECOND_VTABLE: u32 = 0x010d_1cc4;
const THIRD_VTABLE: u32 = 0x010d_1cb4;
/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpWorld::removeAction` (`00c97f50`: world, action) and `addAction` (`00c97370`: world,
/// action); `hkpEntity::activate` (`00c9c1d0`); the reference-count release (`00c905b0`).
const REMOVE_ACTION: u32 = 0x00c9_7f50;
const ADD_ACTION: u32 = 0x00c9_7370;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkVector4::setTransformedInversePos` (`00c7f690`: result, transform, vector).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// `hkpMouseSpringAction::hkpMouseSpringAction_ov2` (`00cbb560`) and `setMaxRelativeForce`
/// (`00cbb1b0`, one float).
const SPRING_CONSTRUCT: u32 = 0x00cb_b560;
const SPRING_SET_MAX_FORCE: u32 = 0x00cb_b1b0;
/// The float constants the spring is built with: 0.5, 0.3 and 0.95.
const SPRING_FIRST_FLOAT: u32 = 0x0101_6248;
const SPRING_SECOND_FLOAT: u32 = 0x0101_7870;
const SPRING_THIRD_FLOAT: u32 = 0x0101_ffc0;
/// `hkProcessFactory::registerProcess` (`00dd9380`: factory, name, creation function), the
/// factory pointer (`01269cd0`), the id the registration returns (`01268ff8`), the name
/// `"Mouse Picking"` (`010d1ca0`) and the creation function (`00d66c50`).
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const PROCESS_ID: u32 = 0x0126_8ff8;
const VIEWER_NAME: u32 = 0x010d_1ca0;
const VIEWER_CREATOR: u32 = 0x00d6_6c50;
/// `hkDisplaySerializeIStream::readQuadVector4` (`00ddac60`: stream, vector),
/// `hkIArchive::readArrayGeneric` (`00dd8fe0`: stream, buffer, size, count) and
/// `hkIArchive::isOk` (`00dd90e0`: stream, result byte; returns a pointer to the result).
const READ_QUAD_VECTOR: u32 = 0x00dd_ac60;
const READ_ARRAY: u32 = 0x00dd_8fe0;
const IS_OK: u32 = 0x00dd_90e0;
/// The base destructor the viewer jumps to (`00ddcc60`).
const BASE_DESTRUCT: u32 = 0x00dd_cc60;

// Translated from 00d66c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMousePickingViewer::releaseObject` (Xbox PDB; fastcall): when the spring action at
/// +0x2C exists, removes it from the world at +0x28 (and activates the entity at action
/// `+0x18`) if the action's word at +8 is not zero, drops the reference to it and clears
/// +0x2C. Clears +0x28 in any case.
pub fn hkp_mouse_picking_viewer_release_object(e: &mut Engine, this: Ptr) {
    let viewer = this.addr();
    let action = e.mem.u32(viewer + 0x2c);
    if action != 0 {
        if e.mem.u32(action + 8) != 0 {
            let world = e.mem.u32(viewer + 0x28);
            e.call(REMOVE_ACTION, &args![world, action]);
            let action = e.mem.u32(viewer + 0x2c);
            let entity = e.mem.u32(action + 0x18);
            e.call(ENTITY_ACTIVATE, &args![entity]);
        }
        let action = e.mem.u32(viewer + 0x2c);
        e.call(RELEASE_REFERENCE, &args![action]);
        e.mem.set_u32(viewer + 0x2c, 0);
    }
    e.mem.set_u32(viewer + 0x28, 0);
}

// Translated from 00d66c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMousePickingViewer::registerViewer` (Xbox PDB): registers the process `"Mouse Picking"`
/// with the creation function `00d66c50` at the factory the global `01269cd0` points to
/// (`registerProcess`, `this` the factory) and stores the returned id in the global
/// `01268ff8`.
pub fn hkp_mouse_picking_viewer_register_viewer(e: &mut Engine) {
    let factory = e.mem.u32(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, VIEWER_NAME, VIEWER_CREATOR],
        )
        .u32();
    e.mem.set_u32(PROCESS_ID, id);
}

// Translated from 00d66cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` in `ECX`, `RET 4`): `this` is the interface object at +0x20 of the viewer;
/// when `world` is the word at `this + 8` (the viewer's +0x28, the world it picked in),
/// releases the object of the viewer (`this - 0x20`, `releaseObject`).
pub fn fn_00d66cb0(e: &mut Engine, this: Ptr, world: u32) {
    if world == e.mem.u32(this.addr() + 8) {
        hkp_mouse_picking_viewer_release_object(e, Ptr::new(this.addr().wrapping_sub(0x20)));
    }
}

// Translated from 00d66cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (fastcall; the destructor of the viewer by its body): stores the vtables
/// `010d1ce4`, `010d1cc4` (at +8) and `010d1cb4` (at +0x20), releases the picked object
/// (`releaseObject`) and jumps to the base destructor `00ddcc60` with the same `this`.
pub fn fn_00d66cd0(e: &mut Engine, this: Ptr) {
    let viewer = this.addr();
    e.mem.set_u32(viewer, VIEWER_VTABLE);
    e.mem.set_u32(viewer + 8, SECOND_VTABLE);
    e.mem.set_u32(viewer + 0x20, THIRD_VTABLE);
    hkp_mouse_picking_viewer_release_object(e, this);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00d66d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMousePickingViewer::pickObject` (Xbox PDB; `this` in `ECX`, `RET 0x10`): `handle` is a
/// tagged pointer: tag 0 is taken as it is, tag 3 has its two low bits cleared and any other
/// tag stores 0 in the byte `result` and returns. When the byte at `handle + 0x18` is 1 the
/// entity is `handle` plus the signed byte at `handle + 0x10` (it must not be zero); its word
/// at +8 (the world) goes to `this + 0x28`, and unless the byte at entity `+0xE8` is 5 a
/// 0x60-byte spring action is allocated and built with
/// `(setTransformedInversePos(entity + 0xF0, position), position, 0.5, 0.3, 0.95, entity, 0)`,
/// stored at `this + 0x2C`, given `[this + 0x30]` as its maximum relative force and added to
/// the world. The byte `result` is 1 on every other path. The third word is not used.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_mouse_picking_viewer_pick_object(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    handle: u32,
    _unused_0: u32,
    position: Ptr,
) -> Ptr {
    let viewer = this.addr();
    let handle = match handle & 3 {
        3 => handle & !3,
        0 => handle,
        _ => {
            e.mem.set_u8(result.addr(), 0);
            return result;
        }
    };
    if e.mem.u8(handle + 0x18) == 1 {
        let entity = i32::from(e.mem.u8(handle + 0x10) as i8) as u32;
        let entity = entity.wrapping_add(handle);
        if entity != 0 {
            let world = e.mem.u32(entity + 8);
            e.mem.set_u32(viewer + 0x28, world);
            if e.mem.u8(entity + 0xe8) != 5 && e.mem.u32(entity + 8) == world {
                e.with_stack(0x10, |e, local| {
                    let local = local.addr();
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![local, entity + 0xf0, position],
                    );
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let allocator = e.mem.u32(router + 0x10);
                    let block = e.vcall(allocator, 4, &args![0x60u32]).u32();
                    e.mem.set_u16(block + 4, 0x60);
                    let first = x87_quiet_bits(e.mem.u32(SPRING_FIRST_FLOAT));
                    let second = x87_quiet_bits(e.mem.u32(SPRING_SECOND_FLOAT));
                    let third = x87_quiet_bits(e.mem.u32(SPRING_THIRD_FLOAT));
                    let spring = e
                        .call(
                            SPRING_CONSTRUCT,
                            &args![block, local, position, first, second, third, entity, 0u32],
                        )
                        .u32();
                    let force = x87_quiet_bits(e.mem.u32(viewer + 0x30));
                    e.mem.set_u32(viewer + 0x2c, spring);
                    e.call(SPRING_SET_MAX_FORCE, &args![spring, force]);
                    let spring = e.mem.u32(viewer + 0x2c);
                    let world = e.mem.u32(viewer + 0x28);
                    e.call(ADD_ACTION, &args![world, spring]);
                });
            }
        }
    }
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00d66e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMousePickingViewer::consumeCommand` (Xbox PDB; `RET 4`): `this` is the interface object
/// at +8 of the viewer, whose word at +8 is the input stream. Command `0xB0`: reads a vector
/// (`readQuadVector4`) and an 8-byte pair (`readArrayGeneric` with size 8, count 1) and, when
/// the stream is still ok, calls `pickObject` on the viewer (`this - 8`) with the pair's two
/// words and the vector. Command `0xB1`: reads a vector and, when ok, sets the mouse
/// position (`fn 00d66bf0`) of the viewer. Command `0xB2`: releases the picked object. Other
/// commands do nothing.
pub fn hkp_mouse_picking_viewer_consume_command(e: &mut Engine, this: Ptr, command: u8) {
    let interface = this.addr();
    let viewer = interface.wrapping_sub(8);
    match command {
        0xb0 => {
            e.with_stack(0x30, |e, frame| {
                // Frame: `+0xD` / `+0xE` / `+0xF` the status and result bytes, `+0x10` the pair,
                // `+0x20` the vector.
                let f = frame.addr();
                let stream = e.mem.u32(interface + 8);
                e.call(READ_QUAD_VECTOR, &args![stream, f + 0x20]);
                let stream = e.mem.u32(interface + 8);
                e.call(READ_ARRAY, &args![stream, f + 0x10, 8u32, 1u32]);
                let stream = e.mem.u32(interface + 8);
                let ok = e.call(IS_OK, &args![stream, f + 0xe]).u32();
                if e.mem.u8(ok) != 0 {
                    let first = e.mem.u32(f + 0x10);
                    let second = e.mem.u32(f + 0x14);
                    hkp_mouse_picking_viewer_pick_object(
                        e,
                        Ptr::new(viewer),
                        Ptr::new(f + 0xf),
                        first,
                        second,
                        Ptr::new(f + 0x20),
                    );
                }
            });
        }
        0xb1 => {
            e.with_stack(0x30, |e, frame| {
                let f = frame.addr();
                let stream = e.mem.u32(interface + 8);
                e.call(READ_QUAD_VECTOR, &args![stream, f + 0x10]);
                let stream = e.mem.u32(interface + 8);
                let ok = e.call(IS_OK, &args![stream, f + 0xd]).u32();
                if e.mem.u8(ok) != 0 {
                    e.call(0x00d6_6bf0, &args![viewer, f + 0x10]);
                }
            });
        }
        0xb2 => hkp_mouse_picking_viewer_release_object(e, Ptr::new(viewer)),
        _ => {}
    }
}

// Translated from 00d66c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x34 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d66b90` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d66c96.
pub fn fn_00d66c50(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x34]).u32();
    e.mem.set_u16(block + 4, 0x34);
    let object = e.call(0x00d6_6b90, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_6c10, hkp_mouse_picking_viewer_release_object(Ptr)),
        entry!(0x00d6_6c90, hkp_mouse_picking_viewer_register_viewer()),
        entry!(0x00d6_6cb0, fn_00d66cb0(Ptr, u32)),
        entry!(0x00d6_6cd0, fn_00d66cd0(Ptr)),
        entry!(
            0x00d6_6d00,
            hkp_mouse_picking_viewer_pick_object(Ptr, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00d6_6e10,
            hkp_mouse_picking_viewer_consume_command(Ptr, u8)
        ),
        entry!(0x00d6_6c50, fn_00d66c50(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Doubles over callees that log their arguments and return 0.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn new_log() -> Log {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn release_object_removes_activates_and_drops_the_action() {
        let mut e = engine();
        let log = new_log();
        log_callees(
            &mut e,
            &log,
            &[REMOVE_ACTION, ENTITY_ACTIVATE, RELEASE_REFERENCE],
        );
        let viewer = e.mem.alloc(0x40);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 8, 1);
        e.mem.set_u32(action + 0x18, 0x99);
        e.mem.set_u32(viewer + 0x28, 0x77);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6c10, &args![viewer]);
        assert_eq!(calls_to(&log, REMOVE_ACTION), vec![vec![0x77, action]]);
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![0x99]]);
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![action]]);
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
    }

    #[test]
    fn release_object_of_an_action_outside_the_world_only_drops_it() {
        let mut e = engine();
        let log = new_log();
        log_callees(
            &mut e,
            &log,
            &[REMOVE_ACTION, ENTITY_ACTIVATE, RELEASE_REFERENCE],
        );
        let viewer = e.mem.alloc(0x40);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x28, 0x77);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6c10, &args![viewer]);
        assert!(calls_to(&log, REMOVE_ACTION).is_empty());
        assert!(calls_to(&log, ENTITY_ACTIVATE).is_empty());
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![action]]);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
    }

    #[test]
    fn release_object_without_an_action_clears_the_world_word() {
        let mut e = engine();
        let log = new_log();
        log_callees(&mut e, &log, &[RELEASE_REFERENCE]);
        let viewer = e.mem.alloc(0x40);
        e.mem.set_u32(viewer + 0x28, 0x77);
        e.call(0x00d6_6c10, &args![viewer]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
    }

    #[test]
    fn register_viewer_stores_the_id_the_factory_returns() {
        let mut e = engine();
        e.mem.set_u32(PROCESS_FACTORY, 0x4321);
        e.register_double(REGISTER_PROCESS, |_, _| Ret {
            eax: 0x1357,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d6_6c90, &args![]);
        assert_eq!(e.mem.u32(PROCESS_ID), 0x1357);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|call| call.0 == REGISTER_PROCESS)
            .map(|call| call.1)
            .collect();
        assert_eq!(calls, [vec![0x4321, VIEWER_NAME, VIEWER_CREATOR]]);
    }

    #[test]
    fn the_inner_release_callback_releases_only_for_its_own_world() {
        let mut e = engine();
        let log = new_log();
        log_callees(&mut e, &log, &[RELEASE_REFERENCE]);
        let viewer = e.mem.alloc(0x40);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x28, 0x555);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6cb0, &args![viewer + 0x20, 0x556u32]);
        assert!(log.borrow().is_empty());
        e.call(0x00d6_6cb0, &args![viewer + 0x20, 0x555u32]);
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![action]]);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
    }

    #[test]
    fn the_destructor_sets_the_vtables_and_jumps_to_the_base() {
        let mut e = engine();
        let log = new_log();
        log_callees(&mut e, &log, &[RELEASE_REFERENCE, BASE_DESTRUCT]);
        let viewer = e.mem.alloc(0x40);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6cd0, &args![viewer]);
        assert_eq!(e.mem.u32(viewer), VIEWER_VTABLE);
        assert_eq!(e.mem.u32(viewer + 8), SECOND_VTABLE);
        assert_eq!(e.mem.u32(viewer + 0x20), THIRD_VTABLE);
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![action]]);
        assert_eq!(calls_to(&log, BASE_DESTRUCT), vec![vec![viewer]]);
    }

    /// A viewer, a handle whose entity lies 0x40 bytes above it (the signed byte at +0x10) and
    /// the doubles pickObject calls. Returns the engine, the log, the viewer, the handle, the
    /// entity and the position vector.
    fn pick_setup() -> (Engine, Log, u32, u32, u32, u32) {
        let mut e = engine();
        e.set_global(SPRING_FIRST_FLOAT, 0.5f32);
        e.set_global(SPRING_SECOND_FLOAT, 0.3f32);
        e.set_global(SPRING_THIRD_FLOAT, 0.95f32);
        let log = new_log();
        log_callees(
            &mut e,
            &log,
            &[
                SET_TRANSFORMED_INVERSE_POS,
                SPRING_SET_MAX_FORCE,
                ADD_ACTION,
            ],
        );
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0041);
        e.register_double(0x0900_0041, |e, a| {
            assert_eq!(a[1], 0x60);
            Ret {
                eax: e.mem.alloc(0x60),
                ..Ret::default()
            }
        });
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SPRING_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((SPRING_CONSTRUCT, a.to_vec()));
            assert_eq!(e.mem.u16(a[0] + 4), 0x60);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let viewer = e.mem.alloc(0x40);
        e.mem.set_f32(viewer + 0x30, 100.0);
        let handle = e.mem.alloc(0x40);
        let entity = e.mem.alloc(0x100);
        e.mem.set_u8(handle + 0x18, 1);
        e.mem
            .set_u8(handle + 0x10, (entity.wrapping_sub(handle)) as u8);
        e.mem.set_u32(entity + 8, 0x7777);
        let position = e.mem.alloc(0x10);
        (e, log, viewer, handle, entity, position)
    }

    #[test]
    fn pick_object_builds_a_spring_for_the_entity() {
        let (mut e, log, viewer, handle, entity, position) = pick_setup();
        let result = e.mem.alloc(4);
        e.call(
            0x00d6_6d00,
            &args![viewer, result, handle, 0xdeadu32, position],
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(viewer + 0x28), 0x7777);
        let construct = calls_to(&log, SPRING_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        let spring = construct[0][0];
        assert_eq!(e.mem.u32(viewer + 0x2c), spring);
        assert_eq!(
            construct[0][2..],
            [
                position,
                0.5f32.to_bits(),
                0.3f32.to_bits(),
                0.95f32.to_bits(),
                entity,
                0
            ]
        );
        assert_eq!(
            calls_to(&log, SET_TRANSFORMED_INVERSE_POS),
            vec![vec![construct[0][1], entity + 0xf0, position]]
        );
        assert_eq!(
            calls_to(&log, SPRING_SET_MAX_FORCE),
            vec![vec![spring, 100.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, ADD_ACTION), vec![vec![0x7777, spring]]);
    }

    #[test]
    fn fn_00d66d00_answers_its_result_pointer() {
        let (mut e, _log, viewer, handle, _entity, position) = pick_setup();
        let result = e.mem.alloc(4);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(
                0x00d6_6d00,
                &args![viewer, result, handle, 0xdeadu32, position],
            )
            .u32(),
            args![result][0]
        );
    }

    #[test]
    fn pick_object_accepts_a_handle_tagged_with_three() {
        let (mut e, log, viewer, handle, _, position) = pick_setup();
        let result = e.mem.alloc(4);
        e.call(
            0x00d6_6d00,
            &args![viewer, result, handle | 3, 0u32, position],
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(calls_to(&log, SPRING_CONSTRUCT).len(), 1);
    }

    #[test]
    fn pick_object_rejects_other_tags() {
        let (mut e, log, viewer, handle, _, position) = pick_setup();
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 9);
        for tag in [1u32, 2] {
            e.call(
                0x00d6_6d00,
                &args![viewer, result, handle | tag, 0u32, position],
            );
            assert_eq!(e.mem.u8(result), 0);
            e.mem.set_u8(result, 9);
        }
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
    }

    #[test]
    fn pick_object_skips_entities_of_kind_five_and_handles_of_another_kind() {
        let (mut e, log, viewer, handle, entity, position) = pick_setup();
        let result = e.mem.alloc(4);
        e.mem.set_u8(entity + 0xe8, 5);
        e.call(0x00d6_6d00, &args![viewer, result, handle, 0u32, position]);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(viewer + 0x28), 0x7777);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(handle + 0x18, 2);
        e.mem.set_u32(viewer + 0x28, 0);
        e.call(0x00d6_6d00, &args![viewer, result, handle, 0u32, position]);
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
    }

    /// Doubles for a stream whose reads write `vector` and `pair`, with `ok` as the status.
    fn consume_setup(ok: u8) -> (Engine, Log, u32, u32) {
        let mut e = engine();
        let log = new_log();
        let viewer = e.mem.alloc(0x40);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x10, stream);
        let sink = Rc::clone(&log);
        e.register_double(READ_QUAD_VECTOR, move |e, a| {
            sink.borrow_mut().push((READ_QUAD_VECTOR, a.to_vec()));
            for lane in 0..4u32 {
                e.mem.set_u32(a[1] + 4 * lane, 0x1000 + lane);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(READ_ARRAY, move |e, a| {
            sink.borrow_mut().push((READ_ARRAY, a.to_vec()));
            e.mem.set_u32(a[1], 0x2000);
            e.mem.set_u32(a[1] + 4, 0x2001);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(IS_OK, move |e, a| {
            sink.borrow_mut().push((IS_OK, a.to_vec()));
            e.mem.set_u8(a[1], ok);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        log_callees(&mut e, &log, &[0x00d6_6bf0]);
        (e, log, viewer, stream)
    }

    #[test]
    fn command_b0_picks_an_object_when_the_stream_is_ok() {
        let (mut e, log, viewer, stream) = consume_setup(1);
        // The pair read is (0x2000, 0x2001): the handle is its first word. The entity is the
        // handle itself (offset 0), of kind 5 so that no spring is made; its world, 0x7777,
        // lands in the viewer.
        e.map(0x2000, 0x200);
        e.mem.set_u8(0x2018, 1);
        e.mem.set_u32(0x2008, 0x7777);
        e.mem.set_u8(0x20e8, 5);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb0u8]);
        assert_eq!(calls_to(&log, READ_QUAD_VECTOR).len(), 1);
        assert_eq!(calls_to(&log, READ_QUAD_VECTOR)[0][0], stream);
        let read = calls_to(&log, READ_ARRAY);
        assert_eq!(read.len(), 1);
        assert_eq!(read[0][0], stream);
        assert_eq!(read[0][2..], [8, 1]);
        assert_eq!(calls_to(&log, IS_OK)[0][0], stream);
        assert_eq!(e.mem.u32(viewer + 0x28), 0x7777);
    }

    #[test]
    fn command_b0_does_nothing_more_when_the_stream_is_not_ok() {
        let (mut e, log, viewer, _) = consume_setup(0);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb0u8]);
        assert_eq!(calls_to(&log, READ_QUAD_VECTOR).len(), 1);
        assert_eq!(calls_to(&log, READ_ARRAY).len(), 1);
        assert_eq!(e.mem.u32(viewer + 0x28), 0);
    }

    #[test]
    fn command_b1_sets_the_mouse_position_when_the_stream_is_ok() {
        let (mut e, log, viewer, stream) = consume_setup(1);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb1u8]);
        assert_eq!(calls_to(&log, READ_QUAD_VECTOR)[0][0], stream);
        assert!(calls_to(&log, READ_ARRAY).is_empty());
        let set = calls_to(&log, 0x00d6_6bf0);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], viewer);
        assert_eq!(set[0][1], calls_to(&log, READ_QUAD_VECTOR)[0][1]);
    }

    #[test]
    fn command_b1_does_nothing_when_the_stream_is_not_ok() {
        let (mut e, log, viewer, _) = consume_setup(0);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb1u8]);
        assert!(calls_to(&log, 0x00d6_6bf0).is_empty());
    }

    #[test]
    fn command_b2_releases_the_picked_object() {
        let (mut e, log, viewer, _) = consume_setup(1);
        log_callees(&mut e, &log, &[RELEASE_REFERENCE]);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb2u8]);
        assert_eq!(calls_to(&log, RELEASE_REFERENCE), vec![vec![action]]);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
    }

    #[test]
    fn other_commands_do_nothing() {
        let (mut e, log, viewer, _) = consume_setup(1);
        e.call(0x00d6_6e10, &args![viewer + 8, 0xb3u8]);
        e.call(0x00d6_6e10, &args![viewer + 8, 0x30u8]);
        assert!(log.borrow().is_empty());
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
    fn fn_00d66c50_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_6b90, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_6c50, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x34]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x34);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d66c50_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_6b90, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_6c50, &args![0x1234u32]).u32(), 0);
    }
}
