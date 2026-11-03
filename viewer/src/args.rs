//! Command-line arguments. Kept free of Bevy so it can be tested on its own.

use std::path::PathBuf;

use esm::FormId;

pub const USAGE: &str = "\
nv-viewer - walk around a Fallout: New Vegas cell

USAGE:
    nv-viewer <DATA FOLDER> <CELL> [OPTIONS]

    DATA FOLDER  the game's Data folder, or the install folder containing it
    CELL         an interior cell's editor ID, form ID or name, e.g.
                 GSDocMitchellHouse (nvinspect's `cells` command lists them);
                 an outdoor cell (Goodsprings); or a worldspace
                 (WastelandNV, with --at to say where; `nvinspect worlds`
                 lists them). Outdoors, the cells around the player load
                 as they walk.

OPTIONS:
    --official              load only FalloutNV.esm and the official DLC
    --plugins FILE          read the active plugin list from FILE
    --ini FILE              read the archive list (SArchiveList) from FILE
    --brightness F          scale every light (default 1)
    --keep-root-transforms  place models as model viewers do (not the game)
    --at X,Y,Z,HEADING[,PITCH]
                            start standing here instead: the numbers the
                            game's console gives for player.getpos x, y, z
                            and player.getangle z (and x, for looking up or
                            down), to line up with an in-game screenshot
    --screenshot FILE       open at 1920x1080, save a PNG of the view to
                            FILE once everything has loaded, then quit
    --wait SECONDS          with --screenshot, let the game run this long
                            first (people walking, scripts)
    --screen-size W,H       with --screenshot or --background, open at this
                            size instead (to check other shapes of screen)
    --walk                  walk even with --screenshot (which otherwise
                            flies, keeping the exact eye position given)
    --fps                   print the frame rate every two seconds, and how
                            long each frame's own work took on the main and
                            render threads (apart from waiting for the
                            display)
    --talk                  start talking to the nearest person once loaded
    --choose N,N,...        with --talk or a script's talking: pick these
                            replies in order (as the number keys would),
                            for playing a conversation through unattended
    --stage QUEST STAGE     set a quest's stage once loaded, as a script
                            would (VCG01 0 starts Doc Mitchell's intro)
    --new-game              start the game: the opening quest (VCG00) from
                            its first stage, which plays the intro movie;
                            scripts take you to Doc's house (the CELL can
                            then be left out)
    --load-fos FILE         start from one of the original game's saves
                            (.fos, read-only) instead of a new game: its
                            quests, globals, inventory, the player's place
                            (the CELL can then be left out); prints what
                            it took (docs/FOS_SAVES.md)
    --character FILE        start as a ready-made test character: a file
                            of the game's script lines (editor IDs) run
                            on the new game before its first frame, plus
                            `level N` (see characters/README.md)
    --movies, --no-movies   play the movies scripts ask for (PlayBink), or
                            skip them; by default they play, except when
                            taking a screenshot
    --weapon ID             start with this weapon (editor ID or form ID)
                            equipped and 50 rounds for it; screenshots
                            then show it in your hands
    --weather-region ID     the player's weather region (editor ID or form
                            ID), as a saved game would have it; it colours
                            glows without an Emittance of their own.
                            Starting indoors otherwise acts as if you came
                            in through the place's door from outside
    --run \"COMMAND\"         once loaded, run a line of the game's script
                            language, as its console does (for example
                            \"SunnyREF.StartCombat player\"); can be given
                            more than once
    --run-at SECONDS \"COMMAND\"
                            for testing: run a script line that many
                            seconds after loading (in the game, not in a
                            menu); can be given more than once
    --say TEXT              for testing: when the dialogue menu offers
                            topics, choose the first whose text contains
                            TEXT (ignoring case); each --say is used once,
                            in order
    --say-id FORM           for testing: choose the offered dialogue line
                            with this INFO form ID; each is used once
    --no-hud                leave out the game's HUD (health, compass,
                            crosshair, messages); screenshots then show
                            the scene alone
    --background            a test run out of the way: the window opens
                            behind the others without the focus, and the
                            mouse is never held in it (with --screen-size,
                            at that size)
    --freeze-ai             people and creatures stand where they are and
                            do nothing but their idle, as after the game's
                            console command tai (for lining up with a
                            recording made that way)
    --pipboy MENU[:PAGE]    once loaded, raise the Pip-Boy on this menu
                            (stats, items or data) and page or tab (from
                            0: stats:1 is S.P.E.C.I.A.L., data:2 the
                            quests); screenshots then show it
    --pipboy-keys K[,K...]  for testing: once the Pip-Boy is up, press these
                            keys in it, one a frame: up, down, left, right,
                            enter, a letter (r: ITEMS' Repair, x: Mod), padx
                            or pady (Shift + Enter, Alt + Enter: the pad's
                            X and Y, ITEMS' Drop with --pad), or close (put
                            it away)
    --pad                   for testing: the menus as with a 360 pad
                            connected (its buttons shown)
    --vats [N]              for testing: open V.A.T.S. three seconds after
                            loading (as V does); with N, queue N attacks on
                            the part it opens on and play them
    --lockpick REF          for testing: once loaded, try the lock on this
                            placed door or container (editor ID or form
                            ID) as E on it would: the lockpicking menu
    --cloud-time SECONDS    hold the clouds where this many seconds of
                            drift put them, to line up with a recording of
                            the game (Goodsprings' reference: 58.022)
    --open-menu MENU[:ID]   for testing: once loaded, open one of the game's
                            menus as the game would: container:REF (a
                            container or a body), barter:REF (a merchant),
                            recipes:CATEGORY (the recipe menu, e.g.
                            recipes:CampfireRecipes),
                            repair:REF (a merchant's repairs),
                            teammate:REF (trading with a companion),
                            wheel:REF (a companion's wheel of orders),
                            quantity:N (how many, up to N), levelup (the
                            player goes up a level; levelup:perks also
                            gives the points and goes on to the perks),
                            wait or sleep (the sleep/wait menu; wait:N or
                            sleep:N also chooses N hours and presses Wait)
    --use REF               for testing: once loaded, press E on this
                            object (editor ID or form ID), as if the
                            player's crosshair were on it
    --menu-pointer X,Y      for testing: put the menus' pointer at this
                            pixel (screenshots have no mouse); the
                            Pip-Boy's too, through its screen
    --key-at SECONDS KEY[:HOLD]
                            for testing: press a key (a letter or digit,
                            mouse-left, mouse-right, escape, tab, mouse-x=COUNTS: the
                            mouse moving sideways each frame, or
                            wheel=NOTCHES, negative out) that
                            many seconds after you're placed, held HOLD
                            seconds (0.1): F third person, R:1 put the
                            weapon away, F:9 with mouse-x=20:1 turn the
                            third-person camera around you; can be given
                            more than once
    --menu-click S[,S...]   for testing: click the menus' left button at
                            these seconds after starting (with
                            --menu-pointer)
    --menu-keys S:K[,S:K...]
                            for testing: type key K (a character, or
                            left, right, up, down, enter) into the top menu at
                            S seconds after starting; tab leaves the
                            hacking and terminal menus, as Tab does
    --menu-keys-player-clock
                            time --menu-keys from player readiness, like
                            --key-at, rather than application startup
    --answer-boxes          for testing (the acceptance routes): answer the
                            prompts on top by rule as soon as they're shown,
                            as a player would: a message box with one button
                            (OK) with it, one with more with the next of
                            --box-answers (left open when none is left); a
                            tutorial box closed; the name entry accepted
                            with Enter (the name in it). The DLCs' start-up
                            messages and VCG01's prompts would otherwise hold
                            a scripted route
    --box-answers I[,I...]  with --answer-boxes: the buttons (0 the first,
                            by the box's own order) to answer the boxes with
                            more than one button, one per box in turn
    --movement MODE         how the player moves: engine (the default: the
                            game's own translated code, crates/engine, with
                            crates/physics for a frame it can't run yet),
                            old (crates/physics only), or compare
                            (crates/physics moves you, the game's code runs
                            beside it; see --movement-log)
    --movement-probe        for testing: the game's movement code goes on
                            past functions not translated yet (with zero
                            results) to list more of them; it never moves
                            you then
    --movement-log FILE     write each frame's position and velocity from
                            both paths (tab-separated) to FILE
    --movement-missing FILE write the untranslated functions the game's
                            movement code reached, with counts, to FILE
                            (every 5 s and at exit)
    --physics MODE          how loose objects move: old (the default:
                            crates/physics) or engine (the game's own Havok
                            world, crates/engine, stepped by the game's frame
                            code; crates/physics for a frame it can't run)

CONTROLS:
    mouse                          look around (walking; flying: hold
                                   a button)
    left mouse button              attack (walking)
    right mouse button, Left Alt   aim down the sights (held)
    V                              V.A.T.S.: Left/Right target, Up/Down
                                   part, Enter or E queue an attack, X a
                                   special one, Backspace undo, Space or
                                   R play the queue, V/Esc/Tab leave
    W A S D                        move
    F                              walk (with collision) / fly
    walking:  Shift walk slowly, Ctrl sneak on/off, Caps Lock always
              run on/off, Q auto move, Space jump, R reload (held:
              put the weapon away or draw it)
    flying:   Space / Ctrl (or Q) up, down; Shift faster; mouse
              wheel changes speed
    E                              go through the load door in view,
                                   talk to the person in view, take
                                   an item, open a container, search
                                   a body, or use a machine
    picking a lock: move the mouse to place the pick, W A S D (the
              game's movement controls) turn the lock, F force it,
              E leave
    1-9, Space, Tab                choose, skip a line, end talking
    menus: arrows, Space, Enter    move, pick, accept
    Tab                            the Pip-Boy (held: its light)
    Pip-Boy: arrows, Enter         move, equip / use; Shift + left or
                                   right change menu; the letters
                                   press their buttons; the mouse
                                   points and clicks; right button:
                                   drop (ITEMS), your map marker
                                   (DATA); wheel, Page Up / Down:
                                   zoom the map; F1-F3 STATS, ITEMS,
                                   DATA
    F5, F9                         save, load (nv-rs-quicksave.txt)
    F12                            report something that differs from
                                   the game: a picture, where you are and
                                   the state, in reports\\NNN (write what's
                                   wrong in its note.txt)
    [ and ] (or - and =)           darker, brighter
    G                              the cell's image space (color
                                   adjustment) off and on
    Home                           back to the start
    Esc                            quit
";

/// The game's opening quest ("Welcome To Fabulous New Vegas"): its first
/// stage moves the player into Doc Mitchell's house and starts his intro
/// (`VCG01` stage 0), through its own scripts.
const NEW_GAME_QUEST: &str = "VCG00";
/// Where to load first for a new game (the scripts move the player there
/// anyway).
const NEW_GAME_CELL: &str = "GSDocMitchellHouse";

/// The CELL when `--load-fos` gives the place.
pub const FROM_SAVE: &str = "(the save's place)";

/// What the viewer was asked to do.
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub data: PathBuf,
    pub cell: String,
    pub options: cellview::Options,
    pub brightness: f32,
    pub at: Option<Stance>,
    pub screenshot: Option<PathBuf>,
    /// Seconds to let the game run before the screenshot.
    pub wait: f32,
    /// Walk even when taking a screenshot.
    pub walk: bool,
    /// Print the frame rate.
    pub fps: bool,
    /// Talk to the nearest person once loaded.
    pub talk: bool,
    /// `--choose`: the replies to pick, in order, as number keys would.
    pub choose: Vec<usize>,
    /// A quest stage to set once loaded: the quest's editor ID and stage.
    pub stage: Option<(String, u16)>,
    /// A ready-made test character to start as (`world::character`).
    pub character: Option<PathBuf>,
    /// `--load-fos`: an original save to start from (`world::fos_import`).
    pub load_fos: Option<PathBuf>,
    /// A weapon to start with, equipped.
    pub weapon: Option<String>,
    /// Script lines to run once loaded, as console commands.
    pub run: Vec<String>,
    /// `--run-at`: script lines to run this many seconds after loading.
    pub run_at: Vec<(f32, String)>,
    /// `--say`: topics to choose in the dialogue menu, in order.
    pub say: Vec<String>,
    /// `--say-id`: INFO records to choose in the dialogue menu, in order.
    pub say_ids: Vec<FormId>,
    /// The player's weather region to start with.
    pub weather_region: Option<String>,
    /// Draw the game's HUD.
    pub hud: bool,
    /// `--vats [N]`: open V.A.T.S. once loaded, and queue and play N
    /// attacks.
    pub vats: Option<u32>,
    /// Seconds of cloud drift to hold the clouds at.
    pub cloud_time: Option<f32>,
    /// Hold everyone's AI still (the console's `tai`).
    pub freeze_ai: bool,
    /// `--background`: a test run that stays out of the way: the window
    /// opens behind the others without taking the focus, and the mouse
    /// is never held in it.
    pub background: bool,
    /// Raise the Pip-Boy once loaded: its menu and page (stats:1).
    pub pipboy: Option<String>,
    /// `--pipboy-keys`: keys to press in the Pip-Boy once it's up.
    pub pipboy_keys: Vec<String>,
    /// `--pad`: the menus as with a pad connected.
    pub pad: bool,
    /// `--lockpick REF`: try this lock once loaded.
    pub lockpick: Option<String>,
    /// `--open-menu`: a game menu to open once loaded (`name[:id]`).
    pub open_menu: Option<String>,
    /// `--use`: an object to use (E) once loaded.
    pub use_on: Option<String>,
    /// `--menu-pointer`: the menus' pointer at this pixel.
    pub menu_pointer: Option<(f32, f32)>,
    /// `--key-at`: keys to press: when, and the key (`KEY[:HOLD]`).
    pub key_at: Vec<(f32, String)>,
    /// `--screen-size`: the screenshot's window size.
    pub screen_size: Option<(u32, u32)>,
    /// Play the movies scripts ask for (`PlayBink`).
    pub movies: bool,
    /// `--menu-click`: when to click (seconds after starting).
    pub menu_clicks: Vec<f64>,
    /// `--menu-keys`: keys typed into the menus (seconds after starting,
    /// the key).
    pub menu_keys: Vec<(f64, String)>,
    /// Use the player-ready `--key-at` clock for `--menu-keys`.
    pub menu_keys_player_clock: bool,
    /// `--answer-boxes`: message boxes answered with their first button,
    /// tutorial boxes closed, once shown (a test aid).
    pub answer_boxes: bool,
    /// `--box-answers`: the buttons for boxes with several, in turn.
    pub box_answers: Vec<usize>,
    /// `--movement`: how the player moves.
    pub movement: MovementMode,
    /// `--movement-probe`.
    pub movement_probe: bool,
    /// `--movement-log`.
    pub movement_log: Option<PathBuf>,
    /// `--movement-missing`.
    pub movement_missing: Option<PathBuf>,
    /// `--physics`: how loose objects move.
    pub physics: PhysicsMode,
}

/// How the player moves (`--movement`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementMode {
    /// The game's own code (`host::player_move`), `crates/physics` for a
    /// frame it can't run.
    #[default]
    Engine,
    /// `crates/physics` only.
    Old,
    /// `crates/physics` moves the player; the game's code runs beside it.
    Compare,
}

/// How loose objects move (`--physics`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PhysicsMode {
    /// `crates/physics`.
    #[default]
    Old,
    /// The game's own Havok world, stepped on its AI thread 2
    /// (`host::player_move::PlayerMovement::run_ai_threads`),
    /// `crates/physics` for a frame it can't run.
    Engine,
}

/// Where to stand, in the game's terms: feet position in game units, and
/// the console's angles in degrees (heading clockwise from north; pitch
/// positive looking down).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stance {
    pub feet: [f32; 3],
    pub heading: f32,
    pub pitch: f32,
}

fn parse_stance(v: &str) -> Result<Stance, String> {
    let numbers: Vec<f32> = v
        .split(',')
        .map(|n| n.trim().parse::<f32>().ok().filter(|f| f.is_finite()))
        .collect::<Option<_>>()
        .ok_or_else(|| format!("--at expects numbers separated by commas, got '{v}'"))?;
    match numbers[..] {
        [x, y, z, heading] => Ok(Stance {
            feet: [x, y, z],
            heading,
            pitch: 0.0,
        }),
        [x, y, z, heading, pitch] => Ok(Stance {
            feet: [x, y, z],
            heading,
            pitch,
        }),
        _ => Err(format!(
            "--at expects X,Y,Z,HEADING or X,Y,Z,HEADING,PITCH, got '{v}'"
        )),
    }
}

/// Parses the arguments after the program name. `Err` carries a message
/// for the user; `Ok(None)` means help was asked for.
pub fn parse(args: &[String]) -> Result<Option<Args>, String> {
    let mut positional = Vec::new();
    let mut options = cellview::Options::default();
    let mut brightness = 1.0;
    let mut at = None;
    let mut screenshot = None;
    let mut wait = 0.0;
    let mut walk = false;
    let mut fps = false;
    let mut talk = false;
    let mut choose = Vec::new();
    let mut stage = None;
    let mut new_game = false;
    let mut weapon = None;
    let mut character = None;
    let mut load_fos = None;
    let mut run = Vec::new();
    let mut run_at = Vec::new();
    let mut say = Vec::new();
    let mut say_ids = Vec::new();
    let mut weather_region = None;
    let mut hud = true;
    let mut vats = None;
    let mut cloud_time = None;
    let mut freeze_ai = false;
    let mut movement = MovementMode::default();
    let mut movement_probe = false;
    let mut movement_log = None;
    let mut movement_missing = None;
    let mut physics = PhysicsMode::default();
    let mut answer_boxes = false;
    let mut box_answers = Vec::new();
    let mut background = false;
    let mut pipboy = None;
    let mut pipboy_keys = Vec::new();
    let mut pad = false;
    let mut lockpick = None;
    let mut open_menu = None;
    let mut use_on = None;
    let mut menu_pointer = None;
    let mut key_at = Vec::new();
    let mut screen_size = None;
    let mut movies = None;
    let mut menu_clicks = Vec::new();
    let mut menu_keys = Vec::new();
    let mut menu_keys_player_clock = false;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let mut value = |flag: &str| {
            iter.next()
                .cloned()
                .ok_or_else(|| format!("{flag} needs a value"))
        };
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--official" => options.official = true,
            "--keep-root-transforms" => options.keep_root_transforms = true,
            "--plugins" => options.plugins_txt = Some(value("--plugins")?.into()),
            "--ini" => options.ini = Some(value("--ini")?.into()),
            "--brightness" => {
                let v = value("--brightness")?;
                brightness = v
                    .parse::<f32>()
                    .ok()
                    .filter(|b| b.is_finite() && *b > 0.0)
                    .ok_or_else(|| format!("--brightness expects a number above 0, got '{v}'"))?;
            }
            "--at" => at = Some(parse_stance(&value("--at")?)?),
            "--screenshot" => screenshot = Some(value("--screenshot")?.into()),
            "--wait" => {
                let v = value("--wait")?;
                wait = v
                    .parse::<f32>()
                    .ok()
                    .filter(|s| s.is_finite() && *s >= 0.0)
                    .ok_or_else(|| format!("--wait expects seconds, got '{v}'"))?;
            }
            "--walk" => walk = true,
            "--fps" => fps = true,
            "--talk" => talk = true,
            "--choose" => {
                let v = value("--choose")?;
                choose = v
                    .split(',')
                    .map(|n| n.trim().parse::<usize>().ok().filter(|n| *n >= 1))
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| {
                        format!("--choose expects reply numbers like 1,2,1, got '{v}'")
                    })?;
            }
            "--stage" => {
                let quest = value("--stage")?;
                let n = value("--stage")?;
                let n = n.parse::<u16>().map_err(|_| {
                    format!("--stage expects a quest and a stage number, got '{n}'")
                })?;
                stage = Some((quest, n));
            }
            "--new-game" => new_game = true,
            "--weapon" => weapon = Some(value("--weapon")?),
            "--character" => character = Some(value("--character")?.into()),
            "--load-fos" => load_fos = Some(value("--load-fos")?.into()),
            "--run" => run.push(value("--run")?),
            "--run-at" => {
                let v = value("--run-at")?;
                let at = v
                    .parse::<f32>()
                    .ok()
                    .filter(|s| s.is_finite() && *s >= 0.0)
                    .ok_or_else(|| format!("--run-at expects seconds and a line, got '{v}'"))?;
                run_at.push((at, value("--run-at")?));
            }
            "--key-at" => {
                let v = value("--key-at")?;
                let at = v
                    .parse::<f32>()
                    .ok()
                    .filter(|s| s.is_finite() && *s >= 0.0)
                    .ok_or_else(|| format!("--key-at expects seconds and a key, got '{v}'"))?;
                key_at.push((at, value("--key-at")?));
            }
            "--say" => say.push(value("--say")?),
            "--say-id" => {
                let v = value("--say-id")?;
                let digits = v
                    .strip_prefix("0x")
                    .or_else(|| v.strip_prefix("0X"))
                    .unwrap_or(&v);
                let id = u32::from_str_radix(digits, 16).map_err(|_| {
                    format!("--say-id expects a hexadecimal INFO form ID, got '{v}'")
                })?;
                say_ids.push(FormId(id));
            }
            "--weather-region" => weather_region = Some(value("--weather-region")?),
            "--no-hud" => hud = false,
            "--movies" => movies = Some(true),
            "--no-movies" => movies = Some(false),
            "--freeze-ai" => freeze_ai = true,
            "--movement" => {
                movement = match value("--movement")?.as_str() {
                    "engine" => MovementMode::Engine,
                    "old" => MovementMode::Old,
                    "compare" => MovementMode::Compare,
                    other => {
                        return Err(format!(
                            "--movement expects engine, old or compare, got '{other}'"
                        ))
                    }
                }
            }
            "--physics" => {
                physics = match value("--physics")?.as_str() {
                    "engine" => PhysicsMode::Engine,
                    "old" => PhysicsMode::Old,
                    other => return Err(format!("--physics expects engine or old, got '{other}'")),
                }
            }
            "--movement-probe" => movement_probe = true,
            "--movement-log" => movement_log = Some(PathBuf::from(value("--movement-log")?)),
            "--movement-missing" => {
                movement_missing = Some(PathBuf::from(value("--movement-missing")?))
            }
            "--answer-boxes" => answer_boxes = true,
            "--box-answers" => {
                let v = value("--box-answers")?;
                for n in v.split(',') {
                    box_answers.push(n.trim().parse::<usize>().map_err(|_| {
                        format!(
                            "--box-answers expects button numbers separated by commas, got '{v}'"
                        )
                    })?);
                }
            }
            "--background" => background = true,
            "--lockpick" => lockpick = Some(value("--lockpick")?),
            "--pipboy" => {
                let v = value("--pipboy")?;
                let menu = v.split(':').next().unwrap_or_default().to_ascii_lowercase();
                if !["stats", "items", "data"].contains(&menu.as_str()) {
                    return Err(format!("--pipboy expects stats, items or data, got '{v}'"));
                }
                pipboy = Some(v);
            }
            "--pipboy-keys" => {
                let v = value("--pipboy-keys")?;
                for k in v.split(',').map(|k| k.trim().to_ascii_lowercase()) {
                    let known = [
                        "up", "down", "left", "right", "enter", "padx", "pady", "close",
                    ]
                    .contains(&k.as_str())
                        || (k.len() == 1 && k.as_bytes()[0].is_ascii_lowercase());
                    if !known {
                        return Err(format!("--pipboy-keys: don't know the key '{k}'"));
                    }
                    pipboy_keys.push(k);
                }
            }
            "--pad" => pad = true,
            "--open-menu" => open_menu = Some(value("--open-menu")?),
            "--use" => use_on = Some(value("--use")?),
            "--menu-pointer" => {
                let v = value("--menu-pointer")?;
                let p: Vec<f32> = v.split(',').filter_map(|n| n.trim().parse().ok()).collect();
                match p[..] {
                    [x, y] => menu_pointer = Some((x, y)),
                    _ => return Err(format!("--menu-pointer expects X,Y, got '{v}'")),
                }
            }
            "--screen-size" => {
                let v = value("--screen-size")?;
                let p: Vec<u32> = v.split(',').filter_map(|n| n.trim().parse().ok()).collect();
                match p[..] {
                    [w, h] if w > 0 && h > 0 => screen_size = Some((w, h)),
                    _ => return Err(format!("--screen-size expects W,H, got '{v}'")),
                }
            }
            "--menu-keys" => {
                let v = value("--menu-keys")?;
                for item in v.split(',') {
                    let parsed = item
                        .split_once(':')
                        .and_then(|(s, k)| Some((s.trim().parse::<f64>().ok()?, k.to_string())));
                    let Some((at, key)) = parsed.filter(|(_, k)| !k.is_empty()) else {
                        return Err(format!(
                            "--menu-keys expects S:K separated by commas, got '{v}'"
                        ));
                    };
                    menu_keys.push((at, key));
                }
            }
            "--menu-keys-player-clock" => menu_keys_player_clock = true,
            "--menu-click" => {
                let v = value("--menu-click")?;
                for n in v.split(',') {
                    menu_clicks.push(n.trim().parse::<f64>().map_err(|_| {
                        format!("--menu-click expects seconds separated by commas, got '{v}'")
                    })?);
                }
            }
            "--cloud-time" => {
                let v = value("--cloud-time")?;
                cloud_time = Some(
                    v.parse::<f32>()
                        .ok()
                        .filter(|s| s.is_finite() && *s >= 0.0)
                        .ok_or_else(|| format!("--cloud-time expects seconds, got '{v}'"))?,
                );
            }
            "--vats" => {
                // An optional count of attacks after it.
                let n = iter.clone().next().and_then(|v| v.parse::<u32>().ok());
                if n.is_some() {
                    iter.next();
                }
                vats = Some(n.unwrap_or(0));
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            _ => positional.push(arg.clone()),
        }
    }
    // A new game: the opening quest's first stage (its scripts move the
    // player to where the game starts), from Doc Mitchell's house unless
    // a place is given.
    if new_game {
        if stage.is_some() {
            return Err("--new-game sets the opening quest's stage; leave out --stage".into());
        }
        stage = Some((NEW_GAME_QUEST.to_string(), 0));
        if positional.len() == 1 {
            positional.push(NEW_GAME_CELL.to_string());
        }
    }
    let movies = movies.unwrap_or(screenshot.is_none());
    // An original save says where to start.
    if load_fos.is_some() {
        if new_game || stage.is_some() {
            return Err("--load-fos starts from the save; leave out --new-game and --stage".into());
        }
        if positional.len() == 1 {
            positional.push(FROM_SAVE.to_string());
        }
    }
    match positional.as_slice() {
        [data, cell] => Ok(Some(Args {
            data: data.into(),
            cell: cell.clone(),
            options,
            brightness,
            at,
            screenshot,
            wait,
            walk,
            fps,
            talk,
            choose,
            stage,
            character,
            load_fos,
            weapon,
            run,
            run_at,
            say,
            say_ids,
            weather_region,
            hud,
            vats,
            cloud_time,
            freeze_ai,
            background,
            movement,
            movement_probe,
            movement_log,
            movement_missing,
            physics,
            pipboy,
            pipboy_keys,
            pad,
            lockpick,
            open_menu,
            use_on,
            menu_pointer,
            key_at,
            screen_size,
            movies,
            menu_clicks,
            menu_keys,
            menu_keys_player_clock,
            answer_boxes,
            box_answers,
        })),
        [] | [_] => Err("expected the Data folder and a cell".into()),
        [_, _, extra, ..] => Err(format!("unexpected argument '{extra}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movies_play_unless_skipped_or_taking_a_screenshot() {
        let p = |a: &[&str]| parse(&strings(a)).unwrap().unwrap().movies;
        assert!(p(&["Data", "Cell"]));
        assert!(!p(&["Data", "Cell", "--no-movies"]));
        assert!(!p(&["Data", "Cell", "--screenshot", "a.png"]));
        assert!(p(&["Data", "Cell", "--screenshot", "a.png", "--movies"]));
    }

    fn strings(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_the_folder_cell_and_options() {
        let args = parse(&strings(&[
            "C:\\Games\\FNV",
            "GSDocMitchellHouse",
            "--official",
            "--brightness",
            "1.5",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(args.data, PathBuf::from("C:\\Games\\FNV"));
        assert_eq!(args.cell, "GSDocMitchellHouse");
        assert!(args.options.official);
        assert_eq!(args.brightness, 1.5);
        assert_eq!(args.at, None);
        assert!(args.hud);
        let bare = parse(&strings(&["Data", "Cell", "--no-hud"]))
            .unwrap()
            .unwrap();
        assert!(!bare.hud);
        assert!(!bare.freeze_ai);
        let frozen = parse(&strings(&["Data", "Cell", "--freeze-ai"]))
            .unwrap()
            .unwrap();
        assert!(frozen.freeze_ai);
        assert!(!frozen.answer_boxes);
        let answering = parse(&strings(&["Data", "Cell", "--answer-boxes"]))
            .unwrap()
            .unwrap();
        assert!(answering.answer_boxes);
        let choosing = parse(&strings(&["Data", "Cell", "--box-answers", "1, 0"]))
            .unwrap()
            .unwrap();
        assert_eq!(choosing.box_answers, vec![1, 0]);
        let menu_clock = parse(&strings(&[
            "Data",
            "Cell",
            "--menu-keys-player-clock",
            "--menu-keys",
            "2:right",
        ]))
        .unwrap()
        .unwrap();
        assert!(menu_clock.menu_keys_player_clock);
        assert_eq!(menu_clock.menu_keys, [(2.0, "right".to_string())]);
        assert!(parse(&strings(&["Data", "Cell", "--box-answers", "no"])).is_err());
        let character = parse(&strings(&["Data", "Cell", "--character", "c.txt"]))
            .unwrap()
            .unwrap();
        assert_eq!(character.character, Some(PathBuf::from("c.txt")));
        assert_eq!(frozen.character, None);
    }

    #[test]
    fn parses_later_lines_and_topics_to_choose() {
        let args = parse(&strings(&[
            "Data",
            "Cell",
            "--run-at",
            "40",
            "SunnyREF.evp",
            "--say",
            "I'm in",
            "--run-at",
            "2.5",
            "SetStage VCG02 30",
            "--say",
            "Sure",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(
            args.run_at,
            [
                (40.0, "SunnyREF.evp".to_string()),
                (2.5, "SetStage VCG02 30".to_string())
            ]
        );
        assert_eq!(args.say, ["I'm in", "Sure"]);
        let by_id = parse(&strings(&["Data", "Cell", "--say-id", "00144BAF"]))
            .unwrap()
            .unwrap();
        assert_eq!(by_id.say_ids, [FormId(0x0014_4BAF)]);
        assert!(parse(&strings(&["Data", "Cell", "--say-id", "not-a-form"]))
            .unwrap_err()
            .contains("--say-id"));
        assert!(parse(&strings(&["Data", "Cell", "--run-at", "soon", "x"]))
            .unwrap_err()
            .contains("--run-at"));
    }

    #[test]
    fn parses_a_stance_and_a_screenshot() {
        let args = parse(&strings(&[
            "Data",
            "Cell",
            "--at",
            "2352,1604.5,7360,270",
            "--screenshot",
            "out.png",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(
            args.at,
            Some(Stance {
                feet: [2352.0, 1604.5, 7360.0],
                heading: 270.0,
                pitch: 0.0
            })
        );
        assert_eq!(args.screenshot, Some(PathBuf::from("out.png")));
        let tilted = parse(&strings(&["Data", "Cell", "--at", "1,2,3,90,-10"]))
            .unwrap()
            .unwrap();
        assert_eq!(tilted.at.unwrap().pitch, -10.0);
        assert!(parse(&strings(&["Data", "Cell", "--at", "1,2,3"]))
            .unwrap_err()
            .contains("X,Y,Z,HEADING"));
    }

    #[test]
    fn an_original_save_gives_the_place() {
        let args = parse(&strings(&["C:\\Games\\FNV", "--load-fos", "Save 1.fos"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.load_fos, Some(PathBuf::from("Save 1.fos")));
        assert_eq!(args.cell, FROM_SAVE);
        assert!(parse(&strings(&["D", "--load-fos", "a.fos", "--new-game"])).is_err());
        assert!(parse(&strings(&["D", "--load-fos", "a.fos", "--stage", "Q", "1"])).is_err());
        assert!(parse(&strings(&["D", "--load-fos"])).is_err());
    }

    #[test]
    fn a_new_game_starts_the_opening_quest() {
        let args = parse(&strings(&["Data", "--new-game"])).unwrap().unwrap();
        assert_eq!(args.stage, Some(("VCG00".to_string(), 0)));
        assert_eq!(args.cell, "GSDocMitchellHouse");
        assert!(parse(&strings(&["Data", "--new-game", "--stage", "Q", "1"])).is_err());
    }

    #[test]
    fn parses_a_weather_region() {
        let args = parse(&strings(&[
            "Data",
            "Cell",
            "--weather-region",
            "VMapGoodspringsRegion",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(
            args.weather_region.as_deref(),
            Some("VMapGoodspringsRegion")
        );
        assert!(parse(&strings(&["Data", "Cell", "--weather-region"])).is_err());
    }

    #[test]
    fn parses_vats_with_or_without_a_count() {
        let args = parse(&strings(&["Data", "Cell", "--vats"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.vats, Some(0));
        let args = parse(&strings(&["Data", "Cell", "--vats", "3", "--fps"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.vats, Some(3));
        assert!(args.fps);
        // The cell can come after it.
        let args = parse(&strings(&["Data", "--vats", "Cell"]))
            .unwrap()
            .unwrap();
        assert_eq!((args.vats, args.cell.as_str()), (Some(0), "Cell"));
    }

    #[test]
    fn parses_a_lock_to_try() {
        let args = parse(&strings(&["Data", "Cell", "--lockpick", "SafeREF"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.lockpick.as_deref(), Some("SafeREF"));
        assert!(parse(&strings(&["Data", "Cell", "--lockpick"])).is_err());
    }

    #[test]
    fn parses_a_cloud_time() {
        let args = parse(&strings(&["Data", "Cell", "--cloud-time", "58.022"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.cloud_time, Some(58.022));
        assert!(parse(&strings(&["Data", "Cell", "--cloud-time", "-1"])).is_err());
    }

    #[test]
    fn parses_how_the_player_moves() {
        let bare = parse(&strings(&["Data", "Cell"])).unwrap().unwrap();
        assert_eq!(bare.movement, MovementMode::Engine);
        assert!(!bare.movement_probe && bare.movement_log.is_none());
        let args = parse(&strings(&[
            "Data",
            "Cell",
            "--movement",
            "compare",
            "--movement-probe",
            "--movement-log",
            "m.tsv",
            "--movement-missing",
            "x.tsv",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(args.movement, MovementMode::Compare);
        assert!(args.movement_probe);
        assert_eq!(args.movement_log, Some(PathBuf::from("m.tsv")));
        assert_eq!(args.movement_missing, Some(PathBuf::from("x.tsv")));
        assert!(parse(&strings(&["Data", "Cell", "--movement", "fast"])).is_err());
    }

    #[test]
    fn parses_the_pipboy_to_raise() {
        let args = parse(&strings(&["Data", "Cell", "--pipboy", "items:1"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.pipboy.as_deref(), Some("items:1"));
        assert!(parse(&strings(&["Data", "Cell", "--pipboy", "map"]))
            .unwrap_err()
            .contains("stats, items or data"));
    }

    #[test]
    fn parses_a_menu_to_open() {
        let args = parse(&strings(&["Data", "Cell", "--open-menu", "container:Box"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.open_menu.as_deref(), Some("container:Box"));
        let args = parse(&strings(&["Data", "Cell", "--use", "TerminalRef"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.use_on.as_deref(), Some("TerminalRef"));
        assert!(parse(&strings(&["Data", "Cell", "--use"])).is_err());
        let args = parse(&strings(&["Data", "Cell", "--menu-pointer", "10,20.5"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.menu_pointer, Some((10.0, 20.5)));
        assert!(parse(&strings(&["Data", "Cell", "--menu-pointer", "10"])).is_err());
        let args = parse(&strings(&["Data", "Cell", "--screen-size", "1024,768"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.screen_size, Some((1024, 768)));
        assert!(parse(&strings(&["Data", "Cell", "--screen-size", "0,768"])).is_err());
        assert!(parse(&strings(&["Data", "Cell", "--open-menu"])).is_err());
    }

    #[test]
    fn parses_a_quest_stage() {
        let args = parse(&strings(&["Data", "Cell", "--stage", "VCG01", "0"]))
            .unwrap()
            .unwrap();
        assert_eq!(args.stage, Some(("VCG01".to_string(), 0)));
        assert!(parse(&strings(&["Data", "Cell", "--stage", "VCG01", "x"]))
            .unwrap_err()
            .contains("stage number"));
    }

    #[test]
    fn explains_mistakes() {
        assert_eq!(parse(&strings(&["--help"])), Ok(None));
        assert!(parse(&strings(&["Data"])).unwrap_err().contains("a cell"));
        assert!(parse(&strings(&["Data", "Cell", "--brightness", "-1"]))
            .unwrap_err()
            .contains("above 0"));
        assert_eq!(
            parse(&strings(&["Data", "Cell", "--fly"])).unwrap_err(),
            "unknown option '--fly'"
        );
        assert_eq!(
            parse(&strings(&["Data", "Cell", "More"])).unwrap_err(),
            "unexpected argument 'More'"
        );
    }
}
