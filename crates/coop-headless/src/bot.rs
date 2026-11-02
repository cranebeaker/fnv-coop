//! The `coop VERB ...` test-bot lines, as `viewer/src/coop_live/bot.rs` runs
//! them, and the stand-in for the viewer's dialogue menu.

use coop_session::{Intent, PlayerId, Session};
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::facts::LootOp;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use crate::data::Data;

/// Whether a script line is for the bot.
pub fn is_bot_line(line: &str) -> bool {
    line.trim_start().to_ascii_lowercase().starts_with("coop ")
}

/// What a bot line asks of the peer that the session cannot do itself.
#[derive(Debug, PartialEq)]
pub enum Out {
    /// Nothing more to do.
    None,
    /// Put the physical player here (feet).
    Teleport([f32; 3]),
    /// A verb the bot does not know (the viewer prints the same note).
    Unknown(String),
}

/// Runs one bot line on a peer (verbs and arguments as in the viewer).
pub fn run_line(line: &str, d: &Data, gs: &mut GameState, s: &mut Session, cx: Ctx) -> Out {
    let words: Vec<&str> = line.split_whitespace().collect();
    let arg = |i: usize| words.get(i).copied();
    let f = |i: usize| arg(i).and_then(|w| d.form(w));
    let n = |i: usize| arg(i).and_then(|w| w.parse::<u32>().ok()).unwrap_or(1);
    let order = &d.order;
    match arg(1).map(str::to_ascii_lowercase).as_deref() {
        Some("hit") => {
            if let (Some(who), Some(amount)) = (f(2), arg(3).and_then(|w| w.parse::<f64>().ok())) {
                world::combat::hurt(order, gs, who, amount, PLAYER_REF);
            }
        }
        Some("talk") => {
            if let Some(npc) = f(2) {
                s.live_intent(Intent::DialogueOpen(npc));
            }
        }
        Some("pick") => {
            if let Some(info) = f(2) {
                s.live_intent(Intent::DialoguePick(info));
            }
        }
        Some("linedone") => s.live_intent(Intent::DialogueLineDone),
        Some("close") => s.live_intent(Intent::DialogueClose),
        Some("lootopen") => {
            if let Some(c) = f(2) {
                gs.coop_loot(LootOp::Open(c));
            }
        }
        Some("loottake") => {
            if let (Some(container), Some(item)) = (f(2), f(3)) {
                gs.coop_loot(LootOp::Take { container, item, n: n(4) });
            }
        }
        Some("lootput") => {
            if let (Some(container), Some(item)) = (f(2), f(3)) {
                gs.coop_loot(LootOp::Put { container, item, n: n(4) });
            }
        }
        Some("lootclose") => {
            if let Some(c) = f(2) {
                gs.coop_loot(LootOp::Close(c));
            }
        }
        Some("say") => s.live_intent(Intent::Say(words[2..].join(" "))),
        Some("travel") => {
            if let Some(m) = f(2) {
                let marker = world::map::MapMarker { reference: m, name: String::new(), position: [0.0; 3], radius: 0.0, flags: 0, kind: 0, arrival: None };
                let _ = world::map::travel(order, gs, &marker);
            }
        }
        Some("tether") => {
            if let Some(anchor) = f(2) {
                s.live_tether(cx, gs, Some(anchor));
            }
        }
        Some("untether") => s.live_tether(cx, gs, None),
        Some("revive") => s.live_intent(Intent::ReviveBegin(PlayerId(n(2) as u8))),
        Some("unrevive") => s.live_intent(Intent::ReviveCancel),
        Some("teleport") => {
            let c = |i: usize| arg(i).and_then(|w| w.parse::<f32>().ok());
            if let (Some(x), Some(y), Some(z)) = (c(2), c(3), c(4)) {
                return Out::Teleport([x, y, z]);
            }
        }
        other => return Out::Unknown(format!("{other:?}")),
    }
    Out::None
}

/// The viewer's `intents::loot`: container actions the local player queued
/// are carried to the party (the host registers a container on first open).
pub fn carry_loot(d: &Data, gs: &mut GameState, s: &mut Session, cx: Ctx) {
    if gs.coop.live.loot_ops.is_empty() {
        return;
    }
    let host = s.role() == coop_session::Role::Host;
    let _ = d;
    for op in std::mem::take(&mut gs.coop.live.loot_ops) {
        match op {
            LootOp::Open(c) => {
                if host {
                    s.live_register_container(cx, gs, c);
                }
                s.live_intent(Intent::LootOpen(c));
            }
            LootOp::Take { container, item, n } => s.live_intent(Intent::LootTake { container, item, n }),
            LootOp::Put { container, item, n } => s.live_intent(Intent::LootPut { container, item, n }),
            LootOp::Close(c) => s.live_intent(Intent::LootClose(c)),
        }
    }
}

/// Seconds the stand-in reads a line before it reports it done (the
/// viewer's `READ_SECONDS` for spectators; the menu's own timing for the
/// speaker, which the stand-in does not have).
pub const READ_SECONDS: f64 = 2.5;

/// What the stand-in menu is doing.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Step {
    Idle,
    Opened,
    Picked(f64),
    LineDone(f64),
    Closed,
}

/// Stand-in for `--talk`: opens a session with the nearest person who has a
/// greeting, picks the greeting, finishes the line and closes. The viewer does
/// this through its dialogue menu (`intents::tap_dialogue`).
pub struct TalkBot {
    step: Step,
    npc: Option<FormId>,
    /// Counters for the dump.
    pub opened: bool,
}

impl Default for TalkBot {
    fn default() -> Self {
        TalkBot { step: Step::Idle, npc: None, opened: false }
    }
}

impl TalkBot {
    /// The nearest placed person with a greeting line, from the cell's people.
    pub fn nearest_speaker(d: &Data, gs: &GameState, from: [f32; 3], people: &[(FormId, FormId)]) -> Option<(FormId, FormId)> {
        let greeting = d.order.form_by_editor_id("GREETING")?;
        let mut best: Option<(f32, FormId, FormId)> = None;
        for &(p, base) in people {
            if !world::enabled_now(&d.order, p, &gs.disabled) {
                continue;
            }
            let Some((_, _, at, _)) = gs.place(&d.order, p) else { continue };
            let Some(sp) = world::dialogue::Speaker::load(&d.order, p, base) else { continue };
            let Some(info) = world::dialogue::pick(&d.order, greeting, &sp, gs) else { continue };
            let dist = ((at[0] - from[0]).powi(2) + (at[1] - from[1]).powi(2)).sqrt();
            if best.as_ref().is_none_or(|b| dist < b.0) {
                best = Some((dist, p, info.form_id));
            }
        }
        best.map(|(_, p, i)| (p, i))
    }

    /// Starts the conversation.
    pub fn open(&mut self, s: &mut Session, npc: FormId) {
        self.npc = Some(npc);
        self.step = Step::Opened;
        self.opened = true;
        s.live_intent(Intent::DialogueOpen(npc));
    }

    /// One frame: `view` is the last live view, `greeting` the line to pick.
    pub fn frame(&mut self, t: f64, s: &mut Session, view: Option<&coop_session::LiveView>, greeting: FormId) {
        let dv = view.and_then(|v| v.dialogue.as_ref());
        match self.step {
            Step::Opened if dv.is_some_and(|x| x.mine && x.playing.is_none()) => {
                s.live_intent(Intent::DialoguePick(greeting));
                self.step = Step::Picked(t);
            }
            Step::Picked(since) if dv.is_some_and(|x| x.playing.is_some()) && t - since >= 0.0 => {
                self.step = Step::LineDone(t + READ_SECONDS);
            }
            Step::LineDone(at) if t >= at => {
                s.live_intent(Intent::DialogueLineDone);
                self.step = Step::Closed;
            }
            Step::Closed if dv.is_some_and(|x| x.playing.is_none()) => {
                s.live_intent(Intent::DialogueClose);
                self.step = Step::Idle;
            }
            _ => {}
        }
    }
}
