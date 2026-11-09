//! nv-launcher: the player-facing terminal launcher for the nv-rs co-op build.
//!
//! Every module except `app`, `ui` and `run` is pure (or takes its inputs as arguments) and has
//! unit tests. The launcher never modifies, copies or deletes game files; it only reads the
//! installation and writes under its own home folder (`%APPDATA%\nv-rs-coop`).

pub mod app;
pub mod cli;
pub mod command;
pub mod diag;
pub mod extras;
pub mod install;
pub mod joincode;
pub mod json;
pub mod party;
pub mod portcheck;
pub mod redact;
pub mod run;
pub mod settings;
pub mod steam;
pub mod ui;

/// The launcher's version, shown in the banner and in diagnostics.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
