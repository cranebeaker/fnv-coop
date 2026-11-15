//! roster: every player's name in every game (docs/coop/SPECS.md, "voice list").
//!
//! The network knows names only where a player joined: the host has every guest's (each said its
//! name in its join), a guest has only "host" for the host and nothing for the other guests. So
//! the host writes the party's names into the replicated store, one record ([`ROSTER_TAG`] on form
//! 0): every slot in the session with its name, the host's own included. The store goes to a guest
//! with its first snapshot (a join, a rejoin, a late join) and every change after it, so each game
//! knows each player by name, and per-name mutes and volumes work for everyone.
//!
//! The record is the host's: a guest never writes it, and a guest's game does nothing with it but
//! read it (the guest's apply leaves custom records it does not know alone).

use std::collections::BTreeMap;

use coop_core::delta::StateStore;
use coop_core::ids::{FormId, PlayerId};
use coop_core::schema::{Field, MAX_CUSTOM_BYTES};
use coop_core::sync::HostSync;

/// The tag of the party's names (`Field::Custom` on form 0, written by the host).
pub const ROSTER_TAG: u16 = 140;
/// The longest name kept, in characters (the launcher allows 24).
pub const MAX_NAME: usize = 24;

/// A name as every game shows and keys it: letters, digits, spaces, `-` and `_` (the launcher's
/// rule for a name; anything else becomes `_`), at most [`MAX_NAME`] characters, trimmed. An
/// empty name is `player N`.
pub fn clean(name: &str, slot: PlayerId) -> String {
    let n: String = name
        .trim()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || matches!(c, ' ' | '-' | '_') {
                c
            } else {
                '_'
            }
        })
        .take(MAX_NAME)
        .collect();
    let n = n.trim().to_string();
    if n.is_empty() {
        format!("player {}", slot.0)
    } else {
        n
    }
}

/// The record of `names`: each slot, the name's length in bytes and its UTF-8 bytes. Names that
/// would not fit the record are left out (eight players of 24 characters fit many times over).
pub fn field(names: &BTreeMap<PlayerId, String>) -> Field {
    let mut data = Vec::new();
    for (p, n) in names {
        let b = n.as_bytes();
        let b = &b[..b.len().min(u8::MAX as usize)];
        if data.len() + 2 + b.len() > MAX_CUSTOM_BYTES {
            break;
        }
        data.push(p.0);
        data.push(b.len() as u8);
        data.extend_from_slice(b);
    }
    Field::Custom {
        tag: ROSTER_TAG,
        data,
    }
}

/// The names in a record; `None` for another field or a cut one.
pub fn names_of(field: &Field) -> Option<BTreeMap<PlayerId, String>> {
    let Field::Custom { tag, data } = field else {
        return None;
    };
    if *tag != ROSTER_TAG {
        return None;
    }
    let mut out = BTreeMap::new();
    let mut at = 0;
    while at < data.len() {
        let slot = *data.get(at)?;
        let len = usize::from(*data.get(at + 1)?);
        let bytes = data.get(at + 2..at + 2 + len)?;
        out.insert(
            PlayerId(slot),
            clean(&String::from_utf8_lossy(bytes), PlayerId(slot)),
        );
        at += 2 + len;
    }
    Some(out)
}

/// The names the host's record holds now (empty before the first snapshot).
pub fn read(store: &StateStore) -> BTreeMap<PlayerId, String> {
    let probe = Field::Custom {
        tag: ROSTER_TAG,
        data: Vec::new(),
    };
    store
        .get(FormId(0), &probe)
        .and_then(|en| names_of(&en.field))
        .unwrap_or_default()
}

/// The host's side: the names it was told, written for the players in the session.
#[derive(Debug, Default)]
pub struct HostRoster {
    told: BTreeMap<PlayerId, String>,
    written: Option<BTreeMap<PlayerId, String>>,
    /// Records written.
    pub writes: u32,
}

impl HostRoster {
    /// Player `p` is called `name` (the host's own as slot 0; a guest's from its join, again on a
    /// rejoin).
    pub fn set(&mut self, p: PlayerId, name: &str) {
        self.told.insert(p, clean(name, p));
    }

    /// The names of the players in the session now (the host and every connected guest).
    pub fn names(&self, sync: &HostSync) -> BTreeMap<PlayerId, String> {
        let mut in_session = sync.players();
        in_session.push(PlayerId::HOST);
        in_session
            .into_iter()
            .map(|p| {
                let n = self.told.get(&p).cloned().unwrap_or_else(|| clean("", p));
                (p, n)
            })
            .collect()
    }

    /// One step, before the host's tick sends the store's changes: the record is written again
    /// when someone joined, left or was named anew.
    pub fn step(&mut self, sync: &mut HostSync) {
        let now = self.names(sync);
        if self.written.as_ref() == Some(&now) {
            return;
        }
        sync.write(FormId(0), field(&now));
        self.written = Some(now);
        self.writes += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_are_cleaned() {
        let mut m = BTreeMap::new();
        m.insert(PlayerId::HOST, "Ann".to_string());
        m.insert(PlayerId(2), "Bo Ek".to_string());
        m.insert(PlayerId(3), "Zoë-9".to_string());
        assert_eq!(names_of(&field(&m)), Some(m.clone()));
        assert_eq!(clean("  a;b=c  ", PlayerId(1)), "a_b_c");
        assert_eq!(clean("", PlayerId(4)), "player 4");
        assert_eq!(
            clean(&"x".repeat(40), PlayerId(1)).chars().count(),
            MAX_NAME
        );
        // Another tag, or a record cut short, is no roster.
        assert_eq!(
            names_of(&Field::Custom {
                tag: 141,
                data: vec![]
            }),
            None
        );
        let Field::Custom { mut data, .. } = field(&m) else {
            unreachable!()
        };
        data.pop();
        assert_eq!(
            names_of(&Field::Custom {
                tag: ROSTER_TAG,
                data
            }),
            None
        );
    }

    #[test]
    fn a_full_record_stays_within_a_custom_field() {
        let m: BTreeMap<PlayerId, String> = (0..=255u8)
            .map(|i| (PlayerId(i), "y".repeat(MAX_NAME)))
            .collect();
        let f = field(&m);
        assert!(f.is_sane());
        assert!(names_of(&f).unwrap().len() < m.len());
    }
}
