//! `--load-fos`: start from one of the original game's saves (`.fos`)
//! instead of a new game. The save is read with `world::fos_import`
//! (docs/FOS_SAVES.md, "Import"); the player starts where it says, as F9
//! does for nv-rs's own saves. Read-only: the file isn't changed.

use std::path::Path;

use cellview::{Game, ViewerScene};
use world::dialogue::GameState;

use crate::exterior::ExteriorStart;

/// The imported state and the place to open.
pub struct FosStart {
    pub state: GameState,
    pub scene: Option<ViewerScene>,
    pub outdoors: Option<ExteriorStart>,
}

/// Reads and imports a save, prints what came in, and loads its place.
pub fn open(game: &Game, path: &Path) -> Result<FosStart, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("can't read {}: {e}", path.display()))?;
    let import = world::fos_import::import(&game.order, &bytes)?;
    let order = &game.order;
    let name = |id: esm::FormId| {
        order
            .get(id)
            .and_then(|r| r.editor_id().ok().flatten())
            .map_or_else(|| id.to_string(), |e| format!("{e} ({id})"))
    };
    println!("Original save {}:", path.display());
    for (what, n) in &import.report.counts {
        println!("  {what}: {n}");
    }
    if !import.report.missing_plugins.is_empty() {
        println!(
            "  plugins not loaded: {}",
            import.report.missing_plugins.join(", ")
        );
    }
    for f in &import.report.failures {
        println!("  not imported: {f}");
    }
    println!(
        "  not imported at all: {} kinds of state (world::fos_import::GAPS)",
        world::fos_import::GAPS.len()
    );
    let state = import.state;
    for g in [
        "GameYear",
        "GameMonth",
        "GameDay",
        "GameHour",
        "GameDaysPassed",
    ] {
        if let Some(v) = state.global(order, g) {
            println!("  global {g} = {v}");
        }
    }
    let mut stages: Vec<_> = state.stages.iter().collect();
    stages.sort();
    for (quest, stage) in stages {
        let run = if state.completed.contains(quest) {
            "completed"
        } else if state.running.contains(quest) {
            "running"
        } else {
            "stopped"
        };
        println!("  quest {} stage {stage} ({run})", name(*quest));
    }
    let place = import
        .place
        .ok_or_else(|| "the save doesn't say where the player is".to_string())?;
    println!(
        "  player at {:.1} {:.1} {:.1} heading {:.3} in {}{}",
        place.position[0],
        place.position[1],
        place.position[2],
        place.heading,
        name(place.cell),
        place
            .world
            .map(|w| format!(", {}", name(w)))
            .unwrap_or_default()
    );
    // As F9 opens a saved place (`scripts::load_saved_world`).
    let (scene, outdoors) = match place.world {
        Some(world) => {
            let grid = world::WorldGrid::load(order, world).map_err(|e| e.to_string())?;
            let start = ExteriorStart {
                grid,
                feet: [place.position[0], place.position[1]],
                height: Some(place.position[2]),
                heading: place.heading,
            };
            (None, Some(start))
        }
        None => {
            let mut scene = game
                .load_cell_now(place.cell, &state.disabled)
                .map_err(|e| e.0)?;
            let [x, y, z] = place.position;
            scene.start = cellview::Start {
                eye: [x, y, z + cellview::EYE_HEIGHT],
                heading: place.heading,
                via: "an original save",
            };
            (Some(scene), None)
        }
    };
    Ok(FosStart {
        state,
        scene,
        outdoors,
    })
}
