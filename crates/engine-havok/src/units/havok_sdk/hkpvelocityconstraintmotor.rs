//! `Havok SDK/hkpvelocityconstraintmotor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no layout for this class: the fields and their names are taken from what the code
//! copies (the names follow the Havok SDK's `hkpVelocityConstraintMotor`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVelocityConstraintMotor` (names unconfirmed).
    pub struct HkpVelocityConstraintMotor: 0x20 {
        /// The 16-bit memory size of the block.
        0x04 m_memory_size: u16,
        /// The 16-bit reference count.
        0x06 m_reference_count: u16,
        /// `m_type`.
        0x08 m_type: u8,
        /// `m_minForce`.
        0x0c m_min_force: f32,
        /// `m_maxForce`.
        0x10 m_max_force: f32,
        /// `m_tau`.
        0x14 m_tau: f32,
        /// `m_velocityTarget`.
        0x18 m_velocity_target: f32,
        /// `m_useVelocityTargetFromConstraintTargets`.
        0x1c m_use_velocity_target_from_constraint_targets: u8,
    }
}

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtables the construction chain stores in turn: `010cb76c`, `010cb780` and the motor's own
/// `010cc0d8`.
const FIRST_VTABLE: u32 = 0x010c_b76c;
const SECOND_VTABLE: u32 = 0x010c_b780;
const VTABLE: u32 = 0x010c_c0d8;

// Translated from 00d861c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVelocityConstraintMotor::clone` (Xbox PDB), `(this)`: a 0x20-byte block of the router's
/// allocator (slot `4`) gets the vtables of the construction chain (the last, `010cc0d8`,
/// stays), the 16-bit size `0x20` at `+4`, the reference count `1` at `+6`, and copies of `this`:
/// the byte at `+8`, the floats at `+0xC`, `+0x10`, `+0x14` and `+0x18` (each copy goes through
/// the x87 stack, which quiets a signalling NaN) and the byte at `+0x1C`. Returns the clone.
pub fn clone(
    e: &mut Engine,
    this: Ptr<HkpVelocityConstraintMotor>,
) -> Ptr<HkpVelocityConstraintMotor> {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e
        .vcall(allocator, 4, &args![0x20u32])
        .ptr::<HkpVelocityConstraintMotor>();
    e.mem.set_u32(block.addr(), FIRST_VTABLE);
    e.set(block, HkpVelocityConstraintMotor::m_memory_size, 0x20);
    e.set(block, HkpVelocityConstraintMotor::m_reference_count, 1);
    let kind = e.get(this, HkpVelocityConstraintMotor::m_type);
    e.mem.set_u32(block.addr(), SECOND_VTABLE);
    e.set(block, HkpVelocityConstraintMotor::m_type, kind);
    let min_force = x87_quiet(e.get(this, HkpVelocityConstraintMotor::m_min_force));
    e.set(block, HkpVelocityConstraintMotor::m_min_force, min_force);
    let max_force = x87_quiet(e.get(this, HkpVelocityConstraintMotor::m_max_force));
    e.mem.set_u32(block.addr(), VTABLE);
    e.set(block, HkpVelocityConstraintMotor::m_max_force, max_force);
    let tau = x87_quiet(e.get(this, HkpVelocityConstraintMotor::m_tau));
    e.set(block, HkpVelocityConstraintMotor::m_tau, tau);
    let velocity_target = x87_quiet(e.get(this, HkpVelocityConstraintMotor::m_velocity_target));
    e.set(
        block,
        HkpVelocityConstraintMotor::m_velocity_target,
        velocity_target,
    );
    let flag = e.get(
        this,
        HkpVelocityConstraintMotor::m_use_velocity_target_from_constraint_targets,
    );
    e.set(
        block,
        HkpVelocityConstraintMotor::m_use_velocity_target_from_constraint_targets,
        flag,
    );
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d8_61c0,
        clone(Ptr<HkpVelocityConstraintMotor>) -> Ptr<HkpVelocityConstraintMotor>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::rc::Rc;

    const ALLOCATOR_BASE: u32 = 0x0900_8000;

    /// The memory router with an allocator whose slot 4 hands out `block`; returns the allocator.
    fn install_allocator(e: &mut Engine, log: &Log, block: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        for slot in (0..0x20).step_by(4) {
            e.mem.set_u32(vtable + slot, ALLOCATOR_BASE + slot);
        }
        log_double(e, log, ALLOCATOR_BASE + 4, block);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        let sink = Rc::clone(log);
        e.register_double(MEMORY_ROUTER, move |_, _| {
            sink.borrow_mut().push((MEMORY_ROUTER, Vec::new()));
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        allocator
    }

    #[test]
    fn the_clone_copies_the_motor_settings() {
        let mut e = engine();
        let log = new_log();
        let block = e.mem.alloc(0x20);
        let allocator = install_allocator(&mut e, &log, block);
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 8, 3);
        e.mem.set_f32(this + 0xc, -10.0);
        // A signalling NaN is quieted by the copy through the x87 stack.
        e.mem.set_u32(this + 0x10, 0x7f80_0001);
        e.mem.set_f32(this + 0x14, 0.25);
        e.mem.set_f32(this + 0x18, 2.5);
        e.mem.set_u8(this + 0x1c, 1);
        let clone = e.call(0x00d8_61c0, &args![this]).u32();
        assert_eq!(clone, block);
        assert_eq!(calls(&log, ALLOCATOR_BASE + 4), vec![vec![allocator, 0x20]]);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u8(block + 8), 3);
        assert_eq!(e.mem.f32(block + 0xc), -10.0);
        assert_eq!(e.mem.u32(block + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.f32(block + 0x14), 0.25);
        assert_eq!(e.mem.f32(block + 0x18), 2.5);
        assert_eq!(e.mem.u8(block + 0x1c), 1);
    }
}
