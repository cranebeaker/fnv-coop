//! A world for factions, reputation, karma, crime and faction armour
//! disguises, with `FalloutNV.esm`'s own values: the NCR's and the
//! Legion's reputations (most 80 and 100), `NCRFactionNV`,
//! `VCaesarsLegionFaction`, the armour factions the disguise scripts use
//! (`ArmorNCRFactionNV`, a friend of itself; `ArmorNCRFactionNVEnemy`,
//! the Legion's enemy) and the NCR's sniffers; the NCR trooper armour with
//! `NCRFactionOutfitWarningScript`, the `FactionGearNV…` lists,
//! `DisguiseFactionPulseQuest` with its script and its spell (a touch
//! effect of area 25 for 5 s running `DisguiseFactionPulseScript`); people
//! of each karma band; an owned terminal; and the interior `TestOutpost`
//! with a trooper, a sniffer and a legionary.

use crate::{f32s, group, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const PLAYER: u32 = 0x7;
    /// `PlayerFaction`, a fixed form in every game.
    pub const PLAYER_FACTION: u32 = 0x1B2A4;
    pub const VALUE: u32 = 0xF00;
    pub const SETTINGS: u32 = 0xF01;
    pub const GLOBALS: u32 = 0xF40;
    // Reputations.
    pub const REP_NCR: u32 = 0xF60;
    pub const REP_LEGION: u32 = 0xF61;
    // Factions.
    pub const NCR: u32 = 0xF70;
    pub const LEGION: u32 = 0xF71;
    pub const ARMOR_NCR: u32 = 0xF72;
    pub const ARMOR_NCR_ENEMY: u32 = 0xF73;
    pub const NCR_SNIFFERS: u32 = 0xF74;
    /// Flagged evil (`DATA` 0x02) and tracking crime.
    pub const FIENDS: u32 = 0xF75;
    // Scripts.
    pub const OUTFIT_SCRIPT: u32 = 0xF80;
    pub const PULSE_QUEST_SCRIPT: u32 = 0xF81;
    pub const PULSE_SCRIPT: u32 = 0xF82;
    // The disguise's quest, effect and spell.
    pub const PULSE_QUEST: u32 = 0xF88;
    pub const PULSE_EFFECT: u32 = 0xF89;
    pub const PULSE_SPELL: u32 = 0xF8A;
    // Items and lists.
    /// `ArmorNVNCRTrooper`, with `NCRFactionOutfitWarningScript`.
    pub const TROOPER_ARMOR: u32 = 0xF90;
    pub const GEAR_NCR: u32 = 0xF91;
    pub const GEAR_LEGION: u32 = 0xF92;
    pub const GEAR_BOS: u32 = 0xF93;
    pub const GEAR_KHANS: u32 = 0xF94;
    pub const GEAR_POWDER: u32 = 0xF95;
    /// A terminal, and one flagged unlocked (`DNAM` 0x02).
    pub const TERMINAL: u32 = 0xF96;
    pub const OPEN_TERMINAL: u32 = 0xF97;
    // Messages.
    pub const WARNING_MESSAGE: u32 = 0xFA0;
    pub const WARNING_OFF_MESSAGE: u32 = 0xFA1;
    pub const TUTORIAL_MESSAGE: u32 = 0xFA2;
    // People (aggressive: `AIDT` aggression 1).
    /// In `NCRFactionNV` and `ArmorNCRFactionNV`.
    pub const TROOPER: u32 = 0xFB0;
    /// In those and `VNCRSnifferFaction`.
    pub const SNIFFER: u32 = 0xFB1;
    /// In `VCaesarsLegionFaction`.
    pub const LEGIONARY: u32 = 0xFB2;
    /// Karma 500 (good), 800 (very good), −500 (evil), −800 (very evil),
    /// 0; each in the NCR (which tracks crime).
    pub const GOOD: u32 = 0xFB3;
    pub const VERY_GOOD: u32 = 0xFB4;
    pub const EVIL: u32 = 0xFB5;
    pub const VERY_EVIL: u32 = 0xFB6;
    pub const NEUTRAL: u32 = 0xFB7;
    /// Karma 0, in no faction.
    pub const LONER: u32 = 0xFB8;
    // The cell and its references (people at y = 0 facing north, the
    // player at y = 100 unless moved).
    pub const OUTPOST: u32 = 0xFC0;
    pub const TROOPER_REF: u32 = 0xFD0;
    /// 1000 units east of the others.
    pub const SNIFFER_REF: u32 = 0xFD1;
    pub const LEGIONARY_REF: u32 = 0xFD2;
    /// Owned by the NCR.
    pub const TERMINAL_REF: u32 = 0xFD3;
    pub const OPEN_TERMINAL_REF: u32 = 0xFD4;
    /// `TestBarracks`, with the people of each karma band (refs
    /// `0xFE0` + 0..6 for `GOOD` .. `LONER`).
    pub const BARRACKS: u32 = 0xFC1;
    pub const GOOD_REF: u32 = 0xFE0;
}

/// `NCRFactionOutfitWarningScript` as `FalloutNV.esm` has it, its
/// equipping and unequipping blocks for the NCR, the Legion, the
/// Brotherhood, the Khans and the Powder Gangers (the White Glove
/// Society's items and the companions' `OnAdd` blocks left out).
pub const OUTFIT_SOURCE: &str = "scn NCRFactionOutfitWarningScript\n\
float fUpdatedNegNCR\nfloat fUpdatedPosNCR\nfloat fUpdatedNegCL\nfloat fUpdatedPosCL\n\
begin OnEquip Player\n\
\tif DisguiseFactionPulseQuest.bFactionArmorEquipped == 0\n\
\t\tset DisguiseFactionPulseQuest.bFactionArmorEquipped to 1\n\
\t\tset DisguiseFactionPulseQuest.fNCRNegReputation to GetReputation RepNVNCR 0\n\
\t\tset DisguiseFactionPulseQuest.fNCRPosReputation to GetReputation RepNVNCR 1\n\
\t\tset DisguiseFactionPulseQuest.fLegionNegReputation to GetReputation RepNVCaesarsLegion 0\n\
\t\tset DisguiseFactionPulseQuest.fLegionPosReputation to GetReputation RepNVCaesarsLegion 1\n\
\telse\n\
\t\tset DisguiseFactionPulseQuest.fNCRNegReputation to GetReputation RepNVNCR 0 + DisguiseFactionPulseQuest.fNCRNegReputation\n\
\t\tset DisguiseFactionPulseQuest.fNCRPosReputation to GetReputation RepNVNCR 1+ DisguiseFactionPulseQuest.fNCRPosReputation\n\
\t\tset DisguiseFactionPulseQuest.fLegionNegReputation to GetReputation RepNVCaesarsLegion 0 + DisguiseFactionPulseQuest.fLegionNegReputation\n\
\t\tset DisguiseFactionPulseQuest.fLegionPosReputation to GetReputation RepNVCaesarsLegion 1 + DisguiseFactionPulseQuest.fLegionPosReputation\n\
\tendif\n\
\tif VFactionArmorTutorial == 0\n\
\t\tShowMessage VFactionOutfitTutorial\n\
\t\tset VFactionArmorTutorial to 1\n\
\tendif\n\
\tShowMessage NCRFactionOutfitWarning\n\
\tStartQuest DisguiseFactionPulseQuest ;Kicks the pulse placing quest\n\
\tsetally PlayerFaction ArmorNCRFactionNV 1 1\n\
\tplayer.addtofaction ARMORNCRFactionNVEnemy 0\n\
\tSetReputation RepNVNCR 0 0.0\n\
\tSetReputation RepNVNCR 1 0.0\n\
\tSetReputation RepNVCaesarsLegion 0 0.0\n\
\tSetReputation RepNVCaesarsLegion 1 0.0\n\
end\n\
begin OnUnequip Player\n\
\tif player.GetEquipped FactionGearNVNCR == 0\n\
\t\tStopQuest DisguiseFactionPulseQuest ;Stops the pulse placing quest\n\
\t\tSetEnemy PlayerFaction ArmorNCRFactionNV 1 1\n\
\t\tplayer.RemoveFromFaction ARMORNCRFactionNVEnemy\n\
\t\tShowMessage NCRFactionOutfitWarningOff\n\
\t\tset fUpdatedNegNCR to GetReputation RepNVNCR 0 + DisguiseFactionPulseQuest.fNCRNegReputation\n\
\t\tset fUpdatedPosNCR to GetReputation RepNVNCR 1+ DisguiseFactionPulseQuest.fNCRPosReputation\n\
\t\tset fUpdatedNegCL to GetReputation RepNVCaesarsLegion 0 + DisguiseFactionPulseQuest.fLegionNegReputation\n\
\t\tset fUpdatedPosCL to GetReputation RepNVCaesarsLegion 1 + DisguiseFactionPulseQuest.fLegionPosReputation\n\
\tendif\n\
\tif (Player.GetEquipped FactionGearNVNCR == 0) && (Player.GetEquipped FactionGearNVCaesar == 0) && (Player.GetEquipped FactionGearNVBrotherhood == 0) && (Player.GetEquipped FactionGearNVGreatKhans == 0) && (Player.GetEquipped FactionGearNVPowder == 0)\n\
\t\tset DisguiseFactionPulseQuest.bFactionArmorEquipped to 0\n\
\t\tSetReputation RepNVNCR 0 fUpdatedNegNCR\n\
\t\tSetReputation RepNVNCR 1 fUpdatedPosNCR\n\
\t\tSetReputation RepNVCaesarsLegion 0 fUpdatedNegCL\n\
\t\tSetReputation RepNVCaesarsLegion 1 fUpdatedPosCL\n\
\tendif\n\
end\n";

/// `DisguiseFactionQuestScript` as `FalloutNV.esm` has it, without its
/// five-minute reminders.
pub const PULSE_QUEST_SOURCE: &str = "Scn\tDisguiseFactionQuestScript\n\
float fTimer\nShort\tDebugGlow\nshort bFactionArmorEquipped\n\
float fNCRNegReputation\nfloat fNCRPosReputation\n\
float fLegionNegReputation\nfloat fLegionPosReputation\n\
Begin GameMode\n\
\tPlayer.CIOS DisguiseFactionPulseActorEffect\n\
End\n";

/// `DisguiseFactionPulseScript` as `FalloutNV.esm` has it (its NCR and
/// Legion lines).
pub const PULSE_SOURCE: &str = "Scn\tDisguiseFactionPulseScript\n\
Ref\tSelf\n\
Begin ScriptEffectStart\n\
\tIf GetIsReference Player == 1\n\
\t\tReturn\n\
\tElse\n\
\t\tSet Self to GetSelf\n\
\t\t\tIf Self.GetInFaction VNCRSnifferFaction == 1\n\
\t\t\t\tSelf.RemoveFromFaction ArmorNCRFactionNV\n\
\t\t\tElseif Self.GetinFaction VCaesarsLegionSnifferFaction == 1\n\
\t\t\t\tSelf.RemoveFromFaction ArmorVCaesarsLegionFaction\n\
\t\t\tEndif\n\
\tEndif\n\
End\n\
Begin ScriptEffectFinish\n\
\tSet Self to Getself\n\
\tIf Self.GetInFaction VNCRSnifferFaction == 1\n\
\t\tSelf.AddToFaction ArmorNCRFactionNV 0\n\
\tElseif Self.GetinFaction VCaesarsLegionSnifferFaction == 1\n\
\t\tSelf.AddToFaction ArmorVCaesarsLegionFaction 0\n\
\tEndif\n\
End\n";

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn factions(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
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

    // Game settings the data sets (`FalloutNV.esm`'s values); the others
    // are the exe's.
    let mut settings = Vec::new();
    let numbers: [(&str, f32); 2] = [
        ("fKarmaModStealing", -5.0),
        ("fKarmaModKillingEvilActor", 100.0),
    ];
    let texts: [(&str, &str); 9] = [
        ("sRepNegativeGain", "Infamy Gained!"),
        ("sRepNegativeLoss", "Infamy Reduced"),
        ("sRepPositiveGain", "Fame Gained!"),
        ("sRepPositiveLoss", "Fame Reduced"),
        (
            "sRepTitlePosTwoNegThreeDesc",
            "Most people say you're the devil himself, but they also admit you've done a bit of good.",
        ),
        ("sKarmaMinorLost", "You've lost Karma!"),
        ("sKarmaMinorGained", "You've gained Karma!"),
        ("sKarmaMajorLost", "You've lost Karma!"),
        ("sKarmaMajorGained", "You've gained Karma!"),
    ];
    let mut next = SETTINGS;
    for (name, v) in numbers {
        settings.extend(named(b"GMST", next, name, &sub(b"DATA", &v.to_le_bytes())));
        next += 1;
    }
    for (name, t) in texts {
        settings.extend(named(b"GMST", next, name, &sub(b"DATA", &zstr(t))));
        next += 1;
    }
    plugin.extend(group(*b"GMST", 0, &settings));

    let mut globals = Vec::new();
    for (i, (name, value)) in [
        ("TestValue", 0.0f32),
        ("GameHour", 10.0),
        ("TimeScale", 30.0),
        ("GameDaysPassed", 5.0),
        ("GameDay", 1.0),
        ("GameMonth", 0.0),
        ("GameYear", 2281.0),
        ("VFactionArmorTutorial", 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = sub(b"FNAM", b"s");
        d.extend(sub(b"FLTV", &value.to_le_bytes()));
        let id = if i == 0 { VALUE } else { GLOBALS + i as u32 };
        globals.extend(named(b"GLOB", id, name, &d));
    }
    plugin.extend(group(*b"GLOB", 0, &globals));

    // Reputations: `DATA` the most.
    let reputation = |id: u32, ed: &str, name: &str, most: f32| {
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"DATA", &most.to_le_bytes()));
        named(b"REPU", id, ed, &d)
    };
    let mut reps = reputation(REP_NCR, "RepNVNCR", "NCR", 80.0);
    reps.extend(reputation(
        REP_LEGION,
        "RepNVCaesarsLegion",
        "Caesar's Legion",
        100.0,
    ));
    plugin.extend(group(*b"REPU", 0, &reps));

    // Factions: `DATA` flags (0x01 hidden, 0x02 evil, 0x100 tracks
    // crime), `WMI1` the reputation, `XNAM` relations (faction, modifier,
    // reaction: 0 neutral, 1 enemy, 2 ally, 3 friend), as the data has
    // them (only the relations among these).
    let xnam = |other: u32, reaction: u32| {
        let mut x = other.to_le_bytes().to_vec();
        x.extend(0i32.to_le_bytes());
        x.extend(reaction.to_le_bytes());
        sub(b"XNAM", &x)
    };
    let faction = |id: u32, ed: &str, flags: u32, rep: Option<u32>, relations: &[(u32, u32)]| {
        let mut d = sub(b"DATA", &flags.to_le_bytes());
        if let Some(r) = rep {
            d.extend(sub(b"WMI1", &r.to_le_bytes()));
        }
        for &(o, r) in relations {
            d.extend(xnam(o, r));
        }
        named(b"FACT", id, ed, &d)
    };
    let mut facts = faction(PLAYER_FACTION, "PlayerFaction", 0x01, None, &[]);
    facts.extend(faction(
        NCR,
        "NCRFactionNV",
        0x100,
        Some(REP_NCR),
        &[
            (LEGION, 1),
            (NCR_SNIFFERS, 2),
            (NCR, 2),
            (PLAYER_FACTION, 0),
        ],
    ));
    facts.extend(faction(
        LEGION,
        "VCaesarsLegionFaction",
        0x100,
        Some(REP_LEGION),
        &[
            (LEGION, 2),
            (ARMOR_NCR_ENEMY, 1),
            (NCR, 1),
            (PLAYER_FACTION, 0),
        ],
    ));
    facts.extend(faction(
        ARMOR_NCR,
        "ArmorNCRFactionNV",
        0x101,
        None,
        &[(ARMOR_NCR, 3)],
    ));
    facts.extend(faction(
        ARMOR_NCR_ENEMY,
        "ArmorNCRFactionNVEnemy",
        0x01,
        None,
        &[(LEGION, 1), (ARMOR_NCR_ENEMY, 3)],
    ));
    facts.extend(faction(
        NCR_SNIFFERS,
        "VNCRSnifferFaction",
        0x01,
        None,
        &[(PLAYER_FACTION, 0), (NCR, 2), (NCR_SNIFFERS, 2)],
    ));
    facts.extend(faction(FIENDS, "TestFiends", 0x102, None, &[]));
    plugin.extend(group(*b"FACT", 0, &facts));

    // Scripts.
    let script = |id: u32, name: &str, source: &str| {
        let mut d = sub(b"SCHR", &[0; 20]);
        d.extend(sub(b"SCTX", source.as_bytes()));
        named(b"SCPT", id, name, &d)
    };
    let mut scripts = script(
        OUTFIT_SCRIPT,
        "NCRFactionOutfitWarningScript",
        OUTFIT_SOURCE,
    );
    scripts.extend(script(
        PULSE_QUEST_SCRIPT,
        "DisguiseFactionQuestScript",
        PULSE_QUEST_SOURCE,
    ));
    scripts.extend(script(
        PULSE_SCRIPT,
        "DisguiseFactionPulseScript",
        PULSE_SOURCE,
    ));
    plugin.extend(group(*b"SCPT", 0, &scripts));

    // The pulse: the quest (`DATA` flags, priority, 2 unused, delay 1 s),
    // the effect (a script effect, archetype 1, its script at 8) and the
    // spell (`SPIT` type 0, flags 0x60; `EFIT` magnitude 1, area 25,
    // duration 5, range 1 touch, no actor value), as the data has them.
    let mut quest = sub(b"SCRI", &PULSE_QUEST_SCRIPT.to_le_bytes());
    quest.extend(sub(b"FULL", &zstr("Faction Disguise Pulse Quest")));
    let mut qdata = vec![0u8, 0, 0x72, 0x3d];
    qdata.extend(1.0f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    plugin.extend(group(
        *b"QUST",
        0,
        &named(b"QUST", PULSE_QUEST, "DisguiseFactionPulseQuest", &quest),
    ));
    let mut mgef = vec![0u8; 72];
    mgef[8..12].copy_from_slice(&PULSE_SCRIPT.to_le_bytes());
    mgef[16..20].copy_from_slice(&(-1i32).to_le_bytes());
    mgef[64..68].copy_from_slice(&1u32.to_le_bytes());
    mgef[68..72].copy_from_slice(&(-1i32).to_le_bytes());
    let mut effect = sub(b"FULL", &zstr("Faction Disguise Pulse Effect"));
    effect.extend(sub(b"DATA", &mgef));
    plugin.extend(group(
        *b"MGEF",
        0,
        &named(b"MGEF", PULSE_EFFECT, "DisguiseFactionPulseEffect", &effect),
    ));
    let mut spell = sub(b"FULL", &zstr("Disguise Faction Pulse"));
    let mut spit = 0u32.to_le_bytes().to_vec();
    spit.extend([0; 8]);
    spit.extend([0x60, 0, 0, 0]);
    spell.extend(sub(b"SPIT", &spit));
    spell.extend(sub(b"EFID", &PULSE_EFFECT.to_le_bytes()));
    let mut efit = 1u32.to_le_bytes().to_vec();
    efit.extend(25u32.to_le_bytes());
    efit.extend(5u32.to_le_bytes());
    efit.extend(1u32.to_le_bytes());
    efit.extend((-1i32).to_le_bytes());
    spell.extend(sub(b"EFIT", &efit));
    plugin.extend(group(
        *b"SPEL",
        0,
        &named(
            b"SPEL",
            PULSE_SPELL,
            "DisguiseFactionPulseActorEffect",
            &spell,
        ),
    ));

    // The armour (`BMDT` the upper body, `DATA` value, health, weight) and
    // the gear lists.
    let mut armor = sub(b"FULL", &zstr("NCR Trooper Armor"));
    armor.extend(sub(b"SCRI", &OUTFIT_SCRIPT.to_le_bytes()));
    armor.extend(sub(b"BMDT", &[4, 0, 0, 0, 0, 0, 0, 0]));
    let mut adata = 80i32.to_le_bytes().to_vec();
    adata.extend(150i32.to_le_bytes());
    adata.extend(15.0f32.to_le_bytes());
    armor.extend(sub(b"DATA", &adata));
    plugin.extend(group(
        *b"ARMO",
        0,
        &named(b"ARMO", TROOPER_ARMOR, "ArmorNVNCRTrooper", &armor),
    ));
    let mut lists = named(
        b"FLST",
        GEAR_NCR,
        "FactionGearNVNCR",
        &sub(b"LNAM", &TROOPER_ARMOR.to_le_bytes()),
    );
    for (id, name) in [
        (GEAR_LEGION, "FactionGearNVCaesar"),
        (GEAR_BOS, "FactionGearNVBrotherhood"),
        (GEAR_KHANS, "FactionGearNVGreatKhans"),
        (GEAR_POWDER, "FactionGearNVPowder"),
    ] {
        lists.extend(named(b"FLST", id, name, &[]));
    }
    plugin.extend(group(*b"FLST", 0, &lists));

    // Terminals: `DNAM` difficulty, flags, server type.
    let terminal = |id: u32, ed: &str, flags: u8| {
        let mut d = sub(b"FULL", &zstr("Terminal"));
        d.extend(sub(b"DESC", &zstr("ROBCO INDUSTRIES")));
        d.extend(sub(b"DNAM", &[0, flags, 0]));
        named(b"TERM", id, ed, &d)
    };
    let mut terms = terminal(TERMINAL, "TestNCRTerminal", 0);
    terms.extend(terminal(OPEN_TERMINAL, "TestOpenTerminal", 0x02));
    plugin.extend(group(*b"TERM", 0, &terms));

    // Messages (`DNAM` 0: a notice).
    let message = |id: u32, ed: &str, text: &str| {
        let mut d = sub(b"DESC", &zstr(text));
        d.extend(sub(b"DNAM", &0u32.to_le_bytes()));
        named(b"MESG", id, ed, &d)
    };
    let mut messages = message(
        WARNING_MESSAGE,
        "NCRFactionOutfitWarning",
        "Wearing NCR armor",
    );
    messages.extend(message(
        WARNING_OFF_MESSAGE,
        "NCRFactionOutfitWarningOff",
        "Removed NCR armor",
    ));
    messages.extend(message(
        TUTORIAL_MESSAGE,
        "VFactionOutfitTutorial",
        "Faction armor",
    ));
    plugin.extend(group(*b"MESG", 0, &messages));

    // People: `ACBS` karma f32 at 16; `DATA` health then SPECIAL; `AIDT`
    // aggression first; `SNAM` factions (rank 0).
    let person = |id: u32, name: &str, karma: f32, factions: &[u32]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut acbs = vec![0u8; 24];
        acbs[16..20].copy_from_slice(&karma.to_le_bytes());
        d.extend(sub(b"ACBS", &acbs));
        let mut stats = 100i32.to_le_bytes().to_vec();
        stats.extend([5; 7]);
        d.extend(sub(b"DATA", &stats));
        d.extend(sub(b"DNAM", &[10u8; 14]));
        let mut aidt = vec![0u8; 20];
        aidt[0] = 1;
        aidt[1] = 2;
        d.extend(sub(b"AIDT", &aidt));
        for f in factions {
            let mut m = f.to_le_bytes().to_vec();
            m.extend([0, 0, 0, 0]);
            d.extend(sub(b"SNAM", &m));
        }
        named(b"NPC_", id, &name.replace(' ', ""), &d)
    };
    let mut npcs = person(PLAYER, "Player", 0.0, &[]);
    npcs.extend(person(TROOPER, "NCR Trooper", 0.0, &[NCR, ARMOR_NCR]));
    npcs.extend(person(
        SNIFFER,
        "NCR Sniffer",
        0.0,
        &[NCR, ARMOR_NCR, NCR_SNIFFERS],
    ));
    npcs.extend(person(LEGIONARY, "Legionary", 0.0, &[LEGION]));
    npcs.extend(person(GOOD, "Good", 500.0, &[NCR]));
    npcs.extend(person(VERY_GOOD, "Very Good", 800.0, &[NCR]));
    npcs.extend(person(EVIL, "Evil", -500.0, &[NCR]));
    npcs.extend(person(VERY_EVIL, "Very Evil", -800.0, &[NCR]));
    npcs.extend(person(NEUTRAL, "Neutral", 0.0, &[NCR]));
    npcs.extend(person(LONER, "Loner", 500.0, &[]));
    plugin.extend(group(*b"NPC_", 0, &npcs));

    // The outpost.
    let thing = |kind: &[u8; 4], id: u32, base: u32, pos: [f32; 3], name: &str, extra: &[u8]| {
        let mut d = edid(name);
        d.extend(sub(b"NAME", &base.to_le_bytes()));
        d.extend(extra);
        d.extend(sub(
            b"DATA",
            &f32s(&[pos[0], pos[1], pos[2], 0.0, 0.0, 0.0]),
        ));
        record(kind, id, &d)
    };
    let mut refs = thing(
        b"ACHR",
        TROOPER_REF,
        TROOPER,
        [0.0, 0.0, 0.0],
        "TrooperRef",
        &[],
    );
    refs.extend(thing(
        b"ACHR",
        SNIFFER_REF,
        SNIFFER,
        [1000.0, 0.0, 0.0],
        "SnifferRef",
        &[],
    ));
    refs.extend(thing(
        b"ACHR",
        LEGIONARY_REF,
        LEGIONARY,
        [-200.0, 0.0, 0.0],
        "LegionaryRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        TERMINAL_REF,
        TERMINAL,
        [0.0, 300.0, 0.0],
        "NCRTerminalRef",
        &sub(b"XOWN", &NCR.to_le_bytes()),
    ));
    refs.extend(thing(
        b"REFR",
        OPEN_TERMINAL_REF,
        OPEN_TERMINAL,
        [100.0, 300.0, 0.0],
        "OpenTerminalRef",
        &sub(b"XOWN", &NCR.to_le_bytes()),
    ));
    let mut contents = named(b"CELL", OUTPOST, "TestOutpost", &sub(b"DATA", &[0x01]));
    contents.extend(group(
        OUTPOST.to_le_bytes(),
        6,
        &group(OUTPOST.to_le_bytes(), 9, &refs),
    ));
    let mut barracks = Vec::new();
    for (i, base) in [GOOD, VERY_GOOD, EVIL, VERY_EVIL, NEUTRAL, LONER]
        .into_iter()
        .enumerate()
    {
        barracks.extend(thing(
            b"ACHR",
            GOOD_REF + i as u32,
            base,
            [i as f32 * 100.0, 0.0, 0.0],
            &format!("KarmaRef{i}"),
            &[],
        ));
    }
    contents.extend(named(
        b"CELL",
        BARRACKS,
        "TestBarracks",
        &sub(b"DATA", &[0x01]),
    ));
    contents.extend(group(
        BARRACKS.to_le_bytes(),
        6,
        &group(BARRACKS.to_le_bytes(), 9, &barracks),
    ));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &contents)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
}
