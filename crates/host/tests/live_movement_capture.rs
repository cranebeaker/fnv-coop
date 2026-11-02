//! Opt-in: the player's movement through the game's code
//! ([`host::player_move`]) replaying a capture of the real game's movement
//! (`nv-movecap`, a private recording of the user's own game; not in the
//! repository): each frame's recorded time step, the mover's move bits the
//! game had (`move_mode`) and the heading (`rot_z`), on Doc Mitchell's house
//! floor. Prints, per scenario, the steady horizontal speed, the highest
//! rise and the largest upward speed of the game's code next to the
//! capture's, and how many frames the game's code moved.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" NV_CAPTURE="C:\...\movement-20261016-060253" \
//!     cargo test -p host --release --test live_movement_capture -- --ignored --nocapture
//! ```
//!
//! `NV_FEET=x,y,z` puts the player elsewhere (the default is a point of Doc's
//! floor the viewer stood on). Frames with a jump go to `crates/physics` (the
//! jump control isn't wired), so the jump scenarios only compare once it is.

use esm::{ActivePlugins, LoadOrder};
use host::player_move::MoveInput;
use std::path::{Path, PathBuf};

/// The scenarios compared, by file stem.
const SCENARIOS: [&str; 5] = [
    "walk_flat-r1",
    "run_flat_fixed16-r1",
    "sneak_walk-r1",
    "jump_in_place_fixed16-r1",
    "running_jump-r1",
];

/// One captured frame.
struct Frame {
    keys: String,
    dt: f32,
    z: f32,
    heading: f32,
    vz: f32,
    hspeed: f32,
    move_mode: u32,
}

fn read_capture(path: &Path) -> Vec<Frame> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let header: Vec<&str> = lines.next().expect("header").split('\t').collect();
    let col = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .unwrap_or_else(|| panic!("column {name}"))
    };
    let (keys, dt, z, rot, vz, hs, mode) = (
        col("keys"),
        col("dt"),
        col("pos_z"),
        col("rot_z"),
        col("vel_z"),
        col("hspeed"),
        col("move_mode"),
    );
    lines
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            let num = |i: usize| f[i].parse::<f32>().unwrap_or(0.0);
            Frame {
                keys: f[keys].to_string(),
                dt: num(dt),
                z: num(z),
                heading: num(rot),
                vz: num(vz),
                hspeed: num(hs),
                move_mode: u32::from_str_radix(f[mode].trim_start_matches("0x"), 16).unwrap_or(0),
            }
        })
        .collect()
}

/// Median of the second half of `values` (the steady part of a run).
fn steady(values: &[f32]) -> f32 {
    let mut tail: Vec<f32> = values[values.len() / 2..].to_vec();
    if tail.is_empty() {
        return 0.0;
    }
    tail.sort_by(|a, b| a.total_cmp(b));
    tail[tail.len() / 2]
}

#[test]
#[ignore = "needs the player's FalloutNV.exe, FalloutNV.esm and a movement capture (NV_DATA, NV_CAPTURE)"]
fn the_games_movement_code_against_the_captured_game() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let capture = PathBuf::from(std::env::var("NV_CAPTURE").expect("NV_CAPTURE: a capture folder"));
    let feet: [f32; 3] = std::env::var("NV_FEET")
        .ok()
        .and_then(|s| {
            let v: Vec<f32> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();
            (v.len() == 3).then(|| [v[0], v[1], v[2]])
        })
        .unwrap_or([2351.982, 1603.601, 7360.0]);
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let cell = order
        .form_by_editor_id("GSDocMitchellHouse")
        .expect("GSDocMitchellHouse");
    println!("scenario\tframes by the game's code\tsteady speed game/capture\trise game/capture\tmax up speed game/capture");
    for name in SCENARIOS {
        let frames = read_capture(&capture.join(format!("{name}.tsv")));
        // A fresh game state per scenario, as the capture starts each one anew.
        let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
        let mut at = feet;
        let start_z = feet[2];
        let (mut moved, mut speeds, mut rise, mut up) = (0, Vec::new(), 0.0f32, 0.0f32);
        for f in &frames {
            let input = MoveInput {
                feet: at,
                heading: f.heading,
                move_bits: f.move_mode,
                seconds: f.dt,
                jump: f.keys.contains("jump"),
            };
            if let Some(out) = host.move_player(&order, Some(cell), &input) {
                moved += 1;
                speeds.push(out.velocity[0].hypot(out.velocity[1]));
                rise = rise.max(out.feet[2] - start_z);
                up = up.max(out.velocity[2]);
                at = out.feet;
            }
        }
        let cap_start = frames.first().map_or(0.0, |f| f.z);
        let cap_speeds: Vec<f32> = frames.iter().map(|f| f.hspeed).collect();
        let cap_rise = frames
            .iter()
            .map(|f| f.z - cap_start)
            .fold(0.0f32, f32::max);
        let cap_up = frames.iter().map(|f| f.vz).fold(0.0f32, f32::max);
        println!(
            "{name}\t{moved}/{}\t{:.2}/{:.2}\t{:.2}/{:.2}\t{:.2}/{:.2}",
            frames.len(),
            steady(&speeds),
            steady(&cap_speeds),
            rise,
            cap_rise,
            up,
            cap_up
        );
        if moved == 0 {
            println!("  {}", host.movement.summary());
        }
    }
}
