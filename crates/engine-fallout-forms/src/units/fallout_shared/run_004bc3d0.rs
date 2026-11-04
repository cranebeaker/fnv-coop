//! `fallout shared/run_004bc3d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Ten small functions between the `bgssceneinfo.cpp` code and
//! `inventorychanges.cpp`: accessors for scene-graph globals, a few field
//! setters of an object that the engine map does not name, and the
//! adjustment of the `uGridsToLoad` setting. The map gives no names, and what
//! the globals and fields mean has not been confirmed, so the functions
//! keep `fn_<address>` names and say what they do to memory.

#[allow(unused_imports)]
use crate::prelude::*;

/// A global dword of flag bits: `InitSceneGraph` (`0086d826`) stores a mask
/// of its own flags in it through `004bc3d0`, and `005bf770` sets and
/// clears bit `0x10`.
const FLAG_BITS_011F91D8: u32 = 0x011f_91d8;
/// The object `004bc3f0` dereferences: a global `NiPointer` (its static
/// initializer `00f415b0` calls the `NiPointer` constructor `00633c90` with
/// null on it, and its exit function `NiPointer`'s destructor `0045cec0`).
const GLOBAL_POINTER_SLOT: u32 = 0x011c_73b4;
/// The `Setting` object whose static initializer (`00f3fee0`) constructs it
/// with the name "bDoCanopyShadowPass:Display" (the string at `01021a6c`)
/// and the default 1.
const SETTING_BOOL_011C75A4: u32 = 0x011c_75a4;
/// The `uGridsToLoad:General` setting (`Setting` object).
const SETTING_GRIDS_TO_LOAD: u32 = 0x011c_63cc;
/// Two `float` globals written by `004bc420`, and one by `004bc440`.
const VALUE_PAIR_FIRST_011FD874: u32 = 0x011f_d874;
const VALUE_PAIR_SECOND_011FD878: u32 = 0x011f_d878;
const VALUE_SINGLE_011FD880: u32 = 0x011f_d880;

/// Reads a dword through a pointer (`ECX` = the object; returns `*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// Returns a pointer to the value byte of a `Setting` in `ECX`.
const SETTING_VALUE_BYTE: u32 = 0x0040_8d60;
/// Returns a pointer to the unsigned value of a `Setting` in `ECX`.
const SETTING_VALUE_UINT: u32 = 0x0043_d4d0;
/// Sets the unsigned value of a `Setting` in `ECX` (one argument).
const SETTING_SET_UINT: u32 = 0x0045_ce80;

layout! {
    /// The object whose fields `004bc450`, `004bc480` and `004bc4a0`
    /// change (its class is not identified; callers include the
    /// navmesh and obstacle drawing and `InitSceneGraph`). Only the touched
    /// fields are declared; the size is not known.
    pub struct SceneObject: 0x88 {
        /// Three dwords (a 3-float vector) at `+0x28`.
        0x28 vector: u32,
        /// A counter bumped by `004bc480`.
        0x44 counter: u32,
        /// A byte set by `004bc4a0`.
        0x84 flag: u8,
    }
}

// Translated from 004bc3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the global flag word `011f91d8`.
pub fn fn_004bc3d0(e: &mut Engine, value: u32) {
    e.set_global(FLAG_BITS_011F91D8, value);
}

// Translated from 004bc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global flag word `011f91d8`.
pub fn fn_004bc3e0(e: &mut Engine) -> u32 {
    e.global(FLAG_BITS_011F91D8)
}

// Translated from 004bc3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the pointer held by the global slot `011c73b4` (`*slot`, through
/// `00559450`).
pub fn fn_004bc3f0(e: &mut Engine) -> u32 {
    e.call(POINTER_GET, &args![GLOBAL_POINTER_SLOT]).u32()
}

// Translated from 004bc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the value byte of the "bDoCanopyShadowPass:Display" setting
/// (`011c75a4`).
pub fn fn_004bc400(e: &mut Engine) -> u8 {
    let value = e
        .call(SETTING_VALUE_BYTE, &args![SETTING_BOOL_011C75A4])
        .ptr::<()>();
    e.mem.u8(value.addr())
}

// Translated from 004bc420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores two floats in the globals `011fd874` and `011fd878`.
pub fn fn_004bc420(e: &mut Engine, first: f32, second: f32) {
    e.set_global(VALUE_PAIR_FIRST_011FD874, first);
    e.set_global(VALUE_PAIR_SECOND_011FD878, second);
}

// Translated from 004bc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in the global `011fd880`.
pub fn fn_004bc440(e: &mut Engine, value: f32) {
    e.set_global(VALUE_SINGLE_011FD880, value);
}

// Translated from 004bc450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three dwords at `source` to `this + 0x28` and bumps the
/// counter (`fn_004bc480`).
pub fn fn_004bc450(e: &mut Engine, this: Ptr<SceneObject>, source: Ptr) {
    let words = [
        e.mem.u32(source.addr()),
        e.mem.u32(source.addr() + 4),
        e.mem.u32(source.addr() + 8),
    ];
    let target = this.addr() + 0x28;
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(target + 4 * index as u32, word);
    }
    fn_004bc480(e, this, 1);
}

// Translated from 004bc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the counter at `this + 0x44`. The function takes one stack
/// argument that it never reads (its caller `004bc450` passes 1).
pub fn fn_004bc480(e: &mut Engine, this: Ptr<SceneObject>, _unused_1: u32) {
    let counter = e.get(this, SceneObject::counter);
    e.set(this, SceneObject::counter, counter.wrapping_add(1));
}

// Translated from 004bc4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of `value` at `this + 0x84`.
pub fn fn_004bc4a0(e: &mut Engine, this: Ptr<SceneObject>, value: u32) {
    e.set(this, SceneObject::flag, value as u8);
}

/// The `uGridsToLoad` value, through the pointer the setting hands out.
fn grids_to_load(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_VALUE_UINT, &args![SETTING_GRIDS_TO_LOAD])
        .ptr::<()>();
    e.mem.u32(value.addr())
}

fn set_grids_to_load(e: &mut Engine, value: u32) {
    e.call(SETTING_SET_UINT, &args![SETTING_GRIDS_TO_LOAD, value]);
}

// Translated from 004bc4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forces the `uGridsToLoad` setting to a valid value: at least 5, and odd
/// (an even value is raised by one).
///
/// The code keeps a local flag that is always 0, so the branch for a set flag
/// (a minimum of 3 instead of 5) never runs; it is kept because the
/// build does not fold it. Each test reads the setting again.
pub fn fn_004bc4c0(e: &mut Engine) {
    let flag = false;
    if flag && grids_to_load(e) < 3 {
        set_grids_to_load(e, 3);
        return;
    }
    if !flag && grids_to_load(e) < 5 {
        set_grids_to_load(e, 5);
        return;
    }
    let value = grids_to_load(e);
    if value & 1 == 0 {
        let value = grids_to_load(e);
        set_grids_to_load(e, value.wrapping_add(1));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004bc3d0, fn_004bc3d0(u32)),
        entry!(0x004bc3e0, fn_004bc3e0() -> u32),
        entry!(0x004bc3f0, fn_004bc3f0() -> u32),
        entry!(0x004bc400, fn_004bc400() -> u8),
        entry!(0x004bc420, fn_004bc420(f32, f32)),
        entry!(0x004bc440, fn_004bc440(f32)),
        entry!(0x004bc450, fn_004bc450(Ptr<SceneObject>, Ptr)),
        entry!(0x004bc480, fn_004bc480(Ptr<SceneObject>, u32)),
        entry!(0x004bc4a0, fn_004bc4a0(Ptr<SceneObject>, u32)),
        entry!(0x004bc4c0, fn_004bc4c0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011f_9000, 0x011f_d000, 0x011c_7000, 0x011c_6000] {
            e.map(page, 0x1000);
        }
        e
    }

    #[test]
    fn flag_word_round_trips_through_the_global() {
        let mut e = engine();
        e.call(0x004b_c3d0, &args![0x30u32]);
        assert_eq!(e.global::<u32>(FLAG_BITS_011F91D8), 0x30);
        assert_eq!(e.call(0x004b_c3e0, &args![]).u32(), 0x30);
    }

    #[test]
    fn pointer_slot_reads_through_the_pointer_getter() {
        let mut e = engine();
        e.register(POINTER_GET, |_, a| {
            assert_eq!(a[0], GLOBAL_POINTER_SLOT);
            Ret {
                eax: 0x1234_5678,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x004b_c3f0, &args![]).u32(), 0x1234_5678);
    }

    #[test]
    fn setting_byte_is_read_through_the_value_pointer() {
        let mut e = engine();
        let value = e.mem.alloc(1);
        e.mem.set_u8(value, 1);
        e.register_double(SETTING_VALUE_BYTE, move |_, a| {
            assert_eq!(a[0], SETTING_BOOL_011C75A4);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x004b_c400, &args![]).u8(), 1);
        e.mem.set_u8(value, 0);
        assert_eq!(e.call(0x004b_c400, &args![]).u8(), 0);
    }

    #[test]
    fn float_globals_are_stored() {
        let mut e = engine();
        e.call(0x004b_c420, &args![1.5f32, -2.25f32]);
        assert_eq!(e.global::<f32>(VALUE_PAIR_FIRST_011FD874), 1.5);
        assert_eq!(e.global::<f32>(VALUE_PAIR_SECOND_011FD878), -2.25);
        e.call(0x004b_c440, &args![8.0f32]);
        assert_eq!(e.global::<f32>(VALUE_SINGLE_011FD880), 8.0);
    }

    #[test]
    fn vector_copy_bumps_the_counter() {
        let mut e = engine();
        let this: Ptr<SceneObject> = e.new_object();
        e.mem.set_u32(this.addr() + 0x44, 6);
        let source = e.mem.alloc(12);
        for (i, word) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(source + 4 * i as u32, word);
        }
        e.call(0x004b_c450, &args![this, source]);
        assert_eq!(e.mem.f32(this.addr() + 0x28), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x2c), 2.0);
        assert_eq!(e.mem.f32(this.addr() + 0x30), 3.0);
        assert_eq!(e.get(this, SceneObject::counter), 7);
    }

    #[test]
    fn counter_ignores_its_argument() {
        let mut e = engine();
        let this: Ptr<SceneObject> = e.new_object();
        e.call(0x004b_c480, &args![this, 99u32]);
        e.call(0x004b_c480, &args![this, 0u32]);
        assert_eq!(e.get(this, SceneObject::counter), 2);
    }

    #[test]
    fn flag_byte_takes_the_low_byte() {
        let mut e = engine();
        let this: Ptr<SceneObject> = e.new_object();
        e.call(0x004b_c4a0, &args![this, 0x1_0201u32]);
        assert_eq!(e.get(this, SceneObject::flag), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x85), 0);
    }

    /// A `Setting` double holding `start`; returns the values it was set to.
    fn grids_setting(e: &mut Engine, start: u32) -> Rc<RefCell<Vec<u32>>> {
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, start);
        e.register_double(SETTING_VALUE_UINT, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
        let sets = Rc::new(RefCell::new(Vec::new()));
        let sink = sets.clone();
        e.register_double(SETTING_SET_UINT, move |e, a| {
            assert_eq!(a[0], SETTING_GRIDS_TO_LOAD);
            sink.borrow_mut().push(a[1]);
            e.mem.set_u32(value, a[1]);
            Ret::default()
        });
        sets
    }

    #[test]
    fn grids_below_five_become_five() {
        for start in [0, 3, 4] {
            let mut e = engine();
            let sets = grids_setting(&mut e, start);
            e.call(0x004b_c4c0, &args![]);
            assert_eq!(*sets.borrow(), vec![5]);
        }
    }

    #[test]
    fn even_grids_become_odd_and_odd_grids_stay() {
        let mut e = engine();
        let sets = grids_setting(&mut e, 6);
        e.call(0x004b_c4c0, &args![]);
        assert_eq!(*sets.borrow(), vec![7]);
        for start in [5, 7, 9] {
            let mut e = engine();
            let sets = grids_setting(&mut e, start);
            e.call(0x004b_c4c0, &args![]);
            assert!(sets.borrow().is_empty());
        }
    }
}
