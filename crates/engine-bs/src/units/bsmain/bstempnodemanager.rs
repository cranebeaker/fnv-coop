//! `BSMain/bstempnodemanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNode::UpdateDownwardPass` (engine map, `00a5dd70`; `thiscall`, two words).
const NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
/// `BSTempNodeManager::fPrevTime` and `BSTempNode::fTime` (Xbox PDB +0xD0; `NiNode` is 0xAC
/// bytes on PC, so both sit at +0xAC).
const TIME: u32 = 0xac;
/// `NiNode`'s child array (`NiTArray` at +0x9C): `m_pBase` +0xA0, `m_usSize` +0xA6.
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;
/// The double compared with the times (`.rdata`, 0.0 in the exe).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The RTTI record a child must derive from to count down (`.data`; by this unit
/// `BSTempNode`'s, unconfirmed).
const TEMP_NODE_RTTI: u32 = 0x0120_2e00;
/// The imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c44e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::UpdateDownwardPass` (Xbox PDB; `thiscall`, two words, `RET 8`):
/// `RemoveExpired` with the float `update_data` points at (its first member, copied through
/// the x87 stack), then `NiNode::UpdateDownwardPass(update_data, flags)`.
pub fn bs_temp_node_manager_update_downward_pass(
    e: &mut Engine,
    this: u32,
    update_data: u32,
    flags: u32,
) {
    let time = x87_quiet_bits(e.mem.u32(update_data));
    bs_temp_node_manager_remove_expired(e, this, time);
    e.call(NODE_UPDATE_DOWNWARD_PASS, &args![this, update_data, flags]);
}

// Translated from 00c44ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::RemoveExpired` (Xbox PDB; `thiscall`, one float, `RET 4`): the time
/// passed is `time - fPrevTime`, or `fPrevTime` itself while that equals 0.0 (the double at
/// `01012060`); `fPrevTime` becomes `time`. Then over the children from the last to the
/// first: a child whose RTTI chain (vtable slot +0x8, then each record's base at +4)
/// reaches `01202e00` has the passed time taken off its `fTime`, clamped at 0.0 when below
/// it; a child whose time is then 0.0 is detached (vtable slot +0xEC of `this`, (index,
/// the address of a local `NiPointer`)). The local pointer, null at first (the compiler's
/// increment of it on construction is guarded by that null), is released at the end
/// (`InterlockedDecrement` of its count; `DeleteThis`, vtable slot +4, when it reaches 0).
// Translation incomplete: 00c44ed0 - the FS:[0] exception frame (releasing the local NiPointer on unwinding) is not translated
pub fn bs_temp_node_manager_remove_expired(e: &mut Engine, this: u32, time: u32) {
    e.with_stack(4, |e, held| {
        let held = held.addr();
        e.mem.set_u32(held, 0);
        let initial = e.mem.u32(held);
        if initial != 0 {
            // Never taken: the pointer was just set to null.
            e.call(INTERLOCKED_INCREMENT, &args![initial + 4]);
        }
        let zero = e.mem.f64(ZERO_DOUBLE);
        let previous_bits = e.mem.u32(this + TIME);
        let previous = x87_load32(previous_bits);
        // FCOMP; TEST AH,0x44; JNP: only an ordered equal keeps the stored time.
        let passed = if previous == zero {
            previous
        } else {
            x87_sub(x87_load32(time), x87_m32(previous_bits))
        };
        let passed = x87_store32(passed);
        e.mem.set_u32(this + TIME, x87_quiet_bits(time));
        let passed = x87_quiet_bits(passed);

        let mut index = u32::from(e.mem.u16(this + CHILDREN_SIZE));
        while index != 0 {
            index -= 1;
            let base = e.mem.u32(this + CHILDREN_BASE);
            let child = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            let node = if child == 0 {
                0
            } else {
                let mut rtti = e.vcall(child, 0x8, &[]).u32();
                let mut derives = false;
                while rtti != 0 {
                    if rtti == TEMP_NODE_RTTI {
                        derives = true;
                        break;
                    }
                    rtti = e.mem.u32(rtti + 4);
                }
                if derives {
                    child
                } else {
                    0
                }
            };
            if node == 0 {
                continue;
            }
            let left = x87_sub(x87_m32(e.mem.u32(node + TIME)), x87_load32(passed));
            let left = x87_store32(left);
            e.mem.set_u32(node + TIME, x87_quiet_bits(left));
            // FCOMP; TEST AH,0x5; JP: only an ordered below clamps.
            if x87_load32(e.mem.u32(node + TIME)) < zero {
                e.mem.set_f32(node + TIME, 0.0);
            }
            // FCOMP; TEST AH,0x44; JP: only an ordered equal detaches.
            if x87_load32(e.mem.u32(node + TIME)) == zero {
                e.vcall(this, 0xec, &args![index, held]);
            }
        }

        let pointer = e.mem.u32(held);
        if pointer != 0 && e.call(INTERLOCKED_DECREMENT, &args![pointer + 4]).u32() == 0 {
            e.vcall(pointer, 0x4, &[]);
        }
    });
}

/// `NiNode::UpdateSelectedDownwardPass` (`00a5ded0`) and `NiNode::UpdateRigidDownwardPass`
/// (`00a5e1f0`), each `thiscall` with two words.
const NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;
const NODE_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0x00a5_e1f0;

// Translated from 00c44e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::UpdateSelectedDownwardPass` (Xbox PDB; `thiscall`, two words, `RET
/// 8`): `RemoveExpired` with the float `update_data` points at (copied through the x87
/// stack), then `NiNode::UpdateSelectedDownwardPass(update_data, flags)`.
pub fn bs_temp_node_manager_update_selected_downward_pass(
    e: &mut Engine,
    this: u32,
    update_data: u32,
    flags: u32,
) {
    let time = x87_quiet_bits(e.mem.u32(update_data));
    bs_temp_node_manager_remove_expired(e, this, time);
    e.call(
        NODE_UPDATE_SELECTED_DOWNWARD_PASS,
        &args![this, update_data, flags],
    );
}

// Translated from 00c44ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::UpdateRigidDownwardPass` (Xbox PDB; `thiscall`, two words, `RET 8`):
/// `RemoveExpired` with the float `update_data` points at (copied through the x87 stack),
/// then `NiNode::UpdateRigidDownwardPass(update_data, flags)`.
pub fn bs_temp_node_manager_update_rigid_downward_pass(
    e: &mut Engine,
    this: u32,
    update_data: u32,
    flags: u32,
) {
    let time = x87_quiet_bits(e.mem.u32(update_data));
    bs_temp_node_manager_remove_expired(e, this, time);
    e.call(
        NODE_UPDATE_RIGID_DOWNWARD_PASS,
        &args![this, update_data, flags],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_4e70,
            bs_temp_node_manager_update_selected_downward_pass(u32, u32, u32)
        ),
        entry!(
            0x00c4_4ea0,
            bs_temp_node_manager_update_rigid_downward_pass(u32, u32, u32)
        ),
        entry!(
            0x00c4_4e40,
            bs_temp_node_manager_update_downward_pass(u32, u32, u32)
        ),
        entry!(0x00c4_4ed0, bs_temp_node_manager_remove_expired(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A manager with a vtable of `0x100` words at `0x0090_0000` and the given children.
    fn manager(e: &mut Engine, children: &[u32]) -> u32 {
        let this = e.mem.alloc(0xb0);
        e.put_vtable(0x0090_0000, &[0; 0x40]);
        e.mem.set_u32(this, 0x0090_0000);
        let base = e.mem.alloc(0x40);
        for (i, child) in (0u32..).zip(children) {
            e.mem.set_u32(base + i * 4, *child);
        }
        e.mem.set_u32(this + CHILDREN_BASE, base);
        e.mem.set_u16(this + CHILDREN_SIZE, children.len() as u16);
        this
    }

    /// A child whose `GetRTTI` (slot +8, `0x0090_1000`) answers the word at its +0x10.
    fn child(e: &mut Engine, rtti: u32, time: f32) -> u32 {
        let child = e.mem.alloc(0xb0);
        e.put_vtable(0x0090_2000, &[0, 0, 0x0090_1000]);
        e.mem.set_u32(child, 0x0090_2000);
        e.mem.set_u32(child + 0x10, rtti);
        e.mem.set_f32(child + TIME, time);
        child
    }

    #[test]
    fn remove_expired_counts_down_and_detaches_temp_nodes() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(TEMP_NODE_RTTI & !0xfff, 0x1000);
        e.register(0x0090_1000, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0090_3000, |_, _| Ret::default());
        // An RTTI record deriving from the temp node's one.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, TEMP_NODE_RTTI);
        let other = e.mem.alloc(8);
        let expiring = child(&mut e, derived, 0.5);
        let lasting = child(&mut e, TEMP_NODE_RTTI, 3.0);
        let foreign = child(&mut e, other, 0.25);
        let this = manager(&mut e, &[expiring, 0, lasting, foreign]);
        e.mem.set_u32(0x0090_0000 + 0xec, 0x0090_3000);
        e.mem.set_f32(this + TIME, 9.0);

        // fPrevTime 9.0, time 10.0: one second passes.
        e.call_log = Some(vec![]);
        e.call(0x00c4_4ed0, &[this, 10.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + TIME), 10.0);
        assert_eq!(e.mem.f32(expiring + TIME), 0.0);
        assert_eq!(e.mem.f32(lasting + TIME), 2.0);
        assert_eq!(e.mem.f32(foreign + TIME), 0.25);
        let detached = calls_to(&e, 0x0090_3000);
        assert_eq!(detached.len(), 1);
        assert_eq!(&detached[0][..2], &[this, 0]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());

        // fPrevTime 0.0: the passed time is 0.0, nothing changes and nothing is detached.
        e.mem.set_f32(this + TIME, 0.0);
        e.mem.set_f32(expiring + TIME, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4ed0, &[this, 4.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + TIME), 4.0);
        assert_eq!(e.mem.f32(lasting + TIME), 2.0);
        assert!(calls_to(&e, 0x0090_3000).is_empty());
    }

    #[test]
    fn remove_expired_releases_the_detached_pointer() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(TEMP_NODE_RTTI & !0xfff, 0x1000);
        e.register(0x0090_1000, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let expiring = child(&mut e, TEMP_NODE_RTTI, 0.5);
        let this = manager(&mut e, &[expiring]);
        e.mem.set_u32(0x0090_0000 + 0xec, 0x0090_3000);
        e.mem.set_f32(this + TIME, 1.0);
        // The detach hands the child out through the pointer, with a count of 1.
        e.register_double(0x0090_3000, move |e, a| {
            e.mem.set_u32(a[2], expiring);
            Ret::default()
        });
        e.mem.set_u32(expiring + 4, 1);
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.mem.set_u32(0x0090_2000 + 4, 0x0090_4000);
        e.register(0x0090_4000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_4ed0, &[this, 2.0f32.to_bits()]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![expiring + 4]]
        );
        assert_eq!(calls_to(&e, 0x0090_4000), vec![vec![expiring]]);
    }

    #[test]
    fn update_downward_pass_removes_then_updates() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.register(NODE_UPDATE_DOWNWARD_PASS, |_, _| Ret::default());
        let this = manager(&mut e, &[]);
        let data = e.mem.alloc(8);
        e.mem.set_f32(data, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4e40, &[this, data, 3]);
        assert_eq!(e.mem.f32(this + TIME), 5.0);
        assert_eq!(
            calls_to(&e, NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![this, data, 3]]
        );
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn childless_manager(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xb0);
        e.mem.set_f32(this + TIME, 2.0);
        this
    }

    #[test]
    fn selected_and_rigid_passes_remove_then_update() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.register(NODE_UPDATE_SELECTED_DOWNWARD_PASS, |_, _| Ret::default());
        e.register(NODE_UPDATE_RIGID_DOWNWARD_PASS, |_, _| Ret::default());
        let data = e.mem.alloc(8);
        e.mem.set_f32(data, 5.0);
        for (address, pass) in [
            (0x00c4_4e70, NODE_UPDATE_SELECTED_DOWNWARD_PASS),
            (0x00c4_4ea0, NODE_UPDATE_RIGID_DOWNWARD_PASS),
        ] {
            let this = childless_manager(&mut e);
            e.call_log = Some(vec![]);
            e.call(address, &[this, data, 7]);
            assert_eq!(e.mem.f32(this + TIME), 5.0);
            assert_eq!(calls_to(&e, pass), vec![vec![this, data, 7]]);
            let other = if pass == NODE_UPDATE_RIGID_DOWNWARD_PASS {
                NODE_UPDATE_SELECTED_DOWNWARD_PASS
            } else {
                NODE_UPDATE_RIGID_DOWNWARD_PASS
            };
            assert!(calls_to(&e, other).is_empty());
        }
    }
}
