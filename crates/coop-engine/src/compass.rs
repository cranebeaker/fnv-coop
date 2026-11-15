//! The other players on the compass (C-14): each one this game shows (an avatar) has a tick of its
//! own on the HUD's compass, in purple, a colour the game's HUD never uses (its colours are the
//! HUD colour the player picks, and the warning colour of a hostile tick).
//!
//! The game ticks the actors the player's list names (`PlayerCharacter +0xd48`) once per frame in
//! `HUDMainMenu::UpdateCompass` (`00779070`), from a set of tiles made with the HUD (one per
//! `iMaxCompassIcons`), each in the HUD colour or, for a hostile actor, the warning colour. A wrap
//! of that function:
//!
//! - keeps the avatars out of the game's own ticks for that call (the list's byte that keeps an
//!   actor off the compass, set and put back around the call), so a player is never ticked twice
//!   or in an NPC's colour;
//! - then ticks each avatar itself, the way the game ticks an actor (the same place on the strip,
//!   the same fade at its ends, hidden when behind the player), on one of the tiles the game left
//!   unused, its system colour taken away so that the tile shows its own `red`, `green` and
//!   `blue`. A tile the game uses again in a later frame gets its system colour back from the
//!   game, so no NPC is ever left purple.
//!
//! Unlike an NPC's, an avatar's tick is not limited by distance or by being detected: the other
//! player is shown wherever they are in this game (an avatar exists only while both players are
//! in the same interior or worldspace).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr};

/// Co-op's place in front of `HUDMainMenu::UpdateCompass`.
pub(crate) const UPDATE: WrapSlot = WrapSlot::new(Owner::Coop, addr::HUD_UPDATE_COMPASS);

/// The other players' tick colour (red, green, blue, 0..255): a purple none of the game's HUD
/// colours (amber, white, blue, green, and the warning red) is near.
pub const PURPLE: [u8; 3] = [190, 80, 255];

/// Most list entries looked at (the list is the game's; a broken one is not walked forever).
const LIST_MAX: usize = 4096;
/// The game's own numbers in `UpdateCompass`: the actor tick is put this far right of its place
/// (`01011590`), it fades out past this part of the strip (`010217d8`), and alpha is 0..255
/// (`0101e568`).
const TICK_NUDGE: f64 = 2.0;
const FADE_FROM: f64 = 0.8f32 as f64;
const ALPHA_FULL: f64 = 255.0;

/// The marks' game-wide state (a lock-free shared slot, as the copies'): the wrap runs where the
/// game draws its HUD, the session writes between frames.
#[derive(Debug, Default)]
pub struct Marks {
    /// The other players' avatars in this game: their form ids.
    players: RwLock<Vec<u32>>,
    /// Ticks drawn, ever.
    drawn: AtomicU64,
    /// Avatars behind the player (outside the strip), ever (one per frame each).
    behind: AtomicU64,
    /// Avatars with no tile left to draw them on, ever.
    no_tile: AtomicU64,
}

/// What [`counts`] answers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Ticks drawn, ever.
    pub drawn: u64,
    /// Avatars that were behind the player (not on the strip), ever.
    pub behind: u64,
    /// Avatars with no tile to draw them on, ever.
    pub no_tile: u64,
}

/// Installs the wrap where the game has the function (the translation, or a test's stand-in);
/// called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    if e.is_registered(addr::HUD_UPDATE_COMPASS) {
        e.wrap(UPDATE, update_compass);
    }
    let _ = e.ext_free::<Marks>();
}

/// Names the other players' avatars in this game (form ids); every other actor is ticked by the
/// game as before.
pub fn set_players(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    *e.ext_free::<Marks>()
        .players
        .write()
        .unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// The marks drawn and not drawn so far.
pub fn counts(e: &mut Engine) -> Counts {
    let m = e.ext_free::<Marks>();
    Counts {
        drawn: m.drawn.load(Ordering::SeqCst),
        behind: m.behind.load(Ordering::SeqCst),
        no_tile: m.no_tile.load(Ordering::SeqCst),
    }
}

fn players(e: &mut Engine) -> Vec<u32> {
    e.ext_free::<Marks>()
        .players
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

/// The wrap at `00779070`. Its words are the call's: the compass, the landmark marks, the
/// objective marks and the actor ticks (the group whose tiles are the ticks).
pub(crate) fn update_compass(e: &mut Engine, a: &[u32]) -> Ret {
    let ids = players(e);
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    let avatars: Vec<u32> = ids
        .into_iter()
        .filter_map(|id| actors::actor(e, id))
        .collect();
    // (no other player here: the game's frame as it is, without walking its list and tiles)
    if avatars.is_empty() || player == 0 {
        return e.call_next(addr::HUD_UPDATE_COMPASS, a);
    }
    let kept = keep_out(e, player, &avatars);
    let r = e.call_next(addr::HUD_UPDATE_COMPASS, a);
    for item in kept {
        e.mem
            .set_u8(item.wrapping_add(addr::COMPASS_ACTOR_SKIP_OFFSET), 0);
    }
    let ticks = a.get(3).copied().unwrap_or(0);
    mark(e, player, ticks, &avatars);
    r
}

/// Sets the keep-off byte of every entry of the player's compass list that is one of `avatars`
/// and did not have it; answers those entries, for the byte to be put back.
fn keep_out(e: &mut Engine, player: u32, avatars: &[u32]) -> Vec<u32> {
    let mut kept = Vec::new();
    let mut node = e
        .mem
        .u32(player.wrapping_add(addr::PLAYER_COMPASS_ACTORS_OFFSET));
    for _ in 0..LIST_MAX {
        if node == 0 {
            break;
        }
        let item = e.mem.u32(node);
        if item != 0 && avatars.contains(&e.mem.u32(item)) {
            let skip = item.wrapping_add(addr::COMPASS_ACTOR_SKIP_OFFSET);
            if e.mem.u8(skip) == 0 {
                e.mem.set_u8(skip, 1);
                kept.push(item);
            }
        }
        node = e.mem.u32(node.wrapping_add(4));
    }
    kept
}

fn place(e: &mut Engine, r: u32) -> [f32; 3] {
    let p = e.vcall(r, addr::REF_GET_POS_SLOT, &args![]).u32();
    [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
}

/// The tiles of the group `ticks` (its children, in order), each with whether it is shown.
fn tiles(e: &mut Engine, ticks: u32) -> Vec<(u32, bool)> {
    let mut out = Vec::new();
    // `Tile::xChildren`: the head node at +4; a node is (next, previous, tile).
    let mut node = e.mem.u32(ticks.wrapping_add(4));
    for _ in 0..LIST_MAX {
        if node == 0 {
            break;
        }
        let tile = e.mem.u32(node.wrapping_add(8));
        if tile != 0 {
            let shown = e
                .call(addr::TILE_GET_FLOAT, &args![tile, addr::TRAIT_VISIBLE])
                .f32()
                != 0.0;
            out.push((tile, shown));
        }
        node = e.mem.u32(node);
    }
    out
}

/// Takes a tile for one more tick: the first the game left hidden, else (every tile was used,
/// the farthest actors last) the last one; never one taken already.
fn take(free: &mut Vec<(u32, bool)>) -> Option<u32> {
    let at = free
        .iter()
        .position(|(_, shown)| !shown)
        .or_else(|| free.len().checked_sub(1))?;
    Some(free.remove(at).0)
}

fn set_float(e: &mut Engine, tile: u32, which: u32, v: f32) {
    e.call(addr::TILE_SET_FLOAT, &args![tile, which, v, 1u32]);
}

fn set_int(e: &mut Engine, tile: u32, which: u32, v: i32) {
    e.call(addr::TILE_SET_INT, &args![tile, which, v]);
}

/// The alpha the game gives a tick at its full (`UpdateCompass`): the HUD's opacity setting, or
/// full while the word at `011f2250 + 8` is not 0 (`VATS`'s mode: V.A.T.S. is up), times 255.
fn full_alpha(e: &mut Engine) -> f32 {
    let opacity = if e.mem.u32(addr::VATS + addr::VATS_MODE_OFFSET) == 0 {
        let v = e
            .call(addr::SETTING_VALUE, &args![addr::HUD_OPACITY_SETTING])
            .u32();
        e.mem.f32(v)
    } else {
        1.0
    };
    (f64::from(opacity) * ALPHA_FULL) as f32
}

/// Ticks each of `avatars` (pointers) on a tile of the group `ticks` the game left over.
fn mark(e: &mut Engine, player: u32, ticks: u32, avatars: &[u32]) {
    let hud = e.mem.u32(addr::HUD_SINGLETON);
    let heading = e.mem.u32(addr::COMPASS_HEADING_TRAIT);
    if hud == 0 || ticks == 0 || heading == 0 {
        return;
    }
    let half = e
        .mem
        .i32(hud.wrapping_add(addr::HUD_COMPASS_HALF_ANGLE_OFFSET));
    let width = e
        .mem
        .i32(hud.wrapping_add(addr::HUD_COMPASS_DISPLAY_WIDTH_OFFSET));
    let inner = (f64::from(width) * FADE_FROM * 2.0) as f32;
    let outer = (f64::from(width) * 2.0) as f32;
    let alpha_full = full_alpha(e);
    let me = place(e, player);
    let mut free = tiles(e, ticks);
    for &avatar in avatars {
        let at = place(e, avatar);
        let angle = e
            .call(
                addr::HUD_GET_COMPASS_ANGLE,
                &args![at[0], at[1], at[2], me[0], me[1], me[2]],
            )
            .f64() as f32;
        if f64::from(angle).abs() >= f64::from(half) {
            e.ext_free::<Marks>().behind.fetch_add(1, Ordering::SeqCst);
            continue;
        }
        let Some(tile) = take(&mut free) else {
            e.ext_free::<Marks>().no_tile.fetch_add(1, Ordering::SeqCst);
            continue;
        };
        let x = e
            .call(
                addr::LERP,
                &args![
                    0.0f32,
                    width.wrapping_shl(1) as f32,
                    half.wrapping_neg() as f32,
                    half as f32,
                    angle
                ],
            )
            .f32();
        e.call(
            addr::TILE_SET_STRING,
            &args![tile, addr::TRAIT_FILENAME, addr::TICK_TEXTURE, 1u32],
        );
        set_float(e, tile, heading, (f64::from(x) + TICK_NUDGE) as f32);
        set_int(e, tile, addr::TRAIT_VISIBLE, 1);
        set_int(e, tile, addr::TRAIT_SYSTEM_COLOR, 0);
        // voiceui: a speaking player's tick in its own colour, at full alpha.
        let id = e.mem.u32(avatar.wrapping_add(addr::FORM_ID_OFFSET));
        let speaks = crate::voicemark::speaking(e, id);
        for (which, v) in [addr::TRAIT_RED, addr::TRAIT_GREEN, addr::TRAIT_BLUE]
            .into_iter()
            .zip(crate::voicemark::tick_colour(e, id))
        {
            set_float(e, tile, which, f32::from(v));
        }
        let alpha = if x <= inner || speaks {
            alpha_full
        } else {
            e.call(addr::LERP, &args![alpha_full, 0.0f32, inner, outer, x])
                .f32()
        };
        set_float(e, tile, addr::TRAIT_ALPHA, alpha);
        e.ext_free::<Marks>().drawn.fetch_add(1, Ordering::SeqCst);
    }
}

/// One tick tile as the HUD has it now (for the harness).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tick {
    /// Shown.
    pub shown: bool,
    /// Its place across the strip (the `_Heading` trait).
    pub heading: f32,
    /// Its `systemcolor` trait.
    pub system_color: f32,
    /// Its `red`, `green`, `blue` traits.
    pub rgb: [f32; 3],
    /// Its `alpha` trait.
    pub alpha: f32,
    /// The colour its picture is drawn in (0..1 each): what the tile's last colour update wrote
    /// to its shader property; `None` before it has one.
    pub drawn: Option<[f32; 3]>,
}

/// The actor tick tiles of the HUD now, in order; empty without a HUD.
pub fn ticks(e: &mut Engine) -> Vec<Tick> {
    let hud = e.mem.u32(addr::HUD_SINGLETON);
    if hud == 0 {
        return Vec::new();
    }
    let group = e.mem.u32(hud.wrapping_add(addr::HUD_ACTOR_TICKS_OFFSET));
    if group == 0 {
        return Vec::new();
    }
    let heading = e.mem.u32(addr::COMPASS_HEADING_TRAIT);
    let get = |e: &mut Engine, tile: u32, which: u32| {
        e.call(addr::TILE_GET_FLOAT, &args![tile, which]).f32()
    };
    tiles(e, group)
        .into_iter()
        .map(|(tile, shown)| Tick {
            shown,
            heading: if heading == 0 {
                0.0
            } else {
                get(e, tile, heading)
            },
            system_color: get(e, tile, addr::TRAIT_SYSTEM_COLOR),
            rgb: [
                get(e, tile, addr::TRAIT_RED),
                get(e, tile, addr::TRAIT_GREEN),
                get(e, tile, addr::TRAIT_BLUE),
            ],
            alpha: get(e, tile, addr::TRAIT_ALPHA),
            drawn: drawn_colour(e, tile),
        })
        .collect()
}

/// The colour a tile's picture is drawn in: its model's first geometry's shader property colour.
fn drawn_colour(e: &mut Engine, tile: u32) -> Option<[f32; 3]> {
    // `Tile::spModel` (+0x2c), an `NiNode`: children at +0xa0, how many at +0xa6.
    let model = e.mem.u32(tile.wrapping_add(0x2c));
    if model == 0 {
        return None;
    }
    let children = e.mem.u32(model + 0xa0);
    for i in 0..u32::from(e.mem.u16(model + 0xa6)) {
        let child = e.mem.u32(children + i * 4);
        if child == 0 {
            continue;
        }
        let geometry = e.vcall(child, 0x18, &args![]).u32();
        if geometry == 0 {
            continue;
        }
        let p = e
            .call(addr::NI_OBJECT_GET_PROPERTY, &args![geometry, 3u32])
            .u32();
        if p == 0 {
            return None;
        }
        return Some([
            e.mem.f32(p + 0x68),
            e.mem.f32(p + 0x6c),
            e.mem.f32(p + 0x70),
        ]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    // A small world: the player and two actors (an avatar and an NPC), the player's compass list,
    // a HUD and a group of three tick tiles whose traits are kept in a map.
    const PLAYER: u32 = 0x0300_0000;
    const AVATAR: u32 = 0x0300_1000;
    const NPC: u32 = 0x0300_2000;
    const AVATAR_ID: u32 = 0xff00_0001;
    const NPC_ID: u32 = 0x0001_0001;
    const VTABLE: u32 = 0x0301_0000;
    const GET_POS: u32 = 0x0302_0000;
    const NODES: u32 = 0x0303_0000;
    const ITEMS: u32 = 0x0304_0000;
    const HUD: u32 = 0x0305_0000;
    const GROUP: u32 = 0x0306_0000;
    const CHILDREN: u32 = 0x0306_1000;
    const TILES: [u32; 3] = [0x0307_0000, 0x0307_1000, 0x0307_2000];
    const HEADING: u32 = 0x1005;
    const OPACITY: u32 = 0x0308_0000;
    // What the game's stand-in saw of the avatar's entry: its keep-off byte during the call.
    const SEEN_SKIP: u32 = 0x0309_0000;
    // Traits by (tile, trait), as floats.
    type Traits = BTreeMap<(u32, u32), f32>;

    fn traits(e: &mut Engine) -> &mut Traits {
        e.ext::<Traits>()
    }

    fn eax(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn st0(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    fn get(e: &mut Engine, tile: u32, which: u32) -> f32 {
        traits(e).get(&(tile, which)).copied().unwrap_or(0.0)
    }

    fn put(e: &mut Engine, tile: u32, which: u32, v: f32) {
        traits(e).insert((tile, which), v);
    }

    fn actor(e: &mut Engine, r: u32, id: u32, pos: [f32; 3]) {
        e.mem.set_u32(r, VTABLE);
        e.mem.set_u8(r + addr::FORM_TYPE_OFFSET, 0x3c);
        e.mem.set_u32(r + addr::FORM_ID_OFFSET, id);
        for (i, v) in pos.into_iter().enumerate() {
            e.mem.set_f32(r + addr::REF_POS_OFFSET + i as u32 * 4, v);
        }
    }

    /// The game's tick pass, cut down: every entry not kept off takes the next tile, in the
    /// HUD colour; the tiles left over are hidden.
    fn game_pass(e: &mut Engine, a: &[u32]) -> Ret {
        let item = ITEMS;
        e.mem.set_u8(SEEN_SKIP, e.mem.u8(item + 5));
        let mut next = 0;
        let mut node = e.mem.u32(PLAYER + addr::PLAYER_COMPASS_ACTORS_OFFSET);
        while node != 0 {
            let item = e.mem.u32(node);
            if e.mem.u8(item + 5) == 0 {
                put(e, TILES[next], addr::TRAIT_VISIBLE, 1.0);
                put(e, TILES[next], addr::TRAIT_SYSTEM_COLOR, 1.0);
                put(e, TILES[next], HEADING, 500.0);
                next += 1;
            }
            node = e.mem.u32(node + 4);
        }
        for &t in &TILES[next..] {
            put(e, t, addr::TRAIT_VISIBLE, 0.0);
        }
        let _ = a;
        e.mem.set_u32(addr::COMPASS_HEADING_TRAIT, HEADING);
        Ret::default()
    }

    /// The world with the avatar at `avatar` and the NPC, both in the player's compass list.
    fn world(avatar: [f32; 3]) -> Engine {
        world_with(avatar, game_pass)
    }

    /// [`world`], with `game` as the game's `UpdateCompass`.
    fn world_with(avatar: [f32; 3], game: fn(&mut Engine, &[u32]) -> Ret) -> Engine {
        let mut e = Engine::new();
        e.mem.map(PLAYER, 0x0a0_0000);
        // The game's globals: the player, the HUD, the heading trait's id, V.A.T.S.
        e.mem.map(0x011d_0000, 0x2_0000);
        e.mem.map(addr::VATS & !0xfff, 0x1000);
        e.register(addr::LOOKUP_FORM, |_, a| {
            eax(match a[0] {
                AVATAR_ID => AVATAR,
                NPC_ID => NPC,
                _ => 0,
            })
        });
        e.mem.set_u32(VTABLE + addr::REF_GET_POS_SLOT, GET_POS);
        e.register(GET_POS, |_, a| eax(a[0] + addr::REF_POS_OFFSET));
        actor(&mut e, PLAYER, 0x14, [0.0, 0.0, 0.0]);
        actor(&mut e, AVATAR, AVATAR_ID, avatar);
        actor(&mut e, NPC, NPC_ID, [0.0, 100.0, 0.0]);
        e.mem.set_u32(addr::PLAYER_SINGLETON, PLAYER);
        // The list: the avatar's entry, then the NPC's.
        e.mem
            .set_u32(PLAYER + addr::PLAYER_COMPASS_ACTORS_OFFSET, NODES);
        e.mem.set_u32(NODES, ITEMS);
        e.mem.set_u32(NODES + 4, NODES + 8);
        e.mem.set_u32(NODES + 8, ITEMS + 8);
        e.mem.set_u32(NODES + 12, 0);
        e.mem.set_u32(ITEMS, AVATAR);
        e.mem.set_u32(ITEMS + 8, NPC);
        // The HUD: a strip 300 wide each side of its middle, 90 compass degrees each side.
        e.mem.set_u32(addr::HUD_SINGLETON, HUD);
        e.mem
            .set_i32(HUD + addr::HUD_COMPASS_DISPLAY_WIDTH_OFFSET, 300);
        e.mem.set_i32(HUD + addr::HUD_COMPASS_HALF_ANGLE_OFFSET, 90);
        e.mem.set_f32(OPACITY, 1.0);
        e.register(addr::SETTING_VALUE, |_, _| eax(OPACITY));
        // The tick group's three children.
        e.mem.set_u32(GROUP + 4, CHILDREN);
        for (i, t) in TILES.into_iter().enumerate() {
            let node = CHILDREN + i as u32 * 0x10;
            let next = if i + 1 < TILES.len() { node + 0x10 } else { 0 };
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, t);
        }
        e.register(addr::TILE_GET_FLOAT, |e, a| {
            st0(f64::from(get(e, a[0], a[1])))
        });
        e.register(addr::TILE_SET_FLOAT, |e, a| {
            put(e, a[0], a[1], f32::from_bits(a[2]));
            Ret::default()
        });
        e.register(addr::TILE_SET_INT, |e, a| {
            put(e, a[0], a[1], a[2] as i32 as f32);
            Ret::default()
        });
        e.register(addr::TILE_SET_STRING, |e, a| {
            put(e, a[0], a[1], a[2] as f32);
            Ret::default()
        });
        // The compass angle: degrees of the bearing (the player faces +Y), as the game's.
        e.register(addr::HUD_GET_COMPASS_ANGLE, |_, a| {
            let f = |i: usize| f32::from_bits(a[i]);
            let (dx, dy) = (f(0) - f(3), f(1) - f(4));
            st0(f64::from(dx.atan2(dy).to_degrees()))
        });
        e.register(addr::LERP, |_, a| {
            let f = |i: usize| f64::from(f32::from_bits(a[i]));
            let (a0, b, low, high, x) = (f(0), f(1), f(2), f(3), f(4));
            st0(f64::from((a0 + (b - a0) * (x - low) / (high - low)) as f32))
        });
        e.register(addr::HUD_UPDATE_COMPASS, game);
        install(&mut e);
        e
    }

    fn update(e: &mut Engine) {
        e.call(addr::HUD_UPDATE_COMPASS, &args![0u32, 0u32, 0u32, GROUP]);
    }

    fn purple(e: &mut Engine, tile: u32) -> bool {
        get(e, tile, addr::TRAIT_SYSTEM_COLOR) == 0.0
            && get(e, tile, addr::TRAIT_RED) == 190.0
            && get(e, tile, addr::TRAIT_GREEN) == 80.0
            && get(e, tile, addr::TRAIT_BLUE) == 255.0
    }

    #[test]
    fn the_other_player_gets_a_purple_tick_and_no_npc_tick() {
        // The avatar ahead and to the right: 45 compass degrees.
        let mut e = world([100.0, 100.0, 0.0]);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        // The game ticked only the NPC (first tile, HUD colour); the avatar took the next.
        assert_eq!(
            e.mem.u8(SEEN_SKIP),
            1,
            "the avatar was kept off the game's ticks"
        );
        assert_eq!(get(&mut e, TILES[0], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert!(purple(&mut e, TILES[1]));
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_VISIBLE), 1.0);
        assert_eq!(
            get(&mut e, TILES[1], addr::TRAIT_FILENAME),
            addr::TICK_TEXTURE as f32
        );
        // 45 of 90 degrees: three quarters along the 600-wide strip, nudged as the game's.
        assert_eq!(get(&mut e, TILES[1], HEADING), 452.0);
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_ALPHA), 255.0);
        assert_eq!(get(&mut e, TILES[2], addr::TRAIT_VISIBLE), 0.0);
        // The list is as it was.
        assert_eq!(e.mem.u8(ITEMS + 5), 0);
        assert_eq!(counts(&mut e).drawn, 1);
    }

    #[test]
    fn without_players_the_game_ticks_everyone() {
        let mut e = world([100.0, 100.0, 0.0]);
        update(&mut e);
        assert_eq!(e.mem.u8(SEEN_SKIP), 0);
        assert_eq!(get(&mut e, TILES[0], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert_eq!(get(&mut e, TILES[2], addr::TRAIT_VISIBLE), 0.0);
        assert_eq!(counts(&mut e).drawn, 0);
        // A player named but not in this game now changes nothing either.
        set_players(&mut e, [0xff00_0099]);
        update(&mut e);
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert_eq!(counts(&mut e).drawn, 0);
    }

    #[test]
    fn an_npc_on_a_tile_once_purple_gets_the_hud_colour_back() {
        let mut e = world([100.0, 100.0, 0.0]);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert!(purple(&mut e, TILES[1]));
        // The player leaves: the game ticks both actors again, its own colour on each tile.
        set_players(&mut e, []);
        update(&mut e);
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_SYSTEM_COLOR), 1.0);
    }

    #[test]
    fn a_player_behind_is_not_on_the_strip() {
        let mut e = world([0.0, -100.0, 0.0]);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert!(!purple(&mut e, TILES[1]));
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_VISIBLE), 0.0);
        let c = counts(&mut e);
        assert_eq!((c.drawn, c.behind), (0, 1));
    }

    #[test]
    fn a_player_right_beside_is_at_the_strip_end_and_hidden_as_an_npc_would_be() {
        // Exactly 90 compass degrees to the right: the game hides a tick there.
        let mut e = world([100.0, 0.0, 0.0]);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert!(!purple(&mut e, TILES[1]));
        assert_eq!(counts(&mut e).behind, 1);
    }

    #[test]
    fn a_player_far_off_is_shown_and_fades_at_the_strip_end() {
        // 80 compass degrees right, 10 000 units off: past the fade's start (0.8 of 600 = 480).
        let r = 10_000.0f32;
        let a = 80f32.to_radians();
        let mut e = world([r * a.sin(), r * a.cos(), 0.0]);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert!(purple(&mut e, TILES[1]));
        let x = get(&mut e, TILES[1], HEADING) - 2.0;
        let alpha = get(&mut e, TILES[1], addr::TRAIT_ALPHA);
        assert!(x > 480.0, "{x}");
        assert!((alpha - 255.0 * (600.0 - x) / 120.0).abs() < 0.5, "{alpha}");
    }

    #[test]
    fn with_every_tile_used_the_last_one_is_the_players() {
        let mut e = world([100.0, 100.0, 0.0]);
        // A third actor in the list, and the avatar's entry taken out: the game uses all three.
        actor(&mut e, 0x0300_3000, 0x0001_0002, [10.0, 50.0, 0.0]);
        actor(&mut e, 0x0300_4000, 0x0001_0003, [-10.0, 50.0, 0.0]);
        e.mem.set_u32(ITEMS, 0x0300_3000);
        e.mem.set_u32(NODES + 12, NODES + 16);
        e.mem.set_u32(NODES + 16, ITEMS + 16);
        e.mem.set_u32(ITEMS + 16, 0x0300_4000);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert_eq!(get(&mut e, TILES[0], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_SYSTEM_COLOR), 1.0);
        assert!(purple(&mut e, TILES[2]));
        assert_eq!(counts(&mut e).no_tile, 0);
    }

    #[test]
    fn before_the_game_named_its_heading_trait_nothing_is_drawn() {
        let mut e = world_with([100.0, 100.0, 0.0], |e, a| {
            game_pass(e, a);
            e.mem.set_u32(addr::COMPASS_HEADING_TRAIT, 0);
            Ret::default()
        });
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert_eq!(e.mem.u8(SEEN_SKIP), 1, "the wrap ran");
        assert!(!purple(&mut e, TILES[1]));
        assert_eq!(e.mem.u8(ITEMS + 5), 0);
    }

    #[test]
    fn an_entry_the_game_keeps_off_stays_off() {
        let mut e = world([100.0, 100.0, 0.0]);
        e.mem.set_u8(ITEMS + 5, 1);
        set_players(&mut e, [AVATAR_ID]);
        update(&mut e);
        assert_eq!(e.mem.u8(ITEMS + 5), 1);
        // The player is still shown, in purple.
        assert!(purple(&mut e, TILES[1]));
    }

    // voiceui
    #[test]
    fn a_speaking_player_is_ticked_in_the_speaking_colour_at_full_alpha() {
        // 80 compass degrees right and far: an unspoken tick would fade there.
        let r = 10_000.0f32;
        let a = 80f32.to_radians();
        let mut e = world([r * a.sin(), r * a.cos(), 0.0]);
        crate::voicemark::install(&mut e);
        set_players(&mut e, [AVATAR_ID]);
        crate::voicemark::set_speaking(&mut e, [AVATAR_ID]);
        update(&mut e);
        let s = crate::voicemark::SPEAKING;
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_RED), f32::from(s[0]));
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_GREEN), f32::from(s[1]));
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_BLUE), f32::from(s[2]));
        assert_eq!(get(&mut e, TILES[1], addr::TRAIT_ALPHA), 255.0);
        // Silent again: purple, and faded as before.
        crate::voicemark::set_speaking(&mut e, []);
        update(&mut e);
        assert!(purple(&mut e, TILES[1]));
        assert!(get(&mut e, TILES[1], addr::TRAIT_ALPHA) < 255.0);
    }
}
