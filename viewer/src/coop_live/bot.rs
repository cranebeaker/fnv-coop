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
//! coop attack [KIND] [ARG]    a combat action (swing, shoot, aim, reload,
//!                              block, hurt, knockdown, death, draw,
//!                              holster) for the other players' puppets
//! coop say TEXT...             chat
//! coop ping [DISTANCE]         ping that far ahead (default 600 units)
//! coop waypoint X Y | unwaypoint   set or clear the waypoint
//! coop voice|mute|unmute|deafen|undeafen   need a microphone: logged and skipped here
//! coop travel MARKER           fast travel (the party votes first)
//! coop tether REF | untether   host: story tether at a placed reference
//! coop revive SLOT | unrevive  hold the revive key next to a downed player
//! coop barter VENDOR TOTAL [buy:ITEM:N] [sell:ITEM:N] ...
//! coop door REF | activate REF | pickup REF ITEM [N] | drop ITEM N
//! coop steal TARGET ITEM [N] [pickpocket|loot|plant]
//! coop craft BENCH RECIPE [N] | repair ITEM PART [VENDOR COST]
//! coop use ITEM | equip ITEM [on|off] | ammo ITEM | hotkey SLOT ITEM | note FORM
//! coop companion WHO follow|wait|dismiss|recruit|stimpak|gear
//! coop lockpick REF won|lost|broke [PINS] | hack REF won|lost | key REF KEY
//! coop trap REF disarm|detonate|destroy | discover MARKER
//! coop setlock REF LEVEL|none | give SLOT|REF ITEM [N]   (test setup, never on a guest)
//! coop givefor SLOT RECIPE|any:noskill [TIMES]   the recipe's ingredients (test setup)
//! coop enter DOOR FROMCELL TOCELL [oneway]   a load door (the party follows)
//! coop movie FILE [1]          host: a script plays a movie (1: interruptable)
//! coop lock 1111111 [QUEST]    host: a script disables the controls (seven 0/1)
//! coop unlock                  host: the script enables them again
//! coop initiate QUEST SLOT     a player starts the quest's dialogue (policy initiator)
//! coop cinematic QUEST | ending QUEST | slide N | endingdone | sceneabort
//! coop skip [yes|no]           vote to skip the scene on screen
//! coop wait N | sleep N        the sleep/wait menu's Begin (the party votes)
//! coop vote [yes|no]           answer the party prompt
//! coop npchit NPC SLOT DMG [limb]   an NPC's blow on player SLOT
//! coop pvp SLOT DMG | fire WEAPON X Y Z | explode EXPL RADIUS DMG
//! coop vats TARGET LIMB CHANCE DMG | vatsmark 0|1 | sneak 0|1
//! coop cripple LIMB | rads N | cure | spawnenc BASE N   (see world::coop::combat_live)
//! coop recruit|dismiss|command|down|downrevive|stash|level|objvar|fork|pcond|ffhit|hardcore ...
//!                              the campaign verbs, host only
//!                              (`world::coop::campaign::bot`), shared with the headless bot;
//!                              `give SLOT ...` and `vote QUEST SLOT yes|no` are theirs too
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
    order
        .form_by_editor_id(word)
        .or_else(|| u32::from_str_radix(word, 16).ok().map(FormId))
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
    mut book: ResMut<super::hud_social::Book>,
    mut live: ResMut<super::LiveNow>,
    mut answers: ResMut<crate::game_menus::AnswerBoxes>,
    mut say: ResMut<crate::dialogue::AutoSay>,
) {
    let t = time.elapsed_secs();
    if bot.queue.iter().all(|(at, _)| *at > t) {
        return;
    }
    let (due, later): (Vec<_>, Vec<_>) = std::mem::take(&mut bot.queue)
        .into_iter()
        .partition(|(at, _)| *at <= t);
    bot.queue = later;
    let order = &game.0.order;
    let cx = Ctx {
        order,
        scripts: &scripts.0,
    };
    let gs = &mut state.0;
    for (_, line) in due {
        link.0.log_event("script", &[("line", V::S(&line))]);
        let words: Vec<&str> = line.split_whitespace().collect();
        let arg = |i: usize| words.get(i).copied();
        let f = |i: usize| arg(i).and_then(|w| form(order, w));
        let n = |i: usize| arg(i).and_then(|w| w.parse::<u32>().ok()).unwrap_or(1);
        // the campaign verbs (down, give SLOT, level, objvar, pcond, hardcore ...), host only,
        // shared with the headless simulator's bot
        if world::coop::campaign::bot::bot_line(cx, gs, &words, &|w| form(order, w)) {
            continue;
        }
        match arg(1).map(str::to_ascii_lowercase).as_deref() {
            Some("hit") => {
                if let (Some(who), Some(amount)) =
                    (f(2), arg(3).and_then(|w| w.parse::<f64>().ok()))
                {
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
            // `coop topic TEXT`: the local player picks a topic in the game's
            // dialogue menu, the first offered one whose text contains TEXT,
            // as soon as the menu offers topics (the `--say` pick, timed).
            // `coop topic Goodbye.` ends a conversation as a player does.
            Some("topic") if words.len() > 2 => say.0.push_back(words[2..].join(" ")),
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
                        form(order, target),
                    );
                }
            }
            Some("givefor") => {
                let recipe = arg(3).and_then(|w| {
                    world::coop::interact::special_form(order, w).or_else(|| form(order, w))
                });
                if let (Some(slot), Some(recipe)) = (arg(2), recipe) {
                    world::coop::interact::bot_give_ingredients(order, gs, slot, recipe, n(4));
                }
            }
            Some("attack") => {
                let kind = arg(2)
                    .and_then(coop_session::ActionKind::parse)
                    .unwrap_or(coop_session::ActionKind::Swing);
                gs.coop
                    .live
                    .local_actions
                    .push((kind, arg(3).and_then(|w| w.parse().ok()).unwrap_or(0)));
            }
            Some("say") => link.0.live_intent(Intent::Say(words[2..].join(" "))),
            Some("ping") => {
                let d = arg(2).and_then(|w| w.parse::<f32>().ok()).unwrap_or(600.0);
                let p = gs.player_position.unwrap_or([0.0; 3]);
                let h = gs.player_heading;
                let at = [p[0] + h.sin() * d, p[1] + h.cos() * d, p[2]];
                let st = link.0.status();
                let cell = gs.player_cell.map_or(0, |c| c.0);
                if let Some(msgs) = book.0.ping(st.player, cell, at, "ping", st.now_ms) {
                    live.counters.pings += 1;
                    for m in msgs {
                        link.0.live_intent(Intent::Social(m));
                    }
                }
            }
            Some("waypoint") => {
                let p = gs.player_position.unwrap_or([0.0; 3]);
                let x = arg(2).and_then(|w| w.parse::<f32>().ok()).unwrap_or(p[0]);
                let y = arg(3).and_then(|w| w.parse::<f32>().ok()).unwrap_or(p[1]);
                let st = link.0.status();
                let m = book.0.waypoint(
                    st.player,
                    gs.player_cell.map_or(0, |c| c.0),
                    [x, y, p[2]],
                    "waypoint",
                );
                live.counters.pings += 1;
                link.0.live_intent(Intent::Social(m));
            }
            Some("unwaypoint") => {
                if let Some(m) = book.0.clear_waypoint() {
                    link.0.live_intent(Intent::Social(m));
                }
            }
            Some("voice" | "mute" | "unmute" | "deafen" | "undeafen") => {
                println!("Co-op bot: '{line}' needs a microphone, skipped (the headless simulator models it)");
            }
            Some("travel") => {
                if let Some(m) = f(2) {
                    let found = markers
                        .as_ref()
                        .and_then(|ms| ms.list.iter().find(|x| x.reference == m).cloned());
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
            Some("revive") => link
                .0
                .live_intent(Intent::ReviveBegin(PlayerId(n(2) as u8))),
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
            Some("vote") => link.0.live_intent(Intent::Answer(arg(2) != Some("no"))),
            // `coop button N`: the local player clicks button N of the message box
            // (the open one, else the next one with more than one button). The
            // click closes the menu, and `game_menus::message::after` sets the
            // button that GetButtonPressed reads.
            Some("button") => {
                if let Some(b) = arg(2).and_then(|w| w.parse::<usize>().ok()) {
                    answers.choices.push_back(b);
                }
            }
            Some("initiate") => {
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
                .push((FormId(n(2)), coop_session::SceneKind::Cinematic)),
            Some("ending") => gs
                .coop
                .live
                .scene
                .stage_notes
                .push((FormId(n(2)), coop_session::SceneKind::Ending)),
            Some("slide") => gs.coop_scene_slide(n(2) as u16),
            Some("endingdone") => gs.coop_scene_ending_done(),
            Some("sceneabort") => gs.coop_scene_abort(),
            Some("teleport") => {
                let c = |i: usize| arg(i).and_then(|w| w.parse::<f32>().ok());
                if let (Some(x), Some(y), Some(z)) = (c(2), c(3), c(4)) {
                    player.arrive([x, y, z]);
                }
            }
            Some("unrevive") => link.0.live_intent(Intent::ReviveCancel),
            Some("ff") => link.0.live_friendly_fire(n(2) as u8),
            Some("spawnenc") => {
                if let (Some(base), Some(cell)) = (f(2), gs.player_cell) {
                    let base =
                        world::more_functions::placed::base_now(order, gs, base).unwrap_or(base);
                    let at = gs.player_position.unwrap_or([0.0; 3]);
                    link.0.live_spawn_encounter(base, cell, at, n(3), 20);
                }
            }
            // combat verbs (npchit, pvp, fire, explode, vats, sneak ...) are shared with the headless simulator
            _ if world::coop::combat_live::bot_line(order, gs, &words, &|w| form(order, w)) => {}
            other => {
                // player-world interactions (barter, doors, locks, crafting ...)
                let here = (
                    gs.player_cell.map_or(0, |c| c.0),
                    gs.player_position
                        .map_or([0; 3], |p| [p[0] as i32, p[1] as i32, p[2] as i32]),
                );
                match world::coop::interact::parse_act(
                    &words[1..],
                    &|w| form(order, w).or_else(|| world::coop::interact::special_form(order, w)),
                    here,
                ) {
                    Some(act) => {
                        let enter = match &act {
                            coop_session::interact::Act::Transition { door, to, .. } => {
                                Some((*door, *to))
                            }
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
                    None => println!("Co-op bot: unknown line {other:?} in {line:?}"),
                }
            }
        }
    }
}
