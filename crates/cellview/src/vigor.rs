//! Vit-o-matic rendered-menu placement (`007928e0`) and camera
//! (`007945f0`). These are scene inputs, not a substitute menu. See
//! `docs/VIGOR.md` for the executable evidence and remaining presentation.

use nif::math::{mat_mul, Transform};
use std::sync::Arc;

pub const ACTIVATE_MODEL: &str =
    "meshes\\architecture\\Goodsprings\\NV_VitoMaticVigorTester_Activate.NIF";
pub const CABINET_MODEL: &str =
    "meshes\\architecture\\Goodsprings\\NV_VitoMaticVigorTester_Cabinet.NIF";
pub const ACTIVATE_REFERENCE: u32 = 1;
pub const CABINET_REFERENCE: u32 = 2;

pub const ATTRIBUTES: [&str; 7] = [
    "Strength",
    "Perception",
    "Endurance",
    "Charisma",
    "Intelligence",
    "Agility",
    "Luck",
];

/// A texture replacement requested by the menu's model callbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Picture {
    Number(u8),
    Right(bool),
    Left,
}

impl Picture {
    /// 00796100: the PC texture branch (004b71d0 false; its true branch
    /// spells XBOX at0101fd1c, PC at0101fd24).
    pub fn path(self) -> String {
        match self {
            Self::Number(n) => format!("textures\\terminals\\BBNumber{n}.dds"),
            Self::Right(on) => format!(
                "textures\\terminals\\PC\\BBRT{}.dds",
                if on { "On" } else { "Off" }
            ),
            Self::Left => "textures\\terminals\\PC\\BBLTOff.dds".into(),
        }
    }
}

/// Inputs to 00795e20 and its individual shape callbacks. Values are
/// read afresh from the player, including while this menu is open.
pub struct DisplayState {
    pub page: u8,
    pub selection: u8,
    pub controller: bool,
    pub values: [i32; 7],
    pub budget: i32,
}

impl DisplayState {
    pub fn current_value(&self) -> i32 {
        let page = if self.page == 8 && self.controller {
            self.selection
        } else {
            self.page
        };
        page.checked_sub(1)
            .and_then(|i| self.values.get(usize::from(i)))
            .copied()
            .unwrap_or(0)
    }

    /// (visible, replacement texture). Cabinet bulb visibility is a
    /// separate callback from the activate model's numbers and buttons.
    /// The main +/- buttons have AV=-1 and skip the index visibility test.
    pub fn appearance(&self, reference: u32, name: &str) -> (bool, Option<Picture>) {
        if reference == CABINET_REFERENCE {
            if let Some(tail) = name.strip_prefix("P1_PointVal_") {
                let stem = tail.split(':').next().unwrap_or(tail);
                if let Some(n) = stem
                    .strip_suffix("_GLOW")
                    .and_then(|n| n.parse::<u8>().ok())
                    .filter(|n| (1..=10).contains(n))
                {
                    return (self.current_value() >= i32::from(n), None);
                }
            }
            return (true, None);
        }
        let total: i32 = self.values.iter().sum();
        match name {
            // 007953f0: show BOTH digits even when the tens digit is zero.
            "P1_PointsRemainDigit1:0" => {
                return (
                    true,
                    Some(Picture::Number(
                        ((self.budget - total) / 10).clamp(0, 9) as u8
                    )),
                )
            }
            "P1_PointsRemainDigit2:0" => {
                return (
                    true,
                    Some(Picture::Number(((self.budget - total) % 10).max(0) as u8)),
                )
            }
            // 00795700: On is selected while points REMAIN on page8.
            "P1_RT_Btn:0" | "LookInside_Btn:0" | "AllDone_Btn:0" => {
                return (
                    true,
                    Some(Picture::Right(self.page == 8 && total < self.budget)),
                )
            }
            "P1_LT_Btn:0" => return (true, Some(Picture::Left)),
            _ => {}
        }
        if let Some(tail) = name.strip_prefix("Index_") {
            for (i, attribute) in ATTRIBUTES.iter().enumerate() {
                if let Some(part) = tail.strip_prefix(attribute) {
                    if part == "PointVal:0" {
                        // The exe indexes its 11 entries directly. Refuse an
                        // invalid value instead of inventing a clamped digit.
                        return (
                            true,
                            u8::try_from(self.values[i])
                                .ok()
                                .filter(|&v| v <= 10)
                                .map(Picture::Number),
                        );
                    }
                    let available = match part {
                        "Increase_Btn:0" => self.values[i] < 10 && total < self.budget,
                        "Decrease_Btn:0" => self.values[i] > 1,
                        _ => continue,
                    };
                    let selected =
                        !self.controller || self.page != 8 || usize::from(self.selection) == i + 1;
                    return (available && selected, None);
                }
            }
        }
        (true, None)
    }
}

/// Both menu models in their own coordinates, with their file root
/// transforms removed. Apply model_transform to every draw. Animation
/// layers are the activate model's sequences, not the world's idle loop.
pub struct VigorScene {
    pub scene: crate::ViewerScene,
    pub sequences: Arc<Vec<nif::Sequence>>,
    pub lights: Vec<crate::LightData>,
    pub pictures: Vec<(Picture, usize)>,
    /// Original local vertices and winding for NiPick, separate from the
    /// renderer's normal-based winding conversion.
    buttons: Vec<nif::Mesh>,
}

impl VigorScene {
    pub fn load(assets: &assets::Assets) -> Result<Self, crate::Error> {
        let read = |path: &str| -> Result<nif::Nif, crate::Error> {
            let bytes = assets
                .read(path)
                .map_err(|e| crate::Error(e.to_string()))?
                .ok_or_else(|| crate::Error(format!("Missing Vit-o-matic model: {path}")))?;
            nif::Nif::parse(bytes).map_err(|e| crate::Error(format!("{path}: {e}")))
        };
        let activate = read(ACTIVATE_MODEL)?;
        let buttons = activate
            .placed_scene()
            .map_err(|e| crate::Error(e.to_string()))?
            .meshes
            .into_iter()
            .filter(|m| m.name.ends_with("_Btn:0"))
            .collect();
        let bound_radius = activate
            .roots()
            .first()
            .and_then(|&r| {
                crate::lockpick::bound(
                    &activate,
                    r,
                    &Transform::IDENTITY,
                    Some(&Transform::IDENTITY),
                )
            })
            .map_or(0.0, |(_, radius)| radius);
        let embedded = activate.lights().map_err(|e| crate::Error(e.to_string()))?;
        if !embedded.is_empty() {
            return Err(crate::Error(
                "Vit-o-matic embedded menu lights are not supported yet".into(),
            ));
        }
        // 00792e36 -> 00440460 sets translation (0,0,0). 0050dd50
        // writes (20*bound_radius,0,0) to +e0/e4/e8, the menu shader's
        // radius fields, NOT the light's position. 004bc2e0 sets diffuse.
        let lights = vec![crate::LightData {
            position: [0.0; 3],
            color: [0.85; 3],
            radius: 20.0 * bound_radius,
            fade: 1.0,
        }];
        // Fail before returning a partial scene if either model is absent.
        let _cabinet = read(CABINET_MODEL)?;
        let sequences = Arc::new(
            activate
                .sequences()
                .map_err(|e| crate::Error(e.to_string()))?,
        );
        let mut cell = world::LoadedCell::actors_only(Vec::new());
        for (reference, model) in [
            (ACTIVATE_REFERENCE, ACTIVATE_MODEL),
            (CABINET_REFERENCE, CABINET_MODEL),
        ] {
            cell.objects.push(world::Placement {
                form_id: esm::FormId(reference),
                record_type: esm::FourCC::new(b"REFR"),
                editor_id: None,
                base: esm::FormId(0),
                base_type: esm::FourCC::new(b"STAT"),
                base_editor_id: None,
                position: [0.0; 3],
                rotation: [0.0; 3],
                scale: 1.0,
                model: Some(model.into()),
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
        }
        // 007928e0 replaces both file roots. Placed-scene conversion
        // removes those roots; the explicit menu transform replaces them.
        let built = preview::cell::build_scene(assets, cell);
        let mut cache = crate::TextureCache::new(assets);
        let mut scene = crate::convert(&built, &mut cache);
        let pictures = (0..=10)
            .map(Picture::Number)
            .chain([Picture::Right(false), Picture::Right(true), Picture::Left])
            .map(|picture| {
                let path = picture.path();
                cache
                    .get(&path)
                    .map(|index| (picture, index))
                    .ok_or_else(|| crate::Error(format!("Missing Vit-o-matic texture: {path}")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (textures, notes) = cache.finish();
        scene.textures = textures;
        scene.notes.extend(notes);
        for reference in [ACTIVATE_REFERENCE, CABINET_REFERENCE] {
            if !scene.draws.iter().any(|d| d.reference == reference) {
                return Err(crate::Error(format!(
                    "Vit-o-matic model {reference} has no drawable pieces"
                )));
            }
        }
        Ok(Self {
            scene,
            sequences,
            lights,
            pictures,
            buttons,
        })
    }

    /// 00791e40 picks the current page's buttons in registration order,
    /// then the common buttons, stopping at the first hit in EACH list.
    /// Call again for the common list after applying the page click.
    pub fn pick_button(
        &self,
        page_specific: bool,
        display: &DisplayState,
        width: u32,
        origin: [f32; 3],
        direction: [f32; 3],
        layer: Option<(&nif::Sequence, f32)>,
    ) -> Option<&str> {
        let names = if !page_specific {
            vec![
                "P1_Decrease_Btn:0".into(),
                "P1_Increase_Btn:0".into(),
                "P1_RT_Btn:0".into(),
                "P1_LT_Btn:0".into(),
            ]
        } else if display.page == 0 {
            vec!["LookInside_Btn:0".into()]
        } else if display.page == 8 {
            ATTRIBUTES
                .iter()
                .flat_map(|a| {
                    [
                        format!("Index_{a}Decrease_Btn:0"),
                        format!("Index_{a}Increase_Btn:0"),
                    ]
                })
                .chain(["AllDone_Btn:0".into()])
                .collect()
        } else {
            Vec::<String>::new()
        };
        for name in names {
            if !display.appearance(ACTIVATE_REFERENCE, &name).0 {
                continue;
            }
            let Some(mesh) = self.buttons.iter().find(|m| m.name == name) else {
                continue;
            };
            let layers: Vec<_> = layer.into_iter().collect();
            let inverse = model_transform(width)
                .then_child(&mesh.posed_transform(&layers))
                .inverse();
            let local_origin = inverse.apply_point(origin);
            // 00e9b4f0 divides both the ray position and direction by the
            // shape scale. Do not renormalize the transformed direction.
            let local_direction = inverse
                .apply_direction(direction)
                .map(|v| v * inverse.scale);
            if mesh.triangles.iter().any(|t| {
                let Some(a) = mesh.positions.get(usize::from(t[0])) else {
                    return false;
                };
                let Some(b) = mesh.positions.get(usize::from(t[1])) else {
                    return false;
                };
                let Some(c) = mesh.positions.get(usize::from(t[2])) else {
                    return false;
                };
                pick_triangle(local_origin, local_direction, *a, *b, *c)
            }) {
                return Some(&mesh.name);
            }
        }
        None
    }

    /// A piece's animated transform, followed by the menu model root.
    /// Passing no layer leaves the file's mesh pose intact.
    pub fn piece_matrix(
        screen_width: u32,
        motion: Option<&crate::PieceMotion>,
        layer: Option<(&nif::Sequence, f32)>,
    ) -> [f32; 16] {
        let root = model_transform(screen_width);
        let transform = match (motion, layer) {
            (Some(motion), Some(layer)) => root.then_child(&motion.motion.transform(&[layer])),
            _ => root,
        };
        crate::column_major(&transform)
    }
}

/// 00eadb90, the front-only branch selected by NiPick's default +0x10=1.
/// Inclusive edges, nonnegative ray distance, determinant >= 1e-5
/// (FLOAT010718c0). Work in the original geometry's local coordinates.
fn pick_triangle(o: [f32; 3], d: [f32; 3], a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> bool {
    let sub = |a: [f32; 3], b: [f32; 3]| std::array::from_fn(|i| a[i] - b[i]);
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let cross = |a: [f32; 3], b: [f32; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let e1 = sub(b, a);
    let e2 = sub(c, a);
    let p = cross(d, e2);
    let determinant = dot(e1, p);
    if determinant < 1.0e-5 {
        return false;
    }
    let t = sub(o, a);
    let u = dot(t, p);
    if u < 0.0 || u > determinant {
        return false;
    }
    let q = cross(t, e1);
    let v = dot(d, q);
    v >= 0.0 && u + v <= determinant && dot(e2, q) >= 0.0
}

/// Both models have their root transforms replaced, not composed with
/// their file transforms. `0079299d..00792a1b` multiplies the game's X,
/// Z, X matrices in that order. The helpers' signs match the lockpick
/// scene (`00524ac0`, `004a0c90`, `0043f8d0`).
pub fn model_transform(screen_width: u32) -> Transform {
    let x = |angle: f32| {
        let (s, c) = angle.sin_cos();
        [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]]
    };
    let (s, c) = std::f32::consts::PI.sin_cos();
    let z = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
    // Exact stored float at 01074b90, not a fitted camera angle.
    let tilt = f32::from_bits(0x3e00_adfd);
    Transform {
        rotation: mat_mul(&mat_mul(&x(std::f32::consts::FRAC_PI_2), &z), &x(tilt)),
        // 00792a22 compares against DOUBLE01074b88 = 640.0.
        translation: [0.0, -85.0, if screen_width < 640 { -75.0 } else { -85.0 }],
        scale: 1.0,
    }
}

/// Frustum extents from 007945f0. The camera itself is at (0,0,1), looking
/// at the origin with +Y up. The model placement above is in that scene's
/// axes; it must not use the lockpick camera's +X forward convention.
pub fn camera(
    settings: &assets::IniSettings,
    width: u32,
    height: u32,
) -> crate::lockpick::MenuCamera {
    let fov = settings.float("Display", "fDefaultFOV").unwrap_or(75.0);
    let near = settings.float("Display", "fNearDistance").unwrap_or(5.0);
    // DOUBLE01023128 and DOUBLE01074b98 contain widened float values.
    // Reading the latter as a single float would incorrectly yield -2.
    let radians = f64::from(fov) * f64::from(1.0f32.to_radians()) * f64::from(0.65f32);
    let tangent = (radians as f32).tan();
    crate::lockpick::MenuCamera {
        tan_half_width: tangent * 0.75,
        tan_half_height: tangent * (height as f32 / width.max(1) as f32) * 0.75,
        near,
    }
}

/// The animation name for the new page after navigation (007949f0).
/// Forward indexes 011a03a0 by page-1; backward indexes 011a03c0 by page.
/// Reaching forward page9 is a Done request, not another animation.
pub fn page_sequence(page: u8, forward: bool) -> Option<&'static str> {
    const FORWARD: [&str; 8] = [
        "Forward",
        "Backward",
        "Left",
        "Right",
        "FastForward",
        "FastBackward",
        "FastLeft",
        "FastRight",
    ];
    const BACKWARD: [&str; 8] = [
        "AimUp",
        "Aim",
        "Holster",
        "JumpLand",
        "JumpLoop",
        "JumpStart",
        "TurnRight",
        "TurnLeft",
    ];
    if forward {
        FORWARD.get(usize::from(page.checked_sub(1)?)).copied()
    } else {
        BACKWARD.get(usize::from(page)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_triangle_picker_rejects_backs_and_preserves_edge_hits() {
        let (a, b, c) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!(pick_triangle([0.25, 0.25, 1.0], [0.0, 0.0, -1.0], a, b, c));
        assert!(pick_triangle([0.0, 0.0, 1.0], [0.0, 0.0, -1.0], a, b, c));
        assert!(!pick_triangle([0.25, 0.25, -1.0], [0.0, 0.0, 1.0], a, b, c));
        assert!(!pick_triangle([0.75, 0.75, 1.0], [0.0, 0.0, -1.0], a, b, c));
        assert!(!pick_triangle(
            [0.25, 0.25, -1.0],
            [0.0, 0.0, -1.0],
            a,
            b,
            c
        ));
        assert!(!pick_triangle(
            [0.25, 0.25, 1.0],
            [0.0, 0.0, -0.000001],
            a,
            b,
            c
        ));
    }

    #[test]
    fn menu_callbacks_drive_numbers_bulbs_and_index_button_visibility() {
        let mut state = DisplayState {
            page: 1,
            selection: 1,
            controller: false,
            values: [5; 7],
            budget: 40,
        };
        assert_eq!(state.appearance(2, "P1_PointVal_05_GLOW:0"), (true, None));
        assert_eq!(state.appearance(2, "P1_PointVal_06_GLOW:9"), (false, None));
        assert_eq!(
            state.appearance(1, "P1_PointsRemainDigit1:0"),
            (true, Some(Picture::Number(0)))
        );
        assert_eq!(
            state.appearance(1, "P1_PointsRemainDigit2:0"),
            (true, Some(Picture::Number(5)))
        );
        assert_eq!(
            state.appearance(1, "Index_LuckPointVal:0"),
            (true, Some(Picture::Number(5)))
        );
        state.page = 8;
        assert_eq!(state.appearance(2, "P1_PointVal_01_GLOW"), (false, None));
        assert_eq!(
            state.appearance(1, "AllDone_Btn:0"),
            (true, Some(Picture::Right(true)))
        );
        state.controller = true;
        assert_eq!(state.appearance(2, "P1_PointVal_05_GLOW:0"), (true, None));
        assert_eq!(
            state.appearance(1, "Index_LuckIncrease_Btn:0"),
            (false, None)
        );
        assert_eq!(
            state.appearance(1, "Index_StrengthIncrease_Btn:0"),
            (true, None)
        );
        state.values[0] = 10;
        assert_eq!(
            state.appearance(1, "Index_StrengthIncrease_Btn:0"),
            (false, None)
        );
        assert_eq!(state.appearance(1, "P1_Increase_Btn:0"), (true, None));
        assert_eq!(
            state.appearance(1, "AllDone_Btn:0"),
            (true, Some(Picture::Right(false)))
        );
    }

    #[test]
    fn model_root_uses_the_dimension_threshold_and_rotation_order() {
        assert_eq!(model_transform(639).translation, [0.0, -85.0, -75.0]);
        assert_eq!(model_transform(640).translation, [0.0, -85.0, -85.0]);
        let root = model_transform(1920);
        // Z(pi) reverses model X. The final tilt makes model Y point
        // mainly scene +Z and model Z mainly scene +Y.
        let right = root.apply_direction([1.0, 0.0, 0.0]);
        let forward = root.apply_direction([0.0, 1.0, 0.0]);
        let up = root.apply_direction([0.0, 0.0, 1.0]);
        assert!((right[0] + 1.0).abs() < 1e-6);
        assert!((forward[1] + 0.125_333_23).abs() < 1e-6);
        assert!((forward[2] - 0.992_114_7).abs() < 1e-6);
        assert!((up[1] - 0.992_114_7).abs() < 1e-6);
        assert!((up[2] - 0.125_333_23).abs() < 1e-6);
    }

    #[test]
    fn camera_uses_the_menu_fov_share_and_screen_aspect() {
        let ini = assets::IniSettings::default();
        let wide = camera(&ini, 1920, 1080);
        // 75 * 0.65 = 48.75 degrees before tan and the 0.75 factor.
        assert!((wide.tan_half_width - 0.855_211_5).abs() < 1e-5);
        assert!((wide.tan_half_height / wide.tan_half_width - 0.5625).abs() < 1e-6);
        assert_eq!(wide.near, 5.0);
        let square = camera(&ini, 640, 640);
        assert_eq!(square.tan_half_width, square.tan_half_height);
    }

    #[test]
    fn navigation_uses_distinct_forward_and_backward_animation_tables() {
        assert_eq!(page_sequence(1, true), Some("Forward"));
        assert_eq!(page_sequence(8, true), Some("FastRight"));
        assert_eq!(page_sequence(1, false), Some("Aim"));
        assert_eq!(page_sequence(0, false), Some("AimUp"));
        assert_eq!(page_sequence(7, false), Some("TurnLeft"));
        assert_eq!(page_sequence(0, true), None);
        assert_eq!(page_sequence(9, true), None);
        assert_eq!(page_sequence(8, false), None);
    }
}
