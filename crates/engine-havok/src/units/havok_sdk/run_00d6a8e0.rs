//! `Havok SDK/run_00d6a8e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld::addWorldPostSimulationListener` (engine map name): `this` the world, one word,
/// the listener.
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
/// The removal paired with `00c93920` (the map has no name for it): `this` the world, one word.
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;
/// `hkpWorldViewerBase::hkpWorldViewerBase` (engine map name): one word, passed on.
const WORLD_VIEWER_BASE_CONSTRUCTOR: u32 = 0x00dd_ccb0;

// Translated from 00d6a8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadphaseViewer::worldAddedCallback` (Xbox PDB name; slot 1 of the vtable at
/// `010d2384`, which the exe's RTTI gives to `hkpBroadphaseViewer`): `this` is the subobject at
/// `+0x20`. Calls `hkpWorld::addWorldPostSimulationListener` (`00c93920`) on `world` with the
/// listener `this + 8`, or null when `this - 0x20` is null.
pub fn fn_00d6a8e0(e: &mut Engine, this: Ptr, world: u32) {
    let listener = if this.addr().wrapping_sub(0x20) == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, listener]);
}

// Translated from 00d6a910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadphaseViewer::worldRemovedCallback` (Xbox PDB name; slot 2 of the vtable at
/// `010d2384`): as `00d6a8e0` but through `00c918e0`, the removal paired with `00c93920`.
pub fn fn_00d6a910(e: &mut Engine, this: Ptr, world: u32) {
    let listener = if this.addr().wrapping_sub(0x20) == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(
        WORLD_REMOVE_POST_SIMULATION_LISTENER,
        &args![world, listener],
    );
}

// Translated from 00d6a940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadphaseViewer::hkpBroadphaseViewer` (Xbox PDB name; the exe's RTTI names the class of
/// the vtable `010d23b4`): runs the base constructor `00ddccb0` with `context`, sets the four
/// vtables (`this`, `+8`, `+0x20`, `+0x28`), clears the array at `this + 0x2c` (data, count,
/// capacity word `0x80000000`) and registers the listener `this + 0x28` with every world of the
/// context (the object at `this + 0x24`: world pointers at `+0x6c`, count at `+0x70`).
/// Returns `this`.
pub fn fn_00d6a940(e: &mut Engine, this: Ptr, context: u32) -> Ptr {
    let this = this.addr();
    e.call(WORLD_VIEWER_BASE_CONSTRUCTOR, &args![this, context]);
    // The scalar deleting destructor of the base listener, replaced two stores later.
    e.mem.set_u32(this + 0x28, 0x010c_cf7c);
    let listener = this + 0x28;
    e.mem.set_u32(this, 0x010d_23b4);
    e.mem.set_u32(this + 8, 0x010d_2394);
    e.mem.set_u32(this + 0x20, 0x010d_2384);
    e.mem.set_u32(listener, 0x010d_2374);
    e.mem.set_u32(this + 0x2c, 0);
    e.mem.set_u32(this + 0x30, 0);
    e.mem.set_u32(this + 0x34, 0x8000_0000);
    let worlds = e.mem.u32(this + 0x24);
    if worlds != 0 && e.mem.i32(worlds + 0x70) > 0 {
        let mut index = 0u32;
        loop {
            let worlds = e.mem.u32(this + 0x24);
            let data = e.mem.u32(worlds + 0x6c);
            let world = e.mem.u32(data + index * 4);
            e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, listener]);
            let worlds = e.mem.u32(this + 0x24);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(worlds + 0x70) {
                break;
            }
        }
    }
    Ptr::new(this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_a8e0, fn_00d6a8e0(Ptr, u32)),
        entry!(0x00d6_a910, fn_00d6a910(Ptr, u32)),
        entry!(0x00d6_a940, fn_00d6a940(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// The recorded calls to `address`, each as its argument words.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn world_added_thunk_passes_this_plus_8_or_null() {
        let mut e = Engine::new();
        e.register(WORLD_ADD_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_a8e0, &args![0x3000u32, 0x41u32]);
        e.call(0x00d6_a8e0, &args![0x20u32, 0x42u32]);
        assert_eq!(
            calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER),
            [vec![0x41, 0x3008], vec![0x42, 0]]
        );
    }

    #[test]
    fn world_removed_thunk_passes_this_plus_8_or_null() {
        let mut e = Engine::new();
        e.register(WORLD_REMOVE_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d6_a910, &args![0x3000u32, 0x43u32]);
        e.call(0x00d6_a910, &args![0x20u32, 0x44u32]);
        assert_eq!(
            calls(&e, WORLD_REMOVE_POST_SIMULATION_LISTENER),
            [vec![0x43, 0x3008], vec![0x44, 0]]
        );
    }

    fn constructor_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(WORLD_VIEWER_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(WORLD_ADD_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        // Stale contents the constructor must overwrite.
        for offset in [0x2c, 0x30, 0x34] {
            e.mem.set_u32(this + offset, 0x9999);
        }
        (e, this)
    }

    #[test]
    fn constructor_sets_the_vtables_and_registers_with_every_world() {
        let (mut e, this) = constructor_engine();
        let context = e.mem.alloc(0x80);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 0x501);
        e.mem.set_u32(data + 4, 0x502);
        e.mem.set_u32(context + 0x6c, data);
        e.mem.set_u32(context + 0x70, 2);
        e.mem.set_u32(this + 0x24, context);
        e.call_log = Some(vec![]);
        let result = e.call(0x00d6_a940, &args![this, 0x77u32]).u32();
        assert_eq!(result, this);
        assert_eq!(calls(&e, WORLD_VIEWER_BASE_CONSTRUCTOR), [vec![this, 0x77]]);
        assert_eq!(
            calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER),
            [vec![0x501, this + 0x28], vec![0x502, this + 0x28]]
        );
        assert_eq!(e.mem.u32(this), 0x010d_23b4);
        assert_eq!(e.mem.u32(this + 8), 0x010d_2394);
        assert_eq!(e.mem.u32(this + 0x20), 0x010d_2384);
        assert_eq!(e.mem.u32(this + 0x28), 0x010d_2374);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0x8000_0000);
    }

    #[test]
    fn constructor_without_a_context_or_worlds_registers_nothing() {
        let (mut e, this) = constructor_engine();
        e.call_log = Some(vec![]);
        e.call(0x00d6_a940, &args![this, 0u32]);
        assert!(calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER).is_empty());
        let context = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x24, context);
        e.call(0x00d6_a940, &args![this, 0u32]);
        assert!(calls(&e, WORLD_ADD_POST_SIMULATION_LISTENER).is_empty());
    }
}
