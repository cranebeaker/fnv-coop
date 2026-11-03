//! `coop-core`: the game-free core of nv-rs co-op.
//!
//! Host-authoritative replication keyed by FormID, never by memory address:
//!
//! * [`ids`] and [`schema`]: identifiers and the record vocabulary.
//! * [`delta`]: the revisioned [`delta::StateStore`] and state-based deltas
//!   (last writer wins in host order, idempotent, reorder tolerant).
//! * [`authority`]: who may write what (story = host, actors = owner with
//!   epoch, avatar = its player).
//! * [`snapshot`], [`chunk`], [`baseline`]: join snapshot (lz4), chunking, and
//!   the per-client acked-baseline ring with full-snapshot fallback.
//! * [`hash`]: 128-shard hierarchical state hash, desync report, repair.
//! * [`proto`], [`session`]: wire messages (postcard), channels, size limits,
//!   handshake and rejoin decisions.
//! * [`sync`]: the host and guest protocol state machines.
//! * [`transport`]: a `Transport` trait, loopback, and a seeded
//!   `ChaosTransport` on a virtual clock.
//! * Live layer (protocol 2): [`avatar`], [`interest`], [`actors`], [`dialogue`],
//!   [`loot`], [`rules`], [`social`], wired through `sync::HostLive` and
//!   `sync::GuestLive`.
//! * `sim_test` (tests or the `testkit` feature): deterministic multi-peer
//!   simulation harness.
//!
//! This crate depends on no other nv-rs crate and has no async runtime.

#![warn(missing_docs)]

pub mod actors;
pub mod authority;
pub mod avatar;
pub mod baseline;
pub mod chunk;
pub mod combat;
pub mod compat;
pub mod delta;
pub mod dialogue;
pub mod env;
pub mod error;
pub mod hash;
pub mod ids;
pub mod interact;
pub mod interest;
pub mod loot;
pub mod once;
pub mod proto;
pub mod roam;
pub mod rules;
pub mod scene;
pub mod schema;
pub mod session;
pub mod snapshot;
pub mod social;
pub mod sync;
pub mod transport;

#[cfg(any(test, feature = "testkit"))]
pub mod sim_test;

pub use error::{Error, Result};
