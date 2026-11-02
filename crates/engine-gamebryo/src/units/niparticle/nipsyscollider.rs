//! `NiParticle/nipsyscollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c327e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27e8.
pub fn ni_p_sys_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27e8
}

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a5d450` (unit `niobject.cpp`; two words: the clone, the cloning
/// process): the `NiObject` part of the member copy.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::ProcessClone` (one word).
const OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `00a89af0` (unnamed, one word, a bare `RET 4`): the `NiObject` link step.
const OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiObject::LoadBinary` (one word).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a5d3a0` (unit `niobject.cpp`, no words): the `NiObject` constructor.
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `00a53c30` (unnamed, on the cloning process's map; two words: the key, the
/// result slot): the clone of an object, `bool` in `AL`.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (unit `nistream.cpp`, no words): reads one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// The `NiPSysCollider` vtable.
const COLLIDER_VTABLE: u32 = 0x010c_0c4c;
/// The vector the constructor copies into the collision point (`011f426c`).
const ORIGIN: u32 = 0x011f_426c;

layout! {
    /// `NiPSysCollider` (Xbox PDB), 0x2C bytes.
    pub struct NiPSysCollider: 0x2C {
        /// `m_fBounce` (Xbox PDB), as bits.
        0x08 m_fBounce: u32,
        /// `m_bSpawnOnCollide` (Xbox PDB).
        0x0C m_bSpawnOnCollide: u8,
        /// `m_bDieOnCollide` (Xbox PDB).
        0x0D m_bDieOnCollide: u8,
        /// `m_pkSpawnModifier` (Xbox PDB).
        0x10 m_pkSpawnModifier: u32,
        /// `m_kCollisionPoint` (Xbox PDB), three floats.
        0x14 m_kCollisionPoint: u32,
        /// `m_fCollisionTime` (Xbox PDB), as bits.
        0x20 m_fCollisionTime: u32,
        /// `m_pkManager` (Xbox PDB).
        0x24 m_pkManager: u32,
        /// `m_spNext` (Xbox PDB): `NiPointer<NiPSysCollider>`.
        0x28 m_spNext: u32,
    }
}

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

// Translated from 00c32510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the clone, the cloning
/// process; the `CopyMembers` shape): the `NiObject` part (`00a5d450`), the
/// bounce (`FLD`/`FSTP`, quieting a signalling NaN) and the two flags, then,
/// when there is a next collider, its `CreateClone` (vtable `+0x48`) goes into
/// the clone's `m_spNext`.
pub fn fn_00c32510(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    clone: Ptr<NiPSysCollider>,
    process: Ptr,
) {
    e.call(OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    let bounce = x87_quiet_bits(e.get(this, NiPSysCollider::m_fBounce));
    e.set(clone, NiPSysCollider::m_fBounce, bounce);
    let spawn = e.get(this, NiPSysCollider::m_bSpawnOnCollide);
    e.set(clone, NiPSysCollider::m_bSpawnOnCollide, spawn);
    let die = e.get(this, NiPSysCollider::m_bDieOnCollide);
    e.set(clone, NiPSysCollider::m_bDieOnCollide, die);
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next == 0 {
        return;
    }
    let copy = e.vcall(next, 0x48, &args![process]).u32();
    assign_pointer(e, clone.addr() + 0x28, copy);
}

// Translated from 00c32580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::ProcessClone` (Xbox PDB): the `NiObject` part; then this
/// collider's clone is looked up in the process's map (`00a53c30`, the result
/// slot is the argument slot, set to 0 first) and its spawn modifier and
/// manager become their clones when the map has them, else stay; then the
/// next collider's `ProcessClone` (vtable `+0x68`).
pub fn ni_p_sys_collider_process_clone(e: &mut Engine, this: Ptr<NiPSysCollider>, process: Ptr) {
    e.call(OBJECT_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = e.mem.u32(slot.addr());
        for offset in [0x10u32, 0x24] {
            let object = e.mem.u32(this.addr() + offset);
            if object != 0 {
                let map = e.mem.u32(process.addr());
                let value = if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
                    e.mem.u32(slot.addr())
                } else {
                    e.mem.u32(this.addr() + offset)
                };
                e.mem.set_u32(clone.wrapping_add(offset), value);
            }
        }
    });
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next != 0 {
        e.vcall(next, 0x68, &args![process]);
    }
}

// Translated from 00c32610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::LinkObject` (Xbox PDB): the `NiObject` link step
/// (`00a89af0`), then three linked objects: the spawn modifier, the manager
/// and the next collider (a `NiPointer`).
pub fn ni_p_sys_collider_link_object(e: &mut Engine, this: Ptr<NiPSysCollider>, stream: Ptr) {
    e.call(OBJECT_LINK_OBJECT, &args![this, stream]);
    let spawn = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysCollider::m_pkSpawnModifier, spawn);
    let manager = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysCollider::m_pkManager, manager);
    let next = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x28, next);
}

// Translated from 00c32780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::NiPSysCollider` (Xbox PDB; `thiscall`, four words: the
/// bounce, spawn-on-collide, die-on-collide, the spawn modifier): the
/// `NiObject` constructor, the fields (the bounce through `FLD`/`FSTP`), the
/// vtable, the collision point from `011f426c`, time 0, no manager, no next.
pub fn ni_p_sys_collider_ni_p_sys_collider(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    bounce: f32,
    spawn_on_collide: u8,
    die_on_collide: u8,
    spawn_modifier: u32,
) -> Ptr<NiPSysCollider> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.set(
        this,
        NiPSysCollider::m_fBounce,
        x87_quiet_bits(bounce.to_bits()),
    );
    e.set(this, NiPSysCollider::m_bSpawnOnCollide, spawn_on_collide);
    e.mem.set_u32(this.addr(), COLLIDER_VTABLE);
    e.set(this, NiPSysCollider::m_bDieOnCollide, die_on_collide);
    e.set(this, NiPSysCollider::m_pkSpawnModifier, spawn_modifier);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(this.addr() + 0x14 + 4 * i, word);
    }
    e.set(this, NiPSysCollider::m_fCollisionTime, 0);
    e.set(this, NiPSysCollider::m_pkManager, 0);
    e.set(this, NiPSysCollider::m_spNext, 0);
    this
}

// Translated from 00c32840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::LoadBinary` (Xbox PDB): the `NiObject` part, the bounce
/// (four bytes), then two one-byte booleans (the first read into the
/// argument slot, whose low byte holds the stream's until the read, the
/// second into a byte of the frame) stored as `byte != 0`, then three link
/// IDs (`00a645b0`). Each read goes through the stream's input file (`+0x24c`,
/// its read function `+8`, `cdecl (file, dest, size, &count, 1)`, the result
/// added to its position `+4`); the count word is a local.
pub fn ni_p_sys_collider_load_binary(e: &mut Engine, this: Ptr<NiPSysCollider>, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        // frame + 0: the argument slot; + 4: the frame's byte (at +7) and
        // + 8: the count word, as the exe lays them out.
        let f = frame.addr();
        e.mem.set_u32(f, stream.addr());
        let read = |e: &mut Engine, dest: u32, size: u32| {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let function = e.mem.u32(file + 8);
            e.mem.set_u32(f + 8, size);
            let got = e
                .call(function, &args![file, dest, size, f + 8, 1u32])
                .u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(got));
        };
        read(e, this.addr() + 8, 4);
        read(e, f, 1);
        let spawn = u8::from(e.mem.u8(f) != 0);
        e.set(this, NiPSysCollider::m_bSpawnOnCollide, spawn);
        read(e, f + 7, 1);
        let die = u8::from(e.mem.u8(f + 7) != 0);
        e.set(this, NiPSysCollider::m_bDieOnCollide, die);
    });
    for _ in 0..3 {
        e.call(READ_LINK_ID, &args![stream]);
    }
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};

/// `NiObject`'s `RegisterStreamables` (`00a5d2f0`), `IsEqual` (`00a5d470`)
/// and `GetViewerStrings` (`00a5d6e0`) (Xbox PDB names; `thiscall`, one
/// word), its destructor body (`00a5d3d0`), and `00a89af0` (unnamed, one
/// word: what `SaveBinary` runs first).
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysCollider`'s `Resolve` slot (`+0x8C`, Xbox PDB: start time,
/// address of the collision time, particle data, index), the spawn
/// modifier's slot `+0xA4` (time, collision time, index, particle system)
/// and the particle data's slot `+0xA4` (index), and `NiObject`'s
/// `RegisterStreamables` (`+0x54`), `IsEqual` (`+0x5C`) and
/// `GetViewerStrings` (`+0x60`), `NiStream::SaveLinkID` (`+0x30`) and
/// `NiRefObject::DeleteThis` (`+4`) slots (Xbox PDB).
const RESOLVE_SLOT: u32 = 0x8c;
const SPAWN_SLOT: u32 = 0xa4;
const DATA_REMOVE_SLOT: u32 = 0xa4;
const REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const IS_EQUAL_SLOT: u32 = 0x5c;
const GET_VIEWER_STRINGS_SLOT: u32 = 0x60;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
const DELETE_THIS_SLOT: u32 = 0x4;
/// The particle data's positions (+0x20, 12 bytes each), the manager's
/// particle system (`NiPSysModifier::m_pkTarget`, +0x10) and an object's
/// `NiObjectNET` name (+8).
const DATA_VERTICES: u32 = 0x20;
const MANAGER_TARGET: u32 = 0x10;
const OBJECT_NAME: u32 = 0x8;
/// `NiGetViewerString` (`00a7d330`, the text), `_ov2` (`00a7d370`: a label
/// and a `bool`), `_ov9` (`00a7d550`: a label and a `float`), `_ov10`
/// (`00a7d5a0`: a label and a pointer) and `_ov11` (`00a7d5e0`: a label and
/// a text) (Xbox PDB names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `NiPSysCollider::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_27e8;
/// The labels "Bounce" (`010c0d10`), "Spawn on Collide" (`010c0cfc`), "Die
/// on Collide" (`010c0cec`), "Spawn Modifier" (`010c03a8`) and "Manager"
/// (`010c0ce4`), and "None" (`0102909c`) for no spawn modifier.
const LABEL_BOUNCE: u32 = 0x010c_0d10;
const LABEL_SPAWN_ON_COLLIDE: u32 = 0x010c_0cfc;
const LABEL_DIE_ON_COLLIDE: u32 = 0x010c_0cec;
const LABEL_SPAWN_MODIFIER: u32 = 0x010c_03a8;
const LABEL_MANAGER: u32 = 0x010c_0ce4;
const TEXT_NONE: u32 = 0x0102_909c;

/// Whether two optional objects match as the comparisons inline it: both
/// null, or both set and `ours`' `IsEqual` (slot `+0x5C`) given `theirs`.
fn same_object(e: &mut Engine, ours: u32, theirs: u32) -> bool {
    if ours != 0 {
        if theirs == 0 {
            return false;
        }
        return e.vcall(ours, IS_EQUAL_SLOT, &args![theirs]).bool();
    }
    theirs == 0
}

// Translated from 00c32450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::Resolve` (engine map name): the next collider's
/// `Resolve` (slot `+0x8C`, the same words) when there is one, its answer
/// (a collider or null); null otherwise.
pub fn ni_p_sys_collider_resolve(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    start: f32,
    collision_time: u32,
    data: u32,
    index: u32,
) -> u32 {
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next == 0 {
        return 0;
    }
    let start = x87_quiet_bits(start.to_bits());
    e.vcall(
        next,
        RESOLVE_SLOT,
        &args![start, collision_time, data, index],
    )
    .u32()
}

// Translated from 00c32490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::Update` (Xbox PDB): the particle's position becomes the
/// collision point; with spawn-on-collide and a spawn modifier, its slot
/// `+0xA4` (time, collision time, index, the manager's particle system);
/// with die-on-collide, the particle data's slot `+0xA4` (index).
pub fn ni_p_sys_collider_update(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    time: f32,
    data: u32,
    index: u32,
) {
    type C = NiPSysCollider;
    let vertex = e
        .mem
        .u32(data + DATA_VERTICES)
        .wrapping_add((index & 0xffff) * 12);
    let point = this.addr() + C::m_kCollisionPoint.off;
    for word in 0..3 {
        let value = e.mem.u32(point + 4 * word);
        e.mem.set_u32(vertex + 4 * word, value);
    }
    let spawn = e.get(this, C::m_pkSpawnModifier);
    if e.get(this, C::m_bSpawnOnCollide) != 0 && spawn != 0 {
        let manager = e.get(this, C::m_pkManager);
        let collision_time = x87_quiet_bits(e.get(this, C::m_fCollisionTime));
        let system = e.mem.u32(manager + MANAGER_TARGET);
        let time = x87_quiet_bits(time.to_bits());
        e.vcall(
            spawn,
            SPAWN_SLOT,
            &args![time, collision_time, index, system],
        );
    }
    if e.get(this, C::m_bDieOnCollide) != 0 {
        e.vcall(data, DATA_REMOVE_SLOT, &args![index]);
    }
}

// Translated from 00c32680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::RegisterStreamables` (Xbox PDB): `NiObject`'s; when that
/// succeeds the next collider's (slot `+0x54`, its answer not read) and
/// true.
pub fn ni_p_sys_collider_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    stream: Ptr,
) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next != 0 {
        e.vcall(next, REGISTER_STREAMABLES_SLOT, &args![stream]);
    }
    true
}

// Translated from 00c326b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::IsEqual` (Xbox PDB): `NiObject`'s, equal bounce
/// (`FUCOMPP`: a NaN is unequal), the same two flags, matching spawn
/// modifiers (both null, or equal by this one's `IsEqual`), both or neither
/// with a manager (not compared further), and matching next colliders.
pub fn ni_p_sys_collider_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    other: Ptr<NiPSysCollider>,
) -> bool {
    type C = NiPSysCollider;
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if x87_m32(e.get(other, C::m_fBounce)) != x87_m32(e.get(this, C::m_fBounce)) {
        return false;
    }
    if e.get(this, C::m_bSpawnOnCollide) != e.get(other, C::m_bSpawnOnCollide)
        || e.get(this, C::m_bDieOnCollide) != e.get(other, C::m_bDieOnCollide)
    {
        return false;
    }
    let ours = e.get(this, C::m_pkSpawnModifier);
    let theirs = e.get(other, C::m_pkSpawnModifier);
    if !same_object(e, ours, theirs) {
        return false;
    }
    if (e.get(this, C::m_pkManager) == 0) != (e.get(other, C::m_pkManager) == 0) {
        return false;
    }
    let ours = e.get(this, C::m_spNext);
    let theirs = e.get(other, C::m_spNext);
    same_object(e, ours, theirs)
}

// Translated from 00c327f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::_scalar_deleting_destructor_` (Xbox PDB): the next
/// collider released (`DeleteThis`, slot `+4`, when its count reaches 0),
/// `NiObject`'s destructor body, then the sized delete of 0x2C bytes when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_p_sys_collider_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    flags: u32,
) -> Ptr<NiPSysCollider> {
    let next = e.get(this, NiPSysCollider::m_spNext);
    if next != 0 && e.call(INTERLOCKED_DECREMENT, &args![next + 4]).u32() == 0 {
        e.vcall(next, DELETE_THIS_SLOT, &args![]);
    }
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x2cu32]);
    }
    this
}

// Translated from 00c32900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::SaveBinary` (Xbox PDB): `00a89af0` first, then the
/// bounce (4 bytes), the two flags (a byte each), and the link ids of the
/// spawn modifier, the manager and the next collider
/// (`NiStream::SaveLinkID`).
pub fn ni_p_sys_collider_save_binary(e: &mut Engine, this: Ptr<NiPSysCollider>, stream: Ptr) {
    type C = NiPSysCollider;
    let stream = stream.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, locals| {
        let (byte, slot) = (locals.addr(), locals.addr() + 4);
        let bounce = this.addr() + C::m_fBounce.off;
        stream_write(e, stream, bounce, 4, 4, slot);
        e.mem.set_u8(byte, e.get(this, C::m_bSpawnOnCollide));
        stream_write(e, stream, byte, 1, 1, slot);
        e.mem.set_u8(byte, e.get(this, C::m_bDieOnCollide));
        stream_write(e, stream, byte, 1, 1, slot);
    });
    for field in [C::m_pkSpawnModifier, C::m_pkManager, C::m_spNext] {
        let link = e.get(this, field);
        e.vcall(stream, STREAM_SAVE_LINK_ID_SLOT, &args![link]);
    }
}

// Translated from 00c329c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCollider::GetViewerStrings` (Xbox PDB): `NiObject`'s strings, the
/// class name, the bounce, the two flags, the spawn modifier's name (its
/// `NiObjectNET` name, or "None") and the manager pointer, then the next
/// collider's strings (slot `+0x60`) when there is one.
pub fn ni_p_sys_collider_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysCollider>,
    strings: Ptr,
) {
    type C = NiPSysCollider;
    let strings = strings.addr();
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let bounce = x87_quiet_bits(e.get(this, C::m_fBounce));
    let string = e
        .call(VIEWER_STRING_FLOAT, &args![LABEL_BOUNCE, bounce])
        .u32();
    add_viewer_string(e, strings, string);
    for (label, field) in [
        (LABEL_SPAWN_ON_COLLIDE, C::m_bSpawnOnCollide),
        (LABEL_DIE_ON_COLLIDE, C::m_bDieOnCollide),
    ] {
        let flag = u32::from(e.get(this, field));
        let string = e.call(VIEWER_STRING_BOOL, &args![label, flag]).u32();
        add_viewer_string(e, strings, string);
    }
    let spawn = e.get(this, C::m_pkSpawnModifier);
    let text = if spawn != 0 {
        e.mem.u32(spawn + OBJECT_NAME)
    } else {
        TEXT_NONE
    };
    let string = e
        .call(VIEWER_STRING_TEXT, &args![LABEL_SPAWN_MODIFIER, text])
        .u32();
    add_viewer_string(e, strings, string);
    let manager = e.get(this, C::m_pkManager);
    let string = e
        .call(VIEWER_STRING_POINTER, &args![LABEL_MANAGER, manager])
        .u32();
    add_viewer_string(e, strings, string);
    let next = e.get(this, C::m_spNext);
    if next != 0 {
        e.vcall(next, GET_VIEWER_STRINGS_SLOT, &args![strings]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c327e0, ni_p_sys_collider_get_r_t_t_i() -> u32),
        entry!(
            0x00c3_2510,
            fn_00c32510(Ptr<NiPSysCollider>, Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2580,
            ni_p_sys_collider_process_clone(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2610,
            ni_p_sys_collider_link_object(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2780,
            ni_p_sys_collider_ni_p_sys_collider(
                Ptr<NiPSysCollider>,
                f32,
                u8,
                u8,
                u32,
            ) -> Ptr<NiPSysCollider>
        ),
        entry!(
            0x00c3_2840,
            ni_p_sys_collider_load_binary(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_2450,
            ni_p_sys_collider_resolve(Ptr<NiPSysCollider>, f32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00c3_2490,
            ni_p_sys_collider_update(Ptr<NiPSysCollider>, f32, u32, u32)
        ),
        entry!(
            0x00c3_2680,
            ni_p_sys_collider_register_streamables(Ptr<NiPSysCollider>, Ptr) -> bool
        ),
        entry!(
            0x00c3_26b0,
            ni_p_sys_collider_is_equal(Ptr<NiPSysCollider>, Ptr<NiPSysCollider>) -> bool
        ),
        entry!(
            0x00c3_27f0,
            ni_p_sys_collider_scalar_deleting_destructor(
                Ptr<NiPSysCollider>,
                u32,
            ) -> Ptr<NiPSysCollider>
        ),
        entry!(
            0x00c3_2900,
            ni_p_sys_collider_save_binary(Ptr<NiPSysCollider>, Ptr)
        ),
        entry!(
            0x00c3_29c0,
            ni_p_sys_collider_get_viewer_strings(Ptr<NiPSysCollider>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_27e0, &args![]).u32(), 0x0120_27e8);
    }
}

#[cfg(test)]
mod collider_tests {
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

    fn collider(e: &mut Engine, count: u32) -> Ptr<NiPSysCollider> {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.mem.set_u32(vtable + 0x48, CREATE_CLONE);
        e.mem.set_u32(vtable + 0x68, PROCESS_CLONE);
        let object: Ptr<NiPSysCollider> = e.new_object();
        e.mem.set_u32(object.addr(), vtable);
        e.mem.set_u32(object.addr() + 4, count);
        object
    }

    #[test]
    fn copy_members_copies_the_fields_and_clones_the_next() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = collider(&mut e, 1);
        let clone = collider(&mut e, 1);
        let copy = collider(&mut e, 0);
        e.register_double(CREATE_CLONE, move |_, _| word(copy.addr()));
        e.set(this, NiPSysCollider::m_fBounce, 0xff80_0001);
        e.set(this, NiPSysCollider::m_bSpawnOnCollide, 1);
        e.set(this, NiPSysCollider::m_bDieOnCollide, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2510, &args![this, clone, 0x7000u32]);
        assert_eq!(e.get(clone, NiPSysCollider::m_fBounce), 0xffc0_0001);
        assert_eq!(e.get(clone, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(clone, NiPSysCollider::m_bDieOnCollide), 1);
        assert!(calls_to(&e, CREATE_CLONE).is_empty());
        let next = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_spNext, next.addr());
        e.call(0x00c3_2510, &args![this, clone, 0x7000u32]);
        assert_eq!(calls_to(&e, CREATE_CLONE), vec![vec![next.addr(), 0x7000]]);
        assert_eq!(e.get(clone, NiPSysCollider::m_spNext), copy.addr());
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS)[0],
            vec![this.addr(), clone.addr(), 0x7000]
        );
    }

    #[test]
    fn process_clone_maps_the_spawn_modifier_and_manager() {
        let mut e = Engine::new();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(PROCESS_CLONE, |_, _| Ret::default());
        let this = collider(&mut e, 1);
        let clone = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_pkSpawnModifier, 0x4440);
        e.set(this, NiPSysCollider::m_pkManager, 0x5550);
        let (key, value) = (this.addr(), clone.addr());
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = match a[1] {
                k if k == key => value,
                0x4440 => 0x4441,
                _ => return word(0),
            };
            e.mem.set_u32(a[2], found);
            word(1)
        });
        let process = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2580, &args![this, process]);
        assert_eq!(e.get(clone, NiPSysCollider::m_pkSpawnModifier), 0x4441);
        assert_eq!(e.get(clone, NiPSysCollider::m_pkManager), 0x5550);
        assert!(calls_to(&e, PROCESS_CLONE).is_empty());
        let next = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_spNext, next.addr());
        e.set(this, NiPSysCollider::m_pkManager, 0);
        e.call(0x00c3_2580, &args![this, process]);
        assert_eq!(
            calls_to(&e, PROCESS_CLONE),
            vec![vec![next.addr(), process]]
        );
        assert_eq!(calls_to(&e, CLONE_MAP_LOOKUP).len(), 5);
    }

    #[test]
    fn link_object_takes_three_linked_objects() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(OBJECT_LINK_OBJECT, |_, _| Ret::default());
        let next = collider(&mut e, 0);
        let mut linked = vec![0x4440, 0x5550, next.addr()].into_iter();
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            word(linked.next().unwrap())
        });
        let this = collider(&mut e, 1);
        e.call(0x00c3_2610, &args![this, 0x6000u32]);
        assert_eq!(e.get(this, NiPSysCollider::m_pkSpawnModifier), 0x4440);
        assert_eq!(e.get(this, NiPSysCollider::m_pkManager), 0x5550);
        assert_eq!(e.get(this, NiPSysCollider::m_spNext), next.addr());
        assert_eq!(e.mem.u32(next.addr() + 4), 1);
    }

    #[test]
    fn constructor_sets_the_fields() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.map(ORIGIN, 0xc);
        e.mem.set_f32(ORIGIN + 8, 3.0);
        let this: Ptr<NiPSysCollider> = e.new_object();
        let r = e
            .call(0x00c3_2780, &args![this, 0.5f32, 0x101u32, 0u32, 0x4440u32])
            .u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), COLLIDER_VTABLE);
        assert_eq!(e.get(this, NiPSysCollider::m_fBounce), 0.5f32.to_bits());
        assert_eq!(e.get(this, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(this, NiPSysCollider::m_bDieOnCollide), 0);
        assert_eq!(e.get(this, NiPSysCollider::m_pkSpawnModifier), 0x4440);
        assert_eq!(e.mem.f32(this.addr() + 0x1c), 3.0);
    }

    #[test]
    fn load_binary_reads_the_bounce_flags_and_link_ids() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        let mut bytes = vec![2u8, 0].into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            if a[2] == 4 {
                e.mem.set_f32(a[1], 0.25);
            } else {
                e.mem.set_u8(a[1], bytes.next().unwrap());
            }
            word(a[2])
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = collider(&mut e, 1);
        e.set(this, NiPSysCollider::m_bDieOnCollide, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2840, &args![this, stream]);
        assert_eq!(e.mem.f32(this.addr() + 8), 0.25);
        assert_eq!(e.get(this, NiPSysCollider::m_bSpawnOnCollide), 1);
        assert_eq!(e.get(this, NiPSysCollider::m_bDieOnCollide), 0);
        assert_eq!(e.mem.u32(file + 4), 6);
        assert_eq!(calls_to(&e, READ_LINK_ID).len(), 3);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};
    use crate::units::niparticle::nipsysmodifier::test_stream;

    type C = NiPSysCollider;

    fn collider(e: &mut Engine) -> Ptr<C> {
        let this = e.new_object::<C>();
        e.set(this, C::m_fBounce, 0.5f32.to_bits());
        this
    }

    #[test]
    fn resolve_asks_the_next_collider() {
        const RESOLVE: u32 = 0x0000_f08c;
        let mut e = Engine::new();
        e.register(RESOLVE, |_, a| answer(a[4] + 0x100));
        let this = collider(&mut e);
        let call = |e: &mut Engine| {
            e.call(
                0x00c3_2450,
                &args![this, 0x7f80_0001u32, 0x10u32, 0x20u32, 3u32],
            )
            .u32()
        };
        assert_eq!(call(&mut e), 0);
        let next = viewer_test::object(&mut e, &[(0x8c, RESOLVE)]);
        e.set(this, C::m_spNext, next);
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e), 0x103);
        assert_eq!(
            calls_to(&e, RESOLVE),
            vec![vec![next, 0x7fc0_0001, 0x10, 0x20, 3]]
        );
    }

    #[test]
    fn update_moves_the_particle_and_spawns_or_kills_it() {
        const SPAWN: u32 = 0x0000_f0a4;
        const REMOVE: u32 = 0x0000_f1a4;
        let mut e = Engine::new();
        quiet(&mut e, &[SPAWN, REMOVE]);
        let this = collider(&mut e);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0x14 + 4 * i as u32, *value);
        }
        let data = viewer_test::object(&mut e, &[(0xa4, REMOVE)]);
        let vertices = e.mem.alloc(36);
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        let spawn = viewer_test::object(&mut e, &[(0xa4, SPAWN)]);
        let manager = e.mem.alloc(0x20);
        e.mem.set_u32(manager + MANAGER_TARGET, 0x9999);
        e.set(this, C::m_pkManager, manager);
        e.call_log = Some(vec![]);
        // Index 2 (only its low 16 bits pick the vertex).
        e.call(0x00c3_2490, &args![this, 5.0f32, data, 0x0001_0002u32]);
        let moved: Vec<f32> = (0..3).map(|i| e.mem.f32(vertices + 24 + 4 * i)).collect();
        assert_eq!(moved, vec![1.0, 2.0, 3.0]);
        assert!(calls_to(&e, SPAWN).is_empty());
        assert!(calls_to(&e, REMOVE).is_empty());
        // Spawn needs the flag and a spawn modifier.
        e.set(this, C::m_bSpawnOnCollide, 1);
        e.call(0x00c3_2490, &args![this, 5.0f32, data, 1u32]);
        assert!(calls_to(&e, SPAWN).is_empty());
        e.set(this, C::m_pkSpawnModifier, spawn);
        e.set(this, C::m_bDieOnCollide, 1);
        e.call(0x00c3_2490, &args![this, 5.0f32, data, 1u32]);
        assert_eq!(
            calls_to(&e, SPAWN),
            vec![vec![spawn, 5.0f32.to_bits(), 4.0f32.to_bits(), 1, 0x9999]]
        );
        assert_eq!(calls_to(&e, REMOVE), vec![vec![data, 1]]);
    }

    #[test]
    fn register_streamables_adds_the_next_collider() {
        const REGISTER: u32 = 0x0000_f054;
        let mut e = Engine::new();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| answer(0));
        quiet(&mut e, &[REGISTER]);
        let this = collider(&mut e);
        let next = viewer_test::object(&mut e, &[(0x54, REGISTER)]);
        e.set(this, C::m_spNext, next);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_2680, &args![this, 0x500u32]).bool());
        assert!(calls_to(&e, REGISTER).is_empty());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| answer(1));
        assert!(e.call(0x00c3_2680, &args![this, 0x500u32]).bool());
        assert_eq!(calls_to(&e, REGISTER), vec![vec![next, 0x500]]);
    }

    #[test]
    fn is_equal_compares_settings_and_linked_objects() {
        const EQUAL: u32 = 0x0000_f05c;
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| answer(1));
        e.register(EQUAL, |_, a| answer(u32::from(a[1] & 0xff == 0x77)));
        let this = collider(&mut e);
        let other = collider(&mut e);
        let check = |e: &mut Engine| e.call(0x00c3_26b0, &args![this, other]).bool();
        assert!(check(&mut e));
        e.set(other, C::m_fBounce, f32::NAN.to_bits());
        e.set(this, C::m_fBounce, f32::NAN.to_bits());
        assert!(!check(&mut e));
        e.set(other, C::m_fBounce, 0.5f32.to_bits());
        e.set(this, C::m_fBounce, 0.5f32.to_bits());
        e.set(other, C::m_bSpawnOnCollide, 1);
        assert!(!check(&mut e));
        e.set(this, C::m_bSpawnOnCollide, 1);
        e.set(other, C::m_bDieOnCollide, 1);
        assert!(!check(&mut e));
        e.set(this, C::m_bDieOnCollide, 1);
        // Spawn modifiers.
        let ours = viewer_test::object(&mut e, &[(0x5c, EQUAL)]);
        e.set(this, C::m_pkSpawnModifier, ours);
        assert!(!check(&mut e));
        e.set(other, C::m_pkSpawnModifier, 0x1177);
        assert!(check(&mut e));
        e.set(other, C::m_pkSpawnModifier, 0x1178);
        assert!(!check(&mut e));
        e.set(other, C::m_pkSpawnModifier, 0x1177);
        // Managers: only whether there is one.
        e.set(this, C::m_pkManager, 0x10);
        assert!(!check(&mut e));
        e.set(other, C::m_pkManager, 0x20);
        assert!(check(&mut e));
        // Next colliders.
        e.set(other, C::m_spNext, 0x2277);
        assert!(!check(&mut e));
        let next = viewer_test::object(&mut e, &[(0x5c, EQUAL)]);
        e.set(this, C::m_spNext, next);
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(calls_to(&e, EQUAL)[1], vec![next, 0x2277]);
        e.register(OBJECT_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e));
    }

    #[test]
    fn destructor_releases_the_next_collider() {
        const DELETE: u32 = 0x0000_f004;
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        quiet(&mut e, &[DELETE, OBJECT_DESTRUCTOR, NI_DELETE]);
        let this = collider(&mut e);
        let next = viewer_test::object(&mut e, &[(0x4, DELETE)]);
        e.mem.set_u32(next + 4, 1);
        e.set(this, C::m_spNext, next);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_27f0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, DELETE), vec![vec![next]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x2c]]);
        e.mem.set_u32(next + 4, 2);
        e.call(0x00c3_27f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, DELETE).len(), 1);
        assert_eq!(calls_to(&e, NI_DELETE).len(), 1);
    }

    #[test]
    fn save_binary_writes_settings_and_links() {
        const SAVE_LINK: u32 = 0x0000_f130;
        let mut e = Engine::new();
        quiet(&mut e, &[BASE_SAVE_BINARY, SAVE_LINK]);
        let stream = viewer_test::out_stream(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + 0x30, SAVE_LINK);
        let this = collider(&mut e);
        e.set(this, C::m_bDieOnCollide, 1);
        e.set(this, C::m_pkSpawnModifier, 0x11);
        e.set(this, C::m_pkManager, 0x22);
        e.set(this, C::m_spNext, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2900, &args![this, stream]);
        let mut bytes = 0.5f32.to_le_bytes().to_vec();
        bytes.extend([0, 1]);
        assert_eq!(viewer_test::written(&e, stream), (bytes, 6));
        assert_eq!(
            calls_to(&e, SAVE_LINK),
            vec![vec![stream, 0x11], vec![stream, 0x22], vec![stream, 0x33]]
        );
    }

    #[test]
    fn viewer_strings_name_the_settings() {
        const STRINGS: u32 = 0x0000_f060;
        let mut e = Engine::new();
        quiet(&mut e, &[OBJECT_GET_VIEWER_STRINGS, STRINGS]);
        viewer_test::counting(
            &mut e,
            &[
                VIEWER_STRING,
                VIEWER_STRING_FLOAT,
                VIEWER_STRING_BOOL,
                VIEWER_STRING_TEXT,
                VIEWER_STRING_POINTER,
            ],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0120);
        let this = collider(&mut e);
        e.set(this, C::m_bSpawnOnCollide, 1);
        e.set(this, C::m_pkManager, 0x44);
        let strings = viewer_test::strings(&mut e, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_29c0, &args![this, strings]);
        assert_eq!(
            viewer_test::collected(&e, strings),
            (1..=6).collect::<Vec<u32>>()
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_BOOL),
            vec![
                vec![LABEL_SPAWN_ON_COLLIDE, 1],
                vec![LABEL_DIE_ON_COLLIDE, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_TEXT),
            vec![vec![LABEL_SPAWN_MODIFIER, TEXT_NONE]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_POINTER),
            vec![vec![LABEL_MANAGER, 0x44]]
        );
        assert!(calls_to(&e, STRINGS).is_empty());
        let next = viewer_test::object(&mut e, &[(0x60, STRINGS)]);
        e.set(this, C::m_spNext, next);
        let spawn = e.mem.alloc(0x10);
        e.mem.set_u32(spawn + 8, 0x0100_0130);
        e.set(this, C::m_pkSpawnModifier, spawn);
        e.call(0x00c3_29c0, &args![this, strings]);
        assert_eq!(calls_to(&e, STRINGS), vec![vec![next, strings]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_TEXT)[1],
            vec![LABEL_SPAWN_MODIFIER, 0x0100_0130]
        );
    }
}
