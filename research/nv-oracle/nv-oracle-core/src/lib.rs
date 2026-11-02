//! Platform-independent parts of the nv-oracle tools.
//!
//! Everything here is plain Rust with no operating system calls, so it is
//! unit-tested on any host. The Windows-only tools (`nv-call`, `nv-probe`,
//! `nv-inject`) build on it:
//!
//! - [`manifest`]: the line-based probe manifest (`nv-probe.txt`).
//! - [`vectors`]: the JSON-lines vector and result formats of `nv-call`.
//! - [`abi`]: calling conventions and where each argument lives.
//! - [`decode`]: a length decoder for the instructions found at the start
//!   of functions, used to validate and relocate stolen bytes.
//! - [`pe`]: a bounds-checked PE32 header, section and import reader.
//! - [`x87`]: 80-bit extended value conversion.
//! - [`sha256`], [`hex`], [`json`]: small helpers so no crates are needed.

pub mod abi;
pub mod decode;
pub mod hex;
pub mod json;
pub mod manifest;
pub mod pe;
pub mod sha256;
pub mod vectors;
pub mod x87;
