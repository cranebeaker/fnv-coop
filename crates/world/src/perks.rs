//! Perks (`PERK`): what they are (`DATA`: trait, the level they open at,
//! ranks, playable, hidden) and their entries, which change numbers the
//! game works out elsewhere or give abilities.
//!
//! An entry is `PRKE` (kind: 0 quest stage, 1 ability, 2 entry point; the
//! rank it belongs to, counting from 0; priority), then for an entry point
//! `DATA` (the entry point's number, its function, how many condition
//! tabs), conditions grouped by tab (`PRKC` tab number, then its `CTDA`s),
//! `EPFT` (1 = a float follows) and `EPFD` (the value); `PRKF` ends it. For
//! an ability, `DATA` is the spell (`SPEL`) the holder has. Read from the
//! game's records: Educated has entry point 10 ("Adjust Gained Skill
//! Points") with function 2 (add) 2.0; Swift Learner 9 ("Adjust Experience
//! Points") with function 3 (multiply) 1.1, 1.2, 1.3 for its three ranks;
//! Toughness 56 ("Modify Damage Threshold (defender)") + 3, + 6. The
//! numbers and names are the game's own table ([`ENTRY_POINT_NAMES`],
//! built at `00f4fa60`).
//!
//! **How the game applies an entry point** (`005e58f0(entry, owner,
//! params…, &value)`, read from the code; `%USERPROFILE%\nv-re\findings\
//! perks.md`): the owner's perk entries for that entry point are taken from
//! the player's lists (player `+0x884`, one per entry point, `00963ba0`);
//! for anyone else the lists are empty (`Character`'s `+0x4ac` is
//! `0050fbe0`, 0), except a teammate, who gets the player's entries from
//! perks given with `AddPerk`'s teammate flag (lists at `+0xadc`; no
//! script in the game's data sets the flag, so that path isn't here). Each
//! entry's conditions are checked tab by tab ("CheckConditionFilters",
//! `005ea060`): tab 0 against the owner, the others against the entry
//! point's parameters in order ([`tab_kinds`], the table at `01196e44`:
//! the weapon, the target, the attacker, …); an entry whose tab count
//! (`DATA` byte 2) isn't the parameter count, or a call with a null
//! parameter, applies nothing. Passing entries run their function on the
//! value in list order (`01197388`): 1 set, 2 add, 3 multiply, 4 add a
//! random value between two, 5 add an actor value × a factor, 6 absolute
//! value, 7 negative absolute value, 8 add a leveled list, 9 add an
//! activate choice. The game's perks use 1–3 and 9 ([`apply_for`] does
//! 1–3; 9 is the activation prompt, not drawn).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::functions::GET_IS_SEX;
use crate::dialogue::{read_condition, Condition, PLAYER_REF};
use crate::scripting::{Facts, GameState};

const PERK: FourCC = FourCC::new(b"PERK");
const DESC: FourCC = FourCC::new(b"DESC");

/// The game's entry points by number (its table at `01196ee0`, filled at
/// `00f4fa60`).
pub const ENTRY_POINT_NAMES: [&str; 74] = [
    "Calculate Weapon Damage",
    "Calculate My Critical Hit Chance",
    "Calculate My Critical Hit Damage",
    "Calculate Weapon Attack AP Cost",
    "Calculate Mine Explode Chance",
    "Adjust Range Penalty",
    "Adjust Limb Damage",
    "Calculate Weapon Range",
    "Calculate To Hit Chance",
    "Adjust Experience Points",
    "Adjust Gained Skill Points",
    "Adjust Book Skill Points",
    "Modify Recovered Health",
    "Calculate Inventory AP Cost",
    "Get Disposition",
    "Get Should Attack",
    "Get Should Assist",
    "Calculate Buy Price",
    "Get Bad Karma",
    "Get Good Karma",
    "Ignore Locked Terminal",
    "Add Leveled List On Death",
    "Get Max Carry Weight",
    "Modify Addiction Chance",
    "Modify Addiction Duration",
    "Modify Positive Chem Duration",
    "Adjust Drinking Radiation",
    "Activate",
    "Mysterious Stranger",
    "Has Paralyzing Palm",
    "Hacking Science Bonus",
    "Ignore Running During Detection",
    "Ignore Broken Lock",
    "Has Concentrated Fire",
    "Calculate Gun Spread",
    "Player Kill AP Reward",
    "Modify Enemy Critical Hit Chance",
    "Reload Speed",
    "Equip Speed",
    "Action Point Regen",
    "Action Point Cost",
    "Miss Fortune",
    "Modify Run Speed",
    "Modify Attack Speed",
    "Modify Radiation Consumed",
    "Has Pip Hacker",
    "Has Meltdown",
    "See Enemy Health",
    "Has Jury Rigging",
    "Modify Threat Range",
    "Modify Threat",
    "Has Fast Travel Always",
    "Knockdown Chance",
    "Modify Weapon Strength Req",
    "Modify Aiming Move Speed",
    "Modify Light Items",
    "Modify Damage Threshold (defender)",
    "Modify Chance for Ammo Item",
    "Modify Damage Threshold (attacker)",
    "Modify Throwing Velocity",
    "Chance for Item On Fire",
    "Has Unarmed Forward Power Attack",
    "Has Unarmed Back Power Attack",
    "Has Unarmed Crouched Power Attack",
    "Has Unarmed Counter Attack",
    "Has Unarmed Left Power Attack",
    "Has Unarmed Right Power Attack",
    "VATS HelperChance",
    "Modify Item Damage",
    "Has Improved Detection",
    "Has Improved Spotting",
    "Has Improved Item Detection",
    "Adjust Explosion Radius",
    "Adjust Heavy Weapon Weight",
];

/// Entry point numbers used here.
pub mod entry {
    /// A hit's damage after armour (`009b5a30`: attacker, weapon, target).
    pub const CALCULATE_WEAPON_DAMAGE: u8 = 0;
    /// The attacker's critical chance (`009b7060`: attacker, weapon,
    /// target).
    pub const CALCULATE_MY_CRITICAL_HIT_CHANCE: u8 = 1;
    /// The weapon's critical damage on a critical (`009b7060`).
    pub const CALCULATE_MY_CRITICAL_HIT_DAMAGE: u8 = 2;
    /// A mine's chance (from 100) of going off for the player near it
    /// (`009c39e0`: the player, the mine's projectile; Light Step sets 0).
    pub const CALCULATE_MINE_EXPLODE_CHANCE: u8 = 4;
    /// On the one hit: their limb damage (`0089a760`: the one hit, the
    /// attacker, the attacker's weapon).
    pub const ADJUST_LIMB_DAMAGE: u8 = 6;
    /// V.A.T.S.'s chance to hit (`007f1290`: owner, weapon, target).
    pub const CALCULATE_TO_HIT_CHANCE: u8 = 8;
    pub const ADJUST_EXPERIENCE_POINTS: u8 = 9;
    pub const ADJUST_GAINED_SKILL_POINTS: u8 = 10;
    pub const ADJUST_BOOK_SKILL_POINTS: u8 = 11;
    /// The player's regeneration (`0088b510`, from 0) and the health a
    /// medicine or food restores (`00815d00`).
    pub const MODIFY_RECOVERED_HEALTH: u8 = 12;
    pub const CALCULATE_BUY_PRICE: u8 = 17;
    pub const GET_MAX_CARRY_WEIGHT: u8 = 22;
    /// An addiction's roll (`00824e70`; the roll isn't here).
    pub const MODIFY_ADDICTION_CHANCE: u8 = 23;
    /// An addiction spell's effects' durations (`00823210`).
    pub const MODIFY_ADDICTION_DURATION: u8 = 24;
    /// A food's or chem's effects' durations, the hostile ones aside
    /// (`00823210`).
    pub const MODIFY_POSITIVE_CHEM_DURATION: u8 = 25;
    /// V.A.T.S. (`007e9200`, `007f1290`, `009c7240`; see `world::vats`).
    pub const MYSTERIOUS_STRANGER: u8 = 28;
    pub const HAS_PARALYZING_PALM: u8 = 29;
    /// Detection (`008a0d10`): the one noticed isn't running, nor moving
    /// while sneaking.
    pub const IGNORE_RUNNING_DURING_DETECTION: u8 = 31;
    pub const IGNORE_BROKEN_LOCK: u8 = 32;
    pub const HAS_CONCENTRATED_FIRE: u8 = 33;
    pub const CALCULATE_GUN_SPREAD: u8 = 34;
    pub const PLAYER_KILL_AP_REWARD: u8 = 35;
    /// On the one hit: the attacker's critical chance (`009b7060`: the
    /// one hit, the weapon, the attacker).
    pub const MODIFY_ENEMY_CRITICAL_HIT_CHANCE: u8 = 36;
    /// The reload animations' rate (`008c17c0`: owner, weapon).
    pub const RELOAD_SPEED: u8 = 37;
    /// The equip animations' rate (`008c1940`: owner, weapon).
    pub const EQUIP_SPEED: u8 = 38;
    pub const ACTION_POINT_REGEN: u8 = 39;
    pub const ACTION_POINT_COST: u8 = 40;
    pub const MISS_FORTUNE: u8 = 41;
    /// Movement speed, walking and running alike (`00885bf0`).
    pub const MODIFY_RUN_SPEED: u8 = 42;
    /// The attack animations' rate (`00893a40`, `00895110`: owner, weapon).
    pub const MODIFY_ATTACK_SPEED: u8 = 43;
    /// A food's radiation (`00815d00`).
    pub const MODIFY_RADIATION_CONSUMED: u8 = 44;
    /// Fast travel while over-encumbered (`0093cdf0`, `0093d660`).
    pub const HAS_FAST_TRAVEL_ALWAYS: u8 = 51;
    /// The chance (0..1) a hit knocks the one hit down (`0089a760`:
    /// Super Slam, `world::melee::knockdown_chance`).
    pub const KNOCKDOWN_CHANCE: u8 = 52;
    pub const MODIFY_WEAPON_STRENGTH_REQ: u8 = 53;
    pub const MODIFY_AIMING_MOVE_SPEED: u8 = 54;
    /// Light items' weight (`004d0900`).
    pub const MODIFY_LIGHT_ITEMS: u8 = 55;
    pub const MODIFY_DAMAGE_THRESHOLD_DEFENDER: u8 = 56;
    /// The chance of getting an ammunition's case or cell back on firing
    /// (`00523150`: owner, weapon).
    pub const MODIFY_CHANCE_FOR_AMMO_ITEM: u8 = 57;
    /// The attacker's perks against the target's damage threshold
    /// (`009b5a30`: attacker, weapon, target).
    pub const MODIFY_DAMAGE_THRESHOLD_ATTACKER: u8 = 58;
    /// A throw's speed (`009bca60`: thrower, weapon).
    pub const MODIFY_THROWING_VELOCITY: u8 = 59;
    /// What an item loses when damaged (`00891360`: owner).
    pub const MODIFY_ITEM_DAMAGE: u8 = 68;
    pub const HAS_IMPROVED_DETECTION: u8 = 69;
    /// The player's explosions' radius (`009ac9c0`: the player, the
    /// weapon).
    pub const ADJUST_EXPLOSION_RADIUS: u8 = 72;
    /// Heavy weapons' weight (`004d0900`: weapons of 10 or more).
    pub const ADJUST_HEAVY_WEAPON_WEIGHT: u8 = 73;
}

/// The default unarmed weapon the game hands an entry point when the
/// actor holds none (`[011ca278]`; `009b5a30` makes it by form ID 500:
/// `Fists`, `000001F4`).
pub const FISTS: FormId = FormId(0x1F4);

/// What an entry point's parameters after the first (the perk's owner)
/// are: its condition tabs 1, 2, … are asked about these (the game's
/// table at `01196e44`, named "Perk Owner", "Weapon", "Target",
/// "Attacker", "Attacker Weapon", "Attackee", "Item", "Mine").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabKind {
    /// A weapon record (the fists when none is held).
    Weapon,
    /// A person or creature: the target, the attacker, the one attacked.
    Actor,
    /// An item record (a mine for entry point 4).
    Item,
}

/// The parameters each entry point takes after its owner (`00f4fa60`:
/// each entry's descriptor names a parameter list).
pub fn tab_kinds(entry: u8) -> &'static [TabKind] {
    use TabKind::*;
    match entry {
        // Calculate Weapon Damage, the critical chance and damage, To Hit
        // Chance, Player Kill AP Reward, Modify Enemy Critical Hit Chance,
        // Modify Damage Threshold (attacker): owner, weapon, target.
        0 | 1 | 2 | 8 | 35 | 36 | 58 => &[Weapon, Actor],
        // Owner and weapon: Weapon Attack AP Cost, Range Penalty, Weapon
        // Range, Gun Spread, Reload and Equip Speed, AP Regen, AP Cost,
        // Attack Speed, Threat Range and Threat, Knockdown Chance, Weapon
        // Strength Req, Aiming Move Speed, Chance for Ammo Item, Throwing
        // Velocity, Item On Fire, Explosion Radius.
        3 | 5 | 7 | 34 | 37 | 38 | 39 | 40 | 43 | 49 | 50 | 52 | 53 | 54 | 57 | 59 | 60 | 72 => {
            &[Weapon]
        }
        // Mine Explode Chance (the mine), Calculate Buy Price (the item).
        4 | 17 => &[Item],
        // Adjust Limb Damage, Modify Damage Threshold (defender): the one
        // hit owns the perk; the attacker and the attacker's weapon.
        6 | 56 => &[Actor, Weapon],
        // Get Disposition, Add Leveled List On Death, Activate: the target.
        14 | 21 | 27 => &[Actor],
        // Get Should Attack: the attacker.
        15 => &[Actor],
        // Get Should Assist: the attacker and the one attacked.
        16 => &[Actor, Actor],
        _ => &[],
    }
}

/// A parameter an entry point is applied with: what its later condition
/// tabs are asked about (tab 1 is the first, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// A weapon's record: conditions are asked about it
    /// ([`weapon_condition_value`]); [`FISTS`] when none is held.
    Weapon(FormId),
    /// A person or creature: conditions are asked about them.
    Target(FormId),
    /// An item's record, asked about as a weapon's is.
    Item(FormId),
}

/// A perk record's `DATA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerkData {
    pub is_trait: bool,
    pub min_level: u8,
    pub ranks: u8,
    pub playable: bool,
    pub hidden: bool,
}

/// One entry point of a perk.
#[derive(Debug, Clone, PartialEq)]
pub struct EntryPoint {
    pub rank: u8,
    pub entry: u8,
    pub function: u8,
    /// How many condition tabs it has (`DATA` byte 2): the owner's and one
    /// per parameter of the entry point, or the game doesn't apply it.
    pub tabs: u8,
    pub value: Option<f32>,
    /// Its conditions, each with the tab it's on (0 the perk's holder).
    pub conditions: Vec<(u8, Condition)>,
}

/// A perk's `DATA` (the first one: entries have `DATA`s of their own).
pub fn perk_data(order: &LoadOrder, perk: FormId) -> Option<PerkData> {
    let rr = order.get(perk).filter(|r| r.entry.header.kind == PERK)?;
    let record = rr.record().ok()?;
    let d = &record
        .subrecords
        .iter()
        .find(|s| s.kind == esm::sig::DATA && s.data.len() >= 4)?
        .data;
    Some(PerkData {
        is_trait: d[0] != 0,
        min_level: d[1],
        ranks: d[2],
        playable: d[3] != 0,
        hidden: d.get(4).is_some_and(|&h| h != 0),
    })
}

/// A perk's entry points.
pub fn entry_points(order: &LoadOrder, perk: FormId) -> Vec<EntryPoint> {
    let Some(rr) = order.get(perk).filter(|r| r.entry.header.kind == PERK) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    let mut out: Vec<EntryPoint> = Vec::new();
    let mut current: Option<EntryPoint> = None;
    let mut tab = 0;
    for sub in &record.subrecords {
        match sub.kind.as_bytes() {
            b"PRKE" if sub.data.len() >= 2 => {
                out.extend(current.take());
                // Only entry points (kind 2) are kept here.
                if sub.data[0] == 2 {
                    current = Some(EntryPoint {
                        rank: sub.data[1],
                        entry: 0,
                        function: 0,
                        tabs: 0,
                        value: None,
                        conditions: Vec::new(),
                    });
                }
            }
            b"DATA" if sub.data.len() == 3 => {
                if let Some(e) = current.as_mut() {
                    e.entry = sub.data[0];
                    e.function = sub.data[1];
                    e.tabs = sub.data[2];
                }
            }
            b"PRKC" if !sub.data.is_empty() => tab = sub.data[0],
            b"CTDA" => {
                if let Some(e) = current.as_mut() {
                    e.conditions
                        .extend(read_condition(&rr, &sub.data).map(|c| (tab, c)));
                }
            }
            b"EPFD" if sub.data.len() >= 4 => {
                if let Some(e) = current.as_mut() {
                    e.value = Some(le_f32(&sub.data, 0));
                }
            }
            b"PRKF" => out.extend(current.take()),
            _ => {}
        }
    }
    out.extend(current.take());
    out
}

/// The spells a perk gives its holder at a rank (`PRKE` kind 1, abilities;
/// rank counting from 1).
pub fn abilities(order: &LoadOrder, perk: FormId, rank: u8) -> Vec<FormId> {
    let Some(rr) = order.get(perk).filter(|r| r.entry.header.kind == PERK) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut in_ability = false;
    for sub in &record.subrecords {
        match sub.kind.as_bytes() {
            b"PRKE" if sub.data.len() >= 2 => {
                in_ability = sub.data[0] == 1 && sub.data[1] + 1 == rank;
            }
            b"DATA" if in_ability && sub.data.len() >= 4 => {
                out.push(rr.plugin.to_global(FormId(le_u32(&sub.data, 0))));
                in_ability = false;
            }
            b"PRKF" => in_ability = false,
            _ => {}
        }
    }
    out
}

/// A value after the player's perks' entry points of kind `entry`, for an
/// entry point with no parameters beyond its owner: set (function 1), add
/// (2) or multiply (3), in perk order; each perk by the entries of the
/// rank held (a perk lists its entries once per rank: Toughness + 3 at
/// rank 1, + 6 at rank 2; Swift Learner × 1.1, 1.2, 1.3), and only
/// entries whose holder conditions pass (see the module notes).
pub fn apply(order: &LoadOrder, state: &GameState, entry: u8, value: f32) -> f32 {
    apply_for(order, state, PLAYER_REF, entry, value, &[])
}

/// [`apply`] for the player with what the entry point's later tabs are
/// about (tab 1 is `tabs[0]`, …).
pub fn apply_with(
    order: &LoadOrder,
    state: &GameState,
    entry: u8,
    value: f32,
    tabs: &[Tab],
) -> f32 {
    apply_for(order, state, PLAYER_REF, entry, value, tabs)
}

/// A value after `owner`'s perks' entry points of kind `entry`, applied
/// with the entry point's parameters `tabs` (as [`tab_kinds`] lists
/// them; tab 1 is `tabs[0]`, …), as the game's `005e58f0` does: only the
/// player holds perks (anyone else gets the value unchanged), and an entry
/// applies when its tab count is the entry point's parameters' plus the
/// owner's and every tab's conditions pass: tab 0 about the owner, tab n
/// about `tabs[n − 1]`. The game refuses a call short of parameters
/// outright; here a caller that doesn't pass them yet (V.A.T.S.'s 35 and
/// 39) gets the entries whose conditions keep to the tabs it gave.
pub fn apply_for(
    order: &LoadOrder,
    state: &GameState,
    owner: FormId,
    entry: u8,
    value: f32,
    tabs: &[Tab],
) -> f32 {
    let kinds = tab_kinds(entry);
    if owner != PLAYER_REF || tabs.len() > kinds.len() {
        return value;
    }
    let mut perks: Vec<FormId> = state.perks.iter().copied().collect();
    perks.sort_by_key(|p| p.0);
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let mut v = value;
    for perk in perks {
        let held = rank(state, perk);
        for e in entry_points(order, perk) {
            if e.entry != entry || e.rank + 1 != held {
                continue;
            }
            if usize::from(e.tabs) != kinds.len() + 1 {
                continue;
            }
            let passes = (0..e.tabs).all(|tab| {
                let on_tab: Vec<Condition> = e
                    .conditions
                    .iter()
                    .filter(|(t, _)| *t == tab)
                    .map(|(_, c)| c.clone())
                    .collect();
                if on_tab.is_empty() {
                    return true;
                }
                match tab.checked_sub(1).map(|i| tabs.get(usize::from(i))) {
                    None => facts.conditions_pass(&on_tab, owner, owner),
                    Some(None) => false,
                    Some(Some(Tab::Target(who))) => facts.conditions_pass(&on_tab, *who, owner),
                    Some(Some(Tab::Weapon(w))) | Some(Some(Tab::Item(w))) => {
                        joined(state, &on_tab, |c| {
                            weapon_condition_value(order, c, Some(*w)) as f32
                        })
                    }
                }
            });
            if !passes {
                continue;
            }
            let Some(x) = e.value else { continue };
            match e.function {
                1 => v = x,
                2 => v += x,
                3 => v *= x,
                _ => {}
            }
        }
    }
    v
}

/// The perk tab for an attack with `weapon` (or the default unarmed one).
pub fn weapon_tab(weapon: Option<FormId>) -> Tab {
    Tab::Weapon(weapon.unwrap_or(FISTS))
}

/// Conditions joined as `Facts::conditions_pass` joins them (left to
/// right, OR binding before AND), each function's value from `value_of`.
fn joined(
    state: &GameState,
    conditions: &[Condition],
    value_of: impl Fn(&Condition) -> f32,
) -> bool {
    let mut all = true;
    let mut group: Option<bool> = None;
    for c in conditions {
        let wanted = match c.global {
            Some(g) => state.globals.get(&g).copied().unwrap_or(0.0),
            None => c.value,
        };
        let ok = c.compare(value_of(c), wanted);
        let so_far = group.map_or(ok, |g| g || ok);
        if c.or {
            group = Some(so_far);
        } else {
            all &= so_far;
            group = None;
        }
    }
    if let Some(g) = group {
        all &= g;
    }
    all
}

/// A condition asked about a weapon's record (a perk's weapon tab; `None`:
/// no weapon): `IsWeaponSkillType` is the weapon's skill (`DNAM` i32 at
/// 104) — without a weapon, true only for Unarmed (45) (`005a0ab0`);
/// `GetIsID` the record itself; `IsInList` the record in the form list.
/// Other functions give 0 (a rule of this reimplementation).
pub fn weapon_condition_value(order: &LoadOrder, c: &Condition, weapon: Option<FormId>) -> f64 {
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    match c.function_name().as_str() {
        "IsWeaponSkillType" => {
            let skill = c.params[0] as i32;
            match weapon.and_then(|w| crate::combat::Weapon::load(order, w)) {
                Some(w) => flag(i32::from(w.skill) == skill),
                None => flag(skill == 45),
            }
        }
        "GetIsID" => flag(weapon == Some(c.param_forms[0])),
        "IsInList" => flag(weapon.is_some_and(|w| form_list(order, c.param_forms[0]).contains(&w))),
        _ => 0.0,
    }
}

/// A form list's entries (`FLST` `LNAM`s), in load-order numbering.
pub fn form_list(order: &LoadOrder, list: FormId) -> Vec<FormId> {
    let Some(rr) = order
        .get(list)
        .filter(|r| r.entry.header.kind.as_bytes() == b"FLST")
    else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(FourCC::new(b"LNAM"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

/// The ranks of a perk the player holds.
pub fn rank(state: &GameState, perk: FormId) -> u8 {
    if !state.perks.contains(&perk) {
        return 0;
    }
    state.perk_ranks.get(&perk).copied().unwrap_or(1).max(1)
}

/// Gives the player a perk, or its next rank, with that rank's abilities
/// (the last rank's taken off).
pub fn add(order: &LoadOrder, state: &mut GameState, perk: FormId) {
    state.emit(crate::coop::Domain::Perk, perk);
    let was = rank(state, perk);
    let next = was + 1;
    state.perks.insert(perk);
    if next > 1 {
        state.perk_ranks.insert(perk, next);
    }
    for spell in abilities(order, perk, was) {
        crate::magic::remove(state, PLAYER_REF, spell);
    }
    for spell in abilities(order, perk, next) {
        crate::magic::add_spell(order, state, PLAYER_REF, spell, PLAYER_REF, true);
    }
}

/// Takes a perk away from the player, with its abilities.
pub fn remove(order: &LoadOrder, state: &mut GameState, perk: FormId) {
    state.emit(crate::coop::Domain::Perk, perk);
    for spell in abilities(order, perk, rank(state, perk)) {
        crate::magic::remove(state, PLAYER_REF, spell);
    }
    state.perks.remove(&perk);
    state.perk_ranks.remove(&perk);
}

/// A perk the level-up menu can offer.
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub perk: FormId,
    pub name: String,
    pub description: String,
    /// The rank it would be (1 for a new perk).
    pub rank: u8,
    pub ranks: u8,
    pub min_level: u8,
    /// Whether its requirements pass now (level, conditions).
    pub available: bool,
}

/// A perk's inventory picture (`ICON`), "" without one.
pub fn icon(order: &LoadOrder, perk: FormId) -> String {
    order
        .get(perk)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(FourCC::new(b"ICON")).map(|s| s.zstring()))
        .unwrap_or_default()
}

/// The text the level-up menu shows for a perk (`005ebac0`): "`Req`:
/// `Level` n" (just "`Req`: " below level 2), then its requirements, then
/// "\n`Ranks`: n\n\n" and its description. Requirements: each
/// `GetPermanentActorValue` condition but on Karma as "name value" (the
/// value one less for `>`, " < value" for `<`, " < value + 1" for `<=`;
/// left out when that comes to 0), ", " after it when another condition
/// follows (" `OR` " between OR'd ones), the first one after ", " when
/// there's a level; a `HasPerk` as "perk rank" (or the perk's name); "--"
/// when nothing was written. The game walks the conditions 0, 1, 3, 5, …:
/// its loop steps its counter twice, so the third, fifth … are never read.
/// `setting` gives the text settings (`sRequirementsText`, `sLevelAbbrev`,
/// `sOr`, `sRanksText`).
pub fn requirements_text(
    order: &LoadOrder,
    state: &GameState,
    perk: FormId,
    setting: &dyn Fn(&str) -> String,
) -> String {
    let Some(rr) = order.get(perk).filter(|r| r.entry.header.kind == PERK) else {
        return String::new();
    };
    let Ok(record) = rr.record() else {
        return String::new();
    };
    let data = perk_data(order, perk);
    let min_level = data.as_ref().map_or(0, |d| d.min_level);
    let ranks = data.as_ref().map_or(0, |d| d.ranks);
    let conditions: Vec<Condition> = record
        .subrecords
        .iter()
        .take_while(|s| s.kind.as_bytes() != b"PRKE")
        .filter(|s| s.kind.as_bytes() == b"CTDA")
        .filter_map(|s| read_condition(&rr, &s.data))
        .collect();
    let mut out = if min_level < 2 {
        format!("{}: ", setting("sRequirementsText"))
    } else {
        format!(
            "{}: {} {}",
            setting("sRequirementsText"),
            setting("sLevelAbbrev"),
            min_level
        )
    };
    let mut written = false;
    let mut i = 0usize;
    let mut current = conditions.first();
    while let Some(c) = current {
        let separator = |out: &mut String| {
            if i == 0 && min_level >= 2 {
                out.push_str(", ");
            }
        };
        match c.function {
            // GetPermanentActorValue.
            0x1EF => {
                let av = c.params[0] as u16;
                let name = crate::chargen::actor_value_name(order, av);
                let (prefix, adjust) = match c.comparison {
                    crate::dialogue::Comparison::Greater => (" ", -1),
                    crate::dialogue::Comparison::Less => (" < ", 0),
                    crate::dialogue::Comparison::LessOrEqual => (" < ", 1),
                    _ => (" ", 0),
                };
                let value = match c.global {
                    Some(g) => state.globals.get(&g).copied().unwrap_or(0.0),
                    None => c.value,
                } as i32;
                if av != 23 && !name.is_empty() && value + adjust != 0 {
                    written = true;
                    separator(&mut out);
                    let next = conditions.get(i + 1).is_some();
                    let v = value + adjust;
                    if c.or && next {
                        out.push_str(&format!(" {name}{prefix}{v} {} ", setting("sOr")));
                    } else if next {
                        out.push_str(&format!("{name}{prefix}{v}, "));
                    } else {
                        out.push_str(&format!("{name}{prefix}{v}"));
                    }
                }
            }
            // HasPerk.
            0x1C1 => {
                written = true;
                separator(&mut out);
                let other = c.param_forms[0];
                if other.0 != 0 {
                    let name = order
                        .get(other)
                        .and_then(|r| r.record().ok())
                        .and_then(|r| r.full_name())
                        .unwrap_or_default();
                    if c.params[1] != 0 {
                        out.push_str(&format!("{name} {}", c.params[1]));
                    } else {
                        out.push_str(&name);
                    }
                }
            }
            _ => {
                written = true;
                separator(&mut out);
            }
        }
        i += 1;
        current = conditions.get(i);
        i += 1;
    }
    if !written {
        out.push_str("--");
    }
    out.push('\n');
    out.push_str(&format!("{}: {ranks}\n\n", setting("sRanksText")));
    out.push_str(&record.get(DESC).map(|s| s.zstring()).unwrap_or_default());
    out
}

/// The perks the level-up menu lists (`00784c80`): playable ones that
/// aren't traits, opening at a level above 0, with ranks left, whose sex
/// conditions (`GetIsSex`) pass for the player; by name. The rest of
/// their requirements (the perk's other conditions, its level) decide
/// whether it can be picked (`available`); the game lists the others too
/// unless the INI's `bHideUnavailablePerks` is set (off by default).
pub fn level_up_choices(order: &LoadOrder, state: &GameState) -> Vec<Choice> {
    menu_choices(order, state, false)
}

/// The traits the trait menu lists (`007e6990`): the same rules as the
/// level-up menu's perks (`level_up_choices`) but for perks flagged as
/// traits. The menu doesn't look at the hidden flag; a trait whose
/// minimum level isn't above 0 (the byte read signed) isn't listed.
pub fn trait_choices(order: &LoadOrder, state: &GameState) -> Vec<Choice> {
    menu_choices(order, state, true)
}

fn menu_choices(order: &LoadOrder, state: &GameState, traits: bool) -> Vec<Choice> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let mut out: Vec<Choice> = order
        .records_of_type(PERK)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let data = perk_data(order, rr.form_id)?;
            let held = rank(state, rr.form_id);
            if data.is_trait != traits
                || (data.min_level as i8) <= 0
                || !data.playable
                || held >= data.ranks
            {
                return None;
            }
            let record = rr.record().ok()?;
            // The perk's own requirements: the conditions before its first
            // entry.
            let conditions: Vec<Condition> = record
                .subrecords
                .iter()
                .take_while(|s| s.kind.as_bytes() != b"PRKE")
                .filter(|s| s.kind.as_bytes() == b"CTDA")
                .filter_map(|s| read_condition(&rr, &s.data))
                .collect();
            let sex: Vec<Condition> = conditions
                .iter()
                .filter(|c| c.function == GET_IS_SEX)
                .cloned()
                .collect();
            if !facts.conditions_pass(&sex, PLAYER_REF, PLAYER_REF) {
                return None;
            }
            let available = u16::from(data.min_level) <= state.player_level
                && facts.conditions_pass(&conditions, PLAYER_REF, PLAYER_REF);
            Some(Choice {
                perk: rr.form_id,
                name: record.full_name().unwrap_or_else(|| rr.form_id.to_string()),
                description: record.get(DESC).map(|s| s.zstring()).unwrap_or_default(),
                rank: held + 1,
                ranks: data.ranks,
                min_level: data.min_level,
                available,
            })
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

#[cfg(test)]
mod requirement_tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    /// A condition `GetPermanentActorValue av <op> value` (op as the
    /// game's comparison number).
    fn av_condition(op: u8, av: u32, value: f32) -> Vec<u8> {
        let mut d = vec![op << 5, 0, 0, 0];
        d.extend(value.to_le_bytes());
        d.extend(0x1EFu16.to_le_bytes());
        d.extend([0, 0]);
        d.extend(av.to_le_bytes());
        d.extend(0u32.to_le_bytes());
        d.extend(0u32.to_le_bytes());
        d.extend(0u32.to_le_bytes());
        sub(b"CTDA", &d)
    }

    /// `005ebac0`: "Req: Level 6, Endurance 5, Perception 4" (`>` shows
    /// one less; the third condition is never read, the game's loop
    /// stepping twice), then the ranks and the description.
    #[test]
    fn a_perks_requirements_as_the_menu_writes_them() {
        let dir = std::env::temp_dir().join(format!("nv-rs-perk-req-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        let mut d = sub(b"EDID", &zstr("Tough"));
        d.extend(sub(b"FULL", &zstr("Tough")));
        d.extend(sub(b"DESC", &zstr("Harder.")));
        d.extend(sub(b"ICON", &zstr("Interface\\Icons\\tough.dds")));
        d.extend(av_condition(3, 7, 5.0));
        d.extend(av_condition(2, 6, 5.0));
        d.extend(av_condition(3, 5, 3.0));
        d.extend(sub(b"DATA", &[0, 6, 2, 1, 0]));
        plugin.extend(group(*b"PERK", 0, &record(b"PERK", 0x800, &d)));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        let state = GameState::default();
        let setting = |n: &str| {
            match n {
                "sRequirementsText" => "Req",
                "sLevelAbbrev" => "Level",
                "sOr" => "OR",
                "sRanksText" => "Ranks",
                _ => "",
            }
            .to_string()
        };
        assert_eq!(
            requirements_text(&order, &state, FormId(0x800), &setting),
            "Req: Level 6, Endurance 5, Perception 4\nRanks: 2\n\nHarder."
        );
        assert_eq!(icon(&order, FormId(0x800)), "Interface\\Icons\\tough.dds");
        let _ = std::fs::remove_dir_all(dir);
    }

    /// `007e6990`: the trait menu lists playable traits whose level byte
    /// is above 0, hidden or not; the level-up menu the other perks.
    #[test]
    fn the_trait_menu_lists_traits() {
        let dir = std::env::temp_dir().join(format!("nv-rs-trait-list-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        let perk = |id: u32, name: &str, data: [u8; 5]| {
            let mut d = sub(b"EDID", &zstr(name));
            d.extend(sub(b"FULL", &zstr(name)));
            d.extend(sub(b"DATA", &data));
            record(b"PERK", id, &d)
        };
        let mut perks = perk(0x800, "Kamikaze", [1, 1, 1, 1, 0]);
        perks.extend(perk(0x801, "Hidden Trait", [1, 1, 1, 1, 1]));
        perks.extend(perk(0x802, "Level Zero", [1, 0, 1, 1, 0]));
        perks.extend(perk(0x803, "Toughness", [0, 2, 2, 1, 0]));
        perks.extend(perk(0x804, "Unplayable", [1, 1, 1, 0, 0]));
        plugin.extend(group(*b"PERK", 0, &perks));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        let state = GameState::default();
        let names = |c: Vec<Choice>| c.into_iter().map(|c| c.name).collect::<Vec<_>>();
        assert_eq!(
            names(trait_choices(&order, &state)),
            ["Hidden Trait", "Kamikaze"]
        );
        assert_eq!(names(level_up_choices(&order, &state)), ["Toughness"]);
        let _ = std::fs::remove_dir_all(dir);
    }
}
