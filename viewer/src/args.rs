//! Command-line arguments. Kept free of Bevy so it can be tested on its own.

use std::path::PathBuf;

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
    --no-hud                leave out the game's HUD (health, compass,
                            crosshair, messages); screenshots then show
                            the scene alone
    --coop-host [PORT]      host a co-op session on UDP PORT (default 27961):
                            quest and story state is shared with guests
    --coop-join ADDR        join a host: an NVRS- join code (UDP or a Steam
                            lobby), or HOST[:PORT] together with
                            --coop-test-token.
                            The CELL can be left out (you load the host's
                            place); --new-game and --load-fos are the
                            host's. --stage and --run lines are sent to the
                            host and run there
    --coop-steam            use Steam (Spacewar, AppID 480) instead of UDP:
                            with --coop-host, create a friends-only lobby
                            and log its join code; with --coop-join, the
                            code names the lobby. Needs Steam running and
                            steam_api64.dll next to this exe; otherwise
                            falls back to UDP (logged as transport_fallback)
    --coop-steam-friend     join a friend who is hosting through Steam, no
                            code needed (implies --coop-steam)
    --coop-test-token       test harness only: use the fixed session token,
                            so a plain HOST:PORT works through a relay
    --coop-token HEX        host: use this 32 hex digit session token (the
                            launcher makes one so its join code matches)
    --coop-load DIR         host: continue a campaign from a party save folder
                            (SAVE.partysave next to a saved game): the world
                            and the host's character start from it, and
                            returning guests get their saved characters back
                            by their player id
    --coop-character FILE   guest: this player's own character file; it is
                            written when the host saves (and on F5) and read
                            on F9 and when joining a campaign that has no
                            character for this player
    --coop-player-key HEX   guest: this player's persistent id (16 hex
                            digits), so a rejoining guest finds its character
    --coop-name NAME        your name in the session
    --coop-log FILE         write the co-op event log (JSON lines) here
    --coop-state-dump FILE  write the shared state (JSON) here every second
                            and at exit
    --coop-net PROFILE      for testing: delay and lose this peer's packets
                            as the profile says (perfect, good_wifi,
                            bad_wifi, mobile, congested, flaky, partition)
    --coop-control PORT     a live control port on 127.0.0.1:PORT (JSON lines:
                            state, run, bot, keys, screenshot, entities,
                            quit; tools/coop-run/live.py talks to it)
    --coop-allow-mismatch   developer override: accept a guest whose plugins,
                            load order, game version or build differ (the
                            host's flag decides; protocol and rules still
                            must match)
    --coop-voice MODE       voice chat: off, ptt (push to talk, the default)
                            or vad (voice activity). Needs a viewer built
                            with --features voice
    --coop-ptt-key KEY      push-to-talk key (default Backslash; a KeyCode
                            name or MouseMiddle/MouseBack/MouseForward)
    --coop-ping-key KEY     ping key (default MouseMiddle); Shift sets the
                            waypoint, Ctrl clears it
    --coop-couch SEAT/PLAYERS
                            split-screen on one PC: this process is local
                            seat SEAT (0 hosts) of PLAYERS; uses the fixed
                            local session token (see docs/COOP-COUCH.md)
    --coop-window X,Y,W,H   place the window (borderless) at this rectangle
    --coop-gamepad N        the controller this seat uses (recorded and
                            logged; the launcher routes the input)
    --coop-quit-file FILE   quit cleanly when this file appears
    --coop-quit-after SECONDS
                            quit cleanly this long after the session is
                            ready
    --background            a test run out of the way: the window opens
                            behind the others without the focus, and the
                            mouse is never held in it (with --screen-size,
                            at that size)
    --help-overlay          show the key help at the top of the screen in a
                            co-op session too (hidden there by default;
                            F4 turns it on and off)
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
                            many seconds after you're placed (with
                            --coop-go-file, after the go file), held HOLD
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

/// What `--coop-help` prints.
pub const COOP_USAGE: &str = "\
nv-viewer co-op flags (docs/COOP.md)

  --coop-host [PORT]       host on UDP PORT (default 27961)
  --coop-join CODE         join with an NVRS- code (UDP or Steam lobby); HOST[:PORT] with --coop-test-token
  --coop-steam             Steam (Spacewar) transport
  --coop-steam-friend      join a Steam friend who is hosting, no code
  --coop-token HEX         host: the session token the join code carries
  --coop-test-token        test harness: the fixed token
  --coop-name NAME         your name in the session
  --coop-log FILE          event log (JSON lines)
  --coop-state-dump FILE   shared-state JSON once a second (has the start.phase object)
  --coop-net PROFILE       test: delay and lose packets
  --coop-quit-file FILE    quit cleanly when the file appears
  --coop-go-file FILE      start the scripts when the file appears (after the session is ready)
  --coop-quit-after SEC    quit cleanly this long after the session is ready
  --coop-load FILE         host: start from this nv-rs save (the launcher's Continue);
                           guest: take only your character from it
  --coop-no-chargen        test harness: a guest skips making its character
  --coop-difficulty MODE   host: normal or balanced (the default), kept in the party save
  --coop-friendly-fire SW  host: on or off (the default), kept in the party save
  --coop-help              this list

The start of a game:
  host   --new-game        the normal opening (Doc Mitchell, name, Vit-o-Matic, tags, traits)
  host   --coop-load FILE  or --load-fos FILE: play on from a save
  guest  (nothing)         joins any time; makes its own character with the same menus
  guest  --new-game        the same, said out loud
  guest  --load-fos / --coop-load FILE
                           brings its character from a save; the host's world is never replaced
";

/// The CELL when `--coop-join` gives the place (the host's).
pub const COOP_PLACE: &str = "(the host's place)";

/// Chaos profile names `--coop-net` takes (`coop_core::transport`).
pub const COOP_PROFILES: [&str; 7] = [
    "perfect",
    "good_wifi",
    "bad_wifi",
    "mobile",
    "congested",
    "flaky",
    "partition",
];

/// Port `--coop-host` listens on when none is given.
pub const COOP_DEFAULT_PORT: u16 = 27961;

/// The `--coop-*` options.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CoopArgs {
    /// `--coop-host [PORT]`: host on this UDP port.
    pub host: Option<u16>,
    /// `--coop-join ADDR`: join this host (address or join code).
    pub join: Option<String>,
    /// `--coop-load PATH`: the host starts from this party save or one of this viewer's own saves.
    pub load: Option<PathBuf>,
    /// `--coop-character FILE`: the guest's own character file.
    pub character: Option<PathBuf>,
    /// `--coop-player-key HEX`: the guest's persistent player id.
    pub player_key: Option<u64>,
    /// `--coop-name`.
    pub name: Option<String>,
    /// `--coop-log`.
    pub log: Option<PathBuf>,
    /// `--coop-state-dump`.
    pub state_dump: Option<PathBuf>,
    /// `--coop-net`: chaos profile name.
    pub net: Option<String>,
    /// `--coop-control`: the live control port (localhost only).
    pub control: Option<u16>,
    /// `--coop-quit-file`.
    pub quit_file: Option<PathBuf>,
    /// `--coop-go-file`: scripts start when this file appears (after Ready).
    pub go_file: Option<PathBuf>,
    /// `--coop-quit-after`: seconds after the session is ready.
    pub quit_after: Option<f32>,
    /// `--coop-steam`: Steam (Spacewar) transport instead of UDP.
    pub steam: bool,
    /// `--coop-steam-friend`: join a hosting Steam friend (a guest, no code).
    pub steam_friend: bool,
    /// `--coop-test-token`: the fixed test token (plain HOST:PORT, harness only).
    pub test_token: bool,
    /// `--coop-token HEX`: the host's session token, 16 bytes.
    pub token: Option<[u8; 16]>,
    /// Host: `--new-game` (the opening runs for the host player, at once).
    pub new_game: bool,
    /// Guest: `--new-game`, make a new character (the default for a guest).
    pub own_new_game: bool,
    /// Guest: `--load-fos FILE` or `--coop-load FILE`, take the character from that save
    /// (only the name, SPECIAL, tags and perks; the host's world is never replaced).
    pub own_load: Option<PathBuf>,
    /// `--coop-no-chargen`: test harness only, a guest skips the character making.
    pub no_chargen: bool,
    /// `--coop-allow-mismatch`: accept guests whose game differs (developers).
    pub allow_mismatch: bool,
    /// Host: `--coop-difficulty normal|balanced` and `--coop-friendly-fire on|off`, the
    /// campaign rules (`None`: a loaded party save's, else Co-op balanced, friendly fire off).
    pub rules: Option<world::coop::party_save::PartyRules>,
    /// `--coop-voice`: off, ptt or vad.
    pub voice: Option<String>,
    /// `--coop-ptt-key`.
    pub ptt_key: Option<String>,
    /// `--coop-ping-key`.
    pub ping_key: Option<String>,
    /// `--coop-couch SEAT/PLAYERS`.
    pub couch: Option<(u8, u8)>,
    /// `--coop-window X,Y,W,H`.
    pub window: Option<(i32, i32, u32, u32)>,
    /// `--coop-gamepad N`.
    pub gamepad: Option<u8>,
}

/// Parses 32 hex digits into a session token.
pub fn parse_token_hex(s: &str) -> Option<[u8; 16]> {
    let s = s.trim();
    if s.len() != 32 || !s.is_ascii() {
        return None;
    }
    let mut out = [0u8; 16];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

impl CoopArgs {
    /// True when a session was asked for.
    pub fn active(&self) -> bool {
        self.host.is_some() || self.join.is_some() || self.steam_friend
    }

    /// True when this process joins someone (by address, code or Steam friend).
    pub fn is_guest(&self) -> bool {
        self.join.is_some() || self.steam_friend
    }
}

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
    /// The `--coop-*` options.
    pub coop: CoopArgs,
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
    /// `--help-overlay`: keep the key help line on screen in a co-op session.
    pub help_overlay: bool,
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
    let mut coop = CoopArgs::default();
    let mut new_game = false;
    let mut weapon = None;
    let mut character = None;
    let mut load_fos = None;
    let mut coop_load: Option<PathBuf> = None;
    let mut run = Vec::new();
    let mut run_at = Vec::new();
    let mut say = Vec::new();
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
    let mut help_overlay = false;
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
            "--coop-load" => coop_load = Some(value("--coop-load")?.into()),
            "--coop-no-chargen" => coop.no_chargen = true,
            "--coop-help" => {
                print!("{COOP_USAGE}");
                std::process::exit(0);
            }
            "--coop-host" => {
                // An optional port after it.
                let n = iter.clone().next().and_then(|v| v.parse::<u16>().ok());
                if n.is_some() {
                    iter.next();
                }
                coop.host = Some(n.unwrap_or(COOP_DEFAULT_PORT)).filter(|p| *p != 0);
                if coop.host.is_none() {
                    return Err("--coop-host expects a port from 1 to 65535".into());
                }
            }
            "--coop-join" => {
                let v = value("--coop-join")?;
                if v.trim().is_empty() {
                    return Err("--coop-join expects HOST[:PORT] or a join code".into());
                }
                coop.join = Some(v);
            }
            "--coop-steam" => coop.steam = true,
            "--coop-steam-friend" => {
                coop.steam_friend = true;
                coop.steam = true;
            }
            "--coop-test-token" => coop.test_token = true,
            "--coop-token" => {
                let v = value("--coop-token")?;
                coop.token = Some(parse_token_hex(&v).ok_or_else(|| {
                    "--coop-token expects 32 hex digits (a 16 byte token)".to_string()
                })?);
            }
            "--coop-allow-mismatch" => coop.allow_mismatch = true,
            "--coop-difficulty" => {
                let v = value("--coop-difficulty")?;
                let mode =
                    world::coop::party_save::PartyRules::parse_mode(&v).ok_or_else(|| {
                        format!("--coop-difficulty expects normal or balanced, got '{v}'")
                    })?;
                coop.rules.get_or_insert_with(Default::default).mode = mode;
            }
            "--coop-friendly-fire" => {
                let v = value("--coop-friendly-fire")?;
                let on = world::coop::party_save::PartyRules::parse_switch(&v)
                    .ok_or_else(|| format!("--coop-friendly-fire expects on or off, got '{v}'"))?;
                coop.rules
                    .get_or_insert_with(Default::default)
                    .friendly_fire = on;
            }
            "--coop-voice" => {
                let v = value("--coop-voice")?;
                if !["off", "ptt", "vad"].contains(&v.as_str()) {
                    return Err(format!("--coop-voice expects off, ptt or vad, got '{v}'"));
                }
                coop.voice = Some(v);
            }
            "--coop-ptt-key" => coop.ptt_key = Some(value("--coop-ptt-key")?),
            "--coop-ping-key" => coop.ping_key = Some(value("--coop-ping-key")?),
            "--coop-couch" => {
                let v = value("--coop-couch")?;
                let seat = v.split_once('/').and_then(|(a, b)| {
                    Some((a.trim().parse::<u8>().ok()?, b.trim().parse::<u8>().ok()?))
                });
                match seat {
                    Some((s, n)) if (1..=4).contains(&n) && s < n => {
                        coop.couch = Some((s, n));
                        // all seats are on this PC: the fixed local token, no join code
                        coop.test_token = true;
                        if s != 0 && coop.voice.is_none() {
                            coop.voice = Some("off".into());
                        }
                    }
                    _ => return Err(format!("--coop-couch expects SEAT/PLAYERS with SEAT below PLAYERS (1 to 4), got '{v}'")),
                }
            }
            "--coop-window" => {
                let v = value("--coop-window")?;
                let p: Vec<i64> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
                if p.len() != 4 || v.split(',').count() != 4 || p[2] < 160 || p[3] < 120 {
                    return Err(format!(
                        "--coop-window expects X,Y,W,H (at least 160x120), got '{v}'"
                    ));
                }
                coop.window = Some((p[0] as i32, p[1] as i32, p[2] as u32, p[3] as u32));
            }
            "--coop-gamepad" => {
                let v = value("--coop-gamepad")?;
                coop.gamepad = Some(v.parse::<u8>().map_err(|_| {
                    format!("--coop-gamepad expects a controller number, got '{v}'")
                })?);
            }
            "--coop-name" => coop.name = Some(value("--coop-name")?),
            "--coop-character" => coop.character = Some(value("--coop-character")?.into()),
            "--coop-player-key" => {
                let v = value("--coop-player-key")?;
                coop.player_key = Some(
                    u64::from_str_radix(v.trim(), 16)
                        .ok()
                        .filter(|k| *k != 0 && v.trim().len() == 16)
                        .ok_or_else(|| {
                            "--coop-player-key expects 16 hex digits (not all zero)".to_string()
                        })?,
                );
            }
            "--coop-log" => coop.log = Some(value("--coop-log")?.into()),
            "--coop-state-dump" => coop.state_dump = Some(value("--coop-state-dump")?.into()),
            "--coop-net" => {
                let v = value("--coop-net")?;
                if !COOP_PROFILES.contains(&v.as_str()) {
                    return Err(format!(
                        "--coop-net expects one of {}, got '{v}'",
                        COOP_PROFILES.join(", ")
                    ));
                }
                coop.net = Some(v);
            }
            "--coop-control" => {
                let v = value("--coop-control")?;
                coop.control = Some(
                    v.parse::<u16>()
                        .ok()
                        .filter(|p| *p > 0)
                        .ok_or_else(|| format!("--coop-control expects a port, got '{v}'"))?,
                );
            }
            "--coop-quit-file" => coop.quit_file = Some(value("--coop-quit-file")?.into()),
            "--coop-go-file" => coop.go_file = Some(value("--coop-go-file")?.into()),
            "--coop-quit-after" => {
                let v = value("--coop-quit-after")?;
                coop.quit_after = Some(
                    v.parse::<f32>()
                        .ok()
                        .filter(|s| s.is_finite() && *s >= 0.0)
                        .ok_or_else(|| format!("--coop-quit-after expects seconds, got '{v}'"))?,
                );
            }
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
            "--help-overlay" => help_overlay = true,
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
    if coop.host.is_some() && coop.is_guest() {
        return Err("--coop-host and --coop-join are different roles; give one".into());
    }
    if coop.steam_friend && coop.join.is_some() {
        return Err("--coop-steam-friend finds the host by itself; leave out --coop-join".into());
    }
    if coop.token.is_some() && coop.host.is_none() {
        return Err("--coop-token is the host's session token; use it with --coop-host".into());
    }
    if coop.test_token && coop.token.is_some() {
        return Err("--coop-test-token and --coop-token both set the token; give one".into());
    }
    if coop.test_token && coop.steam {
        return Err("--coop-test-token is for UDP test runs; it does not apply to Steam".into());
    }
    if coop.load.is_some() && coop.host.is_none() {
        return Err(
            "--coop-load continues a party save as the host; use it with --coop-host".into(),
        );
    }
    if coop.load.is_some() && (new_game || load_fos.is_some() || stage.is_some()) {
        return Err(
            "--coop-load starts from the party save; leave out --new-game, --load-fos and --stage"
                .into(),
        );
    }
    if (coop.character.is_some() || coop.player_key.is_some()) && !coop.is_guest() {
        return Err(
            "--coop-character and --coop-player-key are the guest's; use them with --coop-join"
                .into(),
        );
    }
    if coop.load.is_some() && positional.len() == 1 {
        positional.push(FROM_SAVE.to_string());
    }
    if !coop.active()
        && (coop.name.is_some()
            || coop.load.is_some()
            || coop.character.is_some()
            || coop.player_key.is_some()
            || coop.log.is_some()
            || coop.state_dump.is_some()
            || coop.net.is_some()
            || coop.quit_file.is_some()
            || coop.go_file.is_some()
            || coop.control.is_some()
            || coop.quit_after.is_some()
            || coop.steam
            || coop.test_token
            || coop.no_chargen)
    {
        return Err("the --coop-* options need --coop-host or --coop-join".into());
    }
    // A guest never replaces the host's game: --new-game, --load-fos and --coop-load mean "my own
    // character" for it (docs/COOP.md, "The start of a game").
    if coop.is_guest() {
        if new_game && (load_fos.is_some() || coop_load.is_some()) {
            return Err(
                "a guest's own character comes from --new-game or from a save, not both".into(),
            );
        }
        if load_fos.is_some() && coop_load.is_some() {
            return Err(
                "give one of --load-fos and --coop-load for a guest's own character".into(),
            );
        }
        coop.own_new_game = std::mem::take(&mut new_game);
        coop.own_load = load_fos.take().or_else(|| coop_load.take());
    } else if coop.host.is_some() {
        coop.new_game = new_game;
        if coop_load.is_some() {
            if new_game || load_fos.is_some() || stage.is_some() {
                return Err("--coop-load starts from the party save or nv-rs save; leave out --new-game, --load-fos and --stage".into());
            }
            coop.load = coop_load.take();
        }
    } else if coop_load.is_some() {
        return Err("--coop-load needs --coop-host or --coop-join".into());
    }
    if coop.no_chargen && !coop.is_guest() {
        return Err("--coop-no-chargen is for a guest (--coop-join)".into());
    }
    if coop.rules.is_some() && coop.host.is_none() {
        return Err(
            "--coop-difficulty and --coop-friendly-fire are for the host (--coop-host)".into(),
        );
    }
    if coop.is_guest() && positional.len() == 1 {
        positional.push(COOP_PLACE.to_string());
    }
    // A host started from its own save gets its place from the save.
    if coop.load.is_some() && positional.len() == 1 {
        positional.push(FROM_SAVE.to_string());
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
            coop,
            character,
            load_fos,
            weapon,
            run,
            run_at,
            say,
            weather_region,
            hud,
            vats,
            cloud_time,
            freeze_ai,
            background,
            help_overlay,
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
    fn parses_the_party_save_options() {
        let host = parse(&strings(&[
            "Data",
            "--coop-host",
            "--coop-load",
            "Save.txt.partysave",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(host.coop.load, Some(PathBuf::from("Save.txt.partysave")));
        assert_eq!(host.cell, FROM_SAVE, "the party save gives the place");
        let guest = parse(&strings(&[
            "Data",
            "--coop-join",
            "h:1",
            "--coop-character",
            "me.char",
            "--coop-player-key",
            "00000000000a11ce",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(guest.coop.character, Some(PathBuf::from("me.char")));
        assert_eq!(guest.coop.player_key, Some(0xa11ce));
        let err = |a: &[&str]| parse(&strings(a)).unwrap_err();
        assert!(err(&["Data", "Cell", "--coop-load", "x"]).contains("--coop-host"));
        assert!(
            err(&["Data", "--coop-host", "--coop-load", "x", "--new-game"]).contains("party save")
        );
        assert!(err(&[
            "Data",
            "--coop-host",
            "--coop-load",
            "x",
            "--load-fos",
            "a.fos"
        ])
        .contains("party save"));
        assert!(err(&[
            "Data",
            "Cell",
            "--coop-host",
            "--coop-player-key",
            "00000000000a11ce"
        ])
        .contains("guest"));
        assert!(
            err(&["Data", "--coop-join", "h:1", "--coop-player-key", "abc"]).contains("16 hex")
        );
        assert!(err(&[
            "Data",
            "--coop-join",
            "h:1",
            "--coop-player-key",
            "0000000000000000"
        ])
        .contains("16 hex"));
        // an original save can still start a hosted campaign
        let fos = parse(&strings(&[
            "Data",
            "--coop-host",
            "--load-fos",
            "Save 1.fos",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(fos.load_fos, Some(PathBuf::from("Save 1.fos")));
        assert!(fos.coop.load.is_none());
    }

    #[test]
    fn parses_the_coop_options() {
        let host = parse(&strings(&[
            "Data",
            "Cell",
            "--coop-host",
            "4000",
            "--coop-name",
            "Ann",
            "--coop-log",
            "h.jsonl",
            "--coop-state-dump",
            "h.json",
            "--coop-net",
            "bad_wifi",
            "--coop-quit-file",
            "q",
            "--coop-quit-after",
            "30",
            "--coop-control",
            "4100",
        ]))
        .unwrap()
        .unwrap()
        .coop;
        assert_eq!(host.host, Some(4000));
        assert_eq!(host.name.as_deref(), Some("Ann"));
        assert_eq!(host.log, Some(PathBuf::from("h.jsonl")));
        assert_eq!(host.state_dump, Some(PathBuf::from("h.json")));
        assert_eq!(host.net.as_deref(), Some("bad_wifi"));
        assert_eq!(host.quit_file, Some(PathBuf::from("q")));
        assert_eq!(host.quit_after, Some(30.0));
        assert_eq!(host.control, Some(4100));
        assert!(host.active());
        // no port: the default; the option after it is still read
        let bare = parse(&strings(&["Data", "Cell", "--coop-host", "--no-hud"]))
            .unwrap()
            .unwrap();
        assert_eq!(bare.coop.host, Some(COOP_DEFAULT_PORT));
        assert!(!bare.hud);
        assert!(!parse(&strings(&["Data", "Cell"]))
            .unwrap()
            .unwrap()
            .coop
            .active());
    }

    #[test]
    fn a_guest_may_leave_out_the_cell() {
        let g = parse(&strings(&[
            "Data",
            "--coop-join",
            "10.0.0.5:4000",
            "--stage",
            "VCG01",
            "5",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(g.cell, COOP_PLACE);
        assert_eq!(g.coop.join.as_deref(), Some("10.0.0.5:4000"));
        assert_eq!(g.stage, Some(("VCG01".to_string(), 5)));
        let with_cell = parse(&strings(&["Data", "Cell", "--coop-join", "h:1"]))
            .unwrap()
            .unwrap();
        assert_eq!(with_cell.cell, "Cell");
    }

    #[test]
    fn the_start_flags_mean_own_character_for_a_guest_and_the_normal_start_for_a_host() {
        let host = parse(&strings(&["D", "--new-game", "--coop-host"]))
            .unwrap()
            .unwrap();
        assert!(host.coop.new_game);
        assert_eq!(
            host.stage,
            Some(("VCG00".to_string(), 0)),
            "the opening starts at once"
        );
        assert_eq!(host.cell, "GSDocMitchellHouse");
        let guest = parse(&strings(&["D", "--new-game", "--coop-join", "h"]))
            .unwrap()
            .unwrap();
        assert!(guest.coop.own_new_game && !guest.coop.new_game);
        assert_eq!(guest.stage, None, "a guest never sets the host's quest");
        assert_eq!(guest.cell, COOP_PLACE);
        let g = parse(&strings(&["D", "--load-fos", "a.fos", "--coop-join", "h"]))
            .unwrap()
            .unwrap();
        assert_eq!(g.coop.own_load, Some(PathBuf::from("a.fos")));
        assert_eq!(g.load_fos, None, "the host's world is not replaced");
        let g = parse(&strings(&[
            "D",
            "--coop-load",
            "s.txt",
            "--coop-join",
            "h",
            "--coop-no-chargen",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(g.coop.own_load, Some(PathBuf::from("s.txt")));
        assert!(g.coop.no_chargen);
        let h = parse(&strings(&["D", "--coop-load", "s.txt", "--coop-host"]))
            .unwrap()
            .unwrap();
        assert_eq!(h.coop.load, Some(PathBuf::from("s.txt")));
        assert_eq!(h.cell, FROM_SAVE);
        assert!(parse(&strings(&["D", "C", "--coop-load", "s.txt"])).is_err());
        assert!(parse(&strings(&[
            "D",
            "C",
            "--coop-host",
            "--coop-load",
            "s.txt",
            "--new-game"
        ]))
        .is_err());
        assert!(parse(&strings(&["D", "C", "--coop-host", "--coop-no-chargen"])).is_err());
    }

    #[test]
    fn host_chooses_the_campaign_rules() {
        let h = parse(&strings(&[
            "D",
            "C",
            "--coop-host",
            "--coop-difficulty",
            "normal",
            "--coop-friendly-fire",
            "on",
        ]))
        .unwrap()
        .unwrap();
        assert_eq!(
            h.coop.rules.map(|r| (r.mode_name(), r.friendly_fire)),
            Some(("normal", true))
        );
        let h = parse(&strings(&["D", "C", "--coop-host"]))
            .unwrap()
            .unwrap();
        assert_eq!(
            h.coop.rules, None,
            "no flag: the save's rules or the default"
        );
        let err = |a: &[&str]| parse(&strings(a)).unwrap_err();
        assert!(err(&["D", "C", "--coop-host", "--coop-difficulty", "hard"])
            .contains("normal or balanced"));
        assert!(
            err(&["D", "--coop-join", "h", "--coop-friendly-fire", "on"]).contains("for the host")
        );
    }

    #[test]
    fn explains_coop_mistakes() {
        let err = |a: &[&str]| parse(&strings(a)).unwrap_err();
        assert!(err(&["D", "C", "--coop-host", "--coop-join", "h"]).contains("different roles"));
        assert!(err(&["D", "C", "--coop-net", "perfect"]).contains("need --coop-host"));
        assert!(err(&["D", "C", "--coop-host", "--coop-net", "weird"]).contains("--coop-net"));
        assert!(
            err(&["D", "--coop-join", "h", "--new-game", "--load-fos", "a.fos"])
                .contains("not both")
        );
        assert!(err(&["D", "C", "--coop-host", "0"]).contains("port"));
        assert!(err(&["D", "C", "--coop-quit-after", "soon", "--coop-host"]).contains("seconds"));
        assert!(err(&["D", "C", "--coop-join"]).contains("needs a value"));
    }

    #[test]
    fn help_overlay_flag_is_parsed() {
        let a = parse(&strings(&["Data", "Cell", "--help-overlay"]))
            .unwrap()
            .unwrap();
        assert!(a.help_overlay);
        assert!(
            !parse(&strings(&["Data", "Cell"]))
                .unwrap()
                .unwrap()
                .help_overlay
        );
    }

    #[test]
    fn parses_the_steam_and_token_options() {
        let h = parse(&strings(&["Data", "Cell", "--coop-host", "--coop-steam"]))
            .unwrap()
            .unwrap()
            .coop;
        assert!(h.steam && !h.steam_friend && h.host == Some(COOP_DEFAULT_PORT) && !h.is_guest());
        // a code for a Steam lobby: --coop-steam is optional (the code names it)
        let g = parse(&strings(&[
            "Data",
            "--coop-join",
            "NVRS-ABCDE",
            "--coop-steam",
        ]))
        .unwrap()
        .unwrap();
        assert!(g.coop.steam && g.coop.is_guest());
        assert_eq!(g.cell, COOP_PLACE);
        // a friend needs no code and no cell
        let f = parse(&strings(&["Data", "--coop-steam-friend"]))
            .unwrap()
            .unwrap();
        assert!(f.coop.steam_friend && f.coop.steam && f.coop.active() && f.coop.is_guest());
        assert!(f.coop.join.is_none());
        assert_eq!(f.cell, COOP_PLACE);
        // test token
        let t = parse(&strings(&[
            "Data",
            "Cell",
            "--coop-join",
            "127.0.0.1:9",
            "--coop-test-token",
        ]))
        .unwrap()
        .unwrap()
        .coop;
        assert!(t.test_token && !t.steam);
        // explicit host token
        let k = parse(&strings(&[
            "Data",
            "Cell",
            "--coop-host",
            "--coop-token",
            "000102030405060708090a0b0c0d0e0f",
        ]))
        .unwrap()
        .unwrap()
        .coop;
        assert_eq!(
            k.token,
            Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        );
    }

    #[test]
    fn explains_steam_and_token_mistakes() {
        let err = |a: &[&str]| parse(&strings(a)).unwrap_err();
        assert!(err(&["D", "C", "--coop-steam"]).contains("need --coop-host"));
        assert!(err(&["D", "C", "--coop-test-token"]).contains("need --coop-host"));
        assert!(err(&["D", "--coop-steam-friend", "--coop-join", "NVRS-AB"]).contains("by itself"));
        assert!(err(&["D", "C", "--coop-host", "--coop-steam-friend"]).contains("different roles"));
        assert!(err(&["D", "--coop-join", "h:1", "--coop-token", "00"]).contains("32 hex"));
        assert!(err(&[
            "D",
            "--coop-join",
            "h:1",
            "--coop-token",
            "000102030405060708090a0b0c0d0e0f"
        ])
        .contains("with --coop-host"));
        assert!(
            err(&["D", "C", "--coop-host", "--coop-steam", "--coop-test-token"])
                .contains("UDP test runs")
        );
        assert!(err(&[
            "D",
            "C",
            "--coop-host",
            "--coop-test-token",
            "--coop-token",
            "000102030405060708090a0b0c0d0e0f"
        ])
        .contains("give one"));
        assert!(err(&[
            "D",
            "--coop-steam-friend",
            "--new-game",
            "--coop-load",
            "s.txt"
        ])
        .contains("not both"));
        assert_eq!(parse_token_hex("zz"), None);
        assert_eq!(parse_token_hex(&"\u{e9}".repeat(16)), None);
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
