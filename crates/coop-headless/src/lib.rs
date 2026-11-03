//! Headless co-op scenario simulator.
//!
//! Runs the scenarios of `tools/coop-run` in one process, with no window and
//! no renderer: a host and a guest `GameState` driven by the real
//! `coop_session::Session` and live layers over coop-core's seeded chaos
//! transport, on a virtual clock. The viewer's per-frame glue (scripts, the
//! bot, movement, the dialogue menu) is modelled in [`peer`] and [`bot`];
//! every stand-in is listed in the verdict's `modelled` field.

pub mod backend;
pub mod bot;
pub mod campaign_bot;
pub mod checklist;
pub mod cli;
pub mod data;
pub mod expect;
pub mod json;
pub mod multi;
pub mod net;
pub mod party;
pub mod peer;
pub mod questwalk;
pub mod recorder;
pub mod scenario;
pub mod sim;
pub mod social_bot;
pub mod suite;
pub mod verdict;
pub mod watch;
