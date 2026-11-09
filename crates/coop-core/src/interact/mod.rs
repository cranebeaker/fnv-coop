//! Player-world interactions: barter, doors and cell transitions, locks and
//! terminals, world items, crafting, repair, item use, companions and more.
//!
//! One host-authoritative shape for all of them:
//!
//! * a guest's action is an [`Act`] sent as [`InteractMsg::Request`] with a
//!   per-guest transaction id; the host applies it **once** (a replayed
//!   request gets the cached verdict, never a second effect);
//! * the host answers the requester with a [`InteractMsg::Verdict`];
//! * changes to shared world objects that no replicated field carries (door
//!   swing, lock state, terminal state, mines, dropped items) go to everyone
//!   as [`Shared`] records and are kept in a log so a late joiner gets them;
//! * cell transitions are decided by [`Transit`] (follow the leader, same
//!   cell group for the others, a barrier for one-way trips) and answered with
//!   [`InteractMsg::Follow`] to the players who must load the destination.
//!
//! The crate is game-free: the world side (`world::coop::interact`) executes
//! the acts on a `GameState`. Everything per player (inventory, equipment,
//! health, rads) travels through the existing per-player shadow rows; shared
//! stock (vendors) through the `items` domain.

mod client;
mod host;
mod transit;

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, FormId, PlayerId};

pub use client::{InteractClient, Settled};
pub use host::{IOut, InteractHost, InteractStats, Outcome};
pub use transit::{Decision, Transit};

/// Most (item, count) entries in a barter message.
pub const MAX_LINES: usize = 128;
/// Most shared records in one sync chunk.
pub const SYNC_CHUNK: usize = 48;
/// First form id of items a player dropped into the world.
pub const DROP_BASE: u32 = 0xFD00_0000;

/// Why a request was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Why {
    /// Not refused.
    None,
    /// The host does not know what this is.
    Unknown,
    /// Already taken, destroyed or used up.
    Gone,
    /// Not enough of something (items, caps, ingredients, skill).
    Insufficient,
    /// Locked or otherwise closed to this player.
    Locked,
    /// The rules say no (party policy, cell group).
    Policy,
    /// Malformed.
    Invalid,
    /// Someone else is using it.
    Busy,
}

/// State of a lock, door-independent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockState {
    /// `None` unlocked, `Some(level)` locked.
    pub level: Option<u8>,
    /// Broken by failed forcing (needs its key).
    pub broken: bool,
}

/// What became of a placed mine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinePhase {
    /// Turned off, can be picked up.
    Disarmed,
    /// Exploded or taken.
    Gone,
}

/// A shared world fact that no replicated field carries. Newest wins.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shared {
    /// A swing door is open or shut.
    Door {
        /// Door reference.
        door: FormId,
        /// Open now.
        open: bool,
    },
    /// A lock changed.
    Lock {
        /// Locked reference.
        obj: FormId,
        /// New state.
        state: LockState,
    },
    /// A terminal was hacked or locked the player out.
    Terminal {
        /// Terminal reference.
        obj: FormId,
        /// Hacked.
        hacked: bool,
        /// Lock-outs so far.
        lockouts: u8,
    },
    /// A placed mine.
    Mine {
        /// Mine reference.
        obj: FormId,
        /// What happened.
        phase: MinePhase,
    },
    /// A player dropped an item into the world.
    Placed {
        /// The new reference (from [`DROP_BASE`]).
        drop: FormId,
        /// Item form.
        item: FormId,
        /// Count.
        count: u32,
        /// Cell it lies in.
        cell: CellId,
        /// The interior cell or worldspace it lies in.
        space: CellId,
        /// Position (game units, rounded).
        pos: [i32; 3],
    },
}

impl Shared {
    /// The key the log keeps one record under (kind, subject).
    pub fn key(&self) -> (u8, FormId) {
        match self {
            Shared::Door { door, .. } => (0, *door),
            Shared::Lock { obj, .. } => (1, *obj),
            Shared::Terminal { obj, .. } => (2, *obj),
            Shared::Mine { obj, .. } => (3, *obj),
            Shared::Placed { drop, .. } => (4, *drop),
        }
    }
}

/// What a companion is told.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanionCmd {
    /// Follow the player.
    Follow,
    /// Stay here.
    Wait,
    /// Send away.
    Dismiss,
    /// Take along.
    Recruit,
    /// Heal with a stimpak from the player's pack.
    Stimpak,
    /// Wear the best gear.
    SortGear,
    /// One of the wheel's four switches (0 aggressive, 1 waiting, 2 keep
    /// distance, 3 ranged), on or off.
    Switch(u8, bool),
}

/// Pip-Boy actions beyond using or equipping items.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipOp {
    /// Bind an item to a hot key (item 0 clears it).
    Hotkey {
        /// Slot 0..8.
        slot: u8,
        /// Item.
        item: FormId,
    },
    /// Read a note or holotape (shared note list).
    Note(FormId),
    /// Choose the tracked quest (a per-player view choice, acknowledged).
    ActiveQuest(FormId),
}

/// How an item leaves an actor's pack without a container menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StealMode {
    /// Pickpocket a living person (the sneak check was played locally).
    Pickpocket,
    /// Take from a body or an unattended pack.
    Loot,
    /// Plant an item on the target.
    Plant,
}

/// What happened to a trapped or destructible object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrapOutcome {
    /// Turned off.
    Disarmed,
    /// Went off.
    Detonated,
    /// Broken beyond use.
    Destroyed,
}

/// Everything a player can ask the host to do in the world.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Act {
    /// The guest ran a rule locally that produced shared world state; the
    /// host adopts it.
    World(Shared),
    /// Activate an object: its `OnActivate` script runs for the player
    /// (levers, switches, lights, buttons, generic activators).
    Activate {
        /// The reference.
        obj: FormId,
    },
    /// Swing a door open or shut (not a load door).
    Door {
        /// The door.
        door: FormId,
    },
    /// Take an item lying in the world.
    Pickup {
        /// The placed (or dropped) reference.
        obj: FormId,
        /// Item form.
        item: FormId,
        /// Count.
        count: u32,
    },
    /// Put an item on the ground.
    Drop {
        /// Item form.
        item: FormId,
        /// Count.
        count: u32,
        /// Cell.
        cell: CellId,
        /// Position (game units).
        pos: [i32; 3],
    },
    /// Accept a trade with a vendor.
    Barter {
        /// The vendor.
        vendor: FormId,
        /// Items the player buys.
        buys: Vec<(FormId, u32)>,
        /// Items the player sells.
        sells: Vec<(FormId, u32)>,
        /// Caps that change hands: positive the vendor pays, negative the
        /// player pays.
        total: i32,
    },
    /// Take from or plant on an actor's pack.
    Steal {
        /// The actor.
        target: FormId,
        /// Item.
        item: FormId,
        /// Count.
        count: u32,
        /// How.
        mode: StealMode,
    },
    /// Make something at a workbench.
    Craft {
        /// The bench reference.
        bench: FormId,
        /// The recipe.
        recipe: FormId,
        /// How many times.
        times: u32,
    },
    /// Repair an item: with a part, or paid to a vendor.
    Repair {
        /// The item to mend.
        item: FormId,
        /// The part spent (0 for a vendor's service).
        part: FormId,
        /// The vendor (0 when mending with a part).
        vendor: FormId,
        /// Caps the vendor charges.
        cost: u32,
    },
    /// Use an aid item (food, drink, chem).
    Use {
        /// Item.
        item: FormId,
    },
    /// Equip or unequip an item.
    Equip {
        /// Item.
        item: FormId,
        /// True to equip.
        on: bool,
    },
    /// Choose the ammunition.
    Ammo {
        /// The ammo item (0 clears).
        ammo: FormId,
    },
    /// A Pip-Boy action.
    Pip(PipOp),
    /// A command to a companion.
    Companion {
        /// The companion.
        who: FormId,
        /// The command.
        cmd: CompanionCmd,
    },
    /// The lockpicking minigame ended (played locally).
    Lockpick {
        /// The lock.
        obj: FormId,
        /// Opened.
        won: bool,
        /// Bobby pins spent.
        pins: u8,
        /// The forcing failed and broke the lock.
        broke: bool,
    },
    /// The hacking minigame ended (played locally).
    Hack {
        /// The terminal.
        obj: FormId,
        /// Hacked.
        won: bool,
    },
    /// Use a key on a lock.
    Key {
        /// The lock.
        obj: FormId,
        /// The key item.
        key: FormId,
    },
    /// A trap or mine went off, or was disarmed, or a thing was destroyed.
    Trap {
        /// The reference.
        obj: FormId,
        /// What happened.
        what: TrapOutcome,
    },
    /// A map marker was found (fast travel unlock).
    Discover {
        /// The marker reference.
        marker: FormId,
    },
    /// Use a door or other trigger that moves the player to another cell.
    Transition {
        /// The door (or marker) used.
        door: FormId,
        /// The cell the player leaves.
        from: CellId,
        /// The cell the player enters.
        to: CellId,
        /// No way back (a barrier is needed).
        one_way: bool,
    },
    /// The level-up menu closed (played locally): the level is taken, the
    /// skill points go into the skills, the perk is given.
    LevelUp {
        /// Skill actor values and the points put into each.
        skills: Vec<(u16, u32)>,
        /// The perk picked (0 when the level gives none).
        perk: FormId,
    },
    /// A caravan game ended (played locally): the loser pays the stake and
    /// the player's record counts the game.
    Caravan {
        /// The opponent.
        npc: FormId,
        /// The stake (the smaller ante).
        stake: u32,
        /// The player won.
        won: bool,
    },
    /// A casino game closed (played locally): the player's ledger at that
    /// casino, and the chips the menu ended with.
    Casino {
        /// The casino record (`CSNO`).
        casino: FormId,
        /// The menu's chips at closing.
        chips: u32,
        /// The net chips won there, all games (`iTotalWinnings`).
        winnings: i32,
        /// The winnings level reached (0 to 4).
        level: u8,
        /// A round reached a new level (the comps quest starts).
        new_level: bool,
    },
}

impl Act {
    /// A short stable name (counters, logs, bot verbs).
    pub fn name(&self) -> &'static str {
        match self {
            Act::World(_) => "world",
            Act::Activate { .. } => "activate",
            Act::Door { .. } => "door",
            Act::Pickup { .. } => "pickup",
            Act::Drop { .. } => "drop",
            Act::Barter { .. } => "barter",
            Act::Steal { .. } => "steal",
            Act::Craft { .. } => "craft",
            Act::Repair { .. } => "repair",
            Act::Use { .. } => "use",
            Act::Equip { .. } => "equip",
            Act::Ammo { .. } => "ammo",
            Act::Pip(_) => "pip",
            Act::Companion { .. } => "companion",
            Act::Lockpick { .. } => "lockpick",
            Act::Hack { .. } => "hack",
            Act::Key { .. } => "key",
            Act::Trap { .. } => "trap",
            Act::Discover { .. } => "discover",
            Act::Transition { .. } => "transition",
            Act::LevelUp { .. } => "levelup",
            Act::Caravan { .. } => "caravan",
            Act::Casino { .. } => "casino",
        }
    }

    /// The world object whose shared state this act may have changed.
    pub fn object(&self) -> Option<FormId> {
        match self {
            Act::Activate { obj }
            | Act::Lockpick { obj, .. }
            | Act::Hack { obj, .. }
            | Act::Key { obj, .. }
            | Act::Trap { obj, .. } => Some(*obj),
            Act::Door { door } => Some(*door),
            Act::World(s) => Some(s.key().1),
            _ => None,
        }
    }
}

/// Where the players who follow a transition must go.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Follow {
    /// The door that was used.
    pub door: FormId,
    /// Destination cell.
    pub to: CellId,
    /// Who led.
    pub leader: PlayerId,
}

/// Interaction wire messages (all reliable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractMsg {
    /// Guest asks; applied exactly once per `(guest, txn)`.
    Request {
        /// Per-guest increasing id (from 1).
        txn: u32,
        /// The action.
        act: Act,
    },
    /// Host verdict for the requester.
    Verdict {
        /// Request id.
        txn: u32,
        /// Done.
        ok: bool,
        /// Why not.
        why: Why,
    },
    /// Host: a shared world fact changed.
    Shared(Shared),
    /// Host: shared facts a joiner missed.
    Sync(Vec<Shared>),
    /// Host: load this destination now.
    Follow(Follow),
}

impl InteractMsg {
    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            InteractMsg::Request {
                act: Act::Barter { buys, sells, .. },
                ..
            } => buys.len() <= MAX_LINES && sells.len() <= MAX_LINES,
            InteractMsg::Sync(v) => v.len() <= SYNC_CHUNK,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests;
