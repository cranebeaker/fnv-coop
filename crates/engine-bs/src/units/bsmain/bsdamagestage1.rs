//! `BSMain/bsdamagestage1.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c45220` (named `BSRangeNode::LoadBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00C45220: u32 = 0x00c4_5220;

// Translated from 00c4b000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c2120 of the class in unit `bsdamagestage1.obj`.
/// Forwards `this` and the stream to `00c45220` (`BSRangeNode::LoadBinary`). Returns nothing:
/// Xbox PDB `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4b000(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00C45220, &args![this, stream]);
}

/// `00c452a0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C452A0: u32 = 0x00c4_52a0;

// Translated from 00c4b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c2124 of the `bsdamagestage1.obj` class, the slot
/// after the `LoadBinary` word 010c2120. Forwards `this` and the stream to `00c452a0`
/// (`bsrangenode.obj`, unnamed; its body calls `NiNode::LinkObject` `00a5e9d0`). Returns
/// nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4b020(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C452A0, &args![this, stream]);
}

/// `00c452f0` (named `BSRangeNode::SaveBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00C452F0: u32 = 0x00c4_52f0;

// Translated from 00c4b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c212c of the `bsdamagestage1.obj` class, three
/// words after the `LoadBinary` word 010c2120. Forwards `this` and the stream to `00c452f0`
/// (`BSRangeNode::SaveBinary`). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4b070(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00C452F0, &args![this, stream]);
}

// Translated from 00c4b210 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e8c without reading the receiver. The map names this body `BSDamageStage::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c4b210(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e8c
}

/// `BSDamageStage`'s vtable (its class confirmed by the RTTI before it), `operator new`
/// (`cdecl`, `(size)`), `BSRangeNode::BSRangeNode` (`thiscall`, no arguments),
/// `BSRangeNode::CopyMembers` (`thiscall`, `(destination, cloning)`) and
/// `NiNode::UpdateDownwardPass` / `NiNode::UpdateSelectedDownwardPass` (`thiscall`, two
/// words).
const BS_DAMAGE_STAGE_VTABLE: u32 = 0x010c_20d4;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const RANGE_NODE_CONSTRUCTOR: u32 = 0x00c4_50d0;
const RANGE_NODE_COPY_MEMBERS: u32 = 0x00c4_51b0;
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;

/// A 0xB0-byte block (`operator new`) constructed as a `BSRangeNode` and given
/// `BSDamageStage`'s vtable; null when the allocation fails.
fn new_damage_stage(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(RANGE_NODE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, BS_DAMAGE_STAGE_VTABLE);
    Ptr::new(block)
}

// Translated from 00c4b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDamageStage` factory (unnamed in the map): a 0xB0-byte block constructed as a
/// `BSRangeNode` with `BSDamageStage`'s vtable, or null when the allocation fails.
// Translation incomplete: 00c4b0c0 - SEH exception unwinding is unsupported.
pub fn fn_00c4b0c0(e: &mut Engine) -> Ptr {
    new_damage_stage(e)
}

// Translated from 00c4afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage`'s `CopyMembers` (unnamed in the map), `(destination, cloning)`:
/// `BSRangeNode::CopyMembers` with the same arguments.
pub fn fn_00c4afa0(e: &mut Engine, this: Ptr, destination: Ptr, cloning: Ptr) {
    e.call(RANGE_NODE_COPY_MEMBERS, &args![this, destination, cloning]);
}

// Translated from 00c4af00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage::CreateClone` (Xbox PDB), `(cloning)`: a new damage stage (as the factory
/// makes it) and this node's members copied into it (`fn_00c4afa0`, also with a null
/// destination when the allocation failed). Answers the clone.
// Translation incomplete: 00c4af00 - SEH exception unwinding is unsupported.
pub fn bs_damage_stage_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_damage_stage(e);
    fn_00c4afa0(e, this, clone, cloning);
    clone
}

// Translated from 00c4b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage::UpdateDownwardPass` (Xbox PDB): `NiNode::UpdateDownwardPass` with the
/// same two words unless bit 0 of the flags (`NiAVObject::m_uFlags`, +0x30) is set.
pub fn bs_damage_stage_update_downward_pass(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    if e.mem.u32(this.addr() + 0x30) & 1 == 0 {
        e.call(NI_NODE_UPDATE_DOWNWARD_PASS, &args![this, first, second]);
    }
}

// Translated from 00c4b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage::UpdateSelectedDownwardPass` (Xbox PDB): `NiNode::UpdateSelectedDownwardPass`
/// with the same two words unless bit 0 of the flags (+0x30) is set.
pub fn bs_damage_stage_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) {
    if e.mem.u32(this.addr() + 0x30) & 1 == 0 {
        e.call(
            NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS,
            &args![this, first, second],
        );
    }
}

/// `BSRangeNode::GetViewerStrings` (`00c454c0`), `00c452c0` (unnamed,
/// `BSMain/bsrangenode.obj`) and `BSRangeNode::IsEqual` (`00c45380`):
/// `thiscall`, one word each.
const RANGE_NODE_GET_VIEWER_STRINGS: u32 = 0x00c4_54c0;
const RANGE_NODE_REGISTER_STREAMABLES: u32 = 0x00c4_52c0;
const RANGE_NODE_IS_EQUAL: u32 = 0x00c4_5380;
/// `NiNode::UpdateRigidDownwardPass` (`00a5e1f0`, `thiscall`, two words).
const NI_NODE_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0x00a5_e1f0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name) and the
/// viewer-string array's `Add` (`0096a610`, `thiscall` on the array, the
/// string's address; the engine map places it in a folded unit).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_ADD: u32 = 0x0096_a610;
/// `BSDamageStage`'s `NiRTTI` record (its first word is the class name).
const BS_DAMAGE_STAGE_RTTI: u32 = 0x0120_2e8c;

// Translated from 00c4afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage::GetViewerStrings` (Xbox PDB): the range-node strings
/// (`00c454c0`), then the class name added through `0096a610` with the
/// address of the local holding it.
pub fn bs_damage_stage_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(RANGE_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(BS_DAMAGE_STAGE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c4b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSDamageStage` vtable's
/// `RegisterStreamables` slot (word `010c2128`). Answers whether `00c452c0`
/// (the range node's, `thiscall`, the stream) answered nonzero in AL.
pub fn fn_00c4b040(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(RANGE_NODE_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c4b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSDamageStage` vtable's `IsEqual` slot
/// (word `010c2130`). Answers `BSRangeNode::IsEqual` (`00c45380`): its AL as it is.
pub fn fn_00c4b090(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    e.call(RANGE_NODE_IS_EQUAL, &args![this, other]).u8()
}

// Translated from 00c4b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDamageStage::UpdateRigidDownwardPass` (Xbox PDB):
/// `NiNode::UpdateRigidDownwardPass` with the same two words unless bit 0
/// of the flags (`NiAVObject::m_uFlags`, +0x30) is set.
pub fn bs_damage_stage_update_rigid_downward_pass(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) {
    if e.mem.u32(this.addr() + 0x30) & 1 == 0 {
        e.call(
            NI_NODE_UPDATE_RIGID_DOWNWARD_PASS,
            &args![this, first, second],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_afc0, bs_damage_stage_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c4_b040, fn_00c4b040(Ptr, Ptr) -> bool),
        entry!(0x00c4_b090, fn_00c4b090(Ptr, Ptr) -> u8),
        entry!(
            0x00c4_b1d0,
            bs_damage_stage_update_rigid_downward_pass(Ptr, u32, u32)
        ),
        entry!(0x00c4_b210, fn_00c4b210(Ptr) -> u32),
        entry!(0x00c4_b000, fn_00c4b000(Ptr, Ptr)),
        entry!(0x00c4_b020, fn_00c4b020(Ptr, Ptr)),
        entry!(0x00c4_b070, fn_00c4b070(Ptr, Ptr)),
        entry!(0x00c4b0c0, fn_00c4b0c0() -> Ptr),
        entry!(0x00c4afa0, fn_00c4afa0(Ptr, Ptr, Ptr)),
        entry!(0x00c4af00, bs_damage_stage_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c4b150,
            bs_damage_stage_update_downward_pass(Ptr, u32, u32)
        ),
        entry!(
            0x00c4b190,
            bs_damage_stage_update_selected_downward_pass(Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c4b000_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00C45220, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_b000, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00C45220)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4b020_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C452A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_b020, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C452A0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4b070_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00C452F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_b070, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00C452F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c4b210_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_b210, &args![0x1000u32]).u32(), 0x0120_2e8c);
        assert_eq!(e.call(0x00c4_b210, &args![0u32]).u32(), 0x0120_2e8c);
    }

    #[test]
    fn damage_stage_factory_and_clone() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(RANGE_NODE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(RANGE_NODE_COPY_MEMBERS, |_, _| Ret::default());
        let made = e.call(0x00c4b0c0, &args![]).u32();
        assert_eq!(e.mem.u32(made), BS_DAMAGE_STAGE_VTABLE);
        let clone = e.call(0x00c4af00, &args![0x5000u32, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), BS_DAMAGE_STAGE_VTABLE);
        let log = e.call_log.take().unwrap();
        let copies: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == RANGE_NODE_COPY_MEMBERS)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(copies, vec![vec![0x5000, clone, 0x6000]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c4b0c0, &args![]).u32(), 0);
    }

    #[test]
    fn damage_stage_copy_members_forwards_to_the_range_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(RANGE_NODE_COPY_MEMBERS, |_, _| Ret::default());
        e.call(0x00c4afa0, &args![1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (RANGE_NODE_COPY_MEMBERS, vec![1, 2, 3]));
    }

    #[test]
    fn damage_stage_downward_passes_skip_hidden_nodes() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_NODE_UPDATE_DOWNWARD_PASS, |_, _| Ret::default());
        e.register(NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        e.call(0x00c4b150, &args![this, 7u32, 8u32]);
        e.call(0x00c4b190, &args![this, 7u32, 8u32]);
        e.mem.set_u32(this + 0x30, 1);
        e.call(0x00c4b150, &args![this, 7u32, 8u32]);
        e.call(0x00c4b190, &args![this, 7u32, 8u32]);
        let log = e.call_log.take().unwrap();
        let forwarded: Vec<_> = log
            .iter()
            .filter(|c| {
                c.0 == NI_NODE_UPDATE_DOWNWARD_PASS || c.0 == NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS
            })
            .cloned()
            .collect();
        assert_eq!(
            forwarded,
            vec![
                (NI_NODE_UPDATE_DOWNWARD_PASS, vec![this, 7, 8]),
                (NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS, vec![this, 7, 8])
            ]
        );
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        e.map(BS_DAMAGE_STAGE_RTTI, 4);
        e.set_global(BS_DAMAGE_STAGE_RTTI, 0x0555_0000u32);
        e.register(RANGE_NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(STRINGS_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7000_0001);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c4_afc0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, RANGE_NODE_GET_VIEWER_STRINGS),
            vec![vec![0x1000, 0x2000]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0555_0000]]);
        assert_eq!(calls_to(&e, STRINGS_ADD)[0][0], 0x2000);
    }

    #[test]
    fn register_streamables_answers_whether_the_base_answered_nonzero() {
        let mut e = Engine::new();
        e.register(RANGE_NODE_REGISTER_STREAMABLES, |_, _| word(0x1234_5600));
        assert_eq!(e.call(0x00c4_b040, &args![0x1000u32, 0x2000u32]).u32(), 0);
        e.register(RANGE_NODE_REGISTER_STREAMABLES, |_, _| word(0x4));
        assert_eq!(e.call(0x00c4_b040, &args![0x1000u32, 0x2000u32]).u32(), 1);
    }

    #[test]
    fn is_equal_passes_on_the_range_node_answer() {
        let mut e = Engine::new();
        e.register(RANGE_NODE_IS_EQUAL, |_, a| word(a[1] & 0xff));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_b090, &args![0x1000u32, 0x2001u32]).u32(), 1);
        assert_eq!(e.call(0x00c4_b090, &args![0x1000u32, 0x2000u32]).u32(), 0);
        assert_eq!(calls_to(&e, RANGE_NODE_IS_EQUAL)[0], vec![0x1000, 0x2001]);
    }

    #[test]
    fn update_rigid_downward_pass_skips_flagged_nodes() {
        let mut e = Engine::new();
        e.register(NI_NODE_UPDATE_RIGID_DOWNWARD_PASS, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00c4_b1d0, &args![this, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, NI_NODE_UPDATE_RIGID_DOWNWARD_PASS),
            vec![vec![this, 7, 8]]
        );
        e.mem.set_u32(this + 0x30, 0xffff_ff01);
        e.call_log = Some(vec![]);
        e.call(0x00c4_b1d0, &args![this, 7u32, 8u32]);
        assert!(calls_to(&e, NI_NODE_UPDATE_RIGID_DOWNWARD_PASS).is_empty());
    }
}
