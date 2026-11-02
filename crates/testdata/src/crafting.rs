//! A world for `world::crafting`: a recipe category with two subcategories,
//! another category, items (an aid item, two ingredients, casino chips, a
//! gun, a grenade), a global, the "added" settings, and recipes covering
//! the rules: a skill to meet, a condition, an output made several at a
//! time, chips as the first ingredient, an ingredient without a count, a
//! recipe without a subcategory, and recipes in the other category.

use crate::{group, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const WORKBENCH: u32 = 0xF00;
    /// Subcategory "Aid".
    pub const AID: u32 = 0xF01;
    /// Subcategory "Ammo".
    pub const AMMO: u32 = 0xF02;
    pub const CAMPFIRE: u32 = 0xF03;
    pub const STIMPAK: u32 = 0xF10;
    pub const FLOWER: u32 = 0xF11;
    pub const SCRAP: u32 = 0xF12;
    pub const CHIPS: u32 = 0xF13;
    /// Animation type 3 (a pistol).
    pub const GUN: u32 = 0xF14;
    /// Animation type 10 (thrown grenade).
    pub const GRENADE: u32 = 0xF15;
    pub const ROUND: u32 = 0xF16;
    /// Read by a recipe's condition (`GetGlobalValue` = 1).
    pub const UNLOCKED: u32 = 0xF20;
    /// Stimpak: Medicine (37) 50; 2 flowers + 1 scrap; aid.
    pub const R_STIMPAK: u32 = 0xF30;
    /// Rounds: no skill; 1 scrap makes 10; ammo.
    pub const R_ROUNDS: u32 = 0xF31;
    /// Gun: no skill; 3 scrap; aid; only when `TestUnlocked` is 1.
    pub const R_GUN: u32 = 0xF32;
    /// Grenade: 1 chips (first) + 1 scrap; aid.
    pub const R_GRENADE: u32 = 0xF33;
    /// A flower whose `RCIL` has no `RCQY` (never an ingredient), and 1
    /// scrap; makes a flower; aid. Named "Bloom".
    pub const R_COUNTLESS: u32 = 0xF34;
    /// No subcategory: never makeable.
    pub const R_LOOSE: u32 = 0xF35;
    /// Campfire: 1 flower makes a stimpak.
    pub const R_CAMP: u32 = 0xF36;
}

fn le(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn crafting(tag: &str) -> TempData {
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
    // The notice's words.
    let gmst =
        |id: u32, name: &str, text: &str| named(b"GMST", id, name, &sub(b"DATA", &zstr(text)));
    let mut gmsts = gmst(0xF40, "sAddItemtoInventory", "added");
    gmsts.extend(gmst(0xF41, "sPlural", "s"));
    plugin.extend(group(*b"GMST", 0, &gmsts));
    let mut g = sub(b"FNAM", b"f");
    g.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    plugin.extend(group(
        *b"GLOB",
        0,
        &named(b"GLOB", UNLOCKED, "TestUnlocked", &g),
    ));

    let full = |name: &str| sub(b"FULL", &zstr(name));
    let mut misc = named(b"MISC", FLOWER, "TestFlower", &full("Flower"));
    misc.extend(named(b"MISC", SCRAP, "TestScrap", &full("Scrap")));
    plugin.extend(group(*b"MISC", 0, &misc));
    plugin.extend(group(
        *b"CHIP",
        0,
        &named(b"CHIP", CHIPS, "TestChips", &full("Chips")),
    ));
    plugin.extend(group(
        *b"ALCH",
        0,
        &named(b"ALCH", STIMPAK, "TestStimpak", &full("Stimpak")),
    ));
    plugin.extend(group(
        *b"AMMO",
        0,
        &named(b"AMMO", ROUND, "TestRound", &full("Round")),
    ));
    let weapon = |id: u32, editor: &str, name: &str, anim: u8| {
        let mut d = full(name);
        let mut dnam = vec![anim];
        dnam.extend([0u8; 203]);
        d.extend(sub(b"DNAM", &dnam));
        named(b"WEAP", id, editor, &d)
    };
    let mut weapons = weapon(GUN, "TestGun", "Gun", 3);
    weapons.extend(weapon(GRENADE, "TestGrenade", "Grenade", 10));
    plugin.extend(group(*b"WEAP", 0, &weapons));

    let category = |id: u32, editor: &str, name: &str| {
        let mut d = full(name);
        d.extend(sub(b"DATA", &[0]));
        named(b"RCCT", id, editor, &d)
    };
    let mut cats = category(WORKBENCH, "TestWorkbench", "Workbench");
    cats.extend(category(AID, "TestAid", "Aid"));
    cats.extend(category(AMMO, "TestAmmo", "Ammo"));
    cats.extend(category(CAMPFIRE, "TestCampfire", "Campfire"));
    plugin.extend(group(*b"RCCT", 0, &cats));

    // DATA: skill (-1 none), level, category, subcategory.
    let data_of = |skill: i32, level: i32, cat: u32, subcat: u32| {
        let mut d = skill.to_le_bytes().to_vec();
        d.extend(level.to_le_bytes());
        d.extend(le(cat));
        d.extend(le(subcat));
        sub(b"DATA", &d)
    };
    let part = |kind: &[u8; 4], item: u32, n: Option<u32>| {
        let mut d = sub(kind, &le(item));
        if let Some(n) = n {
            d.extend(sub(b"RCQY", &le(n)));
        }
        d
    };
    let recipe = |id: u32, editor: &str, name: &str, body: Vec<u8>| {
        let mut d = full(name);
        d.extend(body);
        named(b"RCPE", id, editor, &d)
    };
    let mut recipes = Vec::new();
    recipes.extend(recipe(R_STIMPAK, "TestRecipeStimpak", "Stimpak", {
        let mut b = data_of(37, 50, WORKBENCH, AID);
        b.extend(part(b"RCIL", FLOWER, Some(2)));
        b.extend(part(b"RCIL", SCRAP, Some(1)));
        b.extend(part(b"RCOD", STIMPAK, Some(1)));
        b
    }));
    recipes.extend(recipe(R_ROUNDS, "TestRecipeRounds", "Rounds", {
        let mut b = data_of(-1, 0, WORKBENCH, AMMO);
        b.extend(part(b"RCIL", SCRAP, Some(1)));
        b.extend(part(b"RCOD", ROUND, Some(10)));
        b
    }));
    recipes.extend(recipe(R_GUN, "TestRecipeGun", "Gun", {
        // CTDA: compare "==" (0), value 1.0, GetGlobalValue (74) on the
        // subject, parameter the global.
        let mut c = vec![0u8, 0, 0, 0];
        c.extend(1.0f32.to_le_bytes());
        c.extend(74u16.to_le_bytes());
        c.extend([0, 0]);
        c.extend(le(UNLOCKED));
        c.extend(le(0));
        c.extend(le(0));
        c.extend(le(0));
        let mut b = sub(b"CTDA", &c);
        b.extend(data_of(-1, 0, WORKBENCH, AID));
        b.extend(part(b"RCIL", SCRAP, Some(3)));
        b.extend(part(b"RCOD", GUN, Some(1)));
        b
    }));
    recipes.extend(recipe(R_GRENADE, "TestRecipeGrenade", "Grenade", {
        let mut b = data_of(-1, 0, WORKBENCH, AID);
        b.extend(part(b"RCIL", CHIPS, Some(1)));
        b.extend(part(b"RCIL", SCRAP, Some(1)));
        b.extend(part(b"RCOD", GRENADE, Some(1)));
        b
    }));
    recipes.extend(recipe(R_COUNTLESS, "TestRecipeCountless", "Bloom", {
        let mut b = data_of(-1, 0, WORKBENCH, AID);
        b.extend(part(b"RCIL", FLOWER, None));
        b.extend(part(b"RCIL", SCRAP, Some(1)));
        b.extend(part(b"RCOD", FLOWER, Some(1)));
        b
    }));
    recipes.extend(recipe(R_LOOSE, "TestRecipeLoose", "Loose", {
        let mut b = data_of(-1, 0, WORKBENCH, 0);
        b.extend(part(b"RCIL", SCRAP, Some(1)));
        b.extend(part(b"RCOD", SCRAP, Some(2)));
        b
    }));
    recipes.extend(recipe(R_CAMP, "TestRecipeCamp", "Camp Stimpak", {
        let mut b = data_of(-1, 0, CAMPFIRE, AID);
        b.extend(part(b"RCIL", FLOWER, Some(1)));
        b.extend(part(b"RCOD", STIMPAK, Some(1)));
        b
    }));
    plugin.extend(group(*b"RCPE", 0, &recipes));
    data.write("FalloutNV.esm", &plugin);
    data
}

/// The Dead Money contributor's crafting world, kept for its tests: a
/// workbench and a campfire category with Aid and Ammo subcategories, four
/// recipes (one without a skill, one that asks Science 40, one listed only
/// for those who hold its note (`GetHasNote`), one at the campfire), the
/// items they use and the note.
pub mod notes {
    use crate::{group, record, sub, zstr, TempData};

    pub mod ids {
        pub const SCRAP: u32 = 0xC01;
        pub const WATER: u32 = 0xC02;
        pub const STEW: u32 = 0xC03;
        pub const BULLET: u32 = 0xC04;
        pub const NOTE: u32 = 0xC05;
        pub const BENCH: u32 = 0xC10;
        pub const FIRE: u32 = 0xC11;
        pub const AID: u32 = 0xC12;
        pub const AMMO: u32 = 0xC13;
        /// No skill: 2 scrap + 1 water make 1 stew (bench, Aid).
        pub const STEW_RECIPE: u32 = 0xC20;
        /// Science 40: 1 scrap makes 3 bullets (bench, Ammo).
        pub const BULLET_RECIPE: u32 = 0xC21;
        /// Listed only with the note: 1 water makes 1 stew (bench, Aid).
        pub const SECRET_RECIPE: u32 = 0xC22;
        /// At the campfire: 1 scrap makes 1 water (Aid).
        pub const FIRE_RECIPE: u32 = 0xC23;
    }

    /// The actor value number of Science (the script language's list).
    pub const SCIENCE: u32 = 40;

    /// The function number of `GetHasNote`.
    const GET_HAS_NOTE: u16 = 382;

    /// The world, written as `FalloutNV.esm` into a temporary Data folder.
    pub fn crafting(tag: &str) -> TempData {
        use ids::*;
        let data = TempData::new(tag);
        let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
            let mut d = sub(b"EDID", &zstr(name));
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
        let mut misc = Vec::new();
        for (id, edid, name) in [
            (SCRAP, "TestScrap", "Scrap"),
            (WATER, "TestWater", "Water"),
            (STEW, "TestStew", "Stew"),
            (BULLET, "TestBullet", "Bullet"),
        ] {
            misc.extend(named(b"MISC", id, edid, &sub(b"FULL", &zstr(name))));
        }
        plugin.extend(group(*b"MISC", 0, &misc));
        plugin.extend(group(
            *b"NOTE",
            0,
            &named(
                b"NOTE",
                NOTE,
                "TestRecipeNote",
                &sub(b"FULL", &zstr("Secret")),
            ),
        ));
        let category = |id: u32, edid: &str, name: &str| {
            let mut d = sub(b"FULL", &zstr(name));
            d.extend(sub(b"DATA", &[0]));
            named(b"RCCT", id, edid, &d)
        };
        let mut rcct = category(BENCH, "TestBench", "Workbench");
        rcct.extend(category(FIRE, "TestFire", "Campfire"));
        rcct.extend(category(AID, "TestAid", "Aid"));
        rcct.extend(category(AMMO, "TestAmmo", "Ammo"));
        plugin.extend(group(*b"RCCT", 0, &rcct));

        type Named = dyn Fn(&[u8; 4], u32, &str, &[u8]) -> Vec<u8>;
        #[allow(clippy::too_many_arguments)]
        fn recipe(
            named: &Named,
            id: u32,
            edid: &str,
            name: &str,
            (skill, level, cat, subcat): (u32, u32, u32, u32),
            inputs: &[(u32, i32)],
            outputs: &[(u32, i32)],
            condition: Option<&[u8]>,
        ) -> Vec<u8> {
            let mut d = sub(b"FULL", &zstr(name));
            if let Some(c) = condition {
                d.extend(sub(b"CTDA", c));
            }
            let mut dat = skill.to_le_bytes().to_vec();
            for v in [level, cat, subcat] {
                dat.extend(v.to_le_bytes());
            }
            d.extend(sub(b"DATA", &dat));
            for (item, n) in inputs {
                d.extend(sub(b"RCIL", &item.to_le_bytes()));
                d.extend(sub(b"RCQY", &n.to_le_bytes()));
            }
            for (item, n) in outputs {
                d.extend(sub(b"RCOD", &item.to_le_bytes()));
                d.extend(sub(b"RCQY", &n.to_le_bytes()));
            }
            named(b"RCPE", id, edid, &d)
        }
        // `GetHasNote Note == 1`.
        let has_note = {
            let mut c = vec![0u8; 4];
            c.extend(1.0f32.to_le_bytes());
            c.extend(GET_HAS_NOTE.to_le_bytes());
            c.extend([0, 0]);
            c.extend(NOTE.to_le_bytes());
            c.extend([0; 12]);
            c
        };
        let none = u32::MAX;
        let mut rcpe = recipe(
            &named,
            STEW_RECIPE,
            "TestStewRecipe",
            "Stew",
            (none, 0, BENCH, AID),
            &[(SCRAP, 2), (WATER, 1)],
            &[(STEW, 1)],
            None,
        );
        rcpe.extend(recipe(
            &named,
            BULLET_RECIPE,
            "TestBulletRecipe",
            "Bullets",
            (SCIENCE, 40, BENCH, AMMO),
            &[(SCRAP, 1)],
            &[(BULLET, 3)],
            None,
        ));
        rcpe.extend(recipe(
            &named,
            SECRET_RECIPE,
            "TestSecretRecipe",
            "Secret Stew",
            (none, 0, BENCH, AID),
            &[(WATER, 1)],
            &[(STEW, 1)],
            Some(&has_note),
        ));
        rcpe.extend(recipe(
            &named,
            FIRE_RECIPE,
            "TestFireRecipe",
            "Boiled Water",
            (none, 0, FIRE, AID),
            &[(SCRAP, 1)],
            &[(WATER, 1)],
            None,
        ));
        plugin.extend(group(*b"RCPE", 0, &rcpe));
        data.write("FalloutNV.esm", &plugin);
        data
    }
}
