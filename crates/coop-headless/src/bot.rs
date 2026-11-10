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
    /// Turn the physical player to this heading, radians (the mouse look).
    Face(f32),
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
    if let Some(v) = arg(1).map(str::to_ascii_lowercase) {
        if crate::campaign_bot::run(&v, &words, d, gs, cx) {
            return Out::None;
        }
    }
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
        // `coop topic TEXT` picks a topic in the viewer's game menu. Modelled:
        // the stand-in conversation has no topics, and only the Goodbye. pick
        // has an effect here (it ends the conversation).
        Some("topic") => {
            if words[2..]
                .join(" ")
                .to_ascii_lowercase()
                .starts_with("goodbye")
            {
                s.live_intent(Intent::DialogueClose);
            }
        }
        Some("lootopen") => {
            if let Some(c) = f(2) {
                gs.coop_loot(LootOp::Open(c));
            }
        }
        Some("loottake") => {
            if let (Some(container), Some(item)) = (f(2), f(3)) {
                gs.coop_loot(LootOp::Take {
                    container,
                    item,
                    n: n(4),
                });
            }
        }
        Some("lootput") => {
            if let (Some(container), Some(item)) = (f(2), f(3)) {
                gs.coop_loot(LootOp::Put {
                    container,
                    item,
                    n: n(4),
                });
            }
        }
        Some("lootclose") => {
            if let Some(c) = f(2) {
                gs.coop_loot(LootOp::Close(c));
            }
        }
        Some("give") => {
            if let (Some(target), Some(item)) = (arg(2), f(3)) {
                world::coop::interact::bot_give(
                    order,
                    gs,
                    target,
                    item,
                    n(4) as i32,
                    d.form(target),
                );
            }
        }
        Some("givefor") => {
            let recipe = arg(3)
                .and_then(|w| world::coop::interact::special_form(order, w).or_else(|| d.form(w)));
            if let (Some(slot), Some(recipe)) = (arg(2), recipe) {
                world::coop::interact::bot_give_ingredients(order, gs, slot, recipe, n(4));
            }
        }
        Some("attack") => {
            // `coop attack [KIND] [ARG]`: a combat action the other peers' puppets play
            let kind = arg(2)
                .and_then(coop_core::avatar::ActionKind::parse)
                .unwrap_or(coop_core::avatar::ActionKind::Swing);
            gs.coop
                .live
                .local_actions
                .push((kind, arg(3).and_then(|w| w.parse().ok()).unwrap_or(0)));
        }
        Some("say") => s.live_intent(Intent::Say(words[2..].join(" "))),
        // fault injection: garble one record of this guest's replicated store
        // (the hash exchange must find it, the host must repair it)
        Some("desync") => {
            s.inject_desync();
        }
        Some("travel") => {
            if let Some(m) = f(2) {
                let marker = world::map::MapMarker {
                    reference: m,
                    name: String::new(),
                    position: [0.0; 3],
                    radius: 0.0,
                    flags: 0,
                    kind: 0,
                    arrival: None,
                };
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
        // scenes and time (a script's PlayBink, DisablePlayerControls, a stage the
        // policy marks, the sleep/wait menu); the viewer's bot runs the same verbs
        Some("movie") => {
            if let Some(file) = arg(2) {
                let v = world::scripting::Video {
                    file: file.to_string(),
                    interruptable: arg(3) == Some("1"),
                    mute_audio: true,
                    pause_music: true,
                    letterbox: true,
                };
                if gs.coop.authority.replicates() {
                    gs.coop.live.scene.note_video(None, &v);
                }
            }
        }
        Some("lock") => {
            // the policy row by editor id, else an editor id or a hex form id of the load order
            let quest = arg(3)
                .and_then(|w| gs.coop_policy_quest(w))
                .or_else(|| f(3));
            gs.coop_controls_begin();
            for (i, c) in arg(2).unwrap_or("").chars().take(7).enumerate() {
                gs.controls_off[i] = c == '1';
            }
            gs.coop_controls_end(quest);
        }
        Some("unlock") => {
            gs.coop_controls_begin();
            gs.controls_off = [false; 7];
            gs.coop_controls_end(None);
        }
        Some("skip") => gs.coop_scene_skip(arg(2) != Some("no")),
        Some("vote") => s.live_intent(Intent::Answer(arg(2) != Some("no"))),
        // `coop button N`: the local player presses button N of the open message box (the
        // viewer's message menu sets the same field, `game_menus::message::after`)
        Some("button") => {
            if let Some(b) = arg(2).and_then(|w| w.parse::<i32>().ok()) {
                gs.button = Some(b);
            }
        }
        Some("initiate") => {
            // a player starts the dialogue or activation of a quest (the policy's initiator)
            let q = arg(2)
                .and_then(|w| gs.coop_policy_quest(w))
                .or_else(|| f(2))
                .map(|q| q.0);
            if let (Some(q), Some(g)) = (q, gs.coop.gate.as_mut()) {
                g.begin_interaction(FormId(q), n(3) as u8);
            }
        }
        Some("wait") => world::living::sleep::begin(gs, n(2), false),
        Some("sleep") => world::living::sleep::begin(gs, n(2), true),
        Some("cinematic") => gs
            .coop
            .live
            .scene
            .stage_notes
            .push((FormId(n(2)), coop_core::scene::SceneKind::Cinematic)),
        Some("ending") => gs
            .coop
            .live
            .scene
            .stage_notes
            .push((FormId(n(2)), coop_core::scene::SceneKind::Ending)),
        Some("slide") => gs.coop_scene_slide(n(2) as u16),
        Some("endingdone") => gs.coop_scene_ending_done(),
        Some("sceneabort") => gs.coop_scene_abort(),
        Some("teleport") => {
            let c = |i: usize| arg(i).and_then(|w| w.parse::<f32>().ok());
            if let (Some(x), Some(y), Some(z)) = (c(2), c(3), c(4)) {
                return Out::Teleport([x, y, z]);
            }
        }
        Some("face") => {
            // `coop face DEG`: the player turns to that heading (degrees clockwise from north)
            if let Some(deg) = arg(2).and_then(|w| w.parse::<f32>().ok()) {
                return Out::Face(deg.to_radians().rem_euclid(std::f32::consts::TAU));
            }
        }
        Some("cell") => {
            // `coop cell CELL`: the player's scene becomes that cell (an editor id or a hex form id;
            // the cell need not exist: the live layer only sees the id)
            if let Some(c) = f(2) {
                gs.player_cell = Some(c);
                // an exterior cell is in its worldspace, as the viewer has it
                gs.player_world = d.world_of_cell(c);
            }
        }
        Some("ff") => s.live_friendly_fire(n(2) as u8),
        Some("spawnenc") => {
            // coop spawnenc BASE|REF N: an encounter of N creatures through the host's arbiter
            if let (Some(base), Some(cell)) = (f(2), gs.player_cell) {
                let base = world::more_functions::placed::base_now(order, gs, base).unwrap_or(base);
                let at = gs.player_position.unwrap_or([0.0; 3]);
                s.live_spawn_encounter(base, cell, at, n(3), 20);
            }
        }
        _ if world::coop::combat_live::bot_line(order, gs, &words, &|w| d.form(w)) => {}
        other => {
            // player-world interactions (barter, doors, locks, crafting ...)
            let here = (
                gs.player_cell.map_or(0, |c| c.0),
                gs.player_position
                    .map_or([0; 3], |p| [p[0] as i32, p[1] as i32, p[2] as i32]),
            );
            let Some(act) = world::coop::interact::parse_act(
                &words[1..],
                &|w| {
                    d.form(w)
                        .or_else(|| world::coop::interact::special_form(order, w))
                },
                here,
            ) else {
                return Out::Unknown(format!("{other:?}"));
            };
            let enter = match &act {
                coop_session::interact::Act::Transition { door, to, .. } => Some((*door, *to)),
                _ => None,
            };
            gs.coop_do(cx, act);
            // a bot has no door to walk through: it arrives as a door would
            if let (Some((door, to)), false) =
                (enter, gs.coop.authority == world::coop::Authority::Guest)
            {
                gs.coop.interact.follow = Some(coop_session::interact::Follow {
                    door,
                    to,
                    leader: PlayerId(0),
                });
            }
        }
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
            // the host's container screen moves the host player's items itself (the stand-in
            // does what the screen does); a guest's screen only asks
            LootOp::Take { container, item, n } if host => {
                gs.move_item(
                    cx.order,
                    container,
                    world::dialogue::PLAYER_REF,
                    item,
                    n as i32,
                );
            }
            LootOp::Put { container, item, n } if host => {
                gs.move_item(
                    cx.order,
                    world::dialogue::PLAYER_REF,
                    container,
                    item,
                    n as i32,
                );
            }
            LootOp::Take { container, item, n } => {
                s.live_intent(Intent::LootTake { container, item, n })
            }
            LootOp::Put { container, item, n } => {
                s.live_intent(Intent::LootPut { container, item, n })
            }
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
    /// The menu stays up after the first line (a script's conversation the
    /// player has not left).
    pub stay: bool,
}

impl Default for TalkBot {
    fn default() -> Self {
        TalkBot {
            step: Step::Idle,
            npc: None,
            opened: false,
            stay: false,
        }
    }
}

impl TalkBot {
    /// The nearest placed person with a greeting line, from the cell's people.
    pub fn nearest_speaker(
        d: &Data,
        gs: &GameState,
        from: [f32; 3],
        people: &[(FormId, FormId)],
    ) -> Option<(FormId, FormId)> {
        let greeting = d.order.form_by_editor_id("GREETING")?;
        let mut best: Option<(f32, FormId, FormId)> = None;
        for &(p, base) in people {
            if !world::enabled_now(&d.order, p, &gs.disabled) {
                continue;
            }
            let Some((_, _, at, _)) = gs.place(&d.order, p) else {
                continue;
            };
            let Some(sp) = world::dialogue::Speaker::load(&d.order, p, base) else {
                continue;
            };
            let Some(info) = world::dialogue::pick(&d.order, greeting, &sp, gs) else {
                continue;
            };
            let dist = ((at[0] - from[0]).powi(2) + (at[1] - from[1]).powi(2)).sqrt();
            if best.as_ref().map_or(true, |b| dist < b.0) {
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
    pub fn frame(
        &mut self,
        t: f64,
        s: &mut Session,
        view: Option<&coop_session::LiveView>,
        greeting: FormId,
    ) {
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
                if !self.stay {
                    s.live_intent(Intent::DialogueClose);
                }
                self.step = Step::Idle;
            }
            _ => {}
        }
    }
}
