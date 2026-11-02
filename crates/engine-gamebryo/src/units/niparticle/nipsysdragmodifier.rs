//! `NiParticle/nipsysdragmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c23cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_drag_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

// Translated from 00c2c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2768.
pub fn ni_p_sys_drag_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2768
}

use crate::units::niparticle::nipsysmodifier::{stream_read, READ_LINK_ID};

layout! {
    /// `NiPSysDragModifier` (Xbox PDB), 0x34 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysDragModifier: 0x34 {
        /// `m_pkDragObj` (Xbox PDB): `NiAVObject*`.
        0x18 m_pkDragObj: u32,
        /// `m_kDragAxis` (Xbox PDB): an `NiPoint3`.
        0x1C m_kDragAxis: u32,
        /// `m_fPercentage` (Xbox PDB).
        0x28 m_fPercentage: u32,
        /// `m_fRange` (Xbox PDB).
        0x2C m_fRange: u32,
        /// `m_fFalloff` (Xbox PDB).
        0x30 m_fFalloff: u32,
    }
}

/// The vtable of `NiPSysDragModifier` (RTTI `.?AVNiPSysDragModifier@@`,
/// `010bfe44`).
const VTABLE: u32 = 0x010b_fe44;
/// The constructor's defaults: the axis (three `float`s at `011a946c`,
/// copied by `MOV`), the percentage (the `float` at `010181bc`, 0.05) and
/// the range and falloff (the `float` at `010bfe38`, `FLT_MAX`), the two
/// floats through the x87 stack.
const DEFAULT_AXIS: u32 = 0x011a_946c;
const DEFAULT_PERCENTAGE: u32 = 0x0101_81bc;
const DEFAULT_RANGE: u32 = 0x010b_fe38;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`) and `LoadBinary`
/// (`00c318b0`); `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the
/// colour, the stream) reads the axis.
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00c2c030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project: the `NiPSysDragModifier`
/// constructor (`thiscall`; the stream factory `00c2c080` jumps to it). The
/// `NiPSysModifier` constructor, the vtable `010bfe44`, no drag object, the
/// axis from `011a946c`, the percentage from `010181bc`, the range and the
/// falloff from `010bfe38`. Returns `this`.
pub fn fn_00c2c030(e: &mut Engine, this: Ptr<NiPSysDragModifier>) -> Ptr<NiPSysDragModifier> {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    let percentage = x87_quiet_bits(e.global::<u32>(DEFAULT_PERCENTAGE));
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiPSysDragModifier::m_pkDragObj, 0);
    for word in 0..3 {
        let value = e.global::<u32>(DEFAULT_AXIS + 4 * word);
        e.mem.set_u32(
            this.addr() + NiPSysDragModifier::m_kDragAxis.off + 4 * word,
            value,
        );
    }
    e.set(this, NiPSysDragModifier::m_fPercentage, percentage);
    let range = x87_quiet_bits(e.global::<u32>(DEFAULT_RANGE));
    e.set(this, NiPSysDragModifier::m_fRange, range);
    e.set(this, NiPSysDragModifier::m_fFalloff, range);
    this
}

// Translated from 00c2c080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1ab0a`): the `NiPSysDragModifier` stream factory (`cdecl`, no
/// arguments): a 0x34-byte object from `00aa13e0` given to the constructor
/// `00c2c030` (a tail jump); 0 when the allocation fails.
pub fn fn_00c2c080(e: &mut Engine) -> Ptr<NiPSysDragModifier> {
    let object = Ptr::<NiPSysDragModifier>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if object.is_null() {
        return Ptr::NULL;
    }
    fn_00c2c030(e, object)
}

// Translated from 00c2c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, a link id (`00a645b0`, the drag object), the axis
/// (`NiColor::LoadBinary`), then the percentage, range and falloff (4 bytes
/// each; the component-size word is the stream argument's stack word).
pub fn ni_p_sys_drag_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.call(READ_LINK_ID, &args![s]);
    e.call(
        COLOR_LOAD_BINARY,
        &args![this.addr() + NiPSysDragModifier::m_kDragAxis.off, s],
    );
    e.with_stack(4, |e, slot| {
        for field in [
            NiPSysDragModifier::m_fPercentage,
            NiPSysDragModifier::m_fRange,
            NiPSysDragModifier::m_fFalloff,
        ] {
            stream_read(e, s, this.addr() + field.off, 4, 4, slot.addr());
        }
    });
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};

/// `NiPSysModifier`'s member copy (`00c31780`, `thiscall`: clone, process),
/// `SaveBinary` (`00c31950`), `IsEqual` (`00c31830`) and `GetViewerStrings`
/// (`00c319e0`) (Xbox PDB names but the first; `thiscall`, one word).
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `NiColor::SaveBinary` (`00a69780`, Xbox PDB name; `thiscall` on the
/// three floats, the stream) writes the axis; `NiPoint3::GetViewerString`
/// (`00a59af0`, Xbox PDB name; `thiscall` on the point, a label) names it.
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `00439090` (unnamed; `thiscall` on a point, the other point; a nonzero
/// answer counts as unequal here).
const POINT_UNEQUAL: u32 = 0x0043_9090;
/// `NiStream::SaveLinkID` (`+0x30`) and `NiObject::IsEqual` (`+0x5C`) slots
/// (Xbox PDB).
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
const IS_EQUAL_SLOT: u32 = 0x5c;
/// `NiTransform::InvertNonUniform` (`004b4880`, Xbox PDB name; `thiscall`
/// on a transform, the result), `0062c250` (unnamed; `thiscall` on a
/// transform: result, right-hand transform; the product), `004b4500`
/// (unnamed; `thiscall` on a transform: result, point; returns the result:
/// the point turned by the transform) and `004a0c10` (unnamed; `thiscall`
/// on a point, no words; it normalizes the point, by its use here).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
const TRANSFORM_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// The CRT square root (`00ec6040`: the `ST0` value as a leading `f64`,
/// the root back in `ST0`).
const SQRT: u32 = 0x00ec_6040;
/// The `double`s the update divides the elapsed time by (`010bfee8`) and
/// compares the scaled strength with (`01012070`).
const TIME_STEP: u32 = 0x010b_fee8;
const ONE: u32 = 0x0101_2070;
/// `NiAVObject`'s world transform (`m_kWorld`, +0x68 as the PC code reads
/// it, 0x34 bytes) and `NiObjectNET`'s name (+8).
const WORLD_TRANSFORM: u32 = 0x68;
const TRANSFORM_SIZE: u32 = 0x34;
const OBJECT_NAME: u32 = 0x8;
/// `NiPSysModifier::m_pkTarget` (Xbox PDB, +0x10): the particle system.
const MODIFIER_TARGET: u32 = 0x10;
/// The particle data (`NiPSysData`) fields the update reads: the active
/// count (+0x50), the positions (+0x20, 12 bytes each) and the particle
/// infos (+0x70, 0x1c bytes each: velocity at +0, last update at +0x14).
const DATA_ACTIVE: u32 = 0x50;
const DATA_VERTICES: u32 = 0x20;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;
/// `NiGetViewerString` (`00a7d330`, the text), `_ov9` (`00a7d550`: a label
/// and a `float`) and `_ov11` (`00a7d5e0`: a label and a text) (Xbox PDB
/// names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `NiPSysDragModifier::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2768;
/// The labels "Drag Object" (`010bff18`), "Drag Axis" (`010bff0c`),
/// "Percentage" (`010bff00`), "Range" (`01062600`) and "RangeFalloff"
/// (`010bfef0`), and "None" (`0102909c`) for no drag object.
const LABEL_DRAG_OBJECT: u32 = 0x010b_ff18;
const LABEL_DRAG_AXIS: u32 = 0x010b_ff0c;
const LABEL_PERCENTAGE: u32 = 0x010b_ff00;
const LABEL_RANGE: u32 = 0x0106_2600;
const LABEL_FALLOFF: u32 = 0x010b_fef0;
const TEXT_NONE: u32 = 0x0102_909c;

// Translated from 00c2c0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::CreateClone` (Xbox PDB): a 0x34-byte object from
/// `00aa13e0` through the constructor `00c2c030` (null when the allocation
/// fails), `NiPSysModifier`'s member copy into it, then the axis (by `MOV`)
/// and the percentage, range and falloff (through the x87 stack). The drag
/// object is not copied here. Returns the clone.
pub fn ni_p_sys_drag_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    process: Ptr,
) -> Ptr<NiPSysDragModifier> {
    let block = e.call(NI_NEW, &args![0x34u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00c2c030(e, Ptr::new(block))
    };
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let axis = NiPSysDragModifier::m_kDragAxis.off;
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + axis + 4 * word);
        e.mem.set_u32(clone.addr() + axis + 4 * word, value);
    }
    for field in [
        NiPSysDragModifier::m_fPercentage,
        NiPSysDragModifier::m_fRange,
        NiPSysDragModifier::m_fFalloff,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    }
    clone
}

/// `(y * y + x * x) + z * z` as the update computes squared lengths
/// (`FMULP`, `FADDP`, `FMUL ST0`; one call per instruction).
fn squared_length(v: [f64; 3]) -> f64 {
    let xx = x87_mul(v[0], v[0]);
    let yy = x87_mul(v[1], v[1]);
    let zz = x87_mul(v[2], v[2]);
    x87_add(x87_add(yy, xx), zz)
}

// Translated from 00c2c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::Update` (Xbox PDB): with a positive percentage,
/// active particles and a drag object: the drag axis in the particle
/// system's space (the inverse of the system's world transform times the
/// drag object's, applied to the axis, normalized; its squared length
/// kept), then for each active particle: its distance from the drag
/// object's position in that space; the strength is the percentage, scaled
/// by `1 - (distance - range) / (falloff - range)` beyond the range, and
/// the particle is skipped at or beyond the falloff; the elapsed time is
/// `(time - last update) / 0.0333..` (the `double` at `010bfee8`). The
/// velocity loses `strength * elapsed` of its part along the axis (all of
/// it when that product is above 1.0): `v += axis * (dot / -|axis|^2 * ..)`.
/// The x87 arithmetic is done instruction by instruction in the exe's order.
pub fn ni_p_sys_drag_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    time: f32,
    data: Ptr,
) {
    if !(0.0 < x87_m32(e.get(this, NiPSysDragModifier::m_fPercentage))) {
        return;
    }
    let data = data.addr();
    if e.mem.u16(data + DATA_ACTIVE) == 0 {
        return;
    }
    let drag_object = e.get(this, NiPSysDragModifier::m_pkDragObj);
    if drag_object == 0 {
        return;
    }
    e.with_stack(0x100, |e, locals| {
        // The exe's frame: the drag object's world transform, the system's
        // (later the product), the inverse, and the turned axis.
        let drag_world = locals.addr();
        let system_world = locals.addr() + 0x40;
        let inverse = locals.addr() + 0x80;
        let turned = locals.addr() + 0xc0;
        let axis_slot = locals.addr() + 0xd0;
        let words = e.mem.bytes(drag_object + WORLD_TRANSFORM, TRANSFORM_SIZE);
        e.mem.write(drag_world, &words);
        let target = e.mem.u32(this.addr() + MODIFIER_TARGET);
        let words = e.mem.bytes(target + WORLD_TRANSFORM, TRANSFORM_SIZE);
        e.mem.write(system_world, &words);
        e.call(TRANSFORM_INVERT, &args![system_world, inverse]);
        e.call(
            TRANSFORM_MULTIPLY,
            &args![inverse, system_world, drag_world],
        );
        let axis = this.addr() + NiPSysDragModifier::m_kDragAxis.off;
        let result = e
            .call(TRANSFORM_POINT, &args![system_world, turned, axis])
            .u32();
        let words = e.mem.bytes(result, 12);
        e.mem.write(axis_slot, &words);
        e.call(POINT_UNITIZE, &args![axis_slot]);
        let axis_bits = [
            e.mem.u32(axis_slot),
            e.mem.u32(axis_slot + 4),
            e.mem.u32(axis_slot + 8),
        ];
        let axis = axis_bits.map(x87_load32);
        let length_squared = x87_store32(squared_length(axis));
        let falloff = e.get(this, NiPSysDragModifier::m_fFalloff);
        let range = e.get(this, NiPSysDragModifier::m_fRange);
        let span = x87_store32(x87_sub(x87_load32(falloff), x87_m32(range)));
        // The system's translation in the product (+0x24 of the transform).
        let origin = [
            e.mem.u32(system_world + 0x24),
            e.mem.u32(system_world + 0x28),
            e.mem.u32(system_world + 0x2c),
        ];
        let mut index: u16 = 0;
        while index < e.mem.u16(data + DATA_ACTIVE) {
            let vertex = e
                .mem
                .u32(data + DATA_VERTICES)
                .wrapping_add(u32::from(index) * 12);
            let position = [
                e.mem.u32(vertex),
                e.mem.u32(vertex + 4),
                e.mem.u32(vertex + 8),
            ];
            let info = e
                .mem
                .u32(data + DATA_INFOS)
                .wrapping_add(u32::from(index) * INFO_SIZE);
            let offset: [u32; 3] = std::array::from_fn(|i| {
                x87_store32(x87_sub(x87_load32(position[i]), x87_m32(origin[i])))
            });
            let offset = offset.map(x87_load32);
            let squared = x87_store32(squared_length(offset));
            let root = e.call(SQRT, &args![x87_load32(squared)]).st0;
            let mut distance = x87_store32(x87_load32(x87_store32(root)));
            let mut strength = x87_quiet_bits(e.get(this, NiPSysDragModifier::m_fPercentage));
            let elapsed = x87_sub(
                x87_load32(time.to_bits()),
                x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)),
            );
            let elapsed = x87_store32(x87_div(elapsed, e.global::<f64>(TIME_STEP)));
            let range = e.get(this, NiPSysDragModifier::m_fRange);
            let loaded = x87_load32(distance);
            if x87_load32(range) < loaded {
                let falloff = e.get(this, NiPSysDragModifier::m_fFalloff);
                let falloff = x87_load32(falloff);
                if falloff <= loaded {
                    index = index.wrapping_add(1);
                    continue;
                }
                distance = x87_store32(x87_sub(loaded, x87_m32(range)));
                let ratio = x87_div(x87_load32(distance), x87_m32(span));
                let scale = x87_sub(1.0, ratio);
                strength = x87_store32(x87_mul(scale, x87_m32(strength)));
            }
            // The dot product: `vz * Dz + (vy * Dy + vx * Dx)`.
            let velocity =
                [e.mem.u32(info), e.mem.u32(info + 4), e.mem.u32(info + 8)].map(x87_load32);
            let xy = x87_add(x87_mul(velocity[1], axis[1]), x87_mul(velocity[0], axis[0]));
            let dot = x87_store32(x87_add(x87_mul(velocity[2], axis[2]), xy));
            let elapsed = x87_load32(elapsed);
            let strength = x87_load32(strength);
            let scaled = x87_mul(elapsed, strength);
            let factor = if scaled > e.global::<f64>(ONE) {
                let ratio = x87_div(-x87_load32(dot), x87_m32(length_squared));
                x87_load32(x87_store32(ratio))
            } else {
                let negated = x87_mul(elapsed, -strength);
                let ratio = x87_div(x87_load32(dot), x87_m32(length_squared));
                x87_load32(x87_store32(x87_mul(negated, ratio)))
            };
            let change = [
                x87_store32(x87_mul(axis[0], factor)),
                x87_store32(x87_mul(axis[1], factor)),
                x87_store32(x87_mul(axis[2], factor)),
            ];
            let x = x87_add(x87_load32(change[0]), x87_m32(e.mem.u32(info)));
            e.mem.set_u32(info, x87_store32(x));
            let y = x87_add(x87_load32(e.mem.u32(info + 4)), x87_m32(change[1]));
            e.mem.set_u32(info + 4, x87_store32(y));
            let z = x87_add(x87_load32(e.mem.u32(info + 8)), x87_m32(change[2]));
            e.mem.set_u32(info + 8, x87_store32(z));
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c2c4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::SaveBinary` (Xbox PDB): `NiPSysModifier`'s, the
/// drag object's link id (`NiStream::SaveLinkID`), the axis
/// (`NiColor::SaveBinary`), then the percentage, range and falloff (4
/// bytes each).
pub fn ni_p_sys_drag_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    let drag_object = e.get(this, NiPSysDragModifier::m_pkDragObj);
    e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![drag_object]);
    let axis = this.addr() + NiPSysDragModifier::m_kDragAxis.off;
    e.call(POINT_SAVE_BINARY, &args![axis, stream]);
    e.with_stack(4, |e, slot| {
        for field in [
            NiPSysDragModifier::m_fPercentage,
            NiPSysDragModifier::m_fRange,
            NiPSysDragModifier::m_fFalloff,
        ] {
            let value = this.addr() + field.off;
            stream_write(e, stream.addr(), value, 4, 4, slot.addr());
        }
    });
}

// Translated from 00c2c590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::IsEqual` (Xbox PDB): `NiPSysModifier`'s; both or
/// neither must have a drag object, and two must be equal (this one's
/// `IsEqual`, slot `+0x5C`, given the other's); the axes must be equal
/// (`00439090` on the other's answering 0); then equal percentage, range
/// and falloff (`FUCOMPP`: a NaN is unequal).
pub fn ni_p_sys_drag_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    other: Ptr<NiPSysDragModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let ours = e.get(this, NiPSysDragModifier::m_pkDragObj);
    let theirs = e.get(other, NiPSysDragModifier::m_pkDragObj);
    if ours != 0 {
        if theirs == 0 {
            return false;
        }
        if !e.vcall(ours, IS_EQUAL_SLOT, &args![theirs]).bool() {
            return false;
        }
    } else if theirs != 0 {
        return false;
    }
    let axis = NiPSysDragModifier::m_kDragAxis.off;
    if e.call(
        POINT_UNEQUAL,
        &args![other.addr() + axis, this.addr() + axis],
    )
    .bool()
    {
        return false;
    }
    for field in [
        NiPSysDragModifier::m_fPercentage,
        NiPSysDragModifier::m_fRange,
        NiPSysDragModifier::m_fFalloff,
    ] {
        if x87_m32(e.get(this, field)) != x87_m32(e.get(other, field)) {
            return false;
        }
    }
    true
}

// Translated from 00c2c620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragModifier::GetViewerStrings` (Xbox PDB): `NiPSysModifier`'s
/// strings, the class name, the drag object's name (its `NiObjectNET`
/// name, or "None"), the axis (`NiPoint3::GetViewerString`), the
/// percentage, the range and the falloff.
pub fn ni_p_sys_drag_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysDragModifier>,
    strings: Ptr,
) {
    let strings = strings.addr();
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let drag_object = e.get(this, NiPSysDragModifier::m_pkDragObj);
    let text = if drag_object != 0 {
        e.mem.u32(drag_object + OBJECT_NAME)
    } else {
        TEXT_NONE
    };
    let string = e
        .call(VIEWER_STRING_TEXT, &args![LABEL_DRAG_OBJECT, text])
        .u32();
    add_viewer_string(e, strings, string);
    let axis = this.addr() + NiPSysDragModifier::m_kDragAxis.off;
    let string = e
        .call(POINT_GET_VIEWER_STRING, &args![axis, LABEL_DRAG_AXIS])
        .u32();
    add_viewer_string(e, strings, string);
    for (label, field) in [
        (LABEL_PERCENTAGE, NiPSysDragModifier::m_fPercentage),
        (LABEL_RANGE, NiPSysDragModifier::m_fRange),
        (LABEL_FALLOFF, NiPSysDragModifier::m_fFalloff),
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        add_viewer_string(e, strings, string);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_c030,
            fn_00c2c030(Ptr<NiPSysDragModifier>) -> Ptr<NiPSysDragModifier>
        ),
        entry!(0x00c2_c080, fn_00c2c080() -> Ptr<NiPSysDragModifier>),
        entry!(
            0x00c2_c450,
            ni_p_sys_drag_modifier_load_binary(Ptr<NiPSysDragModifier>, Ptr)
        ),
        entry!(0x00c2c0a0, ni_p_sys_drag_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c23cf0, ni_p_sys_drag_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c2_c0b0,
            ni_p_sys_drag_modifier_create_clone(
                Ptr<NiPSysDragModifier>,
                Ptr,
            ) -> Ptr<NiPSysDragModifier>
        ),
        entry!(
            0x00c2_c170,
            ni_p_sys_drag_modifier_update(Ptr<NiPSysDragModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_c4f0,
            ni_p_sys_drag_modifier_save_binary(Ptr<NiPSysDragModifier>, Ptr)
        ),
        entry!(
            0x00c2_c590,
            ni_p_sys_drag_modifier_is_equal(
                Ptr<NiPSysDragModifier>,
                Ptr<NiPSysDragModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_c620,
            ni_p_sys_drag_modifier_get_viewer_strings(Ptr<NiPSysDragModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_drag_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c23cf0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c23cf0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_drag_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_c0a0, &args![]).u32(), 0x0120_2768);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    #[test]
    fn factory_constructs_the_defaults_or_answers_null() {
        let mut e = Engine::new();
        e.map(DEFAULT_AXIS & !0xfff, 0x1000);
        e.set_global(DEFAULT_AXIS, 1.0f32);
        e.map(DEFAULT_PERCENTAGE & !0xfff, 0x1000);
        e.set_global(DEFAULT_PERCENTAGE, 0.05f32);
        e.map(DEFAULT_RANGE & !0xfff, 0x1000);
        e.set_global(DEFAULT_RANGE, f32::MAX);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        let made = e.call(0x00c2_c080, &args![]).ptr::<NiPSysDragModifier>();
        assert_eq!(e.mem.u32(made.addr()), VTABLE);
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_kDragAxis),
            1.0f32.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fPercentage),
            0.05f32.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fRange),
            f32::MAX.to_bits()
        );
        assert_eq!(
            e.get(made, NiPSysDragModifier::m_fFalloff),
            f32::MAX.to_bits()
        );
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_c080, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_link_axis_and_three_floats() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        let bytes: Vec<u8> = [0.5f32, 2.0, 3.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysDragModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_c450, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == COLOR_LOAD_BINARY && c.1 == vec![this.addr() + 0x1c, stream]));
        assert!(log
            .iter()
            .any(|c| c.0 == READ_LINK_ID && c.1 == vec![stream]));
        assert_eq!(
            e.get(this, NiPSysDragModifier::m_fPercentage),
            0.5f32.to_bits()
        );
        assert_eq!(
            e.get(this, NiPSysDragModifier::m_fFalloff),
            3.0f32.to_bits()
        );
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};

    type M = NiPSysDragModifier;

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(addr + 4 * i as u32, value.to_bits());
        }
    }

    fn floats(e: &Engine, addr: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    fn modifier(e: &mut Engine) -> Ptr<M> {
        let this = e.new_object::<M>();
        set_floats(e, this.addr() + 0x1c, &[1.0, 2.0, 3.0, 0.5, 10.0, 20.0]);
        this
    }

    #[test]
    fn create_clone_copies_axis_and_settings() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        quiet(&mut e, &[MODIFIER_CONSTRUCTOR, MODIFIER_COPY_MEMBERS]);
        e.map(DEFAULT_AXIS, 12);
        e.map(DEFAULT_PERCENTAGE, 4);
        e.map(DEFAULT_RANGE, 4);
        let this = modifier(&mut e);
        e.set(this, M::m_pkDragObj, 0x4444);
        e.set(this, M::m_fFalloff, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_c0b0, &args![this, 0x900u32]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(floats(&e, clone + 0x1c, 5), vec![1.0, 2.0, 3.0, 0.5, 10.0]);
        assert_eq!(e.mem.u32(clone + 0x30), 0x7fc0_0001);
        // The drag object is not copied.
        assert_eq!(e.mem.u32(clone + 0x18), 0);
    }

    /// Doubles for the transform helpers: the product is the drag object's
    /// world transform, a point turns into itself, normalizing does nothing,
    /// and the square root is exact.
    fn transforms(e: &mut Engine) {
        quiet(e, &[TRANSFORM_INVERT, POINT_UNITIZE]);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            let words = e.mem.bytes(a[2], TRANSFORM_SIZE);
            e.mem.write(a[1], &words);
            Ret::default()
        });
        e.register(TRANSFORM_POINT, |e, a| {
            let words = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &words);
            answer(a[1])
        });
        e.register(SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        e.map(ONE, 8);
        e.set_global::<f64>(ONE, 1.0);
        e.map(TIME_STEP, 8);
        e.set_global::<f64>(TIME_STEP, 0.5);
    }

    /// A system with one particle at `position`, velocity (1, 2, 4), last
    /// updated at 1.5; the drag object sits at the origin with axis z.
    fn scene(e: &mut Engine, position: [f32; 3]) -> (Ptr<M>, u32, u32) {
        let this = modifier(e);
        set_floats(e, this.addr() + 0x1c, &[0.0, 0.0, 1.0]);
        let drag = e.mem.alloc(0x100);
        let system = e.mem.alloc(0x100);
        e.set(this, M::m_pkDragObj, drag);
        e.mem.set_u32(this.addr() + MODIFIER_TARGET, system);
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        let vertices = e.mem.alloc(12);
        set_floats(e, vertices, &position);
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        let infos = e.mem.alloc(INFO_SIZE);
        set_floats(e, infos, &[1.0, 2.0, 4.0]);
        e.mem.set_u32(infos + INFO_LAST_UPDATE, 1.5f32.to_bits());
        e.mem.set_u32(data + DATA_INFOS, infos);
        (this, data, infos)
    }

    #[test]
    fn update_takes_a_share_of_the_velocity_along_the_axis() {
        let mut e = Engine::new();
        transforms(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        // Inside the range: strength 0.25 * elapsed 1.0; dot 4 over |axis|^2 1.
        e.set(this, M::m_fPercentage, 0.25f32.to_bits());
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 3.0]);
        // A scaled strength above 1: the whole axis part goes.
        e.set(this, M::m_fPercentage, 4.0f32.to_bits());
        set_floats(&mut e, infos, &[1.0, 2.0, 4.0]);
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 0.0]);
    }

    #[test]
    fn update_fades_between_range_and_falloff() {
        let mut e = Engine::new();
        transforms(&mut e);
        // Distance 5 from the drag object.
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        e.set(this, M::m_fPercentage, 0.5f32.to_bits());
        e.set(this, M::m_fRange, 3.0f32.to_bits());
        e.set(this, M::m_fFalloff, 7.0f32.to_bits());
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        // Strength 0.5 * (1 - 2/4) = 0.25.
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 3.0]);
        // At the falloff the particle is left alone.
        e.set(this, M::m_fFalloff, 5.0f32.to_bits());
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 3.0]);
        // The drag object's position comes from the product's translation.
        let drag = e.get(this, M::m_pkDragObj);
        set_floats(&mut e, drag + WORLD_TRANSFORM + 0x24, &[3.0, 4.0, 0.0]);
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 1.5]);
    }

    #[test]
    fn update_needs_a_percentage_particles_and_a_drag_object() {
        let mut e = Engine::new();
        transforms(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        e.call_log = Some(vec![]);
        for percentage in [0.0f32, -1.0, f32::NAN] {
            e.set(this, M::m_fPercentage, percentage.to_bits());
            e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        }
        e.set(this, M::m_fPercentage, 0.25f32.to_bits());
        e.mem.set_u16(data + DATA_ACTIVE, 0);
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        e.set(this, M::m_pkDragObj, 0);
        e.call(0x00c2_c170, &args![this, 2.0f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        assert_eq!(floats(&e, infos, 3), vec![1.0, 2.0, 4.0]);
    }

    #[test]
    fn save_binary_writes_link_axis_and_settings() {
        const SAVE_LINK: u32 = 0x0000_b030;
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[MODIFIER_SAVE_BINARY, POINT_SAVE_BINARY, SAVE_LINK],
        );
        let stream = viewer_test::out_stream(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + 0x30, SAVE_LINK);
        let this = modifier(&mut e);
        e.set(this, M::m_pkDragObj, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00c2_c4f0, &args![this, stream]);
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x4444]]);
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this.addr() + 0x1c, stream]]
        );
        let bytes: Vec<u8> = [0.5f32, 10.0, 20.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        assert_eq!(viewer_test::written(&e, stream), (bytes, 12));
    }

    #[test]
    fn is_equal_compares_object_axis_and_settings() {
        const EQUAL: u32 = 0x0000_b05c;
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(1));
        e.register(POINT_UNEQUAL, |e, a| {
            answer(u32::from(e.mem.bytes(a[0], 12) != e.mem.bytes(a[1], 12)))
        });
        e.register(EQUAL, |_, a| answer(u32::from(a[1] == 0x5555)));
        let this = modifier(&mut e);
        let other = modifier(&mut e);
        let check = |e: &mut Engine| e.call(0x00c2_c590, &args![this, other]).bool();
        assert!(check(&mut e));
        let ours = viewer_test::object(&mut e, &[(0x5c, EQUAL)]);
        e.set(this, M::m_pkDragObj, ours);
        assert!(!check(&mut e));
        e.set(other, M::m_pkDragObj, 0x5555);
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(calls_to(&e, EQUAL), vec![vec![ours, 0x5555]]);
        assert_eq!(
            calls_to(&e, POINT_UNEQUAL),
            vec![vec![other.addr() + 0x1c, this.addr() + 0x1c]]
        );
        e.set(other, M::m_pkDragObj, 0x5556);
        assert!(!check(&mut e));
        e.set(this, M::m_pkDragObj, 0);
        assert!(!check(&mut e));
        e.set(other, M::m_pkDragObj, 0);
        e.mem.set_u32(other.addr() + 0x20, 0);
        assert!(!check(&mut e));
        e.mem.set_u32(other.addr() + 0x20, 2.0f32.to_bits());
        for field in [M::m_fPercentage, M::m_fRange, M::m_fFalloff] {
            let kept = e.get(other, field);
            e.set(other, field, f32::NAN.to_bits());
            e.set(this, field, f32::NAN.to_bits());
            assert!(!check(&mut e));
            e.set(other, field, kept);
            e.set(this, field, kept);
        }
        assert!(check(&mut e));
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e));
    }

    #[test]
    fn viewer_strings_name_object_axis_and_settings() {
        let mut e = Engine::new();
        quiet(&mut e, &[MODIFIER_GET_VIEWER_STRINGS]);
        viewer_test::counting(
            &mut e,
            &[
                VIEWER_STRING,
                VIEWER_STRING_TEXT,
                VIEWER_STRING_FLOAT,
                POINT_GET_VIEWER_STRING,
            ],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_00a0);
        let this = modifier(&mut e);
        let strings = viewer_test::strings(&mut e, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_c620, &args![this, strings]);
        assert_eq!(
            viewer_test::collected(&e, strings),
            (1..=6).collect::<Vec<u32>>()
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_TEXT),
            vec![vec![LABEL_DRAG_OBJECT, TEXT_NONE]]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0x1c, LABEL_DRAG_AXIS]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_PERCENTAGE, 0.5f32.to_bits()],
                vec![LABEL_RANGE, 10.0f32.to_bits()],
                vec![LABEL_FALLOFF, 20.0f32.to_bits()]
            ]
        );
        let drag = e.mem.alloc(0x10);
        e.mem.set_u32(drag + 8, 0x0100_00b0);
        e.set(this, M::m_pkDragObj, drag);
        e.call(0x00c2_c620, &args![this, strings]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_TEXT)[1],
            vec![LABEL_DRAG_OBJECT, 0x0100_00b0]
        );
    }
}
