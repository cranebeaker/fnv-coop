//! Idle animations: the game keeps them as a tree of `IDLE` records. Each
//! has a model (`MODL`: an animation, `Characters\_Male\IdleAnims\
//! DynamicIdle_ChairSit.kf`, or a folder for a group), the conditions under
//! which it plays (`CTDA`), its place in the tree (`ANAM`: parent and
//! previous sibling, 0 for none) and `DATA` (8 bytes: the animation's body
//! section, loop counts, flags, replay delay).
//!
//! The pick, read from the game's code (`FalloutNV.exe` `005ff2e0` per
//! node, `00600950` over the roots; see [`IdleTree::evaluate`]): depth
//! first, the first passing child in sibling order wins, a node with
//! nothing below returns itself if it's an animation (and not waiting out
//! its replay delay) or a folder flagged "blocking" (`DATA` byte 0 0x80,
//! which ends the search with no idle). There's no random choice in the
//! walk itself: randomness comes from conditions like `GetRandomPercent`.
//! How often an actor asks: [`IDLE_WAIT_TIME`].

use std::collections::HashMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::Condition;

const IDLE: FourCC = FourCC::new(b"IDLE");
const MODL: FourCC = FourCC::new(b"MODL");
const CTDA: FourCC = FourCC::new(b"CTDA");
const ANAM: FourCC = FourCC::new(b"ANAM");

/// One `IDLE` record.
#[derive(Debug, Clone, PartialEq)]
pub struct Idle {
    pub form_id: FormId,
    pub editor_id: String,
    /// Relative to `meshes\`: an animation (`.kf`), or a group's folder.
    pub model: String,
    pub conditions: Vec<Condition>,
    pub parent: Option<FormId>,
    pub previous: Option<FormId>,
    pub data: [u8; 8],
}

impl Idle {
    /// Whether it plays an animation (rather than grouping others).
    pub fn is_animation(&self) -> bool {
        self.model.to_ascii_lowercase().ends_with(".kf")
    }

    /// The body section it plays on (`DATA` byte 0, low six bits): 0 the
    /// base loop ("dynamic idle"), 7 a special idle, 0x14 the whole body,
    /// 0x15 the upper body (the game resets anything else to 7, `005fe810`).
    pub fn group(&self) -> u8 {
        self.data[0] & 0x3f
    }

    /// A folder that stops the search when it matches with nothing under
    /// it (`DATA` byte 0 flag 0x80: `Sitting` and `SitDown` have it, so a
    /// seated actor never falls through to the standing idles).
    pub fn is_blocking(&self) -> bool {
        self.data[0] & 0x80 != 0
    }

    /// Kept out of the tree (`DATA` byte 0 flag 0x40, `005ff610`): loose
    /// idles that only scripts and idle markers play (what the flag means
    /// is inferred from that code).
    pub fn is_loose(&self) -> bool {
        self.data[0] & 0x40 != 0
    }

    /// The loop counts (`DATA` bytes 1 and 2).
    pub fn loops(&self) -> (u8, u8) {
        (self.data[1], self.data[2])
    }

    /// How long before it may play again once it has (`DATA` i16 at 4;
    /// counted in seconds, which is inferred: the countdown isn't traced).
    pub fn replay_delay(&self) -> i16 {
        i16::from_le_bytes([self.data[4], self.data[5]])
    }

    /// How many more times it plays after the first, rolled when it
    /// starts (`005ff770`): none if either count is 0, for ever (255) if
    /// the least is 255, else a number from the least to the most, less
    /// one. `roll(lo, hi)` gives a number from `lo` to `hi`, both included.
    pub fn extra_loops(&self, roll: impl FnOnce(u8, u8) -> u8) -> u8 {
        let (min, max) = self.loops();
        if min == 0 || max == 0 {
            0
        } else if min == 255 {
            255
        } else if min < max {
            roll(min, max).saturating_sub(1)
        } else {
            max - 1
        }
    }
}

/// How often an actor with nothing else to do asks the idle tree again:
/// `fAIIdleWaitTime` (1 second, the exe's default; `FalloutNV.esm`
/// doesn't set it), counted down while no idle plays (`008dafd0`).
pub const IDLE_WAIT_TIME: f32 = 1.0;

/// How near the player someone must be to pick idles:
/// `fAIIdleAnimationDistance` (2000, the exe's default) × the actor's
/// bound radius ÷ 64 (`008dafd0`; that the factor is the bound radius is
/// inferred).
pub const IDLE_ANIMATION_DISTANCE: f32 = 2000.0;

/// An idle the tree gave, playing.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayingIdle {
    pub idle: FormId,
    /// The `.kf`, under `meshes\`.
    pub model: String,
    pub length: f32,
    /// Seconds into the current play-through.
    pub elapsed: f32,
    /// Play-throughs left after this one (255: for ever).
    pub loops_left: u8,
}

/// One actor's idles over time: the AI process's idle timer and special
/// idle (`008dafd0` each frame, `008dab40` to ask the tree). While the
/// actor is free (near the player, no special idle playing, nothing else
/// to do; the caller judges) a timer counts down; below zero the tree is
/// asked and the timer starts again at [`IDLE_WAIT_TIME`]. An idle for the
/// base loop (section 0) replaces the actor's standing or seated loop; any
/// other plays its rolled number of times ([`Idle::extra_loops`]) and the
/// timer waits while it does. An idle with a replay delay can't be picked
/// again until it has passed (`00498290`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IdleClock {
    pub wait: f32,
    pub playing: Option<PlayingIdle>,
    /// Idles waiting out their replay delays: (idle, seconds left).
    pub delays: Vec<(FormId, f32)>,
    /// The last idle played (`IsLastIdlePlayed`).
    pub last: Option<FormId>,
}

impl IdleClock {
    /// Time passes: the idle plays on (round again while it has loops
    /// left), replay delays count down. Whether an idle ended just now.
    pub fn advance(&mut self, dt: f32) -> bool {
        for d in &mut self.delays {
            d.1 -= dt;
        }
        self.delays.retain(|d| d.1 > 0.0);
        let Some(p) = self.playing.as_mut() else {
            return false;
        };
        p.elapsed += dt;
        if p.elapsed < p.length {
            return false;
        }
        if p.loops_left > 0 {
            if p.loops_left != 255 {
                p.loops_left -= 1;
            }
            p.elapsed = if p.length > 0.0 {
                p.elapsed % p.length
            } else {
                0.0
            };
            return false;
        }
        self.last = Some(p.idle);
        self.playing = None;
        true
    }

    /// Counts the timer down while the actor is `free`; whether it's run
    /// out, when the tree is to be asked (and the timer starts again).
    pub fn due(&mut self, dt: f32, free: bool) -> bool {
        if !free || self.playing.is_some() {
            return false;
        }
        if self.wait >= 0.0 {
            self.wait -= dt;
            return false;
        }
        self.wait = IDLE_WAIT_TIME;
        true
    }

    /// Whether an idle is waiting out its replay delay.
    pub fn is_delayed(&self, idle: FormId) -> bool {
        self.delays.iter().any(|d| d.0 == idle)
    }

    /// Plays an idle from the tree (not a base-loop one): `length` its
    /// animation's, `roll(lo, hi)` a number from `lo` to `hi`.
    pub fn start(&mut self, idle: &Idle, length: f32, roll: impl FnOnce(u8, u8) -> u8) {
        if idle.replay_delay() > 0 {
            self.delays.retain(|d| d.0 != idle.form_id);
            self.delays
                .push((idle.form_id, f32::from(idle.replay_delay())));
        }
        self.playing = Some(PlayingIdle {
            idle: idle.form_id,
            model: idle.model.clone(),
            length,
            elapsed: 0.0,
            loops_left: idle.extra_loops(roll),
        });
    }

    /// An idle requested by name or by the dialogue menu went to the
    /// actor's special-idle section (its own clock is the animation's):
    /// it's the last idle played and waits out its replay delay, as one
    /// the clock started does (`00498290`).
    pub fn played(&mut self, idle: &Idle) {
        if idle.replay_delay() > 0 {
            self.delays.retain(|d| d.0 != idle.form_id);
            self.delays
                .push((idle.form_id, f32::from(idle.replay_delay())));
        }
        self.playing = None;
        self.last = Some(idle.form_id);
    }

    /// Stops the idle playing (a fight, getting up for good).
    pub fn stop(&mut self) {
        if let Some(p) = self.playing.take() {
            self.last = Some(p.idle);
        }
    }
}

/// Every idle, and each one's children in sibling order.
#[derive(Debug, Clone, Default)]
pub struct IdleTree {
    idles: HashMap<FormId, Idle>,
    children: HashMap<Option<FormId>, Vec<FormId>>,
}

impl IdleTree {
    pub fn load(order: &LoadOrder) -> IdleTree {
        let mut idles = HashMap::new();
        for rr in order.records_of_type(IDLE) {
            // Deleted idles are skipped in the walk (`005ff2e0`).
            if rr.entry.header.is_deleted() {
                continue;
            }
            let Ok(record) = rr.record() else { continue };
            let form = |raw: u32| Some(rr.plugin.to_global(FormId(raw))).filter(|f| f.0 != 0);
            let (parent, previous) = record
                .get(ANAM)
                .filter(|s| s.data.len() >= 8)
                .map_or((None, None), |s| {
                    (form(le_u32(&s.data, 0)), form(le_u32(&s.data, 4)))
                });
            let mut data = [0u8; 8];
            if let Some(d) = record.get(esm::sig::DATA) {
                let n = d.data.len().min(8);
                data[..n].copy_from_slice(&d.data[..n]);
            }
            idles.insert(
                rr.form_id,
                Idle {
                    form_id: rr.form_id,
                    editor_id: record.editor_id().unwrap_or_default(),
                    model: record.get(MODL).map(|s| s.zstring()).unwrap_or_default(),
                    conditions: record
                        .get_all(CTDA)
                        .filter_map(|s| crate::dialogue::read_condition(&rr, &s.data))
                        .collect(),
                    parent,
                    previous,
                    data,
                },
            );
        }
        IdleTree::from_idles(idles)
    }

    /// Orders each parent's children by their sibling links: the one with
    /// no previous sibling first, then whoever names it, and so on (any
    /// left over, by form ID, after).
    pub fn from_idles(idles: HashMap<FormId, Idle>) -> IdleTree {
        let mut by_parent: HashMap<Option<FormId>, Vec<FormId>> = HashMap::new();
        for idle in idles.values() {
            by_parent.entry(idle.parent).or_default().push(idle.form_id);
        }
        let mut children = HashMap::new();
        for (parent, mut ids) in by_parent {
            ids.sort();
            let mut ordered = Vec::with_capacity(ids.len());
            let mut previous: Option<FormId> = None;
            loop {
                let next = ids.iter().position(|id| idles[id].previous == previous);
                let Some(i) = next else { break };
                let id = ids.remove(i);
                ordered.push(id);
                previous = Some(id);
            }
            ordered.extend(ids);
            children.insert(parent, ordered);
        }
        IdleTree { idles, children }
    }

    pub fn get(&self, id: FormId) -> Option<&Idle> {
        self.idles.get(&id)
    }

    /// A group's children in order (`None`: the top of the tree).
    pub fn children(&self, parent: Option<FormId>) -> &[FormId] {
        self.children.get(&parent).map_or(&[], Vec::as_slice)
    }

    /// The idle by editor ID.
    pub fn by_name(&self, editor_id: &str) -> Option<&Idle> {
        self.idles
            .values()
            .find(|i| i.editor_id.eq_ignore_ascii_case(editor_id))
    }

    /// The animation a group gives: the first child whose conditions pass
    /// (`passes`), followed down through groups, as [`Self::evaluate`]
    /// walks them. `None` when nothing in it passes (or only a blocking
    /// folder does).
    pub fn pick(&self, group: FormId, passes: &dyn Fn(&Idle) -> bool) -> Option<&Idle> {
        let found = self
            .children(Some(group))
            .iter()
            .find_map(|&id| self.node(id, passes, &|_| false, 0))?;
        found.is_animation().then_some(found)
    }

    /// The idle the game's tree gives an actor, from `roots` in order
    /// ([`Self::roots_for`]), as read from the game's code (`00600950`,
    /// `005ff2e0`): a node whose conditions (`passes`) fail gives nothing
    /// and its branch is skipped; one whose conditions pass tries its
    /// children in sibling order and the first that gives something wins;
    /// with nothing from them it gives itself if it's an animation not
    /// waiting out its replay delay (`delayed`), or if it's a blocking
    /// folder ([`Idle::is_blocking`]); any other folder gives nothing, so
    /// the search goes on with the next sibling. A blocking folder as the
    /// answer means no idle (and no further roots are tried).
    pub fn evaluate(
        &self,
        roots: &[FormId],
        passes: &dyn Fn(&Idle) -> bool,
        delayed: &dyn Fn(FormId) -> bool,
    ) -> Option<&Idle> {
        let found = roots
            .iter()
            .find_map(|&id| self.node(id, passes, delayed, 0))?;
        found.is_animation().then_some(found)
    }

    fn node(
        &self,
        id: FormId,
        passes: &dyn Fn(&Idle) -> bool,
        delayed: &dyn Fn(FormId) -> bool,
        depth: u8,
    ) -> Option<&Idle> {
        let idle = self.idles.get(&id)?;
        if depth > 32 || !passes(idle) {
            return None;
        }
        let below = self
            .children(Some(id))
            .iter()
            .find_map(|&c| self.node(c, passes, delayed, depth + 1));
        if below.is_some() {
            return below;
        }
        let itself = if idle.is_animation() {
            !delayed(id)
        } else {
            idle.is_blocking()
        };
        itself.then_some(idle)
    }

    /// The tree's roots an actor's idles come from: the top-level idles
    /// (not loose ones) whose files are in the actor's skeleton's
    /// `IdleAnims` folder, in sibling order. The folder is the skeleton
    /// model's path cut after "IdleAnims" if it has it, else its folder
    /// plus "IdleAnims" (`Characters\_Male\Skeleton.nif` →
    /// `Characters\_Male\IdleAnims`, `006007f0`); an idle's folder is
    /// worked out the same way from its own model (`005ff610`).
    pub fn roots_for(&self, skeleton_model: &str) -> Vec<FormId> {
        let want = idle_folder(skeleton_model);
        self.children(None)
            .iter()
            .copied()
            .filter(|id| {
                let idle = &self.idles[id];
                !idle.is_loose() && idle_folder(&idle.model) == want
            })
            .collect()
    }
}

/// A model path's idle folder (lower case, backslashes): cut after
/// "IdleAnims", else its folder plus "IdleAnims" (`006007f0`).
fn idle_folder(model: &str) -> String {
    let path = model.replace('/', "\\").to_ascii_lowercase();
    let path = path.strip_prefix("meshes\\").unwrap_or(&path);
    if let Some(at) = path.find("idleanims") {
        return path[..at + "idleanims".len()].to_string();
    }
    match path.rfind('\\') {
        Some(at) => format!("{}idleanims", &path[..=at]),
        None => "idleanims".into(),
    }
}

/// Whether conditions hold with `value` giving each function's result:
/// joined as [`crate::scripting::Facts::conditions_pass`] joins them (OR
/// binds before AND).
pub fn conditions_hold(conditions: &[Condition], value: impl Fn(&Condition) -> f32) -> bool {
    let mut all = true;
    let mut group: Option<bool> = None;
    for c in conditions {
        let ok = c.compare(value(c), c.value);
        let so_far = group.map_or(ok, |g| g || ok);
        if c.or {
            group = Some(so_far);
        } else {
            all &= so_far;
            group = None;
        }
    }
    if let Some(g) = group {
        all &= g;
    }
    all
}

/// Someone using a piece of furniture, as the idle tree's furniture
/// branch asks about them without the game's state: `GetSitting` (1
/// loading the seated loop, 2 sitting down, 3 seated, 4 getting up),
/// `GetFurnitureMarkerID` (the marker used), `GetIsSex`, `GetIsID` of the
/// player, `IsChild`. Everything else is answered as [`IdleQuestion`]
/// does (not alerted or attacked, no item used), else 0 (no quest stage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InFurniture {
    pub sitting: u8,
    pub marker: u8,
    pub female: bool,
    pub player: bool,
    pub child: bool,
}

impl InFurniture {
    fn question(&self) -> IdleQuestion {
        IdleQuestion {
            sitting: self.sitting,
            marker: self.marker,
            female: self.female,
            player: self.player,
            child: self.child,
            procedure: procedures::NONE,
            ..IdleQuestion::default()
        }
    }
}

/// `GetCurrentAIProcedure` numbers, as the game's handler gives them
/// (`005a1210`'s switch; all procedures in `crate::more_functions::
/// procedures`). 4 (dialogue) and 10 (eating) are also what the idle tree
/// asks (the chair branch's `ChairDialogueIdles` and `SitChairFood`).
pub mod procedures {
    pub const TRAVEL: i32 = 0;
    pub const WAIT: i32 = 3;
    pub const DIALOGUE: i32 = 4;
    pub const WANDER: i32 = 7;
    pub const SLEEP: i32 = 8;
    pub const EAT: i32 = 10;
    pub const COMBAT: i32 = 13;
    /// None of the above (a number the tree never asks about).
    pub const NONE: i32 = 255;
}

/// What an actor is doing, as the idle tree's conditions ask it: the
/// functions answered here before the general ones
/// ([`crate::scripting::Facts`]).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct IdleQuestion {
    /// `GetSitting` and `GetSleeping`: the sit state as those functions
    /// give it (`crate::furniture::SitState::get_sitting`).
    pub sitting: u8,
    pub sleeping: u8,
    /// `GetFurnitureMarkerID`: the number of the marker in use.
    pub marker: u8,
    /// `GetCurrentAIProcedure` ([`procedures`]).
    pub procedure: i32,
    /// `IsMoving` (walking), `IsTalking` (saying a line), `GetIsAlerted`
    /// and `GetAttacked` (in a fight).
    pub moving: bool,
    pub talking: bool,
    pub alerted: bool,
    /// `IsLastIdlePlayed`, `GetIsUsedItem`.
    pub last_idle: Option<FormId>,
    pub used_item: Option<FormId>,
    /// Asked without the game's state only: `GetIsSex` (1 female),
    /// `GetIsID` of the player.
    pub female: bool,
    pub player: bool,
    /// `IsChild` (the race's child flag, [`is_child`]).
    pub child: bool,
    /// `IsPC1stPerson`: the player's view is first person. Only asked
    /// about the player here (people's questions leave it false, as
    /// before; whether the PC's view changes their picks isn't checked).
    pub first_person: bool,
    /// `MenuMode`: the menu open (the dialogue menu, [`DIALOG_MENU`],
    /// while the player talks to someone), as the condition answers it
    /// (`0059c380`: 0 any menu, else that menu).
    pub menu: Option<u16>,
    /// `GetDialogueEmotion` (`005a4480`): the emotion of the response the
    /// actor last said, when that response's emotion is to be used
    /// ([`crate::dialogue::Response::use_emotion`]); else −1.
    pub emotion: Option<u32>,
    /// `GetHitLocation` (`005a3c30`): the body part of the hit being
    /// taken (the process's last hit data, set while the damage is dealt,
    /// `0089a760`), else −1.
    pub hit_location: Option<i32>,
    /// `IsSneaking` and `IsRunning`: the movement flags 0x400 (without
    /// 0x800) and 0x200.
    pub sneaking: bool,
    pub running: bool,
    /// `IsGreetingPlayer` (`005a5330`): the process's greeting flag
    /// (vtable +0x30c) with the player as the one greeted.
    pub greeting_player: bool,
}

/// The dialogue menu's number (`MenuMode 1009`; the idle tree's
/// `DialogueIdles` and `TalkToPlayer` ask for it).
pub const DIALOG_MENU: u16 = 1009;

/// Asks the idle tree's conditions about one actor: what
/// [`IdleQuestion`] knows, then the general functions (`facts`, asked
/// about the actor with the player as the target), else 0. Conditions
/// naming another reference go straight to the general functions.
pub struct IdleAsker<'a> {
    pub actor: FormId,
    pub about: IdleQuestion,
    pub facts: Option<&'a crate::scripting::Facts<'a>>,
    /// `GetRandomPercent` draws afresh each time it's asked (a new random
    /// number per call is inferred; its handler isn't traced), from a
    /// seed the caller takes from the game state's dice.
    dice: std::cell::Cell<u64>,
}

impl<'a> IdleAsker<'a> {
    pub fn new(
        actor: FormId,
        about: IdleQuestion,
        facts: Option<&'a crate::scripting::Facts<'a>>,
        seed: u64,
    ) -> IdleAsker<'a> {
        IdleAsker {
            actor,
            about,
            facts,
            dice: std::cell::Cell::new(seed.max(1)),
        }
    }

    fn percent(&self) -> f64 {
        // xorshift64, as the game state's dice.
        let mut x = self.dice.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.dice.set(x);
        (x % 100) as f64
    }

    /// A condition function's value.
    pub fn value(&self, c: &Condition) -> f64 {
        let yes = |b: bool| if b { 1.0 } else { 0.0 };
        let a = &self.about;
        if c.run_on == 0 {
            let answer = match c.function_name().as_str() {
                "GetSitting" => Some(f64::from(a.sitting)),
                "GetSleeping" => Some(f64::from(a.sleeping)),
                "GetFurnitureMarkerID" => Some(f64::from(a.marker)),
                "GetRandomPercent" => Some(self.percent()),
                "GetCurrentAIProcedure" => Some(f64::from(a.procedure)),
                "IsMoving" => Some(yes(a.moving)),
                "IsTalking" => Some(yes(a.talking)),
                "GetIsAlerted" | "GetAttacked" => Some(yes(a.alerted)),
                "IsLastIdlePlayed" => Some(yes(a.last_idle == Some(c.param_forms[0]))),
                "GetIsUsedItem" => Some(yes(a.used_item == Some(c.param_forms[0]))),
                "IsChild" => Some(yes(a.child)),
                "IsPC1stPerson" => Some(yes(a.first_person)),
                // Translated from 0059c380 (decompiled, FalloutNV.exe
                // 1.4.0.525): 0 any menu, else that menu open.
                "MenuMode" => Some(yes(match c.params[0] {
                    0 => a.menu.is_some(),
                    n => a.menu.is_some_and(|m| u32::from(m) == n),
                })),
                // Translated from 005a4480 (decompiled, FalloutNV.exe
                // 1.4.0.525): the speaking emotion, −1 when not used.
                "GetDialogueEmotion" => Some(a.emotion.map_or(-1.0, f64::from)),
                "GetHitLocation" => Some(f64::from(a.hit_location.unwrap_or(-1))),
                "IsSneaking" => Some(yes(a.sneaking)),
                "IsRunning" => Some(yes(a.running)),
                // Nothing knocked down, greeting, using an item or in VATS
                // here.
                "IsGreetingPlayer" => Some(yes(a.greeting_player)),
                "GetKnockedState"
                | "GetForceHitReaction"
                | "GetUsedItemActivate"
                | "GetIsUsedItemType"
                | "GetVATSMode"
                | "GetCannibal"
                | "GetSandman"
                | "GetPlantedExplosive" => Some(0.0),
                "GetIsSex" if self.facts.is_none() => Some(yes(u32::from(a.female) == c.params[0])),
                "GetIsID" if self.facts.is_none() => Some(yes(
                    a.player && c.param_forms[0] == crate::dialogue::PLAYER_BASE
                )),
                _ => None,
            };
            if let Some(v) = answer {
                return v;
            }
        }
        self.facts.map_or(0.0, |f| {
            f.condition_value(c, self.actor, crate::dialogue::PLAYER_REF)
        })
    }

    /// Whether an idle's conditions pass, joined as
    /// [`crate::scripting::Facts::conditions_pass`] joins them (a value
    /// from a global is that global's).
    pub fn passes(&self, idle: &Idle) -> bool {
        let mut all = true;
        let mut group: Option<bool> = None;
        for c in &idle.conditions {
            let against = match (c.global, self.facts) {
                (Some(g), Some(f)) => f.state.globals.get(&g).copied().unwrap_or(0.0),
                _ => c.value,
            };
            let ok = c.compare(self.value(c) as f32, against);
            let so_far = group.map_or(ok, |g| g || ok);
            if c.or {
                group = Some(so_far);
            } else {
                all &= so_far;
                group = None;
            }
        }
        if let Some(g) = group {
            all &= g;
        }
        all
    }
}

/// Whether someone is a child: their race's child flag (`RACE` `DATA`
/// flags u32 at 32, 0x04).
pub fn is_child(order: &LoadOrder, actor: FormId) -> bool {
    let Some(base) = crate::scripting::base_of(order, actor) else {
        return false;
    };
    let race = order.get(base).and_then(|rr| {
        let record = rr.record_shared().ok()?;
        let s = record
            .get(FourCC::new(b"RNAM"))
            .filter(|s| s.data.len() >= 4)?;
        Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
    });
    race.and_then(|r| order.get(r))
        .and_then(|rr| rr.record_shared().ok())
        .and_then(|record| {
            let d = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 36)?;
            Some(le_u32(&d.data, 32) & 0x04 != 0)
        })
        .unwrap_or(false)
}

/// An idle marker (`IDLM`): a spot where people play idles. `IDLF` flags
/// (0x01 in sequence, else at random: `00901c70`), `IDLT` a timer (not
/// traced), `IDLA` the idles (`IDLC` of them).
#[derive(Debug, Clone, PartialEq)]
pub struct IdleMarker {
    pub flags: u8,
    pub timer: f32,
    pub idles: Vec<FormId>,
}

impl IdleMarker {
    /// The marker a placed reference (or a base) is.
    pub fn load(order: &LoadOrder, id: FormId) -> Option<IdleMarker> {
        let base = crate::scripting::base_of(order, id).unwrap_or(id);
        let rr = order.get(base)?;
        if rr.entry.header.kind != FourCC::new(b"IDLM") {
            return None;
        }
        let record = rr.record().ok()?;
        Some(IdleMarker {
            flags: record
                .get(FourCC::new(b"IDLF"))
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            timer: record
                .get(FourCC::new(b"IDLT"))
                .filter(|s| s.data.len() >= 4)
                .map_or(0.0, |s| crate::cell::le_f32(&s.data, 0)),
            idles: record
                .get(FourCC::new(b"IDLA"))
                .map(|s| {
                    s.data
                        .chunks_exact(4)
                        .map(|c| rr.plugin.to_global(FormId(le_u32(c, 0))))
                        .filter(|f| f.0 != 0)
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    /// Played in order (else one at random).
    pub fn in_sequence(&self) -> bool {
        self.flags & 0x01 != 0
    }
}

impl IdleTree {
    /// Whether an idle's conditions pass, and its parents' up the tree
    /// (`005ff430` asked with its parents, as an idle marker's idles are,
    /// `00479fb0`).
    pub fn passes_with_parents(&self, id: FormId, passes: &dyn Fn(&Idle) -> bool) -> bool {
        let mut at = Some(id);
        let mut depth = 0;
        while let Some(i) = at {
            let Some(idle) = self.idles.get(&i) else {
                return false;
            };
            if !passes(idle) {
                return false;
            }
            at = idle.parent;
            depth += 1;
            if depth > 32 {
                return false;
            }
        }
        true
    }
}

/// The root of the furniture idles (sitting, sleeping, leaning).
pub const FURNITURE_IDLES: &str = "FurnitureIdles";

impl IdleTree {
    /// The animation the furniture branch gives someone using furniture,
    /// asked without the game's state ([`InFurniture`]).
    pub fn furniture_idle(&self, asked: InFurniture) -> Option<&Idle> {
        let root = self.by_name(FURNITURE_IDLES)?;
        let asker = IdleAsker::new(FormId(0), asked.question(), None, 0x2545_F491_4F6C_DD1D);
        let passes = |i: &Idle| asker.passes(i);
        if !passes(root) {
            return None;
        }
        self.pick(root.form_id, &passes)
    }
}

/// Where someone sits from a furniture marker, by the game's own settings
/// for that marker number: `fFurnitureMarkerNNDeltaX` / `DeltaY` turned
/// clockwise by the marker's heading from the marker, and the heading plus
/// `fFurnitureMarkerNNHeadingDelta` (chairs: 11 (2.61, 53.96) −90°, 12
/// (2.67, 53.65) +90°, 13 (2.79, 57.42) 0°, 14 (2.48, 57.36) 180°; on Doc
/// Mitchell's chair every marker faces the sitter out along +y, and the
/// seats come within 5 units of where the exit animations put it, see
/// [`seat`]). `DeltaZ` (−28.95 for chairs) only counts × (scale − 1), so
/// not at all here at scale 1 (read: `00509920`; the placed version is
/// `crate::furniture::seat`). `setting` reads a game setting. In the
/// model's space, as (seat, heading).
pub fn seat_by_settings(
    marker: &nif::FurnitureMarker,
    setting: impl Fn(&str) -> Option<f32>,
) -> Option<([f32; 3], f32)> {
    let name = |what: &str| format!("fFurnitureMarker{:02}{what}", marker.marker);
    let dx = setting(&name("DeltaX"))?;
    let dy = setting(&name("DeltaY"))?;
    let turn = setting(&name("HeadingDelta"))?;
    let (s, c) = marker.heading.sin_cos();
    Some((
        [
            marker.offset[0] + dx * c + dy * s,
            marker.offset[1] - dx * s + dy * c,
            marker.offset[2],
        ],
        (marker.heading + turn).rem_euclid(std::f32::consts::TAU),
    ))
}

/// Where someone sits in a piece of furniture, in its model's space: the
/// seat (at the markers' floor height) and the heading (radians clockwise
/// from the model's +y), worked out from the exit animations (the game's
/// own numbers are [`seat_by_settings`]). The markers are where people stand to get in and
/// out (see `nif::FurnitureMarker`); each marker's exit animation moves
/// its user from the seat to it by its travel turned clockwise by the
/// marker's heading, so the seat is the marker less that. Checked on Doc
/// Mitchell's chair: from the left (11, `Chair_LeftExit.kf` travel -0.5,
/// -51.8), right (12, `Chair_RightExit.kf`) and front (14,
/// `Chair_ForwardExit.kf` 0, -55.4) markers the seat comes out within 0.6
/// units of (-2.2, 7.4). Facing out past the front marker (its heading
/// less half a turn), else the model's +y (a guess). `travel` gives a
/// marker's exit travel, when known.
pub fn seat(
    markers: &[nif::FurnitureMarker],
    travel: impl Fn(u8) -> Option<[f32; 3]>,
) -> Option<([f32; 3], f32)> {
    let seats: Vec<[f32; 3]> = markers
        .iter()
        .filter_map(|m| {
            let t = travel(m.marker)?;
            let (s, c) = m.heading.sin_cos();
            // Clockwise by the heading.
            let turned = [t[0] * c + t[1] * s, -t[0] * s + t[1] * c];
            Some([
                m.offset[0] - turned[0],
                m.offset[1] - turned[1],
                m.offset[2],
            ])
        })
        .collect();
    if seats.is_empty() {
        return None;
    }
    let n = seats.len() as f32;
    let mean = |k: usize| seats.iter().map(|s| s[k]).sum::<f32>() / n;
    let heading = markers.iter().find(|m| m.marker == 14).map_or(0.0, |m| {
        (m.heading - std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
    });
    Some(([mean(0), mean(1), mean(2)], heading))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_seat_is_where_each_markers_exit_starts() {
        // Doc Mitchell's chair (`SubChairDirty01.nif`) and the game's exits.
        // Headings as stored: thousandths of a radian.
        let marker = |offset: [f32; 3], heading: u16, marker: u8| nif::FurnitureMarker {
            offset,
            heading: f32::from(heading) / 1000.0,
            marker,
        };
        let markers = [
            marker([-53.7, 7.5, -35.6], 1570, 11),
            marker([49.0, 7.6, -35.6], 4712, 12),
            marker([-2.0, 62.8, -37.2], 3141, 14),
        ];
        let travel = |m: u8| match m {
            11 => Some([-0.5, -51.8, 0.0]),
            12 => Some([-0.2, -51.6, 0.0]),
            14 => Some([0.0, -55.4, 0.0]),
            _ => None,
        };
        let one = |m: usize| seat(&markers[m..=m], travel).unwrap().0;
        for m in 0..3 {
            let s = one(m);
            assert!(
                (s[0] + 2.2).abs() < 0.6 && (s[1] - 7.4).abs() < 0.6,
                "{m}: {s:?}"
            );
        }
        let (all, heading) = seat(&markers, travel).unwrap();
        assert!((all[2] + 36.1).abs() < 0.1);
        // Facing out of the chair, along +y.
        assert!(
            heading.min(std::f32::consts::TAU - heading) < 0.01,
            "{heading}"
        );
        assert!(seat(&markers, |_| None).is_none());
        // The game's settings for chair markers give nearly the same seat,
        // and every marker faces the sitter out along +y.
        let setting = |name: &str| match name {
            "fFurnitureMarker11DeltaX" => Some(2.6068),
            "fFurnitureMarker11DeltaY" => Some(53.961),
            "fFurnitureMarker11HeadingDelta" => Some(-std::f32::consts::FRAC_PI_2),
            "fFurnitureMarker12DeltaX" => Some(2.6669),
            "fFurnitureMarker12DeltaY" => Some(53.6492),
            "fFurnitureMarker12HeadingDelta" => Some(std::f32::consts::FRAC_PI_2),
            "fFurnitureMarker14DeltaX" => Some(2.4809),
            "fFurnitureMarker14DeltaY" => Some(57.3572),
            "fFurnitureMarker14HeadingDelta" => Some(std::f32::consts::PI),
            _ => None,
        };
        for m in &markers {
            let (s, h) = seat_by_settings(m, setting).unwrap();
            assert!(
                (s[0] + 2.2).abs() < 5.0 && (s[1] - 7.4).abs() < 5.0,
                "{m:?}: {s:?}"
            );
            assert!(h.min(std::f32::consts::TAU - h) < 0.01, "{m:?}: {h}");
        }
        assert!(seat_by_settings(&markers[0], |_| None).is_none());
    }

    fn idle(id: u32, name: &str, model: &str, parent: u32, previous: u32) -> Idle {
        let form = |v: u32| (v != 0).then_some(FormId(v));
        Idle {
            form_id: FormId(id),
            editor_id: name.into(),
            model: model.into(),
            conditions: Vec::new(),
            parent: form(parent),
            previous: form(previous),
            data: [0; 8],
        }
    }

    #[test]
    fn siblings_follow_their_links_and_the_first_passing_one_plays() {
        // A root group with three children stored out of order: Sit (first),
        // then Lean, then Smoke (a group with one animation).
        let idles: HashMap<FormId, Idle> = [
            idle(1, "Root", "Characters\\_Male\\IdleAnims", 0, 0),
            idle(4, "Smoke", "Characters\\_Male\\IdleAnims", 1, 3),
            idle(3, "Lean", "Lean.kf", 1, 2),
            idle(2, "Sit", "Sit.kf", 1, 0),
            idle(5, "Puff", "Puff.kf", 4, 0),
        ]
        .into_iter()
        .map(|i| (i.form_id, i))
        .collect();
        let tree = IdleTree::from_idles(idles);
        assert_eq!(
            tree.children(Some(FormId(1))),
            &[FormId(2), FormId(3), FormId(4)]
        );
        let all = |_: &Idle| true;
        assert_eq!(tree.pick(FormId(1), &all).unwrap().editor_id, "Sit");
        // Sit and Lean don't pass: down the Smoke group to Puff.
        let smoke = |i: &Idle| !matches!(i.editor_id.as_str(), "Sit" | "Lean");
        assert_eq!(tree.pick(FormId(1), &smoke).unwrap().editor_id, "Puff");
        assert!(tree.pick(FormId(1), &|_| false).is_none());
    }

    fn condition(function: u16, comparison: Comparison, value: f32) -> Condition {
        Condition {
            comparison,
            or: false,
            value,
            global: None,
            function,
            params: [0, 0],
            param_forms: [FormId(0), FormId(0)],
            run_on: 0,
            reference: None,
        }
    }

    use crate::dialogue::Comparison;
    const GET_SITTING: u16 = 159;
    const GET_RANDOM_PERCENT: u16 = 77;

    /// Two roots like the game's: `Furniture` (blocking, `GetSitting` > 0)
    /// with `Relax` (a seated idle when `GetSitting` is 3) and `Seat` (the
    /// seated loop at 1); then `General` (not blocking) with `Shrug`.
    fn game_like_tree() -> IdleTree {
        let mut furniture = idle(10, "Furniture", "Characters\\_Male\\IdleAnims", 0, 0);
        furniture.data[0] = 0x84;
        furniture.conditions = vec![condition(GET_SITTING, Comparison::Greater, 0.0)];
        let mut relax = idle(
            11,
            "Relax",
            "Characters\\_Male\\IdleAnims\\SitChairRelaxA.kf",
            10,
            0,
        );
        relax.data = [7, 1, 3, 0, 40, 0, 0, 0];
        relax.conditions = vec![condition(GET_SITTING, Comparison::Equal, 3.0)];
        let mut seat = idle(
            12,
            "Seat",
            "Characters\\_Male\\IdleAnims\\DynamicIdle_ChairSit.kf",
            10,
            11,
        );
        seat.conditions = vec![condition(GET_SITTING, Comparison::Equal, 1.0)];
        let general = idle(20, "General", "Characters\\_Male\\IdleAnims", 0, 10);
        let shrug = idle(21, "Shrug", "Characters\\_Male\\IdleAnims\\Shrug.kf", 20, 0);
        let mut loose = idle(30, "Loose", "Characters\\_Male\\IdleAnims\\Wave.kf", 0, 20);
        loose.data[0] = 0x47;
        let gecko = idle(40, "Gecko", "Creatures\\NV_Gecko\\IdleAnims", 0, 30);
        IdleTree::from_idles(
            [furniture, relax, seat, general, shrug, loose, gecko]
                .into_iter()
                .map(|i| (i.form_id, i))
                .collect(),
        )
    }

    #[test]
    fn the_walk_stops_at_blocking_folders_and_skips_delayed_idles() {
        let tree = game_like_tree();
        // Loose idles and other skeletons' folders aren't roots.
        let roots = tree.roots_for("Characters\\_Male\\Skeleton.nif");
        assert_eq!(roots, [FormId(10), FormId(20)]);
        assert_eq!(
            tree.roots_for("meshes\\creatures\\nv_gecko\\skeleton.nif"),
            [FormId(40)]
        );
        let asked = |sitting: u8| {
            IdleAsker::new(
                FormId(1),
                IdleQuestion {
                    sitting,
                    ..IdleQuestion::default()
                },
                None,
                1,
            )
        };
        let never = |_: FormId| false;
        let pick = |sitting: u8, delayed: &dyn Fn(FormId) -> bool| {
            let a = asked(sitting);
            tree.evaluate(&roots, &|i| a.passes(i), delayed)
                .map(|i| i.editor_id.clone())
        };
        // Standing: past the furniture branch to the general idles.
        assert_eq!(pick(0, &never).as_deref(), Some("Shrug"));
        assert_eq!(pick(3, &never).as_deref(), Some("Relax"));
        assert_eq!(pick(1, &never).as_deref(), Some("Seat"));
        // Seated, with the relax idle waiting out its replay delay: the
        // blocking folder answers, so nothing (not the standing shrug).
        assert_eq!(pick(3, &|id| id == FormId(11)), None);
        // Getting up (4) with no exit: also nothing.
        assert_eq!(pick(4, &never), None);
        let relax = tree.get(FormId(11)).unwrap();
        assert!(!relax.is_blocking() && relax.group() == 7);
        assert_eq!(relax.replay_delay(), 40);
        assert!(tree.get(FormId(10)).unwrap().is_blocking());
        assert!(tree.get(FormId(30)).unwrap().is_loose());
    }

    #[test]
    fn loop_counts_are_rolled_as_the_game_does() {
        let mut i = idle(1, "A", "A.kf", 0, 0);
        let set = |i: &mut Idle, lo: u8, hi: u8| {
            i.data[1] = lo;
            i.data[2] = hi;
        };
        set(&mut i, 0, 4);
        assert_eq!(i.extra_loops(|_, _| 9), 0);
        set(&mut i, 255, 255);
        assert_eq!(i.extra_loops(|_, _| 9), 255);
        set(&mut i, 1, 4);
        assert_eq!(i.extra_loops(|lo, hi| (lo + hi) / 2), 1);
        set(&mut i, 3, 3);
        assert_eq!(i.extra_loops(|_, _| 9), 2);
    }

    #[test]
    fn the_idle_clock_asks_once_a_second_of_free_time() {
        let mut clock = IdleClock::default();
        // Asked at once the first time (the timer starts at 0, then runs
        // below it), then after a second of free time.
        assert!(!clock.due(0.5, true));
        assert!(clock.due(0.1, true));
        assert!(!clock.due(0.6, true));
        assert!(!clock.due(5.0, false));
        assert!(!clock.due(0.6, true));
        assert!(clock.due(0.1, true));
        // An idle played twice (loops 2-2), with a 40 s replay delay; the
        // timer waits while it plays.
        let mut relax = idle(11, "Relax", "SitChairRelaxA.kf", 0, 0);
        relax.data = [7, 2, 2, 0, 40, 0, 0, 0];
        clock.start(&relax, 16.0, |_, _| unreachable!());
        assert!(clock.is_delayed(FormId(11)));
        assert!(!clock.due(5.0, true));
        assert!(!clock.advance(10.0));
        assert!(!clock.advance(10.0));
        assert_eq!(clock.playing.as_ref().unwrap().loops_left, 0);
        assert!(clock.advance(12.0));
        assert_eq!(clock.last, Some(FormId(11)));
        assert!(clock.playing.is_none());
        // 32 s in: still waiting out its 40.
        assert!(clock.is_delayed(FormId(11)));
        clock.advance(10.0);
        assert!(!clock.is_delayed(FormId(11)));
    }

    #[test]
    fn is_pc_1st_person_answers_the_players_view() {
        let index = (0..u16::MAX)
            .find(|&i| crate::functions::function_name(i) == "IsPC1stPerson")
            .unwrap();
        let c = condition(index, Comparison::Equal, 1.0);
        let ask = |first_person: bool| {
            IdleAsker::new(
                crate::dialogue::PLAYER_REF,
                IdleQuestion {
                    player: true,
                    first_person,
                    ..IdleQuestion::default()
                },
                None,
                1,
            )
            .value(&c)
        };
        assert_eq!(ask(true), 1.0);
        assert_eq!(ask(false), 0.0);
        assert!(!IdleQuestion::default().first_person);
    }

    /// A branch shaped like the game's `DialogueIdles` (`GetCurrentAIProcedure`
    /// 4 OR `MenuMode 1009`) with a happy talk (`IsTalking` 1,
    /// `GetDialogueEmotion` 5), a plain talk (`IsTalking` 1) and a listen.
    #[test]
    fn the_dialogue_branch_asks_the_menu_talking_and_the_emotion() {
        let index = |name: &str| {
            (0..u16::MAX)
                .find(|&i| crate::functions::function_name(i) == name)
                .unwrap()
        };
        let mut root = idle(20, "DialogueIdles", "Characters\\_Male\\IdleAnims", 0, 0);
        let mut procedure = condition(index("GetCurrentAIProcedure"), Comparison::Equal, 4.0);
        procedure.or = true;
        let mut menu = condition(index("MenuMode"), Comparison::Equal, 1.0);
        menu.params[0] = u32::from(DIALOG_MENU);
        root.conditions = vec![procedure, menu];
        let talking = condition(index("IsTalking"), Comparison::Equal, 1.0);
        let mut happy = idle(21, "Happy", "Characters\\_Male\\IdleAnims\\Happy.kf", 20, 0);
        happy.conditions = vec![
            talking.clone(),
            condition(index("GetDialogueEmotion"), Comparison::Equal, 5.0),
        ];
        let mut talk = idle(22, "Talk", "Characters\\_Male\\IdleAnims\\Talk.kf", 20, 21);
        talk.conditions = vec![talking];
        let listen = idle(
            23,
            "Listen",
            "Characters\\_Male\\IdleAnims\\Listen.kf",
            20,
            22,
        );
        let tree = IdleTree::from_idles(
            [root, happy, talk, listen]
                .into_iter()
                .map(|i| (i.form_id, i))
                .collect(),
        );
        let roots = tree.roots_for("Characters\\_Male\\Skeleton.nif");
        let pick = |about: IdleQuestion| {
            let asker = IdleAsker::new(FormId(1), about, None, 3);
            tree.evaluate(&roots, &|i| asker.passes(i), &|_| false)
                .map(|i| i.editor_id.clone())
        };
        let in_menu = IdleQuestion {
            menu: Some(DIALOG_MENU),
            procedure: procedures::NONE,
            ..IdleQuestion::default()
        };
        // Out of the menu and not in a dialogue procedure: nothing.
        assert_eq!(
            pick(IdleQuestion {
                procedure: procedures::NONE,
                ..IdleQuestion::default()
            }),
            None
        );
        // Another menu doesn't count.
        assert_eq!(
            pick(IdleQuestion {
                menu: Some(1003),
                ..in_menu
            }),
            None
        );
        assert_eq!(pick(in_menu).as_deref(), Some("Listen"));
        let saying = IdleQuestion {
            talking: true,
            ..in_menu
        };
        // No emotion to use (−1): the plain talk.
        assert_eq!(pick(saying).as_deref(), Some("Talk"));
        assert_eq!(
            pick(IdleQuestion {
                emotion: Some(5),
                ..saying
            })
            .as_deref(),
            Some("Happy")
        );
        // A dialogue procedure (people talking) passes without the menu.
        assert_eq!(
            pick(IdleQuestion {
                procedure: procedures::DIALOGUE,
                ..IdleQuestion::default()
            })
            .as_deref(),
            Some("Listen")
        );
    }

    #[test]
    fn random_percent_draws_afresh_for_each_condition() {
        let a = IdleAsker::new(FormId(1), IdleQuestion::default(), None, 7);
        let c = condition(GET_RANDOM_PERCENT, Comparison::Less, 50.0);
        let values: Vec<f64> = (0..20).map(|_| a.value(&c)).collect();
        assert!(values.iter().all(|v| (0.0..100.0).contains(v)));
        assert!(values.windows(2).any(|w| w[0] != w[1]), "{values:?}");
        // Asked about someone else: the general functions (none here: 0).
        let mut other = condition(GET_SITTING, Comparison::Equal, 0.0);
        other.run_on = 2;
        let seated = IdleAsker::new(
            FormId(1),
            IdleQuestion {
                sitting: 3,
                ..IdleQuestion::default()
            },
            None,
            7,
        );
        assert_eq!(seated.value(&other), 0.0);
        assert_eq!(
            seated.value(&condition(GET_SITTING, Comparison::Equal, 0.0)),
            3.0
        );
    }
}
