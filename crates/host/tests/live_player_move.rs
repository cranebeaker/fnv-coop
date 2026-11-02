//! Opt-in: the player's movement through the game's code
//! ([`host::player_move`]) in Doc Mitchell's house, headless: the stages it
//! makes, a few frames walking forward, and the untranslated addresses
//! reached.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_player_move -- --ignored --nocapture
//! ```
//!
//! `NV_MOVE_PROBE=1` goes on past each miss with a zero result (the
//! addresses reached after the first are indicative); `NV_MOVE_TSV=<file>`
//! writes the list of addresses reached.

use esm::{ActivePlugins, FourCC, LoadOrder};
use host::player_move::{move_bits, MoveInput};
use std::path::PathBuf;

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn the_player_walks_through_the_games_code_or_says_what_is_missing() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    host.movement.probe = std::env::var("NV_MOVE_PROBE").is_ok_and(|v| v == "1");
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let cell = order
        .form_by_editor_id("GSDocMitchellHouse")
        .expect("GSDocMitchellHouse");
    // Feet on the cell's first placed reference with a position.
    let feet = order
        .references_in_cell(cell)
        .iter()
        .find_map(|r| {
            let data = r.subrecord(FourCC(*b"DATA")).ok().flatten()?;
            (data.len() >= 12)
                .then(|| [0, 4, 8].map(|o| f32::from_le_bytes(data[o..o + 4].try_into().unwrap())))
        })
        .unwrap_or([0.0; 3]);
    let mut moved = 0;
    for frame in 0..30 {
        let input = MoveInput {
            feet,
            heading: 0.0,
            move_bits: move_bits::FORWARD | move_bits::RUNNING,
            seconds: 1.0 / 60.0,
            jump: false,
        };
        if let Some(out) = host.move_player(&order, Some(cell), &input) {
            moved += 1;
            if frame % 10 == 0 {
                println!(
                    "frame {frame}: feet {:?}, velocity {:?}",
                    out.feet, out.velocity
                );
            }
        }
    }
    println!("{}", host.movement.summary());
    for ((stage, why), n) in &host.movement.stops {
        println!("  stop in {stage} ({n}x): {why}");
    }
    let tsv = host.movement.missing_tsv(&|_| None);
    print!("{tsv}");
    if let Ok(path) = std::env::var("NV_MOVE_TSV") {
        std::fs::write(&path, &tsv).unwrap();
    }
    println!("{moved} of 30 frames moved by the game's code");
}
