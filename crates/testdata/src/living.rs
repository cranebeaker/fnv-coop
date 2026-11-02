//! A world for `world::living`: hardcore needs' stages (`DEHY` written out
//! of order, `HUNG`, `SLPD`, `RADS`) with their spells, purified water,
//! beds (free, owned by a person, by a faction, the player's, one with the
//! game's `PlayerBedSCRIPT` and Well Rested), people to pickpocket (one
//! evil, one a teammate), a grenade and a pistol, the town faction that
//! tracks crime with its reputation, and the interiors `TestHome`,
//! `TestInn` (owned by Owner, not public), `TestOffLimits` (the town's,
//! record flag 0x20000), `TestBarracks` (record flag 0x80000: no waiting)
//! and `TestSaloon` (public).

use crate::{f32s, group, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const PLAYER: u32 = 0x7;
    /// The hard-coded topics.
    pub const PICKPOCKET_TOPIC: u32 = 0xE3;
    pub const GUARD_TRESPASS: u32 = 0xF2;
    /// A global the tests set to an expression's value.
    pub const VALUE: u32 = 0xD00;
    pub const SETTINGS: u32 = 0xD01;
    pub const GLOBALS: u32 = 0xD40;
    // Magic effects.
    pub const REDUCE_ENDURANCE: u32 = 0xD60;
    pub const REDUCE_STRENGTH: u32 = 0xD61;
    pub const REDUCE_AGILITY: u32 = 0xD62;
    pub const DAMAGE_HEALTH: u32 = 0xD63;
    pub const RESTORE_DEHYDRATION: u32 = 0xD64;
    pub const WELL_RESTED_EFFECT: u32 = 0xD65;
    pub const DAMAGE_RADS: u32 = 0xD66;
    // Spells.
    pub const DEHYDRATION_200: u32 = 0xD70;
    pub const DEHYDRATION_400: u32 = 0xD71;
    pub const DEHYDRATION_1000: u32 = 0xD72;
    pub const HUNGER_200: u32 = 0xD73;
    pub const SLEEP_200: u32 = 0xD74;
    pub const RADS_200: u32 = 0xD75;
    /// `WellRestedSpell`: the script effect for 1440 s.
    pub const WELL_RESTED: u32 = 0xD76;
    /// A poison: −5 Health a second for 10 s.
    pub const POISON: u32 = 0xD77;
    // Stage records.
    pub const WATER_400: u32 = 0xD80;
    pub const WATER_200: u32 = 0xD81;
    pub const WATER_1000: u32 = 0xD82;
    pub const HUNGRY_200: u32 = 0xD83;
    pub const SLEEPY_200: u32 = 0xD84;
    pub const RAD_200: u32 = 0xD85;
    // Scripts, a message, a perk.
    pub const PLAYER_BED_SCRIPT: u32 = 0xD90;
    pub const WELL_RESTED_SCRIPT: u32 = 0xD91;
    pub const WELL_RESTED_MESSAGE: u32 = 0xD92;
    pub const WELL_RESTED_PERK: u32 = 0xD93;
    // Items.
    /// Restores Dehydration 50.
    pub const WATER: u32 = 0xDA0;
    /// Value 10.
    pub const MONEY: u32 = 0xDA1;
    /// Value 200.
    pub const GOLD: u32 = 0xDA2;
    /// Value 0.
    pub const PENCIL: u32 = 0xDA3;
    /// Animation type 10, a lobber projectile.
    pub const GRENADE: u32 = 0xDA4;
    /// Animation type 3, a missile projectile.
    pub const PISTOL: u32 = 0xDA5;
    pub const LOBBER: u32 = 0xDA6;
    pub const BULLET: u32 = 0xDA7;
    // Factions and a reputation.
    /// Tracks crime (`DATA` 0x100), reputation `TestTownRep` (`WMI1`).
    pub const TOWN: u32 = 0xDB0;
    pub const TOWN_REP: u32 = 0xDB1;
    // People.
    /// In the town (rank 0), Sneak 20; carries 5 money, a gold bar and a
    /// pencil.
    pub const OWNER: u32 = 0xDC0;
    /// In the town.
    pub const GUARD: u32 = 0xDC1;
    /// In no faction.
    pub const STRANGER: u32 = 0xDC2;
    /// Karma −1000; carries 3 money.
    pub const VILLAIN: u32 = 0xDC3;
    pub const PAL: u32 = 0xDC4;
    // Furniture.
    /// `MNAM` 0x80000003.
    pub const BED: u32 = 0xDD0;
    /// The bed with `PlayerBedSCRIPT`.
    pub const SCRIPTED_BED: u32 = 0xDD1;
    /// `MNAM` 0x40000001.
    pub const CHAIR: u32 = 0xDD2;
    // Cells.
    pub const HOME: u32 = 0xDE0;
    pub const INN: u32 = 0xDE1;
    pub const OFF_LIMITS: u32 = 0xDE2;
    pub const BARRACKS: u32 = 0xDE3;
    pub const SALOON: u32 = 0xDE4;
    // References in TestHome (people at y = 0 facing north, the player
    // to stand at y = 100).
    pub const FREE_BED_REF: u32 = 0xE00;
    /// Owned by Owner.
    pub const OWNED_BED_REF: u32 = 0xE01;
    /// Owned by the town, rank 5 asked (`XRNK`).
    pub const TOWN_BED_REF: u32 = 0xE02;
    pub const SCRIPTED_BED_REF: u32 = 0xE03;
    /// Owned by Owner.
    pub const CHAIR_REF: u32 = 0xE04;
    pub const MARK_REF: u32 = 0xE05;
    pub const VILLAIN_REF: u32 = 0xE06;
    pub const PAL_REF: u32 = 0xE07;
    // In TestInn.
    pub const INN_OWNER_REF: u32 = 0xE10;
    pub const INN_GUARD_REF: u32 = 0xE11;
    pub const INN_STRANGER_REF: u32 = 0xE12;
    /// The player's own (`XOWN` the player).
    pub const INN_BED_REF: u32 = 0xE13;
    // Elsewhere.
    pub const OFF_LIMITS_GUARD_REF: u32 = 0xE20;
    pub const BARRACKS_BED_REF: u32 = 0xE21;
    pub const SALOON_OWNER_REF: u32 = 0xE22;
}

/// `PlayerBedSCRIPT` as `FalloutNV.esm` has it.
pub const PLAYER_BED_SOURCE: &str = "scn PlayerBedSCRIPT\n\
short playerSleep\n\
begin OnActivate\n\
\tif isActionRef player == 1\n\
\t\tset playerSleep to 1\n\
\tendif\n\
\tActivate\n\
end\n\
begin menumode\n\
\tif playerSleep == 1 && IsPCSleeping == 1\n\
\t\tset playerSleep to 2\n\
\tendif\n\
end\n\
begin gamemode\n\
\tif playerSleep > 0\n\
\t\tif playerSleep == 2\n\
\t\t\tplayer.CastImmediateOnSelf WellRestedSpell\n\
\t\tendif\n\
\t\tset playerSleep to 0\n\
\tendif\n\
end\n";

/// `WellRestedMessageEffectSCRIPT` as `FalloutNV.esm` has it.
pub const WELL_RESTED_SOURCE: &str = "scn WellRestedMessageEffectSCRIPT\n\
begin ScriptEffectStart\n\
\tshowMessage AbilityWellRestedStartMsg\n\
\tplayer.resetHealth\n\
\tIf HasPerk WellRestedPerk == 0\n\
\t\tAddPerk WellRestedPerk\n\
\tEndIf\n\
end\n\
begin ScriptEffectFinish\n\
\tRemovePerk WellRestedPerk\n\
end\n";

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn living(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };
    let with_flags = |mut r: Vec<u8>, flags: u32| {
        r[8..12].copy_from_slice(&flags.to_le_bytes());
        r
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

    // Game settings: the data's values (`FalloutNV.esm`), leaving out
    // those only the exe has (`fHCDehydrationRate`, the hunger texts).
    let mut settings = Vec::new();
    let numbers: [(&str, f32); 10] = [
        ("fHCSleepDeprivationRate", 50.0),
        ("fHCStarvationRate", 25.0),
        ("fPickPocketActorSkillBase", 40.0),
        ("fPickPocketActorSkillMult", 0.6),
        ("fPickPocketTargetSkillMult", -0.6),
        ("fPickPocketAmountMult", -0.5),
        ("fPickPocketMaxChance", 85.0),
        ("fKarmaModStealing", -5.0),
        ("fReputationMinorCrimeNeg", 2.0),
        ("fAVDHealRateEndurance9Bonus", 10.0),
    ];
    let texts: [(&str, &str); 7] = [
        (
            "sDehydrationIncrease",
            "Your dehydration level has increased.",
        ),
        (
            "sDehydrationDecrease",
            "Your dehydration level has decreased.",
        ),
        (
            "sDehydrationNotSick",
            "You no longer have dehydration sickness.",
        ),
        (
            "sNoSleepHostileActorsNear",
            "You cannot sleep when enemies are nearby.",
        ),
        ("sNoPickPocketAgain", "has already caught you."),
        ("sNoTalkUnConscious", "is unconscious."),
        ("sNoTalkFleeing", "is fleeing."),
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
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = sub(b"FNAM", b"f");
        d.extend(sub(b"FLTV", &value.to_le_bytes()));
        let id = if i == 0 { VALUE } else { GLOBALS + i as u32 };
        globals.extend(named(b"GLOB", id, name, &d));
    }
    plugin.extend(group(*b"GLOB", 0, &globals));

    // Topics the game hard-codes.
    let mut topics = named(b"DIAL", PICKPOCKET_TOPIC, "Pickpocket", &[]);
    topics.extend(named(b"DIAL", GUARD_TRESPASS, "GuardTrespass", &[]));
    plugin.extend(group(*b"DIAL", 0, &topics));

    // Scripts.
    let script = |id: u32, name: &str, source: &str| {
        let mut d = sub(b"SCHR", &[0; 20]);
        d.extend(sub(b"SCTX", source.as_bytes()));
        named(b"SCPT", id, name, &d)
    };
    let mut scripts = script(PLAYER_BED_SCRIPT, "PlayerBedSCRIPT", PLAYER_BED_SOURCE);
    scripts.extend(script(
        WELL_RESTED_SCRIPT,
        "WellRestedMessageEffectSCRIPT",
        WELL_RESTED_SOURCE,
    ));
    plugin.extend(group(*b"SCPT", 0, &scripts));

    // Magic effects: `DATA` flags at 0 (0x02 recover, 0x04 detrimental),
    // the script at 8, no resistance (-1 at 16), archetype at 64, actor
    // value at 68.
    let mgef = |id: u32, name: &str, flags: u32, script: u32, archetype: u32, av: i32| {
        let mut d = vec![0u8; 72];
        d[0..4].copy_from_slice(&flags.to_le_bytes());
        d[8..12].copy_from_slice(&script.to_le_bytes());
        d[16..20].copy_from_slice(&(-1i32).to_le_bytes());
        d[64..68].copy_from_slice(&archetype.to_le_bytes());
        d[68..72].copy_from_slice(&av.to_le_bytes());
        let mut r = sub(b"FULL", &zstr(name));
        r.extend(sub(b"DATA", &d));
        named(b"MGEF", id, &name.replace(' ', ""), &r)
    };
    let mut effects = mgef(REDUCE_ENDURANCE, "Reduced Endurance", 0x06, 0, 0, 7);
    effects.extend(mgef(REDUCE_STRENGTH, "Reduced Strength", 0x06, 0, 0, 5));
    effects.extend(mgef(REDUCE_AGILITY, "Reduced Agility", 0x06, 0, 0, 10));
    effects.extend(mgef(DAMAGE_HEALTH, "Damage Health", 0x04, 0, 0, 16));
    effects.extend(mgef(
        RESTORE_DEHYDRATION,
        "Restore Dehydration Level",
        0,
        0,
        0,
        73,
    ));
    effects.extend(mgef(
        WELL_RESTED_EFFECT,
        "Well Rested Message Effect",
        0,
        WELL_RESTED_SCRIPT,
        1,
        -1,
    ));
    effects.extend(mgef(DAMAGE_RADS, "Damage Rads", 0x04, 0, 0, 54));
    plugin.extend(group(*b"MGEF", 0, &effects));

    // Spells: `SPIT` type at 0 (0 spell, 1 disease, 4 ability).
    let effect = |mgef: u32, magnitude: u32, duration: u32, av: i32| {
        let mut e = sub(b"EFID", &mgef.to_le_bytes());
        let mut efit = magnitude.to_le_bytes().to_vec();
        efit.extend(0u32.to_le_bytes());
        efit.extend(duration.to_le_bytes());
        efit.extend(0u32.to_le_bytes());
        efit.extend(av.to_le_bytes());
        e.extend(sub(b"EFIT", &efit));
        e
    };
    let spell = |id: u32, edid: &str, name: &str, kind: u32, effects: &[Vec<u8>]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut spit = kind.to_le_bytes().to_vec();
        spit.extend([0; 12]);
        d.extend(sub(b"SPIT", &spit));
        for e in effects {
            d.extend(e);
        }
        named(b"SPEL", id, edid, &d)
    };
    let mut spells = spell(
        DEHYDRATION_200,
        "Dehydration200",
        "Minor Dehydration",
        1,
        &[effect(REDUCE_ENDURANCE, 1, 0, 7)],
    );
    spells.extend(spell(
        DEHYDRATION_400,
        "Dehydration400",
        "Adv. Dehydration",
        1,
        &[effect(REDUCE_ENDURANCE, 2, 0, 7)],
    ));
    spells.extend(spell(
        DEHYDRATION_1000,
        "Dehydration1000",
        "Fatal Dehydration",
        4,
        &[effect(DAMAGE_HEALTH, 10000, 0, 16)],
    ));
    spells.extend(spell(
        HUNGER_200,
        "Starvation200",
        "Minor Starvation",
        1,
        &[effect(REDUCE_STRENGTH, 1, 0, 5)],
    ));
    spells.extend(spell(
        SLEEP_200,
        "SleepDeprivation200",
        "Minor Sleep Dep.",
        1,
        &[effect(REDUCE_AGILITY, 1, 0, 10)],
    ));
    spells.extend(spell(
        RADS_200,
        "Radiation200",
        "Minor Rad Poison.",
        1,
        &[effect(REDUCE_ENDURANCE, 1, 0, 7)],
    ));
    spells.extend(spell(
        WELL_RESTED,
        "WellRestedSpell",
        "Well Rested",
        0,
        &[effect(WELL_RESTED_EFFECT, 10, 1440, -1)],
    ));
    spells.extend(spell(
        POISON,
        "TestPoison",
        "Poison",
        0,
        &[effect(DAMAGE_HEALTH, 5, 10, 16)],
    ));
    plugin.extend(group(*b"SPEL", 0, &spells));

    // Stages: u32 threshold, u32 spell. Dehydration's written 400, 200,
    // 1000: kept highest first all the same.
    let stage = |kind: &[u8; 4], id: u32, name: &str, threshold: u32, spell: u32| {
        let mut d = threshold.to_le_bytes().to_vec();
        d.extend(spell.to_le_bytes());
        named(kind, id, name, &sub(b"DATA", &d))
    };
    let mut dehy = stage(b"DEHY", WATER_400, "Water2", 400, DEHYDRATION_400);
    dehy.extend(stage(b"DEHY", WATER_200, "Water1", 200, DEHYDRATION_200));
    dehy.extend(stage(b"DEHY", WATER_1000, "Water5", 1000, DEHYDRATION_1000));
    plugin.extend(group(*b"DEHY", 0, &dehy));
    plugin.extend(group(
        *b"HUNG",
        0,
        &stage(b"HUNG", HUNGRY_200, "hunger1", 200, HUNGER_200),
    ));
    plugin.extend(group(
        *b"SLPD",
        0,
        &stage(b"SLPD", SLEEPY_200, "sleep1", 200, SLEEP_200),
    ));
    plugin.extend(group(
        *b"RADS",
        0,
        &stage(b"RADS", RAD_200, "Rad1", 200, RADS_200),
    ));

    plugin.extend(group(
        *b"MESG",
        0,
        &named(
            b"MESG",
            WELL_RESTED_MESSAGE,
            "AbilityWellRestedStartMsg",
            &sub(b"DESC", &zstr("You are now Well Rested!")),
        ),
    ));
    plugin.extend(group(
        *b"PERK",
        0,
        &named(
            b"PERK",
            WELL_RESTED_PERK,
            "WellRestedPerk",
            &sub(b"FULL", &zstr("Well Rested")),
        ),
    ));

    // Items.
    let mut water = sub(b"FULL", &zstr("Purified Water"));
    water.extend(sub(b"DATA", &1.0f32.to_le_bytes()));
    water.extend(sub(b"ENIT", &{
        let mut e = 20i32.to_le_bytes().to_vec();
        e.extend([0; 16]);
        e
    }));
    water.extend(effect(RESTORE_DEHYDRATION, 50, 0, 73));
    plugin.extend(group(
        *b"ALCH",
        0,
        &named(b"ALCH", WATER, "WaterPurified", &water),
    ));
    let misc = |id: u32, name: &str, value: i32| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut v = value.to_le_bytes().to_vec();
        v.extend(0.1f32.to_le_bytes());
        d.extend(sub(b"DATA", &v));
        named(b"MISC", id, &name.replace(' ', ""), &d)
    };
    let mut miscs = misc(MONEY, "Money", 10);
    miscs.extend(misc(GOLD, "Gold Bar", 200));
    miscs.extend(misc(PENCIL, "Pencil", 0));
    plugin.extend(group(*b"MISC", 0, &miscs));
    let projectile = |id: u32, name: &str, kind: u16| {
        let mut d = 0u16.to_le_bytes().to_vec();
        d.extend(kind.to_le_bytes());
        d.extend(f32s(&[1.0, 1000.0, 10000.0]));
        named(b"PROJ", id, name, &sub(b"DATA", &d))
    };
    let mut projectiles = projectile(LOBBER, "TestLobber", 2);
    projectiles.extend(projectile(BULLET, "TestBullet", 1));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    let weapon = |id: u32, name: &str, anim: u8, proj: u32| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut data = 25i32.to_le_bytes().to_vec();
        data.extend(100i32.to_le_bytes());
        data.extend(1.0f32.to_le_bytes());
        data.extend(10i16.to_le_bytes());
        data.push(1);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        // No resist type (-1 at 120), as the game's guns.
        dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
        dnam[0] = anim;
        dnam[36..40].copy_from_slice(&proj.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        named(b"WEAP", id, &name.replace(' ', ""), &d)
    };
    let mut weapons = weapon(GRENADE, "Frag Grenade", 10, LOBBER);
    weapons.extend(weapon(PISTOL, "Pistol", 3, BULLET));
    plugin.extend(group(*b"WEAP", 0, &weapons));

    // The town and its reputation.
    plugin.extend(group(
        *b"REPU",
        0,
        &named(
            b"REPU",
            TOWN_REP,
            "TestTownRep",
            &sub(b"DATA", &15.0f32.to_le_bytes()),
        ),
    ));
    let mut town = sub(b"FULL", &zstr("Test Town"));
    town.extend(sub(b"DATA", &0x100u32.to_le_bytes()));
    town.extend(sub(b"WMI1", &TOWN_REP.to_le_bytes()));
    plugin.extend(group(*b"FACT", 0, &named(b"FACT", TOWN, "TestTown", &town)));

    // People: `ACBS` karma f32 at 16; `DATA` health then SPECIAL; `DNAM`
    // skills from Barter (Sneak the 11th).
    let person = |id: u32, name: &str, karma: f32, sneak: u8, extra: &[u8]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut acbs = vec![0u8; 24];
        acbs[16..20].copy_from_slice(&karma.to_le_bytes());
        d.extend(sub(b"ACBS", &acbs));
        let mut stats = 100i32.to_le_bytes().to_vec();
        stats.extend([5; 7]);
        d.extend(sub(b"DATA", &stats));
        let mut skills = vec![10u8; 14];
        skills[10] = sneak;
        d.extend(sub(b"DNAM", &skills));
        d.extend(extra);
        named(b"NPC_", id, &name.replace(' ', ""), &d)
    };
    let member = |faction: u32| {
        let mut m = faction.to_le_bytes().to_vec();
        m.extend([0, 0, 0, 0]);
        sub(b"SNAM", &m)
    };
    let carries = |item: u32, n: i32| {
        let mut c = item.to_le_bytes().to_vec();
        c.extend(n.to_le_bytes());
        sub(b"CNTO", &c)
    };
    let mut owner_extra = member(TOWN);
    owner_extra.extend(carries(MONEY, 5));
    owner_extra.extend(carries(GOLD, 1));
    owner_extra.extend(carries(PENCIL, 1));
    let mut npcs = person(PLAYER, "Player", 0.0, 10, &[]);
    npcs.extend(person(OWNER, "Owner", 0.0, 20, &owner_extra));
    npcs.extend(person(GUARD, "Guard", 0.0, 10, &member(TOWN)));
    npcs.extend(person(STRANGER, "Stranger", 0.0, 10, &[]));
    npcs.extend(person(VILLAIN, "Villain", -1000.0, 10, &carries(MONEY, 3)));
    npcs.extend(person(PAL, "Pal", 0.0, 10, &carries(MONEY, 1)));
    plugin.extend(group(*b"NPC_", 0, &npcs));

    // Furniture.
    let furniture = |id: u32, name: &str, mnam: u32, script: Option<u32>| {
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"MODL", &zstr("Furniture\\TestBed.nif")));
        if let Some(s) = script {
            d.extend(sub(b"SCRI", &s.to_le_bytes()));
        }
        d.extend(sub(b"MNAM", &mnam.to_le_bytes()));
        named(b"FURN", id, &name.replace(' ', ""), &d)
    };
    let mut furn = furniture(BED, "Bed", 0x8000_0003, None);
    furn.extend(furniture(
        SCRIPTED_BED,
        "Safehouse Bed",
        0x8000_0003,
        Some(PLAYER_BED_SCRIPT),
    ));
    furn.extend(furniture(CHAIR, "Chair", 0x4000_0001, None));
    plugin.extend(group(*b"FURN", 0, &furn));

    // The cells and what's in them.
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
    let owned = |o: u32| sub(b"XOWN", &o.to_le_bytes());
    let mut home = thing(
        b"REFR",
        FREE_BED_REF,
        BED,
        [0.0, 300.0, 0.0],
        "FreeBedRef",
        &[],
    );
    home.extend(thing(
        b"REFR",
        OWNED_BED_REF,
        BED,
        [100.0, 300.0, 0.0],
        "OwnedBedRef",
        &owned(OWNER),
    ));
    let mut town_bed = owned(TOWN);
    town_bed.extend(sub(b"XRNK", &5i32.to_le_bytes()));
    home.extend(thing(
        b"REFR",
        TOWN_BED_REF,
        BED,
        [200.0, 300.0, 0.0],
        "TownBedRef",
        &town_bed,
    ));
    home.extend(thing(
        b"REFR",
        SCRIPTED_BED_REF,
        SCRIPTED_BED,
        [300.0, 300.0, 0.0],
        "ScriptedBedRef",
        &[],
    ));
    home.extend(thing(
        b"REFR",
        CHAIR_REF,
        CHAIR,
        [400.0, 300.0, 0.0],
        "ChairRef",
        &owned(OWNER),
    ));
    home.extend(thing(
        b"ACHR",
        MARK_REF,
        OWNER,
        [0.0, 0.0, 0.0],
        "MarkRef",
        &[],
    ));
    home.extend(thing(
        b"ACHR",
        VILLAIN_REF,
        VILLAIN,
        [-200.0, 0.0, 0.0],
        "VillainRef",
        &[],
    ));
    home.extend(thing(
        b"ACHR",
        PAL_REF,
        PAL,
        [-400.0, 0.0, 0.0],
        "PalRef",
        &[],
    ));
    let mut inn = thing(
        b"ACHR",
        INN_OWNER_REF,
        OWNER,
        [0.0, 0.0, 0.0],
        "InnOwnerRef",
        &[],
    );
    inn.extend(thing(
        b"ACHR",
        INN_GUARD_REF,
        GUARD,
        [100.0, 0.0, 0.0],
        "InnGuardRef",
        &[],
    ));
    inn.extend(thing(
        b"ACHR",
        INN_STRANGER_REF,
        STRANGER,
        [-100.0, 0.0, 0.0],
        "InnStrangerRef",
        &[],
    ));
    inn.extend(thing(
        b"REFR",
        INN_BED_REF,
        BED,
        [0.0, 300.0, 0.0],
        "InnBedRef",
        &owned(PLAYER),
    ));
    let off_limits = thing(
        b"ACHR",
        OFF_LIMITS_GUARD_REF,
        GUARD,
        [0.0, 0.0, 0.0],
        "OffLimitsGuardRef",
        &[],
    );
    let barracks = thing(
        b"REFR",
        BARRACKS_BED_REF,
        BED,
        [0.0, 300.0, 0.0],
        "BarracksBedRef",
        &[],
    );
    let saloon = thing(
        b"ACHR",
        SALOON_OWNER_REF,
        OWNER,
        [0.0, 0.0, 0.0],
        "SaloonOwnerRef",
        &[],
    );
    let cell = |id: u32, name: &str, flags: u8, owner: Option<u32>, record_flags: u32| {
        let mut d = sub(b"DATA", &[flags]);
        if let Some(o) = owner {
            d.extend(owned(o));
        }
        with_flags(named(b"CELL", id, name, &d), record_flags)
    };
    let cell_group =
        |id: u32, refs: &[u8]| group(id.to_le_bytes(), 6, &group(id.to_le_bytes(), 9, refs));
    let mut contents = cell(HOME, "TestHome", 0x01, None, 0);
    contents.extend(cell_group(HOME, &home));
    contents.extend(cell(INN, "TestInn", 0x01, Some(OWNER), 0));
    contents.extend(cell_group(INN, &inn));
    contents.extend(cell(
        OFF_LIMITS,
        "TestOffLimits",
        0x01,
        Some(TOWN),
        0x2_0000,
    ));
    contents.extend(cell_group(OFF_LIMITS, &off_limits));
    contents.extend(cell(BARRACKS, "TestBarracks", 0x01, None, 0x8_0000));
    contents.extend(cell_group(BARRACKS, &barracks));
    contents.extend(cell(SALOON, "TestSaloon", 0x21, Some(OWNER), 0));
    contents.extend(cell_group(SALOON, &saloon));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &contents)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
}
