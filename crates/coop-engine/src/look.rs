//! The other player's avatar as that player (docs/coop/SPECS.md S32): it walks and runs with the
//! game's own animations, holds and fires its weapon, wears what the player wears and has the
//! player's face, body and name.
//!
//! - **Moving.** An avatar used to be held with its AI off, which stops its animation altogether
//!   (`bProcessMe`, docs/coop/DECISIONS.md D6). [`lead`] keeps its AI on and puts it on its
//!   player's place every frame, and [`gait`] gives it the player's movement flags through the
//!   game's `ForceMoveMode`, the way a copy of an actor another game runs is led (D6 B2, D7): its
//!   legs walk, run or sneak as the player's do, and stand when the flags are 0. Its own AI is
//!   given nothing to do: a fight it starts by itself is ended at once ([`calm`]).
//! - **Weapon.** The player's drawn state, weapon and animation action are read ([`pose`]); every
//!   attack this game's player starts goes through `Actor::StartAttack` (`00893a40`, the player's
//!   attack button), and a wrap of it notes each one ([`player_attack`]). On an avatar the weapon
//!   is drawn or put away with the game's own call ([`draw`]), an attack is the game's
//!   `QueueAttack` ([`attack`]) and a reload its `QueueReload` ([`reload`]). An attack is a cue
//!   only: what it hits is decided by the hit wrap, as for any actor (S15).
//! - **Equipment.** What a reference wears is each item of its inventory the game says it wears
//!   (`InventoryChanges::WearingObject`); [`wear`] gives an avatar each item it lacks, as `AddItem`
//!   does, and has it put on and take off what it should through the `EquipItem` and
//!   `UnequipItem` commands' calls.
//! - **Face and body.** A [`Face`] is what character creation sets on the player's base form
//!   (sex, race, hair, eyes, hair colour, head parts, the face-gen coordinate, height, weight)
//!   and its name. [`base_copy`] makes a new NPC base with the game's own `CreateFormOfType` and
//!   `TESNPC::Copy` of this game's player base, and [`set_face`] writes another player's face onto
//!   it, so an avatar placed from it looks like that player and is named as that player.
//!
//! Nothing here runs unless a session asks for it; the wrap passes every call on unchanged.

use std::sync::atomic::{AtomicU32, Ordering};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::puppet::Attack;
use crate::{addr, forms, items, live, puppet};

/// Co-op's place in front of `Actor::StartAttack`.
pub(crate) const START_ATTACK: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_START_ATTACK);

/// The longest name kept (bytes).
pub const NAME_MAX: usize = 64;
/// The most values one face-gen matrix may hold here (the game's are a few tens).
pub const MATRIX_VALUES_MAX: u32 = 256;
/// The most head parts kept.
pub const HEAD_PARTS_MAX: usize = 16;
/// An avatar holding fewer rounds of its weapon's ammunition than this is given
/// [`AMMO_FILL`] more (co-op's own numbers: a presentation copy's attacks need ammunition, and
/// what it fires does no damage in its game's own count).
pub const AMMO_LOW: i32 = 50;
/// See [`AMMO_LOW`].
pub const AMMO_FILL: i32 = 500;
/// The most items read from one inventory.
const ITEMS_MAX: usize = 4096;

/// The game-wide state (a lock-free shared slot): the wrap runs on the game's thread, the
/// session reads between frames.
#[derive(Debug, Default)]
pub struct Looks {
    /// The last attack this game's player started: its count (bits 8..16, wrapping) and group
    /// (bits 0..8); 0 before the first.
    attack: AtomicU32,
}

/// Installs the wrap where the game has the function (the translation, or a test's stand-in);
/// called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    if e.is_registered(addr::ACTOR_START_ATTACK) {
        e.wrap(START_ATTACK, start_attack);
    }
    let _ = e.ext_free::<Looks>();
}

/// The wrap at `00893a40`. Its words are the call's: the actor and the animation group. An
/// attack the game started for this game's player is noted; every call goes on unchanged.
pub(crate) fn start_attack(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a.first().copied().unwrap_or(0);
    let group = a.get(1).copied().unwrap_or(0);
    let r = e.call_next(addr::ACTOR_START_ATTACK, a);
    if r.eax & 0xff != 0 && actor != 0 && live::player(e) == Some(actor) {
        let l = e.ext_free::<Looks>();
        let old = l.attack.load(Ordering::SeqCst);
        let seq = ((old >> 8) as u8).wrapping_add(1);
        l.attack
            .store((u32::from(seq) << 8) | (group & 0xff), Ordering::SeqCst);
    }
    r
}

/// The last attack this game's player started; `None` before its first.
pub fn player_attack(e: &mut Engine) -> Option<Attack> {
    let v = e.ext_free::<Looks>().attack.load(Ordering::SeqCst);
    (v != 0).then_some(Attack {
        seq: (v >> 8) as u8,
        group: v as u8,
    })
}

/// What this game's player is doing, as its avatar in the other games shows it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pose {
    /// Its movement flags ([`addr::ACTOR_MOVE_FLAGS`]: directions, walking, running, sneaking).
    pub moves: u16,
    /// Its weapon is drawn.
    pub drawn: bool,
    /// The form id of the weapon it holds, 0 for none.
    pub weapon: u32,
    /// It is reloading (its animation action is [`addr::ANIM_ACTION_RELOAD`]).
    pub reloading: bool,
}

fn id_of(e: &mut Engine, form: u32) -> u32 {
    match form {
        0 => 0,
        f => e.mem.u32(f.wrapping_add(addr::FORM_ID_OFFSET)),
    }
}

/// This game's player's [`Pose`]; `None` before the game has a player. A part the program has
/// no function for reads as nothing.
pub fn pose(e: &mut Engine) -> Option<Pose> {
    let p = live::player(e)?;
    Some(pose_of(e, p))
}

fn pose_of(e: &mut Engine, actor: u32) -> Pose {
    let mut out = Pose::default();
    if e.is_registered(addr::ACTOR_MOVE_FLAGS) {
        out.moves = e.call(addr::ACTOR_MOVE_FLAGS, &args![actor]).u32() as u16;
    }
    if e.is_registered(addr::ACTOR_IS_WEAPON_DRAWN) {
        out.drawn = e.call(addr::ACTOR_IS_WEAPON_DRAWN, &args![actor]).u32() & 0xff != 0;
    }
    if e.is_registered(addr::ACTOR_GET_CURRENT_WEAPON) {
        let w = e.call(addr::ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32();
        out.weapon = id_of(e, w);
    }
    if e.is_registered(addr::ACTOR_GET_ANIM_ACTION) {
        out.reloading =
            e.call(addr::ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == addr::ANIM_ACTION_RELOAD;
    }
    out
}

/// The [`Pose`] of avatar (or any actor) `id` in this game; `None` when it is not here.
pub fn pose_of_id(e: &mut Engine, id: u32) -> Option<Pose> {
    let a = forms::form(e, id)?;
    Some(pose_of(e, a))
}

/// Writes a reference's position and heading through the game's `SetPos` and `SetAngle` bodies
/// (as `live` places an avatar).
fn place(e: &mut Engine, r: u32, pos: [f32; 3], heading: f32) {
    for (axis, v) in (*b"XYZ").into_iter().zip(pos) {
        e.call(addr::SET_POS_BASE, &args![r, u32::from(axis), v.to_bits()]);
    }
    e.call(
        addr::SET_ANGLE_BASE,
        &args![r, u32::from(b'Z'), heading.to_degrees().to_bits()],
    );
}

/// Whether `pos` is within [`live::LOADED_REACH`] of this game's player, in the world.
fn near_player(e: &mut Engine, pos: [f32; 3]) -> bool {
    match live::player(e) {
        Some(p) if e.mem.u32(p.wrapping_add(addr::REF_LOADED_OFFSET)) != 0 => {
            let at = [0, 4, 8].map(|o| e.mem.f32(p.wrapping_add(addr::REF_POS_OFFSET + o)));
            (0..2).map(|i| (pos[i] - at[i]).powi(2)).sum::<f32>().sqrt() <= live::LOADED_REACH
        }
        _ => false,
    }
}

/// One frame of an avatar that is animated: as [`live::avatar_hold`], but its AI is kept on (so
/// the game animates it) instead of off. `Held::woke` is always false. `None` when the reference
/// is gone.
pub fn lead(e: &mut Engine, a: live::Avatar, pos: [f32; 3], heading: f32) -> Option<live::Held> {
    let r = forms::form(e, a.id)?;
    let flag = r.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET);
    if e.mem.u8(flag) == 0 {
        e.mem.set_u8(flag, 1);
    }
    let rehomed = live::bring_into_view(e, a.id, pos);
    place(e, r, pos, heading);
    let unseen = near_player(e, pos) && e.vcall(r, addr::REF_GET_3D_SLOT, &args![]).u32() == 0;
    Some(live::Held {
        woke: false,
        rehomed,
        pos: [0, 4, 8].map(|o| e.mem.f32(r.wrapping_add(addr::REF_POS_OFFSET + o))),
        unseen,
    })
}

/// Has avatar `id` move its legs as `moves` say (0: stand): the game's `ForceMoveMode`. False
/// when it is not here or has no mover.
pub fn gait(e: &mut Engine, id: u32, moves: u16) -> bool {
    puppet::force_moves(e, id, moves)
}

/// Ends a fight avatar `id`'s own AI started (it fights only as its player does). Whether one
/// was ended.
pub fn calm(e: &mut Engine, id: u32) -> bool {
    let Some(a) = forms::form(e, id) else {
        return false;
    };
    if !e.is_registered(addr::ACTOR_IN_COMBAT) || !e.call(addr::ACTOR_IN_COMBAT, &args![a]).bool() {
        return false;
    }
    puppet::stop(e, id)
}

/// Has avatar `id` draw its weapon (`true`) or put it away, through the game's own call. False
/// when it is not here or the program has no such function.
pub fn draw(e: &mut Engine, id: u32, drawn: bool) -> bool {
    match forms::form(e, id) {
        Some(a) if e.is_registered(addr::ACTOR_DRAW_WEAPON) => {
            e.call(addr::ACTOR_DRAW_WEAPON, &args![a, u32::from(drawn)]);
            true
        }
        _ => false,
    }
}

/// Has avatar `id` attack with animation group `group`: the game's own `QueueAttack`, which
/// starts it when its weapon is drawn and loaded. Whether the game took it.
pub fn attack(e: &mut Engine, id: u32, group: u8) -> bool {
    puppet::replay(e, id, group)
}

/// Has avatar `id` reload (`Actor::QueueReload`). False when it is not here or the program has
/// no such function.
pub fn reload(e: &mut Engine, id: u32) -> bool {
    match forms::form(e, id) {
        Some(a) if e.is_registered(addr::ACTOR_QUEUE_RELOAD) => {
            e.call(addr::ACTOR_QUEUE_RELOAD, &args![a]);
            true
        }
        _ => false,
    }
}

fn changes(e: &mut Engine, r: u32) -> u32 {
    if !e.is_registered(addr::REF_INVENTORY_CHANGES) {
        return 0;
    }
    e.call(addr::REF_INVENTORY_CHANGES, &args![r]).u32()
}

fn wears_form(e: &mut Engine, r: u32, form: u32) -> bool {
    match changes(e, r) {
        0 => false,
        c => {
            e.call(addr::CHANGES_WEARING_OBJECT, &args![c, form, 0u32])
                .u32()
                != 0
        }
    }
}

/// The form ids of what reference `id` wears or holds equipped (armour, clothes, its weapon, its
/// ammunition), in the order its inventory lists them; empty when it is not here or the
/// program has no inventory functions.
pub fn worn(e: &mut Engine, id: u32) -> Vec<u32> {
    let Some(r) = forms::form(e, id) else {
        return Vec::new();
    };
    if changes(e, r) == 0 || !e.is_registered(addr::CHANGES_WEARING_OBJECT) {
        return Vec::new();
    }
    let mut out: Vec<u32> = Vec::new();
    for s in items::list(e, r).into_iter().take(ITEMS_MAX) {
        if out.contains(&s.item) {
            continue;
        }
        if let Some(form) = forms::form(e, s.item) {
            if wears_form(e, r, form) {
                out.push(s.item);
            }
        }
    }
    out
}

/// What one [`wear`] did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Wear {
    /// Items put on.
    pub put_on: u32,
    /// Items taken off.
    pub taken_off: u32,
    /// Items (and ammunition) given to it first.
    pub given: u32,
}

/// Has avatar `id` wear each of `want` (form ids), giving it any it does not hold (one, as
/// `AddItem` gives), and its weapon's ammunition when it is low; with `strip`, it takes off
/// whatever else it wears. Items this game does not have are left out. Nothing is asked of the
/// game for an item already as wanted.
pub fn wear(e: &mut Engine, id: u32, want: &[u32], strip: bool) -> Wear {
    let mut out = Wear::default();
    let Some(a) = forms::form(e, id) else {
        return out;
    };
    if !e.is_registered(addr::ACTOR_QUEUE_EQUIP_OBJECT) || changes(e, a) == 0 {
        return out;
    }
    if strip {
        for item in worn(e, id) {
            if want.contains(&item) {
                continue;
            }
            if let Some(form) = forms::form(e, item) {
                e.call(
                    addr::ACTOR_QUEUE_UNEQUIP_OBJECT,
                    &args![a, form, 1u32, 0u32, 1u32, 0u32, 1u32],
                );
                out.taken_off += 1;
            }
        }
    }
    for &item in want {
        let Some(form) = items::item_form(e, item) else {
            continue;
        };
        if items::count(e, a, item) <= 0 {
            if !items::add(e, a, item, 1, None) {
                continue;
            }
            out.given += 1;
        }
        if e.mem.u8(form.wrapping_add(addr::FORM_TYPE_OFFSET)) == addr::FORM_TYPE_WEAPON
            && e.is_registered(addr::WEAPON_GET_CURRENT_AMMO)
        {
            let ammo = e.call(addr::WEAPON_GET_CURRENT_AMMO, &args![form, a]).u32();
            let ammo_id = id_of(e, ammo);
            if ammo_id != 0
                && items::count(e, a, ammo_id) < AMMO_LOW
                && items::add(e, a, ammo_id, AMMO_FILL, None)
            {
                out.given += 1;
            }
        }
        if !wears_form(e, a, form) {
            e.call(
                addr::ACTOR_QUEUE_EQUIP_OBJECT,
                &args![a, form, 1u32, 0u32, 1u32, 0u32, 1u32],
            );
            out.put_on += 1;
        }
    }
    out
}

/// What character creation sets on a player's base form, and its name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Face {
    /// The female flag.
    pub female: bool,
    /// The race's form id.
    pub race: u32,
    /// The hair's form id (0: none).
    pub hair: u32,
    /// The hair's length.
    pub hair_length: f32,
    /// The eyes' form id (0: none).
    pub eyes: u32,
    /// The hair colour (the game's packed colour).
    pub hair_color: u32,
    /// Height and weight.
    pub height: f32,
    /// See [`Face::height`].
    pub weight: f32,
    /// The head parts' form ids, in the game's order.
    pub head_parts: Vec<u32>,
    /// The face-gen coordinate's four matrices: rows, columns and values (row by row).
    pub coord: [(u32, u32, Vec<f32>); 4],
    /// The name.
    pub name: String,
}

fn form_at(e: &mut Engine, at: u32) -> u32 {
    let f = e.mem.u32(at);
    id_of(e, f)
}

fn c_string(e: &mut Engine, at: u32, max: usize) -> String {
    let mut bytes = Vec::new();
    if at == 0 || !e.mem.is_mapped(at) {
        return String::new();
    }
    while bytes.len() < max {
        let b = e.mem.u8(at + bytes.len() as u32);
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// The NPC base at `npc` (a pointer) as a [`Face`].
pub fn face_of(e: &mut Engine, npc: u32) -> Face {
    let mut f = Face {
        female: e.mem.u32(npc + addr::NPC_BASE_FLAGS_OFFSET) & addr::NPC_FLAG_FEMALE != 0,
        race: form_at(e, npc + addr::NPC_RACE_OFFSET),
        hair: form_at(e, npc + addr::NPC_HAIR_OFFSET),
        hair_length: e.mem.f32(npc + addr::NPC_HAIR_LENGTH_OFFSET),
        eyes: form_at(e, npc + addr::NPC_EYES_OFFSET),
        hair_color: e.mem.u32(npc + addr::NPC_HAIR_COLOR_OFFSET),
        height: e.mem.f32(npc + addr::NPC_HEIGHT_OFFSET),
        weight: e.mem.f32(npc + addr::NPC_WEIGHT_OFFSET),
        ..Face::default()
    };
    let mut node = npc + addr::NPC_HEAD_PARTS_OFFSET;
    while node != 0 && f.head_parts.len() < HEAD_PARTS_MAX {
        let part = e.mem.u32(node);
        if part != 0 {
            let id = id_of(e, part);
            f.head_parts.push(id);
        }
        node = e.mem.u32(node + 4);
    }
    let coord = e.call(addr::NPC_FACE_COORD, &args![npc]).u32();
    if coord != 0 {
        for (k, m) in f.coord.iter_mut().enumerate() {
            let at = coord + k as u32 * addr::MATRIX_SIZE;
            let rows = e.mem.u32(at + addr::MATRIX_ROWS_OFFSET);
            let cols = e.mem.u32(at + addr::MATRIX_COLUMNS_OFFSET);
            let n = rows.saturating_mul(cols);
            let data = e.mem.u32(at + addr::MATRIX_DATA_OFFSET);
            if n == 0 || n > MATRIX_VALUES_MAX || data == 0 {
                continue;
            }
            *m = (
                rows,
                cols,
                (0..n).map(|i| e.mem.f32(data + 4 * i)).collect(),
            );
        }
    }
    let text = e.call(addr::FULL_NAME_TEXT, &args![npc]).u32();
    f.name = c_string(e, text, NAME_MAX);
    f
}

/// This game's player's [`Face`] (its base form's); `None` before the game has a player.
pub fn player_face(e: &mut Engine) -> Option<Face> {
    let p = live::player(e)?;
    let base = e.mem.u32(p + addr::REF_BASE_FORM_OFFSET);
    (base != 0).then(|| face_of(e, base))
}

/// A new NPC base, made by the game (`CreateFormOfType`, which gives it a new id of its own) as
/// a copy of the NPC base `from` (`TESNPC::Copy`). Its form id; `None` when `from` is not an NPC
/// of this game, or the game made nothing.
pub fn base_copy(e: &mut Engine, from: u32) -> Option<u32> {
    let source = forms::form(e, from)?;
    if e.mem.u8(source + addr::FORM_TYPE_OFFSET) != addr::FORM_TYPE_NPC
        || !e.is_registered(addr::CREATE_FORM_OF_TYPE)
    {
        return None;
    }
    let npc = e
        .call(
            addr::CREATE_FORM_OF_TYPE,
            &args![u32::from(addr::FORM_TYPE_NPC)],
        )
        .u32();
    if npc == 0 {
        return None;
    }
    e.vcall(npc, addr::FORM_COPY_SLOT, &args![source]);
    match id_of(e, npc) {
        0 => None,
        id => Some(id),
    }
}

/// Writes `face` onto the NPC base `id` (one [`base_copy`] made): sex, race, hair, eyes, hair
/// colour, head parts, face-gen coordinate, height, weight and name. Forms this game does not
/// have are left as they are. Its face NPC is cleared (the copy made it the source's), so the
/// game takes its face from it. False when `id` is not an NPC of this game.
pub fn set_face(e: &mut Engine, id: u32, face: &Face) -> bool {
    let Some(npc) = forms::form(e, id) else {
        return false;
    };
    if e.mem.u8(npc + addr::FORM_TYPE_OFFSET) != addr::FORM_TYPE_NPC {
        return false;
    }
    let flags = e.mem.u32(npc + addr::NPC_BASE_FLAGS_OFFSET);
    let flags = if face.female {
        flags | addr::NPC_FLAG_FEMALE
    } else {
        flags & !addr::NPC_FLAG_FEMALE
    };
    e.mem.set_u32(npc + addr::NPC_BASE_FLAGS_OFFSET, flags);
    for (at, form) in [
        (addr::NPC_RACE_OFFSET, face.race),
        (addr::NPC_HAIR_OFFSET, face.hair),
        (addr::NPC_EYES_OFFSET, face.eyes),
    ] {
        if form == 0 {
            if at != addr::NPC_RACE_OFFSET {
                e.mem.set_u32(npc + at, 0);
            }
        } else if let Some(f) = forms::form(e, form) {
            e.mem.set_u32(npc + at, f);
        }
    }
    e.mem
        .set_f32(npc + addr::NPC_HAIR_LENGTH_OFFSET, face.hair_length);
    e.mem
        .set_u32(npc + addr::NPC_HAIR_COLOR_OFFSET, face.hair_color);
    e.mem.set_f32(npc + addr::NPC_HEIGHT_OFFSET, face.height);
    e.mem.set_f32(npc + addr::NPC_WEIGHT_OFFSET, face.weight);
    e.mem.set_u32(npc + addr::NPC_FACE_NPC_OFFSET, 0);
    let list = npc + addr::NPC_HEAD_PARTS_OFFSET;
    e.call(addr::LIST_CLEAR, &args![list]);
    for part in face.head_parts.iter().take(HEAD_PARTS_MAX) {
        let Some(f) = forms::form(e, *part) else {
            continue;
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), f);
            e.call(addr::LIST_APPEND, &args![list, slot.addr()]);
        });
    }
    let coord = e.call(addr::NPC_FACE_COORD, &args![npc]).u32();
    if coord != 0 {
        for (k, (rows, cols, values)) in face.coord.iter().enumerate() {
            let at = coord + k as u32 * addr::MATRIX_SIZE;
            let n = rows.saturating_mul(*cols);
            if n > MATRIX_VALUES_MAX || values.len() != n as usize {
                continue;
            }
            e.call(addr::MATRIX_RESIZE, &args![at, *rows, *cols]);
            let data = e.mem.u32(at + addr::MATRIX_DATA_OFFSET);
            if data == 0 {
                continue;
            }
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(data + 4 * i as u32, *v);
            }
        }
    }
    let name: Vec<u8> = face
        .name
        .bytes()
        .filter(|b| *b != 0)
        .take(NAME_MAX)
        .collect();
    if !name.is_empty() {
        e.with_stack(name.len() as u32 + 1, |e, text| {
            for (i, b) in name.iter().enumerate() {
                e.mem.set_u8(text.addr() + i as u32, *b);
            }
            e.mem.set_u8(text.addr() + name.len() as u32, 0);
            e.call(
                addr::STRING_SET,
                &args![npc + addr::NPC_NAME_STRING_OFFSET, text.addr(), 0u32],
            );
        });
    }
    true
}

/// The [`Face`] of the NPC base `id` in this game; `None` when it is not an NPC here.
pub fn face_of_id(e: &mut Engine, id: u32) -> Option<Face> {
    let npc = forms::form(e, id)?;
    (e.mem.u8(npc + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_NPC).then(|| face_of(e, npc))
}

/// The base form avatar (or any reference) `id` was placed from; `None` when it is not here.
pub fn base_of(e: &mut Engine, id: u32) -> Option<u32> {
    let r = forms::form(e, id)?;
    let base = e.mem.u32(r + addr::REF_BASE_FORM_OFFSET);
    (base != 0).then(|| id_of(e, base))
}

#[cfg(test)]
#[path = "look_tests.rs"]
mod tests;
