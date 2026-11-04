//! `fallout shared/bgsprimitive.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004a7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall` on a `float[3]`: returns its squared length
/// `z*z + y*y + x*x` in ST0, stored to a `float` local and reloaded. Computed
/// in `f64` and rounded to `f32` once, as the crate does for x87 code.
pub fn fn_004a7290(e: &mut Engine, this: Ptr) -> f32 {
    let x = e.mem.f32(this.addr()) as f64;
    let y = e.mem.f32(this.addr() + 4) as f64;
    let z = e.mem.f32(this.addr() + 8) as f64;
    (x * x + y * y + z * z) as f32
}

/// The function `fn_004a7040` calls on the member (`00457990`).
const CALLEE_00457990: u32 = 0x0045_7990;

// Translated from 004a7040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSMultiBoundAABB::QMultiBoundRadius` (Xbox PDB name in the engine map). Thin wrapper: calls `00457990` with `ECX` = `this + 0x18` and passes on its result (`ST0`).
pub fn fn_004a7040(e: &mut Engine, this: Ptr) -> f64 {
    e.call(CALLEE_00457990, &args![this.byte_add(0x18)]).f64()
}

// Translated from 004a71f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004a7220` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004a71f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004a_7220, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004a51d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the address of three
/// words and a float): copies the three words to `+0..+8` and the float to
/// `+0xc` through `FLD`/`FSTP` (a signalling NaN is quieted).
pub fn fn_004a51d0(e: &mut Engine, this: Ptr, point: Ptr, value: f32) {
    let t = this.addr();
    let p = point.addr();
    for word in 0..3u32 {
        let v = e.mem.u32(p + 4 * word);
        e.mem.set_u32(t + 4 * word, v);
    }
    e.mem.set_u32(t + 0xc, x87_quiet_bits(value.to_bits()));
}

/// `00414430` (`thiscall`, `RET 0x10`: four floats): sets a colour.
const COLOR_SET: u32 = 0x0041_4430;

// Translated from 004a7800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `00414430` on
/// `this` with four 0.0 floats.
pub fn fn_004a7800(e: &mut Engine, this: Ptr) {
    let zero = x87_store32(0.0);
    e.call(COLOR_SET, &args![this, zero, zero, zero, zero]);
}

// Translated from 004a51b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: an index): the address of
/// the word at that index (`this + index * 4`).
pub fn fn_004a51b0(_e: &mut Engine, this: Ptr, index: u32) -> u32 {
    this.addr().wrapping_add(index.wrapping_mul(4))
}

// Translated from 004a4f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a two-float point, `RET 8`: the
/// result's address and a float): both coordinates times the float (`float`
/// stores) into the result, which is answered.
pub fn fn_004a4f40(e: &mut Engine, this: Ptr, out: Ptr, scale: f32) -> Ptr {
    let x = e.mem.u32(this.addr());
    let y = e.mem.u32(this.addr() + 4);
    let x = x87_store32(x87_mul(x87_load32(x), x87_m32(scale.to_bits())));
    let y = x87_store32(x87_mul(x87_load32(y), x87_m32(scale.to_bits())));
    e.mem.set_u32(out.addr(), x);
    e.mem.set_u32(out.addr() + 4, y);
    out
}

// Translated from 004a7ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::SetCenter` (engine map name), `thiscall`, `RET 4`
/// (the centre's address): copies its three words to `+0xc..+0x14`.
pub fn bs_multi_bound_sphere_set_center(e: &mut Engine, this: Ptr, center: Ptr) {
    for word in 0..3u32 {
        let v = e.mem.u32(center.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0xc + 4 * word, v);
    }
}

/// `NiObject`'s destructor body (`00a5d3d0`, `thiscall`, no stack words).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;

// Translated from 004a7090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs `NiObject`'s
/// destructor body (`00a5d3d0`) on `this`.
pub fn fn_004a7090(e: &mut Engine, this: Ptr) {
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 004a7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `004a7090`.
pub fn fn_004a7c20(e: &mut Engine, this: Ptr) {
    fn_004a7090(e, this);
}

/// `004a4bd0` (`thiscall`, no stack words): the destructor body `004a7220`
/// forwards to.
const DESTRUCT_004A4BD0: u32 = 0x004a_4bd0;

// Translated from 004a7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `004a4bd0`.
pub fn fn_004a7220(e: &mut Engine, this: Ptr) {
    e.call(DESTRUCT_004A4BD0, &args![this]);
}

/// The sized free `00aa1460` (`cdecl`: the block and its size).
const SIZED_FREE: u32 = 0x00aa_1460;

// Translated from 004a5250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: another four-word
/// record): `004a51d0` with its first three words and its float at `+0xc`
/// (pushed through `FLD`/`FSTP`). Answers `this`.
pub fn fn_004a5250(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(other.addr() + 0xc));
    fn_004a51d0(e, this, other, f32::from_bits(value));
    this
}

// Translated from 004a7060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::_scalar_deleting_destructor_` (engine map name),
/// `thiscall`, `RET 4` (the flags): `004a7c20`, then with bit 0 the sized free
/// of 0x24 bytes. Answers `this`.
pub fn bs_multi_bound_aabb_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004a7c20(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x24u32]);
    }
    this
}

// Translated from 004a6110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::_scalar_deleting_destructor_` (engine map name),
/// `thiscall`, `RET 4`: `004a7090`, then with bit 0 the sized free of 0xfc
/// bytes. Answers `this`.
pub fn bs_occlusion_plane_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004a7090(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0xfcu32]);
    }
    this
}

// Translated from 004a7bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::_scalar_deleting_destructor_` (engine map name),
/// `thiscall`, `RET 4`: `004a7c20`, then with bit 0 the sized free of 0x1c
/// bytes. Answers `this`.
pub fn bs_multi_bound_sphere_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004a7c20(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 004a4cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on four floats, `RET 4`: another
/// four): whether all four are equal (`FUCOMPP`, `TEST AH,0x44`; a NaN is
/// never equal), compared in order and stopping at the first difference.
pub fn fn_004a4cc0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    (0..4u32).all(|i| {
        x87_m32(e.mem.u32(other.addr() + 4 * i)) == x87_m32(e.mem.u32(this.addr() + 4 * i))
    })
}

/// The float 2.0 (`010162c0`) and 0.5 (`01016248`), the double 0.5
/// (`01011588`) and 0.001 (`0101e788`).
const FLOAT_TWO: u32 = 0x0101_62c0;
const FLOAT_HALF: u32 = 0x0101_6248;
const DOUBLE_HALF: u32 = 0x0101_1588;
const DOUBLE_THOUSANDTH: u32 = 0x0101_e788;
/// `00452dc0` (`thiscall` on a two-float point, `RET 8`: two floats; answers
/// the point), `0045bb20` (`thiscall` on a point, `RET 8`: the result's
/// address and a float) and `004a5100` (`cdecl`: a float and a word; answers
/// the made primitive).
const POINT2_SET: u32 = 0x0045_2dc0;
const POINT3_SCALED: u32 = 0x0045_bb20;
const MAKE_004A5100: u32 = 0x004a_5100;

// Translated from 004a4e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: the primitive type, the
/// address of its size and a word): makes a primitive. Type 1: the size
/// times 2.0 (`0045bb20`) into a box (`004a5040`); type 2: `004a5100` with
/// the first float; type 3: the first and third floats as a two-float point
/// (`00452dc0`) times 2.0 (`004a4f40`) into a plane (`004a4f80`); any other
/// answers 0. Floats go through `FLD`/`FSTP` as they are pushed.
pub fn fn_004a4e90(e: &mut Engine, kind: u32, size: Ptr, word: u32) -> u32 {
    let s = size.addr();
    e.with_stack(0x1c, |e, frame| {
        // The frame: the two-float point (+0), the scaled size (+8), the
        // scaled point (+0x14).
        let point = frame.addr();
        let scaled = point + 8;
        let half_size = point + 0x14;
        let two = x87_quiet_bits(e.mem.u32(FLOAT_TWO));
        match kind {
            1 => {
                e.call(POINT3_SCALED, &args![s, scaled, two]);
                fn_004a5040(e, Ptr::new(scaled), word)
            }
            2 => {
                let first = x87_quiet_bits(e.mem.u32(s));
                e.call(MAKE_004A5100, &args![first, word]).u32()
            }
            3 => {
                let x = x87_quiet_bits(e.mem.u32(s));
                let z = x87_quiet_bits(e.mem.u32(s + 8));
                let made = e.call(POINT2_SET, &args![point, x, z]).u32();
                fn_004a4f40(e, Ptr::new(made), Ptr::new(half_size), f32::from_bits(two));
                fn_004a4f80(e, Ptr::new(half_size), word)
            }
            _ => 0,
        }
    })
}

/// The allocation `00401000` (`cdecl`: the size).
const ALLOCATE_00401000: u32 = 0x0040_1000;

// Translated from 004a4f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4f80 - the C++ exception-unwinding frame (FS:[0] chain, handler 00ef8c3b, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map (`cdecl`, two words: the address of a two-float
/// size and a word): a `BGSPrimitivePlane` (0x34 bytes, `00401000`, built by
/// `004a5280`; a failed allocation is used as it is) gets the word through
/// its vtable slot `+8`, half of each size coordinate at `+0x18` and `+0x20`
/// (times the double 0.5), 1.0 at `+0x1c` and the type 3 at `+4`. Answers it.
pub fn fn_004a4f80(e: &mut Engine, size: Ptr, word: u32) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x34u32]).u32();
    let plane = if block != 0 {
        fn_004a5280(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.vcall(plane, 8, &args![word]);
    let half = x87_load64(e.mem.u64(DOUBLE_HALF));
    let x = x87_store32(x87_mul(x87_load32(e.mem.u32(size.addr())), half));
    e.mem.set_u32(plane.wrapping_add(0x18), x);
    e.mem.set_u32(plane.wrapping_add(0x1c), x87_store32(1.0));
    let y = x87_store32(x87_mul(x87_load32(e.mem.u32(size.addr() + 4)), half));
    e.mem.set_u32(plane.wrapping_add(0x20), y);
    e.mem.set_u32(plane.wrapping_add(4), 3);
    plane
}

// Translated from 004a5040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5040 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eefe9b, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map (`cdecl`, two words: the address of a
/// three-float size and a word): a `BGSPrimitiveBox` (0x4c bytes, built by
/// `004a6370`; a failed allocation is used as it is) gets the word (slot
/// `+8`) and half the size (`0045bb20` with 0.5, through slot `+0xc`), and the
/// type 1 at `+4`. Answers it.
pub fn fn_004a5040(e: &mut Engine, size: Ptr, word: u32) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x4cu32]).u32();
    let shape = if block != 0 {
        fn_004a6370(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.vcall(shape, 8, &args![word]);
    e.with_stack(0xc, |e, half| {
        let factor = x87_quiet_bits(e.mem.u32(FLOAT_HALF));
        e.call(POINT3_SCALED, &args![size, half, factor]);
        e.vcall(shape, 0xc, &args![half]);
    });
    e.mem.set_u32(shape.wrapping_add(4), 1);
    shape
}

/// `BGSPrimitive`'s vtable (`0101e72c`), `BGSPrimitivePlane`'s (`0101e75c`)
/// and `BGSPrimitiveBox`'s (`0101e8c4`), by their RTTI.
const BGS_PRIMITIVE_VTABLE: u32 = 0x0101_e72c;
const BGS_PRIMITIVE_PLANE_VTABLE: u32 = 0x0101_e75c;
const BGS_PRIMITIVE_BOX_VTABLE: u32 = 0x0101_e8c4;
/// `006815c0` (`thiscall`, no stack words), the `NiPointer` constructor from
/// a raw pointer `00633c90` and the `NiPointer` assignment `0066b0d0` (both
/// `thiscall`, `RET 4`).
const MEMBER_18_SET_UP: u32 = 0x0068_15c0;
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
const NI_POINTER_SET: u32 = 0x0066_b0d0;

// Translated from 004a4ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4ac0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eefe71, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map: `BGSPrimitive`'s constructor (by its vtable's
/// RTTI; `thiscall`, no stack words). The vtable, the colour at `+8` (four
/// 0.0 through `00414430`), `006815c0` on `+0x18`, the three `NiPointer`s at
/// `+0x24`, `+0x28`, `+0x2c` made null (`00633c90`) and assigned null again
/// (`0066b0d0`, in the order `+0x24`, `+0x2c`, `+0x28`), and 7 at `+0x30`.
/// Answers `this`.
pub fn fn_004a4ac0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, BGS_PRIMITIVE_VTABLE);
    let zero = x87_store32(0.0);
    e.call(COLOR_SET, &args![t + 8, zero, zero, zero, zero]);
    e.call(MEMBER_18_SET_UP, &args![t + 0x18]);
    for at in [0x24u32, 0x28, 0x2c] {
        e.call(NI_POINTER_FROM_RAW, &args![t + at, 0u32]);
    }
    for at in [0x24u32, 0x2c, 0x28] {
        e.call(NI_POINTER_SET, &args![t + at, 0u32]);
    }
    e.mem.set_u32(t + 0x30, 7);
    this
}

// Translated from 004a5280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5280 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eeffd8, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map: `BGSPrimitivePlane`'s constructor (`thiscall`,
/// no stack words): `004a4ac0`, the class's vtable, the pointers at `+0x24`
/// and `+0x2c` assigned null, the type 3 at `+4`. Answers `this`.
pub fn fn_004a5280(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    fn_004a4ac0(e, this);
    e.mem.set_u32(t, BGS_PRIMITIVE_PLANE_VTABLE);
    e.call(NI_POINTER_SET, &args![t + 0x24, 0u32]);
    e.call(NI_POINTER_SET, &args![t + 0x2c, 0u32]);
    e.mem.set_u32(t + 4, 3);
    this
}

// Translated from 004a6370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a6370 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eeffd8, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map: `BGSPrimitiveBox`'s constructor (`thiscall`, no
/// stack words): `004a4ac0`, the class's vtable, the pointers at `+0x24` and
/// `+0x2c` assigned null, the type 1 at `+4`. Answers `this`.
pub fn fn_004a6370(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    fn_004a4ac0(e, this);
    e.mem.set_u32(t, BGS_PRIMITIVE_BOX_VTABLE);
    e.call(NI_POINTER_SET, &args![t + 0x24, 0u32]);
    e.call(NI_POINTER_SET, &args![t + 0x2c, 0u32]);
    e.mem.set_u32(t + 4, 1);
    this
}

/// The `NiPointer` read `00559450` (`thiscall`; the object), the Gamebryo
/// allocation `00aa13e0` (`cdecl`), `BSFadeNode::BSFadeNode` (`00b4e150`,
/// `thiscall`), the fixed-string constructor `00438170` (`thiscall`, `RET 4`:
/// the text) and destructor `004381b0`, `00a5b950` (`thiscall` on the node,
/// `RET 4`: the name) and `NiAVObject::UpdateProperties` (`00a5a040`).
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const FADE_NODE_CONSTRUCT: u32 = 0x00b4_e150;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const NODE_SET_NAME: u32 = 0x00a5_b950;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `"Plane"` (`0101e77c`) and the box's name (`0101e8dc`) in `.rdata`.
const PLANE_NAME: u32 = 0x0101_e77c;
const BOX_NAME: u32 = 0x0101_e8dc;

/// The shared node build of the plane and the box: a new `BSFadeNode`
/// (0xe4 bytes; null when the allocation fails) into the pointer at `+0x2c`,
/// named `name` through a temporary fixed string, then its properties
/// updated.
fn build_node(e: &mut Engine, this: Ptr, name: u32) {
    let t = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0xe4u32]).u32();
    let node = if block != 0 {
        e.call(FADE_NODE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_SET, &args![t + 0x2c, node]);
    e.with_stack(4, |e, text| {
        let made = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.call(NODE_SET_NAME, &args![node, made]);
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
    });
    let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
    e.call(UPDATE_PROPERTIES, &args![node]);
}

// Translated from 004a5300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5300 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eefeda, state variable) and its stack cookie word are not translated
/// `BGSPrimitivePlane::Build` (engine map name), `thiscall`, no stack words:
/// a new `BSFadeNode` named `"Plane"` into `+0x2c`, its properties updated.
pub fn bgs_primitive_plane_build(e: &mut Engine, this: Ptr) {
    build_node(e, this, PLANE_NAME);
}

// Translated from 004a63f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a63f0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00eefeda, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map (the box's build, `thiscall`, no stack words):
/// only without a node at `+0x2c`, a new `BSFadeNode` named with `0101e8dc`,
/// its properties updated.
pub fn fn_004a63f0(e: &mut Engine, this: Ptr) {
    if e.call(NI_POINTER_GET, &args![this.addr() + 0x2c]).u32() == 0 {
        build_node(e, this, BOX_NAME);
    }
}

/// `005495f0` (`thiscall` on a geometry; its data), `0045cd60` (`thiscall`
/// on the data; its colour array), `NiGeometryData::MarkAsChanged`
/// (`00a67090`, `RET 4`), `TESActorBaseData::GetFatigue` (engine map name of
/// `004a8ae0`, here the data's 16-bit vertex count) and `00450f90`
/// (`thiscall`, `RET 4`: a byte; the alpha switch).
const GEOMETRY_DATA: u32 = 0x0054_95f0;
const DATA_COLORS: u32 = 0x0045_cd60;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;
const VERTEX_COUNT: u32 = 0x004a_8ae0;
const SET_ALPHA_00450F90: u32 = 0x0045_0f90;

/// Copies the four colour words at `from` to `to`.
fn copy_color(e: &mut Engine, to: u32, from: u32) {
    for word in 0..4u32 {
        let v = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, v);
    }
}

/// The alpha switch both colour setters end with: `00450f90` on the object
/// at `+0x28` with whether the colour's alpha is below 0.001.
fn set_alpha(e: &mut Engine, this: Ptr, color: u32) {
    let alpha = x87_m32(e.mem.u32(color + 0xc));
    let clear = alpha < x87_load64(e.mem.u64(DOUBLE_THOUSANDTH));
    let object = e.call(NI_POINTER_GET, &args![this.addr() + 0x28]).u32();
    e.call(SET_ALPHA_00450F90, &args![object, u32::from(clear)]);
}

// Translated from 004a53e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitivePlane::SetColor` (engine map name), `thiscall`, `RET 4` (the
/// colour's address): the colour into `+8`; with a geometry at `+0x24` its
/// data's eight colours are set from it (the last first, each copied from
/// the one after it) and marked changed (4); with an object at `+0x28` the
/// alpha switch.
pub fn bgs_primitive_plane_set_color(e: &mut Engine, this: Ptr, color: Ptr) {
    let t = this.addr();
    copy_color(e, t + 8, color.addr());
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let colors = e.call(DATA_COLORS, &args![data]).u32();
        copy_color(e, colors + 0x70, t + 8);
        for to in (0..7u32).rev() {
            copy_color(e, colors + 0x10 * to, colors + 0x10 * (to + 1));
        }
        e.call(MARK_AS_CHANGED, &args![data, 4u32]);
    }
    if e.call(NI_POINTER_GET, &args![t + 0x28]).u32() != 0 {
        set_alpha(e, this, color.addr());
    }
}

/// Sets every vertex colour of the geometry in the pointer at `slot` from the
/// colour at `this + 8` and marks the data changed (4).
fn paint_geometry(e: &mut Engine, this: Ptr, slot: u32) {
    let geometry = e.call(NI_POINTER_GET, &args![slot]).u32();
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    let colors = e.call(DATA_COLORS, &args![data]).u32();
    let mut index: i32 = 0;
    while index < i32::from(e.call(VERTEX_COUNT, &args![data]).u16()) {
        copy_color(e, colors.wrapping_add((index as u32) << 4), this.addr() + 8);
        index += 1;
    }
    e.call(MARK_AS_CHANGED, &args![data, 4u32]);
}

// Translated from 004a64e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box's colour setter, `thiscall`, `RET 4`:
/// the colour's address): the colour into `+8`; with a geometry at `+0x24`
/// every vertex colour of it and of the geometry at `+0x28` (the vertex count
/// asked again for each vertex) is set and marked changed, then the alpha
/// switch.
pub fn fn_004a64e0(e: &mut Engine, this: Ptr, color: Ptr) {
    let t = this.addr();
    copy_color(e, t + 8, color.addr());
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() != 0 {
        paint_geometry(e, this, t + 0x24);
        paint_geometry(e, this, t + 0x28);
        set_alpha(e, this, color.addr());
    }
}

/// `00408840` (`cdecl`, a float; the magnitude in `ST0`), `00416870`
/// (`thiscall` on a point, `RET 0xc`: three floats; answers the point),
/// `007af430` (`thiscall` on geometry data; its vertex array), `0043d410`
/// (`thiscall` on a point, `RET 0xc`: a float and two words), `006240d0`
/// (`thiscall`, no stack words: a bound's constructor), `004a5200` and
/// `00a59c60` (`thiscall`, `RET 4`: a bound's or a point's address).
const MAGNITUDE_00408840: u32 = 0x0040_8840;
const POINT3_SET: u32 = 0x0041_6870;
const DATA_VERTICES: u32 = 0x007a_f430;
const POINT3_ZERO_0043D410: u32 = 0x0043_d410;
const BOUND_CONSTRUCT_006240D0: u32 = 0x0062_40d0;
const GEOMETRY_SET_BOUND_004A5200: u32 = 0x004a_5200;
const NODE_SET_00A59C60: u32 = 0x00a5_9c60;
/// The record the bound's centre comes from (`011f426c`), the double 0.01
/// (`01016408`), the float 0.01 (`01013ea4`), the double 0.0 (`01012060`)
/// and the float -1.0 (`01012054`).
const ORIGIN_011F426C: u32 = 0x011f_426c;
const DOUBLE_SMALL: u32 = 0x0101_6408;
const FLOAT_SMALL: u32 = 0x0101_3ea4;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// The sign patterns of the box's eight corners (x, y, z), in the exe's
/// order.
const CORNERS: [[bool; 3]; 8] = [
    [true, false, false],
    [false, false, false],
    [false, true, false],
    [true, true, false],
    [true, false, true],
    [false, false, true],
    [false, true, true],
    [true, true, true],
];

/// A size coordinate pushed as a float: `FLD`, `FCHS` when negated, `FSTP`.
fn pushed(bits: u32, negate: bool) -> u32 {
    let bits = x87_quiet_bits(bits);
    if negate {
        bits ^ 0x8000_0000
    } else {
        bits
    }
}

// Translated from 004a6640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the box's size setter, `thiscall`, `RET 4`: the
/// address of three floats). Each size coordinate becomes its magnitude
/// (`00408840`), written back, and the size is kept at `+0x18`. With a
/// geometry at `+0x24` its eight vertices are the corners `(±x, ±y, ±z)`
/// (made through `00416870`, signs negated with `FCHS`) and its data marked
/// changed (1). With a geometry at `+0x28` each vertex of it becomes the
/// sign of each coordinate (-1.0 below 0.0, else 1.0) times the size
/// (coordinates below 0.01 in magnitude count as 0.01), the data marked
/// changed after each vertex (the vertex count asked again each time). A zero
/// point is made (`0043d410`). With a node at `+0x2c`: a bound from
/// `011f426c` with the largest coordinate as its radius (the first, the
/// second when larger than the other two, the third when larger than both)
/// goes to the first geometry (`004a5200`), the zero point to the node
/// (`00a59c60`), the geometry's properties are updated and the node's slot
/// `+0xbc` runs.
pub fn fn_004a6640(e: &mut Engine, this: Ptr, size: Ptr) {
    let t = this.addr();
    let s = size.addr();
    for i in 0..3u32 {
        let at = fn_004a51b0(e, size, i);
        let value = x87_quiet_bits(e.mem.u32(at));
        let magnitude = e.call(MAGNITUDE_00408840, &args![value]).f64();
        let stored = x87_store32(magnitude);
        let at = fn_004a51b0(e, size, i);
        e.mem.set_u32(at, x87_quiet_bits(stored));
    }
    for word in 0..3u32 {
        let v = e.mem.u32(s + 4 * word);
        e.mem.set_u32(t + 0x18 + 4 * word, v);
    }
    let kept = Ptr::new(t + 0x18);
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let vertices = e.call(DATA_VERTICES, &args![data]).u32();
        for (corner, signs) in CORNERS.iter().enumerate() {
            let z_at = fn_004a51b0(e, kept, 2);
            let z = pushed(e.mem.u32(z_at), signs[2]);
            let y_at = fn_004a51b0(e, kept, 1);
            let y = pushed(e.mem.u32(y_at), signs[1]);
            let x_at = fn_004a51b0(e, kept, 0);
            let x = pushed(e.mem.u32(x_at), signs[0]);
            e.with_stack(0xc, |e, point| {
                let made = e.call(POINT3_SET, &args![point, x, y, z]).u32();
                for word in 0..3u32 {
                    let v = e.mem.u32(made + 4 * word);
                    e.mem.set_u32(vertices + 0xc * corner as u32 + 4 * word, v);
                }
            });
        }
        e.call(MARK_AS_CHANGED, &args![data, 1u32]);
    }
    if e.call(NI_POINTER_GET, &args![t + 0x28]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x28]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let vertices = e.call(DATA_VERTICES, &args![data]).u32();
        let mut scale = [0u32; 3];
        for (i, word) in scale.iter_mut().enumerate() {
            *word = e.mem.u32(s + 4 * i as u32);
        }
        let small = x87_load64(e.mem.u64(DOUBLE_SMALL));
        for word in scale.iter_mut() {
            let magnitude = e
                .call(MAGNITUDE_00408840, &args![x87_quiet_bits(*word)])
                .f64();
            if magnitude < small {
                *word = x87_quiet_bits(e.mem.u32(FLOAT_SMALL));
            }
        }
        let zero = x87_load64(e.mem.u64(DOUBLE_ZERO));
        let mut index: i32 = 0;
        while index < i32::from(e.call(VERTEX_COUNT, &args![data]).u16()) {
            let vertex = vertices.wrapping_add((index as u32).wrapping_mul(0xc));
            for c in 0..3u32 {
                let sign = if x87_m32(e.mem.u32(vertex + 4 * c)) < zero {
                    x87_quiet_bits(e.mem.u32(FLOAT_MINUS_ONE))
                } else {
                    x87_store32(1.0)
                };
                e.mem.set_u32(vertex + 4 * c, x87_quiet_bits(sign));
            }
            for (c, factor) in scale.iter().enumerate() {
                let at = vertex + 4 * c as u32;
                let v = x87_store32(x87_mul(x87_load32(e.mem.u32(at)), x87_m32(*factor)));
                e.mem.set_u32(at, v);
            }
            e.call(MARK_AS_CHANGED, &args![data, 1u32]);
            index += 1;
        }
    }
    e.with_stack(0x40, |e, frame| {
        // The frame: the zero point (+0) and the bound (+0x10).
        let point = frame.addr();
        let bound = point + 0x10;
        let zero = x87_store32(0.0);
        e.call(POINT3_ZERO_0043D410, &args![point, zero, 0u32, 0u32]);
        if e.call(NI_POINTER_GET, &args![t + 0x2c]).u32() == 0 {
            return;
        }
        let first = e.mem.u32(s);
        let second = e.mem.u32(s + 4);
        let third = e.mem.u32(s + 8);
        let mut radius = x87_quiet_bits(first);
        if x87_m32(first) < x87_m32(second) && x87_m32(third) < x87_m32(second) {
            radius = x87_quiet_bits(e.mem.u32(s + 4));
        }
        if x87_m32(e.mem.u32(s + 4)) < x87_m32(e.mem.u32(s + 8))
            && x87_m32(e.mem.u32(s)) < x87_m32(e.mem.u32(s + 8))
        {
            radius = x87_quiet_bits(e.mem.u32(s + 8));
        }
        e.call(BOUND_CONSTRUCT_006240D0, &args![bound]);
        fn_004a51d0(
            e,
            Ptr::new(bound),
            Ptr::new(ORIGIN_011F426C),
            f32::from_bits(x87_quiet_bits(radius)),
        );
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        e.call(GEOMETRY_SET_BOUND_004A5200, &args![geometry, bound]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.call(NODE_SET_00A59C60, &args![node, point]);
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        e.call(UPDATE_PROPERTIES, &args![geometry]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.vcall(node, 0xbc, &args![]);
    });
}

/// The `NiRTTI` records the `GetRTTI`s answer: `BSOcclusionPlane`
/// (`01202804`), `BSPortal` (`012030b0`), `BSMultiBoundAABB` (`01202820`),
/// `BSMultiBoundOBB` (`01202818`) and `BSMultiBoundSphere` (`01203108`).
const RTTI_OCCLUSION_PLANE: u32 = 0x0120_2804;
const RTTI_PORTAL: u32 = 0x0120_30b0;
const RTTI_MULTI_BOUND_AABB: u32 = 0x0120_2820;
const RTTI_MULTI_BOUND_OBB: u32 = 0x0120_2818;
const RTTI_MULTI_BOUND_SPHERE: u32 = 0x0120_3108;
/// The vtables (by their RTTI): `BSOcclusionPlane` (`0101e794`), `BSPortal`
/// (`0101e82c`), `BSMultiBoundAABB` (`0101e8e4`), `BSMultiBoundOBB`
/// (`0101e9a4`), `BGSPrimitiveSphere` (`0101ea64`), `BSMultiBoundSphere`
/// (`0101ea8c`).
const OCCLUSION_PLANE_VTABLE: u32 = 0x0101_e794;
const PORTAL_VTABLE: u32 = 0x0101_e82c;
const MULTI_BOUND_AABB_VTABLE: u32 = 0x0101_e8e4;
const MULTI_BOUND_OBB_VTABLE: u32 = 0x0101_e9a4;
const SPHERE_VTABLE: u32 = 0x0101_ea64;
const MULTI_BOUND_SPHERE_VTABLE: u32 = 0x0101_ea8c;
/// `BSMultiBoundShape::BSMultiBoundShape` (Xbox PDB, `00c4f980`) and the
/// `NiObject` constructor body (`00a5d3a0`), both `this` only.
const MULTI_BOUND_SHAPE_CONSTRUCT: u32 = 0x00c4_f980;
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiFrustumPlanes::NiFrustumPlanes` (Xbox PDB, `0045c620`).
const FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
/// `` `vector constructor iterator' `` (`00401050`, `stdcall`: array,
/// element size, count, constructor).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// The `NiPointer` release (`0045cec0`, `this` only).
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// The identity rotation the occlusion plane, portal and box shape are
/// given (nine floats at `011a9448`).
const IDENTITY_ROTATION: u32 = 0x011a_9448;
/// `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB, `00a59540`, `RET 0xC`: three
/// floats) and `004396b0` (`RET 4`: a matrix's address) that gives the box
/// shape its rotation.
const FROM_EULER_ANGLES: u32 = 0x00a5_9540;
const SET_ROTATION_4396B0: u32 = 0x0043_96b0;
/// The 0x70 unit-sphere vertices (three `double`s each, 0x18 bytes apart,
/// from `01187ac8`).
const SPHERE_POINTS: u32 = 0x0118_7ac8;
/// The vertex count and strip length of the sphere (0x70) and its triangle
/// count (0x6E).
const SPHERE_VERTICES: u32 = 0x70;
/// Scales a point by a float (`00439180`, `RET 4`).
const POINT3_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// `NiTriStrips::NiTriStrips` (Xbox PDB, `00a71c40`, `RET 0x2C`),
/// `BSShaderNoLightingProperty::BSShaderNoLightingProperty` (Xbox PDB,
/// `00b6fc90`), `005a8060` (`RET 4`; the engine map's
/// `LowProcess::SetNumberOfItemsActivated` is a folded body) given 0x21 on
/// the property, `NiAVObject::AttachProperty` (Xbox PDB, `00439410`,
/// `RET 4`), `BSShaderManager::PrepareObject` (Xbox PDB, `00b57e30`, cdecl,
/// three words), the alpha property's constructor (`004391c0`, 0x1C
/// bytes), `0049ed90` (`RET 4`) given 1 on it, `NiAVObject::GetProperty`
/// (Xbox PDB, `00a59d30`, `RET 4`) and `BSShaderProperty::SetFlag` (Xbox
/// PDB, `00441130`, `RET 8`).
const TRI_STRIPS_CONSTRUCT: u32 = 0x00a7_1c40;
const NO_LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00b6_fc90;
const PROPERTY_SET_5A8060: u32 = 0x005a_8060;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const ALPHA_PROPERTY_SET_49ED90: u32 = 0x0049_ed90;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const SHADER_PROPERTY_SET_FLAG: u32 = 0x0044_1130;
/// `"primitive: alpha"` and `"Sphere"`.
const ALPHA_NAME: u32 = 0x0101_e744;
const SPHERE_NAME: u32 = 0x0101_ea7c;
/// The primitive's size getter (`00413fc0`, `RET 4`: copies the three
/// floats at +0x18 to the address given and answers it) and its colour
/// getter (`00413f90`, `RET 4`: the result's address).
const GET_SIZE_413FC0: u32 = 0x0041_3fc0;
const GET_COLOR_413F90: u32 = 0x0041_3f90;
/// The sphere's radius getter (`004a7bd0`, the engine map's
/// `VirtualActorPathHandler::GetDistanceTraveled` is a folded body; its
/// value in `ST0`).
const GET_RADIUS_4A7BD0: u32 = 0x004a_7bd0;
/// The primitive's type getter (`00726070`, the word at +4).
const GET_TYPE: u32 = 0x0072_6070;

/// `FLD dword` then `FSTP dword` of a float word (a signalling NaN comes out
/// quiet).
fn reloaded(bits: u32) -> u32 {
    x87_quiet_bits(bits)
}

// Translated from 004a4c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `NiColorA::operator_` (`RET 4`): 1 when the two colours
/// are not all equal (`fn_004a4cc0` false), else 0 (`NEG`/`SBB`/`ADD`).
pub fn fn_004a4c90(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    u32::from(!fn_004a4cc0(e, this, other))
}

// Translated from 004a4d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a primitive like `primitive` (cdecl, one word), by its type
/// (`00726070`), each type checked in turn: 3, a plane (`004a4f80`) of the
/// size's first and third floats doubled (`FADD ST0,ST0`, stored as
/// `float`s) with the colour (`00413f90`); 1, a box (`004a5040`) of the
/// size doubled (`0045bb20` with 2.0) with the colour; 2, a sphere
/// (`004a5100`) of the radius (`004a7bd0`) with the colour. The plane case
/// also doubles the size into a stack copy (`0045bb20`) and reads the
/// colour once more without using either, as the exe does. 0 for another
/// type.
pub fn fn_004a4d40(e: &mut Engine, primitive: Ptr) -> u32 {
    // The frame from EBP-0xA0: a local at EBP-k is at `frame + 0xa0 - k`.
    e.with_stack(0xa0, |e, frame| {
        let at = |k: u32| frame.addr() + 0xa0 - k;
        let mut made = 0u32;
        let two = reloaded(e.mem.u32(FLOAT_TWO));
        if e.call(GET_TYPE, &args![primitive]).u32() == 3 {
            let size = e.call(GET_SIZE_413FC0, &args![primitive, at(0x70)]).u32();
            e.call(POINT3_SCALED, &args![size, at(0x30), two]);
            e.call(GET_COLOR_413F90, &args![primitive, at(0x24)]);
            let size = e.call(GET_SIZE_413FC0, &args![primitive, at(0x7c)]).u32();
            let z = x87_load32(e.mem.u32(size + 8));
            let z2 = x87_store32(x87_add(z, z));
            let size = e.call(GET_SIZE_413FC0, &args![primitive, at(0x88)]).u32();
            let x = x87_load32(e.mem.u32(size));
            let x2 = x87_store32(x87_add(x, x));
            e.call(POINT2_SET, &args![at(0x38), reloaded(x2), reloaded(z2)]);
            e.call(GET_COLOR_413F90, &args![primitive, at(0x14)]);
            made = fn_004a4f80(e, Ptr::new(at(0x38)), at(0x14));
        }
        if e.call(GET_TYPE, &args![primitive]).u32() == 1 {
            let size = e.call(GET_SIZE_413FC0, &args![primitive, at(0x94)]).u32();
            e.call(POINT3_SCALED, &args![size, at(0x54), two]);
            e.call(GET_COLOR_413F90, &args![primitive, at(0x48)]);
            made = fn_004a5040(e, Ptr::new(at(0x54)), at(0x48));
        }
        if e.call(GET_TYPE, &args![primitive]).u32() == 2 {
            e.call(GET_COLOR_413F90, &args![primitive, at(0x64)]);
            let radius = e.call(GET_RADIUS_4A7BD0, &args![primitive]).f64();
            made = fn_004a5100(e, f32::from_bits(x87_store32(radius)), at(0x64));
        }
        made
    })
}

// Translated from 004a5100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5100 - the exception frame (FS:[0]) around the sphere's construction is not translated
/// Makes a sphere (cdecl: a radius `float`, a word): a `BGSPrimitiveSphere`
/// (0x34 bytes from `00401000`, built by `fn_004a7150`; a failed allocation
/// is used as it is) gets the word through its vtable slot 8, the radius
/// (`SetRadius`, the float through `FLD`/`FSTP`) and the type 2 at +4.
pub fn fn_004a5100(e: &mut Engine, radius: f32, word: u32) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x34u32]).u32();
    let sphere = if block == 0 {
        0
    } else {
        fn_004a7150(e, Ptr::new(block)).addr()
    };
    e.vcall(sphere, 8, &args![word]);
    bgs_primitive_sphere_set_radius(
        e,
        Ptr::new(sphere),
        f32::from_bits(reloaded(radius.to_bits())),
    );
    e.mem.set_u32(sphere.wrapping_add(4), 2);
    sphere
}

// Translated from 004a5200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the geometry's bound (`RET 4`): `fn_004a5230` on the object in the
/// pointer at +0xB8, its answer passed on.
pub fn fn_004a5200(e: &mut Engine, this: Ptr, bound: Ptr) -> Ptr {
    let object = e.call(NI_POINTER_GET, &args![this.addr() + 0xb8]).u32();
    fn_004a5230(e, Ptr::new(object), bound)
}

// Translated from 004a5230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a bound into the one at +0x10 (`fn_004a5250`, `RET 4`), its
/// answer passed on.
pub fn fn_004a5230(e: &mut Engine, this: Ptr, bound: Ptr) -> Ptr {
    fn_004a5250(e, Ptr::new(this.addr().wrapping_add(0x10)), bound)
}

/// One corner of the box the plane and the box use, read from the size at
/// `kept` (z, then y, then x, each through `004a51b0` and `FLD`, negated by
/// `FCHS` by `signs`) and made with `00416870`; the point's three words.
fn corner(e: &mut Engine, kept: Ptr, signs: [bool; 3]) -> [u32; 3] {
    let z_at = fn_004a51b0(e, kept, 2);
    let z = pushed(e.mem.u32(z_at), signs[2]);
    let y_at = fn_004a51b0(e, kept, 1);
    let y = pushed(e.mem.u32(y_at), signs[1]);
    let x_at = fn_004a51b0(e, kept, 0);
    let x = pushed(e.mem.u32(x_at), signs[0]);
    e.with_stack(0xc, |e, point| {
        let made = e.call(POINT3_SET, &args![point, x, y, z]).u32();
        [e.mem.u32(made), e.mem.u32(made + 4), e.mem.u32(made + 8)]
    })
}

fn put_vertex(e: &mut Engine, vertices: u32, index: u32, point: [u32; 3]) {
    for (word, value) in point.iter().enumerate() {
        e.mem
            .set_u32(vertices + 0xc * index + 4 * word as u32, *value);
    }
}

fn copy_vertex(e: &mut Engine, vertices: u32, to: u32, from: u32) {
    for word in 0..3u32 {
        let value = e.mem.u32(vertices + 0xc * from + 4 * word);
        e.mem.set_u32(vertices + 0xc * to + 4 * word, value);
    }
}

// Translated from 004a55a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitivePlane::SetRadii` (Xbox PDB; `RET 4`: the address of three
/// floats). The first and third become their magnitudes (`00408840`,
/// stored as `float`s), the second 0.0, and the size is kept at +0x18.
/// With a geometry at +0x24 its eight vertices are the box corners
/// `(±x, ±y, ±z)` in the exe's order (`00416870`), its data marked changed
/// (1). With a geometry at +0x28 its sixteen vertices are the corners
/// (-x,y,z), (x,y,z), (x,-y,z), (-x,-y,z), the first again, (-x,y,-z),
/// (-x,-y,-z), (x,-y,-z), (x,y,-z), then copies of vertices 5, 3, 6, 1, 8,
/// 2 and 7; its data marked changed (1). A zero point is made
/// (`0043d410`). With a node at +0x2c: a bound from `011f426c` with the
/// largest of the three floats as its radius (the first, the second when
/// larger than the other two, the third when larger than both; `FCOMPP`,
/// no NaN wins) goes to the first geometry (`fn_004a5200`), the zero point
/// to the node (`00a59c60`) and its slot 0xB8, the geometry's properties
/// are updated and the node's slot 0xBC runs.
pub fn bgs_primitive_plane_set_radii(e: &mut Engine, this: Ptr, size: Ptr) {
    let t = this.addr();
    let s = size.addr();
    for (index, magnitude) in [(0u32, true), (1, false), (2, true)] {
        let value = if magnitude {
            let at = fn_004a51b0(e, size, index);
            let value = reloaded(e.mem.u32(at));
            let result = e.call(MAGNITUDE_00408840, &args![value]).f64();
            reloaded(x87_store32(result))
        } else {
            x87_store32(0.0)
        };
        let at = fn_004a51b0(e, size, index);
        e.mem.set_u32(at, value);
    }
    for word in 0..3u32 {
        let v = e.mem.u32(s + 4 * word);
        e.mem.set_u32(t + 0x18 + 4 * word, v);
    }
    let kept = Ptr::new(t + 0x18);
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let vertices = e.call(DATA_VERTICES, &args![data]).u32();
        for (index, signs) in CORNERS.iter().enumerate() {
            let point = corner(e, kept, *signs);
            put_vertex(e, vertices, index as u32, point);
        }
        e.call(MARK_AS_CHANGED, &args![data, 1u32]);
    }
    if e.call(NI_POINTER_GET, &args![t + 0x28]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x28]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let vertices = e.call(DATA_VERTICES, &args![data]).u32();
        for index in 0..4u32 {
            let point = corner(e, kept, CORNERS[index as usize]);
            put_vertex(e, vertices, index, point);
        }
        copy_vertex(e, vertices, 4, 0);
        for (index, corner_index) in [(5u32, 4usize), (6, 7), (7, 6), (8, 5)] {
            let point = corner(e, kept, CORNERS[corner_index]);
            put_vertex(e, vertices, index, point);
        }
        for (to, from) in [
            (9u32, 5u32),
            (10, 3),
            (11, 6),
            (12, 1),
            (13, 8),
            (14, 2),
            (15, 7),
        ] {
            copy_vertex(e, vertices, to, from);
        }
        e.call(MARK_AS_CHANGED, &args![data, 1u32]);
    }
    e.with_stack(0x40, |e, frame| {
        // The frame: the zero point (+0) and the bound (+0x10).
        let point = frame.addr();
        let bound = point + 0x10;
        let zero = x87_store32(0.0);
        e.call(POINT3_ZERO_0043D410, &args![point, zero, 0u32, 0u32]);
        if e.call(NI_POINTER_GET, &args![t + 0x2c]).u32() == 0 {
            return;
        }
        let mut radius = reloaded(e.mem.u32(s));
        if x87_m32(e.mem.u32(s)) < x87_m32(e.mem.u32(s + 4))
            && x87_m32(e.mem.u32(s + 8)) < x87_m32(e.mem.u32(s + 4))
        {
            radius = reloaded(e.mem.u32(s + 4));
        }
        if x87_m32(e.mem.u32(s + 4)) < x87_m32(e.mem.u32(s + 8))
            && x87_m32(e.mem.u32(s)) < x87_m32(e.mem.u32(s + 8))
        {
            radius = reloaded(e.mem.u32(s + 8));
        }
        e.call(BOUND_CONSTRUCT_006240D0, &args![bound]);
        fn_004a51d0(
            e,
            Ptr::new(bound),
            Ptr::new(ORIGIN_011F426C),
            f32::from_bits(reloaded(radius)),
        );
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        fn_004a5200(e, Ptr::new(geometry), Ptr::new(bound));
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.call(NODE_SET_00A59C60, &args![node, point]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.vcall(node, 0xb8, &args![point]);
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        e.call(UPDATE_PROPERTIES, &args![geometry]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.vcall(node, 0xbc, &[]);
    });
}

/// The constructors `make_plane_object` takes (`this`, centre, size,
/// rotation).
type PlaneConstructor = fn(&mut Engine, Ptr, Ptr, Ptr, Ptr) -> Ptr;

/// The shared body of `MakeOcclusionPlane` and `MakePortal`: an object of
/// `size` bytes from the Gamebryo allocator; when there is one, a two-float
/// point of the size's first and third floats (`00452dc0`) and `construct`
/// with the origin record `011f426c`, that point and the identity rotation.
fn make_plane_object(e: &mut Engine, this: Ptr, size: u32, construct: PlaneConstructor) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.with_stack(8, |e, point| {
        let x = reloaded(e.mem.u32(this.addr() + 0x18));
        let z = reloaded(e.mem.u32(this.addr() + 0x20));
        let made = e.call(POINT2_SET, &args![point, x, z]).u32();
        construct(
            e,
            Ptr::new(block),
            Ptr::new(ORIGIN_011F426C),
            Ptr::new(made),
            Ptr::new(IDENTITY_ROTATION),
        )
        .addr()
    })
}

// Translated from 004a5f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5f50 - the exception frame (FS:[0]) around the occlusion plane's construction is not translated
/// `BGSPrimitivePlane::MakeOcclusionPlane` (Xbox PDB): a `BSOcclusionPlane`
/// (0xFC bytes, `fn_004a5ff0`) at the origin with the plane's width and
/// depth and the identity rotation; null when the allocation fails.
pub fn bgs_primitive_plane_make_occlusion_plane(e: &mut Engine, this: Ptr) -> u32 {
    make_plane_object(e, this, 0xfc, fn_004a5ff0)
}

// Translated from 004a5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a5ff0 - the exception frame (FS:[0]) around the base construction is not translated
/// `BSOcclusionPlane`'s constructor (by its vtable's RTTI; `RET 0xC`: the
/// centre's address, the two-float size's address, the nine-float
/// rotation's address): the `NiObject` base (`00a5d3a0`), the vtable, the
/// centre at +8, the size at +0x14, the rotation at +0x1C, the frustum
/// planes (`0045c620` on +0x44), four 0xC-byte points at +0xA8 (the vector
/// iterator with `006815c0`) and `006815c0` on +0xD8, the bytes +0xE4 and
/// +0xE5 = 1, +0x40 = 0, +0xEA = 1. Returns `this`.
pub fn fn_004a5ff0(e: &mut Engine, this: Ptr, center: Ptr, size: Ptr, rotation: Ptr) -> Ptr {
    let t = this.addr();
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, OCCLUSION_PLANE_VTABLE);
    for word in 0..3u32 {
        let v = e.mem.u32(center.addr() + 4 * word);
        e.mem.set_u32(t + 8 + 4 * word, v);
    }
    let first = e.mem.u32(size.addr());
    let second = e.mem.u32(size.addr() + 4);
    e.mem.set_u32(t + 0x14, first);
    e.mem.set_u32(t + 0x18, second);
    for word in 0..9u32 {
        let v = e.mem.u32(rotation.addr() + 4 * word);
        e.mem.set_u32(t + 0x1c + 4 * word, v);
    }
    e.call(FRUSTUM_PLANES_CONSTRUCT, &args![t + 0x44]);
    e.call(
        VECTOR_CONSTRUCTOR,
        &args![t + 0xa8, 0xcu32, 4u32, MEMBER_18_SET_UP],
    );
    e.call(MEMBER_18_SET_UP, &args![t + 0xd8]);
    e.mem.set_u8(t + 0xe4, 1);
    e.mem.set_u8(t + 0xe5, 1);
    e.mem.set_u32(t + 0x40, 0);
    e.mem.set_u8(t + 0xea, 1);
    this
}

// Translated from 004a60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::GetRTTI` (Xbox PDB): its `NiRTTI` (`01202804`).
pub fn bs_occlusion_plane_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_OCCLUSION_PLANE
}

// Translated from 004a6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::QDebugGeomShapeSize` (Xbox PDB): 0x1E.
pub fn bs_occlusion_plane_q_debug_geom_shape_size(_e: &mut Engine, _this: Ptr) -> u32 {
    0x1e
}

// Translated from 004a6150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a6150 - the exception frame (FS:[0]) around the portal's construction is not translated
/// `BGSPrimitivePlane::MakePortal` (Xbox PDB): a `BSPortal` (0x108 bytes,
/// `fn_004a61f0`) at the origin with the plane's width and depth and the
/// identity rotation; null when the allocation fails.
pub fn bgs_primitive_plane_make_portal(e: &mut Engine, this: Ptr) -> u32 {
    make_plane_object(e, this, 0x108, fn_004a61f0)
}

// Translated from 004a61f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a61f0 - the exception frame (FS:[0]) around the base construction is not translated
/// `BSPortal`'s constructor (by its vtable's RTTI; `RET 0xC`, as
/// `fn_004a5ff0`): the occlusion plane, the vtable, a null `NiPointer` at
/// +0x104 (`00633c90`), the bytes +0xE4 and +0xE5 = 1, the words +0x40,
/// +0x100 and +0xFC = 0, the pointer at +0x104 assigned null (`0066b0d0`).
/// Returns `this`.
pub fn fn_004a61f0(e: &mut Engine, this: Ptr, center: Ptr, size: Ptr, rotation: Ptr) -> Ptr {
    let t = this.addr();
    fn_004a5ff0(e, this, center, size, rotation);
    e.mem.set_u32(t, PORTAL_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![t + 0x104, 0u32]);
    e.mem.set_u8(t + 0xe4, 1);
    e.mem.set_u8(t + 0xe5, 1);
    e.mem.set_u32(t + 0x40, 0);
    e.mem.set_u32(t + 0x100, 0);
    e.mem.set_u32(t + 0xfc, 0);
    e.call(NI_POINTER_SET, &args![t + 0x104, 0u32]);
    this
}

// Translated from 004a62c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortal::GetRTTI` (Xbox PDB): its `NiRTTI` (`012030b0`).
pub fn bs_portal_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_PORTAL
}

// Translated from 004a62d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortal::_scalar_deleting_destructor_` (Xbox PDB; `RET 4`): the
/// destructor `fn_004a6310`, then with bit 0 the sized free of 0x108 bytes.
/// Answers `this`.
pub fn bs_portal_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004a6310(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x108u32]);
    }
    this
}

// Translated from 004a6310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a6310 - the exception frame (FS:[0]) around the member's release is not translated
/// `BSPortal`'s destructor body: the `NiPointer` at +0x104 released
/// (`0045cec0`), then the base's (`fn_004a7090`).
pub fn fn_004a6310(e: &mut Engine, this: Ptr) {
    e.call(NI_POINTER_RELEASE, &args![this.addr() + 0x104]);
    fn_004a7090(e, this);
}

// Translated from 004a6d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitivePlane::IsInside` (Xbox PDB; `RET 4`: a point's address):
/// false when a coordinate is below the negated size (+0x18..+0x20) or the
/// size is below the coordinate (`FCOMPP`, `TEST AH,5`: a NaN never fails
/// a test), checked x, y, z low then x, y, z high; true otherwise.
pub fn bgs_primitive_plane_is_inside(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let t = this.addr();
    let p = point.addr();
    for i in 0..3u32 {
        let low = -x87_m32(e.mem.u32(t + 0x18 + 4 * i));
        if x87_m32(e.mem.u32(p + 4 * i)) < low {
            return false;
        }
    }
    for i in 0..3u32 {
        let high = x87_m32(e.mem.u32(t + 0x18 + 4 * i));
        if high < x87_m32(e.mem.u32(p + 4 * i)) {
            return false;
        }
    }
    true
}

// Translated from 004a6e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a6e30 - the exception frames (FS:[0]) around the shapes' construction are not translated
/// The box's multi-bound shape (`RET 4`: the address of three rotation
/// angles): when any angle's magnitude (`00408840`) is above the `double`
/// 0.01 (`01016408`; `TEST AH,0x41`, not for NaN), a `BSMultiBoundOBB`
/// (0x48 bytes, `fn_004a70b0`) at the origin with the box's size (+0x18)
/// and the identity rotation, then given the rotation of the angles
/// (`NiMatrix3::FromEulerAnglesXYZ` on a stack matrix set up by `006815c0`,
/// then `004396b0`); otherwise a `BSMultiBoundAABB` (0x24 bytes,
/// `fn_004a6fc0`) at the origin with the size. A failed allocation gives
/// null (the rotation still goes to it in the first case, as the exe does).
pub fn fn_004a6e30(e: &mut Engine, this: Ptr, angles: Ptr) -> u32 {
    let t = this.addr();
    let a = angles.addr();
    let small = x87_load64(e.mem.u64(DOUBLE_SMALL));
    let mut rotated = false;
    for i in 0..3u32 {
        let angle = reloaded(e.mem.u32(a + 4 * i));
        let magnitude = e.call(MAGNITUDE_00408840, &args![angle]).f64();
        if magnitude > small {
            rotated = true;
        }
    }
    if rotated {
        let block = e.call(NI_ALLOCATE, &args![0x48u32]).u32();
        let shape = if block == 0 {
            0
        } else {
            fn_004a70b0(
                e,
                Ptr::new(block),
                Ptr::new(ORIGIN_011F426C),
                Ptr::new(t + 0x18),
                Ptr::new(IDENTITY_ROTATION),
            )
            .addr()
        };
        e.with_stack(0x24, |e, matrix| {
            e.call(MEMBER_18_SET_UP, &args![matrix]);
            let x = reloaded(e.mem.u32(a));
            let y = reloaded(e.mem.u32(a + 4));
            let z = reloaded(e.mem.u32(a + 8));
            e.call(FROM_EULER_ANGLES, &args![matrix, x, y, z]);
            e.call(SET_ROTATION_4396B0, &args![shape, matrix]);
        });
        shape
    } else {
        let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
        if block == 0 {
            0
        } else {
            fn_004a6fc0(
                e,
                Ptr::new(block),
                Ptr::new(ORIGIN_011F426C),
                Ptr::new(t + 0x18),
            )
            .addr()
        }
    }
}

// Translated from 004a6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB`'s constructor (by its vtable's RTTI; `RET 8`: the
/// centre's and the size's addresses): `BSMultiBoundShape`'s constructor,
/// the vtable, the centre at +0xC and the size at +0x18. Returns `this`.
pub fn fn_004a6fc0(e: &mut Engine, this: Ptr, center: Ptr, size: Ptr) -> Ptr {
    let t = this.addr();
    e.call(MULTI_BOUND_SHAPE_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, MULTI_BOUND_AABB_VTABLE);
    for word in 0..3u32 {
        let v = e.mem.u32(center.addr() + 4 * word);
        e.mem.set_u32(t + 0xc + 4 * word, v);
    }
    for word in 0..3u32 {
        let v = e.mem.u32(size.addr() + 4 * word);
        e.mem.set_u32(t + 0x18 + 4 * word, v);
    }
    this
}

// Translated from 004a7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::GetRTTI` (Xbox PDB): its `NiRTTI` (`01202820`).
pub fn bs_multi_bound_aabb_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_MULTI_BOUND_AABB
}

// Translated from 004a7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::QDebugGeomShapeSize` (Xbox PDB): 0x24.
pub fn bs_multi_bound_aabb_q_debug_geom_shape_size(_e: &mut Engine, _this: Ptr) -> u32 {
    0x24
}

// Translated from 004a70b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB`'s constructor (by its vtable's RTTI; `RET 0xC`: the
/// centre's, the size's and the nine-float rotation's addresses): the
/// `BSMultiBoundAABB` part (`fn_004a6fc0`), the vtable, the rotation at
/// +0x24. Returns `this`.
pub fn fn_004a70b0(e: &mut Engine, this: Ptr, center: Ptr, size: Ptr, rotation: Ptr) -> Ptr {
    let t = this.addr();
    fn_004a6fc0(e, this, center, size);
    e.mem.set_u32(t, MULTI_BOUND_OBB_VTABLE);
    for word in 0..9u32 {
        let v = e.mem.u32(rotation.addr() + 4 * word);
        e.mem.set_u32(t + 0x24 + 4 * word, v);
    }
    this
}

// Translated from 004a70f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::GetRTTI` (Xbox PDB): its `NiRTTI` (`01202818`).
pub fn bs_multi_bound_obb_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_MULTI_BOUND_OBB
}

// Translated from 004a7100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::_scalar_deleting_destructor_` (Xbox PDB; `RET 4`): the
/// destructor `fn_004a7130`, then with bit 0 the sized free of 0x48 bytes.
/// Answers `this`.
pub fn bs_multi_bound_obb_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004a7130(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 004a7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB`'s destructor body: `fn_004a7c20`.
pub fn fn_004a7130(e: &mut Engine, this: Ptr) {
    fn_004a7c20(e, this);
}

// Translated from 004a7150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a7150 - the exception frame (FS:[0]) around the base construction is not translated
/// `BGSPrimitiveSphere`'s constructor (by its vtable's RTTI; the engine
/// map's name is a folded library match): `fn_004a4ac0`, the vtable, the
/// pointers at +0x24 and +0x2C assigned null (`0066b0d0`), the type 2 at
/// +4. Returns `this`.
pub fn fn_004a7150(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    fn_004a4ac0(e, this);
    e.mem.set_u32(t, SPHERE_VTABLE);
    e.call(NI_POINTER_SET, &args![t + 0x24, 0u32]);
    e.call(NI_POINTER_SET, &args![t + 0x2c, 0u32]);
    e.mem.set_u32(t + 4, 2);
    this
}

// Translated from 004a71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitiveSphere::SetRadii` (Xbox PDB; `RET 4`): `SetRadius` with the
/// first float at the address (through `FLD`/`FSTP`).
pub fn bgs_primitive_sphere_set_radii(e: &mut Engine, this: Ptr, radii: Ptr) {
    let radius = reloaded(e.mem.u32(radii.addr()));
    bgs_primitive_sphere_set_radius(e, this, f32::from_bits(radius));
}

// Translated from 004a7240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitiveSphere::IsInside` (Xbox PDB; `RET 4`): whether the point's
/// squared length (`fn_004a7290`) is below the radius (+0x18) squared
/// (`FMUL`, kept as a `double`; `TEST AH,5`, false for NaN).
pub fn bgs_primitive_sphere_is_inside(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let radius = e.mem.u32(this.addr() + 0x18);
    let squared = x87_mul(x87_load32(radius), x87_m32(radius));
    let length = f64::from(fn_004a7290(e, point));
    length < squared
}

// Translated from 004a72d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a72d0 - the exception frames (FS:[0]) around the allocations, the property pointer and the fixed strings are not translated
/// `BGSPrimitiveSphere::Build` (Xbox PDB). Without a geometry at +0x24: 0x70
/// colours (0x700 bytes, each built by `fn_004a7800`) and points (0x540
/// bytes, each by `006815c0`) through the vector iterator (null when the
/// allocation fails), 0x70 16-bit indices (0xE0 bytes); each point is the
/// unit-sphere point at `01187ac8` (`double`s stored as `float`s) times the
/// radius (+0x18), each colour the sphere's (+8), each index its number; a
/// strip length of 0x70 (2 bytes). An `NiTriStrips` (0xC4 bytes: 0x70
/// vertices, the points, the colours, 0x6E triangles, one strip) with a
/// `BSShaderNoLightingProperty` (0x80 bytes, `005a8060` given 0x21)
/// attached, prepared (`BSShaderManager::PrepareObject(geometry, 0, 0)`)
/// and stored at +0x24. An alpha property (0x1C bytes, `004391c0`) in a
/// stack `NiPointer`, named `"primitive: alpha"`, given 1 (`0049ed90`) and
/// attached; the geometry's property 3 gets flag 0x20 cleared (`SetFlag(0x20,
/// 0)`). A `BSFadeNode` (0xE4 bytes) at +0x2C gets the geometry (its slot
/// 0xDC with 1), is named `"Sphere"`; both have their properties updated;
/// the stack pointer is released. A failed allocation leaves its object
/// null as the exe does. With a geometry already, `SetRadius` with the first
/// size float and the sphere's slot 8 with its colour (+8). Either way a
/// zero point (`0043d410`) goes to the node (`00a59c60`).
pub fn bgs_primitive_sphere_build(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() == 0 {
        let colors_block = e.call(ALLOCATE_00401000, &args![0x700u32]).u32();
        let colors = if colors_block == 0 {
            0
        } else {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![colors_block, 0x10u32, SPHERE_VERTICES, 0x004a_7800u32],
            );
            colors_block
        };
        let points_block = e.call(ALLOCATE_00401000, &args![0x540u32]).u32();
        let points = if points_block == 0 {
            0
        } else {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![points_block, 0xcu32, SPHERE_VERTICES, MEMBER_18_SET_UP],
            );
            points_block
        };
        let indices = e.call(ALLOCATE_00401000, &args![0xe0u32]).u32();
        let radius = e.mem.u32(t + 0x18);
        for index in 0..SPHERE_VERTICES {
            for c in 0..3u32 {
                let unit = e.mem.u64(SPHERE_POINTS + index * 0x18 + 8 * c);
                let unit = x87_store32(x87_load64(unit));
                let value = x87_store32(x87_mul(x87_load32(unit), x87_m32(radius)));
                e.mem
                    .set_u32(points.wrapping_add(index * 0xc + 4 * c), value);
            }
            copy_color(e, colors.wrapping_add(index << 4), t + 8);
            e.mem.set_u16(indices.wrapping_add(index * 2), index as u16);
        }
        let lengths = e.call(ALLOCATE_00401000, &args![2u32]).u32();
        e.mem.set_u16(lengths, SPHERE_VERTICES as u16);
        let strips_block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
        let geometry = if strips_block == 0 {
            0
        } else {
            e.call(
                TRI_STRIPS_CONSTRUCT,
                &args![
                    strips_block,
                    SPHERE_VERTICES,
                    points,
                    0u32,
                    colors,
                    0u32,
                    0u32,
                    0u32,
                    0x6eu32,
                    1u32,
                    lengths,
                    indices
                ],
            )
            .u32()
        };
        let property_block = e.call(NI_ALLOCATE, &args![0x80u32]).u32();
        let property = if property_block == 0 {
            0
        } else {
            e.call(NO_LIGHTING_PROPERTY_CONSTRUCT, &args![property_block])
                .u32()
        };
        e.call(PROPERTY_SET_5A8060, &args![property, 0x21u32]);
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
        e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);
        e.call(NI_POINTER_SET, &args![t + 0x24, geometry]);
        let alpha_block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
        let alpha = if alpha_block == 0 {
            0
        } else {
            e.call(ALPHA_PROPERTY_CONSTRUCT, &args![alpha_block]).u32()
        };
        e.with_stack(0xc, |e, locals| {
            // The `NiPointer` (+0) and the two fixed strings (+4, +8).
            let pointer = locals.addr();
            e.call(NI_POINTER_FROM_RAW, &args![pointer, alpha]);
            let text = pointer + 4;
            let name = e
                .call(FIXED_STRING_CONSTRUCT, &args![text, ALPHA_NAME])
                .u32();
            let object = e.call(NI_POINTER_GET, &args![pointer]).u32();
            e.call(NODE_SET_NAME, &args![object, name]);
            e.call(FIXED_STRING_DESTRUCT, &args![text]);
            let object = e.call(NI_POINTER_GET, &args![pointer]).u32();
            e.call(ALPHA_PROPERTY_SET_49ED90, &args![object, 1u32]);
            let object = e.call(NI_POINTER_GET, &args![pointer]).u32();
            let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
            e.call(ATTACH_PROPERTY, &args![geometry, object]);
            let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
            let shader = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
            e.call(SHADER_PROPERTY_SET_FLAG, &args![shader, 0x20u32, 0u32]);
            let node_block = e.call(NI_ALLOCATE, &args![0xe4u32]).u32();
            let node = if node_block == 0 {
                0
            } else {
                e.call(FADE_NODE_CONSTRUCT, &args![node_block]).u32()
            };
            e.call(NI_POINTER_SET, &args![t + 0x2c, node]);
            let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
            let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
            e.vcall(node, 0xdc, &args![geometry, 1u32]);
            let text = pointer + 8;
            let name = e
                .call(FIXED_STRING_CONSTRUCT, &args![text, SPHERE_NAME])
                .u32();
            let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
            e.call(NODE_SET_NAME, &args![node, name]);
            e.call(FIXED_STRING_DESTRUCT, &args![text]);
            let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
            e.call(UPDATE_PROPERTIES, &args![node]);
            let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
            e.call(UPDATE_PROPERTIES, &args![geometry]);
            e.call(NI_POINTER_RELEASE, &args![pointer]);
        });
    } else {
        let at = fn_004a51b0(e, Ptr::new(t + 0x18), 0);
        let radius = reloaded(e.mem.u32(at));
        bgs_primitive_sphere_set_radius(e, this, f32::from_bits(radius));
        e.vcall(t, 8, &args![t + 8]);
    }
    e.with_stack(0xc, |e, point| {
        let zero = x87_store32(0.0);
        e.call(POINT3_ZERO_0043D410, &args![point, zero, 0u32, 0u32]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.call(NODE_SET_00A59C60, &args![node, point]);
    });
}

// Translated from 004a7830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitiveSphere::SetColor` (Xbox PDB; `RET 4`): the colour into +8;
/// with a geometry at +0x24 its data's 0x70 colours are set from it and
/// marked changed (4).
pub fn bgs_primitive_sphere_set_color(e: &mut Engine, this: Ptr, color: Ptr) {
    let t = this.addr();
    copy_color(e, t + 8, color.addr());
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() == 0 {
        return;
    }
    let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    let colors = e.call(DATA_COLORS, &args![data]).u32();
    for index in 0..SPHERE_VERTICES {
        copy_color(e, colors.wrapping_add(index << 4), t + 8);
    }
    e.call(MARK_AS_CHANGED, &args![data, 4u32]);
}

// Translated from 004a78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPrimitiveSphere::SetRadius` (Xbox PDB; `RET 4`): the radius becomes
/// its magnitude (`00408840`, stored as a `float`). With a geometry at
/// +0x24 each of its 0x70 vertices is the unit-sphere point at `01187ac8`
/// (`double`s stored as `float`s) scaled by the radius (`00439180`), the
/// data marked changed (1). The radius goes to +0x20, +0x1C and +0x18. A
/// zero point is made (`0043d410`); with a node at +0x2C a bound of a zero
/// centre (`00416870`) and the radius goes to the geometry
/// (`fn_004a5200`), the zero point to the node (`00a59c60`), the geometry's
/// properties are updated and the node's slot 0xBC runs.
pub fn bgs_primitive_sphere_set_radius(e: &mut Engine, this: Ptr, radius: f32) {
    let t = this.addr();
    let magnitude = e
        .call(MAGNITUDE_00408840, &args![reloaded(radius.to_bits())])
        .f64();
    let radius = x87_store32(magnitude);
    if e.call(NI_POINTER_GET, &args![t + 0x24]).u32() != 0 {
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        let vertices = e.call(DATA_VERTICES, &args![data]).u32();
        for index in 0..SPHERE_VERTICES {
            let vertex = vertices.wrapping_add(index * 0xc);
            for c in 0..3u32 {
                let unit = e.mem.u64(SPHERE_POINTS + index * 0x18 + 8 * c);
                let value = reloaded(x87_store32(x87_load64(unit)));
                e.mem.set_u32(vertex.wrapping_add(4 * c), value);
            }
            e.call(POINT3_SCALE_IN_PLACE, &args![vertex, reloaded(radius)]);
        }
        e.call(MARK_AS_CHANGED, &args![data, 1u32]);
    }
    for field in [0x20u32, 0x1c, 0x18] {
        e.mem.set_u32(t + field, reloaded(radius));
    }
    e.with_stack(0x30, |e, frame| {
        // The frame: the zero point (+0), the bound (+0x10), the centre
        // (+0x20).
        let point = frame.addr();
        let bound = point + 0x10;
        let center = point + 0x20;
        let zero = x87_store32(0.0);
        e.call(POINT3_ZERO_0043D410, &args![point, zero, 0u32, 0u32]);
        if e.call(NI_POINTER_GET, &args![t + 0x2c]).u32() == 0 {
            return;
        }
        e.call(BOUND_CONSTRUCT_006240D0, &args![bound]);
        let made = e.call(POINT3_SET, &args![center, zero, zero, zero]).u32();
        fn_004a51d0(
            e,
            Ptr::new(bound),
            Ptr::new(made),
            f32::from_bits(reloaded(radius)),
        );
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        fn_004a5200(e, Ptr::new(geometry), Ptr::new(bound));
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.call(NODE_SET_00A59C60, &args![node, point]);
        let geometry = e.call(NI_POINTER_GET, &args![t + 0x24]).u32();
        e.call(UPDATE_PROPERTIES, &args![geometry]);
        let node = e.call(NI_POINTER_GET, &args![t + 0x2c]).u32();
        e.vcall(node, 0xbc, &[]);
    });
}

// Translated from 004a7aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a7aa0 - the exception frame (FS:[0]) around the shape's construction is not translated
/// `BGSPrimitiveSphere::MakeMultiBoundShape` (Xbox PDB; `RET 4`, the word
/// never read): a `BSMultiBoundSphere` (0x1C bytes from the Gamebryo
/// allocator, `BSMultiBoundSphere::BSMultiBoundSphere`) at a zero centre
/// (`00416870`) with the radius (+0x18); null when the allocation fails.
pub fn bgs_primitive_sphere_make_multi_bound_shape(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    let radius = reloaded(e.mem.u32(this.addr() + 0x18));
    e.with_stack(0xc, |e, center| {
        let zero = x87_store32(0.0);
        let made = e.call(POINT3_SET, &args![center, zero, zero, zero]).u32();
        bs_multi_bound_sphere_bs_multi_bound_sphere(
            e,
            Ptr::new(block),
            Ptr::new(made),
            f32::from_bits(radius),
        )
        .addr()
    })
}

// Translated from 004a7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::BSMultiBoundSphere` (Xbox PDB; `RET 8`: the
/// centre's address and the radius): `BSMultiBoundShape`'s constructor, the
/// vtable, the centre at +0xC, the radius at +0x18 (`FLD`/`FSTP`). Returns
/// `this`.
pub fn bs_multi_bound_sphere_bs_multi_bound_sphere(
    e: &mut Engine,
    this: Ptr,
    center: Ptr,
    radius: f32,
) -> Ptr {
    let t = this.addr();
    e.call(MULTI_BOUND_SHAPE_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, MULTI_BOUND_SPHERE_VTABLE);
    for word in 0..3u32 {
        let v = e.mem.u32(center.addr() + 4 * word);
        e.mem.set_u32(t + 0xc + 4 * word, v);
    }
    e.mem.set_u32(t + 0x18, reloaded(radius.to_bits()));
    this
}

// Translated from 004a7b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::GetRTTI` (Xbox PDB): its `NiRTTI` (`01203108`).
pub fn bs_multi_bound_sphere_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_MULTI_BOUND_SPHERE
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a7290, fn_004a7290(Ptr) -> f32),
        entry!(0x004a_7040, fn_004a7040(Ptr) -> f64),
        entry!(0x004a_71f0, fn_004a71f0(Ptr, u32) -> Ptr),
        entry!(0x004a_51d0, fn_004a51d0(Ptr, Ptr, f32)),
        entry!(0x004a_7800, fn_004a7800(Ptr)),
        entry!(0x004a_51b0, fn_004a51b0(Ptr, u32) -> u32),
        entry!(0x004a_4f40, fn_004a4f40(Ptr, Ptr, f32) -> Ptr),
        entry!(0x004a_7ba0, bs_multi_bound_sphere_set_center(Ptr, Ptr)),
        entry!(0x004a_7090, fn_004a7090(Ptr)),
        entry!(0x004a_7c20, fn_004a7c20(Ptr)),
        entry!(0x004a_7220, fn_004a7220(Ptr)),
        entry!(0x004a_5250, fn_004a5250(Ptr, Ptr) -> Ptr),
        entry!(0x004a_7060, bs_multi_bound_aabb_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004a_6110, bs_occlusion_plane_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004a_7bf0, bs_multi_bound_sphere_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004a_4cc0, fn_004a4cc0(Ptr, Ptr) -> bool),
        entry!(0x004a_4e90, fn_004a4e90(u32, Ptr, u32) -> u32),
        entry!(0x004a_4f80, fn_004a4f80(Ptr, u32) -> u32),
        entry!(0x004a_5040, fn_004a5040(Ptr, u32) -> u32),
        entry!(0x004a_4ac0, fn_004a4ac0(Ptr) -> Ptr),
        entry!(0x004a_5280, fn_004a5280(Ptr) -> Ptr),
        entry!(0x004a_6370, fn_004a6370(Ptr) -> Ptr),
        entry!(0x004a_5300, bgs_primitive_plane_build(Ptr)),
        entry!(0x004a_63f0, fn_004a63f0(Ptr)),
        entry!(0x004a_53e0, bgs_primitive_plane_set_color(Ptr, Ptr)),
        entry!(0x004a_64e0, fn_004a64e0(Ptr, Ptr)),
        entry!(0x004a_6640, fn_004a6640(Ptr, Ptr)),
        entry!(0x004a_4c90, fn_004a4c90(Ptr, Ptr) -> u32),
        entry!(0x004a_4d40, fn_004a4d40(Ptr) -> u32),
        entry!(0x004a_5100, fn_004a5100(f32, u32) -> u32),
        entry!(0x004a_5200, fn_004a5200(Ptr, Ptr) -> Ptr),
        entry!(0x004a_5230, fn_004a5230(Ptr, Ptr) -> Ptr),
        entry!(0x004a_55a0, bgs_primitive_plane_set_radii(Ptr, Ptr)),
        entry!(0x004a_5f50, bgs_primitive_plane_make_occlusion_plane(Ptr) -> u32),
        entry!(0x004a_5ff0, fn_004a5ff0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004a_60e0, bs_occlusion_plane_get_rtti(Ptr) -> u32),
        entry!(
            0x004a_6100,
            bs_occlusion_plane_q_debug_geom_shape_size(Ptr) -> u32
        ),
        entry!(0x004a_6150, bgs_primitive_plane_make_portal(Ptr) -> u32),
        entry!(0x004a_61f0, fn_004a61f0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004a_62c0, bs_portal_get_rtti(Ptr) -> u32),
        entry!(0x004a_62d0, bs_portal_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004a_6310, fn_004a6310(Ptr)),
        entry!(0x004a_6d90, bgs_primitive_plane_is_inside(Ptr, Ptr) -> bool),
        entry!(0x004a_6e30, fn_004a6e30(Ptr, Ptr) -> u32),
        entry!(0x004a_6fc0, fn_004a6fc0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004a_7020, bs_multi_bound_aabb_get_rtti(Ptr) -> u32),
        entry!(
            0x004a_7030,
            bs_multi_bound_aabb_q_debug_geom_shape_size(Ptr) -> u32
        ),
        entry!(0x004a_70b0, fn_004a70b0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004a_70f0, bs_multi_bound_obb_get_rtti(Ptr) -> u32),
        entry!(
            0x004a_7100,
            bs_multi_bound_obb_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004a_7130, fn_004a7130(Ptr)),
        entry!(0x004a_7150, fn_004a7150(Ptr) -> Ptr),
        entry!(0x004a_71d0, bgs_primitive_sphere_set_radii(Ptr, Ptr)),
        entry!(0x004a_7240, bgs_primitive_sphere_is_inside(Ptr, Ptr) -> bool),
        entry!(0x004a_72d0, bgs_primitive_sphere_build(Ptr)),
        entry!(0x004a_7830, bgs_primitive_sphere_set_color(Ptr, Ptr)),
        entry!(0x004a_78e0, bgs_primitive_sphere_set_radius(Ptr, f32)),
        entry!(
            0x004a_7aa0,
            bgs_primitive_sphere_make_multi_bound_shape(Ptr, u32) -> u32
        ),
        entry!(
            0x004a_7b40,
            bs_multi_bound_sphere_bs_multi_bound_sphere(Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x004a_7b90, bs_multi_bound_sphere_get_rtti(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_squared_length() {
        let mut e = Engine::new();
        let v = Ptr::<()>::new(e.mem.alloc(12));
        e.mem.set_f32(v.addr(), 1.0);
        e.mem.set_f32(v.addr() + 4, 2.0);
        e.mem.set_f32(v.addr() + 8, 3.0);
        assert_eq!(e.call(0x004a_7290, &args![v]).f32(), 14.0);
    }

    #[test]
    fn l68d_004a7040_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00457990, |_, a| Ret {
            st0: a[0] as f64 / 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_7040, &args![0x4100u32]).f64(), 4166.0);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00457990).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
    #[test]
    fn at_004a71f0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004a_7220, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004a_71f0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004a_71f0)
                .collect();
            let mut expected = vec![(0x004a_7220, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    fn engine(doubles: bool) -> Engine {
        let mut e = Engine::new();
        if doubles {
            e.map(FLOAT_TWO, 4);
            e.mem.set_f32(FLOAT_TWO, 2.0);
            e.map(FLOAT_HALF, 4);
            e.mem.set_f32(FLOAT_HALF, 0.5);
            e.map(DOUBLE_HALF, 8);
            e.mem.set_u64(DOUBLE_HALF, 0.5f64.to_bits());
            e.map(DOUBLE_THOUSANDTH, 8);
            e.mem.set_u64(DOUBLE_THOUSANDTH, 0.001f64.to_bits());
        }
        e
    }

    #[test]
    fn fn_004a51d0_and_004a5250_set_the_record() {
        let mut e = engine(false);
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_u32(other + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(other + 0xc, 0x7f80_0001);
        assert_eq!(e.call(0x004a_5250, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0x102);
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        e.call(0x004a_51d0, &args![this, other, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xc), 2.5);
    }

    #[test]
    fn small_helpers() {
        let mut e = engine(false);
        e.register(COLOR_SET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004a_7800, &args![0x500u32]);
        assert_eq!(calls(&mut e), vec![(COLOR_SET, vec![0x500, 0, 0, 0, 0])]);
        assert_eq!(e.call(0x004a_51b0, &args![0x500u32, 3u32]).u32(), 0x50c);
        let point = e.mem.alloc(8);
        e.mem.set_f32(point, 1.5);
        e.mem.set_f32(point + 4, -2.0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x004a_4f40, &args![point, out, 2.0f32]).u32(), out);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (3.0, -4.0));
        let sphere = e.mem.alloc(0x1c);
        e.call(0x004a_7ba0, &args![sphere, point]);
        assert_eq!(e.mem.f32(sphere + 0xc), 1.5);
        assert_eq!(e.mem.f32(sphere + 0x10), -2.0);
    }

    #[test]
    fn destructors_free_their_sizes() {
        for (address, size) in [
            (0x004a_7060u32, 0x24u32),
            (0x004a_6110, 0xfc),
            (0x004a_7bf0, 0x1c),
        ] {
            let mut e = engine(false);
            e.register(NI_OBJECT_DESTRUCT, |_, _| Ret::default());
            e.register(SIZED_FREE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![0x600u32, 1u32]).u32(), 0x600);
            assert_eq!(
                calls(&mut e),
                vec![
                    (NI_OBJECT_DESTRUCT, vec![0x600]),
                    (SIZED_FREE, vec![0x600, size])
                ]
            );
            e.call_log = Some(vec![]);
            e.call(address, &args![0x600u32, 0u32]);
            assert_eq!(calls(&mut e), vec![(NI_OBJECT_DESTRUCT, vec![0x600])]);
        }
        let mut e = engine(false);
        e.register(DESTRUCT_004A4BD0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004a_7220, &args![0x700u32]);
        assert_eq!(calls(&mut e), vec![(DESTRUCT_004A4BD0, vec![0x700])]);
    }

    #[test]
    fn fn_004a4cc0_compares_four_floats() {
        let mut e = engine(false);
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_f32(a + 4 * i, i as f32);
            e.mem.set_f32(b + 4 * i, i as f32);
        }
        assert!(e.call(0x004a_4cc0, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 9.0);
        assert!(!e.call(0x004a_4cc0, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 3.0);
        e.mem.set_f32(a, f32::NAN);
        e.mem.set_f32(b, f32::NAN);
        assert!(!e.call(0x004a_4cc0, &args![a, b]).bool());
    }

    const SHAPE_SLOTS: u32 = 0x0138_0000;

    /// Engine for the makers: allocations from the heap, the vtables of
    /// both primitive classes mapped with recorder slots.
    fn maker_engine() -> Engine {
        let mut e = engine(true);
        e.register(ALLOCATE_00401000, |e, a| e.mem.alloc(a[0]).into_ret());
        for f in [
            COLOR_SET,
            MEMBER_18_SET_UP,
            NI_POINTER_FROM_RAW,
            NI_POINTER_SET,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        for table in [BGS_PRIMITIVE_PLANE_VTABLE, BGS_PRIMITIVE_BOX_VTABLE] {
            e.map(table, 0x20);
            for slot in (0..0x20).step_by(4) {
                e.mem.set_u32(table + slot, SHAPE_SLOTS + slot);
                e.register(SHAPE_SLOTS + slot, |_, _| Ret::default());
            }
        }
        e.register(POINT3_SCALED, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT2_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn fn_004a4e90_makes_a_plane() {
        let mut e = maker_engine();
        let size = e.mem.alloc(12);
        e.mem.set_f32(size, 3.0);
        e.mem.set_f32(size + 4, 7.0);
        e.mem.set_f32(size + 8, 5.0);
        e.call_log = Some(vec![]);
        let plane = e.call(0x004a_4e90, &args![3u32, size, 0x44u32]).u32();
        assert_eq!(e.mem.u32(plane), BGS_PRIMITIVE_PLANE_VTABLE);
        assert_eq!(e.mem.u32(plane + 4), 3);
        assert_eq!(e.mem.u32(plane + 0x30), 7);
        // The point (3, 5) doubled, then halved.
        assert_eq!(e.mem.f32(plane + 0x18), 3.0);
        assert_eq!(e.mem.f32(plane + 0x1c), 1.0);
        assert_eq!(e.mem.f32(plane + 0x20), 5.0);
        assert!(calls(&mut e).contains(&(SHAPE_SLOTS + 8, vec![plane, 0x44])));
    }

    #[test]
    fn fn_004a4e90_makes_a_box_and_the_rest() {
        let mut e = maker_engine();
        const SEEN: u32 = 0x0138_1000;
        e.map(SEEN, 4);
        e.register(SHAPE_SLOTS + 0xc, |e, a| {
            let first = e.mem.u32(a[1]);
            e.mem.set_u32(SEEN, first);
            Ret::default()
        });
        let size = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_f32(size + 4 * i, 2.0 + i as f32);
        }
        e.call_log = Some(vec![]);
        let shape = e.call(0x004a_4e90, &args![1u32, size, 0x44u32]).u32();
        assert_eq!(e.mem.u32(shape), BGS_PRIMITIVE_BOX_VTABLE);
        assert_eq!(e.mem.u32(shape + 4), 1);
        let log = calls(&mut e);
        assert_eq!(log.iter().filter(|c| c.0 == SHAPE_SLOTS + 0xc).count(), 1);
        // The size doubled (0045bb20 with 2.0), then halved for the box.
        assert_eq!(e.mem.f32(SEEN), 2.0);
        e.register(MAKE_004A5100, |_, a| {
            (a[1] + f32::from_bits(a[0]) as u32).into_ret()
        });
        assert_eq!(e.call(0x004a_4e90, &args![2u32, size, 0x40u32]).u32(), 0x42);
        assert_eq!(e.call(0x004a_4e90, &args![4u32, size, 0x40u32]).u32(), 0);
    }

    #[test]
    fn fn_004a4ac0_builds_the_base() {
        let mut e = maker_engine();
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_4ac0, &args![this]).u32(), this);
        let log = calls(&mut e);
        assert_eq!(log[0], (COLOR_SET, vec![this + 8, 0, 0, 0, 0]));
        let sets: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == NI_POINTER_SET)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(sets, vec![0x24, 0x2c, 0x28]);
        assert_eq!(e.mem.u32(this), BGS_PRIMITIVE_VTABLE);
    }

    fn node_engine(existing: u32) -> Engine {
        let mut e = engine(false);
        let block = 0x0139_0000u32;
        e.register(NI_ALLOCATE, |_, _| 0x0139_0000u32.into_ret());
        e.register(FADE_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register_double(NI_POINTER_GET, move |_, _| {
            (if existing != 0 { existing } else { block }).into_ret()
        });
        for f in [
            NI_POINTER_SET,
            NODE_SET_NAME,
            FIXED_STRING_DESTRUCT,
            UPDATE_PROPERTIES,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn bgs_primitive_plane_build_names_a_new_node() {
        let mut e = node_engine(0);
        e.call(0x004a_5300, &args![0x800u32]);
        let log = calls(&mut e);
        assert!(log.contains(&(NI_POINTER_SET, vec![0x82c, 0x0139_0000])));
        let named: Vec<_> = log
            .iter()
            .filter(|c| c.0 == FIXED_STRING_CONSTRUCT)
            .collect();
        assert_eq!(named[0].1[1], PLANE_NAME);
        assert_eq!(log.last().unwrap(), &(UPDATE_PROPERTIES, vec![0x0139_0000]));
        // The box builds only without a node.
        let mut e = node_engine(0x7777);
        e.call(0x004a_63f0, &args![0x800u32]);
        assert_eq!(calls(&mut e), vec![(NI_POINTER_GET, vec![0x82c])]);
    }

    /// The colour setters' engine: the geometry pointers answer `geometry`,
    /// whose data's colours are `colors`, with `vertices` vertices.
    fn color_engine(geometry: u32, colors: u32, vertices: u32) -> Engine {
        let mut e = engine(true);
        e.register_double(NI_POINTER_GET, move |_, _| geometry.into_ret());
        e.register(GEOMETRY_DATA, |_, a| (a[0] + 1).into_ret());
        e.register_double(DATA_COLORS, move |_, _| colors.into_ret());
        e.register_double(VERTEX_COUNT, move |_, _| vertices.into_ret());
        e.register(MARK_AS_CHANGED, |_, _| Ret::default());
        e.register(SET_ALPHA_00450F90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn bgs_primitive_plane_set_color_fills_eight_colours() {
        let mut e = color_engine(0x900, 0, 0);
        let colors = e.mem.alloc(0x80);
        e.register_double(DATA_COLORS, move |_, _| colors.into_ret());
        let this = e.mem.alloc(0x34);
        let color = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_f32(color + 4 * i, 0.25 * i as f32);
        }
        e.call(0x004a_53e0, &args![this, color]);
        for c in 0..8 {
            assert_eq!(e.mem.f32(colors + 0x10 * c + 4), 0.25);
        }
        let log = calls(&mut e);
        assert!(log.contains(&(MARK_AS_CHANGED, vec![0x901, 4])));
        // Alpha 0.75 is not below 0.001.
        assert!(log.contains(&(SET_ALPHA_00450F90, vec![0x900, 0])));
        // No geometry: only the colour is kept.
        let mut e = color_engine(0, 0, 0);
        let this = e.mem.alloc(0x34);
        e.call(0x004a_53e0, &args![this, color]);
        assert!(calls(&mut e).iter().all(|c| c.0 != SET_ALPHA_00450F90));
    }

    #[test]
    fn fn_004a64e0_paints_every_vertex_of_both_geometries() {
        let mut e = color_engine(0x900, 0, 3);
        let colors = e.mem.alloc(0x40);
        e.register_double(DATA_COLORS, move |_, _| colors.into_ret());
        let this = e.mem.alloc(0x34);
        let color = e.mem.alloc(16);
        e.mem.set_f32(color, 1.0);
        e.call(0x004a_64e0, &args![this, color]);
        for v in 0..3 {
            assert_eq!(e.mem.f32(colors + 0x10 * v), 1.0);
        }
        assert_eq!(e.mem.f32(colors + 0x30), 0.0);
        let log = calls(&mut e);
        assert_eq!(log.iter().filter(|c| c.0 == MARK_AS_CHANGED).count(), 2);
        // Alpha 0.0 is below 0.001.
        assert!(log.contains(&(SET_ALPHA_00450F90, vec![0x900, 1])));
    }
}

#[cfg(test)]
mod b008_box_tests {
    use super::*;

    const GEOMETRY_A: u32 = 0x013e_0000;
    const GEOMETRY_B: u32 = 0x013e_1000;
    const NODE_SLOT: u32 = 0x013e_2000;
    const SEEN_RADIUS: u32 = 0x013e_3000;

    struct BoxCase {
        e: Engine,
        this: u32,
        corners: u32,
        normals: u32,
        node: u32,
    }

    fn engine(node_present: bool) -> BoxCase {
        let mut e = Engine::new();
        for (at, bits) in [(DOUBLE_SMALL, 0.01f64.to_bits()), (DOUBLE_ZERO, 0)] {
            e.map(at, 8);
            e.mem.set_u64(at, bits);
        }
        e.map(FLOAT_SMALL, 4);
        e.mem.set_f32(FLOAT_SMALL, 0.01);
        e.map(FLOAT_MINUS_ONE, 4);
        e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
        e.map(ORIGIN_011F426C, 0xc);
        e.register(MAGNITUDE_00408840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).abs(),
            ..Ret::default()
        });
        e.register(POINT3_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        let corners = e.mem.alloc(0x60);
        let normals = e.mem.alloc(0x18);
        e.mem.set_f32(normals, -3.0);
        e.mem.set_f32(normals + 4, 2.0);
        e.mem.set_f32(normals + 8, 0.0);
        e.mem.set_f32(normals + 0xc, 1.0);
        e.mem.set_f32(normals + 0x10, -1.0);
        e.mem.set_f32(normals + 0x14, 5.0);
        let node = e.mem.alloc(4);
        let table = e.mem.alloc(0xc0);
        e.mem.set_u32(table + 0xbc, NODE_SLOT);
        e.mem.set_u32(node, table);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x24, GEOMETRY_A);
        e.mem.set_u32(this + 0x28, GEOMETRY_B);
        e.mem
            .set_u32(this + 0x2c, if node_present { node } else { 0 });
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GEOMETRY_DATA, |_, a| (a[0] + 1).into_ret());
        e.register_double(DATA_VERTICES, move |_, a| {
            (if a[0] == GEOMETRY_A + 1 {
                corners
            } else {
                normals
            })
            .into_ret()
        });
        e.register(VERTEX_COUNT, |_, _| 2u32.into_ret());
        e.map(SEEN_RADIUS, 4);
        e.register(GEOMETRY_SET_BOUND_004A5200, |e, a| {
            let radius = e.mem.u32(a[1] + 0xc);
            e.mem.set_u32(SEEN_RADIUS, radius);
            Ret::default()
        });
        for f in [
            MARK_AS_CHANGED,
            POINT3_ZERO_0043D410,
            BOUND_CONSTRUCT_006240D0,
            NODE_SET_00A59C60,
            UPDATE_PROPERTIES,
            NODE_SLOT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        BoxCase {
            e,
            this,
            corners,
            normals,
            node,
        }
    }

    #[test]
    fn fn_004a6640_builds_the_box() {
        let mut b = engine(true);
        let size = b.e.mem.alloc(12);
        b.e.mem.set_f32(size, -2.0);
        b.e.mem.set_f32(size + 4, 4.0);
        b.e.mem.set_f32(size + 8, 0.005);
        b.e.call(0x004a_6640, &args![b.this, size]);
        let e = &mut b.e;
        assert_eq!(e.mem.f32(size), 2.0);
        assert_eq!(e.mem.f32(b.this + 0x18), 2.0);
        // Corner 0 is (-x, y, z), corner 7 (-x, -y, -z).
        assert_eq!(e.mem.f32(b.corners), -2.0);
        assert_eq!(e.mem.f32(b.corners + 4), 4.0);
        assert_eq!(e.mem.f32(b.corners + 0x54 + 8), -0.005);
        // The normals: signs times the size, the tiny z counting as 0.01.
        let n: Vec<f32> = (0..6).map(|i| e.mem.f32(b.normals + 4 * i)).collect();
        assert_eq!(n, vec![-2.0, 4.0, 0.01, 2.0, -4.0, 0.01]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == MARK_AS_CHANGED).count(), 3);
        let bound: Vec<_> = log
            .iter()
            .filter(|c| c.0 == GEOMETRY_SET_BOUND_004A5200)
            .collect();
        assert_eq!(bound[0].1[0], GEOMETRY_A);
        // The radius is the largest coordinate (4.0, the second).
        assert_eq!(e.mem.f32(SEEN_RADIUS), 4.0);
        assert!(log.contains(&(NODE_SLOT, vec![b.node])));
    }

    #[test]
    fn fn_004a6640_without_a_node() {
        let mut b = engine(false);
        let size = b.e.mem.alloc(12);
        b.e.mem.set_f32(size + 8, 1.0);
        b.e.call(0x004a_6640, &args![b.this, size]);
        let log = b.e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != GEOMETRY_SET_BOUND_004A5200));
        assert!(log.iter().any(|c| c.0 == POINT3_ZERO_0043D410));
    }
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn f(value: f32) -> u32 {
        value.to_bits()
    }

    const VTABLE: u32 = 0x0600_0000;
    const SLOT_TARGET: u32 = 0x7000_0000;

    /// Doubles: an `NiPointer` is its word, a geometry is its data whose
    /// word +0 is the vertex array and +4 the colour array, magnitudes are
    /// `abs`, points are written where asked; one vtable whose slot `n` runs
    /// the no-op double at `0x70000000 + n`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| SLOT_TARGET + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x40 {
            e.register(SLOT_TARGET + 4 * i, |_, _| Ret::default());
        }
        for addr in [
            MARK_AS_CHANGED,
            BOUND_CONSTRUCT_006240D0,
            NODE_SET_00A59C60,
            UPDATE_PROPERTIES,
            POINT3_ZERO_0043D410,
            NI_OBJECT_DESTRUCT,
            NI_POINTER_RELEASE,
            SIZED_FREE,
            NI_OBJECT_CONSTRUCT,
            MULTI_BOUND_SHAPE_CONSTRUCT,
            FRUSTUM_PLANES_CONSTRUCT,
            VECTOR_CONSTRUCTOR,
            MEMBER_18_SET_UP,
            COLOR_SET,
            FROM_EULER_ANGLES,
            SET_ROTATION_4396B0,
            POINT3_SCALE_IN_PLACE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(GEOMETRY_DATA, |_, a| ret(a[0]));
        e.register(DATA_VERTICES, |e, a| ret(e.mem.u32(a[0])));
        e.register(DATA_COLORS, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(MAGNITUDE_00408840, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).abs())
        });
        e.register(POINT3_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(POINT2_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(ALLOCATE_00401000, |e, a| ret(e.mem.alloc(a[0])));
        e.map(ORIGIN_011F426C, 0x10);
        e.map(IDENTITY_ROTATION, 0x24);
        for vtable in [
            BGS_PRIMITIVE_PLANE_VTABLE,
            BGS_PRIMITIVE_BOX_VTABLE,
            SPHERE_VTABLE,
        ] {
            e.put_vtable(vtable, &slots[..0x10]);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// A geometry with `count` vertices and colours (its object at +0xB8
    /// left for the bound).
    fn geometry(e: &mut Engine, count: u32) -> u32 {
        let data = e.mem.alloc(0xbc);
        let vertices = e.mem.alloc(0xc * count);
        let colors = e.mem.alloc(0x10 * count);
        e.mem.set_u32(data, vertices);
        e.mem.set_u32(data + 4, colors);
        data
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn rtti_sizes_and_colour_inequality() {
        let mut e = engine();
        for (addr, value) in [
            (0x004a_60e0u32, RTTI_OCCLUSION_PLANE),
            (0x004a_6100, 0x1e),
            (0x004a_62c0, RTTI_PORTAL),
            (0x004a_7020, RTTI_MULTI_BOUND_AABB),
            (0x004a_7030, 0x24),
            (0x004a_70f0, RTTI_MULTI_BOUND_OBB),
            (0x004a_7b90, RTTI_MULTI_BOUND_SPHERE),
        ] {
            assert_eq!(e.call(addr, &[0x1234]).u32(), value);
        }
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004a_4c90, &[a, b]).u32(), 0);
        e.mem.set_u32(b + 0xc, f(1.0));
        assert_eq!(e.call(0x004a_4c90, &[a, b]).u32(), 1);
    }

    #[test]
    fn destructor_chains() {
        let mut e = engine();
        let portal = e.mem.alloc(0x108);
        assert_eq!(e.call(0x004a_62d0, &[portal, 1]).u32(), portal);
        assert_eq!(calls_to(&e, NI_POINTER_RELEASE), vec![vec![portal + 0x104]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCT), vec![vec![portal]]);
        assert_eq!(calls_to(&e, SIZED_FREE), vec![vec![portal, 0x108]]);
        let obb = e.mem.alloc(0x48);
        assert_eq!(e.call(0x004a_7100, &[obb, 0]).u32(), obb);
        e.call(0x004a_7100, &[obb, 1]);
        assert_eq!(calls_to(&e, SIZED_FREE)[1], vec![obb, 0x48]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCT).len(), 3);
    }

    #[test]
    fn shape_constructors() {
        let mut e = engine();
        let center = e.mem.alloc(0xc);
        let size = e.mem.alloc(0xc);
        let rotation = e.mem.alloc(0x24);
        for i in 0..3 {
            e.mem.set_u32(center + 4 * i, 0x10 + i);
            e.mem.set_u32(size + 4 * i, 0x20 + i);
        }
        for i in 0..9 {
            e.mem.set_u32(rotation + 4 * i, 0x30 + i);
        }
        let plane = e.mem.alloc(0xfc);
        assert_eq!(
            e.call(0x004a_5ff0, &[plane, center, size, rotation]).u32(),
            plane
        );
        assert_eq!(e.mem.u32(plane), OCCLUSION_PLANE_VTABLE);
        assert_eq!(e.mem.u32(plane + 0x10), 0x12);
        assert_eq!(e.mem.u32(plane + 0x18), 0x21);
        assert_eq!(e.mem.u32(plane + 0x3c), 0x38);
        assert_eq!(e.mem.u8(plane + 0xe4), 1);
        assert_eq!(e.mem.u8(plane + 0xea), 1);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR),
            vec![vec![plane + 0xa8, 0xc, 4, MEMBER_18_SET_UP]]
        );
        let portal = e.mem.alloc(0x108);
        e.mem.set_u32(portal + 0x100, 9);
        e.call(0x004a_61f0, &[portal, center, size, rotation]);
        assert_eq!(e.mem.u32(portal), PORTAL_VTABLE);
        assert_eq!(e.mem.u32(portal + 0x100), 0);
        assert_eq!(
            calls_to(&e, NI_POINTER_FROM_RAW),
            vec![vec![portal + 0x104, 0]]
        );
        let obb = e.mem.alloc(0x48);
        e.call(0x004a_70b0, &[obb, center, size, rotation]);
        assert_eq!(e.mem.u32(obb), MULTI_BOUND_OBB_VTABLE);
        assert_eq!(e.mem.u32(obb + 0xc), 0x10);
        assert_eq!(e.mem.u32(obb + 0x20), 0x22);
        assert_eq!(e.mem.u32(obb + 0x44), 0x38);
        let aabb = e.mem.alloc(0x24);
        e.call(0x004a_6fc0, &[aabb, center, size]);
        assert_eq!(e.mem.u32(aabb), MULTI_BOUND_AABB_VTABLE);
        let sphere_shape = e.mem.alloc(0x1c);
        e.call(0x004a_7b40, &[sphere_shape, center, 0x7f80_0001]);
        assert_eq!(e.mem.u32(sphere_shape), MULTI_BOUND_SPHERE_VTABLE);
        assert_eq!(e.mem.u32(sphere_shape + 0x14), 0x12);
        assert_eq!(e.mem.u32(sphere_shape + 0x18), 0x7fc0_0001);
        assert_eq!(calls_to(&e, MULTI_BOUND_SHAPE_CONSTRUCT).len(), 3);
        let sphere = e.mem.alloc(0x34);
        assert_eq!(e.call(0x004a_7150, &[sphere]).u32(), sphere);
        assert_eq!(e.mem.u32(sphere), SPHERE_VTABLE);
        assert_eq!(e.mem.u32(sphere + 4), 2);
        assert_eq!(e.mem.u32(sphere + 0x30), 7);
    }

    #[test]
    fn bound_goes_to_the_object_at_0xb8() {
        let mut e = engine();
        let geometry = e.mem.alloc(0xbc);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0xb8, target);
        let bound = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(bound + 4 * i, 0x100 + i);
        }
        assert_eq!(e.call(0x004a_5200, &[geometry, bound]).u32(), target + 0x10);
        assert_eq!(e.mem.u32(target + 0x10), 0x100);
        assert_eq!(e.mem.u32(target + 0x1c), 0x103);
    }

    #[test]
    fn plane_set_radii_builds_both_geometries_and_the_bound() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        let first = geometry(&mut e, 8);
        let second = geometry(&mut e, 16);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0xb8, target);
        let node = object(&mut e, 8);
        e.mem.set_u32(this + 0x24, first);
        e.mem.set_u32(this + 0x28, second);
        e.mem.set_u32(this + 0x2c, node);
        let size = e.mem.alloc(0xc);
        e.mem.set_u32(size, f(-2.0));
        e.mem.set_u32(size + 4, f(9.0));
        e.mem.set_u32(size + 8, f(3.0));
        e.call(0x004a_55a0, &[this, size]);
        assert_eq!(point(&e, size), [2.0, 0.0, 3.0]);
        assert_eq!(point(&e, this + 0x18), [2.0, 0.0, 3.0]);
        let v = e.mem.u32(first);
        assert_eq!(point(&e, v), [-2.0, 0.0, 3.0]);
        assert_eq!(point(&e, v + 0xc * 7), [-2.0, -0.0, -3.0]);
        let w = e.mem.u32(second);
        let expected: [[f32; 3]; 16] = [
            [-2.0, 0.0, 3.0],
            [2.0, 0.0, 3.0],
            [2.0, -0.0, 3.0],
            [-2.0, -0.0, 3.0],
            [-2.0, 0.0, 3.0],
            [-2.0, 0.0, -3.0],
            [-2.0, -0.0, -3.0],
            [2.0, -0.0, -3.0],
            [2.0, 0.0, -3.0],
            [-2.0, 0.0, -3.0],
            [-2.0, -0.0, 3.0],
            [-2.0, -0.0, -3.0],
            [2.0, 0.0, 3.0],
            [2.0, 0.0, -3.0],
            [2.0, -0.0, 3.0],
            [2.0, -0.0, -3.0],
        ];
        for (i, vertex) in expected.iter().enumerate() {
            let got = point(&e, w + 0xc * i as u32);
            assert_eq!(got.map(f32::to_bits), vertex.map(f32::to_bits), "{i}");
        }
        assert_eq!(
            calls_to(&e, MARK_AS_CHANGED),
            vec![vec![first, 1], vec![second, 1]]
        );
        // The radius: the largest of (2, 0, 3).
        assert_eq!(e.mem.f32(target + 0x1c), 3.0);
        assert_eq!(e.mem.u32(target + 0x10), e.mem.u32(ORIGIN_011F426C));
        let pt = calls_to(&e, NODE_SET_00A59C60)[0][1];
        assert_eq!(calls_to(&e, SLOT_TARGET + 0xb8), vec![vec![node, pt]]);
        assert_eq!(calls_to(&e, SLOT_TARGET + 0xbc), vec![vec![node]]);
        assert_eq!(calls_to(&e, UPDATE_PROPERTIES), vec![vec![first]]);
    }

    #[test]
    fn plane_set_radii_without_geometry_or_node() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        let size = e.mem.alloc(0xc);
        e.mem.set_u32(size, f(1.0));
        e.call(0x004a_55a0, &[this, size]);
        assert!(calls_to(&e, MARK_AS_CHANGED).is_empty());
        assert_eq!(calls_to(&e, POINT3_ZERO_0043D410).len(), 1);
        assert!(calls_to(&e, BOUND_CONSTRUCT_006240D0).is_empty());
    }

    #[test]
    fn plane_is_inside() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_u32(this + 0x18 + 4 * i as u32, f(v));
        }
        let p = e.mem.alloc(0xc);
        assert!(e.call(0x004a_6d90, &[this, p]).bool());
        for (i, outside) in [(0u32, 1.5f32), (1, -2.5), (2, 3.5)] {
            e.mem.set_u32(p + 4 * i, f(outside));
            assert!(!e.call(0x004a_6d90, &[this, p]).bool());
            e.mem.set_u32(p + 4 * i, f(f32::NAN));
            assert!(e.call(0x004a_6d90, &[this, p]).bool());
            e.mem.set_u32(p + 4 * i, 0);
        }
        e.mem.set_u32(p, f(1.0));
        assert!(e.call(0x004a_6d90, &[this, p]).bool());
    }

    #[test]
    fn occlusion_plane_and_portal_from_the_plane() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x18, f(4.0));
        e.mem.set_u32(this + 0x20, f(6.0));
        let plane = e.call(0x004a_5f50, &[this]).u32();
        assert_eq!(e.mem.u32(plane), OCCLUSION_PLANE_VTABLE);
        assert_eq!(e.mem.f32(plane + 0x14), 4.0);
        assert_eq!(e.mem.f32(plane + 0x18), 6.0);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0xfc]]);
        let portal = e.call(0x004a_6150, &[this]).u32();
        assert_eq!(e.mem.u32(portal), PORTAL_VTABLE);
        assert_eq!(e.mem.f32(portal + 0x18), 6.0);
        e.register(NI_ALLOCATE, |_, _| ret(0));
        assert_eq!(e.call(0x004a_5f50, &[this]).u32(), 0);
        assert_eq!(e.call(0x004a_6150, &[this]).u32(), 0);
    }

    #[test]
    fn box_multi_bound_shape_by_rotation() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        let angles = e.mem.alloc(0xc);
        e.mem.map(DOUBLE_SMALL, 8);
        e.mem.set_u64(DOUBLE_SMALL, 0.01f64.to_bits());
        e.mem.set_u32(angles + 4, f(0.005));
        let aabb = e.call(0x004a_6e30, &[this, angles]).u32();
        assert_eq!(e.mem.u32(aabb), MULTI_BOUND_AABB_VTABLE);
        assert!(calls_to(&e, FROM_EULER_ANGLES).is_empty());
        e.mem.set_u32(angles + 8, f(-0.5));
        let obb = e.call(0x004a_6e30, &[this, angles]).u32();
        assert_eq!(e.mem.u32(obb), MULTI_BOUND_OBB_VTABLE);
        let matrix = calls_to(&e, FROM_EULER_ANGLES)[0][0];
        assert_eq!(
            calls_to(&e, FROM_EULER_ANGLES),
            vec![vec![matrix, 0, f(0.005), f(-0.5)]]
        );
        assert_eq!(calls_to(&e, SET_ROTATION_4396B0), vec![vec![obb, matrix]]);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x24], vec![0x48]]);
    }

    #[test]
    fn sphere_is_inside_and_colour() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x18, f(2.0));
        let p = e.mem.alloc(0xc);
        e.mem.set_u32(p, f(1.0));
        assert!(e.call(0x004a_7240, &[this, p]).bool());
        e.mem.set_u32(p + 4, f(2.0));
        assert!(!e.call(0x004a_7240, &[this, p]).bool());
        let color = e.mem.alloc(0x10);
        e.mem.set_u32(color + 0xc, f(0.5));
        e.call(0x004a_7830, &[this, color]);
        assert_eq!(e.mem.f32(this + 0x14), 0.5);
        assert!(calls_to(&e, MARK_AS_CHANGED).is_empty());
        let geometry = geometry(&mut e, 0x70);
        e.mem.set_u32(this + 0x24, geometry);
        e.call(0x004a_7830, &[this, color]);
        let colors = e.mem.u32(geometry + 4);
        assert_eq!(e.mem.f32(colors + 0x6f0 + 0xc), 0.5);
        assert_eq!(calls_to(&e, MARK_AS_CHANGED), vec![vec![geometry, 4]]);
    }

    fn map_sphere_points(e: &mut Engine) {
        e.mem.map(SPHERE_POINTS, 0x70 * 0x18);
        for i in 0..0x70u32 {
            e.mem.set_u64(SPHERE_POINTS + i * 0x18, 1.0f64.to_bits());
            e.mem
                .set_u64(SPHERE_POINTS + i * 0x18 + 8, (-0.5f64).to_bits());
            e.mem
                .set_u64(SPHERE_POINTS + i * 0x18 + 16, f64::from(i).to_bits());
        }
    }

    #[test]
    fn sphere_set_radius_and_radii() {
        let mut e = engine();
        map_sphere_points(&mut e);
        let this = e.mem.alloc(0x34);
        let geometry = geometry(&mut e, 0x70);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0xb8, target);
        let node = object(&mut e, 8);
        e.mem.set_u32(this + 0x24, geometry);
        e.mem.set_u32(this + 0x2c, node);
        let radii = e.mem.alloc(0xc);
        e.mem.set_u32(radii, f(-3.0));
        e.call(0x004a_71d0, &[this, radii]);
        assert_eq!(point(&e, this + 0x18), [3.0, 3.0, 3.0]);
        let vertices = e.mem.u32(geometry);
        assert_eq!(point(&e, vertices + 0xc * 5), [1.0, -0.5, 5.0]);
        let scaled = calls_to(&e, POINT3_SCALE_IN_PLACE);
        assert_eq!(scaled.len(), 0x70);
        assert_eq!(scaled[1], vec![vertices + 0xc, f(3.0)]);
        assert_eq!(e.mem.f32(target + 0x1c), 3.0);
        assert_eq!(e.mem.u32(target + 0x10), 0);
        assert_eq!(calls_to(&e, SLOT_TARGET + 0xbc), vec![vec![node]]);
        assert_eq!(calls_to(&e, UPDATE_PROPERTIES), vec![vec![geometry]]);
    }

    #[test]
    fn sphere_build_makes_the_geometry_once() {
        let mut e = engine();
        map_sphere_points(&mut e);
        for addr in [
            PROPERTY_SET_5A8060,
            ATTACH_PROPERTY,
            PREPARE_OBJECT,
            ALPHA_PROPERTY_SET_49ED90,
            SHADER_PROPERTY_SET_FLAG,
            FIXED_STRING_DESTRUCT,
            NODE_SET_NAME,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TRI_STRIPS_CONSTRUCT, |e, a| {
            let target = e.mem.alloc(0x20);
            e.mem.set_u32(a[0] + 0xb8, target);
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[4]);
            ret(a[0])
        });
        e.register(NO_LIGHTING_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ALPHA_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FADE_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            ret(a[0])
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| ret(a[1]));
        e.register(GET_PROPERTY, |_, a| ret(a[0] + 3));
        let this = object(&mut e, 0x34);
        e.mem.set_u32(this + 0x18, f(2.0));
        e.mem.set_u32(this + 8, f(0.25));
        e.call(0x004a_72d0, &[this]);
        let strips = calls_to(&e, TRI_STRIPS_CONSTRUCT);
        assert_eq!(strips.len(), 1);
        let s = &strips[0];
        assert_eq!(s[1], 0x70);
        assert_eq!(s[3..10], [0, s[4], 0, 0, 0, 0x6e, 1]);
        let points = s[2];
        let colors = s[4];
        assert_eq!(point(&e, points + 0xc * 3), [2.0, -1.0, 6.0]);
        assert_eq!(e.mem.f32(colors + 0x10 * 0x6f), 0.25);
        assert_eq!(e.mem.u16(s[10]), 0x70);
        assert_eq!(e.mem.u16(s[11] + 2 * 0x45), 0x45);
        let geometry = s[0];
        assert_eq!(e.mem.u32(this + 0x24), geometry);
        let node = e.mem.u32(this + 0x2c);
        assert_eq!(
            calls_to(&e, SLOT_TARGET + 0xdc),
            vec![vec![node, geometry, 1]]
        );
        assert_eq!(
            calls_to(&e, NODE_SET_NAME),
            vec![
                vec![calls_to(&e, ALPHA_PROPERTY_CONSTRUCT)[0][0], ALPHA_NAME],
                vec![node, SPHERE_NAME]
            ]
        );
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_SET_FLAG),
            vec![vec![geometry + 3, 0x20, 0]]
        );
        assert_eq!(calls_to(&e, NODE_SET_00A59C60).len(), 1);
        // Built already: the radius and colour are set again.
        e.call_log = Some(vec![]);
        e.call(0x004a_72d0, &[this]);
        assert!(calls_to(&e, TRI_STRIPS_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, SLOT_TARGET + 8), vec![vec![this, this + 8]]);
        assert_eq!(calls_to(&e, POINT3_SCALE_IN_PLACE).len(), 0x70);
    }

    #[test]
    fn primitive_factories() {
        let mut e = engine();
        e.mem.map(FLOAT_TWO, 4);
        e.mem.set_u32(FLOAT_TWO, f(2.0));
        e.mem.map(FLOAT_HALF, 4);
        e.mem.set_u32(FLOAT_HALF, f(0.5));
        e.mem.map(DOUBLE_HALF, 8);
        e.mem.set_u64(DOUBLE_HALF, 0.5f64.to_bits());
        e.register(POINT3_SCALED, |e, a| {
            let factor = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_u32(a[1] + 4 * i, v.to_bits());
            }
            ret(a[1])
        });
        e.register(GET_SIZE_413FC0, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[0] + 0x18 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, v);
            }
            ret(a[1])
        });
        e.register(GET_COLOR_413F90, |_, _| Ret::default());
        e.register(GET_TYPE, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(GET_RADIUS_4A7BD0, |e, a| {
            st0(f64::from(e.mem.f32(a[0] + 0x18)))
        });
        let source = e.mem.alloc(0x34);
        e.mem.set_u32(source + 0x18, f(1.0));
        e.mem.set_u32(source + 0x1c, f(2.0));
        e.mem.set_u32(source + 0x20, f(3.0));
        // A plane: x and z doubled, then halved by the plane maker.
        e.mem.set_u32(source + 4, 3);
        let plane = e.call(0x004a_4d40, &[source]).u32();
        assert_eq!(e.mem.u32(plane), BGS_PRIMITIVE_PLANE_VTABLE);
        assert_eq!(e.mem.f32(plane + 0x18), 1.0);
        assert_eq!(e.mem.f32(plane + 0x20), 3.0);
        // A box: the size doubled, halved through the box's slot 0xC.
        e.mem.set_u32(source + 4, 1);
        let shape = e.call(0x004a_4d40, &[source]).u32();
        assert_eq!(e.mem.u32(shape), BGS_PRIMITIVE_BOX_VTABLE);
        // A sphere: the radius.
        e.mem.set_u32(source + 4, 2);
        e.mem.set_u32(source + 0x18, f(-4.0));
        let sphere = e.call(0x004a_4d40, &[source]).u32();
        assert_eq!(e.mem.u32(sphere), SPHERE_VTABLE);
        assert_eq!(e.mem.u32(sphere + 4), 2);
        assert_eq!(e.mem.f32(sphere + 0x18), 4.0);
        e.mem.set_u32(source + 4, 9);
        assert_eq!(e.call(0x004a_4d40, &[source]).u32(), 0);
        // The sphere's own multi-bound shape: a zero centre and its radius.
        let made = e.call(0x004a_7aa0, &[sphere, 0x77]).u32();
        assert_eq!(e.mem.u32(made), MULTI_BOUND_SPHERE_VTABLE);
        assert_eq!(e.mem.f32(made + 0x18), 4.0);
        assert_eq!(e.mem.u32(made + 0xc), 0);
    }
}
