//! `BSHavok/bhkcylindershape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCylinderShape` is a 0x14-byte `bhkConvexShape`: `+8` the Havok cylinder shape, `+0xC`
//! its cinfo. The cinfo (0x50 bytes) holds, as read here, the two end points as four floats
//! at `+0x10` and `+0x20` and the radius at `+0x30`; a fresh one has a zero first point, a
//! `1.0` second point and a zero radius.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    aligned_allocate, enter_scope, havok_allocate, push_viewer_string, CLONE_LOCK, CLONE_UNLOCK,
    HEAP_ALLOCATE, NI_GET_VIEWER_STRING, NI_GET_VIEWER_STRING_FLOAT, NI_POINT3_GET_VIEWER_STRING,
    REMOVE_REFERENCE, SERIALIZABLE_KILL, VECTOR_TO_POINT,
};

/// `bhkRefObject` constructor (`00c87c30`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkConvexShape` base methods: destructor body (`00c9fb00`), `CopyCinfo` (`00c9fb70`),
/// `SaveBinary` (`00c9fc00`) and `GetViewerStrings` (`00c9fc50`).
const CONVEX_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape::CopyMembers` (`00c84df0`, `(this, a, b)`).
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// Copy of a `hkVector4` into a point (`004b4db0`, thiscall on the destination, one stack
/// argument: the source).
const COPY_VECTOR: u32 = 0x004b_4db0;
/// The Havok cylinder's radius getter (`00d571a0`, answers it in `ST0`).
const GET_CYLINDER_RADIUS: u32 = 0x00d5_71a0;
/// `hkpCylinderShape` constructor (`00d57aa0`, `(block, a, b, radius, padding)`, RET 0x10) and
/// the constructor `BuildDisplayGeometry` uses (`00d58510`, RET 0x14).
const HKP_CYLINDER_CONSTRUCT: u32 = 0x00d5_7aa0;
const DISPLAY_SHAPE_CONSTRUCT: u32 = 0x00d5_8510;
/// This unit's own `Init` (`00ccd610`), `CopyCinfo` (`00ccd1d0`), `Scale` (`00ccd170`) and
/// constructor (`00ccd100`).
const INIT: u32 = 0x00cc_d610;
const COPY_CINFO: u32 = 0x00cc_d1d0;
const SCALE_CINFO: u32 = 0x00cc_d170;
const CONSTRUCT: u32 = 0x00cc_d100;
/// Counters of live objects the constructor raises (one per base class and the class
/// itself) and the vtables it installs, in order; the destructor installs the last one.
const COUNTERS: [u32; 4] = [0x0126_80f8, 0x0126_8274, 0x0126_8250, 0x0126_868c];
const VTABLES: [u32; 4] = [0x0103_0bf4, 0x0103_0b0c, 0x0103_0a24, 0x010c_a834];
/// The `NiRTTI` global of the class.
const CYLINDER_RTTI: u32 = 0x0126_8694;
/// Default vector used when there is no Havok shape.
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
/// Float constants: the cinfo's `+4` word (`0.1f`) and the padding radius `Init` hands to
/// the Havok constructor (`0.05f`).
const CINFO_WORD_CONSTANT: u32 = 0x010c_72bc;
const PADDING_RADIUS_CONSTANT: u32 = 0x011b_153c;
/// Name strings of `GetViewerStrings` (`RAD`, `VA`, `VB`) and the class name
/// `bhkCylinderShape` that `BuildDisplayGeometry` passes.
const RADIUS_NAME: u32 = 0x0105_f388;
const FIRST_POINT_NAME: u32 = 0x010c_a918;
const SECOND_POINT_NAME: u32 = 0x010c_a914;
const CLASS_NAME_STRING: u32 = 0x010c_4048;

// Translated from 00ccb140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape::QCinfoSize` (Xbox PDB): the size of the cinfo, 0x40 (takes no
/// arguments).
pub fn bhk_cylinder_shape_q_cinfo_size(_e: &mut Engine) -> u32 {
    0x40
}

// Translated from 00ccd080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd080 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCylinderShape::~bhkCylinderShape` (Xbox PDB): installs the cylinder vtable, runs
/// `bhkSerializable::Kill`, decrements the live cylinder counter and runs the
/// `bhkConvexShape` destructor body.
pub fn bhk_cylinder_shape_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLES[3]);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNTERS[3]);
    e.mem.set_u32(COUNTERS[3], count.wrapping_sub(1));
    e.call(CONVEX_DESTRUCT, &args![this]);
}

// Translated from 00ccd100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape` constructor (no PDB name): runs the `bhkRefObject` constructor,
/// clears `+0xC` and `+0x10`, then installs each base class's vtable in turn and raises its
/// counter by one, ending with the cylinder's. Returns `this`.
pub fn fn_00ccd100(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (counter, vtable) in COUNTERS.into_iter().zip(VTABLES) {
        e.mem.set_u32(this.addr(), vtable);
        let count = e.mem.u32(counter);
        e.mem.set_u32(counter, count.wrapping_add(1));
    }
    this
}

// Translated from 00ccd170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales a cylinder cinfo (no PDB name): multiplies the nine floats at `+0x10..=+0x30` by
/// `scale`. Each product is formed on the x87 stack from a loaded float and the loaded
/// scale and stored back as a float.
pub fn fn_00ccd170(e: &mut Engine, cinfo: Ptr, scale: f32) {
    let scale = x87_quiet(scale);
    for offset in (0x10..=0x30).step_by(4) {
        let value = x87_quiet(e.mem.f32(cinfo.addr() + offset));
        e.mem
            .set_f32(cinfo.addr() + offset, x87_quiet(value * scale));
    }
}

// Translated from 00ccd1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape::CopyCinfo` (Xbox PDB): the `bhkConvexShape` copy, then, from the Havok
/// shape (`+8`; the default vector `01267e30` without one or without `this`), the vectors at
/// shape `+0x20` and `+0x30` into cinfo `+0x10` and `+0x20`, and the shape's radius
/// (`00d571a0`, or `0.0`) into cinfo `+0x30`.
pub fn bhk_cylinder_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    e.call(CONVEX_COPY_CINFO, &args![this, cinfo]);
    let shape = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let first = if shape != 0 {
        shape + 0x20
    } else {
        DEFAULT_VECTOR
    };
    e.call(COPY_VECTOR, &args![cinfo.addr() + 0x10, first]);
    let second = if shape != 0 {
        shape + 0x30
    } else {
        DEFAULT_VECTOR
    };
    e.call(COPY_VECTOR, &args![cinfo.addr() + 0x20, second]);
    let radius = if shape != 0 {
        x87_quiet(e.call(GET_CYLINDER_RADIUS, &args![shape]).f32())
    } else {
        0.0
    };
    e.mem.set_f32(cinfo.addr() + 0x30, radius);
}

// Translated from 00ccd250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd250 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCylinderShape` factory (no PDB name): allocates 0x14 bytes from the game heap and,
/// when that worked, constructs a cylinder in them. Answers the object or 0 (takes no
/// arguments).
pub fn fn_00ccd250(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 00ccd2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape::CopyMembers` (Xbox PDB): gets the cinfo through virtual slot `0xB4`
/// (its out argument points at an unused stack byte), reads the float at `context + 0x10`
/// and, unless it is exactly 1.0 (a NaN is not), scales the cinfo by it (`00ccd170`); then
/// the `bhkShape` copy of `(this, source, context)`.
pub fn bhk_cylinder_shape_copy_members(e: &mut Engine, this: Ptr, source: u32, context: u32) {
    let cinfo = e
        .with_stack(4, |e, slot| {
            e.vcall(this.addr(), 0xb4, &args![slot.addr() + 3])
        })
        .u32();
    let scale = x87_quiet(e.mem.f32(context + 0x10));
    if scale != 1.0 {
        e.call(SCALE_CINFO, &args![cinfo, scale]);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, context]);
}

// Translated from 00ccd3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cylinder `SaveBinary` (the engine map lists the address as `bhkCapsuleShape::SaveBinary`,
/// which shares its code; unconfirmed for the cylinder): gets the cinfo through virtual
/// slot `0xB4` (out argument: an unused stack byte), byte-swaps its eight words at
/// `+0x10..=+0x2c` when the stream's byte at `+0x1E4` is not 1, runs the `bhkConvexShape`
/// save, and swaps them back under the same test (read again).
pub fn fn_00ccd3a0(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = e
        .with_stack(4, |e, slot| {
            e.vcall(this.addr(), 0xb4, &args![slot.addr() + 3])
        })
        .u32();
    swap_cinfo_words(e, cinfo, stream);
    e.call(CONVEX_SAVE_BINARY, &args![this, stream]);
    swap_cinfo_words(e, cinfo, stream);
}

/// Byte-swaps the eight cinfo words at `+0x10..=+0x2c` unless the stream's byte at `+0x1E4`
/// is 1.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32, stream: u32) {
    if e.mem.u8(stream + 0x1e4) != 1 {
        for offset in (0x10..=0x2c).step_by(4) {
            let word = e.mem.u32(cinfo + offset);
            e.mem.set_u32(cinfo + offset, word.swap_bytes());
        }
    }
}

// Translated from 00ccd460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd460 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCylinderShape::CreateClone` (Xbox PDB): with the allocation-scope word at 0x10 and
/// the clone lock held, allocates and constructs a cylinder (0x14 bytes from the game
/// heap; null when the allocation fails), has `this` fill it through virtual slot `0xC4`
/// (`CopyMembers`, with `context`), and answers the clone.
pub fn bhk_cylinder_shape_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    let clone = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.vcall(this.addr(), 0xc4, &args![clone, context]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

/// Fills a fresh cylinder cinfo: `+0` and `+0x10..=+0x1c` and `+0x30` zero, `+4` the
/// `0.1f` constant (through the x87 stack), `+0x20..=+0x2c` one.
fn initialise_cinfo(e: &mut Engine, cinfo: u32) {
    e.mem.set_u32(cinfo, 0);
    let constant = x87_quiet_bits(e.global::<u32>(CINFO_WORD_CONSTANT));
    e.mem.set_u32(cinfo + 4, constant);
    for offset in (0x10..=0x1c).step_by(4) {
        e.mem.set_u32(cinfo + offset, 0);
    }
    e.mem.set_u32(cinfo + 0x30, 0);
    for offset in (0x20..=0x2c).step_by(4) {
        e.mem.set_u32(cinfo + offset, 0x3f80_0000);
    }
}

// Translated from 00ccd520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCylinderShape::CreateCinfo` (Xbox PDB): with the allocation-scope word at 0x10, writes
/// whether there was no cinfo yet into the byte at `created`; if there was none, allocates a
/// 0x50-byte 16-aligned block, fills it as a fresh cinfo, stores it at `+0xC` and, when the
/// Havok shape (`+8`) exists, fills it from the shape with `CopyCinfo`. Answers the cinfo.
pub fn bhk_cylinder_shape_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let none_yet = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created, none_yet as u8);
    if none_yet {
        let cinfo = aligned_allocate(e, 0x50);
        initialise_cinfo(e, cinfo);
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            e.call(COPY_CINFO, &args![this, cinfo]);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00ccd610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd610 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCylinderShape::Init` (Xbox PDB): with the allocation-scope word at 0x10, allocates
/// 0x60 bytes from the Havok allocator, constructs the Havok cylinder in them from the two
/// end-point vectors, the radius (through the x87 stack) and the `0.05f` padding constant,
/// gives it to `this` through virtual slot `0x8C` and drops the construction reference.
pub fn bhk_cylinder_shape_init(e: &mut Engine, this: Ptr, first: u32, second: u32, radius: f32) {
    let (word, saved) = enter_scope(e);
    let block = havok_allocate(e, 0x60);
    let padding = x87_quiet_bits(e.global::<u32>(PADDING_RADIUS_CONSTANT));
    let shape = e
        .call(
            HKP_CYLINDER_CONSTRUCT,
            &args![block, first, second, x87_quiet(radius), padding],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ccd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd6f0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCylinderShape::BuildDisplayGeometry` (Xbox PDB): with the allocation-scope word at
/// 0x10, fills a temporary fresh cinfo from `this` (`CopyCinfo`), allocates 0x90 bytes from
/// the Havok allocator and constructs a display shape in them (`00d58510`) from the two
/// end-point vectors, the radius, `9` and `1`; calls its virtual slot `0x10`; and, when
/// the object has a word at `+8`, calls `this` virtual slot `0xDC` with `(first, that word,
/// second, "bhkCylinderShape")`.
pub fn bhk_cylinder_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) {
    let (word, saved) = enter_scope(e);
    e.with_stack(0x50, |e, cinfo| {
        let cinfo = cinfo.addr();
        initialise_cinfo(e, cinfo);
        e.call(COPY_CINFO, &args![this, cinfo]);
        let block = havok_allocate(e, 0x90);
        let radius = x87_quiet(e.mem.f32(cinfo + 0x30));
        e.with_stack(0x20, |e, vectors| {
            let end_b = vectors.addr();
            let end_a = end_b + 0x10;
            for lane in 0..4 {
                let high = e.mem.u32(cinfo + 0x20 + 4 * lane);
                let low = e.mem.u32(cinfo + 0x10 + 4 * lane);
                e.mem.set_u32(end_a + 4 * lane, high);
                e.mem.set_u32(end_b + 4 * lane, low);
            }
            let display = e
                .call(
                    DISPLAY_SHAPE_CONSTRUCT,
                    &args![block, end_b, end_a, radius, 9u32, 1u32],
                )
                .u32();
            e.vcall(display, 0x10, &args![]);
            let inner = e.mem.u32(display + 8);
            if inner != 0 {
                e.vcall(
                    this.addr(),
                    0xdc,
                    &args![first, inner, second, CLASS_NAME_STRING],
                );
            }
        });
    });
    e.mem.set_u32(word, saved);
}

// Translated from 00ccd8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd8c0 - the /GS cookie check is not modeled.
/// `bhkCylinderShape::Init_ov2` (Xbox PDB): copies the cinfo's two end-point vectors
/// (`+0x10..` and `+0x20..`, four words each) into stack vectors and its radius (`+0x30`,
/// through the x87 stack) and calls `Init` (`00ccd610`) with them, then virtual slot `0xBC`
/// on `this` with the cinfo.
pub fn bhk_cylinder_shape_init_ov2(e: &mut Engine, this: Ptr, cinfo: u32) {
    let radius = x87_quiet(e.mem.f32(cinfo + 0x30));
    e.with_stack(0x20, |e, vectors| {
        let first = vectors.addr();
        let second = first + 0x10;
        for lane in 0..4 {
            let low = e.mem.u32(cinfo + 0x10 + 4 * lane);
            let high = e.mem.u32(cinfo + 0x20 + 4 * lane);
            e.mem.set_u32(first + 4 * lane, low);
            e.mem.set_u32(second + 4 * lane, high);
        }
        e.call(INIT, &args![this, first, second, radius]);
    });
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
}

// Translated from 00ccd960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccd960 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCylinderShape::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10,
/// runs the `bhkConvexShape` strings, appends the class name, fills a temporary fresh cinfo
/// from `this` (`CopyCinfo`) and appends its radius (`RAD`) and its two end points as
/// `NiPoint3` strings (`VA` for the vector at cinfo `+0x10`, `VB` for `+0x20`).
pub fn bhk_cylinder_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(CONVEX_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CYLINDER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.with_stack(0x50, |e, cinfo| {
        let cinfo = cinfo.addr();
        initialise_cinfo(e, cinfo);
        e.call(COPY_CINFO, &args![this, cinfo]);
        let radius = x87_quiet(e.mem.f32(cinfo + 0x30));
        let text = e
            .call(NI_GET_VIEWER_STRING_FLOAT, &args![RADIUS_NAME, radius])
            .u32();
        push_viewer_string(e, strings, text);
        e.with_stack(0x20, |e, scratch| {
            let point = scratch.addr();
            let vector = point + 0x10;
            for (group, name) in [(0x10, FIRST_POINT_NAME), (0x20, SECOND_POINT_NAME)] {
                for lane in 0..4 {
                    let word = e.mem.u32(cinfo + group + 4 * lane);
                    e.mem.set_u32(vector + 4 * lane, word);
                }
                e.call(VECTOR_TO_POINT, &args![point, vector]);
                let text = e
                    .call(NI_POINT3_GET_VIEWER_STRING, &args![point, name])
                    .u32();
                push_viewer_string(e, strings, text);
            }
        });
    });
    e.mem.set_u32(word, saved);
}

// Translated from 00ccd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccd080`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_cylinder_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_d080;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccd0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCylinderShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8694.
pub fn bhk_cylinder_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8694
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc_b140, bhk_cylinder_shape_q_cinfo_size() -> u32),
        entry!(0x00cc_d080, bhk_cylinder_shape_destruct(Ptr)),
        entry!(0x00cc_d0f0, bhk_cylinder_shape_get_r_t_t_i() -> u32),
        entry!(0x00cc_d100, fn_00ccd100(Ptr) -> Ptr),
        entry!(0x00cc_d150, bhk_cylinder_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc_d170, fn_00ccd170(Ptr, f32)),
        entry!(0x00cc_d1d0, bhk_cylinder_shape_copy_cinfo(Ptr, Ptr)),
        entry!(0x00cc_d250, fn_00ccd250() -> u32),
        entry!(0x00cc_d2c0, bhk_cylinder_shape_copy_members(Ptr, u32, u32)),
        entry!(0x00cc_d3a0, fn_00ccd3a0(Ptr, u32)),
        entry!(0x00cc_d460, bhk_cylinder_shape_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc_d520, bhk_cylinder_shape_create_cinfo(Ptr, u32) -> u32),
        entry!(0x00cc_d610, bhk_cylinder_shape_init(Ptr, u32, u32, f32)),
        entry!(
            0x00cc_d6f0,
            bhk_cylinder_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00cc_d8c0, bhk_cylinder_shape_init_ov2(Ptr, u32)),
        entry!(0x00cc_d960, bhk_cylinder_shape_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{SCOPE_WORD, STRINGS_SET_AT};
    use super::*;

    /// Doubles for the `bhkConvexShape` and Havok callees the cinfo routines use.
    fn cinfo_doubles(e: &mut Engine) {
        e.register(CONVEX_COPY_CINFO, |_, _| Ret::default());
        e.register(COPY_VECTOR, |e, a| {
            for lane in 0..4 {
                let word = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, word);
            }
            Ret::default()
        });
        e.register(GET_CYLINDER_RADIUS, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.map(DEFAULT_VECTOR, 0x10);
        e.map(CINFO_WORD_CONSTANT, 4);
        e.mem.set_u32(CINFO_WORD_CONSTANT, 0x3dcc_cccd);
    }

    /// A cylinder with a Havok shape whose vectors at `+0x20`/`+0x30` and radius are set up.
    fn cylinder_with_shape(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x14);
        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(this + 8, shape);
        for lane in 0..4 {
            e.mem.set_f32(shape + 0x20 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(shape + 0x30 + 4 * lane, 5.0 + lane as f32);
        }
        (this, shape)
    }

    #[test]
    fn cinfo_size_is_0x40() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_b140, &args![]).u32(), 0x40);
    }

    #[test]
    fn destructor_counts_down_and_runs_the_convex_destructor() {
        let mut e = logging_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONVEX_DESTRUCT, |_, _| Ret::default());
        e.map(COUNTERS[3], 4);
        e.mem.set_u32(COUNTERS[3], 3);
        let this = e.mem.alloc(0x14);
        e.call(0x00cc_d080, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLES[3]);
        assert_eq!(e.mem.u32(COUNTERS[3]), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONVEX_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn constructor_raises_every_counter_and_ends_with_the_cylinder_vtable() {
        let mut e = logging_engine();
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        for counter in COUNTERS {
            e.map(counter, 4);
            e.mem.set_u32(counter, 10);
        }
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 7);
        e.mem.set_u32(this + 0x10, 7);
        assert_eq!(e.call(0x00cc_d100, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), VTABLES[3]);
        for counter in COUNTERS {
            assert_eq!(e.mem.u32(counter), 11);
        }
    }

    #[test]
    fn scale_multiplies_the_nine_floats_and_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x50);
        for index in 0..9 {
            e.mem.set_f32(cinfo + 0x10 + 4 * index, 1.0 + index as f32);
        }
        e.mem.set_f32(cinfo + 0x34, 99.0);
        e.call(0x00cc_d170, &args![cinfo, 2.0f32]);
        for index in 0..9 {
            assert_eq!(
                e.mem.f32(cinfo + 0x10 + 4 * index),
                2.0 * (1.0 + index as f32)
            );
        }
        assert_eq!(e.mem.f32(cinfo + 0x34), 99.0);
        e.mem.set_u32(cinfo + 0x14, 0x7f80_0001);
        e.call(0x00cc_d170, &args![cinfo, 1.0f32]);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x7fc0_0001);
    }

    #[test]
    fn copy_cinfo_takes_the_shape_vectors_and_radius() {
        let mut e = logging_engine();
        cinfo_doubles(&mut e);
        let (this, shape) = cylinder_with_shape(&mut e);
        let cinfo = e.mem.alloc(0x50);
        e.call(0x00cc_d1d0, &args![this, cinfo]);
        assert_eq!(calls_to(&e, CONVEX_COPY_CINFO), vec![vec![this, cinfo]]);
        assert_eq!(
            calls_to(&e, COPY_VECTOR),
            vec![
                vec![cinfo + 0x10, shape + 0x20],
                vec![cinfo + 0x20, shape + 0x30]
            ]
        );
        assert_eq!(e.mem.f32(cinfo + 0x10), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x2c), 8.0);
        assert_eq!(e.mem.f32(cinfo + 0x30), 2.5);
        // Without a Havok shape the default vector and a zero radius are used.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cc_d1d0, &args![this, cinfo]);
        let copies = calls_to(&e, COPY_VECTOR);
        assert_eq!(
            copies[2..],
            [
                vec![cinfo + 0x10, DEFAULT_VECTOR],
                vec![cinfo + 0x20, DEFAULT_VECTOR]
            ]
        );
        assert_eq!(e.mem.f32(cinfo + 0x30), 0.0);
        assert_eq!(calls_to(&e, GET_CYLINDER_RADIUS).len(), 1);
    }

    #[test]
    fn factory_constructs_only_in_a_block_it_got() {
        let mut e = logging_engine();
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: if a[0] == 0x14 {
                e.mem.u32(0x0900_1000)
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, 0x4000);
        assert_eq!(e.call(0x00cc_d250, &args![]).u32(), 0x4001);
        e.mem.set_u32(0x0900_1000, 0);
        assert_eq!(e.call(0x00cc_d250, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRUCT), vec![vec![0x4000]]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x14]; 2]);
    }

    #[test]
    fn copy_members_scales_the_cinfo_unless_the_factor_is_exactly_one() {
        let mut e = logging_engine();
        e.register(SCALE_CINFO, |_, _| Ret::default());
        e.register(SHAPE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(0x0900_00b4, |e, a| {
            assert_eq!(a.len(), 2);
            Ret {
                eax: e.mem.u32(0x0900_2000),
                ..Ret::default()
            }
        });
        e.map(0x0900_2000, 4);
        e.mem.set_u32(0x0900_2000, 0x7000);
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        let context = e.mem.alloc(0x20);
        e.mem.set_f32(context + 0x10, 1.0);
        e.call(0x00cc_d2c0, &args![this, 0x55u32, context]);
        assert!(calls_to(&e, SCALE_CINFO).is_empty());
        e.mem.set_f32(context + 0x10, 3.0);
        e.call(0x00cc_d2c0, &args![this, 0x55u32, context]);
        assert_eq!(
            calls_to(&e, SCALE_CINFO),
            vec![vec![0x7000, 3.0f32.to_bits()]]
        );
        e.mem.set_u32(context + 0x10, 0x7fc0_0000);
        e.call(0x00cc_d2c0, &args![this, 0x55u32, context]);
        assert_eq!(calls_to(&e, SCALE_CINFO).len(), 2);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x55, context]; 3]
        );
    }

    #[test]
    fn save_swaps_the_cinfo_words_around_the_convex_save() {
        let mut e = logging_engine();
        e.register(0x0900_00b4, |e, _| Ret {
            eax: e.mem.u32(0x0900_2000),
            ..Ret::default()
        });
        e.register(CONVEX_SAVE_BINARY, |e, _| {
            let cinfo = e.mem.u32(0x0900_2000);
            let seen = e.mem.u32(cinfo + 0x10);
            e.mem.set_u32(cinfo + 0x40, seen);
            Ret::default()
        });
        let cinfo = e.mem.alloc(0x50);
        e.map(0x0900_2000, 4);
        e.mem.set_u32(0x0900_2000, cinfo);
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        for index in 0..8 {
            e.mem.set_u32(cinfo + 0x10 + 4 * index, 0x1122_3344 + index);
        }
        e.mem.set_u32(cinfo + 0x30, 0x1122_3344);
        let stream = e.mem.alloc(0x200);
        e.call(0x00cc_d3a0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x4433_2211);
        for index in 0..8 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * index), 0x1122_3344 + index);
        }
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x1122_3344);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00cc_d3a0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x1122_3344);
    }

    #[test]
    fn create_clone_constructs_fills_through_slot_0xc4_and_restores_the_scope() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0900_00c4, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xc4, 0x0900_00c4);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 5);
        let clone = e.call(0x00cc_d460, &args![this, 0x66u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, 0x0900_00c4), vec![vec![this, clone, 0x66]]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 5);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        let lock = order.iter().position(|a| *a == CLONE_LOCK).unwrap();
        let fill = order.iter().position(|a| *a == 0x0900_00c4).unwrap();
        let unlock = order.iter().position(|a| *a == CLONE_UNLOCK).unwrap();
        assert!(lock < fill && fill < unlock);
    }

    #[test]
    fn create_clone_passes_null_when_the_allocation_fails() {
        let mut e = logging_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(0x0900_00c4, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xc4, 0x0900_00c4);
        assert_eq!(e.call(0x00cc_d460, &args![this, 0x66u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_00c4), vec![vec![this, 0, 0x66]]);
    }

    #[test]
    fn create_cinfo_builds_a_fresh_cinfo_once() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(COPY_CINFO, |_, _| Ret::default());
        e.map(CINFO_WORD_CONSTANT, 4);
        e.mem.set_u32(CINFO_WORD_CONSTANT, 0x3dcc_cccd);
        let this = e.mem.alloc(0x14);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        let cinfo = e.call(0x00cc_d520, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(cinfo & 0xf, 0);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo + 4), 0x3dcc_cccd);
        assert_eq!(e.mem.f32(cinfo + 0x20), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x2c), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x30), 0.0);
        assert!(calls_to(&e, COPY_CINFO).is_empty());
        // Second call finds it and reports so; with a Havok shape a new cinfo is filled.
        assert_eq!(e.call(0x00cc_d520, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0x1234);
        let again = e.call(0x00cc_d520, &args![this, created]).u32();
        assert_eq!(calls_to(&e, COPY_CINFO), vec![vec![this, again]]);
    }

    #[test]
    fn init_builds_the_havok_cylinder_and_hands_it_over() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(HKP_CYLINDER_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(0x0900_008c, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.map(PADDING_RADIUS_CONSTANT, 4);
        e.mem.set_u32(PADDING_RADIUS_CONSTANT, 0x3d4c_cccd);
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x8c, 0x0900_008c);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 5);
        e.call(0x00cc_d610, &args![this, 0x11u32, 0x22u32, 1.5f32]);
        let built = calls_to(&e, HKP_CYLINDER_CONSTRUCT);
        assert_eq!(built.len(), 1);
        let block = built[0][0];
        assert_eq!(built[0][1..], [0x11, 0x22, 1.5f32.to_bits(), 0x3d4c_cccd]);
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(calls_to(&e, 0x0900_008c), vec![vec![this, block + 0x100]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block + 0x100]]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 5);
        // A signalling NaN radius reaches the constructor quiet.
        e.call(
            0x00cc_d610,
            &args![this, 1u32, 2u32, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(calls_to(&e, HKP_CYLINDER_CONSTRUCT)[1][3], 0x7fc0_0001);
    }

    #[test]
    fn init_ov2_passes_the_cinfo_vectors_and_radius_to_init() {
        let mut e = logging_engine();
        e.register(INIT, |e, a| {
            // The vectors are alive during the call: record their contents.
            for lane in 0..4 {
                let low = e.mem.u32(a[1] + 4 * lane);
                let high = e.mem.u32(a[2] + 4 * lane);
                e.mem.set_u32(0x0900_3000 + 4 * lane, low);
                e.mem.set_u32(0x0900_3010 + 4 * lane, high);
            }
            Ret::default()
        });
        e.register(0x0900_00bc, |_, _| Ret::default());
        e.map(0x0900_3000, 0x20);
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xbc, 0x0900_00bc);
        let cinfo = e.mem.alloc(0x50);
        for index in 0..8 {
            e.mem.set_u32(cinfo + 0x10 + 4 * index, 0x100 + index);
        }
        e.mem.set_f32(cinfo + 0x30, 0.75);
        e.call(0x00cc_d8c0, &args![this, cinfo]);
        let init = calls_to(&e, INIT);
        assert_eq!(init.len(), 1);
        assert_eq!((init[0][0], init[0][3]), (this, 0.75f32.to_bits()));
        assert_eq!(e.mem.u32(0x0900_3000), 0x100);
        assert_eq!(e.mem.u32(0x0900_300c), 0x103);
        assert_eq!(e.mem.u32(0x0900_3010), 0x104);
        assert_eq!(e.mem.u32(0x0900_301c), 0x107);
        assert_eq!(calls_to(&e, 0x0900_00bc), vec![vec![this, cinfo]]);
    }

    #[test]
    fn build_display_geometry_hands_the_inner_object_to_slot_0xdc() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        e.register(COPY_CINFO, |e, a| {
            e.mem.set_f32(a[1] + 0x30, 4.0);
            Ret::default()
        });
        e.register(DISPLAY_SHAPE_CONSTRUCT, |e, a| {
            // Record the vectors' contents.
            let first = e.mem.u32(a[1]);
            let second = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_3000, first);
            e.mem.set_u32(0x0900_3004, second);
            Ret {
                eax: e.mem.u32(0x0900_3008),
                ..Ret::default()
            }
        });
        e.map(0x0900_3000, 0x10);
        e.map(CINFO_WORD_CONSTANT, 4);
        e.mem.set_u32(CINFO_WORD_CONSTANT, 0x3dcc_cccd);
        let display = e.mem.alloc(0x20);
        e.mem.set_u32(0x0900_3008, display);
        let display_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(display, display_vtable);
        e.mem.set_u32(display_vtable + 0x10, 0x0900_0010);
        e.register(0x0900_0010, |_, _| Ret::default());
        e.mem.set_u32(display + 8, 0x3333);
        e.register(0x0900_00dc, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xdc, 0x0900_00dc);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 5);
        e.call(0x00cc_d6f0, &args![this, 0x71u32, 0x72u32]);
        let built = calls_to(&e, DISPLAY_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][3..], [4.0f32.to_bits(), 9, 1]);
        assert_eq!(e.mem.u16(built[0][0] + 4), 0x90);
        // The first vector is the cinfo's +0x10 group (zeros), the second its +0x20 (ones).
        assert_eq!(e.mem.u32(0x0900_3000), 0);
        assert_eq!(e.mem.u32(0x0900_3004), 0x3f80_0000);
        assert_eq!(calls_to(&e, 0x0900_0010), vec![vec![display]]);
        assert_eq!(
            calls_to(&e, 0x0900_00dc),
            vec![vec![this, 0x71, 0x3333, 0x72, CLASS_NAME_STRING]]
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 5);
        // Without an inner object the slot is not called.
        e.mem.set_u32(display + 8, 0);
        e.call(0x00cc_d6f0, &args![this, 0x71u32, 0x72u32]);
        assert_eq!(calls_to(&e, 0x0900_00dc).len(), 1);
    }

    #[test]
    fn viewer_strings_list_class_radius_and_both_points() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        e.register(CONVEX_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(COPY_CINFO, |e, a| {
            for lane in 0..8 {
                e.mem.set_u32(a[1] + 0x10 + 4 * lane, 0x200 + lane);
            }
            e.mem.set_f32(a[1] + 0x30, 6.0);
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| Ret {
            eax: 0x9000 + a[1],
            ..Ret::default()
        });
        e.register(VECTOR_TO_POINT, |e, a| {
            let first = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], first);
            Ret::default()
        });
        e.register(NI_POINT3_GET_VIEWER_STRING, |e, a| Ret {
            eax: 0xa000 + e.mem.u32(a[0]) + a[1],
            ..Ret::default()
        });
        e.map(CYLINDER_RTTI, 4);
        e.mem.set_u32(CYLINDER_RTTI, 0x300);
        e.map(CINFO_WORD_CONSTANT, 4);
        let this = e.mem.alloc(0x14);
        let strings = strings_header(&mut e, 0, 8, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 5);
        e.call(0x00cc_d960, &args![this, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 5);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![RADIUS_NAME, 6.0f32.to_bits()]]
        );
        let names: Vec<u32> = calls_to(&e, NI_POINT3_GET_VIEWER_STRING)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(names, vec![FIRST_POINT_NAME, SECOND_POINT_NAME]);
        assert_eq!(
            pushed_texts(&e, strings, 0, 4),
            vec![
                0x7300,
                0x9000 + 6.0f32.to_bits(),
                0xa000 + 0x200 + FIRST_POINT_NAME,
                0xa000 + 0x204 + SECOND_POINT_NAME
            ]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 4);
        let points = calls_to(&e, VECTOR_TO_POINT);
        assert_eq!(points.len(), 2);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = logging_engine();
        e.register(0x00cc_d080, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00cc_d150, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc_d150, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_d080), vec![vec![this]; 2]);
    }

    #[test]
    fn get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_d0f0, &args![]).u32(), 0x0126_8694);
    }
}
