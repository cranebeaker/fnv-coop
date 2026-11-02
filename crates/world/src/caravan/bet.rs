//! The bet (the ante menu: `PrepareAnteMenu`, `PrepareBettingCurrency`,
//! `ItemSelectCallback` (PC `0074a090`), `UpdateCaravanFlags`;
//! `PrepareDeckMenu` fixes the stake) and the results (`PrepareResultsMenu`,
//! PC `00740980`; `ExchangeCurrency`, PC `0074cc70`).

use esm::{FormId, LoadOrder};

use crate::scripting::GameState;

/// The Barter actor value the opponent's most is scaled by.
const BARTER: u16 = 32;
/// The misc stats a game counts in (`IncrementMiscStat` 0x25 and 0x26).
pub const GAMES_WON: u8 = 0x25;
pub const GAMES_LOST: u8 = 0x26;

/// The kinds of money: caravan money (`CMNY`, form type 0x74) with its
/// value.
fn money(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<(FormId, i32, i32)> {
    let mut v: Vec<(FormId, i32, i32)> = state
        .items
        .iter()
        .filter(|((h, _), n)| *h == who && **n > 0)
        .filter_map(|((_, item), n)| {
            let kind = order.get(*item)?.entry.header.kind;
            (kind.as_bytes() == b"CMNY")
                .then(|| (*item, *n, crate::barter::base_value(order, *item) as i32))
        })
        .collect();
    v.sort_by_key(|(f, _, _)| *f);
    v
}

/// What someone has to bet (`PrepareBettingCurrency`): their caps and their
/// caravan money's worth (each kind's value × how many).
pub fn funds(order: &LoadOrder, state: &mut GameState, who: FormId) -> i32 {
    state.stock(order, who);
    let caps = state
        .item_count(order, who, crate::barter::caps(order))
        .max(0);
    caps + money(order, state, who)
        .iter()
        .map(|(_, n, v)| n * v)
        .sum::<i32>()
}

/// The antes (`iPlayerAnte`, `iNPCAnte`, `iTotalFunds`, `iNPCTotalFunds`,
/// `fBetPercentage`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bet {
    pub player_funds: i32,
    pub npc_funds: i32,
    /// `ShowCaravanMenu`'s share of the opponent's funds.
    pub share: f32,
    pub player_ante: i32,
    pub npc_ante: i32,
}

impl Bet {
    /// `PrepareAnteMenu`: the opponent opens with its share of its funds.
    pub fn new(player_funds: i32, npc_funds: i32, share: f32) -> Bet {
        Bet {
            player_funds,
            npc_funds,
            share,
            player_ante: 0,
            npc_ante: (npc_funds as f32 * share) as i32,
        }
    }

    /// The most the opponent will put in (`0074a090`): its share, more by
    /// the player's Barter (0 to 100) as a percentage, at most all it has.
    pub fn npc_most(&self, barter: f32) -> i32 {
        let x = f64::from(self.npc_funds as f32 * self.share);
        let most = (f64::from(barter.clamp(0.0, 100.0)) / 100.0 * x + x) as i32;
        most.min(self.npc_funds)
    }

    /// Raise (`ItemSelectCallback`, from a "how many" of up to what's left):
    /// the opponent matches up to its most.
    pub fn raise(&mut self, amount: i32, barter: f32) {
        self.player_ante += amount;
        let most = self.npc_most(barter);
        if self.npc_ante < self.player_ante {
            self.npc_ante = most.min(self.player_ante);
        }
    }

    /// Match (the A button): the player puts in the opponent's ante, or
    /// all they have.
    pub fn automatch(&mut self) {
        self.player_ante = self.npc_ante.min(self.player_funds);
    }

    /// Which buttons work (`UpdateCaravanFlags`, the ante menu).
    pub fn can_accept(&self) -> bool {
        !(self.player_ante < 1 && self.npc_ante != 0)
    }

    pub fn can_automatch(&self) -> bool {
        self.player_funds != 0
            && (self.player_ante < 1
                || (self.player_ante < self.npc_ante && self.player_ante < self.player_funds))
    }

    pub fn can_raise(&self) -> bool {
        self.player_funds != 0 && (self.player_ante < 1 || self.player_ante < self.player_funds)
    }

    /// The stake (`PrepareDeckMenu`): the smaller ante.
    pub fn stake(&self) -> i32 {
        self.player_ante.min(self.npc_ante)
    }
}

/// The loser pays the winner the stake (`ExchangeCurrency`): caravan money
/// first (each kind as far as it goes without passing what's owed), then
/// caps, as many as they have.
fn pay(order: &LoadOrder, state: &mut GameState, from: FormId, to: FormId, stake: i32) {
    let mut owed = stake;
    for (item, n, value) in money(order, state, from) {
        if value <= 0 {
            continue;
        }
        let mut count = n;
        if value * n > owed {
            count = owed / value;
        }
        if count > 0 {
            state.move_item(order, from, to, item, count);
            owed -= count * value;
        }
    }
    if owed != 0 {
        let caps_form = crate::barter::caps(order);
        let caps = state.item_count(order, from, caps_form).max(0);
        let count = owed.min(caps);
        if count > 0 {
            state.move_item(order, from, to, caps_form, count);
        }
    }
}

/// The game over (`PrepareResultsMenu`): the player's record (caps won or
/// lost, games won or lost, the biggest win), the misc stat, the stake
/// paid. The sound to play (`GAMECaravanWin` / `GAMECaravanLose`).
pub fn settle(
    order: &LoadOrder,
    state: &mut GameState,
    npc: FormId,
    stake: i32,
    player_won: bool,
) -> &'static str {
    let stake_u = stake.max(0) as u32;
    let c = &mut state.caravan;
    let (stat, sound) = if player_won {
        c.cap_winnings += stake_u;
        c.winnings += 1;
        c.largest_winning = c.largest_winning.max(stake_u);
        (GAMES_WON, "GAMECaravanWin")
    } else {
        c.cap_losses += stake_u;
        c.losses += 1;
        (GAMES_LOST, "GAMECaravanLose")
    };
    *state.misc_stats.entry(stat).or_insert(0) += 1;
    state.stock(order, npc);
    state.stock(order, crate::dialogue::PLAYER_REF);
    let player = crate::dialogue::PLAYER_REF;
    if player_won {
        pay(order, state, npc, player, stake);
    } else {
        pay(order, state, player, npc, stake);
    }
    sound
}

/// The player's Barter for [`Bet::npc_most`].
pub fn player_barter(order: &LoadOrder, state: &GameState) -> f32 {
    crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(crate::dialogue::PLAYER_REF, BARTER)
    .unwrap_or(0.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `PrepareAnteMenu`, `0074a090`, `UpdateCaravanFlags`, `PrepareDeckMenu`.
    #[test]
    fn antes() {
        let mut b = Bet::new(500, 200, 0.4);
        assert_eq!(b.npc_ante, 80);
        assert!(!b.can_accept());
        assert!(b.can_raise() && b.can_automatch());
        // Barter 50: it'll go to 80 × 1.5 = 120.
        assert_eq!(b.npc_most(50.0), 120);
        b.raise(150, 50.0);
        assert_eq!((b.player_ante, b.npc_ante), (150, 120));
        assert_eq!(b.stake(), 120);
        assert!(!b.can_automatch());
        let mut b = Bet::new(50, 200, 0.4);
        b.automatch();
        assert_eq!(b.player_ante, 50);
        assert!(b.can_accept() && !b.can_raise());
        // Nothing asked: playing for nothing is fine.
        let b = Bet::new(0, 0, 0.5);
        assert!(b.can_accept() && !b.can_raise());
        // At most all it has.
        assert_eq!(Bet::new(10, 100, 1.0).npc_most(100.0), 100);
    }
}
