//! `BSMain/bsmultiboundcapsule.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00526ac0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0x24]`, the value in `ST0`. The
/// map names it `BSMultiBoundCapsule::QMultiBoundRadius`; the script
/// interpreter calls the same body for a `TESGlobal`'s value (operand `G`,
/// `005ac7a0`). The `FLD` quiets a signaling NaN.
pub fn fn_00526ac0(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0x24) as f64
}

/// `00c4fa10` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C4FA10: u32 = 0x00c4_fa10;

// Translated from 00c60ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSMultiBoundCapsule` vtable word 010c34fc, right after
/// `BSMultiBoundCapsule::LoadBinary` (010c34f8). Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c60ec0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

// Translated from 00c61270 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204398 without reading the receiver. The map names this body `BSMultiBoundCapsule::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c61270(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4398
}

// ---- Batch n067 ---------------------------------------------------------------------------
//
// A `BSMultiBoundCapsule` is a `BSMultiBoundShape` (0xC bytes) followed by `PointA` (Xbox
// PDB, +0xC), `PointB` (+0x18) and `Radius` (+0x24): 0x28 bytes, vtable `010c34ac`. Floats
// the exe copies with `FLD`/`FSTP` are quieted; comparisons follow its `FUCOMPP` tests (a
// NaN is never equal).

/// `PointA`, `PointB` and `Radius` (Xbox PDB, PC offsets), the object's size and vtable.
const POINT_A: u32 = 0x0c;
const POINT_B: u32 = 0x18;
const RADIUS: u32 = 0x24;
const CAPSULE_SIZE: u32 = 0x28;
const CAPSULE_VTABLE: u32 = 0x010c_34ac;
/// `BSMultiBoundShape`'s constructor, `CopyMembers`, `LoadBinary`, `RegisterStreamables`,
/// save body, `IsEqual` and viewer strings (`bsmultiboundshape.obj`).
const SHAPE_CONSTRUCT: u32 = 0x00c4_f980;
const SHAPE_COPY_MEMBERS: u32 = 0x00c4_f9b0;
const SHAPE_LOAD_BINARY: u32 = 0x00c4_f9f0;
const SHAPE_REGISTER_STREAMABLES: u32 = 0x00c4_fa30;
const SHAPE_SAVE_BINARY: u32 = 0x00c4_fa60;
const SHAPE_IS_EQUAL: u32 = 0x00c4_fa80;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c4_fab0;
/// The `NiObject` destructor body (`00a5d3d0`) and the sized delete (`00aa1460`, cdecl:
/// the block and the size).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// A point's `LoadBinary` and `SaveBinary` (`this` the point, the stream).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
/// Reads and writes a float through the stream (cdecl: the stream, the float's address).
const STREAM_LOAD_FLOAT: u32 = 0x00a2_6df0;
const STREAM_SAVE_FLOAT: u32 = 0x00a3_da50;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiGetViewerString` (cdecl, the name) and the word at `01204398` it is given (the first
/// word of the class's `NiRTTI` record), a point's `GetViewerString` (`this` the point,
/// the label) and `NiGetViewerString_ov9` (cdecl: a label and a float).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const TYPE_NAME_WORD: u32 = 0x0120_4398;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const FLOAT_VIEWER_STRING: u32 = 0x00a7_d550;
/// Adds a string to the viewer strings (`this` the strings, the address of a word holding
/// the string).
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// The labels `"PointA"` (`010c3570`), `"PointB"` (`010c3568`) and `"Radius"` (`01015a9c`).
const POINT_A_LABEL: u32 = 0x010c_3570;
const POINT_B_LABEL: u32 = 0x010c_3568;
const RADIUS_LABEL: u32 = 0x0101_5a9c;

/// Copies `count` words from `from` to `to` (`MOV` copies: no quieting).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Adds `string` to `strings` through a local word holding it, as the exe does.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

/// Whether the points at `a` and `b` are equal component by component (`FUCOMPP` with
/// `a`'s float loaded first; stops at the first difference).
fn points_equal(e: &Engine, a: u32, b: u32) -> bool {
    (0..3).all(|axis| x87_load32(e.mem.u32(a + 4 * axis)) == x87_load32(e.mem.u32(b + 4 * axis)))
}

// Translated from 00c60d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::BSMultiBoundCapsule` (Xbox PDB): the `BSMultiBoundShape`
/// constructor, then the class's vtable (`010c34ac`). Returns `this`.
pub fn bs_multi_bound_capsule_bs_multi_bound_capsule(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SHAPE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), CAPSULE_VTABLE);
    this
}

/// `operator new(0x28)` and the constructor when the block is not null.
fn new_capsule(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![CAPSULE_SIZE]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        bs_multi_bound_capsule_bs_multi_bound_capsule(e, Ptr::new(block))
    }
}

// Translated from 00c60d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c60d70 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSMultiBoundCapsule::CreateClone` (Xbox PDB): a new capsule (`operator new(0x28)`,
/// constructed unless the allocation failed), then this capsule's members copied into it
/// (`CopyMembers` runs on a null clone too). Returns the clone.
pub fn bs_multi_bound_capsule_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_capsule(e);
    bs_multi_bound_capsule_copy_members(e, this, clone, cloning);
    clone
}

// Translated from 00c60e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::CopyMembers` (Xbox PDB): the shape's part, the two points word by
/// word, then the radius (copied with `FLD`/`FSTP` twice).
pub fn bs_multi_bound_capsule_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, cloning]);
    copy_words(e, this.addr() + POINT_A, clone.addr() + POINT_A, 3);
    copy_words(e, this.addr() + POINT_B, clone.addr() + POINT_B, 3);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    e.mem.set_u32(clone.addr() + RADIUS, radius);
}

// Translated from 00c60e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::LoadBinary` (Xbox PDB): the shape's part, the two points (a
/// point's `LoadBinary`), then the radius (`00a26df0`).
pub fn bs_multi_bound_capsule_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    e.call(POINT_LOAD_BINARY, &args![this.addr() + POINT_A, stream]);
    e.call(POINT_LOAD_BINARY, &args![this.addr() + POINT_B, stream]);
    e.call(STREAM_LOAD_FLOAT, &args![stream, this.addr() + RADIUS]);
}

// Translated from 00c60ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSMultiBoundCapsule`'s `RegisterStreamables` slot (vtable
/// word 010c3500, after the `LinkObject` word 010c34fc): `BSMultiBoundShape::RegisterStreamables`
/// (`00c4fa30`), its byte normalised to true or false.
pub fn fn_00c60ee0(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(SHAPE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00c60f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::SaveBinary` (Xbox PDB): the shape's save body, the two points (a
/// point's `SaveBinary`), then the radius (`NiStreamSaveBinary<float>`).
pub fn bs_multi_bound_capsule_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    e.call(POINT_SAVE_BINARY, &args![this.addr() + POINT_A, stream]);
    e.call(POINT_SAVE_BINARY, &args![this.addr() + POINT_B, stream]);
    e.call(STREAM_SAVE_FLOAT, &args![stream, this.addr() + RADIUS]);
}

// Translated from 00c60f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::IsEqual` (Xbox PDB): the shape's `IsEqual`, and when it holds both
/// points equal component by component and the radii equal (this capsule's radius copied
/// first); all three tests run.
pub fn bs_multi_bound_capsule_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(SHAPE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let first = points_equal(e, this.addr() + POINT_A, other.addr() + POINT_A);
    let second = points_equal(e, this.addr() + POINT_B, other.addr() + POINT_B);
    let radius = x87_load32(x87_quiet_bits(e.mem.u32(this.addr() + RADIUS)))
        == x87_load32(e.mem.u32(other.addr() + RADIUS));
    first & second & radius
}

// Translated from 00c610a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c610a0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// The class's stream creator (no arguments; unnamed in the map): a new capsule from
/// `operator new(0x28)`, constructed; null when the allocation fails.
pub fn fn_00c610a0(e: &mut Engine) -> Ptr {
    new_capsule(e)
}

// Translated from 00c61120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::GetViewerStrings` (Xbox PDB): the shape's strings, the string
/// `NiGetViewerString` makes of the word at `01204398`, the points' (`"PointA"`,
/// `"PointB"`) and the radius's (`00a7d550` with `"Radius"` and the radius, copied with
/// `FLD`/`FSTP`), each added through `0096a610`.
pub fn bs_multi_bound_capsule_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let type_name = e.mem.u32(TYPE_NAME_WORD);
    let string = e.call(GET_VIEWER_STRING, &args![type_name]).u32();
    add_viewer_string(e, strings, string);
    let string = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + POINT_A, POINT_A_LABEL],
        )
        .u32();
    add_viewer_string(e, strings, string);
    let string = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + POINT_B, POINT_B_LABEL],
        )
        .u32();
    add_viewer_string(e, strings, string);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    let string = e
        .call(FLOAT_VIEWER_STRING, &args![RADIUS_LABEL, radius])
        .u32();
    add_viewer_string(e, strings, string);
}

// Translated from 00c611d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::QType` (Xbox PDB): returns 4.
pub fn bs_multi_bound_capsule_q_type(_e: &mut Engine, _this: Ptr) -> u32 {
    4
}

// Translated from 00c611e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::CheckBound` for a bound sphere (Xbox PDB): returns 0 without
/// reading it.
pub fn bs_multi_bound_capsule_check_bound(_e: &mut Engine, _this: Ptr, _sphere: Ptr) -> u32 {
    0
}

// Translated from 00c611f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::CheckBound` for another bound (Xbox PDB `CheckBound_ov2`): asks the
/// other's shape (its word at +0xC) for its type (slot 0x8c), then returns 0 whatever it
/// is.
pub fn bs_multi_bound_capsule_check_bound_ov2(e: &mut Engine, _this: Ptr, other: Ptr) -> u32 {
    let shape = e.mem.u32(other.addr() + 0xc);
    e.vcall(shape, 0x8c, &[]);
    0
}

// Translated from 00c61230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::WithinFrustum` (Xbox PDB): false without reading the frustum.
pub fn bs_multi_bound_capsule_within_frustum(_e: &mut Engine, _this: Ptr, _frustum: Ptr) -> bool {
    false
}

// Translated from 00c61240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::CompletelyWithinFrustum` (Xbox PDB): false without reading the
/// frustum.
pub fn bs_multi_bound_capsule_completely_within_frustum(
    _e: &mut Engine,
    _this: Ptr,
    _frustum: Ptr,
) -> bool {
    false
}

// Translated from 00c61250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::GetNiBound` (Xbox PDB): does nothing (`RET 4`; the bound is not
/// written).
pub fn bs_multi_bound_capsule_get_ni_bound(_e: &mut Engine, _this: Ptr, _out: Ptr) {}

// Translated from 00c61260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::MakeDebugGeom` (Xbox PDB): does nothing (`RET 0x18`: the two
/// geometries and the four colour words).
#[allow(clippy::too_many_arguments)]
pub fn bs_multi_bound_capsule_make_debug_geom(
    _e: &mut Engine,
    _this: Ptr,
    _lines: Ptr,
    _triangles: Ptr,
    _color_0: u32,
    _color_1: u32,
    _color_2: u32,
    _color_3: u32,
) {
}

// Translated from 00c61280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundCapsule::_scalar_deleting_destructor_` (Xbox PDB): the `NiObject`
/// destructor body (`00a5d3d0`), then the sized delete of 0x28 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_multi_bound_capsule_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, CAPSULE_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_0d50, bs_multi_bound_capsule_bs_multi_bound_capsule(Ptr) -> Ptr),
        entry!(0x00c6_0d70, bs_multi_bound_capsule_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c6_0e00,
            bs_multi_bound_capsule_copy_members(Ptr, Ptr, Ptr)
        ),
        entry!(0x00c6_0e70, bs_multi_bound_capsule_load_binary(Ptr, Ptr)),
        entry!(0x00c6_0ee0, fn_00c60ee0(Ptr, Ptr) -> bool),
        entry!(0x00c6_0f10, bs_multi_bound_capsule_save_binary(Ptr, Ptr)),
        entry!(0x00c6_0f60, bs_multi_bound_capsule_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c6_10a0, fn_00c610a0() -> Ptr),
        entry!(
            0x00c6_1120,
            bs_multi_bound_capsule_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c6_11d0, bs_multi_bound_capsule_q_type(Ptr) -> u32),
        entry!(0x00c6_11e0, bs_multi_bound_capsule_check_bound(Ptr, Ptr) -> u32),
        entry!(0x00c6_11f0, bs_multi_bound_capsule_check_bound_ov2(Ptr, Ptr) -> u32),
        entry!(0x00c6_1230, bs_multi_bound_capsule_within_frustum(Ptr, Ptr) -> bool),
        entry!(
            0x00c6_1240,
            bs_multi_bound_capsule_completely_within_frustum(Ptr, Ptr) -> bool
        ),
        entry!(0x00c6_1250, bs_multi_bound_capsule_get_ni_bound(Ptr, Ptr)),
        entry!(
            0x00c6_1260,
            bs_multi_bound_capsule_make_debug_geom(Ptr, Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00c6_1280,
            bs_multi_bound_capsule_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c6_1270, fn_00c61270(Ptr) -> u32),
        entry!(0x00526ac0, fn_00526ac0(Ptr) -> f64),
        entry!(0x00c6_0ec0, fn_00c60ec0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0x24_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x28);
        e.mem.set_f32(object + 0x20, 9.0);
        for value in [1.5f32, -1.0, 0.0, f32::NEG_INFINITY] {
            e.mem.set_f32(object + 0x24, value);
            assert_eq!(e.call(0x00526ac0, &[object]).f64(), value as f64);
        }
        // A signaling single NaN comes back quiet, its payload widened.
        e.mem.set_u32(object + 0x24, 0x7f80_0001);
        assert_eq!(
            e.call(0x00526ac0, &[object]).f64().to_bits(),
            0x7ff8_0000_2000_0000
        );
    }

    #[test]
    fn forwarder_fn_00c60ec0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_0ec0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c61270_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_1270, &args![0x1000u32]).u32(), 0x0120_4398);
        assert_eq!(e.call(0x00c6_1270, &args![0u32]).u32(), 0x0120_4398);
    }
}

#[cfg(test)]
mod n067_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn capsule(e: &mut Engine, a: [f32; 3], b: [f32; 3], radius: f32) -> u32 {
        let this = e.mem.alloc(CAPSULE_SIZE);
        for i in 0..3u32 {
            e.mem.set_f32(this + POINT_A + 4 * i, a[i as usize]);
            e.mem.set_f32(this + POINT_B + 4 * i, b[i as usize]);
        }
        e.mem.set_f32(this + RADIUS, radius);
        this
    }

    #[test]
    fn constructor_clone_factory_and_destructor() {
        let mut e = Engine::new();
        e.register(SHAPE_CONSTRUCT, nothing);
        e.register(SHAPE_COPY_MEMBERS, nothing);
        e.register(NI_OBJECT_DESTRUCT, nothing);
        e.register(SIZED_DELETE, nothing);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let this = capsule(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 7.0);
        e.mem.set_u32(this + RADIUS, 0xff80_0001);
        let clone = e.call(0x00c6_0d70, &args![this, 0x66u32]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x28]]);
        assert_eq!(calls(&e, SHAPE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(e.mem.u32(clone), CAPSULE_VTABLE);
        assert_eq!(calls(&e, SHAPE_COPY_MEMBERS), vec![vec![this, clone, 0x66]]);
        assert_eq!(e.mem.f32(clone + POINT_A + 8), 3.0);
        assert_eq!(e.mem.f32(clone + POINT_B), 4.0);
        assert_eq!(e.mem.u32(clone + RADIUS), 0xffc0_0001);
        let made = e.call(0x00c6_10a0, &args![]).u32();
        assert_eq!(e.mem.u32(made), CAPSULE_VTABLE);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c6_10a0, &args![]).u32(), 0);
        assert_eq!(e.call(0x00c6_1280, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00c6_1280, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x28]]);
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCT).len(), 2);
    }

    #[test]
    fn streaming_and_register_streamables() {
        let mut e = Engine::new();
        for address in [
            SHAPE_LOAD_BINARY,
            POINT_LOAD_BINARY,
            STREAM_LOAD_FLOAT,
            SHAPE_SAVE_BINARY,
            POINT_SAVE_BINARY,
            STREAM_SAVE_FLOAT,
        ] {
            e.register(address, nothing);
        }
        e.register(SHAPE_REGISTER_STREAMABLES, |_, a| (0x100 | a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c6_0e70, &args![0x100u32, 0x200u32]);
        e.call(0x00c6_0f10, &args![0x100u32, 0x200u32]);
        assert_eq!(
            calls(&e, POINT_LOAD_BINARY),
            vec![vec![0x10c, 0x200], vec![0x118, 0x200]]
        );
        assert_eq!(calls(&e, STREAM_LOAD_FLOAT), vec![vec![0x200, 0x124]]);
        assert_eq!(calls(&e, SHAPE_LOAD_BINARY), vec![vec![0x100, 0x200]]);
        assert_eq!(
            calls(&e, POINT_SAVE_BINARY),
            vec![vec![0x10c, 0x200], vec![0x118, 0x200]]
        );
        assert_eq!(calls(&e, STREAM_SAVE_FLOAT), vec![vec![0x200, 0x124]]);
        assert_eq!(calls(&e, SHAPE_SAVE_BINARY), vec![vec![0x100, 0x200]]);
        assert!(e.call(0x00c6_0ee0, &args![0x100u32, 1u32]).bool());
        assert!(!e.call(0x00c6_0ee0, &args![0x100u32, 0u32]).bool());
    }

    #[test]
    fn is_equal_compares_points_and_radius() {
        let mut e = Engine::new();
        e.register(SHAPE_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = capsule(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 7.0);
        let b = capsule(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 7.0);
        assert!(e.call(0x00c6_0f60, &args![a, b]).bool());
        e.mem.set_f32(b + POINT_B + 4, 5.5);
        assert!(!e.call(0x00c6_0f60, &args![a, b]).bool());
        e.mem.set_f32(b + POINT_B + 4, 5.0);
        e.mem.set_f32(b + POINT_A, f32::NAN);
        assert!(!e.call(0x00c6_0f60, &args![a, b]).bool());
        e.mem.set_f32(b + POINT_A, 1.0);
        e.mem.set_f32(b + RADIUS, 7.5);
        assert!(!e.call(0x00c6_0f60, &args![a, b]).bool());
        e.register(SHAPE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c6_0f60, &args![a, a]).bool());
    }

    #[test]
    fn viewer_strings_and_the_empty_slots() {
        let mut e = Engine::new();
        e.register(SHAPE_GET_VIEWER_STRINGS, nothing);
        e.register(GET_VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(FLOAT_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(ADD_VIEWER_STRING, nothing);
        e.map(TYPE_NAME_WORD, 4);
        e.set_global(TYPE_NAME_WORD, 0x4321u32);
        let this = capsule(&mut e, [0.0; 3], [0.0; 3], 1.5);
        e.call_log = Some(vec![]);
        e.call(0x00c6_1120, &args![this, 0x900u32]);
        assert_eq!(calls(&e, GET_VIEWER_STRING), vec![vec![0x4321]]);
        assert_eq!(
            calls(&e, POINT_GET_VIEWER_STRING),
            vec![
                vec![this + POINT_A, POINT_A_LABEL],
                vec![this + POINT_B, POINT_B_LABEL]
            ]
        );
        assert_eq!(
            calls(&e, FLOAT_VIEWER_STRING),
            vec![vec![RADIUS_LABEL, 1.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, ADD_VIEWER_STRING).len(), 4);
        assert_eq!(e.call(0x00c6_11d0, &args![this]).u32(), 4);
        assert_eq!(e.call(0x00c6_11e0, &args![this, 0x10u32]).u32(), 0);
        assert!(!e.call(0x00c6_1230, &args![this, 0x10u32]).bool());
        assert!(!e.call(0x00c6_1240, &args![this, 0x10u32]).bool());
        e.call(0x00c6_1250, &args![this, 0x10u32]);
        e.call(
            0x00c6_1260,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        // The other bound's type is asked for, then ignored.
        let vtable = e.mem.alloc(0x90);
        e.mem.set_u32(vtable + 0x8c, 0x0090_4000);
        e.register(0x0090_4000, |_, _| 1u32.into_ret());
        let shape = e.mem.alloc(4);
        e.mem.set_u32(shape, vtable);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 0xc, shape);
        assert_eq!(e.call(0x00c6_11f0, &args![this, other]).u32(), 0);
        assert_eq!(calls(&e, 0x0090_4000), vec![vec![shape]]);
    }
}
