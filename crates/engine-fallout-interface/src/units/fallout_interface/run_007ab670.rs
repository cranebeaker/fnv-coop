//! `fallout/interface/run_007ab670` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `010757a8` (RTTI `.?AV?$BSSimpleArray@PAUPerkActivation@MessageMenu@@$0EAA@@@`).
const PERK_ACTIVATION_ARRAY_VTABLE: u32 = 0x0107_57a8;
/// `008454f0` (`thiscall`, one word): the array's clear.
const ARRAY_CLEAR_008454F0: u32 = 0x0084_54f0;
/// The `Menu` constructor `00a1c4a0` (`thiscall`).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// `PlayerNameEntryMenu`'s vtable (`010757fc`).
const PLAYER_NAME_ENTRY_MENU_VTABLE: u32 = 0x0107_57fc;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `BGSSaveFormBuffer::GetForm` (engine map name for the folded getter `007af430`).
const GET_FORM_007AF430: u32 = 0x007a_f430;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `01183108` to `01183a1c`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_01183108: u32 = 0x0118_3108;
const CAST_TARGET_01183A1C: u32 = 0x0118_3a1c;
/// `007ab7c0` (no words): clears the entered name.
const CLEAR_NAME_007AB7C0: u32 = 0x007a_b7c0;
/// `MapMarkerData::GetLocationName` (engine map name for `00408da0`, `thiscall`).
const GET_NAME_00408DA0: u32 = 0x0040_8da0;
/// `004c69f0` (`thiscall`) on `011d3b38`.
const OBJECT_004C69F0: u32 = 0x004c_69f0;
const OBJECT_011D3B38: u32 = 0x011d_3b38;
/// `007e6320` (`cdecl`, three words): opens the text entry with a check function.
const OPEN_TEXT_ENTRY_007E6320: u32 = 0x007e_6320;
/// The name check `007ab820` passed to it.
const NAME_CHECK_007AB820: u32 = 0x007a_b820;
/// `007ab9a0` (`thiscall`, one word: a name): takes the entered name.
const TAKE_NAME_007AB9A0: u32 = 0x007a_b9a0;

// Translated from 007ab670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of a `BSSimpleArray` of `MessageMenu::PerkActivation` pointers (RTTI of
/// the vtable it stores, `010757a8`; unnamed in the engine map; `thiscall`): stores the
/// vtable and clears the array (`008454f0` with 1).
pub fn fn_007ab670(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PERK_ACTIVATION_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR_008454F0, &args![this, 1u32]);
}

// Translated from 007ab690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PlayerNameEntryMenu` constructor (unnamed in the engine map; `thiscall`):
/// constructs the `Menu` base, stores the vtable and keeps at +0x28 the player's form
/// (`007af430`) cast from `01183108` to `01183a1c` (null without a player). With it the
/// entered name is cleared (`007ab7c0`) and the text entry opened (`007e6320` with the
/// object of `004c69f0` on `011d3b38`, the name of the form's +0xd0 string, and the check
/// `007ab820`); without it the menu takes no name (`007ab9a0` with null). Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007ab690 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_007ab690(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), PLAYER_NAME_ENTRY_MENU_VTABLE);
    let player = e.mem.u32(PLAYER);
    let form = if player != 0 {
        let base = e.call(GET_FORM_007AF430, &args![player]).u32();
        e.call(
            DYNAMIC_CAST,
            &args![base, 0u32, CAST_SOURCE_01183108, CAST_TARGET_01183A1C, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 0x28, form);
    if e.mem.u32(this.addr() + 0x28) != 0 {
        e.call(CLEAR_NAME_007AB7C0, &args![]);
        let form = e.mem.u32(this.addr() + 0x28);
        let name = e.call(GET_NAME_00408DA0, &args![form + 0xd0]).u32();
        let object = e.call(OBJECT_004C69F0, &args![OBJECT_011D3B38]).u32();
        e.call(
            OPEN_TEXT_ENTRY_007E6320,
            &args![object, name, NAME_CHECK_007AB820],
        );
    } else {
        e.call(TAKE_NAME_007AB9A0, &args![this, 0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_b670, fn_007ab670(Ptr)),
        entry!(0x007a_b690, fn_007ab690(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn array_destructor_stores_the_vtable_and_clears() {
        let mut e = Engine::new();
        e.register(ARRAY_CLEAR_008454F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x007a_b670, &[this]);
        assert_eq!(e.mem.u32(this), PERK_ACTIVATION_ARRAY_VTABLE);
        assert_eq!(called(&e, ARRAY_CLEAR_008454F0), vec![vec![this, 1]]);
    }

    fn engine(player: u32, cast: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(PLAYER, player);
        for address in [
            MENU_CONSTRUCTOR,
            CLEAR_NAME_007AB7C0,
            OPEN_TEXT_ENTRY_007E6320,
            TAKE_NAME_007AB9A0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(GET_FORM_007AF430, |_, a| (a[0] + 1).into_ret());
        e.register_double(DYNAMIC_CAST, move |_, _| cast.into_ret());
        e.register(GET_NAME_00408DA0, |_, a| (a[0] + 2).into_ret());
        e.register(OBJECT_004C69F0, |_, _| 0x6969u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_opens_the_text_entry_for_the_player() {
        let mut e = engine(0x5000, 0x6000);
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x007a_b690, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), PLAYER_NAME_ENTRY_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x28), 0x6000);
        assert_eq!(
            called(&e, DYNAMIC_CAST),
            vec![vec![
                0x5001,
                0,
                CAST_SOURCE_01183108,
                CAST_TARGET_01183A1C,
                0
            ]]
        );
        assert_eq!(called(&e, CLEAR_NAME_007AB7C0).len(), 1);
        assert_eq!(called(&e, GET_NAME_00408DA0), vec![vec![0x60d0]]);
        assert_eq!(
            called(&e, OPEN_TEXT_ENTRY_007E6320),
            vec![vec![0x6969, 0x60d2, NAME_CHECK_007AB820]]
        );
        assert!(called(&e, TAKE_NAME_007AB9A0).is_empty());
    }

    #[test]
    fn constructor_without_a_player_takes_no_name() {
        for (player, cast) in [(0u32, 0x6000u32), (0x5000, 0)] {
            let mut e = engine(player, cast);
            let this = e.mem.alloc(0x30);
            e.call(0x007a_b690, &[this]);
            assert_eq!(e.mem.u32(this + 0x28), 0);
            assert_eq!(called(&e, TAKE_NAME_007AB9A0), vec![vec![this, 0]]);
            assert!(called(&e, OPEN_TEXT_ENTRY_007E6320).is_empty());
            assert_eq!(called(&e, DYNAMIC_CAST).len(), usize::from(player != 0));
        }
    }
}
