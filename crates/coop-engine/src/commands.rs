//! What a story script tells an actor to do, as distinct from state (docs/coop/SPECS.md S13,
//! docs/coop/DECISIONS.md D5: host commands). The first: `evp`, an actor told to evaluate its
//! package again. A stage's result script says `SunnyREF.evp` so she acts on the new stage at
//! once; that script runs on the host only, and the guest's copy of her would stand still.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, fight, forms, gate, journal};

pub(crate) const EVALUATE: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_EVALUATE_PACKAGE);

/// The commands noted since the last drain (game-wide, as the journal).
#[derive(Debug, Default)]
pub struct Commands {
    /// Reference ids of the actors a script told to evaluate their package.
    evaluate: Mutex<Vec<u32>>,
    /// Conversations between two actors a script started, in order.
    scenes: Mutex<Vec<Scene>>,
    /// Lines a script had an actor say to the player (`SayTo`), in order: speaker, topic.
    says: Mutex<Vec<(u32, u32)>>,
    /// Host: scenes played for a guest's ask (actor, target, topic), and when.
    asked: Mutex<Vec<AskedScene>>,
    /// Host: `SayTo` commands seen, for the log.
    says_seen: AtomicU32,
    /// A guest's own conversations between two actors, dropped.
    dropped: AtomicU32,
    /// Guest: the package the host's copy of each actor runs (reference id to package id).
    wanted: Mutex<BTreeMap<u32, u32>>,
    /// Guest: whether those packages are given to this game's copies.
    packages_on: AtomicBool,
    /// Guest: how many evaluations were answered with the host's package.
    given: AtomicU32,
    /// Guest: the actors the host's game runs near this player now. Only they are given the
    /// host's package: one the host has left behind is this game's own again.
    followed: Mutex<std::collections::BTreeSet<u32>>,
    /// Whether conversations between actors are the host's (`NV_COOP_SCENES`; on unless a
    /// session turns it off, when every game runs its own as before).
    scenes_off: AtomicBool,
    /// Lines for the log: each scene a guest dropped, each first giving of a package.
    notes: Mutex<Vec<String>>,
    /// Guest: the (actor, package) pairs already written to the log.
    given_seen: Mutex<std::collections::BTreeSet<(u32, u32)>>,
}

fn note(e: &mut Engine, line: String) {
    let c = e.ext_free::<Commands>();
    let mut notes = c.notes.lock().unwrap_or_else(PoisonError::into_inner);
    if notes.len() < 200 {
        notes.push(line);
    }
}

/// Takes the lines for the log.
pub fn drain_notes(e: &mut Engine) -> Vec<String> {
    let c = e.ext_free::<Commands>();
    std::mem::take(&mut *c.notes.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Turns the sharing of scenes off or on (every game then runs its own, as before co-op).
pub fn set_scenes_on(e: &mut Engine, on: bool) {
    e.ext_free::<Commands>()
        .scenes_off
        .store(!on, Ordering::SeqCst);
}

/// Guest: names the actors the host's game runs near this player now.
pub fn set_followed(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    let c = e.ext_free::<Commands>();
    *c.followed.lock().unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

pub(crate) const FIND_PACKAGE: WrapSlot =
    WrapSlot::new(Owner::Coop, addr::PACKAGE_LIST_FIND_CURRENT);

/// The form id of the package actor `id` runs in this game now (0: none, or one made while
/// playing, which the other game does not have); `None` when the actor is not here.
pub fn current_package(e: &mut Engine, id: u32) -> Option<u32> {
    let a = actors::actor(e, id)?;
    if !e.is_registered(addr::ACTOR_GET_CURRENT_EDITOR_PACKAGE) {
        return Some(0);
    }
    let package = e
        .call(addr::ACTOR_GET_CURRENT_EDITOR_PACKAGE, &args![a])
        .u32();
    if package == 0 {
        return Some(0);
    }
    let pid = e.mem.u32(package.wrapping_add(addr::FORM_ID_OFFSET));
    if pid >= actors::FIRST_DYNAMIC_ID {
        return Some(0);
    }
    Some(pid)
}

/// Whether package `id` keeps company with the player (follow, escort, accompany: its target is
/// the player's reference): the actor goes where that player goes.
pub fn keeps_company(e: &mut Engine, id: u32) -> bool {
    let Some(package) = crate::forms::form(e, id)
        .filter(|p| e.mem.u8(p.wrapping_add(addr::FORM_TYPE_OFFSET)) == addr::FORM_TYPE_PACKAGE)
    else {
        return false;
    };
    let target = e.mem.u32(package.wrapping_add(addr::PACKAGE_TARGET_OFFSET));
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    let aimed = target != 0
        && e.mem.u32(target) & 0xff == 0
        && player != 0
        && e.mem.u32(target + 4) == player;
    aimed && !about_the_player(e, package)
}

/// Whether package `id` is aimed at the player other than to keep company ("wait here until the
/// player comes near, then talk"). The host says such a package too: on each game the actor then
/// waits for, and talks to, that game's own player, so whichever player comes to it is served
/// (PC run 18: Sunny spoke only to the host's player and her copy on the guest, left to its own
/// choice, walked off and was dragged back). A copy running one is not kept at the host's exact
/// place, since the two may each turn to their own player.
pub fn aimed_at_player(e: &mut Engine, id: u32) -> bool {
    match crate::forms::form(e, id) {
        Some(package)
            if e.mem.u8(package.wrapping_add(addr::FORM_TYPE_OFFSET))
                == addr::FORM_TYPE_PACKAGE =>
        {
            about_the_player(e, package)
        }
        _ => false,
    }
}

/// Whether a package is aimed at the player in a way that must stay each game's own: its target
/// is the player's reference and it is not one that keeps company (follow, escort, accompany).
/// "Walk up to the player and talk" given to a guest's copy would send it to the guest while the
/// host's goes to the host; keeping company works for both players when they are together.
fn about_the_player(e: &mut Engine, package: u32) -> bool {
    let target = e.mem.u32(package.wrapping_add(addr::PACKAGE_TARGET_OFFSET));
    if target == 0 {
        return false;
    }
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    let aimed = e.mem.u32(target) & 0xff == 0 && player != 0 && e.mem.u32(target + 4) == player;
    if !aimed {
        return false;
    }
    let kind = if e.is_registered(addr::PACKAGE_TYPE) {
        e.call(addr::PACKAGE_TYPE, &args![package]).u32()
    } else {
        0
    };
    !matches!(kind, 1 | 2 | 7)
}

/// Guest: the host's copy of actor `actor` runs package `package` (0: none this game has; this
/// game's own choice stands).
pub fn set_wanted_package(e: &mut Engine, actor: u32, package: u32) {
    let c = e.ext_free::<Commands>();
    let mut wanted = c.wanted.lock().unwrap_or_else(PoisonError::into_inner);
    if package == 0 {
        wanted.remove(&actor);
    } else {
        wanted.insert(actor, package);
    }
}

/// Guest: the package the host's copy of `actor` runs, if known.
pub fn wanted_package(e: &mut Engine, actor: u32) -> Option<u32> {
    let c = e.ext_free::<Commands>();
    let wanted = c.wanted.lock().unwrap_or_else(PoisonError::into_inner);
    wanted.get(&actor).copied()
}

/// Guest: the actors the host has said a package of.
pub fn wanted_ids(e: &mut Engine) -> std::collections::BTreeSet<u32> {
    let c = e.ext_free::<Commands>();
    let wanted = c.wanted.lock().unwrap_or_else(PoisonError::into_inner);
    wanted.keys().copied().collect()
}

/// Guest: whether the host's packages are given to this game's copies (off while this player's
/// own opening runs, whose actors act for this player).
pub fn set_packages_on(e: &mut Engine, on: bool) {
    e.ext_free::<Commands>()
        .packages_on
        .store(on, Ordering::SeqCst);
}

/// Guest: how many evaluations were answered with the host's package.
pub fn packages_given(e: &mut Engine) -> u32 {
    e.ext_free::<Commands>().given.load(Ordering::SeqCst)
}

/// The wrap at `0067e780`. On a guest, for an actor whose host's package is known and is a
/// package of this game, the answer is that package (its procedure type computed first, as the
/// game does for the one it picks); this game's own schedule and conditions are not asked. They
/// are each game's own: a condition on "the player" means this game's player, and the two
/// players' copies of an actor would do different things (PC run 12: Sunny led the host outdoors
/// and stayed with the guest indoors). Everything else is the game's own answer.
pub(crate) fn find_package(e: &mut Engine, a: &[u32]) -> Ret {
    let reference = a.get(1).copied().unwrap_or(0);
    if reference == 0 || !gate::is_guest(e) {
        return e.call_next(addr::PACKAGE_LIST_FIND_CURRENT, a);
    }
    let id = e.mem.u32(reference.wrapping_add(addr::FORM_ID_OFFSET));
    let wanted = {
        let c = e.ext_free::<Commands>();
        let followed = c
            .followed
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(&id);
        if c.packages_on.load(Ordering::SeqCst) && followed {
            wanted_package(e, id)
        } else {
            None
        }
    };
    let package = wanted
        .and_then(|w| forms::form(e, w))
        .filter(|p| e.mem.u8(p.wrapping_add(addr::FORM_TYPE_OFFSET)) == addr::FORM_TYPE_PACKAGE);
    let Some(package) = package else {
        return e.call_next(addr::PACKAGE_LIST_FIND_CURRENT, a);
    };
    picked(e, package, reference);
    let first = {
        let c = e.ext_free::<Commands>();
        c.given.fetch_add(1, Ordering::SeqCst);
        c.given_seen
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert((id, wanted.unwrap_or(0)))
    };
    if first {
        note(
            e,
            format!(
                "co-op packages: actor {id:08x} runs the host's package {:08x} here",
                wanted.unwrap_or(0)
            ),
        );
    }
    package.into_ret()
}

/// What `FindCurrentPackage` does for the package it picks, before it returns it (its `picked`,
/// without the debug line): the procedure type computed when it is not yet; when the package
/// replaces another, the old one's end (a follow package's follower removed from whom it
/// followed, action flag 0x800, the process's `PackageDone` and `SetPackageDoneOnce(0)`) and the
/// new one's beginning (action flag 0x200).
fn picked(e: &mut Engine, package: u32, reference: u32) {
    if e.call(addr::PACKAGE_PROCEDURE_TYPE, &args![package]).u32() as i32 == -1 {
        let value = e.call(addr::REF_PROCEDURE_VALUE, &args![reference]).u32();
        e.call(addr::PACKAGE_CALCULATE_PROCEDURE, &args![package, value]);
    }
    if !e.is_registered(addr::REF_GET_CURRENT_PACKAGE) {
        return;
    }
    let current = e
        .call(addr::REF_GET_CURRENT_PACKAGE, &args![reference])
        .u32();
    if current == package {
        return;
    }
    let process = e
        .mem
        .u32(reference.wrapping_add(addr::ACTOR_PROCESS_OFFSET));
    if current != 0 {
        if process != 0 && e.call(addr::PACKAGE_TYPE, &args![current]).u32() == 1 {
            let followed = e
                .vcall(process, addr::PROCESS_FOLLOWED_SLOT, &args![])
                .u32();
            // The game casts the followed object to an actor; a mobile reference is one.
            if followed != 0
                && e.vcall(followed, addr::REF_IS_MOBILE_SLOT, &args![]).u32() & 0xff != 0
            {
                let list = e.call(addr::REF_EXTRA_LIST, &args![followed]).u32();
                e.call(addr::EXTRA_REMOVE_FOLLOWER, &args![list, reference]);
            }
        }
        let list = e.call(addr::REF_EXTRA_LIST, &args![reference]).u32();
        e.call(
            addr::PACKAGE_SET_ACTION_FLAG,
            &args![current, list, 0x800u32],
        );
        if process != 0 {
            e.vcall(
                process,
                addr::PROCESS_PACKAGE_DONE_SLOT,
                &args![reference, current],
            );
            e.vcall(process, addr::PROCESS_SET_DONE_ONCE_SLOT, &args![0u32]);
        }
    }
    let list = e.call(addr::REF_EXTRA_LIST, &args![reference]).u32();
    e.call(
        addr::PACKAGE_SET_ACTION_FLAG,
        &args![package, list, 0x200u32],
    );
}

pub(crate) const CONVERSE: WrapSlot = WrapSlot::new(Owner::Coop, addr::CMD_START_CONVERSATION);
/// The wrap of the `SayTo` command.
pub(crate) const SAY_TO: WrapSlot = WrapSlot::new(Owner::Coop, addr::CMD_SAY_TO);

/// A conversation a script started between two actors (`StartConversation`): who, with whom,
/// about what, and where both stood.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scene {
    /// The reference id of the actor the command ran on.
    pub actor: u32,
    /// The reference id of the one it talks to.
    pub target: u32,
    /// The topic's form id, 0 for none.
    pub topic: u32,
    /// The two location references' ids, 0 for none.
    pub locations: [u32; 2],
    /// The command's two flags as the game passes them on.
    pub flags: [bool; 2],
    /// The space both were read in (the actor's) and their places.
    pub space: u32,
    /// See [`Scene::space`].
    pub actor_pos: [f32; 3],
    /// See [`Scene::space`].
    pub target_pos: [f32; 3],
}

/// The command's six parameters as its body parses them (target, topic, two locations, two
/// flags; defaults 0, 0, 0, 0, 1, 0); `None` when the parse fails. The offset into the script
/// data is moved on as the body's own parse moves it; with `keep` it is put back afterwards, so
/// the body can parse again.
fn parse(e: &mut Engine, a: &[u32], keep: bool) -> Option<[u32; 6]> {
    let word = |i: usize| a.get(i).copied().unwrap_or(0);
    let offset_at = word(7);
    let before = if offset_at != 0 {
        e.mem.u32(offset_at)
    } else {
        0
    };
    let parsed = e.with_stack(24, |e, block| {
        let outs: Vec<u32> = (0..6).map(|i| block.addr() + 4 * i).collect();
        for (out, value) in outs.iter().zip([0u32, 0, 0, 0, 1, 0]) {
            e.mem.set_u32(*out, value);
        }
        let mut words = args![
            word(0),
            word(1),
            offset_at,
            word(2),
            word(3),
            word(4),
            word(5)
        ];
        words.extend_from_slice(&outs);
        let ok = e.call(addr::SCRIPT_PARSE_PARAMETERS, &words).u32() & 0xff != 0;
        let mut values = [0u32; 6];
        for (v, out) in values.iter_mut().zip(&outs) {
            *v = e.mem.u32(*out);
        }
        ok.then_some(values)
    });
    if keep && offset_at != 0 {
        e.mem.set_u32(offset_at, before);
    }
    parsed
}

/// The wrap at `005c9100` (`SayTo`). On the host, a line a script has an actor say to the
/// player is noted (speaker, topic) before the body runs: the host's game may not say it at all
/// (an actor far from its player has no process), and the guest whose game runs that actor says
/// it there to its own player (PC run 22: Sunny's "go get them" after the sneak, said by a stage's
/// result to the host's player far away, never came, and the quest waited for the host).
pub(crate) fn say_to(e: &mut Engine, a: &[u32]) -> Ret {
    let this_obj = a.get(2).copied().unwrap_or(0);
    if this_obj == 0 || gate::is_guest(e) || !journal::armed(e) {
        return e.call_next(addr::CMD_SAY_TO, a);
    }
    let speaker = e.mem.u32(this_obj.wrapping_add(addr::FORM_ID_OFFSET));
    let is_actor = actors::is_actor_type(e.mem.u8(this_obj.wrapping_add(addr::FORM_TYPE_OFFSET)));
    if is_actor && actors::shareable(speaker) {
        if let Some(v) = parse(e, a, true) {
            let player = e.mem.u32(addr::PLAYER_SINGLETON);
            if v[0] == player && v[1] != 0 {
                let topic = e.mem.u32(v[1].wrapping_add(addr::FORM_ID_OFFSET));
                e.ext_free::<Commands>()
                    .says
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .push((speaker, topic));
            }
        }
    }
    let ret = e.call_next(addr::CMD_SAY_TO, a);
    let n = e
        .ext_free::<Commands>()
        .says_seen
        .fetch_add(1, Ordering::SeqCst);
    if n < 40 && is_actor {
        note(
            e,
            format!("co-op says: actor {speaker:08x} was made to say a line (SayTo)"),
        );
    }
    ret
}

/// A scene played for a guest's ask: (actor, target, topic), and when.
type AskedScene = ((u32, u32, u32), std::time::Instant);

/// How long after a scene played for a guest's ask the host's own same scene is not played.
pub const ASKED_FOR: std::time::Duration = std::time::Duration::from_secs(60);

/// Host: scene `s`, asked by a guest's game, was played here.
pub fn played_for_a_guest(e: &mut Engine, s: &Scene) {
    e.ext_free::<Commands>()
        .asked
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(((s.actor, s.target, s.topic), std::time::Instant::now()));
}

/// Host: the lines scripts had an actor say to the player since the last call (speaker, topic).
pub fn drain_says(e: &mut Engine) -> Vec<(u32, u32)> {
    std::mem::take(
        &mut *e
            .ext_free::<Commands>()
            .says
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    )
}

/// Has actor `speaker` say topic `topic` to this game's player, as `SayTo` does for an actor in
/// high process that is not run at once: the line is said through its process, and its result
/// runs in this game when it ends.
pub fn say_to_player(e: &mut Engine, speaker: u32, topic: u32) -> Result<(), NoScene> {
    let (Some(a), Some(t)) = (actors::actor(e, speaker), crate::forms::form(e, topic)) else {
        return Err(NoScene::Absent);
    };
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    if player == 0 || actors::dead(e, speaker) != Some(false) {
        return Err(NoScene::Absent);
    }
    let process = |e: &mut Engine, r: u32| e.mem.u32(r.wrapping_add(addr::ACTOR_PROCESS_OFFSET));
    if process(e, a) == 0 || process(e, player) == 0 {
        return Err(NoScene::NotMobile);
    }
    e.call(addr::MOBILE_STOP_CURRENT_DIALOGUE, &args![a]);
    if e.vcall(a, addr::ACTOR_SAY_BUSY_SLOT, &args![1u32]).u32() & 0xff != 0 {
        return Err(NoScene::Refused);
    }
    if e.call(addr::MOBILE_PROCESS_TYPE, &args![a]).u32() == 0 {
        let pp = process(e, player);
        e.vcall(pp, addr::PROCESS_TALK_STATE_SLOT, &args![0u32]);
        let pp = process(e, player);
        e.vcall(pp, addr::PROCESS_TALK_TO_SLOT, &args![a]);
        e.call(addr::ACTOR_SAY_PAIR, &args![player, a]);
        let sp = process(e, a);
        e.vcall(sp, addr::PROCESS_TALK_STATE_SLOT, &args![0u32]);
        let sp = process(e, a);
        e.vcall(sp, addr::PROCESS_TALK_TO_SLOT, &args![player]);
    }
    e.call(addr::ACTOR_SAY_PAIR, &args![a, player]);
    let sp = process(e, a);
    e.vcall(
        sp,
        addr::PROCESS_SAY_TOPIC_SLOT,
        &args![a, t, 0u32, 0u32, 1u32, 1u32],
    );
    Ok(())
}

/// The wrap at `005c8740`. A conversation with the player on either side is that player's own
/// and runs as it is. One between two actors is the host's to decide: the host's game runs it
/// and notes it; a guest's game does not run its own (the command's parameters are still read,
/// so the script goes on after them) and plays the host's when it comes.
pub(crate) fn converse(e: &mut Engine, a: &[u32]) -> Ret {
    let this_obj = a.get(2).copied().unwrap_or(0);
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    let guest = gate::is_guest(e);
    let off = e.ext_free::<Commands>().scenes_off.load(Ordering::SeqCst);
    if off || this_obj == 0 || this_obj == player || !(guest || journal::armed(e)) {
        return e.call_next(addr::CMD_START_CONVERSATION, a);
    }
    let Some(v) = parse(e, a, true) else {
        return e.call_next(addr::CMD_START_CONVERSATION, a);
    };
    let target = v[0];
    if target == 0 || target == player {
        return e.call_next(addr::CMD_START_CONVERSATION, a);
    }
    let id_of = |e: &mut Engine, form: u32| match form {
        0 => 0,
        f => e.mem.u32(f.wrapping_add(addr::FORM_ID_OFFSET)),
    };
    let (actor_id, target_id) = (id_of(e, this_obj), id_of(e, target));
    let is_actor = |e: &mut Engine, f: u32| {
        actors::is_actor_type(e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET)))
    };
    if !actors::shareable(actor_id)
        || !actors::shareable(target_id)
        || !is_actor(e, this_obj)
        || !is_actor(e, target)
    {
        return e.call_next(addr::CMD_START_CONVERSATION, a);
    }
    let pos = |e: &mut Engine, r: u32| {
        [0, 4, 8].map(|o| e.mem.f32(r.wrapping_add(addr::REF_POS_OFFSET + o)))
    };
    if guest {
        // Read past the parameters as the body would, and say the command ran. The scene is
        // asked of the host's game, which plays it for everyone (PC run 23: the guest walked
        // into the trigger by Trudy and Joe Cobb and nothing happened anywhere).
        let _ = parse(e, a, false);
        let scene = Scene {
            actor: actor_id,
            target: target_id,
            topic: id_of(e, v[1]),
            locations: [id_of(e, v[2]), id_of(e, v[3])],
            flags: [v[4] == 1, v[5] == 1],
            space: fight::space_of(e, this_obj),
            actor_pos: pos(e, this_obj),
            target_pos: pos(e, target),
        };
        let c = e.ext_free::<Commands>();
        c.dropped.fetch_add(1, Ordering::SeqCst);
        c.scenes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(scene);
        note(
            e,
            format!(
                "co-op scenes: this game's own script asked actor {actor_id:08x} to talk to {target_id:08x}: asked of the host's game, which plays it for everyone"
            ),
        );
        return 1u32.into_ret();
    }
    // The same scene a guest's game asked of this one, played here a moment ago: not twice (a
    // trigger fires once in each game; D5, condition 5).
    let topic_id = id_of(e, v[1]);
    let recent = {
        let c = e.ext_free::<Commands>();
        let mut asked = c.asked.lock().unwrap_or_else(PoisonError::into_inner);
        asked.retain(|(_, at)| at.elapsed() < ASKED_FOR);
        asked
            .iter()
            .any(|(k, _)| *k == (actor_id, target_id, topic_id))
    };
    if recent {
        let _ = parse(e, a, false);
        note(
            e,
            format!(
                "co-op scenes: actor {actor_id:08x} with {target_id:08x}: already played for a guest's ask, not again"
            ),
        );
        return 1u32.into_ret();
    }
    let scene = Scene {
        actor: actor_id,
        target: target_id,
        topic: id_of(e, v[1]),
        locations: [id_of(e, v[2]), id_of(e, v[3])],
        flags: [v[4] == 1, v[5] == 1],
        space: fight::space_of(e, this_obj),
        actor_pos: pos(e, this_obj),
        target_pos: pos(e, target),
    };
    // Noted once the game's own body has run, so what a guest is told happened here.
    let ret = e.call_next(addr::CMD_START_CONVERSATION, a);
    e.ext_free::<Commands>()
        .scenes
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(scene);
    ret
}

/// Takes the conversations between actors that scripts started since the last call.
pub fn drain_scenes(e: &mut Engine) -> Vec<Scene> {
    let c = e.ext_free::<Commands>();
    std::mem::take(&mut *c.scenes.lock().unwrap_or_else(PoisonError::into_inner))
}

/// How many conversations between two actors this guest's own scripts asked for and did not get.
pub fn dropped_scenes(e: &mut Engine) -> u32 {
    e.ext_free::<Commands>().dropped.load(Ordering::SeqCst)
}

/// Why a host's conversation was not started here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoScene {
    /// One of the two is not in this game now, or is disabled.
    Absent,
    /// One of the two is not an actor the game can make talk.
    NotMobile,
    /// The game's own body would refuse it here: one of the two has no 3D or no process, is dead
    /// or fading out, or the actor's state forbids it.
    Refused,
}

/// Plays the host's conversation in this game: both are put where the host had them when they
/// are in that space here, then the actor starts the conversation as the command's body does
/// (the actor's virtual slot 0x280). The game's own checks beyond presence are the slot's.
pub fn play_scene(e: &mut Engine, s: &Scene) -> Result<(), NoScene> {
    let (Some(actor), Some(target)) = (
        actors::reference(e, s.actor),
        actors::reference(e, s.target),
    ) else {
        return Err(NoScene::Absent);
    };
    if actors::disabled(e, s.actor) != Some(false) || actors::disabled(e, s.target) != Some(false) {
        return Err(NoScene::Absent);
    }
    for r in [actor, target] {
        if e.vcall(r, addr::REF_IS_MOBILE_SLOT, &args![]).u32() & 0xff == 0 {
            return Err(NoScene::NotMobile);
        }
    }
    // The body's own checks before the slot (`run_005c8740.rs`), and two it has no need of in
    // its own game: both loaded here, and alive.
    for (r, id) in [(actor, s.actor), (target, s.target)] {
        if e.mem.u32(r.wrapping_add(addr::REF_LOADED_OFFSET)) == 0
            || e.mem.u32(r.wrapping_add(addr::ACTOR_PROCESS_OFFSET)) == 0
            || actors::dead(e, id) != Some(false)
            || (e.is_registered(addr::REF_IS_PENDING_FADE_DISABLED)
                && e.call(addr::REF_IS_PENDING_FADE_DISABLED, &args![r]).u32() & 0xff != 0)
        {
            return Err(NoScene::Refused);
        }
    }
    if e.vcall(actor, addr::ACTOR_SLOT_214, &args![]).u32() == 9 {
        return Err(NoScene::Refused);
    }
    for (r, pos) in [(actor, s.actor_pos), (target, s.target_pos)] {
        if fight::space_of(e, r) == s.space {
            for (axis, v) in (*b"XYZ").into_iter().zip(pos) {
                e.call(addr::SET_POS_BASE, &args![r, u32::from(axis), v.to_bits()]);
            }
        }
    }
    let topic = forms::form(e, s.topic).unwrap_or(0);
    let places = s.locations.map(|id| forms::form(e, id).unwrap_or(0));
    e.with_stack(2 * addr::PACKAGE_LOCATION_SIZE, |e, block| {
        let at = [block.addr(), block.addr() + addr::PACKAGE_LOCATION_SIZE];
        let mut used = [0u32; 2];
        for i in 0..2 {
            e.call(addr::PACKAGE_LOCATION_CONSTRUCT, &args![at[i]]);
            if places[i] != 0 {
                used[i] = at[i];
                e.call(
                    addr::PACKAGE_LOCATION_SET_REFERENCE,
                    &args![at[i], places[i]],
                );
            }
        }
        e.vcall(
            actor,
            addr::ACTOR_START_CONVERSATION_SLOT,
            &args![
                target,
                used[0],
                used[1],
                u32::from(s.flags[0]),
                u32::from(s.flags[1]),
                0u32,
                topic,
                0u32,
                0u32
            ],
        );
        // As the body: a dialogue package made by the slot is given to the player's record.
        if e.is_registered(addr::REF_GET_CURRENT_PACKAGE) {
            let made = e.call(addr::REF_GET_CURRENT_PACKAGE, &args![actor]).u32();
            if made != 0 && e.call(addr::PACKAGE_TYPE, &args![made]).u32() == 0x1c {
                let player = e.mem.u32(addr::PLAYER_SINGLETON);
                e.call(addr::PLAYER_SET_CONVERSATION_PACKAGE, &args![player, made]);
            }
        }
        e.call(addr::PACKAGE_LOCATION_DESTRUCT, &args![at[1]]);
        e.call(addr::PACKAGE_LOCATION_DESTRUCT, &args![at[0]]);
    });
    Ok(())
}

/// The wrap at `008a6ce0`: the game's call runs as it is; on a host whose journal is armed, an
/// evaluation asked for while a script runs, of an actor both games know, is noted.
pub(crate) fn evaluate(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a.first().copied().unwrap_or(0);
    if actor != 0 && journal::armed(e) && !gate::is_guest(e) && gate::cause(e).is_some() {
        let id = e.mem.u32(actor.wrapping_add(addr::FORM_ID_OFFSET));
        if actors::shareable(id) {
            let c = e.ext_free::<Commands>();
            let mut list = c.evaluate.lock().unwrap_or_else(PoisonError::into_inner);
            if !list.contains(&id) {
                list.push(id);
            }
        }
    }
    e.call_next(addr::ACTOR_EVALUATE_PACKAGE, a)
}

/// Takes the actors scripts told to evaluate their package since the last call.
pub fn drain_evaluations(e: &mut Engine) -> Vec<u32> {
    let c = e.ext_free::<Commands>();
    std::mem::take(&mut *c.evaluate.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Tells actor `id` to evaluate its package again, as a script's `evp` does (buffered; AI reset).
/// False when the actor is not in this game now.
/// Tells actor `id` to pick its package again without resetting its AI (buffered): an interrupt
/// it is in (a conversation) is not ended. False when the actor is not in this game now.
pub fn repick_package(e: &mut Engine, id: u32) -> bool {
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    e.call(addr::ACTOR_EVALUATE_PACKAGE, &args![a, 0u32, 0u32]);
    true
}

pub fn evaluate_package(e: &mut Engine, id: u32) -> bool {
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    e.call(addr::ACTOR_EVALUATE_PACKAGE, &args![a, 0u32, 1u32]);
    true
}
