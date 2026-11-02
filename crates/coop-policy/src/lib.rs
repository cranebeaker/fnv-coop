//! Campaign policy layer for co-op, as data plus pure logic.
//!
//! `data/policy.json` holds one row per quest (shared or per-player, who may advance a stage,
//! tethers, barriers, reward routing, companion and shadow-variable special cases).
//! [`PolicyTable`] loads and validates it; [`PolicyTable::decide`] turns an [`Event`] and the
//! current [`State`] into a [`Decision`]. Form ids are plain `u32`; the crate does not depend on
//! the rest of the co-op code.

pub mod engine;
pub mod load;
pub mod model;

pub use engine::*;
pub use load::{LoadError, PolicyTable};
pub use model::*;
