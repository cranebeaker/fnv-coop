//! The avatars' damage rules on the upstream engine: two overrides by address.
//!
//! The decision is the adapter's (`coop_engine::avatar::Guard`). This module puts it in front
//! of the game's hit funnel and its kill function. As the guest gate, the state is one
//! static behind a lock that is never held across a call into the program, and an address
//! with a wrap or with no translation is refused.

use std::sync::Mutex;

use coop_engine::addr;
use coop_engine::avatar::{Guard, Hit};
use engine::{AbiFn, Engine, Ret};

use crate::EngineProgram;

extern "Rust" {
    /// The engine's translations by address (the upstream `engine` crate defines it).
    fn nv_engine_assembled_funcs(out: &mut Vec<(u32, AbiFn)>);
}

/// The addresses that the guard overrides: the hit funnel, then the kill function.
pub const ADDRESSES: [u32; 2] = [addr::ACTOR_HIT, addr::ACTOR_KILL];
const OVERRIDES: [AbiFn; 2] = [hit, kill];

struct Installed {
    guard: Guard,
    originals: [AbiFn; 2],
}

static GUARD: Mutex<Option<Installed>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut Installed) -> R) -> Option<R> {
    GUARD
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_mut()
        .map(f)
}

/// Puts the avatars' damage rules on this game. `Err` names the address that cannot be
/// taken, and nothing is overridden then. A second call changes nothing.
pub fn install(e: &mut Engine) -> Result<(), String> {
    if installed() {
        return Ok(());
    }
    let mut table = Vec::new();
    // SAFETY: a plain Rust function that only appends to `table`.
    unsafe { nv_engine_assembled_funcs(&mut table) };
    let mut originals: Vec<AbiFn> = Vec::new();
    for address in ADDRESSES {
        if !e.wraps_at(address).is_empty() {
            return Err(format!(
                "{address:08x} has a wrap: an override would remove it"
            ));
        }
        let original = table
            .iter()
            .find(|(a, _)| *a == address)
            .map(|(_, f)| *f)
            .ok_or(format!("{address:08x} has no translation"))?;
        originals.push(original);
    }
    let originals: [AbiFn; 2] = originals
        .try_into()
        .map_err(|_| "the guard's address list changed".to_string())?;
    *GUARD.lock().unwrap_or_else(|e| e.into_inner()) = Some(Installed {
        guard: Guard::default(),
        originals,
    });
    for (address, function) in ADDRESSES.iter().zip(OVERRIDES) {
        e.register(*address, function);
    }
    Ok(())
}

/// True when this game has the avatars' damage rules.
pub fn installed() -> bool {
    with(|_| ()).is_some()
}

/// The avatars that are in the game now: (address, slot). Call it after each make and each
/// removal.
pub fn set_bodies(bodies: impl IntoIterator<Item = (u32, u8)>) {
    with(|g| g.guard.set_bodies(bodies));
}

/// The hits on avatars that wait for the host. The list is empty after.
pub fn take_hits() -> Vec<Hit> {
    with(|g| g.guard.take_hits()).unwrap_or_default()
}

/// How many kills of an avatar the guard stopped.
pub fn kills_stopped() -> u64 {
    with(|g| g.guard.kills_stopped).unwrap_or(0)
}

/// The hit funnel: a hit on an avatar is recorded and not applied.
fn hit(e: &mut Engine, args: &[u32]) -> Ret {
    let decided = with(|g| {
        let blocked = match args {
            [victim, data, ..] => g.guard.hit(&EngineProgram(e), *victim, *data),
            _ => false,
        };
        (blocked, g.originals[0])
    });
    match decided {
        Some((false, original)) => original(e, args),
        // The funnel answers nothing.
        Some((true, _)) | None => Ret::default(),
    }
}

/// The kill function: local code does not kill an avatar.
fn kill(e: &mut Engine, args: &[u32]) -> Ret {
    let decided = with(|g| {
        let blocked = args.first().is_some_and(|actor| g.guard.kill(*actor));
        (blocked, g.originals[1])
    });
    match decided {
        Some((false, original)) => original(e, args),
        Some((true, _)) | None => Ret::default(),
    }
}
