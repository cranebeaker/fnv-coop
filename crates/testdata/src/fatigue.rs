//! Fatigue, knock-outs, `ForceFlee` and combat groups (`world::fatigue`,
//! `world::ai::flee::force`, `world::combat_groups`): the player and
//! people with `FalloutNV.esm`'s fatigue values (`ACBS` u16 at 4: the
//! player 200, people 50, `GSSunnySmiles`' flags 0x11), a creature, a
//! person who is never knocked down (`ACBS` flag 0x4000000), and the bean
//! bag round's effects (`AmmoEffectBeanBagFatigue` kind 5 add 250,
//! `AmmoEffectBeanBagDamageReduction` damage × 0.05) on a shotgun.

use crate::{group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`fatigue`] world.
pub mod ids {
    /// The player's base: `ACBS` fatigue 200, health 100, SPECIAL 5.
    pub const PLAYER: u32 = 0x7;
    /// A person: fatigue 50, SPECIAL 5, level 1.
    pub const PERSON: u32 = 0xE01;
    /// A person whose stats the game works out (`ACBS` flags 0x10):
    /// fatigue 50, Endurance 5, level 3.
    pub const AUTO: u32 = 0xE02;
    /// A person never knocked down (`ACBS` flag 0x4000000), fatigue 50.
    pub const STURDY: u32 = 0xE03;
    /// A creature: `ACBS` fatigue 100, bite damage 5.
    pub const CREATURE: u32 = 0xE04;
    /// A second person, the first's friend (both in the faction, which is
    /// its own ally), helping friends and allies (`AIDT` assistance 2).
    pub const FRIEND: u32 = 0xE05;
    pub const FACTION: u32 = 0xE06;
    pub const CELL: u32 = 0xE10;
    pub const PERSON_REF: u32 = 0xE11;
    pub const AUTO_REF: u32 = 0xE12;
    pub const STURDY_REF: u32 = 0xE13;
    pub const CREATURE_REF: u32 = 0xE14;
    pub const FRIEND_REF: u32 = 0xE15;
    /// A marker to flee to.
    pub const MARKER_REF: u32 = 0xE16;
    /// A shotgun firing bean bags.
    pub const SHOTGUN: u32 = 0xE20;
    pub const BEAN_BAG: u32 = 0xE21;
    pub const BEAN_BAG_FATIGUE: u32 = 0xE22;
    pub const BEAN_BAG_DAMAGE: u32 = 0xE23;
    pub const SETTINGS: u32 = 0xE30;
}

/// The [`ids`] world in a temporary Data folder.
pub fn fatigue(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    // `FalloutNV.esm`'s values for the fists' damage; the fatigue settings
    // stay the exe's.
    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fAVDUnarmedDamageBase", 0.5f32),
        ("fAVDUnarmedDamageMult", 0.05),
        ("fMinDamMultiplier", 0.2),
        ("fDamageSkillBase", 0.5),
        ("fDamageSkillMult", 0.5),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", SETTINGS + i as u32, &d));
    }

    // People: ACBS (flags, fatigue, barter, level ...), AIDT, DATA
    // (health, SPECIAL).
    let npc = |id: u32, name: &str, flags: u32, fatigue: u16, level: i16, faction: bool| {
        let mut d = edid(name);
        let mut acbs = flags.to_le_bytes().to_vec();
        acbs.extend(fatigue.to_le_bytes());
        acbs.extend(0u16.to_le_bytes());
        acbs.extend(level.to_le_bytes());
        acbs.extend([0; 14]);
        d.extend(sub(b"ACBS", &acbs));
        if faction {
            let mut snam = FACTION.to_le_bytes().to_vec();
            snam.extend([0, 0, 0, 0]);
            d.extend(sub(b"SNAM", &snam));
        }
        let mut aidt = vec![0u8; 20];
        aidt[14] = 2;
        d.extend(sub(b"AIDT", &aidt));
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend([5; 7]);
        d.extend(sub(b"DATA", &data));
        record(b"NPC_", id, &d)
    };
    let mut npcs = npc(PLAYER, "Player", 0, 200, 1, false);
    npcs.extend(npc(PERSON, "TestFatiguePerson", 0, 50, 1, true));
    npcs.extend(npc(AUTO, "TestFatigueAuto", 0x10, 50, 3, false));
    npcs.extend(npc(STURDY, "TestFatigueSturdy", 0x0400_0000, 50, 1, false));
    npcs.extend(npc(FRIEND, "TestFatigueFriend", 0, 50, 1, true));

    let mut creature = edid("TestFatigueCreature");
    let mut acbs = 0u32.to_le_bytes().to_vec();
    acbs.extend(100u16.to_le_bytes());
    acbs.extend([0; 18]);
    creature.extend(sub(b"ACBS", &acbs));
    creature.extend(sub(b"AIDT", &[0; 20]));
    let mut cdata = vec![0u8, 40, 50, 50];
    cdata.extend(20i16.to_le_bytes());
    cdata.extend([0, 0]);
    cdata.extend(5i16.to_le_bytes());
    cdata.extend([5; 7]);
    creature.extend(sub(b"DATA", &cdata));
    let creatures = record(b"CREA", CREATURE, &creature);

    // The faction: its own ally (XNAM: faction, modifier, combat
    // reaction 2 = ally).
    let mut fact = edid("TestFatigueFaction");
    let mut xnam = FACTION.to_le_bytes().to_vec();
    xnam.extend(0i32.to_le_bytes());
    xnam.extend(2u32.to_le_bytes());
    fact.extend(sub(b"XNAM", &xnam));
    fact.extend(sub(b"DATA", &[0, 0, 0, 0]));
    let factions = record(b"FACT", FACTION, &fact);

    // The bean bag: two ammunition effects (AMEF DATA kind, operation,
    // value), the round (RCIL), a shotgun firing it (NAM0; DATA value,
    // health, weight, damage 55, clip; DNAM animation 6 (two-handed
    // rifle), skill Guns 41).
    let effect = |id: u32, name: &str, kind: u32, op: u32, value: f32| {
        let mut d = edid(name);
        let mut data = kind.to_le_bytes().to_vec();
        data.extend(op.to_le_bytes());
        data.extend(value.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"AMEF", id, &d)
    };
    let mut ammo_effects = effect(BEAN_BAG_FATIGUE, "AmmoEffectBeanBagFatigue", 5, 0, 250.0);
    ammo_effects.extend(effect(
        BEAN_BAG_DAMAGE,
        "AmmoEffectBeanBagDamageReduction",
        0,
        1,
        0.05,
    ));
    let mut bag = edid("TestBeanBag");
    bag.extend(sub(b"RCIL", &BEAN_BAG_FATIGUE.to_le_bytes()));
    bag.extend(sub(b"RCIL", &BEAN_BAG_DAMAGE.to_le_bytes()));
    let ammunition = record(b"AMMO", BEAN_BAG, &bag);
    let mut gun = edid("TestBeanBagShotgun");
    gun.extend(sub(b"FULL", &zstr("Bean Bag Shotgun")));
    gun.extend(sub(b"NAM0", &BEAN_BAG.to_le_bytes()));
    let mut wdata = 100i32.to_le_bytes().to_vec();
    wdata.extend(250i32.to_le_bytes());
    wdata.extend(5.0f32.to_le_bytes());
    wdata.extend(55i16.to_le_bytes());
    wdata.push(2);
    gun.extend(sub(b"DATA", &wdata));
    let mut dnam = vec![0u8; 204];
    dnam[0..4].copy_from_slice(&6u32.to_le_bytes());
    dnam[4..8].copy_from_slice(&1.0f32.to_le_bytes());
    dnam[104..108].copy_from_slice(&41u32.to_le_bytes());
    dnam[116..120].copy_from_slice(&1.0f32.to_le_bytes());
    dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
    gun.extend(sub(b"DNAM", &dnam));
    let weapons = record(b"WEAP", SHOTGUN, &gun);

    let actor = |id: u32, base: u32, at: [f32; 3], name: &str| {
        let mut r = placed(id, base, at, [0.0; 3], &edid(name));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(
        PERSON_REF,
        PERSON,
        [0.0, 100.0, 0.0],
        "TestFatiguePersonRef",
    );
    refs.extend(actor(
        AUTO_REF,
        AUTO,
        [100.0, 0.0, 0.0],
        "TestFatigueAutoRef",
    ));
    refs.extend(actor(
        STURDY_REF,
        STURDY,
        [-100.0, 0.0, 0.0],
        "TestFatigueSturdyRef",
    ));
    let mut c = placed(
        CREATURE_REF,
        CREATURE,
        [0.0, -100.0, 0.0],
        [0.0; 3],
        &edid("TestFatigueCreatureRef"),
    );
    c[..4].copy_from_slice(b"ACRE");
    refs.extend(c);
    refs.extend(actor(
        FRIEND_REF,
        FRIEND,
        [0.0, 200.0, 0.0],
        "TestFatigueFriendRef",
    ));
    refs.extend(placed(
        MARKER_REF,
        0x34,
        [500.0, 500.0, 0.0],
        [0.0; 3],
        &edid("TestFatigueMarkerRef"),
    ));
    let mut cell = edid("TestFatigueCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GMST", 0, &settings));
    plugin.extend(group(*b"FACT", 0, &factions));
    plugin.extend(group(*b"AMEF", 0, &ammo_effects));
    plugin.extend(group(*b"AMMO", 0, &ammunition));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"CREA", 0, &creatures));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}
