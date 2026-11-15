//! coop-app: the co-op program on a game program by address.
//!
//! One player runs one program: the game with the co-op session in its frame loop. This
//! crate is that program without the engine: [`App`] runs on a [`Machine`], which is the
//! game's frames, its screen and the guest gate. The binary `nv-coop` gives it the upstream
//! engine. The tests give it a fake program.
//!
//! The command line, the control port and the state dump follow the contract of the
//! project's test harness (`tools/coop-run`), so that one scenario drives each program that
//! follows it.

pub mod app;
pub mod args;
pub mod dump;
pub mod fake;
pub mod intents;
pub mod machine;
pub mod names;

pub use app::{App, Flow};
pub use args::Args;
pub use machine::{Machine, Screen};
