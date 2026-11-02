//! nvinspect: a command-line window into Fallout: New Vegas game data.
//!
//! It reads a single plugin, a whole Data folder (base game, DLC and active
//! mods combined in load order), a BSA archive, a NIF mesh or a DDS texture.

mod archive_cmd;
mod assets_cmd;
mod collision_cmd;
mod coverage_cmd;
mod coverage_table;
mod dds_cmd;
mod doors_cmd;
mod face_cmd;
mod fmt;
mod fos_cmd;
mod fos_import_cmd;
mod grass_cmd;
mod impacts_cmd;
mod land_cmd;
mod living_cmd;
mod lockpick_cmd;
mod meshes_cmd;
mod movie_cmd;
mod music_cmd;
mod music_place_cmd;
mod nif_cmd;
mod nvse_cmd;
mod particles_cmd;
mod play_cmd;
mod records;
mod render_cmd;
mod scripts_cmd;
mod sha256;
mod shader_cmd;
mod trees_cmd;
mod ui_cmd;
mod vats_cmd;
mod walk_cmd;
mod water_cmd;

use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "\
nvinspect - look inside Fallout: New Vegas game data

USAGE:
    nvinspect <TARGET> [COMMAND] [ARGUMENTS] [OPTIONS]

TARGET is one of:
    a plugin file (.esm/.esp)  that one file on its own
    the game's Data folder     FalloutNV.esm, the DLC and your active mods,
                               combined in load order (the install folder
                               that contains Data works too)
    an archive (.bsa)          the meshes, textures and sounds packed in it
    a mesh (.nif)              one model
    a texture (.dds)           one texture
    a shader package (.sdp)    the game's compiled shaders, from
                               Data\\Shaders
    an MP3 (.mp3)              one music track, from Data\\Music
    a Bink movie (.bik)        e.g. Data\\Video\\FNVIntro.bik
    a folder of MP3s           e.g. Data\\Music: every track in it and in
                               the folders below

THE ORIGINAL GAME'S SAVES (.fos), read-only:
    nvinspect fos <SAVE> [PLUGIN]
                          the save's header, plugins, location table,
                          global data, change forms by type and change
                          flag, quests, globals, misc statistics and the
                          player's place, checking that every part ends
                          where it should (docs/FOS_SAVES.md); with a
                          plugin (FalloutNV.esm) its forms get editor IDs
    nvinspect fos-import <SAVE> <PLUGIN|DATA FOLDER>
                          what nv-rs takes from the save
                          (world::fos_import): counts, the player's
                          place, the game time and the quests' stages

COMMANDS FOR PLUGINS AND DATA FOLDERS:
    info                  the file header or load order, plus the most
                          common record types (the default command)
    types                 every record type with its count
    list <TYPE>           form ID, editor ID and name of each record of a
                          type, e.g. WEAP, NPC_, CREA, CELL, WRLD, QUST, ARMO
    weapons               weapons with damage, clip size, value and weight
    show <ID>             every subrecord of one record, by hex form ID or
                          editor ID; for a Data folder, also which plugins
                          change it
    cells                 every interior cell: form ID, editor ID, name and
                          how many objects are placed in it
    worlds                every outdoor worldspace with its size
    land <WORLD> <X> <Y>  one outdoor grid square: its terrain, textures
                          and objects
    water <WORLD|CELL|ID> [X Y]
                          where water stands: a worldspace's squares whose
                          water is above part of their terrain and its
                          placed water (or one square's), a cell's placed
                          water, or a water type's values (WATR)
    actor <ID>            what a person or creature is assembled from
                          (skeleton, body parts, head, idle animation)
    dialogue <ID>         the lines a person's conditions name them for,
                          and the greeting they'd say now
    scripts               parse every script source and say which don't;
                          with --grep TEXT, every if/elseif containing TEXT
                          as written and as the game compiled it
    functions             every script function the game's scripts call
                          and its conditions ask, counted, and the ones
                          not carried out yet, most used first
    source <ID>           one record's script sources, numbered, and
                          whether each parses
    scripted <CELL>       the objects in a cell that have scripts, their
                          script blocks, and trigger volumes (and how many
                          items and containers there are)
    ai <REF|PACK> [QUEST STAGE]
                          a package's begin/end/change actions, or
                          a placed person's AI packages, which one they
                          follow in a new game (after setting a stage),
                          where it leads and the navmesh path there
    barter <REF>          what a merchant sells (their merchant
                          container) and the prices a new character pays
    recipes <CATEGORY> [ITEM:N ...] [AV=VALUE ...]
                          the crafting menu of a recipe category (RCCT,
                          e.g. CampfireRecipes) for a new character given
                          those items and actor values (skills 32 to 45):
                          each listed recipe, how many can be made, its
                          skill, ingredients and products, and the filter
    vats <ID> [WEAPON] [DISTANCE]
                          what V.A.T.S. offers a new character against a
                          person or creature with a weapon (else fists):
                          action points, the attack's cost and shots, and
                          each part's chance shown at some distances
                          between the bodies' edges (DISTANCE marked)
    vats-cameras [ID]     V.A.T.S.'s camera paths (CPTH) in the order an
                          attack tries them, each with its conditions,
                          zoom and shots (CAMS: action, place, look, flags,
                          time multipliers, times, model, image space);
                          with ID one path or shot
    hostile <CELL|ID>     who in a cell (or one person) attacks the player
                          on sight in a new game: factions, how each
                          reacts to the player, aggression
    living [CELL [X,Y,Z]] hardcore needs' and radiation's stages and rates;
                          with CELL, what a new character there meets
                          (at the arrival point, or X,Y,Z): trespassing,
                          waiting, who sees them, each bed's answer, each
                          person's pockets and the chances
    settings <WORD>       game settings (GMST) whose names contain WORD,
                          with their values
    idles [WORD]          the idle animation tree (IDLE records): each
                          idle's animation and conditions; with WORD, the
                          branches named with it
    play [SECONDS [QUEST STAGE]]
                          start a new game (optionally setting a quest's
                          stage first) and run the quest scripts for
                          SECONDS of game time (60 by default): what
                          happened, and which functions they needed that
                          aren't carried out yet (with the first call of
                          each); message boxes get their first button.
                          --character FILE starts as a test character
                          (characters/README.md), --cell CELL puts the
                          player in that interior cell for the scripts

COMMANDS FOR DATA FOLDERS ONLY:
    find <PATH>           where the game loads a file from (loose file or
                          which archive), and which copies it overrides
    nif <PATH>            describe a model wherever the game loads it from
                          (loose or any archive): its meshes, how each is
                          drawn, and whether each texture can be found
                          (for a skeleton, its nodes)
    first-person [WEAPON] what the player's first-person view is built
                          from with that weapon (or fists): parts, hold
                          pose, attack and reload, and what loads
    sit <ID>              how people sit in a piece of furniture (or a
                          placed one): its markers, their exits, the seat
                          and the seated idle
    hits <ID>             where shots from in front land on a person or
                          creature: their body part data (BPTD) and a map
                          of the part each shot's body (their skeleton's
                          capsules, posed by their idle) gives
    lockpick <REF|LEVEL> [SKILL]
                          the lockpicking game for a placed lock (or a lock
                          level) and a Lockpick skill (else a new
                          character's): whether the menu opens, the sweet
                          spot and how far the lock turns where at this
                          install's screen, how long a pin lasts, forcing's
                          chance, the menu's models, lights and camera
    impacts <ID> [MATERIAL]
                          what a weapon's (or impact data set's) hits play
                          on each material (or one: stone, dirt, organic,
                          metal ...): effect model and how long it plays,
                          decal, sounds; for a person or creature, what
                          hitting them strikes, their blood, wall spatter,
                          bite and death sounds, hurt and death lines
    grass <WORLD> <X> <Y> the grass the game grows on one outdoor grid
                          square: its settings, which textures grow what
                          where, and per grass its blades and model
    trees <TREE> [SEED]   a tree record grown as the game grows it: its
                          size, branches and leaves per level of detail,
                          textures and where each level shows
    trees <WORLD> <X> <Y> the trees placed on one outdoor grid square, with
                          their seeds
    face <ID>             a person's head parts and the morphs (.tri) they
                          talk and blink with: which the game uses, which
                          never match a channel
    lip <PATH>            a line's lip sync file (or a voice file's): its
                          frames, when each is reached and when the voice
                          starts (--limit N frames)
    music <PLACE> [X Y] [at X Y] [hour H] [seconds S] [combat S [E]]
          [play MUSIC] [wav OUT]
                          the music the game plays at a place (a cell, or a
                          worldspace's grid square X Y, or `at` a point;
                          an interior's arrival point by default): its audio
                          markers, the controller and sets, the region
                          music, and the music manager run for S seconds
                          (15) at the hour (the new game's by default), with
                          battle music wanted from second S (to E), a music
                          type `PlayMusic` plays first; wav mixes the decks
                          to a file
    menu <PATH>           a menu file (e.g. main\\hud_main_menu.xml, hud, or
                          vats: V.A.T.S.'s, filled for a sample target)
                          worked out as the game's code does: each tile's
                          place on the screen, size, colour, text or
                          picture, and what's drawn
    font <N|PATH>         one of the game's fonts (1 to 8 as the INI names
                          them, or a .fnt path): line height, pictures and
                          each glyph's size, kerning and baseline
    pipboy <stats|items|data> [PAGE] [ITEM...]
                          the Pip-Boy's menu for a new character, filled as
                          the game's code fills it, on a page or tab (from
                          0), after giving it items (Stimpak:3, and = to
                          equip: WeapNV9mmPistol=): its tiles and what's
                          drawn into the Pip-Boy's picture
    check-assets          check that every model named by a record and
                          every texture named by a mesh can be found
    render-cell <CELL> [OUT]
                          draw an interior cell into two pictures: a floor
                          plan from above with the ceiling cut away
                          (OUT_plan.png) and the view from where the player
                          arrives (OUT_view.png). CELL is an editor ID, a
                          form ID or the cell's name; OUT defaults to the
                          editor ID
    meshes <CELL>         every piece of every model placed in a cell:
                          vertices, triangles, shader flags and how the
                          game draws it (blending, depth, sides), for
                          finding draws in a recording of the game
    doors <CELL|WORLD X Y|all>
                          the doors placed in a cell or an outdoor square:
                          load doors and doors that swing where they stand,
                          how each starts, its lock, flags and sounds, its
                          model's Open and Close sequences with their text
                          keys, and the collision its leaf swings with;
                          `all` counts every DOOR record's flags and models
    walk <CELL> [SECONDS] run the player's capsule through the cell's
                          collision from where the player arrives, forward,
                          right, back and left (2 seconds each by default),
                          and say how far it got each way
    collision             survey every model's collision (Havok shapes,
                          bodies and phantoms by type; the layers they sit on
                          and which stop the player; how nv-rs reads each)
                          and the references placed with a primitive (XPRM)
    particles [PATH [run SECONDS]]
                          survey every model's particle systems (each block
                          read and checked, what the systems use: emitters,
                          modifiers, controllers, how they're drawn), or with
                          a model's path, its systems in full; with run, the
                          systems run as the game runs them, each second's
                          particles summed up (count, sizes, colours, where)
    coverage records     every record type and subrecord in the loaded
                          plugins, counted per plugin, against what nv-rs
                          reads (Markdown)
    coverage files        every file in every archive and loose, by kind;
                          every NIF block type and whether nv-rs decodes it;
                          texture and sound formats; menu files (Markdown)
    coverage nvse         the script extender (xNVSE) functions and NVSE
                          plugin opcodes the scripts call, against what
                          nv-rs runs (Markdown; also for a single plugin)

COMMANDS FOR ARCHIVES:
    info                  format, flags and what the archive holds
    files                 every file in the archive
    extract <PATH> [OUT]  save one file, to OUT or the current folder
    nif <PATH>            describe a mesh inside the archive
    obj <PATH> [OUT]      convert a mesh inside the archive to .obj
    check-nifs            read every mesh in the archive and report problems
    dds <PATH>            describe a texture inside the archive
    png <PATH> [OUT]      convert a texture inside the archive to .png
    check-textures        decode every texture in the archive and report

COMMANDS FOR MESHES (.nif):
    info                  format, visible meshes, textures and block types
    blocks                every block with its type, size and name
    block <N>             one block's bytes, four at a time as hex,
                          integer and float
    obj [OUT]             convert to .obj, viewable in Blender or 3D Viewer

COMMANDS FOR TEXTURES (.dds):
    info                  format, size, mip levels and transparency
    png [OUT]             convert to .png

COMMANDS FOR SHADER PACKAGES (.sdp):
    info                  every shader: name, model, size, instruction
                          count and the names of the constants it reads
    shader <NAME>         one shader's instructions, with the game's names
                          for its constants
    dump [OUT]            every shader's instructions, one text file each,
                          into OUT (default: <package>_shaders)

COMMANDS FOR MUSIC (.mp3):
    info                  tags, format, frame count, duration, and the
                          encoder's delay and padding when recorded
    check                 decode the whole track: problems, peak and RMS
                          level, DC offset, clipping, leading silence
    wav [OUT] [SECONDS]   decode to a 16-bit .wav (all of it, or the first
                          SECONDS)

COMMANDS FOR MOVIES (.bik):
    info                  size, frame rate, frame count, keyframes and the
                          audio tracks
    frames [OUT] [bgrx]   decode every frame; write each frame's number, 0
                          (or 1 if it had an error) and the SHA-256 of its
                          Y, V and U planes, one line each, to OUT (the
                          format research/nv-oracle's nv-bink writes from
                          the game's own library); with bgrx, the SHA-256 of
                          the frame in 32-bit colour as the game shows it
    audio OUT [TRACK]     decode an audio track (default 0) to OUT as raw
                          interleaved 16-bit little-endian samples

COMMANDS FOR A FOLDER OF MP3s:
    check                 decode every track, one line each

OPTIONS:
    --grep TEXT       only entries whose name or path contains TEXT
    --limit N         stop after N matching entries
    --plugins FILE    Data folder: read the active plugin list from FILE
                      instead of %LOCALAPPDATA%\\FalloutNV\\plugins.txt
    --official        Data folder: load only the official master files
    --ini FILE        Data folder: read the archive list (SArchiveList) from
                      FILE; by default it comes from Fallout.ini in
                      Documents\\My Games\\FalloutNV, else the install's
                      Fallout_default.ini
    --all-meshes      check-assets: also check meshes no record uses
    --force           extract, obj, png, dump, wav, frames, audio: overwrite
                      existing files

RENDER-CELL OPTIONS:
    --variants        render with both rotation orders (xyz, the game's,
                      and zyx) side by side
    --rotation NAME   rotation convention: xyz (default, matches the game),
                      zyx, xyz-ccw, zyx-ccw
    --keep-root-transforms
                      apply the transform stored on each model's top node,
                      as model viewers do; the game ignores it for placed
                      objects, and so does render-cell by default
    --mark-tilted     tint objects turned on more than one axis, the only
                      ones the rotation order affects
    --arrival N       stand at arrival point N (listed in the output)
    --from X,Y[,Z]    stand here instead (feet position, as the console's
                      player.getpos reports it)
    --heading DEG     face this compass direction (0 = north, 90 = east),
                      as player.getangle z reports it
    --pitch DEG       look up (positive) or down (negative)
    --fov DEG         the game's field of view setting (default 75, its
                      fDefaultFOV): the width of a 4:3 picture; wider
                      pictures keep its height, as in the game
    --size WxH        view image size (default 1600x900)
    --plan-size N     longest side of the plan image (default 1600)
    --cut Z           plan: remove everything above height Z (default: 160
                      units above the main floor)
    --brightness F    view: scale the cell's lights (default 1)
    --fullbright      view: even lighting instead of the cell's lights
    --no-cull         draw the back faces of one-sided surfaces too
    --supersample N   render N times larger and shrink, for smoother edges
                      (default 2)

EXAMPLES:
    nvinspect \"C:\\Program Files (x86)\\Steam\\steamapps\\common\\Fallout New Vegas\\Data\"
    nvinspect Data\\FalloutNV.esm list NPC_ --grep ranger
    nvinspect Data weapons --grep rifle
    nvinspect Data show 0000000F
    nvinspect Data find textures\\weapons\\1handpistol\\10mmpistol.dds
    nvinspect Data check-assets
    nvinspect Data nif clutter\\museum\\tornpaintingsm01.nif
    nvinspect Data cells --grep mitchell
    nvinspect Data render-cell GSDocMitchellHouse
    nvinspect Data render-cell GSDocMitchellHouse --variants
    nvinspect \"Data\\Fallout - Meshes.bsa\" files --grep 10mm
    nvinspect \"Data\\Fallout - Meshes.bsa\" extract <path shown by files>
    nvinspect \"Data\\Fallout - Meshes.bsa\" obj <path shown by files>
    nvinspect \"Data\\Fallout - Meshes.bsa\" check-nifs
    nvinspect \"Data\\Fallout - Textures.bsa\" check-textures
    nvinspect Data\\Music\\SCR\\mus_SCR_DocMitchell.mp3 wav doc.wav 10
    nvinspect Data\\Music check
";

pub enum CliError {
    Usage(String),
    NotFound(String),
    /// A file or folder couldn't be opened at all.
    Open {
        path: String,
        message: String,
    },
    /// A file opened but its contents are a problem.
    InFile {
        path: String,
        message: String,
    },
    Esm(esm::Error),
    Bsa(bsa::Error),
    Nif(nif::Error),
    Io(io::Error),
}

impl From<nif::Error> for CliError {
    fn from(e: nif::Error) -> Self {
        CliError::Nif(e)
    }
}

impl From<esm::Error> for CliError {
    fn from(e: esm::Error) -> Self {
        CliError::Esm(e)
    }
}

impl From<bsa::Error> for CliError {
    fn from(e: bsa::Error) -> Self {
        CliError::Bsa(e)
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::Io(e)
    }
}

#[derive(Default)]
pub struct Options {
    pub grep: Option<String>,
    pub limit: Option<usize>,
    pub plugins_txt: Option<PathBuf>,
    pub official: bool,
    pub force: bool,
    pub ini: Option<PathBuf>,
    pub all_meshes: bool,
    /// `play`: a ready-made test character to start as
    /// (`world::character`).
    pub character: Option<PathBuf>,
    /// `play`: the cell the player is in (editor ID), for scripts that ask.
    pub cell: Option<String>,
    pub render: render_cmd::RenderFlags,
}

impl Options {
    /// Case-insensitive `--grep` match against any of the given texts.
    pub fn matches(&self, texts: &[Option<&str>]) -> bool {
        let Some(needle) = &self.grep else {
            return true;
        };
        let needle = needle.to_lowercase();
        texts
            .iter()
            .flatten()
            .any(|s| s.to_lowercase().contains(&needle))
    }

    pub fn limit_reached(&self, shown: usize) -> bool {
        self.limit.is_some_and(|limit| shown >= limit)
    }
}

enum Target {
    Plugin(PathBuf),
    DataFolder(PathBuf),
    Archive(PathBuf),
    Mesh(PathBuf),
    Texture(PathBuf),
    ShaderPackage(PathBuf),
    Music(PathBuf),
    MusicFolder(PathBuf),
    Movie(PathBuf),
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    }
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        // Output piped into `head` and closed early: not an error.
        Err(CliError::Io(e)) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(CliError::Usage(msg)) => {
            eprintln!("error: {msg}\n\nRun `nvinspect --help` for usage.");
            ExitCode::from(2)
        }
        Err(err) => {
            let message = match err {
                CliError::Open { path, message } => format!("could not open '{path}': {message}"),
                CliError::InFile { path, message } => format!("'{path}': {message}"),
                CliError::NotFound(msg) => msg,
                CliError::Esm(e) => e.to_string(),
                CliError::Bsa(e) => e.to_string(),
                CliError::Nif(e) => e.to_string(),
                CliError::Io(e) => e.to_string(),
                CliError::Usage(_) => unreachable!(),
            };
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), CliError> {
    let (positional, options) = parse_args(args)?;
    if positional[0] == "fos" {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        fos_cmd::run(&mut out, &positional[1..])?;
        out.flush()?;
        return Ok(());
    }
    if positional[0] == "fos-import" {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        fos_import_cmd::run(&mut out, &positional[1..])?;
        out.flush()?;
        return Ok(());
    }
    let target = classify(&positional[0])?;
    let command = positional.get(1).map_or("info", String::as_str);
    let rest = positional.get(2..).unwrap_or(&[]);

    let folder_only = options.plugins_txt.is_some()
        || options.official
        || options.ini.is_some()
        || options.all_meshes;
    if folder_only && !matches!(target, Target::DataFolder(_)) {
        return Err(CliError::Usage(
            "--plugins, --official, --ini and --all-meshes only apply when the target is a Data folder"
                .into(),
        ));
    }
    if options.force
        && !matches!(
            target,
            Target::Archive(_)
                | Target::Mesh(_)
                | Target::Texture(_)
                | Target::ShaderPackage(_)
                | Target::Music(_)
                | Target::Movie(_)
        )
    {
        return Err(CliError::Usage(
            "--force only applies to extract, obj, png, dump, wav, frames and audio".into(),
        ));
    }

    if options.render.any && command != "render-cell" {
        return Err(CliError::Usage(
            "the camera, size and rotation options only apply to render-cell".into(),
        ));
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    match target {
        Target::Archive(path) => archive_cmd::run(&mut out, &path, command, rest, &options)?,
        Target::Plugin(path) => records::run_plugin(&mut out, &path, command, rest, &options)?,
        Target::DataFolder(path) => records::run_folder(&mut out, &path, command, rest, &options)?,
        Target::Mesh(path) => nif_cmd::run_file(&mut out, &path, command, rest, &options)?,
        Target::Texture(path) => dds_cmd::run_file(&mut out, &path, command, rest, &options)?,
        Target::ShaderPackage(path) => {
            shader_cmd::run_file(&mut out, &path, command, rest, &options)?
        }
        Target::Music(path) => music_cmd::run_file(&mut out, &path, command, rest, &options)?,
        Target::Movie(path) => movie_cmd::run_file(&mut out, &path, command, rest, &options)?,
        Target::MusicFolder(path) => {
            let command = positional.get(1).map_or("check", String::as_str);
            music_cmd::run_folder(&mut out, &path, command, rest, &options)?
        }
    }
    out.flush()?;
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(Vec<String>, Options), CliError> {
    let mut positional = Vec::new();
    let mut options = Options::default();
    let mut iter = args.iter();
    let value_for = |flag: &str, iter: &mut std::slice::Iter<String>| {
        iter.next()
            .cloned()
            .ok_or_else(|| CliError::Usage(format!("{flag} needs a value")))
    };
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--grep" => options.grep = Some(value_for("--grep", &mut iter)?),
            // Every byte of every subrecord in `show`, not just the first.
            "--full" => fmt::FULL.store(true, std::sync::atomic::Ordering::Relaxed),
            "--limit" => {
                let value = value_for("--limit", &mut iter)?;
                let n = value.parse().map_err(|_| {
                    CliError::Usage(format!("--limit expects a number, got '{value}'"))
                })?;
                options.limit = Some(n);
            }
            "--plugins" => options.plugins_txt = Some(value_for("--plugins", &mut iter)?.into()),
            "--official" => options.official = true,
            "--force" => options.force = true,
            "--ini" => options.ini = Some(value_for("--ini", &mut iter)?.into()),
            "--all-meshes" => options.all_meshes = true,
            "--character" => options.character = Some(value_for("--character", &mut iter)?.into()),
            "--cell" => options.cell = Some(value_for("--cell", &mut iter)?),
            flag if options.render.parse(flag, || value_for(flag, &mut iter))? => {}
            flag if flag.starts_with("--") => {
                return Err(CliError::Usage(format!("unknown option '{flag}'")))
            }
            _ => positional.push(arg.clone()),
        }
    }
    if positional.is_empty() {
        return Err(CliError::Usage(
            "missing the target: a plugin file, the Data folder or a .bsa archive".into(),
        ));
    }
    Ok((positional, options))
}

/// Works out whether the target is a plugin, a Data folder or an archive.
fn classify(arg: &str) -> Result<Target, CliError> {
    let path = Path::new(arg);
    if path.is_dir() {
        for folder in [path.to_path_buf(), path.join("Data")] {
            if has_file_ignoring_case(&folder, esm::load_order::MAIN_MASTER) {
                return Ok(Target::DataFolder(folder));
            }
        }
        if music_cmd::has_music(path) {
            return Ok(Target::MusicFolder(path.into()));
        }
        return Err(CliError::NotFound(format!(
            "'{arg}' is a folder, but neither it nor a Data folder inside it contains {}. {}",
            esm::load_order::MAIN_MASTER,
            describe_plugins_in(path)
        )));
    }

    let is_package = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("sdp"));
    if is_package && path.is_file() {
        return Ok(Target::ShaderPackage(path.into()));
    }
    let is_mp3 = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("mp3"));
    if is_mp3 && path.is_file() {
        return Ok(Target::Music(path.into()));
    }
    let mut magic = [0u8; 4];
    let read = std::fs::File::open(path).and_then(|mut f| f.read(&mut magic));
    match read {
        Err(e) => Err(CliError::Open {
            path: arg.to_string(),
            message: e.to_string(),
        }),
        Ok(_) if &magic == bsa::MAGIC || &magic == b"BTDX" => Ok(Target::Archive(path.into())),
        // "Gamebryo File Format..." or "NetImmerse File Format..."
        Ok(_) if &magic == b"Game" || &magic == b"NetI" => Ok(Target::Mesh(path.into())),
        Ok(_) if &magic == b"DDS " => Ok(Target::Texture(path.into())),
        Ok(_) if &magic[..3] == b"BIK" => Ok(Target::Movie(path.into())),
        Ok(_) => Ok(Target::Plugin(path.into())),
    }
}

fn has_file_ignoring_case(folder: &Path, name: &str) -> bool {
    std::fs::read_dir(folder).is_ok_and(|entries| {
        entries.flatten().any(|e| {
            e.file_name().to_string_lossy().eq_ignore_ascii_case(name) && e.path().is_file()
        })
    })
}

fn describe_plugins_in(folder: &Path) -> String {
    let mut plugins: Vec<String> = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|name| {
                    let lower = name.to_ascii_lowercase();
                    lower.ends_with(".esm") || lower.ends_with(".esp")
                })
                .collect()
        })
        .unwrap_or_default();
    plugins.sort();
    if plugins.is_empty() {
        "It contains no .esm or .esp files.".into()
    } else {
        format!("Plugin files in it: {}", plugins.join(", "))
    }
}

/// Checks a command's argument count.
pub fn expect_args(
    command: &str,
    rest: &[String],
    required: usize,
    optional: usize,
) -> Result<(), CliError> {
    if rest.len() < required {
        return Err(CliError::Usage(format!("'{command}' needs an argument")));
    }
    if rest.len() > required + optional {
        return Err(CliError::Usage(format!(
            "unexpected argument '{}'",
            rest[required + optional]
        )));
    }
    Ok(())
}

pub fn file_name_of(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    )
}
