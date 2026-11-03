//! Weather mirroring on a guest through the game's own weather calls.

use esm::FormId;
use world::coop::env::{apply_sky, read_sky, Applied, SkyShot};
use world::weather::WeatherState;

const CLEAR: u32 = 0x100;
const RAIN: u32 = 0x200;
const STORM: u32 = 0x300;

fn host_with(w: u32) -> WeatherState {
    let mut s = WeatherState::default();
    s.force(FormId(w), false, 8.0);
    s
}

#[test]
fn read_sky_reports_the_fade_and_the_override() {
    let mut s = host_with(CLEAR);
    assert_eq!(
        read_sky(&s),
        SkyShot {
            current: CLEAR,
            previous: 0,
            forced: 0,
            fade_milli: 1000,
            started_centi: 800
        }
    );
    s.set(FormId(RAIN), true, 9.25);
    s.previous = s.current;
    s.current = Some(FormId(RAIN));
    s.fade = 0.4;
    s.started = 9.25;
    let r = read_sky(&s);
    assert_eq!(
        (
            r.current,
            r.previous,
            r.forced,
            r.fade_milli,
            r.started_centi
        ),
        (RAIN, CLEAR, RAIN, 400, 925)
    );
}

#[test]
fn first_sky_is_installed_as_the_hosts_override() {
    let mut g = WeatherState::default();
    let shot = read_sky(&host_with(CLEAR));
    assert_eq!(apply_sky(&mut g, &shot, 8.0), Applied::Cut);
    assert_eq!(g.current, Some(FormId(CLEAR)));
    assert_eq!(
        g.forced,
        Some(FormId(CLEAR)),
        "the guest never rolls its own"
    );
    assert_eq!(g.fade, 1.0);
    assert_eq!(apply_sky(&mut g, &shot, 8.1), Applied::Same);
}

#[test]
fn a_host_fade_starts_a_fade_on_the_guest_without_a_cut() {
    let mut g = WeatherState::default();
    apply_sky(&mut g, &read_sky(&host_with(CLEAR)), 8.0);
    let shot = SkyShot {
        current: RAIN,
        previous: CLEAR,
        forced: 0,
        fade_milli: 100,
        started_centi: 905,
    };
    assert_eq!(apply_sky(&mut g, &shot, 9.05), Applied::Transition);
    assert_eq!(g.current, Some(FormId(RAIN)));
    assert_eq!(
        g.previous,
        Some(FormId(CLEAR)),
        "the old weather is still fading out"
    );
    assert!((g.fade - 0.1).abs() < 1e-3);
    assert_eq!(g.started, 9.05);
    assert_eq!(g.forced, Some(FormId(RAIN)));
}

#[test]
fn a_missed_change_is_a_cut_to_the_hosts_weather() {
    let mut g = WeatherState::default();
    apply_sky(&mut g, &read_sky(&host_with(CLEAR)), 8.0);
    let shot = SkyShot {
        current: STORM,
        previous: RAIN,
        forced: 0,
        fade_milli: 500,
        started_centi: 1000,
    };
    assert_eq!(apply_sky(&mut g, &shot, 10.0), Applied::Cut);
    assert_eq!(g.current, Some(FormId(STORM)));
    assert_eq!(g.previous, Some(FormId(RAIN)));
    assert!((g.fade - 0.5).abs() < 1e-3);
}

#[test]
fn small_fade_drift_is_left_big_drift_is_corrected() {
    let mut g = WeatherState::default();
    apply_sky(&mut g, &read_sky(&host_with(CLEAR)), 8.0);
    apply_sky(
        &mut g,
        &SkyShot {
            current: RAIN,
            previous: CLEAR,
            forced: 0,
            fade_milli: 100,
            started_centi: 905,
        },
        9.05,
    );
    g.fade = 0.15;
    let near = SkyShot {
        current: RAIN,
        previous: CLEAR,
        forced: 0,
        fade_milli: 200,
        started_centi: 905,
    };
    assert_eq!(apply_sky(&mut g, &near, 9.1), Applied::Same);
    assert!((g.fade - 0.15).abs() < 1e-6, "untouched");
    let far = SkyShot {
        fade_milli: 600,
        ..near
    };
    assert_eq!(apply_sky(&mut g, &far, 9.2), Applied::FadeCorrected);
    assert!((g.fade - 0.6).abs() < 1e-3);
}

#[test]
fn a_finished_host_fade_finishes_the_guests() {
    let mut g = WeatherState::default();
    apply_sky(&mut g, &read_sky(&host_with(CLEAR)), 8.0);
    apply_sky(
        &mut g,
        &SkyShot {
            current: RAIN,
            previous: CLEAR,
            forced: 0,
            fade_milli: 100,
            started_centi: 905,
        },
        9.05,
    );
    g.fade = 0.3;
    let done = SkyShot {
        current: RAIN,
        previous: 0,
        forced: RAIN,
        fade_milli: 1000,
        started_centi: 905,
    };
    assert_eq!(apply_sky(&mut g, &done, 9.6), Applied::FadeCorrected);
    assert_eq!(g.previous, None);
    assert_eq!(g.fade, 1.0);
}

#[test]
fn a_guest_that_missed_the_fade_start_joins_it() {
    let mut g = WeatherState::default();
    apply_sky(&mut g, &read_sky(&host_with(RAIN)), 8.0);
    let shot = SkyShot {
        current: RAIN,
        previous: CLEAR,
        forced: RAIN,
        fade_milli: 700,
        started_centi: 900,
    };
    assert_eq!(apply_sky(&mut g, &shot, 9.2), Applied::FadeCorrected);
    assert_eq!(g.previous, Some(FormId(CLEAR)));
}

#[test]
fn an_empty_sky_changes_nothing() {
    let mut g = host_with(CLEAR);
    assert_eq!(apply_sky(&mut g, &SkyShot::default(), 8.0), Applied::Same);
    assert_eq!(g.current, Some(FormId(CLEAR)));
}
