//! `NiParticle/nipsysplanarcollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c32680`: the jump target of `00c28d00`.
const JUMP_TARGET_00C28D00: u32 = 0x00c3_2680;

// Translated from 00c28d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c32680` (engine map: `NiPSysCollider::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysPlanarCollider@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_planar_collider_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C28D00, &args![this, stream]).bool()
}

// Translated from 00c299d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26c8.
pub fn ni_p_sys_planar_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26c8
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `NiPSysCollider` functions this unit's code calls (unit
/// `nipsyscollider.cpp`).
const COLLIDER_LINK_OBJECT: u32 = 0x00c3_2610;
const COLLIDER_PROCESS_CLONE: u32 = 0x00c3_2580;
const COLLIDER_LOAD_BINARY: u32 = 0x00c3_2840;
const COLLIDER_COPY_MEMBERS: u32 = 0x00c3_2510;
const COLLIDER_CONSTRUCT: u32 = 0x00c3_2780;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (unit `nistream.cpp`, no words): reads one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiColor::LoadBinary` (engine map; one word: the stream): three floats.
const THREE_FLOATS_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `00a53c30` (unnamed, on the cloning process's map; two words: the key, the
/// result slot): the clone of an object, `bool` in `AL`.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPlane::NiPlane_ov2` (two words: the normal, the constant).
const PLANE_CONSTRUCT: u32 = 0x00a6_9970;
/// `NiTransform::MakeIdentity` (no words).
const TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
/// `00a5d3d0` (unit `niobject.cpp`, no words): the `NiObject` destructor.
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// The `NiPSysPlanarCollider` vtable.
const PLANAR_COLLIDER_VTABLE: u32 = 0x010b_eb0c;
/// `0.5` (`float`), the constructor's two half-squared sizes.
const HALF_FLOAT: u32 = 0x0101_6248;
/// The vector the constructor uses for the plane's normal and the position
/// (`011f426c`), and the nine words it copies as the rotation (`011a9448`).
const ORIGIN: u32 = 0x011f_426c;
const IDENTITY_ROTATION: u32 = 0x011a_9448;
/// The two axes `CreateClone` and the creation function pass (`011a946c`,
/// `011a9478`, three words each).
const FIRST_AXIS: u32 = 0x011a_946c;
const SECOND_AXIS: u32 = 0x011a_9478;
/// The size of a `NiPSysPlanarCollider` on PC.
const PLANAR_COLLIDER_SIZE: u32 = 0x100;

// Translated from 00c27ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::LinkObject` (Xbox PDB): the collider part
/// (`00c32610`), then the linked object becomes `m_pkColliderObj` (`+0x2C`).
pub fn ni_p_sys_planar_collider_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.mem.set_u32(this.addr() + 0x2c, object);
}

// Translated from 00c29450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::ProcessClone` (Xbox PDB): the collider part, then
/// this collider's clone is looked up (`00a53c30`, the result slot is the
/// argument slot, set to 0 first) and, when there is a collider object, the
/// clone's becomes that object's clone, or the object itself when the map
/// has none.
pub fn ni_p_sys_planar_collider_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(COLLIDER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let object = e.mem.u32(this.addr() + 0x2c);
        let clone = e.mem.u32(slot.addr());
        if object == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        let value = if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
            e.mem.u32(slot.addr())
        } else {
            e.mem.u32(this.addr() + 0x2c)
        };
        e.mem.set_u32(clone.wrapping_add(0x2c), value);
    });
}

// Translated from 00c29770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::LoadBinary` (Xbox PDB): the collider part, the
/// collider object's link ID (`00a645b0`), width and height (four bytes each,
/// through the stream's input file `+0x24c`: its read function `+8`, `cdecl
/// (file, dest, size, &count, 1)`, the result added to its position `+4`;
/// the count word is the argument slot), then the two axes (`00a59be0`).
pub fn ni_p_sys_planar_collider_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
    e.with_stack(4, |e, count| {
        for offset in [0x30u32, 0x34] {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let read = e.mem.u32(file + 8);
            e.mem.set_u32(count.addr(), 4);
            let got = e
                .call(read, &args![file, this.addr() + offset, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(got));
        }
    });
    e.call(THREE_FLOATS_LOAD_BINARY, &args![this.addr() + 0x38, stream]);
    e.call(THREE_FLOATS_LOAD_BINARY, &args![this.addr() + 0x44, stream]);
}

// Translated from 00c29900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::NiPSysPlanarCollider` (Xbox PDB; `thiscall`, 13
/// words): the collider constructor (`00c32780`) with the first four (the
/// bounce through `FLD`/`FSTP`), then the collider object (`+0x2C`), width
/// and height (`FLD`/`FSTP`), the two axes (`+0x38`, `+0x44`), both
/// half-squared sizes 0.5, the vtable, the plane (`NiPlane` from `011f426c`
/// and 0), the position from `011f426c`, the rotation from `011a9448`, and
/// two identity transforms (`+0x98`, `+0xCC`). Answers `this`.
pub fn ni_p_sys_planar_collider_ni_p_sys_planar_collider(
    e: &mut Engine,
    this: Ptr,
    bounce: f32,
    spawn_on_collide: u32,
    die_on_collide: u32,
    spawn_modifier: u32,
    collider_object: u32,
    width: f32,
    height: f32,
    x_axis_x: u32,
    x_axis_y: u32,
    x_axis_z: u32,
    y_axis_x: u32,
    y_axis_y: u32,
    y_axis_z: u32,
) -> Ptr {
    let a = this.addr();
    let bounce = x87_quiet_bits(bounce.to_bits());
    e.call(
        COLLIDER_CONSTRUCT,
        &args![
            this,
            bounce,
            spawn_on_collide,
            die_on_collide,
            spawn_modifier
        ],
    );
    e.mem.set_u32(a + 0x30, x87_quiet_bits(width.to_bits()));
    e.mem.set_u32(a + 0x34, x87_quiet_bits(height.to_bits()));
    e.mem.set_u32(a + 0x38, x_axis_x);
    let half = x87_quiet_bits(e.global::<u32>(HALF_FLOAT));
    e.mem.set_u32(a + 0x50, half);
    e.mem.set_u32(a + 0x54, half);
    e.mem.set_u32(a + 0x2c, collider_object);
    e.mem.set_u32(a + 0x3c, x_axis_y);
    e.mem.set_u32(a + 0x44, y_axis_x);
    e.mem.set_u32(a + 0x40, x_axis_z);
    e.mem.set_u32(a + 0x48, y_axis_y);
    e.mem.set_u32(a, PLANAR_COLLIDER_VTABLE);
    e.mem.set_u32(a + 0x4c, y_axis_z);
    e.call(PLANE_CONSTRUCT, &args![a + 0x58, ORIGIN, 0.0f32]);
    for i in 0..3 {
        let word = e.mem.u32(ORIGIN + 4 * i);
        e.mem.set_u32(a + 0x68 + 4 * i, word);
    }
    for i in 0..9 {
        let word = e.mem.u32(IDENTITY_ROTATION + 4 * i);
        e.mem.set_u32(a + 0x74 + 4 * i, word);
    }
    e.call(TRANSFORM_MAKE_IDENTITY, &args![a + 0x98]);
    e.call(TRANSFORM_MAKE_IDENTITY, &args![a + 0xcc]);
    this
}

// Translated from 00c299e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::_scalar_deleting_destructor_` (Xbox PDB): releases
/// `m_spNext` (`+0x28`), the `NiObject` destructor (`00a5d3d0`), then the sized
/// delete of 0x100 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_p_sys_planar_collider_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let next = e.mem.u32(this.addr() + 0x28);
    if next != 0 && e.call(INTERLOCKED_DECREMENT, &args![next + 4]).u32() == 0 {
        e.vcall(next, 4, &[]);
    }
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, PLANAR_COLLIDER_SIZE]);
    }
    this
}

/// The planar collider `00c29a30` and `00c29af0` construct in `block`: bounce
/// 1, no flags, spawn modifier or object, width and height 1, the axes
/// `011a946c` and `011a9478`.
fn construct_default(e: &mut Engine, block: u32) -> Ptr {
    let one = 1.0f32;
    let word = |e: &Engine, addr: u32| e.mem.u32(addr);
    let axes = [
        word(e, FIRST_AXIS),
        word(e, FIRST_AXIS + 4),
        word(e, FIRST_AXIS + 8),
        word(e, SECOND_AXIS),
        word(e, SECOND_AXIS + 4),
        word(e, SECOND_AXIS + 8),
    ];
    ni_p_sys_planar_collider_ni_p_sys_planar_collider(
        e,
        Ptr::new(block),
        one,
        0,
        0,
        0,
        0,
        one,
        one,
        axes[0],
        axes[1],
        axes[2],
        axes[3],
        axes[4],
        axes[5],
    )
}

// Translated from 00c29a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::CreateClone` (Xbox PDB): a default planar collider
/// (null when the allocation fails, still used as the exe does), the
/// collider members copied (`00c32510`), width and height copied with
/// `FLD`/`FSTP`, the axes word by word. Answers the clone.
pub fn ni_p_sys_planar_collider_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![PLANAR_COLLIDER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_default(e, block)
    };
    e.call(COLLIDER_COPY_MEMBERS, &args![this, clone, process]);
    let a = this.addr();
    let c = clone.addr();
    for offset in [0x30u32, 0x34] {
        let value = x87_quiet_bits(e.mem.u32(a + offset));
        e.mem.set_u32(c.wrapping_add(offset), value);
    }
    for offset in (0x38u32..0x50).step_by(4) {
        let word = e.mem.u32(a + offset);
        e.mem.set_u32(c.wrapping_add(offset), word);
    }
    clone
}

// Translated from 00c29af0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysPlanarCollider` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x100 bytes constructed as the default planar collider (the
/// constructor's result is answered), or null.
pub fn fn_00c29af0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![PLANAR_COLLIDER_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_default(e, block)
}

/// This unit's translated functions, by exe address.
use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, add_viewer_string, stream_write, NI_GET_VIEWER_STRING_FIXED,
    NI_GET_VIEWER_STRING_FLOAT, STREAM_SAVE_LINK_ID,
};

/// `NiPSysCollider::Update` (`00c32490`, `thiscall`: the time as a
/// `float`, the particle data, the particle index), `SaveBinary`
/// (`00c32900`), `IsEqual` (`00c326b0`) and `GetViewerStrings`
/// (`00c329c0`) (one word each).
const COLLIDER_UPDATE: u32 = 0x00c3_2490;
const COLLIDER_SAVE_BINARY: u32 = 0x00c3_2900;
const COLLIDER_IS_EQUAL: u32 = 0x00c3_26b0;
const COLLIDER_GET_VIEWER_STRINGS: u32 = 0x00c3_29c0;
/// The point helpers the code calls on the axes (`thiscall`, one word):
/// `00a69780` writes its three floats to a stream (the engine map names it
/// after `NiColor`, whose body it shares), `00439090` answers whether it
/// differs from another point, `00a59af0` (`NiPoint3::GetViewerString`)
/// makes its viewer string with a label.
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiTransform::operator!=` (`00a86c30`, `thiscall`, one word: the other
/// transform; the engine map calls it `operator_`).
const TRANSFORM_NOT_EQUAL: u32 = 0x00a8_6c30;
/// `NiPlane::NiPlane_ov3` (`00a69990`, `thiscall`: the normal's and a
/// point's addresses): the plane through the point.
const PLANE_FROM_NORMAL_AND_POINT: u32 = 0x00a6_9990;
/// `004a0c10` (`thiscall` on an `NiPoint3`): normalizes it in place.
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall` on the
/// transform, the address of the result) and the transform product
/// `0062c250` (`thiscall` on the left transform, `(result, right)`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `0.001` (`float`): the closeness below which a particle resting on the
/// plane is only pushed out.
const EPSILON: u32 = 0x010b_eaf8;
/// The `NiRTTI` record of `NiPSysPlanarCollider` (`012026c8`).
const RTTI: u32 = 0x0120_26c8;
/// Labels: `"Collider Object"`, `"Width"`, `"Height"`, `"X-Axis"`,
/// `"Y-Axis"`, and `"None"` (the object's name when there is none).
const LABEL_COLLIDER_OBJECT: u32 = 0x010b_e8cc;
const LABEL_WIDTH: u32 = 0x0109_dc8c;
const LABEL_HEIGHT: u32 = 0x0109_dc84;
const LABEL_X_AXIS: u32 = 0x010b_ebac;
const LABEL_Y_AXIS: u32 = 0x010b_eba4;
const TEXT_NONE: u32 = 0x0102_909c;
/// `NiParticleInfo` (0x1c bytes): the velocity at +0, +4, +8.
const INFO_SIZE: u32 = 0x1c;
/// `NiPSysData::m_pkParticleInfo` (+0x70) as the PC code reads it.
const DATA_PARTICLE_INFO: u32 = 0x70;
/// `NiPSysCollider::m_pkManager` (+0x24, Xbox PDB) and the manager's
/// particle system (+0x10 of the manager, as the PC code reads it).
const COLLIDER_MANAGER: u32 = 0x24;
const MANAGER_SYSTEM: u32 = 0x10;
/// `NiPSysCollider::m_spNext` (+0x28, Xbox PDB) and
/// `NiPSysCollider::SetResolveConstants` (vtable slot +0x94).
const COLLIDER_NEXT: u32 = 0x28;
const SET_RESOLVE_CONSTANTS_SLOT: u32 = 0x94;
/// `NiAVObject::m_kWorld` as the PC code reads it: the 13-word world
/// transform at +0x68.
const WORLD_TRANSFORM: u32 = 0x68;

/// The float at `addr` as `FLD dword` loads it.
fn load(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The float at `addr` as an x87 `float` memory operand.
fn operand(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

// Translated from 00c29320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::Update` (Xbox PDB): the particle's velocity
/// component along the plane normal (`d`, rounded to a float in the stack
/// word of the index argument) and that component as a vector (`n * d`).
/// A particle at the collision time whose `d` is below 0.001 (`010beaf8`)
/// only loses the component; any other has it subtracted twice (reflected)
/// and the velocity scaled by the bounce (`m_fBounce`). Then
/// `NiPSysCollider::Update` with the same time, data and index. x87
/// operations one by one in the exe's order.
pub fn ni_p_sys_planar_collider_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    data: Ptr,
    index: u32,
) {
    let a = this.addr();
    let infos = e.mem.u32(data.addr() + DATA_PARTICLE_INFO);
    let info = infos.wrapping_add((index & 0xffff) * INFO_SIZE);
    e.with_stack(0x30, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        // The index argument's stack word, reused for float temporaries.
        let slot = at(0x2c);
        let d = x87_add(
            x87_add(
                x87_mul(load(e, a + 0x5c), operand(e, info + 4)),
                x87_mul(load(e, info), operand(e, a + 0x58)),
            ),
            x87_mul(load(e, a + 0x60), operand(e, info + 8)),
        );
        e.mem.set_u32(slot, x87_store32(d));
        let d = load(e, slot);
        e.mem
            .set_u32(at(0x8), x87_store32(x87_mul(load(e, a + 0x58), d)));
        e.mem
            .set_u32(at(0xc), x87_store32(x87_mul(load(e, a + 0x5c), d)));
        e.mem
            .set_u32(at(0x10), x87_store32(x87_mul(load(e, a + 0x60), d)));
        let collision_time = load(e, a + 0x20);
        let now = x87_load32(time.to_bits());
        let epsilon = x87_load32(e.global::<u32>(EPSILON));
        // `FUCOMP`; `TEST AH,0x44; JP`: equal times only. `FLD eps; FCOMP
        // d; TEST AH,0x41; JNZ`: the push-out only when 0.001 is above `d`.
        if now == collision_time && epsilon > d {
            for k in 0..3 {
                let v = info.wrapping_add(4 * k);
                let value = x87_sub(load(e, v), operand(e, at(0x8 + 4 * k)));
                e.mem.set_u32(v, x87_store32(value));
            }
        } else {
            for k in 0..3 {
                let doubled = x87_add(load(e, at(0x8 + 4 * k)), load(e, at(0x8 + 4 * k)));
                e.mem.set_u32(at(0x14 + 4 * k), x87_store32(doubled));
            }
            let mut reflected = [0.0f64; 3];
            for k in 0..3 {
                let v = info.wrapping_add(4 * k);
                let value = x87_sub(load(e, v), operand(e, at(0x14 + 4 * k)));
                e.mem.set_u32(slot, x87_store32(value));
                reflected[k as usize] = load(e, slot);
                e.mem.set_u32(v, x87_store32(reflected[k as usize]));
            }
            e.mem.set_u32(slot, x87_quiet_bits(e.mem.u32(a + 0x8)));
            let bounce = load(e, slot);
            e.mem
                .set_u32(info, x87_store32(x87_mul(reflected[0], bounce)));
            e.mem.set_u32(
                info.wrapping_add(4),
                x87_store32(x87_mul(reflected[1], bounce)),
            );
            e.mem.set_u32(
                info.wrapping_add(8),
                x87_store32(x87_mul(bounce, reflected[2])),
            );
        }
    });
    e.call(
        COLLIDER_UPDATE,
        &args![this, x87_quiet_bits(time.to_bits()), data, index],
    );
}

// Translated from 00c294b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::SetResolveConstants` (Xbox PDB). When the plane
/// is still the zero plane (`NiPlane` from `011f426c` and 0), or the
/// collider object's or the particle system's world transform differs from
/// the last one kept (`NiTransform::operator!=`; without a collider object
/// nothing is recomputed): the position becomes `011f426c` and the
/// rotation `011a9448`; the normal is `X axis x Y axis`, normalized; with a
/// collider object its transform relative to the particle system gives the
/// position (its translation), the rotation, the normal rotated by it
/// (normalized again) and the scale applied to width and height, and both
/// world transforms are kept; then `m_fHalfSqScaledWidth` and `...Height`
/// are the squares of half of them and the plane goes through the position
/// (`NiPlane_ov3`). Finally, with a next collider, its
/// `SetResolveConstants` (slot +0x94, a tail jump). x87 operations one by
/// one in the exe's order; the stack slots are the exe's.
pub fn ni_p_sys_planar_collider_set_resolve_constants(e: &mut Engine, this: Ptr) {
    let a = this.addr();
    e.with_stack(0x108, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        let zero_plane = e
            .call(PLANE_CONSTRUCT, &args![at(0x24), ORIGIN, 0u32])
            .u32();
        // `FUCOMPP; TEST AH,0x44; JP`: each word must compare equal.
        let is_zero_plane =
            (0..4).all(|i| load(e, a + 0x58 + 4 * i) == load(e, zero_plane + 4 * i));
        let recompute = if is_zero_plane {
            true
        } else {
            let object = e.mem.u32(a + 0x2c);
            if object == 0 {
                false
            } else if e
                .call(
                    TRANSFORM_NOT_EQUAL,
                    &args![a + 0x98, object.wrapping_add(WORLD_TRANSFORM)],
                )
                .bool()
            {
                true
            } else {
                let manager = e.mem.u32(a + COLLIDER_MANAGER);
                let system = e.mem.u32(manager.wrapping_add(MANAGER_SYSTEM));
                e.call(
                    TRANSFORM_NOT_EQUAL,
                    &args![a + 0xcc, system.wrapping_add(WORLD_TRANSFORM)],
                )
                .bool()
            }
        };
        if !recompute {
            return;
        }
        for i in 0..3 {
            let word = e.mem.u32(ORIGIN + 4 * i);
            e.mem.set_u32(a + 0x68 + 4 * i, word);
        }
        for i in 0..9 {
            let word = e.mem.u32(IDENTITY_ROTATION + 4 * i);
            e.mem.set_u32(a + 0x74 + 4 * i, word);
        }
        // The normal: X axis (+0x38) x Y axis (+0x44).
        let n0 = x87_sub(
            x87_mul(load(e, a + 0x4c), operand(e, a + 0x3c)),
            x87_mul(load(e, a + 0x48), operand(e, a + 0x40)),
        );
        e.mem.set_u32(at(0x18), x87_store32(n0));
        let n1 = x87_sub(
            x87_mul(load(e, a + 0x44), operand(e, a + 0x40)),
            x87_mul(load(e, a + 0x4c), operand(e, a + 0x38)),
        );
        e.mem.set_u32(at(0x1c), x87_store32(n1));
        let n2 = x87_sub(
            x87_mul(load(e, a + 0x38), operand(e, a + 0x48)),
            x87_mul(load(e, a + 0x3c), operand(e, a + 0x44)),
        );
        e.mem.set_u32(at(0x20), x87_store32(n2));
        e.call(POINT_UNITIZE, &args![at(0x18)]);
        let width = x87_quiet_bits(e.mem.u32(a + 0x30));
        e.mem.set_u32(at(0x14), width);
        let height = x87_quiet_bits(e.mem.u32(a + 0x34));
        e.mem.set_u32(at(0x10), height);
        let object = e.mem.u32(a + 0x2c);
        if object != 0 {
            for i in 0..13 {
                let word = e.mem.u32(object.wrapping_add(WORLD_TRANSFORM + 4 * i));
                e.mem.set_u32(at(0x68 + 4 * i), word);
            }
            let manager = e.mem.u32(a + COLLIDER_MANAGER);
            let system = e.mem.u32(manager.wrapping_add(MANAGER_SYSTEM));
            for i in 0..13 {
                let word = e.mem.u32(system.wrapping_add(WORLD_TRANSFORM + 4 * i));
                e.mem.set_u32(at(0x9c + 4 * i), word);
            }
            e.call(TRANSFORM_INVERT, &args![at(0x9c), at(0xd0)]);
            e.call(TRANSFORM_PRODUCT, &args![at(0xd0), at(0x34), at(0x68)]);
            for i in 0..3 {
                let word = e.mem.u32(at(0x58 + 4 * i));
                e.mem.set_u32(a + 0x68 + 4 * i, word);
            }
            for i in 0..9 {
                let word = e.mem.u32(at(0x34 + 4 * i));
                e.mem.set_u32(a + 0x74 + 4 * i, word);
            }
            let r = |i: u32| a + 0x74 + 4 * i;
            let (n0, n1, n2) = (load(e, at(0x18)), load(e, at(0x1c)), load(e, at(0x20)));
            let r1n1 = x87_mul(load(e, r(1)), n1);
            let n0r0 = x87_mul(n0, operand(e, r(0)));
            let s01 = x87_add(r1n1, n0r0);
            let r2n2 = x87_mul(load(e, r(2)), n2);
            let m0 = x87_store32(x87_add(r2n2, s01));
            e.mem.set_u32(at(0x24), m0);
            e.mem.set_u32(at(0x18), m0);
            let r4n1 = x87_mul(load(e, r(4)), n1);
            let r3n0 = x87_mul(load(e, r(3)), n0);
            let s = x87_add(r4n1, r3n0);
            let m1 = x87_store32(x87_add(s, x87_mul(load(e, r(5)), n2)));
            e.mem.set_u32(at(0x28), m1);
            e.mem.set_u32(at(0x1c), m1);
            let n1r7 = x87_mul(n1, load(e, r(7)));
            let n0r6 = x87_mul(n0, operand(e, r(6)));
            let s = x87_add(n1r7, n0r6);
            let n2r8 = x87_mul(n2, load(e, r(8)));
            let m2 = x87_store32(x87_add(n2r8, s));
            e.mem.set_u32(at(0x2c), m2);
            e.mem.set_u32(at(0x20), m2);
            e.call(POINT_UNITIZE, &args![at(0x18)]);
            let scale = load(e, at(0x64));
            let scaled_width = x87_store32(x87_mul(scale, operand(e, at(0x14))));
            for i in 0..13 {
                let word = e.mem.u32(at(0x68 + 4 * i));
                e.mem.set_u32(a + 0x98 + 4 * i, word);
            }
            e.mem.set_u32(at(0x14), scaled_width);
            let scaled_height = x87_store32(x87_mul(scale, operand(e, at(0x10))));
            e.mem.set_u32(at(0x10), scaled_height);
            for i in 0..13 {
                let word = e.mem.u32(at(0x9c + 4 * i));
                e.mem.set_u32(a + 0xcc + 4 * i, word);
            }
        }
        let half: f64 = e.global(HALF_DOUBLE);
        // `FMUL ST1,ST0` (`DC C9`): the width times 0.5.
        let half_width = x87_store32(x87_mul(load(e, at(0x14)), half));
        e.mem.set_u32(at(0x14), half_width);
        let half_height = x87_store32(x87_mul(half, operand(e, at(0x10))));
        e.mem.set_u32(at(0x10), half_height);
        let w = load(e, at(0x14));
        e.mem.set_u32(a + 0x50, x87_store32(x87_mul(w, w)));
        let h = load(e, at(0x10));
        e.mem.set_u32(a + 0x54, x87_store32(x87_mul(h, h)));
        let plane = e
            .call(
                PLANE_FROM_NORMAL_AND_POINT,
                &args![at(0x24), at(0x18), a + 0x68],
            )
            .u32();
        for i in 0..4 {
            let word = e.mem.u32(plane + 4 * i);
            e.mem.set_u32(a + 0x58 + 4 * i, word);
        }
    });
    let next = e.mem.u32(a + COLLIDER_NEXT);
    if next != 0 {
        e.vcall(next, SET_RESOLVE_CONSTANTS_SLOT, &args![]);
    }
}

// Translated from 00c297f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::SaveBinary` (Xbox PDB): `NiPSysCollider`'s
/// `SaveBinary`, the collider object's link id (stream slot +0x30), width
/// and height (4 bytes each; the component sizes kept in the stack word
/// that held the stream argument), then the X and Y axes (`00a69780`).
pub fn ni_p_sys_planar_collider_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (a, s) = (this.addr(), stream.addr());
    e.call(COLLIDER_SAVE_BINARY, &args![a, s]);
    let object = e.mem.u32(a + 0x2c);
    e.vcall(s, STREAM_SAVE_LINK_ID, &args![object]);
    e.with_stack(4, |e, argument_slot| {
        e.mem.set_u32(argument_slot.addr(), s);
        stream_write(e, s, a + 0x30, 4, 4, argument_slot.addr());
        stream_write(e, s, a + 0x34, 4, 4, argument_slot.addr());
    });
    e.call(POINT_SAVE_BINARY, &args![a + 0x38, s]);
    e.call(POINT_SAVE_BINARY, &args![a + 0x44, s]);
}

// Translated from 00c29870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::IsEqual` (Xbox PDB): `NiPSysCollider::IsEqual`;
/// both or neither must have a collider object, two objects must be equal
/// (the first's `IsEqual`, slot +0x5c); equal widths and heights
/// (`FUCOMPP`: a NaN is unequal); the X axes and then the Y axes must not
/// differ (`00439090` on this one's with the other's).
pub fn ni_p_sys_planar_collider_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    if !e.call(COLLIDER_IS_EQUAL, &args![a, b]).bool() {
        return false;
    }
    let mine = e.mem.u32(a + 0x2c);
    let theirs = e.mem.u32(b + 0x2c);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 && !e.vcall(mine, 0x5c, &args![theirs]).bool() {
        return false;
    }
    if load(e, a + 0x30) != load(e, b + 0x30) || load(e, a + 0x34) != load(e, b + 0x34) {
        return false;
    }
    if e.call(POINT_NOT_EQUAL, &args![a + 0x38, b + 0x38]).bool() {
        return false;
    }
    !e.call(POINT_NOT_EQUAL, &args![a + 0x44, b + 0x44]).bool()
}

// Translated from 00c29b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::GetViewerStrings` (Xbox PDB):
/// `NiPSysCollider::GetViewerStrings`, then the class name (`NiRTTI`
/// `012026c8`), `"Collider Object"` (the object's name handle, or
/// `"None"`), `"Width"`, `"Height"` (the floats through `FLD`/`FSTP`) and
/// the axes' strings with `"X-Axis"` and `"Y-Axis"`
/// (`NiPoint3::GetViewerString`) appended.
pub fn ni_p_sys_planar_collider_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (a, strings) = (this.addr(), strings.addr());
    e.call(COLLIDER_GET_VIEWER_STRINGS, &args![a, strings]);
    add_class_name(e, strings, RTTI);
    let object = e.mem.u32(a + 0x2c);
    let name = if object == 0 {
        TEXT_NONE
    } else {
        e.mem.u32(object + 8)
    };
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FIXED,
        LABEL_COLLIDER_OBJECT,
        name,
    );
    let width = x87_quiet_bits(e.mem.u32(a + 0x30));
    add_labelled(e, strings, NI_GET_VIEWER_STRING_FLOAT, LABEL_WIDTH, width);
    let height = x87_quiet_bits(e.mem.u32(a + 0x34));
    add_labelled(e, strings, NI_GET_VIEWER_STRING_FLOAT, LABEL_HEIGHT, height);
    for (axis, label) in [(a + 0x38, LABEL_X_AXIS), (a + 0x44, LABEL_Y_AXIS)] {
        let text = e.call(POINT_GET_VIEWER_STRING, &args![axis, label]).u32();
        add_viewer_string(e, strings, text);
    }
}

/// `NiPSysCollider::Resolve` (`00c32450`, `thiscall`: the start time as a
/// `float`, the address of the end time, the particle data, the index).
const COLLIDER_RESOLVE: u32 = 0x00c3_2450;
/// `004b4500` (`thiscall` on an `NiMatrix3`, `(result, point)`): the
/// matrix times a point.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `_CIpow` (`00ec9b00`: base, exponent as doubles).
const POW: u32 = 0x00ec_9b00;
/// `Resolve`'s function-local `static float` (`012026c0`, `pow(2, -21)`
/// from the doubles `2.0` at `01011590` and `-21.0` at `010beb00`) and its
/// guard word (`012026c4`, bit 0).
const NUDGE: u32 = 0x0120_26c0;
const NUDGE_GUARD: u32 = 0x0120_26c4;
const NUDGE_BASE: u32 = 0x0101_1590;
const NUDGE_EXPONENT: u32 = 0x010b_eb00;
/// `0.0` (`double`, `01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `NiPSysData` positions (+0x20) as the PC code reads them.
const DATA_VERTICES: u32 = 0x20;

// Translated from 00c28d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysPlanarCollider::Resolve` (Xbox PDB; `RET 0x10`: start time, the
/// address of the end time, particle data, particle index; returns `EAX`).
/// The particle's signed distance from the plane (`n . p - c`, rounded). A
/// particle within 0.001 of the plane collides at the start time when its
/// offset from the plane's position, projected on the rotated X and Y axes
/// (`004b4500`), stays inside the half sizes squared (`m_fHalfSqScaled...`,
/// strictly): the end time, the collision time and point are set and
/// `this` is answered at once. Otherwise the motion over the time step
/// (`end - start`) is checked for crossing the plane (distance after the
/// step, its magnitude below 0.001 taken as 0, of the other sign): the
/// crossing time `-distance * step / normal speed` and point; a point still
/// on the far side is moved along the normal by `pow(2, -21)` (a
/// function-local static set on first use) times its largest coordinate
/// magnitude, towards the side the particle came from; inside the
/// rectangle (as above) the end time and the collision time become start
/// plus the crossing time and the point is kept. Then
/// `NiPSysCollider::Resolve` with the same words; its non-null answer is
/// returned, else `this` when this collider was hit, else 0. x87
/// operations one by one in the exe's order; the stack slots are the exe's
/// (the data and end-time argument words hold float temporaries).
pub fn ni_p_sys_planar_collider_resolve(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    end_time: Ptr,
    data: Ptr,
    index: u32,
) -> u32 {
    let a = this.addr();
    let particle = index & 0xffff;
    let position = e
        .mem
        .u32(data.addr() + DATA_VERTICES)
        .wrapping_add(particle * 12);
    let info = e
        .mem
        .u32(data.addr() + DATA_PARTICLE_INFO)
        .wrapping_add(particle * INFO_SIZE);
    let (on_plane, collided) = e.with_stack(0x68, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        let lf = |e: &Engine, x: u32| load(e, at(x));
        let mf = |e: &Engine, x: u32| operand(e, at(x));
        e.mem.set_u32(at(0x58), time.to_bits());
        e.mem.set_u32(at(0x5c), end_time.addr());
        e.mem.set_u32(at(0x60), data.addr());
        e.mem.set_u32(at(0x64), index);
        e.mem.set_u8(at(0x13), 0);
        let half_width = |e: &Engine| load(e, a + 0x50);
        let half_height = |e: &Engine| load(e, a + 0x54);
        let d = x87_add(
            x87_add(
                x87_mul(load(e, a + 0x5c), operand(e, position + 4)),
                x87_mul(load(e, position), operand(e, a + 0x58)),
            ),
            x87_mul(load(e, a + 0x60), operand(e, position + 8)),
        );
        e.mem.set_u32(at(0x60), x87_store32(d));
        let distance = x87_sub(lf(e, 0x60), operand(e, a + 0x64));
        e.mem.set_u32(at(0x60), x87_store32(distance));
        let epsilon = x87_load32(e.global::<u32>(EPSILON));
        let mut distance = lf(e, 0x60);
        // `FCOM; TEST AH,5; JP` twice: on the plane only when both
        // `distance` and `-distance` are below 0.001.
        if distance < epsilon && -distance < epsilon {
            for k in 0..3 {
                let r = x87_sub(load(e, position + 4 * k), operand(e, a + 0x68 + 4 * k));
                e.mem.set_u32(at(0x18 + 4 * k), x87_store32(r));
            }
            e.call(MATRIX_TIMES_POINT, &args![a + 0x74, at(0x30), a + 0x38]);
            e.call(MATRIX_TIMES_POINT, &args![a + 0x74, at(0x3c), a + 0x44]);
            let (rx, ry) = (lf(e, 0x18), lf(e, 0x1c));
            let (axx, axy, axz) = (lf(e, 0x30), lf(e, 0x34), lf(e, 0x38));
            let s = x87_add(x87_mul(rx, axx), x87_mul(axy, ry));
            let u = x87_add(s, x87_mul(axz, mf(e, 0x20)));
            e.mem.set_u32(at(0x60), x87_store32(u));
            let u = lf(e, 0x60);
            e.mem.set_u32(at(0x24), x87_store32(x87_mul(axx, u)));
            e.mem.set_u32(at(0x28), x87_store32(x87_mul(axy, u)));
            e.mem.set_u32(at(0x2c), x87_store32(x87_mul(u, axz)));
            let (ayx, ayy, ayz) = (lf(e, 0x3c), lf(e, 0x40), lf(e, 0x44));
            let s = x87_add(x87_mul(rx, ayx), x87_mul(ry, ayy));
            let v = x87_add(s, x87_mul(ayz, mf(e, 0x20)));
            e.mem.set_u32(at(0x60), x87_store32(v));
            let v = lf(e, 0x60);
            e.mem.set_u32(at(0x18), x87_store32(x87_mul(ayx, v)));
            e.mem.set_u32(at(0x1c), x87_store32(x87_mul(ayy, v)));
            e.mem.set_u32(at(0x20), x87_store32(x87_mul(ayz, v)));
            let (px, py, pz) = (lf(e, 0x24), lf(e, 0x28), lf(e, 0x2c));
            let squared = x87_add(x87_add(x87_mul(px, px), x87_mul(py, py)), x87_mul(pz, pz));
            e.mem.set_u32(at(0x60), x87_store32(squared));
            // `FCOMPP; TEST AH,0x41; JNZ`: inside only when strictly below.
            if !(half_width(e) > lf(e, 0x60)) {
                return (false, e.mem.u8(at(0x13)));
            }
            let (qx, qy, qz) = (lf(e, 0x18), lf(e, 0x1c), lf(e, 0x20));
            let squared = x87_add(x87_add(x87_mul(qx, qx), x87_mul(qy, qy)), x87_mul(qz, qz));
            e.mem.set_u32(at(0x60), x87_store32(squared));
            if !(half_height(e) > lf(e, 0x60)) {
                return (false, e.mem.u8(at(0x13)));
            }
            let start = x87_quiet_bits(e.mem.u32(at(0x58)));
            e.mem.set_u32(end_time.addr(), start);
            e.mem.set_u32(a + 0x20, start);
            copy_point(e, position, a + 0x14);
            return (true, e.mem.u8(at(0x13)));
        }
        // Across the time step.
        let step = x87_sub(load(e, end_time.addr()), mf(e, 0x58));
        e.mem.set_u32(at(0x5c), x87_store32(step));
        let step = lf(e, 0x5c);
        e.mem
            .set_u32(at(0x24), x87_store32(x87_mul(step, operand(e, info))));
        e.mem
            .set_u32(at(0x28), x87_store32(x87_mul(step, operand(e, info + 4))));
        e.mem
            .set_u32(at(0x2c), x87_store32(x87_mul(load(e, info + 8), step)));
        let normal_speed = x87_add(
            x87_add(
                x87_mul(load(e, a + 0x5c), mf(e, 0x28)),
                x87_mul(load(e, a + 0x58), mf(e, 0x24)),
            ),
            x87_mul(load(e, a + 0x60), mf(e, 0x2c)),
        );
        e.mem.set_u32(at(0x5c), x87_store32(normal_speed));
        let normal_speed = lf(e, 0x5c);
        e.mem
            .set_u32(at(0x5c), x87_store32(x87_add(normal_speed, distance)));
        e.mem.set_u32(at(0x14), x87_store32(lf(e, 0x5c).abs()));
        // `FCOMP eps; TEST AH,5; JP`: zeroed only when below 0.001.
        if lf(e, 0x14) < epsilon {
            e.mem.set_u32(at(0x5c), 0);
        }
        let zero: f64 = e.global(ZERO_DOUBLE);
        if !(x87_mul(lf(e, 0x5c), distance) < zero) {
            return (false, e.mem.u8(at(0x13)));
        }
        let crossing = x87_div(x87_mul(step, -distance), normal_speed);
        e.mem.set_u32(at(0x14), x87_store32(crossing));
        let crossing = lf(e, 0x14);
        for k in 0..3 {
            let moved = x87_mul(crossing, operand(e, info + 4 * k));
            e.mem.set_u32(at(0x24 + 4 * k), x87_store32(moved));
        }
        e.mem.set_u32(
            at(0x18),
            x87_store32(x87_add(load(e, position), mf(e, 0x24))),
        );
        e.mem.set_u32(
            at(0x1c),
            x87_store32(x87_add(lf(e, 0x28), operand(e, position + 4))),
        );
        e.mem.set_u32(
            at(0x20),
            x87_store32(x87_add(lf(e, 0x2c), operand(e, position + 8))),
        );
        if e.global::<u8>(NUDGE_GUARD) & 1 == 0 {
            let guard: u32 = e.global(NUDGE_GUARD);
            e.set_global(NUDGE_GUARD, guard | 1);
            let base: f64 = e.global(NUDGE_BASE);
            let exponent: f64 = e.global(NUDGE_EXPONENT);
            let power = e.call(POW, &args![base, exponent]).f64();
            e.mem.set_u32(at(0x5c), x87_store32(power));
            let nudge = x87_quiet_bits(e.mem.u32(at(0x5c)));
            e.set_global(NUDGE, nudge);
            distance = lf(e, 0x60);
        }
        let (mut hx, mut hy, mut hz) = (lf(e, 0x18), lf(e, 0x1c), lf(e, 0x20));
        let g = x87_add(
            x87_mul(load(e, a + 0x60), hz),
            x87_add(
                x87_mul(load(e, a + 0x58), hx),
                x87_mul(load(e, a + 0x5c), hy),
            ),
        );
        e.mem.set_u32(at(0x5c), x87_store32(g));
        let side = x87_sub(lf(e, 0x5c), operand(e, a + 0x64));
        e.mem.set_u32(at(0x5c), x87_store32(side));
        // `FMUL dist; FCOMP 0.0; TEST AH,5; JP`: still across only when the
        // product is below 0.
        if x87_mul(lf(e, 0x5c), distance) < zero {
            e.mem.set_u32(at(0x5c), x87_store32(hx.abs()));
            let ahx = lf(e, 0x5c);
            e.mem.set_u32(at(0x60), x87_store32(hy.abs()));
            let ahy = lf(e, 0x60);
            let mut largest = ahx;
            if ahx < ahy {
                e.mem.set_u32(at(0x5c), x87_store32(ahy));
                largest = lf(e, 0x5c);
            }
            e.mem.set_u32(at(0x5c), x87_store32(hz.abs()));
            let ahz = lf(e, 0x5c);
            // `FCOM; TEST AH,0x41; JNZ`: replaced only when strictly above.
            if ahz > largest {
                e.mem.set_u32(at(0x5c), x87_store32(ahz));
                largest = lf(e, 0x5c);
            }
            let nudge = x87_load32(e.global::<u32>(NUDGE));
            e.mem
                .set_u32(at(0x5c), x87_store32(x87_mul(largest, nudge)));
            let shift = lf(e, 0x5c);
            // `FLDZ; FCOMP dist; TEST AH,0x41; JNZ`: added unless the
            // distance is below 0.
            let towards_positive = !(0.0 > distance);
            let (kx, ky, kz) = if towards_positive {
                (
                    x87_mul(load(e, a + 0x58), shift),
                    x87_mul(load(e, a + 0x5c), shift),
                    x87_mul(shift, operand(e, a + 0x60)),
                )
            } else {
                (
                    x87_mul(shift, operand(e, a + 0x58)),
                    x87_mul(load(e, a + 0x5c), shift),
                    x87_mul(shift, operand(e, a + 0x60)),
                )
            };
            e.mem.set_u32(at(0x24), x87_store32(kx));
            e.mem.set_u32(at(0x28), x87_store32(ky));
            e.mem.set_u32(at(0x2c), x87_store32(kz));
            let (nx, ny, nz) = if towards_positive {
                (
                    x87_add(hx, lf(e, 0x24)),
                    x87_add(hy, mf(e, 0x28)),
                    x87_add(hz, mf(e, 0x2c)),
                )
            } else {
                (
                    x87_sub(hx, lf(e, 0x24)),
                    x87_sub(hy, mf(e, 0x28)),
                    x87_sub(hz, mf(e, 0x2c)),
                )
            };
            e.mem.set_u32(at(0x18), x87_store32(nx));
            e.mem.set_u32(at(0x1c), x87_store32(ny));
            e.mem.set_u32(at(0x20), x87_store32(nz));
            hz = lf(e, 0x20);
            hy = lf(e, 0x1c);
            hx = lf(e, 0x18);
        }
        e.mem
            .set_u32(at(0x24), x87_store32(x87_sub(hx, load(e, a + 0x68))));
        e.mem
            .set_u32(at(0x28), x87_store32(x87_sub(hy, operand(e, a + 0x6c))));
        e.mem
            .set_u32(at(0x2c), x87_store32(x87_sub(hz, operand(e, a + 0x70))));
        e.call(MATRIX_TIMES_POINT, &args![a + 0x74, at(0x3c), a + 0x38]);
        e.call(MATRIX_TIMES_POINT, &args![a + 0x74, at(0x48), a + 0x44]);
        let (rx, ry) = (lf(e, 0x24), lf(e, 0x28));
        let (axx, axy, axz) = (lf(e, 0x3c), lf(e, 0x40), lf(e, 0x44));
        let s = x87_add(x87_mul(axy, ry), x87_mul(rx, axx));
        let u = x87_add(s, x87_mul(axz, mf(e, 0x2c)));
        e.mem.set_u32(at(0x5c), x87_store32(u));
        let u = lf(e, 0x5c);
        e.mem.set_u32(at(0x30), x87_store32(x87_mul(axx, u)));
        e.mem.set_u32(at(0x34), x87_store32(x87_mul(axy, u)));
        e.mem.set_u32(at(0x38), x87_store32(x87_mul(u, axz)));
        let (ayx, ayy, ayz) = (lf(e, 0x48), lf(e, 0x4c), lf(e, 0x50));
        let s = x87_add(x87_mul(ry, ayy), x87_mul(rx, ayx));
        let v = x87_add(s, x87_mul(ayz, mf(e, 0x2c)));
        e.mem.set_u32(at(0x5c), x87_store32(v));
        let v = lf(e, 0x5c);
        e.mem.set_u32(at(0x24), x87_store32(x87_mul(ayx, v)));
        e.mem.set_u32(at(0x28), x87_store32(x87_mul(ayy, v)));
        e.mem.set_u32(at(0x2c), x87_store32(x87_mul(v, ayz)));
        let (px, py, pz) = (lf(e, 0x30), lf(e, 0x34), lf(e, 0x38));
        let squared = x87_add(x87_add(x87_mul(py, py), x87_mul(px, px)), x87_mul(pz, pz));
        e.mem.set_u32(at(0x5c), x87_store32(squared));
        if !(half_width(e) > lf(e, 0x5c)) {
            return (false, e.mem.u8(at(0x13)));
        }
        let (qx, qy, qz) = (lf(e, 0x24), lf(e, 0x28), lf(e, 0x2c));
        let squared = x87_add(x87_add(x87_mul(qy, qy), x87_mul(qx, qx)), x87_mul(qz, qz));
        e.mem.set_u32(at(0x5c), x87_store32(squared));
        if !(half_height(e) > lf(e, 0x5c)) {
            return (false, e.mem.u8(at(0x13)));
        }
        let when = x87_add(lf(e, 0x14), mf(e, 0x58));
        e.mem.set_u8(at(0x13), 1);
        e.mem.set_u32(at(0x5c), x87_store32(when));
        let when = x87_store32(lf(e, 0x5c));
        e.mem.set_u32(end_time.addr(), when);
        copy_point(e, at(0x18), a + 0x14);
        e.mem.set_u32(a + 0x20, when);
        (false, e.mem.u8(at(0x13)))
    });
    // Collided on the plane: answered at once.
    if on_plane {
        return a;
    }
    let start = x87_quiet_bits(time.to_bits());
    let answer = e
        .call(COLLIDER_RESOLVE, &args![this, start, end_time, data, index])
        .u32();
    if answer != 0 {
        return answer;
    }
    // `NEG; SBB; AND`: `this` when this collider was hit, else 0.
    if collided != 0 {
        a
    } else {
        0
    }
}

/// Copies the three words of a point.
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for k in 0..3 {
        let word = e.mem.u32(from + 4 * k);
        e.mem.set_u32(to + 4 * k, word);
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_8d10,
            ni_p_sys_planar_collider_resolve(Ptr, f32, Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00c2_9320,
            ni_p_sys_planar_collider_update(Ptr, f32, Ptr, u32)
        ),
        entry!(
            0x00c2_94b0,
            ni_p_sys_planar_collider_set_resolve_constants(Ptr)
        ),
        entry!(0x00c2_97f0, ni_p_sys_planar_collider_save_binary(Ptr, Ptr)),
        entry!(0x00c2_9870, ni_p_sys_planar_collider_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_9b70,
            ni_p_sys_planar_collider_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c2_7ac0, ni_p_sys_planar_collider_link_object(Ptr, Ptr)),
        entry!(
            0x00c2_9450,
            ni_p_sys_planar_collider_process_clone(Ptr, Ptr)
        ),
        entry!(0x00c2_9770, ni_p_sys_planar_collider_load_binary(Ptr, Ptr)),
        entry!(
            0x00c2_9900,
            ni_p_sys_planar_collider_ni_p_sys_planar_collider(
                Ptr, f32, u32, u32, u32, u32, f32, f32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00c2_99e0,
            ni_p_sys_planar_collider_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_9a30, ni_p_sys_planar_collider_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_9af0, fn_00c29af0() -> Ptr),
        entry!(0x00c299d0, ni_p_sys_planar_collider_get_r_t_t_i() -> u32),
        entry!(0x00c2_8d00, ni_p_sys_planar_collider_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c28d00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_8d00;
        const TARGET: u32 = 0x00c3_2680;
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
    fn ni_p_sys_planar_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_99d0, &args![]).u32(), 0x0120_26c8);
    }
}

#[cfg(test)]
mod planar_tests {
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

    #[test]
    fn link_object_takes_the_collider_object() {
        let mut e = Engine::new();
        e.register(COLLIDER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| word(0x4321));
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7ac0, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, COLLIDER_LINK_OBJECT), vec![vec![this, 0x6000]]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x4321);
    }

    #[test]
    fn process_clone_maps_the_collider_object() {
        let mut e = Engine::new();
        e.register(COLLIDER_PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let clone = e.mem.alloc(0x100);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = match a[1] {
                k if k == this => clone,
                0x4440 => 0x4441,
                _ => return word(0),
            };
            e.mem.set_u32(a[2], found);
            word(1)
        });
        let process = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        // No object: one lookup only.
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(calls_to(&e, CLONE_MAP_LOOKUP).len(), 1);
        e.mem.set_u32(this + 0x2c, 0x4440);
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(e.mem.u32(clone + 0x2c), 0x4441);
        e.mem.set_u32(this + 0x2c, 0x5550);
        e.call(0x00c2_9450, &args![this, process]);
        assert_eq!(e.mem.u32(clone + 0x2c), 0x5550);
    }

    #[test]
    fn load_binary_reads_size_and_axes() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(COLLIDER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(THREE_FLOATS_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            e.mem.set_f32(a[1], 2.0);
            word(4)
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_9770, &args![this, stream]);
        assert_eq!(e.mem.f32(this + 0x30), 2.0);
        assert_eq!(e.mem.f32(this + 0x34), 2.0);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_LOAD_BINARY),
            vec![vec![this + 0x38, stream], vec![this + 0x44, stream]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
    }

    fn constructor_doubles(e: &mut Engine) {
        e.register(COLLIDER_CONSTRUCT, |_, a| word(a[0]));
        e.register(PLANE_CONSTRUCT, |_, _| Ret::default());
        e.register(TRANSFORM_MAKE_IDENTITY, |_, _| Ret::default());
        for addr in [
            HALF_FLOAT,
            ORIGIN,
            IDENTITY_ROTATION,
            FIRST_AXIS,
            SECOND_AXIS,
        ] {
            e.map(addr, 0x24);
        }
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(IDENTITY_ROTATION, 1.0);
        e.mem.set_f32(FIRST_AXIS, 1.0);
        e.mem.set_f32(SECOND_AXIS + 4, 1.0);
    }

    #[test]
    fn constructor_lays_out_the_planar_collider() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let this = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0x7f80_0001u32,
            1u32,
            0u32,
            0x44u32,
            0x55u32,
            2.0f32,
            3.0f32,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32
        ];
        assert_eq!(e.call(0x00c2_9900, &words).u32(), this);
        assert_eq!(
            calls_to(&e, COLLIDER_CONSTRUCT),
            vec![vec![this, 0x7fc0_0001, 1, 0, 0x44]]
        );
        assert_eq!(
            calls_to(&e, PLANE_CONSTRUCT),
            vec![vec![this + 0x58, ORIGIN, 0]]
        );
        assert_eq!(
            calls_to(&e, TRANSFORM_MAKE_IDENTITY),
            vec![vec![this + 0x98], vec![this + 0xcc]]
        );
        assert_eq!(e.mem.u32(this), PLANAR_COLLIDER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x2c), 0x55);
        assert_eq!(e.mem.f32(this + 0x30), 2.0);
        assert_eq!(e.mem.f32(this + 0x34), 3.0);
        let axes: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 0x38 + 4 * i)).collect();
        assert_eq!(axes, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(e.mem.f32(this + 0x50), 0.5);
        assert_eq!(e.mem.f32(this + 0x54), 0.5);
        assert_eq!(e.mem.f32(this + 0x74), 1.0);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_next_collider() {
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let next = e.mem.alloc(8);
        e.mem.set_u32(next, vtable);
        e.mem.set_u32(next + 4, 1);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x28, next);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_99e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![next]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0x100]]);
        e.mem.set_u32(this + 0x28, 0);
        e.call(0x00c2_99e0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn create_clone_and_create_function_build_the_default_collider() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(COLLIDER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x30, 0x7f80_0003);
        e.mem.set_u32(this + 0x4c, 0x99);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_9a30, &args![this, 0x7000u32]).u32();
        assert_eq!(
            calls_to(&e, COLLIDER_CONSTRUCT),
            vec![vec![clone, 0x3f80_0000, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, COLLIDER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone + 0x30), 0x7fc0_0003);
        assert_eq!(e.mem.u32(clone + 0x4c), 0x99);
        let made = e.call(0x00c2_9af0, &args![]).u32();
        assert_eq!(e.mem.u32(made), PLANAR_COLLIDER_VTABLE);
        assert_eq!(e.mem.f32(made + 0x38), 1.0);
        assert_eq!(e.mem.f32(made + 0x48), 1.0);
        assert_eq!(e.mem.f32(made + 0x34), 1.0);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_9af0, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod collide_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn f(v: f32) -> u32 {
        v.to_bits()
    }

    /// A collider whose plane normal is `(0, 1, 0)`, bounce 0.5, collision
    /// time 2.0, and particle data with one particle moving `(1, -2, 0)`.
    fn update_setup(e: &mut Engine) -> (u32, u32) {
        e.map(EPSILON & !0xfff, 0x1000);
        e.mem.set_u32(EPSILON, 0x3a83_126f);
        e.register(COLLIDER_UPDATE, |_, _| Ret::default());
        let this = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        e.mem.set_u32(this + 0x8, f(0.5));
        e.mem.set_u32(this + 0x20, f(2.0));
        e.mem.set_u32(this + 0x5c, f(1.0));
        let data = e.mem.alloc(0x80);
        let infos = e.mem.alloc(2 * INFO_SIZE);
        e.mem.set_u32(data + DATA_PARTICLE_INFO, infos);
        let info = infos + INFO_SIZE;
        e.mem.set_u32(info, f(1.0));
        e.mem.set_u32(info + 4, f(-2.0));
        (this, data)
    }

    fn velocity(e: &Engine, data: u32) -> [u32; 3] {
        let info = e.mem.u32(data + DATA_PARTICLE_INFO) + INFO_SIZE;
        [e.mem.u32(info), e.mem.u32(info + 4), e.mem.u32(info + 8)]
    }

    #[test]
    fn update_reflects_and_bounces_the_velocity() {
        let mut e = Engine::new();
        let (this, data) = update_setup(&mut e);
        e.call_log = Some(vec![]);
        // Index 1 in the low half; the whole word goes on to the base.
        e.call(0x00c2_9320, &args![this, 1.0f32, data, 0x0005_0001u32]);
        // (1, -2, 0) - 2 * (0, -2, 0) = (1, 2, 0), times 0.5.
        assert_eq!(velocity(&e, data), [f(0.5), f(1.0), 0]);
        assert_eq!(
            calls_to(&e, COLLIDER_UPDATE),
            vec![vec![this, f(1.0), data, 0x0005_0001]]
        );
    }

    #[test]
    fn update_at_the_collision_time_pushes_out_a_slow_particle_only() {
        let mut e = Engine::new();
        let (this, data) = update_setup(&mut e);
        // d = -2 < 0.001: only the normal component is removed.
        e.call(0x00c2_9320, &args![this, 2.0f32, data, 1u32]);
        assert_eq!(velocity(&e, data), [f(1.0), 0, 0]);
        // Moving away (d = 3 >= 0.001): reflected and bounced.
        let info = e.mem.u32(data + DATA_PARTICLE_INFO) + INFO_SIZE;
        e.mem.set_u32(info + 4, f(3.0));
        e.call(0x00c2_9320, &args![this, 2.0f32, data, 1u32]);
        assert_eq!(velocity(&e, data), [f(0.5), f(-1.5), 0]);
    }

    /// A collider for `SetResolveConstants`: X axis `(2, 0, 0)`, Y axis
    /// `(0, 0, 3)`, width 4 and height 6, with doubles for its callees; the
    /// unitize double divides by the length.
    fn constants_setup(e: &mut Engine) -> u32 {
        e.map(ORIGIN & !0xfff, 0x1000);
        e.map(IDENTITY_ROTATION & !0xfff, 0x1000);
        for i in 0..9 {
            let v = if i % 4 == 0 { f(1.0) } else { 0 };
            e.mem.set_u32(IDENTITY_ROTATION + 4 * i, v);
        }
        e.map(HALF_DOUBLE & !0xfff, 0x1000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.register(PLANE_CONSTRUCT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            e.mem.set_u32(a[0] + 12, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(PLANE_FROM_NORMAL_AND_POINT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            e.mem.set_u32(a[0] + 12, f(7.0));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT_UNITIZE, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, c) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, c / length);
            }
            Ret::default()
        });
        e.register(TRANSFORM_NOT_EQUAL, |_, _| Ret::default());
        let this = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        e.mem.set_u32(this + 0x38, f(2.0));
        e.mem.set_u32(this + 0x4c, f(3.0));
        e.mem.set_u32(this + 0x30, f(4.0));
        e.mem.set_u32(this + 0x34, f(6.0));
        this
    }

    #[test]
    fn set_resolve_constants_builds_the_plane_without_an_object() {
        let mut e = Engine::new();
        let this = constants_setup(&mut e);
        e.call(0x00c2_94b0, &args![this]);
        // X x Y = (0, -6, 0), normalized: the plane's normal.
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.u32(this + 0x5c), f(-1.0));
        assert_eq!(e.mem.u32(this + 0x64), f(7.0));
        assert_eq!(e.mem.u32(this + 0x50), f(4.0));
        assert_eq!(e.mem.u32(this + 0x54), f(9.0));
        assert_eq!(e.mem.u32(this + 0x74), f(1.0));
    }

    #[test]
    fn set_resolve_constants_uses_the_object_transform_and_chains() {
        const NEXT_SET: u32 = 0x0f70_0000;
        let mut e = Engine::new();
        let this = constants_setup(&mut e);
        // A plane already built: recomputed only when a transform moved.
        e.mem.set_u32(this + 0x5c, f(1.0));
        e.call_log = Some(vec![]);
        e.call(0x00c2_94b0, &args![this]);
        assert!(calls_to(&e, PLANE_FROM_NORMAL_AND_POINT).is_empty());
        let object = e.mem.alloc(0x100);
        let system = e.mem.alloc(0x100);
        let manager = e.mem.alloc(0x20);
        e.mem.set_u32(manager + MANAGER_SYSTEM, system);
        e.mem.set_u32(this + COLLIDER_MANAGER, manager);
        e.mem.set_u32(this + 0x2c, object);
        e.mem.set_u32(object + WORLD_TRANSFORM + 0x30, f(2.0));
        e.register_double(TRANSFORM_NOT_EQUAL, move |_, a| Ret {
            eax: u32::from(a[0] == this + 0x98),
            ..Ret::default()
        });
        e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        // The relative transform: identity rotation, translation (1, 2, 3),
        // scale 2.
        e.register(TRANSFORM_PRODUCT, |e, a| {
            for i in 0..13 {
                e.mem.set_u32(a[1] + 4 * i, 0);
            }
            for i in [0, 4, 8] {
                e.mem.set_f32(a[1] + 4 * i, 1.0);
            }
            for (i, v) in [1.0f32, 2.0, 3.0, 2.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 0x24 + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(NEXT_SET, |_, _| Ret::default());
        let next = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(next, vtable);
        e.mem.set_u32(vtable + SET_RESOLVE_CONSTANTS_SLOT, NEXT_SET);
        e.mem.set_u32(this + COLLIDER_NEXT, next);
        e.call_log = Some(vec![]);
        e.call(0x00c2_94b0, &args![this]);
        assert_eq!(
            calls_to(&e, TRANSFORM_NOT_EQUAL)[0],
            vec![this + 0x98, object + WORLD_TRANSFORM]
        );
        assert_eq!(e.mem.u32(this + 0x68), f(1.0));
        assert_eq!(e.mem.u32(this + 0x70), f(3.0));
        // The object's world transform is kept.
        assert_eq!(e.mem.u32(this + 0x98 + 0x30), f(2.0));
        // Scale 2: half widths 4 and 6, squared.
        assert_eq!(e.mem.u32(this + 0x50), f(16.0));
        assert_eq!(e.mem.u32(this + 0x54), f(36.0));
        assert_eq!(calls_to(&e, PLANE_FROM_NORMAL_AND_POINT)[0][2], this + 0x68);
        assert_eq!(calls_to(&e, NEXT_SET), vec![vec![next]]);
    }

    #[test]
    fn save_binary_writes_link_sizes_and_axes() {
        const SAVE_LINK: u32 = 0x0f70_0010;
        let mut e = Engine::new();
        e.register(COLLIDER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        e.register(POINT_SAVE_BINARY, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + STREAM_SAVE_LINK_ID, SAVE_LINK);
        let this = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        e.mem.set_u32(this + 0x2c, 0x7700);
        e.mem.set_u32(this + 0x30, f(4.0));
        e.mem.set_u32(this + 0x34, f(6.0));
        e.call_log = Some(vec![]);
        e.call(0x00c2_97f0, &args![this, stream]);
        assert_eq!(calls_to(&e, COLLIDER_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x7700]]);
        assert_eq!(
            *writes.borrow(),
            vec![
                (4.0f32.to_le_bytes().to_vec(), 4),
                (6.0f32.to_le_bytes().to_vec(), 4)
            ]
        );
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this + 0x38, stream], vec![this + 0x44, stream]]
        );
    }

    #[test]
    fn is_equal_checks_object_sizes_and_axes() {
        let mut e = Engine::new();
        e.register(COLLIDER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(POINT_NOT_EQUAL, |_, _| Ret::default());
        let a = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        let b = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_9870, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, POINT_NOT_EQUAL),
            vec![vec![a + 0x38, b + 0x38], vec![a + 0x44, b + 0x44]]
        );
        e.mem.set_u32(b + 0x34, 0x7fc0_0000);
        assert!(!e.call(0x00c2_9870, &args![a, b]).bool());
        e.mem.set_u32(b + 0x34, 0);
        e.mem.set_u32(a + 0x2c, 0x9000);
        assert!(!e.call(0x00c2_9870, &args![a, b]).bool());
        e.mem.set_u32(a + 0x2c, 0);
        e.register_double(POINT_NOT_EQUAL, move |_, p| Ret {
            eax: u32::from(p[0] == a + 0x44),
            ..Ret::default()
        });
        assert!(!e.call(0x00c2_9870, &args![a, b]).bool());
        e.register(COLLIDER_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c2_9870, &args![a, b]).bool());
    }

    #[test]
    fn get_viewer_strings_appends_object_sizes_and_axes() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_000b);
        e.register(COLLIDER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(POINT_GET_VIEWER_STRING, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        e.mem.set_u32(this + 0x30, f(4.0));
        e.call_log = Some(vec![]);
        e.call(0x00c2_9b70, &args![this, strings]);
        assert_eq!(
            *stored.borrow(),
            vec![
                0x0abc_000b,
                LABEL_COLLIDER_OBJECT,
                LABEL_WIDTH,
                LABEL_HEIGHT,
                LABEL_X_AXIS + 1,
                LABEL_Y_AXIS + 1
            ]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_COLLIDER_OBJECT, TEXT_NONE]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT)[0],
            vec![LABEL_WIDTH, f(4.0)]
        );
    }
}

#[cfg(test)]
mod resolve_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    /// A collider with plane normal `normal`, constant 0, position 0, X axis
    /// `(1, 0, 0)` and Y axis `(0, 0, 1)` (the rotation is the identity: the
    /// matrix double copies the point), half sizes squared `half_squared`;
    /// particle data with one particle at `position` moving `velocity`.
    fn setup(
        e: &mut Engine,
        normal: [f32; 3],
        half_squared: f32,
        position: [f32; 3],
        velocity: [f32; 3],
    ) -> (u32, u32, u32) {
        for addr in [EPSILON, ZERO_DOUBLE, NUDGE, NUDGE_BASE, NUDGE_EXPONENT] {
            e.map(addr & !0xfff, 0x1000);
        }
        e.mem.set_u32(EPSILON, 0x3a83_126f);
        e.mem.set_f64(NUDGE_BASE, 2.0);
        e.mem.set_f64(NUDGE_EXPONENT, -21.0);
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for k in 0..3 {
                let word = e.mem.u32(a[2] + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(POW, |_, a| {
            let base = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let exponent = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: base.powf(exponent),
                ..Ret::default()
            }
        });
        e.register(COLLIDER_RESOLVE, |_, _| Ret::default());
        let this = e.mem.alloc(PLANAR_COLLIDER_SIZE);
        for (k, v) in normal.iter().enumerate() {
            e.mem.set_f32(this + 0x58 + 4 * k as u32, *v);
        }
        e.mem.set_f32(this + 0x38, 1.0);
        e.mem.set_f32(this + 0x4c, 1.0);
        e.mem.set_f32(this + 0x50, half_squared);
        e.mem.set_f32(this + 0x54, half_squared);
        let data = e.mem.alloc(0x80);
        let vertices = e.mem.alloc(12);
        let infos = e.mem.alloc(INFO_SIZE);
        for k in 0..3 {
            e.mem.set_f32(vertices + 4 * k, position[k as usize]);
            e.mem.set_f32(infos + 4 * k, velocity[k as usize]);
        }
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        e.mem.set_u32(data + DATA_PARTICLE_INFO, infos);
        let end = e.mem.alloc(4);
        e.mem.set_f32(end, 1.0);
        (this, data, end)
    }

    #[test]
    fn resolve_on_the_plane_collides_at_once() {
        let mut e = Engine::new();
        let (this, data, end) = setup(&mut e, [0.0, 1.0, 0.0], 4.0, [1.0, 0.0005, 1.0], [0.0; 3]);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00c2_8d10,
                &args![this, 0.25f32, end, data, 0x0003_0000u32],
            )
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.f32(end), 0.25);
        assert_eq!(e.mem.f32(this + 0x20), 0.25);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.f32(this + 0x18), 0.0005);
        assert!(calls_to(&e, COLLIDER_RESOLVE).is_empty());
        // Outside the rectangle: the base resolves.
        e.mem.set_f32(this + 0x50, 1.0);
        e.mem.set_f32(end, 1.0);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00c2_8d10,
                &args![this, 0.25f32, end, data, 0x0003_0000u32],
            )
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(
            calls_to(&e, COLLIDER_RESOLVE),
            vec![vec![this, 0.25f32.to_bits(), end, data, 0x0003_0000]]
        );
        assert_eq!(e.mem.f32(end), 1.0);
    }

    #[test]
    fn resolve_across_the_step_collides_at_the_crossing() {
        let mut e = Engine::new();
        let (this, data, end) = setup(
            &mut e,
            [0.0, 1.0, 0.0],
            4.0,
            [1.0, 1.0, 0.0],
            [0.0, -4.0, 0.0],
        );
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00c2_8d10, &args![this, 0.0f32, end, data, 0u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.f32(end), 0.25);
        assert_eq!(e.mem.f32(this + 0x20), 0.25);
        assert_eq!(
            [
                e.mem.f32(this + 0x14),
                e.mem.f32(this + 0x18),
                e.mem.f32(this + 0x1c)
            ],
            [1.0, 0.0, 0.0]
        );
        // The static nudge is made once.
        assert_eq!(calls_to(&e, POW), vec![args![2.0f64, -21.0f64]]);
        assert_eq!(e.mem.f32(NUDGE), 2.0f32.powi(-21));
        assert_eq!(e.mem.u8(NUDGE_GUARD) & 1, 1);
        // The base's answer wins; the static is not made again.
        e.register(COLLIDER_RESOLVE, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.mem.set_f32(end, 1.0);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00c2_8d10, &args![this, 0.0f32, end, data, 0u32])
            .u32();
        assert_eq!(answer, 0x1234);
        assert!(calls_to(&e, POW).is_empty());
    }

    #[test]
    fn resolve_without_crossing_or_outside_defers_to_the_base() {
        let mut e = Engine::new();
        // Moving away from the plane.
        let (this, data, end) = setup(
            &mut e,
            [0.0, 1.0, 0.0],
            4.0,
            [1.0, 1.0, 0.0],
            [0.0, 4.0, 0.0],
        );
        assert_eq!(
            e.call(0x00c2_8d10, &args![this, 0.0f32, end, data, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.f32(end), 1.0);
        // Crossing outside the rectangle.
        let (this, data, end) = setup(
            &mut e,
            [0.0, 1.0, 0.0],
            4.0,
            [3.0, 1.0, 0.0],
            [0.0, -4.0, 0.0],
        );
        assert_eq!(
            e.call(0x00c2_8d10, &args![this, 0.0f32, end, data, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.f32(end), 1.0);
    }

    #[test]
    fn resolve_nudges_a_point_left_on_the_far_side_back() {
        // With a huge nudge any point the crossing leaves on the far side
        // moves far back towards the start side; with a zero nudge it stays.
        let mut nudged = 0;
        for i in 1..16 {
            for j in 1..6 {
                let start = [0.37 * i as f32, 0.5 + 0.11 * i as f32, 0.0];
                let velocity = [-0.29 * j as f32, -1.3 - 0.17 * j as f32, 0.0];
                let mut points = Vec::new();
                for nudge in [0.0f32, 1000.0] {
                    let mut e = Engine::new();
                    let (this, data, end) = setup(&mut e, [0.6, 0.8, 0.0], 1e30, start, velocity);
                    e.mem.set_u32(NUDGE_GUARD, 1);
                    e.mem.set_f32(NUDGE, nudge);
                    let answer = e
                        .call(0x00c2_8d10, &args![this, 0.0f32, end, data, 0u32])
                        .u32();
                    let point = [
                        e.mem.f32(this + 0x14),
                        e.mem.f32(this + 0x18),
                        e.mem.f32(this + 0x1c),
                    ];
                    points.push((answer, point));
                }
                if points[0] != points[1] {
                    nudged += 1;
                    // The start side is the positive one: moved along +n.
                    let side = |p: [f32; 3]| 0.6 * p[0] + 0.8 * p[1];
                    assert!(side(points[1].1) > side(points[0].1) + 1.0);
                }
            }
        }
        assert!(nudged > 0);
    }
}
