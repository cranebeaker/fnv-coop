//! What a person wears now, as drawn: the clothes and armour their record
//! has them wear from the start, changed by what the game state has done
//! since (equipping, taking off, taking away).
//!
//! In the game an actor's worn items are flags on its inventory entries.
//! Putting something on (`Actor::AddWornItem`, Xbox PDB; `0088db20`, from
//! the equip routine `0088c830`, which does nothing for an item the actor
//! doesn't carry) first takes off whatever is worn on any body slot the
//! new item fills, then, for a person other than the player, puts the item
//! and the addons of its biped model list into the biped's slots
//! (`TESNPC::InitWornObject` `006061b0` → `TESBipedModelForm::AddToBiped`
//! `00480bd0` → `BipedAnim::SetBipedPart` `004abad0`). A worn item leaving
//! the inventory has the actor pick again what to wear
//! (`MiddleHighProcess::ProcessRemoveWorn`, Xbox `8272e1f8` →
//! `TESNPC::InitDefaultWorn`). The model is then rebuilt part by part
//! (`BipedAnim::LoadBipedParts`, Xbox `822fd188`: a slot whose model didn't
//! change keeps its loaded 3D).
//!
//! Here the start is what the look draws from the record
//! ([`crate::actor::worn_of`] over the record's inventory, a guess at the
//! game's `GetBestArmor`); `GameState::equipped` holds what scripts and
//! fights put on since, and `GameState::taken_off` which of the start's
//! came off. [`worn_armour`] puts them together.

use esm::{FormId, LoadOrder};

use crate::actor::{data_record, inventory_entries, is_female, worn_of, USE_INVENTORY};
use crate::scripting::{base_of, GameState};

/// One piece of clothing or armour a person wears from the start.
#[derive(Debug, Clone, PartialEq)]
pub struct StartPiece {
    pub item: FormId,
    /// The body slots it covers (its `BMDT`).
    pub slots: u32,
    /// What the record's entry it came from can give: the item itself, or
    /// for a leveled list everything the list can give.
    pub from: Vec<FormId>,
    /// The entry is the person's own (not their template's): the state's
    /// copy of their inventory (`GameState::stock`, from their own record)
    /// holds what it gave.
    pub own: bool,
}

/// What a person wears from the start, by their base record.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartWorn {
    pub pieces: Vec<StartPiece>,
}

impl StartWorn {
    pub fn contains(&self, item: FormId) -> bool {
        self.pieces.iter().any(|p| p.item == item)
    }
}

/// What a person (base record) wears from the start: the clothes and
/// armour the game puts on from their record's container
/// ([`crate::actor::pick_worn`], `006047c0`), as their look draws them.
pub fn start_worn(order: &LoadOrder, base: FormId) -> StartWorn {
    let Some((rr, record)) = data_record(order, base, USE_INVENTORY) else {
        return StartWorn::default();
    };
    let own = rr.form_id == base;
    let entries = inventory_entries(order, &rr, &record);
    let items: Vec<(FormId, FormId)> = entries
        .iter()
        .flat_map(|(entry, items)| items.iter().map(move |(item, _)| (*item, *entry)))
        .collect();
    let pieces = worn_of(
        order,
        crate::actor::pick_worn(order, &entries),
        is_female(order, base),
    )
    .into_iter()
    .map(|(item, armor)| {
        let entry = items
            .iter()
            .find(|(i, _)| *i == item)
            .map_or(item, |(_, e)| *e);
        StartPiece {
            item,
            slots: armor.slots,
            from: if entry == item {
                vec![item]
            } else {
                crate::leveled::outcomes(order, entry)
            },
            own,
        }
    })
    .collect();
    StartWorn { pieces }
}

/// The clothes and armour `who` wears now, in order (the start's first,
/// then what was put on since), given what they wore from the start
/// (`start`, [`start_worn`] of their base record):
/// - what was put on since (`GameState::equipped`'s clothes and armour)
///   while they carry it (or, their inventory never touched, while it's
///   among the start's);
/// - each piece of the start's unless what was put on covers one of its
///   slots, it was taken off (by `UnequipItem`, or for something put on
///   over it that they still carry: when that leaves their inventory they
///   pick again, approximated by the start's coming back), or their own
///   inventory, once copied into the state, no longer holds anything its
///   record entry can give (it was taken away: `RemoveItem`,
///   `RemoveAllItems`, trading, stealing).
pub fn worn_armour(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    start: &StartWorn,
) -> Vec<FormId> {
    let stocked = state.stocked.contains(&who);
    let has = |item: FormId| state.item_count(order, who, item) > 0;
    let put_on: Vec<(FormId, u32)> = state
        .equipped
        .get(&who)
        .into_iter()
        .flatten()
        .filter_map(|&item| Some((item, crate::actor::Armor::load(order, item)?.slots)))
        .filter(|&(item, _)| has(item) || (!stocked && start.contains(item)))
        .collect();
    let off = state.taken_off.get(&who);
    let mut out: Vec<FormId> = start
        .pieces
        .iter()
        .filter(|p| !put_on.iter().any(|&(_, slots)| slots & p.slots != 0))
        .filter(|p| {
            !off.is_some_and(|l| {
                l.iter()
                    .any(|&(item, by)| item == p.item && by.map_or(true, has))
            })
        })
        .filter(|p| !(stocked && p.own && !p.from.iter().any(|&f| has(f))))
        .map(|p| p.item)
        .collect();
    out.extend(put_on.into_iter().map(|(item, _)| item));
    out
}

/// What equipping `item` on `who` takes off of what they wore from the
/// start: the pieces on any body slot it covers (`0088db20`), with what
/// replaced them.
pub fn replaced_by(order: &LoadOrder, who: FormId, item: FormId) -> Vec<(FormId, Option<FormId>)> {
    let Some(slots) = crate::actor::Armor::load(order, item).map(|a| a.slots) else {
        return Vec::new();
    };
    let Some(base) = base_of(order, who) else {
        return Vec::new();
    };
    start_worn(order, base)
        .pieces
        .into_iter()
        .filter(|p| p.item != item && p.slots & slots != 0)
        .map(|p| (p.item, Some(item)))
        .collect()
}

/// Whether `item` is among what `who` wore from the start.
pub fn worn_from_start(order: &LoadOrder, who: FormId, item: FormId) -> bool {
    base_of(order, who).is_some_and(|base| start_worn(order, base).contains(item))
}
