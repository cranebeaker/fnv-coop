//! People talking without the player starting it, read from
//! `FalloutNV.exe` with Ghidra (`%USERPROFILE%\nv-re\findings\
//! ai_rules.md` §5b and §6):
//!
//! - Greetings ([`Social::greeting`], [`HelloCooldown`]): someone free to
//!   talk who notices the player within `fAIMinGreetingDistance` (150)
//!   says a line from `HELLO`, no dialogue menu, and turns to them; then not
//!   again for `fAIGreetingTimer` (20 s) after the line, and nobody else
//!   greets the player for `fHelloCooldownTime` (30 s).
//! - Idle chatter ([`Social::chatter_due`]): a line from `IdleChatter`
//!   every 15–60 s (the data's `fIdleChatterCommentTimer…`), only when the
//!   player isn't near enough to be greeted.
//! - Conversations between two people ([`Social::start_conversation`],
//!   [`conversation`]): within 200, when their timer (30–60 s) has run out,
//!   a 25 % chance each; the lines are worked out at the start from
//!   `HELLO`, following each line's topics at random, who speaks next by the
//!   line's own rule.
//!
//! Saying the lines (the voice, the face) is the viewer's.

use esm::{FormId, LoadOrder};

use crate::dialogue::{topic_lines, Info, Speaker};
use crate::scripting::{game_setting, Facts, GameState};

/// The default topics of kind 1 (conversation), by the game's table
/// (`0061a2d0(1, n)`, tables `0119a41c`/`0119a43c`): fixed forms in every
/// game.
pub mod topics {
    use esm::FormId;
    /// 0: `HELLO` ("Hello").
    pub const HELLO: FormId = FormId(0xD2);
    /// 1: `ANY`.
    pub const ANY: FormId = FormId(0xD3);
    /// 2: `GOODBYE` ("Goodbye.").
    pub const GOODBYE: FormId = FormId(0xD4);
    /// 3: `IdleChatter`.
    pub const IDLE_CHATTER: FormId = FormId(0xD5);
}

/// The settings greetings and conversations read: the data's (`GMST`), else
/// the exe's defaults.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialSettings {
    /// `fAIMinGreetingDistance` (150).
    pub greeting_distance: f32,
    /// `fAIGreetingTimer` (20 s).
    pub greeting_timer: f32,
    /// `fHelloCooldownTime` (30 s; setting `011d03a0`, initialiser
    /// `00f65740`; not in the data).
    pub hello_cooldown: f32,
    /// `fIdleChatterCommentTimer` and `…Max` (exe 5 and 30, data 15 and 60).
    pub chatter: (f32, f32),
    /// `fAISocialTimerForConversationsMin` and `…Max` (exe 5 and 30, data 30
    /// and 60).
    pub conversation_timer: (f32, f32),
    /// `fAItalktosameNPCtimer` (120 s).
    pub same_npc: f32,
    /// `fAISocialRadiusToTriggerConversation` (2000) and `…Interior` (exe
    /// 500, data 1000).
    pub radius: f32,
    pub radius_interior: f32,
    /// `fAISocialchanceForConversation` (exe 60, data 25) and `…Interior`
    /// (25).
    pub chance: f32,
    pub chance_interior: f32,
}

impl SocialSettings {
    pub fn read(order: &LoadOrder) -> SocialSettings {
        let g = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        SocialSettings {
            greeting_distance: g("fAIMinGreetingDistance", 150.0),
            greeting_timer: g("fAIGreetingTimer", 20.0),
            hello_cooldown: g("fHelloCooldownTime", 30.0),
            chatter: (
                g("fIdleChatterCommentTimer", 5.0),
                g("fIdleChatterCommentTimerMax", 30.0),
            ),
            conversation_timer: (
                g("fAISocialTimerForConversationsMin", 5.0),
                g("fAISocialTimerForConversationsMax", 30.0),
            ),
            same_npc: g("fAItalktosameNPCtimer", 120.0),
            radius: g("fAISocialRadiusToTriggerConversation", 2000.0),
            radius_interior: g("fAISocialRadiusToTriggerConversationInterior", 500.0),
            chance: g("fAISocialchanceForConversation", 60.0),
            chance_interior: g("fAISocialchanceForConversationInterior", 25.0),
        }
    }

    /// How near two people must be to start talking (`00904800`): 200,
    /// unless the place's social radius (outdoors' or indoors') is below
    /// 200, when it's the outdoor setting's value (as the code reads it).
    pub fn trigger_distance(&self, interior: bool) -> f32 {
        let place = if interior {
            self.radius_interior
        } else {
            self.radius
        };
        if place < 200.0 {
            self.radius
        } else {
            200.0
        }
    }

    /// The chance in 100 of starting a conversation (`00904800`).
    pub fn chance(&self, interior: bool) -> f32 {
        if interior {
            self.chance_interior
        } else {
            self.chance
        }
    }
}

/// A random number between two (`006465f0`), from `unit` in 0..1.
fn between((lo, hi): (f32, f32), unit: f32) -> f32 {
    lo + (hi - lo) * unit
}

/// What [`Social::greeting`] decided for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Greeting {
    /// Busy: no greeting and no idle chatter.
    Busy,
    /// Near the player: no idle chatter, no greeting now.
    Near,
    /// Greet the player (`HELLO`), if the player's [`HelloCooldown`] is
    /// free.
    Greet,
    /// The player isn't near: the idle chatter timer runs.
    Away,
}

/// The facts `008eeec0` asks before a greeting (process = their high
/// process; the player's are the `PlayerCharacter`'s). Names marked
/// (Xbox PDB) are the prototype's; the PC offsets are its less 0x10 where
/// `TESForm` comes first (the PC's has no editor ID string).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GreetingCheck {
    /// `PlayerCharacter::IsImportantConversationRunning` (Xbox PDB,
    /// `00969860`): the player's running AI conversation (+0x224,
    /// `pAIConversationRunning`, a `DialoguePackage`) has someone talking
    /// in it (its +0x94, `pTalkingActor`). Busy for everyone, and nobody
    /// starts a conversation with another (`008efa4e`).
    pub important_conversation: bool,
    /// In combat (actor +0x104, `00493bb0`).
    pub in_combat: bool,
    /// Fleeing (`008a6650(0)`: a flee package, type 22 or 10, or a low
    /// combat package fleeing).
    pub fleeing: bool,
    /// Unconscious (life state 3, `00437bd0`).
    pub unconscious: bool,
    /// Knocked down (actor +0x230 → process +0x40c knock state, `00884560`).
    pub knocked: bool,
    /// A voice of theirs playing (process +0x48c slot 0, the sound handle
    /// +0x314 that saying a line keeps and `00934250` stops; `00ad8ce0`).
    pub speaking: bool,
    /// Their GREET procedure saying a line (process +0x30c, byte +0x32c,
    /// set by `008bc3d0` and cleared when no line is found or the speech
    /// stops).
    pub greeting_line: bool,
    /// Someone saying a GREET line to the player (player +0x6cc,
    /// `008defe0`: set by `008dd880` as `008dbe30` begins a line with the
    /// player as listener, cleared by `00953ce0` when it ends, the speech
    /// is stopped or the dialogue menu closes).
    pub player_spoken_to: bool,
    /// How well they detect the player (`008a0d10`).
    pub detection: i32,
    /// Asleep (actor +0x1ac, the process's furniture state copy, 9;
    /// `00579670`).
    pub asleep: bool,
    /// The player sneaking and not swimming (move flags 0x400 without
    /// 0x800, `004997b0`).
    pub player_sneaking: bool,
    /// The player trespassing (player +0x1c0, `008d1e70` through vfunc
    /// +0x448; set from `00546da0` by `008d2a40`).
    pub player_trespassing: bool,
    /// Their target is the player: the actor's `+0x2c8` (`008815a0`) gives
    /// the process's `pTarget` (+0x40, Xbox PDB; `GetTarget` `+0x128`,
    /// `008d6f30`), which a package sets as it starts (`0090a1a0`:
    /// `006780e0` → `SetTarget` `+0x12c`); in combat it can give the combat
    /// target instead, but people in combat are busy anyway.
    pub target_is_player: bool,
    /// Running a run-once package (`IsRunningRunOnce` (Xbox PDB), process
    /// `+0x35c` `008d8310`: `GetRunOncePackage` `+0x20c` not null, the
    /// `RunOncePackage` at +0xe4). With the player their target, only then
    /// are they near.
    pub running_run_once: bool,
    /// Someone asked them to look at someone (`CanSetActionHeadTrackTarget`
    /// (Xbox PDB), process `+0x66c` `00901460`, false: a flag of head-track
    /// slots 1–5, `HeadTrackingTargetFlags` +0x411…+0x415, set;
    /// `world::head_track::HeadTrack::action_free`). Not near then.
    pub head_track_asked: bool,
    /// Their package goes on only because the player is near
    /// (`ContinuingPackageforPC` (Xbox PDB), process `+0x4e0` `008d9050`:
    /// byte +0x374, set by `SetContinuingPackage` `+0x4e4` in `0090a1a0`
    /// when a package with general flag 0x200, "continue if PC near"
    /// (`008840f0`), would have given way). No greeting then.
    pub continuing_for_player: bool,
    /// The distance between them and the player (`005723b0`).
    pub distance: f32,
    /// The player in combat (player +0xdf0 `bPlayerInCombat` (Xbox PDB),
    /// `00953c50`).
    pub player_in_combat: bool,
    /// Their package forbids hellos ([`package_forbids_hellos`],
    /// `008a78f0(0)`).
    pub hellos_forbidden: bool,
    /// Their package is an alarm package (type 21, `008a61b0`).
    pub alarm_package: bool,
    /// In a made conversation package (type 0x1c, `009336c0`) and not
    /// moving (move flags & 0xf clear, `004938e0`).
    pub still_in_made_dialogue: bool,
}

impl GreetingCheck {
    /// The first gate (`008ef5cc`…`008ef6c0`): busy people don't greet or
    /// chatter.
    pub fn busy(&self) -> bool {
        self.important_conversation
            || self.in_combat
            || self.fleeing
            || self.unconscious
            || self.knocked
            || self.speaking
            || self.greeting_line
            || self.player_spoken_to
    }
}

/// The player's greeting cooldown (`PlayerCharacter` +0xe24): negative
/// while anyone may greet the player; else the time (`GetTickCount`
/// milliseconds, as a float) of the last greeting. Starts at −1 (the
/// constructor `00938180`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HelloCooldown(pub f32);

impl Default for HelloCooldown {
    fn default() -> Self {
        HelloCooldown(-1.0)
    }
}

impl HelloCooldown {
    /// Whether someone may greet the player now (`008bc520`: +0xe24 < 0).
    pub fn free(&self) -> bool {
        self.0 < 0.0
    }

    /// Someone greeted the player (`008bc560`, after the GREET procedure
    /// started in `008bc3d0`): the tick count is noted.
    pub fn greeted(&mut self, now_ms: u32) {
        self.0 = now_ms as f32;
    }

    /// The player's update (`0094417x`…`009441e4`): once more than
    /// `fHelloCooldownTime` × 1000 ms have passed since the last greeting,
    /// −1 again.
    // Translated from the player update at 00944179 (decompiled,
    // FalloutNV.exe 1.4.0.525).
    pub fn update(&mut self, now_ms: u32, settings: &SocialSettings) {
        if self.0 > 0.0 {
            let passed = f64::from(now_ms) - f64::from(self.0);
            if f64::from(settings.hello_cooldown) * 1000.0 < passed {
                self.0 = -1.0;
            }
        }
    }
}

/// A package's general flags (`PKDT` u32 at 0, the package's +0x1c) and
/// its behaviour flags (`PKDT` u16 at 6, +0x22). The offsets in memory
/// follow from the sandbox flags' (`PKDT` u16 at 8, +0x24).
fn package_flag_words(order: &LoadOrder, package: FormId) -> (u32, u16) {
    order
        .get(package)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(esm::FourCC::new(b"PKDT"))
                .filter(|s| s.data.len() >= 8)
                .map(|s| {
                    (
                        u32::from_le_bytes([s.data[0], s.data[1], s.data[2], s.data[3]]),
                        u16::from_le_bytes([s.data[6], s.data[7]]),
                    )
                })
        })
        .unwrap_or((0, 0))
}

/// Whether a package forbids greeting the player (`008a78f0(0)`): its
/// general flag 0x1000 is set (`0067a380`) and behaviour flag 0x01 (the
/// editor's "Hellos to player", inferred name) clear (`0067a850`).
pub fn package_forbids_hellos(order: &LoadOrder, package: FormId) -> bool {
    let (general, behaviour) = package_flag_words(order, package);
    flags_forbid_hellos(general, behaviour)
}

/// [`package_forbids_hellos`] on the flag words.
// Translated from 008a78f0 case 0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn flags_forbid_hellos(general: u32, behaviour: u16) -> bool {
    general & 0x1000 != 0 && behaviour & 0x01 == 0
}

/// Whether a package forbids conversations with others (`008a78f0(1)`, in
/// `00904800`): its general flag 0x1000 set and behaviour flag 0x02 clear
/// (`0067a8d0`).
pub fn package_forbids_conversations(order: &LoadOrder, package: FormId) -> bool {
    let (general, behaviour) = package_flag_words(order, package);
    flags_forbid_conversations(general, behaviour)
}

/// [`package_forbids_conversations`] on the flag words.
// Translated from 008a78f0 case 1 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn flags_forbid_conversations(general: u32, behaviour: u16) -> bool {
    general & 0x1000 != 0 && behaviour & 0x02 == 0
}

/// The package types `00678610` holds (0x12…0x24 but 0x1a and 0x1e: the
/// packages the game makes for combat, alarms, fleeing, dialogue and the
/// like); someone running one doesn't look for others to talk to
/// (`00904800`).
pub fn made_package_kind(kind: u8) -> bool {
    matches!(kind, 0x12..=0x24) && kind != 0x1a && kind != 0x1e
}

/// Whether a package allows idle chatter (`0067abd0`: behaviour flag 0x80,
/// the editor's "Allow Idle Chatter", inferred name; asked whatever the
/// general flags).
pub fn package_allows_chatter(order: &LoadOrder, package: FormId) -> bool {
    package_flag_words(order, package).1 & 0x80 != 0
}

/// One person's social timers (their high process): the greeting timer
/// (+0x330), idle chatter (+0x298), conversations (+0x2c8), the same-pair
/// timer (+0x2a0) and the people they've talked with lately.
#[derive(Debug, Clone, PartialEq)]
pub struct Social {
    pub greeting: f32,
    pub chatter: f32,
    pub conversation: f32,
    pub pair: f32,
    pub partners: Vec<FormId>,
}

impl Social {
    /// As a process starts (`008d7510`): the chatter and conversation
    /// timers at random within their settings, the pair timer at
    /// `fAItalktosameNPCtimer`, no greeting wait.
    pub fn new(settings: &SocialSettings, unit: &mut dyn FnMut() -> f32) -> Social {
        Social {
            greeting: 0.0,
            chatter: between(settings.chatter, unit()),
            conversation: between(settings.conversation_timer, unit()),
            pair: settings.same_npc,
            partners: Vec::new(),
        }
    }

    /// What the high-process update does about the player this frame
    /// (`008eeec0`, `008ef5cc`…`008ef9af`; [`GreetingCheck`] has each
    /// condition with its address):
    ///
    /// 1. Someone busy ([`GreetingCheck::busy`]) neither greets nor
    ///    chatters ([`Greeting::Busy`]).
    /// 2. Detecting the player (above 0), not asleep, the player neither
    ///    sneaking nor trespassing, their target not the player (unless
    ///    they run a run-once package), nobody having asked them to look at
    ///    someone, and the player within `fAIMinGreetingDistance`: no idle
    ///    chatter, and a greeting ([`Greeting::Greet`]) when the player
    ///    isn't in combat, their package allows hellos and isn't going on
    ///    only for the player, isn't an alarm package, they aren't standing
    ///    still in a made conversation package, their target isn't the
    ///    player, and their greeting timer has run out; else
    ///    [`Greeting::Near`].
    /// 3. Otherwise [`Greeting::Away`]: the idle chatter timer runs
    ///    ([`Self::chatter_due`]).
    ///
    /// A greeting says `HELLO` through `008bc3d0` only when the player's
    /// [`HelloCooldown`] is free.
    // Translated from 008eeec0 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn greeting(&self, check: &GreetingCheck, settings: &SocialSettings) -> Greeting {
        if check.busy() {
            return Greeting::Busy;
        }
        let near = check.detection > 0
            && !check.asleep
            && !check.player_sneaking
            && !check.player_trespassing
            && (!check.target_is_player || check.running_run_once)
            && !check.head_track_asked
            && check.distance <= settings.greeting_distance;
        if !near {
            return Greeting::Away;
        }
        let greets = !check.player_in_combat
            && !check.hellos_forbidden
            && !check.continuing_for_player
            && !check.alarm_package
            && !check.still_in_made_dialogue
            && !check.target_is_player
            && self.greeting <= 0.0;
        if greets {
            Greeting::Greet
        } else {
            Greeting::Near
        }
    }

    /// The GREET procedure saying a line to someone (`008dbe30`: process
    /// +0x330 set to `fAIGreetingTimer` on every update while a line with a
    /// listener is said, and when no line was found), and the player's
    /// dialogue with them closing (`00762160`): no greeting for
    /// `fAIGreetingTimer` from now.
    pub fn greeted(&mut self, settings: &SocialSettings) {
        self.greeting = settings.greeting_timer;
    }

    /// The frame's time passes on the greeting timer (`008eeec0`: process
    /// +0x330 less the frame's time on every high-process update).
    pub fn tick(&mut self, dt: f32) {
        if self.greeting > 0.0 {
            self.greeting -= dt;
        }
    }

    /// The idle chatter timer (`008eeec0`, `008ef8ee`), only run when the
    /// player isn't near ([`Greeting::Away`]): while it's above 0, or while
    /// they stand in a made conversation package (`009336c0`, type 0x1c), it
    /// counts down; else, if they have no package, or it isn't a dialogue
    /// package (type 15) and allows idle chatter (`0067abd0`: behaviour
    /// flag 0x80), a line from `IdleChatter` is due and the timer starts
    /// again at random within `fIdleChatterCommentTimer…`. `package` is
    /// their package's type and whether it allows chatter
    /// ([`package_allows_chatter`]). Whether a line is due.
    // Translated from 008eeec0 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn chatter_due(
        &mut self,
        dt: f32,
        package: Option<(u8, bool)>,
        in_made_dialogue: bool,
        settings: &SocialSettings,
        unit: &mut dyn FnMut() -> f32,
    ) -> bool {
        if self.chatter > 0.0 || in_made_dialogue {
            self.chatter -= dt;
            return false;
        }
        let allowed = match package {
            None => true,
            Some((kind, chatter)) => kind != crate::ai::kinds::DIALOGUE && chatter,
        };
        if !allowed {
            return false;
        }
        self.chatter = between(settings.chatter, unit());
        true
    }

    /// Someone looks for another to talk to (`00904800`, each high-process
    /// update): `allowed` says their package lets them (not sleep, use item
    /// at, ambush, guard, dialogue or use weapon packages; a sandbox only
    /// when its "no conversation" flag is clear; the caller's) and `only`
    /// limits them to their follow, escort or accompany target. When the
    /// conversation timer has run out, each candidate (others who can talk
    /// both ways, not the player, a sandboxer only if its sandbox allows
    /// it) within the trigger distance, in order: the same-pair timer (reset
    /// to `fAItalktosameNPCtimer` while nobody is remembered, counting down
    /// by the frame for each candidate in reach otherwise) clears the
    /// remembered partners when it runs out; someone remembered is passed
    /// over; else a roll (`rand() % 100`) below the chance starts it, and
    /// both remember each other. Then the timer is set at random within
    /// `fAISocialTimerForConversations…`. Whom they start talking to.
    #[allow(clippy::too_many_arguments)]
    pub fn start_conversation(
        &mut self,
        me: FormId,
        allowed: bool,
        only: Option<FormId>,
        candidates: &[(FormId, f32)],
        interior: bool,
        dt: f32,
        settings: &SocialSettings,
        roll: &mut dyn FnMut() -> u64,
        unit: &mut dyn FnMut() -> f32,
    ) -> Option<FormId> {
        if self.conversation > 0.0 {
            self.conversation -= dt;
            return None;
        }
        if !allowed {
            return None;
        }
        let reach = settings.trigger_distance(interior);
        let chance = settings.chance(interior);
        let mut chosen = None;
        if let Some(target) = only {
            chosen = Some(target);
        } else {
            for &(other, distance) in candidates {
                if other == me || other == crate::dialogue::PLAYER_REF || distance > reach {
                    continue;
                }
                if self.partners.is_empty() {
                    self.pair = settings.same_npc;
                } else {
                    self.pair -= dt;
                }
                if self.pair <= 0.0 {
                    self.partners.clear();
                }
                let rolled = (roll() % 100) as f32;
                if self.partners.contains(&other) {
                    continue;
                }
                if rolled < chance {
                    self.partners.push(other);
                    chosen = Some(other);
                    break;
                }
            }
        }
        let other = chosen?;
        self.conversation = between(settings.conversation_timer, unit());
        Some(other)
    }
}

/// A line's "next speaker" (`INFO` `DATA` byte 1, the line's +0x24 read by
/// `0061b9f0`): 0 the other, 1 the same, 2 either (a coin flip).
pub fn next_speaker(order: &LoadOrder, info: FormId) -> u8 {
    order
        .get(info)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.get(1).copied()))
        .unwrap_or(0)
}

/// The first line of a topic `speaker` can say to `listener` now
/// (`0061b320`): as [`crate::dialogue::pick`], its conditions asked with
/// the listener as their target; lines already in this conversation
/// (`said`) aren't said again (the code is given the list; that it skips
/// them is inferred).
pub fn pick_for(
    order: &LoadOrder,
    topic: FormId,
    speaker: &Speaker,
    listener: FormId,
    state: &GameState,
    said: &[FormId],
) -> Option<Info> {
    let facts = Facts {
        order,
        state,
        speaker: Some(speaker),
    };
    let mut quest_ok: std::collections::HashMap<FormId, bool> = Default::default();
    // `0061b320` asks `0061a790` (the same choice as the menu's): random
    // runs and the Intelligence classes as `dialogue::pick`.
    let available = topic_lines(order, topic).into_iter().filter(|info| {
        if said.contains(&info.form_id) {
            return false;
        }
        if let Some(q) = info.quest {
            let ok = *quest_ok.entry(q).or_insert_with(|| {
                state.running.contains(&q)
                    && facts.conditions_pass(
                        &crate::quest::quest_conditions(order, q),
                        speaker.reference,
                        listener,
                    )
            });
            if !ok {
                return false;
            }
        }
        crate::dialogue::line_available(order, info, speaker, listener, state)
    });
    crate::dialogue::choose(available, state.dice)
}

/// One line of a conversation between two people: who says it, to whom.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub speaker: FormId,
    pub listener: FormId,
    pub info: Info,
}

/// The most lines a conversation holds (`0061b440` stops at 100).
pub const MOST_LINES: usize = 100;

/// A conversation's lines, worked out as it starts (`0061b440`): from the
/// topic given, else `HELLO`, `a` speaking first. Each step: the first line
/// the speaker can say on the topic ([`pick_for`]); with none, when no topic
/// was given or the last line keeps the same speaker, `GOODBYE` instead;
/// still none, the end. Then one of the line's follow-ups (`TCLT`) at random
/// (`rand() % count`, each tried once), the speakers swapped by the line's
/// rule ([`next_speaker`]: 0 swap, 1 keep, 2 a coin flip, at every try) until
/// one has a line; a line without follow-ups ends it, and when none of them
/// has a line the last tried is looked at again (and so to `GOODBYE`). At
/// most [`MOST_LINES`].
pub fn conversation(
    order: &LoadOrder,
    state: &GameState,
    a: &Speaker,
    b: &Speaker,
    topic: Option<FormId>,
    roll: &mut dyn FnMut() -> u64,
) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let (mut speaker, mut listener) = (a, b);
    let mut current = topic.unwrap_or(topics::HELLO);
    let mut last_rule: Option<u8> = None;
    let mut said: Vec<FormId> = Vec::new();
    loop {
        if lines.len() >= MOST_LINES {
            break;
        }
        let mut line = pick_for(order, current, speaker, listener.reference, state, &said);
        if line.is_none() && (topic.is_none() || last_rule == Some(1)) {
            current = topics::GOODBYE;
            line = pick_for(order, current, speaker, listener.reference, state, &said);
        }
        let Some(info) = line else {
            break;
        };
        let rule = next_speaker(order, info.form_id);
        let follow_ups = info.choices.clone();
        said.push(info.form_id);
        lines.push(Line {
            speaker: speaker.reference,
            listener: listener.reference,
            info,
        });
        last_rule = Some(rule);
        if follow_ups.is_empty() {
            break;
        }
        let mut tried = vec![false; follow_ups.len()];
        let mut next: Option<FormId> = None;
        let mut last_tried = None;
        loop {
            // The code rolls again until it meets one not yet tried: one of
            // those, evenly.
            let untried: Vec<usize> = (0..tried.len()).filter(|&i| !tried[i]).collect();
            if untried.is_empty() {
                break;
            }
            let k = untried[(roll() % untried.len() as u64) as usize];
            tried[k] = true;
            let swap = match rule {
                0 => true,
                2 => roll() & 1 == 0,
                _ => false,
            };
            if swap {
                std::mem::swap(&mut speaker, &mut listener);
            }
            last_tried = Some(follow_ups[k]);
            if pick_for(
                order,
                follow_ups[k],
                speaker,
                listener.reference,
                state,
                &said,
            )
            .is_some()
            {
                next = Some(follow_ups[k]);
                break;
            }
        }
        match next.or(last_tried) {
            Some(t) => current = t,
            None => break,
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> SocialSettings {
        SocialSettings {
            greeting_distance: 150.0,
            greeting_timer: 20.0,
            hello_cooldown: 30.0,
            chatter: (15.0, 60.0),
            conversation_timer: (30.0, 60.0),
            same_npc: 120.0,
            radius: 2000.0,
            radius_interior: 1000.0,
            chance: 25.0,
            chance_interior: 25.0,
        }
    }

    fn near(distance: f32) -> GreetingCheck {
        GreetingCheck {
            detection: 10,
            distance,
            ..Default::default()
        }
    }

    #[test]
    fn greetings_wait_for_the_timer_and_the_distance() {
        let s = settings();
        let mut social = Social::new(&s, &mut || 0.5);
        assert_eq!(social.chatter, 37.5);
        assert_eq!(social.conversation, 45.0);
        assert_eq!(social.greeting(&near(140.0), &s), Greeting::Greet);
        let unseen = GreetingCheck {
            detection: 0,
            ..near(140.0)
        };
        assert_eq!(social.greeting(&unseen, &s), Greeting::Away);
        assert_eq!(social.greeting(&near(160.0), &s), Greeting::Away);
        social.greeted(&s);
        // Near but waiting: no greeting, and no chatter either.
        assert_eq!(social.greeting(&near(140.0), &s), Greeting::Near);
        for _ in 0..200 {
            social.tick(0.1);
        }
        assert_eq!(social.greeting(&near(140.0), &s), Greeting::Greet);
    }

    #[test]
    fn busy_people_neither_greet_nor_chatter() {
        let s = settings();
        let social = Social::new(&s, &mut || 0.5);
        for busy in [
            GreetingCheck {
                in_combat: true,
                ..near(100.0)
            },
            GreetingCheck {
                fleeing: true,
                ..near(100.0)
            },
            GreetingCheck {
                unconscious: true,
                ..near(100.0)
            },
            GreetingCheck {
                knocked: true,
                ..near(100.0)
            },
            GreetingCheck {
                speaking: true,
                ..near(100.0)
            },
            GreetingCheck {
                greeting_line: true,
                ..near(100.0)
            },
            // Someone else is saying a line to the player.
            GreetingCheck {
                player_spoken_to: true,
                ..near(100.0)
            },
            // Someone talking in the player's AI conversation.
            GreetingCheck {
                important_conversation: true,
                ..near(100.0)
            },
        ] {
            assert_eq!(social.greeting(&busy, &s), Greeting::Busy, "{busy:?}");
        }
    }

    #[test]
    fn what_keeps_a_near_person_from_greeting() {
        let s = settings();
        let social = Social::new(&s, &mut || 0.5);
        // Not near at all (the chatter path): asleep, the player sneaking
        // or trespassing.
        for away in [
            GreetingCheck {
                asleep: true,
                ..near(100.0)
            },
            GreetingCheck {
                player_sneaking: true,
                ..near(100.0)
            },
            GreetingCheck {
                player_trespassing: true,
                ..near(100.0)
            },
            // Their target is the player, with no run-once package.
            GreetingCheck {
                target_is_player: true,
                ..near(100.0)
            },
            // Asked to look at someone (a head-track slot above the
            // default one): `CanSetActionHeadTrackTarget` false.
            GreetingCheck {
                head_track_asked: true,
                ..near(100.0)
            },
        ] {
            assert_eq!(social.greeting(&away, &s), Greeting::Away, "{away:?}");
        }
        // Near, no chatter, no greeting.
        for quiet in [
            GreetingCheck {
                player_in_combat: true,
                ..near(100.0)
            },
            GreetingCheck {
                hellos_forbidden: true,
                ..near(100.0)
            },
            GreetingCheck {
                alarm_package: true,
                ..near(100.0)
            },
            GreetingCheck {
                still_in_made_dialogue: true,
                ..near(100.0)
            },
            GreetingCheck {
                continuing_for_player: true,
                ..near(100.0)
            },
            // The player their target, in a run-once package: near, but
            // no greeting.
            GreetingCheck {
                target_is_player: true,
                running_run_once: true,
                ..near(100.0)
            },
        ] {
            assert_eq!(social.greeting(&quiet, &s), Greeting::Near, "{quiet:?}");
        }
    }

    #[test]
    fn a_packages_behaviour_flags_count_only_with_general_flag_0x1000() {
        assert!(!flags_forbid_hellos(0, 0));
        assert!(flags_forbid_hellos(0x1000, 0));
        assert!(flags_forbid_hellos(0x1000, 0x80));
        assert!(!flags_forbid_hellos(0x1000, 0x01));
        assert!(!flags_forbid_conversations(0, 0));
        assert!(flags_forbid_conversations(0x1000, 0x01));
        assert!(!flags_forbid_conversations(0x1000, 0x02));
    }

    #[test]
    fn the_games_own_packages_dont_look_for_conversations() {
        for kind in [0x12, 0x15, 0x16, 0x1c, 0x1d, 0x24] {
            assert!(made_package_kind(kind), "{kind:#x}");
        }
        for kind in [0, 12, 15, 0x11, 0x1a, 0x1e, 0x25] {
            assert!(!made_package_kind(kind), "{kind:#x}");
        }
    }

    #[test]
    fn one_greeting_holds_everyone_else_off_for_the_hello_cooldown() {
        let s = settings();
        let mut cooldown = HelloCooldown::default();
        assert!(cooldown.free());
        cooldown.greeted(5_000);
        assert!(!cooldown.free());
        // 30 s exactly isn't past it; just after is.
        cooldown.update(35_000, &s);
        assert!(!cooldown.free());
        cooldown.update(35_001, &s);
        assert!(cooldown.free());
        assert_eq!(cooldown, HelloCooldown(-1.0));
    }

    #[test]
    fn idle_chatter_comes_round_at_random_but_not_in_dialogue_packages() {
        let s = settings();
        let mut social = Social::new(&s, &mut || 0.0);
        assert_eq!(social.chatter, 15.0);
        let mut t = 0.0f32;
        while !social.chatter_due(0.5, None, false, &s, &mut || 1.0) {
            t += 0.5;
        }
        assert!((t - 15.0).abs() < 0.6, "{t}");
        assert_eq!(social.chatter, 60.0);
        social.chatter = 0.0;
        let dialogue = Some((crate::ai::kinds::DIALOGUE, true));
        assert!(!social.chatter_due(0.5, dialogue, false, &s, &mut || 1.0));
        // A package that doesn't allow idle chatter: none, the timer left.
        let sandbox = crate::ai::kinds::SANDBOX;
        assert!(!social.chatter_due(0.5, Some((sandbox, false)), false, &s, &mut || 1.0));
        assert_eq!(social.chatter, 0.0);
        // Standing in a made conversation package, the timer only runs.
        assert!(!social.chatter_due(0.5, Some((sandbox, true)), true, &s, &mut || 1.0));
        assert_eq!(social.chatter, -0.5);
        assert!(social.chatter_due(0.5, Some((sandbox, true)), false, &s, &mut || 1.0));
    }

    #[test]
    fn conversations_start_within_200_by_chance_and_not_twice_with_one_person() {
        let s = settings();
        assert_eq!(s.trigger_distance(true), 200.0);
        let me = FormId(1);
        let mut social = Social::new(&s, &mut || 0.0);
        social.conversation = 0.0;
        let mut rolls = [80u64, 10, 10, 10].into_iter();
        let mut roll = || rolls.next().unwrap_or(99);
        // The first candidate is too far; the second's roll fails (80); the
        // third's passes (10).
        let candidates = [(FormId(2), 250.0), (FormId(3), 150.0), (FormId(4), 150.0)];
        let got = social.start_conversation(
            me,
            true,
            None,
            &candidates,
            true,
            0.1,
            &s,
            &mut roll,
            &mut || 0.0,
        );
        assert_eq!(got, Some(FormId(4)));
        assert_eq!(social.conversation, 30.0);
        assert_eq!(social.partners, vec![FormId(4)]);
        // Not again for the timer; then not with the same person.
        social.conversation = 0.0;
        let mut always = || 0u64;
        let got = social.start_conversation(
            me,
            true,
            None,
            &[(FormId(4), 100.0)],
            true,
            0.1,
            &s,
            &mut always,
            &mut || 0.0,
        );
        assert_eq!(got, None);
        // A package that forbids it.
        let got = social.start_conversation(
            me,
            false,
            None,
            &[(FormId(5), 100.0)],
            true,
            0.1,
            &s,
            &mut always,
            &mut || 0.0,
        );
        assert_eq!(got, None);
    }
}
