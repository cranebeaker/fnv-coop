//! Locked doors and containers. A placed reference's `XLOC`: the lock
//! level (u8: 0 very easy, 25 easy, 50 average, 75 hard, 100 very hard,
//! above that only its key opens it), three unused bytes, the key (`KEYM`,
//! 0 for none), flags (u8, 0x04 leveled) and padding; a reference with one
//! is locked when its cell loads. Scripts lock and unlock them (`Lock`,
//! `Unlock`), and terminals do it for their linked reference (`XLKR`:
//! Goodsprings' schoolhouse terminal `GetLinkedRef` → the safe
//! `LootSafeGoodspringsSchoolhouse`, `XLOC` level 25).
//!
//! Getting in: with the key; else picking it. A lock's difficulty is the
//! first bracket its level doesn't pass (`00430b40`): `iLockLevelMax…`
//! very easy 0, easy 25, average 50, hard 75, very hard 100 (the exe's
//! defaults), above that key only; a lock broken by failed forcing needs
//! its key too (`005180b0` asks `00430ae0`: `sImpossibleLock`). Otherwise
//! the lockpicking game opens (`world::lockpick`), with Lockpick at least
//! the bracket's top (`0078db00`; else `sLockpickSkillTooLow`); picking it
//! gives `iXPRewardPickLock<bracket>` the first time (`0078eb50`).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::scripting::{Facts, GameState};

const XLOC: FourCC = FourCC::new(b"XLOC");
const XLKR: FourCC = FourCC::new(b"XLKR");

/// The Lockpick skill's actor value.
pub const LOCKPICK: u16 = 36;

/// A lock: its level and key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lock {
    pub level: u8,
    pub key: Option<FormId>,
}

impl Lock {
    /// Whether only the key opens it.
    pub fn key_only(&self) -> bool {
        self.level > 100
    }

    /// Its difficulty (0 very easy … 4 very hard; 5 key only) and the
    /// Lockpick skill that takes (`00430b40`, `0078db00`).
    pub fn difficulty(&self, order: &LoadOrder) -> (u8, u16) {
        for (i, word) in crate::experience::DIFFICULTIES.iter().enumerate() {
            let top = crate::scripting::game_setting(order, &format!("iLockLevelMax{word}"))
                .unwrap_or(25.0 * i as f32);
            if f32::from(self.level) <= top {
                return (i as u8, top.max(0.0) as u16);
            }
        }
        (5, u16::from(self.level))
    }
}

/// The lock a reference was placed with (`XLOC`), if any.
pub fn placed_lock(order: &LoadOrder, reference: FormId) -> Option<Lock> {
    let rr = order.get(reference)?;
    let record = rr.record_shared().ok()?;
    let s = record.get(XLOC).filter(|s| !s.data.is_empty())?;
    let key = (s.data.len() >= 8)
        .then(|| rr.plugin.to_global(FormId(le_u32(&s.data, 4))))
        .filter(|k| k.0 != 0);
    Some(Lock {
        level: s.data[0],
        key,
    })
}

/// The lock on a reference now: as placed, unless scripts or the player
/// changed it (`GameState::locks`: `None` unlocked, `Some(level)` locked).
pub fn lock_now(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<Lock> {
    let placed = placed_lock(order, reference);
    match state.locks.get(&reference) {
        Some(None) => None,
        Some(Some(level)) => Some(Lock {
            level: *level,
            key: placed.and_then(|l| l.key),
        }),
        None => placed,
    }
}

/// A reference's linked reference (`XLKR`: a keyword then the reference,
/// or just the reference).
pub fn linked_ref(order: &LoadOrder, reference: FormId) -> Option<FormId> {
    let rr = order.get(reference)?;
    let record = rr.record().ok()?;
    let s = record.get(XLKR).filter(|s| s.data.len() >= 4)?;
    let at = if s.data.len() >= 8 { 4 } else { 0 };
    Some(rr.plugin.to_global(FormId(le_u32(&s.data, at)))).filter(|f| f.0 != 0)
}

/// How the player gets past a lock, or why not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opening {
    Open,
    WithKey,
    /// The lockpicking game opens on it (`world::lockpick`), a lock of
    /// this difficulty (0..4).
    Pick(u8),
    NeedsKey,
    NeedsSkill(u16),
}

/// The player tries a locked reference (`005180b0`): open already; their
/// key (it's unlocked for good); a lock only its key opens, or broken by
/// failed forcing (`sImpossibleLock`); too little Lockpick
/// (`sLockpickSkillTooLow`, `0078db00`); else the lockpicking game, which
/// unlocks it if it's won (`world::lockpick::picked`).
pub fn try_open(order: &LoadOrder, state: &mut GameState, reference: FormId) -> Opening {
    let Some(lock) = lock_now(order, state, reference) else {
        return Opening::Open;
    };
    let player = crate::dialogue::PLAYER_REF;
    let opening = if lock
        .key
        .is_some_and(|k| state.item_count(order, player, k) > 0)
    {
        Opening::WithKey
    } else if lock.key_only()
        || lock.difficulty(order).0 > 4
        || crate::lockpick::is_broken(order, state, reference)
    {
        Opening::NeedsKey
    } else {
        let skill = Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(player, LOCKPICK)
        .unwrap_or(0.0)
        .floor();
        let (difficulty, needs) = lock.difficulty(order);
        if skill >= f64::from(needs) {
            Opening::Pick(difficulty)
        } else {
            Opening::NeedsSkill(needs)
        }
    };
    if opening == Opening::WithKey {
        state.locks.insert(reference, None);
        if let Some(key) = lock.key {
            unlocked_with(order, state, key);
        }
    }
    opening
}

/// Opened with the key (`005180b0` for doors, `00516dc0` for containers):
/// `UILockpickingUnlock`, and "Unlocked with <key>." (`sOpenWithKey`, the
/// key's name for its `%s`) with the key picture.
fn unlocked_with(order: &LoadOrder, state: &mut GameState, key: FormId) {
    use crate::scripting::Event;
    if let Some(sound) = order.form_by_editor_id("UILockpickingUnlock") {
        state.events.push(Event::Sound(sound));
    }
    let name = order
        .get(key)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default();
    let text = crate::scripting::game_setting_text(order, "sOpenWithKey")
        .unwrap_or_else(|| "Unlocked with %s.".to_string())
        .replacen("%s", &name, 1);
    state.events.push(Event::Message {
        title: None,
        text,
        buttons: Vec::new(),
        icon: Some(crate::message_icon::KEY.to_string()),
    });
}
