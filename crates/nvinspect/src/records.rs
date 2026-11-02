//! Commands that read records: from one plugin, or from a whole Data
//! folder combined in load order.

use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use esm::load_order::{default_plugins_txt, parse_plugins_txt};
use esm::{
    flags, sig, ActivePlugins, FormId, FourCC, LoadOrder, Plugin, Record, RecordRef, Weapon,
};

use crate::fmt::{describe_type, human_bytes, one_line, preview, thousands, truncate};
use crate::{expect_args, file_name_of, CliError, Options};

enum Command {
    /// Data folder only: the lockpicking game for a lock and a skill.
    Lockpick(String, Option<i32>),
    Info,
    Types,
    List(FourCC),
    Weapons,
    Show(String),
    /// Data folder only: where a file comes from.
    Find(String),
    /// Data folder only: check every model and texture reference.
    CheckAssets,
    Cells,
    /// Data folder only: describe a model the game would load.
    Nif(String),
    /// Data folder only: draw a cell into PNG files.
    RenderCell(String, Option<String>),
    /// Data folder only: every mesh placed in a cell and how it's drawn.
    Meshes(String),
    /// Data folder only: a cell's or square's doors (`all`: every door).
    Doors(String, Option<(i32, i32)>),
    /// Data folder only: walk the player's capsule through a cell.
    Walk(String, f32),
    /// Every worldspace.
    Worlds,
    /// One exterior grid square's terrain and objects.
    Land(String, (i32, i32)),
    /// Where water stands (a worldspace, a square, a cell) or a water type.
    Water(String, Option<(i32, i32)>),
    /// What a person or creature is made of.
    Actor(String),
    /// The lines a person's conditions name them for.
    Dialogue(String),
    /// Parse every script source.
    Scripts,
    /// Every script function the game uses, and those not carried out.
    Functions,
    /// One record's script sources, numbered, and whether they parse.
    Source(String),
    /// A new game's quest scripts run for some seconds of game time,
    /// optionally after setting a quest's stage.
    Play(f32, Option<(String, u16)>),
    /// A cell's objects with scripts, and their trigger volumes.
    Scripted(String),
    /// A person's packages and path, optionally after setting a stage.
    Ai(String, Option<(String, u16)>),
    /// What a merchant sells, and the prices.
    Barter(String),
    /// A recipe category's crafting menu for a new character, with items
    /// given (`ITEM:N`) and skills set (`AV=VALUE`).
    Recipes(String, Vec<String>),
    /// What V.A.T.S. offers a new character against someone: target,
    /// weapon, distance.
    Vats(String, Option<String>, Option<f32>),
    /// V.A.T.S.'s camera paths and shots, as a tree (or one path's or
    /// shot's).
    VatsCameras(Option<String>),
    /// The experience each level takes, kill rewards, and the perks a new
    /// character could pick at a level.
    Levels(Option<u16>),
    /// Every perk entry point the game's perks use (or one of them), with
    /// each perk's function, value and conditions by tab.
    Perks(Option<u8>),
    /// Who in a cell attacks the player on sight.
    Hostile(String),
    /// Hardcore needs' stages; or sleeping, pickpocketing and trespassing
    /// in a cell (`world::living`), the player standing at X,Y,Z if given.
    Living(Option<String>, Option<String>),
    /// The idle animation tree, or the parts of it named like a word.
    Idles(Option<String>),
    /// Game settings (`GMST`) whose names contain a word, with values.
    Settings(String),
    /// Data folder only: the first-person view with a weapon (or fists).
    FirstPerson(Option<String>),
    /// Data folder only: how people sit in a piece of furniture.
    Sit(String),
    /// Data folder only: where shots land on a person or creature.
    Hits(String),
    /// Data folder only: what a weapon's hits play on each material, or
    /// what hitting a person or creature plays and shows.
    Impacts(String, Option<String>),
    /// Data folder only: the grass on one exterior grid square.
    Grass(String, (i32, i32)),
    /// Data folder only: a tree record grown, or the trees on a square.
    Trees(Vec<String>),
    /// Data folder only: a person's head parts and their morphs.
    Face(String),
    /// Data folder only: a line's lip sync file.
    Lip(String),
    /// Data folder only: the music at a place.
    Music(Vec<String>),
    /// Data folder only: a menu file worked out tile by tile.
    Menu(String),
    /// Data folder only: a font's glyphs.
    Font(String),
    /// Data folder only: the Pip-Boy's menu for a new character (stats,
    /// items or data), with a page or tab number, and items to give the
    /// character first (editor IDs, `ID:COUNT`, `ID=` to equip).
    Pipboy(String, Option<usize>, Vec<String>),
    /// Data folder only: a survey of every model's collision.
    Collision,
    /// Data folder only: every model's particle systems surveyed, or one
    /// model's described.
    Particles(Vec<String>),
    /// Data folder only: what the game has against what nv-rs covers
    /// (`records` or `files`).
    Coverage(String),
    /// `coverage nvse`: the script extender functions the scripts call.
    NvseCoverage,
}

impl Command {
    fn needs_data_folder(&self) -> bool {
        matches!(
            self,
            Command::Find(_)
                | Command::Lockpick(..)
                | Command::CheckAssets
                | Command::RenderCell(..)
                | Command::Nif(_)
                | Command::Walk(..)
                | Command::Meshes(_)
                | Command::Doors(..)
                | Command::FirstPerson(_)
                | Command::Sit(_)
                | Command::Hits(_)
                | Command::Impacts(..)
                | Command::Grass(..)
                | Command::Trees(_)
                | Command::Face(_)
                | Command::Lip(_)
                | Command::Music(_)
                | Command::Menu(_)
                | Command::Font(_)
                | Command::Pipboy(..)
                | Command::Collision
                | Command::Particles(_)
                | Command::Coverage(_)
        )
    }
}

/// How the records were loaded, for `info`.
enum Source<'a> {
    Plugin {
        path: &'a Path,
        elapsed: Duration,
    },
    Folder {
        data_dir: &'a Path,
        active_list: String,
        elapsed: Duration,
    },
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    let (parsed, required) = match command {
        "info" => (Command::Info, 0),
        "types" => (Command::Types, 0),
        "weapons" => (Command::Weapons, 0),
        "list" => {
            expect_args(command, rest, 1, 0)?;
            let kind = FourCC::parse(&rest[0]).ok_or_else(|| {
                CliError::Usage(format!(
                    "'{}' is not a record type; types are four characters, like WEAP or NPC_",
                    rest[0]
                ))
            })?;
            (Command::List(kind), 1)
        }
        "show" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Show(rest[0].clone()), 1)
        }
        "find" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Find(rest[0].clone()), 1)
        }
        "check-assets" => (Command::CheckAssets, 0),
        "collision" => (Command::Collision, 0),
        "particles" => {
            expect_args(command, rest, 0, 3)?;
            return Ok(Command::Particles(rest.to_vec()));
        }
        "coverage" => {
            expect_args(command, rest, 1, 0)?;
            if rest[0] == "nvse" {
                return Ok(Command::NvseCoverage);
            }
            (Command::Coverage(rest[0].clone()), 1)
        }
        "cells" => (Command::Cells, 0),
        "worlds" => (Command::Worlds, 0),
        "scripts" => (Command::Scripts, 0),
        "functions" => (Command::Functions, 0),
        "source" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Source(rest[0].clone()), 1)
        }
        "scripted" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Scripted(rest[0].clone()), 1)
        }
        "ai" => {
            expect_args(command, rest, 1, 2)?;
            let stage = match (rest.get(1), rest.get(2)) {
                (Some(q), Some(s)) => Some((
                    q.clone(),
                    s.parse::<u16>()
                        .map_err(|_| CliError::Usage(format!("'{s}' isn't a stage number")))?,
                )),
                _ => None,
            };
            return Ok(Command::Ai(rest[0].clone(), stage));
        }
        "play" => {
            expect_args(command, rest, 0, 3)?;
            let seconds = match rest.first() {
                Some(s) => s
                    .parse::<f32>()
                    .ok()
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .ok_or_else(|| CliError::Usage(format!("'{s}' isn't a number of seconds")))?,
                None => 60.0,
            };
            let stage = match (rest.get(1), rest.get(2)) {
                (Some(quest), Some(stage)) => Some((
                    quest.clone(),
                    stage
                        .parse::<u16>()
                        .map_err(|_| CliError::Usage(format!("'{stage}' isn't a stage number")))?,
                )),
                (Some(_), None) => {
                    return Err(CliError::Usage(
                        "play: give a quest and a stage, e.g. play 30 VCG01 0".into(),
                    ))
                }
                _ => None,
            };
            return Ok(Command::Play(seconds, stage));
        }
        "actor" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Actor(rest[0].clone()), 1)
        }
        "barter" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Barter(rest[0].clone()), 1)
        }
        "recipes" => {
            if rest.is_empty() {
                return Err(CliError::Usage(
                    "recipes: give a recipe category, e.g. recipes CampfireRecipes".into(),
                ));
            }
            return Ok(Command::Recipes(rest[0].clone(), rest[1..].to_vec()));
        }
        "vats" => {
            expect_args(command, rest, 1, 2)?;
            let number = |s: &String| s.parse::<f32>().ok().filter(|v| v.is_finite() && *v >= 0.0);
            let (weapon, distance) = match (rest.get(1), rest.get(2)) {
                (Some(d), None) if number(d).is_some() => (None, number(d)),
                (Some(w), None) => (Some(w.clone()), None),
                (Some(w), Some(d)) => (
                    Some(w.clone()),
                    Some(number(d).ok_or_else(|| {
                        CliError::Usage(format!("'{d}' isn't a distance in units"))
                    })?),
                ),
                _ => (None, None),
            };
            return Ok(Command::Vats(rest[0].clone(), weapon, distance));
        }
        "vats-cameras" => {
            expect_args(command, rest, 0, 1)?;
            return Ok(Command::VatsCameras(rest.first().cloned()));
        }
        "levels" => {
            expect_args(command, rest, 0, 1)?;
            let level = match rest.first() {
                Some(l) => Some(
                    l.parse()
                        .map_err(|_| CliError::Usage(format!("not a level: {l}")))?,
                ),
                None => None,
            };
            (Command::Levels(level), rest.len())
        }
        "perks" => {
            expect_args(command, rest, 0, 1)?;
            let entry = match rest.first() {
                Some(e) => Some(
                    e.parse()
                        .map_err(|_| CliError::Usage(format!("not an entry point number: {e}")))?,
                ),
                None => None,
            };
            (Command::Perks(entry), rest.len())
        }
        "hostile" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Hostile(rest[0].clone()), 1)
        }
        "living" => {
            expect_args(command, rest, 0, 2)?;
            (
                Command::Living(rest.first().cloned(), rest.get(1).cloned()),
                rest.len(),
            )
        }
        "idles" => {
            expect_args(command, rest, 0, 1)?;
            (Command::Idles(rest.first().cloned()), rest.len())
        }
        "settings" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Settings(rest[0].clone()), 1)
        }
        "first-person" => {
            expect_args(command, rest, 0, 1)?;
            return Ok(Command::FirstPerson(rest.first().cloned()));
        }
        "sit" => {
            expect_args(command, rest, 1, 0)?;
            return Ok(Command::Sit(rest[0].clone()));
        }
        "hits" => {
            expect_args(command, rest, 1, 0)?;
            return Ok(Command::Hits(rest[0].clone()));
        }
        "lockpick" => {
            expect_args(command, rest, 1, 1)?;
            let skill = match rest.get(1) {
                Some(s) => Some(s.parse::<i32>().map_err(|_| {
                    CliError::Usage(format!("'{s}' isn't a Lockpick skill (a whole number)"))
                })?),
                None => None,
            };
            return Ok(Command::Lockpick(rest[0].clone(), skill));
        }
        "impacts" => {
            expect_args(command, rest, 1, 1)?;
            return Ok(Command::Impacts(rest[0].clone(), rest.get(1).cloned()));
        }
        "face" => {
            expect_args(command, rest, 1, 0)?;
            return Ok(Command::Face(rest[0].clone()));
        }
        "lip" => {
            expect_args(command, rest, 1, 0)?;
            return Ok(Command::Lip(rest[0].clone()));
        }
        "music" => {
            expect_args(command, rest, 1, 16)?;
            return Ok(Command::Music(rest.to_vec()));
        }
        "menu" | "font" => {
            expect_args(command, rest, 1, 0)?;
            let arg = rest[0].clone();
            return Ok(if command == "menu" {
                Command::Menu(arg)
            } else {
                Command::Font(arg)
            });
        }
        "pipboy" => {
            expect_args(command, rest, 1, usize::MAX - 1)?;
            let page = match rest.get(1) {
                Some(p) => Some(
                    p.parse::<usize>()
                        .map_err(|_| CliError::Usage(format!("not a page number: {p}")))?,
                ),
                None => None,
            };
            let give = rest.iter().skip(2).cloned().collect();
            return Ok(Command::Pipboy(rest[0].clone(), page, give));
        }
        "dialogue" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Dialogue(rest[0].clone()), 1)
        }
        "land" | "grass" => {
            expect_args(command, rest, 3, 0)?;
            let coordinate = |s: &String| {
                s.parse::<i32>().map_err(|_| {
                    CliError::Usage(format!("'{s}' isn't a grid coordinate (a whole number)"))
                })
            };
            let square = (coordinate(&rest[1])?, coordinate(&rest[2])?);
            if command == "grass" {
                return Ok(Command::Grass(rest[0].clone(), square));
            }
            (Command::Land(rest[0].clone(), square), 3)
        }
        "trees" => {
            expect_args(command, rest, 1, 2)?;
            return Ok(Command::Trees(rest.to_vec()));
        }
        "water" => {
            expect_args(command, rest, 1, 2)?;
            let coordinate = |s: &String| {
                s.parse::<i32>().map_err(|_| {
                    CliError::Usage(format!("'{s}' isn't a grid coordinate (a whole number)"))
                })
            };
            let square = match (rest.get(1), rest.get(2)) {
                (Some(x), Some(y)) => Some((coordinate(x)?, coordinate(y)?)),
                (Some(_), None) => {
                    return Err(CliError::Usage(
                        "water: give both grid coordinates, e.g. water WastelandNV -18 0".into(),
                    ))
                }
                _ => None,
            };
            return Ok(Command::Water(rest[0].clone(), square));
        }
        "nif" => {
            expect_args(command, rest, 1, 0)?;
            (Command::Nif(rest[0].clone()), 1)
        }
        "meshes" => {
            expect_args(command, rest, 1, 0)?;
            return Ok(Command::Meshes(rest[0].clone()));
        }
        "doors" => {
            expect_args(command, rest, 1, 2)?;
            let square = match (rest.get(1), rest.get(2)) {
                (Some(x), Some(y)) => {
                    let coordinate = |s: &String| {
                        s.parse::<i32>().map_err(|_| {
                            CliError::Usage(format!(
                                "'{s}' isn't a grid coordinate (a whole number)"
                            ))
                        })
                    };
                    Some((coordinate(x)?, coordinate(y)?))
                }
                (None, None) => None,
                _ => {
                    return Err(CliError::Usage(
                        "doors takes a cell, or a worldspace with X and Y".into(),
                    ))
                }
            };
            return Ok(Command::Doors(rest[0].clone(), square));
        }
        "render-cell" => {
            expect_args(command, rest, 1, 1)?;
            return Ok(Command::RenderCell(rest[0].clone(), rest.get(1).cloned()));
        }
        "walk" => {
            expect_args(command, rest, 1, 1)?;
            let seconds = match rest.get(1) {
                Some(s) => s
                    .parse::<f32>()
                    .ok()
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .ok_or_else(|| CliError::Usage(format!("'{s}' isn't a number of seconds")))?,
                None => 2.0,
            };
            return Ok(Command::Walk(rest[0].clone(), seconds));
        }
        "files" | "extract" => {
            return Err(CliError::Usage(format!(
                "'{command}' works on .bsa archives, not plugins or Data folders"
            )))
        }
        other => return Err(CliError::Usage(format!("unknown command '{other}'"))),
    };
    expect_args(command, rest, required, 0)?;
    Ok(parsed)
}

pub fn run_plugin(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    if command.needs_data_folder() {
        return Err(CliError::Usage(
            "find, nif, check-assets, render-cell and walk need the Data folder as the target, not a single plugin".into(),
        ));
    }
    let started = Instant::now();
    let plugin = Plugin::open(path).map_err(|e| match e {
        esm::Error::Io(io) => CliError::Open {
            path: path.display().to_string(),
            message: io.to_string(),
        },
        other => CliError::InFile {
            path: path.display().to_string(),
            message: other.to_string(),
        },
    })?;
    let order = LoadOrder::single(file_name_of(path), Some(path.to_path_buf()), plugin)?;
    let source = Source::Plugin {
        path,
        elapsed: started.elapsed(),
    };
    execute(out, &order, command, options, &source)
}

pub fn run_folder(
    out: &mut impl Write,
    data_dir: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let (active, active_list) = active_plugins(options)?;
    let started = Instant::now();
    let order = LoadOrder::from_data_dir(data_dir, &active)?;
    if !matches!(command, Command::Info) {
        for warning in order.warnings() {
            eprintln!("warning: {warning}");
        }
    }
    if let Command::Coverage(what) = &command {
        return crate::coverage_cmd::run(out, &order, data_dir, what);
    }
    if command.needs_data_folder() {
        let (assets, list) = crate::assets_cmd::open(&order, data_dir, options)?;
        return match command {
            Command::Find(path) => crate::assets_cmd::find(out, &assets, &path),
            Command::Nif(path) => crate::nif_cmd::info_in_game(out, &assets, &path),
            Command::RenderCell(cell, output) => crate::render_cmd::render_cell(
                out,
                &order,
                &assets,
                &cell,
                output.as_deref(),
                options,
            ),
            Command::Meshes(cell) => crate::meshes_cmd::meshes(out, &order, &assets, &cell),
            Command::Doors(target, square) => {
                crate::doors_cmd::doors(out, &order, &assets, &target, square)
            }
            Command::Walk(cell, seconds) => {
                crate::walk_cmd::walk(out, &order, &assets, &cell, seconds, options)
            }
            Command::FirstPerson(weapon) => {
                crate::play_cmd::first_person(out, &order, &assets, weapon.as_deref())
            }
            Command::Sit(target) => crate::play_cmd::sit(out, &order, &assets, &target),
            Command::Hits(target) => crate::play_cmd::hits(out, &order, &assets, &target),
            Command::Lockpick(target, skill) => {
                crate::lockpick_cmd::lockpick(out, &order, &assets, data_dir, &target, skill)
            }
            Command::Impacts(target, material) => {
                crate::impacts_cmd::impacts(out, &order, &assets, &target, material.as_deref())
            }
            Command::Face(target) => crate::face_cmd::face(out, &order, &assets, &target),
            Command::Lip(path) => {
                let ini = assets::IniSettings::load(&assets::default_settings_files(data_dir));
                crate::face_cmd::lip(out, &order, &assets, &ini, &path, options)
            }
            Command::Grass(world, square) => {
                crate::grass_cmd::grass(out, &order, &assets, data_dir, &world, square)
            }
            Command::Music(args) => {
                crate::music_place_cmd::music(out, &order, &assets, data_dir, &args)
            }
            Command::Trees(args) => crate::trees_cmd::trees(out, &order, &assets, data_dir, &args),
            Command::Menu(path) => crate::ui_cmd::menu(out, &order, &assets, data_dir, &path),
            Command::Font(which) => crate::ui_cmd::font(out, &assets, data_dir, &which),
            Command::Pipboy(which, page, give) => {
                crate::ui_cmd::pipboy(out, &order, &assets, data_dir, &which, page, &give)
            }
            Command::Collision => crate::collision_cmd::survey(out, &order, &assets),
            Command::Particles(args) => crate::particles_cmd::run(out, &assets, &args),
            _ => crate::assets_cmd::check(out, &order, &assets, &list, options),
        };
    }
    let source = Source::Folder {
        data_dir,
        active_list,
        elapsed: started.elapsed(),
    };
    execute(out, &order, command, options, &source)
}

/// Which plugins to load, and a description of where that choice came from.
fn active_plugins(options: &Options) -> Result<(ActivePlugins, String), CliError> {
    if options.official {
        return Ok((
            ActivePlugins::OfficialOnly,
            "official files only (--official)".into(),
        ));
    }
    let path = match &options.plugins_txt {
        Some(path) => Some(path.clone()),
        None => default_plugins_txt(),
    };
    let Some(path) = path else {
        return Ok((
            ActivePlugins::OfficialOnly,
            "official files only (no plugins.txt found)".into(),
        ));
    };
    let bytes = std::fs::read(&path).map_err(|e| CliError::Open {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    Ok((
        ActivePlugins::List(parse_plugins_txt(&bytes)),
        path.display().to_string(),
    ))
}

fn execute(
    out: &mut impl Write,
    order: &LoadOrder,
    command: Command,
    options: &Options,
    source: &Source,
) -> Result<(), CliError> {
    match command {
        Command::Info => info(out, order, source),
        Command::Types => types(out, order),
        Command::List(kind) => list(out, order, kind, options),
        Command::Weapons => weapons(out, order, options),
        Command::Show(target) => show(out, order, &target),
        Command::Cells => crate::render_cmd::cells(out, order, options),
        Command::Worlds => crate::land_cmd::worlds(out, order),
        Command::Scripts => match &options.grep {
            Some(text) => {
                crate::scripts_cmd::conditions(out, order, text, options.limit.unwrap_or(20))
            }
            None => crate::scripts_cmd::scripts(out, order, options.limit.unwrap_or(20)),
        },
        Command::Functions => {
            crate::scripts_cmd::functions(out, order, options.limit.unwrap_or(60))
        }
        Command::NvseCoverage => crate::nvse_cmd::nvse(out, order),
        Command::Source(target) => {
            let rr = find_record(order, &target)?;
            crate::scripts_cmd::source(out, &rr.record()?)
        }
        Command::Scripted(cell) => crate::play_cmd::scripted(out, order, &cell),
        Command::Ai(target, stage) => crate::play_cmd::ai(
            out,
            order,
            &target,
            stage.as_ref().map(|(q, s)| (q.as_str(), *s)),
        ),
        Command::Barter(target) => crate::play_cmd::barter(out, order, &target),
        Command::Recipes(category, extra) => {
            crate::play_cmd::recipes(out, order, &category, &extra)
        }
        Command::Vats(target, weapon, distance) => {
            crate::vats_cmd::vats(out, order, &target, weapon.as_deref(), distance)
        }
        Command::VatsCameras(which) => crate::vats_cmd::cameras(out, order, which.as_deref()),
        Command::Levels(level) => crate::play_cmd::levels(out, order, level),
        Command::Perks(entry) => crate::play_cmd::perks(out, order, entry),
        Command::Hostile(cell) => crate::play_cmd::hostile(out, order, &cell),
        Command::Living(cell, at) => {
            crate::living_cmd::living(out, order, cell.as_deref(), at.as_deref())
        }
        Command::Idles(word) => idles(out, order, word.as_deref()),
        Command::Settings(word) => settings(out, order, &word),
        Command::Play(seconds, stage) => crate::play_cmd::play(
            out,
            order,
            seconds,
            stage.as_ref().map(|(q, s)| (q.as_str(), *s)),
            options.limit.unwrap_or(40),
            options.character.as_deref(),
            options.cell.as_deref(),
        ),
        Command::Actor(target) => actor(out, order, &target),
        Command::Dialogue(target) => dialogue(out, order, &target),
        Command::Land(world, square) => crate::land_cmd::land(out, order, &world, square),
        Command::Water(target, square) => crate::water_cmd::water(out, order, &target, square),
        Command::Find(_)
        | Command::CheckAssets
        | Command::RenderCell(..)
        | Command::Nif(_)
        | Command::Walk(..)
        | Command::Meshes(_)
        | Command::Doors(..)
        | Command::FirstPerson(_)
        | Command::Sit(_)
        | Command::Hits(_)
        | Command::Impacts(..)
        | Command::Grass(..)
        | Command::Trees(_)
        | Command::Face(_)
        | Command::Lip(_)
        | Command::Music(_)
        | Command::Menu(_)
        | Command::Font(_)
        | Command::Pipboy(..)
        | Command::Lockpick(..)
        | Command::Collision
        | Command::Particles(_)
        | Command::Coverage(_) => {
            unreachable!("handled by run_folder")
        }
    }
}

// ---------------------------------------------------------------------------
// info / types
// ---------------------------------------------------------------------------

fn info(out: &mut impl Write, order: &LoadOrder, source: &Source) -> Result<(), CliError> {
    match source {
        Source::Plugin { path, elapsed } => plugin_header(out, order, path, *elapsed)?,
        Source::Folder {
            data_dir,
            active_list,
            elapsed,
        } => load_order_summary(out, order, data_dir, active_list, *elapsed)?,
    }

    let counts = order.type_counts();
    writeln!(out)?;
    writeln!(out, "Most common record types:")?;
    for (kind, count) in counts.iter().take(12) {
        write_type_line(out, *kind, *count)?;
    }
    if counts.len() > 12 {
        writeln!(
            out,
            "  ... and {} more types (see `types`)",
            counts.len() - 12
        )?;
    }
    Ok(())
}

fn plugin_header(
    out: &mut impl Write,
    order: &LoadOrder,
    path: &Path,
    elapsed: Duration,
) -> Result<(), CliError> {
    let plugin = &order.plugins()[0].plugin;
    let h = plugin.header();
    writeln!(
        out,
        "File:         {} ({})",
        file_name_of(path),
        human_bytes(plugin.file_size() as u64)
    )?;
    let kind = if h.is_master {
        "master file (master flag set)"
    } else {
        "plugin (master flag not set)"
    };
    writeln!(out, "Kind:         {kind}")?;
    writeln!(out, "Format:       version {:.2}", h.version)?;
    if let Some(author) = h.author.as_deref().filter(|s| !s.is_empty()) {
        writeln!(out, "Author:       {author}")?;
    }
    if let Some(desc) = h.description.as_deref().filter(|s| !s.is_empty()) {
        writeln!(out, "Description:  {}", one_line(desc, 100))?;
    }
    if h.masters.is_empty() {
        writeln!(out, "Masters:      none")?;
    } else {
        for (i, master) in h.masters.iter().enumerate() {
            let label = if i == 0 { "Masters:" } else { "" };
            writeln!(out, "{label:<14}[{i:02X}] {master}")?;
        }
    }
    // The header's count covers records *and* groups (plus the TES4 record
    // itself in some files), so compare against both.
    let found = plugin.records().len() + plugin.group_count();
    let declared = h.declared_record_count.max(0) as usize;
    let check = if declared == found || declared == found + 1 {
        "matches the header's count".to_string()
    } else {
        format!(
            "header declares {} records and groups, {} found",
            thousands(declared),
            thousands(found)
        )
    };
    writeln!(
        out,
        "Records:      {} in {} groups ({check}), indexed in {} ms",
        thousands(plugin.records().len()),
        thousands(plugin.group_count()),
        elapsed.as_millis()
    )?;
    Ok(())
}

fn load_order_summary(
    out: &mut impl Write,
    order: &LoadOrder,
    data_dir: &Path,
    active_list: &str,
    elapsed: Duration,
) -> Result<(), CliError> {
    writeln!(out, "Data folder:  {}", data_dir.display())?;
    writeln!(out, "Active list:  {active_list}")?;
    writeln!(out, "Load order:")?;
    let width = order
        .plugins()
        .iter()
        .map(|p| p.name.len())
        .max()
        .unwrap_or(0);
    let mut versions = 0;
    for (i, p) in order.plugins().iter().enumerate() {
        let (new, overrides) = order.plugin_stats(i);
        versions += new + overrides;
        let kind = if p.plugin.header().is_master {
            "master"
        } else {
            "plugin"
        };
        writeln!(
            out,
            "  [{:02X}] {:<width$}  {kind}  {:>9} new  {:>9} overrides",
            p.load_index,
            p.name,
            thousands(new),
            thousands(overrides)
        )?;
    }
    writeln!(
        out,
        "Records:      {} after overrides ({} versions in all), loaded in {} ms",
        thousands(order.len()),
        thousands(versions),
        elapsed.as_millis()
    )?;
    for warning in order.warnings() {
        writeln!(out, "Warning:      {warning}")?;
    }
    Ok(())
}

fn types(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    let counts = order.type_counts();
    for (kind, count) in &counts {
        write_type_line(out, *kind, *count)?;
    }
    writeln!(out, "{} record types", counts.len())?;
    Ok(())
}

fn write_type_line(out: &mut impl Write, kind: FourCC, count: usize) -> std::io::Result<()> {
    match describe_type(kind) {
        Some(desc) => writeln!(out, "  {kind}  {:>9}  {desc}", thousands(count)),
        None => writeln!(out, "  {kind}  {:>9}", thousands(count)),
    }
}

// ---------------------------------------------------------------------------
// list / weapons
// ---------------------------------------------------------------------------

/// In a Data folder, names the plugin a record's winning version comes from
/// when that isn't the base game.
fn source_tag(order: &LoadOrder, rr: &RecordRef) -> String {
    if order.is_single() || rr.plugin_index == 0 {
        String::new()
    } else {
        format!("  [{}]", rr.plugin.name)
    }
}

fn list(
    out: &mut impl Write,
    order: &LoadOrder,
    kind: FourCC,
    options: &Options,
) -> Result<(), CliError> {
    let total = order.count_of_type(kind);
    if total == 0 {
        writeln!(out, "No {kind} records found.")?;
        return Ok(());
    }
    writeln!(out, "{:<8}  {:<40}  Name", "FormID", "EditorID")?;
    let mut shown = 0;
    for rr in order.records_of_type(kind) {
        if options.limit_reached(shown) {
            break;
        }
        let record = rr.record()?;
        let (editor_id, name) = (record.editor_id(), record.full_name());
        if !options.matches(&[editor_id.as_deref(), name.as_deref()]) {
            continue;
        }
        let name = name.or_else(|| exterior_cell_label(&record));
        let deleted = if rr.entry.header.is_deleted() {
            "  [deleted]"
        } else {
            ""
        };
        let line = format!(
            "{}  {:<40}  {}{deleted}{}",
            rr.form_id,
            editor_id.unwrap_or_default(),
            name.unwrap_or_default(),
            source_tag(order, &rr)
        );
        writeln!(out, "{}", line.trim_end())?;
        shown += 1;
    }
    writeln!(
        out,
        "\n{} shown of {} {kind} records",
        thousands(shown),
        thousands(total)
    )?;
    Ok(())
}

fn weapons(out: &mut impl Write, order: &LoadOrder, options: &Options) -> Result<(), CliError> {
    let total = order.count_of_type(sig::WEAP);
    writeln!(
        out,
        "{:<8}  {:<32}  {:<28}  {:>6}  {:>4}  {:>6}  {:>6}",
        "FormID", "EditorID", "Name", "Damage", "Clip", "Value", "Weight"
    )?;
    let mut shown = 0;
    let mut undecoded = 0;
    for rr in order.records_of_type(sig::WEAP) {
        if options.limit_reached(shown) {
            break;
        }
        let Some(weapon) = Weapon::from_record(&rr.record()?) else {
            continue;
        };
        if !options.matches(&[weapon.editor_id.as_deref(), weapon.name.as_deref()]) {
            continue;
        }
        let stats = match weapon.stats {
            Some(s) => format!(
                "{:>6}  {:>4}  {:>6}  {:>6.1}",
                s.base_damage, s.clip_size, s.value, s.weight
            ),
            None => {
                undecoded += 1;
                format!("{:>6}  {:>4}  {:>6}  {:>6}", "?", "?", "?", "?")
            }
        };
        writeln!(
            out,
            "{}  {:<32}  {:<28}  {stats}{}",
            rr.form_id,
            truncate(weapon.editor_id.as_deref().unwrap_or(""), 32),
            truncate(weapon.name.as_deref().unwrap_or(""), 28),
            source_tag(order, &rr)
        )?;
        shown += 1;
    }
    writeln!(
        out,
        "\n{} shown of {} weapons",
        thousands(shown),
        thousands(total)
    )?;
    if undecoded > 0 {
        writeln!(
            out,
            "{undecoded} weapon(s) had a DATA subrecord that wasn't the expected 15 bytes; \
             run `show` on one to see the raw bytes."
        )?;
    }
    Ok(())
}

/// Unnamed exterior cells are identified by their grid position, stored as
/// two i32s at the start of the `XCLC` subrecord.
fn exterior_cell_label(record: &Record) -> Option<String> {
    if record.header.kind != sig::CELL {
        return None;
    }
    let d = &record.get(FourCC::new(b"XCLC"))?.data;
    let x = i32::from_le_bytes(d.get(0..4)?.try_into().ok()?);
    let y = i32::from_le_bytes(d.get(4..8)?.try_into().ok()?);
    Some(format!("(exterior cell {x}, {y})"))
}

// ---------------------------------------------------------------------------
// show
// ---------------------------------------------------------------------------

/// Accepts a hex form ID (`0000000F`, `0xF`) or an editor ID.
pub(crate) fn find_record<'a>(
    order: &'a LoadOrder,
    target: &str,
) -> Result<RecordRef<'a>, CliError> {
    if let Some(rr) = FormId::parse_hex(target).and_then(|id| order.get(id)) {
        return Ok(rr);
    }
    if let Some(rr) = order.find_by_editor_id(target, None)? {
        return Ok(rr);
    }
    Err(CliError::NotFound(format!(
        "no record with form ID or editor ID '{target}'"
    )))
}

/// `settings`: every game setting whose name contains a word (any case),
/// with its value as the load order leaves it, by name.
fn settings(out: &mut impl Write, order: &LoadOrder, word: &str) -> Result<(), CliError> {
    let word = word.to_ascii_lowercase();
    let mut found: Vec<(String, String)> = Vec::new();
    for rr in order.records_of_type(esm::FourCC::new(b"GMST")) {
        let Ok(record) = rr.record() else { continue };
        let Some(name) = record.editor_id() else {
            continue;
        };
        if !name.to_ascii_lowercase().contains(&word) {
            continue;
        }
        let value = match (name.as_bytes().first(), record.get(sig::DATA)) {
            (Some(b's'), Some(d)) => format!("\"{}\"", d.zstring()),
            (Some(b'f' | b'i' | b'u' | b'b'), Some(_)) => {
                world::scripting::game_setting(order, &name)
                    .map_or_else(|| "?".into(), |v| v.to_string())
            }
            _ => "?".into(),
        };
        found.push((name, value));
    }
    found.sort();
    found.dedup_by(|a, b| a.0 == b.0);
    for (name, value) in &found {
        writeln!(out, "{name} = {value}")?;
    }
    writeln!(out, "{} settings", found.len())?;
    Ok(())
}

/// `idles`: the idle animation tree, each idle with its animation and
/// conditions; with a word, the branches named with it (and the groups
/// above them).
fn idles(out: &mut impl Write, order: &LoadOrder, word: Option<&str>) -> Result<(), CliError> {
    let tree = world::idles::IdleTree::load(order);
    let word = word.map(str::to_ascii_lowercase);
    let named = |id: FormId| {
        tree.get(id).is_some_and(|i| {
            word.as_ref()
                .map_or(true, |w| i.editor_id.to_ascii_lowercase().contains(w))
        })
    };
    // Whether a branch has anything named with the word.
    fn any(tree: &world::idles::IdleTree, id: FormId, named: &dyn Fn(FormId) -> bool) -> bool {
        named(id) || tree.children(Some(id)).iter().any(|&c| any(tree, c, named))
    }
    fn show(
        out: &mut impl Write,
        tree: &world::idles::IdleTree,
        id: FormId,
        depth: usize,
        all: bool,
        named: &dyn Fn(FormId) -> bool,
    ) -> Result<(), CliError> {
        let Some(idle) = tree.get(id) else {
            return Ok(());
        };
        let all = all || named(id);
        let pad = "  ".repeat(depth.min(30));
        let file = idle
            .model
            .rsplit_once(['\\', '/'])
            .map_or(idle.model.as_str(), |(_, f)| f);
        // DATA: body section, blocking / loose flags, loops, replay delay.
        let (lo, hi) = idle.loops();
        let mut data = format!("[section {}", idle.group());
        if idle.is_blocking() {
            data.push_str(", blocking");
        }
        if idle.is_loose() {
            data.push_str(", loose");
        }
        if lo != 0 || hi != 0 {
            data.push_str(&format!(", loops {lo}-{hi}"));
        }
        if idle.replay_delay() != 0 {
            data.push_str(&format!(", replay delay {}", idle.replay_delay()));
        }
        data.push(']');
        writeln!(
            out,
            "{pad}{} {}{}  {data}",
            idle.form_id,
            idle.editor_id,
            if idle.is_animation() {
                format!("  {file}")
            } else {
                String::new()
            }
        )?;
        for c in &idle.conditions {
            writeln!(
                out,
                "{pad}    if {}({:08X}, {:08X}) {:?} {}{}",
                c.function_name(),
                c.param_forms[0].0,
                c.params[1],
                c.comparison,
                c.value,
                if c.or { " OR" } else { "" }
            )?;
        }
        for &child in tree.children(Some(id)) {
            if all || any(tree, child, named) {
                show(out, tree, child, depth + 1, all, named)?;
            }
        }
        Ok(())
    }
    let mut shown = 0;
    for &root in tree.children(None) {
        if any(&tree, root, &named) {
            show(out, &tree, root, 0, word.is_none(), &named)?;
            shown += 1;
        }
    }
    if shown == 0 {
        writeln!(out, "no idles found")?;
    }
    Ok(())
}

/// `actor`: the models an NPC or creature is assembled from.
fn actor(out: &mut impl Write, order: &LoadOrder, target: &str) -> Result<(), CliError> {
    let mut rr = find_record(order, target)?;
    // A placed actor: its base.
    if rr.entry.header.kind == sig::ACHR || rr.entry.header.kind == sig::ACRE {
        let record = rr.record()?;
        if let Some(base) = record.get(sig::NAME).filter(|s| s.data.len() >= 4) {
            let id = rr.plugin.to_global(FormId(u32::from_le_bytes(
                base.data[..4].try_into().unwrap(),
            )));
            rr = order
                .get(id)
                .ok_or_else(|| CliError::NotFound(format!("base {id} not loaded")))?;
        }
    }
    let Some(look) = world::actor_look(order, rr.form_id) else {
        return Err(CliError::NotFound(format!(
            "{} isn't a person or creature this can assemble",
            describe_id(order, rr.form_id)
        )));
    };
    writeln!(
        out,
        "{}: {}, height {}",
        describe_id(order, rr.form_id),
        if look.creature {
            "creature"
        } else if look.female {
            "female"
        } else {
            "male"
        },
        look.scale
    )?;
    writeln!(out, "  skeleton: {}", look.skeleton)?;
    writeln!(out, "  idle:     {}", look.idle)?;
    for p in &look.parts {
        let mut line = format!("  part:     {}", p.model);
        if let Some(t) = &p.skin_texture {
            line.push_str(&format!("  (skin {t})"));
        }
        if let Some(t) = &p.texture {
            line.push_str(&format!("  (texture {t})"));
        }
        if let Some(b) = &p.bone {
            line.push_str(&format!("  (at {b})"));
        }
        writeln!(out, "{line}")?;
    }
    if let Some(w) = &look.fighting {
        match w.weapon {
            Some(id) => writeln!(out, "  weapon:   {}", describe_id(order, id))?,
            None if look.creature => writeln!(out, "  fights:   with its own attacks")?,
            None => writeln!(out, "  fights:   with fists")?,
        }
        for (what, path) in [
            ("put away", w.holster.as_ref()),
            ("ready", Some(&w.aim)),
            ("running", Some(&w.run)),
            ("attack", Some(&w.attack)),
        ] {
            if let Some(path) = path {
                writeln!(out, "            {what:<9} {path}")?;
            }
        }
    }
    Ok(())
}

/// `dialogue`: every line a person's conditions name them (or their
/// voice) for, by topic, with the conditions and the responses.
fn dialogue(out: &mut impl Write, order: &LoadOrder, target: &str) -> Result<(), CliError> {
    let found = find_record(order, target)?;
    // A placed person: their base, and the reference speaks.
    let (reference, rr) =
        if found.entry.header.kind == sig::ACHR || found.entry.header.kind == sig::ACRE {
            let base = world::scripting::base_of(order, found.form_id)
                .and_then(|b| order.get(b))
                .ok_or_else(|| CliError::NotFound(format!("{target}'s base isn't loaded")))?;
            (found.form_id, base)
        } else {
            (FormId(0), found)
        };
    let record = rr.record()?;
    let voice = record
        .get(FourCC::new(b"VTCK"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| {
            rr.plugin
                .to_global(FormId(u32::from_le_bytes(s.data[..4].try_into().unwrap())))
        });
    let lines = world::dialogue::lines_for(order, rr.form_id, voice);
    // What a new game would have them say first.
    if let Some(speaker) = world::dialogue::Speaker::load(order, reference, rr.form_id) {
        let state = world::dialogue::GameState::new(order);
        if let Some(info) = world::dialogue::pick(order, FormId(0xC8), &speaker, &state) {
            writeln!(out, "Greets a new player with {}:", info.form_id)?;
            for r in &info.responses {
                let voice_file = voice
                    .and_then(|v| world::dialogue::voice_path(order, &info, r, v))
                    .unwrap_or_default();
                writeln!(out, "  \"{}\"  [{voice_file}]", r.text)?;
            }
            if info.flags & world::dialogue::GOODBYE == 0 {
                let top = world::dialogue::top_level_topics(order);
                let menu = world::dialogue::next_choices(order, &info, &top, &speaker, &state);
                writeln!(
                    out,
                    "Then can be asked ({} top-level topics in the game):",
                    top.len()
                )?;
                for c in menu {
                    writeln!(
                        out,
                        "  {}  [{} {}, priority {}]",
                        c.label,
                        describe_id(order, c.topic.form_id),
                        c.info.form_id,
                        c.topic.priority
                    )?;
                }
            }
        }
    }
    writeln!(
        out,
        "{}: {} lines (voice {})",
        describe_id(order, rr.form_id),
        lines.len(),
        voice.map_or("none".into(), |v| describe_id(order, v))
    )?;
    let mut by_topic: std::collections::BTreeMap<u32, Vec<&world::dialogue::Info>> =
        Default::default();
    for info in &lines {
        by_topic
            .entry(info.topic.map_or(0, |t| t.0))
            .or_default()
            .push(info);
    }
    for (topic, infos) in by_topic {
        let t = world::dialogue::Topic::load(order, FormId(topic));
        writeln!(
            out,
            "\n{} {}",
            describe_id(order, FormId(topic)),
            t.and_then(|t| t.name)
                .map(|n| format!("\"{n}\""))
                .unwrap_or_default()
        )?;
        for info in infos {
            writeln!(
                out,
                "  INFO {}{}{}",
                info.form_id,
                if info.flags & world::dialogue::GOODBYE != 0 {
                    " (goodbye)"
                } else {
                    ""
                },
                info.prompt
                    .as_ref()
                    .map(|p| format!(" prompt \"{p}\""))
                    .unwrap_or_default()
            )?;
            for c in &info.conditions {
                writeln!(
                    out,
                    "      if {}({:08X}, {:08X}) {:?} {}{}{}",
                    c.function_name(),
                    c.param_forms[0].0,
                    c.params[1],
                    c.comparison,
                    c.value,
                    c.global
                        .map(|g| format!(" (global {g})"))
                        .unwrap_or_default(),
                    if c.or { " OR" } else { "" }
                )?;
            }
            for r in &info.responses {
                writeln!(out, "      {}: \"{}\"", r.number, r.text)?;
            }
            if !info.choices.is_empty() {
                let names: Vec<String> = info
                    .choices
                    .iter()
                    .map(|c| {
                        world::dialogue::Topic::load(order, *c)
                            .and_then(|t| t.name.or(t.editor_id))
                            .unwrap_or_else(|| c.to_string())
                    })
                    .collect();
                writeln!(out, "      then: {}", names.join(" | "))?;
            }
        }
    }
    Ok(())
}

/// A form ID with the record's editor ID and type, when it exists.
pub(crate) fn describe_id(order: &LoadOrder, id: FormId) -> String {
    match order.get(id) {
        Some(rr) => {
            let edid = rr.editor_id().ok().flatten();
            match edid {
                Some(e) => format!("{} {id} ({e})", rr.entry.header.kind),
                None => format!("{} {id}", rr.entry.header.kind),
            }
        }
        None => format!("{id} (not loaded)"),
    }
}

fn show(out: &mut impl Write, order: &LoadOrder, target: &str) -> Result<(), CliError> {
    let rr = find_record(order, target)?;
    let record = rr.record()?;
    let h = &rr.entry.header;

    let title = match (record.full_name(), record.editor_id()) {
        (Some(name), Some(edid)) => format!("\"{name}\" ({edid})"),
        (None, Some(edid)) => edid,
        (Some(name), None) => format!("\"{name}\""),
        (None, None) => String::new(),
    };
    writeln!(
        out,
        "{}",
        format!("{} {}  {title}", h.kind, rr.form_id).trim_end()
    )?;

    let origin_index = rr.form_id.mod_index();
    let origin_name = order.slot_name(origin_index).unwrap_or("?");
    if order.is_single() {
        if origin_index == rr.plugin.load_index {
            writeln!(out, "  origin:    defined in this file")?;
        } else {
            writeln!(out, "  origin:    override of a record from {origin_name}")?;
        }
    } else {
        writeln!(out, "  origin:    defined in {origin_name}")?;
        let versions = order.versions(rr.form_id);
        if versions.len() > 1 {
            let names: Vec<&str> = versions.iter().map(|v| v.plugin.name.as_str()).collect();
            writeln!(out, "  versions:  {}", names.join(" → "))?;
            writeln!(
                out,
                "  showing:   {}'s version (the last one loaded wins)",
                rr.plugin.name
            )?;
        }
    }
    writeln!(out, "  group:     {}", rr.entry.top_group)?;
    if let Some(world) = order.world_of(&rr) {
        writeln!(out, "  world:     {}", describe_id(order, world))?;
    }
    if let Some(cell) = order.cell_of(&rr) {
        writeln!(out, "  cell:      {}", describe_id(order, cell))?;
    }
    writeln!(
        out,
        "  flags:     {:#010x} ({})",
        h.flags,
        flags::describe(h.flags)
    )?;
    let size = if h.is_compressed() {
        let unpacked = rr.plugin.plugin.data(rr.entry)?.len();
        format!("{} bytes compressed, {unpacked} unpacked", h.data_size)
    } else {
        format!("{} bytes", h.data_size)
    };
    writeln!(out, "  size:      {size}")?;
    writeln!(
        out,
        "  version:   {}, revision {:#010x}",
        h.version, h.revision
    )?;
    writeln!(
        out,
        "  location:  offset {:#x} in {}",
        rr.entry.offset, rr.plugin.name
    )?;

    writeln!(out, "\nSubrecords ({}):", record.subrecords.len())?;
    for sub in &record.subrecords {
        writeln!(
            out,
            "  {}  {:>6}  {}",
            sub.kind,
            sub.data.len(),
            preview(sub)
        )?;
    }

    // Aid items' effects (and any other record with EFID/EFIT).
    let effects = world::items::effects(order, rr.form_id);
    if !effects.is_empty() {
        writeln!(out, "\nEffects:")?;
        for e in &effects {
            writeln!(
                out,
                "  {} ({}): {} for {} s on actor value {}{}, archetype {}, {} condition(s)",
                e.name,
                e.effect,
                e.magnitude,
                e.duration,
                e.actor_value,
                if e.harmful { " (harmful)" } else { "" },
                e.archetype,
                e.conditions.len()
            )?;
        }
    }
    if let Some(weapon) = Weapon::from_record(&record) {
        writeln!(out)?;
        match weapon.stats {
            Some(s) => writeln!(
                out,
                "Weapon stats: damage {}, clip {}, value {}, weight {:.1}, condition {}",
                s.base_damage, s.clip_size, s.value, s.weight, s.health
            )?,
            None => writeln!(out, "Weapon stats: DATA subrecord missing or not 15 bytes")?,
        }
        if let Some(w) = world::combat::Weapon::load(order, rr.form_id) {
            writeln!(
                out,
                "Fighting: {} (animation type {}), skill {}, {:.3} attacks a second, reload {:.2} s, \
                 range {}–{}, spread {}–{}, ammo {}, {} projectile(s) {}, critical +{} ×{}, \
                 limb damage ×{}{}",
                if w.is_melee() { "melee" } else { "ranged" },
                w.animation,
                script::ACTOR_VALUES
                    .get(usize::from(w.skill))
                    .copied()
                    .unwrap_or("?"),
                w.shots_per_second,
                w.reload_time,
                w.min_range,
                w.max_range,
                w.min_spread,
                w.spread,
                if w.ammo.is_empty() {
                    "none".to_string()
                } else {
                    w.ammo
                        .iter()
                        .map(|a| describe_id(order, *a))
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                w.projectiles,
                w.projectile.map_or("none".into(), |p| describe_id(order, p)),
                w.crit_damage,
                w.crit_mult,
                w.limb_damage_mult,
                if w.two_handed() { ", two-handed" } else { "" }
            )?;
        }
    }
    // Body part data: a BPTD's parts, or the one a person or creature uses.
    let body = match h.kind.as_bytes() {
        b"BPTD" => world::body_parts::BodyPartData::load(order, rr.form_id),
        b"NPC_" | b"CREA" | b"ACHR" | b"ACRE" => {
            world::body_parts::BodyPartData::of(order, rr.form_id)
        }
        _ => None,
    };
    if let Some(body) = body {
        writeln!(
            out,
            "\nBody parts ({}, {}):",
            describe_id(order, body.form_id),
            body.model.as_deref().unwrap_or("no skeleton")
        )?;
        for p in body.parts.iter().flatten() {
            writeln!(
                out,
                "  {:>2} {:<14} from {:<18} damage ×{}, {}% of health, condition {}, flags {:#04x}, explode {}%",
                p.part_type,
                p.name,
                p.node,
                p.damage_mult,
                p.health_percent,
                if p.actor_value >= 0 {
                    script::ACTOR_VALUES
                        .get(p.actor_value as usize)
                        .copied()
                        .unwrap_or("?")
                } else {
                    "none"
                },
                p.flags,
                p.explode_chance
            )?;
        }
    }
    Ok(())
}
