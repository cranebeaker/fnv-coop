//! `NiAnimation/nioldanimationconverter.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTransformController::ms_RTTI` (`011f36ec`, as its static initializer names it).
const NI_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36ec;
/// `NiObjectNET::GetController` (`00a5c570`, thiscall, an `NiRTTI` record),
/// `NiObjectNET::RemoveController` (`00a5c480`, thiscall, the controller) and `00a5c000`
/// (unit `niobjectnet.cpp`, thiscall: sets the object's controller list head, +0xc, counted).
const GET_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
const SET_CONTROLLERS: u32 = 0x00a5_c000;
/// `00aa13e0` (cdecl, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiPathInterpolator::NiPathInterpolator(path data, percent data)` (`00a3f060`, thiscall),
/// `00a3ea40` (unit `nipathinterpolator.cpp`, thiscall, no arguments) and the
/// `NiTransformController` constructor (`00a36560`, thiscall).
const PATH_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_f060;
const PATH_INTERPOLATOR_00A3EA40: u32 = 0x00a3_ea40;
const TRANSFORM_CONTROLLER_CONSTRUCT: u32 = 0x00a3_6560;
/// `00a424e0` (unit unknown in the engine map; cdecl, `(old controller, new controller)`):
/// copies the time controller settings across.
const COPY_CONTROLLER_SETTINGS: u32 = 0x00a4_24e0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Virtual slots: `DeleteThis` (`0x04`); on the controller `GetInterpolator` (`0xc4`, an
/// index), `SetInterpolator` (`0xc8`, interpolator and index) and `SetTarget` (`0x98`); on the
/// interpolator `Collapse` (`0xbc`).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_INTERPOLATOR: u32 = 0xc4;
const SLOT_SET_INTERPOLATOR: u32 = 0xc8;
const SLOT_SET_TARGET: u32 = 0x98;
const SLOT_COLLAPSE: u32 = 0xbc;
/// The old `NiPathController`'s fields as the PC code uses them: the target +0x2c, the next
/// controller +0x30, a flags byte +0x34, the path data +0x40 and percent data +0x44, two
/// floats +0x50 and +0x54, a 16-bit word +0x58 and a word +0x60; and the path interpolator's
/// 16-bit flags +0xc and the fields they go to (+0x28, +0x2c, +0x30, +0x38).
const OLD_TARGET: u32 = 0x2c;
const NEXT_CONTROLLER: u32 = 0x30;
const OLD_FLAGS: u32 = 0x34;
const OLD_PATH_DATA: u32 = 0x40;
const OLD_PERCENT_DATA: u32 = 0x44;
const INTERPOLATOR_FLAGS: u32 = 0x0c;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at +4 and, when it
/// reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// Sets or clears `mask` in the interpolator's 16-bit flags by bit `bit` of the old
/// controller's flags byte (read again each time, as the exe does).
fn copy_flag(e: &mut Engine, old: u32, interpolator: u32, bit: u32, mask: u16) {
    let set = (e.mem.u8(old.wrapping_add(OLD_FLAGS)) >> bit) & 1 != 0;
    let at = interpolator.wrapping_add(INTERPOLATOR_FLAGS);
    let flags = e.mem.u16(at);
    e.mem
        .set_u16(at, if set { flags | mask } else { flags & !mask });
}

// Translated from 00a42900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOldAnimationConverter::ConvertPathController` (Xbox PDB; cdecl, the old controller): an
/// `NiPathInterpolator` (0x5c bytes, `00a3f060` on the old path and percent data; a failed
/// allocation leaves a null interpolator the exe then writes through) takes the old
/// controller's flag bits 2, 3, 4 (also running `00a3ea40`), 5, 6 and 1 as its flag bits
/// 0x4, 0x8, 0x10, 0x20, 0x40 and 0x2, its word +0x38 (from +0x60), floats +0x28/+0x2c (from
/// +0x50/+0x54) and 16-bit word +0x30 (from +0x58). When the target already has an
/// `NiTransformController` (`GetController`, counted) whose interpolator 0 is empty, the
/// interpolator goes there (`SetInterpolator(.., 0)`) and the settings are copied
/// (`00a424e0`). Otherwise a new `NiTransformController` (0x38 bytes) replaces it (counted;
/// the old reference released), gets the interpolator, the target (`SetTarget`) and the
/// settings, and is moved behind the old controller in the target's list (`00a5c000` with
/// the new controller's next, then the new one's next becomes the old one's and the old
/// one's next the new one, all counted). Then the interpolator collapses (slot `0xbc`), the
/// old controller is removed from the target (when there is one) and the controller
/// reference is dropped (`DeleteThis` at zero).
pub fn ni_old_animation_converter_convert_path_controller(e: &mut Engine, old: Ptr) {
    let old = old.addr();
    let target = e.mem.u32(old.wrapping_add(OLD_TARGET));
    let mut controller = e
        .call(GET_CONTROLLER, &args![target, NI_TRANSFORM_CONTROLLER_RTTI])
        .u32();
    if controller != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![controller.wrapping_add(4)]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    let interpolator = if block != 0 {
        let path = e.mem.u32(old.wrapping_add(OLD_PATH_DATA));
        let percent = e.mem.u32(old.wrapping_add(OLD_PERCENT_DATA));
        e.call(PATH_INTERPOLATOR_CONSTRUCT, &args![block, path, percent])
            .u32()
    } else {
        0
    };
    copy_flag(e, old, interpolator, 2, 0x4);
    copy_flag(e, old, interpolator, 3, 0x8);
    let word = e.mem.u32(old.wrapping_add(0x60));
    e.mem.set_u32(interpolator.wrapping_add(0x38), word);
    if (e.mem.u8(old.wrapping_add(OLD_FLAGS)) >> 4) & 1 != 0 {
        let at = interpolator.wrapping_add(INTERPOLATOR_FLAGS);
        e.mem.set_u16(at, e.mem.u16(at) | 0x10);
        e.call(PATH_INTERPOLATOR_00A3EA40, &args![interpolator]);
    } else {
        let at = interpolator.wrapping_add(INTERPOLATOR_FLAGS);
        e.mem.set_u16(at, e.mem.u16(at) & 0xffef);
    }
    copy_flag(e, old, interpolator, 5, 0x20);
    let value = x87_quiet_bits(e.mem.u32(old.wrapping_add(0x50)));
    e.mem.set_u32(interpolator.wrapping_add(0x28), value);
    let value = x87_quiet_bits(e.mem.u32(old.wrapping_add(0x54)));
    e.mem.set_u32(interpolator.wrapping_add(0x2c), value);
    let half = e.mem.u16(old.wrapping_add(0x58));
    e.mem.set_u16(interpolator.wrapping_add(0x30), half);
    copy_flag(e, old, interpolator, 6, 0x40);
    copy_flag(e, old, interpolator, 1, 0x2);
    let reuse = controller != 0
        && e.vcall(controller, SLOT_GET_INTERPOLATOR, &args![0u32])
            .u32()
            == 0;
    if reuse {
        e.vcall(
            controller,
            SLOT_SET_INTERPOLATOR,
            &args![interpolator, 0u32],
        );
        e.call(COPY_CONTROLLER_SETTINGS, &args![old, controller]);
    } else {
        let block = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
        let made = if block != 0 {
            e.call(TRANSFORM_CONTROLLER_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        if controller != made {
            if controller != 0 {
                release(e, controller);
            }
            controller = made;
            if made != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
            }
        }
        e.vcall(
            controller,
            SLOT_SET_INTERPOLATOR,
            &args![interpolator, 0u32],
        );
        e.vcall(controller, SLOT_SET_TARGET, &args![target]);
        e.call(COPY_CONTROLLER_SETTINGS, &args![old, controller]);
        let next = e.mem.u32(controller.wrapping_add(NEXT_CONTROLLER));
        e.call(SET_CONTROLLERS, &args![target, next]);
        let old_next = e.mem.u32(old.wrapping_add(NEXT_CONTROLLER));
        let new_next = e.mem.u32(controller.wrapping_add(NEXT_CONTROLLER));
        if new_next != old_next {
            if new_next != 0 {
                release(e, new_next);
            }
            e.mem
                .set_u32(controller.wrapping_add(NEXT_CONTROLLER), old_next);
            if old_next != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![old_next.wrapping_add(4)]);
            }
        }
        let current = e.mem.u32(old.wrapping_add(NEXT_CONTROLLER));
        if current != controller {
            if current != 0 {
                release(e, current);
            }
            e.mem.set_u32(old.wrapping_add(NEXT_CONTROLLER), controller);
            e.call(INTERLOCKED_INCREMENT, &args![controller.wrapping_add(4)]);
        }
    }
    e.vcall(interpolator, SLOT_COLLAPSE, &args![]);
    if target != 0 {
        e.call(REMOVE_CONTROLLER, &args![target, old]);
    }
    release(e, controller);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a4_2900,
        ni_old_animation_converter_convert_path_controller(Ptr)
    )]
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A counted object (count `count`) with a vtable whose `GetInterpolator` answers the
    /// cell, `SetInterpolator`, `SetTarget` and `DeleteThis` log.
    fn controller(e: &mut Engine, count: u32, interpolator: &Rc<Cell<u32>>) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in [
            (SLOT_DELETE_THIS, 0x00ee_4100u32),
            (SLOT_GET_INTERPOLATOR, 0x00ee_4110),
            (SLOT_SET_INTERPOLATOR, 0x00ee_4120),
            (SLOT_SET_TARGET, 0x00ee_4130),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        let answer = interpolator.clone();
        e.register_double(0x00ee_4110, move |_, _| ret(answer.get()));
        let object = e.mem.alloc(0x38);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// The doubles both paths use. Returns the old controller, its target and the
    /// interpolator block.
    fn setup(e: &mut Engine, log: &Log, existing: u32) -> (u32, u32, u32) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                ret(count)
            });
        }
        for address in [
            0x00ee_4000u32,
            0x00ee_4100,
            0x00ee_4120,
            0x00ee_4130,
            PATH_INTERPOLATOR_00A3EA40,
            COPY_CONTROLLER_SETTINGS,
            REMOVE_CONTROLLER,
            SET_CONTROLLERS,
        ] {
            logged(e, log, address, 0);
        }
        logged(e, log, GET_CONTROLLER, existing);
        e.register(PATH_INTERPOLATOR_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TRANSFORM_CONTROLLER_CONSTRUCT, |_, a| ret(a[0]));
        let interpolator = {
            let vtable = e.mem.alloc(0x100);
            e.mem.set_u32(vtable + SLOT_COLLAPSE, 0x00ee_4000);
            let block = e.mem.alloc(0x5c);
            e.mem.set_u32(block, vtable);
            e.mem.set_u16(block + INTERPOLATOR_FLAGS, 0x0080);
            block
        };
        let old = e.mem.alloc(0x64);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(old + OLD_TARGET, target);
        e.mem.set_u32(old + OLD_PATH_DATA, 0x0aa0);
        e.mem.set_u32(old + OLD_PERCENT_DATA, 0x0bb0);
        // Bits 1, 4 and 6 set; 2, 3 and 5 clear.
        e.mem.set_u8(old + OLD_FLAGS, 0x52);
        e.mem.set_f32(old + 0x50, 1.5);
        e.mem.set_u32(old + 0x54, 0x7f80_0002);
        e.mem.set_u16(old + 0x58, 0x1234);
        e.mem.set_u32(old + 0x60, 0x5678);
        (old, target, interpolator)
    }

    fn check_interpolator(e: &Engine, interpolator: u32) {
        assert_eq!(
            e.mem.u16(interpolator + INTERPOLATOR_FLAGS),
            0x0080 | 0x10 | 0x40 | 0x2
        );
        assert_eq!(e.mem.u32(interpolator + 0x38), 0x5678);
        assert_eq!(e.mem.f32(interpolator + 0x28), 1.5);
        assert_eq!(e.mem.u32(interpolator + 0x2c), 0x7fc0_0002);
        assert_eq!(e.mem.u16(interpolator + 0x30), 0x1234);
    }

    #[test]
    fn convert_path_controller_reuses_an_empty_transform_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        let empty = Rc::new(Cell::new(0u32));
        let existing = controller(&mut e, 1, &empty);
        // The allocations: the interpolator block made in setup.
        let (old, target, interpolator) = setup(&mut e, &log, existing);
        let queue = RefCell::new(vec![interpolator]);
        e.register_double(NI_ALLOCATE, move |_, _| ret(queue.borrow_mut().remove(0)));
        e.call(0x00a4_2900, &args![old]);
        check_interpolator(&e, interpolator);
        assert_eq!(
            calls(&log, GET_CONTROLLER),
            [vec![target, NI_TRANSFORM_CONTROLLER_RTTI]]
        );
        assert_eq!(
            calls(&log, PATH_INTERPOLATOR_00A3EA40),
            [vec![interpolator]]
        );
        assert_eq!(calls(&log, 0x00ee_4120), [vec![existing, interpolator, 0]]);
        assert_eq!(calls(&log, COPY_CONTROLLER_SETTINGS), [vec![old, existing]]);
        assert_eq!(calls(&log, 0x00ee_4000), [vec![interpolator]]);
        assert_eq!(calls(&log, REMOVE_CONTROLLER), [vec![target, old]]);
        assert!(calls(&log, 0x00ee_4130).is_empty());
        // Counted once and released once: back to 1, not deleted.
        assert_eq!(e.mem.u32(existing + 4), 1);
        assert!(calls(&log, 0x00ee_4100).is_empty());
    }

    #[test]
    fn convert_path_controller_inserts_a_new_transform_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        let used = Rc::new(Cell::new(0x9999u32));
        let existing = controller(&mut e, 0, &used);
        let made = controller(&mut e, 0, &used);
        let (old, target, interpolator) = setup(&mut e, &log, existing);
        let queue = RefCell::new(vec![interpolator, made]);
        e.register_double(NI_ALLOCATE, move |_, _| ret(queue.borrow_mut().remove(0)));
        let after = controller(&mut e, 1, &used);
        e.mem.set_u32(old + NEXT_CONTROLLER, after);
        e.call(0x00a4_2900, &args![old]);
        check_interpolator(&e, interpolator);
        // The existing controller's count went from 1 back to 0: deleted.
        assert_eq!(calls(&log, 0x00ee_4100), [vec![existing]]);
        assert_eq!(calls(&log, 0x00ee_4120), [vec![made, interpolator, 0]]);
        assert_eq!(calls(&log, 0x00ee_4130), [vec![made, target]]);
        assert_eq!(calls(&log, COPY_CONTROLLER_SETTINGS), [vec![old, made]]);
        assert_eq!(calls(&log, SET_CONTROLLERS), [vec![target, 0]]);
        // The new controller now sits between the old one and the one after it.
        assert_eq!(e.mem.u32(made + NEXT_CONTROLLER), after);
        assert_eq!(e.mem.u32(old + NEXT_CONTROLLER), made);
        assert_eq!(e.mem.u32(after + 4), 1);
        // Counted by the local and by the old controller's next, the local dropped.
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls(&log, REMOVE_CONTROLLER), [vec![target, old]]);
        assert_eq!(calls(&log, 0x00ee_4000), [vec![interpolator]]);
    }
}
