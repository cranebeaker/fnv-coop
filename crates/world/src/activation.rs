//! What the reference under the crosshair offers the player, and what the
//! HUD's Info panel says about it, read from `FalloutNV.exe` 1.4.0.525:
//!
//! - the action class (`00579280`): what E would do, from the base's form
//!   type and the reference's state (a door with a name opens, furniture
//!   is sat in or slept in by its `MNAM` flags, the dead are searched, a
//!   person is talked to or, while the player sneaks, pickpocketed …);
//! - the words for it (`00775a00`, the HUD's Info update, with the table of
//!   `sTargetType…` settings at `011d5160` that `00f80050` fills): the
//!   action, the target's name (with its count, or "Door to …" for load
//!   doors), a lock line, "Empty" for empty containers, and the item's
//!   weight and value (`WG`, `VAL`);
//! - the "added" message a pickup shows, with the item's pickup sound
//!   (`004ce380`, sound `008adcf0`, `world::sound::item_sound`).
//!
//! The pick itself (which reference the crosshair is on) is
//! `physics::view_caster` (`0070bc20` → `00631d60`): a sphere of
//! `fActivatePickSphereRadius` (16) cast from the camera node (`Camera1st`
//! in first person) along the view, as far as `iActivatePickLength` (150),
//! see [`PICK_LENGTH`], [`PICK_RADIUS`]; [`fuzzy_replaces`] is its question
//! about base forms. Not followed: `0070bc20`'s second look, for a pick
//! that isn't an activator, container, door, actor or item, at the loaded
//! cells' placeable water (cell `+0xc4` `+0x5c`, read as `BGSPlaceableWater`
//! by `0057b240`'s cast) nearer than it (drinking from water).
//!
//! Not here (labelled where they would apply): the enemy health bar taking
//! a living target's name instead of the Info line, activators' own
//! activation text (extra data 0x53 and the base's text, read before the
//! table), class 10 ("Crown", `0087f3d0`), terminals' lock line (their
//! hacking state `00966c60`), and doors' "inaccessible" flag (`0057b460`,
//! door flags 0x100: no `DOOR` record field it could be was found).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{base_of, game_setting, game_setting_text, GameState};

/// `iActivatePickLength`: how far the activation pick reaches (the exe's
/// 150, read by `0070bc20` through the pick object's +0x18).
pub const PICK_LENGTH: f32 = 150.0;

/// `fActivatePickSphereRadius:Interface`: the radius of the sphere the pick
/// casts (16; `0070bc20` makes the shape with it, `00631ab0`).
pub const PICK_RADIUS: f32 = 16.0;

/// The action classes `00579280` returns, the index into the action words
/// (`ACTION_TEXT`).
pub mod class {
    pub const NONE: u8 = 0;
    pub const TAKE: u8 = 1;
    pub const OPEN: u8 = 2;
    pub const SIT: u8 = 3;
    pub const ACTIVATE: u8 = 4;
    pub const SLEEP: u8 = 5;
    pub const READ: u8 = 6;
    pub const TALK: u8 = 7;
    pub const OPEN_DOOR: u8 = 8;
    pub const CROWN: u8 = 10;
    pub const VAMPIRE: u8 = 11;
    pub const DRINK: u8 = 14;
}

/// The words for each class (`00f80050` fills the table at `011d5160`):
/// the setting and the exe's default when the data has none.
pub const ACTION_TEXT: [(&str, &str); 20] = [
    ("", ""),
    ("sTargetTypeTake", "Take"),
    ("sTargetTypeOpen", "Open"),
    ("sTargetTypeSit", "Sit"),
    ("sTargetTypeActivate", "Activate"),
    ("sTargetTypeSleep", "Sleep"),
    ("sTargetTypeRead", "Read"),
    ("sTargetTypeTalk", "Talk"),
    ("sTargetTypeOpenDoor", "Open"),
    ("sTargetTypeHorse", "Ride"),
    ("sTargetTypeCrown", "Talk"),
    ("sTargetTypeVampire", "Feed/Talk"),
    ("sTargetTypeEquip", "Equip"),
    ("sTargetTypeUnequip", "Unequip"),
    ("sTargetTypeDrink", "Drink"),
    ("sTargetTypeEat", "Eat"),
    ("sTargetTypeRecharge", "Recharge"),
    ("sTargetTypeBrew", "Brew"),
    ("sTargetTypeApply", "Apply"),
    ("sTargetTypeRepair", "Repair"),
];

/// The lock levels' names (the setting table at `01184a98`), by
/// difficulty (`world::locks::Lock::difficulty`).
pub const LOCK_LEVEL_NAMES: [(&str, &str); 6] = [
    ("sLockLevelNameVeryEasy", "Very Easy"),
    ("sLockLevelNameEasy", "Easy"),
    ("sLockLevelNameAverage", "Average"),
    ("sLockLevelNameHard", "Hard"),
    ("sLockLevelNameVeryHard", "Very Hard"),
    ("sLockLevelNameImpossible", "Requires Key"),
];

/// A text setting, else the exe's default.
fn text(order: &LoadOrder, setting: &str, exe: &str) -> String {
    game_setting_text(order, setting).unwrap_or_else(|| exe.to_string())
}

/// `ACBS` flags (`TESActorBaseData`, Xbox PDB): `GetAllowPCDialogue`
/// (vtable +0x18, `0047c790`) and `GetAllowPickPocket` (+0x1c, `0047c7b0`).
pub const ALLOW_PC_DIALOGUE: u32 = 0x0020_0000;
pub const ALLOW_PICKPOCKET: u32 = 0x1000_0000;

/// A base's `ACBS` flags (0 without).
pub fn actor_flags(order: &LoadOrder, base: FormId) -> u32 {
    order
        .get(base)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            r.get(FourCC::new(b"ACBS"))
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_u32(&s.data, 0))
        })
        .unwrap_or(0)
}

/// Whether a reference is a person or creature (the player too): what the
/// game asks through the reference's vtable +0x100.
fn is_actor(order: &LoadOrder, reference: FormId) -> bool {
    reference == PLAYER_REF
        || order
            .get(reference)
            .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"ACHR" | b"ACRE"))
}

/// A base record's type and record (a reference scripts or the player
/// made too: dropped items, ash piles).
fn base_record(
    order: &LoadOrder,
    state: &GameState,
    reference: FormId,
) -> Option<(FormId, FourCC, esm::Record)> {
    let base = crate::more_functions::placed::base_now(order, state, reference)?;
    let rr = order.get(base)?;
    let record = rr.record().ok()?;
    Some((base, rr.entry.header.kind, record))
}

/// What E would do to `reference` (`00579280`, translated). Not for
/// references set destroyed (`SetDestroyed`, form flag 0x800000,
/// `00477ba0`) unless they're people. People: nothing for the player
/// itself; the dead are searched (class 2; the essential-unconscious life
/// state 6 isn't modelled and counts as dead here); else, while the player
/// sneaks, class 1 (their pockets; not for talking activators), else talk.
/// Creatures likewise, but only those whose record allows talking to the
/// player (`ACBS` 0x200000) get the living classes. Class 10 (`0087f3d0`,
/// commanding a companion) isn't traced: never given here.
// Translated from 00579280 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn action_class(order: &LoadOrder, state: &GameState, reference: FormId) -> u8 {
    let actor = is_actor(order, reference);
    if !actor && state.destroyed.contains(&reference) {
        return class::NONE;
    }
    let Some((base, kind, record)) = base_record(order, state, reference) else {
        return class::NONE;
    };
    let named = || record.full_name().is_some_and(|n| !n.is_empty());
    let dead = state.dead.contains(&reference);
    match kind.as_bytes() {
        // ACTI: with a name (`00579620`).
        b"ACTI" => {
            if named() {
                class::ACTIVATE
            } else {
                class::NONE
            }
        }
        b"TACT" | b"NPC_" => {
            if reference == PLAYER_REF {
                class::NONE
            } else if actor && dead {
                class::OPEN
            } else if actor && state.player_sneaking && kind.as_bytes() != b"TACT" {
                class::TAKE
            } else {
                class::TALK
            }
        }
        b"ARMO" | b"CLOT" | b"INGR" | b"MISC" | b"WEAP" | b"AMMO" | b"KEYM" | b"ALCH" | b"NOTE"
        | b"COBJ" | b"LVLI" | b"PGRE" | b"IMOD" | b"CHIP" | b"CCRD" | b"CMNY" | b"FLOR" => {
            class::TAKE
        }
        b"BOOK" => class::READ,
        b"CONT" => class::OPEN,
        b"DOOR" => {
            if named() {
                class::OPEN_DOOR
            } else {
                class::NONE
            }
        }
        // A light that can be carried (`DATA` flags 0x2, `0046f070`) is
        // taken; others are activated (the case falls through to TERM's).
        b"LIGH" => {
            let flags = record
                .get(esm::sig::DATA)
                .filter(|s| s.data.len() >= 16)
                .map_or(0, |s| le_u32(&s.data, 12));
            if flags & 0x2 != 0 {
                class::TAKE
            } else {
                class::ACTIVATE
            }
        }
        b"TERM" => class::ACTIVATE,
        b"PWAT" => class::DRINK,
        b"FURN" => {
            let flags = crate::furniture::marker_flags(order, base);
            if flags & crate::furniture::SIT_FURNITURE != 0 {
                class::SIT
            } else if flags & crate::furniture::BED != 0 {
                class::SLEEP
            } else {
                class::NONE
            }
        }
        b"CREA" => {
            if !actor {
                return class::NONE;
            }
            if dead {
                return class::OPEN;
            }
            if actor_flags(order, base) & ALLOW_PC_DIALOGUE == 0 {
                class::NONE
            } else if state.player_sneaking {
                class::TAKE
            } else {
                class::TALK
            }
        }
        _ => class::NONE,
    }
}

/// Whether the view caster's fuzzy candidate takes the place of an exact
/// pick on `reference` (`00631d60`'s last switch on the base's form type):
/// a static, static collection or tree (0x20, 0x21, 0x25), a light that
/// can't be carried (0x1e without `DATA` flags 0x2, `0046f070`), or a
/// reference with no base.
// Translated from 00631d60 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fuzzy_replaces(order: &LoadOrder, reference: FormId) -> bool {
    // A reference of the plugins (the view caster's candidates are placed
    // references; none made in game).
    let Some((kind, record)) = base_of(order, reference).and_then(|base| {
        let rr = order.get(base)?;
        Some((rr.entry.header.kind, rr.record().ok()?))
    }) else {
        return true;
    };
    match kind.as_bytes() {
        b"STAT" | b"SCOL" | b"TREE" => true,
        b"LIGH" => {
            let flags = record
                .get(esm::sig::DATA)
                .filter(|s| s.data.len() >= 16)
                .map_or(0, |s| le_u32(&s.data, 12));
            flags & 0x2 == 0
        }
        _ => false,
    }
}

/// The reference activating `reference` really activates
/// (`TESObjectREFR::Activate`, `00573170`): an ash or goo pile
/// (`DefaultAshPile1` / `2`) carrying a link to a corpse
/// (`ExtraAshPileRef`, `0041e310`; `AttachAshPile` made it) passes the
/// activation on to that corpse, so the disintegrated are searched
/// through their pile; anything else is itself.
// Translated from 00573170 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn stands_for(order: &LoadOrder, state: &GameState, reference: FormId) -> FormId {
    use crate::more_functions::placed;
    let pile = placed::base_now(order, state, reference)
        .is_some_and(|b| b == placed::ASH_PILE || b == placed::GOO_PILE);
    match state.more.ash_piles.get(&reference) {
        Some(&corpse) if pile => corpse,
        _ => reference,
    }
}

/// What the Info panel shows for a reference (`00775a00`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Info {
    /// The action and the key's line (`HUDMainMenu` +0x9c), when there's
    /// one.
    pub action: Option<String>,
    /// The target line (+0xa8): its name, with its count, or "Door to …".
    pub target: String,
    /// The crime colour (system colour 2) instead of the HUD's (1)
    /// (`00579690`).
    pub crime: bool,
    /// The lock line (+0xac): "[Locked - Easy]", "[Locked - Use Key]",
    /// "[Requires Key]", "[Locked - Broken]".
    pub lock: Option<String>,
    /// The "Empty" line (+0xb0), for empty containers and bodies.
    pub empty: Option<String>,
    /// Weight and value with their labels (+0xb4 … +0xc0), for items.
    pub weight_value: Option<WeightValue>,
}

/// An item's weight and value lines (`00775a00`, formats `%.1f` and
/// `%.0f`, `--` for none).
#[derive(Debug, Clone, PartialEq)]
pub struct WeightValue {
    pub weight: String,
    pub weight_label: String,
    pub value: String,
    pub value_label: String,
}

/// The name of where a load door leads, for "Door to …" (`00578870` on
/// the destination door): outdoors, the worldspace's name for the place
/// the door stands (`world::region::location_name`, `00586500`: the map
/// region there), else the worldspace's own name (the game's further
/// fallbacks aren't followed); indoors, the cell's `FULL` (the cell's own
/// vtable +0x138, which isn't followed).
pub fn door_destination(order: &LoadOrder, door: FormId) -> Option<String> {
    let rr = order.get(door)?;
    let record = rr.record().ok()?;
    let xtel = record
        .get(FourCC::new(b"XTEL"))
        .filter(|s| s.data.len() >= 4)?;
    let to = rr.plugin.to_global(FormId(le_u32(&xtel.data, 0)));
    let to_rr = order.get(to)?;
    let cell = order.cell_of(&to_rr);
    let named = |id: FormId| {
        order
            .get(id)
            .and_then(|c| c.record().ok())
            .and_then(|c| c.full_name())
            .filter(|n| !n.is_empty())
    };
    match order.world_of(&to_rr) {
        Some(world) => {
            let at = crate::scripting::whereabouts(order, to).map(|w| w.position);
            at.and_then(|p| crate::region::location_name(order, world, cell, p[0], p[1]))
                .or_else(|| named(world))
        }
        None => cell.and_then(named),
    }
}

/// How many a placed item stands for (`XCNT`, else 1; `00418770`).
pub fn placed_count(order: &LoadOrder, reference: FormId) -> i32 {
    order
        .get(reference)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            r.get(FourCC::new(b"XCNT"))
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_u32(&s.data, 0) as i32)
        })
        .unwrap_or(1)
}

/// Whether the crosshair's line is red (`00579690`): `living::
/// crosshair_red`, doors (whose rule isn't followed) counting as not.
fn crime(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    crate::living::crosshair_red(order, state, reference).unwrap_or(false)
}

/// Whether a holder has nothing: its own items if they've been copied into
/// the state, else its record's contents (leveled lists counting as
/// something, since the game resolves them when the cell loads).
fn holds_nothing(order: &LoadOrder, state: &GameState, holder: FormId) -> bool {
    if state.stocked.contains(&holder) {
        return state.inventory(order, holder).is_empty();
    }
    let Some(base) = base_of(order, holder) else {
        return true;
    };
    !order
        .get(base)
        .and_then(|r| r.record().ok())
        .is_some_and(|r| {
            r.get_all(FourCC::new(b"CNTO"))
                .any(|s| s.data.len() >= 8 && le_u32(&s.data, 4) as i32 > 0)
        })
}

/// What the Info panel shows for `reference` (`00775a00`, translated for
/// the parts listed in the module notes). `None` when it shows nothing
/// (statics, trees, nameless things without an action).
// Translated from 00775a00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn info(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<Info> {
    let (base, kind, _) = base_record(order, state, reference)?;
    let k = *kind.as_bytes();
    // Statics, static collections and trees: nothing (the type switch's
    // 0x20, 0x21, 0x25 case); a light that can't be carried leaves the
    // panel as it was (the 0x1e case returns): nothing here either.
    if matches!(&k, b"STAT" | b"SCOL" | b"TREE") {
        return None;
    }
    let class = action_class(order, state, reference);
    let actor = is_actor(order, reference);
    let dead = state.dead.contains(&reference);
    let mut out = Info {
        crime: crime(order, state, reference),
        ..Info::default()
    };
    // The name: the reference's display name, "%s (%d)" with its count,
    // or for a load door "%s %s %s" (its name, `sTo`, where it leads).
    let name = crate::script_functions::full_name(order, state, reference).unwrap_or_default();
    let count = if actor {
        1
    } else {
        placed_count(order, reference)
    };
    out.target = if k == *b"DOOR" {
        match door_destination(order, reference) {
            Some(to) if !name.is_empty() => format!("{name} {} {to}", text(order, "sTo", "to")),
            _ => name.clone(),
        }
    } else if count > 1 && !name.is_empty() {
        format!("{name} ({count})")
    } else {
        name.clone()
    };
    // The action words.
    let crime_take = class == class::TAKE && out.crime;
    let teammate = actor && state.teammates.contains(&reference);
    out.action = if crime_take {
        Some(if teammate {
            text(order, ACTION_TEXT[7].0, ACTION_TEXT[7].1)
        } else if k == *b"NPC_"
            || (k == *b"CREA" && actor_flags(order, base) & ALLOW_PC_DIALOGUE != 0)
        {
            text(order, "sPickpocket", "Pickpocket")
        } else {
            text(order, "sSteal", "Steal")
        })
    } else if class == class::OPEN && matches!(&k, b"NPC_" | b"CREA") {
        Some(text(order, "sSearch", "Search"))
    } else if class == class::OPEN_DOOR && k == *b"DOOR" {
        // A locked door the player can't open with a key: "Pick"; an open
        // door: "Close" (`sCloseButton`); else "Open".
        let lock = crate::locks::lock_now(order, state, reference);
        let has_key = lock
            .and_then(|l| l.key)
            .is_some_and(|key| state.item_count(order, PLAYER_REF, key) > 0);
        let open = crate::doors::open_state(order, state, reference).is_open();
        Some(if lock.is_some() && !has_key {
            text(order, "sPick", "Pick")
        } else if open {
            text(order, "sCloseButton", "Close")
        } else {
            text(order, ACTION_TEXT[8].0, ACTION_TEXT[8].1)
        })
    } else if class == class::ACTIVATE && k == *b"CREA" {
        None
    } else if k == *b"PROJ" && crate::mines::is_mine_reference(order, state, reference) {
        // A placed projectile (no action class of its own) with a
        // proximity: "Disarm Mine" (`sDisarmMine`) while armed, the table's
        // "Take" once disarmed (run-time flag 0x200; `00775a00` at
        // `007778fe`).
        Some(if crate::mines::is_armed_mine(order, state, reference) {
            text(order, "sDisarmMine", "Disarm Mine")
        } else {
            text(order, ACTION_TEXT[1].0, ACTION_TEXT[1].1)
        })
    } else if class == class::READ {
        // Books say "Take" (the table's entry 1).
        Some(text(order, ACTION_TEXT[1].0, ACTION_TEXT[1].1))
    } else if class != class::NONE && usize::from(class) < ACTION_TEXT.len() {
        // A living person whose pockets the sneaking player would pick but
        // who may not be (`0087f3d0`, not followed): the class 1 text.
        let (setting, exe) = ACTION_TEXT[usize::from(class)];
        Some(text(order, setting, exe))
    } else {
        None
    };
    // The lock line: containers and doors that are locked; broken locks.
    if matches!(&k, b"CONT" | b"DOOR") {
        if let Some(lock) = crate::locks::lock_now(order, state, reference) {
            let has_key = lock
                .key
                .is_some_and(|key| state.item_count(order, PLAYER_REF, key) > 0);
            let locked = text(order, "sLocked", "Locked");
            out.lock = Some(if crate::lockpick::is_broken(order, state, reference) {
                text(order, "sBroken", "[Locked - Broken]")
            } else if has_key {
                format!("[{locked} - {}]", text(order, "sHUDUseKey", "Use Key"))
            } else {
                let (difficulty, _) = lock.difficulty(order);
                let (setting, exe) = LOCK_LEVEL_NAMES[usize::from(difficulty.min(5))];
                if difficulty >= 5 {
                    format!("[{}]", text(order, setting, exe))
                } else {
                    format!("[{locked} - {}]", text(order, setting, exe))
                }
            });
        }
    }
    // "Empty": a container or a body holding nothing, when no lock line
    // shows.
    let holder = k == *b"CONT" || (actor && dead);
    if holder && out.lock.is_none() && holds_nothing(order, state, reference) {
        out.empty = Some(text(order, "sEmpty", "Empty"));
    }
    // An ash or goo pile standing for a corpse (`ExtraAshPileRef`,
    // `0041e310`): "Empty" when the corpse holds nothing (`00775a00`,
    // the test after its call to `0041e310`).
    let corpse = stands_for(order, state, reference);
    if corpse != reference && holds_nothing(order, state, corpse) {
        out.empty = Some(text(order, "sEmpty", "Empty"));
    }
    // Weight and value for the item kinds the type switch flags (0xfc):
    // armour, carried lights, weapons and ammunition (the playable ones:
    // `0047bcf0`, `004c94d0`, not followed), misc items, aid, item mods,
    // placeable water (form type 0x23), placed projectiles aren't here.
    let carried_light = k == *b"LIGH" && class == class::TAKE;
    if carried_light
        || matches!(
            &k,
            b"ARMO" | b"WEAP" | b"AMMO" | b"MISC" | b"ALCH" | b"IMOD"
        )
    {
        out.weight_value = Some(weight_value(order, state, reference, base, k));
    }
    let shown = out.action.is_some() || !out.target.is_empty();
    shown.then_some(out)
}

/// The weight and value strings (`00775a00`): value `004bd400`
/// (`barter::item_value`), "--" at 0 or less, else `%.1f` under 1 and
/// `%.0f` from 1; weight `0048ebc0` (weapons `004be380`, and from 10 up
/// × the player's perks' entry point 73, Adjust Heavy Weapon Weight),
/// "--" at 0 or less, else `%.1f`.
fn weight_value(
    order: &LoadOrder,
    state: &GameState,
    reference: FormId,
    base: FormId,
    kind: [u8; 4],
) -> WeightValue {
    let value = crate::barter::item_value(order, state, reference, base);
    let value_text = if value <= 0.0 {
        "--".to_string()
    } else if value < 1.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.0}")
    };
    let hardcore = state.living.hardcore;
    let mut weight = crate::items::weight(order, base, hardcore);
    if &kind == b"WEAP" && weight >= 10.0 {
        weight *= crate::perks::apply(
            order,
            state,
            crate::perks::entry::ADJUST_HEAVY_WEAPON_WEIGHT,
            1.0,
        );
    }
    let weight_text = if weight <= 0.0 {
        "--".to_string()
    } else {
        format!("{weight:.1}")
    };
    WeightValue {
        weight: weight_text,
        weight_label: text(order, "sInventoryWeightUpper", "WG"),
        value: value_text,
        value_label: text(order, "sInventoryValue", "VAL"),
    }
}

/// The message a pickup shows (`004ce380`): "%s %s" (the item's name,
/// `sAddItemtoInventory` "added") for one, "%i %s%s %s" (the count, the
/// name, `sPlural` "(s)", "added") for more; only for the item kinds the
/// game announces (armour, books, lights, misc, weapons, ammunition, keys,
/// aid, recipes, item mods, casino chips, caravan cards and money). The
/// message carries the item's pickup sound ([`crate::sound::item_sound`],
/// `008adcf0(item, 1, 0)`).
pub fn pickup_message(order: &LoadOrder, item: FormId, count: i32) -> Option<String> {
    let rr = order.get(item)?;
    let announced = matches!(
        rr.entry.header.kind.as_bytes(),
        b"ARMO"
            | b"BOOK"
            | b"LIGH"
            | b"MISC"
            | b"WEAP"
            | b"AMMO"
            | b"KEYM"
            | b"ALCH"
            | b"COBJ"
            | b"IMOD"
            | b"CHIP"
            | b"CCRD"
            | b"CMNY"
    );
    if !announced {
        return None;
    }
    let name = rr.record().ok()?.full_name().unwrap_or_default();
    let added = text(order, "sAddItemtoInventory", "added");
    Some(if count < 2 {
        format!("{name} {added}")
    } else {
        format!("{count} {name}{} {added}", text(order, "sPlural", "(s)"))
    })
}

/// A float setting (for the callers' picks), else the exe's value.
pub fn setting(order: &LoadOrder, name: &str, exe: f32) -> f32 {
    game_setting(order, name).unwrap_or(exe)
}
