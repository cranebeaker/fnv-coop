//! `SpeedTree/run_00b3d800` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The node allocation (`0044d670`, cdecl: 1, 0) and the container's
/// set-up (`00b2e080`, thiscall, one word).
const ALLOCATE_NODE_0044D670: u32 = 0x0044_d670;
const CONTAINER_INIT_00B2E080: u32 = 0x00b2_e080;

// Translated from 00b3d830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3d830 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// A container's constructor (unnamed in the engine map): one node
/// (`0044d670(1, 0)`) whose first word points back at the container (a
/// null node stays null), stored at +0, then `00b2e080(this, 0)`. Returns
/// `this`. The code also keeps the addresses of two stack bytes (the
/// allocator objects) in locals it never reads.
pub fn fn_00b3d830(e: &mut Engine, this: Ptr) -> Ptr {
    let node = e.call(ALLOCATE_NODE_0044D670, &args![1u32, 0u32]).u32();
    let head = if node == 0 {
        0
    } else {
        e.mem.set_u32(node, this.addr());
        node
    };
    e.mem.set_u32(this.addr(), head);
    e.call(CONTAINER_INIT_00B2E080, &args![this, 0u32]);
    this
}

/// `0044a730` (`thiscall`, two stack words).
const CALL_0044A730: u32 = 0x0044_a730;

// Translated from 00b3d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `0044a730(this + 0x14, 1, 0)`,
/// answered as it is.
pub fn fn_00b3d800(e: &mut Engine, this: Ptr) -> u32 {
    e.call(
        CALL_0044A730,
        &args![this.addr().wrapping_add(0x14), 1u32, 0u32],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3_d830, fn_00b3d830(Ptr) -> Ptr),
        entry!(0x00b3_d800, fn_00b3d800(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_the_head_node_point_back_then_initialises() {
        let mut e = Engine::new();
        e.register(ALLOCATE_NODE_0044D670, |e, a| {
            assert_eq!(a, &[1, 0]);
            e.mem.alloc(8).into_ret()
        });
        e.register(CONTAINER_INIT_00B2E080, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b3_d830, &args![this]).u32(), this);
        let node = e.mem.u32(this);
        assert_ne!(node, 0);
        assert_eq!(e.mem.u32(node), this);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == CONTAINER_INIT_00B2E080)
            .collect();
        assert_eq!(calls[0].1, vec![this, 0]);
        // A failed allocation leaves a null head.
        e.register(ALLOCATE_NODE_0044D670, |_, _| Ret::default());
        e.call(0x00b3_d830, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    #[test]
    fn fn_00b3d800_passes_the_member_and_the_answer() {
        let mut e = Engine::new();
        e.register(CALL_0044A730, |_, a| (a[0] + a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b3_d800, &args![0x1000u32]).u32(), 0x1015);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == CALL_0044A730).collect();
        assert_eq!(calls, vec![&(CALL_0044A730, vec![0x1014, 1, 0])]);
    }
}
