//! The testkit's companions (docs/coop/SPECS.md S32): the teammate flag, the companion wheel's
//! switches and the three `ProcessFollow`s, as stand-ins that do what co-op relies on the real
//! ones to do, and no more:
//!
//! - `Actor::SetPlayerTeammate` stores the flag at +0x18d (the player is never one);
//! - an actor's script variable by name reads and writes the wheel's four switches;
//! - each `ProcessFollow` notes whom the actor follows now: its process's target (+0x40);
//! - the process's `SetTarget` (virtual 0x12c) sets that target.

use engine_core::prelude::*;

use super::{Story, PLAYER};
use crate::{actors, addr, forms};

/// In a story actor's own block: the wheel's four switches (floats), then whom its last follow
/// step followed (a pointer).
const SWITCHES: u32 = 0xf00;
const FOLLOWED: u32 = 0xf20;
/// In a package's own block: its `PackageTarget` (type, reference).
const PACKAGE_TARGET: u32 = 0x200;
/// The process's `SetTarget` stand-in.
const SET_TARGET_FN: u32 = 0x0130_00a0;

/// Which of the wheel's switches a variable's name (the second word) is.
fn slot(a: &[u32]) -> Option<usize> {
    addr::WHEEL_SWITCH_TEXTS.iter().position(|t| *t == a[1])
}

/// Called by `Story::new` after the fight kit has made its actors, before co-op is installed.
pub(super) fn register(e: &mut Engine) {
    e.register(addr::ACTOR_SET_PLAYER_TEAMMATE, |e, a| {
        if a[0] != PLAYER {
            e.mem.set_u8(
                a[0] + addr::ACTOR_TEAMMATE_OFFSET,
                u8::from(a[1] & 0xff != 0),
            );
        }
        Ret::default()
    });
    e.register(addr::ACTOR_GET_VARIABLE, |e, a| {
        let v = slot(a).map_or(0.0, |i| e.mem.f32(a[0] + SWITCHES + 4 * i as u32));
        f64::from(v).into_ret()
    });
    e.register(addr::ACTOR_SET_VARIABLE, |e, a| {
        if let Some(i) = slot(a) {
            let v = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            e.mem.set_f32(a[0] + SWITCHES + 4 * i as u32, v as f32);
        }
        Ret::default()
    });
    for f in [
        addr::HIGH_PROCESS_FOLLOW,
        addr::MIDDLE_HIGH_PROCESS_FOLLOW,
        addr::LOW_PROCESS_FOLLOW,
    ] {
        e.register(f, |e, a| {
            let target = e.mem.u32(a[0] + addr::PROCESS_TARGET_OFFSET);
            e.mem.set_u32(a[1] + FOLLOWED, target);
            Ret::default()
        });
    }
    // The process's vtable is the fight kit's, shared by every actor's process.
    let process = e.mem.u32(PLAYER + addr::ACTOR_PROCESS_OFFSET);
    let vtable = e.mem.u32(process);
    e.mem
        .set_u32(vtable + addr::PROCESS_SET_TARGET_SLOT, SET_TARGET_FN);
    e.register(SET_TARGET_FN, |e, a| {
        e.mem.set_u32(a[0] + addr::PROCESS_TARGET_OFFSET, a[1]);
        Ret::default()
    });
}

impl Story {
    fn companion_ptr(&mut self, id: u32) -> u32 {
        actors::actor(&mut self.e, id).expect("an actor of the story")
    }

    /// A script's `SetPlayerTeammate` on actor `id`.
    pub fn script_set_teammate(&mut self, id: u32, on: bool) {
        let a = self.companion_ptr(id);
        self.e
            .call(addr::ACTOR_SET_PLAYER_TEAMMATE, &args![a, u32::from(on)]);
    }

    /// Places an avatar (the base form 7) in `cell` at `pos`, with its 3D: its reference id.
    pub fn place_avatar_at(&mut self, cell: u32, pos: [f32; 3]) -> u32 {
        let base = forms::form(&mut self.e, 7).expect("the avatar's base form");
        let player_cell = self.e.mem.u32(PLAYER + addr::REF_CELL_OFFSET);
        let c = forms::form(&mut self.e, cell).expect("a cell of the story");
        self.e.mem.set_u32(PLAYER + addr::REF_CELL_OFFSET, c);
        let r = self
            .e
            .call(addr::PLACE_AT_ME, &args![PLAYER, base, 1u32, 0u32, 0u32])
            .u32();
        self.e
            .mem
            .set_u32(PLAYER + addr::REF_CELL_OFFSET, player_cell);
        assert_ne!(r, 0, "placed");
        for (i, v) in pos.into_iter().enumerate() {
            self.e
                .mem
                .set_f32(r + addr::REF_POS_OFFSET + 4 * i as u32, v);
        }
        self.e.mem.u32(r + addr::FORM_ID_OFFSET)
    }

    /// Sets actor `id`'s process's target (a pointer), as the game's `ResetTarget` does.
    pub fn set_follow_target(&mut self, id: u32, target: u32) {
        let a = self.companion_ptr(id);
        let p = self.e.mem.u32(a + addr::ACTOR_PROCESS_OFFSET);
        self.e.mem.set_u32(p + addr::PROCESS_TARGET_OFFSET, target);
    }

    /// Runs one step of a follow package of actor `id` through the `ProcessFollow` at `f`.
    pub fn process_follow(&mut self, f: u32, id: u32) {
        let a = self.companion_ptr(id);
        let p = self.e.mem.u32(a + addr::ACTOR_PROCESS_OFFSET);
        self.e.call(f, &args![p, a, 1u32, 0u32, 0u32]);
    }

    /// Whom actor `id`'s last follow step followed (a pointer; 0 for none).
    pub fn followed(&mut self, id: u32) -> u32 {
        let a = self.companion_ptr(id);
        self.e.mem.u32(a + FOLLOWED)
    }

    /// Aims package `package` at the reference at `target` (a pointer).
    pub fn aim_package_at(&mut self, package: u32, target: u32) {
        let p = forms::form(&mut self.e, package).expect("a package of the story");
        let t = p + PACKAGE_TARGET;
        self.e.mem.set_u32(t, 0);
        self.e.mem.set_u32(t + 4, target);
        self.e.mem.set_u32(p + addr::PACKAGE_TARGET_OFFSET, t);
    }
}
