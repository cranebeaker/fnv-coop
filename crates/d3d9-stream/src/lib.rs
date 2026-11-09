//! The game's Direct3D 9 calls as a stream of commands: the seam between the platform's Direct3D 9
//! model and what draws.
//!
//! In engine-platform the game's calls reach a **model** of Direct3D 9 (ADR-0007), which answers
//! every value the game reads. The calls that change what is drawn are then repeated on a
//! **backend**: today the real `d3d9.dll` (ADR-0008, `NV_D3D9=real`), later a wgpu layer. This
//! crate is what passes between the two:
//!
//! - [`Cmd`]: one recorded call, its objects named by the modelled object's address ([`Handle`]),
//!   the bytes it points at carried along ([`Data`]): states, draws, creations and destructions,
//!   uploads at unlocks, and the few calls whose answer comes from the backend (sync: the device's
//!   creation, read-backs, query results), which answer a [`Reply`].
//! - [`Backend`]: what carries commands out, in order, never reading the game's memory.
//! - [`Stream`]: where it runs. [`Stream::direct`] carries each command out at once on the calling
//!   thread (the same calls in the same place as calling the backend directly: tests, scripted
//!   runs, `NV_RENDER_THREAD=0`). [`Stream::threaded`] (play's default, chosen by the host)
//!   records the commands into chunks and hands them
//!   to a render thread; a sync command waits for its reply, which comes after every earlier
//!   command; a `Present` returns once at most one earlier frame is left to carry out. The game
//!   thread keeps only the model's work and the recording.
//! - [`Tee`]: two backends fed the same commands, the primary answering.
//! - [`wire`]: commands as bytes: a file of them (recorded once, replayed into any backend), and
//!   the hashes that check the hand-over (`NV_D3D9_STREAM_CHECK=1`).
//!
//! The order of the commands is the order of the calls: the stream lives beside the model, under
//! the lock that guards it, so the game's threads record in the order they made their calls.

mod chunk;
mod cmd;
mod runner;
mod tee;
pub mod wire;

pub use cmd::{Backend, Cmd, Data, Handle, Native, Rect, Reply, Rows};
pub use runner::{Settings, Stream};
pub use tee::Tee;

#[cfg(test)]
mod tests;
