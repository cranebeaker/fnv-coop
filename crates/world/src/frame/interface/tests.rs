use super::*;
use crate::frame::ai_stage::{self, ThreadOp};
use crate::frame::{FrameState, Gate, STEPS};

/// `research/engine-map/frame.tsv`, the committed call tree.
const FRAME_TSV: &str = include_str!("../../../../../research/engine-map/frame.tsv");

/// The rows of `frame.tsv` as (depth, address).
fn rows() -> Vec<(u32, u32)> {
    FRAME_TSV
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            (
                f[1].parse().unwrap(),
                u32::from_str_radix(f[3], 16).unwrap(),
            )
        })
        .collect()
}

/// The direct calls `frame.tsv` lists under the first row of `f` at depth 1
/// (called by `Main::OnIdle`) or 2 (called by such a function).
fn tsv_callees(f: &Function) -> Option<Vec<u32>> {
    let rows = rows();
    let c = f.callers[0];
    let (depth, start) = if c.function == ON_IDLE {
        let i = rows.iter().position(|&(d, a)| d == 1 && a == f.address)?;
        (1, i)
    } else {
        let parent = function(c.function)?;
        if parent.callers[0].function != ON_IDLE {
            return None;
        }
        let p = rows
            .iter()
            .position(|&(d, a)| d == 1 && a == parent.address)?;
        let i = p + rows[p..]
            .iter()
            .position(|&(d, a)| d == 2 && a == f.address)?;
        (2, i)
    };
    Some(
        rows[start + 1..]
            .iter()
            .take_while(|&&(d, _)| d > depth)
            .filter(|&&(d, _)| d == depth + 1)
            .map(|&(_, a)| a)
            .collect(),
    )
}

/// Each function `frame.tsv` reaches (depth 1 or 2) has its direct calls
/// listed under it in the same order (the model leaves queries out); the
/// depth-3 ones (`0070b8f0`, `0070c4a0`, `00711ea0`, `00713c70`) aren't
/// listed.
#[test]
fn direct_calls_are_in_frame_tsv_in_order() {
    let mut checked = Vec::new();
    for f in &FUNCTIONS {
        let Some(listed) = tsv_callees(f) else {
            continue;
        };
        checked.push(f.address);
        let mut sites: Vec<(u32, u32)> = f
            .steps
            .iter()
            .filter_map(|st| match st.callee {
                Callee::Direct(a) => Some(st.sites.iter().map(move |&s| (s, a))),
                _ => None,
            })
            .flatten()
            .collect();
        sites.sort_unstable();
        let mut at = 0;
        for (_, a) in sites {
            let p = listed[at..]
                .iter()
                .position(|&x| x == a)
                .unwrap_or_else(|| panic!("{:08x}: {a:08x} not in order", f.address));
            at += p + 1;
        }
    }
    assert!(checked.len() >= 14, "{checked:08x?}");
    assert!(!checked.contains(&MANAGER_IDLE));
}

#[test]
fn sites_and_tests_are_inside_their_functions_in_order() {
    for f in &FUNCTIONS {
        let firsts: Vec<u32> = f.steps.iter().map(|s| s.sites[0]).collect();
        assert!(firsts.windows(2).all(|w| w[0] < w[1]), "{:08x}", f.address);
        for st in f.steps {
            let a = f.address;
            assert!(st.sites.windows(2).all(|w| w[0] < w[1]), "{a:08x}");
            assert!(st.sites.iter().all(|&s| f.contains(s)), "{a:08x}");
            let b = st.gate.branches();
            assert_eq!(b.is_empty(), st.gate == SubGate::Always, "{a:08x}");
            assert!(
                b.iter().all(|&x| f.contains(x) && x < st.sites[0]),
                "{a:08x}: {:?}",
                st.gate
            );
            let last = *st.sites.last().unwrap();
            assert!(
                st.own_tests.iter().all(|&x| f.contains(x) && x < last),
                "{:08x}",
                st.sites[0]
            );
        }
    }
}

/// Each caller calls there: a frame step of that function, a sub-step of a
/// modelled function, or an AI thread's call.
#[test]
fn callers_call_there() {
    for f in &FUNCTIONS {
        for c in f.callers {
            if c.function == ON_IDLE {
                assert!(STEPS.iter().any(|s| s.address == f.address));
            } else if let Some(g) = function(c.function) {
                let k = step_at(g, c.site).expect("the caller's sub-step");
                assert_eq!(g.steps[k].callee, Callee::Direct(f.address));
            } else {
                let t = ai_stage::thread(c.function).expect("an AI thread");
                assert!(t.steps.iter().any(|s| s.site == c.site
                    && matches!(s.op, ThreadOp::Call { callee, .. } if callee == f.address)));
            }
        }
    }
}

/// The interface idle is made once a frame, by the caller the thread count
/// and the AI work pick, as the frame's gates say.
#[test]
fn who_calls_the_interface_idle() {
    let idle: Vec<usize> = STEPS
        .iter()
        .enumerate()
        .filter(|(_, s)| s.address == DO_INTERFACE_IDLE)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(idle.len(), 2);
    for (threads, menu) in [
        (1, false),
        (1, true),
        (2, false),
        (2, true),
        (4, false),
        (4, true),
    ] {
        let s = FrameState {
            threads,
            in_menu_mode: menu,
            ..FrameState::default()
        };
        let c = idle_caller(&s).unwrap();
        assert_eq!(STEPS[idle[0]].gate.holds(&s), c.site == 0x0086_eb36);
        assert_eq!(STEPS[idle[1]].gate.holds(&s), c.site == 0x0086_ecba);
        let thread = Gate::AiThreadsStart.holds(&s);
        assert_eq!(thread, [0x008c_7be9, 0x008c_7db9].contains(&c.site));
        if thread {
            assert_eq!(ai_stage::threads_for(threads)[0].address, c.function);
        }
    }
    assert!(idle_caller(&FrameState {
        alt_tab_held: true,
        ..FrameState::default()
    })
    .is_none());
}

fn reached(f: &Function, s: &InterfaceState) -> Vec<u32> {
    steps_run(s, f)
        .iter()
        .map(|&i| f.steps[i].sites[0])
        .collect()
}

// `0070b952`, `0070b961`, `0070ba9b`.
#[test]
fn pre_idle_modes() {
    let f = function(MANAGER_PRE_IDLE).unwrap();
    let game = InterfaceState::default();
    assert_eq!(
        reached(f, &game),
        [
            0x0070_b946,
            0x0070_bae0,
            0x0070_bb03,
            0x0070_bb48,
            0x0070_bb79
        ]
    );
    let opening = InterfaceState {
        mode: mode::OPENING,
        ..game
    };
    let r = reached(f, &opening);
    assert_eq!(r.len(), 9);
    assert!(r.contains(&0x0070_b96a) && !r.contains(&0x0070_bae0));
    let locked = InterfaceState {
        fade_lock: true,
        ..opening
    };
    assert_eq!(reached(f, &locked), [0x0070_b946]);
    for m in [mode::MENUS, mode::CLOSING, mode::TAKING_OVER] {
        let s = InterfaceState { mode: m, ..game };
        assert_eq!(reached(f, &s), [0x0070_b946]);
    }
}

// `0070c887`, `0070c897`, `0070ca18`, `0070cbd5`, `0070cc72`, `0070cc7e`,
// `0070e72a`, `0070e820`.
#[test]
fn idle_parts() {
    let f = function(MANAGER_IDLE).unwrap();
    let game = InterfaceState::default();
    let r = reached(f, &game);
    assert!(r.contains(&0x0070_cbf1) && r.contains(&0x0070_cc63));
    assert!(!r.contains(&0x0070_cf97) && !r.contains(&0x0070_c936));
    assert!(r.contains(&0x0070_eb15) && r.contains(&0x0070_e7fb));
    // Two threads queue the start menu; one creates it.
    assert!(r.contains(&0x0070_e752) && !r.contains(&0x0070_e730));
    let one = InterfaceState { threads: 1, ..game };
    let r = reached(f, &one);
    assert!(r.contains(&0x0070_e730) && !r.contains(&0x0070_e752));
    let back = InterfaceState {
        mode_changed: true,
        ..game
    };
    assert!(reached(f, &back).contains(&0x0070_c936));
    assert!(!reached(f, &back).contains(&0x0070_ca97));
    for m in [mode::MENUS, mode::TAKING_OVER] {
        let s = InterfaceState { mode: m, ..game };
        let r = reached(f, &s);
        assert!(r.contains(&0x0070_e1cb) && r.contains(&0x0070_e27e));
        assert!(!r.contains(&0x0070_cbf1));
    }
    let taking = InterfaceState {
        mode: mode::TAKING_OVER,
        mode_changed: true,
        ..game
    };
    assert!(reached(f, &taking).contains(&0x0070_ca97));
    for m in [mode::OPENING, mode::CLOSING] {
        let s = InterfaceState { mode: m, ..game };
        let r = reached(f, &s);
        assert!(!r.contains(&0x0070_e1cb) && !r.contains(&0x0070_cbf1));
    }
    let stopped = InterfaceState {
        idle_stopped: true,
        ..game
    };
    let r = reached(f, &stopped);
    assert!(!r.contains(&0x0070_eb04) && !r.contains(&0x0070_eb15));
    assert!(r.contains(&0x0070_e7fb));
}

// `007122c1`, `007122d0`, `007122e2`.
#[test]
fn post_idle_gives_the_game_back() {
    let f = function(MANAGER_POST_IDLE).unwrap();
    let closing = InterfaceState {
        mode: mode::CLOSING,
        ..InterfaceState::default()
    };
    assert!(reached(f, &closing).contains(&0x0071_22e7));
    let fading = InterfaceState {
        fading: true,
        ..closing
    };
    assert!(!reached(f, &fading).contains(&0x0071_22e7));
    let tests = InterfaceState {
        cell_tests: true,
        ..fading
    };
    assert!(reached(f, &tests).contains(&0x0071_22e7));
    assert!(!reached(f, &InterfaceState::default()).contains(&0x0071_22e7));
}

#[test]
fn last_minute_update() {
    let f = function(MANAGER_LAST_MINUTE_UPDATE).unwrap();
    let s = InterfaceState::default();
    let r = reached(f, &s);
    assert!(!r.contains(&0x0071_3ca4) && r.contains(&0x0071_3d01) && !r.contains(&0x0071_3d35));
    let waiting = InterfaceState {
        threads_running: true,
        loading_menu: true,
        tile_queue_empty: false,
        ..s
    };
    let r = reached(f, &waiting);
    assert!(r.contains(&0x0071_3ca4) && r.contains(&0x0071_3cf8));
    assert!(!r.contains(&0x0071_3d01) && r.contains(&0x0071_3d35));
    let one = InterfaceState {
        threads: 1,
        ..waiting
    };
    assert!(!reached(f, &one).contains(&0x0071_3ca4));
    let w = function(LAST_MINUTE_UPDATE).unwrap();
    let not_ready = InterfaceState {
        manager_ready: false,
        ..s
    };
    assert!(reached(w, &not_ready).is_empty());
}

#[test]
fn sleeping() {
    let f = function(UPDATE_SLEEPING).unwrap();
    let s = InterfaceState::default();
    assert_eq!(reached(f, &s), [0x0070_56fc, 0x0070_5710]);
    let menu = InterfaceState {
        sleep_menu: true,
        ..s
    };
    assert_eq!(reached(f, &menu).len(), 3);
    let frame = FrameState {
        top_menu: SLEEP_WAIT_MENU,
        ..FrameState::default()
    };
    assert!(from_frame(&frame, false).sleep_menu);
    assert_eq!(SLEEP_WAIT_MENU, crate::frame::SLEEP_WAIT_MENU);
}

// `00870190`-`0087027a`, `0086ffa2`, `0087008a`, `0087009e`, `008705a6`.
#[test]
fn swap_draws_the_world_or_the_menus() {
    let f = function(SWAP).unwrap();
    let game = InterfaceState::default();
    let r = reached(f, &game);
    assert!(r.contains(&0x0087_0244) && !r.contains(&0x0087_02f7) && !r.contains(&0x0087_02a9));
    assert!(r.contains(&0x0087_05aa) && !r.contains(&0x0087_000f));
    let held = InterfaceState {
        background_held: true,
        menus_on_screen: true,
        ..game
    };
    let r = reached(f, &held);
    assert!(!r.contains(&0x0087_0244) && r.contains(&0x0087_02f7));
    let under = InterfaceState {
        menus_on_screen: false,
        ..held
    };
    assert!(reached(f, &under).contains(&0x0087_02a9));
    let changed = InterfaceState {
        menu_changed: true,
        faders_up: true,
        threads: 1,
        ..game
    };
    let r = reached(f, &changed);
    assert!(r.contains(&0x0087_000f) && r.contains(&0x0087_00b9));
    assert!(!r.contains(&0x0087_05aa));
}

// `0086f8a4`-`0086f920`.
#[test]
fn process_lists_after_the_swap() {
    let f = function(UPDATE_PROCESS_LISTS).unwrap();
    let game = InterfaceState::default();
    assert_eq!(reached(f, &game), [0x0086_f905]);
    let one = InterfaceState { threads: 1, ..game };
    assert_eq!(
        reached(f, &one),
        [0x0086_f8ab, 0x0086_f8f0, 0x0086_f8fb, 0x0086_f905]
    );
    let menu = InterfaceState {
        menu_flag: true,
        ..game
    };
    assert_eq!(reached(f, &menu), [0x0086_f928]);
    let fader = InterfaceState {
        fader_visible: true,
        ..menu
    };
    assert_eq!(reached(f, &fader), [0x0086_f905]);
    let console = InterfaceState {
        console_visible: true,
        ..game
    };
    assert!(reached(f, &console).is_empty());
    // The frame's own test for the process lists says the same.
    for (menu, fader, console, frozen) in [
        (false, false, false, false),
        (true, false, false, false),
        (true, true, false, false),
        (false, false, true, false),
        (false, false, false, true),
    ] {
        let frame = FrameState {
            in_menu_mode: menu,
            fader_visible: fader,
            console_visible: console,
            world_frozen: frozen,
            threads: 4,
            ..FrameState::default()
        };
        assert_eq!(from_frame(&frame, false).lists_run(), frame.process_lists());
    }
}

// `0086f3c6`-`0086f434`.
#[test]
fn the_poll_clears_actions_in_playback_and_as_the_pipboy_comes_up() {
    let f = function(POLL_CONTROLS).unwrap();
    let game = InterfaceState::default();
    assert_eq!(reached(f, &game), [0x0086_f39e, 0x0086_f3b8]);
    let playback = InterfaceState {
        vats_playback: true,
        ..game
    };
    assert_eq!(reached(f, &playback).len(), 3);
    let opening = InterfaceState {
        pipboy_opening: true,
        ..game
    };
    assert_eq!(reached(f, &opening).len(), 3);
    for s in [
        InterfaceState {
            fly_camera: true,
            ..opening
        },
        InterfaceState {
            message_menu: true,
            ..opening
        },
        InterfaceState {
            vats_test: true,
            ..playback
        },
    ] {
        assert_eq!(reached(f, &s).len(), 2);
    }
    let frame = FrameState {
        vats_mode: 4,
        ..FrameState::default()
    };
    assert!(from_frame(&frame, false).vats_playback);
}

#[test]
fn open_console_post_threads_and_the_dialogue() {
    let f = function(OPEN_CONSOLE).unwrap();
    let s = InterfaceState::default();
    assert_eq!(reached(f, &s).len(), 2);
    let shown = InterfaceState {
        console_visible: true,
        ..s
    };
    assert!(reached(f, &shown).is_empty());
    let p = function(POST_THREADS_PROCESS).unwrap();
    assert_eq!(reached(p, &s).len(), 7);
    let one = InterfaceState { threads: 1, ..s };
    assert_eq!(reached(p, &one).len(), 4);
    let n = function(NON_RENDER_SAFE_AI_TASKS).unwrap();
    let talk = InterfaceState {
        in_dialog: true,
        ..s
    };
    assert!(reached(n, &talk).contains(&0x0086_f6cd));
    assert!(!reached(n, &s).contains(&0x0086_f6cd));
}

#[test]
fn follows_checks_order_and_unreached_calls() {
    let f = function(POST_SWAP_PROCESS).unwrap();
    let s = InterfaceState::default();
    let ok = [
        Callee::Direct(AUDIO_UPDATE),
        Callee::Direct(UPDATE_PROCESS_LISTS),
        Callee::Direct(0x0055_2570),
        Callee::Direct(0x0086_f670),
    ];
    assert!(follows(f, &s, &ok, &[]).is_ok());
    let one = InterfaceState { threads: 1, ..s };
    assert!(follows(f, &one, &ok, &[]).is_err());
    let wrong = [ok[1], ok[0], ok[2], ok[3]];
    assert!(follows(f, &s, &wrong, &[]).is_err());
}

#[test]
fn wiring() {
    let partial: Vec<u32> = FUNCTIONS
        .iter()
        .flat_map(|f| f.steps.iter())
        .filter(|s| matches!(s.wiring, Wiring::Partial(_)))
        .map(|s| s.sites[0])
        .collect();
    assert!(FUNCTIONS
        .iter()
        .flat_map(|f| f.steps.iter())
        .all(|s| s.wiring.is_open()));
    assert_eq!(
        partial,
        [
            0x0086_fd7c,
            0x0070_b946,
            0x0070_2836,
            0x0070_e730,
            0x0070_e752,
            0x0070_e7fb,
            0x0070_e913,
            0x0070_eb04,
            0x0070_eb15,
            0x0071_20ef,
            0x0070_574c,
            0x0087_212d,
            0x0087_02f7,
            0x0087_05da,
            0x0087_05e2,
            0x0086_f650,
            0x0086_f657,
            0x0086_f65f,
            0x0086_f905,
            // `Controls::Poll` (Phase 1 PR 8).
            0x0086_f39e,
        ]
    );
}
