//! Crafting on a generated world (`testdata::crafting`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::crafting::ids::*;
use world::crafting::{self, Component, Recipe, ITEMS_CRAFTED, MOST};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::crafting::crafting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn give(order: &LoadOrder, state: &mut GameState, item: u32, n: i32) {
    state.stock(order, PLAYER_REF);
    *state.items.entry((PLAYER_REF, FormId(item))).or_insert(0) += n;
}

fn count(order: &LoadOrder, state: &GameState, item: u32) -> i32 {
    state.item_count(order, PLAYER_REF, FormId(item))
}

#[test]
fn recipes_read_their_data_and_components() {
    let (_data, order) = order("crafting-load");
    let r = Recipe::load(&order, FormId(R_STIMPAK)).unwrap();
    assert_eq!(r.name, "Stimpak");
    assert_eq!(r.skill, Some(37));
    assert_eq!(r.skill_level, 50);
    assert_eq!(r.category, Some(FormId(WORKBENCH)));
    assert_eq!(r.subcategory, Some(FormId(AID)));
    assert_eq!(
        r.ingredients,
        vec![
            Component {
                item: FormId(FLOWER),
                count: 2
            },
            Component {
                item: FormId(SCRAP),
                count: 1
            }
        ]
    );
    assert_eq!(
        r.outputs,
        vec![Component {
            item: FormId(STIMPAK),
            count: 1
        }]
    );
    // No skill: -1.
    assert_eq!(Recipe::load(&order, FormId(R_ROUNDS)).unwrap().skill, None);
    // An ingredient without RCQY is never added (TESRecipe::Load).
    let bloom = Recipe::load(&order, FormId(R_COUNTLESS)).unwrap();
    assert_eq!(
        bloom.ingredients,
        vec![Component {
            item: FormId(SCRAP),
            count: 1
        }]
    );
    assert_eq!(crafting::recipes(&order).len(), 7);
}

#[test]
fn how_many_can_be_made_follows_skill_ingredients_and_category() {
    let (_data, order) = order("crafting-can-make");
    let mut state = GameState::new(&order);
    let stimpak = Recipe::load(&order, FormId(R_STIMPAK)).unwrap();
    let workbench = FormId(WORKBENCH);
    give(&order, &mut state, FLOWER, 5);
    give(&order, &mut state, SCRAP, 3);
    // Medicine below 50: none.
    state.actor_values.insert((PLAYER_REF, 37), 49.0);
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &stimpak, workbench),
        0
    );
    // At 50: flowers 5 / 2 = 2, scrap 3 / 1 = 3: the fewer.
    state.actor_values.insert((PLAYER_REF, 37), 50.0);
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &stimpak, workbench),
        2
    );
    // From another category's menu: none.
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &stimpak, FormId(CAMPFIRE)),
        0
    );
    // A short ingredient: none.
    give(&order, &mut state, FLOWER, -4);
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &stimpak, workbench),
        0
    );
    // At most 100, however much there is.
    let rounds = Recipe::load(&order, FormId(R_ROUNDS)).unwrap();
    give(&order, &mut state, SCRAP, 500);
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &rounds, workbench),
        MOST
    );
    // No subcategory: never.
    let loose = Recipe::load(&order, FormId(R_LOOSE)).unwrap();
    assert_eq!(
        crafting::can_make(&order, &state, PLAYER_REF, &loose, workbench),
        0
    );
}

#[test]
fn the_list_shows_passing_recipes_makeable_first_then_by_name() {
    let (_data, order) = order("crafting-list");
    let mut state = GameState::new(&order);
    give(&order, &mut state, SCRAP, 2);
    give(&order, &mut state, CHIPS, 1);
    let workbench = FormId(WORKBENCH);
    let list = crafting::listing(&order, &state, PLAYER_REF, workbench, None);
    let names: Vec<(&str, u32)> = list
        .lines
        .iter()
        .map(|l| (l.text.as_str(), l.makeable))
        .collect();
    // The gun's condition (TestUnlocked = 1) fails: not listed. The
    // campfire's recipe is in another category. Makeable first (Bloom,
    // Grenade, Rounds), then the rest (Loose, Stimpak), each by name; the
    // rounds come ten at a time.
    assert_eq!(
        names,
        vec![
            ("Bloom", 2),
            ("Grenade", 1),
            ("Rounds (10)", 2),
            ("Loose (2)", 0),
            ("Stimpak", 0)
        ]
    );
    assert_eq!(list.makeable, 3);
    // The filter: the listed recipes' subcategories, by name.
    assert_eq!(list.subcategories, vec![FormId(AID), FormId(AMMO)]);
    // With the global set the gun is listed; narrowed to Ammo, only rounds.
    state.globals.insert(FormId(UNLOCKED), 1.0);
    let list = crafting::listing(&order, &state, PLAYER_REF, workbench, None);
    assert!(list.lines.iter().any(|l| l.recipe == FormId(R_GUN)));
    let ammo = crafting::listing(&order, &state, PLAYER_REF, workbench, Some(FormId(AMMO)));
    assert_eq!(ammo.lines.len(), 1);
    assert_eq!(ammo.lines[0].recipe, FormId(R_ROUNDS));
    assert_eq!(ammo.subcategories, vec![FormId(AID), FormId(AMMO)]);
}

#[test]
fn making_takes_ingredients_gives_outputs_and_counts_it() {
    let (_data, order) = order("crafting-make");
    let mut state = GameState::new(&order);
    give(&order, &mut state, SCRAP, 10);
    give(&order, &mut state, FLOWER, 4);
    state.actor_values.insert((PLAYER_REF, 37), 60.0);
    let stimpak = Recipe::load(&order, FormId(R_STIMPAK)).unwrap();
    let made = crafting::make(&order, &mut state, PLAYER_REF, &stimpak, 2).unwrap();
    assert_eq!(count(&order, &state, FLOWER), 0);
    assert_eq!(count(&order, &state, SCRAP), 8);
    assert_eq!(count(&order, &state, STIMPAK), 2);
    assert_eq!(made.notices, vec!["2 Stimpaks added".to_string()]);
    assert_eq!(state.misc_stats.get(&ITEMS_CRAFTED), Some(&2));

    // One output: "name added".
    let rounds = Recipe::load(&order, FormId(R_ROUNDS)).unwrap();
    let made = crafting::make(&order, &mut state, PLAYER_REF, &rounds, 1).unwrap();
    assert_eq!(made.notices, vec!["10 Rounds added".to_string()]);
    assert_eq!(count(&order, &state, ROUND), 10);
    assert_eq!(state.misc_stats.get(&ITEMS_CRAFTED), Some(&3));

    // Chips first: not counted as crafted. A grenade (thrown) comes whole.
    give(&order, &mut state, CHIPS, 1);
    let grenade = Recipe::load(&order, FormId(R_GRENADE)).unwrap();
    let made = crafting::make(&order, &mut state, PLAYER_REF, &grenade, 1).unwrap();
    assert_eq!(made.notices, vec!["Grenade added".to_string()]);
    assert_eq!(state.misc_stats.get(&ITEMS_CRAFTED), Some(&3));
    assert_eq!(
        state.weapon_health.get(&(PLAYER_REF, FormId(GRENADE))),
        None
    );

    // A gun comes at 80 % condition.
    let gun = Recipe::load(&order, FormId(R_GUN)).unwrap();
    crafting::make(&order, &mut state, PLAYER_REF, &gun, 1).unwrap();
    assert_eq!(
        state.weapon_health.get(&(PLAYER_REF, FormId(GUN))),
        Some(&0.8)
    );
    assert!(crafting::make(&order, &mut state, PLAYER_REF, &gun, 0).is_none());
}

/// The Dead Money contributor's crafting tests, on their own world
/// (`testdata::crafting::notes`), adapted to the traced implementation: the
/// list order and the filter's order are `00728c10` and `007278c0`'s (by
/// name), and whether a recipe can be made is `CanMakeRecipe`'s count.
mod dead_money {
    use esm::{ActivePlugins, FormId, LoadOrder};
    use testdata::crafting::notes::ids::*;
    use world::crafting::{self, Component, Recipe};
    use world::dialogue::PLAYER_REF;
    use world::scripting::{GameState, Runner, ScriptCache};

    fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
        let data = testdata::crafting::notes::crafting(tag);
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    fn give(order: &LoadOrder, state: &mut GameState, item: u32, n: i32) {
        state.stock(order, PLAYER_REF);
        *state.items.entry((PLAYER_REF, FormId(item))).or_insert(0) += n;
    }

    fn have(order: &LoadOrder, state: &GameState, item: u32) -> i32 {
        state.item_count(order, PLAYER_REF, FormId(item))
    }

    fn run(order: &LoadOrder, state: &mut GameState, source: &str) {
        let scripts = ScriptCache::default();
        Runner::new(order, &scripts, state).run_source(source, None, None);
    }

    fn names(order: &LoadOrder, state: &GameState, category: u32) -> Vec<String> {
        crafting::listing(order, state, PLAYER_REF, FormId(category), None)
            .lines
            .into_iter()
            .map(|l| l.name)
            .collect()
    }

    fn recipe(order: &LoadOrder, id: u32) -> Recipe {
        Recipe::load(order, FormId(id)).unwrap()
    }

    #[test]
    fn recipes_read_from_their_records() {
        let (_d, order) = order("craft-read");
        let r = recipe(&order, BULLET_RECIPE);
        assert_eq!(r.name, "Bullets");
        assert_eq!(r.skill, Some(40));
        assert_eq!(r.skill_level, 40);
        assert_eq!(
            (r.category, r.subcategory),
            (Some(FormId(BENCH)), Some(FormId(AMMO)))
        );
        let c = |item: u32, count: u32| Component {
            item: FormId(item),
            count,
        };
        assert_eq!(r.ingredients, vec![c(SCRAP, 1)]);
        assert_eq!(r.outputs, vec![c(BULLET, 3)]);
        let stew = recipe(&order, STEW_RECIPE);
        assert_eq!(stew.skill, None);
        assert_eq!(stew.ingredients, vec![c(SCRAP, 2), c(WATER, 1)]);
        assert_eq!(crafting::category_name(&order, FormId(BENCH)), "Workbench");
        assert!(Recipe::load(&order, FormId(SCRAP)).is_none());
        assert_eq!(
            world::stats::NAMES[usize::from(crafting::ITEMS_CRAFTED)],
            "Items Crafted"
        );
    }

    #[test]
    fn the_menu_lists_a_categorys_known_recipes_by_name() {
        let (_d, order) = order("craft-list");
        let mut state = GameState::new(&order);
        // The secret recipe waits for its note.
        assert_eq!(names(&order, &state, BENCH), ["Bullets", "Stew"]);
        assert_eq!(names(&order, &state, FIRE), ["Boiled Water"]);
        run(&order, &mut state, "player.AddNote TestRecipeNote");
        assert_eq!(
            names(&order, &state, BENCH),
            ["Bullets", "Secret Stew", "Stew"]
        );
        // The filter's subcategories, by name (`007278c0`).
        let list = crafting::listing(&order, &state, PLAYER_REF, FormId(BENCH), None);
        assert_eq!(list.subcategories, [FormId(AID), FormId(AMMO)]);
    }

    #[test]
    fn making_asks_for_the_skill_and_every_ingredient() {
        let (_d, order) = order("craft-make");
        let mut state = GameState::new(&order);
        let bench = FormId(BENCH);
        let stew = recipe(&order, STEW_RECIPE);
        let can =
            |state: &GameState, r: &Recipe| crafting::can_make(&order, state, PLAYER_REF, r, bench);
        run(&order, &mut state, "player.ForceActorValue Science 39");
        // Nothing yet.
        assert_eq!(can(&state, &stew), 0);
        give(&order, &mut state, SCRAP, 2);
        assert_eq!(can(&state, &stew), 0);
        give(&order, &mut state, WATER, 1);
        assert_eq!(can(&state, &stew), 1);
        // Makeable first (`00728c10`); the bullets lack the skill.
        assert_eq!(names(&order, &state, BENCH), ["Stew", "Bullets"]);
        crafting::make(&order, &mut state, PLAYER_REF, &stew, 1).unwrap();
        assert_eq!(have(&order, &state, SCRAP), 0);
        assert_eq!(have(&order, &state, WATER), 0);
        assert_eq!(have(&order, &state, STEW), 1);
        assert_eq!(state.misc_stats.get(&crafting::ITEMS_CRAFTED), Some(&1));

        // Bullets want Science 40, however much scrap there is.
        let bullets = recipe(&order, BULLET_RECIPE);
        give(&order, &mut state, SCRAP, 5);
        assert_eq!(can(&state, &bullets), 0);
        run(&order, &mut state, "player.ForceActorValue Science 40");
        assert_eq!(can(&state, &bullets), 5);
        crafting::make(&order, &mut state, PLAYER_REF, &bullets, 1).unwrap();
        assert_eq!(have(&order, &state, BULLET), 3);
        assert_eq!(have(&order, &state, SCRAP), 4);
        assert_eq!(state.misc_stats.get(&crafting::ITEMS_CRAFTED), Some(&2));
    }

    #[test]
    fn unknown_recipes_and_other_forms_are_not_offered() {
        let (_d, order) = order("craft-refuse");
        let mut state = GameState::new(&order);
        give(&order, &mut state, WATER, 1);
        let listed = |state: &GameState| {
            crafting::listing(&order, state, PLAYER_REF, FormId(BENCH), None)
                .lines
                .iter()
                .any(|l| l.recipe == FormId(SECRET_RECIPE))
        };
        // Not listed without its note, so the menu can't make it.
        assert!(!listed(&state));
        assert!(Recipe::load(&order, FormId(WATER)).is_none());
        // A campfire recipe from the workbench's menu: none.
        give(&order, &mut state, SCRAP, 1);
        let fire = recipe(&order, FIRE_RECIPE);
        assert_eq!(
            crafting::can_make(&order, &state, PLAYER_REF, &fire, FormId(BENCH)),
            0
        );
        run(&order, &mut state, "player.AddNote TestRecipeNote");
        assert!(listed(&state));
        let secret = recipe(&order, SECRET_RECIPE);
        crafting::make(&order, &mut state, PLAYER_REF, &secret, 1).unwrap();
        assert_eq!(have(&order, &state, STEW), 1);
        assert_eq!(have(&order, &state, WATER), 0);
    }
}
