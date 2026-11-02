//! `fallout shared/havok/run_0062e860` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::teswaterlistener::TESWaterListener;

/// `TESWaterListener` vtable (RTTI `.?AVTESWaterListener@@`, read from the
/// exe): the vtable at `+0`.
const LISTENER_VTABLE: u32 = 0x0104_e450;
/// The vtable of its `BGSZoneTargetListener` part at `+0x14`
/// (`teswaterlistener.rs` stores the same one).
const ZONE_LISTENER_VTABLE: u32 = 0x0104_e440;
/// Offset of the `BGSZoneTargetListener` part.
const ZONE_LISTENER: u32 = 0x14;
/// Offset of the `QueuedRemovals` list.
const QUEUED_REMOVALS: u32 = 0x34;

/// `00631370(this, 0, 0, 0)` (`RET 0xc`): works through the queued
/// removals at `+0x34` (it is given three words, all 0 here).
const PROCESS_REMOVALS: u32 = 0x0063_1370;
/// `00401030(block)`: the game's `operator delete` (cdecl, one argument).
const FREE: u32 = 0x0040_1030;
/// The destructor of the removal list (`0054da00`, on `this + 0x34`).
const REMOVALS_DESTRUCTOR: u32 = 0x0054_da00;
/// The destructor of the zone listener part (`00620110`, on `this + 0x14`).
const ZONE_LISTENER_DESTRUCTOR: u32 = 0x0062_0110;
/// The destructor of the `bhkEntityListener` base (`005545d0`).
const BASE_DESTRUCTOR: u32 = 0x0055_45d0;

// Translated from 0062e860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062e860 - the FS:[0] exception frame (its states 2, 1, 0 and -1 decide which members the unwind destroys) is not translated
/// `TESWaterListener::~TESWaterListener` (the body that `0062e750`, the
/// scalar deleting destructor in `teswaterlistener.rs`, calls): sets the
/// two vtables, lets `00631370` process the queued removals, frees the
/// water level table (`pfWaterLevels`), then destroys the removal list, the
/// zone listener part and the base, in that order.
pub fn tes_water_listener_dtor(e: &mut Engine, this: Ptr<TESWaterListener>) {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    e.mem
        .set_u32(this.addr() + ZONE_LISTENER, ZONE_LISTENER_VTABLE);
    e.call(PROCESS_REMOVALS, &args![this, 0u32, 0u32, 0u32]);
    let water_levels = e.get(this, TESWaterListener::pfWaterLevels);
    e.call(FREE, &args![water_levels]);
    e.call(REMOVALS_DESTRUCTOR, &args![this.addr() + QUEUED_REMOVALS]);
    e.call(
        ZONE_LISTENER_DESTRUCTOR,
        &args![this.addr() + ZONE_LISTENER],
    );
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0062_e860,
        tes_water_listener_dtor(Ptr<TESWaterListener>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALLEES: [u32; 5] = [
        PROCESS_REMOVALS,
        FREE,
        REMOVALS_DESTRUCTOR,
        ZONE_LISTENER_DESTRUCTOR,
        BASE_DESTRUCTOR,
    ];

    #[test]
    fn members_are_destroyed_in_order_after_the_vtables_are_set() {
        let mut e = Engine::new();
        for address in CALLEES {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.new_object::<TESWaterListener>();
        e.set(this, TESWaterListener::pfWaterLevels, 0x7700);
        // Both vtables are in place by the time the first callee runs.
        e.register(PROCESS_REMOVALS, |e, a| {
            assert_eq!(e.mem.u32(a[0]), LISTENER_VTABLE);
            assert_eq!(e.mem.u32(a[0] + 0x14), ZONE_LISTENER_VTABLE);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0062_e860, &args![this]);
        let t = this.addr();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| CALLEES.contains(a))
            .collect();
        assert_eq!(
            calls,
            vec![
                (PROCESS_REMOVALS, vec![t, 0, 0, 0]),
                (FREE, vec![0x7700]),
                (REMOVALS_DESTRUCTOR, vec![t + 0x34]),
                (ZONE_LISTENER_DESTRUCTOR, vec![t + 0x14]),
                (BASE_DESTRUCTOR, vec![t]),
            ]
        );
        assert_eq!(e.mem.u32(t), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(t + 0x14), ZONE_LISTENER_VTABLE);
    }
}
