//! `GetLineOfSight` (`005c1ce0` → `0059c990`): whether one actor sees a
//! reference. Notes: `docs/DEAD_MONEY.md` "Line of sight".
//!
//! The game's rules are here; the viewer, which has the camera and the
//! collision, answers the questions they ask through a [`Sight`] given to
//! the [`Runner`](crate::scripting::Runner). Without one (headless), the
//! player's test isn't carried out.

use esm::FormId;

use crate::dialogue::PLAYER_REF;
use crate::scripting::Runner;

/// What the viewer tells the world for a line-of-sight test.
pub trait Sight {
    /// A reference's 3D bound in game space (lowest and highest corner);
    /// `None` when it has no 3D loaded.
    fn bound(&self, reference: FormId) -> Option<([f32; 3], [f32; 3])>;
    /// The camera's position (game units).
    fn camera(&self) -> Option<[f32; 3]>;
    /// Whether a box (game space) is inside the camera's view.
    fn in_view(&self, lo: [f32; 3], hi: [f32; 3]) -> bool;
    /// How far from `from` toward `to` the first solid surface is, within
    /// the segment; `None` when nothing is in the way.
    fn ray(&self, from: [f32; 3], to: [f32; 3]) -> Option<f32>;
}

/// The functions here, by the game's own names.
pub const FUNCTIONS: &[&str] = &["GetLineOfSight"];

/// Heights on the target the rays go to, as fractions of its bound's
/// height above its position, in the order tried (`0059c990`,
/// `0088b880`).
pub const RAY_HEIGHTS: [f32; 3] = [0.75, 0.5, 0.25];

/// How far short of the target's box a ray may stop and still count as
/// having hit the target. The game asks which object the ray hit
/// (`00458420` → `0056f930`); nv-rs's collision doesn't know objects, so
/// a hit at the target's box counts as the target (a stand-in).
const AT_TARGET: f32 = 1.0;

/// `GetLineOfSight` on `caller` with `target`: 1 or 0; `None` when it
/// can't be worked out here.
pub fn line_of_sight(runner: &Runner, caller: Option<FormId>, target: FormId) -> Option<f64> {
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    let order = runner.order;
    let state = &*runner.state;
    let actor = |r: FormId| crate::more_functions::is_actor(order, state, r);
    // The caller must be an actor (vtable +0x100), the target given.
    let Some(caller) = caller.filter(|&c| actor(c)) else {
        return Some(0.0);
    };
    if target.0 == 0 {
        return Some(0.0);
    }
    if caller != PLAYER_REF {
        return actor_test(runner, caller, target).map(flag);
    }
    // The player: is the target in view (its 3D bound against the camera,
    // `004b5fc0` / `00444ed0`)? No 3D: not in view.
    let sight = runner.sight?;
    let Some((lo, hi)) = sight.bound(target) else {
        return Some(0.0);
    };
    if !sight.in_view(lo, hi) {
        return Some(0.0);
    }
    // Rays from the camera (`0043c490`) to the target at three heights.
    let eye = sight.camera()?;
    let (_, _, at, _) = state.place(order, target)?;
    if clear_to(sight, eye, at, (lo, hi)) {
        return Some(1.0);
    }
    // All blocked: the actor test decides.
    actor_test(runner, PLAYER_REF, target).map(flag)
}

/// Whether a ray from `eye` reaches the target (at `at`, with its bound)
/// at any of [`RAY_HEIGHTS`]: nothing in the way, or the first thing in
/// the way is the target.
pub fn clear_to(
    sight: &dyn Sight,
    eye: [f32; 3],
    at: [f32; 3],
    bound: ([f32; 3], [f32; 3]),
) -> bool {
    let (lo, hi) = bound;
    let height = hi[2] - lo[2];
    RAY_HEIGHTS.iter().any(|f| {
        let to = [at[0], at[1], at[2] + height * f];
        match sight.ray(eye, to) {
            None => true,
            Some(d) => enters(eye, to, lo, hi).is_some_and(|e| d >= e - AT_TARGET),
        }
    })
}

/// How far along the segment `from` → `to` it enters a box.
fn enters(from: [f32; 3], to: [f32; 3], lo: [f32; 3], hi: [f32; 3]) -> Option<f32> {
    let d: [f32; 3] = std::array::from_fn(|i| to[i] - from[i]);
    let length = d.iter().map(|v| v * v).sum::<f32>().sqrt();
    if length <= 0.0 {
        return Some(0.0);
    }
    let (mut near, mut far) = (0.0f32, 1.0f32);
    for axis in 0..3 {
        if d[axis].abs() < 1e-9 {
            if from[axis] < lo[axis] || from[axis] > hi[axis] {
                return None;
            }
            continue;
        }
        let a = (lo[axis] - from[axis]) / d[axis];
        let b = (hi[axis] - from[axis]) / d[axis];
        near = near.max(a.min(b));
        far = far.min(a.max(b));
    }
    (near <= far).then_some(near * length)
}

/// The actor test, a script's `GetLineOfSight` on a caller that isn't the
/// player (`0059c990` calls `0088b880` with its first argument 0 and the
/// target): the target needs 3D (else 0); within 2 units of the caller
/// (`005723b0` against 2.0) the answer is 1; otherwise the caller's AI
/// process answers for any target (`HighProcess` `008f6930`: the entry's +0x1e
/// in its detection list, the line of sight its last detection run found;
/// other processes say 0). For a target that isn't an actor no detection
/// entry exists: 0. The view cone and rays (`0088c570`) belong to a nonzero
/// first argument, which no script command passes.
fn actor_test(runner: &Runner, caller: FormId, target: FormId) -> Option<bool> {
    let order = runner.order;
    let state = &*runner.state;
    if let Some(sight) = runner.sight {
        if sight.bound(target).is_none() {
            return Some(false);
        }
    }
    if let (Some((sa, _, a, _)), Some((sb, _, b, _))) =
        (state.place(order, caller), state.place(order, target))
    {
        let d: f32 = (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt();
        if sa == sb && d <= 2.0 {
            return Some(true);
        }
    }
    if !crate::more_functions::is_actor(order, state, target) {
        return Some(false);
    }
    Some(
        state
            .more
            .detection_sight
            .get(&(caller, target))
            .copied()
            .unwrap_or(false),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_enter_boxes() {
        let lo = [10.0, -1.0, -1.0];
        let hi = [12.0, 1.0, 1.0];
        assert_eq!(enters([0.0; 3], [20.0, 0.0, 0.0], lo, hi), Some(10.0));
        assert_eq!(enters([0.0, 5.0, 0.0], [20.0, 5.0, 0.0], lo, hi), None);
        // Ends short of the box.
        assert_eq!(enters([0.0; 3], [5.0, 0.0, 0.0], lo, hi), None);
    }
}
