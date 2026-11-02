//! The test bot: `coop ...` lines in `--run` / `--run-at` (the co-op
//! harness's scenario scripts) do what a player would do through the live
//! layer, on the machine that has the line. No human playtest needed.
//!
//! ```text
//! coop hit REF DAMAGE          hit a person (a guest's hit is a request)
//! coop talk REF                open a dialogue session with a person
//! coop pick INFO               pick a line (the session's initiator)
//! coop linedone                this machine's lines are played
//! coop close                   leave the conversation
//! coop lootopen CONTAINER      open a container (host: registers it first)
//! coop loottake CONTAINER ITEM N
//! coop lootput CONTAINER ITEM N
//! coop lootclose CONTAINER
//! coop say TEXT...             chat
//! coop travel MARKER           fast travel (the party votes first)
//! coop tether REF | untether   host: story tether at a placed reference
//! coop revive SLOT | unrevive  hold the revive key next to a downed player
//! coop teleport X Y Z          put this machine's physical player at the
//!                              feet position (game units), as arriving
//!                              from a MoveTo or a door does
//! ```
//!
//! REF, INFO, ... are editor IDs or 8 digit hex form ids.

use bevy::prelude::*;
use coop_session::json::V;
use coop_session::{Intent, PlayerId};
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::facts::LootOp;
use world::dialogue::PLAYER_REF;

use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::scripts::Scripts;
use crate::GameFiles;

/// Lines waiting for their time (session clock seconds).
#[derive(Resource, Default)]
pub struct Bot {
    queue: Vec<(f32, String)>,
}

impl Bot {
    /// Queues a line to run at `at` (seconds since the app started).
    pub fn push_at(&mut self, at: f32, line: String) {
        self.queue.push((at, line));
    }
}

/// Whether a script line is for the bot.
pub fn is_bot_line(line: &str) -> bool {
    line.trim_start().to_ascii_lowercase().starts_with("coop ")
}

fn form(order: &esm::LoadOrder, word: &str) -> Option<FormId> {
    if word.eq_ignore_ascii_case("player") {
        return Some(PLAYER_REF);
    }
    order.form_by_editor_id(word).or_else(|| u32::from_str_radix(word, 16).ok().map(FormId))
}

/// Runs the lines that are due.
pub fn run(
    time: Res<Time>,
    mut bot: ResMut<Bot>,
    mut link: NonSendMut<CoopLink>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    markers: Option<Res<crate::map::MapMarkers>>,
    mut player: ResMut<crate::walk::Player>,
) {
    let t = time.elapsed_secs();
    if bot.queue.iter().all(|(at, _)| *at > t) {
        return;
    }
    let (due, later): (Vec<_>, Vec<_>) = std::mem::take(&mut bot.queue).into_iter().partition(|(at, _)| *at <= t);
    bot.queue = later;
    let order = &game.0.order;
    let cx = Ctx { order, scripts: &scripts.0 };
    let gs = &mut state.0;
    for (_, line) in due {
        link.0.log_event("script", &[("line", V::S(&line))]);
        let words: Vec<&str> = line.split_whitespace().collect();
        let arg = |i: usize| words.get(i).copied();
        let f = |i: usize| arg(i).and_then(|w| form(order, w));
        let n = |i: usize| arg(i).and_then(|w| w.parse::<u32>().ok()).unwrap_or(1);
        match arg(1).map(str::to_ascii_lowercase).as_deref() {
            Some("hit") => {
                if let (Some(who), Some(amount)) = (f(2), arg(3).and_then(|w| w.parse::<f64>().ok())) {
                    world::combat::hurt(order, gs, who, amount, PLAYER_REF);
                }
            }
            Some("talk") => {
                if let Some(npc) = f(2) {
                    link.0.live_intent(Intent::DialogueOpen(npc));
                }
            }
            Some("pick") => {
                if let Some(info) = f(2) {
                    link.0.live_intent(Intent::DialoguePick(info));
                }
            }
            Some("linedone") => link.0.live_intent(Intent::DialogueLineDone),
            Some("close") => link.0.live_intent(Intent::DialogueClose),
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
            Some("say") => link.0.live_intent(Intent::Say(words[2..].join(" "))),
            Some("travel") => {
                if let Some(m) = f(2) {
                    let found = markers.as_ref().and_then(|ms| ms.list.iter().find(|x| x.reference == m).cloned());
                    let marker = found.unwrap_or_else(|| world::map::MapMarker {
                        reference: m,
                        name: String::new(),
                        position: [0.0; 3],
                        radius: 0.0,
                        flags: 0,
                        kind: 0,
                        arrival: None,
                    });
                    let _ = world::map::travel(order, gs, &marker);
                }
            }
            Some("tether") => {
                if let Some(anchor) = f(2) {
                    link.0.live_tether(cx, gs, Some(anchor));
                }
            }
            Some("untether") => link.0.live_tether(cx, gs, None),
            Some("revive") => link.0.live_intent(Intent::ReviveBegin(PlayerId(n(2) as u8))),
            Some("teleport") => {
                let c = |i: usize| arg(i).and_then(|w| w.parse::<f32>().ok());
                if let (Some(x), Some(y), Some(z)) = (c(2), c(3), c(4)) {
                    player.arrive([x, y, z]);
                }
            }
            Some("unrevive") => link.0.live_intent(Intent::ReviveCancel),
            other => println!("Co-op bot: unknown line {other:?} in {line:?}"),
        }
    }
}
