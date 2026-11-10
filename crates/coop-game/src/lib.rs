//! `coop-game`: what a co-op session needs from one running game.
//!
//! A session talks to a game only through the traits of this crate. Each game adapter
//! implements them. The crate names no engine type: it depends on `coop-core` and
//! `coop-policy` only.
//!
//! * [`Names`]: editor IDs, form IDs and plugin indexes.
//! * [`Shared`]: the role of the game, and its shared state as `coop_core` fields.
//! * [`Story`]: story actions, which only a host's game runs.
//! * [`fake::FakeGame`]: a game made of maps, for the tests of other crates.
//! * [`conformance`]: the checks that each adapter must pass.
//!
//! The traits for players, for the live layer and for the state dump come in later steps.

#![warn(missing_docs)]

use std::sync::Arc;

pub use coop_core::ids::FormId;
pub use coop_core::schema::{Field, Key};
use coop_policy::PolicyTable;

pub mod conformance;
pub mod fake;

/// A player's slot in the party. Slot 0 is the host's player.
pub type Slot = u8;

/// One shared slot to write. `None`: the slot returns to its default.
pub type Change = (Key, Option<Field>);

/// The part that a game has in a session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// The authority for the story. This game runs the story code.
    Host,
    /// A follower. This game runs no quest script. It applies the host's state.
    Guest,
}

/// How a game takes part in a session ([`Shared::configure`]).
#[derive(Clone, Debug)]
pub struct Setup {
    /// The part of this game.
    pub role: Role,
    /// The slot of this game's own player.
    pub me: Slot,
    /// The campaign policy to install. Its quest IDs are those of this game's load order.
    /// `None`: the game keeps the policy that it has. A new game has no policy.
    pub policy: Option<Arc<PolicyTable>>,
    /// True while this game makes the rows of its own player (a guest that makes its
    /// character). The host's copy of these rows does not replace them.
    pub keep_own_rows: bool,
}

/// What one [`Shared::apply_shared`] call did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Applied {
    /// The number of slots that got a value.
    pub set: u32,
    /// The number of slots that returned to their default.
    pub cleared: u32,
    /// The number of quest announcements that the game showed.
    pub announced: u32,
}

/// A story action. Only a host's game runs it, with the game's own code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    /// Sets a stage of a quest as the game's `SetStage` does. The stage's script runs.
    SetStage {
        /// The quest.
        quest: FormId,
        /// The stage to set.
        stage: u16,
    },
    /// Runs one line of the game's script language, as the game's console does.
    ScriptLine(String),
}

/// Why a game did not try to run an [`Intent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// This game is not a host's game.
    NotHost,
    /// The game has no form that the intent names.
    NoSuchForm,
    /// The campaign policy holds the action for now.
    Held,
}

/// The names of the game's data.
pub trait Names {
    /// The form that has this editor ID. The match ignores the case of letters.
    /// `None`: no form has this editor ID.
    fn form_of(&self, editor_id: &str) -> Option<FormId>;

    /// The editor ID of a form. [`Names::form_of`] of the answer gives `form` again.
    /// `None`: the game has no such form, or the form has no editor ID.
    fn name_of(&self, form: FormId) -> Option<String>;

    /// The load index of a plugin file. It is the top byte of the IDs of the forms that the
    /// plugin adds. The match ignores the case of letters. `None`: the plugin is not loaded.
    fn load_index(&self, plugin: &str) -> Option<u8>;
}

/// The role of the game, and its shared state.
pub trait Shared {
    /// Sets the role. After this call a guest's game runs no quest script.
    ///
    /// Call it at the start of a session, and again when a value of `setup` changes.
    /// A `setup` with a policy installs that policy and starts the per-player rows again.
    /// `Err` tells why the game cannot take the role.
    fn configure(&mut self, setup: &Setup) -> Result<(), String>;

    /// Every shared field as it is now. The caller finds changes with a diff.
    ///
    /// The list has one entry for each slot. A slot that has its default value
    /// ([`Field::is_tombstone`]) is not in the list. Two calls with no change between them
    /// give equal lists.
    ///
    /// `None` only when `force` is false and the game is sure that nothing changed since the
    /// last call that gave a list. Force the first export of a session: before it, the game
    /// cannot know what the caller has.
    fn export_shared(&mut self, force: bool) -> Option<Vec<(FormId, Field)>>;

    /// Writes slots as plain state: no script, no reward, no random list.
    ///
    /// A change with `None` returns its slot to the default. Then the slot is not in the
    /// export. `present`: show the journal lines and the announcements of these changes to
    /// the local player.
    fn apply_shared(&mut self, changes: &[Change], present: bool) -> Applied;

    /// The current stage and the flag bits ([`coop_core::schema::flag`]) of a quest, from the
    /// rows of the live player. A quest that has flags and no stage gives stage 0.
    /// `None`: these rows hold no stage and no flag for the quest.
    fn quest(&self, quest: FormId) -> Option<(u16, u32)>;

    /// A number that moves when shared state possibly changed. An equal number does not
    /// prove that the state is equal.
    fn activity(&self) -> u64;
}

/// The story code of the game.
pub trait Story {
    /// Runs `intent` with the rows of player `slot` live. The game uses its own story code:
    /// scripts, rewards and random lists run as in a game for one player.
    ///
    /// `Ok(true)`: the game ran the action. `Ok(false)`: the game's rules did nothing, for
    /// example because the stage is done already. `Err`: the game did not try.
    ///
    /// A game that is not a host's game answers [`Refused::NotHost`] to each intent, and
    /// changes no state.
    fn run_intent(&mut self, slot: Slot, intent: &Intent) -> Result<bool, Refused>;
}
