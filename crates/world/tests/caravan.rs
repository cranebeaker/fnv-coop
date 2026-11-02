//! Caravan: cards and decks as the records give them, and the player's
//! cards (`AddCardToPlayer`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::caravan::{self, Card};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// `TESCaravanCard::Load` (the suit, then the value), a deck's cards; a
/// card picked up joins the player's cards and leaves their things
/// (`CardAddToPlayerScript`, `PlayerCharacter::AddCaravanCard`), once.
#[test]
fn cards_decks_and_the_players_cards() {
    let (_data, order) = order("caravan-cards");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let ace = Card {
        form: FormId(CARD_ACE),
        value: 1,
        suit: 1,
    };
    let queen = Card {
        form: FormId(CARD_QUEEN),
        value: 13,
        suit: 2,
    };
    assert_eq!(caravan::card(&order, FormId(CARD_ACE)), Some(ace));
    assert_eq!(caravan::card(&order, FormId(CAPS)), None);
    assert_eq!(
        caravan::deck(&order, FormId(CARAVAN_DECK)),
        vec![ace, queen]
    );
    let run = |state: &mut GameState, line: &str| {
        Runner::new(&order, &scripts, state).run_source(line, None, None);
        Runner::new(&order, &scripts, state).update(0.1);
        assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
    };
    run(&mut state, "player.AddItem TestCardAce 1");
    assert_eq!(state.caravan.inactive, vec![FormId(CARD_ACE)]);
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CARD_ACE)), 0);
    run(&mut state, "player.AddItem TestCardAce 1");
    assert_eq!(state.caravan.inactive, vec![FormId(CARD_ACE)]);
    // Kept in a save.
    state.caravan.active.push(FormId(CARD_QUEEN));
    state.caravan.winnings = 3;
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert_eq!(back.caravan, state.caravan);
}

/// `PrepareBettingCurrency` (caps and caravan money), `PrepareResultsMenu`
/// and `ExchangeCurrency`: the loser pays the stake, the record and the
/// misc stat count the game.
#[test]
fn funds_and_settling_up() {
    let (_data, order) = order("caravan-bet");
    let mut state = GameState::new(&order);
    let caps = world::barter::caps(&order);
    let doc = FormId(DOC_REF);
    state.items.insert((PLAYER_REF, caps), 120);
    state.items.insert((doc, caps), 30);
    assert_eq!(caravan::bet::funds(&order, &mut state, PLAYER_REF), 120);
    // The player loses 50: theirs to Doc.
    let sound = caravan::bet::settle(&order, &mut state, doc, 50, false);
    assert_eq!(sound, "GAMECaravanLose");
    assert_eq!(state.item_count(&order, PLAYER_REF, caps), 70);
    assert_eq!(state.item_count(&order, doc, caps), 80);
    assert_eq!((state.caravan.losses, state.caravan.cap_losses), (1, 50));
    assert_eq!(state.misc_stats.get(&caravan::bet::GAMES_LOST), Some(&1));
    // The player wins 100 but Doc has 80: all he has.
    caravan::bet::settle(&order, &mut state, doc, 100, true);
    assert_eq!(state.item_count(&order, PLAYER_REF, caps), 150);
    assert_eq!(state.item_count(&order, doc, caps), 0);
    assert_eq!(state.caravan.largest_winning, 100);
}
