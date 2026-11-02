//! FalloutNV.exe 1.4.0.525 translated into Rust function by function, on a
//! model of the game's own memory (ADR-0006, docs/ENGINE_CRATE.md):
//! the game's own units (`fallout/*`, `fallout shared/*`, FaceGen, `d3dx9`,
//! `RemoteLog` and the unplaced runs), and the adapters over its classes
//! ([`extra_data`], [`reference`]; `engine::extra_data` and
//! `engine::reference`).
//!
//! One of the engine crates: the model and the CRT and platform units are in
//! `engine-core`, and the `engine` crate assembles every part and registers
//! every function. Translations live in `units/<subsystem>/<unit>.rs`, one
//! file per source unit of the Xbox 360 prototype (Xbox PDB, ADR-0002); each
//! is marked `// Translated from <addr> (decompiled, FalloutNV.exe 1.4.0.525)`
//! (ADR-0003); `docs/LEDGER.md` counts them.

// Translation doc comments quote offsets and arithmetic ("+ 0x3c", "+ 1")
// that markdown reads as list items; that only affects rendering, so the
// lint is allowed for the crate rather than reflowing thousands of comments.
#![allow(clippy::doc_lazy_continuation)]
// Translations follow the exe, not Rust style, in three ways these lints flag:
// float compares must match the exe's flag tests with NaN included, which
// negated compares (`!(a < b)`) express directly; signatures take exactly the
// words the exe's functions take; and recursive functions pass on arguments
// the exe passes on even when nothing else reads them.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::only_used_in_recursion)]
// Lane loops index several parallel arrays the way the exe walks them.
#![allow(clippy::needless_range_loop)]
// x87 code multiplies by a -1.0 constant, which keeps a NaN's sign; negation flips it.
#![allow(clippy::neg_multiply)]
// Branch structure follows the exe's compare-and-jump order.
#![allow(clippy::collapsible_match)]
// Allocation sizes spell out the exe's "all ones on carry" (`SETO`/`NEG`/`OR`)
// as `checked_mul(..).unwrap_or(u32::MAX)`, next to the instructions it mirrors.
#![allow(clippy::manual_saturating_arithmetic)]
// SSE lanes the exe computes as `a*b - a*b` stay: they give NaN, not 0, for
// infinite or NaN inputs.
#![allow(clippy::eq_op)]
// Results assigned in each branch keep the exe's branch layout.
#![allow(clippy::needless_late_init)]
// `a = b op a` keeps the exe's operand order (it decides which NaN comes out).
#![allow(clippy::assign_op_pattern)]
// Test doubles record calls in shared logs.
#![cfg_attr(test, allow(clippy::type_complexity))]
// Tests build their expected call lists and buffers with `vec!`.
#![cfg_attr(test, allow(clippy::useless_vec))]

pub mod extra_data;
pub mod reference;
pub mod units;

// The core, re-exported so the unit files reach it as `crate::...`, as in the
// single crate the engine was before the split.
pub use engine_core::{abi, exe, mem, prelude, ptr, sse, static_exit, static_init, types, x87};
pub use engine_core::{args, entry, layout};
pub use engine_core::{AbiFn, Engine, Field, Inline, Layout, Mem, Ptr, Ret, Scalar};
