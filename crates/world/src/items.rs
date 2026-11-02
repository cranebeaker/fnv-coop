//! What an item is worth and weighs, read from its record. Each type keeps
//! them in its own place (checked on one record of each):
//!
//! - `MISC`, `KEYM`, `IMOD`: `DATA` value i32, weight f32.
//! - `WEAP`: `DATA` value i32, health i32, weight f32, base damage i16,
//!   clip size u8 (the sawed-off shotgun: 1950, 80, 4, 100, 2).
//! - `ARMO`: `DATA` value i32, health i32, weight f32.
//! - `AMMO`: `DATA` speed f32, flags u8, 3 unused, value i32, clip rounds
//!   u8; the weight is in `DAT2` (projectiles per shot u32, projectile,
//!   weight f32, …).
//! - `ALCH`: `DATA` weight f32; `ENIT` value i32 first.
//! - `BOOK`: `DATA` flags u8, skill i8, value i32, weight f32.
//! - `CMNY` (other currencies), `CCRD` (Caravan cards): `DATA` value; no
//!   weight. `NOTE` and `CHIP`: neither.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::Condition;
use crate::scripting::{Event, GameState};

const ENIT: FourCC = FourCC::new(b"ENIT");
const DAT2: FourCC = FourCC::new(b"DAT2");
const EFID: FourCC = FourCC::new(b"EFID");
const EFIT: FourCC = FourCC::new(b"EFIT");
const CTDA: FourCC = FourCC::new(b"CTDA");
const MGEF: FourCC = FourCC::new(b"MGEF");

/// One effect of an item: `EFID` the magic effect, `EFIT` magnitude,
/// area, duration (seconds), range (0 self) and actor value, then the
/// `CTDA`s that decide whether it applies. Checked on the Stimpak: four
/// effects, two `RestoreHealth` (5 a second for 6 s, 6 with the Fast
/// Metabolism perk) for hardcore mode, two `RestoreHealthStimpak` (30 at
/// once, 36 with the perk) otherwise.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemEffect {
    pub effect: FormId,
    pub name: String,
    pub magnitude: i32,
    pub duration: i32,
    /// From the magic effect (`MGEF` `DATA`, 72 bytes; see
    /// [`crate::magic`]): flags u32 at 0 (0x04 harmful: it lowers the
    /// value; 0x02 recover: the value comes back when it ends), the
    /// associated item at 8 (a script effect's script), the resisting
    /// actor value i32 at 16 (-1 none), archetype u32 at 64 (0 value
    /// modifier, 1 script, 34 value and limb conditions), actor value i32
    /// at 68.
    pub harmful: bool,
    pub recover: bool,
    /// Flag 0x01, hostile: a chem's effect that isn't a benefit (its
    /// duration isn't lengthened by Chemist, `crate::magic`).
    pub hostile: bool,
    /// The magic effect's flags as they are (`MGEF` `DATA` u32 at 0; the
    /// game's +0x58): 0x80 no duration, 0x100 no magnitude, 0x2000 its
    /// name shown alone, 0x400000 (with hostile) poison.
    pub flags: u32,
    pub script: Option<FormId>,
    pub resist: i32,
    pub archetype: u32,
    pub actor_value: i32,
    pub conditions: Vec<Condition>,
}

/// An item's effects, in order.
pub fn effects(order: &LoadOrder, item: FormId) -> Vec<ItemEffect> {
    let Some(rr) = order.get(item) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    let mut out: Vec<ItemEffect> = Vec::new();
    for sub in &record.subrecords {
        if sub.kind == EFID && sub.data.len() >= 4 {
            let effect = rr.plugin.to_global(FormId(le_u32(&sub.data, 0)));
            let mgef = order
                .get(effect)
                .filter(|r| r.entry.header.kind == MGEF)
                .and_then(|r| r.record().ok());
            let data = mgef
                .as_ref()
                .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
                .unwrap_or_default();
            let u = |at: usize| (data.len() >= at + 4).then(|| le_u32(&data, at));
            let archetype = u(64).unwrap_or(0);
            let script = u(8)
                .filter(|&s| s != 0 && archetype == crate::magic::archetype::SCRIPT)
                .and_then(|s| Some(order.get(effect)?.plugin.to_global(FormId(s))));
            out.push(ItemEffect {
                effect,
                name: mgef
                    .as_ref()
                    .and_then(|r| r.full_name())
                    .unwrap_or_default(),
                magnitude: 0,
                duration: 0,
                harmful: u(0).is_some_and(|f| f & 0x04 != 0),
                recover: u(0).is_some_and(|f| f & 0x02 != 0),
                hostile: u(0).is_some_and(|f| f & 0x01 != 0),
                flags: u(0).unwrap_or(0),
                script,
                resist: u(16).map_or(-1, |v| v as i32),
                archetype,
                actor_value: u(68).map_or(-1, |v| v as i32),
                conditions: Vec::new(),
            });
        } else if sub.kind == EFIT && sub.data.len() >= 20 {
            if let Some(e) = out.last_mut() {
                e.magnitude = le_u32(&sub.data, 0) as i32;
                e.duration = le_u32(&sub.data, 8) as i32;
                let av = le_u32(&sub.data, 16) as i32;
                if av >= 0 {
                    e.actor_value = av;
                }
            }
        } else if sub.kind == CTDA {
            if let Some(e) = out.last_mut() {
                e.conditions
                    .extend(crate::dialogue::read_condition(&rr, &sub.data));
            }
        }
    }
    out
}

/// An item's value (its `DATA` value, as [`item_info`] reads it), or as
/// `SetItemValue` set it; 0 for a form without one.
pub fn value(order: &LoadOrder, state: &GameState, item: FormId) -> i32 {
    match state.more.item_values.get(&item) {
        Some(&v) => v,
        None => item_info(order, item).map_or(0, |i| i.value),
    }
}

/// A default object (the `DOBJ` record `DefaultObjectManager`: its `DATA`
/// an array of forms, `0058db10` reading slot n): 0 the Stimpak, 2 Rad-X,
/// 3 RadAway, 21 the Doctor's Bag (the STATS menu's aid buttons,
/// `007da2c0`).
pub fn default_object(order: &LoadOrder, index: usize) -> Option<FormId> {
    let rr = order.records_of_type(FourCC::new(b"DOBJ")).next()?;
    let rr = order.get(rr.form_id)?;
    let record = rr.record().ok()?;
    let data = record.get(esm::sig::DATA)?.data.clone();
    let at = index * 4;
    let raw = u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?);
    (raw != 0).then(|| rr.plugin.to_global(FormId(raw)))
}

/// An aid item's (`ALCH`) `ENIT` flags (u8 at 4): 0x01 no auto-calculate,
/// 0x02 a food item, 0x04 a medicine (the Stimpak's 0x05). `None` for
/// anything but an aid item.
pub fn ingestible_flags(order: &LoadOrder, item: FormId) -> Option<u8> {
    let rr = order
        .get(item)
        .filter(|r| r.entry.header.kind.as_bytes() == b"ALCH")?;
    let record = rr.record().ok()?;
    Some(record.get(ENIT)?.data.get(4).copied().unwrap_or(0))
}

/// `ENIT` flags of aid items.
pub mod ingestible {
    pub const FOOD: u8 = 0x02;
    pub const MEDICINE: u8 = 0x04;
}

/// Using an aid item (eating, drinking, injecting): one is used up, its
/// sound plays (`ENIT`'s consume sound, a form at 16), and each effect
/// whose conditions pass for the user applies ([`crate::magic::apply`]).
/// What it did, or `None` when the user has none.
pub fn use_item(
    order: &LoadOrder,
    state: &mut GameState,
    user: FormId,
    item: FormId,
) -> Option<String> {
    if state.item_count(order, user, item) <= 0 {
        return None;
    }
    state.stock(order, user);
    if let Some(n) = state.items.get_mut(&(user, item)) {
        *n -= 1;
    }
    let rr = order.get(item)?;
    let record = rr.record().ok()?;
    if let Some(enit) = record.get(ENIT).filter(|s| s.data.len() >= 20) {
        let sound = rr.plugin.to_global(FormId(le_u32(&enit.data, 16)));
        if sound.0 != 0 {
            state.events.push(Event::Sound(sound));
        }
    }
    let name = record.full_name().unwrap_or_default();
    let done = crate::magic::apply(order, state, user, item, user, false);
    Some(if done.is_empty() {
        format!("Used {name}.")
    } else {
        format!("Used {name}: {}.", done.join(", "))
    })
}

/// The skill a book teaches (`DATA` byte 1, counting the skills from
/// Barter, actor value 32; −1 for none), if it's a skill book.
pub fn book_skill(order: &LoadOrder, book: FormId) -> Option<u16> {
    let rr = order
        .get(book)
        .filter(|r| r.entry.header.kind.as_bytes() == b"BOOK")?;
    let record = rr.record().ok()?;
    let skill = record.get(esm::sig::DATA)?.data.get(1).copied()? as i8;
    (skill >= 0).then(|| 32 + skill as u16)
}

/// Why the player can't read a book now, outside the menus (`00515040`):
/// in combat (the actor's +0x104 flag), `sCanNotReadBook` "You cannot
/// read a book during combat!" is shown and the book kept. From a menu
/// (the Pip-Boy, `00702360`: the interface not in game mode) it's never
/// refused.
pub fn cannot_read_in_combat(order: &LoadOrder, state: &GameState) -> Option<String> {
    let player = crate::dialogue::PLAYER_REF;
    let fighting =
        state.combat.contains_key(&player) || state.combat.values().any(|t| *t == player);
    fighting.then(|| {
        crate::scripting::game_setting_text(order, "sCanNotReadBook")
            .unwrap_or_else(|| "You cannot read a book during combat!".to_string())
    })
}

/// Why the player can't drop an item from the Pip-Boy (ITEMS' Drop,
/// `00780140` case 7), as the game asks in turn: a quest item
/// (`sDropQuestItemWarning`); an equipped one while they're in the middle
/// of an action (`008a7570` not -1: `sDropEquippedItemWarning`); in the air
/// (`009337b0`: `sNoJumpWarning`). The game's further checks, a worn item
/// that can't come off (`sCantRemoveWornItem`) and no room in front of them
/// (`009614b0`: `sNotEnoughRoomWarning`), aren't asked here. The setting
/// whose text is shown.
pub fn drop_refusal(
    order: &LoadOrder,
    state: &GameState,
    item: FormId,
    acting: bool,
    in_air: bool,
) -> Option<&'static str> {
    if crate::script_functions::is_quest_item(order, state, item) {
        return Some("sDropQuestItemWarning");
    }
    if acting && state.is_equipped(crate::dialogue::PLAYER_REF, item) {
        return Some("sDropEquippedItemWarning");
    }
    if in_air {
        return Some("sNoJumpWarning");
    }
    None
}

/// How many the Pip-Boy drops without asking (`00780140`: more than
/// `iInventoryAskQuantityAt`, 5, asks "How many?"; fewer drop one).
pub fn drop_asks(order: &LoadOrder, count: i32) -> bool {
    let at = crate::scripting::game_setting(order, "iInventoryAskQuantityAt").unwrap_or(5.0);
    count > at as i32
}

/// The player reads a skill book (read from the game's code, `00515040`
/// and its caller `0088c830`): while the skill's permanent value is under
/// 200, it rises by `fBookPerkBonus` (3; the exe's default 1) after the
/// perks' entry point 11 "Adjust Book Skill Points" (Comprehension + 1),
/// for good, the book is used up (one taken away), "Books Read" counts
/// one more, and the notice is `sSkillIncreasedNum` "%s increased by %d"
/// with the skill's name and the whole points. At 200 or more nothing
/// happens and nothing is said (the book stays). What to tell the player,
/// or `None` if it isn't a skill book they have or nothing happened. New
/// Vegas has no book-reading menu: `BookMenu` (1026) is only reached by
/// the debug console's menu commands (`0071b210`), and its file names
/// pictures the game doesn't ship.
pub fn read_book(order: &LoadOrder, state: &mut GameState, book: FormId) -> Option<String> {
    let player = crate::dialogue::PLAYER_REF;
    let skill = book_skill(order, book)?;
    if state.item_count(order, player, book) <= 0 {
        return None;
    }
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    let now = facts.permanent_actor_value(player, skill).unwrap_or(0.0);
    let name = crate::chargen::actor_value_name(order, skill);
    if now >= 200.0 {
        return None;
    }
    let bonus = crate::scripting::game_setting(order, "fBookPerkBonus").unwrap_or(1.0);
    let points = crate::perks::apply(
        order,
        state,
        crate::perks::entry::ADJUST_BOOK_SKILL_POINTS,
        bonus,
    );
    let points = points.max(0.0) as u32;
    if state.actor_values.contains_key(&(player, skill)) {
        // A value scripts set is kept as it is; raise it there.
        *state.actor_values.get_mut(&(player, skill))? += f64::from(points);
    } else {
        *state.skill_points.entry(skill).or_insert(0) += points;
    }
    state.stock(order, player);
    if let Some(n) = state.items.get_mut(&(player, book)) {
        *n -= 1;
    }
    crate::stats::bump(state, crate::stats::BOOKS_READ, 1);
    let text = crate::scripting::game_setting_text(order, "sSkillIncreasedNum")
        .unwrap_or_else(|| "%s increased by %d".to_string());
    Some(
        text.replacen("%s", &name, 1)
            .replacen("%d", &points.to_string(), 1),
    )
}

/// An item's name, value (caps) and weight.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
    pub form_id: FormId,
    pub kind: FourCC,
    pub name: String,
    pub value: i32,
    pub weight: f32,
}

pub fn item_info(order: &LoadOrder, id: FormId) -> Option<ItemInfo> {
    let rr = order.get(id)?;
    let kind = rr.entry.header.kind;
    let record = rr.record().ok()?;
    let data = record
        .get(esm::sig::DATA)
        .map(|s| s.data.as_slice())
        .unwrap_or(&[]);
    let i32_at = |d: &[u8], at: usize| (d.len() >= at + 4).then(|| le_u32(d, at) as i32);
    let f32_at = |d: &[u8], at: usize| (d.len() >= at + 4).then(|| le_f32(d, at));
    let (value, weight) = match kind.as_bytes() {
        b"MISC" | b"KEYM" | b"IMOD" => (i32_at(data, 0), f32_at(data, 4)),
        b"WEAP" | b"ARMO" => (i32_at(data, 0), f32_at(data, 8)),
        b"AMMO" => {
            let dat2 = record.get(DAT2).map(|s| s.data.as_slice()).unwrap_or(&[]);
            (i32_at(data, 8), f32_at(dat2, 8))
        }
        b"ALCH" => {
            let enit = record.get(ENIT).map(|s| s.data.as_slice()).unwrap_or(&[]);
            (i32_at(enit, 0), f32_at(data, 0))
        }
        b"BOOK" => (i32_at(data, 2), f32_at(data, 6)),
        b"CMNY" | b"CCRD" => (i32_at(data, 0), None),
        _ => (None, None),
    };
    Some(ItemInfo {
        form_id: id,
        kind,
        name: record
            .full_name()
            .filter(|n| !n.is_empty())
            .or_else(|| record.editor_id())
            .unwrap_or_else(|| id.to_string()),
        value: value.unwrap_or(0),
        weight: weight.unwrap_or(0.0),
    })
}

/// An item's weight as the game counts it (`0048ebc0`): `ARMO`, `WEAP`,
/// `BOOK`, `MISC`, `KEYM`, `IMOD`, `ALCH` as [`item_info`] reads them;
/// `INGR` its `DATA` f32 at 0, `LIGH` its `DATA` f32 at 28, `CONT` its
/// `DATA` f32 at 1; ammunition only in hardcore mode (`DAT2` f32 at 8);
/// anything else (and ammunition outside hardcore) -1, which the menus
/// count as weighing nothing.
pub fn weight(order: &LoadOrder, item: FormId, hardcore: bool) -> f32 {
    const NONE: f32 = -1.0;
    let Some(rr) = order.get(item) else {
        return NONE;
    };
    let Ok(record) = rr.record_shared() else {
        return NONE;
    };
    let data = record
        .get(esm::sig::DATA)
        .map(|s| s.data.as_slice())
        .unwrap_or(&[]);
    let f32_at = |d: &[u8], at: usize| (d.len() >= at + 4).then(|| le_f32(d, at));
    let w = match rr.entry.header.kind.as_bytes() {
        b"ARMO" | b"WEAP" => f32_at(data, 8),
        b"BOOK" => f32_at(data, 6),
        b"MISC" | b"KEYM" | b"IMOD" | b"COBJ" => f32_at(data, 4),
        b"ALCH" | b"INGR" => f32_at(data, 0),
        b"LIGH" => f32_at(data, 28),
        b"CONT" => f32_at(data, 1),
        b"AMMO" if hardcore => {
            let dat2 = record.get(DAT2).map(|s| s.data.as_slice()).unwrap_or(&[]);
            f32_at(dat2, 8)
        }
        _ => None,
    };
    w.unwrap_or(NONE)
}

/// What someone carries and can carry, as trading with a companion asks
/// it: the most is their Carry Weight (actor value 13) through the
/// holder's perks' "Get Max Carry Weight" (entry point 22, the player's
/// only; `008a0c20`); what they carry is their things' weight, for anyone
/// but the player at most that (`00577250`).
pub fn carry(order: &LoadOrder, state: &GameState, who: FormId) -> (f32, f32) {
    let value = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, 13)
    .unwrap_or(0.0) as f32;
    let most = crate::perks::apply_for(
        order,
        state,
        who,
        crate::perks::entry::GET_MAX_CARRY_WEIGHT,
        value,
        &[],
    );
    let mut carried = state.inventory_weight(order, who);
    if who != crate::dialogue::PLAYER_REF {
        carried = carried.min(most);
    }
    (carried, most)
}

/// Whether a companion has room for `count` more of an item (`0075dc80`
/// in mode 3): what they carry plus the item's weight × `count`, at most
/// the most they can carry.
pub fn has_room(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    item: FormId,
    count: i32,
) -> bool {
    let (carried, most) = carry(order, state, who);
    let each = weight(order, item, false).max(0.0);
    each * count as f32 + carried <= most
}

/// One thing someone holds, as the game's item menus read it (the
/// container menu's filling `0075c280`, `00719ef0`, `0075cfc0`, its filter
/// `0075e650` and its labels `0075d160`).
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryLine {
    pub item: FormId,
    /// Its `FULL` name ("" without one: the menus leave it out).
    pub name: String,
    pub count: i32,
    /// The game's form type number (`script_functions::form_type`).
    pub form_type: u8,
    /// Worn or held by its holder.
    pub equipped: bool,
    /// Form flag 0x400 (or a script's `SetQuestObject`).
    pub quest_item: bool,
    /// Not marked not playable: `ARMO` `BMDT` general flags (byte 4)
    /// 0x40, `WEAP` `DNAM` flags (byte 12) 0x80, `AMMO` `DATA` flags (byte
    /// 4) 0x02.
    pub playable: bool,
    /// Ammunition in the `RegeneratingAmmo` form list.
    pub regenerating_ammo: bool,
    /// Caps (`0000000F`, `00481f10`) or weighing nothing ([`weight`] = 0).
    pub weightless: bool,
    /// Its inventory picture (`ICON`; armour's male one, `004be200`
    /// without an owner on the item).
    pub icon: String,
}

/// Caps' form ID.
pub const CAPS: FormId = FormId(0xF);

/// A holder's things as [`InventoryLine`]s, in the state's order.
pub fn inventory_lines(order: &LoadOrder, state: &GameState, holder: FormId) -> Vec<InventoryLine> {
    let regenerating = order
        .form_by_editor_id("RegeneratingAmmo")
        .map(|list| crate::script_functions::form_list(order, state, list))
        .unwrap_or_default();
    let mut out = Vec::new();
    for (item, count) in state.inventory(order, holder) {
        if count <= 0 {
            continue;
        }
        let Some(rr) = order.get(item) else {
            continue;
        };
        let kind = rr.entry.header.kind;
        let Some(form_type) = crate::script_functions::form_type(kind) else {
            continue;
        };
        let Ok(record) = rr.record() else {
            continue;
        };
        let byte = |sig: &[u8; 4], at: usize| {
            record
                .get(FourCC::new(sig))
                .and_then(|s| s.data.get(at).copied())
                .unwrap_or(0)
        };
        let playable = match kind.as_bytes() {
            b"ARMO" => byte(b"BMDT", 4) & 0x40 == 0,
            b"WEAP" => byte(b"DNAM", 12) & 0x80 == 0,
            b"AMMO" => byte(b"DATA", 4) & 0x02 == 0,
            _ => true,
        };
        out.push(InventoryLine {
            item,
            name: record.full_name().unwrap_or_default(),
            count,
            form_type,
            equipped: state.is_equipped(holder, item),
            quest_item: crate::script_functions::is_quest_item(order, state, item),
            playable,
            regenerating_ammo: kind.as_bytes() == b"AMMO" && regenerating.contains(&item),
            weightless: item == CAPS || weight(order, item, false) <= 0.0,
            icon: record
                .get(FourCC::new(b"ICON"))
                .map(|s| s.zstring())
                .unwrap_or_default(),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    /// A Data folder with a few items: a gun, a gun marked not playable,
    /// two kinds of rounds (one in `RegeneratingAmmo`), a hat marked not
    /// playable, a weightless pebble, a quest item, something unnamed and
    /// caps.
    fn items_plugin(tag: &str) -> (std::path::PathBuf, LoadOrder) {
        let dir = std::env::temp_dir().join(format!("nv-rs-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let named = |kind: &[u8; 4], id: u32, edid: &str, name: Option<&str>, rest: &[u8]| {
            let mut d = sub(b"EDID", &zstr(edid));
            if let Some(n) = name {
                d.extend(sub(b"FULL", &zstr(n)));
            }
            d.extend(rest);
            record(kind, id, &d)
        };
        let weapon = |id: u32, name: &str, flags: u8| {
            let mut data = 25i32.to_le_bytes().to_vec();
            data.extend(100i32.to_le_bytes());
            data.extend(3.5f32.to_le_bytes());
            data.extend(10i16.to_le_bytes());
            data.push(12);
            let mut dnam = vec![0u8; 204];
            dnam[12] = flags;
            let mut rest = sub(b"ICON", &zstr("interface\\icons\\gun.dds"));
            rest.extend(sub(b"DATA", &data));
            rest.extend(sub(b"DNAM", &dnam));
            named(b"WEAP", id, name, Some(name), &rest)
        };
        let ammo = |id: u32, name: &str, flags: u8| {
            let mut data = 1.0f32.to_le_bytes().to_vec();
            data.extend([flags, 0, 0, 0]);
            data.extend(1i32.to_le_bytes());
            data.push(1);
            let mut dat2 = 1u32.to_le_bytes().to_vec();
            dat2.extend(0u32.to_le_bytes());
            dat2.extend(0.05f32.to_le_bytes());
            let mut rest = sub(b"DATA", &data);
            rest.extend(sub(b"DAT2", &dat2));
            named(b"AMMO", id, name, Some(name), &rest)
        };
        let misc = |id: u32, edid: &str, name: Option<&str>, weight: f32| {
            let mut data = 1i32.to_le_bytes().to_vec();
            data.extend(weight.to_le_bytes());
            named(b"MISC", id, edid, name, &sub(b"DATA", &data))
        };
        let mut plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        let mut weapons = weapon(0x800, "Gun", 0);
        weapons.extend(weapon(0x801, "PropGun", 0x80));
        plugin.extend(group(*b"WEAP", 0, &weapons));
        let mut rounds = ammo(0x810, "Rounds", 0);
        rounds.extend(ammo(0x811, "Cell", 0));
        plugin.extend(group(*b"AMMO", 0, &rounds));
        let mut bmdt = 0u32.to_le_bytes().to_vec();
        bmdt.extend([0x40, 0, 0, 0]);
        let mut hat = sub(b"BMDT", &bmdt);
        hat.extend(sub(b"DATA", &[0u8; 12]));
        plugin.extend(group(
            *b"ARMO",
            0,
            &named(b"ARMO", 0x820, "Hat", Some("Hat"), &hat),
        ));
        let mut miscs = misc(0x830, "Pebble", Some("Pebble"), 0.0);
        let mut quest = misc(0x831, "Locket", Some("Locket"), 0.5);
        quest[8..12].copy_from_slice(&0x400u32.to_le_bytes());
        miscs.extend(quest);
        miscs.extend(misc(0x832, "Unnamed", None, 1.0));
        miscs.extend(misc(0xF, "Caps001", Some("Bottle Cap"), 0.0));
        plugin.extend(group(*b"MISC", 0, &miscs));
        let list = named(
            b"FLST",
            0x840,
            "RegeneratingAmmo",
            None,
            &sub(b"LNAM", &0x811u32.to_le_bytes()),
        );
        plugin.extend(group(*b"FLST", 0, &list));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        (dir, order)
    }

    /// `0048ebc0`: ammunition weighs something only in hardcore mode;
    /// what has no weight is -1.
    #[test]
    fn weights_as_the_game_counts_them() {
        let (dir, order) = items_plugin("items-weight");
        assert_eq!(weight(&order, FormId(0x800), false), 3.5);
        assert_eq!(weight(&order, FormId(0x810), false), -1.0);
        assert_eq!(weight(&order, FormId(0x810), true), 0.05);
        assert_eq!(weight(&order, FormId(0x830), false), 0.0);
        assert_eq!(weight(&order, FormId(0x840), false), -1.0);
        // The inventory's weight leaves the rounds out.
        let holder = FormId(0x14);
        let mut state = GameState::default();
        state.stocked.insert(holder);
        state.items.insert((holder, FormId(0x800)), 2);
        state.items.insert((holder, FormId(0x810)), 100);
        assert_eq!(state.inventory_weight(&order, holder), 7.0);
        let _ = std::fs::remove_dir_all(dir);
    }

    /// What the item menus read of each thing held.
    #[test]
    fn inventory_lines_for_the_menus() {
        let (dir, order) = items_plugin("items-lines");
        let holder = FormId(0x14);
        let mut state = GameState::default();
        state.stocked.insert(holder);
        for (id, n) in [
            (0x800, 1),
            (0x801, 1),
            (0x810, 30),
            (0x811, 5),
            (0x820, 1),
            (0x830, 3),
            (0x831, 1),
            (0x832, 1),
            (0xF, 40),
        ] {
            state.items.insert((holder, FormId(id)), n);
        }
        state.equipped.insert(holder, vec![FormId(0x800)]);
        let lines = inventory_lines(&order, &state, holder);
        let line = |id: u32| lines.iter().find(|l| l.item == FormId(id)).unwrap();
        assert_eq!(line(0x800).name, "Gun");
        assert_eq!(line(0x800).form_type, 0x28);
        assert!(line(0x800).equipped && line(0x800).playable);
        assert_eq!(line(0x800).icon, "interface\\icons\\gun.dds");
        assert!(!line(0x801).playable);
        assert!(!line(0x810).regenerating_ammo && line(0x811).regenerating_ammo);
        // Rounds weigh nothing outside hardcore.
        assert!(line(0x810).weightless);
        assert!(!line(0x820).playable);
        assert!(line(0x830).weightless && !line(0x830).quest_item);
        assert!(line(0x831).quest_item && !line(0x831).weightless);
        assert_eq!(line(0x832).name, "");
        assert!(line(0xF).weightless);
        assert_eq!(line(0xF).count, 40);
        let _ = std::fs::remove_dir_all(dir);
    }
}
