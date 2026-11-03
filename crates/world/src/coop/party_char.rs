//! A guest's own character as a file, and how a character that comes from elsewhere joins a
//! campaign.
//!
//! * [`CharacterFile`]: what a guest keeps locally when the host orders a save (and what it
//!   offers when it joins a campaign with a character it saved before). One shadow blob, the
//!   persistent player id, the campaign it belongs to and a baseline of the shared quest stages
//!   at save time.
//! * [`sanitize_offer`]: the host's rule for a character it did not make: the character is the
//!   player's (inventory, skills, perks, level, name), but anything the campaign shares wins
//!   over what the file says. Per-player quest rows for quests this campaign shares are dropped,
//!   synthetic forms are removed and, for a character that joins late, the old place is cleared
//!   because the host's place wins.
//! * [`baseline_diff`]: the shared quest stages the file remembers that differ from the host's.
//!   Informational: the host's value is what the guest plays with.

use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use esm::FormId;

use super::party_save::{fnv64, is_synthetic, scrub_shadow};
use super::shadow::{decode, encode, PlayerShadow, ShadowPlan};
use crate::scripting::GameState;

const MAGIC: &[u8; 4] = b"NVCH";
const VERSION: u8 = 1;
/// Longest name kept.
const MAX_NAME: usize = 64;
/// Most baseline entries kept (a save with more is cut, never refused).
const MAX_BASELINE: usize = 20_000;

/// A guest's character on disk.
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterFile {
    /// Persistent player id.
    pub key: u64,
    /// Display name.
    pub name: String,
    /// Campaign the character was saved in (0: from outside any campaign).
    pub campaign: u64,
    /// The host save sequence this was saved for (0: none).
    pub seq: u32,
    /// Seconds since the Unix epoch.
    pub saved_unix: u64,
    /// Shared quest stages when the character was saved, sorted by quest.
    pub baseline: Vec<(u32, u16)>,
    /// The character (a shadow blob).
    pub blob: Vec<u8>,
}

/// Why a character file could not be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharError {
    /// I/O failure.
    Io(String),
    /// Not a character file, truncated or edited.
    Corrupt,
    /// Made by a newer build.
    Version(u8),
}

impl std::fmt::Display for CharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharError::Io(e) => write!(f, "character file: {e}"),
            CharError::Corrupt => write!(f, "character file is damaged"),
            CharError::Version(v) => write!(
                f,
                "character file is format {v}, this build reads {VERSION}"
            ),
        }
    }
}

impl std::error::Error for CharError {}

impl CharacterFile {
    /// Builds a file from a character and the current shared stages.
    pub fn new(
        key: u64,
        name: &str,
        campaign: u64,
        seq: u32,
        now: u64,
        s: &PlayerShadow,
        baseline: Vec<(u32, u16)>,
    ) -> Self {
        let mut name = name.to_string();
        name.truncate(
            name.char_indices()
                .nth(MAX_NAME)
                .map_or(name.len(), |(i, _)| i),
        );
        CharacterFile {
            key,
            name,
            campaign,
            seq,
            saved_unix: now,
            baseline,
            blob: encode(s),
        }
    }

    /// The character, `None` when the blob does not parse.
    pub fn shadow(&self) -> Option<PlayerShadow> {
        decode(&self.blob)
    }

    /// Serializes with a trailing hash.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(self.blob.len() + 64 + self.baseline.len() * 6);
        b.extend_from_slice(MAGIC);
        b.push(VERSION);
        b.extend_from_slice(&self.key.to_le_bytes());
        b.extend_from_slice(&self.campaign.to_le_bytes());
        b.extend_from_slice(&self.seq.to_le_bytes());
        b.extend_from_slice(&self.saved_unix.to_le_bytes());
        let name = self.name.as_bytes();
        b.extend_from_slice(&(name.len() as u16).to_le_bytes());
        b.extend_from_slice(name);
        let n = self.baseline.len().min(MAX_BASELINE);
        b.extend_from_slice(&(n as u32).to_le_bytes());
        for (q, st) in self.baseline.iter().take(n) {
            b.extend_from_slice(&q.to_le_bytes());
            b.extend_from_slice(&st.to_le_bytes());
        }
        b.extend_from_slice(&(self.blob.len() as u32).to_le_bytes());
        b.extend_from_slice(&self.blob);
        let h = fnv64(&b);
        b.extend_from_slice(&h.to_le_bytes());
        b
    }

    /// Parses [`CharacterFile::to_bytes`] output.
    pub fn from_bytes(b: &[u8]) -> Result<CharacterFile, CharError> {
        if b.len() < 4 + 1 + 8 || &b[..4] != MAGIC {
            return Err(CharError::Corrupt);
        }
        if b[4] != VERSION {
            return Err(CharError::Version(b[4]));
        }
        let (body, tail) = b.split_at(b.len() - 8);
        if fnv64(body) != u64::from_le_bytes(tail.try_into().map_err(|_| CharError::Corrupt)?) {
            return Err(CharError::Corrupt);
        }
        let mut r = Rd { b: &body[5..] };
        let key = r.u64().ok_or(CharError::Corrupt)?;
        let campaign = r.u64().ok_or(CharError::Corrupt)?;
        let seq = r.u32().ok_or(CharError::Corrupt)?;
        let saved_unix = r.u64().ok_or(CharError::Corrupt)?;
        let nl = r.u16().ok_or(CharError::Corrupt)? as usize;
        let name = String::from_utf8(r.take(nl).ok_or(CharError::Corrupt)?.to_vec())
            .map_err(|_| CharError::Corrupt)?;
        let n = r.u32().ok_or(CharError::Corrupt)? as usize;
        if n > MAX_BASELINE {
            return Err(CharError::Corrupt);
        }
        let mut baseline = Vec::with_capacity(n);
        for _ in 0..n {
            baseline.push((
                r.u32().ok_or(CharError::Corrupt)?,
                r.u16().ok_or(CharError::Corrupt)?,
            ));
        }
        let bl = r.u32().ok_or(CharError::Corrupt)? as usize;
        let blob = r.take(bl).ok_or(CharError::Corrupt)?.to_vec();
        if !r.b.is_empty() || decode(&blob).is_none() {
            return Err(CharError::Corrupt);
        }
        Ok(CharacterFile {
            key,
            name,
            campaign,
            seq,
            saved_unix,
            baseline,
            blob,
        })
    }
}

struct Rd<'a> {
    b: &'a [u8],
}

impl Rd<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        if self.b.len() < n {
            return None;
        }
        let (a, rest) = self.b.split_at(n);
        self.b = rest;
        Some(a)
    }
    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|x| u16::from_le_bytes([x[0], x[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4)
            .map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
    }
    fn u64(&mut self) -> Option<u64> {
        self.take(8)
            .map(|x| u64::from_le_bytes(x.try_into().unwrap_or([0; 8])))
    }
}

/// Writes a character file atomically (temporary sibling, then rename).
pub fn write_character(path: &Path, f: &CharacterFile) -> io::Result<()> {
    use std::io::Write;
    let mut tmp = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    tmp.push(format!(".{}.tmp", std::process::id()));
    let tmp = path.with_file_name(tmp);
    if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(p)?;
    }
    let r = (|| {
        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(&f.to_bytes())?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&tmp, path)
    })();
    if r.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    r
}

/// Reads a character file.
pub fn read_character(path: &Path) -> Result<CharacterFile, CharError> {
    let b = std::fs::read(path).map_err(|e| CharError::Io(e.to_string()))?;
    CharacterFile::from_bytes(&b)
}

/// The shared quest stages in `gs` (quests the plan keeps per player are not shared), sorted.
pub fn baseline_of(gs: &GameState) -> Vec<(u32, u16)> {
    let per_player = gs.coop.bank.plan.as_ref();
    let mut v: Vec<(u32, u16)> = gs
        .stages
        .iter()
        .filter(|(q, _)| !is_synthetic(**q) && !per_player.is_some_and(|p| p.quests.contains(q)))
        .map(|(q, s)| (q.0, *s))
        .collect();
    v.sort();
    v.truncate(MAX_BASELINE);
    v
}

/// One shared stage on which a file and the host disagree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Conflict {
    /// The quest.
    pub quest: u32,
    /// Stage in the character's file (0 when the quest was not started).
    pub theirs: u16,
    /// Stage on the host (0 when not started).
    pub host: u16,
}

/// Where the character's file remembers a different shared stage than the host has. The host's
/// value is the one in force; this tells the player what changed under them.
pub fn baseline_diff(file: &[(u32, u16)], host: &GameState) -> Vec<Conflict> {
    let theirs: BTreeMap<u32, u16> = file.iter().copied().collect();
    let mut out = Vec::new();
    for (q, st) in &theirs {
        let h = host.stages.get(&FormId(*q)).copied().unwrap_or(0);
        if h != *st {
            out.push(Conflict {
                quest: *q,
                theirs: *st,
                host: h,
            });
        }
    }
    out.sort_by_key(|c| c.quest);
    out
}

/// What [`sanitize_offer`] changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sanitized {
    /// Synthetic-form rows removed.
    pub synthetic: usize,
    /// Quest rows dropped because this campaign shares the quest.
    pub shared_quest_rows: usize,
}

/// Makes a character that came from outside fit this campaign: shared state wins, personal
/// state is kept. `keep_place` keeps the cell and position (a rejoin of the same campaign);
/// `false` clears them (a late joiner is placed by the host).
pub fn sanitize_offer(
    mut s: PlayerShadow,
    plan: &ShadowPlan,
    keep_place: bool,
) -> (PlayerShadow, Sanitized) {
    let mut r = Sanitized {
        synthetic: scrub_shadow(&mut s),
        ..Sanitized::default()
    };
    let mine = |q: &FormId| plan.quests.contains(q);
    let b = s.quest_stage.len()
        + s.quest_flags.len()
        + s.stages_done.len()
        + s.objectives.len()
        + s.quest_vars.len();
    s.quest_stage.retain(|q, _| mine(q));
    s.quest_flags.retain(|q, _| mine(q));
    s.stages_done.retain(|(q, _)| mine(q));
    s.objectives.retain(|(q, _), _| mine(q));
    s.quest_vars.retain(|(q, name), _| {
        mine(q)
            && plan
                .quest_vars
                .get(q)
                .is_some_and(|v| v.contains(&name.to_ascii_lowercase()))
    });
    let a = s.quest_stage.len()
        + s.quest_flags.len()
        + s.stages_done.len()
        + s.objectives.len()
        + s.quest_vars.len();
    r.shared_quest_rows = b - a;
    if !keep_place {
        s.cell = None;
        s.world = None;
        s.position = None;
    }
    (s, r)
}

/// Puts `shadow` into a guest's slot on the host so the next sync gives it to the guest.
/// Refuses the host's own live slot. Returns false when shadows are off or the slot is live.
pub fn install_for_slot(gs: &mut GameState, slot: u8, shadow: PlayerShadow) -> bool {
    if !gs.coop.bank.enabled() || slot == gs.coop.bank.active {
        return false;
    }
    gs.coop.bank.parked.insert(slot, shadow);
    gs.coop.bank.dirty.insert(slot);
    gs.emit(super::Domain::Shadow, super::shadow::shadow_form(slot));
    true
}

/// Moves the co-op bookkeeping (authority, policy gate, parked shadows, live facts) from the
/// state a load replaces to the loaded one, and forces the host's next collect to diff
/// everything, since the whole world just changed under it.
pub fn carry_coop(old: &mut GameState, new: &mut GameState, order: Option<&esm::LoadOrder>) {
    new.coop = std::mem::take(&mut old.coop);
    // the clock globals join the coarse export grid when the clock first moves; a loaded world
    // has not moved it yet
    if let Some(order) = order {
        for name in ["GameHour", "GameDaysPassed"] {
            if let Some(id) = order.form_by_editor_id(name) {
                new.coop.clock.insert(id);
            }
        }
    }
    new.coop.out.clear();
    new.coop.overflow = true;
    let slots: Vec<u8> = new.coop.bank.parked.keys().copied().collect();
    for s in slots {
        new.coop.bank.dirty.insert(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coop::party_save::is_synthetic;

    fn shadow() -> PlayerShadow {
        let mut s = PlayerShadow::fresh();
        s.level = 6;
        s.items.insert(FormId(0xF), 120);
        s.perks.insert(FormId(0x77));
        s.cell = Some(FormId(0x100));
        s.quest_stage.insert(FormId(0xA1), 20);
        s.quest_stage.insert(FormId(0xB2), 30);
        s
    }

    #[test]
    fn file_round_trips_and_detects_damage() {
        let f = CharacterFile::new(
            9,
            "Ann",
            0xAB,
            4,
            1_700_000_000,
            &shadow(),
            vec![(1, 2), (3, 4)],
        );
        let b = f.to_bytes();
        assert_eq!(CharacterFile::from_bytes(&b).unwrap(), f);
        for cut in [0, 3, 10, b.len() - 1] {
            assert_eq!(
                CharacterFile::from_bytes(&b[..cut]),
                Err(CharError::Corrupt),
                "{cut}"
            );
        }
        let mut c = b.clone();
        c[20] ^= 1;
        assert_eq!(CharacterFile::from_bytes(&c), Err(CharError::Corrupt));
        let mut v = b.clone();
        v[4] = 9;
        assert_eq!(CharacterFile::from_bytes(&v), Err(CharError::Version(9)));
        assert_eq!(f.shadow().unwrap().level, 6);
    }

    #[test]
    fn file_write_is_atomic_and_readable() {
        let d = std::env::temp_dir().join(format!("nvrs-char-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let p = d.join("me.char");
        let f = CharacterFile::new(9, "Ann", 0xAB, 4, 5, &shadow(), vec![]);
        write_character(&p, &f).unwrap();
        assert_eq!(read_character(&p).unwrap(), f);
        assert_eq!(std::fs::read_dir(&d).unwrap().count(), 1);
        assert!(matches!(
            read_character(&d.join("none")),
            Err(CharError::Io(_))
        ));
    }

    #[test]
    fn shared_quests_win_personal_state_stays() {
        let mut plan = ShadowPlan::all();
        plan.quests.insert(FormId(0xA1));
        let mut s = shadow();
        s.items.insert(FormId(0xFE00_0001), 1);
        let (out, r) = sanitize_offer(s, &plan, false);
        assert_eq!(out.items.get(&FormId(0xF)), Some(&120));
        assert!(out.perks.contains(&FormId(0x77)));
        assert_eq!(out.level, 6);
        assert_eq!(
            out.quest_stage.get(&FormId(0xA1)),
            Some(&20),
            "a per-player quest stays"
        );
        assert!(
            !out.quest_stage.contains_key(&FormId(0xB2)),
            "a shared quest is the host's"
        );
        assert!(out.cell.is_none());
        assert_eq!(
            r,
            Sanitized {
                synthetic: 1,
                shared_quest_rows: 1
            }
        );
        assert!(!out.items.keys().any(|f| is_synthetic(*f)));
        let (kept, _) = sanitize_offer(shadow(), &plan, true);
        assert_eq!(kept.cell, Some(FormId(0x100)));
    }

    #[test]
    fn baseline_diff_lists_changed_shared_stages() {
        let mut gs = GameState::default();
        gs.stages.insert(FormId(1), 50);
        gs.stages.insert(FormId(2), 10);
        let d = baseline_diff(&[(1, 40), (2, 10), (3, 5)], &gs);
        assert_eq!(
            d,
            vec![
                Conflict {
                    quest: 1,
                    theirs: 40,
                    host: 50
                },
                Conflict {
                    quest: 3,
                    theirs: 5,
                    host: 0
                }
            ]
        );
        let b = baseline_of(&gs);
        assert_eq!(b, vec![(1, 50), (2, 10)]);
    }

    #[test]
    fn carry_keeps_parked_shadows_and_forces_a_full_diff() {
        let mut old = GameState::default();
        old.enable_shadows(ShadowPlan::all(), 0);
        old.register_player(1);
        old.set_authority(super::super::Authority::Host);
        let mut new = GameState::default();
        carry_coop(&mut old, &mut new, None);
        assert!(new.coop.overflow);
        assert!(new.coop.bank.parked.contains_key(&1));
        assert!(new.coop.bank.dirty.contains(&1));
        assert!(new.coop.bank.enabled());
        assert!(install_for_slot(&mut new, 1, shadow()));
        assert!(
            !install_for_slot(&mut new, 0, shadow()),
            "the live slot is not a guest's"
        );
        assert_eq!(new.coop.bank.parked[&1].level, 6);
    }
}
