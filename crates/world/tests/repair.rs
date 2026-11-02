//! Repairing (`world::repair`): the Pip-Boy's mending and merchants'
//! repairs, on the test world (the game data's `fRepairSkillMax` 10 and
//! `fItemRepairCostMult` 2; the other settings the exe's).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::dialogue::PLAYER_REF;
use world::repair::{self, ServiceLine};
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

fn give(order: &LoadOrder, state: &mut GameState, item: u32, n: i32) {
    state.stock(order, PLAYER_REF);
    *state.items.entry((PLAYER_REF, FormId(item))).or_insert(0) += n;
}

/// `00648090`: 10 × (hi + `fRepairMin` 0.5 + (`fRepairMax` 2 − 0.5) ×
/// Repair / 100 + lo × `fRepairScavengeMult` 0.05), hi and lo the two
/// conditions / 10; capped at 1 only as a condition. `007b7f70`: a
/// vendor mends to (4 + (10 − 4) × Repair / 100) / 10. `00648230`: 2 ×
/// value × (that × 10 − condition / 10) / 10, rounded, none at or below
/// 0.5.
#[test]
fn the_formulas() {
    let (_data, order) = order("repair-formulas");
    // 6 + 0.5 + 0.75 + 4 × 0.05 = 7.45.
    assert!(close(repair::mended(&order, 50, 40.0, 60.0), 74.5));
    assert!(close(repair::mended(&order, 50, 60.0, 40.0), 74.5));
    // Nothing caps it but the condition: 9 + 0.5 + 1.5 + 0.45 = 11.45.
    assert!(close(repair::mended(&order, 100, 90.0, 90.0), 114.5));
    assert_eq!(repair::mended_condition(&order, 100, 90.0, 90.0), 1.0);
    assert!(close(repair::mended_condition(&order, 0, 10.0, 0.0), 0.15));

    assert!(close(repair::service_target(&order, 0), 0.4));
    assert!(close(repair::service_target(&order, 50), 0.7));
    assert_eq!(repair::service_target(&order, 100), 1.0);

    // 2 × 100 × (7 − 3) / 10.
    assert_eq!(repair::service_cost(&order, 50, 30.0, 100.0), 80);
    assert_eq!(repair::service_cost(&order, 50, 68.0, 100.0), 4);
    // At the target or above, or worth half a cap or less: they can't.
    assert_eq!(repair::service_cost(&order, 50, 70.0, 100.0), 0);
    assert_eq!(repair::service_cost(&order, 50, 90.0, 100.0), 0);
    assert_eq!(repair::service_cost(&order, 50, 69.9, 100.0), 0);
    // 2 × 100 × 0.0035 = 0.7: one cap.
    assert_eq!(repair::service_cost(&order, 50, 69.65, 100.0), 1);

    // Armour's figures at a condition (`00646d40`).
    assert_eq!(repair::armour_at(10.0, 0.8), 10.0);
    assert!(close(repair::armour_at(10.0, 0.3), 8.0));
}

fn names(lines: &[ServiceLine]) -> Vec<(String, i32, bool)> {
    lines
        .iter()
        .map(|l| (l.name.clone(), l.cost, l.can_repair))
        .collect()
}

/// `007b7b40`, `007b7920`, `007b7c40`: the player's damaged weapons below
/// 100%, each with its cost at the vendor's Repair; what can be paid for
/// first, the most health first; the rest after, the least health first.
#[test]
fn a_merchants_list_and_order() {
    let (_data, order) = order("repair-list");
    let mut state = GameState::new(&order);
    let caps = world::barter::caps(&order);
    give(&order, &mut state, PISTOL, 1);
    give(&order, &mut state, REVOLVER, 1);
    give(&order, &mut state, RIFLE, 1);
    state.items.insert((PLAYER_REF, caps), 100);
    // Pistol 30% of 150 (45 health), revolver 50% of 100 (50), rifle 80%.
    state
        .weapon_health
        .insert((PLAYER_REF, FormId(PISTOL)), 0.3);
    state
        .weapon_health
        .insert((PLAYER_REF, FormId(REVOLVER)), 0.5);
    state.weapon_health.insert((PLAYER_REF, FormId(RIFLE)), 0.8);

    let (lines, total) = repair::service_lines(&order, &state, 50);
    // Revolver: 2 × 50 × (7 − 5) / 10 = 20; pistol 80; the rifle past
    // what they mend to: 0, and last.
    assert_eq!(
        names(&lines),
        vec![
            ("Revolver".into(), 20, true),
            ("Pistol".into(), 80, true),
            ("Rifle".into(), 0, false),
        ]
    );
    assert_eq!(total, 100);
    assert!(close(lines[1].health, 45.0));

    // With 50 caps the pistol can't be paid for: among those that can't
    // be, the least health first (pistol 45, rifle 200).
    state.items.insert((PLAYER_REF, caps), 50);
    let (lines, _) = repair::service_lines(&order, &state, 50);
    assert_eq!(
        names(&lines),
        vec![
            ("Revolver".into(), 20, true),
            ("Pistol".into(), 80, false),
            ("Rifle".into(), 0, false),
        ]
    );
    // The rifle at 10% (25 health): before the pistol now.
    state.weapon_health.insert((PLAYER_REF, FormId(RIFLE)), 0.1);
    let (lines, _) = repair::service_lines(&order, &state, 50);
    assert_eq!(lines[1].name, "Rifle");

    // Whole weapons aren't listed.
    state.weapon_health.remove(&(PLAYER_REF, FormId(RIFLE)));
    let (lines, _) = repair::service_lines(&order, &state, 50);
    assert!(lines.iter().all(|l| l.item != FormId(RIFLE)));
}

/// `ShowRepairMenu` on a person (`005d5200`), and a repair (`007b7f70`):
/// the weapon at the vendor's target, the caps paid to the vendor
/// (`008924e0`).
#[test]
fn a_merchant_repairs_for_caps() {
    let (_data, order) = order("repair-merchant");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let caps = world::barter::caps(&order);
    state.events.clear();
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.ShowRepairMenu\nChestRef.ShowRepairMenu",
        None,
        None,
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
    assert_eq!(state.events, vec![Event::RepairServices(FormId(DOC_REF))]);

    state
        .actor_values
        .insert((FormId(DOC_REF), repair::REPAIR), 50.0);
    assert_eq!(repair::skill(&order, &state, FormId(DOC_REF)), 50);
    give(&order, &mut state, PISTOL, 1);
    state.items.insert((PLAYER_REF, caps), 100);
    state
        .weapon_health
        .insert((PLAYER_REF, FormId(PISTOL)), 0.3);
    let doc_caps = state.item_count(&order, FormId(DOC_REF), caps);
    repair::repair_by(&order, &mut state, FormId(DOC_REF), FormId(PISTOL), 80);
    assert!(close(
        world::combat::weapon_condition(&state, PLAYER_REF, FormId(PISTOL)),
        0.7
    ));
    assert_eq!(state.item_count(&order, PLAYER_REF, caps), 20);
    assert_eq!(
        state.item_count(&order, FormId(DOC_REF), caps),
        doc_caps + 80
    );
}

/// `0047bb50`: the same item, its repair list's, and with Jury Rigging
/// something alike (same kind, skill and animation type) — but only for
/// an item that has a repair list. `00781860`, `007b6aa0`, `007b5d80`.
#[test]
fn the_pipboys_repair() {
    let (_data, order) = order("repair-pipboy");
    let mut state = GameState::new(&order);
    let (pistol, rifle, revolver) = (FormId(PISTOL), FormId(RIFLE), FormId(REVOLVER));
    assert_eq!(
        repair::repair_list(&order, pistol),
        Some(FormId(REPAIR_LIST))
    );
    assert!(repair::mends(&order, &state, pistol, pistol));
    assert!(repair::mends(&order, &state, pistol, rifle));
    assert!(!repair::mends(&order, &state, pistol, revolver));
    state.perks.insert(FormId(JURY_RIGGING));
    assert!(repair::mends(&order, &state, pistol, revolver));
    // The rifle has no repair list: only rifles mend it.
    assert!(!repair::mends(&order, &state, rifle, pistol));
    state.perks.remove(&FormId(JURY_RIGGING));

    state
        .actor_values
        .insert((PLAYER_REF, repair::REPAIR), 50.0);
    give(&order, &mut state, PISTOL, 1);
    state.weapon_health.insert((PLAYER_REF, pistol), 0.3);
    // One damaged pistol and nothing to mend it with.
    assert!(!repair::can_repair(&order, &state, pistol));
    give(&order, &mut state, RIFLE, 1);
    state.weapon_health.insert((PLAYER_REF, rifle), 0.4);
    assert!(repair::can_repair(&order, &state, pistol));
    // A worn rifle doesn't count for the button, but is offered.
    state.equip(&order, PLAYER_REF, rifle);
    assert!(!repair::can_repair(&order, &state, pistol));
    let parts = repair::parts(&order, &state, pistol);
    let counts: Vec<(FormId, i32, bool)> =
        parts.iter().map(|p| (p.item, p.count, p.chosen)).collect();
    assert_eq!(counts, vec![(pistol, 0, true), (rifle, 1, false)]);
    state.unequip(PLAYER_REF, rifle);
    // A second pistol: the first is the chosen one.
    give(&order, &mut state, PISTOL, 1);
    let parts = repair::parts(&order, &state, pistol);
    let counts: Vec<(FormId, i32)> = parts.iter().map(|p| (p.item, p.count)).collect();
    assert!(counts.contains(&(pistol, 1)));
    assert!(counts.contains(&(rifle, 1)));
    // With the rifle: 4 + 0.5 + 0.75 + 3 × 0.05 = 5.4 → 54%.
    let rifle_part = parts.iter().find(|p| p.item == rifle).unwrap();
    assert!(close(rifle_part.mends_to, 0.54));
    let to = repair::repair_with(&order, &mut state, pistol, rifle);
    assert!(close(to, 0.54));
    assert!(close(
        world::combat::weapon_condition(&state, PLAYER_REF, pistol),
        0.54
    ));
    assert_eq!(state.item_count(&order, PLAYER_REF, rifle), 0);
    assert_eq!(world::stats::get(&state, repair::ITEMS_REPAIRED), 1);
    // A whole weapon isn't offered for repair.
    state.weapon_health.insert((PLAYER_REF, pistol), 1.0);
    assert!(!repair::can_repair(&order, &state, pistol));
}

/// Armour wears as it takes hits and loses DT with it: a piece's DT at a
/// condition (`004be0b0`: truncated, × 0.5 + c at or below half, rounded
/// up, summed over the pieces worn, `008d2110`); a hit's `fArmorDamage`
/// (`009b5a30`: 0.5 × the DT taken off, at most what's left above the
/// floor); worn on the head piece for head hits, else the body's
/// (`0089d8b0`); the player told below 25% (`00891360`).
#[test]
fn armour_wears_and_loses_its_threshold() {
    use world::combat;
    let (_data, order) = order("repair-armour");
    let mut state = GameState::new(&order);
    let (armour, helmet) = (FormId(ARMOR), FormId(HELMET));
    assert_eq!(combat::armour_figure_at(10.0, 1.0), 10.0);
    assert_eq!(combat::armour_figure_at(10.0, 0.3), 8.0);
    assert_eq!(combat::armour_figure_at(10.0, 0.33), 9.0);
    assert_eq!(combat::armour_figure_at(7.9, 1.0), 7.0);

    give(&order, &mut state, ARMOR, 1);
    give(&order, &mut state, HELMET, 1);
    state.equip(&order, PLAYER_REF, armour);
    state.equip(&order, PLAYER_REF, helmet);
    let base = combat::worn_damage_threshold(&order, &state, PLAYER_REF);
    // 30 damage through DT 14: 16 left, the floor 6; the armour takes
    // 0.5 × min(14, 16 - 6) = 5.
    let hit = combat::armour_hit(&order, &state, 30.0, None, PLAYER_REF, None);
    assert!(close(hit.damage, 30.0 - base));
    assert!(close(
        hit.armour_damage,
        0.5 * (base.min(30.0 - base - 6.0))
    ));
    // A body hit wears the body armour, a head hit the helmet.
    combat::wear_armour(&order, &mut state, PLAYER_REF, None, 5.0);
    assert!(close(
        combat::weapon_condition(&state, PLAYER_REF, armour),
        0.95
    ));
    combat::wear_armour(&order, &mut state, PLAYER_REF, Some(1), 5.0);
    assert!(close(
        combat::weapon_condition(&state, PLAYER_REF, helmet),
        0.9
    ));
    // Only the player's wears.
    combat::wear_armour(&order, &mut state, FormId(DOC_REF), None, 5.0);
    assert!(!state.weapon_health.contains_key(&(FormId(DOC_REF), armour)));
    // At 30% the armour's DT is 8 (ceil(10 × 0.8)).
    state.weapon_health.insert((PLAYER_REF, armour), 0.3);
    assert!(close(
        combat::worn_damage_threshold(&order, &state, PLAYER_REF),
        base - 10.0 + 8.0
    ));
    // Falling below 25% tells the player.
    state.events.clear();
    combat::wear_armour(&order, &mut state, PLAYER_REF, None, 6.0);
    assert!(close(
        combat::weapon_condition(&state, PLAYER_REF, armour),
        0.24
    ));
    assert!(state.events.iter().any(|e| matches!(e,
        Event::Message { text, .. } if text == "Your armor condition is dangerously low.")));
    // Under 1 health left: nothing.
    combat::wear_armour(&order, &mut state, PLAYER_REF, None, 23.5);
    assert_eq!(combat::weapon_condition(&state, PLAYER_REF, armour), 0.0);
    // A merchant mends it now (DT above 0): to 70% at Repair 50.
    state
        .items
        .insert((PLAYER_REF, world::barter::caps(&order)), 1000);
    let (lines, _) = repair::service_lines(&order, &state, 50);
    assert!(lines.iter().any(|l| l.item == armour && l.cost > 0));
    state
        .actor_values
        .insert((FormId(DOC_REF), repair::REPAIR), 50.0);
    repair::repair_by(&order, &mut state, FormId(DOC_REF), armour, 0);
    assert!(close(
        combat::weapon_condition(&state, PLAYER_REF, armour),
        0.7
    ));
}

/// A real hit (`Runner::hit_at`): the doctor (Guns 100) shoots the player
/// in DT 10 armour with the rifle (20): 10 through, the floor 4, the armour
/// takes 0.5 × min(10, 6) = 3 of its 100 (more on a critical).
#[test]
fn a_hit_wears_the_players_armour() {
    let (_data, order) = order("repair-hit");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    give(&order, &mut state, ARMOR, 1);
    state.equip(&order, PLAYER_REF, FormId(ARMOR));
    state.actor_values.insert((FormId(DOC_REF), 41), 100.0);
    let rifle = world::combat::Weapon::load(&order, FormId(RIFLE)).unwrap();
    Runner::new(&order, &scripts, &mut state).hit_at(
        FormId(DOC_REF),
        PLAYER_REF,
        Some(&rifle),
        None,
    );
    let c = world::combat::weapon_condition(&state, PLAYER_REF, FormId(ARMOR));
    assert!(c <= 0.97 + 1e-4, "{c}");
}

/// A weapon worn to nothing breaks (`00891360`): the player's is taken
/// off with "Your weapon has broken."; someone else drops theirs. Nothing
/// broken goes back on (`0088c830`): "Broken items cannot be equipped
/// until they have been repaired."
#[test]
fn broken_things_come_off_and_stay_off() {
    use world::combat;
    let (_data, order) = order("repair-broken");
    let mut state = GameState::new(&order);
    let pistol = FormId(PISTOL);
    give(&order, &mut state, PISTOL, 1);
    state.equip(&order, PLAYER_REF, pistol);
    state.events.clear();
    // 150 health: 149.5 leaves under 1, so nothing.
    combat::damage_item(&order, &mut state, PLAYER_REF, pistol, 149.5);
    assert_eq!(combat::weapon_condition(&state, PLAYER_REF, pistol), 0.0);
    assert!(!state.is_equipped(PLAYER_REF, pistol));
    let said = |state: &GameState, t: &str| {
        state
            .events
            .iter()
            .any(|e| matches!(e, Event::Message { text, .. } if text == t))
    };
    assert!(said(&state, "Your weapon has broken."));
    state.events.clear();
    state.equip(&order, PLAYER_REF, pistol);
    assert!(!state.is_equipped(PLAYER_REF, pistol));
    assert!(said(
        &state,
        "Broken items cannot be equipped until they have been repaired."
    ));
    // Mended, it goes on again.
    state.weapon_health.insert((PLAYER_REF, pistol), 0.5);
    state.equip(&order, PLAYER_REF, pistol);
    assert!(state.is_equipped(PLAYER_REF, pistol));
    // The doctor's breaks and he drops it.
    let doc = FormId(DOC_REF);
    state.stock(&order, doc);
    state.items.insert((doc, pistol), 1);
    state.equip(&order, doc, pistol);
    combat::damage_item(&order, &mut state, doc, pistol, 200.0);
    assert!(!state.is_equipped(doc, pistol));
    assert!(state.dropped.contains(&(doc, pistol)));
}
