//! `BSHavok/hkliquidaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkLiquidAction` (Xbox PDB) is a Havok action that keeps a 16-byte vector at `+0x20` (all
//! four lanes start as the float at 0x01011d78) and, every step, hands it to the body it acts
//! on. `applyAction` carries a `/GS` cookie check (`00ec408c`) and `clone` an `FS:[0]`
//! exception frame; neither is modeled, so both are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, havok_allocate};

/// The base action constructor (`00d2b260`, `this` plus two words).
const ACTION_CONSTRUCT: u32 = 0x00d2_b260;
/// The float every lane of the vector starts as.
const VECTOR_START: u32 = 0x0101_1d78;
/// The class vtable the constructor stores.
const CLASS_VTABLE: u32 = 0x010c_e740;
/// Run on the body before the step is applied (`00c9c1d0`, `this` only).
const PREPARE_BODY: u32 = 0x00c9_c1d0;
/// Size of an `hkLiquidAction`.
const ACTION_SIZE: u32 = 0x30;
/// Virtual slot (of the object embedded at body `+0xE0`) that receives the step, the vector
/// pointer and the body's own vector.
const APPLY_SLOT: u32 = 0x58;

// Translated from 00d2c910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLiquidAction::hkLiquidAction` (Xbox PDB): takes two words. Runs the base constructor
/// (`00d2b260`) on `(this, first, second)`, stores the class vtable (0x010ce740) and the float
/// at 0x01011d78 in all four lanes of the 16-byte vector at `+0x20`. Answers `this`.
pub fn hk_liquid_action_hk_liquid_action(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
) -> u32 {
    e.call(ACTION_CONSTRUCT, &args![this, first, second]);
    let start = e.mem.u32(VECTOR_START);
    e.mem.set_u32(this, CLASS_VTABLE);
    for lane in 0..4 {
        e.mem.set_u32(this + 0x20 + 4 * lane, start);
    }
    this
}

// Translated from 00d2c960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c960 - the /GS cookie check (00ec408c) is not modeled.
/// `hkLiquidAction::applyAction` (Xbox PDB): takes the step information, whose float at `+8`
/// (through the x87 stack) is the step. The body is the word at `+0x18`; its 16-byte vector at
/// `+0x120` is copied to a local, `00c9c1d0` runs on the body, and then virtual slot 0x58 of
/// the object at body `+0xE0` runs with `(step, this + 0x20, the copied vector)`.
pub fn hk_liquid_action_apply_action(e: &mut Engine, this: u32, info: u32) {
    let step = x87_quiet_bits(e.mem.u32(info + 8));
    let body = e.mem.u32(this + 0x18);
    e.with_stack(0x10, |e, copy| {
        for lane in 0..4 {
            let value = e.mem.u32(body + 0x120 + 4 * lane);
            e.mem.set_u32(copy.addr() + 4 * lane, value);
        }
        e.call(PREPARE_BODY, &args![body]);
        e.vcall(
            body + 0xe0,
            APPLY_SLOT,
            &args![step, this + 0x20, copy.addr()],
        );
    });
}

// Translated from 00d2c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d2c9d0 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `hkLiquidAction::clone` (Xbox PDB): takes two words. With the allocation-scope word set to
/// 0x10, answers 0 unless the word at `+4` of the first is 2 and the word at `+4` of the
/// second is 0. Otherwise allocates 0x30 bytes from the Havok allocator, builds a new action
/// on it with the constructor on `(word at the first's first word, this + 0x10)` and copies
/// the vector at `+0x20` to the new one. Answers the new action. The scope word is put back.
pub fn hk_liquid_action_clone(e: &mut Engine, this: u32, first: u32, second: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let mut result = 0;
    if e.mem.u32(first + 4) == 2 && e.mem.u32(second + 4) == 0 {
        let block = havok_allocate(e, ACTION_SIZE);
        let owner = e.mem.u32(e.mem.u32(first));
        let world = e.mem.u32(this + 0x10);
        result = hk_liquid_action_hk_liquid_action(e, block, owner, world);
        for lane in 0..4 {
            let value = e.mem.u32(this + 0x20 + 4 * lane);
            e.mem.set_u32(result + 0x20 + 4 * lane, value);
        }
    }
    e.mem.set_u32(word, saved);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2c910,
            hk_liquid_action_hk_liquid_action(u32, u32, u32) -> u32
        ),
        entry!(0x00d2c960, hk_liquid_action_apply_action(u32, u32)),
        entry!(
            0x00d2c9d0,
            hk_liquid_action_clone(u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{allocator_doubles, calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn construction_fills_the_vector_and_the_vtable() {
        let mut e = logging_engine();
        e.map(VECTOR_START, 4);
        e.mem.set_f32(VECTOR_START, 1.0);
        e.register(ACTION_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x30);
        assert_eq!(
            e.call(0x00d2_c910, &args![this, 0x11u32, 0x22u32]).u32(),
            this
        );
        assert_eq!(calls_to(&e, ACTION_CONSTRUCT), [[this, 0x11, 0x22]]);
        assert_eq!(e.mem.u32(this), CLASS_VTABLE);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0x20 + 4 * lane), 1.0);
        }
    }

    #[test]
    fn apply_hands_the_step_and_the_copied_vector_to_the_embedded_object() {
        let mut e = logging_engine();
        e.register(PREPARE_BODY, nothing);
        e.register_double(0x0900_0058, |e, a| {
            // The copied vector is what the body held before the preparation ran.
            let copy = a[3];
            assert_eq!(e.mem.f32(copy), 1.5);
            assert_eq!(e.mem.f32(copy + 12), 4.5);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x18, body);
        e.mem.set_u32(body + 0xe0, table);
        e.mem.set_u32(table + APPLY_SLOT, 0x0900_0058);
        e.mem.set_f32(body + 0x120, 1.5);
        e.mem.set_f32(body + 0x12c, 4.5);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 8, 0x7f80_0001);
        e.call(0x00d2_c960, &args![this, info]);
        assert_eq!(calls_to(&e, PREPARE_BODY), [[body]]);
        let calls = calls_to(&e, 0x0900_0058);
        assert_eq!(calls.len(), 1);
        assert_eq!(&calls[0][..3], &[body + 0xe0, 0x7fc0_0001, this + 0x20]);
    }

    /// An action to clone with the given words at `+4` of the two arguments.
    fn clone_setup(first_kind: u32, second_kind: u32) -> (Engine, [u32; 3]) {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(ACTION_CONSTRUCT, nothing);
        e.map(VECTOR_START, 4);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x3333);
        e.mem.set_u32(this + 0x20, 0xaaaa);
        e.mem.set_u32(this + 0x2c, 0xbbbb);
        let first = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, 0x2222);
        e.mem.set_u32(first, inner);
        e.mem.set_u32(first + 4, first_kind);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 4, second_kind);
        (e, [this, first, second])
    }

    #[test]
    fn clone_builds_and_copies_the_vector() {
        let (mut e, [this, first, second]) = clone_setup(2, 0);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 4);
        let clone = e.call(0x00d2_c9d0, &args![this, first, second]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u16(clone + 4), 0x30);
        assert_eq!(calls_to(&e, ACTION_CONSTRUCT), [[clone, 0x2222, 0x3333]]);
        assert_eq!(e.mem.u32(clone + 0x20), 0xaaaa);
        assert_eq!(e.mem.u32(clone + 0x2c), 0xbbbb);
        assert_eq!(e.mem.u32(word), 4);
    }

    #[test]
    fn clone_answers_zero_for_other_kinds() {
        for kinds in [(1, 0), (2, 1)] {
            let (mut e, [this, first, second]) = clone_setup(kinds.0, kinds.1);
            assert_eq!(e.call(0x00d2_c9d0, &args![this, first, second]).u32(), 0);
            assert!(calls_to(&e, ACTION_CONSTRUCT).is_empty());
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0);
        }
    }
}
