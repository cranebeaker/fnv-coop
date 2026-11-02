//! What the dialogue menu does to the view and to the speaker, traced from
//! `FalloutNV.exe` 1.4.0.525 (names from the Xbox prototype's symbols are
//! marked `(Xbox PDB)`):
//!
//! - The menu's zoom (`DialogMenu::DoIdle`, `00762950`, states
//!   `eCameraZoomIn` 0 and `eCameraZoomOut` 4, Xbox PDB): a percentage that
//!   rises over `fDialogZoomInSeconds` as the menu opens and falls over
//!   `fDialogZoomOutSeconds` once it is to close, the menu going away only
//!   when it reaches 0 ([`MenuZoom`]).
//! - The player's view on the speaker (`PlayerCharacter::FocusOnActor`,
//!   `00953060`, named by its own error message): the field of view eased
//!   to a zoom on the speaker's head over the rest of the zoom in, and the
//!   player turned to look at the head, with a start and a stop threshold
//!   ([`Focus`]).
//! - The speaker turning to face the player every frame of the menu
//!   (`Actor::UpdateInDialogue`, Xbox PDB, `008a5580`; [`speaker_turns`]).
//!
//! Angles are radians: headings clockwise from north, pitch positive
//! looking up (the game's X angle is positive looking down; the formulas
//! below are the game's with that sign turned). Fields of view are the
//! game's degrees (4:3 widths).

use crate::movement::{wrap_pi, ONE_DEGREE};

/// The settings the menu's view reads: game settings (no record in
/// `FalloutNV.esm` sets them, so the exe's defaults) and the INI's
/// `[Interface]` and `[Display]` ones.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewSettings {
    /// `fDialogZoomInSeconds` (1.5, setting `011d3ee0`).
    pub zoom_in: f32,
    /// `fDialogZoomOutSeconds` (0.5, `011d243c`).
    pub zoom_out: f32,
    /// `[Interface] fDlgFocus` (3.2, `011e0b84`).
    pub focus: f32,
    /// `[Interface] fDlgLookAdj` (-5, `011e08d0`): added to the head's
    /// height.
    pub look_adj: f32,
    /// `[Interface] fDlgLookMult` (2, `011e0958`).
    pub look_mult: f32,
    /// `[Interface] fDlgLookDegStart` (13, `011e094c`) and `…Stop` (0.2,
    /// `011e0900`): the pitch's thresholds.
    pub look_deg_start: f32,
    pub look_deg_stop: f32,
    /// `[Interface] fDlgHeadingDegStart` (13, `011e0b28`) and `…Stop` (0.2,
    /// `011e08c4`): the heading's.
    pub heading_deg_start: f32,
    pub heading_deg_stop: f32,
    /// `[Display] fDefaultFOV` (75, `01203150`).
    pub default_fov: f32,
    /// `fIronSightsFOVTimeChange` (0.25): after the menu the field of view
    /// goes back toward the default at 30 ÷ this degrees a second
    /// (`0095de30`, see `vats_camera::CameraSettings::fov_return_time`).
    pub fov_return_time: f32,
}

impl ViewSettings {
    /// The exe's defaults (the static initialisers `00f6e610`, `00f6e640`,
    /// `00fa8d40`…`00fa8e60`, `00fbc020`).
    pub const DEFAULT: ViewSettings = ViewSettings {
        zoom_in: 1.5,
        zoom_out: 0.5,
        focus: 3.2,
        look_adj: -5.0,
        look_mult: 2.0,
        look_deg_start: 13.0,
        look_deg_stop: 0.2,
        heading_deg_start: 13.0,
        heading_deg_stop: 0.2,
        default_fov: 75.0,
        fov_return_time: 0.25,
    };

    /// The settings: game settings from the load order, INI ones from `ini`
    /// (section, key), each falling back to the exe's default.
    pub fn read(order: &esm::LoadOrder, ini: impl Fn(&str, &str) -> Option<f32>) -> ViewSettings {
        let d = ViewSettings::DEFAULT;
        let g = |name: &str, exe: f32| crate::scripting::game_setting(order, name).unwrap_or(exe);
        let i = |key: &str, exe: f32| ini("Interface", key).unwrap_or(exe);
        ViewSettings {
            zoom_in: g("fDialogZoomInSeconds", d.zoom_in),
            zoom_out: g("fDialogZoomOutSeconds", d.zoom_out),
            focus: i("fDlgFocus", d.focus),
            look_adj: i("fDlgLookAdj", d.look_adj),
            look_mult: i("fDlgLookMult", d.look_mult),
            look_deg_start: i("fDlgLookDegStart", d.look_deg_start),
            look_deg_stop: i("fDlgLookDegStop", d.look_deg_stop),
            heading_deg_start: i("fDlgHeadingDegStart", d.heading_deg_start),
            heading_deg_stop: i("fDlgHeadingDegStop", d.heading_deg_stop),
            default_fov: ini("Display", "fDefaultFOV").unwrap_or(d.default_fov),
            fov_return_time: g("fIronSightsFOVTimeChange", d.fov_return_time),
        }
    }
}

/// The menu's zoom percentage (`DialogMenu::fPercentZoomed`, menu `+0x128`,
/// Xbox PDB).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MenuZoom {
    /// `fElapsedSecs` (`+0x108`): seconds into the zoom in or out.
    elapsed: f32,
    /// In `eCameraZoomOut` (state 4).
    closing: bool,
    /// `fPercentZoomed`, 0..1.
    pub percent: f32,
}

impl MenuZoom {
    /// A menu just opened: nothing zoomed yet (the constructor `007617a0`
    /// starts in state 0 with the elapsed time 0).
    pub fn opening() -> MenuZoom {
        MenuZoom::default()
    }

    /// The menu is to close (`00762ff0`, `00762950`: on entering state 4
    /// the elapsed time is set to 0).
    pub fn close(&mut self) {
        if !self.closing {
            self.closing = true;
            self.elapsed = 0.0;
        }
    }

    pub fn closing(&self) -> bool {
        self.closing
    }

    /// One frame (`00762950`). State 0: the percentage is
    /// min(elapsed ÷ `fDialogZoomInSeconds`, 1); once it is 1 the menu goes
    /// on to the line (state 3) and it stays. State 4: 1 − min(elapsed ÷
    /// `fDialogZoomOutSeconds`, 1). Whether the menu is done (state 4 at
    /// 0: the menu is destroyed).
    // Translated from 00762950 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn step(&mut self, dt: f32, s: &ViewSettings) -> bool {
        if self.closing {
            self.elapsed += dt;
            self.percent = 1.0 - (self.elapsed / s.zoom_out).min(1.0);
            self.percent == 0.0
        } else {
            if self.percent < 1.0 {
                self.elapsed += dt;
                self.percent = (self.elapsed / s.zoom_in).min(1.0);
            }
            false
        }
    }
}

/// What the menu hands the focus each frame: its percentage times the
/// speaker's dialogue package's zoom (`DialogMenu::fPackagePercentZoom`,
/// Xbox PDB, `011a008c`): set at the menu's creation (`00761a20`) from the
/// speaker's running package when it is a dialogue package (type 15,
/// `00672850`: its `PKDD` float), else 100; 0 counts as 100.
pub fn focus_percent(percent: f32, package_zoom: Option<f32>) -> f32 {
    let zoom = package_zoom.unwrap_or(100.0);
    if zoom == 0.0 {
        percent
    } else {
        percent * (zoom / 100.0)
    }
}

/// The player's view on the speaker (`00953060`'s statics): the last
/// percentage (`011e0d48`), the zoom's field of view (`011e0d40`) and
/// whether the pitch (`011e0d39`) and the heading (`011e0d38`) are being
/// turned.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Focus {
    last_percent: f32,
    target_fov: f32,
    pitching: bool,
    turning: bool,
}

/// One frame's input to [`Focus::frame`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusInput {
    /// The camera's position.
    pub eye: [f32; 3],
    /// The speaker's head: the centre and radius of its face node's world
    /// bound (`GetFaceNode`, Xbox PDB, then `0043d450`), or without one the
    /// `Bip01 Head` node's position with radius 32 (`0101e340`).
    pub head: ([f32; 3], f32),
    /// [`focus_percent`].
    pub percent: f32,
    /// The menu is zooming out (`00762950` passes state == 4): the view
    /// isn't turned.
    pub zooming_out: bool,
    /// Seconds since the last frame.
    pub dt: f32,
    /// The player's heading and pitch.
    pub heading: f32,
    pub pitch: f32,
    /// The world's and the first-person view's fields of view (player
    /// `+0x670`, `+0x674`).
    pub fovs: (f32, f32),
}

/// What [`Focus::frame`] gives: the new heading, pitch and fields of view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusOutput {
    pub heading: f32,
    pub pitch: f32,
    pub fovs: (f32, f32),
}

impl Focus {
    /// One frame of `PlayerCharacter::FocusOnActor` (`00953060`).
    ///
    /// The point looked at is the head's centre raised by `fDlgLookAdj`.
    /// When the last percentage was 0 the zoom is chosen: atan(`fDlgFocus`
    /// × radius ÷ distance) radians × 100, at most `fDefaultFOV`. Both
    /// fields of view move toward it by min(dt ÷ ((1 − last) ×
    /// `fDialogZoomInSeconds`), 1) of the way, so that they arrive as the
    /// zoom in ends.
    ///
    /// Unless zooming out, the pitch and then the heading turn toward the
    /// point: each starts once more than its start threshold off (degrees ×
    /// the first-person field of view ÷ `fDefaultFOV`) or while the zoom in
    /// is under way, moves by the same share of the way while zooming in or
    /// else by the gap × `fDlgLookMult` × dt, and stops once the gap (before
    /// the move) is under its stop threshold. The heading gap is brought
    /// into −π..π; the pitch's isn't.
    ///
    /// Not carried out: a seated player's heading change goes to a look
    /// offset of its own (player `+0x6e4`), not the heading; the menu's
    /// depth of field (`fDialogFocalDepth*`); the switch to the first-person
    /// view (`00951a10`, `00950110`).
    // Translated from 00953060 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn frame(&mut self, s: &ViewSettings, i: &FocusInput) -> FocusOutput {
        let percent = i.percent.clamp(0.0, 1.0);
        let last = self.last_percent;
        let (center, radius) = i.head;
        let point = [center[0], center[1], center[2] + s.look_adj];
        let d = [
            point[0] - i.eye[0],
            point[1] - i.eye[1],
            point[2] - i.eye[2],
        ];
        let distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-3);
        if last == 0.0 {
            self.target_fov = ((s.focus * radius / distance).atan() * 100.0).min(s.default_fov);
        }
        // The share of the way this frame while zooming in (1 once in).
        let share = |dt: f32| {
            let left = (1.0 - last) * s.zoom_in;
            if left <= 0.0 {
                1.0
            } else {
                (dt / left).min(1.0)
            }
        };
        let k = share(i.dt);
        let fovs = (
            i.fovs.0 + k * (self.target_fov - i.fovs.0),
            i.fovs.1 + k * (self.target_fov - i.fovs.1),
        );
        let (mut heading, mut pitch) = (i.heading, i.pitch);
        if !i.zooming_out {
            let scale = fovs.1 / s.default_fov;
            let zooming = last < 1.0;
            // The pitch.
            let want_pitch = (d[2] / distance).clamp(-1.0, 1.0).asin();
            let gap = want_pitch - pitch;
            if gap.abs() > s.look_deg_start * ONE_DEGREE * scale || zooming {
                self.pitching = true;
            }
            if self.pitching {
                pitch += if zooming {
                    gap * share(i.dt)
                } else {
                    gap * s.look_mult * i.dt
                };
                if gap.abs() < s.look_deg_stop * ONE_DEGREE * scale {
                    self.pitching = false;
                }
            }
            // The heading.
            let want_heading = d[0].atan2(d[1]);
            let gap = wrap_pi(want_heading - heading);
            if gap.abs() > s.heading_deg_start * ONE_DEGREE * scale || zooming {
                self.turning = true;
            }
            if self.turning {
                heading += if zooming {
                    gap * share(i.dt)
                } else {
                    gap * s.look_mult * i.dt
                };
                if gap.abs() < s.heading_deg_stop * ONE_DEGREE * scale {
                    self.turning = false;
                }
            }
        }
        self.last_percent = percent;
        FocusOutput {
            heading,
            pitch,
            fovs,
        }
    }
}

/// After the menu: the field of view one frame further back toward the
/// default, at 30 ÷ `fIronSightsFOVTimeChange` degrees a second
/// (`0095de30`'s return outside V.A.T.S. and dialogue; which of the game's
/// updates calls it is not traced).
pub fn fov_back(fov: f32, default: f32, dt: f32, s: &ViewSettings) -> f32 {
    let step = dt * 30.0 / s.fov_return_time;
    if fov < default {
        (fov + step).min(default)
    } else {
        (fov - step).max(default)
    }
}

/// Whether the speaker asks for a turn toward the player this frame of the
/// menu (`Actor::UpdateInDialogue`, Xbox PDB, `008a5580`): someone with a
/// mover who isn't seated (`GetSitSleepState` 0) or in combat, isn't
/// turning already (mover state 4), and is more than a degree off
/// (`01023128`). The turn itself (`009dce80`) then goes at the in-place
/// rate (`movement::Turn`), or straight to the heading for someone not
/// loaded, or not at all for someone who can't turn (`008843a0`). The mover
/// is updated every frame meanwhile with its dialogue flag set
/// (`ActorMover::SetInDialog`, Xbox PDB, `009c9900`), so the turn plays out.
// Translated from 008a5580 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn speaker_turns(
    heading: f32,
    toward_player: f32,
    turning: bool,
    seated: bool,
    in_combat: bool,
) -> bool {
    !seated && !in_combat && !turning && wrap_pi(toward_player - heading).abs() > ONE_DEGREE
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: ViewSettings = ViewSettings::DEFAULT;

    #[test]
    fn the_zoom_rises_over_the_zoom_in_and_falls_over_the_zoom_out() {
        let mut z = MenuZoom::opening();
        assert!(!z.step(0.75, &S));
        assert!((z.percent - 0.5).abs() < 1e-6);
        assert!(!z.step(1.0, &S));
        assert_eq!(z.percent, 1.0);
        z.close();
        assert!(!z.step(0.25, &S));
        assert!((z.percent - 0.5).abs() < 1e-6);
        assert!(z.step(0.3, &S), "the menu goes once it reaches 0");
        assert_eq!(z.percent, 0.0);
    }

    #[test]
    fn a_dialogue_packages_zoom_scales_the_percentage() {
        assert_eq!(focus_percent(0.5, None), 0.5);
        assert_eq!(focus_percent(0.5, Some(0.0)), 0.5);
        assert!((focus_percent(0.5, Some(120.0)) - 0.6).abs() < 1e-6);
    }

    fn input(percent: f32, dt: f32, heading: f32, pitch: f32, fovs: (f32, f32)) -> FocusInput {
        FocusInput {
            eye: [0.0, 0.0, 0.0],
            // 100 ahead (north), level once the look adjustment is added.
            head: ([0.0, 100.0, 5.0], 12.0),
            percent,
            zooming_out: false,
            dt,
            heading,
            pitch,
            fovs,
        }
    }

    #[test]
    fn the_view_zooms_on_the_head_by_the_end_of_the_zoom_in() {
        let mut f = Focus::default();
        // atan(3.2 × 12 / 100) × 100 = 36.65.
        let want = (0.384f32).atan() * 100.0;
        let mut fovs = (75.0, 55.0);
        let mut heading = 0.5;
        let mut pitch = 0.0;
        // 1.5 s in 0.1 s frames: the percentage reaches 1 on the last.
        for n in 1..=15 {
            let out = f.frame(&S, &input(n as f32 / 15.0, 0.1, heading, pitch, fovs));
            fovs = out.fovs;
            heading = out.heading;
            pitch = out.pitch;
        }
        // The last frame's share was dt ÷ ((1 − 14/15) × 1.5) = 1.
        assert!((fovs.0 - want).abs() < 1e-3, "{fovs:?}");
        assert!((fovs.1 - want).abs() < 1e-3);
        assert!(heading.abs() < 1e-4, "turned to face the head: {heading}");
        assert!(pitch.abs() < 1e-4);
    }

    #[test]
    fn the_zoom_is_at_most_the_default_field_of_view() {
        let mut f = Focus::default();
        let mut i = input(0.0, 0.1, 0.0, 0.0, (75.0, 75.0));
        i.head = ([0.0, 10.0, 5.0], 40.0);
        let out = f.frame(&S, &i);
        // atan(12.8) × 100 ≈ 149: held at 75.
        assert_eq!(out.fovs.0, 75.0);
    }

    #[test]
    fn once_zoomed_the_view_turns_only_past_the_start_threshold() {
        let mut f = Focus::default();
        // A head so big the zoom is held at the default (atan(1.28) × 100
        // ≈ 90), so the thresholds are their full degrees.
        let at = |heading: f32| {
            let mut i = input(1.0, 0.1, heading, 0.0, (75.0, 75.0));
            i.head.1 = 40.0;
            i
        };
        // Zoomed in already (the last percentage 1).
        f.frame(&S, &at(0.0));
        f.pitching = false;
        f.turning = false;
        // 10° off: under the 13° start, no turn.
        let off = 10.0 * ONE_DEGREE;
        let out = f.frame(&S, &at(off));
        assert_eq!(out.heading, off);
        // 20° off: the turn starts, 2 × 0.1 of the gap this frame.
        let off = 20.0 * ONE_DEGREE;
        let out = f.frame(&S, &at(off));
        assert!((out.heading - off * 0.8).abs() < 1e-5, "{}", out.heading);
        // It goes on below the start until under the 0.2° stop.
        let out = f.frame(&S, &at(off * 0.8));
        assert!(out.heading < off * 0.8);
        let tiny = 0.1 * ONE_DEGREE;
        let out = f.frame(&S, &at(tiny));
        assert!(out.heading < tiny, "this frame still moves");
        assert!(!f.turning, "then stops");
    }

    #[test]
    fn zooming_out_the_view_is_not_turned() {
        let mut f = Focus::default();
        let mut i = input(0.5, 0.1, 1.0, 0.3, (40.0, 40.0));
        i.zooming_out = true;
        let out = f.frame(&S, &i);
        assert_eq!((out.heading, out.pitch), (1.0, 0.3));
    }

    #[test]
    fn after_the_menu_the_field_of_view_goes_back_at_120_degrees_a_second() {
        assert!((fov_back(40.0, 75.0, 0.1, &S) - 52.0).abs() < 1e-4);
        assert_eq!(fov_back(70.0, 75.0, 0.1, &S), 75.0);
    }

    #[test]
    fn the_speaker_turns_when_more_than_a_degree_off_and_free_to() {
        let off = 5.0 * ONE_DEGREE;
        assert!(speaker_turns(0.0, off, false, false, false));
        assert!(!speaker_turns(0.0, 0.5 * ONE_DEGREE, false, false, false));
        assert!(
            !speaker_turns(0.0, off, true, false, false),
            "already turning"
        );
        assert!(!speaker_turns(0.0, off, false, true, false), "seated");
        assert!(!speaker_turns(0.0, off, false, false, true), "in combat");
        // Across north.
        assert!(speaker_turns(6.2, 0.1, false, false, false));
    }
}
