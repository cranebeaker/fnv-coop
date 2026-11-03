//! The system checklist: every content type a full playthrough meets, what
//! exists in the world crate and the viewer, and a status guess. The status
//! is a guess from which files exist and what the docs say; nothing here has
//! been compared with the original game. File pointers are checked against
//! the repository at report time (a pointer to a missing file is flagged).

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// World rules and viewer presentation both exist.
    Present,
    /// Only one side exists, or the doc says part is missing.
    Partial,
    /// Nothing found.
    Absent,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Present => "present",
            Status::Partial => "partial",
            Status::Absent => "absent",
        }
    }
}

pub struct Item {
    pub system: &'static str,
    pub status: Status,
    /// Files in the repository (relative to its root).
    pub files: &'static [&'static str],
    pub note: &'static str,
}

const fn it(
    system: &'static str,
    status: Status,
    files: &'static [&'static str],
    note: &'static str,
) -> Item {
    Item {
        system,
        status,
        files,
        note,
    }
}

use Status::{Absent as A, Partial as P, Present as Y};

pub const ITEMS: &[Item] = &[
    it("Perks (choice, effects, entry points)", P, &["crates/world/src/perks.rs", "viewer/src/game_menus/levelup.rs"], "Rules and menu exist; per-perk entry-point effects not verified for all 150+ perks."),
    it("Level up and XP", Y, &["crates/world/src/experience.rs", "viewer/src/game_menus/levelup.rs"], "RewardXP and level-up menu exist."),
    it("Items with scripts (OnAdd/OnEquip/OnDrop)", Y, &["crates/world/src/scripting.rs"], "Runner::run_item_scripts; per-script function gaps in the function report."),
    it("Weapon mods", Y, &["crates/world/src/weapon_mods.rs", "docs/WEAPON_MODS.md"], ""),
    it("Weapon reload, ammo swap, iron sights", Y, &["crates/world/src/ammo_swap.rs", "crates/world/src/iron_sights.rs"], ""),
    it("Recipes and crafting (workbench, campfire, reloading bench)", P, &["crates/world/src/crafting.rs", "viewer/src/game_menus/recipe.rs"], "Menu and rules exist; skill and perk gating unverified."),
    it("Terminals", Y, &["crates/world/src/terminal.rs", "viewer/src/rendered_terminal.rs", "docs/TERMINALS.md"], ""),
    it("Lockpicking minigame", Y, &["crates/world/src/lockpick.rs", "viewer/src/lockpick.rs"], ""),
    it("Hacking minigame", Y, &["crates/world/src/hacking.rs", "viewer/src/game_menus/hacking.rs"], ""),
    it("Caravan", Y, &["crates/world/src/caravan/mod.rs", "viewer/src/game_menus/caravan.rs"], ""),
    it("Casino: blackjack, roulette, slots", Y, &["crates/world/src/casino/mod.rs", "viewer/src/casino_scene.rs"], ""),
    it("Casino: craps", A, &[], "No craps code found in world or viewer (the game has no craps table in vanilla; listed to be explicit)."),
    it("Companion wheel and companion AI", Y, &["crates/world/src/companions.rs", "viewer/src/game_menus/companion_wheel.rs"], ""),
    it("V.A.T.S.", Y, &["crates/world/src/vats.rs", "viewer/src/vats.rs", "crates/world/src/vats_camera.rs"], "Rules and camera exist; fidelity unverified."),
    it("Hardcore mode (needs, healing, ammo weight)", Y, &["crates/world/src/living/needs.rs", "crates/world/src/living/sleep.rs"], ""),
    it("Radio (stations, signal, playback)", P, &["crates/world/src/radio.rs", "viewer/src/radio.rs"], "Exists; station quest scripts need the music/sound path with full data."),
    it("Pip-Boy pages (STAT, ITEM, DATA, MAP, RADIO)", Y, &["viewer/src/pipboy.rs", "docs/PIPBOY.md"], ""),
    it("World map and local map", Y, &["crates/world/src/map.rs", "viewer/src/map.rs", "viewer/src/local_map.rs"], ""),
    it("Fast travel", P, &["crates/world/src/map.rs"], "Map-marker travel rules are in the world crate; the viewer path is unverified."),
    it("Karma and reputation (fame, infamy, faction standing)", Y, &["crates/world/src/reputation.rs", "crates/world/src/factions.rs"], ""),
    it("Crime, bounties, trespass, pickpocket", Y, &["crates/world/src/crime.rs", "crates/world/src/living/pickpocket.rs", "crates/world/src/living/trespass.rs"], ""),
    it("Barter and trade", Y, &["crates/world/src/barter.rs", "viewer/src/game_menus/barter.rs"], ""),
    it("Repair", Y, &["crates/world/src/repair.rs", "viewer/src/game_menus/repair.rs"], ""),
    it("Sleeping and waiting", Y, &["crates/world/src/living/sleep.rs", "viewer/src/game_menus/sleepwait.rs"], ""),
    it("Weather", Y, &["crates/world/src/weather.rs", "viewer/src/weather.rs"], ""),
    it("Day and night (sky, sun, lighting)", Y, &["viewer/src/daylight.rs", "crates/world/src/region.rs"], ""),
    it("Dialogue (topics, skill checks, speech challenges)", Y, &["crates/world/src/dialogue.rs", "viewer/src/dialogue.rs"], ""),
    it("Character generation (Doc Mitchell, race/sex, traits, skills)", Y, &["crates/world/src/chargen.rs", "viewer/src/game_menus/chargen.rs", "viewer/src/game_menus/traits.rs"], ""),
    it("Combat AI, groups, flee, NPC aim", Y, &["crates/world/src/combat_ai.rs", "crates/world/src/npc_combat.rs", "crates/world/src/combat_groups.rs"], ""),
    it("Pathfinding (navmesh)", P, &["docs/PATHING.md", "crates/world/src/ai.rs"], "Long paths across cells exist; navmesh problems show only in the load sweep on full data."),
    it("Packages (sandbox, travel, follow, sleep, eat, use item)", P, &["crates/world/src/ai.rs", "crates/world/src/sandbox.rs"], "Package types run per the AI docs; coverage per type not measured."),
    it("Saves (own format) and original saves", P, &["crates/world/src/save.rs", "docs/PERSISTENCE.md"], "nv-rs saves exist; reading original .fos saves is partial (fos_import)."),
    it("Movies and intro", Y, &["viewer/src/movie.rs", "docs/MOVIES.md"], ""),
    it("Tutorial messages", Y, &["crates/world/src/tutorial.rs", "viewer/src/game_menus/tutorial.rs"], ""),
    it("Challenges and achievements", Y, &["crates/world/src/more_functions/challenges.rs"], ""),
    it("Traps, mines, explosives", Y, &["crates/world/src/mines.rs", "crates/world/src/more_functions/traps.rs"], ""),
    it("Followers, wait, dismiss, teammate commands", Y, &["crates/world/src/companions.rs"], ""),
    it("Impact effects, decals, blood", Y, &["crates/world/src/impacts.rs", "crates/world/src/decals.rs"], ""),
    it("Vigor and implants (Old World Blues)", P, &["crates/world/src/magic.rs", "viewer/src/game_menus/vigor.rs"], "Menus exist; implants and DLC specifics unmeasured."),
    it("NVSE script functions (xNVSE and plugins)", A, &["crates/script/src/nvse.rs", "docs/NVSE_COVERAGE.md"], "Opcode names are listed; the interpreter does not run them."),
    it("Packaged Linux and macOS builds", A, &["distribution"], "Per the README: not yet."),
    it("VR", A, &["docs/VR.md"], "Design only."),
];

/// The repository root: `NV_RS_REPO`, else an ancestor of the working
/// directory (or the executable) holding `crates/world`.
pub fn repo_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("NV_RS_REPO") {
        return Some(PathBuf::from(p));
    }
    let mut starts = vec![std::env::current_dir().ok()];
    starts.push(std::env::current_exe().ok());
    for s in starts.into_iter().flatten() {
        let mut d: Option<&Path> = Some(&s);
        while let Some(p) = d {
            if p.join("crates").join("world").is_dir() {
                return Some(p.to_path_buf());
            }
            d = p.parent();
        }
    }
    None
}

/// Files of an item that exist (true) or do not (false), when the repository is known.
pub fn file_state(root: Option<&Path>, item: &Item) -> Vec<(&'static str, Option<bool>)> {
    item.files
        .iter()
        .map(|f| (*f, root.map(|r| r.join(f).exists())))
        .collect()
}

/// Event block kinds (`gamemode`, `onactivate`, ...) the runtime reacts to,
/// found by looking for the quoted kind in the world and viewer sources.
/// `None` when the repository is not available (then the report says so).
pub fn events_handled(root: &Path, kinds: &[String]) -> Vec<(String, bool)> {
    let mut text = String::new();
    for sub in ["crates/world/src", "viewer/src"] {
        collect(&root.join(sub), &mut text, 0);
    }
    let lower = text.to_ascii_lowercase();
    kinds
        .iter()
        .map(|k| {
            (
                k.clone(),
                lower.contains(&format!("\"{}\"", k.to_ascii_lowercase())),
            )
        })
        .collect()
}

fn collect(dir: &Path, out: &mut String, depth: usize) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() && depth < 4 {
            collect(&p, out, depth + 1);
        } else if p.extension().is_some_and(|x| x == "rs") {
            if let Ok(t) = std::fs::read_to_string(&p) {
                out.push_str(&t);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_have_unique_names() {
        let mut names: Vec<&str> = ITEMS.iter().map(|i| i.system).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ITEMS.len());
    }

    #[test]
    fn events_found_in_source_text() {
        let dir = std::env::temp_dir()
            .join("nv_cov_events_test")
            .join("crates")
            .join("world")
            .join("src");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.rs"), "fn f() { run(\"onactivate\"); }").unwrap();
        let root = std::env::temp_dir().join("nv_cov_events_test");
        let r = events_handled(&root, &["OnActivate".to_string(), "OnNothing".to_string()]);
        assert!(r[0].1 && !r[1].1);
    }
}
