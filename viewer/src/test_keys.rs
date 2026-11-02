//! `--key-at`: for testing, presses a key or mouse button that many
//! seconds after the player is placed, held for a while, as if it were
//! pressed on the keyboard (the viewer's input, not the game's: the view
//! key F, R to reload or, held, put the weapon away or draw it, the left
//! button to attack). Lets pictures show what only the keys reach
//! (third person, a holstered weapon) without sending input to the
//! desktop.

use bevy::input::ButtonInput;
use bevy::prelude::*;

/// A key or mouse button the option can press.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Press {
    Key(KeyCode),
    Mouse(MouseButton),
    /// The mouse moved sideways this many counts every frame it's held.
    MouseX(f32),
    /// The mouse moved up or down this many counts every frame it's held
    /// (positive down).
    MouseY(f32),
    /// The wheel turned this many notches every frame it's held (negative
    /// toward the player: out).
    Wheel(f32),
}

/// A press: when (seconds after the player is placed), what, and how long
/// it's held.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimedPress {
    pub at: f32,
    pub what: Press,
    pub hold: f32,
}

/// Parses `KEY` or `KEY:HOLD` (`space`, a letter A–Z, a digit, `mouse-left`,
/// `mouse-right`, `mouse-x=COUNTS` / `mouse-y=COUNTS`: the mouse moved
/// sideways / down that much
/// each frame; `wheel=NOTCHES`: the wheel turned that much each frame;
/// HOLD in seconds, 0.1 by default).
pub fn parse(at: f32, v: &str) -> Result<TimedPress, String> {
    let (name, hold) = match v.split_once(':') {
        Some((n, h)) => (
            n,
            h.parse::<f32>()
                .ok()
                .filter(|s| s.is_finite() && *s > 0.0)
                .ok_or_else(|| format!("--key-at expects KEY or KEY:SECONDS, got '{v}'"))?,
        ),
        None => (v, 0.1),
    };
    let what = match name.to_ascii_lowercase().as_str() {
        "mouse-left" => Press::Mouse(MouseButton::Left),
        "mouse-right" => Press::Mouse(MouseButton::Right),
        // The pause menu's key.
        "escape" => Press::Key(KeyCode::Escape),
        // Jump (control 12).
        "space" => Press::Key(KeyCode::Space),
        // The Pip-Boy's key (let go before the light's hold time).
        "tab" => Press::Key(KeyCode::Tab),
        n if n.starts_with("mouse-x=") || n.starts_with("mouse-y=") || n.starts_with("wheel=") => {
            let (kind, amount) = n.split_once('=').unwrap_or_default();
            let amount = amount
                .parse::<f32>()
                .ok()
                .filter(|c| c.is_finite())
                .ok_or_else(|| format!("--key-at: {kind}= expects a number, got '{v}'"))?;
            match kind {
                "wheel" => Press::Wheel(amount),
                "mouse-y" => Press::MouseY(amount),
                _ => Press::MouseX(amount),
            }
        }
        n if n.len() == 1 => {
            let c = n.as_bytes()[0];
            let code = match c {
                b'a'..=b'z' => LETTERS[usize::from(c - b'a')],
                b'0'..=b'9' => DIGITS[usize::from(c - b'0')],
                _ => return Err(format!("--key-at: unknown key '{name}'")),
            };
            Press::Key(code)
        }
        _ => return Err(format!("--key-at: unknown key '{name}'")),
    };
    Ok(TimedPress { at, what, hold })
}

const LETTERS: [KeyCode; 26] = [
    KeyCode::KeyA,
    KeyCode::KeyB,
    KeyCode::KeyC,
    KeyCode::KeyD,
    KeyCode::KeyE,
    KeyCode::KeyF,
    KeyCode::KeyG,
    KeyCode::KeyH,
    KeyCode::KeyI,
    KeyCode::KeyJ,
    KeyCode::KeyK,
    KeyCode::KeyL,
    KeyCode::KeyM,
    KeyCode::KeyN,
    KeyCode::KeyO,
    KeyCode::KeyP,
    KeyCode::KeyQ,
    KeyCode::KeyR,
    KeyCode::KeyS,
    KeyCode::KeyT,
    KeyCode::KeyU,
    KeyCode::KeyV,
    KeyCode::KeyW,
    KeyCode::KeyX,
    KeyCode::KeyY,
    KeyCode::KeyZ,
];

const DIGITS: [KeyCode; 10] = [
    KeyCode::Digit0,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];

/// The presses still to come or held, and when the clock started.
#[derive(Resource, Default)]
pub struct TestKeys {
    pub presses: Vec<TimedPress>,
    started: Option<f32>,
    held: Vec<(Press, f32)>,
}

impl TestKeys {
    pub fn new(presses: Vec<TimedPress>) -> TestKeys {
        TestKeys {
            presses,
            ..Default::default()
        }
    }

    /// What to press and release `since` seconds after the start.
    fn step(&mut self, since: f32) -> (Vec<Press>, Vec<Press>) {
        let mut down = Vec::new();
        self.presses.retain(|p| {
            if p.at <= since {
                down.push((p.what, p.at + p.hold));
                false
            } else {
                true
            }
        });
        let mut up = Vec::new();
        self.held.retain(|(what, until)| {
            if *until <= since {
                up.push(*what);
                false
            } else {
                true
            }
        });
        self.held.extend(down.iter().copied());
        (down.into_iter().map(|(w, _)| w).collect(), up)
    }
}

/// Presses and releases what's due (after Bevy's own input update, so the
/// presses read as `just_pressed` this frame); the clock starts once the
/// player stands in the loaded place.
pub fn press_test_keys(
    time: Res<Time>,
    player: Option<Res<crate::walk::Player>>,
    mut tests: ResMut<TestKeys>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut motion: ResMut<bevy::input::mouse::AccumulatedMouseMotion>,
    mut scroll: ResMut<bevy::input::mouse::AccumulatedMouseScroll>,
) {
    if tests.presses.is_empty() && tests.held.is_empty() {
        return;
    }
    let now = time.elapsed_secs();
    if tests.started.is_none() && player.is_some_and(|p| p.ready) {
        tests.started = Some(now);
    }
    let Some(start) = tests.started else {
        return;
    };
    let (down, up) = tests.step(now - start);
    for p in down {
        match p {
            Press::Key(k) => keys.press(k),
            Press::Mouse(b) => mouse.press(b),
            Press::MouseX(_) | Press::MouseY(_) | Press::Wheel(_) => {}
        }
    }
    for (p, _) in &tests.held {
        match p {
            Press::MouseX(dx) => motion.delta.x += dx,
            Press::MouseY(dy) => motion.delta.y += dy,
            Press::Wheel(notches) => {
                scroll.unit = bevy::input::mouse::MouseScrollUnit::Line;
                scroll.delta.y += notches;
            }
            _ => {}
        }
    }
    for p in up {
        match p {
            Press::Key(k) => keys.release(k),
            Press::Mouse(b) => mouse.release(b),
            Press::MouseX(_) | Press::MouseY(_) | Press::Wheel(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presses_parse_and_come_due_then_release() {
        assert_eq!(
            parse(3.0, "f").unwrap(),
            TimedPress {
                at: 3.0,
                what: Press::Key(KeyCode::KeyF),
                hold: 0.1
            }
        );
        assert_eq!(parse(1.0, "R:1.5").unwrap().what, Press::Key(KeyCode::KeyR));
        assert_eq!(
            parse(1.0, "escape").unwrap().what,
            Press::Key(KeyCode::Escape)
        );
        assert_eq!(
            parse(1.0, "mouse-left").unwrap().what,
            Press::Mouse(MouseButton::Left)
        );
        assert_eq!(
            parse(1.0, "mouse-x=40:2").unwrap(),
            TimedPress {
                at: 1.0,
                what: Press::MouseX(40.0),
                hold: 2.0
            }
        );
        assert_eq!(parse(1.0, "wheel=-1").unwrap().what, Press::Wheel(-1.0));
        assert!(parse(1.0, "F1").is_err() && parse(1.0, "r:x").is_err());
        let mut t = TestKeys {
            presses: vec![parse(2.0, "r:1").unwrap()],
            ..Default::default()
        };
        assert_eq!(t.step(1.0), (vec![], vec![]));
        assert_eq!(t.step(2.0), (vec![Press::Key(KeyCode::KeyR)], vec![]));
        assert_eq!(t.step(2.5), (vec![], vec![]));
        assert_eq!(t.step(3.0), (vec![], vec![Press::Key(KeyCode::KeyR)]));
    }
}
