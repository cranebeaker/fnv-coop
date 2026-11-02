//! `BSMain/bsutilities.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Free functions on scene-graph objects: name lookup, the selective-update
//! flags of a subtree, texture names, rotations that turn the z axis onto a
//! vector, and a camera reflected about a plane. Every function here is
//! `cdecl`.
//!
//! Class identities of the `NiRTTI` addresses the code compares against are
//! confirmed by the `GetRTTI` functions that return them (`00a2ec40`
//! `NiControllerManager`, `00a4f5d0` `NiFlipController`, `00a5fc80`
//! `NiSourceTexture`, `00a6a140` `NiTexturingProperty`, `00a5e9c0` `NiNode`;
//! engine map names). An RTTI test walks the chain from the object's
//! `GetRTTI` (virtual slot +0x08) through each `NiRTTI`'s base pointer (+4).
//!
//! x87 note: a float the code loads with `FLD` and stores with `FSTP` comes
//! out with a signalling NaN quieted ([`quiet`]); words copied with `MOV`
//! keep their bits.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiAVObject;

/// `NiObjectNET::GetController` (engine map, `00a5c570`): `(this, rtti)`,
/// the object's first controller of that class, or null.
const GET_CONTROLLER: u32 = 0x00a5_c570;
/// `NiAVObject::GetProperty` (engine map, `00a59d30`): `(this, type)`.
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `NiFilename::NiFilename` (engine map, `00aa27d0`): `(this, path)`.
const NI_FILENAME_CONSTRUCT: u32 = 0x00aa_27d0;
/// `_strcpy_s` / `_strcat_s` (engine map), `cdecl (destination, size, source)`.
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `strstr` (`00ec7750`, `cdecl (text, pattern)`).
const STRSTR: u32 = 0x00ec_7750;
/// `00a5b690(text)` (`cdecl`): the `NiFixedString` handle for a text.
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// `0043ba10(this, &handle)`: assigns the `NiFixedString` handle at the
/// address passed to the `NiFixedString` at `this`.
const FIXED_STRING_ASSIGN: u32 = 0x0043_ba10;
/// The handle word the code compares a temporary `NiFixedString` with
/// before releasing it (and uses when it has no text).
const FIXED_STRING_NONE: u32 = 0x0109_b220;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `"_:"`: the separator between an object's name and a texture file name.
const NAME_SEPARATOR: u32 = 0x010c_21e0;
/// `00a4f300(this, index)`: one texture of a flip controller.
const FLIP_CONTROLLER_GET_TEXTURE: u32 = 0x00a4_f300;
/// `00c4c8d0(this, &texture)`: what `fn_00c4bd80` hands every texture to.
const TEXTURE_VISIT: u32 = 0x00c4_c8d0;

/// `NiRTTI`s compared against.
const NI_CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
const NI_FLIP_CONTROLLER_RTTI: u32 = 0x011f_3fd4;
const NI_SOURCE_TEXTURE_RTTI: u32 = 0x011f_444c;
const NI_TEXTURING_PROPERTY_RTTI: u32 = 0x011f_49a4;
const NI_NODE_RTTI: u32 = 0x011f_4428;

/// `004a0c10(this)`: normalizes the `NiPoint3` at `this` in place (its
/// result is not used here).
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// `004b3800(this, out, &other)`: writes the cross product `this × other`
/// into `out` and returns `out`.
const POINT_CROSS: u32 = 0x004b_3800;
/// `fabs` (engine map, `00ec6cde`): a `double` argument, result in ST0.
const FABS: u32 = 0x00ec_6cde;
/// `NiPoint3::Dot` (engine map, `004b6190`): `(this, &other)`, result in ST0.
const POINT_DOT: u32 = 0x004b_6190;
/// `00439f50(this, index, out)`: column `index` of the matrix at `this`.
const MATRIX_GET_COLUMN: u32 = 0x0043_9f50;
/// `0045bb20(this, out, scale)`: the point at `this` times a float.
const POINT_SCALE: u32 = 0x0045_bb20;
/// `00439e90(this, out, &other)`: the sum of two points.
const POINT_ADD: u32 = 0x0043_9e90;
/// `004a3760(out, scale, &point)` (`cdecl`): a float times a point.
const SCALE_POINT: u32 = 0x004a_3760;
/// `00439ef0(this, out, &other)`: `this - other` into `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00476980(this, &column0, &column1, &column2)`: sets a matrix's columns.
const MATRIX_SET_COLUMNS: u32 = 0x0047_6980;
/// `NiCamera::SetViewFrustum` (engine map, `00a6faf0`): `(this, &frustum)`.
const CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `0043d410(this, word, byte, byte)`: fills a 12-byte record (the word at
/// +0, the bytes at +4 and +5, zeros at +6, +7 and +8).
const RECORD_INIT: u32 = 0x0043_d410;
/// `00a59c60(this, &record)`: calls the object's virtual slot +0xA4 with
/// the record (and its parent's slot +0xFC when it has one).
const AV_OBJECT_UPDATE_WITH: u32 = 0x00a5_9c60;
/// `2.0` (`double`) the reflection scales distances with.
const TWO: u32 = 0x0101_1590;

/// `NiNode`'s child array (an `NiTArray` at +0x9C on PC): the element
/// pointer and the `m_usSize` count.
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;

/// A float copied through the x87 stack (`FLD`/`FSTP`): a signalling NaN
/// comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// `FLD`, `FCHS`, `FSTP`: the negated float, quieted.
fn negated(value: f32) -> f32 {
    -quiet(value)
}

/// `fabs` on a float as the exe calls it: the float widened to a `double`
/// argument, the ST0 result stored as a float.
fn absolute(e: &mut Engine, value: f32) -> f32 {
    e.call(FABS, &args![quiet(value) as f64]).f64() as f32
}

/// The inlined `NiIsKindOf`: whether `object`'s RTTI chain reaches `rtti`.
/// `object` is not null.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut current = e.vcall(object, 0x08, &[]).u32();
    while current != 0 {
        if current == rtti {
            return true;
        }
        current = e.mem.u32(current.wrapping_add(4));
    }
    false
}

fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(destination.wrapping_add(4 * i), word);
    }
}

// Translated from 00c4b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::GetObjectByName` (Xbox PDB): null when `root` is null or the
/// name handle at `name` is empty. Otherwise asks the root's
/// `NiControllerManager` (when it has one) for the object through the
/// manager's object palette (+0x78, virtual slot +0x8C); failing that,
/// searches the root itself (virtual slot +0x9C) when `search_root` is set.
pub fn bs_utilities_get_object_by_name(
    e: &mut Engine,
    root: Ptr,
    name: Ptr,
    search_root: bool,
) -> Ptr {
    if root.is_null() || e.mem.u32(name.addr()) == 0 {
        return Ptr::NULL;
    }
    let manager = e
        .call(GET_CONTROLLER, &args![root, NI_CONTROLLER_MANAGER_RTTI])
        .u32();
    if manager != 0 {
        let palette = e.mem.u32(manager.wrapping_add(0x78));
        let found = e.vcall(palette, 0x8c, &args![name]).u32();
        if found != 0 {
            return Ptr::new(found);
        }
    }
    if search_root {
        e.vcall(root.addr(), 0x9c, &args![name]).ptr()
    } else {
        Ptr::NULL
    }
}

// Translated from 00c4b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::ProcessSelectiveUpdate` (Xbox PDB): for a non-null object,
/// sets bits 1 and 2 and clears bit 4 of `NiAVObject::m_uFlags` (the
/// compiler folded the three set/clear conditions to constants), then
/// recurses into every child slot of the pointer virtual slot +0x0C returns
/// (when not null).
pub fn bs_utilities_process_selective_update(e: &mut Engine, object: Ptr<NiAVObject>) {
    if object.is_null() {
        return;
    }
    let flags = e.get(object, NiAVObject::m_uFlags);
    e.set(object, NiAVObject::m_uFlags, flags | 2);
    let flags = e.get(object, NiAVObject::m_uFlags);
    e.set(object, NiAVObject::m_uFlags, flags | 4);
    let flags = e.get(object, NiAVObject::m_uFlags);
    e.set(object, NiAVObject::m_uFlags, flags & !0x10);
    let node = e.vcall(object.addr(), 0x0c, &[]).u32();
    if node == 0 {
        return;
    }
    let mut i = 0u32;
    while i < e.mem.u16(node.wrapping_add(CHILDREN_SIZE)) as u32 {
        let base = e.mem.u32(node.wrapping_add(CHILDREN_BASE));
        let child = e.mem.u32(base.wrapping_add(4 * i));
        bs_utilities_process_selective_update(e, Ptr::new(child));
        i = i.wrapping_add(1);
    }
}

/// The shared body of `GetRotationZToVector` and its transpose: normalizes
/// the vector, picks a perpendicular from the smallest component, and
/// builds two more axes with cross products. `transpose` stores the three
/// axes as columns instead of rows.
fn rotation_z_to_vector(
    e: &mut Engine,
    result: Ptr,
    x: f32,
    y: f32,
    z: f32,
    transpose: bool,
) -> Ptr {
    // The stack locals: the vector, two axes and two product temporaries.
    e.with_stack(0x3c, |e, frame| {
        let vector = frame.addr();
        let first = vector + 0x0c;
        let second = vector + 0x18;
        let product_a = vector + 0x24;
        let product_b = vector + 0x30;
        e.mem.set_f32(vector, x);
        e.mem.set_f32(vector + 4, y);
        e.mem.set_f32(vector + 8, z);
        e.call(POINT_UNITIZE, &args![vector]);
        let vx = e.mem.f32(vector);
        let vy = e.mem.f32(vector + 4);
        let vz = e.mem.f32(vector + 8);
        let ax = absolute(e, vx);
        let ay = absolute(e, vy);
        let az = absolute(e, vz);
        // `FCOMPP` then `TEST AH,1`: C0 is set for "below" and for
        // unordered, so each test below is `!(a >= b)`.
        let axis = if ay >= ax && az >= ax {
            [0.0, negated(vz), quiet(vy)]
        } else if ax >= ay && az >= ay {
            [negated(vz), 0.0, quiet(vx)]
        } else {
            [negated(vy), quiet(vx), 0.0]
        };
        for (i, value) in axis.iter().enumerate() {
            e.mem.set_f32(first + 4 * i as u32, *value);
        }
        e.call(POINT_UNITIZE, &args![first]);
        let product = e.call(POINT_CROSS, &args![first, product_a, vector]).u32();
        copy_words(e, second, product, 3);
        e.call(POINT_UNITIZE, &args![second]);
        let product = e.call(POINT_CROSS, &args![second, product_b, vector]).u32();
        copy_words(e, first, product, 3);
        e.call(POINT_UNITIZE, &args![first]);
        for (row, axis) in [first, second, vector].into_iter().enumerate() {
            for column in 0..3u32 {
                let value = quiet(e.mem.f32(axis + 4 * column));
                let (r, c) = if transpose {
                    (column, row as u32)
                } else {
                    (row as u32, column)
                };
                e.mem.set_f32(result.addr() + 12 * r + 4 * c, value);
            }
        }
    });
    result
}

// Translated from 00c4b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::GetRotationZToVector` (Xbox PDB): writes into `result` (a
/// 3x3 matrix, returned) the rotation whose rows are two axes perpendicular
/// to the normalized `(x, y, z)` and the vector itself. The first axis comes
/// from the smallest absolute component (`(0, -z, y)`, `(-z, 0, x)` or
/// `(-y, x, 0)`), the second is the product of the first with the vector,
/// the first is then rebuilt from the second; each is normalized.
pub fn bs_utilities_get_rotation_z_to_vector(
    e: &mut Engine,
    result: Ptr,
    x: f32,
    y: f32,
    z: f32,
) -> Ptr {
    rotation_z_to_vector(e, result, x, y, z, false)
}

// Translated from 00c4b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::GetRotationZToVectorTranspose` (Xbox PDB): the same axes as
/// `GetRotationZToVector`, stored as the columns of `result`.
pub fn bs_utilities_get_rotation_z_to_vector_transpose(
    e: &mut Engine,
    result: Ptr,
    x: f32,
    y: f32,
    z: f32,
) -> Ptr {
    rotation_z_to_vector(e, result, x, y, z, true)
}

// Translated from 00c4bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::GetTextureName` (Xbox PDB): for an `NiSourceTexture` (an
/// RTTI test), the word at +0x34; 0 for anything else or null.
pub fn bs_utilities_get_texture_name(e: &mut Engine, texture: Ptr) -> u32 {
    let source = if texture.is_null() {
        0
    } else if is_kind_of(e, texture.addr(), NI_SOURCE_TEXTURE_RTTI) {
        texture.addr()
    } else {
        0
    };
    if source != 0 {
        e.mem.u32(source + 0x34)
    } else {
        0
    }
}

// Translated from 00c4bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4bbc0 - FS:[0] exception frame (unwinding of the NiFixedString temporary) and the /GS cookie check (00ec408c) are not translated
/// Renames `object` (its `NiFixedString` name at +8) to
/// `"<old name>_:<file name>"`, the file name being `NiFilename`'s
/// `m_acFname` (+0x203, Xbox PDB) of `path`; an object without a name gets
/// `"_:<file name>"`. The 256-byte text is made into a handle by `00a5b690`,
/// assigned to the name by `0043ba10`, and the temporary handle is released
/// (`InterlockedDecrement` on the word 8 bytes before it) unless it is the
/// handle at `0109b220`.
pub fn fn_00c4bbc0(e: &mut Engine, object: Ptr, path: u32) {
    let name = e.mem.u32(object.addr() + 8);
    e.with_stack(0x404, |e, filename| {
        e.with_stack(0x100, |e, text| {
            e.with_stack(4, |e, handle_slot| {
                e.call(NI_FILENAME_CONSTRUCT, &args![filename, path]);
                if name != 0 {
                    e.call(STRCPY_S, &args![text, 0x100u32, name]);
                } else {
                    e.mem.set_u8(text.addr(), 0);
                }
                e.call(STRCAT_S, &args![text, 0x100u32, NAME_SEPARATOR]);
                e.call(STRCAT_S, &args![text, 0x100u32, filename.addr() + 0x203]);
                // The exe tests the buffer's (stack) address for null.
                let handle = if text.is_null() {
                    e.global::<u32>(FIXED_STRING_NONE)
                } else {
                    e.call(FIXED_STRING_CREATE, &args![text]).u32()
                };
                e.mem.set_u32(handle_slot.addr(), handle);
                e.call(FIXED_STRING_ASSIGN, &args![object.addr() + 8, handle_slot]);
                let handle = e.mem.u32(handle_slot.addr());
                if handle != e.global::<u32>(FIXED_STRING_NONE) {
                    e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
                }
            })
        })
    });
}

// Translated from 00c4bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks for `"_:"` in `object`'s name (+8) when it has one; the position
/// after it is computed and dropped. Always returns 0.
pub fn fn_00c4bd20(e: &mut Engine, object: Ptr) -> u32 {
    let name = e.mem.u32(object.addr() + 8);
    if name != 0 {
        let found = e.call(STRSTR, &args![name, NAME_SEPARATOR]).u32();
        if found != 0 {
            let _after_separator = found.wrapping_add(2);
        }
    }
    0
}

// Translated from 00c4bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a subtree and hands every texture of its flip controllers to
/// `00c4c8d0(visitor, &texture)`. An object's flip controller is its own
/// `NiFlipController`, or, when it has none and virtual slot +0x1C or +0x30
/// returns non-zero, the `NiFlipController` of its property of type 5 when
/// that is an `NiTexturingProperty`. Every non-null texture of the
/// controller (`00a4f300`, count at +0x42) is visited; then, for an
/// `NiNode`, the function recurses into each child slot.
pub fn fn_00c4bd80(e: &mut Engine, object: Ptr, visitor: u32) {
    if object.is_null() {
        return;
    }
    let object = object.addr();
    let mut controller = e
        .call(GET_CONTROLLER, &args![object, NI_FLIP_CONTROLLER_RTTI])
        .u32();
    if controller == 0
        && (e.vcall(object, 0x1c, &[]).u32() != 0 || e.vcall(object, 0x30, &[]).u32() != 0)
    {
        let property = e.call(GET_PROPERTY, &args![object, 5u32]).u32();
        let texturing = if property == 0 {
            0
        } else if is_kind_of(e, property, NI_TEXTURING_PROPERTY_RTTI) {
            property
        } else {
            0
        };
        if texturing != 0 {
            controller = e
                .call(GET_CONTROLLER, &args![texturing, NI_FLIP_CONTROLLER_RTTI])
                .u32();
        }
    }
    if controller != 0 {
        let count = e.mem.u16(controller.wrapping_add(0x42)) as u32;
        let mut i = 0u32;
        while i < count {
            let texture = e
                .call(FLIP_CONTROLLER_GET_TEXTURE, &args![controller, i])
                .u32();
            if texture != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), texture);
                    e.call(TEXTURE_VISIT, &args![visitor, slot]);
                });
            }
            i = i.wrapping_add(1);
        }
    }
    let node = if is_kind_of(e, object, NI_NODE_RTTI) {
        object
    } else {
        0
    };
    if node != 0 {
        let mut i = 0u32;
        while i < e.mem.u16(node + CHILDREN_SIZE) as u32 {
            let base = e.mem.u32(node + CHILDREN_BASE);
            let child = e.mem.u32(base.wrapping_add(4 * i));
            fn_00c4bd80(e, Ptr::new(child), visitor);
            i = i.wrapping_add(1);
        }
    }
}

/// The stack frame of the camera reflection (its locals are addressed from
/// the frame's end, as `EBP - offset`).
const REFLECT_FRAME: u32 = 0x1b0;

/// One axis of the reflected rotation: normalizes the point at `work`,
/// scales the plane normal by `2 * dot(normal, work)` (`004a3760` into
/// `temporary`) and writes `scaled normal - work` (`00439ef0`) into `out`.
fn reflect_axis(e: &mut Engine, plane: Ptr, work: u32, out: u32, temporary: u32) {
    e.call(POINT_UNITIZE, &args![work]);
    let dot = e.call(POINT_DOT, &args![plane, work]).f64();
    let two: f64 = e.global(TWO);
    let scale = quiet((dot * two) as f32);
    let scaled = e.call(SCALE_POINT, &args![temporary, scale, plane]).u32();
    e.call(POINT_SUBTRACT, &args![scaled, out, work]);
}

// Translated from 00c4bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSUtilities::ReflectCameraAboutArbitraryPlane` (Xbox PDB): sets `target`
/// to `source` mirrored in `plane` (an `NiPlane`: normal, then constant at
/// +0x0C). The world translation (+0x8C) is moved by `-normal * 2 *
/// (dot(normal, position) - constant)` into the target's local translation
/// (+0x58). The world rotation's (+0x68) column 0 negated, column 2 negated
/// and column 1 as is are each normalized and turned into `normal * 2 *
/// dot(normal, axis) - axis`, which become columns 0, 2 and 1 of the
/// target's local rotation (+0x34); the local scale (+0x64) is `|world
/// scale (+0x98)|`. Then the source's view frustum (+0xDC) is set on the
/// target, and the target is passed a record (`0043d410` with 0.0, 0, 0)
/// through `00a59c60`.
pub fn bs_utilities_reflect_camera_about_arbitrary_plane(
    e: &mut Engine,
    source: Ptr,
    plane: Ptr,
    target: Ptr,
) {
    e.with_stack(REFLECT_FRAME, |e, frame| {
        let ebp = frame.addr() + REFLECT_FRAME;
        let local = |offset: u32| ebp - offset;
        let source = source.addr();
        let plane_at = plane.addr();
        let target = target.addr();

        copy_words(e, local(0x18), source + 0x8c, 3);
        let rotation = source + 0x68;
        e.call(MATRIX_GET_COLUMN, &args![rotation, 0u32, local(0xf0)]);
        copy_words(e, local(0x60), local(0xf0), 3);
        e.call(MATRIX_GET_COLUMN, &args![rotation, 2u32, local(0xfc)]);
        copy_words(e, local(0x78), local(0xfc), 3);
        e.call(MATRIX_GET_COLUMN, &args![rotation, 1u32, local(0x108)]);
        copy_words(e, local(0x54), local(0x108), 3);

        // The signed distance of the position from the plane.
        let dot = e.call(POINT_DOT, &args![plane, local(0x18)]).f64();
        let constant = e.mem.f32(plane_at + 0x0c);
        let distance = (dot - constant as f64) as f32;
        e.mem.set_f32(local(0x10c), distance);
        for i in 0..3 {
            let value = negated(e.mem.f32(plane_at + 4 * i));
            e.mem.set_f32(local(0x118) + 4 * i, value);
            e.mem.set_f32(local(0xa8) + 4 * i, quiet(value));
        }
        let two: f64 = e.global(TWO);
        let scale = quiet((quiet(distance) as f64 * two) as f32);
        let moved = e
            .call(POINT_SCALE, &args![local(0xa8), local(0xb4), scale])
            .u32();
        e.call(POINT_ADD, &args![local(0x18), local(0x24), moved]);

        // Column 0, negated.
        for i in 0..3 {
            let value = negated(e.mem.f32(local(0x60) + 4 * i));
            e.mem.set_f32(local(0x13c) + 4 * i, value);
            e.mem.set_f32(local(0x30) + 4 * i, quiet(value));
        }
        reflect_axis(e, plane, local(0x30), local(0x6c), local(0xc0));
        // Column 2, negated.
        for i in 0..3 {
            let value = negated(e.mem.f32(local(0x78) + 4 * i));
            e.mem.set_f32(local(0x168) + 4 * i, value);
            e.mem.set_f32(local(0xcc) + 4 * i, quiet(value));
        }
        copy_words(e, local(0x30), local(0xcc), 3);
        reflect_axis(e, plane, local(0x30), local(0x48), local(0xd8));
        // Column 1.
        copy_words(e, local(0x30), local(0x54), 3);
        reflect_axis(e, plane, local(0x30), local(0x3c), local(0xe4));

        e.call(
            MATRIX_SET_COLUMNS,
            &args![local(0x9c), local(0x6c), local(0x3c), local(0x48)],
        );
        copy_words(e, target + 0x58, local(0x24), 3);
        copy_words(e, target + 0x34, local(0x9c), 9);
        let world_scale = e.mem.f32(source + 0x98);
        let scale = quiet(absolute(e, world_scale));
        e.mem.set_f32(target + 0x64, scale);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![target, source + 0xdc]);
        e.call(RECORD_INIT, &args![local(0x0c), 0.0f32, 0u32, 0u32]);
        e.call(AV_OBJECT_UPDATE_WITH, &args![target, local(0x0c)]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4b470,
            bs_utilities_get_object_by_name(Ptr, Ptr, bool) -> Ptr
        ),
        entry!(
            0x00c4b500,
            bs_utilities_process_selective_update(Ptr<NiAVObject>)
        ),
        entry!(
            0x00c4b600,
            bs_utilities_get_rotation_z_to_vector(Ptr, f32, f32, f32) -> Ptr
        ),
        entry!(
            0x00c4b8a0,
            bs_utilities_get_rotation_z_to_vector_transpose(Ptr, f32, f32, f32) -> Ptr
        ),
        entry!(0x00c4bb40, bs_utilities_get_texture_name(Ptr) -> u32),
        entry!(0x00c4bbc0, fn_00c4bbc0(Ptr, u32)),
        entry!(0x00c4bd20, fn_00c4bd20(Ptr) -> u32),
        entry!(0x00c4bd80, fn_00c4bd80(Ptr, u32)),
        entry!(
            0x00c4bf40,
            bs_utilities_reflect_camera_about_arbitrary_plane(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An object whose `GetRTTI` (slot +0x08) returns `rtti`, with further
    /// slots as given (slot offset, target).
    fn object_with(e: &mut Engine, vtable: u32, rtti_getter: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0xb0);
        let mut table = vec![0u32; 0x28];
        table[2] = rtti_getter;
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn get_object_by_name_asks_the_manager_then_the_root() {
        let mut e = Engine::new();
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x55);
        // The palette's slot +0x8c answers 0x1234; the root's slot +0x9c 0x5678.
        e.register(0x0050_0000, |_, _| ret(0x1234));
        e.register(0x0050_0004, |_, _| ret(0x5678));
        e.register(0x0050_0008, |_, _| ret(0));
        let root = object_with(&mut e, 0x0090_0000, 0, &[(0x9c, 0x0050_0004)]);
        let palette = object_with(&mut e, 0x0090_1000, 0, &[(0x8c, 0x0050_0000)]);
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(manager + 0x78, palette);
        e.register_double(GET_CONTROLLER, move |_, a| {
            assert_eq!(a[1], NI_CONTROLLER_MANAGER_RTTI);
            ret(manager)
        });
        let found = e.call(0x00c4_b470, &args![root, name, 0u32]).u32();
        assert_eq!(found, 0x1234);

        // The palette finds nothing: only with the flag is the root asked.
        let empty = object_with(&mut e, 0x0090_2000, 0, &[(0x8c, 0x0050_0008)]);
        e.mem.set_u32(manager + 0x78, empty);
        assert_eq!(e.call(0x00c4_b470, &args![root, name, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00c4_b470, &args![root, name, 1u32]).u32(), 0x5678);

        // No manager: straight to the root.
        e.register(GET_CONTROLLER, |_, _| ret(0));
        assert_eq!(e.call(0x00c4_b470, &args![root, name, 1u32]).u32(), 0x5678);

        // Null root or empty name.
        assert_eq!(e.call(0x00c4_b470, &args![0u32, name, 1u32]).u32(), 0);
        e.mem.set_u32(name, 0);
        assert_eq!(e.call(0x00c4_b470, &args![root, name, 1u32]).u32(), 0);
    }

    #[test]
    fn process_selective_update_sets_flags_down_the_tree() {
        let mut e = Engine::new();
        // Slot +0x0c returns the object itself for a node, 0 for a leaf.
        e.register(0x0050_0010, |_, a| ret(a[0]));
        e.register(0x0050_0014, |_, _| ret(0));
        let node = object_with(&mut e, 0x0090_0000, 0, &[(0x0c, 0x0050_0010)]);
        let leaf = object_with(&mut e, 0x0090_1000, 0, &[(0x0c, 0x0050_0014)]);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, leaf);
        e.mem.set_u32(children + 4, 0);
        e.mem.set_u32(node + CHILDREN_BASE, children);
        e.mem.set_u16(node + CHILDREN_SIZE, 2);
        e.mem.set_u32(node + 0x30, 0x11);
        e.mem.set_u32(leaf + 0x30, 0x10);
        e.call(0x00c4_b500, &args![node]);
        assert_eq!(e.mem.u32(node + 0x30), 0x07);
        assert_eq!(e.mem.u32(leaf + 0x30), 0x06);
        // Null does nothing.
        e.call(0x00c4_b500, &args![0u32]);
    }

    /// Doubles for the point helpers: unitize (divide by the length), the
    /// cross product into `out`, and `fabs`.
    fn point_engine() -> Engine {
        let mut e = Engine::new();
        e.register(POINT_UNITIZE, |e, a| {
            let p = a[0];
            let (x, y, z) = (e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8));
            let length = (x * x + y * y + z * z).sqrt();
            if length > 0.0 {
                e.mem.set_f32(p, x / length);
                e.mem.set_f32(p + 4, y / length);
                e.mem.set_f32(p + 8, z / length);
            }
            Ret::default()
        });
        e.register(POINT_CROSS, |e, a| {
            let (p, out, q) = (a[0], a[1], a[2]);
            let (ax, ay, az) = (e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8));
            let (bx, by, bz) = (e.mem.f32(q), e.mem.f32(q + 4), e.mem.f32(q + 8));
            e.mem.set_f32(out, ay * bz - az * by);
            e.mem.set_f32(out + 4, az * bx - ax * bz);
            e.mem.set_f32(out + 8, ax * by - ay * bx);
            ret(out)
        });
        e.register(FABS, |_, a| f64::take(a, &mut 0).abs().into_ret());
        e
    }

    fn matrix(e: &Engine, at: u32) -> [f32; 9] {
        let mut m = [0.0; 9];
        for (i, value) in m.iter_mut().enumerate() {
            *value = e.mem.f32(at + 4 * i as u32);
        }
        m
    }

    #[test]
    fn rotation_z_to_vector_picks_the_axis_of_the_smallest_component() {
        let mut e = point_engine();
        let result = e.mem.alloc(0x24);
        // z is the vector: x is smallest (ties go to x): first axis (0,-1,0).
        let back = e.call(0x00c4_b600, &args![result, 0.0f32, 0.0f32, 1.0f32]);
        assert_eq!(back.u32(), result);
        // first = (0,-1,0); second = first × v = (-1,0,0); first = second × v = (0,1,0).
        assert_eq!(
            matrix(&e, result),
            [0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        // Along x: y is the smallest strictly below x (|y| < |x|), z ties:
        // the second test picks (-z, 0, x) = (0, 0, 1) when |x| >= |y| and |z| >= |y|.
        e.call(0x00c4_b600, &args![result, 1.0f32, 0.0f32, 0.0f32]);
        let m = matrix(&e, result);
        assert_eq!(&m[6..], &[1.0, 0.0, 0.0]);
        // first (-0,0,1) → second = first × v = (0,1,0) → first = second × v = (0,0,-1).
        assert_eq!(&m[3..6], &[0.0, 1.0, 0.0]);
        assert_eq!(m[2], -1.0);
    }

    /// The first axis handed to the first cross product, per input vector.
    fn first_axis(x: f32, y: f32, z: f32) -> [u32; 3] {
        let mut e = point_engine();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let record = seen.clone();
        e.register_double(POINT_CROSS, move |e, a| {
            if record.borrow().is_none() {
                *record.borrow_mut() =
                    Some([e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)]);
            }
            ret(a[1])
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        let result = e.mem.alloc(0x24);
        e.call(0x00c4_b600, &args![result, x, y, z]);
        seen.take().unwrap()
    }

    #[test]
    fn rotation_z_to_vector_first_axis_cases() {
        // (0, 0, 1): x is the smallest (ties included): (0, -z, y).
        assert_eq!(first_axis(0.0, 0.0, 1.0), [0, (-1.0f32).to_bits(), 0]);
        // (1, 0, 0): y smallest: (-z, 0, x).
        assert_eq!(
            first_axis(1.0, 0.0, 0.0),
            [(-0.0f32).to_bits(), 0, 1.0f32.to_bits()]
        );
        // (0.6, 0.8, 0): z smallest: (-y, x, 0).
        assert_eq!(
            first_axis(0.6, 0.8, 0.0),
            [(-0.8f32).to_bits(), 0.6f32.to_bits(), 0]
        );
        // A NaN makes the comparisons unordered: the third case, with the
        // NaN copied through the x87 stack quieted.
        let signalling = f32::from_bits(0x7f80_0001);
        let axis = first_axis(signalling, 0.0, 1.0);
        assert_eq!(axis[0], (-0.0f32).to_bits());
        assert_eq!(axis[1], 0x7fc0_0001);
        assert_eq!(axis[2], 0);
    }

    #[test]
    fn rotation_z_to_vector_transpose_stores_columns() {
        let mut e = point_engine();
        let rows = e.mem.alloc(0x24);
        let columns = e.mem.alloc(0x24);
        e.call(0x00c4_b600, &args![rows, 0.3f32, -0.4f32, 0.8f32]);
        e.call(0x00c4_b8a0, &args![columns, 0.3f32, -0.4f32, 0.8f32]);
        let (r, c) = (matrix(&e, rows), matrix(&e, columns));
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(r[3 * i + j].to_bits(), c[3 * j + i].to_bits());
            }
        }
    }

    #[test]
    fn get_texture_name_reads_source_textures_only() {
        let mut e = Engine::new();
        // RTTI chain: derived -> NiSourceTexture.
        e.map(0x011f_4000, 0x1000);
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, NI_SOURCE_TEXTURE_RTTI);
        let derived_rtti = derived;
        e.register_double(0x0050_0020, move |_, _| ret(derived_rtti));
        e.register(0x0050_0024, |_, _| ret(NI_NODE_RTTI));
        let texture = object_with(&mut e, 0x0090_0000, 0x0050_0020, &[]);
        e.mem.set_u32(texture + 0x34, 0xabcd);
        assert_eq!(e.call(0x00c4_bb40, &args![texture]).u32(), 0xabcd);
        let other = object_with(&mut e, 0x0090_1000, 0x0050_0024, &[]);
        e.mem.set_u32(other + 0x34, 0xabcd);
        assert_eq!(e.call(0x00c4_bb40, &args![other]).u32(), 0);
        assert_eq!(e.call(0x00c4_bb40, &args![0u32]).u32(), 0);
    }

    #[test]
    fn rename_builds_name_separator_file_name() {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_NONE, 0x7777u32);
        e.register(NI_FILENAME_CONSTRUCT, |e, a| {
            e.mem.set_cstr(a[0] + 0x203, b"rock");
            Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.map(NAME_SEPARATOR & !0xfff, 0x1000);
        e.mem.set_cstr(NAME_SEPARATOR, b"_:");
        e.register(FIXED_STRING_CREATE, |e, a| {
            let text = e.mem.cstr(a[0]);
            let block = e.mem.alloc(0x40);
            e.mem.set_cstr(block + 8, &text);
            ret(block + 8)
        });
        e.register(FIXED_STRING_ASSIGN, |e, a| {
            let handle = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], handle);
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(0));
        let object = e.mem.alloc(0x10);
        let old = e.mem.alloc(8);
        e.mem.set_cstr(old, b"Box");
        e.mem.set_u32(object + 8, old);
        e.call_log = Some(vec![]);
        e.call(0x00c4_bbc0, &args![object, 0x1234u32]);
        let new = e.mem.u32(object + 8);
        assert_eq!(e.mem.cstr(new), b"Box_:rock");
        let log = e.call_log.take().unwrap();
        let release = log
            .iter()
            .find(|(a, _)| *a == INTERLOCKED_DECREMENT)
            .unwrap();
        assert_eq!(release.1[0], new - 8);

        // No name: the text starts with the separator; the "none" handle
        // is not released.
        e.mem.set_u32(object + 8, 0);
        e.register(FIXED_STRING_CREATE, |_, _| ret(0x7777));
        e.call_log = Some(vec![]);
        e.call(0x00c4_bbc0, &args![object, 0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != INTERLOCKED_DECREMENT));
        assert!(log.iter().all(|(a, _)| *a != STRCPY_S));
        assert_eq!(e.mem.u32(object + 8), 0x7777);
    }

    #[test]
    fn find_separator_always_returns_zero() {
        let mut e = Engine::new();
        e.register(STRSTR, |_, a| ret(a[0] + 3));
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_bd20, &args![object]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u32(object + 8, 0x4444);
        assert_eq!(e.call(0x00c4_bd20, &args![object]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (STRSTR, vec![0x4444, NAME_SEPARATOR]));
    }

    #[test]
    fn flip_controller_textures_are_visited_down_the_tree() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        // Leaf: no own controller, slot +0x1c says yes, its property 5 is a
        // texturing property with a flip controller of two textures (one null).
        e.register(0x0050_0030, |_, _| ret(NI_NODE_RTTI));
        e.register(0x0050_0034, |_, _| ret(NI_TEXTURING_PROPERTY_RTTI));
        e.register(0x0050_0038, |_, _| ret(0));
        e.register(0x0050_003c, |_, _| ret(1));
        let node = object_with(
            &mut e,
            0x0090_0000,
            0x0050_0030,
            &[(0x1c, 0x0050_0038), (0x30, 0x0050_0038)],
        );
        let leaf = object_with(
            &mut e,
            0x0090_1000,
            0x0050_0038,
            &[(0x1c, 0x0050_003c), (0x30, 0x0050_0038)],
        );
        let property = object_with(&mut e, 0x0090_2000, 0x0050_0034, &[]);
        let controller = e.mem.alloc(0x50);
        e.mem.set_u16(controller + 0x42, 2);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, leaf);
        e.mem.set_u32(node + CHILDREN_BASE, children);
        e.mem.set_u16(node + CHILDREN_SIZE, 1);
        e.register_double(GET_CONTROLLER, move |_, a| {
            ret(if a[0] == property { controller } else { 0 })
        });
        e.register_double(GET_PROPERTY, move |_, a| {
            assert_eq!(a[1], 5);
            ret(property)
        });
        e.register(FLIP_CONTROLLER_GET_TEXTURE, |_, a| {
            ret(if a[1] == 0 { 0x6000 } else { 0 })
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(TEXTURE_VISIT, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x00c4_bd80, &args![node, 0x99u32]);
        assert_eq!(*seen.borrow(), vec![(0x99, 0x6000)]);
    }

    #[test]
    fn reflect_camera_mirrors_position_rotation_and_scale() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        // Column i of the source rotation is (i+1, 0, 0) etc.: a test pattern.
        e.register(MATRIX_GET_COLUMN, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 12 * k + 4 * a[1]);
                e.mem.set_f32(a[2] + 4 * k, v);
            }
            Ret::default()
        });
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f64;
            for k in 0..3 {
                sum += e.mem.f32(a[0] + 4 * k) as f64 * e.mem.f32(a[1] + 4 * k) as f64;
            }
            sum.into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let s = f32::from_bits(a[2]);
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) * s;
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            ret(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) + e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            ret(a[1])
        });
        e.register(SCALE_POINT, |e, a| {
            let s = f32::from_bits(a[1]);
            for k in 0..3 {
                let v = e.mem.f32(a[2] + 4 * k) * s;
                e.mem.set_f32(a[0] + 4 * k, v);
            }
            ret(a[0])
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            ret(a[1])
        });
        e.register(MATRIX_SET_COLUMNS, |e, a| {
            for c in 0..3 {
                for k in 0..3 {
                    let v = e.mem.f32(a[1 + c as usize] + 4 * k);
                    e.mem.set_f32(a[0] + 12 * k + 4 * c, v);
                }
            }
            Ret::default()
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        e.register(FABS, |_, a| f64::take(a, &mut 0).abs().into_ret());
        e.register(CAMERA_SET_VIEW_FRUSTUM, |_, _| Ret::default());
        e.register(RECORD_INIT, |e, a| {
            for k in 0..3 {
                e.mem.set_u32(a[0] + 4 * k as u32, a[1 + k]);
            }
            Ret::default()
        });
        e.register(AV_OBJECT_UPDATE_WITH, |_, _| Ret::default());

        let source = e.mem.alloc(0x100);
        let target = e.mem.alloc(0x100);
        let plane = e.mem.alloc(0x10);
        // The plane z = 1 (normal (0,0,1), constant 1).
        e.mem.set_f32(plane + 8, 1.0);
        e.mem.set_f32(plane + 0x0c, 1.0);
        // Position (2, 3, 5): its mirror is (2, 3, -3).
        e.mem.set_f32(source + 0x8c, 2.0);
        e.mem.set_f32(source + 0x90, 3.0);
        e.mem.set_f32(source + 0x94, 5.0);
        // Identity rotation.
        for k in 0..3 {
            e.mem.set_f32(source + 0x68 + 16 * k, 1.0);
        }
        e.mem.set_f32(source + 0x98, -1.5);
        e.call_log = Some(vec![]);
        e.call(0x00c4_bf40, &args![source, plane, target]);
        assert_eq!(
            [
                e.mem.f32(target + 0x58),
                e.mem.f32(target + 0x5c),
                e.mem.f32(target + 0x60)
            ],
            [2.0, 3.0, -3.0]
        );
        // Columns: from -x: (1,0,0); from y: (0,-1,0); from -z: (0,0,-1).
        assert_eq!(
            matrix(&e, target + 0x34),
            [1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0]
        );
        assert_eq!(e.mem.f32(target + 0x64), 1.5);
        let log = e.call_log.take().unwrap();
        let frustum = log
            .iter()
            .find(|(a, _)| *a == CAMERA_SET_VIEW_FRUSTUM)
            .unwrap();
        assert_eq!(frustum.1, vec![target, source + 0xdc]);
        let set = log.iter().find(|(a, _)| *a == RECORD_INIT).unwrap();
        assert_eq!(&set.1[1..], &[0, 0, 0]);
        let last = log.last().unwrap();
        assert_eq!(last.0, AV_OBJECT_UPDATE_WITH);
        assert_eq!(last.1[1], set.1[0]);
    }
}
