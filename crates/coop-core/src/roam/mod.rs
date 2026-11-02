//! Multi-cell scenes: every peer simulates the scene around its own player.
//! See `docs/COOP-SCENES.md` for the decision and the rules.

pub mod deferred;
pub mod glue;
pub mod graph;
pub mod model;
pub mod wire;

pub use deferred::{DArg, DeferredOp, DeferredQueue};
pub use glue::{GuestRoam, HostRoam};
pub use graph::{CellGraph, GRID_RADIUS};
pub use model::{CellDelta, Placement, SceneModel, Transit, ZoneAnchor};
pub use wire::RoamMsg;
