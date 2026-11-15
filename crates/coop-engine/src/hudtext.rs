//! hudtext: co-op's own text on the HUD (docs/coop/SPECS.md, voice list): the live list of the
//! players speaking now, and a label over each speaking player's head.
//!
//! The tiles are the HUD's own kind: built by the game from the HUD's text template
//! ([`addr::HUD_TEXT_TEMPLATE`], the template its action points label is built from) with
//! `Menu::RenderTemplate` under the HUD's root tile, the way the HUD builds its own, and placed,
//! filled and shown through the tile value setters the HUD uses (`x`, `y`, `string`, `visible`,
//! `alpha`). They are built once for each HUD (a new HUD, after a load, gets new ones) and only
//! when co-op first has something to show; with nothing to show they are hidden.
//!
//! A head label is placed where the game projects the player's head: the place the avatar looks
//! from (`008a2fa0`, its head node), [`HEAD_RISE`] units higher, through the scene camera's
//! `NiCamera::WorldPtToScreenPt`, scaled to the screen as `vatsmenu.rs` scales a V.A.T.S. point
//! (x times the screen width, y flipped times its height). A head behind the camera or outside the
//! view has its label hidden.

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

use engine_core::prelude::*;

use crate::{actors, addr};

/// Lines of the live list at most.
pub const MAX_LINES: usize = 6;
/// Head labels at most.
pub const MAX_HEADS: usize = 8;
/// How far over the head (game units) a label is put: a layout choice of co-op's, so that the
/// label clears the hair.
pub const HEAD_RISE: f32 = 20.0;
/// Where the list starts, as parts of the screen: a layout choice of co-op's (left, a third of
/// the way down, clear of the HUD's own corners and the compass at the bottom).
pub const LIST_AT: [f32; 2] = [0.03, 0.33];
/// A line's height when the tile does not say (pixels of the HUD's space).
const LINE_HEIGHT: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Group {
    List,
    Head,
}

#[derive(Debug, Default)]
struct Inner {
    /// The HUD the tiles belong to.
    hud: u32,
    tiles: BTreeMap<(Group, usize), u32>,
    /// The tiles shown now.
    shown: BTreeMap<(Group, usize), bool>,
    built: u64,
    failed: u64,
}

/// The tiles' state (a lock-free shared slot as the compass's; written between frames).
#[derive(Debug, Default)]
pub struct HudText {
    inner: Mutex<Inner>,
}

/// What [`counts`] answers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Tiles the game built, ever.
    pub built: u64,
    /// Builds the game answered with no tile.
    pub failed: u64,
}

/// Makes the slot before the game's threads start ([`crate::install`], through `voicemark`).
pub(crate) fn install(e: &mut Engine) {
    let _ = e.ext_free::<HudText>();
}

/// The tiles built and not built so far.
pub fn counts(e: &mut Engine) -> Counts {
    let i = e
        .ext_free::<HudText>()
        .inner
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    Counts {
        built: i.built,
        failed: i.failed,
    }
}

fn hud(e: &mut Engine) -> u32 {
    e.mem.u32(addr::HUD_SINGLETON)
}

/// The tile for `key`, built when this HUD has none yet.
fn tile(e: &mut Engine, key: (Group, usize)) -> Option<u32> {
    let menu = hud(e);
    if menu == 0 {
        return None;
    }
    {
        let mut i = e
            .ext_free::<HudText>()
            .inner
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if i.hud != menu {
            // A new HUD: the old one's tiles went with it.
            i.hud = menu;
            i.tiles.clear();
            i.shown.clear();
        }
        if let Some(t) = i.tiles.get(&key) {
            return Some(*t);
        }
    }
    if !e.is_registered(addr::MENU_RENDER_TEMPLATE) {
        return None;
    }
    let root = e.mem.u32(menu.wrapping_add(addr::MENU_ROOT_TILE_OFFSET));
    if root == 0 {
        return None;
    }
    let t = e
        .call(
            addr::MENU_RENDER_TEMPLATE,
            &args![menu, root, addr::HUD_TEXT_TEMPLATE, 0u32],
        )
        .u32();
    let mut i = e
        .ext_free::<HudText>()
        .inner
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if t == 0 {
        i.failed += 1;
        return None;
    }
    i.built += 1;
    i.tiles.insert(key, t);
    Some(t)
}

fn existing(e: &mut Engine, key: (Group, usize)) -> Option<u32> {
    let menu = hud(e);
    let i = e
        .ext_free::<HudText>()
        .inner
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    (i.hud == menu && menu != 0)
        .then(|| i.tiles.get(&key).copied())
        .flatten()
}

fn set_shown(e: &mut Engine, key: (Group, usize), tile: u32, on: bool) {
    let was = e
        .ext_free::<HudText>()
        .inner
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .shown
        .insert(key, on);
    if was != Some(on) {
        e.call(
            addr::TILE_SET_INT,
            &args![tile, addr::TRAIT_VISIBLE, u32::from(on)],
        );
    }
}

fn hide(e: &mut Engine, key: (Group, usize)) {
    if let Some(t) = existing(e, key) {
        set_shown(e, key, t, false);
    }
}

fn get_float(e: &mut Engine, tile: u32, which: u32) -> f32 {
    e.call(addr::TILE_GET_FLOAT, &args![tile, which]).f32()
}

fn set_float(e: &mut Engine, tile: u32, which: u32, v: f32) {
    e.call(addr::TILE_SET_FLOAT, &args![tile, which, v, 1u32]);
}

/// Sets the tile's text (the game copies it).
fn set_text(e: &mut Engine, tile: u32, text: &str) -> bool {
    let mut bytes: Vec<u8> = text
        .chars()
        .map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' })
        .filter(|b| *b != 0)
        .collect();
    bytes.truncate(addr::HUD_MESSAGE_MAX);
    let b = e
        .call(addr::OPERATOR_NEW, &args![bytes.len() as u32 + 1])
        .u32();
    if b == 0 {
        return false;
    }
    e.mem.set_cstr(b, &bytes);
    e.call(
        addr::TILE_SET_STRING,
        &args![tile, addr::TRAIT_STRING, b, 1u32],
    );
    e.call(addr::OPERATOR_DELETE, &args![b]);
    true
}

fn screen(e: &mut Engine) -> Option<[f32; 2]> {
    if !e.is_registered(addr::SCREEN_WIDTH) || !e.is_registered(addr::SCREEN_HEIGHT) {
        return None;
    }
    let w = e.call(addr::SCREEN_WIDTH, &[]).f64() as f32;
    let h = e.call(addr::SCREEN_HEIGHT, &[]).f64() as f32;
    (w > 0.0 && h > 0.0).then_some([w, h])
}

/// Shows `lines` as the live list (the first [`MAX_LINES`]); the list's other tiles are hidden.
/// Nothing is built while `lines` is empty.
pub fn show_list(e: &mut Engine, lines: &[String]) {
    let Some([w, h]) = screen(e) else {
        return;
    };
    let mut y = h * LIST_AT[1];
    for i in 0..MAX_LINES {
        let key = (Group::List, i);
        let Some(text) = lines.get(i) else {
            hide(e, key);
            continue;
        };
        let Some(t) = tile(e, key) else {
            return;
        };
        set_text(e, t, text);
        set_float(e, t, addr::TRAIT_X, w * LIST_AT[0]);
        set_float(e, t, addr::TRAIT_Y, y);
        set_float(e, t, addr::TRAIT_ALPHA, 255.0);
        set_shown(e, key, t, true);
        let line = get_float(e, t, addr::TRAIT_HEIGHT);
        y += if line > 1.0 { line } else { LINE_HEIGHT };
    }
}

/// Where the scene camera shows the world place `at`, in the HUD's space; `None` behind the
/// camera or outside the view.
pub fn project(e: &mut Engine, at: [f32; 3]) -> Option<[f32; 2]> {
    if !e.is_registered(addr::WORLD_PT_TO_SCREEN_PT) || !e.is_registered(addr::CAMERA_HOLDER) {
        return None;
    }
    let [w, h] = screen(e)?;
    let holder = e.call(addr::CAMERA_HOLDER, &[]).u32();
    if holder == 0 {
        return None;
    }
    let camera = e.call(addr::CAMERA_OF, &args![holder]).u32();
    if camera == 0 {
        return None;
    }
    let epsilon = e.mem.u32(addr::SCREEN_EPSILON);
    e.with_stack(0x14, |e, local| {
        let p = local.addr();
        for (k, v) in at.iter().enumerate() {
            e.mem.set_f32(p + 4 * k as u32, *v);
        }
        let (x, y) = (p + 0xc, p + 0x10);
        e.mem.set_f32(x, 0.0);
        e.mem.set_f32(y, 0.0);
        let inside = e
            .call(
                addr::WORLD_PT_TO_SCREEN_PT,
                &args![camera, p, x, y, epsilon],
            )
            .u32()
            & 0xff
            != 0;
        let (sx, sy) = (e.mem.f32(x), e.mem.f32(y));
        (inside && sx.is_finite() && sy.is_finite()).then_some([w * sx, h * (1.0 - sy)])
    })
}

/// The place actor `r` (a pointer) looks from, as the game reckons it.
fn eye(e: &mut Engine, r: u32) -> Option<[f32; 3]> {
    if !e.is_registered(addr::ACTOR_EYE_POSITION) {
        return None;
    }
    e.with_stack(0xc, |e, out| {
        e.call(addr::ACTOR_EYE_POSITION, &args![r, out.addr()]);
        let p = [
            e.mem.f32(out.addr()),
            e.mem.f32(out.addr() + 4),
            e.mem.f32(out.addr() + 8),
        ];
        p.iter().all(|v| v.is_finite()).then_some(p)
    })
}

/// Puts each label over the head of its avatar (`(avatar form id, text)`, the first
/// [`MAX_HEADS`]); the other head labels are hidden, as is one whose head is not in view.
pub fn show_heads(e: &mut Engine, marks: &[(u32, String)]) {
    // No HUD (or no screen): nothing to place, nothing looked up.
    if hud(e) == 0 || screen(e).is_none() {
        return;
    }
    for i in 0..MAX_HEADS {
        let key = (Group::Head, i);
        let Some((id, text)) = marks.get(i) else {
            hide(e, key);
            continue;
        };
        let at = actors::actor(e, *id)
            .and_then(|r| eye(e, r))
            .and_then(|p| project(e, [p[0], p[1], p[2] + HEAD_RISE]));
        let Some([x, y]) = at else {
            hide(e, key);
            continue;
        };
        let Some(t) = tile(e, key) else {
            return;
        };
        set_text(e, t, text);
        let width = get_float(e, t, addr::TRAIT_WIDTH).max(0.0);
        let height = get_float(e, t, addr::TRAIT_HEIGHT).max(0.0);
        set_float(e, t, addr::TRAIT_X, x - width / 2.0);
        set_float(e, t, addr::TRAIT_Y, y - height);
        set_float(e, t, addr::TRAIT_ALPHA, 255.0);
        set_shown(e, key, t, true);
    }
}

/// Hides every tile co-op put on the HUD (the session's end).
pub fn hide_all(e: &mut Engine) {
    show_list(e, &[]);
    show_heads(e, &[]);
}

#[cfg(test)]
mod tests {
    use super::*;

    const HUD: u32 = 0x0600_0000;
    const ROOT: u32 = 0x0600_1000;
    const TILES: u32 = 0x0610_0000;
    const HEAP: u32 = 0x0620_0000;
    const AVATAR: u32 = 0x0630_0000;
    const AVATAR_ID: u32 = 0xff00_0001;
    const HOLDER: u32 = 0x0640_0000;
    const CAMERA: u32 = 0x0640_1000;

    type Traits = BTreeMap<(u32, u32), f32>;
    type Texts = BTreeMap<u32, Vec<u8>>;

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
        e.ext::<Traits>()
            .get(&(tile, which))
            .copied()
            .unwrap_or(0.0)
    }

    fn text(e: &mut Engine, tile: u32) -> String {
        String::from_utf8_lossy(e.ext::<Texts>().get(&tile).map_or(&[][..], |v| v)).into_owned()
    }

    /// The game's side, cut down: a 1000 by 500 screen, a HUD whose template builds a tile at a
    /// time, tiles that keep their traits (a text 10 wide per character and 20 high), and a
    /// camera that sees x from -100 to 100 at y 100 ahead, z from -50 to 50.
    fn game() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x011d_0000, 0x2_0000);
        e.mem.map(addr::SCREEN_EPSILON & !0xfff, 0x1000);
        e.mem.map(HUD, 0x2000);
        e.mem.map(TILES, 0x1_0000);
        e.mem.map(HEAP, 0x1_0000);
        e.mem.map(AVATAR, 0x1000);
        e.mem.map(HOLDER, 0x2000);
        e.mem.set_u32(HEAP, HEAP + 0x10);
        e.mem.set_u32(addr::HUD_SINGLETON, HUD);
        e.mem.set_u32(HUD + addr::MENU_ROOT_TILE_OFFSET, ROOT);
        e.register(addr::OPERATOR_NEW, |e, a| {
            let at = e.mem.u32(HEAP);
            e.mem.set_u32(HEAP, at + ((a[0] + 15) & !15));
            eax(at)
        });
        e.register(addr::OPERATOR_DELETE, |e, a| {
            e.mem.set_u8(a[0], 0);
            Ret::default()
        });
        e.register(addr::MENU_RENDER_TEMPLATE, |e, a| {
            assert_eq!((a[0], a[1], a[2]), (HUD, ROOT, addr::HUD_TEXT_TEMPLATE));
            let made = e.mem.u32(TILES) + 1;
            e.mem.set_u32(TILES, made);
            eax(TILES + 0x100 * made)
        });
        e.register(addr::TILE_SET_FLOAT, |e, a| {
            e.ext::<Traits>().insert((a[0], a[1]), f32::from_bits(a[2]));
            Ret::default()
        });
        e.register(addr::TILE_SET_INT, |e, a| {
            e.ext::<Traits>().insert((a[0], a[1]), a[2] as i32 as f32);
            Ret::default()
        });
        e.register(addr::TILE_GET_FLOAT, |e, a| {
            st0(f64::from(get(e, a[0], a[1])))
        });
        e.register(addr::TILE_SET_STRING, |e, a| {
            assert_eq!(a[1], addr::TRAIT_STRING);
            let t = e.mem.cstr(a[2]);
            e.ext::<Traits>()
                .insert((a[0], addr::TRAIT_WIDTH), 10.0 * t.len() as f32);
            e.ext::<Traits>().insert((a[0], addr::TRAIT_HEIGHT), 20.0);
            e.ext::<Texts>().insert(a[0], t);
            Ret::default()
        });
        e.register(addr::SCREEN_WIDTH, |_, _| st0(1000.0));
        e.register(addr::SCREEN_HEIGHT, |_, _| st0(500.0));
        e.register(addr::CAMERA_HOLDER, |_, _| eax(HOLDER));
        e.register(addr::CAMERA_OF, |_, a| {
            assert_eq!(a[0], HOLDER);
            eax(CAMERA)
        });
        e.register(addr::WORLD_PT_TO_SCREEN_PT, |e, a| {
            assert_eq!(a[0], CAMERA);
            let p = |k: u32| e.mem.f32(a[1] + 4 * k);
            let (x, y, z) = (p(0), p(1), p(2));
            if y <= 0.0 {
                return eax(0);
            }
            let (sx, sy) = ((x / y + 1.0) / 2.0, (z / y + 1.0) / 2.0);
            e.mem.set_f32(a[2], sx);
            e.mem.set_f32(a[3], sy);
            eax(u32::from(
                (0.0..=1.0).contains(&sx) && (0.0..=1.0).contains(&sy),
            ))
        });
        e.register(addr::LOOKUP_FORM, |_, a| {
            eax(if a[0] == AVATAR_ID { AVATAR } else { 0 })
        });
        e.mem.set_u8(AVATAR + addr::FORM_TYPE_OFFSET, 0x3c);
        e.mem.set_u32(AVATAR + addr::FORM_ID_OFFSET, AVATAR_ID);
        // The avatar's eyes: kept at +0x100 of it by the test.
        e.register(addr::ACTOR_EYE_POSITION, |e, a| {
            for k in 0..3 {
                let v = e.mem.u32(a[0] + 0x100 + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, v);
            }
            eax(a[1])
        });
        install(&mut e);
        e
    }

    fn eyes_at(e: &mut Engine, p: [f32; 3]) {
        for (k, v) in p.into_iter().enumerate() {
            e.mem.set_f32(AVATAR + 0x100 + 4 * k as u32, v);
        }
    }

    fn tile_n(n: u32) -> u32 {
        TILES + 0x100 * n
    }

    #[test]
    fn the_live_list_is_the_hud_s_own_text_tiles_shown_and_hidden() {
        let mut e = game();
        show_list(&mut e, &[]);
        assert_eq!(
            counts(&mut e).built,
            0,
            "nothing built with nothing to show"
        );
        show_list(&mut e, &["Ann [||||......]".into(), "Bo".into()]);
        assert_eq!(counts(&mut e).built, 2);
        let (a, b) = (tile_n(1), tile_n(2));
        assert_eq!(text(&mut e, a), "Ann [||||......]");
        assert_eq!(get(&mut e, a, addr::TRAIT_X), 30.0);
        assert_eq!(get(&mut e, a, addr::TRAIT_Y), 500.0 * 0.33);
        assert_eq!(
            get(&mut e, b, addr::TRAIT_Y),
            500.0 * 0.33 + 20.0,
            "a line lower"
        );
        assert_eq!(get(&mut e, a, addr::TRAIT_VISIBLE), 1.0);
        // Next frame Bo stopped: his line is hidden, Ann's kept; nothing new built.
        show_list(&mut e, &["Ann".into()]);
        assert_eq!(get(&mut e, b, addr::TRAIT_VISIBLE), 0.0);
        assert_eq!(get(&mut e, a, addr::TRAIT_VISIBLE), 1.0);
        assert_eq!(counts(&mut e).built, 2);
        hide_all(&mut e);
        assert_eq!(get(&mut e, a, addr::TRAIT_VISIBLE), 0.0);
        // A new HUD (a load): new tiles.
        e.mem.set_u32(addr::HUD_SINGLETON, HUD + 0x1000);
        e.mem
            .set_u32(HUD + 0x1000 + addr::MENU_ROOT_TILE_OFFSET, ROOT);
        e.register(addr::MENU_RENDER_TEMPLATE, |e, _| {
            let made = e.mem.u32(TILES) + 1;
            e.mem.set_u32(TILES, made);
            eax(TILES + 0x100 * made)
        });
        show_list(&mut e, &["Cy".into()]);
        assert_eq!(counts(&mut e).built, 3);
        assert_eq!(text(&mut e, tile_n(3)), "Cy");
    }

    #[test]
    fn a_label_sits_over_the_head_where_the_camera_shows_it() {
        let mut e = game();
        // Straight ahead, eyes 30 under the camera's middle: z + 20 = -10 at y 100.
        eyes_at(&mut e, [0.0, 100.0, -30.0]);
        show_heads(&mut e, &[(AVATAR_ID, "Bo speaking".into())]);
        let t = tile_n(1);
        assert_eq!(text(&mut e, t), "Bo speaking");
        // x: the middle (500) less half the text (55); y: (1 - 0.45) of 500, less its height.
        assert_eq!(get(&mut e, t, addr::TRAIT_X), 445.0);
        assert!((get(&mut e, t, addr::TRAIT_Y) - (275.0 - 20.0)).abs() < 0.01);
        assert_eq!(get(&mut e, t, addr::TRAIT_VISIBLE), 1.0);
        // Behind the camera: hidden.
        eyes_at(&mut e, [0.0, -100.0, 0.0]);
        show_heads(&mut e, &[(AVATAR_ID, "Bo speaking".into())]);
        assert_eq!(get(&mut e, t, addr::TRAIT_VISIBLE), 0.0);
        // Out of the view to the side: hidden.
        eyes_at(&mut e, [500.0, 100.0, 0.0]);
        show_heads(&mut e, &[(AVATAR_ID, "Bo speaking".into())]);
        assert_eq!(get(&mut e, t, addr::TRAIT_VISIBLE), 0.0);
        // An avatar this game does not have: hidden, nothing built.
        show_heads(&mut e, &[(0xff00_0099, "Cy".into())]);
        assert_eq!(counts(&mut e).built, 1);
        assert_eq!(project(&mut e, [0.0, 100.0, 0.0]), Some([500.0, 250.0]));
    }

    #[test]
    fn without_the_hud_or_its_functions_nothing_is_drawn() {
        let mut e = Engine::new();
        e.mem.map(0x011d_0000, 0x2_0000);
        install(&mut e);
        show_list(&mut e, &["Ann".into()]);
        show_heads(&mut e, &[(1, "x".into())]);
        assert_eq!(counts(&mut e), Counts::default());
        let mut e = game();
        e.mem.set_u32(addr::HUD_SINGLETON, 0);
        show_list(&mut e, &["Ann".into()]);
        assert_eq!(counts(&mut e).built, 0);
        // A template that builds nothing is counted, and nothing is set.
        let mut e = game();
        e.register(addr::MENU_RENDER_TEMPLATE, |_, _| eax(0));
        show_list(&mut e, &["Ann".into()]);
        assert_eq!(counts(&mut e).failed, 1);
    }
}
