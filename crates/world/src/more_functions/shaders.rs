//! Effect shaders scripts put on references (`PlayMagicShaderVisuals`,
//! `StopMagicShaderVisuals`): each a `MagicShaderHitEffect` the game keeps
//! in its list of running effects. Notes: `docs/DEAD_MONEY.md` "Effect
//! shaders".
//!
//! What's kept here is which shader runs on which reference and until
//! when. Drawing them (the `EFSH` record's fill and edge, membrane and
//! particles) isn't carried out yet: the viewer gets [`Shown::ShaderVisual`].
//! Whether the game saves them isn't traced; they aren't saved here.

use esm::FormId;

use super::{st, Shown};
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, GameState, Runner, Value};

/// One shader running on a reference.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderVisual {
    pub reference: FormId,
    /// The effect shader (`EFSH`).
    pub shader: FormId,
    /// When it ends on the state's clock (`GameState::seconds`); `None`
    /// until a script stops it.
    pub until: Option<f64>,
}

/// The functions here, by the game's own names.
pub const CHANGES: &[&str] = &["PlayMagicShaderVisuals", "StopMagicShaderVisuals"];

/// The shaders running now (ended ones left out).
pub fn active(state: &GameState) -> impl Iterator<Item = &ShaderVisual> {
    let now = state.seconds;
    state
        .more
        .shader_visuals
        .iter()
        .filter(move |v| v.until.map_or(true, |t| t > now))
}

/// Carries out one of [`CHANGES`]; `None` when it isn't one.
pub(super) fn carry_out(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    // No reference: the player.
    let who = target.unwrap_or(PLAYER_REF);
    let shader = args.first().map(Value::form).unwrap_or(FormId(0));
    let now = runner.state.seconds;
    // Ended ones go.
    st(runner)
        .shader_visuals
        .retain(|v| v.until.map_or(true, |t| t > now));
    match name {
        // `005d1b80`: a shader and seconds (default −1). Nothing happens
        // (the function still succeeds) unless the reference's cell is
        // attached (`004511e0`) and it has 3D (vtable +0x1d0): what the
        // viewer reports loaded. A new `MagicShaderHitEffect` each time
        // (`0081f580`): one already running isn't replaced. Seconds below
        // 0 run until stopped (lifetime `FLT_MAX`). One whose
        // initialisation fails (vtable +0xc4: taken to be one without a
        // shader) isn't added.
        "PlayMagicShaderVisuals" => {
            let seconds = args.get(1).map_or(-1.0, Value::number);
            if shader.0 == 0 || !runner.state.more.loaded.contains(&who) {
                return Some(1.0);
            }
            let until = (seconds >= 0.0).then_some(now + seconds);
            st(runner).shader_visuals.push(ShaderVisual {
                reference: who,
                shader,
                until,
            });
            runner.state.events.push(Event::More(Shown::ShaderVisual {
                reference: who,
                shader,
                seconds: (seconds >= 0.0).then_some(seconds as f32),
            }));
        }
        // `005d2130` → `00974a50`: every running shader effect
        // (`MagicShaderHitEffect`) on the reference with that shader
        // ends.
        "StopMagicShaderVisuals" => {
            let before = runner.state.more.shader_visuals.len();
            st(runner)
                .shader_visuals
                .retain(|v| !(v.reference == who && v.shader == shader));
            if runner.state.more.shader_visuals.len() != before {
                runner
                    .state
                    .events
                    .push(Event::More(Shown::ShaderVisualStopped {
                        reference: who,
                        shader,
                    }));
            }
        }
        _ => return None,
    }
    Some(1.0)
}
