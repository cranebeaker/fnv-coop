//! `NiParticle/nipsysmodifierctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a35850`: the jump target of `00a57630`.
const JUMP_TARGET_00A57630: u32 = 0x00a3_5850;

/// `00a38fe0`: the jump target of `00a57780`.
const JUMP_TARGET_00A57780: u32 = 0x00a3_8fe0;

// Translated from 00a57630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::LinkObject` (Xbox PDB): a 5-byte jump to `00a35850` (engine map: `NiSingleInterpController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysModifierCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57630, &args![this, stream]);
}

// Translated from 00a57780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a38fe0` (engine map: `NiSingleInterpController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysModifierCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_ctlr_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57780, &args![this, stream]).bool()
}

// Translated from 00c31290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27c8.
pub fn ni_p_sys_modifier_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27c8
}

use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::units::niparticle::nipsysmodifier::{
    load_name, CLONE_MAP_LOOKUP, INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT,
};

layout! {
    /// `NiPSysModifierCtlr` (Xbox PDB), 0x40 bytes; the PC code uses the
    /// same offsets. The base's target is `NiTimeController::m_pkTarget`.
    pub struct NiPSysModifierCtlr: 0x40 {
        /// `NiSingleInterpController::m_spInterpolator` (Xbox PDB):
        /// `NiPointer<NiInterpolator>`.
        0x34 m_spInterpolator: u32,
        /// `m_kModifierName` (Xbox PDB): an `NiFixedString` handle.
        0x38 m_kModifierName: u32,
        /// `m_pkModifier` (Xbox PDB): `NiPSysModifier*`.
        0x3C m_pkModifier: u32,
    }
}

/// `00a2f8a0` (unnamed; a 5-byte jump): the base member copy, `thiscall`,
/// `(dest, process)`.
const BASE_COPY_MEMBERS: u32 = 0x00a2_f8a0;
/// `00a5d3f0` (unnamed, `NiMain/niobject.cpp`): the clone of an object for
/// a cloning process (`thiscall` on the object, one word): its entry in the
/// process's map, else a new clone through vtable slot +0x48.
const CLONE_FOR_PROCESS: u32 = 0x00a5_d3f0;
/// `NiSingleInterpController::LoadBinary` (`00a38f80`, one word).
const SINGLE_INTERP_LOAD_BINARY: u32 = 0x00a3_8f80;
/// `NiSingleInterpController::ProcessClone` (`00a38f50`, one word).
const SINGLE_INTERP_PROCESS_CLONE: u32 = 0x00a3_8f50;
/// `00a89af0` (unnamed): the base post-link step, `thiscall`, one word.
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiParticleSystem`'s modifier list head (+0xc8, the list's first item;
/// items are `[next, previous, element]`), as the PC code reads it.
const SYSTEM_MODIFIER_LIST_HEAD: u32 = 0xc8;

// Translated from 00a38ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `NiPSysModifierCtlr::CopyMembers`, but the body
/// (folded code; it is also reached from other classes' `CopyMembers` and
/// `CreateClone`) copies only the single-interpolator controller's part:
/// the base copy `00a2f8a0` into `dest`, then, when `this` has an
/// interpolator, its clone for the process (`00a5d3f0`) becomes `dest`'s
/// interpolator (`NiPointer` assignment: the old one released and deleted
/// through vtable slot +4 at zero, the new one held). `thiscall`, `RET 8`.
pub fn fn_00a38ef0(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    dest: Ptr<NiPSysModifierCtlr>,
    process: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, process]);
    let interpolator = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
    if interpolator == 0 {
        return;
    }
    let clone = e
        .call(CLONE_FOR_PROCESS, &args![interpolator, process])
        .u32();
    let old = e.get(dest, NiPSysModifierCtlr::m_spInterpolator);
    if old == clone {
        return;
    }
    if old != 0 {
        let left = e
            .call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.set(dest, NiPSysModifierCtlr::m_spInterpolator, clone);
    if clone != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![clone.wrapping_add(4)]);
    }
}

// Translated from 00a571a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::LoadBinary` (Xbox PDB): the single-interpolator
/// controller's `LoadBinary`, then the modifier name (a C string before
/// file version 20.1.0.1, a fixed string from it).
pub fn ni_p_sys_modifier_ctlr_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(SINGLE_INTERP_LOAD_BINARY, &args![this, stream]);
    load_name(
        e,
        stream.addr(),
        this.addr() + NiPSysModifierCtlr::m_kModifierName.off,
    );
}

// Translated from 00c31340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::ProcessClone` (Xbox PDB): the single-interpolator
/// controller's `ProcessClone`, then the clone of `this` from the process's
/// map (into a stack word cleared first); when both `this` and the clone
/// have a target, the clone of the modifier (looked up into the same word)
/// becomes the clone's modifier.
pub fn ni_p_sys_modifier_ctlr_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    process: Ptr,
) {
    e.call(SINGLE_INTERP_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::<NiTimeController>::new(e.mem.u32(slot.addr()));
        if e.get(
            this.cast::<NiTimeController>(),
            NiTimeController::m_pkTarget,
        ) == 0
        {
            return;
        }
        if e.get(clone, NiTimeController::m_pkTarget) == 0 {
            return;
        }
        let modifier = e.get(this, NiPSysModifierCtlr::m_pkModifier);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, modifier, slot]);
        let modifier_clone = e.mem.u32(slot.addr());
        e.set(
            clone.cast::<NiPSysModifierCtlr>(),
            NiPSysModifierCtlr::m_pkModifier,
            modifier_clone,
        );
    });
}

// Translated from 00c31400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: finds the controller's modifier by name. Walks
/// the target particle system's modifier list (head at +0xc8); the first
/// non-null modifier whose name (`NiPSysModifier::m_kName`, +8) is the
/// controller's modifier name becomes `m_pkModifier`, else it is cleared.
pub fn fn_00c31400(e: &mut Engine, this: Ptr<NiPSysModifierCtlr>) {
    let target = e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    );
    let mut item = e.mem.u32(target.wrapping_add(SYSTEM_MODIFIER_LIST_HEAD));
    while item != 0 {
        let modifier = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if modifier != 0
            && e.mem.u32(modifier + 8) == e.get(this, NiPSysModifierCtlr::m_kModifierName)
        {
            e.set(this, NiPSysModifierCtlr::m_pkModifier, modifier);
            return;
        }
    }
    e.set(this, NiPSysModifierCtlr::m_pkModifier, 0);
}

// Translated from 00c31590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::PostLinkObject` (Xbox PDB): the base post-link step
/// (`00a89af0`), then, with a target, the modifier lookup (`00c31400`).
pub fn ni_p_sys_modifier_ctlr_post_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    ) != 0
    {
        fn_00c31400(e, this);
    }
}

use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, fixed_string_assign, fixed_string_release, FIXED_STRING_NULL,
    NI_DELETE, NI_GET_VIEWER_STRING_FIXED, SAVE_FIXED_STRING,
};

/// The vtable of `NiPSysModifierCtlr` (RTTI `.?AVNiPSysModifierCtlr@@`).
const VTABLE: u32 = 0x010c_08ac;
/// The `NiRTTI` records of `NiPSysModifierCtlr` (`012027c8`) and
/// `NiParticleSystem` (`012024e0`, the class name its static initializer
/// gives it).
const RTTI: u32 = 0x0120_27c8;
pub(crate) const PARTICLE_SYSTEM_RTTI: u32 = 0x0120_24e0;
/// `NiSingleInterpController::SaveBinary` (`00a39010`), `IsEqual`
/// (`00a39030`) and `GetViewerStrings` (`00a390a0`): `thiscall`, one word.
const SINGLE_INTERP_SAVE_BINARY: u32 = 0x00a3_9010;
const SINGLE_INTERP_IS_EQUAL: u32 = 0x00a3_9030;
const SINGLE_INTERP_GET_VIEWER_STRINGS: u32 = 0x00a3_90a0;
/// `00a38e00` (`thiscall`, no stack words): the base constructor (it runs
/// `00a2f8c0`, stores `NiSingleInterpController`'s vtable `01096bd4` and
/// clears the interpolator; the engine map's name for it,
/// `NiBoolInterpController::CopyMembers`, is a folded body's).
const SINGLE_INTERP_CONSTRUCTOR: u32 = 0x00a3_8e00;
/// `00a6d290` (unnamed, `NiMain/nitimecontroller.cpp`): `NiTimeController`'s
/// destructor body (`thiscall`).
const TIME_CONTROLLER_DESTRUCTOR: u32 = 0x00a6_d290;
/// `NiTimeController::SetTarget` (`00a6d450`, `thiscall`, one word).
const TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
/// `00c1b0f0` (unnamed, `NiParticle/niparticlesystem.cpp`): the particle
/// system's modifier with the name at the address it is given (`thiscall`
/// on the system, one word), or 0.
pub(crate) const SYSTEM_FIND_MODIFIER: u32 = 0x00c1_b0f0;
/// `NiObject::GetRTTI` (vtable slot +8) and `NiRTTI::m_pkBaseRTTI` (+4).
const GET_RTTI_SLOT: u32 = 0x8;
const RTTI_BASE: u32 = 0x4;
/// `"Modifier Name"`.
const LABEL_MODIFIER_NAME: u32 = 0x010c_099c;
/// The address of `NiPSysModifierCtlr::InterpTargetIsCorrectType`
/// (`00c31530`), which the subclasses' checks call.
pub(crate) const INTERP_TARGET_IS_CORRECT_TYPE: u32 = 0x00c3_1530;

/// The `IsKindOf` walk the code inlines: `object`'s `NiRTTI` (vtable slot
/// +8), then each base (+4), until `rtti` (true) or null (false).
pub(crate) fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, GET_RTTI_SLOT, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(RTTI_BASE));
    }
    false
}

/// The `InterpTargetIsCorrectType` the float and bool controller subclasses
/// share: `NiPSysModifierCtlr::InterpTargetIsCorrectType` (`00c31530`) on
/// `target`; then the target system's modifier with this controller's
/// modifier name (`00c1b0f0`) must exist and be of kind `rtti`.
pub(crate) fn interp_target_modifier_is(e: &mut Engine, this: u32, target: u32, rtti: u32) -> bool {
    if !e
        .call(INTERP_TARGET_IS_CORRECT_TYPE, &args![this, target])
        .bool()
    {
        return false;
    }
    let name = this.wrapping_add(NiPSysModifierCtlr::m_kModifierName.off);
    let modifier = e.call(SYSTEM_FIND_MODIFIER, &args![target, name]).u32();
    if modifier == 0 {
        return false;
    }
    is_kind_of(e, modifier, rtti)
}

// Translated from 00a57130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::SaveBinary` (Xbox PDB): the single-interpolator
/// controller's `SaveBinary`, then the modifier name
/// (`NiStream::SaveFixedString`).
pub fn ni_p_sys_modifier_ctlr_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(SINGLE_INTERP_SAVE_BINARY, &args![this, stream]);
    e.call(
        SAVE_FIXED_STRING,
        &args![
            stream,
            this.addr() + NiPSysModifierCtlr::m_kModifierName.off
        ],
    );
}

// Translated from 00c313a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::IsEqual` (Xbox PDB): the single-interpolator
/// controller's `IsEqual`; both or neither must have a modifier, two
/// modifiers must be equal (the first's `IsEqual`, slot +0x5c), and the
/// modifier names must be the same handle.
pub fn ni_p_sys_modifier_ctlr_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    other: Ptr<NiPSysModifierCtlr>,
) -> bool {
    if !e.call(SINGLE_INTERP_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.get(this, NiPSysModifierCtlr::m_pkModifier);
    let theirs = e.get(other, NiPSysModifierCtlr::m_pkModifier);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 && !e.vcall(mine, 0x5c, &args![theirs]).bool() {
        return false;
    }
    e.get(this, NiPSysModifierCtlr::m_kModifierName)
        == e.get(other, NiPSysModifierCtlr::m_kModifierName)
}

// Translated from 00c31440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifierCtlr` constructor with a
/// modifier name (`thiscall`, `RET 4`: the address of the name's handle).
/// The base constructor (`00a38e00`), this class's vtable `010c08ac`, the
/// null name handle and no modifier; then the name is assigned (held unless
/// null, the old one released unless null). Returns `this`.
pub fn fn_00c31440(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    name: Ptr,
) -> Ptr<NiPSysModifierCtlr> {
    e.call(SINGLE_INTERP_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let null = e.global::<u32>(FIXED_STRING_NULL);
    e.set(this, NiPSysModifierCtlr::m_kModifierName, null);
    e.set(this, NiPSysModifierCtlr::m_pkModifier, 0);
    let handle = e.mem.u32(name.addr());
    fixed_string_assign(
        e,
        this.addr() + NiPSysModifierCtlr::m_kModifierName.off,
        handle,
    );
    this
}

// Translated from 00c314a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysModifierCtlr` deleting destructor (unnamed in the engine map;
/// `RET 4`): the vtable back to this class's, the modifier name released
/// unless null, the interpolator released (deleted through its slot +4 when
/// the count reaches 0; the pointer is left as it is), `NiTimeController`'s
/// destructor body (`00a6d290`), then, with bit 0 of `flags`, the sized
/// delete of 0x40 bytes. Returns `this`.
pub fn fn_00c314a0(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    flags: u32,
) -> Ptr<NiPSysModifierCtlr> {
    e.mem.set_u32(this.addr(), VTABLE);
    fixed_string_release(e, this.addr() + NiPSysModifierCtlr::m_kModifierName.off);
    let interpolator = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
    if interpolator != 0
        && e.call(INTERLOCKED_DECREMENT, &args![interpolator.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(interpolator, 4, &args![]);
    }
    e.call(TIME_CONTROLLER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x40u32]);
    }
    this
}

// Translated from 00c31500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::SetTarget` (Xbox PDB): clears the modifier,
/// `NiTimeController::SetTarget`, then, with a target, the modifier lookup
/// by name (`00c31400`).
pub fn ni_p_sys_modifier_ctlr_set_target(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    target: Ptr,
) {
    e.set(this, NiPSysModifierCtlr::m_pkModifier, 0);
    e.call(TIME_CONTROLLER_SET_TARGET, &args![this, target]);
    if e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    ) != 0
    {
        fn_00c31400(e, this);
    }
}

// Translated from 00c31530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::InterpTargetIsCorrectType` (Xbox PDB): true when
/// `target` is non-null and of kind `NiParticleSystem` (its `NiRTTI` chain
/// reaches `012024e0`), the modifier name is not the null handle, the
/// controller has a target, and `target` has a modifier of that name
/// (`00c1b0f0`).
pub fn ni_p_sys_modifier_ctlr_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    target: Ptr,
) -> bool {
    if target.is_null() || !is_kind_of(e, target.addr(), PARTICLE_SYSTEM_RTTI) {
        return false;
    }
    if e.get(this, NiPSysModifierCtlr::m_kModifierName) == e.global::<u32>(FIXED_STRING_NULL) {
        return false;
    }
    if e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    ) == 0
    {
        return false;
    }
    let name = this.addr() + NiPSysModifierCtlr::m_kModifierName.off;
    e.call(SYSTEM_FIND_MODIFIER, &args![target, name]).u32() != 0
}

// Translated from 00c315b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierCtlr::GetViewerStrings` (Xbox PDB): the
/// single-interpolator controller's `GetViewerStrings`, then the class name
/// (`NiRTTI` `012027c8`) and `"Modifier Name"` (the name handle) appended.
pub fn ni_p_sys_modifier_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    strings: Ptr,
) {
    e.call(SINGLE_INTERP_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
    let name = e.get(this, NiPSysModifierCtlr::m_kModifierName);
    add_labelled(
        e,
        strings.addr(),
        NI_GET_VIEWER_STRING_FIXED,
        LABEL_MODIFIER_NAME,
        name,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_7130,
            ni_p_sys_modifier_ctlr_save_binary(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_13a0,
            ni_p_sys_modifier_ctlr_is_equal(
                Ptr<NiPSysModifierCtlr>,
                Ptr<NiPSysModifierCtlr>,
            ) -> bool
        ),
        entry!(
            0x00c3_1440,
            fn_00c31440(Ptr<NiPSysModifierCtlr>, Ptr) -> Ptr<NiPSysModifierCtlr>
        ),
        entry!(
            0x00c3_14a0,
            fn_00c314a0(Ptr<NiPSysModifierCtlr>, u32) -> Ptr<NiPSysModifierCtlr>
        ),
        entry!(
            0x00c3_1500,
            ni_p_sys_modifier_ctlr_set_target(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_1530,
            ni_p_sys_modifier_ctlr_interp_target_is_correct_type(
                Ptr<NiPSysModifierCtlr>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00c3_15b0,
            ni_p_sys_modifier_ctlr_get_viewer_strings(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00a3_8ef0,
            fn_00a38ef0(Ptr<NiPSysModifierCtlr>, Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00a5_71a0,
            ni_p_sys_modifier_ctlr_load_binary(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_1340,
            ni_p_sys_modifier_ctlr_process_clone(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(0x00c3_1400, fn_00c31400(Ptr<NiPSysModifierCtlr>)),
        entry!(
            0x00c3_1590,
            ni_p_sys_modifier_ctlr_post_link_object(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(0x00c31290, ni_p_sys_modifier_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00a5_7630, ni_p_sys_modifier_ctlr_link_object(Ptr, Ptr)),
        entry!(0x00a5_7780, ni_p_sys_modifier_ctlr_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a57630_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7630;
        const TARGET: u32 = 0x00a3_5850;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a57780_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7780;
        const TARGET: u32 = 0x00a3_8fe0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn ni_p_sys_modifier_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1290, &args![]).u32(), 0x0120_27c8);
    }

    use crate::units::niparticle::nipsysmodifier::{test_stream, LOAD_FIXED_STRING};

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A reference-counted object with a vtable whose slot +4 is
    /// `delete_this`.
    fn ref_object(e: &mut Engine, count: u32, delete_this: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, delete_this);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn copy_members_assigns_the_interpolator_clone() {
        const DELETE: u32 = 0x0f10_0000;
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        let dest = e.new_object::<NiPSysModifierCtlr>();
        // No interpolator: only the base copy runs.
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x4000]]
        );
        assert!(calls_to(&e, CLONE_FOR_PROCESS).is_empty());
        let old = ref_object(&mut e, 1, DELETE);
        let clone = ref_object(&mut e, 0, DELETE);
        e.set(this, NiPSysModifierCtlr::m_spInterpolator, 0x7000);
        e.set(dest, NiPSysModifierCtlr::m_spInterpolator, old);
        e.register_double(CLONE_FOR_PROCESS, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert_eq!(calls_to(&e, CLONE_FOR_PROCESS), vec![vec![0x7000, 0x4000]]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.get(dest, NiPSysModifierCtlr::m_spInterpolator), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        // The same clone again: no reference changes.
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ef0, &args![this, dest, 0x4000u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn load_binary_reads_the_modifier_name() {
        let mut e = Engine::new();
        e.register(SINGLE_INTERP_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1402_0000, &[]);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.call_log = Some(vec![]);
        e.call(0x00a5_71a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SINGLE_INTERP_LOAD_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FIXED_STRING),
            vec![vec![stream, this.addr() + 0x38]]
        );
    }

    #[test]
    fn process_clone_maps_the_modifier_only_with_targets() {
        for (this_target, clone_target, expected) in [(1u32, 1u32, 0x6100u32), (0, 1, 0), (1, 0, 0)]
        {
            let mut e = Engine::new();
            e.register(SINGLE_INTERP_PROCESS_CLONE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysModifierCtlr>();
            let clone = e.new_object::<NiPSysModifierCtlr>();
            e.mem.set_u32(this.addr() + 0x2c, this_target);
            e.mem.set_u32(clone.addr() + 0x2c, clone_target);
            e.set(this, NiPSysModifierCtlr::m_pkModifier, 0x6000);
            let map = e.mem.alloc(4);
            e.mem.set_u32(map, clone.addr());
            let process = e.mem.alloc(4);
            e.mem.set_u32(process, map);
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                let value = if a[1] == 0x6000 {
                    0x6100
                } else {
                    e.mem.u32(a[0])
                };
                e.mem.set_u32(a[2], value);
                Ret::default()
            });
            e.call(0x00c3_1340, &args![this, process]);
            assert_eq!(e.get(clone, NiPSysModifierCtlr::m_pkModifier), expected);
        }
    }

    #[test]
    fn modifier_lookup_finds_the_named_modifier_or_clears() {
        let mut e = Engine::new();
        let system = e.mem.alloc(0xd0);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.mem.set_u32(this.addr() + 0x2c, system);
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x55);
        let first = e.mem.alloc(0x18);
        e.mem.set_u32(first + 8, 0x44);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(second + 8, 0x55);
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 8, first);
        e.mem.set_u32(items[1], items[2]);
        e.mem.set_u32(items[2] + 8, second);
        e.mem.set_u32(system + 0xc8, items[0]);
        e.call(0x00c3_1400, &args![this]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), second);
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x66);
        e.call(0x00c3_1400, &args![this]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }

    #[test]
    fn post_link_object_looks_up_the_modifier_only_with_a_target() {
        let mut e = Engine::new();
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_pkModifier, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1590, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, BASE_POST_LINK_OBJECT),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0x99);
        let system = e.mem.alloc(0xd0);
        e.mem.set_u32(this.addr() + 0x2c, system);
        e.call(0x00c3_1590, &args![this, 0x7000u32]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }

    /// An object whose `GetRTTI` (slot +8, `GET_RTTI` in the test) answers
    /// the first of a chain of `NiRTTI` records, each with the next as its
    /// base and the last without one.
    fn object_of_kind(e: &mut Engine, chain: &[u32]) -> u32 {
        const GET_RTTI: u32 = 0x0f10_0100;
        e.register(GET_RTTI, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        for pair in chain.windows(2) {
            e.map(pair[0] & !0xfff, 0x1000);
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        if let Some(last) = chain.last() {
            e.map(last & !0xfff, 0x1000);
            e.mem.set_u32(last + 4, 0);
        }
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem
            .set_u32(object + 4, chain.first().copied().unwrap_or(0));
        object
    }

    #[test]
    fn save_binary_saves_the_modifier_name_after_the_base() {
        let mut e = Engine::new();
        e.register(SINGLE_INTERP_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_FIXED_STRING, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.call_log = Some(vec![]);
        e.call(0x00a5_7130, &args![this, 0x7000u32]);
        let log: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            log,
            [0x00a5_7130, SINGLE_INTERP_SAVE_BINARY, SAVE_FIXED_STRING]
        );
        assert_eq!(
            calls_to(&e, SAVE_FIXED_STRING),
            vec![vec![0x7000, this.addr() + 0x38]]
        );
    }

    #[test]
    fn is_equal_checks_modifiers_and_names() {
        const MODIFIER_IS_EQUAL: u32 = 0x0f10_0200;
        let mut e = Engine::new();
        e.register(SINGLE_INTERP_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(MODIFIER_IS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x5c, MODIFIER_IS_EQUAL);
        let modifier = |e: &mut Engine, tag: u32| {
            let m = e.mem.alloc(8);
            e.mem.set_u32(m, vtable);
            e.mem.set_u32(m + 4, tag);
            m
        };
        let a = e.new_object::<NiPSysModifierCtlr>();
        let b = e.new_object::<NiPSysModifierCtlr>();
        e.set(a, NiPSysModifierCtlr::m_kModifierName, 0x55);
        e.set(b, NiPSysModifierCtlr::m_kModifierName, 0x55);
        // Neither has a modifier.
        assert!(e.call(0x00c3_13a0, &args![a, b]).bool());
        // Only one has one.
        let m1 = modifier(&mut e, 1);
        e.set(a, NiPSysModifierCtlr::m_pkModifier, m1);
        assert!(!e.call(0x00c3_13a0, &args![a, b]).bool());
        assert!(!e.call(0x00c3_13a0, &args![b, a]).bool());
        // Equal and unequal modifiers.
        let m2 = modifier(&mut e, 1);
        e.set(b, NiPSysModifierCtlr::m_pkModifier, m2);
        assert!(e.call(0x00c3_13a0, &args![a, b]).bool());
        e.mem.set_u32(m2 + 4, 2);
        assert!(!e.call(0x00c3_13a0, &args![a, b]).bool());
        e.mem.set_u32(m2 + 4, 1);
        // Different names.
        e.set(b, NiPSysModifierCtlr::m_kModifierName, 0x56);
        assert!(!e.call(0x00c3_13a0, &args![a, b]).bool());
        // The base says no.
        e.set(b, NiPSysModifierCtlr::m_kModifierName, 0x55);
        e.register(SINGLE_INTERP_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c3_13a0, &args![a, b]).bool());
    }

    #[test]
    fn named_constructor_sets_the_vtable_and_holds_the_name() {
        let mut e = Engine::new();
        test_stream::null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(SINGLE_INTERP_CONSTRUCTOR, |_, _| Ret::default());
        let string = e.mem.alloc(16);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, string + 8);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_pkModifier, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_1440, &args![this, name]).u32(), this.addr());
        assert_eq!(
            calls_to(&e, SINGLE_INTERP_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_kModifierName), string + 8);
        assert_eq!(e.mem.u32(string), 1);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }

    #[test]
    fn deleting_destructor_releases_name_and_interpolator() {
        const DELETE_THIS: u32 = 0x0f10_0300;
        for (flags, count) in [(1u32, 1u32), (0, 2)] {
            let mut e = Engine::new();
            test_stream::null_handle(&mut e, 0);
            test_stream::interlocked(&mut e);
            e.register(DELETE_THIS, |_, _| Ret::default());
            e.register(TIME_CONTROLLER_DESTRUCTOR, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysModifierCtlr>();
            let string = e.mem.alloc(16);
            e.mem.set_u32(string, 3);
            e.set(this, NiPSysModifierCtlr::m_kModifierName, string + 8);
            let interpolator = ref_object(&mut e, count, DELETE_THIS);
            e.set(this, NiPSysModifierCtlr::m_spInterpolator, interpolator);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c3_14a0, &args![this, flags]).u32(), this.addr());
            assert_eq!(e.mem.u32(this.addr()), VTABLE);
            assert_eq!(e.mem.u32(string), 2);
            assert_eq!(e.mem.u32(interpolator + 4), count - 1);
            assert_eq!(
                e.get(this, NiPSysModifierCtlr::m_spInterpolator),
                interpolator
            );
            assert_eq!(calls_to(&e, DELETE_THIS).len(), usize::from(count == 1));
            assert_eq!(
                calls_to(&e, TIME_CONTROLLER_DESTRUCTOR),
                vec![vec![this.addr()]]
            );
            let deletes = calls_to(&e, NI_DELETE);
            if flags == 1 {
                assert_eq!(deletes, vec![vec![this.addr(), 0x40]]);
            } else {
                assert!(deletes.is_empty());
            }
        }
    }

    #[test]
    fn set_target_clears_the_modifier_and_looks_it_up_with_a_target() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_SET_TARGET, |e, a| {
            e.mem.set_u32(a[0] + 0x2c, a[1]);
            Ret::default()
        });
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_pkModifier, 0x99);
        e.call(0x00c3_1500, &args![this, 0u32]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
        // With a target, the lookup by name finds its modifier.
        let system = e.mem.alloc(0xd0);
        let modifier = e.mem.alloc(0x18);
        e.mem.set_u32(modifier + 8, 0x55);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 8, modifier);
        e.mem.set_u32(system + 0xc8, item);
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1500, &args![this, system]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SET_TARGET),
            vec![vec![this.addr(), system]]
        );
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), modifier);
    }

    #[test]
    fn interp_target_is_correct_type_needs_a_named_system_modifier() {
        const OTHER_RTTI: u32 = 0x0120_2000;
        let mut e = Engine::new();
        test_stream::null_handle(&mut e, 0);
        e.register(SYSTEM_FIND_MODIFIER, |_, a| Ret {
            eax: u32::from(a[1] != 0) * 0x6000,
            ..Ret::default()
        });
        let system = object_of_kind(&mut e, &[OTHER_RTTI, PARTICLE_SYSTEM_RTTI]);
        let other = object_of_kind(&mut e, &[OTHER_RTTI + 0x10]);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x55);
        e.mem.set_u32(this.addr() + 0x2c, system);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c3_1530, &args![this, system]).bool());
        assert_eq!(
            calls_to(&e, SYSTEM_FIND_MODIFIER),
            vec![vec![system, this.addr() + 0x38]]
        );
        assert!(!e.call(0x00c3_1530, &args![this, other]).bool());
        assert!(!e.call(0x00c3_1530, &args![this, 0u32]).bool());
        // No modifier of that name.
        e.register(SYSTEM_FIND_MODIFIER, |_, _| Ret::default());
        assert!(!e.call(0x00c3_1530, &args![this, system]).bool());
        e.register(SYSTEM_FIND_MODIFIER, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        // No controller target.
        e.mem.set_u32(this.addr() + 0x2c, 0);
        assert!(!e.call(0x00c3_1530, &args![this, system]).bool());
        e.mem.set_u32(this.addr() + 0x2c, system);
        // The null name.
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_1530, &args![this, system]).bool());
        assert!(calls_to(&e, SYSTEM_FIND_MODIFIER).is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_class_and_modifier_name() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0001);
        e.register(SINGLE_INTERP_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_kModifierName, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00c3_15b0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, SINGLE_INTERP_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0001, LABEL_MODIFIER_NAME]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_MODIFIER_NAME, 0x55]]
        );
    }

    #[test]
    fn interp_target_modifier_is_walks_the_modifier_kind() {
        const KIND: u32 = 0x0120_2100;
        let mut e = Engine::new();
        let modifier = object_of_kind(&mut e, &[0x0120_2200, KIND]);
        e.register(INTERP_TARGET_IS_CORRECT_TYPE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(SYSTEM_FIND_MODIFIER, move |_, _| Ret {
            eax: modifier,
            ..Ret::default()
        });
        assert!(interp_target_modifier_is(&mut e, 0x5000, 0x6000, KIND));
        assert!(!interp_target_modifier_is(
            &mut e,
            0x5000,
            0x6000,
            0x0120_2300
        ));
    }
}
