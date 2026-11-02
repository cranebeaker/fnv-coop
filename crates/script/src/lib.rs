//! The game's scripting language. Every script record (`SCPT`), and every
//! dialogue line's and quest stage's result script, carries its source
//! text (`SCTX`) next to the compiled form; this crate parses that source
//! ([`parse`]) and runs it ([`interp`]) against the game through a
//! [`interp::Host`].
//!
//! The language: `ScriptName`, variable declarations (`short`, `int`,
//! `long`, `float`, `ref`), `Begin <block> [argument]` … `End` blocks,
//! `if` / `elseif` / `else` / `endif`, `set <variable> to <expression>`,
//! `return`, and function calls written `[reference.]Function arg arg…`
//! whose arguments are single words, numbers or strings. Expressions have
//! the usual operators; `;` starts a comment. Function signatures come
//! from the game's own table ([`functions::FUNCTIONS`]).

pub mod compiled;
pub mod functions;
pub mod interp;
mod lexer;
pub mod nvse;
mod parser;

pub use parser::{
    parse, parse_expression, Arg, Block, Call, Expr, Item, Op, ParseError, Script, Stmt, VarKind,
};

/// A parameter of a script function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Param {
    /// The parameter type number (see [`param_kind_name`]).
    pub kind: u8,
    pub optional: bool,
}

/// A script function's signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub name: &'static str,
    /// Its abbreviation (`GetAV` for `GetActorValue`), or empty.
    pub short: &'static str,
    /// Called on a reference (`Player.GetActorValue`), the script's owner
    /// when none is given.
    pub reference: bool,
    pub params: &'static [Param],
}

/// A function by name or short name (case-insensitive): its number and
/// signature.
pub fn function(name: &str) -> Option<(u16, &'static Signature)> {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static INDEX: OnceLock<HashMap<String, u16>> = OnceLock::new();
    let index = INDEX.get_or_init(|| {
        let mut map = HashMap::new();
        for (i, s) in functions::FUNCTIONS.iter().enumerate() {
            map.insert(s.name.to_ascii_lowercase(), i as u16);
            if !s.short.is_empty() {
                map.entry(s.short.to_ascii_lowercase()).or_insert(i as u16);
            }
        }
        map
    });
    let i = *index.get(&name.to_ascii_lowercase())?;
    Some((i, &functions::FUNCTIONS[usize::from(i)]))
}

/// A function's name by number, or the number when unknown.
pub fn function_name(index: u16) -> String {
    functions::FUNCTIONS
        .get(usize::from(index))
        .map_or_else(|| format!("function {index}"), |s| s.name.to_string())
}

/// The name of a parameter type, as the game's table gives it.
pub fn param_kind_name(kind: u8) -> &'static str {
    match kind {
        0 => "String",
        1 => "Integer",
        2 => "Float",
        3 | 21 | 50 | 53 => "ObjectID",
        4 => "ObjectReferenceID",
        5 => "Actor Value",
        6 => "Actor",
        7 => "Spell Item",
        8 => "Axis",
        9 => "Cell",
        10 => "Animation Group",
        11 => "Magic Item",
        12 => "Sound",
        13 => "Topic",
        14 => "Quest",
        15 => "Race",
        16 => "Class",
        17 => "Faction",
        18 => "Sex",
        19 => "Global",
        20 => "Furniture",
        22 => "Variable Name",
        23 => "Stage",
        24 => "Map Marker",
        25 => "Actor Base",
        26 => "Container",
        27 => "WorldSpace",
        28 => "Crime Type",
        29 => "Package",
        30 => "Combat Style",
        31 => "Magic Effect",
        32 => "Form Type",
        33 => "Weather ID",
        35 => "Owner",
        36 => "Effect Shader ID",
        37 => "Form List",
        39 => "Perk",
        40 => "Note",
        41 => "Miscellaneous Stat",
        42 => "Imagespace Modifier ID",
        43 => "ImageSpace",
        46 => "VoiceType",
        47 => "EncounterZone",
        48 => "Idle Form",
        49 => "Message",
        51 => "Alignment",
        52 => "EquipType",
        54 => "Music",
        55 => "CriticalStage",
        56 => "NPC or Leveled Character",
        57 => "Creature or Leveled Creature",
        58 => "Leveled Character",
        59 => "Leveled Creature",
        60 => "Leveled Item",
        61 => "Form",
        62 => "Reputation",
        63 => "Casino",
        65 => "Challenge",
        68 => "Caravan Deck",
        69 => "Region",
        _ => "?",
    }
}

/// Whether a parameter type names a record (a form ID) rather than a
/// number, a name or a string.
pub fn param_is_form(kind: u8) -> bool {
    matches!(
        kind,
        3 | 4
            | 6
            | 7
            | 9
            | 11..=17
            | 19..=21
            | 24..=27
            | 29..=31
            | 33
            | 35..=37
            | 39
            | 40
            | 42
            | 43
            | 46..=50
            | 53
            | 54
            | 56..=63
            | 65
            | 68
            | 69
    )
}

/// Actor value names by number, as scripts write them (`GetAV Luck`).
/// The numbers of the ones the game's scripts use were read from their
/// compiled form (Aggression 0, Strength 5 … Luck 11, Health 16,
/// DamageResist 18, SpeedMult 21 … BrainCondition 31, Explosives 35,
/// Lockpick 36, Medicine 37, Repair 39, Science 40, Sneak 42,
/// RadiationRads 54, Variable01 62 … Variable10 71); the others follow the
/// game's `AVIF` records (editor IDs without `AV`, numbered in form ID
/// order, which agrees with every confirmed one). Numbers 58, 59 and 61
/// have no record; their names here are Fallout 3's (unconfirmed).
pub const ACTOR_VALUES: [&str; 77] = [
    "Aggression",
    "Confidence",
    "Energy",
    "Responsibility",
    "Mood",
    "Strength",
    "Perception",
    "Endurance",
    "Charisma",
    "Intelligence",
    "Agility",
    "Luck",
    "ActionPoints",
    "CarryWeight",
    "CritChance",
    "HealRate",
    "Health",
    "MeleeDamage",
    "DamageResist",
    "PoisonResist",
    "RadResist",
    "SpeedMult",
    "Fatigue",
    "Karma",
    "XP",
    "PerceptionCondition",
    "EnduranceCondition",
    "LeftAttackCondition",
    "RightAttackCondition",
    "LeftMobilityCondition",
    "RightMobilityCondition",
    "BrainCondition",
    "Barter",
    "BigGuns",
    "EnergyWeapons",
    "Explosives",
    "Lockpick",
    "Medicine",
    "MeleeWeapons",
    "Repair",
    "Science",
    "SmallGuns",
    "Sneak",
    "Speech",
    "Throwing",
    "Unarmed",
    "InventoryWeight",
    "Paralysis",
    "Invisibility",
    "Chameleon",
    "NightEye",
    "DetectLifeRange",
    "FireResist",
    "WaterBreathing",
    "RadiationRads",
    "BloodyMess",
    "UnarmedDamage",
    "Assistance",
    "ElectricResist",
    "FrostResist",
    "EnergyResist",
    "EmpResist",
    "Variable01",
    "Variable02",
    "Variable03",
    "Variable04",
    "Variable05",
    "Variable06",
    "Variable07",
    "Variable08",
    "Variable09",
    "Variable10",
    "IgnoreCrippledLimbs",
    "Dehydration",
    "Hunger",
    "SleepDeprevation",
    "DamageThreshold",
];

/// An actor value's number by name (case-insensitive). New Vegas renamed
/// two skills and its scripts use the new names (`player.setav guns 70` in
/// `PressDemoQuestScript`, `player.SetAV Survival 0` in
/// `VCG04ActivatorScript`), which are also the `AVIF` records' shown names
/// for `AVSmallGuns` and `AVThrowing`.
pub fn actor_value(name: &str) -> Option<u16> {
    if name.eq_ignore_ascii_case("Guns") {
        return Some(41);
    }
    if name.eq_ignore_ascii_case("Survival") {
        return Some(44);
    }
    ACTOR_VALUES
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name))
        .map(|i| i as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_values_have_the_numbers_the_compiled_scripts_use() {
        assert_eq!(actor_value("aggression"), Some(0));
        assert_eq!(actor_value("Luck"), Some(11));
        assert_eq!(actor_value("health"), Some(16));
        assert_eq!(actor_value("SpeedMult"), Some(21));
        assert_eq!(actor_value("BrainCondition"), Some(31));
        assert_eq!(actor_value("Science"), Some(40));
        assert_eq!(actor_value("RadiationRads"), Some(54));
        assert_eq!(actor_value("Variable10"), Some(71));
        assert_eq!(actor_value("guns"), actor_value("SmallGuns"));
        assert_eq!(actor_value("Survival"), Some(44));
        assert!(param_is_form(14) && !param_is_form(5) && !param_is_form(1));
    }

    #[test]
    fn functions_are_found_by_name_or_short_name() {
        let (i, s) = function("getav").unwrap();
        assert_eq!((i, s.name), (14, "GetActorValue"));
        assert!(s.reference);
        let (i, s) = function("SetStage").unwrap();
        assert_eq!(s.name, "SetStage");
        assert_eq!(function_name(i), "SetStage");
        assert_eq!(s.params.len(), 2);
        assert!(function("NotAFunction").is_none());
    }
}
