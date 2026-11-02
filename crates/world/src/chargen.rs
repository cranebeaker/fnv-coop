//! Making the character: the menus the opening's scripts open (`VCG01`:
//! the name, the face, the vigor tester's SPECIAL, the tag skills, the
//! traits) and what the player's choices set.
//!
//! - SPECIAL: `ShowLoveTesterMenuParams 40` shares 40 points among the
//!   seven, each 1 to 10 (the player starts at 5 each, from the player
//!   record's `DATA`).
//! - Tag skills: `SetTagSkills 3 1` asks for three; a tagged skill gets
//!   `fAVDTagSkillBonus` (15).
//! - Traits: `ShowTraitMenu` offers the playable perks flagged as traits
//!   (`PERK` `DATA`: trait, minimum level, ranks, playable, hidden; ten in
//!   the base game: Built to Destroy … Wild Wasteland), up to
//!   `iTraitMenuMaxNumTraits` (2).
//! - The name starts as `sDefaultPlayerName` ("Courier").
//!
//! The player's skills come from SPECIAL: the player record's own skill
//! numbers (`DNAM`: Melee 30, Guns 25 at SPECIAL 5) aren't what a new
//! character has. Each is `fAVDSkill<Name>Base` (2) + 2 × its attribute +
//! Luck / 2 rounded up, + 15 tagged: the base is the game's setting, the
//! rest the documented formula (not traced in the code), as is which
//! attribute governs which skill.

use esm::{FormId, FourCC, LoadOrder};

use crate::scripting::{game_setting, GameState};

pub mod appearance;
pub mod facegen;

const PERK: FourCC = FourCC::new(b"PERK");
const DESC: FourCC = FourCC::new(b"DESC");

/// A menu a script opened for making the character.
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterMenu {
    /// `ShowNameMenu`, or `GetPlayerName` (what `VCG01` uses).
    Name,
    /// `ShowLoveTesterMenuParams`: SPECIAL, sharing this many points.
    Special { points: u32 },
    /// `SetTagSkills`: pick this many tag skills, the player's own picked
    /// to start with when `preselect`.
    TagSkills { count: u32, preselect: bool },
    /// `ShowTraitMenu`: pick up to this many traits.
    Traits { max: u32 },
}

/// The seven SPECIAL attributes' actor values (Strength … Luck).
pub const SPECIAL: [u16; 7] = [5, 6, 7, 8, 9, 10, 11];

/// An actor value's name as the game shows it: its `AVIF` record's
/// (`AV` + the script name: `AVSmallGuns` is shown as "Guns",
/// `AVThrowing` as "Survival"), else the script name.
pub fn actor_value_name(order: &LoadOrder, av: u16) -> String {
    let script_name = script::ACTOR_VALUES
        .get(usize::from(av))
        .copied()
        .unwrap_or("?");
    order
        .form_by_editor_id(&format!("AV{script_name}"))
        .and_then(|id| order.get(id))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.full_name())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| script_name.to_string())
}

/// An actor value's picture (`ICON`) and description (`DESC`), from its
/// `AVIF` record as the level-up menu shows them (`0066e9f0`,
/// `0066e9b0`); empty without.
pub fn actor_value_icon_and_text(order: &LoadOrder, av: u16) -> (String, String) {
    let Some(script_name) = script::ACTOR_VALUES.get(usize::from(av)) else {
        return (String::new(), String::new());
    };
    let record = order
        .form_by_editor_id(&format!("AV{script_name}"))
        .and_then(|id| order.get(id))
        .and_then(|rr| rr.record().ok());
    let Some(record) = record else {
        return (String::new(), String::new());
    };
    let get = |sig: &[u8; 4]| {
        record
            .get(FourCC::new(sig))
            .map(|s| s.zstring())
            .unwrap_or_default()
    };
    (get(b"ICON"), get(b"DESC"))
}

/// Lowest and highest an attribute can be made.
pub const SPECIAL_RANGE: (u8, u8) = (1, 10);

/// The skills by actor value (32 … 45, the order of the records' `DNAM`),
/// the setting with their base, and the attribute that governs them. Big
/// Guns (33) is a skill New Vegas cut ("Big Guns - OBSOLETE"): its menus
/// don't show it ([`SHOWN_SKILLS`]).
pub const SKILLS: [(u16, &str, u16); 14] = [
    (32, "fAVDSkillBarterBase", 8),
    (33, "fAVDSkillBigGunsBase", 7),
    (34, "fAVDSkillEnergyWeaponsBase", 6),
    (35, "fAVDSkillExplosivesBase", 6),
    (36, "fAVDSkillLockpickBase", 6),
    (37, "fAVDSkillMedicineBase", 9),
    (38, "fAVDSkillMeleeWeaponsBase", 5),
    (39, "fAVDSkillRepairBase", 9),
    (40, "fAVDSkillScienceBase", 9),
    (41, "fAVDSkillSmallGunsBase", 10),
    (42, "fAVDSkillSneakBase", 10),
    (43, "fAVDSkillSpeechBase", 8),
    (44, "fAVDSkillSurvivalBase", 7),
    (45, "fAVDSkillUnarmedBase", 7),
];

/// The skills the menus show, by their shown names' alphabetical order
/// (Barter, Energy Weapons, Explosives, Guns, Lockpick, Medicine, Melee
/// Weapons, Repair, Science, Sneak, Speech, Survival, Unarmed).
pub const SHOWN_SKILLS: [u16; 13] = [32, 34, 35, 41, 36, 37, 38, 39, 40, 42, 43, 44, 45];

/// A trait the player can pick.
#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub form_id: FormId,
    pub name: String,
    pub description: String,
}

/// The traits the trait menu offers, by name.
pub fn traits(order: &LoadOrder) -> Vec<Trait> {
    let mut out: Vec<Trait> = order
        .records_of_type(PERK)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let record = rr.record().ok()?;
            let data = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 5)?;
            // Trait, playable, not hidden.
            if data.data[0] != 1 || data.data[3] != 1 || data.data[4] != 0 {
                return None;
            }
            Some(Trait {
                form_id: rr.form_id,
                name: record.full_name()?,
                description: record.get(DESC).map(|s| s.zstring()).unwrap_or_default(),
            })
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// How many traits the menu allows (`iTraitMenuMaxNumTraits`).
pub fn max_traits(order: &LoadOrder) -> u32 {
    game_setting(order, "iTraitMenuMaxNumTraits").map_or(2, |v| v as u32)
}

/// The player's name: what was chosen, else `sDefaultPlayerName`.
pub fn player_name(order: &LoadOrder, state: &GameState) -> String {
    if let Some(n) = &state.player_name {
        return n.clone();
    }
    order
        .form_by_editor_id("sDefaultPlayerName")
        .and_then(|id| order.get(id))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).map(|s| s.zstring()))
        .unwrap_or_else(|| "Courier".into())
}

/// A skill of the player's from SPECIAL and tags (see the module notes);
/// `special` gives an attribute's value.
pub fn player_skill(
    order: &LoadOrder,
    state: &GameState,
    skill: u16,
    special: impl Fn(u16) -> Option<f64>,
) -> Option<f64> {
    let &(_, setting, attribute) = SKILLS.iter().find(|s| s.0 == skill)?;
    let base = f64::from(game_setting(order, setting)?);
    let luck = special(11)?;
    let tag = if state.tag_skills.contains(&skill) {
        f64::from(game_setting(order, "fAVDTagSkillBonus")?)
    } else {
        0.0
    };
    let points = f64::from(state.skill_points.get(&skill).copied().unwrap_or(0));
    Some(base + 2.0 * special(attribute)? + (luck / 2.0).ceil() + tag + points)
}

/// The most a skill can be raised to with level-up points.
pub const SKILL_MAX: f64 = 100.0;

/// The SPECIAL menu's choice: the seven values (Strength … Luck).
pub fn set_special(state: &mut GameState, values: [u8; 7]) {
    for (av, v) in SPECIAL.iter().zip(values) {
        state
            .actor_values
            .insert((crate::dialogue::PLAYER_REF, *av), f64::from(v));
    }
}

/// The tag skills menu's Done (`007537b0`, `00754530`): every tag slot is
/// cleared, then the skills picked fill slots 0, 1, 2 … in the list's
/// order (the skills' names, A to Z).
pub fn set_tag_skills(state: &mut GameState, picked: &[u16]) {
    state.tag_skills = picked.iter().copied().collect();
    state.tag_slots = (0u8..).zip(picked.iter().copied()).collect();
}

/// Whether SPECIAL values can be kept: each in range, `points` in all.
pub fn special_ok(values: [u8; 7], points: u32) -> bool {
    values
        .iter()
        .all(|v| (SPECIAL_RANGE.0..=SPECIAL_RANGE.1).contains(v))
        && values.iter().map(|&v| u32::from(v)).sum::<u32>() == points
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    /// `0066e9f0`, `0066e9b0`: an actor value's picture and description
    /// come from its `AVIF` record (`AV` + the script name).
    #[test]
    fn an_actor_values_picture_and_description() {
        let dir = std::env::temp_dir().join(format!("nv-rs-avif-{}", std::process::id()));
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
        let mut d = sub(b"EDID", &zstr("AVScience"));
        d.extend(sub(b"FULL", &zstr("Science")));
        d.extend(sub(b"DESC", &zstr("Hacking and chemistry.")));
        d.extend(sub(b"ICON", &zstr("Interface\\Icons\\science.dds")));
        plugin.extend(group(*b"AVIF", 0, &record(b"AVIF", 0x800, &d)));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        assert_eq!(
            actor_value_icon_and_text(&order, 40),
            (
                "Interface\\Icons\\science.dds".to_string(),
                "Hacking and chemistry.".to_string()
            )
        );
        assert_eq!(
            actor_value_icon_and_text(&order, 32),
            (String::new(), String::new())
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `007537b0`: the tag skills picked replace every slot, in order.
    #[test]
    fn tag_skills_replace_the_slots() {
        let mut state = GameState::default();
        state.tag_skills.insert(45);
        state.tag_slots.insert(2, 45);
        set_tag_skills(&mut state, &[32, 41]);
        assert_eq!(state.tag_skills, [32, 41].into());
        assert_eq!(state.tag_slots, [(0, 32), (1, 41)].into());
    }
}
