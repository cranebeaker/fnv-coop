//! nv-viewer: walk around a Fallout: New Vegas cell in real time.
//!
//! The game-specific work (reading the cell, placing models, textures and
//! materials) happens in the `cellview` crate; this file only hands the
//! result to Bevy and flies a camera around it. Written against Bevy 0.16.

mod actors;
mod ai;
mod anim_library;
mod args;
mod background;
mod bolts;
mod caravan_table;
mod casino_scene;
mod chatter;
mod clutter;
mod combat;
mod companions;
mod controls;
mod crosshair;
mod daylight;
mod dialogue;
mod doors;
mod dress;
mod effects;
mod emittance;
mod explosives;
mod exterior;
mod faces;
mod fighting;
mod fos_start;
mod frame_order;
mod frame_work;
mod game_menus;
mod grade;
mod grass;
mod hiteffects;
mod hud;
mod impact_fx;
mod lighting;
mod local_map;
mod lockpick;
mod lod;
mod lod_objects;
mod look;
mod map;
mod menu_background;
mod menus;
mod movie;
mod music;
mod particles;
mod pipboy;
mod player_body;
mod player_camera;
mod player_idle;
mod poll;
mod present;
mod radio;
mod rendered_terminal;
mod report;
mod scope;
mod scripts;
mod shared_light;
mod sight;
mod sitting;
mod sounds;
mod swaps;
mod terrain;
mod test_keys;
mod trees;
mod vats;
mod viewmodel;
mod walk;
mod water;
mod weapon_fx;
mod weather;

use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::audio::AddAudioSource;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::ecs::system::SystemParam;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, Exposure};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_resource::{
    Extent3d, Face, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureViewDescriptor, TextureViewDimension, WgpuFeatures,
};
use bevy::render::renderer::RenderDevice;
#[cfg(target_os = "macos")]
use bevy::render::settings::Backends;
#[cfg(target_os = "macos")]
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::view::screenshot::{save_to_disk, Screenshot, ScreenshotCaptured};
use bevy::render::RenderPlugin;
use bevy::window::{CursorGrabMode, WindowResolution};
use cellview::{space, Blend, Game, GpuFormat, TextureData, ViewerScene};
use exterior::{ExteriorStart, PendingExterior};
use frame_order::{FrameSet, PlayerSet, ViewerSet};
use grade::{GradePlugin, ImageSpaceGrade};
use lighting::{
    DrawKey, GameLight, GameLighting, GameLightingPlugin, GameLit, GameLitMaterial, MAX_LIGHTS,
};
use lod::{LodLandMaterial, LodLandParams, LodPlugin};
use terrain::{Terrain, TerrainMaterial, TerrainPlugin};
use world::frame::Stage;

/// Starting exposure (EV100); lower is brighter. At this exposure lit
/// surfaces show exactly the brightness the game's lighting gives them.
const START_EV100: f32 = 5.5;

/// The luminance (cd/m²) that shows at full brightness at the starting
/// exposure (Bevy scales what the camera sees by 1 / (1.2 × 2^EV100)). The
/// lighting shader scales its result by it, so the exposure keys still
/// work.
fn full_brightness_nits() -> f32 {
    1.2 * 2f32.powf(START_EV100)
}

/// Use Apple's native graphics API for macOS builds. Bevy's default backend
/// set is cross-platform; macOS builds select Metal explicitly.
fn renderer_plugin() -> RenderPlugin {
    #[cfg(target_os = "macos")]
    {
        let settings = WgpuSettings {
            backends: Some(Backends::METAL),
            ..default()
        };
        RenderPlugin {
            render_creation: RenderCreation::Automatic(settings),
            ..default()
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        RenderPlugin::default()
    }
}

const LOOK_SPEED: f32 = 0.003;

/// The camera's near plane, in meters.
const CAMERA_NEAR_METERS: f32 = 0.05;

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn SetProcessDpiAwarenessContext(context: isize) -> i32;
}

fn main() {
    // With the game's own window (`NV_D3D9=real`) the process is DPI-unaware, as FalloutNV.exe is
    // (no `dpiAware` in its manifest): Windows scales the game's window on a display above 100%
    // from the start, as it does the original's. Set before the window library starts, which would
    // make the process per-monitor aware (the awareness is set once per process); the game's window
    // then shrank to its unscaled size several seconds into the run (playtest, 2026-10-30).
    #[cfg(windows)]
    // The same test as the real device's (`d3d9real::wanted`): any case.
    if std::env::var("NV_D3D9").is_ok_and(|v| v.eq_ignore_ascii_case("real")) {
        // DPI_AWARENESS_CONTEXT_UNAWARE.
        // SAFETY: no pointers; a failure leaves the awareness as it was.
        unsafe {
            SetProcessDpiAwarenessContext(-1);
        }
    }
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let args = match args::parse(&raw) {
        Ok(Some(args)) => args,
        Ok(None) => {
            print!("{}", args::USAGE);
            return;
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{}", args::USAGE);
            std::process::exit(2);
        }
    };
    // `NV_GUESSES=1`: untraced behaviour marked [G] runs too
    // (`world::guesses`; the Dead Money contributor's follower rules).
    if std::env::var("NV_GUESSES").is_ok_and(|v| v == "1") {
        world::guesses::set(true);
        println!("NV_GUESSES=1: untraced [G] behaviour is on.");
    }
    let key_presses = match args
        .key_at
        .iter()
        .map(|(at, key)| test_keys::parse(*at, key))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(p) => p,
        Err(message) => {
            eprintln!("error: {message}");
            std::process::exit(2);
        }
    };
    let data = match cellview::find_data_folder(&args.data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    // The window in front before any of ours opens (`--background` gives the focus back to
    // it when the viewer's own window takes it), read before the game makes its real window,
    // which is shown activated (`NV_D3D9_BACKGROUND` keeps scripted runs in the background).
    let user_window = background::foreground();
    // The game's own code (`crates/engine`) runs on the player's
    // FalloutNV.exe, read from the same installation as Data: any build but
    // 1.4.0.525 is refused here, with the reason (the maintainer's decision,
    // 2026-10-13). Started now, it doesn't run anything yet.
    let mut game_code = match host::Host::start(&data) {
        Ok(host) => host,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    println!("{}", game_code.describe());
    // `--movement-probe`: the movement code goes on past untranslated
    // functions to list more of them (`host::player_move`).
    game_code.movement.probe = args.movement_probe;
    let movement = match walk::Movement::new(
        args.movement,
        args.movement_log.as_deref(),
        args.movement_missing.clone(),
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: --movement-log: {e}");
            std::process::exit(1);
        }
    };
    println!("Loading {} from {} ...", args.cell, data.display());
    let started = std::time::Instant::now();
    let game = match Game::open(&data, &args.options) {
        Ok(game) => Arc::new(game),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    // Which quests the game's own interpreter runs (`host::quest_vm`):
    // command-free quests and those whose only function is GetSecondsPassed.
    let (taken, scripted) =
        host::quest_vm::eligible(&game.order, &world::scripting::ScriptCache::default());
    println!(
        "Game's interpreter: runs {taken} of {scripted} scripted quests (the rest through \
         crates/script)."
    );
    // `--load-fos`: an original save gives the state and the place.
    let mut from_save = match args.load_fos.as_deref().map(|p| fos_start::open(&game, p)) {
        Some(Ok(start)) => Some(start),
        Some(Err(e)) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        None => None,
    };
    let opened = match from_save.as_mut() {
        Some(start) => Ok((start.scene.take(), start.outdoors.take())),
        None => open_place(&game, &args.cell, args.at),
    };
    let (scene, outdoors) = match opened {
        Ok(place) => place,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    let title = match (&scene, &outdoors) {
        (Some(scene), _) => {
            for note in &scene.notes {
                println!("  {note}");
            }
            println!(
                "Loaded {} in {:.1} s; starting at {}.",
                scene.cell,
                started.elapsed().as_secs_f32(),
                scene.start.via
            );
            format!("nv-rs viewer - {}", scene.cell)
        }
        (None, Some(start)) => format!("nv-rs viewer - {}", start.grid.world.label()),
        (None, None) => unreachable!("open_place gives one or the other"),
    };
    // The game's console gives pitch positive looking down; the camera's
    // is positive looking up.
    let pitch = args.at.map_or(0.0, |at| -at.pitch.to_radians());

    let grading = Grading {
        grade: ImageSpaceGrade::NEUTRAL,
        on: true,
    };
    let mut window = Window { title, ..default() };
    // With the game's real window (`NV_D3D9=real`) the viewer's own window and scene are not shown
    // (the maintainer, 2026-10-28): the window stays hidden and no camera draws
    // ([`ViewerHidden`]); the game's frame, its input from the real window, the sound and the
    // host go on. `NV_VIEWER_WINDOW=1` keeps the viewer's window and drawing as before.
    let hidden = viewer_hidden();
    if hidden {
        window.visible = false;
        println!("NV_D3D9=real: the viewer's own window is hidden and draws nothing (NV_VIEWER_WINDOW=1 shows it).");
    }
    if args.screenshot.is_some() {
        // The game's own resolution here, so pictures line up pixel for pixel.
        let (w, h) = args.screen_size.unwrap_or((1920, 1080));
        window.resolution =
            WindowResolution::new(w as f32, h as f32).with_scale_factor_override(1.0);
    }
    if args.background {
        // A test run out of the way: behind the other windows and without
        // the focus (and the mouse never held: `grab_cursor`), at
        // `--screen-size` when given.
        window.focused = false;
        window.window_level = bevy::window::WindowLevel::AlwaysOnBottom;
        if let (None, Some((w, h))) = (&args.screenshot, args.screen_size) {
            window.resolution =
                WindowResolution::new(w as f32, h as f32).with_scale_factor_override(1.0);
        }
    }
    // Walking, except for screenshots, which keep the exact eye given.
    let player = walk::Player::new(args.screenshot.is_none() || args.walk);
    // A ready-made test character, set up before the first frame.
    let mut state = match from_save {
        Some(start) => start.state,
        None => world::dialogue::GameState::new(&game.order),
    };
    if let Some(path) = &args.character {
        match std::fs::read_to_string(path) {
            Ok(text) => {
                let scripts = world::scripting::ScriptCache::default();
                let problems = world::character::apply(&game.order, &scripts, &mut state, &text);
                println!(
                    "Character {}: {} problem(s).",
                    path.display(),
                    problems.len()
                );
                for p in problems {
                    println!("  line {}: {} ({})", p.line, p.text, p.why);
                }
                // What it set up isn't news to show on screen.
                state.events.clear();
            }
            Err(e) => {
                eprintln!("error: can't read {}: {e}", path.display());
                std::process::exit(1);
            }
        }
    }
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        // Every frame runs whether or not the window has the focus. The game's own frame runs in
        // `Update` (`walk::game_main_frame`), and Bevy's default for an unfocused window (reactive,
        // at most 60 updates a second) held the game to 60 frames/s whenever its real window
        // (`NV_D3D9=real`), a `--background` run or any other window had the focus (a29's
        // profile: 9 of 17 ms a frame waiting in winit).
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        })
        // Lit surfaces do their own lighting (see `lighting`); nothing else
        // should add Bevy's ambient light.
        .insert_resource(AmbientLight::NONE)
        .insert_resource(Settings {
            brightness: args.brightness,
            cloud_time: args.cloud_time,
            anisotropy: anisotropy_setting(&game.settings),
        })
        .insert_resource(StartPitch(pitch))
        .insert_resource(FrameCounter {
            on: args.fps,
            ..default()
        })
        .insert_resource(dialogue::DialogueState(state))
        // Main thread only: the engine is not `Send` (`host`).
        .insert_non_send_resource(game_code)
        .insert_resource(sitting::Seats::new(&game.order))
        .init_resource::<sitting::PlayerSeat>()
        .insert_resource(faces::Faces::new(&game))
        .insert_resource(actors::AnimSettings::read(&game.settings))
        .insert_resource(ai::Moves::new(&game))
        .insert_resource(ai::CellBuffer::new(&game))
        .insert_resource(player_camera::PlayerView::new(&game.order, &game.settings))
        .insert_resource(controls::Controls::read(&game.settings))
        .init_resource::<player_body::PlayerBody>()
        .insert_resource(look::LookSettings(world::look_ik::Settings::read(
            |section, key| game.settings.float(section, key),
        )))
        .insert_resource(GameFiles({
            // The sound folders listed in the background, so the first
            // gunshot doesn't wait for it (`Game::sound_path`).
            let warm = game.clone();
            std::thread::spawn(move || warm.warm_sound_folders());
            game
        }))
        .insert_resource(movie::Movies::new(data.clone(), args.movies))
        .init_resource::<scripts::Scripts>()
        .init_resource::<player_idle::PlayerIdle>()
        .init_resource::<look::LookAnchors>()
        .init_resource::<scripts::Here>()
        .init_resource::<scripts::ScriptedTalk>()
        .init_resource::<scripts::Notices>()
        .init_resource::<combat::ObjectShots>()
        .init_resource::<swaps::TextureSwaps>()
        .insert_resource(scripts::StartStage(args.stage.clone()))
        .insert_resource(scripts::StartCommands(args.run.clone()))
        .insert_resource(scripts::LaterCommands {
            lines: args.run_at.clone(),
            ready_at: None,
        })
        .insert_resource(dialogue::AutoSay(args.say.iter().cloned().collect()))
        .insert_resource(dialogue::AutoSayId(args.say_ids.iter().copied().collect()))
        .insert_resource(test_keys::TestKeys::new(key_presses))
        .insert_resource(viewmodel::StartWeapon(args.weapon.clone()))
        .insert_resource(lockpick::StartLock(args.lockpick.clone()))
        .insert_resource(emittance::StartRegion(args.weather_region.clone()))
        .insert_resource(viewmodel::ShowInPictures(args.weapon.is_some()))
        .init_resource::<scripts::CellScripts>()
        .init_resource::<ai::CellNav>()
        .init_resource::<ai::PathQueue>()
        .init_resource::<ai::CombatSettings>()
        .init_resource::<fighting::NpcShots>()
        .init_resource::<ai::Moved>()
        .init_resource::<BroughtIn>()
        .init_resource::<ai::Chats>()
        .init_resource::<ai::Starts>()
        .init_resource::<ai::PlayerHello>()
        .init_resource::<chatter::Lines>()
        .init_resource::<anim_library::AnimLibrary>()
        .init_resource::<menus::Menus>()
        .init_resource::<map::MapMarkers>()
        .init_resource::<hud::QuestCompass>()
        .init_resource::<daylight::Daylight>()
        .init_resource::<weather::Weathers>()
        .init_resource::<emittance::Glows>()
        .init_resource::<lod_objects::DistantObjects>()
        .init_resource::<effects::Effects>()
        .init_resource::<menu_background::MenuBackground>()
        .init_resource::<PlayingGroups>()
        .init_resource::<doors::SwingDoors>()
        .init_resource::<doors::DoorPoses>()
        .init_resource::<combat::PlayerAttack>()
        .init_resource::<scripts::LoadRequest>()
        .init_resource::<scope::ScopeOverlay>()
        .init_resource::<viewmodel::Scoped>()
        .init_resource::<viewmodel::GunWobble>()
        .insert_resource(vats::Vats::new(args.vats))
        .init_resource::<viewmodel::PlaceLighting>()
        .init_resource::<viewmodel::ViewModel>()
        .init_resource::<viewmodel::IronSightsFov>()
        .init_resource::<viewmodel::SightingNode>()
        .init_resource::<sounds::SoundRequests>()
        .init_resource::<sounds::Ambient>()
        .init_resource::<scripts::Activatable>()
        .init_resource::<scripts::ActivateRequest>()
        .init_resource::<dialogue::Talkers>()
        .init_resource::<dialogue::TalkTarget>()
        .init_resource::<crosshair::Crosshair>()
        .init_resource::<dialogue::Conversation>()
        .init_resource::<dialogue::DialogueView>()
        .insert_resource(dialogue::AutoTalk(
            args.talk,
            args.choose.iter().copied().collect(),
        ))
        .insert_resource(player)
        .insert_resource(walk::CellCollision(physics::Collider::new()))
        .insert_resource(walk::Doors(Vec::new()))
        .init_resource::<UploadedTextures>()
        .init_resource::<daylight::RemadeLit>()
        .insert_resource(PendingScene(scene))
        .insert_resource(PendingExterior(outdoors))
        .insert_resource(grading)
        .insert_resource(ScreenshotRequest {
            path: args.screenshot,
            frames: 0,
            taken: false,
            wait: args.wait,
            waited: 0.0,
        })
        .add_plugins({
            let plugins = DefaultPlugins.set(renderer_plugin()).set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            });
            // Drawing on its own thread (Bevy's default) or in step with
            // the game's frame (`NV_SYNC_RENDER`, a debugging aid).
            if std::env::var_os("NV_SYNC_RENDER").is_some() {
                plugins.disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>()
            } else {
                plugins
            }
        })
        .insert_resource(hud::ShowHud(args.hud))
        .insert_resource(ai::FrozenAi(args.freeze_ai))
        .insert_resource(movement)
        .insert_resource(clutter::EngineWorld::new(args.physics))
        .insert_resource(Background(args.background))
        .insert_resource(background::UserWindow(if args.background {
            user_window
        } else {
            0
        }))
        .add_plugins(frame_order::FrameOrderPlugin)
        .add_systems(
            Update,
            background::give_focus_back.in_set(ViewerSet::AfterFrame),
        )
        .add_plugins(frame_work::FrameWorkPlugin)
        .insert_resource(game_menus::StartMenu(args.open_menu.clone()))
        .insert_resource(scripts::StartUse(args.use_on.clone()))
        .insert_resource(game_menus::FixedPointer(args.menu_pointer))
        .insert_resource(game_menus::FixedClicks {
            at: args.menu_clicks.clone(),
            release: false,
        })
        .insert_resource(game_menus::FixedKeys::new(
            args.menu_keys.clone(),
            args.menu_keys_player_clock,
        ))
        .insert_resource(game_menus::AnswerBoxes::new(
            args.answer_boxes,
            args.box_answers.clone(),
        ))
        .add_plugins((GradePlugin, GameLightingPlugin, TerrainPlugin, LodPlugin))
        .add_plugins(shared_light::SharedLightPlugin)
        .add_plugins(present::PresentPlugin)
        // After the default plugins: they load shaders.
        .add_plugins((hud::HudPlugin, pipboy::PipboyPlugin))
        .add_plugins(game_menus::GameMenusPlugin)
        .insert_resource(pipboy::StartPipboy(args.pipboy.clone()))
        .insert_resource(pipboy::StartPipboyKeys(args.pipboy_keys.clone()))
        .insert_resource(pipboy::PretendPad(args.pad))
        .add_plugins(grass::GrassPlugin)
        .add_plugins(trees::TreePlugin)
        .add_plugins(water::WaterPlugin)
        .add_plugins(particles::ParticlesPlugin)
        .add_plugins((
            music::MusicPlugin,
            radio::RadioPlugin,
            local_map::LocalMapPlugin,
        ))
        .add_plugins(hiteffects::HitEffectsPlugin)
        .add_plugins(impact_fx::ImpactFxPlugin)
        .add_plugins(weapon_fx::WeaponEffectsPlugin)
        .add_plugins(explosives::ExplosivesPlugin)
        .add_plugins(bolts::BoltsPlugin)
        .add_plugins(clutter::ClutterPlugin)
        .add_plugins(lockpick::LockpickPlugin)
        .add_plugins(caravan_table::CaravanTablePlugin)
        .add_plugins(casino_scene::CasinoScenePlugin)
        .add_plugins(rendered_terminal::RenderedTerminalPlugin)
        .add_plugins(companions::CompanionsPlugin)
        .add_audio_source::<sounds::PcmSound>()
        // With the game's own sound on (`NV_SOUND`), the game plays its music, ambience and
        // effects itself: the viewer's own sound would double them.
        .insert_resource(if std::env::var_os("NV_SOUND").is_some() {
            bevy::audio::GlobalVolume::new(bevy::audio::Volume::SILENT)
        } else {
            bevy::audio::GlobalVolume::default()
        })
        // The first-person camera runs the image space passes with the
        // main camera's grade, once everything has set it.
        .add_systems(
            PreUpdate,
            movie::withhold_input.after(bevy::input::InputSystem),
        )
        .add_systems(PostUpdate, viewmodel::copy_grade.after(grade::adapt_eyes))
        // The camera is the eye during the frame; the third-person view
        // moves it once everything has used it, and the eye comes back
        // first thing next frame (`player_camera`).
        .add_systems(PreUpdate, player_camera::restore_eye)
        .add_systems(
            PreUpdate,
            test_keys::press_test_keys.after(bevy::input::InputSystem),
        )
        .add_systems(
            PostUpdate,
            player_camera::place_view.before(bevy::transform::TransformSystem::TransformPropagate),
        )
        .add_systems(
            Startup,
            (
                setup,
                dialogue::setup_dialogue_text,
                scripts::setup_notices,
                menus::setup_menu_text,
                combat::setup_hud,
            ),
        )
        // The per-frame systems, in the game's frame order (`frame_order`,
        // docs/FRAME_SKELETON.md "PR 3 result"): the stages and steps of
        // `Main::OnIdle` order them; a chain or `.before`/`.after` left here
        // is an order inside one set that the frame doesn't give, and says
        // why it stays.
        //
        // Ahead of the frame: a newly loaded place on screen
        // (`ViewerSet::Loading`). The land around the player streams in stage
        // 4, at the grid's update (below).
        .add_systems(
            Update,
            (spawn_scene, exterior::enter_exterior)
                // Kept: the exterior is entered once the scene is.
                .chain()
                .in_set(ViewerSet::Loading),
        )
        // The interface idle's menus (`frame_order::IdleSet`, in stage 6
        // after the controls' poll, step 33): they take this frame's input
        // before the player's step reads it in the next frame
        // (`world::frame::input`). The viewer's menus take the keyboard;
        // V.A.T.S. takes it next while it's on, and the lockpicking menu
        // (keyboard and mouse) while it's open.
        .add_systems(
            Update,
            (
                menus::run_menus,
                vats::run_vats,
                vats::scale_target_time,
                lockpick::pick_locks,
            )
                // Kept: which of them takes the input first.
                .chain()
                .in_set(frame_order::IdleSet),
        )
        // Stage 2, the player (`Main::OnIdle_UpdatePlayer`, `0086f940`, step
        // 14): its calls and `PlayerCharacter::Update`'s sub-steps are sets
        // (`frame_order::PlayerSet`, `world::frame::player`,
        // docs/FRAME_SKELETON.md "PR 4 result"), in the exe's order under
        // their gates: in menu mode only the grenade hold, with the fly
        // camera only its update, else the player's update.
        //
        // Ahead of the step: the viewer's `--weapon`, given once the place
        // is ready (not the game's).
        .add_systems(
            Update,
            viewmodel::give_start_weapon
                .in_set(FrameSet::Stage(Stage::Player))
                .before(FrameSet::step(frame_order::UPDATE_PLAYER_STEP)),
        )
        // The game's own main loop, every frame (`walk::game_main_frame`), before the walk.
        .add_systems(Update, walk::game_main_frame.before(walk::walk))
        // ... and its end, at the latest at the end of the viewer's frame.
        .add_systems(Last, walk::finish_game_main_frame)
        .add_systems(
            Update,
            (
                // The view key, the wheel, the third-person view and
                // vanity mode; the Pip-Boy's and the dialogue's views. Its
                // exe parts are on the free branch (the Toggle POV control
                // at `00942c48`, `UpdateTemp3rdPerson`/`1stPerson`, the
                // vanity orbit at `00943487`) and the knocked-down
                // `ForceTemp3rdPerson` (`0093fbd3`), all after the look;
                // kept ahead of the step and outside its gates: the mouse
                // look reads its `mouse_taken` in the same frame (the exe's
                // look runs before the view key's block, which reads the
                // flags the previous frame left, `0093f8e9`), and it also
                // takes the Pip-Boy's and the dialogue's views, which are
                // menu mode here.
                player_camera::view_input
                    .in_set(FrameSet::step(frame_order::UPDATE_PLAYER_STEP))
                    .before(PlayerSet::Call(0)),
                // The free camera (` key): `PlayerCharacter::UpdateFlyCamera`
                // in place of the player's update.
                fly_camera.in_set(PlayerSet::call(frame_order::UPDATE_FLY_CAMERA)),
                // The scope's sway on the view, inside the player's update
                // ahead of the look (its place in the exe isn't traced).
                scope::scope_sway
                    .in_set(PlayerSet::update())
                    .before(PlayerSet::at(frame_order::HEADING_AND_LOOKING_AT)),
                // The mouse look, at `PlayerCharacter::UpdateHeadingAndLooking`
                // but outside its gate: it also turns the camera to the view
                // angles every frame, which V.A.T.S.'s menu turns, and is the
                // free camera's look while flying.
                look_around
                    .in_set(FrameSet::step(frame_order::UPDATE_PLAYER_STEP))
                    .after(PlayerSet::at(frame_order::HEADING_AND_LOOKING_AT))
                    .before(PlayerSet::after(frame_order::HEADING_AND_LOOKING_AT)),
                // Attacks (`00948310`, with the Aim and Ammo Swap controls
                // read before it on the free branch).
                combat::player_attack.in_set(PlayerSet::at(frame_order::ATTACK_AT)),
                // Walking: the move (`009ea570` and the mover's slot +0x14).
                walk::walk.in_set(PlayerSet::at(frame_order::MOVE_AT)),
                // The player in furniture: on the seat, turned with it, after
                // the move and before the camera tracks (the exe's seated
                // player is on the controlled branch, `0094076c`..`009407c8`,
                // which the viewer doesn't model).
                sitting::player_furniture
                    .in_set(PlayerSet::update())
                    .after(PlayerSet::at(frame_order::MOVE_AT))
                    .before(PlayerSet::after(frame_order::MOVE_AT)),
                // The first-person view's camera tracks, at its animation
                // update but outside its gate: it takes idle requests in
                // menus too.
                player_idle::animate
                    .in_set(FrameSet::step(frame_order::UPDATE_PLAYER_STEP))
                    .after(PlayerSet::at(frame_order::OWN_VIEW_ANIMATION_AT))
                    .before(PlayerSet::after(frame_order::OWN_VIEW_ANIMATION_AT)),
            ),
        )
        // Kept: the body follows the seat the player took (not a sub-step:
        // it builds and places the third-person body, in menus too).
        .add_systems(
            Update,
            player_body::update_player_body
                .after(sitting::player_furniture)
                .in_set(FrameSet::Stage(Stage::Player)),
        )
        // After the player's step: what isn't the player's update (objects'
        // shots, the spine's facing, texture swaps, dropped weapons, people's
        // lines: their place in the frame isn't traced), the scope's overlay
        // and the first-person model (the HUD's and the renderer's work,
        // which goes on in menus), a V.A.T.S. camera shot taking the view
        // last, and what the crosshair is on, for everything E and the HUD
        // do.
        .add_systems(
            Update,
            (
                combat::object_shots,
                actors::report_facing_up,
                swaps::swap_textures,
                combat::show_dropped_weapons,
                scope::update_scope,
                viewmodel::update_view_model,
                vats::apply_shot_camera,
                crosshair::pick,
                dialogue::talk,
                chatter::say_lines,
            )
                // Kept: the viewer's order among them.
                .chain()
                .in_set(FrameSet::Stage(Stage::Player))
                .after(FrameSet::step(frame_order::UPDATE_PLAYER_STEP)),
        )
        // Stage 3, housekeeping: the cell's grade (G, for comparing), the
        // menus' background (step 44, `Main::OnIdle_HandleMenuBackground`,
        // which is `world::menu_background`) and the screen effects
        // played.
        .add_systems(
            Update,
            (
                toggle_grade.in_set(FrameSet::Stage(Stage::Housekeeping)),
                menu_background::update.in_set(FrameSet::step(frame_order::HANDLE_MENU_BACKGROUND)),
                effects::play_effects.in_set(FrameSet::Stage(Stage::Housekeeping)),
            )
                // Kept: the background's effect is played with the others;
                // the grade switch before both, as before. (The step's set is
                // inside its stage already.)
                .chain(),
        )
        // Stage 4, the world and time (`world::frame::world_time`, its
        // callees' sub-steps as `frame_order::WorldSet`s,
        // docs/FRAME_SKELETON.md "PR 5 result").
        //
        // The scripts at `ProcessLists::RunActorScripts` (step 60), in its
        // `TESObjectREFR::RunScript` sub-step: `run_scripts` runs every
        // script it knows (references', quests', items'; the exe's other
        // script runs aren't traced) and, first, the game clock
        // (`Runner::update`'s `advance_clock`, the work of `Calendar::Update`,
        // step 56, which it skips in menus as that step's gate does). The
        // `--use` test aid presses E for it first.
        .add_systems(
            Update,
            (scripts::start_use, scripts::run_scripts)
                // Kept: the test aid's E before the scripts read it.
                .chain()
                .in_set(frame_order::WorldSet::at(
                    frame_order::RUN_ACTOR_SCRIPTS,
                    frame_order::RUN_SCRIPT_AT,
                )),
        )
        // Right after the scripts: movies, which run inside the script
        // command that starts them (`PlayBink`'s handler `005d15d0` calls the
        // movie player, whose loop owns the screen until the movie ends; here
        // the clock stops instead, `movie`); and, placed for order only (no
        // sub-step of the stage is theirs, their per-frame place isn't
        // traced), E on load doors and the doors' swings, which scripts and E
        // start (their leaves are moved by `move_pieces`, at
        // `TES::RunAnimations` in the AI stage).
        .add_systems(
            Update,
            (
                (walk::doors, doors::update_doors).chain(),
                (movie::start_movies, movie::play_movies).chain(),
            )
                // Kept: the viewer's order among them.
                .chain()
                .in_set(FrameSet::Stage(Stage::WorldAndTime))
                .after(FrameSet::step(frame_order::RUN_ACTOR_SCRIPTS))
                .before(FrameSet::after(frame_order::RUN_ACTOR_SCRIPTS)),
        )
        // The sky: `Sky::Update`, which `TES::UpdateCellMainThread`
        // (`004537c0`) calls (`00453811`) from the main thread's
        // `Main::OnIdle_UpdateAnimationsAndEffects` (stage 6) when there are
        // AI threads (with one thread `TES::UpdateCellAnimations` calls it
        // too, `004536ae`). The weather's step (a new square's weather, the
        // transition at the game's hour) is in its sub-step, under its gates:
        // not in menu mode unless in dialogue (`0086fc72`-`0086fc97`). The
        // sky centred on the eye, the daylight and the glows follow it outside
        // the gate: they also follow the camera, which moves in menus (the
        // V.A.T.S. and dialogue views), and change only when the clock or the
        // weather has moved.
        .add_systems(
            Update,
            (
                weather::run_weather.in_set(frame_order::AiSet::at(
                    frame_order::UPDATE_CELL_MAIN_THREAD,
                    frame_order::SKY_UPDATE_AT,
                )),
                (
                    follow_sky,
                    daylight::follow_the_clock,
                    emittance::arrive_indoors,
                    emittance::follow_emittance,
                )
                    // Kept: the viewer's order among them.
                    .chain()
                    .in_set(FrameSet::Stage(Stage::AiStart))
                    .after(frame_order::AiSet::at(
                        frame_order::UPDATE_CELL_MAIN_THREAD,
                        frame_order::SKY_UPDATE_AT,
                    ))
                    .before(FrameSet::after(frame_order::UPDATE_ANIMATIONS_AND_EFFECTS)),
            ),
        )
        // The land around the player: `Main::OnIdle_UpdateCurrentGridCell`
        // (step 78), whose `TES::UpdateCurrentGridCell` moves the exterior
        // grid with the player once the player's update has moved them. The
        // squares stream at `GridCellArray::SetCenter` (the cells detached
        // and attached around the new centre), the distant land and objects
        // at `BGSTerrainManager::Update`; both outside their gates (the grid
        // update stops in menu mode, and the terrain update runs only on a
        // grid move): the viewer's squares also finish loading a place just
        // entered, which the exe does at once in the position request
        // (`PlayerCharacter::HandlePositionPlayerRequest`) in any mode, and
        // its distant land follows the camera every frame.
        .add_systems(
            Update,
            (
                exterior::stream_squares
                    .after(frame_order::WorldSet::at(
                        frame_order::UPDATE_CURRENT_GRID_CELL,
                        frame_order::GRID_SET_CENTER_AT,
                    ))
                    .before(frame_order::WorldSet::after(
                        frame_order::UPDATE_CURRENT_GRID_CELL,
                        frame_order::GRID_SET_CENTER_AT,
                    )),
                (
                    exterior::stream_distant_land,
                    lod_objects::stream_distant_objects,
                )
                    .chain()
                    .after(frame_order::WorldSet::at(
                        frame_order::UPDATE_CURRENT_GRID_CELL,
                        frame_order::TERRAIN_UPDATE_AT,
                    ))
                    .before(frame_order::WorldSet::after(
                        frame_order::UPDATE_CURRENT_GRID_CELL,
                        frame_order::TERRAIN_UPDATE_AT,
                    )),
            )
                // Kept: each loader works on what the one before put in
                // place (the squares, then the distant land under them).
                .chain()
                .in_set(FrameSet::step(frame_order::UPDATE_CURRENT_GRID_CELL_STEP)),
        )
        // Stage 6, the AI work: on the AI linear task threads with threads
        // > 1, as `main` leaves the PC (`world::frame::ai_stage`,
        // `frame_order::AiSet`, docs/FRAME_SKELETON.md "PR 6 result"). Placed
        // for order only, ahead of the threads' start (their places in the
        // frame aren't traced): map markers, the HUD's quest targets, saving,
        // companions coming along, and people and things scripts enabled or
        // made coming on screen (before anyone moves).
        .add_systems(
            Update,
            (
                map::find_markers,
                hud::follow_quest_targets,
                (bring_in_enabled, bring_in_people).chain(),
                bring_in_made,
                scripts::save_and_load,
            )
                .in_set(FrameSet::Stage(Stage::AiStart))
                .before(FrameSet::step(frame_order::START_THREADS))
                // (and before what is ordered at the threads' calls from
                // outside their sets)
                .before(frame_order::AiSet::Call(frame_order::INTERFACE_IDLE)),
        )
        // The actors' animation updates (`ProcessLists::RunActorAnimationUpdates`,
        // `0096cca0`, a lead: each processed actor's slot +0x268), the
        // threads' work before anyone moves: the idles scripts and lines
        // asked for, the head tracking, then each actor's animations run on
        // and their joints posed (and the dead's ragdolls, whose exe pass,
        // `0096cb50`, comes after the actor updates). At the call but outside
        // its gate: `animate_actors` and the AI hold still in menu mode
        // themselves, except the dialogue menu's speaker, who turns to face
        // the player (the exe runs no AI threads in menu mode; how its
        // dialogue menu moves the speaker isn't traced).
        .add_systems(
            Update,
            (
                look::set_up,
                sitting::idle_requests,
                look::follow_player,
                actors::script_idles,
                actors::animate_actors,
            )
                // Kept: the viewer's order among them.
                .chain()
                .in_set(FrameSet::Stage(Stage::AiStart))
                .after(frame_order::AiSet::Call(
                    frame_order::ACTOR_ANIMATION_UPDATES,
                ))
                .before(frame_order::AiSet::next(
                    frame_order::ACTOR_ANIMATION_UPDATES,
                )),
        )
        // People redrawn where what they wear or hold changed, once posed
        // (their new face pieces then get their faces).
        .init_resource::<dress::StartWorn>()
        .add_systems(
            Update,
            dress::redress
                // Kept: after the pose, before the faces.
                .after(actors::animate_actors)
                .before(faces::start_lines)
                .in_set(FrameSet::Stage(Stage::AiStart))
                .before(frame_order::AiSet::next(
                    frame_order::ACTOR_ANIMATION_UPDATES,
                )),
        )
        // Faces: lines' lip sync and blinking, once lines have started and
        // the bones have moved.
        .add_systems(
            Update,
            (
                faces::start_lines,
                faces::release_voices,
                faces::animate_faces,
                // The dialogue menu's view on the speaker, once their head
                // has moved (the menu's update, `00762950`, is the
                // interface's; kept with the head).
                dialogue::focus_camera,
            )
                .chain()
                // Kept: the faces move with the posed heads.
                .after(actors::animate_actors)
                .in_set(FrameSet::Stage(Stage::AiStart))
                .before(frame_order::AiSet::next(
                    frame_order::ACTOR_ANIMATION_UPDATES,
                )),
        )
        // Models' own animations (a ceiling fan's blades, the doors' leaves,
        // a script's `PlayGroup`): the cells' animations, `TES::RunAnimations`
        // (`00455640`, on the AI thread with threads > 1), under its gate: they
        // hold still in menu mode.
        .add_systems(
            Update,
            move_pieces.in_set(frame_order::AiSet::Call(frame_order::RUN_ANIMATIONS)),
        )
        // The movement (`ProcessLists::UpdateActorsMovement`, `0096db30`, a
        // lead): people walk their paths, use furniture, greet and talk
        // (`move_actors`, which also evaluates their packages, the threads'
        // `0096bcd0` before it, and puts their 3D where they moved,
        // `Actor::Update`'s `0088b150` after it; at the call but outside its
        // gate, as it handles menu mode and the dialogue menu itself).
        .add_systems(
            Update,
            (
                ai::move_actors
                    .after(frame_order::AiSet::prev(frame_order::ACTORS_MOVEMENT))
                    .before(frame_order::AiSet::Call(frame_order::ACTORS_MOVEMENT)),
                // A diagnostic, after the moves.
                ai::ground_log.after(ai::move_actors),
            )
                .in_set(FrameSet::Stage(Stage::AiStart)),
        )
        // The process list's projectiles and explosions move in the same pass
        // (`009bec10`, `009ae580` on its non-actor objects), under its gate:
        // people's shots once everyone has moved (they hold still in menu
        // mode).
        .add_systems(
            Update,
            fighting::resolve_shots
                .after(ai::move_actors)
                .in_set(frame_order::AiSet::Call(frame_order::ACTORS_MOVEMENT)),
        )
        // People out of sight walk on: `Main::PostSwapProcess` (stage 7)
        // calls `Main::OnIdle_UpdateProcessLists` (`0086f890`), whose
        // `ProcessLists::UpdateProcessLists` (`0096d810`) moves the lower
        // process levels (`0096b810`, `0096b470`, `0096b050`, the passes
        // `move_offstage` follows). At its sub-step under the process-list
        // gate, after the sound, radio and music updates (Phase 1 PR 7).
        .add_systems(
            Update,
            ai::move_offstage.in_set(frame_order::InterfaceSet::at(
                world::frame::interface::UPDATE_PROCESS_LISTS,
                0x0086_f905,
            )),
        )
        .add_systems(
            Update,
            sounds::play_sounds.in_set(frame_order::InterfaceSet::at(
                world::frame::interface::AUDIO_UPDATE,
                0x0086_f650,
            )),
        )
        // After the frame (`ViewerSet::AfterFrame`): the viewer's tools and
        // output. Billboards turn to the camera once it has moved; the
        // console key's switch between walking and flying takes effect the
        // next frame.
        .add_systems(
            Update,
            (
                walk::toggle_walking,
                adjust_exposure,
                take_screenshot,
                update_help,
                grab_cursor,
                report::report_key,
                face_camera,
                report_fps,
            )
                .in_set(ViewerSet::AfterFrame),
        )
        .add_plugins(ExecutorChoice)
        .add_plugins(ViewerHidden(hidden))
        .run();
}

/// `NV_BEVY_EXECUTOR=single` (a measurement setting of the viewer): the main world's frame
/// schedules run their systems one after another on the main thread (Bevy's
/// `ExecutorKind::SingleThreaded`) instead of handing them to the task pool, to compare the
/// multi-threaded executor's per-frame cost (a29: about 3 ms a frame of Bevy's own with an empty
/// cell, most of it the main thread waiting). Same systems, same order constraints.
/// Whether the viewer runs without its own window and scene: with the game's real window
/// (`NV_D3D9=real`) unless `NV_VIEWER_WINDOW=1`.
fn viewer_hidden() -> bool {
    let real = std::env::var("NV_D3D9").is_ok_and(|v| v.eq_ignore_ascii_case("real"));
    let keep = std::env::var("NV_VIEWER_WINDOW").is_ok_and(|v| v == "1");
    real && !keep
}

/// The viewer without its own view ([`viewer_hidden`]): every camera inactive, so the render
/// world draws nothing for the hidden window.
struct ViewerHidden(bool);

impl Plugin for ViewerHidden {
    fn build(&self, app: &mut App) {
        if self.0 {
            app.add_systems(Last, no_cameras);
        }
    }
}

/// Turns off every camera that is on (new ones each frame too).
fn no_cameras(mut cameras: Query<&mut Camera>) {
    for mut camera in &mut cameras {
        if camera.is_active {
            camera.is_active = false;
        }
    }
}

struct ExecutorChoice;

impl Plugin for ExecutorChoice {
    fn build(&self, app: &mut App) {
        if !std::env::var("NV_BEVY_EXECUTOR").is_ok_and(|v| v == "single") {
            return;
        }
        fn single(s: &mut Schedule) {
            s.set_executor_kind(bevy::ecs::schedule::ExecutorKind::SingleThreaded);
        }
        app.edit_schedule(First, single)
            .edit_schedule(PreUpdate, single)
            .edit_schedule(Update, single)
            .edit_schedule(PostUpdate, single)
            .edit_schedule(Last, single);
        println!("NV_BEVY_EXECUTOR=single: First to Last run on the main thread.");
    }
}

/// A piece its model's own animation moves or changes
/// (`cellview::MeshData::motion`): its draw's placement (Bevy's space),
/// the point it's built around (`sort_center`; the piece's own space, game
/// units), and, when the animation changes its material, its own material
/// and the values last put in it.
#[derive(Component)]
struct Moving {
    motion: Arc<cellview::PieceMotion>,
    base: Mat4,
    center: Vec3,
    material: Option<Handle<GameLitMaterial>>,
    shown: Option<(Option<f32>, Option<[f32; 3]>)>,
    /// A piece of a door that swings where it stands: its `Open` / `Close`
    /// sequence follows the door's state (`doors::DoorPoses`).
    door: Option<u32>,
}

/// Sequences scripts started on placed objects (`PlayGroup`): by placed
/// reference, the sequence's name and when it started (the viewer's
/// clock).
#[derive(Resource, Default)]
pub struct PlayingGroups(pub std::collections::HashMap<u32, (String, f32)>);

/// Puts moving pieces where their animation has them now, and their
/// material as it has it: the sequences their objects show from the start
/// (started when the viewer did; the game starts them when the object
/// appears), or the one a script started.
fn move_pieces(
    time: Res<Time>,
    groups: Res<PlayingGroups>,
    door_poses: Res<doors::DoorPoses>,
    mut pieces: Query<(&mut Moving, &scripts::PlacedRef, &mut Transform)>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
    mut dialogue: ResMut<crate::dialogue::DialogueState>,
) {
    let seconds = time.elapsed_secs();
    // The sequences active on each object, for `IsAnimPlaying`: a
    // script's group (while it plays: a one-shot that has ended stops
    // counting, though its last pose is held), a door's `Open` or
    // `Close`, or the ones its model plays from the start (those holding a
    // frame aren't counted: a guess).
    let mut active: std::collections::HashMap<esm::FormId, Vec<String>> =
        std::collections::HashMap::new();
    for (mut piece, placed, mut transform) in &mut pieces {
        let group = groups
            .0
            .get(&placed.0)
            .map(|(name, at)| (name.as_str(), seconds - at));
        // A door's piece shows its state's sequence at the door's moment
        // (`doors::update_doors`); a script's group still takes over.
        let door_pose = piece
            .door
            .filter(|_| group.is_none())
            .and_then(|d| door_poses.0.get(&d).copied());
        let names = active.entry(esm::FormId(placed.0)).or_default();
        let mut add = |name: &str| {
            if !names.iter().any(|n| n.eq_ignore_ascii_case(name)) {
                names.push(name.to_string());
            }
        };
        let motion = &piece.motion.motion;
        // A one-shot that has reached its end no longer counts as playing
        // (its pose is held, but `IsAnimPlaying` reads 0).
        let named = |name: &str| {
            motion
                .all
                .iter()
                .find(|s| s.name.eq_ignore_ascii_case(name))
        };
        match (group, door_pose) {
            (Some((name, since)), _) if named(name).is_some() => {
                if named(name).is_some_and(|s| preview::cell::sequence_playing(s, since)) {
                    add(name)
                }
            }
            (None, Some((opening, since))) => {
                let name = if opening { "Open" } else { "Close" };
                // A door model without that sequence still counts as it did.
                if named(name).is_none_or(|s| preview::cell::sequence_playing(s, since)) {
                    add(name)
                }
            }
            _ => {
                for p in motion.sequences.iter().filter(|p| p.runs) {
                    if preview::cell::sequence_playing(&p.sequence, seconds) {
                        add(&p.sequence.name);
                    }
                }
            }
        }
        let now = match door_pose {
            Some((opening, at)) => cellview::piece_in_sequence(
                &piece.motion,
                if opening { "Open" } else { "Close" },
                at,
            )
            .unwrap_or_else(|| cellview::piece_now(&piece.motion, seconds, group)),
            None => cellview::piece_now(&piece.motion, seconds, group),
        };
        let moved = Mat4::from_cols_array(&now.matrix);
        let wanted =
            Transform::from_matrix(piece.base * moved * Mat4::from_translation(piece.center));
        if *transform != wanted {
            *transform = wanted;
        }
        let shown = Some((now.opacity, now.emissive));
        if piece.shown == shown {
            continue;
        }
        let Some(material) = piece.material.as_ref().and_then(|h| materials.get_mut(h)) else {
            continue;
        };
        let opacity = now.opacity.unwrap_or(piece.motion.opacity);
        material.base.base_color.set_alpha(opacity);
        material.extension.lighting.environment.w = opacity;
        if let Some([r, g, b]) = now.emissive {
            let w = material.extension.lighting.emissive.w;
            material.extension.lighting.emissive = Vec4::new(r, g, b, w);
        }
        piece.shown = shown;
    }
    world::more_functions::report_sequences(&mut dialogue.0, active);
}

/// A piece that turns to face the camera (`cellview::MeshData::billboard`):
/// its draw's placement (game space, and Bevy's) and the point it's built
/// around.
#[derive(Component)]
struct Facing {
    billboard: cellview::Billboard,
    draw: [f32; 16],
    base: Mat4,
    center: Vec3,
}

/// Turns billboards toward the camera, after it has moved this frame.
fn face_camera(
    cameras: Query<&Transform, (With<FlyCamera>, Without<Facing>)>,
    mut pieces: Query<(&Facing, &mut Transform)>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    // Bevy's axes (y up, −z north, meters) back to the game's.
    let game = |v: Vec3| [v.x, -v.z, v.y];
    let eye = game(camera.translation / space::METERS_PER_UNIT);
    let axes = [
        game(camera.rotation * Vec3::X),
        game(camera.rotation * Vec3::Y),
        game(camera.rotation * Vec3::Z),
    ];
    for (piece, mut transform) in &mut pieces {
        let turned = Mat4::from_cols_array(&cellview::billboard_matrix(
            &piece.billboard,
            &piece.draw,
            eye,
            axes,
        ));
        *transform =
            Transform::from_matrix(piece.base * turned * Mat4::from_translation(piece.center));
    }
}

#[derive(Resource)]
pub(crate) struct Settings {
    pub(crate) brightness: f32,
    /// `--cloud-time`: the clouds held at this many seconds of drift.
    cloud_time: Option<f32>,
    /// The textures' anisotropic filtering, from the INI
    /// ([`anisotropy_setting`]).
    anisotropy: u16,
}

/// The game's texture filtering, from its INI: `[Display] iMaxAnisotropy`
/// (15 in this install's `FalloutPrefs.ini`, 8 in `Fallout_default.ini`).
/// The Goodsprings recording sets `D3DSAMP_MAXANISOTROPY` to it on every
/// scene texture stage (1,239 of the frame's 2,033 sampler-state calls),
/// `MINFILTER` anisotropic, `MAGFILTER` linear, `MIPFILTER` linear (the
/// HUD's none), `MIPMAPLODBIAS` 0: anisotropic trilinear, no bias, as the
/// textures here are sampled. Below 1 the card refuses it; 1 is plain
/// trilinear.
fn anisotropy_setting(ini: &assets::IniSettings) -> u16 {
    ini.get("Display", "iMaxAnisotropy")
        .and_then(|v| v.trim().parse::<u16>().ok())
        .unwrap_or(8)
        .max(1)
}

/// Starting pitch, radians, positive looking up; used once.
#[derive(Resource)]
pub(crate) struct StartPitch(pub(crate) f32);

/// The game's files, open for the whole run (and shared with the threads
/// loading outdoor squares).
#[derive(Resource)]
pub struct GameFiles(pub Arc<Game>);

/// Where to start: an interior (or one outdoor cell's objects) loaded
/// now, or an outdoor place whose squares load once the window is open.
fn open_place(
    game: &Game,
    name: &str,
    at: Option<args::Stance>,
) -> Result<(Option<ViewerScene>, Option<ExteriorStart>), String> {
    // A placed reference by editor ID (a marker, say): start standing
    // where it stands, facing its way, in its cell or worldspace.
    let placed = game
        .order
        .form_by_editor_id(name)
        .filter(|&r| {
            game.order.get(r).is_some_and(|rr| {
                matches!(rr.entry.header.kind.as_bytes(), b"REFR" | b"ACHR" | b"ACRE")
            })
        })
        .and_then(|r| world::scripting::whereabouts(&game.order, r));
    if let (Some(w), None) = (placed, at) {
        let stance = args::Stance {
            feet: w.position,
            heading: w.heading.to_degrees(),
            pitch: 0.0,
        };
        let space = game
            .order
            .get(w.world.unwrap_or(w.cell))
            .and_then(|r| r.editor_id().ok().flatten())
            .ok_or_else(|| format!("{name}'s cell has no editor ID to open"))?;
        println!("Starting at {name} in {space}.");
        return open_place(game, &space, Some(stance));
    }
    let heading = at.map_or(0.0, |a| a.heading.to_radians());
    // A worldspace by name: start where --at says, else in the middle of
    // square 0,0.
    if let Ok(grid) = game.world(name) {
        let (feet, height) = match at {
            Some(a) => ([a.feet[0], a.feet[1]], Some(a.feet[2])),
            None => ([2048.0, 2048.0], None),
        };
        return Ok((
            None,
            Some(ExteriorStart {
                grid,
                feet,
                height,
                heading,
            }),
        ));
    }
    let cell = game.find_cell(name).map_err(|e| e.0)?;
    let info = world::cell_info(&game.order, cell).map_err(|e| e.to_string())?;
    if let (false, Some(world), Some((x, y))) = (info.interior, info.world, info.grid) {
        // An outdoor cell: its worldspace, starting in the middle of it.
        let grid = world::WorldGrid::load(&game.order, world).map_err(|e| e.to_string())?;
        let size = world::land::CELL_SIZE;
        let (feet, height) = match at {
            Some(a) => ([a.feet[0], a.feet[1]], Some(a.feet[2])),
            None => ([(x as f32 + 0.5) * size, (y as f32 + 0.5) * size], None),
        };
        return Ok((
            None,
            Some(ExteriorStart {
                grid,
                feet,
                height,
                heading,
            }),
        ));
    }
    let mut scene = game.load_cell(cell).map_err(|e| e.0)?;
    if let Some(at) = at {
        let [x, y, z] = at.feet;
        scene.start = cellview::Start {
            eye: [x, y, z + cellview::EYE_HEIGHT],
            heading,
            via: "the position given with --at",
        };
    }
    Ok((Some(scene), None))
}

/// How far outdoors people coming into the worldspace are drawn: the
/// loaded squares (two and a half squares each way).
const BRING_IN_REACH: f32 = 2.5 * world::land::CELL_SIZE;

/// People the game has taken into this place after it loaded (through a
/// door, or by a script's `MoveTo`) come on screen: once a second, anyone
/// the state has here (`world::ai::moved_into`) who isn't drawn yet is
/// spawned, lit as the place is; outdoors, those within the loaded
/// squares. They join the place's people ([`dialogue::Talkers`]: talking,
/// trigger volumes, combat), and are put back among them when an outdoor
/// square load rebuilds that list from the squares' own people. Without
/// this Sunny Smiles, walking out of the saloon for `VCG02`, never counted
/// in `VCG02SunnyPatrolTrigger` (its `OnTrigger SunnyREF` sets stage 20).
/// Also whom a script enabled after their square loaded
/// (`world::ai::enabled_since_load`): `GoodspringsPowderGangMarker.Enable`
/// brings in the Powder Gangers following it, whose marker isn't loaded
/// here, which [`bring_in_enabled`] (run first) doesn't cover.
#[allow(clippy::too_many_arguments)]
fn bring_in_people(
    time: Res<Time>,
    game: Res<GameFiles>,
    state: Res<dialogue::DialogueState>,
    player: Res<walk::Player>,
    walkers: Query<(&ai::Walker, &Visibility)>,
    exterior: Option<Res<exterior::Exterior>>,
    (mut talkers, mut brought): (ResMut<dialogue::Talkers>, ResMut<BroughtIn>),
    mut spawner: Spawner,
    mut last: Local<f32>,
) {
    let now = time.elapsed_secs();
    if !player.ready || now - *last < 1.0 {
        return;
    }
    *last = now;
    let state = &state.0;
    let order = &game.0.order;
    let Some(space) = state.player_world.or(state.player_cell) else {
        return;
    };
    let shown: std::collections::HashSet<esm::FormId> =
        walkers.iter().map(|(w, _)| w.reference).collect();
    let moved = world::ai::moved_into(order, state, space);
    add_talkers(
        &mut talkers.0,
        walkers
            .iter()
            .filter(|(w, v)| **v != Visibility::Hidden && moved.contains(&w.reference))
            .map(|(w, _)| dialogue::Talker {
                reference: w.reference,
                base: world::scripting::base_of(order, w.reference).unwrap_or(w.reference),
                position: w.position,
            }),
    );
    let near = |r: esm::FormId| {
        state.player_world.is_none()
            || state
                .place(order, r)
                .zip(state.player_position)
                .is_some_and(|((_, _, p, _), me)| {
                    (p[0] - me[0]).hypot(p[1] - me[1]) <= BRING_IN_REACH
                })
    };
    let enabled = exterior
        .as_deref()
        .map(|e| world::ai::enabled_since_load(order, state, space, &e.disabled_people()))
        .unwrap_or_default();
    let mut new: Vec<esm::FormId> = moved
        .into_iter()
        .chain(enabled)
        .filter(|r| !shown.contains(r) && near(*r))
        .collect();
    new.sort();
    new.dedup();
    if new.is_empty() {
        return;
    }
    let scene = game.0.actors_scene(&new);
    let lighting = spawner.place_lighting.get();
    for r in &new {
        println!("{r} comes into view");
    }
    add_talkers(
        &mut talkers.0,
        scene.actors.iter().map(dialogue::Talker::from_actor),
    );
    spawner.spawn_with(&scene, lighting);
    brought.remember(space, scene.actors.iter().map(|a| esm::FormId(a.reference)));
}

impl BroughtIn {
    /// People put on screen in `space` after its squares loaded: they can be
    /// talked to, shot and targeted like the place's own people.
    fn remember(&mut self, space: esm::FormId, people: impl IntoIterator<Item = esm::FormId>) {
        if self.space != Some(space) {
            *self = BroughtIn {
                space: Some(space),
                people: Default::default(),
            };
        }
        self.people.extend(people);
    }
}

/// The people [`bring_in_people`] put on screen in the current place (not
/// part of a loaded square or cell): outdoors they stay among the talkers
/// when the loaded squares change (`exterior::stream_squares`).
#[derive(Resource, Default)]
pub struct BroughtIn {
    space: Option<esm::FormId>,
    pub people: std::collections::HashSet<esm::FormId>,
}

/// References a script's `Enable` / `Disable` changed after their place
/// loaded: those of the loaded place now shown (themselves, or through
/// their enable parent: `world::newly_enabled`) come on screen, drawn if
/// they were left out when it loaded (people join its people); drawn ones
/// now hidden through their parent are hidden. Their own `Enable` is
/// carried out by `run_scripts`; this covers what wasn't drawn and the
/// children (`VCG02BottleMarkerREF.Enable` shows the tutorial's bottles,
/// `VCG02Gecko1REF.Enable` brings in a gecko left out at load). The game
/// loads an enabled reference's 3D (`005c43d0` → `005aa5d0`).
#[allow(clippy::too_many_arguments)]
fn bring_in_enabled(
    game: Res<GameFiles>,
    state: Res<dialogue::DialogueState>,
    player: Res<walk::Player>,
    here: Res<scripts::Here>,
    exterior: Option<Res<exterior::Exterior>>,
    mut placed: Query<(&scripts::PlacedRef, &mut Visibility)>,
    (mut talkers, mut brought): (ResMut<dialogue::Talkers>, ResMut<BroughtIn>),
    mut spawner: Spawner,
    mut seen: Local<EnableSeen>,
) {
    let state = &state.0;
    if !player.ready {
        return;
    }
    let space = state.player_world.or(state.player_cell);
    let mut squares: Vec<(i32, i32)> = match (&exterior, state.player_world) {
        (Some(e), Some(_)) => e.loaded_squares().into_iter().collect(),
        _ => Vec::new(),
    };
    squares.sort();
    // Looked at again when scripts enable or disable something, and when
    // the place or its loaded squares change (a square that was loading
    // while a script enabled something may have left it out).
    let place = (space, squares);
    let moved_on = seen.place != place;
    if !moved_on && seen.disabled == state.disabled {
        return;
    }
    let before = std::mem::replace(&mut seen.disabled, state.disabled.clone());
    seen.place = place;
    let order = &game.0.order;
    let refs: Vec<esm::FormId> = match (&exterior, state.player_world, here.0) {
        (Some(e), Some(_), _) => seen
            .place
            .1
            .iter()
            .flat_map(|&square| {
                let mut refs: Vec<esm::FormId> = e
                    .grid
                    .cell_at(square)
                    .map(|c| {
                        order
                            .references_in_cell(c)
                            .into_iter()
                            .map(|rr| rr.form_id)
                            .collect()
                    })
                    .unwrap_or_default();
                refs.extend_from_slice(e.grid.persistent_in(square));
                refs
            })
            .collect(),
        (_, None, Some(cell)) => order
            .references_in_cell(esm::FormId(cell))
            .into_iter()
            .map(|rr| rr.form_id)
            .collect(),
        _ => return,
    };
    // Drawn ones whose state changed since the last look (only then, so
    // people hidden for being elsewhere stay hidden).
    let (came, went) = if moved_on {
        (Vec::new(), Vec::new())
    } else {
        (
            world::newly_enabled(order, refs.iter().copied(), &before, &state.disabled),
            world::newly_enabled(order, refs.iter().copied(), &state.disabled, &before),
        )
    };
    let mut drawn = std::collections::HashSet::new();
    for (p, mut visibility) in &mut placed {
        let r = esm::FormId(p.0);
        drawn.insert(r);
        if came.contains(&r) {
            *visibility = Visibility::Inherited;
        } else if went.contains(&r) {
            *visibility = Visibility::Hidden;
        }
    }
    // Shown now but left out when loaded (as the data has them at first),
    // still here and not drawn.
    let shown_now = world::newly_enabled(
        order,
        refs.iter().copied(),
        &world::Disabled::new(),
        &state.disabled,
    );
    let new: Vec<world::Placement> = shown_now
        .iter()
        .filter(|r| !drawn.contains(r) && !state.dead.contains(r))
        .filter(|&&r| state.place(order, r).is_some_and(|p| Some(p.0) == space))
        .filter_map(|&r| world::placement_of(order, r))
        .filter(|p| !world::is_marker(p.base, p.base_type, p.model.as_deref()))
        .collect();
    if new.is_empty() {
        return;
    }
    for p in &new {
        println!(
            "{} ({}) comes into view (enabled)",
            p.form_id,
            p.base_editor_id.as_deref().unwrap_or("?")
        );
    }
    let scene = game.0.made_scene(new);
    add_talkers(
        &mut talkers.0,
        scene.actors.iter().map(dialogue::Talker::from_actor),
    );
    // Kept among the place's people when its loaded squares change
    // (`exterior::stream_squares` rebuilds them from the squares, which
    // left these out while they were disabled): before, the Powder Gangers
    // enabled for Ghost Town Gunfight dropped out at the next square change
    // and shots passed through them.
    if let Some(space) = space {
        brought.remember(space, scene.actors.iter().map(|a| esm::FormId(a.reference)));
    }
    let lighting = spawner.place_lighting.get();
    spawner.spawn_with(&scene, lighting);
}

/// What [`bring_in_enabled`] last looked at: the place (interior cell or
/// worldspace, and the loaded squares) and the scripts' enable state.
#[derive(Default)]
struct EnableSeen {
    place: (Option<esm::FormId>, Vec<(i32, i32)>),
    disabled: world::Disabled,
}

/// Adds people to the place's people, each once.
fn add_talkers(
    talkers: &mut Vec<dialogue::Talker>,
    people: impl IntoIterator<Item = dialogue::Talker>,
) {
    for p in people {
        if !talkers.iter().any(|t| t.reference == p.reference) {
            talkers.push(p);
        }
    }
}

/// References scripts made (`PlaceAtMe`, `world::more_functions::placed`)
/// come on screen: once a second, those here not drawn yet and enabled,
/// people and creatures and anything with a model, lit as the place is;
/// outdoors, those within the loaded squares. A place loaded afresh draws
/// them again.
fn bring_in_made(
    time: Res<Time>,
    game: Res<GameFiles>,
    state: Res<dialogue::DialogueState>,
    player: Res<walk::Player>,
    mut spawner: Spawner,
    mut shown: Local<(Option<esm::FormId>, std::collections::HashSet<esm::FormId>)>,
    mut last: Local<f32>,
) {
    let now = time.elapsed_secs();
    if !player.ready || now - *last < 1.0 {
        return;
    }
    *last = now;
    let state = &state.0;
    let order = &game.0.order;
    let Some(space) = state.player_world.or(state.player_cell) else {
        return;
    };
    if shown.0 != Some(space) {
        *shown = (Some(space), Default::default());
    }
    let mut new = Vec::new();
    for (&r, made) in &state.more.placed.refs {
        if shown.1.contains(&r) || !world::enabled_now(order, r, &state.disabled) {
            continue;
        }
        let Some((s, _, p, heading)) = state.place(order, r) else {
            continue;
        };
        let far = state.player_world.is_some()
            && state
                .player_position
                .is_some_and(|me| (p[0] - me[0]).hypot(p[1] - me[1]) > BRING_IN_REACH);
        if s != space || far {
            continue;
        }
        shown.1.insert(r);
        let rotation = [made.rotation[0], made.rotation[1], heading];
        if let Some(placement) = world::made_placement(order, r, made.base, p, rotation) {
            new.push(placement);
        }
    }
    if new.is_empty() {
        return;
    }
    for p in &new {
        println!(
            "{} ({}) comes into view",
            p.form_id,
            p.base_editor_id.as_deref().unwrap_or("?")
        );
    }
    let scene = game.0.made_scene(new);
    let lighting = spawner.place_lighting.get();
    spawner.spawn_with(&scene, lighting);
}

/// `--screenshot`: where to save the picture, and how long it's waited.
#[derive(Resource)]
struct ScreenshotRequest {
    path: Option<std::path::PathBuf>,
    frames: u32,
    taken: bool,
    /// `--wait`: seconds to let the game run first, and how long it has.
    wait: f32,
    waited: f32,
}

/// Frames to wait before the screenshot, so every shader has compiled and
/// every texture has reached the graphics card.
const SCREENSHOT_AFTER_FRAMES: u32 = 300;

/// What screenshots hide: the help and the health line.
type KeptOutOfPictures = Or<(With<HelpText>, With<combat::HudText>)>;

fn take_screenshot(
    // Real time: V.A.T.S. stops the game's clock.
    time: Res<Time<bevy::time::Real>>,
    mut commands: Commands,
    mut request: ResMut<ScreenshotRequest>,
    exterior: Option<Res<exterior::Exterior>>,
    // The help and the health line stay out of pictures compared with
    // the game's.
    mut help: Query<&mut Visibility, KeptOutOfPictures>,
    flash: Res<weapon_fx::WeaponEffects>,
    impacts: Res<impact_fx::ImpactFx>,
) {
    let Some(path) = request.path.clone() else {
        return;
    };
    for mut visibility in &mut help {
        *visibility = Visibility::Hidden;
    }
    // Outdoors, the count starts once the squares around have loaded.
    if exterior.is_some_and(|e| e.busy()) {
        request.frames = 0;
        return;
    }
    request.frames += 1;
    request.waited += time.delta_secs();
    if request.taken || request.frames < SCREENSHOT_AFTER_FRAMES || request.waited < request.wait {
        return;
    }
    // `NV_SHOT_ON_FLASH=player` (or `npc`): after that, the picture waits
    // for a muzzle flash on screen (`weapon_fx`), to check one by eye.
    if let Ok(which) = std::env::var("NV_SHOT_ON_FLASH") {
        let want = which.eq_ignore_ascii_case("player");
        if flash.shown != Some(want) {
            return;
        }
    }
    // `NV_SHOT_ON_IMPACT=1`: it waits for an impact's effect model on
    // screen (`impact_fx`).
    if std::env::var("NV_SHOT_ON_IMPACT").is_ok() && !impacts.shown {
        return;
    }
    request.taken = true;
    println!("Saving the view to {}", path.display());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path))
        .observe(
            |_: Trigger<ScreenshotCaptured>, mut exit: EventWriter<AppExit>| {
                exit.write(AppExit::Success);
            },
        );
}

/// The cell's image space values, and whether its color adjustment is
/// shown (G toggles; bloom stays on).
#[derive(Resource)]
struct Grading {
    grade: ImageSpaceGrade,
    on: bool,
}

/// The loaded cell, until `setup` hands it to Bevy.
#[derive(Resource)]
struct PendingScene(Option<ViewerScene>);

#[derive(Component)]
pub(crate) struct FlyCamera {
    yaw: f32,
    pitch: f32,
    /// Meters per second.
    speed: f32,
    start: (Vec3, f32),
}

#[derive(Component)]
struct HelpText;

/// Every texture is sampled as stored, colors too: the game's samplers
/// never decode sRGB (no `D3DSAMP_SRGBTEXTURE` anywhere in the Goodsprings
/// recording), so filtering averages the stored values and its shaders
/// work on them; the viewer's shaders do the same.
fn gpu_format(format: GpuFormat) -> TextureFormat {
    match format {
        GpuFormat::Bc1 => TextureFormat::Bc1RgbaUnorm,
        GpuFormat::Bc2 => TextureFormat::Bc2RgbaUnorm,
        GpuFormat::Bc3 => TextureFormat::Bc3RgbaUnorm,
        GpuFormat::Bc4 => TextureFormat::Bc4RUnorm,
        GpuFormat::Bc5 => TextureFormat::Bc5RgUnorm,
        GpuFormat::Rgba8 => TextureFormat::Rgba8Unorm,
    }
}

/// A place's ambient light, directional light and fog as the lighting
/// shader takes them (Bevy's space, `--brightness` applied).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LightFields {
    pub ambient: Vec4,
    pub directional_color: Vec4,
    pub directional_direction: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
}

pub(crate) fn light_fields(
    ambient: [f32; 3],
    directional: Option<&cellview::Directional>,
    fog: Option<&cellview::Fog>,
    brightness: f32,
) -> LightFields {
    let rgb = |c: [f32; 3], w: f32| Vec4::new(c[0], c[1], c[2], w);
    let (directional_color, directional_direction) = match directional {
        Some(d) => (
            rgb(d.color.map(|c| c * brightness), 0.0),
            rgb(space::direction(d.direction), 0.0),
        ),
        None => (Vec4::ZERO, Vec4::Y),
    };
    let (fog_color, fog_range) = match fog {
        Some(f) => (
            rgb(f.color, f.power),
            Vec4::new(
                f.near * space::METERS_PER_UNIT,
                f.far * space::METERS_PER_UNIT,
                cellview::GAME_NEAR_CLIP * space::METERS_PER_UNIT,
                1.0,
            ),
        ),
        None => (Vec4::ZERO, Vec4::ZERO),
    };
    LightFields {
        ambient: rgb(ambient.map(|c| c * brightness), 0.0),
        directional_color,
        directional_direction,
        fog_color,
        fog_range,
    }
}

impl LightFields {
    /// Puts these into a material's lighting, keeping its own lights.
    pub fn apply(&self, l: &mut GameLighting) {
        l.ambient = self.ambient;
        l.directional_color = self.directional_color;
        l.directional_direction = self.directional_direction;
        l.fog_color = self.fog_color;
        l.fog_range = self.fog_range;
    }
}

/// The cell's lights as the lighting shader takes them, in Bevy's space,
/// scaled by `--brightness`. The first `MAX_LIGHTS` are used.
fn game_lighting(scene: &ViewerScene, brightness: f32) -> GameLighting {
    let lights = game_lights(&scene.lights, brightness);
    let f = light_fields(
        scene.ambient,
        scene.directional.as_ref(),
        scene.fog.as_ref(),
        brightness,
    );
    GameLighting {
        fog_color: f.fog_color,
        fog_range: f.fog_range,
        specular: Vec4::ZERO,
        surface: Vec4::ZERO,
        falloff: NO_FALLOFF,
        environment: Vec4::ZERO,
        // The pull decals get here (kept only on decals' materials): the
        // game's projection differs indoors and out.
        draw: Vec4::new(
            lighting::decal_depth_offset(
                CAMERA_NEAR_METERS,
                if scene.sky.is_none() {
                    cellview::INTERIOR_DEPTH_SCALE
                } else {
                    cellview::EXTERIOR_DEPTH_SCALE
                },
            ),
            0.0,
            0.0,
            0.0,
        ),
        ambient: f.ambient,
        directional_color: f.directional_color,
        directional_direction: f.directional_direction,
        emissive: Vec4::ZERO,
        scale: Vec4::new(
            full_brightness_nits(),
            scene.lights.len().min(MAX_LIGHTS) as f32,
            0.0,
            0.0,
        ),
        lights,
        // Skin's directional light multiplier, from the image space.
        actor: Vec4::new(
            0.0,
            scene.hdr.as_ref().map_or(1.0, |h| h.skin_directional),
            0.0,
            0.0,
        ),
        hair_tint: Vec4::ZERO,
    }
}

/// Placed lights as the lighting shader takes them (Bevy's space,
/// `--brightness` applied); the first `MAX_LIGHTS`.
fn game_lights(list: &[cellview::LightData], brightness: f32) -> [GameLight; MAX_LIGHTS] {
    let mut lights = [GameLight::default(); MAX_LIGHTS];
    for (slot, light) in lights.iter_mut().zip(list) {
        let [x, y, z] = space::point(light.position);
        let [r, g, b] = light
            .color
            .map(|c| c * light.fade.clamp(0.0, 4.0) * brightness);
        *slot = GameLight {
            position_radius: Vec4::new(x, y, z, light.radius * space::METERS_PER_UNIT),
            color: Vec4::new(r, g, b, 0.0),
        };
    }
    lights
}

/// A material lit by placed lights.
trait PlaceLit: Asset {
    fn lighting(&self) -> &GameLighting;
    fn lighting_mut(&mut self) -> &mut GameLighting;
}

impl PlaceLit for GameLitMaterial {
    fn lighting(&self) -> &GameLighting {
        &self.extension.lighting
    }
    fn lighting_mut(&mut self) -> &mut GameLighting {
        &mut self.extension.lighting
    }
}

impl PlaceLit for TerrainMaterial {
    fn lighting(&self) -> &GameLighting {
        &self.extension.lighting
    }
    fn lighting_mut(&mut self) -> &mut GameLighting {
        &mut self.extension.lighting
    }
}

/// Gives these materials placed lights (`count` of them used), leaving
/// alone those that have them already: changing a material (`get_mut`)
/// has the render world prepare it again, the same as it was.
fn give_lights<M: PlaceLit>(
    materials: &mut Assets<M>,
    handles: &[Handle<M>],
    lights: [GameLight; MAX_LIGHTS],
    count: f32,
) {
    for handle in handles {
        let has = |l: &GameLighting| l.lights == lights && l.scale.y == count;
        if materials.get(handle).is_some_and(|m| !has(m.lighting())) {
            if let Some(m) = materials.get_mut(handle) {
                let l = m.lighting_mut();
                l.lights = lights;
                l.scale.y = count;
            }
        }
    }
}

/// A fade by viewing angle that never fades: opacity 1 at every angle.
const NO_FALLOFF: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);

/// The point a blended piece is sorted by ([`cellview::MaterialData::
/// sort_center`]), or `None` for pieces that aren't sorted (opaque ones,
/// and actors' pieces, which are skinned). Bevy sorts blended entities by
/// their origin, so such a piece is built around this point
/// ([`game_mesh_around`]) and placed with it added back.
fn sort_center(data: &cellview::MeshData) -> Option<[f32; 3]> {
    let sorted = matches!(
        alpha_mode(data.material.blend),
        AlphaMode::Blend | AlphaMode::Add | AlphaMode::Multiply | AlphaMode::Premultiplied
    );
    (sorted && data.rig.is_none()).then_some(data.material.sort_center)
}

/// A mesh for Bevy, drawn unindexed: each triangle gets its own three
/// vertices, and every vertex carries its triangle's three corners, so the
/// shader can work things out per vertex and blend them across the
/// triangle as the game does (see `lighting::ATTRIBUTE_CORNER_A`). The
/// mesh's origin is `origin` (in the piece's own space).
fn game_mesh_around(data: &cellview::MeshData, origin: [f32; 3]) -> Mesh {
    let moved: Vec<[f32; 3]>;
    let positions = if origin == [0.0; 3] {
        &data.positions
    } else {
        moved = data
            .positions
            .iter()
            .map(|p| [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]])
            .collect();
        &moved
    };
    let triangles: Vec<[usize; 3]> = data
        .indices
        .as_chunks::<3>()
        .0
        .iter()
        .map(|t| t.map(usize::from))
        .filter(|t| t.iter().all(|&i| i < data.positions.len()))
        .collect();
    fn spread<T: Copy>(values: &[T], triangles: &[[usize; 3]]) -> Vec<T> {
        triangles.iter().flatten().map(|&i| values[i]).collect()
    }
    let corner = |k: usize| -> Vec<[f32; 3]> {
        triangles
            .iter()
            .flat_map(|t| [positions[t[k]]; 3])
            .collect()
    };
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, spread(positions, &triangles));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, spread(&data.normals, &triangles));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, spread(&data.uvs, &triangles));
    if let Some(tangents) = &data.tangents {
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, spread(tangents, &triangles));
    }
    if let Some(colors) = &data.colors {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, spread(colors, &triangles));
    }
    mesh.insert_attribute(lighting::ATTRIBUTE_CORNER_A, corner(0));
    mesh.insert_attribute(lighting::ATTRIBUTE_CORNER_B, corner(1));
    mesh.insert_attribute(lighting::ATTRIBUTE_CORNER_C, corner(2));
    mesh
}

/// Bevy's blending for a surface. The alpha test itself is the game's
/// exact comparison, done in `game_lit.wgsl` (`GameLighting::draw`), so
/// cut-outs are opaque here and surfaces that test and blend just blend.
fn alpha_mode(blend: Blend) -> AlphaMode {
    match blend {
        Blend::Opaque | Blend::Mask(_) => AlphaMode::Opaque,
        Blend::Blend | Blend::MaskedBlend(_) => AlphaMode::Blend,
        Blend::Add => AlphaMode::Add,
        Blend::Multiply => AlphaMode::Multiply,
        Blend::Premultiplied => AlphaMode::Premultiplied,
    }
}

/// Everything drawn for the current cell, removed when another loads.
#[derive(Component)]
struct SceneEntity;

/// The camera and the on-screen text, once.
fn setup(mut commands: Commands) {
    let camera = commands.spawn((
        Camera3d::default(),
        // HDR, so bright lamps can go past white into the game's bloom
        // (see `grade`). The picture is finished and written out by the
        // first-person camera, which draws over it (`viewmodel`).
        Camera {
            hdr: true,
            output_mode: CameraOutputMode::Skip,
            ..default()
        },
        // The game clips what's too bright rather than rolling it off, and
        // the lighting shader already gives the game's brightness.
        Tonemapping::None,
        // No light clusters: every surface is lit by the game's lights in
        // its own shader, never Bevy's (there are none), so working out
        // which of them reach each cluster is wasted.
        bevy::pbr::ClusterConfig::None,
        Projection::from(PerspectiveProjection {
            // The game's: its 75° setting is the width of a 4:3 picture,
            // and wider windows keep that height.
            fov: cellview::vertical_fov(cellview::GAME_FOV_DEGREES),
            near: CAMERA_NEAR_METERS,
            // Outdoors, distant land reaches the game's far clip plane
            // (about 5 km); the sky dome is beyond it. (Only for culling:
            // Bevy's projection has no far plane.)
            far: 6000.0,
            ..default()
        }),
        Exposure { ev100: START_EV100 },
        Transform::default(),
        FlyCamera {
            yaw: 0.0,
            pitch: 0.0,
            speed: 2.5,
            start: (Vec3::ZERO, 0.0),
        },
        // Always: the pass also does the bloom and turns the stored values
        // the lighting writes into linear light for the screen. Set here,
        // run on the first-person camera.
        ImageSpaceGrade::NEUTRAL,
        grade::GradeDeferred,
    ));
    let camera = camera.id();
    viewmodel::spawn_camera(&mut commands, camera, Exposure { ev100: START_EV100 });
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            padding: UiRect::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        HelpText,
    ));
}

/// What putting a loaded place on screen needs.
#[derive(SystemParam)]
pub struct Spawner<'w, 's> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'w, Assets<Mesh>>,
    lit_materials: ResMut<'w, Assets<GameLitMaterial>>,
    terrain_materials: ResMut<'w, Assets<TerrainMaterial>>,
    lod_materials: ResMut<'w, Assets<LodLandMaterial>>,
    inverse_bindposes:
        ResMut<'w, Assets<bevy::render::mesh::skinning::SkinnedMeshInverseBindposes>>,
    images: ResMut<'w, Assets<Image>>,
    device: Option<Res<'w, RenderDevice>>,
    settings: Res<'w, Settings>,
    place_lighting: ResMut<'w, viewmodel::PlaceLighting>,
    water: water::WaterSpawn<'w>,
    particle_materials: ResMut<'w, Assets<particles::ParticleMaterial>>,
    uploaded: ResMut<'w, UploadedTextures>,
    remade_lit: ResMut<'w, daylight::RemadeLit>,
}

/// What a place's texture on the GPU is: the file (or what it was made
/// from: a face's tint and a hair's layer map are in the name, see
/// `cellview::TextureData::plus_face_tint`), read as data or colour, its
/// layers, compressed or not, the anisotropy.
type TextureKey = (String, bool, u32, bool, u16);

/// The places' textures on the GPU by what they are ([`TextureKey`]), so a
/// texture another loaded place already has isn't converted and sent
/// again (outdoor squares share most of theirs). Only weak references to
/// the handles the places hold: a texture goes when no place holds it any
/// more, as before, and is sent again if it's wanted after.
///
/// The handle given out is the one the places already hold (the same
/// `Arc`), not a new strong handle from `Assets::get_strong_handle`: the
/// places' images are the render world's only (`RenderAssetUsages::
/// RENDER_WORLD`), so the main world's copy is gone a frame after it's
/// added, and Bevy 0.16 forgets a new handle's count then; the first
/// place to go would free the texture under the others' feet.
#[derive(Resource, Default)]
pub struct UploadedTextures {
    held: std::collections::HashMap<TextureKey, std::sync::Weak<bevy::asset::StrongHandle>>,
    /// Entries kept before the gone ones are swept out.
    sweep_at: usize,
}

impl UploadedTextures {
    /// The texture still held by some place, else `upload`'s (remembered).
    fn get_or_upload(
        &mut self,
        key: TextureKey,
        upload: impl FnOnce() -> Option<Handle<Image>>,
    ) -> Option<Handle<Image>> {
        if let Some(held) = self.held.get(&key).and_then(std::sync::Weak::upgrade) {
            return Some(Handle::Strong(held));
        }
        let handle = upload()?;
        if let Handle::Strong(strong) = &handle {
            if self.held.len() >= self.sweep_at {
                self.held.retain(|_, w| w.strong_count() > 0);
                self.sweep_at = (2 * self.held.len()).max(1024);
            }
            self.held.insert(key, Arc::downgrade(strong));
        }
        Some(handle)
    }
}

impl Spawner<'_, '_> {
    /// A place's texture: the one already on the GPU when a loaded place
    /// has it, else sent now.
    fn texture(
        &mut self,
        texture: &TextureData,
        compressed: bool,
        anisotropy: u16,
    ) -> Option<Handle<Image>> {
        let key = (
            texture.path.clone(),
            texture.linear,
            texture.layers,
            compressed,
            anisotropy,
        );
        let images = &mut self.images;
        self.uploaded.get_or_upload(key, || {
            upload_texture(images, texture, compressed, anisotropy)
        })
    }

    /// Puts a loaded place's textures, models and terrain on screen, and
    /// returns what it spawned (so an outdoor square can be dropped later).
    fn spawn(&mut self, scene: &ViewerScene) -> Vec<Entity> {
        self.spawn_parts(scene, None, None).entities
    }

    /// [`Self::spawn`], lit with `lighting` when given (people coming into
    /// a place take its light) instead of the scene's own.
    fn spawn_with(&mut self, scene: &ViewerScene, lighting: Option<GameLighting>) -> Vec<Entity> {
        self.spawn_parts(scene, lighting, None).entities
    }

    /// [`Self::spawn`], also giving the materials it made (an outdoor
    /// square's, to be lit again by its neighbours' lights); its terrain
    /// blends toward the distant land as seen from square `here`, with
    /// the distant land's `noise`.
    fn spawn_square(
        &mut self,
        scene: &ViewerScene,
        here: (i32, i32),
        noise: Option<Handle<Image>>,
    ) -> Spawned {
        self.spawn_parts(scene, None, Some((here, noise)))
    }

    /// Gives the materials of a spawned place these lights. Outdoors this
    /// is redone for every loaded square whenever one comes or goes, and
    /// most keep the lights they had ([`give_lights`]).
    fn relight(&mut self, spawned: &Spawned, lights: &[cellview::LightData]) {
        let list = game_lights(lights, self.settings.brightness);
        let count = lights.len().min(MAX_LIGHTS) as f32;
        give_lights(&mut self.lit_materials, &spawned.lit, list, count);
        give_lights(&mut self.terrain_materials, &spawned.terrain, list, count);
    }

    /// `land_blend`: outdoors, the player's square and the distant land's
    /// noise, for the terrain's blend toward the distant land.
    fn spawn_parts(
        &mut self,
        scene: &ViewerScene,
        lighting: Option<GameLighting>,
        land_blend: Option<LandBlendFrom>,
    ) -> Spawned {
        // Its clutter's bodies go to the simulation (`clutter`).
        clutter::arrive(&scene.bodies);
        // Every desktop graphics card takes block-compressed textures;
        // check when the device is visible from here.
        let compressed = self
            .device
            .as_ref()
            .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
        let anisotropy = self.settings.anisotropy;
        let textures: Vec<Option<Handle<Image>>> = scene
            .textures
            .iter()
            .map(|t| self.texture(t, compressed, anisotropy))
            .collect();
        if scene.lights.len() > MAX_LIGHTS {
            println!(
                "  {} has {} lights; only the first {MAX_LIGHTS} are used",
                scene.cell,
                scene.lights.len()
            );
        }
        let lighting = match lighting {
            Some(l) => l,
            None => {
                let lighting = game_lighting(scene, self.settings.brightness);
                // Kept for what's drawn apart from the place (the
                // first-person view, people coming in); outdoors, the last
                // square loaded.
                self.place_lighting.set(lighting);
                lighting
            }
        };
        let mut spawned = Vec::new();
        let mut terrain_materials = Vec::new();

        // One mesh and material per piece of each model, shared by its
        // placements.
        // Actors' pieces are skinned meshes in their bind pose, moved by
        // their bones (see `actors`).
        let mut pieces = Vec::with_capacity(scene.meshes.len());
        let mut binds = Vec::with_capacity(scene.meshes.len());
        // Blended pieces are built around the point the game sorts them by
        // (see `sort_center`).
        let centers: Vec<Option<[f32; 3]>> = scene.meshes.iter().map(sort_center).collect();
        // Pieces with the same material (its data, and whether a normal map
        // can be used) share one, so their draws can go together.
        let mut same_material: std::collections::HashMap<String, Handle<GameLitMaterial>> =
            std::collections::HashMap::new();
        for (data, center) in scene.meshes.iter().zip(&centers) {
            let (mesh, bind) = match actors::skinned_mesh(data) {
                Some((mesh, bind)) => (
                    self.meshes.add(mesh),
                    Some(self.inverse_bindposes.add(bind)),
                ),
                None => (
                    self.meshes
                        .add(game_mesh_around(data, center.unwrap_or([0.0; 3]))),
                    None,
                ),
            };
            let key = format!("{:?} {}", data.material, data.tangents.is_some());
            let material = same_material
                .entry(key)
                .or_insert_with(|| {
                    self.lit_materials
                        .add(lit_material(data, &textures, lighting))
                })
                .clone();
            pieces.push((mesh, material));
            binds.push(bind);
        }
        let actor_joints: Vec<(Entity, Vec<Entity>)> = scene
            .actors
            .iter()
            .enumerate()
            .map(|(i, actor)| {
                actors::spawn_actor(
                    &mut self.commands,
                    actor,
                    i,
                    (SceneEntity, scripts::PlacedRef(actor.reference)),
                )
            })
            .collect();
        spawned.extend(actor_joints.iter().map(|(root, _)| *root));
        let mut own_materials = Vec::new();
        // The doors that swing: their pieces follow the door's state.
        let swing_doors: std::collections::HashSet<u32> =
            scene.swing_doors.iter().map(|d| d.reference.0).collect();
        for draw in &scene.draws {
            let (mesh, material) = &pieces[draw.mesh];
            if let (Some(a), Some(bind)) = (draw.actor, &binds[draw.mesh]) {
                let (root, joints) = &actor_joints[a];
                let joints = actors::skin_joints(&scene.meshes[draw.mesh], joints);
                let mut piece = self.commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    actors::skinned(bind.clone(), joints),
                    // The skin moves it away from where its own box is.
                    bevy::render::view::NoFrustumCulling,
                    ChildOf(*root),
                ));
                // Which part of its look it is, for redrawing (`dress`).
                if let Some(part) = scene.meshes[draw.mesh].actor_part {
                    piece.insert(dress::Piece(part));
                }
                // Head parts talk and blink (`faces`).
                if let Some(face) = faces::FacePiece::of(&scene.meshes[draw.mesh]) {
                    piece.insert(face);
                }
                if let Some(link) = scene.meshes[draw.mesh].material.emittance {
                    piece.insert(emittance::Glow(link));
                }
                continue;
            }
            let base = Mat4::from_cols_array(&space::matrix(&draw.transform));
            let center = Vec3::from(centers[draw.mesh].unwrap_or([0.0; 3]));
            let transform = Transform::from_matrix(base * Mat4::from_translation(center));
            // A piece whose material its animation changes gets one of its
            // own.
            let own_material = scene.meshes[draw.mesh]
                .motion
                .as_ref()
                .filter(|m| m.changes_material())
                .map(|_| {
                    let own = self.lit_materials.add(lit_material(
                        &scene.meshes[draw.mesh],
                        &textures,
                        lighting,
                    ));
                    own_materials.push(own.clone());
                    own
                });
            let mut piece = self.commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(own_material.clone().unwrap_or_else(|| material.clone())),
                transform,
                SceneEntity,
                scripts::PlacedRef(draw.reference),
                swaps::PieceName(scene.meshes[draw.mesh].shape_name.clone()),
            ));
            // What the game puts world decals on (`impact_fx`).
            if scene.meshes[draw.mesh].strips && !scene.meshes[draw.mesh].material.decal {
                piece.insert(impact_fx::DecalReceiver);
            }
            // A glow that follows a region's weather (`emittance`).
            if let Some(link) = scene.meshes[draw.mesh].material.emittance {
                piece.insert(emittance::Glow(link));
            }
            // A piece the model's own animation moves (`move_pieces`), or
            // that turns to face the camera (`face_camera`).
            if let Some(motion) = &scene.meshes[draw.mesh].motion {
                piece.insert(Moving {
                    motion: motion.clone(),
                    base,
                    center,
                    material: own_material,
                    shown: None,
                    door: swing_doors
                        .contains(&draw.reference)
                        .then_some(draw.reference),
                });
            }
            if let Some(billboard) = scene.meshes[draw.mesh].billboard {
                piece.insert(Facing {
                    billboard,
                    draw: draw.transform,
                    base,
                    center,
                });
            }
            spawned.push(piece.id());
        }
        for quarter in &scene.terrain {
            let layers: Vec<terrain::TexturePair> = quarter
                .textures
                .iter()
                .map(|&(diffuse, normal)| {
                    (
                        diffuse.and_then(|i| textures[i].clone()),
                        normal.and_then(|i| textures[i].clone()),
                    )
                })
                .collect();
            let lod = quarter.lod_blend.as_ref();
            let texture = |i: Option<usize>| i.and_then(|i| textures[i].clone());
            let (here, noise) = land_blend.clone().unwrap_or(((0, 0), None));
            let material = self.terrain_materials.add(TerrainMaterial {
                base: StandardMaterial::default(),
                extension: Terrain::new(
                    lighting,
                    &layers,
                    terrain::LandBlend::new(lod.map(|l| l.offset), here),
                    (
                        texture(lod.and_then(|l| l.diffuse)),
                        texture(lod.and_then(|l| l.normals)),
                    ),
                    noise,
                ),
            });
            terrain_materials.push(material.clone());
            let mesh = self.meshes.add(terrain::terrain_mesh(quarter));
            spawned.push(
                self.commands
                    .spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::IDENTITY,
                        SceneEntity,
                        impact_fx::LandReceiver,
                    ))
                    .id(),
            );
        }
        spawned.extend(water::spawn_water(
            &mut self.commands,
            &mut self.meshes,
            &mut self.water,
            scene,
            &textures,
            full_brightness_nits(),
        ));
        spawned.extend(particles::spawn_particles(
            &mut self.commands,
            &mut self.meshes,
            &mut self.particle_materials,
            scene,
            &textures,
        ));
        Spawned {
            entities: spawned,
            lit: pieces
                .into_iter()
                .map(|(_, m)| m)
                .chain(own_materials)
                .collect(),
            terrain: terrain_materials,
        }
    }
}

/// Where an outdoor square's terrain blends toward the distant land from:
/// the player's square, and the distant land's noise texture.
type LandBlendFrom = ((i32, i32), Option<Handle<Image>>);

/// What spawning a place made: its entities and its materials.
pub struct Spawned {
    pub entities: Vec<Entity>,
    pub lit: Vec<Handle<GameLitMaterial>>,
    pub terrain: Vec<Handle<TerrainMaterial>>,
}

/// A lone actor's lit pieces (`Spawner::spawn_lone_actor_lit`): its
/// textures, and each piece's material with what it was made from (its
/// surface and whether it has tangents), to light them again.
#[derive(Default)]
pub struct LitPieces {
    textures: Vec<Option<Handle<Image>>>,
    lit: Vec<(Handle<GameLitMaterial>, cellview::MaterialData, bool)>,
}

impl LitPieces {
    /// Each piece's material made again with `lighting`.
    fn relight(&self, materials: &mut Assets<GameLitMaterial>, lighting: GameLighting) {
        for (handle, material, tangents) in &self.lit {
            if let Some(m) = materials.get_mut(handle) {
                *m = lit_material_of(material, *tangents, &self.textures, lighting);
            }
        }
    }
}

impl Spawner<'_, '_> {
    /// One actor on its own (the first-person view) under `parent`, lit by
    /// `lighting`, drawn by the first-person camera alone: its root, its
    /// joints and its pieces' entities.
    fn spawn_lone_actor(
        &mut self,
        scene: &ViewerScene,
        lighting: GameLighting,
        parent: Entity,
    ) -> Option<(Entity, Vec<Entity>, Vec<Entity>)> {
        self.spawn_lone_actor_on(scene, lighting, parent, viewmodel::FIRST_PERSON_LAYER)
    }

    /// [`Self::spawn_lone_actor`] drawn on render layer `layer` (0: the
    /// main camera's, for the player's third-person body).
    fn spawn_lone_actor_on(
        &mut self,
        scene: &ViewerScene,
        lighting: GameLighting,
        parent: Entity,
        layer: usize,
    ) -> Option<(Entity, Vec<Entity>, Vec<Entity>)> {
        self.spawn_lone_actor_lit(scene, lighting, parent, layer)
            .map(|(root, joints, pieces, _)| (root, joints, pieces))
    }

    /// [`Self::spawn_lone_actor_on`], also giving what it takes to light its
    /// pieces again ([`Self::relight_lone_actor`]).
    fn spawn_lone_actor_lit(
        &mut self,
        scene: &ViewerScene,
        lighting: GameLighting,
        parent: Entity,
        layer: usize,
    ) -> Option<(Entity, Vec<Entity>, Vec<Entity>, LitPieces)> {
        let actor = scene.actors.first()?;
        let compressed = self
            .device
            .as_ref()
            .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
        let anisotropy = self.settings.anisotropy;
        let textures: Vec<Option<Handle<Image>>> = scene
            .textures
            .iter()
            .map(|t| self.texture(t, compressed, anisotropy))
            .collect();
        let root = self
            .commands
            .spawn((Transform::IDENTITY, Visibility::default(), ChildOf(parent)))
            .id();
        let joints: Vec<Entity> = actor
            .skeleton
            .bones
            .iter()
            .map(|_| {
                self.commands
                    .spawn((Transform::IDENTITY, ChildOf(root)))
                    .id()
            })
            .collect();
        let mut pieces = Vec::new();
        let mut lit = Vec::new();
        for draw in &scene.draws {
            let data = &scene.meshes[draw.mesh];
            let Some((mesh, bind)) = actors::skinned_mesh(data) else {
                continue;
            };
            let mesh = self.meshes.add(mesh);
            let bind = self.inverse_bindposes.add(bind);
            let material = self
                .lit_materials
                .add(lit_material(data, &textures, lighting));
            lit.push((
                material.clone(),
                data.material.clone(),
                data.tangents.is_some(),
            ));
            let skin_joints = actors::skin_joints(data, &joints);
            pieces.push(
                self.commands
                    .spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::IDENTITY,
                        actors::skinned(bind, skin_joints),
                        bevy::render::view::NoFrustumCulling,
                        bevy::render::view::RenderLayers::layer(layer),
                        ChildOf(root),
                    ))
                    .id(),
            );
        }
        Some((root, joints, pieces, LitPieces { textures, lit }))
    }

    /// Lights a lone actor's pieces with `lighting`: each material made
    /// again as [`Self::spawn_lone_actor_lit`] would make it now, and
    /// handed to the daylight as a new one is (`daylight::RemadeLit`; it's
    /// the only thing that changes a lone actor's materials after), without
    /// rebuilding the actor.
    fn relight_lone_actor(&mut self, pieces: &LitPieces, lighting: GameLighting) {
        pieces.relight(&mut self.lit_materials, lighting);
        // Outdoors the hour's light goes onto them as onto new ones.
        self.remade_lit
            .ids
            .extend(pieces.lit.iter().map(|(handle, _, _)| handle.id()));
        self.remade_lit.times += 1;
    }
}

/// How far the sky dome is drawn, in meters: just past the game's far clip
/// plane (`lod::GAME_FAR_CLIP`, about 5,030 m), so all the distant land the
/// game draws stands in front of it, inside the camera's far plane. The
/// game draws it at the far plane itself (`SKY.vso`).
const SKY_RADIUS_METERS: f32 = 5100.0;

/// The sky dome, kept centred on the camera.
#[derive(Component)]
pub struct SkyEntity;

impl Spawner<'_, '_> {
    /// The sky dome: its stored colours drawn as they are, with no light
    /// and no fog, behind everything.
    fn spawn_sky(&mut self, dome: &cellview::SkyDome) -> Entity {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        let positions: Vec<[f32; 3]> = dome.positions.iter().map(|&p| space::point(p)).collect();
        // The lighting shader takes vertex colors as linear light and
        // encodes them back; the dome's are stored values, so decode them
        // (past 1 too, where its weights add up past 1).
        let colors: Vec<[f32; 4]> = dome
            .colors
            .iter()
            .map(|c| {
                [
                    srgb_decode(c[0]),
                    srgb_decode(c[1]),
                    srgb_decode(c[2]),
                    c[3],
                ]
            })
            .collect();
        let count = positions.len();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; count]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; count]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(bevy::render::mesh::Indices::U16(dome.indices.clone()));
        let base = StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            double_sided: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            ..default()
        };
        let extension = GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                ambient: Vec4::ZERO,
                directional_color: Vec4::ZERO,
                directional_direction: Vec4::Y,
                emissive: Vec4::new(1.0, 1.0, 1.0, 0.0),
                scale: Vec4::new(full_brightness_nits(), 0.0, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                // No fog on the sky.
                fog_range: Vec4::ZERO,
                specular: Vec4::ZERO,
                surface: Vec4::new(0.0, 0.0, 1.0, 0.0),
                falloff: NO_FALLOFF,
                environment: Vec4::ZERO,
                draw: Vec4::ZERO,
                actor: Vec4::ZERO,
                hair_tint: Vec4::ZERO,
                lights: [GameLight::default(); MAX_LIGHTS],
            },
            glow: None,
            normal_map: None,
            environment: None,
            environment_mask: None,
            key: DrawKey::default(),
        };
        let scale = SKY_RADIUS_METERS / (500.0 * space::METERS_PER_UNIT);
        self.commands
            .spawn((
                Mesh3d(self.meshes.add(mesh)),
                MeshMaterial3d(self.lit_materials.add(GameLitMaterial {
                    base,
                    extension: extension.into(),
                })),
                Transform::from_scale(Vec3::splat(scale)),
                bevy::render::view::NoFrustumCulling,
                daylight::SkyWeights(dome.weights.clone()),
                SkyEntity,
                SceneEntity,
            ))
            .id()
    }
}

/// The sun's square (Bevy's space, game units around the eye): `size`
/// across, facing the eye, toward the sun (`toward`, game space).
pub(crate) fn sun_square(toward: [f32; 3], size: f32) -> Vec<[f32; 3]> {
    let d = Vec3::from(toward).normalize_or(Vec3::Z);
    let side = d.cross(Vec3::Z).try_normalize().unwrap_or(Vec3::X);
    let up = side.cross(d);
    let centre = d * cellview::game::SUN_DISTANCE;
    let h = size * 0.5;
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    corners
        .iter()
        .map(|&(x, y)| space::point((centre + side * (x * h) + up * (y * h)).to_array()))
        .collect()
}

/// A cloud layer's texture scrolling (`cellview::game::CloudLayer`): its
/// own texture coordinates, and how fast they move along V.
#[derive(Component)]
pub struct CloudScroll {
    uvs: Vec<[f32; 2]>,
    speed: f32,
    /// The weather's layer (its colours by time of day) and the dome's
    /// opacity at each vertex.
    pub layer: usize,
    pub alphas: Vec<f32>,
}

impl Spawner<'_, '_> {
    /// The sun (`cellview::game::SunSprite`): its disc and its glare, each a
    /// square of its texture facing the eye, over the dome and under the
    /// clouds; `daylight` places and colours them for the hour.
    fn spawn_sun(&mut self, sun: &cellview::game::SunSprite) {
        self.spawn_sun_square(&sun.texture, sun.half_size, None);
        if let Some(glare) = &sun.glare {
            self.spawn_sun_square(glare, sun.glare_half_size, Some(sun.glare_strength));
        }
    }

    /// One of the sun's squares (dark until `daylight` colours it).
    fn spawn_sun_square(
        &mut self,
        texture: &TextureData,
        half_size: f32,
        glare: Option<f32>,
    ) -> Entity {
        let positions = sun_square([0.0, 0.0, 1.0], half_size * 2.0);
        let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
        let uvs: Vec<[f32; 2]> = corners
            .iter()
            .map(|&(x, y)| [(x + 1.0) * 0.5, (1.0 - y) * 0.5])
            .collect();
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 0.0]; 4]);
        mesh.insert_indices(bevy::render::mesh::Indices::U16(vec![0, 1, 2, 0, 2, 3]));
        let base = StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: self.upload(texture),
            unlit: true,
            double_sided: true,
            cull_mode: None,
            // Added to the sky (a guess: blended over it, the wide halo
            // darkened the sky around the sun).
            alpha_mode: AlphaMode::Add,
            depth_bias: if glare.is_some() { 0.6 } else { 0.5 },
            ..default()
        };
        let extension = GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                ambient: Vec4::ZERO,
                directional_color: Vec4::ZERO,
                directional_direction: Vec4::Y,
                emissive: Vec4::new(1.0, 1.0, 1.0, 0.0),
                scale: Vec4::new(full_brightness_nits(), 0.0, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                fog_range: Vec4::ZERO,
                specular: Vec4::ZERO,
                surface: Vec4::new(0.0, 0.0, 1.0, 0.0),
                falloff: NO_FALLOFF,
                environment: Vec4::ZERO,
                draw: Vec4::ZERO,
                actor: Vec4::ZERO,
                hair_tint: Vec4::ZERO,
                lights: [GameLight::default(); MAX_LIGHTS],
            },
            glow: None,
            normal_map: None,
            environment: None,
            environment_mask: None,
            key: DrawKey::default(),
        };
        let scale = SKY_RADIUS_METERS / (500.0 * space::METERS_PER_UNIT);
        self.commands
            .spawn((
                Mesh3d(self.meshes.add(mesh)),
                MeshMaterial3d(self.lit_materials.add(GameLitMaterial {
                    base,
                    extension: extension.into(),
                })),
                Transform::from_scale(Vec3::splat(scale * 0.99)),
                bevy::render::view::NoFrustumCulling,
                daylight::SunDisk { half_size, glare },
                SkyEntity,
                SceneEntity,
            ))
            .id()
    }
    /// The night sky (`cellview::game::StarDome`) inside the dome, added
    /// onto it; dark until `daylight` colours it for the hour.
    fn spawn_stars(&mut self, stars: &cellview::game::StarDome) -> Entity {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        let radius = stars
            .positions
            .iter()
            .map(|p| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
            .fold(1.0f32, f32::max);
        // Inside the dome (half the sky radius across), by the same margin
        // as the clouds.
        let scale = SKY_RADIUS_METERS * 0.5 * 0.97 / (radius * space::METERS_PER_UNIT);
        // `SKYSHORIZFADE`: gone below the horizon, full 17 units above the
        // eye (in the world: almost at once).
        let units = radius * scale;
        let fade: Vec<f32> = stars
            .positions
            .iter()
            .map(|p| (p[2] / radius * units / 17.0).clamp(0.0, 1.0))
            .collect();
        let positions: Vec<[f32; 3]> = stars.positions.iter().map(|&p| space::point(p)).collect();
        let count = positions.len();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; count]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, stars.uvs.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 0.0]; count]);
        mesh.insert_indices(bevy::render::mesh::Indices::U16(stars.indices.clone()));
        let base = StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: self.upload(&stars.texture),
            unlit: true,
            double_sided: true,
            cull_mode: None,
            // Added (One + One in the model; the fade is the alpha, so it
            // weights what's added: a guess where the two disagree).
            alpha_mode: AlphaMode::Add,
            depth_bias: 0.5,
            ..default()
        };
        let extension = GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                ambient: Vec4::ZERO,
                directional_color: Vec4::ZERO,
                directional_direction: Vec4::Y,
                emissive: Vec4::new(1.0, 1.0, 1.0, 0.0),
                scale: Vec4::new(full_brightness_nits(), 0.0, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                fog_range: Vec4::ZERO,
                specular: Vec4::ZERO,
                surface: Vec4::new(0.0, 0.0, 1.0, 0.0),
                falloff: NO_FALLOFF,
                environment: Vec4::ZERO,
                draw: Vec4::ZERO,
                actor: Vec4::ZERO,
                hair_tint: Vec4::ZERO,
                lights: [GameLight::default(); MAX_LIGHTS],
            },
            glow: None,
            normal_map: None,
            environment: None,
            environment_mask: None,
            key: DrawKey::default(),
        };
        self.commands
            .spawn((
                Mesh3d(self.meshes.add(mesh)),
                MeshMaterial3d(self.lit_materials.add(GameLitMaterial {
                    base,
                    extension: extension.into(),
                })),
                Transform::from_scale(Vec3::splat(scale)),
                bevy::render::view::NoFrustumCulling,
                daylight::Stars { fade },
                SkyEntity,
                SceneEntity,
            ))
            .id()
    }

    /// A weather's cloud layer over the sky dome: its texture times the
    /// layer's colour, faded toward the horizon by the dome's vertex alpha,
    /// no light or fog, kept centred on the camera with the dome.
    fn spawn_clouds(&mut self, layer: &cellview::game::CloudLayer) -> Entity {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        let positions: Vec<[f32; 3]> = layer.positions.iter().map(|&p| space::point(p)).collect();
        let count = positions.len();
        let [r, g, b] = layer.color.map(srgb_decode);
        let colors: Vec<[f32; 4]> = layer.alphas.iter().map(|&a| [r, g, b, a]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; count]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, layer.uvs.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(bevy::render::mesh::Indices::U16(layer.indices.clone()));
        let base = StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: self.upload(&layer.texture),
            unlit: true,
            double_sided: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            // Over the dome, both centred on the camera.
            depth_bias: 1.0,
            ..default()
        };
        let extension = GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                ambient: Vec4::ZERO,
                directional_color: Vec4::ZERO,
                directional_direction: Vec4::Y,
                emissive: Vec4::new(1.0, 1.0, 1.0, 0.0),
                scale: Vec4::new(full_brightness_nits(), 0.0, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                fog_range: Vec4::ZERO,
                specular: Vec4::ZERO,
                surface: Vec4::new(0.0, 0.0, 1.0, 0.0),
                falloff: NO_FALLOFF,
                environment: Vec4::ZERO,
                draw: Vec4::ZERO,
                actor: Vec4::ZERO,
                hair_tint: Vec4::ZERO,
                lights: [GameLight::default(); MAX_LIGHTS],
            },
            glow: None,
            normal_map: None,
            environment: None,
            environment_mask: None,
            key: DrawKey::default(),
        };
        let scale = SKY_RADIUS_METERS / (500.0 * space::METERS_PER_UNIT);
        self.commands
            .spawn((
                Mesh3d(self.meshes.add(mesh)),
                MeshMaterial3d(self.lit_materials.add(GameLitMaterial {
                    base,
                    extension: extension.into(),
                })),
                Transform::from_scale(Vec3::splat(scale * 0.98)),
                bevy::render::view::NoFrustumCulling,
                CloudScroll {
                    uvs: layer.uvs.clone(),
                    speed: layer.scroll,
                    layer: layer.layer,
                    alphas: layer.alphas.clone(),
                },
                SkyEntity,
                SceneEntity,
            ))
            .id()
    }

    /// The `--brightness` setting.
    fn brightness(&self) -> f32 {
        self.settings.brightness
    }

    /// One texture for the graphics card.
    fn upload(&mut self, texture: &TextureData) -> Option<Handle<Image>> {
        let compressed = self
            .device
            .as_ref()
            .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
        upload_texture(
            &mut self.images,
            texture,
            compressed,
            self.settings.anisotropy,
        )
    }
}

/// The light distant land is drawn with: a square's ambient, sun and fog.
fn lod_params(scene: &ViewerScene, brightness: f32) -> LodLandParams {
    let l = game_lighting(scene, brightness);
    LodLandParams {
        ambient: l.ambient,
        sun_color: l.directional_color,
        sun_direction: l.directional_direction,
        fog_color: l.fog_color,
        fog_range: l.fog_range,
        scale: l.scale,
        high_detail: Vec4::ZERO,
    }
}

/// The sRGB curve's decoding, not clamped at 1.
fn srgb_decode(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// `--fps`: frames counted, the time since the last report and the
/// longest frame in it (a hitch).
#[derive(Resource, Default)]
struct FrameCounter {
    on: bool,
    frames: u32,
    seconds: f32,
    longest: f32,
}

fn report_fps(
    time: Res<Time>,
    mut counter: ResMut<FrameCounter>,
    work: Res<frame_work::FrameWork>,
) {
    if !counter.on {
        return;
    }
    counter.frames += 1;
    counter.seconds += time.delta_secs();
    counter.longest = counter.longest.max(time.delta_secs());
    if counter.seconds >= 2.0 {
        // The frames' own work on each thread (`frame_work`), apart from
        // waiting for the display.
        let (main, main_longest) = work.main.lock().map(|mut t| t.take()).unwrap_or_default();
        let (render, render_longest) = work.render.lock().map(|mut t| t.take()).unwrap_or_default();
        println!(
            "{:.0} frames per second ({:.1} ms a frame, longest {:.1} ms)",
            counter.frames as f32 / counter.seconds,
            1000.0 * counter.seconds / counter.frames as f32,
            1000.0 * counter.longest
        );
        println!(
            "  work: main {main:.1} ms a frame (longest {main_longest:.1}), \
             render {render:.1} ms (longest {render_longest:.1})"
        );
        counter.frames = 0;
        counter.seconds = 0.0;
        counter.longest = 0.0;
    }
}

/// The sky stays centred on the eye, as the game's does.
fn follow_sky(
    time: Res<Time>,
    settings: Res<Settings>,
    cameras: Query<&Transform, (With<FlyCamera>, Without<SkyEntity>)>,
    mut skies: Query<&mut Transform, With<SkyEntity>>,
    clouds: Query<(&Mesh3d, &CloudScroll)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    for mut sky in &mut skies {
        sky.translation = camera.translation;
    }
    // Clouds drift: their texture moves along V (`TexCoordYOff`), unless
    // `--cloud-time` holds them.
    let t = settings.cloud_time.unwrap_or_else(|| time.elapsed_secs());
    for (mesh, scroll) in &clouds {
        if scroll.speed == 0.0 {
            continue;
        }
        let off = (t * scroll.speed).fract();
        if let Some(m) = meshes.get_mut(&mesh.0) {
            let uvs: Vec<[f32; 2]> = scroll.uvs.iter().map(|&[u, v]| [u, v + off]).collect();
            m.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        }
    }
}

/// A texture for the graphics card, or `None` (with a message) when its
/// data can't be used.
fn upload_texture(
    images: &mut Assets<Image>,
    texture: &TextureData,
    compressed: bool,
    anisotropy: u16,
) -> Option<Handle<Image>> {
    // The game's sampling (see `anisotropy_setting`): anisotropic
    // trilinear at the INI's anisotropy, no LOD bias, repeating.
    let sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: anisotropy,
        ..default()
    });
    // Reflection cube maps are clamped at their edges, as the game samples
    // them.
    let cube_sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        address_mode_w: ImageAddressMode::ClampToEdge,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: anisotropy,
        ..default()
    });
    let cube = texture.layers == 6;
    let format = texture.gpu_format(compressed);
    let data = match texture.level_data(format) {
        Ok(data) => data,
        Err(e) => {
            println!("  skipping texture {}: {e}", texture.path);
            return None;
        }
    };
    let image = Image {
        data: Some(data),
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: texture.width,
                height: texture.height,
                depth_or_array_layers: texture.layers,
            },
            mip_level_count: texture.mip_levels,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: gpu_format(format),
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        sampler: if cube { cube_sampler } else { sampler },
        texture_view_descriptor: cube.then(|| TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        }),
        asset_usage: RenderAssetUsages::RENDER_WORLD,
    };
    Some(images.add(image))
}

/// A model piece's material: the standard material for its texture,
/// blending and culling, with the game's lighting on top.
fn lit_material(
    data: &cellview::MeshData,
    textures: &[Option<Handle<Image>>],
    lighting: GameLighting,
) -> GameLitMaterial {
    lit_material_of(&data.material, data.tangents.is_some(), textures, lighting)
}

/// [`lit_material`] from the piece's material and whether it has tangents
/// (all it reads of the piece).
fn lit_material_of(
    m: &cellview::MaterialData,
    tangents: bool,
    textures: &[Option<Handle<Image>>],
    lighting: GameLighting,
) -> GameLitMaterial {
    let [r, g, b, a] = m.color;
    let base = StandardMaterial {
        base_color: Color::linear_rgba(r, g, b, a),
        base_color_texture: m.texture.and_then(|i| textures[i].clone()),
        unlit: m.unlit,
        double_sided: m.double_sided,
        cull_mode: if m.double_sided {
            None
        } else {
            Some(Face::Back)
        },
        alpha_mode: alpha_mode(m.blend),
        // Only orders blended surfaces here (the pipeline's own bias is
        // turned off for decals, which the shader pulls nearer instead):
        // the game draws its decals before every other blended surface.
        depth_bias: if m.decal { DECALS_FIRST } else { 0.0 },
        ..default()
    };
    let (test, threshold) = lighting::alpha_test_code(m.alpha_test);
    let draw = Vec4::new(
        if m.decal { lighting.draw.x } else { 0.0 },
        test,
        threshold,
        0.0,
    );
    let key = DrawKey {
        game: true,
        depth_test: m.depth_test,
        depth_write: m.depth_write,
        decal: m.decal,
    };
    let mut extension = if m.unlit {
        let [ur, ug, ub] = m.unlit_color;
        // How the game's no-lighting shaders fog (`Toggles`): added
        // effects fade to black, multiplied ones to white (no effect),
        // the rest toward the fog color.
        let fog_mode = match m.blend {
            Blend::Add => 1.0,
            Blend::Multiply => 2.0,
            _ => 0.0,
        };
        GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                emissive: Vec4::new(ur, ug, ub, 0.0),
                surface: Vec4::new(0.0, 0.0, 1.0, fog_mode),
                falloff: m.falloff.map_or(NO_FALLOFF, |f| {
                    Vec4::new(f.start_cos, f.stop_cos, f.start_opacity, f.stop_opacity)
                }),
                ..lighting
            },
            key,
            glow: None,
            normal_map: None,
            environment: None,
            environment_mask: None,
        }
    } else {
        let glow = m.glow.and_then(|i| textures[i].clone());
        let normal_map = m
            .normal_map
            .filter(|_| tangents)
            .and_then(|i| textures[i].clone());
        let [er, eg, eb] = m.emissive;
        let specular = m.specular.map_or(Vec4::ZERO, |s| {
            Vec4::new(s.color[0], s.color[1], s.color[2], s.glossiness)
        });
        let flag = |on: bool| if on { 1.0 } else { 0.0 };
        let reflection = m
            .environment
            .and_then(|e| Some((e, textures[e.cube].clone()?)));
        let environment_mask = reflection
            .as_ref()
            .and_then(|(e, _)| e.mask)
            .and_then(|i| textures[i].clone());
        GameLit {
            shared: crate::shared_light::TEXTURE,
            lighting: GameLighting {
                emissive: Vec4::new(er, eg, eb, flag(glow.is_some())),
                specular,
                surface: Vec4::new(
                    flag(normal_map.is_some()),
                    flag(m.specular.is_some()),
                    0.0,
                    flag(matches!(alpha_mode(m.blend), AlphaMode::Blend)),
                ),
                environment: reflection.as_ref().map_or(Vec4::ZERO, |(e, _)| {
                    Vec4::new(
                        e.strength,
                        flag(environment_mask.is_some()),
                        flag(e.window),
                        m.color[3],
                    )
                }),
                actor: Vec4::new(
                    GameLighting::shading_code(m.shading),
                    lighting.actor.y,
                    0.0,
                    0.0,
                ),
                hair_tint: m
                    .hair_tint
                    .map_or(Vec4::ZERO, |[r, g, b]| Vec4::new(r, g, b, 1.0)),
                ..lighting
            },
            key,
            glow,
            normal_map,
            environment: reflection.map(|(_, cube)| cube),
            environment_mask,
        }
    };
    extension.lighting.draw = draw;
    GameLitMaterial {
        base,
        extension: extension.into(),
    }
}

/// Where decals sort among blended surfaces (Bevy's depth bias, added to
/// the distance it sorts by): before all of them.
const DECALS_FIRST: f32 = -1.0e6;

/// Puts a newly loaded interior on screen: its textures, meshes and
/// lights, the camera at its start, its image space, collision and doors.
#[allow(clippy::too_many_arguments)]
fn spawn_scene(
    mut pending: ResMut<PendingScene>,
    mut spawner: Spawner,
    mut start_pitch: ResMut<StartPitch>,
    mut grading: ResMut<Grading>,
    mut player: ResMut<walk::Player>,
    mut clear: ResMut<ClearColor>,
    old: Query<Entity, With<SceneEntity>>,
    mut cameras: Query<(&mut Transform, &mut FlyCamera, &mut ImageSpaceGrade)>,
    mut windows: Query<&mut Window>,
    (mut swing_doors, mut state, game): (
        ResMut<doors::SwingDoors>,
        ResMut<dialogue::DialogueState>,
        Res<GameFiles>,
    ),
) {
    let Some(scene) = pending.0.take() else {
        return;
    };
    // The place's doors that swing, and how long their models' sequences
    // play (the state's `GetOpenState` needs it).
    swing_doors.read_settings(&game.0.settings);
    swing_doors.replace(scene.swing_doors.iter(), &mut state.0);
    for entity in &old {
        spawner.commands.entity(entity).despawn();
    }
    exterior::leave(&mut spawner.commands);
    clear.0 = Color::BLACK;
    for mut window in &mut windows {
        window.title = format!("nv-rs viewer - {}", scene.cell);
    }
    spawner.spawn(&scene);
    local_map::capture_interior(&mut spawner.commands, &scene);

    let eye = Vec3::from(space::point(scene.start.eye));
    let yaw = space::heading_to_yaw(scene.start.heading);
    // The starting pitch (`--at`) applies to the first cell only.
    let pitch = std::mem::take(&mut start_pitch.0);
    grading.grade = ImageSpaceGrade::from_cell(scene.grade.as_ref(), scene.hdr.as_ref());
    let shown = if grading.on {
        grading.grade
    } else {
        grading.grade.without_cinematic()
    };
    for (mut transform, mut fly, mut grade) in &mut cameras {
        *transform = Transform::from_translation(eye).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            yaw,
            pitch,
            0.0,
        ));
        fly.yaw = yaw;
        fly.pitch = pitch;
        fly.start = (eye, yaw);
        *grade = shown;
    }
    let [x, y, z] = scene.start.eye;
    player.arrive([x, y, z - cellview::EYE_HEIGHT]);
    spawner.commands.insert_resource(dialogue::Talkers(
        scene
            .actors
            .iter()
            .map(dialogue::Talker::from_actor)
            .collect(),
    ));
    spawner
        .commands
        .insert_resource(walk::CellCollision(scene.collision));
    spawner
        .commands
        .insert_resource(scripts::ObjectBounds::from_scene(&scene.object_bounds));
    spawner.commands.insert_resource(walk::Doors(scene.doors));
    spawner
        .commands
        .insert_resource(scripts::Here(Some(scene.cell_id)));
}

fn help_text(ev100: f32, speed: f32, walking: bool) -> String {
    let moving = if walking {
        "Walking: the mouse looks, left button attacks, right aims (R: reload, hold: holster); \
         WASD, Shift to walk slowly, Ctrl: sneak on/off, Space to jump, E to use things"
            .to_string()
    } else {
        format!(
            "Flying: the mouse looks; WASD, Space/Ctrl up/down, Shift faster, \
             wheel: speed ({speed:.1} m/s), E to use things"
        )
    };
    format!(
        "{moving}\n\
         F: first/third person (hold: look around; wheel: zoom)   `: walk/fly\n\
         V: V.A.T.S.   Tab: Pip-Boy   T: wait   F5/F9: save/load   \
         [ ]: exposure (EV {ev100:.1})   G: image space   Home: start   Esc: pause menu"
    )
}

/// The help line, kept up to date.
fn update_help(
    cameras: Query<(&Exposure, &FlyCamera)>,
    player: Res<walk::Player>,
    mut help: Query<&mut Text, With<HelpText>>,
) {
    let Ok((exposure, camera)) = cameras.single() else {
        return;
    };
    let wanted = help_text(exposure.ev100, camera.speed, player.walking);
    for mut text in &mut help {
        if text.0 != wanted {
            text.0 = wanted.clone();
        }
    }
}

/// Whether the game is in menu mode for the mouse: a menu, the Pip-Boy,
/// lockpicking, the dialogue menu (not a line said in passing) or
/// V.A.T.S. is up. Then the mouse is the menus' pointer; otherwise, walking,
/// it turns the view as the game's always does.
fn mouse_in_menus(
    menus: Option<&menus::Menus>,
    conversation: Option<&dialogue::Conversation>,
    vats: Option<&vats::Vats>,
) -> bool {
    menus.is_some_and(|m| m.is_open())
        || conversation.is_some_and(|c| c.0.as_ref().is_some_and(|t| !t.is_line_only()))
        || vats.is_some_and(|v| v.is_on())
}

/// What says the mouse belongs to the menus ([`mouse_in_menus`]).
type MenuGates<'w> = (
    Option<Res<'w, menus::Menus>>,
    Option<Res<'w, dialogue::Conversation>>,
    Option<Res<'w, vats::Vats>>,
);

/// The mouse turns the view: walking, always (the game's mouse look; the
/// right button is the Aim control, `combat`), except in menus; flying,
/// while a button is held.
#[allow(clippy::too_many_arguments)]
fn look_around(
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    state: Res<dialogue::DialogueState>,
    player: Res<walk::Player>,
    start_stage: Res<scripts::StartStage>,
    view: Option<Res<player_camera::PlayerView>>,
    mut cameras: Query<(&mut Transform, &mut FlyCamera)>,
    (menus, conversation, vats): MenuGates,
) {
    let Ok((mut transform, mut camera)) = cameras.single_mut() else {
        return;
    };
    // Holding the view key (F), the mouse turns the camera around the
    // player instead (`player_camera`).
    let taken = player.walking && view.is_some_and(|v| v.mouse_taken);
    // Scripts can block looking through controls or a player AI package;
    // the free-flying camera (`) isn't the player and always looks.
    // Walking, the left button attacks (`combat`), so the right one looks.
    // The initial stage is dispatched later in this Update chain. Do not
    // accept an input frame before it installs the player's native package
    // lock (005cc4f0 -> 005cc7a0), or while the place is still loading.
    let locked = player.walking
        && (!player.ready || start_stage.0.is_some() || state.0.player_looking_blocked());
    let held = if player.walking {
        !mouse_in_menus(menus.as_deref(), conversation.as_deref(), vats.as_deref())
    } else {
        mouse.pressed(MouseButton::Right) || mouse.pressed(MouseButton::Left)
    };
    if !locked && held && !taken {
        camera.yaw -= motion.delta.x * LOOK_SPEED;
        camera.pitch = (camera.pitch - motion.delta.y * LOOK_SPEED).clamp(-1.54, 1.54);
    }
    transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
}

/// `--background`: a test run that leaves the mouse alone.
#[derive(Resource)]
struct Background(bool);

/// Walking outside menus the pointer is hidden and held in the window, as
/// the game holds it for its mouse look; in menus, flying, with the
/// window in the background or in a `--background` test run it's free.
fn grab_cursor(
    background: Res<Background>,
    player: Res<walk::Player>,
    menus: Option<Res<menus::Menus>>,
    conversation: Option<Res<dialogue::Conversation>>,
    vats: Option<Res<vats::Vats>>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };
    let grab = !background.0
        && player.walking
        && window.focused
        && !mouse_in_menus(menus.as_deref(), conversation.as_deref(), vats.as_deref());
    // Windows can't lock the pointer: there it's confined and put back in
    // the middle each frame (the look reads the mouse's own motion).
    let (mode, visible) = if !grab {
        (CursorGrabMode::None, true)
    } else if cfg!(target_os = "windows") {
        (CursorGrabMode::Confined, false)
    } else {
        (CursorGrabMode::Locked, false)
    };
    if window.cursor_options.grab_mode != mode || window.cursor_options.visible != visible {
        window.cursor_options.grab_mode = mode;
        window.cursor_options.visible = visible;
    }
    if grab && cfg!(target_os = "windows") {
        let middle = Vec2::new(window.width(), window.height()) / 2.0;
        if window.cursor_position() != Some(middle) {
            window.set_cursor_position(Some(middle));
        }
    }
}

/// G switches the cell's color adjustment (saturation, tint, contrast,
/// brightness) off and on, for comparing. Bloom stays on.
fn toggle_grade(
    keys: Res<ButtonInput<KeyCode>>,
    mut grading: ResMut<Grading>,
    mut cameras: Query<&mut ImageSpaceGrade>,
) {
    if !keys.just_pressed(KeyCode::KeyG) {
        return;
    }
    grading.on = !grading.on;
    let grade = if grading.on {
        grading.grade
    } else {
        grading.grade.without_cinematic()
    };
    for mut current in &mut cameras {
        *current = grade;
    }
}

/// Flying: free movement through everything.
fn fly_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    player: Res<walk::Player>,
    mut cameras: Query<(&mut Transform, &mut FlyCamera)>,
) {
    if player.walking {
        return;
    }
    let Ok((mut transform, mut camera)) = cameras.single_mut() else {
        return;
    };
    if scroll.delta.y != 0.0 {
        let notches = match scroll.unit {
            MouseScrollUnit::Line => scroll.delta.y,
            MouseScrollUnit::Pixel => scroll.delta.y / 40.0,
        };
        camera.speed = (camera.speed * 1.15f32.powf(notches.clamp(-5.0, 5.0))).clamp(0.2, 50.0);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        let (eye, yaw) = camera.start;
        transform.translation = eye;
        camera.yaw = yaw;
        camera.pitch = 0.0;
    }
    transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);

    let forward = transform.forward().as_vec3();
    let right = transform.right().as_vec3();
    let mut direction = Vec3::ZERO;
    for (key, step) in [
        (KeyCode::KeyW, forward),
        (KeyCode::KeyS, -forward),
        (KeyCode::KeyD, right),
        (KeyCode::KeyA, -right),
        (KeyCode::Space, Vec3::Y),
        (KeyCode::ControlLeft, -Vec3::Y),
        (KeyCode::KeyQ, -Vec3::Y),
    ] {
        if keys.pressed(key) {
            direction += step;
        }
    }
    if direction != Vec3::ZERO {
        let boost = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            4.0
        } else {
            1.0
        };
        transform.translation += direction.normalize() * camera.speed * boost * time.delta_secs();
    }
}

fn adjust_exposure(keys: Res<ButtonInput<KeyCode>>, mut cameras: Query<&mut Exposure>) {
    for mut exposure in &mut cameras {
        // Lower EV100 means a longer exposure: a brighter picture.
        if keys.just_pressed(KeyCode::BracketRight) || keys.just_pressed(KeyCode::Equal) {
            exposure.ev100 -= 0.5;
        }
        if keys.just_pressed(KeyCode::BracketLeft) || keys.just_pressed(KeyCode::Minus) {
            exposure.ev100 += 0.5;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The images Bevy is told no handle holds any more (the render world
    /// then drops their GPU copies).
    #[derive(Resource, Default)]
    struct Freed(Vec<AssetId<Image>>);

    fn note_freed(mut events: EventReader<AssetEvent<Image>>, mut freed: ResMut<Freed>) {
        for e in events.read() {
            if let AssetEvent::Unused { id } = e {
                freed.0.push(*id);
            }
        }
    }

    fn texture_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>()
            .init_resource::<Freed>()
            .add_systems(Last, note_freed);
        app
    }

    fn render_world_image(app: &mut App) -> Option<Handle<Image>> {
        let image = Image {
            asset_usage: RenderAssetUsages::RENDER_WORLD,
            ..default()
        };
        Some(app.world_mut().resource_mut::<Assets<Image>>().add(image))
    }

    /// What `bevy_render`'s extraction does with a render-world-only image
    /// at the end of the frame it was added in.
    fn extract(app: &mut App, id: AssetId<Image>) {
        app.world_mut().resource_mut::<Assets<Image>>().remove(id);
    }

    fn key() -> TextureKey {
        ("textures\\rock.dds".into(), false, 1, true, 16)
    }

    /// Two places loaded in one frame share a texture; the first one going
    /// mustn't free it while the second still draws with it.
    #[test]
    fn a_texture_shared_in_one_frame_stays_while_a_place_holds_it() {
        let mut app = texture_app();
        let mut cache = UploadedTextures::default();
        let first = cache.get_or_upload(key(), || render_world_image(&mut app));
        let second = cache.get_or_upload(key(), || panic!("sent again"));
        let id = first.as_ref().unwrap().id();
        assert_eq!(second.as_ref().map(Handle::id), Some(id));
        extract(&mut app, id);
        app.update();
        drop(first);
        app.update();
        app.update();
        assert!(app.world().resource::<Freed>().0.is_empty());
        drop(second);
        app.update();
        assert_eq!(app.world().resource::<Freed>().0, vec![id]);
    }

    /// A place loaded frames later takes the texture already on the GPU
    /// (not a copy), and keeps it after the first place goes.
    #[test]
    fn a_texture_is_shared_across_frames() {
        let mut app = texture_app();
        let mut cache = UploadedTextures::default();
        let first = cache.get_or_upload(key(), || render_world_image(&mut app));
        let id = first.as_ref().unwrap().id();
        extract(&mut app, id);
        app.update();
        app.update();
        let later = cache.get_or_upload(key(), || panic!("sent again"));
        assert_eq!(later.as_ref().map(Handle::id), Some(id));
        drop(first);
        app.update();
        assert!(app.world().resource::<Freed>().0.is_empty());
        // Once no place holds it, it goes, and is sent again when wanted.
        drop(later);
        app.update();
        assert_eq!(app.world().resource::<Freed>().0, vec![id]);
        let again = cache.get_or_upload(key(), || render_world_image(&mut app));
        assert_ne!(again.map(|h| h.id()), Some(id));
    }

    /// Sunny Smiles walking out of the saloon (`VCG02`) must join the
    /// outdoor people, once, so trigger volumes see her.
    #[test]
    fn people_brought_in_join_the_place_once() {
        let at = |r: u32, x: f32| dialogue::Talker {
            reference: esm::FormId(r),
            base: esm::FormId(r + 1),
            position: [x, 0.0, 0.0],
        };
        let mut talkers = vec![at(0x10, 0.0)];
        add_talkers(&mut talkers, [at(0x00104E85, 5.0), at(0x10, 9.0)]);
        add_talkers(&mut talkers, [at(0x00104E85, 7.0)]);
        let refs: Vec<u32> = talkers.iter().map(|t| t.reference.0).collect();
        assert_eq!(refs, [0x10, 0x00104E85]);
        // The ones already there keep their places.
        assert_eq!(talkers[0].position[0], 0.0);
        assert_eq!(talkers[1].position[0], 5.0);
    }

    /// People a script enabled after their square loaded (the Powder Gangers
    /// of Ghost Town Gunfight, `GoodspringsPowderGangMarker.Enable`) stay
    /// among the place's people when the loaded squares change: before, the
    /// rebuilt list left them out and shots passed through them.
    #[test]
    fn people_enabled_after_loading_stay_when_squares_change() {
        let at = |r: u32| dialogue::Talker {
            reference: esm::FormId(r),
            base: esm::FormId(r + 1),
            position: [0.0; 3],
        };
        let world = esm::FormId(0x000DA726);
        let ganger = esm::FormId(0x00104C72);
        let mut brought = BroughtIn::default();
        brought.remember(world, [ganger]);
        let old = vec![at(0x10), at(ganger.0)];
        let on_screen: std::collections::HashSet<_> = [esm::FormId(0x10), ganger].into();
        let rebuilt = exterior::with_brought_in(vec![at(0x10)], &old, &brought.people, &on_screen);
        let refs: Vec<u32> = rebuilt.iter().map(|t| t.reference.0).collect();
        assert_eq!(refs, [0x10, ganger.0]);
        // Another place forgets them.
        brought.remember(esm::FormId(0x3C), []);
        assert!(brought.people.is_empty());
    }

    #[test]
    fn mouse_motion_cannot_turn_player_during_script_package_and_releases_afterward() {
        let mut app = App::new();
        let mut mouse = ButtonInput::<MouseButton>::default();
        mouse.press(MouseButton::Right);
        let state = world::scripting::GameState::default();
        app.insert_resource(mouse)
            .insert_resource(AccumulatedMouseMotion {
                delta: Vec2::new(12.0, -8.0),
            })
            .insert_resource(dialogue::DialogueState(state))
            .insert_resource(walk::Player::new(true))
            .insert_resource(scripts::StartStage(Some(("TestOpening".into(), 0))))
            .add_systems(Update, look_around);
        let camera = app
            .world_mut()
            .spawn((
                Transform::default(),
                FlyCamera {
                    yaw: 0.3,
                    pitch: 0.2,
                    speed: 1.0,
                    start: (Vec3::ZERO, 0.3),
                },
            ))
            .id();
        app.update();
        let input = app.world().get::<FlyCamera>(camera).unwrap();
        assert_eq!((input.yaw, input.pitch), (0.3, 0.2));
        // The initial script runs after input in the application schedule.
        // It installs the first package; replacements must not leak input.
        app.world_mut().resource_mut::<scripts::StartStage>().0 = None;
        for frame in 0..600 {
            if frame % 180 == 0 {
                app.world_mut()
                    .resource_mut::<dialogue::DialogueState>()
                    .0
                    .script_packages
                    .insert(world::dialogue::PLAYER_REF, esm::FormId(1 + frame / 180));
            }
            app.update();
            let input = app.world().get::<FlyCamera>(camera).unwrap();
            assert_eq!((input.yaw, input.pitch), (0.3, 0.2), "frame {frame}");
        }
        app.world_mut()
            .resource_mut::<dialogue::DialogueState>()
            .0
            .script_packages
            .clear();
        app.update();
        let input = app.world().get::<FlyCamera>(camera).unwrap();
        assert!(input.yaw < 0.3 && input.pitch > 0.2);
        let previous = (input.yaw, input.pitch);
        app.world_mut()
            .resource_mut::<dialogue::DialogueState>()
            .0
            .script_packages
            .insert(world::dialogue::PLAYER_REF, esm::FormId(1));
        app.world_mut().resource_mut::<walk::Player>().walking = false;
        app.update();
        let input = app.world().get::<FlyCamera>(camera).unwrap();
        assert!(
            input.yaw < previous.0 && input.pitch > previous.1,
            "research free camera remains usable"
        );
    }

    #[test]
    fn textures_are_filtered_at_the_inis_anisotropy() {
        let mut ini = assets::IniSettings::default();
        assert_eq!(anisotropy_setting(&ini), 8, "Fallout_default.ini's");
        ini.add("[Display]\r\niMaxAnisotropy=15\r\n");
        assert_eq!(
            anisotropy_setting(&ini),
            15,
            "this install's FalloutPrefs.ini"
        );
        ini.add("[Display]\r\niMaxAnisotropy=0\r\n");
        assert_eq!(anisotropy_setting(&ini), 1, "off: plain trilinear");
    }

    #[test]
    fn every_texture_is_sampled_as_stored() {
        // The game never sets `D3DSAMP_SRGBTEXTURE`: its filtering averages
        // stored values. (sRGB formats would average decoded light, which
        // brightened the Goodsprings terrain and statics by 1-3%.)
        for format in [
            GpuFormat::Bc1,
            GpuFormat::Bc2,
            GpuFormat::Bc3,
            GpuFormat::Bc4,
            GpuFormat::Bc5,
            GpuFormat::Rgba8,
        ] {
            assert!(!gpu_format(format).is_srgb(), "{format:?}");
        }
    }

    #[test]
    fn fog_is_measured_from_the_games_near_plane() {
        // Goodsprings by day: fog from 10 to 120000 units, power 0.5. The
        // game's fog distance takes the projected depth, which starts at its
        // near plane (5 units): the shaders get that in `fog_range.z`.
        let fog = cellview::Fog {
            color: [0.588, 0.659, 0.745],
            near: 10.0,
            far: 120_000.0,
            power: 0.5,
        };
        let f = light_fields([0.0; 3], None, Some(&fog), 1.0);
        let m = space::METERS_PER_UNIT;
        assert!((f.fog_range.x - 10.0 * m).abs() < 1e-6);
        assert!((f.fog_range.z - 5.0 * m).abs() < 1e-6);
        assert_eq!(f.fog_range.w, 1.0);
        assert_eq!(f.fog_color.w, 0.5);
    }

    fn piece(blend: Blend, center: [f32; 3]) -> cellview::MeshData {
        cellview::MeshData {
            name: "piece".into(),
            shape_name: "piece".into(),
            positions: vec![[1.0, 2.0, 3.0], [4.0, 2.0, 3.0], [1.0, 5.0, 3.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            tangents: None,
            uvs: vec![[0.0; 2]; 3],
            colors: None,
            indices: vec![0, 1, 2],
            material: cellview::MaterialData {
                texture: None,
                color: [1.0; 4],
                blend,
                alpha_test: None,
                depth_test: true,
                depth_write: true,
                sort_center: center,
                unlit: false,
                double_sided: false,
                emissive: [0.0; 3],
                glow: None,
                decal: false,
                normal_map: None,
                specular: None,
                unlit_color: [1.0; 3],
                falloff: None,
                environment: None,
                emittance: None,
                shading: preview::cell::Shading::Plain,
                hair_tint: None,
            },
            effect: false,
            rig: None,
            motion: None,
            billboard: None,
            local_map: false,
            actor_part: None,
            strips: false,
        }
    }

    /// A place's lighting with every value set from `k`, so two differ
    /// everywhere.
    fn place_lighting(k: f32) -> GameLighting {
        let v = |i: f32| Vec4::splat(k + 0.1 * i);
        GameLighting {
            ambient: v(0.0),
            directional_color: v(1.0),
            directional_direction: v(2.0),
            emissive: v(3.0),
            scale: v(4.0),
            fog_color: v(5.0),
            fog_range: v(6.0),
            specular: v(7.0),
            surface: v(8.0),
            falloff: v(9.0),
            environment: v(10.0),
            draw: v(11.0),
            lights: [lighting::GameLight {
                position_radius: v(12.0),
                color: v(13.0),
            }; MAX_LIGHTS],
            actor: v(14.0),
            hair_tint: v(15.0),
        }
    }

    /// The player's body and first-person view are lit again, not rebuilt,
    /// when the place's lighting changes: their materials must come out as
    /// a rebuild in the new light would make them.
    #[test]
    fn a_lone_actor_lit_again_is_as_if_built_in_that_light() {
        let (before, after) = (place_lighting(1.0), place_lighting(2.0));
        let mut skin = piece(Blend::Opaque, [0.0; 3]);
        skin.material.shading = preview::cell::Shading::Skin;
        skin.material.texture = Some(0);
        skin.material.normal_map = Some(1);
        skin.material.specular = Some(cellview::Specular {
            color: [0.5; 3],
            glossiness: 10.0,
        });
        skin.tangents = Some(vec![[1.0, 0.0, 0.0, 1.0]; 3]);
        let mut hair = piece(Blend::Mask(0.5), [0.0; 3]);
        hair.material.shading = preview::cell::Shading::Hair;
        hair.material.hair_tint = Some([0.3, 0.2, 0.1]);
        let mut decal = piece(Blend::Blend, [1.0; 3]);
        decal.material.decal = true;
        let mut unlit = piece(Blend::Add, [1.0; 3]);
        unlit.material.unlit = true;
        let all = [skin, hair, decal, unlit];
        let textures = vec![Some(Handle::default()), Some(Handle::default())];
        let mut materials = Assets::<GameLitMaterial>::default();
        let pieces = LitPieces {
            textures: textures.clone(),
            lit: all
                .iter()
                .map(|d| {
                    let m = materials.add(lit_material(d, &textures, before));
                    (m, d.material.clone(), d.tangents.is_some())
                })
                .collect(),
        };
        pieces.relight(&mut materials, after);
        for ((handle, _, _), data) in pieces.lit.iter().zip(&all) {
            let got = materials.get(handle).unwrap();
            let want = lit_material(data, &textures, after);
            let old = lit_material(data, &textures, before);
            assert_ne!(old.extension.lighting, want.extension.lighting);
            assert_eq!(got.extension.lighting, want.extension.lighting);
            assert_eq!(got.extension.key, want.extension.key);
            assert_eq!(got.extension.normal_map, want.extension.normal_map);
            assert_eq!(got.base.base_color, want.base.base_color);
            assert_eq!(got.base.base_color_texture, want.base.base_color_texture);
            assert_eq!(got.base.alpha_mode, want.base.alpha_mode);
            assert_eq!(got.base.depth_bias, want.base.depth_bias);
            assert_eq!(got.base.unlit, want.base.unlit);
        }
    }

    #[test]
    fn cut_outs_are_tested_in_the_shader_and_tested_glass_blends() {
        // The alpha test is the shader's (the game's exact comparison), so
        // Bevy's own cut-out and coverage modes aren't used.
        assert_eq!(alpha_mode(Blend::Mask(0.5)), AlphaMode::Opaque);
        assert_eq!(alpha_mode(Blend::MaskedBlend(0.8)), AlphaMode::Blend);
        assert_eq!(alpha_mode(Blend::MaskedBlend(0.1)), AlphaMode::Blend);
        assert_eq!(alpha_mode(Blend::Add), AlphaMode::Add);
    }

    #[test]
    fn blended_pieces_are_built_around_their_sort_point() {
        let c = [2.0, 3.0, 3.0];
        // Opaque pieces keep their own origin; blended ones are sorted by
        // the centre of their stored bound.
        assert_eq!(sort_center(&piece(Blend::Opaque, c)), None);
        assert_eq!(sort_center(&piece(Blend::Mask(0.5), c)), None);
        assert_eq!(sort_center(&piece(Blend::Blend, c)), Some(c));
        assert_eq!(sort_center(&piece(Blend::Add, c)), Some(c));
        let data = piece(Blend::Blend, c);
        let mesh = game_mesh_around(&data, c);
        let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(p)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("no positions");
        };
        assert_eq!(p[0], [-1.0, -1.0, 0.0]);
        let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(a)) =
            mesh.attribute(lighting::ATTRIBUTE_CORNER_B)
        else {
            panic!("no corners");
        };
        assert_eq!(a[0], [2.0, -1.0, 0.0]);
    }

    #[derive(Resource, Default)]
    struct Changed(Vec<AssetId<GameLitMaterial>>);

    fn note_changed(
        mut events: EventReader<AssetEvent<GameLitMaterial>>,
        mut changed: ResMut<Changed>,
    ) {
        for e in events.read() {
            if let AssetEvent::Modified { id } = e {
                changed.0.push(*id);
            }
        }
    }

    /// Outdoors every loaded square is given its lights again whenever a
    /// square comes or goes: only the materials whose lights differ are
    /// changed (and prepared again by the render world).
    #[test]
    fn only_materials_whose_lights_differ_are_changed() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<GameLitMaterial>()
            .init_resource::<Changed>()
            .add_systems(Last, note_changed);
        let lighting = place_lighting(1.0);
        let lit = |l: GameLighting| {
            lit_material_of(&piece(Blend::Opaque, [0.0; 3]).material, false, &[], l)
        };
        let (lights, count) = (lighting.lights, lighting.scale.y);
        let mut other = lighting;
        other.lights[1].color.x += 1.0;
        let mut fewer = lighting;
        fewer.scale.y -= 1.0;
        let handles = {
            let mut assets = app.world_mut().resource_mut::<Assets<GameLitMaterial>>();
            [lighting, other, fewer].map(|l| assets.add(lit(l)))
        };
        app.update();
        app.world_mut().resource_mut::<Changed>().0.clear();
        give_lights(
            &mut app.world_mut().resource_mut::<Assets<GameLitMaterial>>(),
            &handles,
            lights,
            count,
        );
        app.update();
        let changed = &app.world().resource::<Changed>().0;
        assert_eq!(changed, &[handles[1].id(), handles[2].id()]);
        let assets = app.world().resource::<Assets<GameLitMaterial>>();
        for h in &handles {
            let l = assets.get(h).unwrap().lighting();
            assert_eq!((l.lights, l.scale.y), (lights, count));
        }
    }
}
