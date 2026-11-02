//! The game's own code, started inside nv-rs.
//!
//! The engine crate holds FalloutNV.exe 1.4.0.525 translated function by
//! function on a model of the game's memory (docs/ENGINE_CRATE.md). This
//! crate starts it for the running game: it finds the player's own
//! FalloutNV.exe in the installation that holds `Data`, refuses any build
//! but 1.4.0.525 (the one every translated address belongs to), maps the
//! executable's data sections (read-only use of the file: nothing is
//! written to the installation, nothing from it is stored), runs the
//! executable's static initializers as `__cinit` does before `WinMain`
//! ([`engine::static_init`]) and the allocator start-up the game runs before
//! any form exists.
//!
//! The engine is not `Send`, `Sync` or `Clone` (its memory and test doubles
//! are `Rc<RefCell<..>>`); a [`Host`] is owned by one thread and handed to
//! the world as plain owned data. What it is connected to: docs/ENGINE_CRATE.md
//! ("In the running game").

pub mod game_file;
pub mod global_form;
pub mod player_move;
pub mod quest_form;
pub mod quest_vm;
pub mod save_out;
pub mod script_form;
pub mod uncap;

use engine::Engine;
use std::path::{Path, PathBuf};

/// The game's translated code, started on the player's executable.
pub struct Host {
    /// Boxed so it never moves: the thread host's coroutines keep the address they were
    /// first handed (`engine::thread_host`), and a game thread started during [`Host::start`]
    /// runs again after the engine went into the `Host`.
    pub engine: Box<Engine>,
    /// The executable whose data sections are mapped.
    pub exe: PathBuf,
    /// What the static initializers' run did (entries run, skipped).
    pub static_init: engine::static_init::Report,
    /// The quests run through the game's interpreter for the world
    /// ([`quest_vm`]).
    pub quests: quest_vm::QuestVm,
    /// The player's movement through the game's code, and what it
    /// reached that isn't translated ([`player_move`]).
    pub movement: player_move::PlayerMovement,
    /// `NiInit` (`00aa20b0`) at start-up: `None` when it ran to the end, else why
    /// not (not translated yet, or where it stopped; its misses are in
    /// [`Host::movement`] under the stage `niinit`).
    pub ni_init: Option<String>,
}

/// A frame of main's still in flight ([`Host::begin_main_frame`]) ends before the engine goes:
/// main's OS thread holds its address until then.
impl Drop for Host {
    fn drop(&mut self) {
        self.join();
    }
}

impl Host {
    /// Starts the game's code for the installation whose `Data` folder is
    /// `data`: its `FalloutNV.exe` ([`engine::exe::installed_exe`]),
    /// checked to be 1.4.0.525 ([`engine::exe::checked_sections`], the check
    /// `engine::exe::verify` makes) and mapped; then the static initializers
    /// ([`engine::static_init::run`], in `__cinit`'s order, the untranslated
    /// ones skipped and counted), the memory manager's among them replaced by
    /// [`start_allocator`] ([`STAND_INS`]); then the game objects forms need
    /// when they are made ([`start_game_objects`]). The error says which file and why (missing,
    /// not a PE file, another build).
    pub fn start(data: &Path) -> Result<Host, String> {
        let exe = engine::exe::installed_exe(data);
        if !exe.is_file() {
            return Err(format!(
                "{}: not found. nv-rs reads the game's own FalloutNV.exe (version \
                 1.4.0.525) from the installation whose Data folder it is given.",
                exe.display()
            ));
        }
        let mut engine = Box::new(Engine::with_exe(&exe)?);
        // The play loop's patches (`crates/play-patch`, empty on main) replace the entries with
        // the same addresses.
        for (address, function) in play_patch::funcs() {
            engine.register(address, function);
        }
        // `NV_UNCAP=1`: an opt-in fix, NOT 1:1 with the exe (off by default): `BSTimer`'s step
        // without its sleep, so the frame rate isn't held to the game's own limit ([`uncap`]).
        if uncap::wanted() {
            uncap::install(&mut engine);
            eprintln!("NV_UNCAP=1: BSTimer's step (00aa4ee0) runs without its sleep (not 1:1)");
        }
        // The game's clock (`GetTickCount`, `timeGetTime`, `QueryPerformanceCounter`) runs with
        // the real time from here on, `Sleep` moving it on too (`Engine::clock_now_ms`).
        engine.start_real_clock();
        // How the game's threads are ordered (`engine::thread_host::Scheme`): by default a
        // signal from the main thread runs the threads it wakes soon after (`NV_WAKE_DELAY`
        // calls later, at main's next platform call), a used-up slice ends at the next
        // blocking call (a wait or sleep; 50 slices at most) and game threads' timed waits
        // time out. `NV_THREADS=slice` keeps the scheme before; `NV_THREADS=` with any of
        // `nowake`, `platformcall` (slices end at any platform call) or `anycall`, `notimeout`
        // changes parts.
        engine.kernel.threads.scheme =
            engine::thread_host::Scheme::from_name(std::env::var("NV_THREADS").ok().as_deref())?;
        if let Some(delay) = std::env::var("NV_WAKE_DELAY")
            .ok()
            .and_then(|v| v.trim().parse().ok())
        {
            engine.kernel.threads.wake_delay = delay;
        }
        // The game's threads take turns every `NV_SLICE` calls (200000 unless set; 0: only at
        // waits and sleeps), as they would run side by side on Windows
        // (`engine::thread_host::preempt`).
        let slice = std::env::var("NV_SLICE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(200_000u64);
        if slice != 0 {
            engine::thread_host::start_slicing(&mut engine, slice);
        }
        // `NV_SOUND=out`: the game's sound on the player's default device, from when the game
        // makes its DirectSound device on; `NV_SOUND_RECORD=FILE.wav` also records what goes
        // to the device, with a log of what started when (`FILE.log`). Off by default.
        // `NV_SOUND=device` keeps the device's pace instead of the game's clock.
        if let Ok(mode @ ("out" | "device")) = std::env::var("NV_SOUND").as_deref() {
            let record = std::env::var_os("NV_SOUND_RECORD").map(std::path::PathBuf::from);
            engine::units::platform::dsound::want_output(&mut engine, record, mode == "device");
        }
        let mut movement = player_move::PlayerMovement::default();
        // Diagnostics: with `NV_MOVE_PROBE` set, start-up's `__mtinit` and `NiInit` are probe
        // runs too (they go on past misses with zero results and record them all).
        movement.probe = std::env::var_os("NV_MOVE_PROBE").is_some();
        start_crt_heap(&mut engine, &mut movement);
        start_crt_threads(&mut engine, &mut movement);
        start_crt_io(&mut engine, &mut movement);
        // The machine table before the C initializers: `__get_sse2_info` (`00ed1927`, from
        // the initializer `00ed454f`) asks the processor, which the table answers.
        load_adapter_table(&mut engine);
        start_cinit_float(&mut engine, &mut movement);
        let static_init = engine::static_init::run(&mut engine, &STAND_INS);
        // A skipped initializer leaves its global (a setting among them) unbuilt, silently
        // null for the code that reads it later: a strict run names each one on stderr.
        if !movement.probe {
            for line in static_init.skip_lines(&engine::static_init::table()) {
                eprintln!("static initializer skipped: {line}");
            }
        }
        let ni_init = if engine.is_translated(WIN_MAIN) {
            // The game's own `main` (`WinMain`, after the CRT start-up above) as its main
            // thread: `NiInit`, the INI files, the window, `Main`, the renderer, the scene
            // graph, the world, then its message loop, one frame per `Host::main_frame`.
            movement.start_main(&mut engine);
            None
        } else {
            let ni_init = start_ni(&mut engine, &mut movement);
            // `main` after `NiInit`: `Main::Main`, the renderer (its FPU side here), the scene
            // graph and `InitTES`'s `TES` (the movement's start-up stages).
            movement.start_up(&mut engine, start_renderer_fpu);
            ni_init
        };
        // `TES::TES` makes the save/load game singleton (`011ddf38`); when it stopped before
        // that, the host makes it as before ([`start_game_objects`]), for the forms.
        if engine.mem.u32(SAVE_LOAD_GAME) == 0 {
            start_game_objects(&mut engine);
        }
        Ok(Host {
            engine,
            exe,
            static_init,
            quests: quest_vm::QuestVm::default(),
            movement,
            ni_init,
        })
    }

    /// The start-up diagnostic line.
    pub fn describe(&self) -> String {
        format!(
            "Game code: {} (1.4.0.525), {} translated functions; static initializers: \
             {} run from the table, {} by address, {} by platform code, {} skipped \
             (not translated yet); NiInit: {}.",
            self.exe.display(),
            self.engine.translated_count(),
            self.static_init.rows,
            self.static_init.translated,
            self.static_init.platform,
            self.static_init.skipped(),
            match &self.ni_init {
                None => "ran".to_string(),
                Some(why) => format!("not run ({why})"),
            }
        )
    }
}

/// `main` (Xbox PDB, `0086a850`), the game's `WinMain`.
const WIN_MAIN: u32 = 0x0086_a850;

/// Where the player's Direct3D 9 adapter table is: `NV_D3D9_ADAPTER`, else
/// `%LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv` (made on this machine by `scripts/d3d9-capture`;
/// it describes the player's hardware and stays local).
pub fn adapter_table_path() -> Option<PathBuf> {
    std::env::var_os("NV_D3D9_ADAPTER")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("LOCALAPPDATA")
                .map(|d| PathBuf::from(d).join("nv-rs").join("d3d9_adapter.tsv"))
        })
}

/// Where the player's D3DX texture cache is: `NV_D3DX_CACHE`, else
/// `%LOCALAPPDATA%\nv-rs\d3dx-cache` (made on this machine by `scripts/d3dx-texture-capture`
/// from the player's own textures; it stays local).
pub fn texture_cache_path() -> Option<PathBuf> {
    std::env::var_os("NV_D3DX_CACHE")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("LOCALAPPDATA")
                .map(|d| PathBuf::from(d).join("nv-rs").join("d3dx-cache"))
        })
}

/// Gives the Direct3D 9 layer the player's adapter table ([`adapter_table_path`]) before the
/// renderer's start-up asks about the adapter. Without the file nothing is loaded: the first
/// Direct3D 9 query stops that stage and its message says how to capture the table.
fn load_adapter_table(e: &mut Engine) {
    if let Some(text) = adapter_table_path().and_then(|p| std::fs::read_to_string(p).ok()) {
        engine::units::platform::d3d9::load_adapter_table(e, &text);
    }
    if let Some(dir) = texture_cache_path() {
        engine::units::platform::d3dx::set_texture_cache(e, dir);
    }
}

/// `NiInit` (`00aa20b0`), which `main` calls with 0 at `0086a9f8`, after the
/// static initializers and before `InitTES`: the Gamebryo libraries' start-up
/// (the allocator holder kept, `00aa3050`, NiMain's pre-init hook, every
/// registered library's init). Run through the movement's miss recorder
/// (stage `niinit`) so a stop is recorded and start-up goes on; `None` when it
/// ran to the end. Nothing of it is done twice by [`start_allocator`]: with
/// the holder at `011f6118` already made (by `00866690` in the game, by
/// [`start_allocator`] here) `NiInit` keeps it, and `00aa2020`, which both
/// call, runs twice in the game too (`00866690` and `NiInit`'s `00aa3050`).
pub fn start_ni(e: &mut Engine, movement: &mut player_move::PlayerMovement) -> Option<String> {
    const NI_INIT: u32 = 0x00aa_20b0;
    if !e.is_translated(NI_INIT) {
        return Some("00aa20b0 not translated yet".into());
    }
    movement
        .guarded(e, "niinit", |e| {
            e.call(NI_INIT, &[0]);
            Ok(())
        })
        .err()
}

/// The CRT's thread start-up `__mtinit` (`00ed1327`) where `mainCRTStartup` runs it: after the
/// heap's start-up and before `__cinit` runs the static initializers (the CRT heap is the
/// engine's, platform). Through the movement's miss recorder (stage `crt`); a stop is kept and
/// start-up goes on.
pub fn start_crt_threads(e: &mut Engine, movement: &mut player_move::PlayerMovement) {
    const MTINIT: u32 = 0x00ed_1327;
    let _ = movement.guarded(e, "crt", |e| {
        e.call(MTINIT, &[]);
        Ok(())
    });
}

/// The first step of `mainCRTStartup` (`00ecc3ca`): `_heap_init(1)` (`00eddb6a`), which
/// makes `_crtheap` (`HeapCreate`; the CRT's allocations themselves are the engine's heap,
/// platform); `__mtinitlocknum` and `_malloc` end the program while it is 0. A failure (0)
/// would end the process (`fast_error_exit(0x1c)`); here it is reported by the miss recorder
/// (stage `crt`) and start-up goes on.
pub fn start_crt_heap(e: &mut Engine, movement: &mut player_move::PlayerMovement) {
    const HEAP_INIT: u32 = 0x00ed_db6a;
    let _ = movement.guarded(e, "crt", |e| {
        if e.call(HEAP_INIT, &[1]).u32() == 0 {
            return Err("_heap_init answered 0".to_string());
        }
        Ok(())
    });
}

/// What `mainCRTStartup` runs after `__mtinit` (from `00ecc3ee`): `_RTC_Initialize`
/// (`00eddb1e`) and `_ioinit` (`00ed7e18`, the handle table the CRT's files use; its failure
/// would end the process with `_amsg_exit(0x1b)`, here it is reported by the miss recorder).
/// The command line and environment steps after them (`GetCommandLineA`,
/// `__crtGetEnvironmentStringsA`, `_setargv`, `_setenvp`) are not run yet. Through the
/// movement's miss recorder (stage `crt`); a stop is kept and start-up goes on.
pub fn start_crt_io(e: &mut Engine, movement: &mut player_move::PlayerMovement) {
    const RTC_INITIALIZE: u32 = 0x00ed_db1e;
    const IOINIT: u32 = 0x00ed_7e18;
    let _ = movement.guarded(e, "crt", |e| {
        e.call(RTC_INITIALIZE, &[]);
        let answer = e.call(IOINIT, &[]).i32();
        if answer < 0 {
            return Err(format!("_ioinit answered {answer}"));
        }
        Ok(())
    });
}

/// `_FPinit` (`010f1f1c`, in `.rdata`): the address of `_fpmath` (`00ec5f4d`) in the exe.
const FP_INIT: u32 = 0x010f_1f1c;
/// `__initp_misc_cfltcvt_tab` (`00ed248e`).
const INITP_MISC_CFLTCVT_TAB: u32 = 0x00ed_248e;

/// The head of `__cinit` (`00eccd14`), before its initializer tables, in its order:
/// `if (_FPinit != NULL && _IsNonwritableInCurrentImage(&_FPinit)) (*_FPinit)(floatflag)`,
/// then `__initp_misc_cfltcvt_tab`. `mainCRTStartup` passes `floatflag` 1 (`PUSH EBX` with
/// EBX 1 at `00ecc43e`). `_IsNonwritableInCurrentImage` (`00eddcc0`) is true in the game
/// (`_FPinit` lies in the image's read-only `.rdata`), so the call is made when the pointer
/// is set. `_fpmath` fills `_cfltcvt_tab` (`sscanf`'s `%f` goes through it), runs the
/// Pentium `FDIV` test and sets the default precision. Through the movement's miss recorder
/// (stage `crt`); a stop is kept and start-up goes on.
pub fn start_cinit_float(e: &mut Engine, movement: &mut player_move::PlayerMovement) {
    let _ = movement.guarded(e, "crt", |e| {
        let fp_init = e.mem.u32(FP_INIT);
        if fp_init != 0 {
            e.call(fp_init, &[1]);
        }
        e.call(INITP_MISC_CFLTCVT_TAB, &[]);
        Ok(())
    });
}

/// What creating the renderer's Direct3D device does to the main thread's FPU,
/// at the point `main` creates it: `0086b028` calls `0086d500`, whose `Init`
/// (`004da670`) creates the renderer at `004daa4c` (`NiDX9Renderer::Create`
/// `00e76210` → `NiDX9Renderer::Initialize` `00e72e60` → `CreateDevice`), after
/// `NiInit` (`0086a9f8`) and before `InitTES` (`0086b0e8`) and the AI threads
/// (`0086b12d`). The game's behaviour flags never hold
/// `D3DCREATE_FPU_PRESERVE` (`engine::units::platform::direct3d_device_created`),
/// so from here on the main thread runs at 24-bit precision (control word
/// `0x007F`, measured), as the player's mover and the character controller do
/// in the game; threads made later start at `0x027F`. The device itself is the
/// viewer's (platform).
pub fn start_renderer_fpu(e: &mut Engine) {
    engine::units::platform::direct3d_device_created(e, 0);
}

/// The save/load game singleton `BGSSaveLoadGame::Create` (`00847480`) stores (`011ddf38`).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;

/// The objects of the game's start-up (`WinMain`) that making forms needs,
/// made by the game's own code. Today one: `BGSSaveLoadGame::Create`
/// (Xbox PDB, `00847480`), which `TES::TES` (`0044fb20`) calls: the save/load
/// game singleton at `011ddf38`, which every form constructor that records a
/// change (`TESForm` `00484b60`/`00484b90` → `BGSSaveLoadGame::AddChange`
/// `0084a690`) reads; the game makes it before it loads any form. The rest
/// of `TES::TES` doesn't run: eight of its callees aren't translated
/// (`00c44980`, `004ba280`, `00528cb0`, `00626fd0`, `0084fc70`, `00856380`,
/// `00c3da50`, `00624bf0`).
pub fn start_game_objects(e: &mut Engine) {
    e.call(0x0084_7480, &[]);
}

/// The static initializers nv-rs runs its own code for. Entry `00fb66a0`
/// constructs the `MemoryManager` (Xbox PDB) singleton at `011f6238`
/// (`00aa3880`): its fields, a memory check, `MemoryManager::Initialize`,
/// the Ni allocator set-up `00866690` and `00aa58d0`. The memory manager is
/// platform (the engine's heap, `Mem`), so in its place, at the same point of
/// the table, runs the allocator set-up of `00866690` ([`start_allocator`]),
/// which the settings' maps need (`00aa1070` allocates through `011f6080`).
pub const STAND_INS: [engine::static_init::StandIn; 1] = [(0x00fb_66a0, start_allocator)];

/// The Ni allocator set-up of `00866690` (called by the memory manager's
/// constructor, `00aa3880`), as the engine's probes and the opt-in
/// interpreter (`crates/engine/tests/real_scda.rs`) run it: `operator new` (`00401000`) of the holder (8 bytes) and of a
/// `BSNiAllocator` (Xbox PDB, 4 bytes) that `00aa2230` gives its vtable,
/// the holder made by `00aa21d0` (allocator stored, byte +4 cleared) and
/// stored at `011f6118`, then `00aa2020`, which resolves that allocator and
/// calls its `Initialize` (slot +0x1c). (`00866690` ends with a call to a
/// library singleton's slot +4 that is not reproduced.)
pub fn start_allocator(e: &mut Engine) {
    // `00aa3880` on the memory manager (`011f6238`), the parts that are game objects and not the
    // heaps: `kThreadScrapHeapMap` (+0x114: `00aa47e0(map, 0x25)`, then its vtable `010a25f8`),
    // which `MemoryManager::GetThreadScrapHeap` (`00aa42e0`) looks the threads up in.
    e.call(0x00aa_47e0, &[0x011f_6238 + 0x114, 0x25]);
    e.mem.set_u32(0x011f_6238 + 0x114, 0x010a_25f8);
    // `00866690`: `operator new(8)` for the holder, `operator new(4)` for the
    // allocator and its constructor `00aa2230`, the holder's constructor
    // `00aa21d0`, the store at `011f6118`, then `00aa2020`. (Its null checks
    // after each `operator new` never fail on the engine's heap.)
    let holder = e.call(0x0040_1000, &[8]).u32();
    let allocator = e.call(0x0040_1000, &[4]).u32();
    let allocator = e.call(0x00aa_2230, &[allocator]).u32();
    let holder = e.call(0x00aa_21d0, &[holder, allocator]).u32();
    e.mem.set_u32(0x011f_6118, holder);
    e.call(0x00aa_2020, &[]);
    // Not reproduced: `00866690`'s last call, slot +4 of the singleton
    // `00af2640` answers with (0, 0, 0), and the heap set-up of `00aa3880` (the
    // memory manager's heaps are platform).
    // `00aa3880`'s last step: the scrap-heap manager (`00aa58d0`).
    e.call(0x00aa_58d0, &[]);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `NiInit` runs when translated, through the miss recorder: a stop is
    /// kept (with its misses under `niinit`) and start-up goes on.
    #[test]
    fn ni_init_runs_through_the_recorder_or_says_why_not() {
        let mut e = Engine::new();
        let mut movement = player_move::PlayerMovement::default();
        e.register(0x00aa_20b0, |e, a| {
            assert_eq!(a, [0]);
            e.call(0x00ab_cdef, &[])
        });
        let why = start_ni(&mut e, &mut movement).expect("stopped");
        assert!(why.starts_with("open function 00abcdef"), "{why}");
        assert_eq!(movement.misses[&0x00ab_cdef].stage, "niinit");
        e.register(0x00aa_20b0, |_, _| engine::Ret::default());
        assert_eq!(start_ni(&mut e, &mut movement), None);
    }

    /// `__cinit`'s head calls `_fpmath(1)` through `_FPinit` and then encodes the table.
    #[test]
    fn the_cinit_float_set_up_runs_fpmath_then_encodes_the_table() {
        let mut e = Engine::new();
        let mut movement = player_move::PlayerMovement::default();
        e.mem.map(0x010f_1000, 0x1000);
        e.mem.set_u32(FP_INIT, 0x00ec_5f4d);
        let calls = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(0x00ec_5f4d, move |_, a| {
            log.borrow_mut().push((0x00ec_5f4d, a.to_vec()));
            engine::Ret::default()
        });
        let log = calls.clone();
        e.register_double(INITP_MISC_CFLTCVT_TAB, move |_, a| {
            log.borrow_mut().push((INITP_MISC_CFLTCVT_TAB, a.to_vec()));
            engine::Ret::default()
        });
        start_cinit_float(&mut e, &mut movement);
        assert_eq!(
            *calls.borrow(),
            vec![(0x00ec_5f4d, vec![1]), (INITP_MISC_CFLTCVT_TAB, vec![])]
        );
        assert!(movement.misses.is_empty());
    }

    /// After the renderer's start-up point the main thread computes at 24-bit
    /// precision (0x007F), as measured in the game.
    #[test]
    fn the_renderer_start_up_sets_24_bit_precision_on_the_main_thread() {
        let mut e = Engine::new();
        assert_eq!(engine::x87::control_word(), 0x027f);
        start_renderer_fpu(&mut e);
        assert_eq!(engine::units::crt::x87_control_word(&mut e), 0x007f);
        assert_eq!(engine::x87::x87_div(1.0, 3.0), f64::from(1.0f32 / 3.0));
    }

    #[test]
    fn an_installation_without_the_executable_is_refused_by_path() {
        let data = std::env::temp_dir()
            .join("nv-rs-host-test-no-game")
            .join("Data");
        let err = match Host::start(&data) {
            Ok(_) => panic!("started without an executable"),
            Err(err) => err,
        };
        assert!(err.contains("FalloutNV.exe: not found"), "{err}");
        assert!(err.contains("1.4.0.525"), "{err}");
    }

    /// With `NV_EXE` set to an installed FalloutNV.exe 1.4.0.525: the game
    /// code starts on its folder, the allocator holder is in place and
    /// `operator new` hands out memory.
    #[test]
    fn starts_on_the_installed_executable_when_given() {
        let Some(path) = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty()) else {
            return;
        };
        let exe = PathBuf::from(path);
        let data = exe.parent().unwrap().join("Data");
        let mut host = Host::start(&data).unwrap();
        assert_eq!(host.exe, exe);
        assert_ne!(host.engine.mem.u32(0x011f_6118), 0);
        let block = host.engine.call(0x0040_1000, &[0x20]).u32();
        assert_ne!(block, 0);
        assert!(host.describe().contains("1.4.0.525"));
    }
}
