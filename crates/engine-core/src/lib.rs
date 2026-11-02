//! FalloutNV.exe 1.4.0.525 translated into Rust function by function, on a
//! model of the game's own memory (ADR-0006, docs/ENGINE_CRATE.md): the core
//! of the engine crates.
//!
//! Game objects live in [`Mem`] at their real field offsets and pointers are
//! the game's 32-bit addresses ([`Ptr`]). Every translated function is
//! registered under its exe address in a uniform calling form
//! ([`abi`]), so a translation calls any other function by address
//! ([`Engine::call`], [`Engine::vcall`]) whether or not it has been
//! translated yet; a call to an untranslated one stops with its address.
//! The data sections of the player's own executable are mapped at run time
//! ([`exe`]), so constants, strings, vtables and initial globals are the
//! game's.
//!
//! This crate holds the model ([`Engine`], [`Mem`], [`abi`], [`ptr`],
//! [`x87`], [`sse`], [`types`], [`exe`], the static initializer and `atexit` tables)
//! and the units every other part calls (`units/crt.rs`, `units/platform.rs`).
//! The other units are in the part crates (`crates/engine-<part>`), and the
//! `engine` crate assembles them: [`Engine::new`] registers this crate's
//! functions and those of the parts the final program links ([`assemble!`]).
//!
//! Translations live in `units/<subsystem>/<unit>.rs` of their crate, one
//! file per source unit of the Xbox 360 prototype (Xbox PDB, ADR-0002), as
//! listed in `research/engine-map/engine_map.tsv`. Each is marked
//! `// Translated from <addr> (decompiled, FalloutNV.exe 1.4.0.525)`
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

pub mod abi;
#[cfg(test)]
pub(crate) mod alloc_count;
pub mod call_trace;
mod engine;
pub mod exe;
pub mod mem;
pub mod profile;
pub mod ptr;
pub mod queue_check;
pub mod sse;
pub mod static_exit;
pub mod static_init;
pub mod thread_host;
pub mod types;
pub mod units;
pub mod watch_check;
pub mod x87;

pub use abi::{AbiFn, Ret};
pub use engine::{Engine, OpenHandler};
pub use mem::Mem;
pub use ptr::{Field, Inline, Layout, Ptr, Scalar};

/// What every unit file imports.
pub mod prelude {
    pub use crate::abi::{AbiFn, Arg, Ret, RetVal};
    pub use crate::ptr::{Field, Inline, Layout, Ptr, Scalar};
    pub use crate::x87::{
        x87_add, x87_div, x87_load32, x87_load64, x87_m32, x87_mul, x87_quiet, x87_quiet_bits,
        x87_round, x87_sqrt, x87_store32, x87_sub,
    };
    pub use crate::x87::{
        x87_f2xm1, x87_fcos, x87_fistp64, x87_fistp64_chop, x87_fpatan, x87_fprem, x87_fprem1,
        x87_fptan, x87_frndint, x87_fscale, x87_fsin, x87_fsincos, x87_fyl2x, x87_fyl2xp1, X87Ext,
        X87_STATUS_C2,
    };
    pub use crate::Engine;
    pub use crate::{args, entry, layout};
}
