//! The casinos (`world::casino`): the record, opening a game and its
//! refusals, the winnings and levels, settling the chips.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::casino::{self, AntiCheat, Casino, Game, Refusal, Settled};
use world::dialogue::PLAYER_REF;
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, line: &str) {
    Runner::new(order, scripts, state).run_source(line, None, None);
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

/// `TESCasino::Load` (`005048d0`): the eighth `MODL` and `MOD2` both the
/// slot machine, `ICO2`s from texture 7, `DATA` as `CASINO_DATA`.
#[test]
fn the_record() {
    let (_data, order) = order("casino-record");
    let c = Casino::load(&order, FormId(CASINO)).unwrap();
    assert_eq!(c.name, "Gommorah");
    assert_eq!(c.models[0], "NV_Blackjack-Chip_001.NIF");
    assert_eq!(c.models[6], "NV_Roulette-Chip.NIF");
    assert_eq!(c.models[7], "NV_SlotMachine-Minigame-Gom.NIF");
    assert_eq!(c.models[8], "NV_Blackjack-Table-Gom.NIF");
    assert_eq!(c.models[9], "NV_Roulette-Table-Gom.NIF");
    assert_eq!(c.textures[6], "NV_SlotMachine-Symbol5.dds");
    assert_eq!(c.textures[10], "DeckG\\cardG_back.dds");
    assert_eq!(c.slot_stops, [2; 7]);
    assert_eq!((c.decks, c.max_winnings), (3, 9000));
    assert_eq!((c.chip, c.quest), (FormId(CHIP), FormId(QUEST)));
    assert!((c.shuffle_percent - 0.2).abs() < 1e-6 && (c.blackjack_payout - 1.5).abs() < 1e-6);
    assert!(!c.dealer_hole_card);
}

/// `ShowSlotMachineMenuParams` asks for the game; `Create`'s checks refuse
/// it in turn, the player's line for the casino made first.
#[test]
fn opening_and_its_refusals() {
    let (_data, order) = order("casino-open");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "ShowSlotMachineMenuParams TestCasino 1 60 0",
    );
    assert!(state.events.iter().any(|e| matches!(
        e,
        Event::Casino {
            game: Game::Slots,
            min_bet: 1,
            max_bet: 60,
            min_winnings: 0,
            ..
        }
    )));
    let c = Casino::load(&order, FormId(CASINO)).unwrap();
    let mut lock = AntiCheat::default();
    let open = |state: &mut GameState, min_winnings: i32, lock: &mut AntiCheat| {
        casino::open_check(&order, state, &c, 1, min_winnings, lock, 1000)
    };
    assert_eq!(open(&mut state, 0, &mut lock), Err(Refusal::Broke));
    assert_eq!(state.casinos.len(), 1);
    run(&order, &scripts, &mut state, "player.AddItem TestChip 50");
    assert_eq!(open(&mut state, 0, &mut lock), Ok(()));
    assert_eq!(open(&mut state, 10, &mut lock), Err(Refusal::NotWonEnough));
    // Closed then a game loaded: a minute's lock.
    lock.closed(990);
    lock.loaded();
    assert_eq!(open(&mut state, 0, &mut lock), Err(Refusal::AntiCheat(50)));
    let text = casino::refusal_text(&order, Game::Slots, Refusal::AntiCheat(50));
    assert!(text.ends_with("\nTime Remaining: 50"), "{text}");
    casino::data_mut(&mut state, FormId(CASINO)).winnings = 9000;
    let mut free = AntiCheat::default();
    assert_eq!(open(&mut state, 0, &mut free), Err(Refusal::Banned));
}

/// The winnings by quarters (`GetCasinoWinningsLevel`, the condition
/// `GetCasinoWinningStage`), `SetCasinoWinningsLevel`, and the save.
#[test]
fn levels_from_scripts() {
    let (_data, order) = order("casino-levels");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let global = |state: &GameState| state.globals.get(&FormId(GLOBAL)).copied().unwrap_or(0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "set TestGlobal to GetCasinoWinningsLevel TestCasino",
    );
    assert_eq!(global(&state), 0.0);
    casino::data_mut(&mut state, FormId(CASINO)).winnings = 4600;
    run(
        &order,
        &scripts,
        &mut state,
        "set TestGlobal to GetCasinoWinningsLevel TestCasino",
    );
    assert_eq!(global(&state), 2.0);
    run(
        &order,
        &scripts,
        &mut state,
        "SetCasinoWinningsLevel TestCasino 3",
    );
    let d = casino::data(&state, FormId(CASINO)).unwrap();
    assert_eq!((d.winnings, d.level), (6750, 3));
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.casinos, state.casinos);
}

/// Closing: the menu's chips against the player's, taken or given, and a
/// new level starting the comps quest.
#[test]
fn settling() {
    let (_data, order) = order("casino-settle");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    run(&order, &scripts, &mut state, "player.AddItem TestChip 50");
    let c = Casino::load(&order, FormId(CASINO)).unwrap();
    casino::data_mut(&mut state, FormId(CASINO));
    let s = casino::settle(&order, &mut state, &c, 30, false);
    assert_eq!(
        s,
        Settled::Removed {
            count: 20,
            message: "20 Test Chip(s) removed".into()
        }
    );
    // Lost chips: the sad Vault Boy (`00734db0`, `007bd2a0`, `007c2c70`).
    assert_eq!(s.icon(), Some(world::message_icon::SAD));
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CHIP)), 30);
    let s = casino::settle(&order, &mut state, &c, 45, false);
    assert_eq!(
        s,
        Settled::Added {
            count: 15,
            message: "15 Test Chip(s) added".into(),
            banned: false
        }
    );
    // The usual "added" notice's gift box (`004821a0`).
    assert_eq!(s.icon(), Some(world::message_icon::GIFT_BOX));
    // One: the name alone (`"%s %s"`).
    let s = casino::settle(&order, &mut state, &c, 46, false);
    assert!(matches!(s, Settled::Added { message, .. } if message == "Test Chip added"));
    // At the limit: given with the ban, and the level's quest started.
    casino::data_mut(&mut state, FormId(CASINO)).winnings = 9000;
    let s = casino::settle(&order, &mut state, &c, 50, true);
    assert_eq!(
        s,
        Settled::Added {
            count: 4,
            message: "4 Test Chip(s) added\nYou have been banned from gambling at this casino."
                .into(),
            banned: true
        }
    );
    // Banned: the very happy one.
    assert_eq!(s.icon(), Some(world::message_icon::VERY_HAPPY));
    assert_eq!(Settled::Nothing.icon(), None);
    assert!(state.running.contains(&FormId(QUEST)));
    // The earnings line pads only a one-character number.
    assert_eq!(
        casino::earnings_line(&order, &c, 7),
        format!("{:<22}7", "Gommorah Earnings: ")
    );
    assert_eq!(
        casino::earnings_line(&order, &c, 120),
        "Gommorah Earnings: 120"
    );
}
