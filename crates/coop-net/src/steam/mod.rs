//! Steam (Spacewar, AppID 480) backend. Off by default (`steam` feature).
//!
//! The feature `steam-mock` compiles [`api`], [`backend`] and [`mock`] only.
//! The feature `steam` adds `flat` and `real` (the DLL loader).
//!
//! * [`api`]: the safe [`SteamApi`] trait the rest of the code talks to.
//! * [`flat`]: the Steam flat C API loaded at runtime with `libloading`:
//!   export names, `repr(C)` structs with size asserts, callback ids and pure
//!   byte decoders. No Valve header or import library is involved.
//! * [`real`]: [`RealSteam`], `SteamApi` over the DLL with the manual-dispatch
//!   loop and sender-offset probing. Only a real two-account test can verify it.
//! * [`mock`]: [`MockSteam`], an in-process universe that follows the playbook's
//!   rules (first message held until accepted, LobbyCreated twice, friends-only
//!   lobbies) with fault injection.
//! * [`backend`]: [`SteamBackend`], lobby create/join/invite logic and the
//!   lane mapping onto message channels 0/1/2.
//!
//! # GPL section 7 note
//!
//! As an additional permission under GPLv3 section 7, the copyright holders of
//! this file permit linking or loading Valve's proprietary `steam_api64.dll`
//! at run time. The DLL is never committed, modified or rebuilt; it ships only
//! inside a release archive next to the executable with a third-party notice.
//!
//! # Fallback
//!
//! [`try_init`] returns a typed [`SteamError`] when the DLL is missing, Steam
//! is not running or an export is absent. The caller then uses a
//! [`crate::UdpBackend`].

pub mod api;
pub mod backend;
#[cfg(feature = "steam")]
pub mod flat;
pub mod mock;
#[cfg(feature = "steam")]
pub mod real;

pub use api::{Friend, RecvMsg, SendResult, SteamApi, SteamError, SteamEvent};
pub use backend::{Invite, SteamBackend, SteamOptions, SteamPhase, MARKER_KEY};
pub use mock::{Faults, MockSteam, MockUniverse};
#[cfg(feature = "steam")]
pub use real::{try_init, RealSteam};
