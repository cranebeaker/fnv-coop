//! Faces that move: lip sync, blinking and the morphs that carry them out,
//! as the game's `BSFaceGenAnimationData` does it (read from its code; the
//! functions are named by address so each fact can be checked again).
//!
//! - Each head part's `.tri` (`nif::Tri`) holds named morphs. A name means
//!   something only if it is one of the engine's channels ([`channel`]):
//!   16 phonemes, 15 expressions, 17 modifiers (blinks, brows, looks,
//!   squints, head turns) and one custom morph. Others are never used.
//! - Each actor has one set of channel weights ([`FaceAnimation`]); every
//!   frame each part adds `weight × morph` for the morphs it has
//!   ([`FaceMorphs::apply`]), weights only counting when `0 < w ≤ 1`.
//! - A spoken line's `.lip` file ([`crate::lip::Lip`]) becomes a queue of
//!   keys, one every 1/30 s, and the voice is held back so the two line up
//!   ([`FaceAnimation::speak`]).
//! - When no modifier keys are waiting, a blink is queued: a pause of
//!   1.5–4 s, then the lids close and open ([`FaceAnimation::update`]).
//! - Without a `.lip` file the mouth doesn't move at all: the game has no
//!   fallback driven by the sound.
//! - While the actor has someone to look at (its head-track target), the
//!   eyes dart: every so often they settle on a small random offset from
//!   straight ahead that depends on the face's strongest emotion, turning
//!   there at `fTrackSpeed` ([`FaceAnimation::track_eyes`], `0064be40`).
//!   They don't aim at the target itself; the head does that
//!   (`crate::look_ik`).

use std::collections::VecDeque;

use esm::LoadOrder;

use crate::lip::{Lip, FRAMES_PER_SECOND};
use crate::scripting::game_setting;

pub const PHONEME_COUNT: usize = 16;
pub const EXPRESSION_COUNT: usize = 15;
pub const MODIFIER_COUNT: usize = 17;
pub const CUSTOM_COUNT: usize = 1;

/// The engine's phoneme channels, in its order (table `0119b4a0`; a
/// `.lip` frame's first 16 weights). Note `Eee`: every `.tri` in the game
/// names that morph `Ee`, which matches nothing, so it never moves.
pub const PHONEMES: [&str; PHONEME_COUNT] = [
    "Aah", "BigAah", "BMP", "ChJSh", "DST", "Eee", "Eh", "FV", "I", "K", "N", "Oh", "OohQ", "R",
    "Th", "W",
];

/// The engine's expression channels (table `0119b41c`).
pub const EXPRESSIONS: [&str; EXPRESSION_COUNT] = [
    "Anger",
    "Fear",
    "Happy",
    "Sad",
    "Surprise",
    "MoodNeutral",
    "MoodAfraid",
    "MoodAnnoyed",
    "MoodCocky",
    "MoodDrugged",
    "MoodPleasant",
    "MoodAngry",
    "MoodSad",
    "Pained",
    "CombatAnger",
];

/// The engine's modifier channels (table `0119b458`; a `.lip` frame's
/// last 17 weights). The last three turn the head instead of morphing it.
pub const MODIFIERS: [&str; MODIFIER_COUNT] = [
    "BlinkLeft",
    "BlinkRight",
    "BrowDownLeft",
    "BrowDownRight",
    "BrowInLeft",
    "BrowInRight",
    "BrowUpLeft",
    "BrowUpRight",
    "LookDown",
    "LookLeft",
    "LookRight",
    "LookUp",
    "SquintLeft",
    "SquintRight",
    "HeadPitch",
    "HeadRoll",
    "HeadYaw",
];

/// The one custom channel (table `0119b49c`).
pub const CUSTOM: [&str; CUSTOM_COUNT] = ["VampireMorph"];

/// Modifier channel numbers.
pub mod modifier {
    pub const BLINK_LEFT: usize = 0;
    pub const BLINK_RIGHT: usize = 1;
    pub const LOOK_DOWN: usize = 8;
    pub const LOOK_LEFT: usize = 9;
    pub const LOOK_RIGHT: usize = 10;
    pub const LOOK_UP: usize = 11;
    /// The first of the head turns (pitch, roll, yaw), which aren't morphs.
    pub const HEAD_PITCH: usize = 14;
}

/// The kinds of channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    Phoneme,
    Expression,
    Modifier,
    Custom,
}

/// One channel: its group and number in that group's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Channel {
    pub group: Group,
    pub index: usize,
}

impl Channel {
    pub fn name(self) -> &'static str {
        match self.group {
            Group::Phoneme => PHONEMES[self.index],
            Group::Expression => EXPRESSIONS[self.index],
            Group::Modifier => MODIFIERS[self.index],
            Group::Custom => CUSTOM[self.index],
        }
    }
}

/// The channel a morph's name drives (`00660ba0`): the whole name, case
/// ignored, tried against the expressions, then the modifiers, the
/// phonemes and the custom morph. The head turns (modifiers 14–16) are
/// never morphs (`00662ca0` skips them), so they don't match here.
pub fn channel(name: &str) -> Option<Channel> {
    let find = |table: &[&str], group: Group| {
        table
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .map(|index| Channel { group, index })
    };
    find(&EXPRESSIONS, Group::Expression)
        .or_else(|| find(&MODIFIERS[..modifier::HEAD_PITCH], Group::Modifier))
        .or_else(|| find(&PHONEMES, Group::Phoneme))
        .or_else(|| find(&CUSTOM, Group::Custom))
}

/// Every channel's weight at one moment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weights {
    pub phonemes: [f32; PHONEME_COUNT],
    pub expressions: [f32; EXPRESSION_COUNT],
    pub modifiers: [f32; MODIFIER_COUNT],
    pub custom: [f32; CUSTOM_COUNT],
}

impl Weights {
    /// A face at rest.
    pub const NEUTRAL: Weights = Weights {
        phonemes: [0.0; PHONEME_COUNT],
        expressions: [0.0; EXPRESSION_COUNT],
        modifiers: [0.0; MODIFIER_COUNT],
        custom: [0.0; CUSTOM_COUNT],
    };

    pub fn get(&self, channel: Channel) -> f32 {
        match channel.group {
            Group::Phoneme => self.phonemes[channel.index],
            Group::Expression => self.expressions[channel.index],
            Group::Modifier => self.modifiers[channel.index],
            Group::Custom => self.custom[channel.index],
        }
    }

    /// The weights a face this far away shows: groups out of `reach` count
    /// as 0, which the morphs skip as the game skips those groups.
    pub fn within(&self, reach: Reach) -> Weights {
        let mut w = *self;
        if !reach.phonemes {
            w.phonemes = [0.0; PHONEME_COUNT];
        }
        if !reach.others {
            w.expressions = [0.0; EXPRESSION_COUNT];
            w.modifiers = [0.0; MODIFIER_COUNT];
            w.custom = [0.0; CUSTOM_COUNT];
        }
        w
    }

    /// The weights the morphs take: opposite looks cancel first
    /// (`00653d70`): looking down counts `max(0, down − up)` and up
    /// `max(0, up − down)`; the same for left and right.
    fn for_morphs(&self) -> Weights {
        let mut w = *self;
        let m = &self.modifiers;
        for (a, b) in [
            (modifier::LOOK_DOWN, modifier::LOOK_UP),
            (modifier::LOOK_LEFT, modifier::LOOK_RIGHT),
        ] {
            w.modifiers[a] = (m[a] - m[b]).max(0.0);
            w.modifiers[b] = (m[b] - m[a]).max(0.0);
        }
        w
    }
}

impl Default for Weights {
    fn default() -> Self {
        Weights::NEUTRAL
    }
}

/// Which channels a face this far from the camera shows (`00663050`):
/// phonemes within `fTalkingDistance` + 10 units, everything else only
/// within `fLodDistance` + 10 as well. (Something not traced can make both
/// distances a quarter as long.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reach {
    pub phonemes: bool,
    pub others: bool,
}

impl Reach {
    pub const ALL: Reach = Reach {
        phonemes: true,
        others: true,
    };

    pub fn at(distance: f32, settings: &FaceSettings) -> Reach {
        let phonemes = distance <= settings.talking_distance + 10.0;
        Reach {
            phonemes,
            others: phonemes && distance <= settings.lod_distance + 10.0,
        }
    }
}

/// One mesh's morphs, ready to apply: per morph, its channel and how far
/// each vertex it moves goes at full strength.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FaceMorphs {
    pub morphs: Vec<Morph>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Morph {
    pub channel: Channel,
    /// (vertex, offset at weight 1), model space.
    pub moves: Vec<(u32, [f32; 3])>,
}

/// An NPC's FaceGen face, for reshaping a part's statistical targets the
/// way its vertices were (see [`FaceMorphs::build`]).
#[derive(Debug, Clone, Copy)]
pub struct Reshape<'a> {
    pub egm: &'a nif::Egm,
    pub symmetric: &'a [f32],
    pub asymmetric: &'a [f32],
}

impl FaceMorphs {
    /// A mesh's morphs from its part's `.tri`, for the mesh's vertices
    /// `positions` (the model's, already shaped by the NPC's face).
    ///
    /// - A differential morph moves the first min(mesh, morph) vertices by
    ///   their offsets (`00660860`); the offsets are kept as they are.
    /// - A statistical morph moves vertex `i` by `w × (T − B[i])`, `T` its
    ///   target and `B[i]` the mesh's vertex (`00660a90`), so at full
    ///   strength the vertex lands on `T`.
    /// - A name that is both: the statistical one is used (`00660ba0` logs
    ///   "Only statistical will be used"). A name twice in the same kind:
    ///   the first is used (not traced).
    ///
    /// With a `reshape`, the targets are moved by the NPC's face as well:
    /// the `.egm`'s rows after the model's own vertices are the `.tri`'s
    /// targets in file order (its vertex count is exactly base + targets in
    /// every head part, and the engine keeps one array of [mesh vertices,
    /// then targets] per part, `006624c0`), so a blink still closes the lid
    /// on a reshaped face. **Inferred**: the code applying the `.egm` to
    /// that array wasn't traced. Files whose counts don't add up keep their
    /// targets as stored.
    pub fn build(tri: &nif::Tri, positions: &[[f32; 3]], reshape: Option<Reshape>) -> FaceMorphs {
        let mut morphs: Vec<Morph> = Vec::new();
        let taken = |morphs: &[Morph], c: Channel| morphs.iter().any(|m| m.channel == c);
        let reshape = reshape.filter(|r| r.egm.vertices == tri.base.len() + tri.target_count());
        for m in &tri.statistical {
            let Some(c) = channel(&m.name) else { continue };
            if taken(&morphs, c) {
                continue;
            }
            let mut targets = m.targets.clone();
            if let Some(r) = reshape {
                r.egm.apply_rows(
                    tri.base.len() + m.first_target,
                    &mut targets,
                    r.symmetric,
                    r.asymmetric,
                );
            }
            let moves = m
                .vertices
                .iter()
                .zip(&targets)
                .filter_map(|(&i, t)| {
                    let b = positions.get(i as usize)?;
                    Some((i, [t[0] - b[0], t[1] - b[1], t[2] - b[2]]))
                })
                .collect();
            morphs.push(Morph { channel: c, moves });
        }
        for m in &tri.differential {
            let Some(c) = channel(&m.name) else { continue };
            if taken(&morphs, c) {
                continue;
            }
            let moves = m
                .offsets
                .iter()
                .take(positions.len())
                .enumerate()
                .filter(|(_, d)| d.iter().any(|&v| v != 0.0))
                .map(|(i, &d)| (i as u32, d))
                .collect::<Vec<_>>();
            morphs.push(Morph { channel: c, moves });
        }
        morphs.retain(|m| !m.moves.is_empty());
        FaceMorphs { morphs }
    }

    pub fn is_empty(&self) -> bool {
        self.morphs.is_empty()
    }

    /// Moves `positions` (the mesh's vertices at rest) by the weights
    /// (see [`Weights::within`] for faces far away): each morph whose
    /// weight is above 0 and at most 1 adds weight × its offsets; other
    /// weights are skipped, not clamped (`00662e90`, `00662ca0`,
    /// `00662bc0`, `00662f70`).
    pub fn apply(&self, weights: &Weights, positions: &mut [[f32; 3]]) {
        let weights = weights.for_morphs();
        for m in &self.morphs {
            let w = weights.get(m.channel);
            if !(w > 0.0 && w <= 1.0) {
                continue;
            }
            for &(i, d) in &m.moves {
                if let Some(p) = positions.get_mut(i as usize) {
                    for k in 0..3 {
                        p[k] += w * d[k];
                    }
                }
            }
        }
    }
}

/// The names in a `.tri` that no channel answers to (never used by the
/// game: `Ee`, `Cocky`, `Disgust`, the hair's `HairMorph`...).
pub fn unused_morphs(tri: &nif::Tri) -> Vec<&str> {
    tri.differential
        .iter()
        .map(|m| m.name.as_str())
        .chain(tri.statistical.iter().map(|m| m.name.as_str()))
        .filter(|n| channel(n).is_none())
        .collect()
}

/// The settings faces follow: game settings (`GMST`, the exe's defaults
/// where `FalloutNV.esm` has none) and the INI's.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaceSettings {
    /// `fBlinkDownTime` (0.04 in `FalloutNV.esm`), `fBlinkUpTime` (0.14):
    /// seconds to close and to open.
    pub blink_down: f32,
    pub blink_up: f32,
    /// `fBlinkDelayMin` 1.5, `fBlinkDelayMax` 4: seconds between blinks.
    pub blink_delay_min: f32,
    pub blink_delay_max: f32,
    /// `fLookDownDisableBlinkingAmt` 0.25: no blinks while looking down
    /// more than this.
    pub look_down_no_blinks: f32,
    /// `fSpeechDelay` (0.17): the voice starts this much after the lead-in.
    pub speech_delay: f32,
    /// `[LOD] fTalkingDistance` (1000 here) and `fLodDistance` (500): see
    /// [`Reach`].
    pub talking_distance: f32,
    pub lod_distance: f32,
    /// `[Audio] fDialogueHead{Pitch,Roll,Yaw}Exaggeration` (2): what a
    /// `.lip`'s head turns are multiplied by as it loads.
    pub head_exaggeration: [f32; 3],
    /// How the eyes dart ([`EyeSettings`]).
    pub eyes: EyeSettings,
}

/// The eyes' settings (`0064be40`, `0064bf90`, `0064c410`), radians where
/// angles; the game settings hold the ranges in degrees and the offsets in
/// radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EyeSettings {
    /// `fTrackSpeed` (2): the most the eyes turn in a second, each way.
    pub speed: f32,
    /// `fTrackEyeXY` (28°) and `fTrackEyeZ` (20°): how far they turn left
    /// or right and up or down, which is full weight on the look morphs;
    /// each kept within 0–90° (`00649f00`, `00649f70`).
    pub heading_range: f32,
    pub pitch_range: f32,
    /// `fEye{Heading,Pitch}{Min,Max}OffsetEmotion{Angry,Happy,Sad,Fear,
    /// Neutral}`, by [`EyeMood`]: (heading min, max, pitch min, max).
    pub offsets: [[f32; 4]; 5],
}

/// Which emotion's eye offsets a face uses (`0064bf90`'s jump tables
/// `0064c3fc`/`0064c3e4`, indexed by the strongest expression + 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EyeMood {
    Angry = 0,
    Happy = 1,
    Sad = 2,
    Fear = 3,
    Neutral = 4,
}

impl EyeMood {
    /// The offsets used for the strongest expression (`None` when no
    /// expression is above 0): none, Anger, MoodCocky and MoodAngry use
    /// the angry ones; Fear and MoodAfraid fear; Happy and Surprise happy;
    /// Sad, MoodDrugged and MoodSad sad; MoodNeutral neutral. The others
    /// (MoodAnnoyed, MoodPleasant, Pained, CombatAnger) leave the offset as
    /// it was.
    pub fn of(strongest: Option<usize>) -> Option<EyeMood> {
        match strongest {
            None | Some(0) | Some(8) | Some(11) => Some(EyeMood::Angry),
            Some(1) | Some(6) => Some(EyeMood::Fear),
            Some(2) | Some(4) => Some(EyeMood::Happy),
            Some(3) | Some(9) | Some(12) => Some(EyeMood::Sad),
            Some(5) => Some(EyeMood::Neutral),
            _ => None,
        }
    }
}

impl EyeSettings {
    /// The exe's defaults (`00f8xxxx` static initialisers).
    pub const DEFAULT: EyeSettings = EyeSettings {
        speed: 2.0,
        heading_range: (28.0 * DEGREE) as f32,
        pitch_range: (20.0 * DEGREE) as f32,
        offsets: [
            [-0.1, 0.1, -0.05, 0.15],
            [-0.2, 0.2, -0.05, 0.15],
            [-0.4, 0.4, -0.15, -0.05],
            [-0.5, 0.5, 0.0, 0.0],
            [-0.2, 0.2, -0.05, 0.15],
        ],
    };

    fn read(gmst: impl Fn(&str, f32) -> f32) -> EyeSettings {
        let d = EyeSettings::DEFAULT;
        // 00649f00/00649f70: degrees x 0.0174533 (a double), below 0 is 0,
        // above pi/2 is pi/2.
        let range = |name: &str, default: f32| {
            ((gmst(name, default) as f64 * DEGREE) as f32).clamp(0.0, std::f32::consts::FRAC_PI_2)
        };
        let mut offsets = d.offsets;
        for (o, mood) in offsets
            .iter_mut()
            .zip(["Angry", "Happy", "Sad", "Fear", "Neutral"])
        {
            let names = ["HeadingMin", "HeadingMax", "PitchMin", "PitchMax"].map(|k| {
                let (what, end) = k.split_at(if k.starts_with("Heading") { 7 } else { 5 });
                format!("fEye{what}{end}OffsetEmotion{mood}")
            });
            for (v, name) in o.iter_mut().zip(&names) {
                *v = gmst(name, *v);
            }
        }
        EyeSettings {
            speed: gmst("fTrackSpeed", d.speed),
            heading_range: range("fTrackEyeXY", 28.0),
            pitch_range: range("fTrackEyeZ", 20.0),
            offsets,
        }
    }
}

impl Default for EyeSettings {
    fn default() -> Self {
        EyeSettings::DEFAULT
    }
}

/// 01023128: degrees to radians, the double holding f32(pi / 180).
const DEGREE: f64 = 0.017_453_292_384_743_69;

/// The look morphs for eye angles (`0064c410`, translated from 0064c410,
/// decompiled, FalloutNV.exe 1.4.0.525): heading and pitch kept within
/// their ranges (`005330e0`), then LookLeft (9) or LookRight (10) =
/// heading / range on its side, LookDown (8) or LookUp (11) = pitch /
/// range on its side, the other of each pair 0. Returns the weights for
/// modifiers 8..=11 and the kept angles (what it stores at +0x140/+0x144).
/// The game skips all of it while the FaceGen manager's byte `011d59e0`
/// is clear (taken as set).
pub fn eye_morphs(heading: f32, pitch: f32, s: &EyeSettings) -> ([f32; 4], f32, f32) {
    // 005330e0: not `v <= hi` (a NaN too) gives hi.
    let clamp = |v: f32, lo: f32, hi: f32| {
        if v <= hi {
            if v < lo {
                lo
            } else {
                v
            }
        } else {
            hi
        }
    };
    let h = clamp(heading, -s.heading_range, s.heading_range);
    let p = clamp(pitch, -s.pitch_range, s.pitch_range);
    let (left, right) = if h < 0.0 {
        (h / -s.heading_range, 0.0)
    } else {
        (0.0, h / s.heading_range)
    };
    let (down, up) = if p < 0.0 {
        (p / -s.pitch_range, 0.0)
    } else {
        (0.0, p / s.pitch_range)
    };
    ([down, left, right, up], h, p)
}

impl FaceSettings {
    /// The exe's own defaults.
    pub const DEFAULT: FaceSettings = FaceSettings {
        blink_down: 0.2,
        blink_up: 0.17,
        blink_delay_min: 1.5,
        blink_delay_max: 4.0,
        look_down_no_blinks: 0.25,
        speech_delay: 0.0,
        talking_distance: 2000.0,
        lod_distance: 500.0,
        head_exaggeration: [2.0; 3],
        eyes: EyeSettings::DEFAULT,
    };

    /// The settings in force: the load order's game settings, `ini(section,
    /// key)` for the INI's, defaults for the rest.
    pub fn read(order: &LoadOrder, ini: impl Fn(&str, &str) -> Option<f32>) -> FaceSettings {
        let d = FaceSettings::DEFAULT;
        let gmst = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        let exaggeration = |axis: &str, default: f32| {
            ini("Audio", &format!("fDialogueHead{axis}Exaggeration")).unwrap_or(default)
        };
        FaceSettings {
            blink_down: gmst("fBlinkDownTime", d.blink_down),
            blink_up: gmst("fBlinkUpTime", d.blink_up),
            blink_delay_min: gmst("fBlinkDelayMin", d.blink_delay_min),
            blink_delay_max: gmst("fBlinkDelayMax", d.blink_delay_max),
            look_down_no_blinks: gmst("fLookDownDisableBlinkingAmt", d.look_down_no_blinks),
            speech_delay: gmst("fSpeechDelay", d.speech_delay),
            talking_distance: ini("LOD", "fTalkingDistance").unwrap_or(d.talking_distance),
            lod_distance: ini("LOD", "fLodDistance").unwrap_or(d.lod_distance),
            head_exaggeration: [
                exaggeration("Pitch", d.head_exaggeration[0]),
                exaggeration("Roll", d.head_exaggeration[1]),
                exaggeration("Yaw", d.head_exaggeration[2]),
            ],
            eyes: EyeSettings::read(gmst),
        }
    }

    /// Blinks happen only with all three times above 0 and the delays in
    /// order (`0064b630`).
    fn blinks(&self) -> bool {
        self.blink_down > 0.0
            && self.blink_up > 0.0
            && self.blink_delay_min > 0.0
            && self.blink_delay_min <= self.blink_delay_max
    }
}

impl Default for FaceSettings {
    fn default() -> Self {
        FaceSettings::DEFAULT
    }
}

/// A key value meaning "leave this channel as it is" (`FLT_MAX`).
const NO_VALUE: f32 = f32::MAX;

/// How long the face takes to settle after a line's last frame (`+0xf4`
/// with 0.2 s, `008a20d0`).
const SETTLE: f32 = 0.2;

/// The longest lead-in eased over before a line, and the longest the voice
/// can be held back (the sound system caps its delay at 10 s, `00adb4e0`).
const MAX_EASE: f32 = 0.2;
const MAX_VOICE_DELAY: f32 = 10.0;

#[derive(Debug, Clone, PartialEq)]
struct Key<const N: usize> {
    values: [f32; N],
    duration: f32,
}

/// One group's keys waiting, its values now and its clock
/// (`BSFaceGenKeyframeMultiple`).
#[derive(Debug, Clone, PartialEq)]
struct Track<const N: usize> {
    queue: VecDeque<Key<N>>,
    current: [f32; N],
    clock: f32,
}

impl<const N: usize> Track<N> {
    fn new() -> Self {
        Track {
            queue: VecDeque::new(),
            current: [0.0; N],
            clock: 0.0,
        }
    }

    fn push(&mut self, values: [f32; N], duration: f32) {
        if duration >= 0.0 {
            self.queue.push_back(Key { values, duration });
        }
    }

    /// Clears the keys and eases back to rest over `ease` seconds (at once
    /// when it's 0) (`0064a520`).
    fn reset(&mut self, ease: f32) {
        self.queue.clear();
        self.clock = 0.0;
        if ease > 0.0 {
            self.push([0.0; N], ease);
        } else {
            self.current = [0.0; N];
        }
    }

    /// Moves on by `dt` seconds (`0064ca60`): keys whose time has come are
    /// taken whole, in order, the rest of the time carrying over; then the
    /// values move toward the next key by the share of it gone by. True
    /// when a value changed.
    fn step(&mut self, dt: f32) -> bool {
        if self.queue.is_empty() {
            self.clock = 0.0;
            return false;
        }
        let before = self.current;
        self.clock += dt;
        while let Some(key) = self.queue.front() {
            if key.duration > self.clock {
                break;
            }
            let key = self.queue.pop_front().expect("a key is waiting");
            blend(&mut self.current, &key.values, 1.0);
            self.clock -= key.duration;
        }
        match self.queue.front() {
            Some(key) => blend(&mut self.current, &key.values, self.clock / key.duration),
            None => self.clock = 0.0,
        }
        self.current != before
    }
}

/// Blends the values toward a key (`0064eea0`): `current × (1 − t) + key ×
/// t` from wherever they are now (not from where the key began, so above
/// 30 frames a second the motion is a little front-loaded, as in the
/// game); a key's "no value" leaves a channel alone, and a current "no
/// value" jumps to the key.
fn blend<const N: usize>(current: &mut [f32; N], key: &[f32; N], t: f32) {
    let t = t.clamp(0.0, 1.0);
    for (c, &k) in current.iter_mut().zip(key) {
        if k >= NO_VALUE {
            continue;
        }
        *c = if t >= 1.0 || *c >= NO_VALUE {
            k
        } else {
            k * t + (1.0 - t) * *c
        };
    }
}

/// One face's moving channels: what it's saying, when it blinks and where
/// its eyes are (`BSFaceGenAnimationData`). Expressions (moods, a line's
/// emotion) aren't driven yet, so they stay at rest.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceAnimation {
    phonemes: Track<PHONEME_COUNT>,
    modifiers: Track<MODIFIER_COUNT>,
    custom: Track<CUSTOM_COUNT>,
    eyes: Eyes,
    /// Its own dice (xorshift), so each face blinks on its own schedule
    /// and pictures repeat.
    dice: u64,
}

/// When a line's voice starts, and how long the line lasts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineTiming {
    /// Seconds before the voice starts: the lead-in + `fSpeechDelay`
    /// (`008a20d0`; at most 10, the sound system's cap).
    pub voice_delay: f32,
    /// The line's length as the game counts it: lead-in + `fSpeechDelay` +
    /// frames / 30.
    pub length: f32,
}

/// A line with lip sync: its voice is held back by the lead-in and
/// `fSpeechDelay`, whoever says it. (A line without one plays at once.)
pub fn line_timing(lip: &Lip, settings: &FaceSettings) -> LineTiming {
    let delay = lip.lead_in() + settings.speech_delay;
    LineTiming {
        voice_delay: delay.clamp(0.0, MAX_VOICE_DELAY),
        length: delay + lip.length(),
    }
}

impl FaceAnimation {
    /// A face at rest; `seed` picks its blinks (the reference's form ID
    /// does).
    pub fn new(seed: u64) -> FaceAnimation {
        // Stirred (splitmix64), so neighbouring form IDs blink apart.
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        FaceAnimation {
            phonemes: Track::new(),
            modifiers: Track::new(),
            custom: Track::new(),
            eyes: Eyes::default(),
            dice: z ^ (z >> 31),
        }
    }

    /// A line with lip sync begins (`008a20d0`, `004d5930`):
    /// 1. lead = `−offset / 30`; the phoneme, modifier and custom channels
    ///    drop their keys (a blink under way too) and ease back to rest over
    ///    min(0.2, lead) seconds; expressions are left alone;
    /// 2. each frame becomes a phoneme key and a modifier key of 1/30 s
    ///    (a phoneme key with any weight outside 0..1 is dropped whole,
    ///    `0064aa00`; the head turns are multiplied by the exaggerations);
    /// 3. then both ease to rest over 0.2 s;
    /// 4. the voice starts lead + `fSpeechDelay` seconds in (at most 10).
    ///
    /// So frame `k` is reached at `ease + (k + 1) / 30`, about a frame after
    /// its sound plays at `lead + fSpeechDelay + (k + offset) / 30`.
    pub fn speak(&mut self, lip: &Lip, settings: &FaceSettings) -> LineTiming {
        let lead = lip.lead_in();
        let ease = lead.min(MAX_EASE);
        self.phonemes.reset(ease);
        self.modifiers.reset(ease);
        self.custom.reset(ease);
        let frame = 1.0 / FRAMES_PER_SECOND;
        for f in &lip.frames {
            if f.phonemes.iter().all(|v| (0.0..=1.0).contains(v)) {
                self.phonemes.push(f.phonemes, frame);
            }
            let mut m = f.modifiers;
            for (v, e) in m[modifier::HEAD_PITCH..]
                .iter_mut()
                .zip(settings.head_exaggeration)
            {
                *v *= e;
            }
            self.modifiers.push(m, frame);
        }
        self.phonemes.push([0.0; PHONEME_COUNT], SETTLE);
        self.modifiers.push([0.0; MODIFIER_COUNT], SETTLE);
        line_timing(lip, settings)
    }

    /// A line cut short (skipped, or the talk ended): its keys are dropped
    /// and the face eases back to rest over 0.2 s, as when a line starts.
    /// **A guess**: what the game does to a face whose line is skipped
    /// isn't traced (the next line's start resets it this way).
    pub fn hush(&mut self) {
        self.phonemes.reset(MAX_EASE);
        self.modifiers.reset(MAX_EASE);
        self.custom.reset(MAX_EASE);
    }

    /// Moves on by `dt` seconds (`0064b630`). First, when no modifier keys
    /// are waiting (so never during a line, whose own blink channels blink
    /// for it), the blink settings allow it and the face looks down less
    /// than `fLookDownDisableBlinkingAmt`, a blink is queued: both lids
    /// held open for a random 1.5–4 s, closed over `fBlinkDownTime`, opened
    /// over `fBlinkUpTime`, the other modifiers left alone. Then each
    /// group's keys play. True when a weight changed.
    pub fn update(&mut self, dt: f32, settings: &FaceSettings) -> bool {
        let looking_down = self.modifiers.current[modifier::LOOK_DOWN];
        if self.modifiers.queue.is_empty()
            && settings.blinks()
            && looking_down < settings.look_down_no_blinks
        {
            let wait = self.random() * (settings.blink_delay_max - settings.blink_delay_min)
                + settings.blink_delay_min;
            let mut key = [NO_VALUE; MODIFIER_COUNT];
            for (lids, duration) in [
                (0.0, wait),
                (1.0, settings.blink_down),
                (0.0, settings.blink_up),
            ] {
                key[modifier::BLINK_LEFT] = lids;
                key[modifier::BLINK_RIGHT] = lids;
                self.modifiers.push(key, duration);
            }
        }
        if dt <= 0.0 {
            return false;
        }
        let a = self.phonemes.step(dt);
        let b = self.modifiers.step(dt);
        let c = self.custom.step(dt);
        a || b || c
    }

    /// One eye-tracking update (`0064be40`), run while the actor has a
    /// head-track target (`00663510`); `dt` seconds:
    ///
    /// 1. The darting timer runs down; at 0 the face's strongest expression
    ///    picks how they dart next ([`EyeMood::of`]; `0064bf90`): a new
    ///    timer and a random (heading, pitch) offset within that mood's
    ///    ranges, or a look straight ahead (offset 0): angry 30% for 2–3 s,
    ///    else 0.5–1.5 s; happy and neutral 30% for 3–4 s, else 0.5–1.5 s;
    ///    sad always 2–3 s, 30% straight; fear 0.5–1.5 s, no pitch, 50%
    ///    no heading.
    /// 2. The eyes turn toward `desired` + the offset, at most
    ///    `fTrackSpeed` × dt each way. `desired` (`+0x150`/`+0x154`) is the
    ///    look controller's eye heading and pitch while its look is on,
    ///    else (0, 0): the actor update `00888970` stores them each update
    ///    through `00888a20` (`crate::look_ik::LookIk::desired_eyes`).
    /// 3. Kept within the ranges, they set the look morphs (`0064c410`):
    ///    LookLeft or LookRight = heading ÷ range, LookDown or LookUp =
    ///    pitch ÷ range.
    ///
    /// True when a weight changed.
    pub fn track_eyes(&mut self, dt: f32, desired: (f32, f32), settings: &FaceSettings) -> bool {
        let strongest = strongest_expression(&[0.0; EXPRESSION_COUNT]);
        let dice = &mut self.dice;
        self.eyes.dart(
            EyeMood::of(strongest),
            !matches!(strongest, Some(e) if e > 12),
            desired,
            dt,
            &settings.eyes,
            &mut || unit(dice),
        );
        let ([down, left, right, up], h, p) =
            eye_morphs(self.eyes.heading, self.eyes.pitch, &settings.eyes);
        self.eyes.heading = h;
        self.eyes.pitch = p;
        let m = &mut self.modifiers.current;
        let before = *m;
        // A range of 0 (only from a GMST of 0, none ships) divides 0 by 0
        // natively, a NaN weight; here the weight is left at 0 instead.
        let ok = |v: f32| if v.is_finite() { v } else { 0.0 };
        m[modifier::LOOK_DOWN] = ok(down);
        m[modifier::LOOK_LEFT] = ok(left);
        m[modifier::LOOK_RIGHT] = ok(right);
        m[modifier::LOOK_UP] = ok(up);
        *m != before
    }

    /// The weights now. Nothing sets channels directly yet (the game lets
    /// scripts and moods do), so they are the keys' values.
    pub fn weights(&self) -> Weights {
        Weights {
            phonemes: self.phonemes.current,
            expressions: [0.0; EXPRESSION_COUNT],
            modifiers: self.modifiers.current,
            custom: self.custom.current,
        }
    }

    /// Lip sync keys are still waiting.
    pub fn speaking(&self) -> bool {
        !self.phonemes.queue.is_empty()
    }

    /// A uniform number in 0..1 from the face's own dice.
    fn random(&mut self) -> f32 {
        unit(&mut self.dice)
    }
}

/// Where the eyes are and where they dart (`BSFaceGenAnimationData`
/// `+0x140`/`+0x144` the angles, `+0x17c` the timer, `+0x180`/`+0x184` the
/// offset). The target they are offset from (`+0x150`/`+0x154`) is passed
/// in each update: the look controller's eye angles (`00888970`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Eyes {
    heading: f32,
    pitch: f32,
    timer: f32,
    offset: (f32, f32),
}

impl Eyes {
    /// Steps 1 and 2 of [`FaceAnimation::track_eyes`]; `in_table` is
    /// whether the strongest expression is one the jump table covers
    /// (none to MoodSad); the others leave the timer and offset alone.
    fn dart(
        &mut self,
        mood: Option<EyeMood>,
        in_table: bool,
        desired: (f32, f32),
        dt: f32,
        s: &EyeSettings,
        random: &mut impl FnMut() -> f32,
    ) {
        self.timer -= dt;
        if self.timer <= 0.0 && in_table {
            if let Some(mood) = mood {
                self.pick(mood, s, random);
            }
        }
        let step = s.speed * dt;
        let toward = |from: f32, to: f32| from + (to - from).clamp(-step, step);
        // 0064be40: the change, not the angle, is limited here; 0064c410
        // keeps the angles within their ranges ([`eye_morphs`]).
        self.heading = toward(self.heading, desired.0 + self.offset.0);
        self.pitch = toward(self.pitch, desired.1 + self.offset.1);
    }

    /// A new timer and offset for `mood` (`0064bf90`).
    fn pick(&mut self, mood: EyeMood, s: &EyeSettings, random: &mut impl FnMut() -> f32) {
        let mut between = |a: f32, b: f32| a + (b - a) * random();
        let [h0, h1, p0, p1] = s.offsets[mood as usize];
        match mood {
            EyeMood::Angry | EyeMood::Happy | EyeMood::Neutral => {
                let (still, quick) = (0.3 >= between(0.0, 1.0), (0.5, 1.5));
                if still {
                    let long = if mood == EyeMood::Angry {
                        (2.0, 3.0)
                    } else {
                        (3.0, 4.0)
                    };
                    self.timer = between(long.0, long.1);
                    self.offset = (0.0, 0.0);
                } else {
                    self.timer = between(quick.0, quick.1);
                    let pitch = between(p0, p1);
                    self.offset = (between(h0, h1), pitch);
                }
            }
            EyeMood::Sad => {
                self.timer = between(2.0, 3.0);
                if 0.3 >= between(0.0, 1.0) {
                    self.offset = (0.0, 0.0);
                } else {
                    let pitch = between(p0, p1);
                    self.offset = (between(h0, h1), pitch);
                }
            }
            EyeMood::Fear => {
                self.timer = between(0.5, 1.5);
                let heading = if 0.5 >= between(0.0, 1.0) {
                    0.0
                } else {
                    between(h0, h1)
                };
                self.offset = (heading, 0.0);
            }
        }
    }
}

/// A uniform number in 0..1 from a face's dice (xorshift).
fn unit(dice: &mut u64) -> f32 {
    let mut x = (*dice).max(1);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *dice = x;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

/// The strongest expression above 0 and at most 1 (`0064bda0`), the first
/// on a tie; `None` when none is.
fn strongest_expression(weights: &[f32; EXPRESSION_COUNT]) -> Option<usize> {
    let mut best: Option<(usize, f32)> = None;
    for (i, &w) in weights.iter().enumerate() {
        if w > 0.0 && w <= 1.0 && w > best.map_or(0.0, |b| b.1) {
            best = Some((i, w));
        }
    }
    best.map(|b| b.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lip::LipFrame;

    fn lip(offset: i32, frames: usize, phoneme: usize) -> Lip {
        Lip {
            offset,
            frames: (0..frames)
                .map(|k| {
                    let mut f = LipFrame {
                        phonemes: [0.0; PHONEME_COUNT],
                        modifiers: [0.0; MODIFIER_COUNT],
                    };
                    f.phonemes[phoneme] = (k + 1) as f32 / frames as f32;
                    f
                })
                .collect(),
        }
    }

    #[test]
    fn morph_names_match_the_engines_channels() {
        assert_eq!(
            channel("bigaah"),
            Some(Channel {
                group: Group::Phoneme,
                index: 1
            })
        );
        assert_eq!(
            channel("BlinkRight").map(|c| c.group),
            Some(Group::Modifier)
        );
        assert_eq!(
            channel("MoodCocky").map(|c| c.group),
            Some(Group::Expression)
        );
        assert_eq!(
            channel("VampireMorph").map(|c| c.group),
            Some(Group::Custom)
        );
        // The game's own files use names the engine doesn't know.
        for never in [
            "Ee",
            "Cocky",
            "Disgust",
            "EyeSquintLeft",
            "HairMorph",
            "HeadYaw",
        ] {
            assert_eq!(channel(never), None, "{never}");
        }
    }

    fn tri() -> nif::Tri {
        nif::Tri {
            base: vec![[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            differential: vec![
                nif::tri::DifferentialMorph {
                    name: "Aah".into(),
                    offsets: vec![[0.0, 0.0, -2.0], [0.0; 3], [0.0; 3]],
                },
                nif::tri::DifferentialMorph {
                    name: "BlinkLeft".into(),
                    offsets: vec![[5.0; 3]; 3],
                },
                nif::tri::DifferentialMorph {
                    name: "Ee".into(),
                    offsets: vec![[1.0; 3]; 3],
                },
            ],
            statistical: vec![nif::tri::StatisticalMorph {
                name: "BlinkLeft".into(),
                vertices: vec![2],
                targets: vec![[0.0, 1.0, -1.0]],
                first_target: 0,
            }],
        }
    }

    #[test]
    fn morphs_add_their_offsets_by_weight() {
        // The mesh's own vertices differ a little from the `.tri`'s base:
        // a statistical morph still lands exactly on its target.
        let mesh = [[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.2, 0.0]];
        let morphs = FaceMorphs::build(&tri(), &mesh, None);
        // `Ee` matches nothing; `BlinkLeft` is statistical only.
        assert_eq!(morphs.morphs.len(), 2);
        let mut w = Weights::NEUTRAL;
        w.phonemes[0] = 0.5;
        w.modifiers[modifier::BLINK_LEFT] = 1.0;
        let mut p = mesh;
        morphs.apply(&w, &mut p);
        assert_eq!(p[0], [0.0, 0.0, -1.0]);
        assert_eq!(p[2], [0.0, 1.0, -1.0]);
        // Out of reach of everything but phonemes: no blink.
        let mut p = mesh;
        let near = Reach {
            phonemes: true,
            others: false,
        };
        morphs.apply(&w.within(near), &mut p);
        assert_eq!(p[0], [0.0, 0.0, -1.0]);
        assert_eq!(p[2], mesh[2]);
        // Weights above 1 (a `.lip`'s 1.0485) are skipped, not clamped.
        w.phonemes[0] = 1.05;
        let mut p = mesh;
        morphs.apply(&w, &mut p);
        assert_eq!(p[0], mesh[0]);
    }

    #[test]
    fn the_faces_shape_moves_the_targets_too() {
        // Three vertices + one target: the `.egm`'s fourth row moves the
        // target, as its first three move the mesh.
        let mut bytes = b"FREGM002".to_vec();
        for v in [4u32, 1, 0] {
            bytes.extend(v.to_le_bytes());
        }
        bytes.extend([0u8; 44]);
        bytes.extend(1.0f32.to_le_bytes());
        for row in [[0i16, 0, 0], [0, 0, 0], [0, 0, 3], [0, 0, 3]] {
            for c in row {
                bytes.extend(c.to_le_bytes());
            }
        }
        let egm = nif::Egm::parse(&bytes).unwrap();
        let mut mesh = [[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        egm.apply(&mut mesh, &[1.0], &[]);
        let reshape = Reshape {
            egm: &egm,
            symmetric: &[1.0],
            asymmetric: &[],
        };
        let morphs = FaceMorphs::build(&tri(), &mesh, Some(reshape));
        let mut w = Weights::NEUTRAL;
        w.modifiers[modifier::BLINK_LEFT] = 1.0;
        let mut p = mesh;
        morphs.apply(&w, &mut p);
        // The lid closes on the reshaped face: target −1 raised by 3.
        assert_eq!(p[2], [0.0, 1.0, 2.0]);
    }

    #[test]
    fn opposite_looks_cancel() {
        let mut w = Weights::NEUTRAL;
        w.modifiers[modifier::LOOK_DOWN] = 0.75;
        w.modifiers[modifier::LOOK_UP] = 0.25;
        let m = w.for_morphs();
        assert_eq!(m.modifiers[modifier::LOOK_DOWN], 0.5);
        assert_eq!(m.modifiers[modifier::LOOK_UP], 0.0);
    }

    #[test]
    fn the_strongest_expression_picks_the_eye_mood() {
        assert_eq!(EyeMood::of(None), Some(EyeMood::Angry));
        assert_eq!(EyeMood::of(Some(11)), Some(EyeMood::Angry));
        assert_eq!(EyeMood::of(Some(6)), Some(EyeMood::Fear));
        assert_eq!(EyeMood::of(Some(4)), Some(EyeMood::Happy));
        assert_eq!(EyeMood::of(Some(9)), Some(EyeMood::Sad));
        assert_eq!(EyeMood::of(Some(5)), Some(EyeMood::Neutral));
        assert_eq!(EyeMood::of(Some(7)), None);
        assert_eq!(EyeMood::of(Some(14)), None);
        let mut w = [0.0; EXPRESSION_COUNT];
        assert_eq!(strongest_expression(&w), None);
        w[3] = 0.4;
        w[5] = 0.6;
        w[8] = 1.5;
        assert_eq!(strongest_expression(&w), Some(5));
    }

    #[test]
    fn eye_settings_read_the_emotion_offsets_by_name() {
        let s = EyeSettings::read(|name, default| match name {
            "fEyeHeadingMaxOffsetEmotionSad" => 0.7,
            "fEyePitchMinOffsetEmotionFear" => -0.1,
            "fTrackEyeXY" => 120.0,
            _ => default,
        });
        assert_eq!(s.offsets[EyeMood::Sad as usize][1], 0.7);
        assert_eq!(s.offsets[EyeMood::Fear as usize][2], -0.1);
        // Kept within 90°.
        assert_eq!(s.heading_range, std::f32::consts::FRAC_PI_2);
        assert_eq!(EyeSettings::read(|_, d| d), EyeSettings::DEFAULT);
    }

    #[test]
    fn the_eyes_turn_to_their_offset_at_track_speed_and_set_the_look_morphs() {
        let s = EyeSettings::DEFAULT;
        let mut eyes = Eyes {
            timer: 10.0,
            offset: (0.6, -0.15),
            ..Eyes::default()
        };
        let mut never = || 0.5;
        // 2 rad/s: 0.1 s turns them 0.2 each way, pitch reaching −0.15.
        eyes.dart(Some(EyeMood::Angry), true, (0.0, 0.0), 0.1, &s, &mut never);
        assert!((eyes.heading - 0.2).abs() < 1e-6 && (eyes.pitch + 0.15).abs() < 1e-6);
        // Then the heading reaches its target; the morphs keep it within
        // the 28° range.
        eyes.dart(Some(EyeMood::Angry), true, (0.0, 0.0), 1.0, &s, &mut never);
        assert!((eyes.heading - 0.6).abs() < 1e-6);

        let mut face = FaceAnimation::new(3);
        face.eyes = eyes;
        face.eyes.timer = 10.0;
        assert!(face.track_eyes(0.0, (0.0, 0.0), &FaceSettings::DEFAULT));
        assert!((face.eyes.heading - 28f32.to_radians()).abs() < 1e-6);
        let m = face.weights().modifiers;
        assert!((m[modifier::LOOK_RIGHT] - 1.0).abs() < 1e-5);
        assert_eq!(m[modifier::LOOK_LEFT], 0.0);
        assert!((m[modifier::LOOK_DOWN] - 0.15 / 20f32.to_radians()).abs() < 1e-5);
        assert_eq!(m[modifier::LOOK_UP], 0.0);
    }

    #[test]
    fn darting_picks_timers_and_offsets_by_mood() {
        let s = EyeSettings::DEFAULT;
        // Each draw 0.5: angry doesn't hold still (0.3 < 0.5): 1 s, offset
        // in the middle of its ranges.
        let mut eyes = Eyes::default();
        eyes.pick(EyeMood::Angry, &s, &mut || 0.5);
        assert_eq!(eyes.timer, 1.0);
        assert!(eyes.offset.0.abs() < 1e-6 && (eyes.offset.1 - 0.05).abs() < 1e-6);
        // Draws of 0.1 hold happy still for 3.1 s.
        eyes.pick(EyeMood::Happy, &s, &mut || 0.1);
        assert!((eyes.timer - 3.1).abs() < 1e-6);
        assert_eq!(eyes.offset, (0.0, 0.0));
        // Fear never looks up or down; at 0.5 it looks straight (50%).
        eyes.pick(EyeMood::Fear, &s, &mut || 0.5);
        assert_eq!(eyes.offset, (0.0, 0.0));
        eyes.pick(EyeMood::Fear, &s, &mut || 0.9);
        assert!((eyes.offset.0 - 0.4).abs() < 1e-6 && eyes.offset.1 == 0.0);
        // Sad always waits 2–3 s.
        eyes.pick(EyeMood::Sad, &s, &mut || 0.9);
        assert!((eyes.timer - 2.9).abs() < 1e-6);
        // A mood outside the table leaves timer and offset alone.
        let before = eyes;
        eyes.timer = 0.0;
        eyes.dart(None, false, (0.0, 0.0), 0.0, &s, &mut || 0.0);
        assert_eq!(eyes.offset, before.offset);
    }

    /// nv-call, 0064c410 and 00649f00/00649f70 on FalloutNV.exe 1.4.0.525
    /// (AMD Ryzen AI 9 HX 370, 2026-10-06), with `fTrackEyeXY` 28 and
    /// `fTrackEyeZ` 20 and the manager's byte set: the weights of
    /// modifiers 8..=11 and the kept angles, bit for bit (tier A).
    #[test]
    fn eye_morphs_match_the_native_vectors() {
        let s = EyeSettings::DEFAULT;
        assert_eq!(s.heading_range.to_bits(), 0x3efa_35dd);
        assert_eq!(s.pitch_range.to_bits(), 0x3eb2_b8c2);
        let cases: [(f32, f32, [u32; 4], [u32; 2]); 5] = [
            (
                0.2,
                -0.1,
                [0x3e92_ad5d, 0, 0x3ed1_89f2, 0],
                [0x3e4c_cccd, 0xbdcc_cccd],
            ),
            (
                -0.3,
                0.25,
                [0, 0x3f1d_2776, 0, 0x3f37_58b4],
                [0xbe99_999a, 0x3e80_0000],
            ),
            (
                0.9,
                -0.6,
                [0x3f80_0000, 0, 0x3f80_0000, 0],
                [0x3efa_35dd, 0xbeb2_b8c2],
            ),
            (0.0, 0.0, [0; 4], [0, 0]),
            (
                -0.05,
                0.4,
                [0, 0x3dd1_89f2, 0, 0x3f80_0000],
                [0xbd4c_cccd, 0x3eb2_b8c2],
            ),
        ];
        for (h, p, want, kept) in cases {
            let (got, kh, kp) = eye_morphs(h, p, &s);
            assert_eq!(got.map(f32::to_bits), want, "{h} {p}: {got:?}");
            assert_eq!([kh.to_bits(), kp.to_bits()], kept, "{h} {p}");
        }
    }

    #[test]
    fn the_eyes_follow_the_look_controllers_angles_plus_the_dart() {
        // 0064be40: target = desired (the controller's eye heading/pitch,
        // 00888970) + the dart offset.
        let s = EyeSettings::DEFAULT;
        let mut eyes = Eyes {
            timer: 10.0,
            offset: (0.05, 0.0),
            ..Eyes::default()
        };
        for _ in 0..10 {
            eyes.dart(None, false, (-0.25, 0.1), 0.1, &s, &mut || 0.5);
        }
        assert!((eyes.heading + 0.2).abs() < 1e-6, "{eyes:?}");
        assert!((eyes.pitch - 0.1).abs() < 1e-6, "{eyes:?}");
    }

    #[test]
    fn faces_this_far_show_these_channels() {
        let s = FaceSettings {
            talking_distance: 1000.0,
            lod_distance: 500.0,
            ..FaceSettings::DEFAULT
        };
        assert_eq!(Reach::at(400.0, &s), Reach::ALL);
        let mid = Reach::at(800.0, &s);
        assert!(mid.phonemes && !mid.others);
        assert!(!Reach::at(1011.0, &s).phonemes);
    }

    fn quiet() -> FaceSettings {
        // No blinks, so only the line moves the face.
        FaceSettings {
            blink_down: 0.0,
            speech_delay: 0.17,
            ..FaceSettings::DEFAULT
        }
    }

    #[test]
    fn a_line_plays_a_frame_every_thirtieth_of_a_second() {
        let mut face = FaceAnimation::new(1);
        let line = lip(-5, 30, 15);
        let timing = face.speak(&line, &quiet());
        // Lead-in 5/30 s, eased over all of it (under 0.2 s); the voice
        // waits for it and `fSpeechDelay`.
        assert!((timing.voice_delay - (5.0 / 30.0 + 0.17)).abs() < 1e-6);
        assert!((timing.length - (5.0 / 30.0 + 0.17 + 1.0)).abs() < 1e-6);
        let step = 1.0 / 30.0;
        // At 30 frames a second each frame lands exactly: frame k at
        // ease + (k + 1) / 30.
        let mut t = 0.0;
        let mut seen = Vec::new();
        for _ in 0..40 {
            face.update(step, &quiet());
            t += step;
            seen.push((t, face.weights().phonemes[15]));
        }
        let at = |time: f32| {
            seen.iter()
                .min_by(|a, b| (a.0 - time).abs().total_cmp(&(b.0 - time).abs()))
                .unwrap()
                .1
        };
        assert!(at(5.0 / 30.0).abs() < 1e-5, "eased to rest first");
        assert!((at(5.0 / 30.0 + 1.0 / 30.0) - 1.0 / 30.0).abs() < 1e-4);
        assert!((at(5.0 / 30.0 + 10.0 / 30.0) - 10.0 / 30.0).abs() < 1e-4);
        assert!(face.speaking());
    }

    #[test]
    fn halfway_through_a_key_is_halfway_there() {
        let mut face = FaceAnimation::new(1);
        face.speak(&lip(0, 2, 0), &quiet());
        // No lead-in: no easing key, so the first frame (0.5) starts now.
        face.update(0.5 / 30.0, &quiet());
        assert!((face.weights().phonemes[0] - 0.25).abs() < 1e-5);
        face.update(0.5 / 30.0, &quiet());
        assert!((face.weights().phonemes[0] - 0.5).abs() < 1e-5);
        // After the last frame the face settles over 0.2 s.
        face.update(1.0 / 30.0 + 0.25, &quiet());
        assert_eq!(face.weights().phonemes[0], 0.0);
        assert!(!face.speaking());
    }

    #[test]
    fn phoneme_frames_out_of_range_are_dropped_whole() {
        let mut bad = lip(0, 3, 0);
        bad.frames[0].phonemes[2] = 1.5;
        let mut face = FaceAnimation::new(1);
        face.speak(&bad, &quiet());
        // Frame 1 (2/3) now comes first.
        face.update(1.0 / 30.0, &quiet());
        assert!((face.weights().phonemes[0] - 2.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn head_turns_are_exaggerated_as_they_load() {
        let mut line = lip(0, 1, 0);
        line.frames[0].modifiers[modifier::HEAD_PITCH] = -0.05;
        let mut face = FaceAnimation::new(1);
        face.speak(&line, &quiet());
        face.update(1.0 / 30.0, &quiet());
        let pitch = face.weights().modifiers[modifier::HEAD_PITCH];
        assert!((pitch + 0.1).abs() < 1e-6, "{pitch}");
    }

    #[test]
    fn faces_blink_every_one_and_a_half_to_four_seconds() {
        let s = FaceSettings {
            blink_down: 0.04,
            blink_up: 0.14,
            ..FaceSettings::DEFAULT
        };
        let mut face = FaceAnimation::new(0x0010_4C0C);
        let dt = 1.0 / 120.0;
        // Closing takes 0.04 s, so between frames the lids start opening
        // before they're seen fully shut (in the game too): "closed" is
        // past 0.9.
        let mut closed_at = Vec::new();
        let mut was_closed = false;
        for i in 0..(120 * 60) {
            face.update(dt, &s);
            let w = face.weights().modifiers;
            assert_eq!(w[modifier::BLINK_LEFT], w[modifier::BLINK_RIGHT]);
            let closed = w[modifier::BLINK_LEFT] > 0.9;
            if closed && !was_closed {
                closed_at.push(i as f32 * dt);
            }
            was_closed = closed;
        }
        assert!(closed_at.len() >= 60 / 4 - 1, "{closed_at:?}");
        for pair in closed_at.windows(2) {
            let gap = pair[1] - pair[0];
            // A pause of 1.5–4 s, plus closing and opening (0.18 s).
            assert!(gap > 1.5 + 0.16 && gap < 4.0 + 0.2, "{gap}");
        }
        // The same face blinks the same way every time; another doesn't.
        let first_blink = |seed: u64| {
            let mut face = FaceAnimation::new(seed);
            (0..120 * 10).find_map(|i| {
                face.update(dt, &s);
                (face.weights().modifiers[0] > 0.9).then_some(i as f32 * dt)
            })
        };
        assert_eq!(first_blink(0x0010_4C0C), closed_at.first().copied());
        assert_ne!(first_blink(0x0010_4C0C), first_blink(0x0010_4C0D));
    }

    #[test]
    fn no_blinks_of_its_own_during_a_line_or_looking_down() {
        let s = FaceSettings::DEFAULT;
        let mut face = FaceAnimation::new(7);
        // Two seconds of a line whose own blink channels stay open.
        face.speak(&lip(0, 60, 0), &s);
        for _ in 0..60 {
            face.update(1.0 / 30.0, &s);
            assert_eq!(face.weights().modifiers[modifier::BLINK_LEFT], 0.0);
        }
        // Looking down past `fLookDownDisableBlinkingAmt`: none queued.
        let mut down = FaceAnimation::new(7);
        down.modifiers.current[modifier::LOOK_DOWN] = 0.3;
        down.update(1.0 / 30.0, &s);
        assert!(down.modifiers.queue.is_empty());
        down.modifiers.current[modifier::LOOK_DOWN] = 0.2;
        down.update(1.0 / 30.0, &s);
        assert_eq!(down.modifiers.queue.len(), 3);
    }
}
