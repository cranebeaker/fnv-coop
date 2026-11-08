//! FalloutNV.exe 1.4.0.525 translated into Rust function by function, on a
//! model of the game's own memory (ADR-0006, docs/ENGINE_CRATE.md).
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
//! The engine is split into crates so a compile fits the machine
//! (docs/ENGINE_CRATE.md, "The engine crates"): `engine-core` (the model,
//! the CRT and platform units) and the part crates `engine-gamebryo`,
//! `engine-bs`, `engine-havok`, `engine-speedtree` and the twelve `engine-fallout-*`
//! crates of the game's own units, by area ("The fallout crates"). This
//! crate assembles them: everything is reachable as `engine::...` as before
//! the split, and [`Engine::new`] registers every translated function.
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

pub mod units;

pub use engine_core::{
    abi, census, cxx_exception, exe, mem, prelude, ptr, real_threads, sse, static_exit,
    static_init, thread_host, types, x87,
};
pub use engine_core::{args, entry, layout};
pub use engine_core::{AbiFn, Engine, Field, Inline, Layout, Mem, OpenHandler, Ptr, Ret, Scalar};
pub use engine_fallout_cell::{extra_data, reference};

#[cfg(test)]
mod tests {
    /// Every exe address is registered by one engine crate only (`Engine::new()` stops on an
    /// address registered twice); this names the two crates when it happens, so the order in
    /// which the facade takes the crates never decides which translation runs.
    #[test]
    fn no_address_is_registered_by_two_crates() {
        let mut by: std::collections::HashMap<u32, &str> = std::collections::HashMap::new();
        let mut twice = Vec::new();
        for &(name, funcs) in crate::units::PARTS {
            let mut list = Vec::new();
            funcs(&mut list);
            for (addr, _) in list {
                if let Some(first) = by.insert(addr, name) {
                    twice.push(format!("{addr:08x}: {first} and {name}"));
                }
            }
        }
        assert!(twice.is_empty(), "registered twice: {twice:?}");
        let mut all = Vec::new();
        crate::units::funcs(&mut all);
        assert_eq!(all.len(), by.len());
    }
}
