//! What the casino games share on screen (`world::casino`): the anti-cheat
//! lock, `Create`'s checks before a game's menu opens, and the corner
//! messages closing a game leaves. The games themselves: `slots`.

use bevy::prelude::Resource;
use esm::{FormId, LoadOrder};
use world::casino::{AntiCheat, Casino, Game, Settled};
use world::scripting::GameState;

/// The casinos' anti-cheat lock (`world::casino::AntiCheat`, the game's
/// globals): stamped as a casino menu closes, armed by loading a game; on
/// the real clock in whole seconds (the game's `GetTickCount() / 1000`).
#[derive(Resource, Default)]
pub struct CasinoLock(pub AntiCheat);

/// A script's `Show…MenuParams` (`Event::Casino`): `Create`'s checks in
/// the game's order (`world::casino::open_check`, the player's line for the
/// casino made first), then the menu to open; or the corner message the
/// game shows instead (the surprised Vault Boy, `UIPopUpMessageGeneral`).
/// `now` is the real clock's seconds. `None` for a form that isn't a
/// casino (the game's `Create` finds no record and opens nothing).
#[allow(clippy::too_many_arguments)]
pub fn create(
    order: &LoadOrder,
    state: &mut GameState,
    lock: &mut AntiCheat,
    game: Game,
    casino: FormId,
    min_bet: i32,
    max_bet: i32,
    min_winnings: i32,
    now: f64,
) -> Option<Result<crate::menus::Menu, String>> {
    let c = Casino::load(order, casino)?;
    Some(
        world::casino::open_check(order, state, &c, min_bet, min_winnings, lock, now as u64)
            .map(|()| crate::menus::Menu::Casino {
                game,
                casino,
                min_bet,
                max_bet,
            })
            .map_err(|why| world::casino::refusal_text(order, game, why)),
    )
}

/// The `MenuMode` number a script's casino request gives the scripts:
/// the game's (1080-1082, [`Game::menu`]) only when `Create`'s checks let
/// the menu open (the command's `Create`, `005cf040` and the others,
/// returns before the menu is made when they refuse). The world's
/// `Event::Menu` for these numbers (`world::scripting`) is left to this.
pub fn menu_mode(game: Game, created: &Option<Result<crate::menus::Menu, String>>) -> Option<u16> {
    matches!(created, Some(Ok(_))).then(|| game.menu())
}

/// The corner message closing a game leaves (`world::casino::settle`):
/// chips taken, or given (with the ban at the limit).
pub fn settled_message(settled: Settled) -> Option<crate::hud::HudMessage> {
    let icon = settled.icon();
    match settled {
        Settled::Removed { message, .. } | Settled::Added { message, .. } => {
            Some(crate::hud::HudMessage::with_icon(message, icon))
        }
        Settled::Nothing => None,
    }
}

/// "Out of chips" (`sGamblingBrokeText`) with the refusals' surprised
/// Vault Boy.
pub fn broke_message(order: &LoadOrder, game: Game) -> crate::hud::HudMessage {
    crate::hud::HudMessage::with_icon(
        world::casino::refusal_text(order, game, world::casino::Refusal::Broke),
        Some(world::casino::REFUSAL_ICON),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use esm::ActivePlugins;
    use testdata::quest_ids::{CASINO, CHIP};
    use world::dialogue::PLAYER_REF;

    /// `MenuMode` for a casino game only once its menu opened.
    #[test]
    fn menu_mode_only_when_opened() {
        let opened = Some(Ok(crate::menus::Menu::Casino {
            game: Game::Roulette,
            casino: FormId(CASINO),
            min_bet: 1,
            max_bet: 100,
        }));
        assert_eq!(menu_mode(Game::Roulette, &opened), Some(1082));
        assert_eq!(menu_mode(Game::Slots, &Some(Err("no chips".into()))), None);
        assert_eq!(menu_mode(Game::Blackjack, &None), None);
    }

    /// A script's request: refused with the game's words until the player
    /// has chips, then the menu; a minute's lock after a casino menu closed
    /// and a game loaded; not a casino, nothing.
    #[test]
    fn create_checks_then_opens() {
        let data = testdata::quests("viewer-casino-create");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let mut state = GameState::new(&order);
        let mut lock = AntiCheat::default();
        let ask = |state: &mut GameState, lock: &mut AntiCheat, now: f64| {
            create(
                &order,
                state,
                lock,
                Game::Slots,
                FormId(CASINO),
                1,
                60,
                0,
                now,
            )
        };
        assert_eq!(
            ask(&mut state, &mut lock, 100.0).unwrap().err().as_deref(),
            Some("You must purchase chips before you can play this game.")
        );
        // The player's line for the casino stays.
        assert_eq!(state.casinos.len(), 1);
        *state.items.entry((PLAYER_REF, FormId(CHIP))).or_insert(0) += 20;
        let opened = ask(&mut state, &mut lock, 100.5).unwrap();
        assert!(matches!(
            opened,
            Ok(crate::menus::Menu::Casino {
                game: Game::Slots,
                min_bet: 1,
                max_bet: 60,
                ..
            })
        ));
        lock.closed(100);
        lock.loaded();
        let refused = ask(&mut state, &mut lock, 130.2).unwrap().err().unwrap();
        assert!(
            refused.starts_with("This machine appears to be taking a minute")
                && refused.ends_with("Time Remaining: 30"),
            "{refused}"
        );
        assert!(ask(&mut state, &mut lock, 160.0).unwrap().is_ok());
        assert!(create(
            &order,
            &mut state,
            &mut lock,
            Game::Slots,
            FormId(CHIP),
            1,
            60,
            0,
            0.0
        )
        .is_none());
    }

    /// The settling messages carry their pictures to the HUD's corner.
    #[test]
    fn settled_messages_have_their_vault_boys() {
        let lost = settled_message(Settled::Removed {
            count: 2,
            message: "2 Chip(s) removed".into(),
        })
        .unwrap();
        assert_eq!(lost.icon.as_deref(), Some(world::message_icon::SAD));
        let banned = settled_message(Settled::Added {
            count: 2,
            message: "banned".into(),
            banned: true,
        })
        .unwrap();
        assert_eq!(
            banned.icon.as_deref(),
            Some(world::message_icon::VERY_HAPPY)
        );
        assert!(settled_message(Settled::Nothing).is_none());
    }
}
