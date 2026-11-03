//! `--coop-control PORT`: the live control port. `coop_session::control` runs the
//! JSON-lines TCP server on background threads (127.0.0.1 only); this plugin
//! answers its requests once a frame, on the Bevy side: state dump, script
//! lines, bot lines, key presses, screenshots, the entity list, quit.
//! tools/coop-run/live.py is the client. See tools/coop-run/README.md.

use std::sync::mpsc::Receiver;

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot, ScreenshotCaptured};
use coop_session::control::{self, Cmd, EntityRow, LiveStats, Request};
use coop_session::json::V;
use coop_session::Command;
use esm::FormId;

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::coop::{CoopLink, Run};
use crate::coop_live::{is_bot_line, Bot, CoopPuppet, LiveNow};
use crate::dialogue::DialogueState;
use crate::frame_order::ViewerSet;
use crate::scripts::Scripts;
use crate::test_keys::{self, TestKeys};
use crate::GameFiles;

/// Added by `CoopPlugin` when `--coop-control` is given.
pub struct ControlPlugin(pub u16);

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        match control::serve(self.0) {
            Ok((rx, addr)) => {
                println!("Co-op control: listening on {addr}");
                app.insert_non_send_resource(Control {
                    rx,
                    fps: 0.0,
                    quit_in: None,
                })
                .add_systems(
                    Update,
                    answer
                        .in_set(ViewerSet::AfterFrame)
                        .before(crate::coop::send),
                );
            }
            Err(e) => eprintln!("Co-op control: {e}"),
        }
    }
}

/// The request queue and a few figures.
struct Control {
    rx: Receiver<Request>,
    /// Smoothed frames a second.
    fps: f32,
    /// Frames until the exit (so the reply gets out first).
    quit_in: Option<u8>,
}

fn editor_name(order: &esm::LoadOrder, f: FormId) -> String {
    order
        .get(f)
        .and_then(|r| r.editor_id().ok().flatten())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{:08X}", f.0))
}

/// Any mesh under `e` in a camera's view this frame.
fn in_view(
    e: Entity,
    views: &Query<(Option<&ViewVisibility>, Option<&Children>)>,
    depth: u8,
) -> bool {
    let Ok((v, kids)) = views.get(e) else {
        return false;
    };
    v.is_some_and(|v| v.get())
        || (depth < 12 && kids.is_some_and(|k| k.iter().any(|c| in_view(c, views, depth + 1))))
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn answer(
    mut control: NonSendMut<Control>,
    mut link: NonSendMut<CoopLink>,
    (time, real): (Res<Time>, Res<Time<Real>>),
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut state: ResMut<DialogueState>,
    live: Res<LiveNow>,
    mut run: ResMut<Run>,
    mut bot: ResMut<Bot>,
    mut keys: ResMut<TestKeys>,
    player: Res<crate::walk::Player>,
    people: Query<(Entity, &Walker, Option<&CoopPuppet>, Option<&ActorRig>)>,
    views: Query<(Option<&ViewVisibility>, Option<&Children>)>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let inst = 1.0 / real.delta_secs().max(1e-4);
    control.fps = if control.fps == 0.0 {
        inst
    } else {
        control.fps * 0.95 + inst * 0.05
    };
    if let Some(n) = control.quit_in {
        if n == 0 {
            exit.write(AppExit::Success);
        }
        control.quit_in = Some(n.saturating_sub(1));
    }
    let order = &game.0.order;
    let rows = || -> Vec<EntityRow> {
        people
            .iter()
            .map(|(e, w, mark, rig)| {
                let (kind, who) = match mark {
                    Some(CoopPuppet(Some(p))) => ("puppet", Some(p.0)),
                    Some(CoopPuppet(None)) => ("remote", None),
                    None => ("npc", None),
                };
                let anim = rig.map_or(0, |r| {
                    u16::from(r.walking)
                        | u16::from(r.running) << 1
                        | u16::from(r.sneaking) << 2
                        | u16::from(r.fighting) << 3
                });
                EntityRow {
                    form: w.reference.0,
                    name: editor_name(order, w.reference),
                    pos: w.position,
                    heading: w.heading,
                    visible: in_view(e, &views, 0),
                    anim,
                    kind,
                    player: who,
                }
            })
            .collect()
    };
    while let Ok(Request { cmd, reply }) = control.rx.try_recv() {
        let name = cmd.name();
        let out = match cmd {
            Cmd::State => {
                let dump = crate::coop::dump_text(&link.0, &mut run, &state.0, &game, now, &live);
                let cell = state
                    .0
                    .player_cell
                    .map_or(String::new(), |c| editor_name(order, c));
                let c = &live.counters;
                let stats = LiveStats {
                    cell,
                    fps: control.fps,
                    t: now,
                    host: run.is_host(),
                    ready: player.ready && run.is_ready(),
                    rows: rows(),
                    counters: [c.lines_played, c.chat_in, c.travels, c.hits_sent],
                };
                control::state_reply(&dump, &stats)
            }
            Cmd::Entities => control::entities_reply(&rows()),
            Cmd::Bot { line } => {
                bot.push_at(now, line);
                control::ok(name, "\"how\":\"bot\"")
            }
            Cmd::Run { line, local } => {
                let local = local || line.starts_with("local ");
                let line = line.strip_prefix("local ").unwrap_or(&line).to_string();
                if is_bot_line(&line) {
                    bot.push_at(now, line);
                    control::ok(name, "\"how\":\"bot\"")
                } else if !player.ready {
                    control::fail(name, "the player is not placed yet")
                } else if local || run.is_host() {
                    link.0.log_event("script", &[("line", V::S(&line))]);
                    let mut runner = world::scripting::Runner::new(order, &scripts.0, &mut state.0);
                    let flow = runner.run_source(&line, None, None);
                    runner.run_item_scripts();
                    control::ok(
                        name,
                        &format!(
                            "\"how\":\"{}\",\"flow\":\"{}\"",
                            if run.is_host() { "host" } else { "local" },
                            coop_session::json::esc(&format!("{flow:?}"))
                        ),
                    )
                } else {
                    link.0.log_event("script", &[("line", V::S(&line))]);
                    link.0.send_command(Command::Line(line));
                    control::ok(name, "\"how\":\"sent\"")
                }
            }
            Cmd::Keys { key, hold, delay } => match test_keys::parse(0.0, &format!("{key}:{hold}"))
            {
                Ok(p) => {
                    keys.press_in(now, delay, p.what, p.hold);
                    control::ok(
                        name,
                        &format!(
                            "\"key\":\"{}\",\"hold\":{hold},\"delay\":{delay}",
                            coop_session::json::esc(&key)
                        ),
                    )
                }
                Err(e) => control::fail(name, &e),
            },
            Cmd::Screenshot { path } => {
                let p = std::path::PathBuf::from(&path);
                if let Some(dir) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
                    let _ = std::fs::create_dir_all(dir);
                }
                let shown = coop_session::json::esc(&path);
                // the reply goes out once the file is written (the observers run in order)
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(p))
                    .observe(move |_: Trigger<ScreenshotCaptured>| {
                        let _ =
                            reply.send(control::ok("screenshot", &format!("\"path\":\"{shown}\"")));
                    });
                continue;
            }
            Cmd::Quit => {
                control.quit_in = Some(5);
                control::ok(name, "")
            }
        };
        let _ = reply.send(out);
    }
}
