//! The game's scripts in the viewer (`world::scripting`): quest scripts run
//! as time passes (not while talking, when the game is in a menu), and
//! what scripts make happen is shown: messages, journal entries and
//! objectives as notices at the top right; people starting to talk to the
//! player; placed objects shown or hidden by `Enable` / `Disable`. Also the
//! things the player can use where they are: E takes an item, takes what's
//! in a container, or runs an object's `OnActivate`.

use bevy::prelude::*;
use esm::FormId;
use world::dialogue::PLAYER_REF;
use world::scripting::{Event, GameState, Interactive, Runner, ScriptCache};

use crate::dialogue::{Conversation, DialogueState};
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles};

/// Parsed scripts and quests, read once.
#[derive(Resource, Default)]
pub struct Scripts(pub ScriptCache);

/// The interior the player is in (a cell's form ID); outdoors it's worked
/// out from where the player stands.
#[derive(Resource, Default)]
pub struct Here(pub Option<u32>);

/// Which placed reference an entity draws (for `Enable` / `Disable`).
#[derive(Component, Clone, Copy)]
pub struct PlacedRef(pub u32);

/// Talking a script started: who speaks, the topic (`None`: their
/// greeting), and whether it's a conversation (`StartConversation`) or
/// just a line said (`SayTo`).
#[derive(Resource, Default)]
pub struct ScriptedTalk(pub Option<(FormId, Option<FormId>, bool, bool)>);

/// Notices on screen, each with the time it appeared.
#[derive(Resource, Default)]
pub struct Notices(Vec<(String, f32)>);

impl Notices {
    /// Shows a notice from `now` (seconds since the viewer started).
    pub fn push(&mut self, text: String, now: f32) {
        self.0.push((text, now));
    }
}

/// `--stage`: a quest stage to set once the place has loaded.
#[derive(Resource, Default)]
pub struct StartStage(pub Option<(String, u16)>);

/// `--run`: script lines to run once the place has loaded, as the game's
/// console runs them.
#[derive(Resource, Default)]
pub struct StartCommands(pub Vec<String>);

/// `--run-at`: for testing, script lines to run this many seconds after
/// the place has loaded (in order of time), and when it was up.
#[derive(Resource, Default)]
pub struct LaterCommands {
    pub lines: Vec<(f32, String)>,
    pub ready_at: Option<f32>,
}

impl LaterCommands {
    /// The lines due `now` (seconds since the start), taken out in order;
    /// the clock starts at the first call.
    pub fn due(&mut self, now: f32) -> Vec<String> {
        let since = now - *self.ready_at.get_or_insert(now);
        let mut due: Vec<(f32, String)> = Vec::new();
        self.lines.retain(|(at, line)| {
            if *at <= since {
                due.push((*at, line.clone()));
                false
            } else {
                true
            }
        });
        due.sort_by(|a, b| a.0.total_cmp(&b.0));
        due.into_iter().map(|(_, l)| l).collect()
    }
}

/// The objects of the attached cells (the interior, or outdoors the
/// `uGridsToLoad` grid's loaded squares), whose scripts run
/// (`world::ref_scripts`) and which the player can use.
#[derive(Resource, Default)]
pub struct CellScripts {
    pub refs: Vec<Interactive>,
    /// The game's reference-script pass: attached cells, pending events,
    /// trigger occupancy.
    scheduler: world::ref_scripts::RefScripts,
}

/// World-space bounds of rendered placed objects in the loaded cell(s).
/// Some base records have an empty OBND despite having real geometry.
#[derive(Resource, Default)]
pub struct ObjectBounds(pub std::collections::HashMap<u32, ([f32; 3], [f32; 3])>);

impl ObjectBounds {
    pub fn from_scene(bounds: &[cellview::ObjectBounds]) -> Self {
        Self(bounds.iter().map(|b| (b.reference, (b.lo, b.hi))).collect())
    }
}

/// The object in view that E would use, and the prompt for it ("Take
/// Bottle Cap", "Open Chest", "Vit-o-matic Vigor Tester"), which
/// `walk::doors` shows when no door is in view.
#[derive(Resource, Default)]
pub struct Activatable(pub Option<(FormId, String)>);

/// E was pressed on [`Activatable`]'s object.
#[derive(Resource, Default)]
pub struct ActivateRequest(pub Option<FormId>);

/// `--use`: an object to press E on once the player is in the place, for
/// testing (an editor ID or form ID).
#[derive(Resource, Default)]
pub struct StartUse(pub Option<String>);

/// Presses E on the `--use` object, as if the crosshair were on it.
pub fn start_use(
    game: Res<GameFiles>,
    mut start: ResMut<StartUse>,
    player: Res<crate::walk::Player>,
    mut request: ResMut<ActivateRequest>,
) {
    if !player.ready || start.0.is_none() {
        return;
    }
    let Some(asked) = start.0.take() else {
        return;
    };
    let order = &game.0.order;
    let form = order.form_by_editor_id(&asked).or_else(|| {
        u32::from_str_radix(asked.trim_start_matches("0x"), 16)
            .ok()
            .map(FormId)
            .filter(|f| order.get(*f).is_some())
    });
    match form {
        Some(f) => {
            println!("--use: using {asked}.");
            request.0 = Some(f);
        }
        None => println!("--use: no object called '{asked}'."),
    }
}

/// Why something locked doesn't open now.
pub enum Locked {
    /// The lockpicking menu opens on it (`lockpick`).
    Pick,
    /// What to tell the player.
    Says(String),
}

/// The player tries something locked (`world::locks`): `None` when it
/// opens (it isn't locked, or they have its key), else the lockpicking
/// menu or a refusal in the game's own words: `sImpossibleLock` (only its
/// key opens it; no sound, `005180b0`) or `sLockpickSkillTooLow` with
/// `UIPopUpMessageGeneral` (`0078db00`).
pub fn locked(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    reference: FormId,
    label: &str,
) -> Option<Locked> {
    use world::locks::Opening;
    match world::locks::try_open(order, state, reference) {
        Opening::Open => None,
        Opening::WithKey => {
            println!("Unlocked {label} with its key.");
            None
        }
        Opening::Pick(_) => Some(Locked::Pick),
        Opening::NeedsKey => Some(Locked::Says(world::lockpick::message_text(
            order,
            "sImpossibleLock",
            "This lock cannot be picked. It requires a key to open.",
        ))),
        Opening::NeedsSkill(level) => {
            if let Some(s) = order.form_by_editor_id(crate::lockpick::POPUP_SOUND) {
                state.events.push(Event::Sound(s));
            }
            Some(Locked::Says(
                world::lockpick::message_text(
                    order,
                    "sLockpickSkillTooLow",
                    "You need a lockpick skill of %d to pick this lock.",
                )
                .replace("%d", &level.to_string()),
            ))
        }
    }
}

/// The player opens a door: a door a script has "destroyed" doesn't open;
/// a locked one opens the lockpicking menu (`pick` set to it) or says why
/// not; a door with an `OnActivate` script runs it and opens only if the
/// script calls `Activate` on it (`GSDocMitchellDoorScript` notes the
/// player left, then does); any other door opens.
pub fn door_opens(
    order: &esm::LoadOrder,
    cache: &ScriptCache,
    state: &mut world::scripting::GameState,
    door: FormId,
    pick: &mut Option<FormId>,
) -> bool {
    if state.destroyed.contains(&door) {
        println!("The door won't open.");
        return false;
    }
    match locked(order, state, door, "The door") {
        None => {}
        Some(Locked::Pick) => {
            *pick = Some(door);
            return false;
        }
        Some(Locked::Says(why)) => {
            println!("{why}");
            // A key needed or too little skill: the padlock (`005180b0`).
            state.events.push(Event::Message {
                title: None,
                text: why,
                buttons: Vec::new(),
                icon: Some(world::message_icon::PADLOCK.to_string()),
            });
            return false;
        }
    }
    // co-op: a guest runs no scripts; the host runs the door's and decides
    if state.is_guest() {
        return true;
    }
    let has_script = world::scripting::script_of(order, door)
        .and_then(|s| cache.script(order, s))
        .is_some_and(|s| s.blocks.iter().any(|b| b.kind == "onactivate"));
    if !has_script {
        return true;
    }
    let before = state.events.len();
    Runner::new(order, cache, state).run_event(door, "onactivate", PLAYER_REF);
    let opened = state.events[before..]
        .iter()
        .any(|e| matches!(e, Event::Activate { what, .. } if *what == door));
    state
        .events
        .retain(|e| !matches!(e, Event::Activate { what, .. } if *what == door));
    opened
}

/// The attached cells (in the game's pass order) and how to read one's
/// objects: outdoors the `uGridsToLoad` grid around the game's grid
/// centre (`world::ref_scripts::grid_center`), those of its squares loaded
/// here; indoors the interior.
fn attached_cells(
    cell_scripts: &mut CellScripts,
    exterior: Option<&crate::exterior::Exterior>,
    interior: Option<FormId>,
    feet: [f32; 3],
) -> Vec<(FormId, Option<(i32, i32)>)> {
    use world::ref_scripts::{grid_squares, GRIDS_TO_LOAD};
    match exterior {
        Some(e) => {
            let center =
                cell_scripts
                    .scheduler
                    .exterior_center(e.grid.world.form_id, feet, GRIDS_TO_LOAD);
            // A square still loading isn't attached yet.
            let loaded = e.loaded_squares();
            grid_squares(center, GRIDS_TO_LOAD)
                .into_iter()
                .filter(|s| loaded.contains(s))
                .filter_map(|s| e.grid.cell_at(s).map(|c| (c, Some(s))))
                .collect()
        }
        None => {
            cell_scripts.scheduler.interior();
            interior.map(|c| (c, None)).into_iter().collect()
        }
    }
}

/// Makes `cells` the attached ones: newly attached cells' objects get
/// `OnLoad`, detached triggers let their occupants go
/// (`world::ref_scripts::RefScripts::attach`).
fn refresh_cell_scripts(
    order: &esm::LoadOrder,
    cache: &ScriptCache,
    state: &mut world::scripting::GameState,
    cell_scripts: &mut CellScripts,
    cells: &[FormId],
    load: impl FnMut(FormId) -> Vec<Interactive>,
) {
    let mut runner = Runner::new(order, cache, state);
    if cell_scripts.scheduler.attach(&mut runner, cells, load) {
        cell_scripts.refs = cell_scripts.scheduler.refs().to_vec();
    }
}

/// The references made while playing that E can use (dropped items, ash
/// piles) and the placed mines, in the player's place.
fn made_and_mines(order: &esm::LoadOrder, state: &GameState) -> Vec<world::scripting::Interactive> {
    let Some((space, ..)) = state.place(order, PLAYER_REF) else {
        return Vec::new();
    };
    let mut out = world::more_functions::placed::dropped_items(order, state, space);
    out.extend(world::mines::interactive_in(order, state, space));
    out
}

/// Whether an object's script has an `OnActivate` block.
fn has_on_activate(order: &esm::LoadOrder, cache: &ScriptCache, r: &Interactive) -> bool {
    r.script
        .and_then(|s| cache.script(order, s))
        .is_some_and(|s| s.blocks.iter().any(|b| b.kind == "onactivate"))
}

/// What using an object did.
enum Used {
    /// Something to tell the player, with its picture (`None` the
    /// neutral Vault Boy).
    Notice(String, Option<&'static str>),
    /// A container to open on screen: its reference and name.
    Container(FormId, String),
    /// A terminal's screen: its record and the placed terminal.
    Terminal(FormId, FormId),
    /// The lockpicking menu, on a locked container.
    Lockpick(FormId),
    /// A bed's sleep menu (the game's checks passed).
    Sleep,
    /// Furniture to sit in or get up from (`sitting::player_furniture`).
    Furniture(FormId),
    /// An item taken that the game doesn't announce (`004ce380`'s types).
    Taken,
    /// A sound to play (a mine's disable sound).
    Sound(FormId),
}

/// E on an object: its `OnActivate` script runs, if it has one, and its
/// usual action happens (an item is taken, a container opens) unless the
/// script has one and doesn't call `Activate`, as for doors.
fn use_object(
    order: &esm::LoadOrder,
    cache: &ScriptCache,
    state: &mut GameState,
    r: &Interactive,
) -> Option<Used> {
    if has_on_activate(order, cache, r) {
        let before = state.events.len();
        Runner::new(order, cache, state).run_event(r.reference, "onactivate", PLAYER_REF);
        let went_on = state.events[before..]
            .iter()
            .any(|e| matches!(e, Event::Activate { what, .. } if *what == r.reference));
        state
            .events
            .retain(|e| !matches!(e, Event::Activate { what, .. } if *what == r.reference));
        if !went_on {
            return None;
        }
    }
    let name = |id: FormId| {
        order
            .get(id)
            .and_then(|rr| rr.record().ok())
            .and_then(|rec| rec.full_name())
            .unwrap_or_else(|| id.to_string())
    };
    // A placed mine: disarmed, then taken (`world::mines::use_placed`).
    if r.kind == esm::FourCC::new(b"PROJ") {
        return match world::mines::use_placed(order, state, r.reference)? {
            world::mines::MineUsed::Disarmed(sound) => {
                println!("Disarmed {}.", r.reference);
                sound.map(Used::Sound)
            }
            world::mines::MineUsed::Taken(_, Some(message)) => Some(Used::Notice(
                message,
                // The "added" notice's gift box (`004821a0`).
                Some(world::message_icon::GIFT_BOX),
            )),
            world::mines::MineUsed::Taken(_, None) => Some(Used::Taken),
        };
    }
    // An ash or goo pile passes the activation on to its corpse
    // (`TESObjectREFR::Activate`, `00573170`): the corpse is searched.
    let corpse = world::activation::stands_for(order, state, r.reference);
    if corpse != r.reference {
        let label = world::script_functions::full_name(order, state, corpse)
            .unwrap_or_else(|| corpse.to_string());
        return Some(Used::Container(corpse, label));
    }
    let counted = |item: FormId, n: i32| {
        if n > 1 {
            format!("{} ({n})", name(item))
        } else {
            name(item)
        }
    };
    if r.is_item() {
        // Someone else's: stealing (`world::crime`).
        let owner = world::crime::owner_of(order, state, r.reference);
        // Its script's `OnAdd` blocks run with the scripted items
        // (`Runner::run_item_scripts`; Caravan cards join the player's
        // cards and leave the inventory).
        // co-op: first claim wins; a guest asks the host, which gives it to them
        if state.coop_route(world::coop::interact::pickup_act(
            r.reference,
            r.base,
            r.count.max(1) as u32,
        )) == world::coop::interact::Route::Remote
        {
            return Some(Used::Taken);
        }
        state.pick_up(order, r.reference, r.base, r.count);
        if let Some(owner) = owner.filter(|_| !world::crime::may_take(order, state, owner)) {
            // A note (`BGSNote::Activate`, `005e9360`): only the karma.
            if r.kind.as_bytes() == b"NOTE" {
                world::crime::stealing_karma(order, state, owner);
            } else if world::crime::steal(order, state, r.reference, owner) {
                println!("Seen stealing {}.", counted(r.base, r.count));
            }
        }
        // "<name> added" / "<n> <name>(s) added" (`004ce380`).
        return Some(
            match world::activation::pickup_message(order, r.base, r.count) {
                Some(message) => Used::Notice(message, None),
                None => Used::Taken,
            },
        );
    }
    if r.is_container() {
        let label = r.name.clone().unwrap_or_else(|| "Container".into());
        return Some(match locked(order, state, r.reference, &label) {
            None => Used::Container(r.reference, label),
            Some(Locked::Pick) => Used::Lockpick(r.reference),
            // A key needed or too little skill: the padlock (`00516dc0`,
            // `0078db00`).
            Some(Locked::Says(why)) => Used::Notice(why, Some(world::message_icon::PADLOCK)),
        });
    }
    // Any furniture while in furniture gets the player up
    // (`TESFurniture::Activate`, `world::furniture::activate`).
    if r.is_furniture() && state.furniture.contains_key(&PLAYER_REF) {
        return Some(Used::Furniture(r.reference));
    }
    // A bed: the game's checks (`world::living::sleep::may_sleep_in`; the
    // player doesn't lie down), then its sleep menu (the game's,
    // `game_menus::sleepwait`).
    if r.is_furniture() && world::living::sleep::is_bed(order, r.reference) {
        return match world::living::sleep::may_sleep_in(order, state, r.reference) {
            Ok(()) => Some(Used::Sleep),
            // The bed's refusals: the sad Vault Boy (`005095b0`).
            Err(why) => Some(Used::Notice(why, Some(world::message_icon::SAD))),
        };
    }
    if r.is_furniture() {
        // Sitting: the sit procedure (`sitting::player_furniture`).
        return Some(Used::Furniture(r.reference));
    }
    if r.is_terminal() {
        return Some(Used::Terminal(r.base, r.reference));
    }
    None
}

/// Each frame in the game (not in a menu): the attached objects'
/// scripts, with the events flagged since their last run (`OnLoad`, and
/// triggers that the player's body, or a person, enters, stays in or
/// leaves; `people`: reference and feet). See `world::ref_scripts`.
fn run_cell_scripts(
    order: &esm::LoadOrder,
    cache: &ScriptCache,
    state: &mut world::scripting::GameState,
    cell_scripts: &mut CellScripts,
    people: &[(FormId, [f32; 3])],
    seconds: f32,
    sight: Option<&dyn world::sight::Sight>,
) {
    let mut runner = Runner::new(order, cache, state);
    runner.sight = sight;
    runner.seconds_passed = seconds;
    cell_scripts.scheduler.frame(&mut runner, people);
}

/// What the player looks at within reach that E would use: an item, a
/// container, or an object whose script has an `OnActivate` block (people
/// and doors have their own handling), with the prompt for it.
fn object_in_view(
    cache: &ScriptCache,
    order: &esm::LoadOrder,
    state: &GameState,
    refs: &[Interactive],
    target: Option<FormId>,
) -> Option<(FormId, String)> {
    // The reference the crosshair is on within reach (`crosshair`, the
    // game's view caster, which also leaves out what a wall hides).
    let target = target?;
    let r = refs.iter().find(|r| {
        r.reference == target
            && r.trigger.is_none()
            && ![*b"NPC_", *b"CREA", *b"DOOR"].contains(r.kind.as_bytes())
    })?;
    // An ash or goo pile standing for a corpse is used to search it; a
    // placed mine to disarm or take it.
    let pile = world::activation::stands_for(order, state, r.reference) != r.reference;
    let mine = r.kind == esm::FourCC::new(b"PROJ");
    let usable = r.is_item()
        || r.is_container()
        || r.is_furniture()
        || pile
        || mine
        || has_on_activate(order, cache, r);
    if !usable || !world::enabled_now(order, r.reference, &state.disabled) {
        return None;
    }
    // A reference a script has "destroyed" (`SetDestroyed 1`, form flag
    // 0x800000, `00477ba0`) under the crosshair has no prompt (the HUD's
    // crosshair update, `00775a00`, clears it for anything but actors) and
    // doesn't activate (`005180b0` returns at once): the VCG02 bottles
    // after their `OnLoad`.
    if state.destroyed.contains(&r.reference) {
        return None;
    }
    let name = r.name.clone().unwrap_or_else(|| r.reference.to_string());
    // Someone else's things: the game's "Steal" (`sSteal`, `00579690`).
    let owned = (r.is_item() || r.is_container())
        && !world::crime::may_take(
            order,
            state,
            world::crime::owner_of(order, state, r.reference),
        );
    let steal =
        || world::scripting::game_setting_text(order, "sSteal").unwrap_or_else(|| "Steal".into());
    let pile = world::activation::stands_for(order, state, r.reference) != r.reference;
    let mine = r.kind == esm::FourCC::new(b"PROJ");
    let prompt = if pile || mine {
        // The pile's own words (an activator with a name: "Activate",
        // `world::activation::info`).
        let info = world::activation::info(order, state, r.reference);
        match info.and_then(|i| i.action) {
            Some(action) => format!("{action} {name}"),
            None => name,
        }
    } else if r.is_item() {
        let verb = if owned { steal() } else { "Take".into() };
        if r.count > 1 {
            format!("{verb} {name} ({})", r.count)
        } else {
            format!("{verb} {name}")
        }
    } else if r.is_container() {
        let verb = if owned { steal() } else { "Open".into() };
        format!("{verb} {name}")
    } else if r.is_furniture() {
        if state.furniture.get(&PLAYER_REF) == Some(&r.reference) {
            return None;
        }
        format!("Sit ({name})")
    } else if r.is_terminal() {
        format!("Access {name}")
    } else {
        name
    };
    Some((r.reference, prompt))
}

/// A placed reference's base record (the reference itself if it has none).
fn base_or(order: &esm::LoadOrder, r: FormId) -> FormId {
    world::scripting::base_of(order, r).unwrap_or(r)
}

/// `MoveTo` on the player: go to where `to` stands now (the place and
/// position the state has for it: someone who walked out of their editor
/// cell is found where they walked to), loading its place.
fn move_player(
    game: &cellview::Game,
    state: &world::scripting::GameState,
    to: FormId,
    at: Option<[f32; 3]>,
    pending: &mut crate::PendingScene,
    pending_exterior: &mut crate::exterior::PendingExterior,
) -> Result<(), String> {
    let order = &game.order;
    let (space, _, spot, heading) = state
        .place(order, to)
        .ok_or_else(|| format!("{to} isn't a placed object"))?;
    let position = at.unwrap_or(spot);
    let interior = order
        .get(space)
        .is_some_and(|r| r.entry.header.kind == esm::sig::CELL);
    if !interior {
        let grid = world::WorldGrid::load(order, space).map_err(|e| e.to_string())?;
        pending_exterior.0 = Some(crate::exterior::ExteriorStart {
            grid,
            feet: [position[0], position[1]],
            height: Some(position[2]),
            heading,
        });
        return Ok(());
    }
    let mut scene = game
        .load_cell_now(space, &state.disabled)
        .map_err(|e| e.0)?;
    let [x, y, z] = position;
    scene.start = cellview::Start {
        eye: [x, y, z + cellview::EYE_HEIGHT],
        heading,
        via: "a script (MoveTo)",
    };
    pending.0 = Some(scene);
    Ok(())
}

/// The game writes keys in messages as `&sUActnForward;` (or
/// `&-sUActnForward;`), filled in with the key bound to that action; here
/// they're the viewer's keys.
fn fill_keys(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 1..];
        let Some(end) = tail.find(';') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let action = tail[..end].trim_start_matches('-');
        let key = match action.strip_prefix("sUActn") {
            Some(a) => match a.to_ascii_lowercase().as_str() {
                "forward" => "W".to_string(),
                "back" => "S".to_string(),
                "sldleft" => "A".to_string(),
                "sldright" => "D".to_string(),
                "jump" => "Space".to_string(),
                "activate" => "E".to_string(),
                "sneak" => "Ctrl".to_string(),
                "togglerun" | "run" => "Shift".to_string(),
                "attack" => "the left mouse button".to_string(),
                "reload" => "R".to_string(),
                "menumode" | "pipboy" => "Tab".to_string(),
                "quicksave" => "F5".to_string(),
                "quickload" => "F9".to_string(),
                "vats" => "V".to_string(),
                other => format!("[{other}]"),
            },
            None => format!("&{};", &tail[..end]),
        };
        out.push_str(&key);
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

/// A record's name (its `FULL`), else its editor ID.
fn record_name(order: &esm::LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name().or_else(|| r.editor_id()))
        .unwrap_or_else(|| id.to_string())
}

/// Where F5 saves and F9 loads (in the folder the viewer runs in).
pub(crate) const QUICKSAVE: &str = "nv-rs-quicksave.txt";

/// A load of the most recent save asked for by the game itself (the
/// player's death, `world::player_death`), done as F9 does it.
#[derive(Resource, Default)]
pub struct LoadRequest(pub bool);

/// A loaded world cannot retain trigger occupancy or a seat from the world
/// it replaces, even when the saved player is in the same cell.
fn restore_script_state(state: &mut GameState, cells: &mut CellScripts, mut loaded: GameState) {
    // A co-op host keeps its session (authority, policy, the guests' parked characters).
    if state.coop.authority != world::coop::Authority::Single {
        world::coop::party_char::carry_coop(state, &mut loaded, None);
    }
    *state = loaded;
    *cells = CellScripts::default();
}

/// Prepare the destination before changing the running world. A missing cell
/// or worldspace must not leave old geometry with a different quest state.
#[allow(clippy::too_many_arguments)]
fn load_saved_world(
    game: &cellview::Game,
    state: &mut GameState,
    cells: &mut CellScripts,
    idle: &mut crate::player_idle::PlayerIdle,
    pending: &mut crate::PendingScene,
    exterior: &mut crate::exterior::PendingExterior,
    seats: &mut crate::sitting::Seats,
    start_pitch: &mut crate::StartPitch,
    text: &str,
) -> Result<bool, String> {
    let (loaded, place) = world::save::load(text)?;
    if loaded.saved_camera.is_some() && place.is_none() {
        return Err("saved camera has no player location".into());
    }
    let restored_idle =
        crate::player_idle::PlayerIdle::restore(game, seats, loaded.saved_camera.as_ref())?;
    let pitch = loaded
        .saved_camera
        .as_ref()
        .map_or(0.0, |camera| camera.pitch);
    let mut scene = None;
    let mut outside = None;
    if let Some(place) = place {
        match place.world {
            Some(world) => {
                let grid = world::WorldGrid::load(&game.order, world).map_err(|e| e.to_string())?;
                outside = Some(crate::exterior::ExteriorStart {
                    grid,
                    feet: [place.position[0], place.position[1]],
                    height: Some(place.position[2]),
                    heading: place.heading,
                });
            }
            None => {
                let mut loaded_scene = game
                    .load_cell_now(place.cell, &loaded.disabled)
                    .map_err(|e| e.0)?;
                let [x, y, z] = place.position;
                loaded_scene.start = cellview::Start {
                    eye: [x, y, z + cellview::EYE_HEIGHT],
                    heading: place.heading,
                    via: "a saved game",
                };
                scene = Some(loaded_scene);
            }
        }
    }
    restore_script_state(state, cells, loaded);
    *idle = restored_idle;
    if place.is_some() {
        start_pitch.0 = pitch;
    }
    pending.0 = scene;
    exterior.0 = outside;
    Ok(place.is_some())
}

/// Serialize current presentation without changing the live state's metadata.
fn save_camera(
    path: &str,
    state: &mut GameState,
    place: Option<world::save::PlayerPlace>,
    camera: world::save::camera::Camera,
) -> Result<(), String> {
    let previous = std::mem::replace(&mut state.saved_camera, place.map(|_| camera));
    let result = world::save::save_to_path(std::path::Path::new(path), state, place)
        .map_err(|e| e.to_string());
    state.saved_camera = previous;
    result
}

/// F5 saves the game; F9 prepares and restores its state and destination.
#[allow(clippy::too_many_arguments)]
pub fn save_and_load(
    mut commands: Commands,
    // The Quick Save and Quick Load controls are the player's
    // (`PlayerCharacter::Update`, `0094285d`, `009403aa`, `00942878`): the
    // controls' buffer as the player's step read it (`crate::poll`).
    player_input: Res<crate::poll::PlayerInput>,
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    mut notices: ResMut<Notices>,
    mut pending: ResMut<crate::PendingScene>,
    mut pending_exterior: ResMut<crate::exterior::PendingExterior>,
    cameras: Query<(&Transform, &FlyCamera)>,
    player: Res<crate::walk::Player>,
    mut player_idle: ResMut<crate::player_idle::PlayerIdle>,
    mut cell_scripts: ResMut<CellScripts>,
    mut seats: ResMut<crate::sitting::Seats>,
    (mut start_pitch, mut request, mut menu_saves, mut party): (
        ResMut<crate::StartPitch>,
        ResMut<LoadRequest>,
        ResMut<crate::game_menus::start::SaveFiles>,
        Option<ResMut<crate::coop_party::PartyBridge>>,
    ),
) {
    let now = time.elapsed_secs();
    let requested = std::mem::take(&mut request.0);
    let mut say = |text: String| {
        println!("{text}");
        notices.0.push((text, now));
    };
    // The start menu's Save and Load pages name their file
    // (`game_menus::start`); F5 and F9 the quick save.
    use crate::game_menus::start::SaveFile;
    let asked = menu_saves.0.take();
    let save_to = match &asked {
        Some(SaveFile::Save(p)) => Some(p.to_string_lossy().to_string()),
        _ if player_input.keys.just_pressed(KeyCode::F5) => Some(QUICKSAVE.to_string()),
        _ => None,
    };
    let load_from = match &asked {
        Some(SaveFile::Load(p)) => Some(p.to_string_lossy().to_string()),
        _ if player_input.keys.just_pressed(KeyCode::F9) || requested => {
            Some(QUICKSAVE.to_string())
        }
        _ => None,
    };
    // A co-op guest saves and loads only its own character (`coop_party`), never the world.
    let guest = party.as_ref().is_some_and(|p| p.guest);
    let (save_to, load_from) = if guest {
        if let Some(p) = party.as_mut() {
            p.guest_save |= save_to.is_some();
            p.guest_load |= load_from.is_some();
        }
        (None, None)
    } else {
        (save_to, load_from)
    };
    if let Some(save_to) = save_to {
        let place = cameras.single().ok().and_then(|(t, input)| {
            let eye = game_point(t.translation);
            let f = t.forward().as_vec3();
            Some(world::save::PlayerPlace {
                cell: state.0.player_cell?,
                world: state.0.player_world,
                position: player.position_for_view(eye),
                heading: if player.walking {
                    -input.yaw
                } else {
                    f.x.atan2(-f.z)
                },
            })
        });
        let result = player_idle
            .snapshot(
                &seats,
                cameras.single().map_or(0.0, |(_, input)| input.pitch),
            )
            .and_then(|camera| save_camera(&save_to, &mut state.0, place, camera));
        match result {
            Ok(()) => {
                say(format!("Saved ({save_to})."));
                if let Some(p) = party.as_mut() {
                    p.host_saved.push(save_to.clone().into());
                }
            }
            Err(e) => say(format!("Couldn't save: {e}")),
        }
    }
    if let Some(load_from) = load_from {
        let result = std::fs::read_to_string(&load_from)
            .map_err(|e| e.to_string())
            .and_then(|text| {
                load_saved_world(
                    &game.0,
                    &mut state.0,
                    &mut cell_scripts,
                    &mut player_idle,
                    &mut pending,
                    &mut pending_exterior,
                    &mut seats,
                    &mut start_pitch,
                    &text,
                )
            });
        queue_reload_cleanup(&mut commands, &result);
        if result.is_ok() {
            if let Some(p) = party.as_mut() {
                p.host_loaded.push(load_from.clone().into());
            }
        }
        match result {
            Ok(true) => say("Loaded.".into()),
            Ok(false) => say("Loaded (the save doesn't say where the player was).".into()),
            Err(e) => say(format!("Couldn't load {load_from}: {e}")),
        }
    }
}

/// Queue transient dialogue cleanup only after the saved world has passed
/// every preflight step and its state has been committed.
fn queue_reload_cleanup(commands: &mut Commands, result: &Result<bool, String>) {
    if result.is_ok() {
        commands.queue(discard_reloaded_dialogue);
    }
}

fn discard_reloaded_dialogue(world: &mut World) {
    // A game loaded arms the casinos' anti-cheat lock if a casino menu
    // closed before (`00956f70` → `00969ac0`).
    if let Some(mut lock) = world.get_resource_mut::<crate::game_menus::casino::CasinoLock>() {
        lock.0.loaded();
    }
    world.resource_mut::<Conversation>().discard();
    world.resource_mut::<ScriptedTalk>().0 = None;
    world
        .resource_mut::<crate::chatter::Lines>()
        .discard_pending();
    world.resource_mut::<DialogueState>().0.speaking.clear();
    if let Some(mut target) = world.get_resource_mut::<crate::dialogue::TalkTarget>() {
        target.0 = None;
    }
    if let Some(mut keys) = world.get_resource_mut::<ButtonInput<KeyCode>>() {
        for key in [
            KeyCode::Escape,
            KeyCode::Tab,
            KeyCode::Space,
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ] {
            keys.clear_just_pressed(key);
        }
    }
    let remaining_game_menu = world
        .get_resource_mut::<crate::game_menus::GameMenus>()
        .and_then(|mut menus| {
            let dialog_removed = menus
                .screen()
                .is_some_and(crate::game_menus::dialog::discard);
            dialog_removed.then(|| menus.screen().is_some_and(|screen| !screen.open.is_empty()))
        });
    if let Some(game_open) = remaining_game_menu {
        if let Some(mut menu_state) = world.get_resource_mut::<crate::menus::Menus>() {
            menu_state.game_open = game_open;
        }
    }
    let menu_open = world
        .get_resource::<crate::menus::Menus>()
        .is_some_and(crate::menus::Menus::is_open);
    if let Some(mut player) = world.get_resource_mut::<crate::walk::Player>() {
        player.ready = !menu_open;
    }
    {
        let mut panel = world
            .query_filtered::<(&mut Text, &mut Visibility), With<crate::dialogue::DialogueText>>();
        for (mut text, mut visibility) in panel.iter_mut(world) {
            text.0.clear();
            *visibility = Visibility::Hidden;
        }
    }
    {
        let mut prompt = world.query_filtered::<&mut Text, With<crate::walk::Prompt>>();
        for mut text in prompt.iter_mut(world) {
            text.0.clear();
        }
    }
    crate::faces::discard_voices(world);
}

/// How long a notice stays up, in seconds.
const NOTICE_SECONDS: f32 = 6.0;

#[derive(Component)]
pub struct NoticeText;

pub fn setup_notices(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.85, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(8.0),
            max_width: Val::Percent(40.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        Visibility::Hidden,
        NoticeText,
    ));
}

/// The notice panel's text and whether it shows.
type NoticePanel = (&'static mut Text, &'static mut Visibility);

/// What's around the player that they can use, and who else is there.
#[derive(bevy::ecs::system::SystemParam)]
pub struct HereNow<'w> {
    cell_scripts: ResMut<'w, CellScripts>,
    activatable: ResMut<'w, Activatable>,
    activate_request: ResMut<'w, ActivateRequest>,
    collision: Res<'w, crate::walk::CellCollision>,
    talkers: Res<'w, crate::dialogue::Talkers>,
    sound_requests: ResMut<'w, crate::sounds::SoundRequests>,
    music_requests: ResMut<'w, crate::music::MusicRequests>,
    moved: ResMut<'w, crate::ai::Moved>,
    starts: ResMut<'w, crate::ai::Starts>,
    menus: ResMut<'w, crate::menus::Menus>,
    hud_messages: ResMut<'w, crate::hud::HudMessages>,
    groups: ResMut<'w, crate::PlayingGroups>,
    lockpicking: ResMut<'w, crate::lockpick::Lockpicking>,
    /// The game's clock speed (`SetGlobalTimeMultiplier`).
    virtual_time: ResMut<'w, Time<Virtual>>,
    /// The game's own menus open (their classes), for `MenuMode` blocks.
    menu_draw: Res<'w, crate::game_menus::MenuDraw>,
    /// Movies scripts asked for (`PlayBink`).
    movies: ResMut<'w, crate::movie::Movies>,
    player_idle: ResMut<'w, crate::player_idle::PlayerIdle>,
    seats: Res<'w, crate::sitting::Seats>,
    object_bounds: Option<Res<'w, ObjectBounds>>,
    /// What the crosshair is on (`crosshair`, the game's view caster).
    crosshair: Res<'w, crate::crosshair::Crosshair>,
    player_seat: ResMut<'w, crate::sitting::PlayerSeat>,
    later: ResMut<'w, LaterCommands>,
    /// The casinos' anti-cheat lock, on the real clock (`game_menus::casino`).
    casino_lock: ResMut<'w, crate::game_menus::casino::CasinoLock>,
    real_time: Res<'w, Time<bevy::time::Real>>,
}

/// The start-up state and requests `run_scripts` works with.
type Starting<'w> = (
    ResMut<'w, StartStage>,
    Res<'w, crate::walk::Player>,
    ResMut<'w, crate::PendingScene>,
    ResMut<'w, crate::exterior::PendingExterior>,
    (
        ResMut<'w, crate::combat::ObjectShots>,
        ResMut<'w, crate::swaps::TextureSwaps>,
        // The game's own code (`host`): its interpreter runs the quests it
        // takes (`host::quest_vm`). Main thread only.
        Option<NonSendMut<'w, host::Host>>,
    ),
);

/// Runs the scripts for this frame and carries out what they asked for.
#[allow(clippy::too_many_arguments)]
pub fn run_scripts(
    time: Res<Time>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut state: ResMut<DialogueState>,
    here: Res<Here>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    conversation: Res<Conversation>,
    mut talk: ResMut<ScriptedTalk>,
    mut notices: ResMut<Notices>,
    (
        mut start_stage,
        player,
        mut pending,
        mut pending_exterior,
        (mut object_shots, mut texture_swaps, mut game_code),
    ): Starting,
    mut start_commands: ResMut<StartCommands>,
    here_now: HereNow,
    cameras: Query<(&Transform, &FlyCamera, Option<&Projection>)>,
    mut placed: Query<(&PlacedRef, &mut Visibility)>,
    mut text: Query<NoticePanel, (With<NoticeText>, Without<PlacedRef>)>,
    // T, the Rest control: the player's controls' buffer (`crate::poll`; where
    // the exe reads the Rest control isn't traced), so a menu that took the
    // key keeps it, as when the menus ran ahead of the frame.
    player_input: Res<crate::poll::PlayerInput>,
) {
    let HereNow {
        mut cell_scripts,
        mut activatable,
        mut activate_request,
        collision,
        talkers,
        mut sound_requests,
        mut music_requests,
        mut moved,
        mut starts,
        menus: mut waiting,
        mut hud_messages,
        mut groups,
        mut lockpicking,
        mut virtual_time,
        menu_draw,
        mut movies,
        mut player_idle,
        seats,
        object_bounds,
        crosshair,
        mut player_seat,
        mut later,
        mut casino_lock,
        real_time,
    } = here_now;
    let order = &game.0.order;
    let now = time.elapsed_secs();
    // What the game announces goes to the HUD's message corner while it's
    // drawn, else to the notice panel.
    // The HUD's quest text takes quest updates and objective lines while
    // the HUD is drawn (`ui::quest_text`); otherwise they're notices.
    let hud_on = hud_messages.on;
    let mut quest_texts = Vec::new();
    let mut objective_lines = Vec::new();
    let mut announce = |n: crate::hud::HudMessage, notices: &mut Notices| {
        println!("{}", n.text);
        if hud_messages.on {
            hud_messages.queue.push(n);
        } else {
            notices.0.push((n.text, now));
        }
    };
    // Tab, I and J open the Pip-Boy (`pipboy`).
    // T (the Rest control): the wait menu, unless the game refuses first
    // (`world::living::sleep`).
    if player_input.keys.just_pressed(KeyCode::KeyT)
        && conversation.0.is_none()
        && !waiting.is_open()
    {
        match world::living::sleep::may_wait(order, &state.0) {
            Ok(()) => waiting.push(crate::menus::Menu::SleepWait { sleep: false }),
            Err(why) => {
                println!("{why}");
                notices.0.push((why, now));
            }
        }
    }
    // `--stage`, once the place is up.
    if player.ready {
        if let Some((quest, n)) = start_stage.0.take() {
            match order.form_by_editor_id(&quest) {
                Some(q) => {
                    // The new game's start (`VCG00` 0) is a leftover intro
                    // script (its source says "DEMO ONLY"): its `Set
                    // GameHour to 23` would start Doc Mitchell's house at
                    // night. The opening is meant in daylight (the
                    // maintainer's decision, 2026-10-08, from the original
                    // game as played), so the clock keeps the hour it had
                    // (`GameHour`'s own value, 12) through that stage.
                    let keep_hour = (quest.eq_ignore_ascii_case("VCG00") && n == 0)
                        .then(|| order.form_by_editor_id("GameHour"))
                        .flatten()
                        .and_then(|g| state.0.globals.get(&g).map(|&h| (g, h)));
                    let set = Runner::new(order, &scripts.0, &mut state.0).set_stage(q, n);
                    if let Some((g, h)) = keep_hour {
                        state.0.globals.insert(g, h);
                    }
                    println!(
                        "SetStage {quest} {n}: {}",
                        if set {
                            "set"
                        } else {
                            "not set (no such stage, or done)"
                        }
                    );
                }
                None => println!("--stage: there's no quest called {quest}"),
            }
        }
    }
    // Where the player is, for GetInCell and GetInWorldspace.
    let state = &mut state.0;
    let (eye, dir, heading) = cameras
        .single()
        .map(|(t, input, _)| {
            let f = t.forward().as_vec3();
            (
                game_point(t.translation),
                [f.x, -f.z, f.y],
                if player.walking {
                    -input.yaw
                } else {
                    f.x.atan2(-f.z)
                },
            )
        })
        .unwrap_or_default();
    // The camera's up and view angle, for `GetLineOfSight`.
    let (up, tan_half_fov, aspect) = cameras
        .single()
        .map(|(t, _, projection)| {
            let u = t.up().as_vec3();
            let (fov, aspect) = match projection {
                Some(Projection::Perspective(p)) => (p.fov, p.aspect_ratio),
                _ => (
                    cellview::vertical_fov(cellview::GAME_FOV_DEGREES),
                    16.0 / 9.0,
                ),
            };
            ([u.x, -u.z, u.y], (fov * 0.5).tan(), aspect)
        })
        .unwrap_or(([0.0, 0.0, 1.0], 1.0, 1.0));
    // Animation and V.A.T.S. can move the view independently of the body.
    // Quest positions, triggers and saves must follow the collision capsule.
    let feet = player.position_for_view(eye);
    state.player_position = Some(feet);
    // What has 3D now (`PlayMagicShaderVisuals` needs it): the rendered
    // placed objects, the people about and the player.
    let mut loaded: std::collections::HashSet<FormId> = object_bounds
        .as_deref()
        .map(|b| b.0.keys().map(|&r| FormId(r)).collect())
        .unwrap_or_default();
    loaded.insert(PLAYER_REF);
    loaded.extend(talkers.0.iter().map(|t| t.reference));
    world::more_functions::report_loaded(state, loaded);
    state.player_heading = heading;
    match &exterior {
        Some(e) => {
            state.player_world = Some(e.grid.world.form_id);
            state.player_cell = e.grid.cells.get(&world::square_of(feet)).copied();
        }
        None => {
            state.player_world = None;
            state.player_cell = here.0.map(FormId);
        }
    }
    // `--run`: each line as the console would run it, once the place is
    // up and the player is in it; the scripted items' runs after each, so
    // the next sees what they did (an item's `OnAdd`).
    if player.ready {
        for line in std::mem::take(&mut start_commands.0) {
            let mut runner = Runner::new(order, &scripts.0, state);
            let flow = runner.run_source(&line, None, None);
            runner.run_item_scripts();
            println!("{line}: {flow:?}");
        }
        // `--run-at`: later, in game mode.
        if conversation.0.as_ref().is_none_or(|t| t.is_line_only()) && !waiting.is_open() {
            for line in later.due(now) {
                let flow = Runner::new(order, &scripts.0, state).run_source(&line, None, None);
                println!("{now:.1} s: {line}: {flow:?}");
            }
        }
    }
    let attached = attached_cells(
        &mut cell_scripts,
        exterior.as_deref(),
        here.0.map(FormId),
        feet,
    );
    let cells: Vec<FormId> = attached.iter().map(|(c, _)| *c).collect();
    refresh_cell_scripts(
        order,
        &scripts.0,
        state,
        &mut cell_scripts,
        &cells,
        |cell| match (
            &exterior,
            attached
                .iter()
                .find(|(c, _)| *c == cell)
                .and_then(|(_, s)| *s),
        ) {
            (Some(e), Some(square)) => {
                world::scripting::interactive_in_square(order, &e.grid, square)
            }
            _ => world::scripting::interactive_references(order, cell),
        },
    );
    // References scripts moved run in the cell they stand in now
    // (`world::ref_scripts::RefScripts::place_moved`).
    let moved_now: Vec<(FormId, Option<FormId>)> =
        world::ref_scripts::moved_references(order, state)
            .into_iter()
            .map(|r| {
                let cell = state
                    .place(order, r)
                    .and_then(|(space, _, at, _)| match &exterior {
                        Some(e) if space == e.grid.world.form_id => {
                            e.grid.cell_at(world::square_of(at))
                        }
                        Some(_) => None,
                        None => here.0.map(FormId).filter(|&c| c == space),
                    });
                (r, cell)
            })
            .collect();
    if cell_scripts
        .scheduler
        .place_moved(&mut Runner::new(order, &scripts.0, state), &moved_now)
    {
        cell_scripts.refs = cell_scripts.scheduler.refs().to_vec();
    }
    // Time stands still in the dialogue menu (not while a line is said)
    // and in the other menus. A co-op host's menu stops everyone's time for
    // one dialogue heartbeat after its last pick at most
    // (`GameState::coop_conversation_stops_time`).
    let dt = time.delta_secs();
    if conversation
        .0
        .as_ref()
        .is_none_or(|t| t.is_line_only() || !state.coop_conversation_stops_time())
        && (!waiting.is_open() || !crate::coop_live::menus_freeze_time())
    {
        let warner = state.living.trespass.as_ref().map(|w| w.warner);
        let mut people: Vec<(FormId, [f32; 3])> = vec![(PLAYER_REF, feet)];
        people.extend(talkers.0.iter().map(|t| (t.reference, t.position)));
        let sight = crate::sight::ViewerSight {
            collision: &collision.0,
            bounds: object_bounds.as_deref(),
            people: &people,
            eye,
            forward: dir,
            up,
            tan_half_fov,
            aspect,
        };
        let mut runner = Runner::new(order, &scripts.0, state).with_sight(&sight);
        if let Some(game_code) = game_code.as_deref_mut() {
            runner = runner.with_quest_vm(game_code);
        }
        runner.update(dt);
        // Someone coming to warn the trespassing player off
        // (`world::living::trespass`).
        let now_warner = state.living.trespass.as_ref().map(|w| w.warner);
        if now_warner != warner {
            match now_warner {
                Some(w) => println!(
                    "{} comes to warn you off (trespassing).",
                    record_name(order, base_or(order, w))
                ),
                None => println!("The trespass warning is over."),
            }
        }
        if player.ready {
            run_cell_scripts(
                order,
                &scripts.0,
                state,
                &mut cell_scripts,
                &people,
                dt,
                Some(&sight),
            );
        }
        // The sleep/wait menu open: the scripts' `MenuMode 1012` blocks
        // run (`PlayerBedSCRIPT` notes `IsPCSleeping` there).
        if menu_draw
            .1
            .contains(&i32::from(world::living::sleep::SLEEP_WAIT_MENU))
        {
            let refs: Vec<FormId> = cell_scripts
                .refs
                .iter()
                .filter(|r| r.script.is_some())
                .map(|r| r.reference)
                .collect();
            world::living::sleep::menu_mode(&mut Runner::new(order, &scripts.0, state), &refs);
        }
        // Sitting down or getting up, E does nothing
        // (`world::furniture::player_activation_blocked`; also while the
        // entry or exit plays).
        let refused = state.sitters.get(&PLAYER_REF).is_some_and(|s| {
            world::furniture::player_activation_blocked(s.state) || s.playing.is_some()
        });
        // Items the player dropped (made references) are taken as placed
        // items are (the hook the Pip-Boy's Drop needs; `pipboy`).
        let dropped = made_and_mines(order, state);
        if let Some(r) = activate_request.0.take().filter(|_| !refused) {
            let object = cell_scripts
                .refs
                .iter()
                .chain(&dropped)
                .find(|o| o.reference == r);
            match object.and_then(|o| use_object(order, &scripts.0, state, o)) {
                Some(Used::Notice(n, icon)) => {
                    announce(crate::hud::HudMessage::with_icon(n, icon), &mut notices)
                }
                Some(Used::Container(c, name)) => {
                    waiting.push(crate::menus::Menu::Container(c, name));
                }
                Some(Used::Terminal(t, r)) => {
                    use crate::game_menus::hacking::{use_terminal, Using};
                    match use_terminal(order, state, t, r) {
                        Using::Open(m) => {
                            world::terminal::opened(order, state, t, r);
                            waiting.push(m);
                        }
                        Using::Refused(why) => {
                            if let Some(s) = order.form_by_editor_id(crate::lockpick::POPUP_SOUND) {
                                sound_requests.0.push(s);
                            }
                            // The angry Vault Boy (`00501310`).
                            announce(
                                crate::hud::HudMessage::with_icon(
                                    why,
                                    Some(world::message_icon::ANGRY),
                                ),
                                &mut notices,
                            );
                        }
                    }
                }
                Some(Used::Lockpick(r)) => lockpicking.request = Some(r),
                Some(Used::Sleep) => {
                    waiting.push(crate::menus::Menu::SleepWait { sleep: true });
                }
                Some(Used::Furniture(f)) => player_seat.activated = Some(f),
                Some(Used::Sound(s)) => sound_requests.0.push(s),
                Some(Used::Taken) | None => {}
            }
            // A taken item's pick-up sound: its own (`YNAM`), else a
            // weapon's by its kind, else the generic one (`008adcf0(item,
            // 1, 0)`, which `004ce380` hands the "added" message).
            if let Some(o) = object.filter(|o| o.is_item()) {
                if !world::enabled_now(order, o.reference, &state.disabled) {
                    if let Some(s) = world::sound::item_sound(order, o.base, true) {
                        println!("Pick-up sound: {}", record_name(order, s));
                        sound_requests.0.push(s);
                    }
                    if dropped.iter().any(|d| d.reference == o.reference) {
                        world::more_functions::placed::taken(state, o.reference);
                    }
                }
            }
        }
    }
    let dropped = made_and_mines(order, state);
    // Scripts can turn the crosshair's roll-over text (and with it using
    // things) off (`DisablePlayerControls`).
    let rollover = !state.controls_off[world::scripting::controls::ROLLOVER];
    activatable.0 = if conversation.0.is_none() && rollover {
        let refs: std::borrow::Cow<[world::scripting::Interactive]> = if dropped.is_empty() {
            std::borrow::Cow::Borrowed(&cell_scripts.refs)
        } else {
            std::borrow::Cow::Owned([cell_scripts.refs.as_slice(), &dropped].concat())
        };
        object_in_view(&scripts.0, order, state, &refs, crosshair.target())
    } else {
        None
    };

    // Menus scripts opened. None are drawn yet: each counts as opened and
    // closed at once, with nothing changed, so its MenuMode blocks run once.
    let mut menus = Vec::new();
    let mut save_requests = Vec::new();
    for event in std::mem::take(&mut state.events) {
        let name = |id: FormId| {
            order
                .get(id)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.full_name().or_else(|| r.editor_id()))
                .unwrap_or_else(|| id.to_string())
        };
        // The picture beside a corner message.
        let mut notice_icon = None;
        let notice = match event {
            // A message box waits for an answer.
            Event::Message {
                title,
                text,
                buttons,
                ..
            } if !buttons.is_empty() => {
                println!("Message box shown: {}", text.lines().next().unwrap_or(""));
                waiting.push(crate::menus::Menu::Message {
                    title,
                    text: fill_keys(&text),
                    buttons,
                });
                None
            }
            Event::Message {
                title, text, icon, ..
            } => {
                notice_icon = icon;
                Some(fill_keys(&match title {
                    Some(t) => format!("{t}\n{text}"),
                    None => text,
                }))
            }
            Event::CharacterMenu(m) => {
                waiting.push(crate::menus::Menu::Character(m));
                None
            }
            // The game's own box (a reputation's title, `game_menus::message`).
            Event::Popup {
                title,
                text,
                icon,
                sound,
            } => {
                waiting.push(crate::menus::Menu::Popup {
                    title,
                    text,
                    icon,
                    sound,
                });
                None
            }
            Event::Barter(merchant) => {
                waiting.push(crate::menus::Menu::Barter(merchant));
                None
            }
            // `game_menus::recipe`, with `world::crafting`'s rules.
            Event::RecipeMenu { actor, category } => {
                waiting.push(crate::menus::Menu::Recipe { actor, category });
                None
            }
            Event::RepairServices(vendor) => {
                waiting.push(crate::menus::Menu::RepairServices(vendor));
                None
            }
            Event::TutorialMenu(message) => {
                waiting.push(crate::menus::Menu::Tutorial(message));
                None
            }
            Event::TeammateContainer(who) => {
                waiting.push(crate::menus::Menu::Teammate(who));
                None
            }
            Event::BackUp(who) => {
                println!("{who} steps back from the player (default package 0x27): not carried out here.");
                None
            }
            // `Create`'s checks as the command runs (`005cf040` and the
            // others call it at once): the menu, or the refusal's corner
            // message with `UIPopUpMessageGeneral`.
            Event::Casino {
                game,
                casino,
                min_bet,
                max_bet,
                min_winnings,
            } => {
                let created = crate::game_menus::casino::create(
                    order,
                    state,
                    &mut casino_lock.0,
                    game,
                    casino,
                    min_bet,
                    max_bet,
                    min_winnings,
                    real_time.elapsed_secs_f64(),
                );
                // The scripts' `MenuMode` for the game (the Sierra Madre's
                // and the base game's tables alike) once it's open.
                menus.extend(crate::game_menus::casino::menu_mode(game, &created));
                match created {
                    Some(Ok(menu)) => {
                        waiting.push(menu);
                        None
                    }
                    Some(Err(why)) => {
                        if let Some(s) = order.form_by_editor_id(crate::lockpick::POPUP_SOUND) {
                            sound_requests.0.push(s);
                        }
                        notice_icon = Some(world::casino::REFUSAL_ICON.to_string());
                        Some(why)
                    }
                    None => {
                        println!("{game:?}: {casino} isn't a casino.");
                        None
                    }
                }
            }
            Event::Caravan {
                npc,
                deck,
                difficulty,
                share,
            } => {
                waiting.push(crate::menus::Menu::Caravan {
                    npc,
                    deck,
                    difficulty,
                    share,
                });
                None
            }
            // `ShowSleepWaitMenu` (its refusals already given): the game's
            // sleep/wait menu (`game_menus::sleepwait`).
            Event::SleepWaitMenu { sleep } => {
                waiting.push(crate::menus::Menu::SleepWait { sleep });
                None
            }
            Event::Died { who, by } => Some(if who == PLAYER_REF {
                "You died.".to_string()
            } else {
                format!("{} was killed by {}.", name(who), name(by))
            }),
            Event::KnockedOut { who } => Some(format!("{} is down.", name(who))),
            Event::GotUp { who } => Some(format!("{} gets up.", name(who))),
            // `ForceFlee` (`world::ai::flee::force`): printed; the AI runs to
            // the place given, or stands (`ai::forced_flee_frame`).
            Event::Flees { who, to } => {
                match to {
                    Some(to) => println!("{} flees (ForceFlee) to {}.", name(who), name(to)),
                    None => println!("{} flees (ForceFlee).", name(who)),
                }
                None
            }
            Event::Journal { quest, text } => Some(format!("{}: {text}", name(quest))),
            // The HUD's objective lines (`0077a5b0`).
            Event::Objective {
                text, completed, ..
            } => {
                if hud_on {
                    // Printed as the notice was (the acceptance routes
                    // read it).
                    if completed {
                        println!("Completed: {text}");
                    } else {
                        println!("{text}");
                    }
                    objective_lines.push(ui::quest_text::Objective {
                        text,
                        completed,
                        reminder: false,
                    });
                    None
                } else {
                    Some(if completed {
                        format!("Completed: {text}")
                    } else {
                        text
                    })
                }
            }
            // The HUD's quest names (`world::quest_text`).
            Event::QuestText(text) => {
                let notice = crate::hud::quest_notice(order, text);
                if hud_on {
                    quest_texts.push(notice);
                    None
                } else {
                    Some(format!("{}\n{}", notice.title, notice.subtitle))
                }
            }
            Event::Talk {
                speaker,
                to,
                topic,
                conversation,
            } => {
                // `StartConversation` by someone else: they come up first
                // (the conversation package, `ai::Starts`).
                if conversation && speaker != PLAYER_REF && to.0 != 0 {
                    starts.0.push((speaker, to, topic));
                } else if (to == PLAYER_REF || to.0 == 0) && talk.0.is_none() {
                    // `Say` speaks to no one in particular (`to` 0): the
                    // player hears it as a line said to them.
                    // `SayTo` (a line to the player, not `Say`'s to no
                    // one): its `SayToDone` blocks run when it's said.
                    let say_to = !conversation && to == PLAYER_REF;
                    talk.0 = Some((speaker, topic, conversation, say_to));
                }
                None
            }
            Event::PackageAction { who, package, kind } => {
                if let Some(p) = world::ai::Package::load(order, package) {
                    let action = p.actions.get(kind);
                    // Camera-bearing player idles with no script or topic
                    // can now use the actual KF.
                    if who == PLAYER_REF
                        && action.topic.is_none()
                        && action.bytecode().is_empty()
                        && action.source().is_none_or(|s| s.trim().is_empty())
                    {
                        if let Some(idle) = action.idle {
                            player_idle.requests.push(idle);
                        }
                        continue;
                    }
                    let busy = action.idle.is_some()
                        || action.topic.is_some()
                        || action.source().is_some_and(|s| !s.trim().is_empty());
                    // Its script, idle and topic (`world::ai::actions`).
                    let idle = world::ai::actions::perform(
                        &mut Runner::new(order, &scripts.0, state),
                        who,
                        &p,
                        kind,
                    );
                    match idle {
                        Some(idle) if who == PLAYER_REF => player_idle.requests.push(idle),
                        Some(idle) => {
                            player_idle.npc_requests.insert(who, idle);
                        }
                        None => {}
                    }
                    if busy {
                        println!("{}: package {} {kind:?} action.", name(who), name(package));
                    }
                }
                None
            }
            Event::PlayIdle { who, idle } => {
                if who != PLAYER_REF {
                    player_idle.npc_requests.insert(who, idle);
                } else {
                    println!("Player PlayIdle {}: scripted dispatch pending.", name(idle));
                }
                None
            }
            Event::SwapTexture {
                what,
                node,
                texture,
            } => {
                texture_swaps.0.push((what, node, texture));
                None
            }
            Event::Enable(r, on) => {
                let mut found = false;
                for (p, mut visibility) in &mut placed {
                    if p.0 == r.0 {
                        *visibility = if on {
                            Visibility::Inherited
                        } else {
                            Visibility::Hidden
                        };
                        found = true;
                    }
                }
                if on && !found {
                    println!(
                        "A script enabled {} ({}), which isn't loaded here.",
                        name(r),
                        r
                    );
                }
                None
            }
            Event::PlayerControls(_) => {
                let names = [
                    "movement",
                    "Pip-Boy",
                    "fighting",
                    "view",
                    "looking",
                    "roll-over",
                    "sneaking",
                ];
                let off: Vec<&str> = names
                    .iter()
                    .zip(state.controls_off)
                    .filter(|(_, off)| *off)
                    .map(|(n, _)| *n)
                    .collect();
                println!(
                    "A script set the player's controls: {}.",
                    if off.is_empty() {
                        "all on".to_string()
                    } else {
                        format!("off: {}", off.join(", "))
                    }
                );
                None
            }
            Event::MoveTo { what, to, at } if what == PLAYER_REF => {
                println!("A script moves the player to {}.", name(to));
                if let Err(e) =
                    move_player(&game.0, state, to, at, &mut pending, &mut pending_exterior)
                {
                    println!("  couldn't: {e}");
                }
                None
            }
            // Someone loaded here goes to the marker (or out of sight, if
            // it's elsewhere); someone moved in from elsewhere shows up
            // only once the place is loaded again.
            Event::MoveTo { what, to, .. } => {
                println!("A script moved {} to {}.", name(what), name(to));
                moved.0.push(what);
                None
            }
            // Played by `effects::play_effects` from the state's list.
            Event::ImageSpace(m, on) => {
                println!(
                    "A script {} the screen effect {}.",
                    if on { "applied" } else { "removed" },
                    name(m)
                );
                None
            }
            // Played by `movie::start_movies`, before anything else moves.
            Event::Video(video) => {
                println!("A script plays the movie {}.", video.file);
                // co-op: the scene layer starts it on every machine together
                if !state.coop_scene_owns_movies() {
                    movies.queue.push_back(video);
                }
                None
            }
            // Played by `music::play_music`.
            Event::Music(m) => {
                println!("A script plays the music {}.", name(m));
                music_requests.0.push(m);
                None
            }
            Event::Weather(w) => {
                match w {
                    Some(w) => println!("A script forces the weather {}.", name(w)),
                    None => println!("A script releases the forced weather."),
                }
                None
            }
            Event::Menu(menu) => {
                // A casino game's number is counted once its menu opens
                // (`Event::Casino`, `game_menus::casino::menu_mode`).
                if !(1080..=1082).contains(&menu) {
                    menus.push(menu);
                }
                match menu {
                    world::scripting::RACE_SEX_MENU => {
                        Some("(The face and body menu would open here; kept as it is.)".to_string())
                    }
                    // Opened by their own events (`Event::RecipeMenu`,
                    // `Event::Casino`); the number is for `MenuMode`.
                    world::crafting::RECIPE_MENU | 1080..=1082 => None,
                    _ => Some(format!("(Menu {menu} would open here.)")),
                }
            }
            Event::Sound(sound) => {
                sound_requests.0.push(sound);
                None
            }
            // A placed object's model plays a sequence from now on
            // (`move_pieces`). The script's flags (a blend-in time on
            // objects' controllers, `005c0df0`) aren't used yet.
            Event::PlayGroup {
                who,
                group,
                flags: _,
            } => {
                groups.0.insert(who.0, (group, now));
                None
            }
            // A script activating a door that swings where it stands opens
            // or closes it, with its sound (`world::doors`).
            Event::Activate { what, by } if world::doors::is_swing_door(order, what) => {
                crate::doors::activate(
                    order,
                    state,
                    &mut sound_requests,
                    what,
                    by.filter(|b| b.0 != 0),
                );
                None
            }
            // A script having the player use a terminal or a container
            // opens it, as the player's own E would (a locked container
            // its lockpicking menu, or says why not).
            Event::Activate {
                what,
                by: Some(PLAYER_REF),
            } => {
                let base = world::scripting::base_of(order, what);
                match base.and_then(|b| order.get(b)).map(|r| r.entry.header.kind) {
                    Some(k) if k.as_bytes() == b"TERM" => {
                        use crate::game_menus::hacking::{use_terminal, Using};
                        match use_terminal(order, state, base.unwrap_or(what), what) {
                            Using::Open(m) => {
                                world::terminal::opened(order, state, base.unwrap_or(what), what);
                                waiting.push(m);
                                None
                            }
                            Using::Refused(why) => {
                                if let Some(s) =
                                    order.form_by_editor_id(crate::lockpick::POPUP_SOUND)
                                {
                                    sound_requests.0.push(s);
                                }
                                notice_icon = Some(world::message_icon::ANGRY.to_string());
                                Some(why)
                            }
                        }
                    }
                    Some(k) if k.as_bytes() == b"CONT" => {
                        match locked(order, state, what, &name(what)) {
                            None => {
                                waiting.push(crate::menus::Menu::Container(what, name(what)));
                                None
                            }
                            Some(Locked::Pick) => {
                                lockpicking.request = Some(what);
                                None
                            }
                            Some(Locked::Says(why)) => {
                                notice_icon = Some(world::message_icon::PADLOCK.to_string());
                                Some(why)
                            }
                        }
                    }
                    // A talking activator with a voice (Elijah's hologram)
                    // starts a conversation, as the player's own E would.
                    Some(k)
                        if k.as_bytes() == b"TACT"
                            && base.is_some_and(|b| {
                                world::dialogue::Speaker::load(order, what, b)
                                    .is_some_and(|s| s.voice.is_some())
                            }) =>
                    {
                        talk.0 = Some((what, None, true, false));
                        None
                    }
                    _ => None,
                }
            }
            Event::Activate { .. } => None,
            // An item's own `ForceTerminalBack` is carried out by the
            // terminal menu (`game_menus::computers`). From another script
            // it isn't (the game's goes back if a terminal menu is open).
            Event::TerminalBack => None,
            Event::More(shown) => {
                println!("{}", world::more_functions::describe(order, state, &shown));
                match shown {
                    // The game's save manager is asked for a save
                    // (`Autosave`, `ForceSave`, `SystemSave`): here this
                    // project's own format, as F5's, to a file of its own.
                    world::more_functions::Shown::Save(kind) => {
                        let file = match kind {
                            world::more_functions::SaveKind::Autosave => "nv-rs-autosave.txt",
                            world::more_functions::SaveKind::Force => "nv-rs-forcesave.txt",
                            world::more_functions::SaveKind::System => "nv-rs-systemsave.txt",
                        };
                        save_requests.push(file);
                    }
                    // Everything runs at the multiplier's speed.
                    world::more_functions::Shown::TimeMultiplier(m) => {
                        virtual_time.set_relative_speed(m.max(0.0));
                    }
                    // `PushActorAway`: knocking someone down alive (a
                    // ragdoll that gets up again) isn't drawn yet; only the
                    // dead go limp here (`ActorRig::go_limp`).
                    world::more_functions::Shown::PushedAway { .. } => {}
                    // `FireWeapon`: shot in `combat::object_shots`.
                    world::more_functions::Shown::WeaponFired { from, weapon } => {
                        object_shots.0.push((from, weapon));
                    }
                    _ => {}
                }
                None
            }
        };
        if let Some(n) = notice {
            announce(
                crate::hud::HudMessage {
                    text: n,
                    icon: notice_icon,
                },
                &mut notices,
            );
        }
    }
    for n in &quest_texts {
        println!("{}: {}", n.title, n.subtitle);
    }
    hud_messages.quests.extend(quest_texts);
    hud_messages.objectives.extend(objective_lines);
    // Script execution has already mutated GameState. Dispatch every pending
    // idle request before snapshotting, so its corresponding presentation
    // queue cannot be lost merely because Save appeared earlier in events.
    for file in save_requests {
        let place = state.player_cell.map(|cell| world::save::PlayerPlace {
            cell,
            world: state.player_world,
            position: feet,
            heading,
        });
        let result = player_idle
            .snapshot(
                &seats,
                cameras.single().map_or(0.0, |(_, input, _)| input.pitch),
            )
            .and_then(|camera| save_camera(file, state, place, camera));
        match result {
            Ok(()) => println!("Saved ({file})."),
            Err(e) => println!("Couldn't save {file}: {e}"),
        }
    }
    // The face menu closing has everyone's packages looked at again, after
    // its MenuMode blocks (`GameState::evaluate_everyone`).
    let face_menu = menus.contains(&world::scripting::RACE_SEX_MENU);
    for menu in menus {
        Runner::new(order, &scripts.0, state).menu_mode(menu);
    }
    if face_menu {
        state.evaluate_everyone = true;
    }

    notices.0.retain(|(_, at)| now - at < NOTICE_SECONDS);
    let shown = notices
        .0
        .iter()
        .map(|(n, _)| n.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");
    for (mut t, mut visibility) in &mut text {
        if t.0 != shown {
            t.0 = shown.clone();
        }
        *visibility = if shown.is_empty() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource)]
    struct TestLoadResult(Result<bool, String>);

    #[test]
    fn later_lines_run_once_when_due_in_time_order() {
        let mut later = LaterCommands {
            lines: vec![(30.0, "b".into()), (10.0, "a".into()), (30.0, "c".into())],
            ready_at: None,
        };
        // The clock starts at the first look.
        assert!(later.due(100.0).is_empty());
        assert_eq!(later.due(111.0), ["a"]);
        assert_eq!(later.due(140.0), ["b", "c"]);
        assert!(later.due(500.0).is_empty());
    }

    fn queue_test_reload_cleanup(mut commands: Commands, result: Res<TestLoadResult>) {
        queue_reload_cleanup(&mut commands, &result.0);
    }

    fn cleanup_fixture(result: Result<bool, String>) -> World {
        let mut world = World::new();
        world.insert_resource(TestLoadResult(result));
        world.insert_resource(Conversation::test_active());
        world.insert_resource(ScriptedTalk(Some((FormId(10), None, true, false))));
        let mut lines = crate::chatter::Lines::default();
        lines.say(
            FormId(11),
            FormId(1),
            world::dialogue::Info {
                form_id: FormId(30),
                topic: None,
                quest: None,
                previous: None,
                flags: 0,
                flags2: 0,
                responses: vec![],
                conditions: vec![],
                prompt: None,
                check: None,
                choices: vec![],
                add_topics: vec![],
                follow_ups: vec![],
                begin_script: Some("SetStage TestQuest 10".into()),
                end_script: Some("SetStage TestQuest 20".into()),
            },
        );
        lines.done.push((FormId(20), FormId(30)));
        world.insert_resource(lines);
        world.insert_resource(DialogueState(GameState {
            speaking: [FormId(10)].into_iter().collect(),
            ..Default::default()
        }));
        world.insert_resource(crate::walk::Player::new(true));
        world.insert_resource(crate::dialogue::TalkTarget(Some((
            crate::dialogue::Talker {
                reference: FormId(10),
                base: FormId(11),
                position: [0.0; 3],
            },
            "Test speaker".into(),
        ))));
        world.insert_resource(crate::game_menus::GameMenus::default());
        world.insert_resource(ButtonInput::<KeyCode>::default());
        world.spawn((Text::new("Talk"), crate::walk::Prompt));
        world.spawn((
            AudioPlayer::<AudioSource>::new(Handle::default()),
            PlaybackSettings::DESPAWN,
            crate::faces::Voice {
                speaker: FormId(10),
                lip: None,
            },
        ));
        world.spawn((
            AudioPlayer::<AudioSource>::new(Handle::default()),
            PlaybackSettings::DESPAWN.paused(),
            crate::faces::Voice {
                speaker: FormId(11),
                lip: None,
            },
            crate::faces::VoiceDelay(1.0),
        ));
        world.spawn((
            AudioPlayer::<AudioSource>::new(Handle::default()),
            PlaybackSettings::DESPAWN,
        ));
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);
        world
    }

    #[test]
    fn failed_load_keeps_live_and_paused_dialogue_audio_and_input() {
        use bevy::ecs::system::RunSystemOnce;

        let data = testdata::functions::functions("dialogue-cleanup-failed");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let result = attempt_load_saved_world(&game, "not a save");
        assert!(result.is_err());
        let mut world = cleanup_fixture(result);
        world.run_system_once(queue_test_reload_cleanup).unwrap();
        world.flush();
        assert_eq!(
            world.query::<&crate::faces::Voice>().iter(&world).count(),
            2
        );
        assert_eq!(
            world
                .query::<&AudioPlayer<AudioSource>>()
                .iter(&world)
                .count(),
            3
        );
        assert!(world
            .resource::<ButtonInput<KeyCode>>()
            .just_pressed(KeyCode::Space));
        assert!(world.resource::<ScriptedTalk>().0.is_some());
        assert!(world.resource::<Conversation>().0.is_some());
        assert_eq!(world.resource::<DialogueState>().0.speaking.len(), 1);
        assert_eq!(world.resource::<crate::chatter::Lines>().done.len(), 1);
        assert_eq!(world.resource::<crate::chatter::Lines>().queue.len(), 1);
        assert!(world.resource::<crate::dialogue::TalkTarget>().0.is_some());

        let invalid_place = world::save::PlayerPlace {
            cell: FormId(0xdead_beef),
            world: None,
            position: [0.0; 3],
            heading: 0.0,
        };
        let bad_destination = world::save::save(&GameState::default(), Some(invalid_place));
        let result = attempt_load_saved_world(&game, &bad_destination);
        assert!(result.is_err());
        let mut world = cleanup_fixture(result);
        world.run_system_once(queue_test_reload_cleanup).unwrap();
        world.flush();
        assert_eq!(
            world.query::<&crate::faces::Voice>().iter(&world).count(),
            2
        );
        assert_eq!(world.resource::<crate::chatter::Lines>().queue.len(), 1);
    }

    #[test]
    fn successful_load_clears_dialogue_voices_and_preserves_other_audio() {
        use bevy::ecs::system::RunSystemOnce;

        let data = testdata::functions::functions("dialogue-cleanup-success");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let destination = world::save::PlayerPlace {
            cell: FormId(testdata::functions::ids::HOUSE),
            world: None,
            position: [10.0, 20.0, 30.0],
            heading: 1.25,
        };
        let text = world::save::save(&GameState::default(), Some(destination));
        let result = attempt_load_saved_world(&game, &text);
        assert!(result.is_ok());
        let mut world = cleanup_fixture(result);
        world.run_system_once(queue_test_reload_cleanup).unwrap();
        world.flush();
        assert_eq!(
            world.query::<&crate::faces::Voice>().iter(&world).count(),
            0
        );
        assert_eq!(
            world
                .query::<&AudioPlayer<AudioSource>>()
                .iter(&world)
                .count(),
            1
        );
        assert!(world.resource::<Conversation>().0.is_none());
        assert!(world.resource::<ScriptedTalk>().0.is_none());
        assert!(world.resource::<crate::chatter::Lines>().done.is_empty());
        assert!(world.resource::<crate::chatter::Lines>().queue.is_empty());
        assert!(world.resource::<DialogueState>().0.speaking.is_empty());
        assert!(world.resource::<crate::walk::Player>().ready);
        assert!(world.resource::<crate::dialogue::TalkTarget>().0.is_none());
        let mut prompt = world.query_filtered::<&Text, With<crate::walk::Prompt>>();
        assert!(prompt.iter(&world).all(|text| text.0.is_empty()));
        assert!(!world
            .resource::<ButtonInput<KeyCode>>()
            .just_pressed(KeyCode::Space));
    }

    #[test]
    fn dialogue_cleanup_does_not_release_movement_from_another_menu() {
        let mut world = cleanup_fixture(Ok(false));
        let mut menus = crate::menus::Menus::default();
        menus.push(crate::menus::Menu::SleepWait { sleep: false });
        world.insert_resource(menus);
        discard_reloaded_dialogue(&mut world);
        assert!(!world.resource::<crate::walk::Player>().ready);
        assert!(world.resource::<crate::menus::Menus>().is_open());
    }

    fn attempt_load_saved_world(game: &cellview::Game, text: &str) -> Result<bool, String> {
        let mut state = GameState::default();
        let mut cells = CellScripts::default();
        let mut idle = crate::player_idle::PlayerIdle::default();
        let mut pending = crate::PendingScene(None);
        let mut exterior = crate::exterior::PendingExterior::default();
        let mut seats = crate::sitting::Seats::new(&game.order);
        let mut pitch = crate::StartPitch(0.0);
        load_saved_world(
            game,
            &mut state,
            &mut cells,
            &mut idle,
            &mut pending,
            &mut exterior,
            &mut seats,
            &mut pitch,
            text,
        )
    }

    #[test]
    fn failed_reload_preserves_quests_triggers_camera_requests_and_destination() {
        let data = testdata::functions::functions("transactional-reload");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let mut state = GameState::new(&game.order);
        state.stages.insert(FormId(0x104c1c), 55);
        let before = world::save::save(&state, None);
        let mut cells = CellScripts::default();
        let mut idle = crate::player_idle::PlayerIdle::default();
        idle.requests.push(FormId(42));
        let mut pending = crate::PendingScene(None);
        let mut exterior = crate::exterior::PendingExterior::default();
        let mut seats = crate::sitting::Seats::new(&game.order);
        let mut pitch = crate::StartPitch(0.5);
        for text in [
            "not a save",
            "nv-rs save 1\nstage 00104C1C 60\nplayer FFFFFFFF - 1 2 3 0\n",
            "nv-rs save 1\nstage 00104C1C 60\nplayer FFFFFFFF FFFFFFFF 1 2 3 0\n",
        ] {
            assert!(load_saved_world(
                &game,
                &mut state,
                &mut cells,
                &mut idle,
                &mut pending,
                &mut exterior,
                &mut seats,
                &mut pitch,
                text
            )
            .is_err());
            assert_eq!(world::save::save(&state, None), before);
            assert_eq!(idle.requests, [FormId(42)]);
            assert!(pending.0.is_none());
            assert!(exterior.0.is_none());
            assert_eq!(pitch.0, 0.5);
        }
        let saved_place = world::save::PlayerPlace {
            cell: FormId(testdata::functions::ids::HOUSE),
            world: None,
            position: [10.0, 20.0, 30.0],
            heading: 1.25,
        };
        let mut loaded = GameState::new(&game.order);
        loaded.stages.insert(FormId(0x104c1c), 60);
        assert!(load_saved_world(
            &game,
            &mut state,
            &mut cells,
            &mut idle,
            &mut pending,
            &mut exterior,
            &mut seats,
            &mut pitch,
            &world::save::save(&loaded, Some(saved_place))
        )
        .unwrap());
        assert_eq!(state.stages[&FormId(0x104c1c)], 60);
        assert!(idle.requests.is_empty());
        assert_eq!(pitch.0, 0.0);
        let start = &pending.0.as_ref().unwrap().start;
        assert_eq!(start.eye, [10.0, 20.0, 30.0 + cellview::EYE_HEIGHT]);
        assert_eq!(start.heading, 1.25);

        let camera = world::save::camera::Camera {
            animation: None,
            requests: vec![FormId(71), FormId(72)],
            npc_requests: [(FormId(80), FormId(81))].into(),
            package: Some(FormId(90)),
            hand_follow: 0.875,
            pitch: -0.25,
        };
        loaded.saved_camera = Some(camera.clone());
        load_saved_world(
            &game,
            &mut state,
            &mut cells,
            &mut idle,
            &mut pending,
            &mut exterior,
            &mut seats,
            &mut pitch,
            &world::save::save(&loaded, Some(saved_place)),
        )
        .unwrap();
        assert_eq!(pitch.0, camera.pitch);
        assert_eq!(idle.snapshot(&seats, pitch.0).unwrap(), camera);

        // A save with an unavailable skeleton must not clear the restored
        // requests, pitch, quest state or already prepared destination.
        let before = world::save::save(&state, None);
        loaded.stages.insert(FormId(0x104c1c), 99);
        loaded.saved_camera.as_mut().unwrap().animation = Some(
            world::animation::Player::default()
                .snapshot(&[], |_| None)
                .unwrap(),
        );
        assert!(load_saved_world(
            &game,
            &mut state,
            &mut cells,
            &mut idle,
            &mut pending,
            &mut exterior,
            &mut seats,
            &mut pitch,
            &world::save::save(&loaded, Some(saved_place)),
        )
        .is_err());
        assert_eq!(world::save::save(&state, None), before);
        assert_eq!(idle.snapshot(&seats, pitch.0).unwrap(), camera);
        assert_eq!(
            pending.0.as_ref().unwrap().start.heading,
            saved_place.heading
        );
    }

    fn attach_test_cell(
        order: &esm::LoadOrder,
        cache: &ScriptCache,
        state: &mut GameState,
        cells: &mut CellScripts,
        cell: u32,
    ) {
        refresh_cell_scripts(order, cache, state, cells, &[FormId(cell)], |c| {
            world::scripting::interactive_references(order, c)
        });
    }

    #[test]
    fn same_cell_reload_reenters_triggers_from_the_loaded_state() {
        use esm::{ActivePlugins, LoadOrder};
        use testdata::functions::ids::{HOUSE, VIGOR_QUEST};

        let data = testdata::functions::functions("vigor-reload");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let cache = ScriptCache::default();
        let mut state = GameState::new(&order);
        state.player_cell = Some(FormId(HOUSE));
        let mut cells = CellScripts::default();
        attach_test_cell(&order, &cache, &mut state, &mut cells, HOUSE);
        let people = [(PLAYER_REF, [1888.0, 1835.0, 7360.0])];
        // Enter before the quest permits the instruction, then load a save
        // at stage55 in that same volume. Old occupancy must not suppress it.
        run_cell_scripts(&order, &cache, &mut state, &mut cells, &people, 0.016, None);
        assert!(!cells
            .scheduler
            .inside(FormId(testdata::functions::ids::VIGOR_TRIGGER_REF))
            .is_empty());
        let mut loaded = GameState::new(&order);
        loaded.player_cell = Some(FormId(HOUSE));
        loaded.stages.insert(FormId(VIGOR_QUEST), 55);
        restore_script_state(&mut state, &mut cells, loaded);
        attach_test_cell(&order, &cache, &mut state, &mut cells, HOUSE);
        run_cell_scripts(&order, &cache, &mut state, &mut cells, &people, 0.016, None);
        assert_eq!(state.stages.get(&FormId(VIGOR_QUEST)), Some(&60));
    }

    #[test]
    fn authored_vigor_activation_opens_special_at_stage_60_and_advances_to_65() {
        use esm::{ActivePlugins, LoadOrder};
        use testdata::functions::ids::{HOUSE, VIGOR_QUEST, VIGOR_TESTER_REF};

        let data = testdata::functions::functions("vigor-activation");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let cache = ScriptCache::default();
        let refs = world::scripting::interactive_references(&order, FormId(HOUSE));
        let tester = refs
            .iter()
            .find(|r| r.reference == FormId(VIGOR_TESTER_REF))
            .cloned()
            .expect("fixture placed the scripted tester");

        let mut state = GameState::new(&order);
        state.stages.insert(FormId(VIGOR_QUEST), 55);
        Runner::new(&order, &cache, &mut state).run_source(
            "SetObjectiveDisplayed VCG01 10 1",
            None,
            None,
        );
        assert!(use_object(&order, &cache, &mut state, &tester).is_none());
        assert!(!state
            .events
            .iter()
            .any(|e| matches!(e, Event::CharacterMenu(_))));
        assert_eq!(state.stages.get(&FormId(VIGOR_QUEST)), Some(&55));
        let mut cell_scripts = CellScripts::default();
        attach_test_cell(&order, &cache, &mut state, &mut cell_scripts, HOUSE);
        run_cell_scripts(
            &order,
            &cache,
            &mut state,
            &mut cell_scripts,
            &[(PLAYER_REF, [1888.0, 1835.0, 7360.0])],
            1.0 / 60.0,
            None,
        );
        assert_eq!(state.stages.get(&FormId(VIGOR_QUEST)), Some(&60));
        assert!(!state.objectives.contains_key(&(FormId(VIGOR_QUEST), 30)));
        assert!(state.objectives.contains_key(&(FormId(VIGOR_QUEST), 10)));
        assert!(use_object(&order, &cache, &mut state, &tester).is_none());
        assert!(!state
            .events
            .iter()
            .any(|e| matches!(e, Event::CharacterMenu(_))));

        // This is the actual installed INFO 001074A2 result script. The
        // headless fixture simulates its execution, not topic selection or
        // Doc's authored dialogue delivery.
        Runner::new(&order, &cache, &mut state).run_source(
            "SetObjectiveDisplayed VCG01 30 1",
            None,
            None,
        );
        assert!(state.objectives.contains_key(&(FormId(VIGOR_QUEST), 30)));
        assert!(use_object(&order, &cache, &mut state, &tester).is_none());
        assert_eq!(state.stages.get(&FormId(VIGOR_QUEST)), Some(&65));
        assert!(state.events.contains(&Event::CharacterMenu(
            world::chargen::CharacterMenu::Special { points: 40 }
        )));

        // Both authored gate conditions matter: stage 60 with objective 30
        // absent leaves the machine closed and the stage unchanged.
        let mut gated = GameState::new(&order);
        gated.stages.insert(FormId(VIGOR_QUEST), 60);
        assert!(use_object(&order, &cache, &mut gated, &tester).is_none());
        assert_eq!(gated.stages.get(&FormId(VIGOR_QUEST)), Some(&60));
        assert!(!gated
            .events
            .iter()
            .any(|e| matches!(e, Event::CharacterMenu(_))));
    }

    /// E uses the object the crosshair is on (`crosshair`, the game's view
    /// caster) and nothing else: the Vigor Tester (an `ACTI` with an
    /// `OnActivate` script and an empty `OBND`, picked by its collision),
    /// not when the crosshair is elsewhere, and not once it's destroyed.
    #[test]
    fn the_object_used_is_the_one_the_crosshair_is_on() {
        use esm::{ActivePlugins, LoadOrder};
        use testdata::functions::ids::{HOUSE, VIGOR_TESTER_REF};

        let data = testdata::functions::functions("crosshair-object");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let cache = ScriptCache::default();
        let refs = world::scripting::interactive_references(&order, FormId(HOUSE));
        let mut state = GameState::new(&order);
        let tester = FormId(VIGOR_TESTER_REF);
        let seen = |state: &GameState, target| {
            object_in_view(&cache, &order, state, &refs, target).map(|(r, _)| r)
        };
        assert_eq!(seen(&state, Some(tester)), Some(tester));
        assert_eq!(seen(&state, None), None);
        assert_eq!(seen(&state, Some(FormId(0x00AB_CDEF))), None);
        state.destroyed.insert(tester);
        assert_eq!(seen(&state, Some(tester)), None);
    }
    #[test]
    fn keys_in_messages_become_the_viewers() {
        assert_eq!(
            fill_keys(
                "Use &-sUActnForward;&-sUActnSldleft;&-sUActnBack;&-sUActnSldright; to move."
            ),
            "Use WASD to move."
        );
        assert_eq!(
            fill_keys("To toggle between run and walk, &sUActnToggleRun;."),
            "To toggle between run and walk, Shift."
        );
        assert_eq!(fill_keys("Grab with &sUActnGrab;"), "Grab with [grab]");
        assert_eq!(
            fill_keys("To enter VATS, &sUActnVATS;."),
            "To enter VATS, V."
        );
        assert_eq!(fill_keys("Rock & roll; no key"), "Rock & roll; no key");
    }
}
