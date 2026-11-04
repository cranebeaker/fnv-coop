//! `fallout shared/havok/run_00621370` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `*this` of a `NiPointer` (`00559450`).
const POINTER_GET: u32 = 0x0055_9450;
/// `hkColor::rgbFromChars` (engine map name of `00cabb00`; cdecl, four
/// bytes).
const RGB_FROM_CHARS: u32 = 0x00ca_bb00;
/// `bhkWorld::SetDebugObjectColor` (engine map name of `00c68e30`):
/// `this` is the world, the stack takes `(object, color)`.
const SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;
/// The held object's virtual function at `+0x9c` (one argument; the Xbox
/// PDB's `bhkWorldObject::Add` is at that slot).
const SLOT_9C: u32 = 0x9c;

// Translated from 00621370 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two `NiPointer`s at `+0` and `+4` of `this` (the layout
/// of `CameraCaster` in `cameracaster.rs`: the player's and the horse's
/// phantom; the exe does not name this function): when the pointer holds an
/// object, calls its virtual function at `+0x9c` with `world`, and when
/// `world` is not null gives `world` the debug colour
/// `rgbFromChars(0xff, 0x2e, 0x2e, 0x19)` for that object.
pub fn fn_00621370(e: &mut Engine, this: Ptr, world: u32) {
    for pointer in [this.addr(), this.addr() + 4] {
        if e.call(POINTER_GET, &args![pointer]).u32() == 0 {
            continue;
        }
        let object = e.call(POINTER_GET, &args![pointer]).u32();
        e.vcall(object, SLOT_9C, &args![world]);
        if world != 0 {
            let color = e
                .call(RGB_FROM_CHARS, &args![0xffu32, 0x2eu32, 0x2eu32, 0x19u32])
                .u32();
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            e.call(SET_DEBUG_OBJECT_COLOR, &args![world, object, color]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_1370, fn_00621370(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const THIS: u32 = 0x0600_0000;
    const FIRST: u32 = 0x0610_0000;
    const SECOND: u32 = 0x0620_0000;
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

    /// Two phantoms (`FIRST`, `SECOND`) behind the pointers at `THIS` and
    /// `THIS + 4`, each with a vtable whose slot `0x9c` is `SLOT_TARGET`.
    fn engine(first: u32, second: u32) -> Engine {
        let mut e = Engine::new();
        e.map(THIS, 0x1000);
        e.map(FIRST, 0x1000);
        e.map(SECOND, 0x1000);
        let table = FIRST + 0x800;
        e.mem.set_u32(table + 0x9c, SLOT_TARGET);
        e.mem.set_u32(FIRST, table);
        e.mem.set_u32(SECOND, table);
        e.mem.set_u32(THIS, first);
        e.mem.set_u32(THIS + 4, second);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SLOT_TARGET, |_, _| Ret::default());
        e.register(RGB_FROM_CHARS, |_, _| 0xff2e_2e19u32.into_ret());
        e.register(SET_DEBUG_OBJECT_COLOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn both_phantoms_get_the_world_and_the_colour() {
        let mut e = engine(FIRST, SECOND);
        e.call(0x0062_1370, &args![THIS, WORLD]);
        assert_eq!(
            logged(&e, SLOT_TARGET),
            vec![vec![FIRST, WORLD], vec![SECOND, WORLD]]
        );
        assert_eq!(
            logged(&e, RGB_FROM_CHARS),
            vec![vec![0xff, 0x2e, 0x2e, 0x19]; 2]
        );
        assert_eq!(
            logged(&e, SET_DEBUG_OBJECT_COLOR),
            vec![
                vec![WORLD, FIRST, 0xff2e_2e19],
                vec![WORLD, SECOND, 0xff2e_2e19]
            ]
        );
    }

    #[test]
    fn a_null_world_clears_without_colouring() {
        let mut e = engine(FIRST, SECOND);
        e.call(0x0062_1370, &args![THIS, 0u32]);
        assert_eq!(
            logged(&e, SLOT_TARGET),
            vec![vec![FIRST, 0], vec![SECOND, 0]]
        );
        assert!(logged(&e, RGB_FROM_CHARS).is_empty());
        assert!(logged(&e, SET_DEBUG_OBJECT_COLOR).is_empty());
    }

    #[test]
    fn empty_pointers_are_skipped_one_by_one() {
        let mut e = engine(0, SECOND);
        e.call(0x0062_1370, &args![THIS, WORLD]);
        assert_eq!(logged(&e, SLOT_TARGET), vec![vec![SECOND, WORLD]]);
        assert_eq!(logged(&e, SET_DEBUG_OBJECT_COLOR).len(), 1);

        let mut e = engine(FIRST, 0);
        e.call(0x0062_1370, &args![THIS, WORLD]);
        assert_eq!(logged(&e, SLOT_TARGET), vec![vec![FIRST, WORLD]]);

        let mut e = engine(0, 0);
        e.call(0x0062_1370, &args![THIS, WORLD]);
        assert!(logged(&e, SLOT_TARGET).is_empty());
        assert!(logged(&e, RGB_FROM_CHARS).is_empty());
    }
}
