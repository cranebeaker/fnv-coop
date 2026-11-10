//! A guest's clock on the upstream engine: the program's clock advance does nothing.
//!
//! The host's game runs its own clock. A guest's game takes the host's clock as plain state
//! (`coop_engine::clock`). This module makes the program's advance a no-op by one override
//! by address, with the gate's rules: an address with a wrap is refused, and a start check
//! proves that the override is in the path.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use coop_engine::{addr, clock, Program};
use engine::{Engine, Ret};

use crate::EngineProgram;

/// The calls of the advance that the override stopped.
static STOPPED: AtomicU64 = AtomicU64::new(0);
/// True when this game has the override.
static INSTALLED: AtomicBool = AtomicBool::new(false);

/// The seconds of the start check's call: one hour of the clock at the game's own time scale
/// of 30. A call that is not stopped moves the hour by a whole number.
const CANARY_SECONDS: f32 = 120.0;

/// The override: the clock does not move.
fn advance(_: &mut Engine, _: &[u32]) -> Ret {
    STOPPED.fetch_add(1, Ordering::Relaxed);
    Ret::default()
}

/// Makes the clock advance of this game do nothing: a guest's game from now on. `Err` gives
/// the reason, and the join must not go on then.
/// - The address has no wrap (an override would remove it).
/// - After the override, the program's own call of the advance reaches it, and no word of
///   the clock changes.
pub fn install(e: &mut Engine) -> Result<(), String> {
    if INSTALLED.load(Ordering::Relaxed) {
        return Ok(());
    }
    let address = addr::CLOCK_ADVANCE;
    if !e.wraps_at(address).is_empty() {
        return Err(format!(
            "the clock advance {address:08x} has a wrap: an override would remove it"
        ));
    }
    let globals = clock::globals(&mut EngineProgram(e))
        .map_err(|why| format!("the clock: {why}"))?
        .ok_or("the clock: the program has no clock object yet")?;
    e.register(address, advance);
    let before = clock::read(&EngineProgram(e), &globals);
    let stopped = STOPPED.load(Ordering::Relaxed);
    EngineProgram(e).call(address, &[addr::CLOCK, CANARY_SECONDS.to_bits()]);
    if STOPPED.load(Ordering::Relaxed) != stopped + 1 {
        return Err("the clock: a call of the advance did not reach the override".into());
    }
    if clock::read(&EngineProgram(e), &globals) != before {
        return Err("the clock: the advance still moves the clock under the override".into());
    }
    INSTALLED.store(true, Ordering::Relaxed);
    Ok(())
}

/// How many calls of the advance the override stopped (the start check's call is one).
pub fn stopped() -> u64 {
    STOPPED.load(Ordering::Relaxed)
}
