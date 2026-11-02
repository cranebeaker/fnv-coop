//! The rendered terminal: how the PC game shows a terminal's screen and
//! the hacking game by default (`[RenderedTerminal] bUseRenderedTerminals`,
//! 1 in the exe's settings table, `011db620`, and in the shipped INI). The
//! game's `FORenderedTerminal` (vtable `0106ebe4`, a `FORenderedMenu`,
//! `0107841c`, as the Pip-Boy's screen is) is made when the terminal's or
//! the hacking menu opens (`00706130`, `00705ec0`; the hacking menu's
//! hand-over to the terminal's keeps it, `0076a540`).
//!
//! Its scene (`007feeb0`, read with the Xbox prototype's
//! `FORenderedTerminal::Initialize`):
//!
//! - the model `meshes\terminals\TerminalInterface01.NIF` (loaded once and
//!   kept, `011db63c`): its node "screen" holds the screen the menu is
//!   drawn on, "PowerButton" the button that switches it off;
//! - the model's top node turned Z(1.57) · Y(1.57) (`004a0c90`,
//!   `0043f850`, `010559cc`: Gamebryo's matrices, so model +y runs away
//!   from the camera, +z up, +x right) and moved to (zoom, `VPos`, `HPos`)
//!   (`004bc1f0`), zoom `fRenderedTerminalZoom` × 0.75 when the screen is
//!   4:3 (`0101de30`; `011c70eb`, set by `004dc360`, is "height / width
//!   isn't 0.75");
//! - a new camera at the origin with no turn (looking along +x, +y up, +z
//!   right), the lockpicking menu's frustum with `fRenderedTerminalFOV` as
//!   its share of `fDefaultFOV` ([`crate::lockpick::MenuCamera`]);
//! - the model's point lights given to the menus' scene with three times
//!   the model's bounding radius as their reach (`00b5ca70`, `01021928`);
//!   only a model without lights gets the code's own "Omni01" with the
//!   `fScreenLight*` settings (`007ff960`), and this model has three.
//!
//! The menu is drawn into a picture of 1280 × 960 menu units
//! (`FORenderedMenu::Draw`, `007fba00`: `0106ec38`, `0106f2dc`), which the
//! screen effect turns into the screen's texture (`007fbee0`, the Pip-Boy's
//! `ISIFSCANBLEND`: blur `fDefaultBlurRadius` / `fDefaultBlurIntensity`
//! (`007fc150`), scanlines × `fRenderedTerminalScanlineScale`, the terminal
//! colour as its tint (system colour 3), the pulse and the passing band).
//! The terminal menu's files fill the top-left 960 × 720 of it
//! (`computers_menu.xml`'s depth rect: the screen's height × 0.75 and that
//! × 4 / 3), which is what the screen's texture coordinates cover.
//!
//! The pointer is where the ray through it meets the screen
//! (`007fb790`): its texture coordinates × the menu height × 1.333333 and
//! × the menu height. A click that meets the power button leaves
//! (`007ffba0` → `007ffd50`).
//!
//! The model fades in when the menu opens and out when the player leaves
//! (`007ff650`, `007ffaf0`: its `BSFadeNode` range set to 100000 and 0),
//! by the fade node's steps (`BSFadeNode::OnVisible`): at most
//! `maxFadeIncrement` (0.1) a frame, the frame's time over `fFadeInTime` or
//! `fFadeOutTime` (`[LOD]`; `0044fb20` copies them to `011ad7e4`,
//! `011ad7e8`). When the fade out stops, the rendered menu ends
//! (`007ff820`).

use std::sync::Arc;

use nif::math::{Transform, Vec3};

use crate::lockpick::{bound, merge, MenuCamera};
use crate::{Game, LightData, ViewerScene};

/// The model (`011a0bb0`).
pub const MODEL: &str = "meshes\\terminals\\terminalinterface01.nif";
/// The node holding the screen and the one holding the power button
/// (`01078bc0`, `01078bb4`).
pub const SCREEN_NODE: &str = "screen";
pub const EXIT_NODE: &str = "PowerButton";
/// The top node's turn about z and then y (`010559cc`).
pub const TURN: f32 = 1.57;
/// The zoom's factor on a 4:3 screen (`0101de30`).
pub const FOUR_THREE_ZOOM: f32 = 0.75;
/// The lights' reach: this × the model's bounding radius.
pub const LIGHT_RADIUS_MULT: f32 = 3.0;
/// The menus' picture in menu units (`0106ec38`, `0106f2dc`).
pub const PICTURE: (f32, f32) = (1280.0, 960.0);
/// The menu height the pointer is scaled by, and the factor across
/// (`007fb790`).
pub const MENU_HEIGHT: f32 = 960.0;
pub const POINTER_ACROSS: f32 = 1.333_333;
/// The fade node's largest step a frame (`011ad7f8`, Xbox
/// `BSFadeNode::maxFadeIncrement`).
pub const MAX_FADE_STEP: f32 = 0.1;

/// The settings the rendered terminal reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `bUseRenderedTerminals`: draw terminals this way at all.
    pub on: bool,
    /// `fRenderedTerminalFOV`: the share of `fDefaultFOV`.
    pub fov_share: f32,
    /// `fRenderedTerminalZoom`, `fRenderedTerminalHPos`,
    /// `fRenderedTerminalVPos`.
    pub zoom: f32,
    pub h_pos: f32,
    pub v_pos: f32,
    /// `fRenderedTerminalScanlineScale`.
    pub scanline_scale: f32,
    /// `[LOD] fFadeInTime`, `fFadeOutTime` (seconds).
    pub fade_in: f32,
    pub fade_out: f32,
}

impl Default for Settings {
    /// The exe's defaults (`011db620`, `011db5ec`, `011db65c`, `011db62c`,
    /// `011db614`, `011db674`, `011c3df4`, `011c3c5c`).
    fn default() -> Settings {
        Settings {
            on: true,
            fov_share: 0.15,
            zoom: 20.0,
            h_pos: 0.0,
            v_pos: 0.38,
            scanline_scale: 130.0,
            fade_in: 1.2,
            fade_out: 1.2,
        }
    }
}

impl Settings {
    /// The exe's defaults with the INI's values over them. (The shipped
    /// INI's `fFadeInTimet=2.0` is misspelt, so the fade in keeps 1.2.)
    pub fn from_ini(ini: &assets::IniSettings) -> Settings {
        let mut s = Settings::default();
        let section = "RenderedTerminal";
        if let Some(v) = ini.get(section, "bUseRenderedTerminals") {
            s.on = v.trim().parse::<f32>().map(|v| v != 0.0).unwrap_or(true);
        }
        let set = |section: &str, key: &str, value: &mut f32| {
            if let Some(v) = ini.float(section, key) {
                *value = v;
            }
        };
        set(section, "fRenderedTerminalFOV", &mut s.fov_share);
        set(section, "fRenderedTerminalZoom", &mut s.zoom);
        set(section, "fRenderedTerminalHPos", &mut s.h_pos);
        set(section, "fRenderedTerminalVPos", &mut s.v_pos);
        set(
            section,
            "fRenderedTerminalScanlineScale",
            &mut s.scanline_scale,
        );
        set("LOD", "fFadeInTime", &mut s.fade_in);
        set("LOD", "fFadeOutTime", &mut s.fade_out);
        s
    }
}

/// Whether a screen of this size counts as wide (`004dc360`: its height
/// over its width, in doubles, isn't 0.75).
pub fn widescreen(width: u32, height: u32) -> bool {
    f64::from(height) / f64::from(width.max(1)) != 0.75
}

/// The top node's turn: Z(1.57) · Y(1.57) as Gamebryo's matrices are laid
/// out (rows; `004a0c90`, `0043f850`).
pub fn root_rotation() -> [[f32; 3]; 3] {
    let (s, c) = TURN.sin_cos();
    let z = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
    let y = [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]];
    nif::math::mat_mul(&z, &y)
}

/// The top node's place in front of the camera for a screen of `width` ×
/// `height` pixels.
pub fn root_transform(settings: &Settings, width: u32, height: u32) -> Transform {
    let zoom = if widescreen(width, height) {
        settings.zoom
    } else {
        settings.zoom * FOUR_THREE_ZOOM
    };
    Transform {
        rotation: root_rotation(),
        translation: [zoom, settings.v_pos, settings.h_pos],
        scale: 1.0,
    }
}

/// The terminal's camera for a screen of `width` × `height` pixels.
pub fn camera(
    ini: &assets::IniSettings,
    settings: &Settings,
    width: u32,
    height: u32,
) -> MenuCamera {
    MenuCamera::with_share(ini, width, height, settings.fov_share)
}

/// A point on the screen's picture, in menu units, for the screen's
/// texture coordinates (`007fb790`).
pub fn menu_point(uv: [f32; 2]) -> (f32, f32) {
    (uv[0] * MENU_HEIGHT * POINTER_ACROSS, uv[1] * MENU_HEIGHT)
}

/// A triangle of the screen with its texture coordinates, in the model's
/// space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub corners: [Vec3; 3],
    pub uvs: [[f32; 2]; 3],
}

/// What the pointer is over.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hit {
    /// The screen, at these texture coordinates.
    Screen([f32; 2]),
    /// The power button.
    Exit,
    Nothing,
}

/// The model's pieces and lights, ready to draw.
pub struct TerminalScene {
    /// The model's pieces in its own space (the draws carry reference 1).
    pub scene: ViewerScene,
    /// The point lights in the model's space (colour the diffuse colour,
    /// fade the dimmer, reach three times the bounding radius).
    pub lights: Vec<LightData>,
    pub bound_radius: f32,
    /// The screen's shape (its name; the piece whose texture is the menu).
    pub screen_shape: Option<String>,
    pub pick: Pick,
}

/// The screen's and the power button's triangles, for the pointer.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pick {
    pub screen: Vec<Triangle>,
    pub exit: Vec<[Vec3; 3]>,
}

impl Game {
    /// The rendered terminal's model ([`terminal_scene`]), read once.
    pub fn terminal_scene(&self) -> Option<Arc<TerminalScene>> {
        terminal_scene(&self.assets).map(Arc::new)
    }
}

/// The names of a node's shapes and the shapes under it, depth first.
fn shapes_under(nif: &nif::Nif, node: &str) -> Vec<String> {
    fn walk(nif: &nif::Nif, index: i32, inside: bool, node: &str, out: &mut Vec<String>) {
        let Ok(i) = usize::try_from(index) else {
            return;
        };
        match nif.block(i) {
            Ok(nif::Block::Node(n)) => {
                let inside = inside || n.av.net.name.eq_ignore_ascii_case(node);
                for &c in &n.children {
                    walk(nif, c, inside, node, out);
                }
            }
            Ok(nif::Block::Geometry(g)) if inside => out.push(g.av.net.name.clone()),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for &r in nif.roots() {
        walk(nif, r, false, node, &mut out);
    }
    out
}

/// The model, its lights and its screen (`007feeb0`); `None` when it can't
/// be read.
pub fn terminal_scene(assets: &assets::Assets) -> Option<TerminalScene> {
    let nif = nif::Nif::parse(assets.read(MODEL).ok()??).ok()?;
    // The screen is the "screen" node's first shape (`007feeb0`: the node,
    // or its first child when the node isn't a shape).
    let screen_shape = shapes_under(&nif, SCREEN_NODE).into_iter().next();
    let exit_shapes = shapes_under(&nif, EXIT_NODE);
    // The top node's transform is the code's (as for a placed model).
    let shapes = nif.placed_scene().ok()?;
    let mut screen = Vec::new();
    let mut exit = Vec::new();
    for m in &shapes.meshes {
        let is_screen = screen_shape.as_deref() == Some(m.name.as_str());
        let is_exit = exit_shapes.iter().any(|n| n == &m.name);
        if !is_screen && !is_exit {
            continue;
        }
        let points: Vec<Vec3> = m.model_positions().collect();
        for t in &m.triangles {
            let [a, b, c] = t.map(usize::from);
            let (Some(&pa), Some(&pb), Some(&pc)) = (points.get(a), points.get(b), points.get(c))
            else {
                continue;
            };
            if is_screen {
                let uv = |i: usize| m.uvs.get(i).copied().unwrap_or([0.0; 2]);
                screen.push(Triangle {
                    corners: [pa, pb, pc],
                    uvs: [uv(a), uv(b), uv(c)],
                });
            } else {
                exit.push([pa, pb, pc]);
            }
        }
    }
    let bound_radius = nif
        .roots()
        .iter()
        .fold(None, |acc, &r| {
            merge(
                acc,
                bound(&nif, r, &Transform::IDENTITY, Some(&Transform::IDENTITY)),
            )
        })
        .map_or(0.0, |(_, r)| r);
    let lights = nif
        .lights()
        .ok()?
        .into_iter()
        .filter(|l| l.kind == nif::LightKind::Point && l.on)
        .map(|l| LightData {
            position: l.position(),
            color: l.diffuse,
            radius: LIGHT_RADIUS_MULT * bound_radius,
            fade: l.dimmer,
        })
        .collect();

    let mut cell = world::LoadedCell::actors_only(Vec::new());
    cell.objects.push(world::Placement {
        form_id: esm::FormId(1),
        record_type: esm::FourCC::new(b"REFR"),
        editor_id: None,
        base: esm::FormId(0),
        base_type: esm::FourCC::new(b"STAT"),
        base_editor_id: None,
        position: [0.0; 3],
        rotation: [0.0; 3],
        scale: 1.0,
        model: Some(MODEL.to_string()),
        parts: Vec::new(),
        light: None,
        radius: None,
        teleport: None,
        emittance: None,
        flags: 0,
        enable_parent: None,
        plugin: String::new(),
        actor: None,
        primitive: None,
        open_by_default: false,
    });
    let built = preview::cell::build_scene_with(assets, cell, false);
    let mut scene = crate::convert_cell(&built, assets);
    // The power button's textures for the keyboard and mouse
    // (`004b7660`, right after the model is read).
    for t in &mut scene.textures {
        let Some(path) = platform_path(&t.path, PLATFORM_PC) else {
            continue;
        };
        let Some(bytes) = assets.read(&path).ok().flatten() else {
            continue;
        };
        if let Ok(mut swapped) = crate::TextureData::from_dds(path, bytes) {
            swapped.linear = t.linear;
            *t = swapped;
        }
    }
    Some(TerminalScene {
        scene,
        lights,
        bound_radius,
        screen_shape,
        pick: Pick { screen, exit },
    })
}

/// The platforms' texture folders (`01189038`), and the one used with the
/// keyboard and mouse (`004b71a0`: the XBOX one only while a pad is in
/// use).
pub const PLATFORMS: [&str; 4] = ["PC", "XBOX", "PS3", "PS3O"];
pub const PLATFORM_PC: usize = 0;

/// A texture path with its platform folder swapped for `platform`'s
/// (`SwapPlatformLanguageTextures`, `004b7660` → `004b7240`: the first of
/// the folders found as a whole path part); `None` when it has none or
/// already names it.
pub fn platform_path(path: &str, platform: usize) -> Option<String> {
    let parts: Vec<&str> = path.split(['\\', '/']).collect();
    let i = parts
        .iter()
        .position(|p| PLATFORMS.iter().any(|f| p.eq_ignore_ascii_case(f)))?;
    let want = PLATFORMS.get(platform)?;
    if parts[i].eq_ignore_ascii_case(want) {
        return None;
    }
    let mut out: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
    out[i] = want.to_ascii_lowercase();
    Some(out.join("\\"))
}

/// Lights in the model's space placed with its top node at `root`, in the
/// camera's space.
pub fn lights_at(lights: &[LightData], root: &Transform) -> Vec<LightData> {
    lights
        .iter()
        .map(|l| LightData {
            position: root.apply_point(l.position),
            ..*l
        })
        .collect()
}

impl Pick {
    /// What the ray through a point of the picture meets (`ndc`: −1 at
    /// the left and bottom, 1 at the right and top), with the model's top
    /// node at `root`; with `click`, the power button first (`007ffba0`).
    pub fn pick(&self, root: &Transform, camera: &MenuCamera, ndc: (f32, f32), click: bool) -> Hit {
        let ray = [
            1.0,
            ndc.1 * camera.tan_half_height,
            ndc.0 * camera.tan_half_width,
        ];
        let placed = |p: Vec3| root.apply_point(p);
        if click
            && self
                .exit
                .iter()
                .any(|t| ray_hits([0.0; 3], ray, t.map(placed)).is_some())
        {
            return Hit::Exit;
        }
        let mut best: Option<(f32, [f32; 2])> = None;
        for t in &self.screen {
            if let Some((d, b)) = ray_hits([0.0; 3], ray, t.corners.map(placed)) {
                if best.map_or(true, |(bd, _)| d < bd) {
                    let uv = [
                        t.uvs[0][0] * b[0] + t.uvs[1][0] * b[1] + t.uvs[2][0] * b[2],
                        t.uvs[0][1] * b[0] + t.uvs[1][1] * b[1] + t.uvs[2][1] * b[2],
                    ];
                    best = Some((d, uv));
                }
            }
        }
        best.map_or(Hit::Nothing, |(_, uv)| Hit::Screen(uv))
    }
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Where a ray meets a triangle (either side): its distance along the ray
/// and the point's weights on the three corners.
pub fn ray_hits(from: Vec3, along: Vec3, t: [Vec3; 3]) -> Option<(f32, [f32; 3])> {
    let e1 = sub(t[1], t[0]);
    let e2 = sub(t[2], t[0]);
    let p = cross(along, e2);
    let det = dot(e1, p);
    if det.abs() < 1e-12 {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(from, t[0]);
    let u = dot(s, p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross(s, e1);
    let v = dot(along, q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let d = dot(e2, q) * inv;
    (d > 0.0).then_some((d, [1.0 - u - v, u, v]))
}

/// The model's fade (`BSFadeNode::OnVisible` as the rendered terminal sets
/// it up: in from 0 when the menu opens, out when the player leaves).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fade {
    pub alpha: f32,
    /// Fading out (`FORenderedTerminal::FadeOut`, `007ffaf0`).
    pub out: bool,
}

impl Fade {
    /// Opening (`007ff650`): from nothing.
    pub fn opening() -> Fade {
        Fade {
            alpha: 0.0,
            out: false,
        }
    }

    /// One frame of `dt` seconds; true once a fade out has ended (the
    /// fade no longer changes, `007ff820`).
    pub fn step(&mut self, settings: &Settings, dt: f32) -> bool {
        let before = self.alpha;
        if self.out {
            let step = (dt / settings.fade_out.max(1e-6)).min(MAX_FADE_STEP);
            self.alpha = (self.alpha - step).max(0.0);
        } else {
            let step = (dt / settings.fade_in.max(1e-6)).min(MAX_FADE_STEP);
            self.alpha = (self.alpha + step).min(1.0);
        }
        self.out && self.alpha == before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3)
    }

    #[test]
    fn the_model_faces_the_camera() {
        // Model +y runs away from the camera (+x), +z is up (+y), +x is
        // right (+z): 1.57 is a hair short of a right angle.
        let r = root_rotation();
        let apply = |v: Vec3| nif::math::mat_vec(&r, v);
        assert!(close(apply([0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]));
        assert!(close(apply([0.0, 0.0, 1.0]), [0.0, 1.0, 0.0]));
        assert!(close(apply([1.0, 0.0, 0.0]), [0.0, 0.0, 1.0]));
    }

    fn ini(text: &str) -> assets::IniSettings {
        let mut s = assets::IniSettings::default();
        s.add(text);
        s
    }

    #[test]
    fn the_settings_are_the_ini_over_the_exe() {
        assert_eq!(Settings::from_ini(&ini("")), Settings::default());
        let s = Settings::from_ini(&ini(
            "[RenderedTerminal]\nbUseRenderedTerminals=1\nfRenderedTerminalFOV=0.15\n\
             fRenderedTerminalZoom=36\nfRenderedTerminalScanlineScale=130.000000\n\
             fRenderedTerminalHPos=0.0\nfRenderedTerminalVPos=0.38\n\
             [LOD]\nfFadeInTimet=2.0\nfFadeOutTime=2.0\n",
        ));
        assert!(s.on);
        assert_eq!((s.zoom, s.v_pos, s.scanline_scale), (36.0, 0.38, 130.0));
        // The misspelt fade in is ignored, as the game ignores it.
        assert_eq!((s.fade_in, s.fade_out), (1.2, 2.0));
        let off = Settings::from_ini(&ini("[RenderedTerminal]\nbUseRenderedTerminals=0\n"));
        assert!(!off.on);
    }

    #[test]
    fn a_square_screen_brings_the_model_nearer() {
        let s = Settings {
            zoom: 36.0,
            ..Settings::default()
        };
        assert!(widescreen(1920, 1080));
        assert!(!widescreen(1024, 768));
        assert!(widescreen(1280, 1024));
        assert_eq!(
            root_transform(&s, 1920, 1080).translation,
            [36.0, 0.38, 0.0]
        );
        assert_eq!(
            root_transform(&s, 1600, 1200).translation,
            [27.0, 0.38, 0.0]
        );
    }

    #[test]
    fn the_camera_takes_the_terminal_share() {
        let i = ini("[Display]\nfDefaultFOV=75\n");
        let s = Settings {
            fov_share: 0.3,
            ..Settings::default()
        };
        let c = camera(&i, &s, 1920, 1080);
        let expected = (75.0f32 * 0.3).to_radians().tan() * 0.75;
        assert!((c.tan_half_width - expected).abs() < 1e-5);
        assert!((c.tan_half_height - expected * 0.5625).abs() < 1e-5);
    }

    /// A screen two units square facing the camera at x = 10 (model space
    /// with the top node at the identity turn), texture coordinates 0..1
    /// from its top left.
    fn square() -> Pick {
        let (tl, tr, bl, br) = (
            [10.0, 1.0, -1.0],
            [10.0, 1.0, 1.0],
            [10.0, -1.0, -1.0],
            [10.0, -1.0, 1.0],
        );
        Pick {
            screen: vec![
                Triangle {
                    corners: [tl, tr, bl],
                    uvs: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
                },
                Triangle {
                    corners: [tr, br, bl],
                    uvs: [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                },
            ],
            exit: vec![[[9.0, -1.0, 0.5], [9.0, -1.0, 1.0], [9.0, -0.5, 1.0]]],
        }
    }

    #[test]
    fn the_pointer_meets_the_screen() {
        let scene = square();
        let camera = MenuCamera {
            tan_half_width: 0.1,
            tan_half_height: 0.1,
            near: 5.0,
        };
        let root = Transform::IDENTITY;
        let hit = |x: f32, y: f32| scene.pick(&root, &camera, (x, y), false);
        // The middle of the picture: the screen's middle.
        let Hit::Screen(uv) = hit(0.0, 0.0) else {
            panic!("missed");
        };
        assert!((uv[0] - 0.5).abs() < 1e-5 && (uv[1] - 0.5).abs() < 1e-5);
        // Right and up: the screen's right edge and top.
        let Hit::Screen(uv) = hit(0.5, 0.5) else {
            panic!("missed");
        };
        assert!((uv[0] - 0.75).abs() < 1e-5 && (uv[1] - 0.25).abs() < 1e-5);
        let Hit::Screen(uv) = hit(0.99, 0.0) else {
            panic!("missed");
        };
        assert!((uv[0] - 0.995).abs() < 1e-4 && (uv[1] - 0.5).abs() < 1e-4);
        assert_eq!(hit(-1.5, 0.0), Hit::Nothing);
        // A click on the button, which hides part of the screen's corner.
        assert_eq!(scene.pick(&root, &camera, (0.9, -0.9), true), Hit::Exit);
        assert!(matches!(
            scene.pick(&root, &camera, (0.9, -0.9), false),
            Hit::Screen(_)
        ));
        // Moved away, the same point of the picture is nearer the middle.
        let far = Transform {
            translation: [10.0, 0.0, 0.0],
            ..Transform::IDENTITY
        };
        let Hit::Screen(uv) = scene.pick(&far, &camera, (0.45, 0.0), false) else {
            panic!("missed");
        };
        assert!((uv[0] - 0.95).abs() < 1e-4);
    }

    #[test]
    fn the_pointer_in_menu_units() {
        // The screen's coordinates reach 0.767 across and 0.755 down: the
        // terminal menu's 960 × 720.
        let (x, y) = menu_point([0.75, 0.75]);
        assert!((x - 960.0).abs() < 0.01 && (y - 720.0).abs() < 0.01);
    }

    #[test]
    fn the_lights_go_with_the_model() {
        let lights = [LightData {
            position: [0.0, 1.0, 0.0],
            color: [1.0; 3],
            radius: 3.0,
            fade: 0.8,
        }];
        let root = root_transform(
            &Settings {
                zoom: 36.0,
                ..Settings::default()
            },
            1920,
            1080,
        );
        let l = lights_at(&lights, &root);
        assert!(close(l[0].position, [37.0, 0.38, 0.0]));
        assert_eq!((l[0].radius, l[0].fade), (3.0, 0.8));
    }

    #[test]
    fn the_button_shows_the_platform_in_use() {
        assert_eq!(
            platform_path(r"textures\terminals\xbox\powerbutton01.dds", PLATFORM_PC).as_deref(),
            Some(r"textures\terminals\pc\powerbutton01.dds")
        );
        assert_eq!(
            platform_path(r"textures\terminals\pc\powerbutton01.dds", PLATFORM_PC),
            None
        );
        assert_eq!(
            platform_path(r"textures\terminals\screen.dds", PLATFORM_PC),
            None
        );
        // A part of a name isn't a folder; forward slashes count.
        assert_eq!(platform_path(r"textures\pcx\a.dds", 1), None);
        assert_eq!(
            platform_path("textures/PS3/a.dds", 1).as_deref(),
            Some(r"textures\xbox\a.dds")
        );
    }

    #[test]
    fn the_fade_in_and_out() {
        let s = Settings {
            fade_in: 1.2,
            fade_out: 2.0,
            ..Settings::default()
        };
        let mut f = Fade::opening();
        // 60 frames a second: 1/72 a frame, in 1.2 s.
        for _ in 0..36 {
            assert!(!f.step(&s, 1.0 / 60.0));
        }
        assert!((f.alpha - 0.5).abs() < 1e-4);
        for _ in 0..40 {
            f.step(&s, 1.0 / 60.0);
        }
        assert_eq!(f.alpha, 1.0);
        // A long frame moves it at most 0.1.
        f.out = true;
        f.step(&s, 1.0);
        assert!((f.alpha - 0.9).abs() < 1e-6);
        // Out over 2 s; ended the frame after it reaches nothing.
        let mut frames = 0;
        while !f.step(&s, 1.0 / 60.0) {
            frames += 1;
            assert!(frames < 200);
        }
        assert_eq!(f.alpha, 0.0);
        assert!((107..=109).contains(&frames));
    }
}
