//! The player's camera: first and third person, the view key (hold it to
//! look around the player, tap it to switch), the wheel zoom, vanity mode
//! (the camera circling an idle player), the temporary views (third person
//! while sitting down or getting up, first person while the Pip-Boy is up)
//! and the chase camera that places the third-person view behind the
//! player's shoulder, kept out of walls.
//!
//! Read from `FalloutNV.exe` 1.4.0.525 with Ghidra. The PC addresses were
//! found from the settings each function reads; the names marked (Xbox
//! PDB) are the Xbox 360 prototype's (`Fallout_Release_MemDebug.pdb`),
//! matched by what the functions do and the `PlayerCharacter` layout, whose
//! fields sit 0x10 lower on PC from +0x648 on (`b3rdPerson` Xbox +0x65a, PC
//! +0x64a). Globals are named by their PC address.
//!
//! | PC | What |
//! | --- | --- |
//! | `0094ae40` | `PlayerCharacter::UpdateCamera` (Xbox PDB): the third-person branch is [`PlayerCamera::update`] |
//! | `0094a0c0` | `PlayerCharacter::UpdateChaseCamera` (Xbox PDB): [`PlayerCamera::chase`] |
//! | `00950110` | `PlayerCharacter::SetFirstPerson` (Xbox PDB): [`PlayerCamera::set_first_person`] |
//! | `00950290` | the end of a switch, at the end of `UpdateCamera`: [`PlayerCamera::finish_switch`] |
//! | `00950340`, `009503d0` | `ForceTemp3rdPerson`, `UpdateTemp3rdPerson` (Xbox PDB) |
//! | `00950460`, `00950530` | `ForceTemp1stPerson`, `UpdateTemp1stPerson` (Xbox PDB) |
//! | `009500a0` | `PlayerCharacter::StopVanityMode` (Xbox PDB) |
//! | `00951a10` | which of the two bodies shows (`Show1stPerson`, Xbox PDB, by its use) |
//! | `00942be8`..`00942e05` | the view key (control 13) in the player's control handler |
//! | `00943483`..`0094362b` | vanity mode starting and turning |
//! | `00945a17`..`00945c62`, `00945ce4`..`00945d92` | the wheel, and the mouse while the view key is held (`009445b0`) |
//! | `00953060` | `PlayerCharacter::FocusOnActor` (its error text): talking forces first person |
//!
//! Not modelled (left out, labelled where they'd act): the fade of the
//! player's body when the camera is inside it (`006214d0` against
//! `fCameraCasterPlayerSize`, alpha at `011a3b38` ±4 a second), the
//! first-person branch of `UpdateCamera` (the viewer's eye is unchanged),
//! the kill-camera game mode (`0044ddc0() == 4`), the HUD mode changes
//! (`00771700`), iron sights forcing first person (`008bb650`), and the
//! unknown globals `011f21d0`/`011f21d1` (taken as false), `011e0780`,
//! `011e0788` (taken as 0) and `011a3b31` (taken as set).

use esm::LoadOrder;

use crate::furniture::TempThirdPerson;
use crate::scripting::game_setting;

/// π/180 as the exe multiplies by it (the double at `01023128`).
const DEG_TO_RAD: f64 = 0.017_453_292_519_943_295;

/// 2π (the double at `0101ff48`), the wrap of the vanity angles.
const TAU: f32 = std::f32::consts::TAU;

/// What a wheel notch counts in the game's mouse input (DirectInput's
/// `WHEEL_DELTA`; `00945a82` divides the wheel by the double 120 at
/// `010718b8`).
pub const WHEEL_NOTCH: i32 = 120;

/// The game settings the camera reads (`GMST` where the data sets them,
/// else the exe's defaults; INI ones from the viewer, see [`IniCamera`]).
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSettings {
    /// `fOverShoulderPosX` (40) and `fOverShoulderPosZ` (-10): the camera's
    /// offset to the right and down from the pivot.
    pub pos_x: f32,
    pub pos_z: f32,
    /// `fOverShoulderStartBlendDist` (exe 0, data 40000): above 0 the
    /// shoulder offset fades as the zoom approaches it.
    pub start_blend_dist: f32,
    /// `fOverShoulderRotMult` (0.6).
    pub rot_mult: f32,
    /// `fOverShoulderOffsetPoint{X,Y,Z}` (98.4, 16, -24.8) and
    /// `fOverShoulderOffsetPointZooming{X,Y,Z}` (0, 10000, -10): the point
    /// vanity mode looks at, beside the pivot.
    pub offset_point: [f32; 3],
    pub offset_point_zooming: [f32; 3],
    /// `fOverShoulderFOV` (55): the third-person field of view
    /// (`PlayerCharacter` +0x678, `f3rdPersonFOV`, Xbox PDB +0x688, set from
    /// it at `00939197`).
    pub fov: f32,
    /// `fVanityModeWheelMin` (30), `fVanityModeWheelMax` (exe 600, data
    /// 250), `fVanityModeWheelDefault` (60, the zoom a new game starts
    /// with, `009385ad`), `fVanityModeWheelDeadMin` (75, the least zoom
    /// while dead), `fVanityModeForceDefault` (300, the least zoom of a
    /// forced temporary third person).
    pub wheel_min: f32,
    pub wheel_max: f32,
    pub wheel_default: f32,
    pub wheel_dead_min: f32,
    pub force_default: f32,
    /// `fVanityModeWheelInMult` (0.05) and `fVanityModeWheelOutMult` (0.1):
    /// the share of the zoom a wheel notch takes off or adds.
    pub wheel_in_mult: f32,
    pub wheel_out_mult: f32,
    /// `fVanityModeXMult` (30), `fVanityModeYMult` (10): degrees a second
    /// per mouse count while the view key is held.
    pub x_mult: f32,
    pub y_mult: f32,
    /// `fVanityModeDelay` (0.25): a shorter press of the view key switches
    /// the view.
    pub delay: f32,
    /// `fVanityModeAutoDelay` (exe 30, data 120), `fVanityModeAutoXSpeed`
    /// (7), `fVanityModeAutoYSpeed` (2), `fVanityModeAutoYDegrees` (22).
    pub auto_delay: f32,
    pub auto_x_speed: f32,
    pub auto_y_speed: f32,
    pub auto_y_degrees: f32,
    /// `fChaseCameraMax` (120): the farthest the camera goes without the
    /// view key held.
    pub chase_max: f32,
    /// `fChase3rdPersonZUnitsPerSecond` (exe 300, data 800): how fast the
    /// camera pulls out or in.
    pub chase_speed: f32,
    /// `fChase3rdPersonVanityXYMult` (0.5): vanity mode's camera follows
    /// this share of the gap a second.
    pub vanity_xy_mult: f32,
    /// `fFirstPersonZoomMinMult` (0.25), `fFirstPersonZoomMaxMult` (5): the
    /// switch's speed factor from the zoom.
    pub zoom_min_mult: f32,
    pub zoom_max_mult: f32,
    /// The INI settings.
    pub ini: IniCamera,
}

/// The INI settings the camera reads (`[General] fZoom3rdPersonSnapDist`,
/// `[HAVOK] fCameraCasterSize`, `fCameraCasterPlayerSize`, `[General]
/// bDisableAutoVanityMode`), with the exe's defaults.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IniCamera {
    pub snap_dist: f32,
    pub caster_size: f32,
    pub caster_player_size: f32,
    pub disable_auto_vanity: bool,
}

impl Default for IniCamera {
    fn default() -> Self {
        IniCamera {
            snap_dist: 50.0,
            caster_size: 10.0,
            caster_player_size: 1.0,
            disable_auto_vanity: false,
        }
    }
}

impl CameraSettings {
    pub fn read(order: &LoadOrder, ini: IniCamera) -> CameraSettings {
        let d = CameraSettings::defaults();
        let g = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        CameraSettings {
            pos_x: g("fOverShoulderPosX", d.pos_x),
            pos_z: g("fOverShoulderPosZ", d.pos_z),
            start_blend_dist: g("fOverShoulderStartBlendDist", d.start_blend_dist),
            rot_mult: g("fOverShoulderRotMult", d.rot_mult),
            offset_point: [
                g("fOverShoulderOffsetPointX", d.offset_point[0]),
                g("fOverShoulderOffsetPointY", d.offset_point[1]),
                g("fOverShoulderOffsetPointZ", d.offset_point[2]),
            ],
            offset_point_zooming: [
                g(
                    "fOverShoulderOffsetPointZoomingX",
                    d.offset_point_zooming[0],
                ),
                g(
                    "fOverShoulderOffsetPointZoomingY",
                    d.offset_point_zooming[1],
                ),
                g(
                    "fOverShoulderOffsetPointZoomingZ",
                    d.offset_point_zooming[2],
                ),
            ],
            fov: g("fOverShoulderFOV", d.fov),
            wheel_min: g("fVanityModeWheelMin", d.wheel_min),
            wheel_max: g("fVanityModeWheelMax", d.wheel_max),
            wheel_default: g("fVanityModeWheelDefault", d.wheel_default),
            wheel_dead_min: g("fVanityModeWheelDeadMin", d.wheel_dead_min),
            force_default: g("fVanityModeForceDefault", d.force_default),
            wheel_in_mult: g("fVanityModeWheelInMult", d.wheel_in_mult),
            wheel_out_mult: g("fVanityModeWheelOutMult", d.wheel_out_mult),
            x_mult: g("fVanityModeXMult", d.x_mult),
            y_mult: g("fVanityModeYMult", d.y_mult),
            delay: g("fVanityModeDelay", d.delay),
            auto_delay: g("fVanityModeAutoDelay", d.auto_delay),
            auto_x_speed: g("fVanityModeAutoXSpeed", d.auto_x_speed),
            auto_y_speed: g("fVanityModeAutoYSpeed", d.auto_y_speed),
            auto_y_degrees: g("fVanityModeAutoYDegrees", d.auto_y_degrees),
            chase_max: g("fChaseCameraMax", d.chase_max),
            chase_speed: g("fChase3rdPersonZUnitsPerSecond", d.chase_speed),
            vanity_xy_mult: g("fChase3rdPersonVanityXYMult", d.vanity_xy_mult),
            zoom_min_mult: g("fFirstPersonZoomMinMult", d.zoom_min_mult),
            zoom_max_mult: g("fFirstPersonZoomMaxMult", d.zoom_max_mult),
            ini,
        }
    }

    /// The exe's own defaults (the settings' static initialisers,
    /// `coverage/raw/settings_all.txt`).
    pub fn defaults() -> CameraSettings {
        CameraSettings {
            pos_x: 40.0,
            pos_z: -10.0,
            start_blend_dist: 0.0,
            rot_mult: 0.6,
            offset_point: [98.4, 16.0, -24.8],
            offset_point_zooming: [0.0, 10000.0, -10.0],
            fov: 55.0,
            wheel_min: 30.0,
            wheel_max: 600.0,
            wheel_default: 60.0,
            wheel_dead_min: 75.0,
            force_default: 300.0,
            wheel_in_mult: 0.05,
            wheel_out_mult: 0.1,
            x_mult: 30.0,
            y_mult: 10.0,
            delay: 0.25,
            auto_delay: 30.0,
            auto_x_speed: 7.0,
            auto_y_speed: 2.0,
            auto_y_degrees: 22.0,
            chase_max: 120.0,
            chase_speed: 300.0,
            vanity_xy_mult: 0.5,
            zoom_min_mult: 0.25,
            zoom_max_mult: 5.0,
            ini: IniCamera::default(),
        }
    }
}

/// What the player is doing this frame, as the camera asks it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    /// The third-person body's root (the feet), game units.
    pub feet: [f32; 3],
    /// `GetHeading` (radians, clockwise from north) and the pitch
    /// (`00931d70`, radians, positive looking down).
    pub heading: f32,
    pub pitch: f32,
    /// `fEyeHeight` (`PlayerCharacter` +0x698, Xbox PDB +0x6a8) and the
    /// scale (`GetScale`, `00567400`).
    pub eye_height: f32,
    pub scale: f32,
    /// The first-person camera node's place (`011e07d0`): the eye.
    pub eye: [f32; 3],
    /// Seconds this frame.
    pub dt: f32,
    /// Dead (`IsDead`, `PlayerCharacter` vfunc +0x22c: Xbox PDB slot
    /// `IsDead` +0x228, the PC's later slots 4 bytes on).
    pub dead: bool,
}

/// Where the camera is this frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    /// At the eye (the viewer's first-person camera, unchanged here).
    FirstPerson,
    /// Third person: the camera's place and the point it looks at (up is
    /// up, `011a9484`), and the field of view: the third-person one
    /// outside vanity mode, else the world's (`None`).
    ThirdPerson {
        position: [f32; 3],
        look_at: [f32; 3],
        fov: Option<f32>,
    },
}

/// The player's camera state: `PlayerCharacter`'s view fields and the
/// globals the camera functions share.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCamera {
    /// +0x64a `b3rdPerson`: the camera is (or is leaving) third person.
    pub third_person: bool,
    /// +0x64b `bActually3rdPerson`: the third-person body is the one shown
    /// (`00951a10` sets it).
    pub actually_third: bool,
    /// +0x64c `bWant3rdPerson`.
    pub want_third: bool,
    /// +0x64d/+0x64e `bTemp3rdPerson`, `bTemp3rdPersonSwitchBack`.
    pub temp_third: TempThirdPerson,
    /// +0x64f/+0x650 `bTemp1stPerson`, `bTemp1stPersonSwitchBack`.
    pub temp_first: bool,
    pub temp_first_back: bool,
    /// `011e0b5c`: the zoom, the wanted distance behind the pivot.
    pub zoom: f32,
    /// `011e0768`: the camera's distance from the pivot now.
    pub distance: f32,
    /// `011e07dc`: during a switch, how far the camera is from where it
    /// wants to be, a second; a switch ends when it's 30 or less.
    pub distance_speed: f32,
    /// `011e0808`: where the chase camera wants to be, smoothed in vanity
    /// mode.
    pub smoothed: [f32; 3],
    /// `011e07b8`: the view key is being held (or vanity mode has it).
    pub view_key: bool,
    /// `011e07b9`: vanity mode.
    pub vanity: bool,
    /// `011e07c1`: the view key was down this press.
    pub view_key_down: bool,
    /// `011e07c2`: the camera is held in by a wall.
    pub blocked: bool,
    /// `011e07c3`: the next chase update snaps instead of moving.
    pub snap: bool,
    /// `011e07c8`: how long the view key has been held.
    pub held: f32,
    /// `011e07c4`: how long nothing has been pressed (for vanity mode).
    pub idle: f32,
    /// `011e0bc0`: third person was shown when the view key went down.
    pub was_third: bool,
    /// `011e0bbc`: the zoom when the view key went down.
    pub zoom_at_press: f32,
    /// `011e0b60`, `011e0b58`: the turn and tilt the mouse gives the camera
    /// while the view key is held.
    pub yaw_offset: f32,
    pub pitch_offset: f32,
    /// `011e08fc`, `011e08f4`, `011e0bb8`: vanity mode's turn, tilt and the
    /// tilt's phase; `011e07bc` the zoom it started from.
    pub vanity_heading: f32,
    pub vanity_pitch: f32,
    pub vanity_phase: f32,
    pub vanity_zoom: f32,
}

impl PlayerCamera {
    /// A new game's camera (`00938180`): first person, the default zoom.
    pub fn new(s: &CameraSettings) -> PlayerCamera {
        PlayerCamera {
            third_person: false,
            actually_third: false,
            want_third: false,
            temp_third: TempThirdPerson::default(),
            temp_first: false,
            temp_first_back: false,
            zoom: s.wheel_default,
            distance: 0.0,
            distance_speed: 0.0,
            smoothed: [0.0; 3],
            view_key: false,
            vanity: false,
            view_key_down: false,
            blocked: false,
            snap: false,
            held: 0.0,
            idle: 0.0,
            was_third: false,
            zoom_at_press: 0.0,
            yaw_offset: 0.0,
            pitch_offset: 0.0,
            vanity_heading: 0.0,
            vanity_pitch: 0.0,
            vanity_phase: 0.0,
            vanity_zoom: 0.0,
        }
    }

    /// Whether `UpdateCamera` takes its third-person branch: in third
    /// person, switching, or with the view key held (`0094b0e6`..`0094b126`).
    pub fn third_person_view(&self) -> bool {
        self.third_person || self.view_key || self.want_third != self.third_person
    }

    /// Shows the first-person (`true`) or the third-person body
    /// (`00951a10`, simplified to its flags): not first person while dead
    /// or in a temporary third person.
    // Translated from 00951a10 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn show_first_person(&mut self, first: bool, dead: bool) {
        if first && (dead || self.temp_third.active) {
            return;
        }
        self.actually_third = !first;
    }

    /// First or third person (`SetFirstPerson`). True when it changes the
    /// wanted view. Going to third person shows the body at once and the
    /// chase starts from the eye; going to first person keeps the
    /// third-person view (`b3rdPerson`) until the camera has pulled in
    /// ([`Self::finish_switch`]).
    // Translated from 00950110 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn set_first_person(&mut self, first: bool, s: &CameraSettings, f: &Frame) -> bool {
        let changes = self.want_third == first;
        self.want_third = !first;
        if self.want_third && f.dead {
            self.zoom = s.wheel_dead_min;
        }
        // The first-person 3D (+0x694) is taken as loaded.
        if !self.view_key {
            if self.want_third && changes {
                self.smoothed = f.eye;
                self.show_first_person(false, f.dead);
                return changes;
            }
            // The first-person body hidden (third person shown) and going
            // to first person: the view pulls in from third person.
            if self.actually_third && !self.want_third {
                self.third_person = true;
            }
        }
        // `011f21d0 && 011f21d1` (unknown, taken as false) would end the
        // switch at once.
        changes
    }

    /// The end of a switch (`00950290`, the last call of `UpdateCamera`):
    /// once the camera is within 30 units a second of where it wants to be
    /// (the double at `0101db88`), the view is what's wanted; reaching
    /// first person shows the first-person body.
    // Translated from 00950290 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn finish_switch(&mut self, dead: bool) {
        if self.want_third != self.third_person && f64::from(self.distance_speed) <= 30.0 {
            self.third_person = self.want_third;
            if !self.view_key && !self.third_person {
                self.show_first_person(true, dead);
            }
        }
    }

    /// Begins a temporary third person (`ForceTemp3rdPerson`): with
    /// `force_zoom`, at least `fVanityModeForceDefault` away. True when it
    /// switched the view. Nothing while a temporary view is on (`005721e0`).
    // Translated from 00950340 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn force_temp_third(&mut self, force_zoom: bool, s: &CameraSettings, f: &Frame) -> bool {
        if self.temp_first || self.temp_third.active {
            return false;
        }
        if force_zoom && self.zoom < s.force_default {
            self.zoom = s.force_default;
        }
        if self.temp_third.begin(self.third_person, self.temp_first) {
            self.set_first_person(false, s, f)
        } else {
            false
        }
    }

    /// Each player update (`UpdateTemp3rdPerson`, `00941d83`): the
    /// temporary third person ends once no animation action plays, the
    /// player isn't knocked down and the view key isn't held; first person
    /// comes back if it was taken from it (then true).
    // Translated from 009503d0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update_temp_third(
        &mut self,
        action_playing: bool,
        knocked: bool,
        s: &CameraSettings,
        f: &Frame,
    ) -> bool {
        let back = self
            .temp_third
            .update(action_playing, knocked, self.view_key || self.vanity);
        if back {
            self.set_first_person(true, s, f);
        }
        back
    }

    /// Begins a temporary first person (`ForceTemp1stPerson`, called with
    /// 0 when the Pip-Boy comes up, `0070ee80`): vanity mode stops; from
    /// third person the view switches and will come back.
    // Translated from 00950460 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn force_temp_first(&mut self, s: &CameraSettings, f: &Frame) {
        self.stop_vanity();
        self.temp_first = true;
        // (Iron sights stop here, `008bb650`; not modelled.)
        if !self.third_person {
            // First person: the third-person body is hidden if shown.
            self.actually_third = false;
            return;
        }
        self.temp_first_back = true;
        self.set_first_person(true, s, f);
    }

    /// Each player update (`UpdateTemp1stPerson`): out of menu mode, the
    /// temporary first person ends, third person coming back if it was
    /// taken from it. (Its iron-sights wait, `008bbc10`, isn't modelled.)
    // Translated from 00950530 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update_temp_first(&mut self, menu_mode: bool, s: &CameraSettings, f: &Frame) {
        if !self.temp_first || menu_mode {
            return;
        }
        self.temp_first = false;
        if self.temp_first_back {
            self.set_first_person(false, s, f);
        }
        self.temp_first_back = false;
    }

    /// Stops vanity mode and the view key's hold (`StopVanityMode`): the
    /// zoom vanity mode started from comes back, and the next chase update
    /// snaps.
    // Translated from 009500a0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn stop_vanity(&mut self) {
        if self.vanity {
            self.zoom = self.vanity_zoom;
        }
        self.view_key = false;
        self.vanity = false;
        self.snap = true;
        self.view_key_down = false;
        self.held = 0.0;
        self.idle = 0.0;
        self.pitch_offset = 0.0;
        self.yaw_offset = 0.0;
        self.distance_speed = 0.0;
    }

    /// Talking to someone (`FocusOnActor`, each frame of the dialogue
    /// camera): the view key lets go, first person, the switch ending at
    /// once if the camera isn't moving, and the temporary third person's
    /// update.
    // Translated from 00953060 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn focus_on_actor(&mut self, s: &CameraSettings, f: &Frame) {
        if self.view_key {
            self.stop_vanity();
            let first = !self.third_person;
            self.show_first_person(first, f.dead);
        }
        self.set_first_person(true, s, f);
        self.finish_switch(f.dead);
        self.update_temp_third(false, false, s, f);
    }

    /// The view key (control 13) this frame: `down` while it's held (or
    /// pressed), `released` the frame it comes up. Pressing switches to
    /// third person and holds the view key; letting go within
    /// `fVanityModeDelay` without zooming switches back to first person if
    /// third person was shown before. Not in menu mode, a temporary first
    /// person, or a temporary third person (unless the key is already
    /// held). `pov_off`: `DisablePlayerControls`' POV flag (+0x680 bit
    /// 0x10, see `0095f590`). `using_pad` (`004b71d0`): a controller.
    // Translated from 00942be8..00942e05 (disassembled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    pub fn view_key_input(
        &mut self,
        down: bool,
        released: bool,
        menu_mode: bool,
        pov_off: bool,
        using_pad: bool,
        s: &CameraSettings,
        f: &Frame,
    ) {
        if menu_mode || self.temp_first || (self.temp_third.active && !self.view_key) {
            return;
        }
        if down && !pov_off {
            // `00702360()` (unknown, taken as false) and `011a3b31` (taken
            // as set) also gate this.
            self.view_key_down = true;
            if !self.view_key {
                self.was_third = self.actually_third;
                self.set_first_person(false, s, f);
                self.idle = 0.0;
                self.vanity = false;
                self.view_key = true;
                self.yaw_offset = 0.0;
                self.zoom_at_press = self.zoom;
            }
            self.held += f.dt;
            return;
        }
        if !(released || (self.view_key && !self.vanity)) {
            return;
        }
        self.idle = 0.0;
        self.vanity = false;
        self.view_key = false;
        if self.held < s.delay && (using_pad || self.zoom == self.zoom_at_press) && !pov_off {
            if self.was_third {
                let first = !self.temp_third.switch_back;
                self.set_first_person(first, s, f);
            }
            self.temp_third = TempThirdPerson::default();
        }
        self.held = 0.0;
        self.view_key_down = false;
        self.pitch_offset = 0.0;
        self.yaw_offset = 0.0;
    }

    /// The mouse wheel (`delta` in the game's units, 120 a notch, positive
    /// away from the player): in third person (or holding the view key)
    /// it zooms, a notch in taking `fVanityModeWheelInMult` of the zoom
    /// off and a notch out adding `fVanityModeWheelOutMult`; zooming in
    /// under `fVanityModeWheelMin` goes to first person; the zoom stays
    /// under `fChaseCameraMax`, or `fVanityModeWheelMax` with the view key
    /// held. In first person a notch out goes to third person.
    /// `ai_controlled` (`0093a740`): the player is under AI control.
    // Translated from 00945a17..00945c62 (disassembled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    pub fn wheel(
        &mut self,
        delta: i32,
        menu_mode: bool,
        pov_off: bool,
        ai_controlled: bool,
        s: &CameraSettings,
        f: &Frame,
    ) {
        if delta == 0 || menu_mode {
            return;
        }
        if self.view_key || self.third_person {
            let share = f64::from(delta) / f64::from(WHEEL_NOTCH) * f64::from(self.zoom);
            let mult = if delta > 0 {
                s.wheel_in_mult
            } else {
                s.wheel_out_mult
            };
            self.zoom = (f64::from(self.zoom) - f64::from(mult) * share) as f32;
            if f.dead {
                if self.zoom < s.wheel_dead_min {
                    self.zoom = s.wheel_dead_min;
                }
            } else if self.zoom < s.wheel_min {
                if !ai_controlled && !self.view_key && !self.temp_third.active && !pov_off {
                    self.set_first_person(true, s, f);
                }
                self.zoom = s.wheel_min;
            }
            let max = if self.view_key {
                s.wheel_max
            } else {
                s.chase_max
            };
            if self.zoom > max {
                self.zoom = max;
            }
        }
        if !ai_controlled && !self.temp_first && !self.third_person && delta < 0 && !pov_off {
            self.set_first_person(false, s, f);
        }
    }

    /// The mouse while the view key is held (or was this press): it turns
    /// and tilts the camera around the player instead of turning the
    /// player (`dx`, `dy` in mouse counts, `dy` positive toward the
    /// player). True when the mouse was taken (the normal look skipped).
    /// The tilt isn't taken with a controller (`004b71d0`). Process vfunc
    /// +0x610, which also gates it, is taken as 0.
    // Translated from 00945ce4..00945d92 (disassembled, FalloutNV.exe 1.4.0.525)
    pub fn look(&mut self, dx: i32, dy: i32, using_pad: bool, s: &CameraSettings, dt: f32) -> bool {
        if !(self.view_key || self.view_key_down) {
            return false;
        }
        let dt = f64::from(dt);
        let x = f64::from(dx) * DEG_TO_RAD;
        self.yaw_offset = (f64::from(self.yaw_offset) + f64::from(s.x_mult) * x * dt) as f32;
        if !using_pad {
            let y = f64::from(dy) * DEG_TO_RAD;
            self.pitch_offset =
                (f64::from(self.pitch_offset) + f64::from(s.y_mult) * y * dt) as f32;
        }
        true
    }

    /// Vanity mode, each frame: on, it turns the camera around the player
    /// and tilts it up and down, and any input (or the POV control turned
    /// off) stops it; off, after `fVanityModeAutoDelay` seconds without
    /// input (unless `bDisableAutoVanityMode`) it starts, in a temporary
    /// third person at least `fVanityModeForceDefault` away. The tilt's
    /// wave is `004e44b0` (a C runtime trig function at `00eca060`, read as
    /// the sine; not confirmed).
    // Translated from 00943483..0094362b (disassembled, FalloutNV.exe 1.4.0.525)
    pub fn vanity_update(&mut self, input: bool, pov_off: bool, s: &CameraSettings, f: &Frame) {
        if self.vanity {
            if input || pov_off {
                self.stop_vanity();
                return;
            }
            let dt = f64::from(f.dt);
            self.vanity_heading = (f64::from(self.vanity_heading)
                - f64::from(s.auto_x_speed) * DEG_TO_RAD * dt)
                as f32;
            self.vanity_phase =
                (f64::from(self.vanity_phase) + f64::from(s.auto_y_speed) * DEG_TO_RAD * dt) as f32;
            let wave = f64::from(self.vanity_phase.sin());
            self.vanity_pitch = (f64::from(s.auto_y_degrees) * wave * DEG_TO_RAD) as f32;
            if self.vanity_phase > TAU {
                self.vanity_phase -= TAU;
            }
            return;
        }
        if self.view_key {
            return;
        }
        if input {
            self.idle = 0.0;
        } else {
            self.idle += f.dt;
        }
        if s.ini.disable_auto_vanity || s.auto_delay >= self.idle || pov_off {
            return;
        }
        self.vanity_zoom = self.zoom;
        self.vanity_heading = 0.0;
        self.vanity_pitch = if self.third_person { f.pitch } else { 0.0 };
        self.force_temp_third(true, s, f);
        self.vanity = true;
        self.view_key = true;
        self.snap = true;
    }

    /// The camera this frame (`UpdateCamera`'s third-person branch, then
    /// the end of a switch). `snap`: place it at once (`UpdateCamera`'s
    /// first argument, as after loading). `cast` is the camera caster
    /// (`pCameraCaster`, +0x21c): from a point toward another, the distance
    /// to the first thing in the way.
    // Translated from 0094ae40 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(
        &mut self,
        s: &CameraSettings,
        f: &Frame,
        snap: bool,
        cast: &mut dyn FnMut([f32; 3], [f32; 3]) -> Option<f32>,
    ) -> View {
        if !self.third_person_view() {
            self.finish_switch(f.dead);
            return View::FirstPerson;
        }
        // The pivot: the body's root, raised.
        let mut pivot = f.feet;
        let (rotation, fov) = if self.vanity {
            if self.vanity_heading < 0.0 {
                self.vanity_heading += TAU;
            } else if self.vanity_heading > TAU {
                self.vanity_heading -= TAU;
            }
            pivot[2] = (f64::from(f.scale) * 100.0 + f64::from(pivot[2])) as f32;
            (
                rotation_zx(f.heading + self.vanity_heading, self.vanity_pitch),
                None,
            )
        } else {
            // `fEyeHeight` × 1 (`011a3b64`) × the scale, and the
            // difference in height of the first-person camera node and
            // `011e07d4` (two first-person nodes, not identified; taken as
            // 0 here).
            let raise = f64::from(f.eye_height) * f64::from(f.scale);
            pivot[2] = (raise + f64::from(pivot[2])) as f32;
            (
                rotation_zx(f.heading + self.yaw_offset, f.pitch + self.pitch_offset),
                Some(s.fov),
            )
        };
        // The shoulder offset, fading with the zoom when a blend distance
        // is set.
        let mut blend = 1.0f32;
        if s.start_blend_dist > 0.0 {
            blend = 1.0 - self.zoom / s.start_blend_dist;
            if blend < 0.0 {
                blend = 0.0;
            }
        }
        let shoulder = scale([s.pos_x, 0.0, s.pos_z], blend);
        let ahead = [s.pos_x, -self.zoom + 1000.0, s.pos_z];
        let switching = self.want_third != self.third_person;
        // Where the camera wants to be, and the point it looks at.
        let back = add([0.0, -self.zoom, 0.0], shoulder);
        let mut desired = add(pivot, apply(&rotation, back));
        let look_ahead = add(pivot, apply(&rotation, ahead));
        let shoulder_world = apply(&rotation, shoulder);
        let shifted = add(pivot, shoulder_world);
        let chase_pivot = [f.feet[0], f.feet[1], shifted[2]];
        self.chase(&mut desired, chase_pivot, snap, s, f, cast);
        let position = desired;
        let mut look_at = look_ahead;
        if self.vanity && blend > 0.0 {
            // Vanity mode looks at a point beside the pivot, turned with
            // the player's heading and pitch.
            let point = if self.held > 0.0 {
                let t = (self.held / s.delay).min(1.0);
                add(
                    scale(s.offset_point, t),
                    scale(s.offset_point_zooming, 1.0 - t),
                )
            } else if switching || (self.blocked && !self.view_key) {
                s.offset_point_zooming
            } else {
                s.offset_point
            };
            let turned = apply(&rotation_zx(f.heading, f.pitch), point);
            look_at = add(pivot, scale(turned, 1.0 - s.rot_mult));
        }
        self.finish_switch(f.dead);
        View::ThirdPerson {
            position,
            look_at,
            fov,
        }
    }

    /// The chase camera (`UpdateChaseCamera`): `desired` (where the camera
    /// wants to be) becomes where it is. Its distance from `pivot` moves
    /// toward the wanted one at `fChase3rdPersonZUnitsPerSecond` while
    /// switching (faster the more zoomed out), pulls in at once when
    /// something is in the way (the caster's hit, less half
    /// `fCameraCasterSize`), and otherwise eases out to it, within
    /// `fVanityModeWheelMin` and `fChaseCameraMax` (`fVanityModeWheelMax`
    /// with the view key held). Under `fZoom3rdPersonSnapDist` a switch to
    /// first person ends and one to third person jumps out to it.
    // Translated from 0094a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn chase(
        &mut self,
        desired: &mut [f32; 3],
        pivot: [f32; 3],
        snap: bool,
        s: &CameraSettings,
        f: &Frame,
        cast: &mut dyn FnMut([f32; 3], [f32; 3]) -> Option<f32>,
    ) {
        let original = *desired;
        let mut switching: i32 = i32::from(self.want_third) - i32::from(self.third_person);
        let mut mult = 1.0f32;
        let snapping = snap || self.snap;
        if switching != 0 {
            if snapping {
                switching = 0;
            } else {
                let share =
                    f64::from(self.zoom - s.wheel_min) / f64::from(s.wheel_max - s.wheel_min);
                mult = (f64::from(s.zoom_max_mult) * share + f64::from(s.zoom_min_mult)) as f32;
            }
        }
        let target = if snapping {
            *desired
        } else if self.vanity {
            let gap = sub(*desired, self.smoothed);
            let step = scale(gap, s.vanity_xy_mult * f.dt);
            self.smoothed = add(self.smoothed, step);
            self.smoothed
        } else {
            *desired
        };
        self.smoothed = target;
        let mut direction = sub(target, pivot);
        let mut wanted = unitize(&mut direction);
        self.blocked = false;
        if let Some(hit) = cast(pivot, target) {
            if wanted - hit > 2.0 {
                self.blocked = true;
            }
            wanted = (f64::from(hit) - f64::from(s.ini.caster_size) * 0.5) as f32;
        }
        let step = s.chase_speed * f.dt * mult;
        let max = if self.view_key {
            s.wheel_max
        } else {
            s.chase_max
        };
        if switching == 0 {
            if !snapping {
                if self.distance < wanted {
                    if self.distance + step <= wanted {
                        self.distance += step;
                    } else {
                        self.distance = wanted;
                    }
                } else {
                    self.distance = wanted;
                }
                if !self.blocked && self.distance < s.wheel_min {
                    self.distance = s.wheel_min;
                }
                if max < self.distance {
                    self.distance = max;
                }
            } else {
                self.distance = wanted;
                self.snap = false;
            }
        } else {
            if switching != 1 || self.distance + step <= wanted {
                self.distance += switching as f32 * step;
            } else {
                self.distance = wanted;
                self.distance_speed = 0.0;
                switching = 0;
            }
            if self.distance <= max {
                if self.distance < 0.0 {
                    self.distance = 0.0;
                    self.distance_speed = 0.0;
                    switching = 0;
                }
            } else {
                self.distance = max;
                self.distance_speed = 0.0;
                switching = 0;
            }
        }
        if switching < 0 {
            if self.distance < s.ini.snap_dist {
                self.distance = 0.0;
                self.distance_speed = 0.0;
                switching = 0;
            }
        } else if switching > 0 && self.distance < s.ini.snap_dist {
            self.distance = s.ini.snap_dist;
        }
        // (With nothing switching, the caster asks whether the camera is
        // inside the player, `006214d0`, to fade the body; not modelled.)
        let out = add(pivot, scale(direction, self.distance));
        if switching != 0 {
            let gap = length(sub(original, out));
            self.distance_speed = (f64::from(gap) / f64::from(f.dt.max(1e-6))) as f32;
            if switching == -1 && length(sub(original, f.eye)) <= gap {
                self.distance_speed = 0.0;
                self.distance = 0.0;
            }
        }
        *desired = out;
    }
}

/// `NiMatrix3` `MakeZRotation(heading)` × `MakeXRotation(pitch)`
/// (`004a0c90`, `00524ac0`, `0043f8d0`): the local axes (x right, y
/// forward, z up) turned by the heading and tilted by the pitch, as rows.
pub fn rotation_zx(heading: f32, pitch: f32) -> [[f32; 3]; 3] {
    let (sh, ch) = heading.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let z = [[ch, sh, 0.0], [-sh, ch, 0.0], [0.0, 0.0, 1.0]];
    let x = [[1.0, 0.0, 0.0], [0.0, cp, sp], [0.0, -sp, cp]];
    let mut m = [[0.0f32; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| z[i][k] * x[k][j]).sum();
        }
    }
    m
}

/// A matrix times a vector (`004b4500`).
pub fn apply(m: &[[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale(a: [f32; 3], k: f32) -> [f32; 3] {
    [a[0] * k, a[1] * k, a[2] * k]
}

fn length(a: [f32; 3]) -> f32 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

/// `NiPoint3::Unitize` (`00457910`): makes `v` one long, returns how long
/// it was (a zero vector stays zero).
fn unitize(v: &mut [f32; 3]) -> f32 {
    let l = length(*v);
    if l > 1e-6 {
        *v = scale(*v, 1.0 / l);
    } else {
        *v = [0.0; 3];
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(dt: f32) -> Frame {
        Frame {
            feet: [0.0; 3],
            heading: 0.0,
            pitch: 0.0,
            eye_height: 120.0,
            scale: 1.0,
            eye: [0.0, 0.0, 120.0],
            dt,
            dead: false,
        }
    }

    fn settings() -> CameraSettings {
        // The data's values (FalloutNV.esm): wheel max 250, speed 800,
        // blend distance 40000.
        CameraSettings {
            wheel_max: 250.0,
            chase_speed: 800.0,
            start_blend_dist: 40000.0,
            auto_delay: 120.0,
            ..CameraSettings::defaults()
        }
    }

    fn nothing(_: [f32; 3], _: [f32; 3]) -> Option<f32> {
        None
    }

    #[test]
    fn the_matrices_turn_forward_to_the_heading_and_pitch_down() {
        // Heading 90° (east), no pitch: forward (local y) is +x.
        let m = rotation_zx(std::f32::consts::FRAC_PI_2, 0.0);
        let f = apply(&m, [0.0, 1.0, 0.0]);
        assert!((f[0] - 1.0).abs() < 1e-6 && f[1].abs() < 1e-6, "{f:?}");
        // Right (local x) is then south (-y).
        let r = apply(&m, [1.0, 0.0, 0.0]);
        assert!((r[1] + 1.0).abs() < 1e-6, "{r:?}");
        // Pitch positive looks down.
        let m = rotation_zx(0.0, 0.5);
        assert!(apply(&m, [0.0, 1.0, 0.0])[2] < 0.0);
    }

    #[test]
    fn a_tap_of_the_view_key_switches_and_the_camera_pulls_out_then_in() {
        let s = settings();
        let f = frame(0.05);
        let mut c = PlayerCamera::new(&s);
        assert_eq!(c.zoom, 60.0);
        // Down then up within fVanityModeDelay: third person.
        c.view_key_input(true, false, false, false, false, &s, &f);
        assert!(c.want_third && c.actually_third && c.view_key);
        assert_eq!(c.smoothed, f.eye);
        c.view_key_input(false, true, false, false, false, &s, &f);
        assert!(c.want_third && !c.view_key && !c.third_person);
        // The camera pulls out at 800 × (0.25 + 5 × (60-30)/(250-30)) a
        // second, jumping first to fZoom3rdPersonSnapDist.
        let View::ThirdPerson { position, fov, .. } = c.update(&s, &f, false, &mut nothing) else {
            panic!("third person");
        };
        assert_eq!(fov, Some(55.0));
        assert_eq!(c.distance, 50.0);
        assert!(position[1] < 0.0, "behind: {position:?}");
        let mut frames = 0;
        while !c.third_person {
            c.update(&s, &f, false, &mut nothing);
            frames += 1;
            assert!(frames < 100);
        }
        // Out at the wanted distance: |(40, -60, -10)| (blend 1 - 60/40000).
        let blend = 1.0 - 60.0 / 40000.0;
        // The chase measures from the pivot at the shoulder's height, so
        // the shoulder's drop doesn't count.
        let wanted = (40.0f32 * blend).hypot(60.0);
        assert!((c.distance - wanted).abs() < 1e-3, "{}", c.distance);
        // A second tap: back to first person once the camera is in.
        c.view_key_input(true, false, false, false, false, &s, &f);
        assert!(c.was_third);
        c.view_key_input(false, true, false, false, false, &s, &f);
        assert!(!c.want_third && c.third_person);
        let mut frames = 0;
        while c.third_person {
            assert!(matches!(
                c.update(&s, &f, false, &mut nothing),
                View::ThirdPerson { .. }
            ));
            frames += 1;
            assert!(frames < 100);
        }
        assert!(!c.actually_third);
        assert_eq!(c.update(&s, &f, false, &mut nothing), View::FirstPerson);
    }

    #[test]
    fn holding_the_view_key_turns_the_camera_and_keeps_the_view() {
        let s = settings();
        let f = frame(0.1);
        let mut c = PlayerCamera::new(&s);
        for _ in 0..5 {
            c.view_key_input(true, false, false, false, false, &s, &f);
        }
        assert!(c.held > s.delay);
        assert!(c.look(10, 0, false, &s, f.dt));
        // 10 counts × 30°/s × 0.1 s = 30°.
        assert!((c.yaw_offset - 30f32.to_radians()).abs() < 1e-5);
        c.view_key_input(false, true, false, false, false, &s, &f);
        // Held too long to count as a tap: still third person, offsets gone.
        assert!(c.want_third);
        assert_eq!((c.yaw_offset, c.pitch_offset), (0.0, 0.0));
        assert!(!c.look(10, 0, false, &s, f.dt));
    }

    #[test]
    fn the_wheel_zooms_and_switches_at_its_ends() {
        let s = settings();
        let f = frame(0.016);
        let mut c = PlayerCamera::new(&s);
        // First person: a notch toward the player goes to third person.
        c.wheel(-WHEEL_NOTCH, false, false, false, &s, &f);
        assert!(c.want_third);
        c.third_person = true;
        // Out: 60 + 0.1 × 60 = 66; in: 66 - 0.05 × 66.
        c.wheel(-WHEEL_NOTCH, false, false, false, &s, &f);
        assert!((c.zoom - 66.0).abs() < 1e-4, "{}", c.zoom);
        c.wheel(WHEEL_NOTCH, false, false, false, &s, &f);
        assert!((c.zoom - 62.7).abs() < 1e-4, "{}", c.zoom);
        // Out past fChaseCameraMax: held there.
        for _ in 0..40 {
            c.wheel(-WHEEL_NOTCH, false, false, false, &s, &f);
        }
        assert_eq!(c.zoom, 120.0);
        // In past fVanityModeWheelMin: first person.
        for _ in 0..40 {
            c.wheel(WHEEL_NOTCH, false, false, false, &s, &f);
        }
        assert_eq!(c.zoom, 30.0);
        assert!(!c.want_third);
        // Not with POV switching turned off, nor in menus.
        let mut c = PlayerCamera::new(&s);
        c.wheel(-WHEEL_NOTCH, false, true, false, &s, &f);
        c.wheel(-WHEEL_NOTCH, true, false, false, &s, &f);
        assert!(!c.want_third);
    }

    #[test]
    fn walls_pull_the_camera_in_at_once_and_it_eases_back_out() {
        let s = settings();
        let f = frame(0.01);
        let mut c = PlayerCamera::new(&s);
        c.want_third = true;
        c.third_person = true;
        c.actually_third = true;
        c.distance = 60.0;
        let wall = |_: [f32; 3], _: [f32; 3]| Some(40.0);
        c.update(&s, &f, false, &mut { wall });
        // 40 less half fCameraCasterSize (10), and blocked, so not raised
        // to fVanityModeWheelMin.
        assert!((c.distance - 35.0).abs() < 1e-4, "{}", c.distance);
        assert!(c.blocked);
        // Clear: out again at 800 a second.
        c.update(&s, &f, false, &mut nothing);
        assert!((c.distance - 43.0).abs() < 1e-3, "{}", c.distance);
    }

    #[test]
    fn furniture_gives_a_temporary_third_person_that_switches_back() {
        let s = settings();
        let f = frame(0.016);
        let mut c = PlayerCamera::new(&s);
        assert!(c.force_temp_third(true, &s, &f));
        assert!(c.want_third && c.temp_third.active && c.temp_third.switch_back);
        assert_eq!(c.zoom, 300.0);
        // The view key is ignored while it lasts.
        c.view_key_input(true, false, false, false, false, &s, &f);
        assert!(!c.view_key);
        // Still playing: on. Done: first person wanted again.
        c.update_temp_third(true, false, &s, &f);
        assert!(c.temp_third.active);
        c.update_temp_third(false, false, &s, &f);
        assert!(!c.temp_third.active && !c.want_third);
    }

    #[test]
    fn idling_starts_vanity_mode_and_input_stops_it() {
        let s = settings();
        let f = frame(1.0);
        let mut c = PlayerCamera::new(&s);
        for _ in 0..120 {
            c.vanity_update(false, false, &s, &f);
        }
        assert!(!c.vanity);
        c.vanity_update(false, false, &s, &f);
        assert!(c.vanity && c.view_key && c.want_third && c.temp_third.active);
        assert_eq!(c.vanity_zoom, 60.0);
        assert_eq!(c.zoom, 300.0);
        c.vanity_update(false, false, &s, &f);
        assert!((c.vanity_heading + 7f32.to_radians()).abs() < 1e-5);
        c.vanity_update(true, false, &s, &f);
        assert!(!c.vanity && !c.view_key && c.snap);
        assert_eq!(c.zoom, 60.0);
    }

    #[test]
    fn talking_ends_third_person_at_once() {
        let s = settings();
        let f = frame(0.016);
        let mut c = PlayerCamera::new(&s);
        c.want_third = true;
        c.third_person = true;
        c.actually_third = true;
        c.focus_on_actor(&s, &f);
        assert!(!c.want_third && !c.third_person && !c.actually_third);
    }

    #[test]
    fn the_pip_boy_takes_first_person_and_gives_it_back() {
        let s = settings();
        let f = frame(0.016);
        let mut c = PlayerCamera::new(&s);
        c.want_third = true;
        c.third_person = true;
        c.actually_third = true;
        c.force_temp_first(&s, &f);
        assert!(!c.want_third && c.temp_first);
        c.update_temp_first(true, &s, &f);
        assert!(c.temp_first);
        c.update_temp_first(false, &s, &f);
        assert!(c.want_third && !c.temp_first);
    }
}
