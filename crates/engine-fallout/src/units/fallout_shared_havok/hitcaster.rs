//! `fallout shared/havok/hitcaster.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `HitCaster` (Xbox PDB), 0x3B0 bytes: an `hkpAllCdPointCollector` at
    /// `+0` (0x3A0 bytes, `cameracaster.rs`), then the phantom the cast runs
    /// through. The PC code reads the phantom at the same offset as the PDB.
    pub struct HitCaster: 0x3b0 {
        /// `spShapePhantom` (Xbox PDB): `NiPointer<bhkSimpleShapePhantom>`.
        0x3a0 spShapePhantom: Ptr,
    }
}

/// `*this` of a `NiPointer` (`00559450`).
const POINTER_GET: u32 = 0x0055_9450;
/// `hkColor::rgbFromChars` (engine map name of `00cabb00`; cdecl, four
/// bytes).
const RGB_FROM_CHARS: u32 = 0x00ca_bb00;
/// `bhkWorld::SetDebugObjectColor` (engine map name of `00c68e30`):
/// `this` is the world, the stack takes `(object, color)`.
const SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;
/// The phantom's virtual function at `+0x9c` (one argument; the Xbox PDB's
/// `bhkWorldObject::Add` is at that slot).
const SLOT_9C: u32 = 0x9c;

// Translated from 00626d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitCaster::Add` (Xbox PDB name, from the engine map): when the caster
/// has a phantom, calls the phantom's virtual function at `+0x9c` with
/// `world`, and when `world` is not null gives `world` the debug colour
/// `rgbFromChars(0x58, 0xad, 0x56, 0x28)` for the phantom.
pub fn hit_caster_add(e: &mut Engine, this: Ptr<HitCaster>, world: u32) {
    let pointer = this.addr() + HitCaster::spShapePhantom.off;
    if e.call(POINTER_GET, &args![pointer]).u32() == 0 {
        return;
    }
    let phantom = e.call(POINTER_GET, &args![pointer]).u32();
    e.vcall(phantom, SLOT_9C, &args![world]);
    if world != 0 {
        let color = e
            .call(RGB_FROM_CHARS, &args![0x58u32, 0xadu32, 0x56u32, 0x28u32])
            .u32();
        let phantom = e.call(POINTER_GET, &args![pointer]).u32();
        e.call(SET_DEBUG_OBJECT_COLOR, &args![world, phantom, color]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_6d20, hit_caster_add(Ptr<HitCaster>, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHANTOM: u32 = 0x0610_0000;
    const WORLD: u32 = 0x0630_0000;
    const SLOT_TARGET: u32 = 0x0700_009c;

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A caster whose `NiPointer` at `+0x3a0` holds `phantom`; the phantom's
    /// vtable has `SLOT_TARGET` at `+0x9c`.
    fn engine(phantom: u32) -> (Engine, Ptr<HitCaster>) {
        let mut e = Engine::new();
        let caster = e.new_object::<HitCaster>();
        e.map(PHANTOM, 0x1000);
        let table = PHANTOM + 0x800;
        e.mem.set_u32(table + 0x9c, SLOT_TARGET);
        e.mem.set_u32(PHANTOM, table);
        e.set(caster, HitCaster::spShapePhantom, Ptr::new(phantom));
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SLOT_TARGET, |_, _| Ret::default());
        e.register(RGB_FROM_CHARS, |_, _| 0xff58_ad56u32.into_ret());
        e.register(SET_DEBUG_OBJECT_COLOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, caster)
    }

    #[test]
    fn phantom_gets_the_world_and_a_colour() {
        let (mut e, caster) = engine(PHANTOM);
        e.call(0x0062_6d20, &args![caster, WORLD]);
        assert_eq!(
            logged(&e, POINTER_GET),
            vec![vec![caster.addr() + 0x3a0]; 3]
        );
        assert_eq!(logged(&e, SLOT_TARGET), vec![vec![PHANTOM, WORLD]]);
        assert_eq!(
            logged(&e, RGB_FROM_CHARS),
            vec![vec![0x58, 0xad, 0x56, 0x28]]
        );
        assert_eq!(
            logged(&e, SET_DEBUG_OBJECT_COLOR),
            vec![vec![WORLD, PHANTOM, 0xff58_ad56]]
        );
    }

    #[test]
    fn a_null_world_is_passed_on_but_not_coloured() {
        let (mut e, caster) = engine(PHANTOM);
        e.call(0x0062_6d20, &args![caster, 0u32]);
        assert_eq!(logged(&e, SLOT_TARGET), vec![vec![PHANTOM, 0]]);
        assert!(logged(&e, RGB_FROM_CHARS).is_empty());
        assert!(logged(&e, SET_DEBUG_OBJECT_COLOR).is_empty());
    }

    #[test]
    fn without_a_phantom_nothing_happens() {
        let (mut e, caster) = engine(0);
        e.call(0x0062_6d20, &args![caster, WORLD]);
        assert_eq!(logged(&e, POINTER_GET).len(), 1);
        assert!(logged(&e, SLOT_TARGET).is_empty());
        assert!(logged(&e, RGB_FROM_CHARS).is_empty());
        assert!(logged(&e, SET_DEBUG_OBJECT_COLOR).is_empty());
    }
}
