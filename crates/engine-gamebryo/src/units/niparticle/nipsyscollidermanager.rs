//! `NiParticle/nipsyscollidermanager.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2780.
pub fn ni_p_sys_collider_manager_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2780
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiPSysModifier::LoadBinary` (one word: the stream).
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `00a645b0` (unit `nistream.cpp`, `thiscall` on the stream, no words): reads
/// one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `00c31780` (unnamed; two words: the clone, the cloning process): the
/// modifier part of the member copy.
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
/// `NiPSysModifier::ProcessClone` (one word).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `NiPSysModifier::LinkObject` (one word).
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00c31720` (unnamed, no words): the `NiPSysModifier` part of the constructor.
const MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysModifier::~NiPSysModifier`.
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
/// The `NiPSysColliderManager` vtable.
const COLLIDER_MANAGER_VTABLE: u32 = 0x010c_008c;
/// `m_spColliders` (Xbox PDB), the `NiPointer<NiPSysCollider>` at `+0x18`.
const COLLIDERS: u32 = 0x18;

/// Stores `value` in the `NiPointer` at `slot` when it differs: the old
/// object released (`DeleteThis`, vtable `+4`, at a zero count), the new
/// one's count incremented.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old != value {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 4, &[]);
        }
        e.mem.set_u32(slot, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

// Translated from 00c2cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::LoadBinary` (Xbox PDB): the modifier part, then the
/// colliders' link ID (`00a645b0` on the stream).
pub fn ni_p_sys_collider_manager_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
}

// Translated from 00c2d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the clone, the cloning
/// process; the manager's `CopyMembers` shape): the modifier part
/// (`00c31780`), then, when this manager has colliders, their `CreateClone`
/// (vtable `+0x48`) goes into the clone's `m_spColliders`.
pub fn fn_00c2d3b0(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders == 0 {
        return;
    }
    let copy = e.vcall(colliders, 0x48, &args![process]).u32();
    assign_pointer(e, clone.addr() + COLLIDERS, copy);
}

// Translated from 00c2d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::ProcessClone` (Xbox PDB): the modifier part, then
/// the colliders' `ProcessClone` (vtable `+0x68`) when there are colliders.
pub fn ni_p_sys_collider_manager_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 {
        e.vcall(colliders, 0x68, &args![process]);
    }
}

// Translated from 00c2d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::LinkObject` (Xbox PDB): the modifier part, then the
/// linked object becomes `m_spColliders`.
pub fn ni_p_sys_collider_manager_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + COLLIDERS, object);
}

/// The inlined constructor: `00c31720`, the vtable, no colliders.
fn construct_collider_manager(e: &mut Engine, block: u32) -> Ptr {
    e.call(MODIFIER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, COLLIDER_MANAGER_VTABLE);
    e.mem.set_u32(block + COLLIDERS, 0);
    Ptr::new(block)
}

// Translated from 00c2d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::CreateClone` (Xbox PDB): 0x1C bytes constructed as
/// a manager (null when the allocation fails, still passed on as the exe
/// does), the members copied (`fn_00c2d3b0`). Answers the clone.
pub fn ni_p_sys_collider_manager_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_collider_manager(e, block)
    };
    fn_00c2d3b0(e, this, clone, process);
    clone
}

// Translated from 00c2d5b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysColliderManager` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x1C bytes constructed as a manager, or null.
pub fn fn_00c2d5b0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_collider_manager(e, block)
}

// Translated from 00c2d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::_scalar_deleting_destructor_` (Xbox PDB): releases
/// the colliders (the field itself is left), the modifier destructor, then
/// the sized delete of 0x1C bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_p_sys_collider_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 && e.call(INTERLOCKED_DECREMENT, &args![colliders + 4]).u32() == 0 {
        e.vcall(colliders, 4, &[]);
    }
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

use crate::units::niparticle::nipsysdata::add_viewer_string;

/// `NiPSysModifier`'s `RegisterStreamables` (`00c31670`), `SaveBinary`
/// (`00c31950`), `IsEqual` (`00c31830`) and `GetViewerStrings` (`00c319e0`)
/// (Xbox PDB names; `thiscall`, one word).
const MODIFIER_REGISTER_STREAMABLES: u32 = 0x00c3_1670;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `NiPSysCollider`'s `Resolve` (`+0x8C`: start time, address of the
/// collision time, particle data, index; the collider hit or null),
/// `Update` (`+0x90`: time, particle data, index) and
/// `SetResolveConstants` (`+0x94`, no words) slots (Xbox PDB names).
const COLLIDER_RESOLVE_SLOT: u32 = 0x8c;
const COLLIDER_UPDATE_SLOT: u32 = 0x90;
const COLLIDER_SET_RESOLVE_CONSTANTS_SLOT: u32 = 0x94;
/// `NiObject`'s `RegisterStreamables` (`+0x54`), `IsEqual` (`+0x5C`) and
/// `GetViewerStrings` (`+0x60`) slots and `NiStream::SaveLinkID` (`+0x30`)
/// (Xbox PDB).
const REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const IS_EQUAL_SLOT: u32 = 0x5c;
const GET_VIEWER_STRINGS_SLOT: u32 = 0x60;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// `NiPSysModifier::m_pkTarget` (Xbox PDB, +0x10), and the float the update
/// starts each particle's resolve at: +0xD4 of that particle system.
const TARGET: u32 = 0x10;
const SYSTEM_START_TIME: u32 = 0xd4;
/// The particle data (`NiPSysData`) fields the update reads: the active
/// count (+0x50) and the particle infos (+0x70, 0x1c bytes each; the last
/// update time at +0x14).
const DATA_ACTIVE: u32 = 0x50;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;
/// `NiGetViewerString` (`00a7d330`, Xbox PDB name; `cdecl`, the text).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// `NiPSysColliderManager::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2780;

// Translated from 00c2d300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::Update` (Xbox PDB): with colliders and active
/// particles, the colliders' `SetResolveConstants` first. Then for each
/// active particle: the collision time starts as `time`; with colliders,
/// their `Resolve` from the system's float at +0xD4 (which may write the
/// collision time) and, when it answers a collider, that collider's
/// `Update` at `time`; otherwise (no colliders or no hit) the collision
/// time is the system's float. The particle's last update time becomes
/// the collision time.
pub fn ni_p_sys_collider_manager_update(e: &mut Engine, this: Ptr, time: f32, data: Ptr) {
    let data = data.addr();
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 && e.mem.u16(data + DATA_ACTIVE) != 0 {
        e.vcall(colliders, COLLIDER_SET_RESOLVE_CONSTANTS_SLOT, &args![]);
    }
    let time = x87_quiet_bits(time.to_bits());
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let mut index: u16 = 0;
        while index < e.mem.u16(data + DATA_ACTIVE) {
            let system = e.mem.u32(this.addr() + TARGET);
            let start = x87_quiet_bits(e.mem.u32(system + SYSTEM_START_TIME));
            let colliders = e.mem.u32(this.addr() + COLLIDERS);
            e.mem.set_u32(slot, time);
            let mut hit = 0;
            if colliders != 0 {
                hit = e
                    .vcall(
                        colliders,
                        COLLIDER_RESOLVE_SLOT,
                        &args![start, slot, data, u32::from(index)],
                    )
                    .u32();
                if hit != 0 {
                    e.vcall(
                        hit,
                        COLLIDER_UPDATE_SLOT,
                        &args![time, data, u32::from(index)],
                    );
                }
            }
            if hit == 0 {
                e.mem.set_u32(slot, start);
            }
            let info = e
                .mem
                .u32(data + DATA_INFOS)
                .wrapping_add(u32::from(index) * INFO_SIZE);
            let collision = x87_quiet_bits(e.mem.u32(slot));
            e.mem.set_u32(info + INFO_LAST_UPDATE, collision);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c2d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::RegisterStreamables` (Xbox PDB):
/// `NiPSysModifier`'s; when that succeeds the colliders' (slot `+0x54`,
/// its answer not read) and true.
pub fn ni_p_sys_collider_manager_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    if !e
        .call(MODIFIER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 {
        e.vcall(colliders, REGISTER_STREAMABLES_SLOT, &args![stream]);
    }
    true
}

// Translated from 00c2d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::SaveBinary` (Xbox PDB): `NiPSysModifier`'s, then
/// the colliders' link id (`NiStream::SaveLinkID`).
pub fn ni_p_sys_collider_manager_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![colliders]);
}

// Translated from 00c2d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::IsEqual` (Xbox PDB): `NiPSysModifier`'s; both or
/// neither must have colliders, and two must be equal (this one's
/// `IsEqual`, slot `+0x5C`, given the other's).
pub fn ni_p_sys_collider_manager_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let ours = e.mem.u32(this.addr() + COLLIDERS);
    let theirs = e.mem.u32(other.addr() + COLLIDERS);
    if ours != 0 {
        if theirs == 0 {
            return false;
        }
        return e.vcall(ours, IS_EQUAL_SLOT, &args![theirs]).bool();
    }
    theirs == 0
}

// Translated from 00c2d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::GetViewerStrings` (Xbox PDB): `NiPSysModifier`'s
/// strings, the class name, then the colliders' strings (slot `+0x60`).
pub fn ni_p_sys_collider_manager_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings.addr(), string);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    e.vcall(colliders, GET_VIEWER_STRINGS_SLOT, &args![strings]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2d540, ni_p_sys_collider_manager_get_r_t_t_i() -> u32),
        entry!(0x00c2_cf90, ni_p_sys_collider_manager_load_binary(Ptr, Ptr)),
        entry!(0x00c2_d3b0, fn_00c2d3b0(Ptr, Ptr, Ptr)),
        entry!(
            0x00c2_d410,
            ni_p_sys_collider_manager_process_clone(Ptr, Ptr)
        ),
        entry!(0x00c2_d440, ni_p_sys_collider_manager_link_object(Ptr, Ptr)),
        entry!(0x00c2_d550, ni_p_sys_collider_manager_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_d5b0, fn_00c2d5b0() -> Ptr),
        entry!(
            0x00c2_d5e0,
            ni_p_sys_collider_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_d300, ni_p_sys_collider_manager_update(Ptr, f32, Ptr)),
        entry!(
            0x00c2_d4a0,
            ni_p_sys_collider_manager_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00c2_d4d0, ni_p_sys_collider_manager_save_binary(Ptr, Ptr)),
        entry!(0x00c2_d4f0, ni_p_sys_collider_manager_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_d630,
            ni_p_sys_collider_manager_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_collider_manager_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d540, &args![]).u32(), 0x0120_2780);
    }
}

#[cfg(test)]
mod manager_tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn with_refcounts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
    }

    const DELETE_THIS: u32 = 0x0100_0004;
    const CREATE_CLONE: u32 = 0x0100_0048;
    const PROCESS_CLONE: u32 = 0x0100_0068;

    fn collider(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.mem.set_u32(vtable + 0x48, CREATE_CLONE);
        e.mem.set_u32(vtable + 0x68, PROCESS_CLONE);
        let object = e.mem.alloc(0x2c);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn load_binary_reads_the_modifier_and_one_link_id() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c2_cf90, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_LOAD_BINARY),
            vec![vec![0x5000, 0x6000]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![0x6000]]);
    }

    #[test]
    fn copy_members_clones_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let copy = collider(&mut e, 0);
        e.register_double(CREATE_CLONE, move |_, _| word(copy));
        let this = e.mem.alloc(0x1c);
        let clone = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        // Without colliders only the modifier part runs.
        e.call(0x00c2_d3b0, &args![this, clone, 0x7000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert!(calls_to(&e, CREATE_CLONE).is_empty());
        let own = collider(&mut e, 1);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call(0x00c2_d3b0, &args![this, clone, 0x7000u32]);
        assert_eq!(calls_to(&e, CREATE_CLONE), vec![vec![own, 0x7000]]);
        assert_eq!(e.mem.u32(clone + COLLIDERS), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
    }

    #[test]
    fn process_clone_forwards_to_the_colliders() {
        let mut e = Engine::new();
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d410, &args![this, 0x7000u32]);
        assert!(calls_to(&e, PROCESS_CLONE).is_empty());
        let own = collider(&mut e, 1);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call(0x00c2_d410, &args![this, 0x7000u32]);
        assert_eq!(calls_to(&e, PROCESS_CLONE), vec![vec![own, 0x7000]]);
        assert_eq!(calls_to(&e, MODIFIER_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn link_object_replaces_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(DELETE_THIS, |_, _| Ret::default());
        let linked = collider(&mut e, 0);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| word(linked));
        let old = collider(&mut e, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + COLLIDERS, old);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d440, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + COLLIDERS), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
    }

    #[test]
    fn create_clone_and_create_function() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MODIFIER_CONSTRUCT, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_d550, &args![this, 0x7000u32]).u32();
        assert_eq!(e.mem.u32(clone), COLLIDER_MANAGER_VTABLE);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        let made = e.call(0x00c2_d5b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), COLLIDER_MANAGER_VTABLE);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x1c], vec![0x1c]]);
        // A failed allocation: the copy still runs, on a null clone.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_d550, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS)[1],
            vec![this, 0, 0x7000]
        );
        assert_eq!(e.call(0x00c2_d5b0, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let own = collider(&mut e, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_d5e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![own]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0x1c]]);
        e.mem.set_u32(this + COLLIDERS, 0);
        e.call(0x00c2_d5e0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};

    const SET_CONSTANTS: u32 = 0x0000_d094;
    const RESOLVE: u32 = 0x0000_d08c;
    const UPDATE: u32 = 0x0000_d090;
    const HIT_UPDATE: u32 = 0x0000_d190;

    /// A manager whose system's start time is 0.5 and two particles; the
    /// colliders' `Resolve` writes 0.75 as the collision time and answers
    /// the collider in the word at `0f900000` for particle 0 only.
    fn scene(e: &mut Engine) -> (Ptr, u32, u32, u32, u32) {
        let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
        let system = e.mem.alloc(0x100);
        e.mem.set_f32(system + SYSTEM_START_TIME, 0.5);
        e.mem.set_u32(this.addr() + TARGET, system);
        let colliders =
            viewer_test::object(e, &[(0x94, SET_CONSTANTS), (0x8c, RESOLVE), (0x90, UPDATE)]);
        let hit = viewer_test::object(e, &[(0x90, HIT_UPDATE)]);
        e.map(0x0f90_0000, 4);
        e.mem.set_u32(0x0f90_0000, hit);
        quiet(e, &[SET_CONSTANTS, UPDATE, HIT_UPDATE]);
        e.register(RESOLVE, |e, a| {
            e.mem.set_f32(a[2], 0.75);
            answer(if a[4] == 0 { e.mem.u32(0x0f90_0000) } else { 0 })
        });
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + DATA_ACTIVE, 2);
        let infos = e.mem.alloc(2 * INFO_SIZE);
        e.mem.set_u32(data + DATA_INFOS, infos);
        (this, colliders, hit, data, infos)
    }

    #[test]
    fn update_resolves_each_particle_through_the_colliders() {
        let mut e = Engine::new();
        let (this, colliders, hit, data, infos) = scene(&mut e);
        e.mem.set_u32(this.addr() + COLLIDERS, colliders);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d300, &args![this, 2.0f32, data]);
        assert_eq!(calls_to(&e, SET_CONSTANTS), vec![vec![colliders]]);
        let resolves = calls_to(&e, RESOLVE);
        assert_eq!(resolves.len(), 2);
        assert_eq!(resolves[0][1], 0.5f32.to_bits());
        assert_eq!((resolves[0][3], resolves[0][4]), (data, 0));
        assert_eq!((resolves[1][3], resolves[1][4]), (data, 1));
        assert_eq!(
            calls_to(&e, HIT_UPDATE),
            vec![vec![hit, 2.0f32.to_bits(), data, 0]]
        );
        // A hit keeps the resolved time; no hit takes the start time.
        assert_eq!(e.mem.f32(infos + INFO_LAST_UPDATE), 0.75);
        assert_eq!(e.mem.f32(infos + INFO_SIZE + INFO_LAST_UPDATE), 0.5);
    }

    #[test]
    fn update_without_colliders_takes_the_start_time() {
        let mut e = Engine::new();
        let (this, _, _, data, infos) = scene(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d300, &args![this, 2.0f32, data]);
        assert!(calls_to(&e, SET_CONSTANTS).is_empty());
        assert_eq!(e.mem.f32(infos + INFO_LAST_UPDATE), 0.5);
        assert_eq!(e.mem.f32(infos + INFO_SIZE + INFO_LAST_UPDATE), 0.5);
        // No particles: no resolve constants either.
        let (this, colliders, _, data, _) = scene(&mut e);
        e.mem.set_u32(this.addr() + COLLIDERS, colliders);
        e.mem.set_u16(data + DATA_ACTIVE, 0);
        e.call(0x00c2_d300, &args![this, 2.0f32, data]);
        assert!(calls_to(&e, SET_CONSTANTS).is_empty());
    }

    #[test]
    fn register_streamables_adds_the_colliders() {
        const REGISTER: u32 = 0x0000_d054;
        let mut e = Engine::new();
        e.register(MODIFIER_REGISTER_STREAMABLES, |_, _| answer(0));
        quiet(&mut e, &[REGISTER]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
        let colliders = viewer_test::object(&mut e, &[(0x54, REGISTER)]);
        e.mem.set_u32(this.addr() + COLLIDERS, colliders);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c2_d4a0, &args![this, 0x500u32]).bool());
        assert!(calls_to(&e, REGISTER).is_empty());
        e.register(MODIFIER_REGISTER_STREAMABLES, |_, _| answer(1));
        assert!(e.call(0x00c2_d4a0, &args![this, 0x500u32]).bool());
        assert_eq!(calls_to(&e, REGISTER), vec![vec![colliders, 0x500]]);
        e.mem.set_u32(this.addr() + COLLIDERS, 0);
        assert!(e.call(0x00c2_d4a0, &args![this, 0x500u32]).bool());
    }

    #[test]
    fn save_binary_saves_the_colliders_link() {
        const SAVE_LINK: u32 = 0x0000_d130;
        let mut e = Engine::new();
        quiet(&mut e, &[MODIFIER_SAVE_BINARY, SAVE_LINK]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
        e.mem.set_u32(this.addr() + COLLIDERS, 0x777);
        let stream = viewer_test::object(&mut e, &[(0x30, SAVE_LINK)]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d4d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x777]]);
    }

    #[test]
    fn is_equal_compares_the_colliders() {
        const EQUAL: u32 = 0x0000_d05c;
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(1));
        e.register(EQUAL, |_, a| answer(u32::from(a[1] == 0x888)));
        let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
        let other: Ptr = Ptr::new(e.mem.alloc(0x1c));
        let check = |e: &mut Engine| e.call(0x00c2_d4f0, &args![this, other]).bool();
        assert!(check(&mut e));
        e.mem.set_u32(other.addr() + COLLIDERS, 0x888);
        assert!(!check(&mut e));
        let ours = viewer_test::object(&mut e, &[(0x5c, EQUAL)]);
        e.mem.set_u32(this.addr() + COLLIDERS, ours);
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(calls_to(&e, EQUAL), vec![vec![ours, 0x888]]);
        e.mem.set_u32(other.addr() + COLLIDERS, 0x889);
        assert!(!check(&mut e));
        e.mem.set_u32(other.addr() + COLLIDERS, 0);
        assert!(!check(&mut e));
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(0));
        e.mem.set_u32(this.addr() + COLLIDERS, 0);
        assert!(!check(&mut e));
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_colliders() {
        const STRINGS: u32 = 0x0000_d060;
        let mut e = Engine::new();
        quiet(&mut e, &[MODIFIER_GET_VIEWER_STRINGS, STRINGS]);
        viewer_test::counting(&mut e, &[VIEWER_STRING]);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_00e0);
        let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
        let colliders = viewer_test::object(&mut e, &[(0x60, STRINGS)]);
        e.mem.set_u32(this.addr() + COLLIDERS, colliders);
        let strings = viewer_test::strings(&mut e, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d630, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1]);
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_00e0]]);
        assert_eq!(calls_to(&e, STRINGS), vec![vec![colliders, strings]]);
    }
}
