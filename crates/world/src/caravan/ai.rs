//! The opponent (`CaravanMenu::ProcessAI`, PC `0074fdc0`): every move it
//! could make becomes a package (`CaravanAIPackage`: what the track would
//! come to, how that stands, the action, the track, row and hand card, and a
//! priority); the packages are sorted by priority, highest first, with the
//! exe's own `qsort` (`00ec6f20`, Visual Studio 2008's, so equal priorities
//! fall as they do in the game), and the first is played.
//!
//! The game's own oddities are kept: a jack meant for one of the player's
//! rows is checked and played on the opponent's own track at that row
//! number; the joker's estimate counts aces by value on an ace and sums the
//! rows whose *suit* equals the card's value otherwise (the PC's
//! `00752cb0` / `00752d40`); a king's worth is counted once when the track is
//! under 21 and twice when it's sold; the queen move on the player's tracks
//! never happens (it asks for an empty row inside a loop over rows that
//! aren't).

use super::{winning_state, Game, JACK, JOKER, KING, ROWS, TRACKS};

/// `CaravanAIPackage::ActionType`.
pub mod action {
    pub const BUILD_TRACK: i32 = 0;
    pub const SHIFT_DIRECTION: i32 = 1;
    pub const SAVE_TRACK: i32 = 2;
    pub const USE_JOKER: i32 = 3;
    pub const SETUP_TRACK: i32 = 4;
    pub const DESTROY_TRACK: i32 = 5;
    pub const DISCARD_CARD: i32 = 6;
    pub const DESTROY_PLAYER: i32 = 7;
}

/// `CaravanAIPackage` (28 bytes: `iValueGoal`, `iCalcResults`,
/// `iActionType`, `iTrack`, `iRow`, `iDeckCard`, `iPriority`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Package {
    pub value_goal: i32,
    pub calc_results: i32,
    pub action: i32,
    pub track: usize,
    pub row: usize,
    pub card: usize,
    pub priority: i32,
}

/// Visual Studio 2008's `qsort` (`00ec6f20`, `shortsort` `00ec6e90`):
/// median-of-three quicksort down to eight, then selection of the largest
/// to the end.
pub fn crt_qsort<T>(v: &mut [T], cmp: &dyn Fn(&T, &T) -> i32) {
    if v.len() < 2 {
        return;
    }
    let shortsort = |v: &mut [T], lo: usize, mut hi: usize| {
        while hi > lo {
            let mut max = lo;
            for p in lo + 1..=hi {
                if cmp(&v[p], &v[max]) > 0 {
                    max = p;
                }
            }
            v.swap(max, hi);
            hi -= 1;
        }
    };
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let (mut lo, mut hi) = (0usize, v.len() - 1);
    loop {
        let size = hi - lo + 1;
        if size <= 8 {
            shortsort(v, lo, hi);
        } else {
            let mut mid = lo + size / 2;
            if cmp(&v[lo], &v[mid]) > 0 {
                v.swap(lo, mid);
            }
            if cmp(&v[lo], &v[hi]) > 0 {
                v.swap(lo, hi);
            }
            if cmp(&v[mid], &v[hi]) > 0 {
                v.swap(mid, hi);
            }
            // Indices as signed so higuy can pass below lo as the C's
            // pointers do.
            let (mut loguy, mut higuy) = (lo as isize, hi as isize);
            loop {
                if (mid as isize) > loguy {
                    loop {
                        loguy += 1;
                        if loguy >= mid as isize || cmp(&v[loguy as usize], &v[mid]) > 0 {
                            break;
                        }
                    }
                }
                if (mid as isize) <= loguy {
                    loop {
                        loguy += 1;
                        if loguy > hi as isize || cmp(&v[loguy as usize], &v[mid]) > 0 {
                            break;
                        }
                    }
                }
                loop {
                    higuy -= 1;
                    if higuy <= mid as isize || cmp(&v[higuy as usize], &v[mid]) <= 0 {
                        break;
                    }
                }
                if higuy < loguy {
                    break;
                }
                v.swap(loguy as usize, higuy as usize);
                if mid as isize == higuy {
                    mid = loguy as usize;
                }
            }
            higuy += 1;
            if (mid as isize) < higuy {
                loop {
                    higuy -= 1;
                    if higuy <= mid as isize || cmp(&v[higuy as usize], &v[mid]) != 0 {
                        break;
                    }
                }
            }
            if (mid as isize) >= higuy {
                loop {
                    higuy -= 1;
                    if higuy <= lo as isize || cmp(&v[higuy as usize], &v[mid]) != 0 {
                        break;
                    }
                }
            }
            if higuy - lo as isize >= hi as isize - loguy {
                if (lo as isize) < higuy {
                    stack.push((lo, higuy as usize));
                }
                if loguy < hi as isize {
                    lo = loguy as usize;
                    continue;
                }
            } else {
                if loguy < hi as isize {
                    stack.push((loguy as usize, hi));
                }
                if (lo as isize) < higuy {
                    hi = higuy as usize;
                    continue;
                }
            }
        }
        match stack.pop() {
            Some((l, h)) => (lo, hi) = (l, h),
            None => return,
        }
    }
}

/// The comparison the packages are sorted with (`0073b6e0`): higher
/// priority first.
fn by_priority(a: &Package, b: &Package) -> i32 {
    if a.priority < b.priority {
        1
    } else if b.priority < a.priority {
        -1
    } else {
        0
    }
}

impl Game {
    /// A row's worth: its number card doubled for each king on it, 0 for an
    /// empty row (`0074f600`).
    pub fn row_value(&self, track: usize, row: usize) -> i32 {
        let cards = &self.tracks[track][row];
        match cards.first() {
            None => 0,
            Some(n) => cards
                .iter()
                .skip(1)
                .filter(|c| c.value == KING)
                .fold(n.value, |v, _| v << 1),
        }
    }

    /// The rows in use whose number card has this value, times the value
    /// (PC `00752cb0`).
    fn value_in_track(&self, track: usize, value: i32) -> i32 {
        let n = (0..ROWS)
            .take_while(|&r| !self.tracks[track][r].is_empty())
            .filter(|&r| self.tracks[track][r][0].value == value)
            .count() as i32;
        n * value
    }

    /// The values of the rows in use whose number card has this suit (PC
    /// `00752d40`).
    fn suit_in_track(&self, track: usize, suit: i32) -> i32 {
        (0..ROWS)
            .take_while(|&r| !self.tracks[track][r].is_empty())
            .filter(|&r| self.tracks[track][r][0].suit == suit)
            .map(|r| self.tracks[track][r][0].value)
            .sum()
    }

    /// The first empty row of a track (7 when full).
    fn first_empty_row(&self, track: usize) -> usize {
        (0..ROWS)
            .find(|&r| self.tracks[track][r].is_empty())
            .unwrap_or(ROWS)
    }

    /// The opponent's move (`ProcessAI`, `0074fdc0`). `difficulty` is
    /// `ShowCaravanMenu`'s (`iDiffLevel`: 0 easy, 1 normal, 2 track
    /// buster, 3 hard).
    pub fn process_ai(&mut self, difficulty: i32) -> Package {
        for t in 0..TRACKS {
            self.update_track_value(t);
        }
        if self.setup {
            return self.setup_package(difficulty);
        }
        let hand = self.npc_hand.clone();
        let valid = |g: &Game, t: usize, r: usize, i: usize| g.is_valid_placement(t, r, i, true);
        let value = |g: &Game, t: usize| g.info[t].value;
        let mut packages: Vec<Package> = Vec::new();
        let mut cur = Package::default();
        // A package kept: a fresh one follows with the loop's place in it.
        let add = |packages: &mut Vec<Package>,
                   cur: &mut Package,
                   row: usize,
                   card: usize,
                   track: usize| {
            packages.push(*cur);
            *cur = Package {
                row,
                card,
                track,
                ..Package::default()
            };
        };
        for t in 3..TRACKS {
            cur.track = t;
            let v = value(self, t);
            let opp = value(self, 5 - t);
            if winning_state(v) < 0 {
                for (i, card) in hand.iter().enumerate() {
                    cur.card = i;
                    if card.is_number() {
                        let row = self.first_empty_row(t);
                        cur.row = row;
                        if row != ROWS {
                            let nv = card.value + v;
                            if winning_state(nv) < 1 && valid(self, t, row, i) {
                                cur.value_goal = nv;
                                cur.calc_results = winning_state(nv);
                                cur.priority = nv;
                                cur.action = action::BUILD_TRACK;
                                add(&mut packages, &mut cur, row, i, t);
                            }
                        }
                    } else if card.value == KING {
                        let mut r = 0;
                        while r < ROWS && !self.tracks[t][r].is_empty() {
                            cur.row = r;
                            if self.tracks[t][r].len() < 4 {
                                let kv = self.row_value(t, r) + v;
                                if winning_state(kv) < 1 && valid(self, t, r, i) {
                                    cur.value_goal = kv;
                                    cur.calc_results = winning_state(kv);
                                    cur.priority = kv + if difficulty != 2 { 1 } else { -5 };
                                    cur.action = action::BUILD_TRACK;
                                    add(&mut packages, &mut cur, r, i, t);
                                }
                            }
                            r += 1;
                        }
                    }
                }
            } else if winning_state(v) == 0 {
                if v < opp && opp < 27 {
                    for (i, card) in hand.iter().enumerate() {
                        cur.card = i;
                        if card.is_number() {
                            let row = self.first_empty_row(t);
                            cur.row = row;
                            let nv = card.value + v;
                            if nv < 27 && opp < nv {
                                cur.priority = 100 - (nv - opp);
                                cur.action = action::BUILD_TRACK;
                            } else if nv < 27 && nv == opp {
                                cur.priority = 50;
                                cur.action = action::BUILD_TRACK;
                            } else if nv < 27 {
                                cur.priority = 25 - (26 - nv);
                                cur.action = action::BUILD_TRACK;
                            }
                            if valid(self, t, row, i) {
                                cur.value_goal = nv;
                                cur.calc_results = winning_state(nv);
                                add(&mut packages, &mut cur, row, i, t);
                            }
                        } else if card.value == JACK {
                            let gap = opp - v;
                            for r in 0..ROWS {
                                cur.row = r;
                                if self.tracks[5 - t][r].is_empty() {
                                    break;
                                }
                                let d = self.row_value(5 - t, r);
                                if gap < d {
                                    let over = v - (opp - d);
                                    cur.priority =
                                        if opp - d < 22 { over + 75 } else { 100 - over };
                                    if valid(self, t, r, i) {
                                        cur.value_goal = v;
                                        cur.calc_results = winning_state(v);
                                        cur.action = action::DESTROY_PLAYER;
                                        add(&mut packages, &mut cur, r, i, t);
                                    }
                                }
                            }
                        } else if difficulty > 0 && card.value == JOKER {
                            self.joker_packages(&mut packages, &mut cur, t, i, difficulty);
                        }
                    }
                } else {
                    for (i, card) in hand.iter().enumerate() {
                        cur.card = i;
                        if card.is_number() {
                            let row = self.first_empty_row(t);
                            cur.row = row;
                            let nv = card.value + v;
                            if valid(self, t, row, i) && winning_state(nv) < 1 {
                                cur.value_goal = nv;
                                cur.calc_results = winning_state(nv);
                                cur.priority = nv;
                                cur.action = action::BUILD_TRACK;
                                add(&mut packages, &mut cur, row, i, t);
                            }
                        } else if card.value == KING {
                            for r in 0..ROWS {
                                cur.row = r;
                                let kv = v + self.row_value(t, r) * 2;
                                if winning_state(kv) < 1 && valid(self, t, r, i) {
                                    cur.value_goal = kv;
                                    cur.calc_results = winning_state(kv);
                                    cur.priority = kv;
                                    cur.action = action::BUILD_TRACK;
                                    add(&mut packages, &mut cur, r, i, t);
                                }
                            }
                        }
                    }
                }
            } else {
                for (i, card) in hand.iter().enumerate() {
                    cur.card = i;
                    if card.value == JACK {
                        for r in 0..ROWS {
                            cur.row = r;
                            let rv = self.row_value(t, r);
                            if v - rv < 27 && valid(self, t, r, i) {
                                cur.value_goal = v - rv;
                                cur.calc_results = winning_state(v - rv);
                                cur.priority = 100 - ((26 - v) + rv);
                                cur.action = action::SAVE_TRACK;
                                add(&mut packages, &mut cur, r, i, t);
                            }
                        }
                    }
                }
            }
        }
        // Face cards on the player's tracks (no placement check).
        for t in 0..3 {
            cur.track = t;
            for (i, card) in hand.iter().enumerate() {
                cur.card = i;
                if card.is_number() {
                    continue;
                }
                let mut r = 0;
                while r < ROWS && !self.tracks[t][r].is_empty() {
                    cur.row = r;
                    if self.tracks[t][r].len() < 4 {
                        let pv = value(self, t);
                        if card.value == JACK {
                            let left = pv - self.row_value(t, r);
                            let state = winning_state(pv);
                            if state <= 0 {
                                cur.priority = if state == 0 { 51 - left } else { 26 - left };
                                cur.action = action::DESTROY_PLAYER;
                                add(&mut packages, &mut cur, r, i, t);
                            }
                        } else if card.value == KING {
                            let kv = self.row_value(t, r) + pv;
                            if winning_state(kv) == 1 {
                                cur.priority = kv - 1;
                                cur.action = action::DESTROY_PLAYER;
                                add(&mut packages, &mut cur, r, i, t);
                            }
                        }
                    }
                    r += 1;
                }
            }
        }
        if packages.is_empty() {
            // Nothing to play: throw away the highest card, or a track.
            let mut best = 0;
            for (i, card) in hand.iter().enumerate() {
                if hand[best].value < card.value {
                    best = i;
                }
            }
            cur.card = best;
            cur.action = action::DISCARD_CARD;
            cur.priority = best as i32;
            packages.push(cur);
            cur = Package::default();
            for t in 3..TRACKS {
                let v = value(self, t);
                if winning_state(v) != 0 {
                    cur.track = t;
                    cur.action = action::DESTROY_TRACK;
                    cur.priority = if difficulty < 1 {
                        v - 26
                    } else if difficulty < 3 {
                        v * 2 - 52
                    } else {
                        let jacks = hand.iter().filter(|c| c.value == JACK).count() as i32;
                        v * 2 - 52 - jacks * 5
                    };
                    packages.push(cur);
                    cur = Package {
                        track: t,
                        ..Package::default()
                    };
                }
            }
        }
        crt_qsort(&mut packages, &by_priority);
        packages[0]
    }

    /// A joker on one of the opponent's rows of track `t` (the part of
    /// `0074fdc0` for difficulty above 0).
    fn joker_packages(
        &self,
        packages: &mut Vec<Package>,
        cur: &mut Package,
        t: usize,
        i: usize,
        difficulty: i32,
    ) {
        let value = |t: usize| self.info[t].value;
        for e in 0..ROWS {
            cur.row = e;
            if self.tracks[t][e].is_empty() {
                break;
            }
            if !self.is_valid_placement(t, e, i, true) {
                continue;
            }
            let base = self.tracks[t][e][0].value;
            let mut est = [0i32; TRACKS];
            for (k, e) in est.iter_mut().enumerate() {
                *e = if base == 1 {
                    self.value_in_track(k, base)
                } else {
                    self.suit_in_track(k, base)
                };
            }
            est[t] += base;
            let push = |packages: &mut Vec<Package>, cur: &mut Package, priority: i32| {
                cur.priority = priority;
                cur.action = action::USE_JOKER;
                packages.push(*cur);
                *cur = Package {
                    row: e,
                    card: i,
                    track: t,
                    ..Package::default()
                };
            };
            if difficulty < 3 {
                let d = (value(t) - est[t]) - value(5 - t) - est[5 - t];
                if d > 0 {
                    push(packages, cur, d + 50);
                }
                continue;
            }
            let mut s = [0i32; 3];
            for (k, s) in s.iter_mut().enumerate() {
                *s = (value(k + 3) - est[k + 3]) - value(k) - est[k];
            }
            let mut score = if s[5 - t] > 0 { 75 } else { 0 };
            for k in 0..3 {
                let (nv, ne, pv, pe) = (value(k + 3), est[k + 3], value(k), est[k]);
                if 20 < nv && nv < 27 && nv - ne < 21 {
                    score -= 25 - (21 - (nv - ne));
                }
                if winning_state(nv) >= 0 {
                    if nv < 27 && nv - ne < 21 {
                        score -= (21 - nv) - ne;
                    }
                    if nv < 27 && s[k] < 0 && winning_state(pv - pe) == 0 {
                        score -= s[k] + 25;
                    }
                    if winning_state(nv) > 0 && winning_state(nv - ne) < 1 {
                        score += nv - ne;
                    }
                }
                if winning_state(nv) > 0 && ne > 0 {
                    let x = nv - ne;
                    score += match winning_state(x) {
                        0 => 25 + x,
                        s if s < 0 => x,
                        _ => ne,
                    };
                }
                if winning_state(pv) == 0 && winning_state(pv - pe) < 0 {
                    score += (46 - pv) + pe;
                }
                if score > 0 && self.is_valid_placement(t, e, i, true) {
                    push(packages, cur, score);
                }
            }
        }
    }

    /// Starting its caravans (`0074fdc0`'s other half): which card to play
    /// or throw away.
    fn setup_package(&self, difficulty: i32) -> Package {
        let hand = &self.npc_hand;
        let mut p = Package {
            action: action::SETUP_TRACK,
            ..Package::default()
        };
        let numbers = hand.iter().filter(|c| c.is_number()).count();
        // Which face card goes: a joker at once; else the first face card,
        // a jack giving way to any later one, a king to any but a jack.
        let face_to_throw = |p: &mut Package| {
            p.action = action::DISCARD_CARD;
            let mut chosen = 0;
            for (i, c) in hand.iter().enumerate() {
                if c.is_number() {
                    continue;
                }
                if c.value == JOKER {
                    p.card = i;
                    break;
                }
                if chosen == 0 || chosen == JACK || (chosen == KING && c.value != JACK) {
                    p.card = i;
                    chosen = c.value;
                }
            }
        };
        if difficulty < 1 {
            let last_number = hand.iter().rposition(|c| c.is_number()).unwrap_or(0);
            let last_face = hand.iter().rposition(|c| !c.is_number()).unwrap_or(0);
            if hand.len() - numbers < 6 {
                p.card = last_number;
            } else {
                p.action = action::DISCARD_CARD;
                p.card = last_face;
            }
        } else if difficulty < 3 {
            let mid = hand.iter().filter(|c| (6..=10).contains(&c.value)).count();
            let low = hand.iter().filter(|c| c.value < 4).count();
            if hand.len() - low - mid > 5 {
                face_to_throw(&mut p);
            } else if low < mid {
                if let Some(i) = hand.iter().position(|c| (6..=10).contains(&c.value)) {
                    p.card = i;
                }
            } else if low != 0 {
                if let Some(i) = hand.iter().position(|c| c.value < 4) {
                    p.card = i;
                }
            }
        } else if hand.len() - numbers > 5 {
            face_to_throw(&mut p);
        } else {
            // The card nearest 6.
            let mut best = (hand[0].value - 6).abs();
            for (i, c) in hand.iter().enumerate() {
                if (c.value - 6).abs() < best {
                    p.card = i;
                    best = (c.value - 6).abs();
                }
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caravan::{Card, Turn, HAND};
    use esm::FormId;

    fn c(value: i32, suit: i32) -> Card {
        Card {
            form: FormId((value * 10 + suit) as u32),
            value,
            suit,
        }
    }

    /// A full deck: four suits of ace to king and two jokers (54).
    fn full_deck() -> Vec<Card> {
        let mut d = Vec::new();
        for suit in 1..=4 {
            for value in (1..=10).chain(12..=14) {
                d.push(c(value, suit));
            }
        }
        d.push(c(JOKER, 5));
        d.push(c(JOKER, 5));
        d
    }

    /// A small generator for the tests (xorshift).
    fn dice(seed: u64) -> impl FnMut(usize) -> usize {
        let mut x = seed.max(1);
        move |n| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x % n as u64) as usize
        }
    }

    /// `00ec6f20`: sorted highest first; ties fall as Visual Studio 2008's
    /// `shortsort` leaves them (the largest-comparing, here the lowest,
    /// picked first-found and moved to the end).
    #[test]
    fn the_exes_sort() {
        let p = |priority: i32, card: usize| Package {
            priority,
            card,
            ..Package::default()
        };
        let mut v = vec![p(1, 0), p(2, 1), p(2, 2), p(1, 3)];
        crt_qsort(&mut v, &by_priority);
        let order: Vec<usize> = v.iter().map(|p| p.card).collect();
        assert_eq!(order, vec![1, 2, 3, 0]);
        // Longer: the quicksort part.
        let mut roll = dice(7);
        let mut v: Vec<Package> = (0..200).map(|i| p((roll(40) as i32) - 20, i)).collect();
        crt_qsort(&mut v, &by_priority);
        assert!(v.windows(2).all(|w| w[0].priority >= w[1].priority));
    }

    /// Under 21 a number card builds the track as high as it can go
    /// without passing 26, the way the track goes (up in spades here: the
    /// 2 of hearts can't follow, the 4 of spades can).
    #[test]
    fn building_a_track() {
        let mut g = Game::new(Vec::new(), Vec::new(), &mut |_| 0);
        g.setup = false;
        g.tracks[5][0] = vec![c(2, 1)];
        g.tracks[5][1] = vec![c(5, 2)];
        g.tracks[4][0] = vec![c(2, 3)];
        g.tracks[3][0] = vec![c(3, 4)];
        g.npc_hand = vec![c(2, 1), c(9, 3), c(4, 2)];
        let p = g.process_ai(1);
        assert_eq!(
            (p.action, p.track, p.row, p.card, p.priority),
            (action::BUILD_TRACK, 5, 2, 1, 16)
        );
    }

    /// Nothing to play: the highest card goes.
    #[test]
    fn throwing_away() {
        let mut g = Game::new(Vec::new(), Vec::new(), &mut |_| 0);
        g.setup = false;
        for t in 3..6 {
            g.tracks[t][0] = vec![c(10, 1), c(KING, 1)];
            g.tracks[t][1] = vec![c(6, 1)];
        }
        g.npc_hand = vec![c(QUEEN_TEST, 2), c(QUEEN_TEST, 3)];
        let p = g.process_ai(1);
        assert_eq!((p.action, p.card), (action::DISCARD_CARD, 0));
    }
    const QUEEN_TEST: i32 = super::super::QUEEN;

    /// Starting the caravans: an easy opponent plays its last number card
    /// with enough of them, else throws its last face card away.
    #[test]
    fn starting_the_caravans() {
        let mut g = Game::new(Vec::new(), Vec::new(), &mut |_| 0);
        g.npc_hand = vec![c(3, 1), c(JACK, 1), c(7, 2), c(KING, 3)];
        let p = g.setup_package(0);
        assert_eq!((p.action, p.card), (action::SETUP_TRACK, 2));
        g.npc_hand = vec![
            c(JACK, 1),
            c(KING, 2),
            c(QUEEN_TEST, 3),
            c(JACK, 2),
            c(KING, 4),
            c(JOKER, 5),
            c(2, 1),
        ];
        let p = g.setup_package(0);
        assert_eq!((p.action, p.card), (action::DISCARD_CARD, 5));
        // Harder: the card nearest 6.
        g.npc_hand = vec![c(1, 1), c(9, 1), c(5, 2), c(JACK, 1)];
        let p = g.setup_package(3);
        assert_eq!((p.action, p.card), (action::SETUP_TRACK, 2));
    }

    /// Whole games against a player who plays the first card that fits:
    /// every one ends.
    #[test]
    fn whole_games_end() {
        for seed in 1..40u64 {
            let mut roll = dice(seed);
            let mut g = Game::new(full_deck(), full_deck(), &mut roll);
            assert_eq!(g.player_hand.len(), HAND);
            let mut turns = 0;
            let mut npc = true;
            loop {
                if g.is_game_over() {
                    break;
                }
                turns += 1;
                assert!(turns < 2000, "seed {seed}: no end");
                if npc {
                    let (_, again) = g.npc_turn((seed % 4) as i32, &mut roll);
                    if again {
                        continue;
                    }
                } else {
                    let mut moved = false;
                    'find: for i in 0..g.player_hand.len() {
                        for t in 0..6 {
                            for r in 0..7 {
                                if (!g.setup || Some(t) == g.setup_track(false))
                                    && g.is_valid_placement(t, r, i, false)
                                {
                                    g.player_play(i, t, r, &mut roll);
                                    moved = true;
                                    break 'find;
                                }
                            }
                        }
                    }
                    if !moved {
                        if g.player_hand.is_empty() {
                            break;
                        }
                        g.discard(false, 0, &mut roll);
                        if g.setup {
                            continue;
                        }
                    }
                }
                npc = !npc;
            }
            let _ = Turn::DiscardedTrack(0);
        }
    }
}
