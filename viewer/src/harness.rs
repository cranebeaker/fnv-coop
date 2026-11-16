//! `NV_HARNESS=FILE` (a test aid for co-op's two-game harness): lines appended to FILE while the
//! game runs are commands, each run once, between two of main's frames, in order. Output goes to
//! the console log, each line starting `harness:`.
//!
//! - `con TEXT`: TEXT run as a console line, as the console runs one (`Script::CompileAndRun` with
//!   the console's compiler): `player.moveto SunnyREF`, `SunnyREF.activate player`, ...
//! - `load NAME`: the save called NAME loaded through the save manager's `LoadGame`
//!   (`00850760`), as the quick load does.
//! - `key HEX FRAMES`: the key with DirectInput code HEX held for FRAMES of main's frames.
//! - `mouse DX DY FRAMES [BUTTONS]`: the mouse moved by (DX, DY) on each of FRAMES frames, with the
//!   buttons of the bit mask BUTTONS (bit 0 the left one) held.
//! - `warp SPACEHEX X Y Z`: the player moved there as a door moves it (co-op's `live::warp`).
//! - `wait FRAMES`: the next commands wait that many frames.
//! - `pos`: the player's place and space. `actor HEX`: an actor's place, space, life and package.
//!   `quest HEX`: a quest's stage, done stages and objectives. `var HEX`: a quest's variables.
//! - `say TEXT`: TEXT printed (to mark a step in the log). `menu`: the dialogue's speaker, if open.

use std::io::Read as _;

use engine_core::prelude::*;

/// The save manager singleton and its `LoadGame` (name, device, statistics, quiet).
const SAVE_MANAGER: u32 = 0x011d_e134;
const SAVE_MANAGER_LOAD: u32 = 0x0085_0760;
/// `MenuConsole::GetSingleton(create)`; the console's compiler is its first word.
const CONSOLE_INSTANCE: u32 = 0x0071_b160;
/// `Script::Script`, `SetTemporary`, `SetText`, `CompileAndRun`, the destructor; its size.
const SCRIPT_CONSTRUCT: u32 = 0x005a_a0f0;
const SCRIPT_SET_TEMPORARY: u32 = 0x0048_4490;
const SCRIPT_SET_TEXT: u32 = 0x005a_be50;
const SCRIPT_COMPILE_AND_RUN: u32 = 0x005a_c400;
const SCRIPT_DESTRUCT: u32 = 0x005a_a1a0;
const SCRIPT_SIZE: u32 = 0x54;

/// The harness: the command file, how much of it was read, and what is held.
pub struct Harness {
    path: std::path::PathBuf,
    read: usize,
    pending: std::collections::VecDeque<String>,
    frame: u64,
    wait_until: u64,
    keys: Vec<(u8, u64)>,
    mouse: Vec<(i32, i32, u64, u8)>,
}

impl Harness {
    /// The harness when `NV_HARNESS` names a file (made empty if missing).
    pub fn from_env() -> Option<Self> {
        let path = std::path::PathBuf::from(std::env::var_os("NV_HARNESS")?);
        install();
        if !path.exists() {
            let _ = std::fs::write(&path, b"");
        }
        Some(Self {
            path,
            read: 0,
            pending: Default::default(),
            frame: 0,
            wait_until: 0,
            keys: Vec::new(),
            mouse: Vec::new(),
        })
    }

    /// The keys held for the next frame.
    pub fn keys(&self) -> Vec<u8> {
        self.keys
            .iter()
            .filter(|(_, until)| *until > self.frame)
            .map(|(k, _)| *k)
            .collect()
    }

    /// The mouse motion and buttons for the next frame.
    pub fn mouse(&self) -> ([i32; 3], [bool; 8]) {
        let mut motion = [0; 3];
        let mut buttons = [false; 8];
        for (dx, dy, until, b) in &self.mouse {
            if *until > self.frame {
                motion[0] += dx;
                motion[1] += dy;
                for (i, slot) in buttons.iter_mut().enumerate() {
                    *slot |= b & (1 << i) != 0;
                }
            }
        }
        (motion, buttons)
    }

    /// One step after one of main's frames: new lines read, due commands run.
    pub fn after_frame(&mut self, e: &mut Engine) -> Vec<String> {
        self.frame += 1;
        self.keys.retain(|(_, until)| *until > self.frame);
        self.mouse.retain(|m| m.2 > self.frame);
        let mut out = Vec::new();
        if let Ok(mut f) = std::fs::File::open(&self.path) {
            let mut text = String::new();
            if f.read_to_string(&mut text).is_ok() && text.len() > self.read {
                let new = &text[self.read..];
                // Only whole lines.
                if let Some(end) = new.rfind('\n') {
                    for line in new[..end].lines() {
                        let line = line.trim();
                        if !line.is_empty() {
                            self.pending.push_back(line.to_string());
                        }
                    }
                    self.read += end + 1;
                }
            }
        }
        while self.frame >= self.wait_until {
            let Some(line) = self.pending.pop_front() else {
                break;
            };
            out.push(format!("harness: [{}] {line}", self.frame));
            // Game calls run on the game's main thread, at the end of its next frame.
            let word = line.split(' ').next().unwrap_or("");
            if matches!(word, "key" | "mouse" | "wait" | "say") {
                out.extend(
                    self.run(e, &line)
                        .into_iter()
                        .map(|l| format!("harness: {l}")),
                );
            } else {
                lock(&TO_MAIN).push_back(line);
            }
        }
        out.extend(
            std::mem::take(&mut *lock(&FROM_MAIN))
                .into_iter()
                .map(|l| format!("harness: {l}")),
        );
        out
    }

    fn run(&mut self, e: &mut Engine, line: &str) -> Vec<String> {
        let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
        let rest = rest.trim();
        let hex = |s: &str| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok();
        match word {
            "con" => vec![console(e, rest)],
            "load" => vec![load(e, rest)],
            // `saveout DIR`: the saves this game has written (kept in memory) copied to DIR.
            "saveout" => host::save_out::copy_changed(
                e,
                std::path::Path::new(rest),
                &mut std::collections::HashMap::new(),
            ),
            "key" => {
                let mut p = rest.split_whitespace();
                match (
                    p.next().and_then(|k| u8::from_str_radix(k, 16).ok()),
                    p.next(),
                ) {
                    (Some(k), Some(n)) => {
                        let n: u64 = n.parse().unwrap_or(1);
                        self.keys.push((k, self.frame + n));
                        vec![]
                    }
                    _ => vec!["key HEX FRAMES".into()],
                }
            }
            "mouse" => {
                let v: Vec<i64> = rest
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if v.len() < 3 {
                    return vec!["mouse DX DY FRAMES [BUTTONS]".into()];
                }
                self.mouse.push((
                    v[0] as i32,
                    v[1] as i32,
                    self.frame + v[2] as u64,
                    v.get(3).copied().unwrap_or(0) as u8,
                ));
                vec![]
            }
            "wait" => {
                self.wait_until = self.frame + rest.parse::<u64>().unwrap_or(0);
                vec![]
            }
            // Fast travel to a map marker reference, as the Pip-Boy map does: a position request
            // whose fast-travel target (+0x30) is the marker (`PlayerCharacter::FastTravel`).
            "travel" => match hex(rest.trim()).and_then(|id| coop_engine::forms::form(e, id)) {
                Some(marker) => {
                    let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                    let request = e
                        .call(
                            coop_engine::addr::OPERATOR_NEW,
                            &args![coop_engine::addr::POSITION_REQUEST_SIZE],
                        )
                        .u32();
                    let request = e
                        .call(
                            coop_engine::addr::POSITION_REQUEST_CONSTRUCT,
                            &args![request],
                        )
                        .u32();
                    e.mem.set_u32(request + 0x30, marker);
                    e.call(
                        coop_engine::addr::REQUEST_POSITION_PLAYER,
                        &args![player, request],
                    );
                    vec![format!("travel {}: asked", rest.trim())]
                }
                None => vec!["travel MARKERHEX".into()],
            },
            "warp" => {
                let p: Vec<&str> = rest.split_whitespace().collect();
                let v: Vec<f32> = p.iter().skip(1).filter_map(|s| s.parse().ok()).collect();
                match (p.first().and_then(|s| hex(s)), v.len()) {
                    (Some(space), 3) => vec![format!(
                        "warp: {:?}",
                        coop_engine::live::warp(e, space, [v[0], v[1], v[2]], 0.0)
                    )],
                    _ => vec!["warp SPACEHEX X Y Z".into()],
                }
            }
            // The open dialogue: `dnext` clicks as on the shown text (on to the next screen);
            // `dtopic N` clicks the list's Nth topic tile (0 the first); `dtopics` counts them.
            "dnext" | "dtopic" | "dtopics" => {
                let menu = e.mem.u32(coop_engine::addr::DIALOG_MENU_OPEN);
                if menu == 0 {
                    return vec!["no dialogue open".into()];
                }
                let mut tiles = Vec::new();
                let mut node = menu + 0x40 + 4;
                while node != 0 && tiles.len() < 32 {
                    let item = e.mem.u32(node);
                    if item != 0 {
                        tiles.push(e.mem.u32(item));
                    }
                    node = e.mem.u32(node + 4);
                }
                match word {
                    "dnext" => {
                        e.vcall(menu, 0xc, &args![0u32, 0u32]);
                        vec!["dnext: clicked".into()]
                    }
                    // With the line being said: `MenuTopicManager::pCurrentTopic` (+0x70, +0x00),
                    // a list node whose item is the `MenuTopic`, and its `pInfo` (+0x18). Each
                    // pointer is checked before it is read (a wrong one stops the game).
                    "dtopics" => {
                        let read = |e: &mut Engine, at: u32| {
                            if at != 0 && e.mem.is_mapped(at) && e.mem.is_mapped(at + 3) {
                                e.mem.u32(at)
                            } else {
                                0
                            }
                        };
                        let node = read(e, menu + 0x70);
                        let node = read(e, node);
                        let topic = read(e, node);
                        let info = read(e, topic.wrapping_add(0x18) * (topic != 0) as u32);
                        let id = read(
                            e,
                            info.wrapping_add(coop_engine::addr::FORM_ID_OFFSET)
                                * (info != 0) as u32,
                        );
                        vec![format!("{} topics; saying info {id:08x}", tiles.len())]
                    }
                    _ => match rest.parse::<usize>().ok().and_then(|n| tiles.get(n)) {
                        Some(&tile) => {
                            e.vcall(menu, 0xc, &args![0xffff_ffffu32, tile]);
                            vec![format!("dtopic {rest}: clicked")]
                        }
                        None => vec![format!("dtopic {rest}: {} topics", tiles.len())],
                    },
                }
            }
            "ref" => {
                match hex(rest).and_then(|id| coop_engine::forms::form(e, id).map(|r| (id, r))) {
                    Some((id, r)) => {
                        let pos = coop_engine::fight::place_of_ptr(e, r);
                        let space = coop_engine::fight::space_of(e, r);
                        // Its parent cell (+0x40), whether the game has that cell loaded
                        // (`004511e0`), and its 3D (vtable +0x1d0): what makes it seen.
                        let cell = e.mem.u32(r + coop_engine::addr::REF_CELL_OFFSET);
                        let cell_id = if cell != 0 {
                            e.mem.u32(cell + coop_engine::addr::FORM_ID_OFFSET)
                        } else {
                            0
                        };
                        let tes = e.mem.u32(coop_engine::addr::TES_SINGLETON);
                        let loaded = cell != 0
                            && tes != 0
                            && e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool();
                        let node = e.vcall(r, 0x1d0, &args![]).u32();
                        vec![format!(
                            "ref {id:08x}: at ({:.0}, {:.0}, {:.0}) in {space:08x}, disabled {:?}, cell {cell_id:08x} loaded {loaded}, 3D {}",
                            pos[0],
                            pos[1],
                            pos[2],
                            coop_engine::actors::disabled(e, id),
                            node != 0
                        )]
                    }
                    None => vec![format!("ref {rest}: not here")],
                }
            }
            "global" => match hex(rest).and_then(|id| coop_engine::forms::global_value(e, id)) {
                Some(v) => vec![format!("global {rest}: {v}")],
                None => vec![format!("global {rest}: not a global")],
            },
            "input" => vec![format!(
                "input devices (keyboard, acquired, buffer, waiting): {:?}; controls {:?}",
                engine_platform::dinput::devices(e),
                coop_engine::fight::controls(e)
            )],
            // Game calls without the console: a quest's stage set as `SetStage` does (its result
            // runs), an actor's package evaluated as `evp` does, an actor killed by the player.
            "stage" => {
                let p: Vec<&str> = rest.split_whitespace().collect();
                match (
                    p.first().and_then(|s| hex(s)),
                    p.get(1).and_then(|s| s.parse().ok()),
                ) {
                    (Some(q), Some(n)) => vec![format!(
                        "stage {:08x} {n}: {}",
                        q,
                        coop_engine::forms::set_quest_stage(e, q, n)
                    )],
                    _ => vec!["stage QUESTHEX N".into()],
                }
            }
            "evp" => match hex(rest) {
                Some(id) => vec![format!(
                    "evp {id:08x}: {}",
                    coop_engine::commands::evaluate_package(e, id)
                )],
                None => vec!["evp HEX".into()],
            },
            "kill" => match hex(rest) {
                Some(id) => vec![format!(
                    "kill {id:08x}: {}",
                    coop_engine::fight::kill_by(e, id, coop_engine::actors::PLAYER_REF_ID)
                )],
                None => vec!["kill HEX".into()],
            },
            // The player activates a reference (talks to an actor), as the activate key does
            // (`TESObjectREFR::Activate`, `00573170`).
            "activate" => {
                match hex(rest).and_then(|id| coop_engine::forms::form(e, id).map(|r| (id, r))) {
                    // Within a player's reach only: in this player's space, loaded, and no
                    // farther than REACH (a player cannot talk to someone in another building).
                    Some((id, r))
                        if coop_engine::fight::space_of(e, r)
                            != coop_engine::live::local(e).map_or(0, |l| l.cell)
                            || coop_engine::live::local(e).is_some_and(|l| {
                                let p = coop_engine::fight::place_of_ptr(e, r);
                                (0..3)
                                    .map(|i| (p[i] - l.pos[i]).powi(2))
                                    .sum::<f32>()
                                    .sqrt()
                                    > REACH
                            }) =>
                    {
                        vec![format!("activate {id:08x}: out of this player's reach")]
                    }
                    Some((id, r)) => {
                        let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                        let ok = e
                            .call(0x0057_3170, &args![r, player, 0u32, 0u32, 1u32])
                            .u32()
                            & 0xff;
                        vec![format!("activate {id:08x}: {ok}")]
                    }
                    None => vec![format!("activate {rest}: not here")],
                }
            }
            // A dialogue line's result scripts run as the game runs them when the line is said
            // (`TESTopicInfo::RunResultScript`, begin then end), with SPEAKER as the speaker.
            "info" => {
                let p: Vec<u32> = rest.split_whitespace().filter_map(hex).collect();
                match (
                    p.first().and_then(|i| coop_engine::forms::form(e, *i)),
                    p.get(1).and_then(|a| coop_engine::actors::actor(e, *a)),
                ) {
                    (Some(info), Some(speaker)) => {
                        for end in [0u32, 1] {
                            e.call(
                                coop_engine::addr::INFO_RUN_RESULT,
                                &args![info, end, speaker],
                            );
                        }
                        vec![format!("info {rest}: ran")]
                    }
                    _ => vec!["info INFOHEX SPEAKERHEX".into()],
                }
            }
            // How many of ITEM the player (or ACTOR) carries (`Actor::GetItemCountinContainer`,
            // `00575610`).
            "count" => {
                let p: Vec<u32> = rest.split_whitespace().filter_map(hex).collect();
                let who = p
                    .get(1)
                    .copied()
                    .unwrap_or(coop_engine::actors::PLAYER_REF_ID);
                match (
                    p.first().and_then(|i| coop_engine::forms::form(e, *i)),
                    coop_engine::actors::actor(e, who),
                ) {
                    (Some(item), Some(a)) => {
                        let n = e.call(0x0057_5610, &args![a, item]).u32() as i32;
                        vec![format!("count {:08x} on {who:08x}: {n}", p[0])]
                    }
                    _ => vec!["count ITEMHEX [ACTORHEX]".into()],
                }
            }
            // Gives or takes N of ITEM from the player (or ACTOR) the way the crafting menu does
            // (`007284f0`): `00574fa0(ref, item, 0, n)` to add, `TESObjectREFR::RemoveItem`
            // (vtable +0x17c) to take.
            "additem" | "removeitem" => {
                let p: Vec<u32> = rest.split_whitespace().filter_map(hex).collect();
                let who = p
                    .get(2)
                    .copied()
                    .unwrap_or(coop_engine::actors::PLAYER_REF_ID);
                let n = rest
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<u32>().ok());
                match (
                    p.first().and_then(|i| coop_engine::forms::form(e, *i)),
                    n,
                    coop_engine::actors::actor(e, who),
                ) {
                    (Some(item), Some(n), Some(a)) => {
                        if word == "additem" {
                            e.call(0x0057_4fa0, &args![a, item, 0u32, n]);
                        } else {
                            e.vcall(
                                a,
                                0x17c,
                                &args![item, 0u32, n, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                            );
                        }
                        let now = e.call(0x0057_5610, &args![a, item]).u32() as i32;
                        vec![format!("{word} {:08x} x{n} on {who:08x}: now {now}", p[0])]
                    }
                    _ => vec![format!("{word} ITEMHEX N [ACTORHEX]")],
                }
            }
            // `drop ITEMHEX N`: drops N of the item from this player's inventory, as the Pip-Boy's
            // drop does (`TESObjectREFR::RemoveItem`, vtable +0x17c, its drop flag set).
            "drop" => {
                let p: Vec<&str> = rest.split_whitespace().collect();
                let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                match (
                    p.first()
                        .and_then(|s| hex(s))
                        .and_then(|i| coop_engine::forms::form(e, i)),
                    p.get(1).and_then(|s| s.parse::<u32>().ok()),
                ) {
                    (Some(item), Some(n)) if player != 0 => {
                        e.vcall(
                            player,
                            0x17c,
                            &args![item, 0u32, n, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                        );
                        vec![format!("drop {rest}: done")]
                    }
                    _ => vec!["drop ITEMHEX N".into()],
                }
            }
            // `near ITEMHEX`: the references of that base in this player's cell (its list of
            // references at +0xac, `{item, next}` nodes), with their ids, places and counts.
            "near" => {
                let base = hex(rest.trim()).unwrap_or(0);
                let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                let cell = if player != 0 {
                    e.mem.u32(player + coop_engine::addr::REF_CELL_OFFSET)
                } else {
                    0
                };
                let mut found = Vec::new();
                let mut node = if cell != 0 { cell + 0xac } else { 0 };
                let mut n = 0;
                while node != 0 && e.mem.is_mapped(node) && n < 20_000 {
                    let r = e.mem.u32(node);
                    if r != 0 && e.mem.is_mapped(r + 0x30) {
                        let b = e.mem.u32(r + 0x20);
                        if b != 0
                            && e.mem.is_mapped(b + 0x10)
                            && e.mem.u32(b + coop_engine::addr::FORM_ID_OFFSET) == base
                        {
                            let id = e.mem.u32(r + coop_engine::addr::FORM_ID_OFFSET);
                            let pos = [0, 4, 8]
                                .map(|o| e.mem.f32(r + coop_engine::addr::REF_POS_OFFSET + o));
                            found.push(format!(
                                "{id:08x} at ({:.0}, {:.0}, {:.0})",
                                pos[0], pos[1], pos[2]
                            ));
                        }
                    }
                    node = e.mem.u32(node + 4);
                    n += 1;
                }
                vec![format!(
                    "near {rest}: {} found: {}",
                    found.len(),
                    found.join(", ")
                )]
            }
            // `scene ACTOR TARGET TOPIC`: actor starts a conversation with target about topic as a
            // replayed scene does (`commands::play_scene`, outside any gate), where both are.
            "scene" => {
                let p: Vec<u32> = rest.split_whitespace().filter_map(hex).collect();
                match (p.first(), p.get(1), p.get(2)) {
                    (Some(&actor), Some(&target), Some(&topic)) => {
                        let place = |e: &mut Engine, id| coop_engine::fight::place_of(e, id);
                        let (ap, tp) = (place(e, actor), place(e, target));
                        let space = coop_engine::actors::actor(e, actor)
                            .map_or(0, |a| coop_engine::fight::space_of(e, a));
                        let scene = coop_engine::commands::Scene {
                            actor,
                            target,
                            topic,
                            locations: [0, 0],
                            flags: [true, false],
                            space,
                            actor_pos: ap.unwrap_or([0.0; 3]),
                            target_pos: tp.unwrap_or([0.0; 3]),
                        };
                        vec![format!(
                            "scene {rest}: {:?}",
                            coop_engine::commands::play_scene(e, &scene)
                        )]
                    }
                    _ => vec!["scene ACTORHEX TARGETHEX TOPICHEX".into()],
                }
            }
            // `npcs`: every actor the game runs at full detail around its player, with its place,
            // space and fight, in one line (`id x y z space combat dead;` each).
            "npcs" => {
                let mut out = Vec::new();
                for a in coop_engine::fight::near(e) {
                    let id = e.mem.u32(a + coop_engine::addr::FORM_ID_OFFSET);
                    if id == coop_engine::actors::PLAYER_REF_ID {
                        continue;
                    }
                    let s = coop_engine::fight::snap(e, a);
                    let pos = coop_engine::fight::place_of_ptr(e, a);
                    out.push(format!(
                        "{id:08x} {:.0} {:.0} {:.0} {:08x} {} {}",
                        pos[0],
                        pos[1],
                        pos[2],
                        coop_engine::fight::space_of(e, a),
                        u8::from(s.in_combat),
                        u8::from(s.dead())
                    ));
                }
                vec![format!("npcs {}: {}", out.len(), out.join("; "))]
            }
            // `watch ACTOR FRAMES`: the actor's steps between frames for that many frames, then
            // one line: how far it went, its largest step, how many were jumps.
            "watch" => {
                let mut p = rest.split_whitespace();
                match (
                    p.next().and_then(hex),
                    p.next().and_then(|n| n.parse::<u32>().ok()),
                ) {
                    (Some(id), Some(frames)) if frames > 0 => {
                        lock(&WATCHES).push(Watch {
                            id,
                            left: frames,
                            frames: 0,
                            last: None,
                            path: 0.0,
                            largest: 0.0,
                            jumps: 0,
                        });
                        vec![format!("watch {id:08x}: {frames} frames")]
                    }
                    _ => vec!["watch ACTORHEX FRAMES".into()],
                }
            }
            // `fight ACTOR TARGET`: the actor starts a combat with the target (0x14: this game's
            // player) as the `StartCombat` command does.
            "fight" => {
                let p: Vec<u32> = rest.split_whitespace().filter_map(hex).collect();
                match (p.first(), p.get(1)) {
                    (Some(&actor), Some(&target)) => vec![format!(
                        "fight {rest}: {}",
                        coop_engine::fight::start_combat(e, actor, target)
                    )],
                    _ => vec!["fight ACTORHEX TARGETHEX".into()],
                }
            }
            // `compass`: the HUD's actor ticks now, the shown ones each as heading, system colour,
            // red green blue, alpha and the colour drawn; then how many purple ticks co-op drew.
            "compass" => {
                let ticks = coop_engine::compass::ticks(e);
                let shown: Vec<String> = ticks
                    .iter()
                    .filter(|t| t.shown)
                    .map(|t| {
                        let drawn = t.drawn.map_or("-".to_string(), |c| {
                            format!("{:.2},{:.2},{:.2}", c[0], c[1], c[2])
                        });
                        format!(
                            "x {:.1} sys {} rgb {:.0},{:.0},{:.0} a {:.0} drawn {drawn}",
                            t.heading, t.system_color, t.rgb[0], t.rgb[1], t.rgb[2], t.alpha
                        )
                    })
                    .collect();
                let c = coop_engine::compass::counts(e);
                vec![format!(
                    "compass {} of {} ticks shown: {}; co-op drew {}, behind {}, no tile {}",
                    shown.len(),
                    ticks.len(),
                    shown.join("; "),
                    c.drawn,
                    c.behind,
                    c.no_tile
                )]
            }
            // Any menu. `menus`: the open menus' types. `tiles TYPE`: that menu's tiles (depth,
            // name, id, shown, texts). `click TYPE ID [N]`: the menu's click on its Nth tile with
            // that id (0 the first). `clicktext TYPE TEXT`: the click on the tile showing TEXT
            // (its nearest tile with an id).
            "menus" => {
                let open: Vec<String> = (1001u32..1100)
                    .filter(|&t| menu_of(e, t).is_some())
                    .map(|t| t.to_string())
                    .collect();
                vec![format!("menus open: {}", open.join(" "))]
            }
            // `clicklist TYPE LISTHEX TEXT`: as `clicktext`, the tile first made the highlighted
            // item of the menu's list box at that offset (as the pointer over it does).
            "tiles" | "click" | "clicktext" | "clicklist" => {
                let (first, more) = rest.split_once(' ').unwrap_or((rest, ""));
                let (list, more) = if word == "clicklist" {
                    let (l, m) = more.split_once(' ').unwrap_or((more, ""));
                    (hex(l), m)
                } else {
                    (None, more)
                };
                let Some((root, menu)) = first.parse().ok().and_then(|t| menu_of(e, t)) else {
                    return vec![format!("{word}: menu {first} is not open")];
                };
                let mut all = Vec::new();
                walk_tiles(e, root, 0, &mut all);
                match word {
                    "tiles" => {
                        let mut out = vec![format!("tiles of {first}: {}", all.len())];
                        for &(tile, depth) in all.iter() {
                            let texts = tile_texts(e, tile);
                            let id = tile_id(e, tile);
                            let said = texts.iter().any(|t| t.starts_with("fc4="));
                            let all_asked = more.contains("all");
                            if !all_asked && (!tile_shown(e, tile) || (!said && id < 0)) {
                                continue;
                            }
                            out.push(format!(
                                "tile {depth} {} id {id} shown {} {}",
                                guest_str(e, e.mem.u32(tile + 0x20)),
                                tile_shown(e, tile),
                                texts
                                    .iter()
                                    .filter(|t| more.contains("all") || t.starts_with("fc4="))
                                    .cloned()
                                    .collect::<Vec<_>>()
                                    .join(" | ")
                            ));
                        }
                        out
                    }
                    "click" => {
                        let mut p = more.split_whitespace();
                        let id: i32 = p.next().and_then(|v| v.parse().ok()).unwrap_or(-2);
                        let n: usize = p.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                        let hits: Vec<u32> = all
                            .iter()
                            .map(|&(t, _)| t)
                            .filter(|&t| tile_id(e, t) == id)
                            .collect();
                        match hits.get(n) {
                            Some(&tile) => {
                                e.vcall(menu, 0xc, &args![id as u32, tile]);
                                vec![format!(
                                    "click {first} {id} {n}: clicked ({} such)",
                                    hits.len()
                                )]
                            }
                            None => {
                                vec![format!("click {first} {id} {n}: {} such tiles", hits.len())]
                            }
                        }
                    }
                    _ => {
                        let want = more.trim().to_lowercase();
                        let hit = all.iter().map(|&(t, _)| t).find(|&t| {
                            tile_shown(e, t)
                                && tile_texts(e, t)
                                    .iter()
                                    .any(|x| x.to_lowercase().contains(&want))
                        });
                        let Some(tile) = hit else {
                            return vec![format!("{word} {first} {more}: not shown")];
                        };
                        // As the game's click: the id of the nearest tile that has one, and the
                        // tile clicked.
                        let mut up = tile;
                        for _ in 0..8 {
                            if tile_id(e, up) >= 0 || up == root {
                                break;
                            }
                            up = e.mem.u32(up + 0x28);
                        }
                        let id = tile_id(e, up);
                        // The pointer over it first (`HandleMouseover`), as lists need.
                        e.vcall(menu, 0x10, &args![id as u32, tile]);
                        if let Some(list) = list {
                            e.mem.set_u32(menu + list + 0x10, tile);
                        }
                        e.vcall(menu, 0xc, &args![id as u32, tile]);
                        vec![format!("clicktext {first} {more}: clicked id {id}")]
                    }
                }
            }
            // `sheet`: the player's S.P.E.C.I.A.L. (base values), tagged skills and perks.
            "sheet" => {
                let player = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                let special: Vec<String> = (5u32..12)
                    .map(|c| format!("{}", coop_engine::stats::base_value(e, player, c)))
                    .collect();
                let tags = coop_engine::stats::tags(e, player);
                let perks = coop_engine::stats::perks(e, player);
                vec![format!(
                    "sheet special {} tags {tags:?} perks {perks:x?}",
                    special.join(" ")
                )]
            }
            // `peek ADDRHEX [WORDS]`: words of the game's memory. `flags REFHEX`: a form's flags.
            "peek" => {
                let mut p = rest.split_whitespace();
                let at = p.next().and_then(hex).unwrap_or(0);
                let n: u32 = p.next().and_then(|v| v.parse().ok()).unwrap_or(1).min(32);
                let words: Vec<String> = (0..n)
                    .map(|i| at + i * 4)
                    .map(|a| {
                        if e.mem.is_mapped(a) && e.mem.is_mapped(a + 3) {
                            format!("{:08x}", e.mem.u32(a))
                        } else {
                            "--------".into()
                        }
                    })
                    .collect();
                vec![format!("peek {at:08x}: {}", words.join(" "))]
            }
            "flags" => match hex(rest).and_then(|id| coop_engine::forms::form(e, id)) {
                Some(r) => vec![format!(
                    "flags {rest}: {:08x} at {r:08x}, player bytes {:02x} {:02x}",
                    e.mem.u32(r + 8),
                    {
                        let p = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                        e.mem.u8(p + 0x64a)
                    },
                    {
                        let p = e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
                        e.mem.u8(p + 0x64c)
                    }
                )],
                None => vec![format!("flags {rest}: not here")],
            },
            "say" => vec![rest.to_string()],
            "menu" => vec![match coop_engine::talk::menu_npc(e) {
                Some(npc) => format!("dialogue open with {npc:08x}"),
                None => "no dialogue open".into(),
            }],
            "pos" => vec![place(e, coop_engine::actors::PLAYER_REF_ID)],
            "actor" => match hex(rest) {
                Some(id) => vec![place(e, id)],
                None => vec!["actor HEX".into()],
            },
            "quest" => match hex(rest).and_then(|id| coop_engine::quest::record(e, id)) {
                Some(r) => vec![format!(
                    "quest {rest}: stage {} done {:?} objectives {:?} flags {:02x}",
                    r.stage, r.done, r.objectives, r.flags
                )],
                None => vec![format!("quest {rest}: not a quest")],
            },
            // `var QUESTHEX [INDEX]`: every variable, or the one of that index (decimal).
            "var" => {
                let p: Vec<&str> = rest.split_whitespace().collect();
                let only = p.get(1).and_then(|s| s.parse::<u32>().ok());
                match p
                    .first()
                    .and_then(|s| hex(s))
                    .and_then(|id| coop_engine::vars::quest_vars(e, id))
                {
                    Some(vs) => vec![format!(
                        "quest {rest} variables: {:?}",
                        vs.iter()
                            .filter(|(i, _)| only.map_or(true, |o| o == *i))
                            .map(|(i, b)| (*i, f64::from_bits(*b)))
                            .collect::<Vec<_>>()
                    )],
                    None => vec![format!("quest {rest}: no variables")],
                }
            }
            // A quest variable set as `set QUEST.var to VALUE` stores it (the variable by its
            // index, in decimal): for a state the replay skipped (`bMetSunny`, set by a line of
            // the opening this replay does not play).
            "setvar" => {
                let p: Vec<&str> = rest.split_whitespace().collect();
                match (
                    p.first().and_then(|s| hex(s)),
                    p.get(1).and_then(|s| s.parse::<u32>().ok()),
                    p.get(2).and_then(|s| s.parse::<f64>().ok()),
                ) {
                    (Some(q), Some(var), Some(v)) => vec![format!(
                        "setvar {rest}: {}",
                        coop_engine::vars::set_quest_var(e, q, var, v.to_bits())
                    )],
                    _ => vec!["setvar QUESTHEX VARINDEX VALUE".into()],
                }
            }
            _ => vec![format!("unknown command {word}")],
        }
    }
}

/// How far `activate` reaches (a door or a person across a room).
const REACH: f32 = 1500.0;

static TO_MAIN: std::sync::Mutex<std::collections::VecDeque<String>> =
    std::sync::Mutex::new(std::collections::VecDeque::new());
static FROM_MAIN: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// An actor watched frame by frame (`watch`): its id, the frames left, its last place, how far it
/// went, its largest step between two frames, and how many steps were longer than
/// [`WATCH_JUMP`].
struct Watch {
    id: u32,
    left: u32,
    frames: u32,
    last: Option<[f32; 3]>,
    path: f32,
    largest: f32,
    jumps: u32,
}

/// A step between two frames longer than this is a jump (units; a run covers about 8 a frame).
const WATCH_JUMP: f32 = 64.0;

static WATCHES: std::sync::Mutex<Vec<Watch>> = std::sync::Mutex::new(Vec::new());

/// One frame of the watches: each one's step; a watch done is reported.
fn watch_frame(e: &mut Engine) {
    let mut watches = lock(&WATCHES);
    let mut out = Vec::new();
    for w in watches.iter_mut() {
        w.left = w.left.saturating_sub(1);
        w.frames += 1;
        let Some(a) = coop_engine::actors::actor(e, w.id) else {
            continue;
        };
        let pos = coop_engine::fight::place_of_ptr(e, a);
        if let Some(last) = w.last {
            let step = (0..3)
                .map(|i| (pos[i] - last[i]).powi(2))
                .sum::<f32>()
                .sqrt();
            w.path += step;
            w.largest = w.largest.max(step);
            w.jumps += u32::from(step > WATCH_JUMP);
        }
        w.last = Some(pos);
        if w.left == 0 {
            out.push(format!(
                "watch {:08x}: {} frames, went {:.0} units, largest step {:.1}, steps over {WATCH_JUMP}: {}",
                w.id, w.frames, w.path, w.largest, w.jumps
            ));
        }
    }
    watches.retain(|w| w.left > 0);
    drop(watches);
    lock(&FROM_MAIN).extend(out);
}

fn lock<T>(m: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The open menu of a type: its root tile and the menu (`011f3508`'s tiles by type - 1001, a
/// tile's menu at +0x3c; `011f308f` the open bytes by type).
fn menu_of(e: &mut Engine, kind: u32) -> Option<(u32, u32)> {
    if !(1001..1100).contains(&kind) || e.mem.u8(0x011f_308f + kind) == 0 {
        return None;
    }
    let tiles = e.mem.u32(0x011f_350c);
    let root = e.mem.u32(tiles + (kind - 1001) * 4);
    if root == 0 || !e.mem.is_mapped(root + 0x3f) {
        return None;
    }
    let menu = e.mem.u32(root + 0x3c);
    (menu != 0).then_some((root, menu))
}

fn walk_tiles(e: &mut Engine, tile: u32, depth: u32, out: &mut Vec<(u32, u32)>) {
    if out.len() > 4000 || depth > 24 {
        return;
    }
    out.push((tile, depth));
    let mut kids = Vec::new();
    let mut node = e.mem.u32(tile + 4);
    while node != 0 && kids.len() < 512 {
        let t = e.mem.u32(node + 8);
        if t != 0 {
            kids.push(t);
        }
        node = e.mem.u32(node);
    }
    for t in kids {
        walk_tiles(e, t, depth + 1, out);
    }
}

/// A tile's values (`Tile +0x14` the array's data, `+0x18` how many): (id, number, text).
fn tile_values(e: &mut Engine, tile: u32) -> Vec<(u32, f32, u32)> {
    let data = e.mem.u32(tile + 0x14);
    let n = e.mem.u32(tile + 0x18).min(256);
    (0..n)
        .filter_map(|i| {
            let v = e.mem.u32(data + i * 4);
            (v != 0).then(|| (e.mem.u32(v), e.mem.f32(v + 8), e.mem.u32(v + 0xc)))
        })
        .collect()
}

fn tile_id(e: &mut Engine, tile: u32) -> i32 {
    tile_values(e, tile)
        .into_iter()
        .find(|v| v.0 == 0xfaa)
        .map_or(-1, |v| v.1 as i32)
}

/// Whether a tile and every tile above it is shown.
fn tile_shown(e: &mut Engine, tile: u32) -> bool {
    let mut t = tile;
    for _ in 0..32 {
        if t == 0 || !e.mem.is_mapped(t + 0x2b) {
            break;
        }
        let hidden = tile_values(e, t)
            .into_iter()
            .find(|v| v.0 == 0xfa3)
            .is_some_and(|v| v.1 == 0.0);
        if hidden {
            return false;
        }
        t = e.mem.u32(t + 0x28);
    }
    true
}

fn tile_texts(e: &mut Engine, tile: u32) -> Vec<String> {
    tile_values(e, tile)
        .into_iter()
        .filter(|v| v.2 != 0)
        .map(|v| format!("{:x}={}", v.0, guest_str(e, v.2)))
        .filter(|t| !t.ends_with('='))
        .collect()
}

fn guest_str(e: &mut Engine, at: u32) -> String {
    let mut out = Vec::new();
    let mut p = at;
    while at != 0 && out.len() < 90 && e.mem.is_mapped(p) {
        let b = e.mem.u8(p);
        if b == 0 {
            break;
        }
        out.push(b);
        p += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Installs the main thread's part (`engine_core::thread_host::set_end_frame_hook`).
pub fn install() {
    engine_core::thread_host::set_end_frame_hook(on_main);
}

/// On the game's main thread at the end of a frame: the queued game commands.
fn on_main(e: &mut Engine) {
    watch_frame(e);
    let lines: Vec<String> = lock(&TO_MAIN).drain(..).collect();
    for line in lines {
        let mut h = Harness {
            path: Default::default(),
            read: 0,
            pending: Default::default(),
            frame: 0,
            wait_until: 0,
            keys: Vec::new(),
            mouse: Vec::new(),
        };
        let out = h.run(e, &line);
        lock(&FROM_MAIN).extend(out);
    }
}

/// An actor's (or the player's) place, space, life and package, in one line.
fn place(e: &mut Engine, id: u32) -> String {
    let Some(a) = coop_engine::actors::actor(e, id) else {
        return format!("{id:08x}: not in this game");
    };
    let pos = coop_engine::fight::place_of_ptr(e, a);
    let space = coop_engine::fight::space_of(e, a);
    let dead = coop_engine::actors::dead(e, id);
    let package = coop_engine::commands::current_package(e, id);
    let ai = e.mem.u8(a + coop_engine::addr::ACTOR_PROCESS_ME_OFFSET);
    let level = if e.mem.u32(a + coop_engine::addr::ACTOR_PROCESS_OFFSET) != 0 {
        e.call(coop_engine::addr::MOBILE_PROCESS_TYPE, &args![a])
            .u32() as i32
    } else {
        -1
    };
    // `Actor::IsWeaponDrawn` (`008a16d0`).
    let drawn = if level >= 0 {
        e.call(0x008a_16d0, &args![a]).u32() & 0xff
    } else {
        0
    };
    let talking = level >= 0 && coop_engine::fight::in_dialogue(e, id);
    // `Actor::IsTalking` (`008a67f0`): saying a line now.
    let saying = level >= 0 && e.call(0x008a_67f0, &args![a]).u32() & 0xff != 0;
    // Its fight: combat flag and target, the attack queued now, and the attacks this game noted.
    let snap = coop_engine::fight::snap(e, a);
    let queued = e.mem.u32(a + coop_engine::addr::ACTOR_QUEUED_ATTACK_OFFSET) & 0xff;
    let attacks = coop_engine::puppet::attack_of(e, id).map_or(0, |n| n.seq);
    format!(
        "{id:08x}: at ({:.0}, {:.0}, {:.0}) heading {:.2} in {space:08x}, dead {dead:?}, package {package:08x?}, ai {ai}, process level {level}, weapon drawn {drawn}, in a conversation {talking}, saying {saying}, in combat {} target {:08x}, queued attack {queued:02x}, attacks noted {attacks}",
        pos[0], pos[1], pos[2], snap.heading, snap.in_combat, snap.target
    )
}

/// Runs `text` as a console line.
fn console(e: &mut Engine, text: &str) -> String {
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    if console == 0 {
        return "con: no console".into();
    }
    let compiler = e.mem.u32(console);
    let len = text.len() as u32 + 1;
    e.with_stack(SCRIPT_SIZE + len + 0x10, |e, base| {
        let script = base.addr();
        let line = script + SCRIPT_SIZE + 8;
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(line + i as u32, b);
        }
        e.mem.set_u8(line + text.len() as u32, 0);
        e.call(SCRIPT_CONSTRUCT, &args![script]);
        e.call(SCRIPT_SET_TEMPORARY, &args![script]);
        e.call(SCRIPT_SET_TEXT, &args![script, line]);
        e.call(SCRIPT_COMPILE_AND_RUN, &args![script, compiler, 1u32, 0u32]);
        e.call(SCRIPT_DESTRUCT, &args![script]);
    });
    "con: ran".into()
}

/// Loads the save called `name` through the save manager.
fn load(e: &mut Engine, name: &str) -> String {
    let manager = e.mem.u32(SAVE_MANAGER);
    if manager == 0 {
        return "load: no save manager".into();
    }
    let len = name.len() as u32 + 1;
    let ok = e.with_stack(len + 0x10, |e, base| {
        let at = base.addr();
        for (i, b) in name.bytes().enumerate() {
            e.mem.set_u8(at + i as u32, b);
        }
        e.mem.set_u8(at + name.len() as u32, 0);
        e.call(SAVE_MANAGER_LOAD, &args![manager, at, u32::MAX, 0u32, 1u32])
            .bool()
    });
    format!("load {name}: {}", if ok { "loaded" } else { "failed" })
}
