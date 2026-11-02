//! The money on the table while betting (`UpdateBettingUI` `0074a2c0`,
//! `LoadNewBill` `0074f9b0`, `LoadNewCoin` `0074fc00`; placements filled by
//! `PrepareAnteMenu` `0073d020`). Only for show: the pieces are picked at
//! random by how much the opponent might bet, whatever money either side
//! has.
//!
//! - Each call adds pieces for an amount: while some is left (and fewer
//!   than 9 bills and 42 coins came this call), two numbers are drawn, b
//!   below 9 − the bills so far and c below 42 − the coins so far; a coin
//!   when c ≥ 1.5 b (c ≥ b from 2000 up), else a bill. Its kind by the
//!   tier (the opponent's most): coins below 120 a draw below 100 thrown
//!   away then 0 or 1; below 400 a draw below 100, ≤ 32 plain, ≤ 74 silver,
//!   else gold; below 1000 ≤ 32, ≤ 65; below 2000 ≤ 4, ≤ 19; from 2000 ≤ 4,
//!   ≤ 9. Bills below 400 any of the three, below 2000 the $20 or $100,
//!   from 2000 the $100. The amount goes down by the piece's worth (bills
//!   2, 8, 40; coins 1, 4, 100).
//! - A piece's own draw picks how it lands: a bill `Top_Place`,
//!   `Bottom_Place` or `Alt_Place`, a coin `Top_Place` or `Bottom_Place`.
//! - Then each new bill, then each new coin, takes a free spot at random
//!   (the 9 `Bill-Placement` and 42 `Coin-Placement` shapes of
//!   `Currency-Bill_Grid.NIF` / `Currency-Coin_Grid.NIF`), the last free
//!   spot moving into the taken one's place (`009a4320`); none left, it
//!   isn't placed.

use super::Dice;

/// The spots on the table.
pub const BILL_SPOTS: usize = 9;
pub const COIN_SPOTS: usize = 42;

/// What a piece counts for, by kind.
pub const BILL_WORTH: [i32; 3] = [2, 8, 40];
pub const COIN_WORTH: [i32; 3] = [1, 4, 100];

/// One bill or coin on the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    /// 0–2: `Currency-Bill_1`–`3` ($5, $20, $100) or `Currency-Coin_1`–`3`
    /// (plain, silver, gold).
    pub kind: usize,
    /// The sequence it lands with.
    pub sequence: &'static str,
    /// Its spot (0-based: `Bill-Placement_0{n+1}` / `Coin-Placement_…`).
    pub spot: Option<usize>,
}

/// The money on the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub bills: Vec<Piece>,
    pub coins: Vec<Piece>,
    /// The first bill and coin not landed yet (`iBillIndexToAnim`,
    /// `iCoinIndexToAnim`).
    pub bills_from: usize,
    pub coins_from: usize,
    free_bills: Vec<usize>,
    free_coins: Vec<usize>,
}

impl Default for Table {
    fn default() -> Table {
        Table {
            bills: Vec::new(),
            coins: Vec::new(),
            bills_from: 0,
            coins_from: 0,
            free_bills: (0..BILL_SPOTS).collect(),
            free_coins: (0..COIN_SPOTS).collect(),
        }
    }
}

/// A draw in `lo..hi` (`00944460`).
fn range(dice: Dice, lo: usize, hi: usize) -> usize {
    if hi <= lo {
        lo
    } else {
        lo + dice(hi - lo)
    }
}

impl Table {
    /// Adds pieces for an amount (`UpdateBettingUI`); true when any came.
    pub fn add(&mut self, most: i32, mut amount: i32, dice: Dice) -> bool {
        let (mut bills, mut coins) = (0usize, 0usize);
        while amount > 0 && bills < BILL_SPOTS && coins < COIN_SPOTS {
            let b = range(dice, 0, BILL_SPOTS - bills);
            let c = range(dice, 0, COIN_SPOTS - coins);
            let coin = if most < 2000 {
                c as f64 >= 1.5 * b as f64
            } else {
                c >= b
            };
            if coin {
                let kind = if most < 120 {
                    range(dice, 0, 100);
                    range(dice, 0, 2)
                } else {
                    let r = range(dice, 0, 100);
                    let (plain, silver) = match most {
                        m if m < 400 => (32, 74),
                        m if m < 1000 => (32, 65),
                        m if m < 2000 => (4, 19),
                        _ => (4, 9),
                    };
                    if r <= plain {
                        0
                    } else if r <= silver {
                        1
                    } else {
                        2
                    }
                };
                let kind = kind.min(2);
                let sequence = ["Top_Place", "Bottom_Place"][range(dice, 0, 2)];
                self.coins.push(Piece {
                    kind,
                    sequence,
                    spot: None,
                });
                amount -= COIN_WORTH[kind];
                coins += 1;
            } else {
                let kind = match most {
                    m if m < 400 => range(dice, 0, 3),
                    m if m < 2000 => range(dice, 1, 3),
                    _ => 2,
                }
                .min(2);
                let sequence = ["Top_Place", "Bottom_Place", "Alt_Place"][range(dice, 0, 3)];
                self.bills.push(Piece {
                    kind,
                    sequence,
                    spot: None,
                });
                amount -= BILL_WORTH[kind];
                bills += 1;
            }
        }
        for bill in &mut self.bills[self.bills_from..] {
            if self.free_bills.is_empty() {
                break;
            }
            let r = range(dice, 0, self.free_bills.len());
            bill.spot = Some(self.free_bills.swap_remove(r));
        }
        for coin in &mut self.coins[self.coins_from..] {
            if self.free_coins.is_empty() {
                break;
            }
            let r = range(dice, 0, self.free_coins.len());
            coin.spot = Some(self.free_coins.swap_remove(r));
        }
        bills + coins > 0
    }

    /// The new pieces have landed (state 4's end).
    pub fn landed(&mut self) {
        self.bills_from = self.bills.len();
        self.coins_from = self.coins.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `0074a2c0`'s draws in order: b, c, the kind, the landing; then the
    /// spots.
    #[test]
    fn pieces_and_spots() {
        let mut t = Table::default();
        // Every draw 0: c (0) ≥ 1.5 b (0), so coins; below 120 the first
        // kind draw is thrown away, the second 0: plain coins worth 1.
        let mut zero = |_: usize| 0;
        assert!(t.add(100, 3, &mut zero));
        assert_eq!(t.coins.len(), 3);
        assert!(t.bills.is_empty());
        assert!(t
            .coins
            .iter()
            .all(|c| c.kind == 0 && c.sequence == "Top_Place"));
        // Spots: the first free each time, the last moving into it.
        let spots: Vec<_> = t.coins.iter().map(|c| c.spot).collect();
        assert_eq!(spots, vec![Some(0), Some(41), Some(40)]);
        t.landed();
        assert_eq!(t.coins_from, 3);
        // A bill when c < 1.5 b: b = 8, c = 0; from 2000 always the $100.
        let mut draws = vec![8usize, 0, 2].into_iter().cycle();
        let mut t = Table::default();
        t.add(5000, 40, &mut |n| draws.next().unwrap() % n);
        assert_eq!(t.bills.len(), 1);
        assert_eq!((t.bills[0].kind, t.bills[0].sequence), (2, "Alt_Place"));
    }
}
