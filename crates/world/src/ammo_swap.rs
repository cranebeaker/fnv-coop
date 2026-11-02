//! The Ammo Swap control (18, "Ammo Swap" in the INI: the 2 key in this
//! install), as `FalloutNV.exe` 1.4.0.525 does it.
//!
//! `0093e860` (at `00940984`…`00940b25`): the player's swap timer (+0xd50)
//! counts the frame's seconds; the control's press, with a weapon equipped
//! (process vfunc +0x148 `GetCurrentWeapon` (Xbox PDB)) and out
//! (`008a16d0`), calls `009462c0(timer > fPlayerAmmoSwapTimer, 0)` and,
//! when the timer had passed, sets it back to 0. (The gamepad's d-pad
//! states it also reads, `00717a40`, are "no controller" on the keyboard.)
//!
//! `009462c0`: the ammunition loaded (`00525980`: the process's current
//! ammo, else the list's first) is looked for in the weapon's ammunition
//! list; one not in the list is dropped (`SetCurrentAmmo(0)`). The next
//! kind after it the player carries (count above 0), going round the list
//! once, is loaded (`SetCurrentAmmo`, Xbox PDB) and the weapon reloads
//! (`ReloadWeaponNV`, Xbox PDB, actor vfunc +0x3ec): with the reload's
//! animation (2) when the timer had passed and the weapon is out, else
//! without (0).

use esm::{FormId, LoadOrder};

use crate::combat::Weapon;
use crate::scripting::{game_setting, GameState};

/// `fPlayerAmmoSwapTimer` (1 s; the data doesn't set it).
pub fn swap_time(order: &LoadOrder) -> f32 {
    game_setting(order, "fPlayerAmmoSwapTimer").unwrap_or(1.0)
}

/// The ammunition `holder` has loaded in `weapon` (`00525980`): the one
/// chosen (`GameState::ammo_loaded`) while it's in the weapon's list and
/// carried, else the first carried kind, else the list's first.
pub fn loaded(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: &Weapon,
) -> Option<FormId> {
    weapon.ammo_in_use(order, state, holder)
}

/// The kind the swap loads next (`009462c0`): after `current` in the
/// weapon's list, round the list once, the first other kind carried.
// Translated from 009462c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn next(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: &Weapon,
    current: Option<FormId>,
) -> Option<FormId> {
    let list = &weapon.ammo;
    // A loaded kind not in the list counts as none.
    let current = current.filter(|c| list.contains(c));
    let start = current
        .and_then(|c| list.iter().position(|&a| a == c))
        .map_or(0, |i| i + 1);
    (0..list.len())
        .map(|k| list[(start + k) % list.len()])
        .find(|&a| Some(a) != current && state.item_count(order, holder, a) > 0)
}

/// What the swap does: the kind to load and whether the reload plays its
/// animation (`ReloadWeaponNV`'s 2) or not (0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Swap {
    pub ammo: FormId,
    pub animated: bool,
}

/// The control pressed: the swap, if there's another kind, and whether
/// the timer goes back to 0 (`timer` past `fPlayerAmmoSwapTimer`).
// Translated from 0093e860 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn press(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: &Weapon,
    out: bool,
    timer: f32,
) -> (Option<Swap>, bool) {
    let passed = timer > swap_time(order);
    let current = loaded(order, state, holder, weapon);
    let swap = next(order, state, holder, weapon, current).map(|ammo| Swap {
        ammo,
        animated: passed && out,
    });
    (swap, passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_swap_goes_round_the_carried_kinds() {
        let order = LoadOrder::from_plugins(Vec::new()).unwrap();
        let mut state = GameState::default();
        let player = crate::dialogue::PLAYER_REF;
        let (a, b, c) = (FormId(0xA), FormId(0xB), FormId(0xC));
        let mut weapon = crate::melee::tests::weapon(4);
        weapon.ammo = vec![a, b, c];
        state.stocked.insert(player);
        state.items.insert((player, b), 20);
        state.items.insert((player, c), 5);
        let next = |state: &GameState, cur| next(&order, state, player, &weapon, cur);
        assert_eq!(next(&state, Some(a)), Some(b));
        assert_eq!(next(&state, Some(b)), Some(c));
        // Round the list, past the kind not carried.
        assert_eq!(next(&state, Some(c)), Some(b));
        // None loaded, or one not in the list: the first carried.
        assert_eq!(next(&state, None), Some(b));
        assert_eq!(next(&state, Some(FormId(0xD))), Some(b));
        // Only the loaded kind carried: nothing to swap to.
        state.items.remove(&(player, c));
        assert_eq!(next(&state, Some(b)), None);
        // The choice sticks while carried (`ammo_in_use`).
        state.items.insert((player, c), 5);
        state.ammo_loaded.insert(player, c);
        assert_eq!(loaded(&order, &state, player, &weapon), Some(c));
        // Pressed within the swap time: no animation, the timer runs on.
        let (swap, reset) = press(&order, &state, player, &weapon, true, 0.5);
        assert_eq!(
            swap,
            Some(Swap {
                ammo: b,
                animated: false
            })
        );
        assert!(!reset);
        let (swap, reset) = press(&order, &state, player, &weapon, true, 1.5);
        assert_eq!(
            swap,
            Some(Swap {
                ammo: b,
                animated: true
            })
        );
        assert!(reset);
    }
}
