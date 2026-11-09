//! Party reducers: how a script condition on "the player" is answered over several players.
//!
//! Rows carry `party_cond: function -> reducer` strings (`initiator`, `any-party-member`,
//! `sum-over-party`, `entry:all-in,exit:any-in` ...). [`Reducer::parse`] reads them and
//! [`Reducer::reduce`] folds one value per present player into the single value the script sees.

use crate::engine::{PlayerMask, Slot};

/// Which phase a script is asking in (for the entry/exit reducers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Entry,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reducer {
    /// The player who started the interaction.
    Initiator,
    /// The host player.
    Host,
    /// True when any present player is true (non-zero).
    Any,
    /// True only when all present players are.
    All,
    Sum,
    Min,
    Max,
    /// All must be in to enter, any in keeps the scene going.
    EntryAllExitAny,
}

impl Reducer {
    /// Parses a reducer name from the table. Unknown names give `None`.
    pub fn parse(s: &str) -> Option<Reducer> {
        Some(match s.trim() {
            "initiator" => Reducer::Initiator,
            "host" => Reducer::Host,
            "any" | "any-party-member" => Reducer::Any,
            "all" | "all-party-members" => Reducer::All,
            "sum-over-party" => Reducer::Sum,
            "min-over-party" => Reducer::Min,
            "max-over-party" => Reducer::Max,
            "entry:all-in,exit:any-in" => Reducer::EntryAllExitAny,
            _ => return None,
        })
    }

    /// Folds `values` (one per slot, only slots in `present` count). `initiator` is
    /// required for [`Reducer::Initiator`]; without one the host player's value is used.
    /// Returns `None` when no present player has a value.
    pub fn reduce(
        self,
        present: PlayerMask,
        initiator: Option<Slot>,
        phase: Phase,
        values: &[(Slot, f64)],
    ) -> Option<f64> {
        let vs: Vec<(Slot, f64)> = values
            .iter()
            .copied()
            .filter(|(s, _)| present.contains(*s))
            .collect();
        if vs.is_empty() {
            return None;
        }
        let of = |slot: Slot| vs.iter().find(|(s, _)| *s == slot).map(|(_, v)| *v);
        let b = |x: bool| if x { 1.0 } else { 0.0 };
        Some(match self {
            Reducer::Initiator => initiator.and_then(of).or_else(|| of(0)).unwrap_or(vs[0].1),
            Reducer::Host => of(0).unwrap_or(vs[0].1),
            Reducer::Any => b(vs.iter().any(|(_, v)| *v != 0.0)),
            Reducer::All => b(vs.iter().all(|(_, v)| *v != 0.0)),
            Reducer::Sum => vs.iter().map(|(_, v)| v).sum(),
            Reducer::Min => vs.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min),
            Reducer::Max => vs.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max),
            Reducer::EntryAllExitAny => match phase {
                Phase::Entry => b(vs.iter().all(|(_, v)| *v != 0.0)),
                Phase::Exit => b(vs.iter().any(|(_, v)| *v != 0.0)),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn m(s: &[u8]) -> PlayerMask {
        s.iter()
            .fold(PlayerMask::NONE, |a, x| a.or(PlayerMask::of(*x)))
    }
    #[test]
    fn every_name_in_the_bundled_table_parses() {
        let t = crate::PolicyTable::bundled().unwrap();
        for r in t.rows() {
            for (f, red) in &r.party_cond {
                assert!(Reducer::parse(red).is_some(), "{} {f}: {red}", r.edid);
            }
        }
    }
    #[test]
    fn folds() {
        let v = [(0u8, 2.0), (1, 0.0), (2, 5.0)];
        let p = m(&[0, 1, 2]);
        let r = |x: Reducer, ph| x.reduce(p, Some(2), ph, &v).unwrap();
        assert_eq!(r(Reducer::Initiator, Phase::Entry), 5.0);
        assert_eq!(r(Reducer::Host, Phase::Entry), 2.0);
        assert_eq!(r(Reducer::Any, Phase::Entry), 1.0);
        assert_eq!(r(Reducer::All, Phase::Entry), 0.0);
        assert_eq!(r(Reducer::Sum, Phase::Entry), 7.0);
        assert_eq!(r(Reducer::Min, Phase::Entry), 0.0);
        assert_eq!(r(Reducer::Max, Phase::Entry), 5.0);
        assert_eq!(r(Reducer::EntryAllExitAny, Phase::Entry), 0.0);
        assert_eq!(r(Reducer::EntryAllExitAny, Phase::Exit), 1.0);
    }
    #[test]
    fn absent_players_do_not_count() {
        let v = [(0u8, 1.0), (1, 0.0)];
        assert_eq!(
            Reducer::All.reduce(m(&[0]), None, Phase::Entry, &v),
            Some(1.0)
        );
        assert_eq!(Reducer::All.reduce(m(&[3]), None, Phase::Entry, &v), None);
    }
}
