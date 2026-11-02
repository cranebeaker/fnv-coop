//! `BSHavok/bhktriangleshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkTriangleShape` (vtable `010ca3f4`, RTTI-confirmed) is a `bhkConvexShape` (0x14 bytes):
//! its Havok shape (`hkpTriangleShape`, vtable `010c749c`, RTTI-confirmed) is at `+8`, its
//! construction info at `+0xC`. The Havok shape keeps the radius at `+0x10` and its three
//! vertices (4 floats each) at `+0x20`, `+0x30` and `+0x40`; the construction info (0x40
//! bytes) has the radius at `+4` and the vertices at `+0x10`, `+0x20` and `+0x30`.
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames
//! (`FS:[0]`) of the functions marked `Translation incomplete` are not modeled. The 16-byte
//! aligned vector moves (`MOVAPS`) are done as plain byte copies; their alignment faults
//! are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope,
};

/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238`: allocate `(size)` (`00aa3e40`) and free
/// `(pointer)` (`00aa4060`).
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
const POOL_FREE: u32 = 0x00aa_4060;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConvexShape` methods: destructor body, `CopyCinfo`, `LoadBinary`, `SaveBinary`,
/// `GetViewerStrings`.
const CONVEX_SHAPE_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_SHAPE_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_SHAPE_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_SHAPE_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape::CopyMembers` `(source, process)`.
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// The vector copy (`004b4db0`, `ECX` = destination, one stack word: the source).
const COPY_VECTOR: u32 = 0x004b_4db0;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` (`00a7d330`, cdecl).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// The vtables of the classes the constructor passes through (RTTI-confirmed) and their
/// live-object counts.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const VTABLE_TRIANGLE_SHAPE: u32 = 0x010c_a3f4;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
const COUNT_TRIANGLE_SHAPE: u32 = 0x0126_8660;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_8664;
/// The `hkpTriangleShape` vtable.
const VTABLE_HK_TRIANGLE_SHAPE: u32 = 0x010c_749c;
/// The float the construction info starts with (`+4`), and the one `Init` first puts in the
/// Havok shape's radius (`+0x10`) before the construction info's radius replaces it.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const HK_DEFAULT_RADIUS: u32 = 0x011b_153c;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// Copies `len` bytes.
fn copy_bytes(e: &mut Engine, to: u32, from: u32, len: u32) {
    let data = e.mem.bytes(from, len);
    e.mem.write(to, &data);
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// `FLD`/`FMUL`/`FSTP` of two floats through the x87 stack: both operands are quieted when
/// loaded, a NaN operand wins (the larger significand, then the positive
/// one, when both are NaN), otherwise the product rounded to a float.
fn x87_mul_bits(first: u32, second: u32) -> u32 {
    // Both operands loaded (quiet), the x87 NaN choice (#199); the product of two floats is
    // exact in f64, so it is rounded once.
    x87_store32(x87_mul(x87_load32(first), x87_load32(second)))
}

/// Byte-swaps the twelve words at `cinfo + 0x10` (the three vertices) in place.
fn swap_vertices(e: &mut Engine, cinfo: u32) {
    for offset in (0x10..0x40).step_by(4) {
        let word = e.mem.u32(cinfo + offset);
        e.mem.set_u32(cinfo + offset, word.swap_bytes());
    }
}

// Translated from 00cca8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cca8d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape::~bhkTriangleShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `01268660` and runs the
/// `bhkConvexShape` destructor body (`00c9fb00`).
pub fn bhk_triangle_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TRIANGLE_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_TRIANGLE_SHAPE);
    e.set_global(COUNT_TRIANGLE_SHAPE, count.wrapping_sub(1));
    e.call(CONVEX_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00cca950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape` constructor (unnamed in the engine map), `ECX` = `this`: the base
/// constructor (`00c87c30`), `+0xC` and `+0x10` cleared, then the vtable of each class on
/// the way down (`bhkShape`, `bhkSphereRepShape`, `bhkConvexShape`, `bhkTriangleShape`),
/// each with its live-object count raised. Answers `this`.
pub fn fn_00cca950(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in [
        (VTABLE_SHAPE, COUNT_SHAPE),
        (VTABLE_SPHERE_REP_SHAPE, COUNT_SPHERE_REP_SHAPE),
        (VTABLE_CONVEX_SHAPE, COUNT_CONVEX_SHAPE),
        (VTABLE_TRIANGLE_SHAPE, COUNT_TRIANGLE_SHAPE),
    ] {
        e.mem.set_u32(this.addr(), vtable);
        increment_count(e, counter);
    }
    this
}

// Translated from 00cca9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the construction info (unnamed in the engine map), `ECX` = the construction info,
/// one stack float `scale`: multiplies the twelve floats at `+0x10` to `+0x3F` (the three
/// vertices) by it through the x87 stack, each rounded to a float on its store.
pub fn fn_00cca9c0(e: &mut Engine, this: Ptr, scale: f32) {
    let scale = x87_quiet_bits(scale.to_bits());
    for offset in (0x10..0x40).step_by(4) {
        let lane = e.mem.u32(this.addr() + offset);
        e.mem
            .set_u32(this.addr() + offset, x87_mul_bits(lane, scale));
    }
}

// Translated from 00ccaa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::CopyCinfo` (Xbox PDB): `bhkConvexShape::CopyCinfo`, then, when there is
/// a Havok shape, its three vertices (`+0x20`, `+0x30`, `+0x40`) into the cinfo's `+0x10`,
/// `+0x20` and `+0x30` with the vector copy (`004b4db0`).
pub fn bhk_triangle_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(CONVEX_SHAPE_COPY_CINFO, &args![this, cinfo]);
    if this.is_null() {
        return;
    }
    let shape = e.mem.u32(this.addr() + 8);
    if shape == 0 {
        return;
    }
    let vertices = shape.wrapping_add(0x20);
    if vertices == 0 {
        return;
    }
    for index in 0..3 {
        e.call(
            COPY_VECTOR,
            &args![cinfo + 0x10 + 0x10 * index, vertices + 0x10 * index],
        );
    }
}

// Translated from 00ccaa80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccaa80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape` allocate-and-construct (unnamed in the engine map): allocates a
/// 0x14-byte block (`operator new`) and constructs a `bhkTriangleShape` in it (`00cca950`).
/// Answers the object, or 0 when the allocation fails.
pub fn fn_00ccaa80(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00cca950(e, Ptr::new(block))
}

// Translated from 00ccaaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::CopyMembers` (Xbox PDB), `(this, source, process)`: takes the cinfo
/// (virtual slot `0xB4`); when the float at `process + 0x10` (through the x87 stack) is not
/// equal to 1.0 (a NaN is not) scales the cinfo with it (`00cca9c0`). Then
/// `bhkShape::CopyMembers`.
pub fn bhk_triangle_shape_copy_members(e: &mut Engine, this: Ptr, source: u32, process: u32) {
    let cinfo = cinfo_of(e, this);
    let scale = x87_quiet(f32::from_bits(e.mem.u32(process + 0x10)));
    if scale != 1.0 {
        fn_00cca9c0(e, Ptr::new(cinfo), scale);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00ccab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::LoadBinary` (Xbox PDB): `bhkConvexShape::LoadBinary`, then, when the
/// stream's byte at `+0x1E5` is not 1, takes the cinfo and byte-swaps its twelve vertex
/// words (`+0x10` to `+0x3F`).
pub fn bhk_triangle_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CONVEX_SHAPE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let cinfo = cinfo_of(e, this);
        swap_vertices(e, cinfo);
    }
}

// Translated from 00ccabf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::SaveBinary` (Xbox PDB): takes the cinfo, byte-swaps its twelve vertex
/// words when the stream's byte at `+0x1E4` is not 1, runs `bhkConvexShape::SaveBinary` and
/// swaps them back under the same condition.
pub fn bhk_triangle_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_vertices(e, cinfo);
    }
    e.call(CONVEX_SHAPE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_vertices(e, cinfo);
    }
}

// Translated from 00ccacf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccacf0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape::CreateClone` (Xbox PDB): in the Havok allocation scope and between the
/// clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x14-byte shape (`operator
/// new` and the constructor `00cca950`; null stays null) and calls this shape's virtual
/// slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_triangle_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00ccaa80(e);
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    clone
}

// Translated from 00ccadb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccadb0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape::CreateCinfo` (Xbox PDB), `(this, created)`: in the Havok allocation
/// scope stores into the byte at `created` whether the shape had no cinfo (`+0xC` null). In
/// that case it takes 0x40 bytes from the pool at `011f6238` and, when that succeeds, writes
/// the cinfo (`+0` zero, `+4` the float at `010c72bc` through the x87 stack, the twelve
/// vertex words at `+0x10` to `+0x3F` zero; `+8` and `+0xC` are left as the pool gave
/// them), stores it (or 0) at `+0xC` and, when the shape has a Havok shape, fills it with
/// `CopyCinfo` (`00ccaa30`). Answers the cinfo at `+0xC`.
pub fn bhk_triangle_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let cinfo = e.call(POOL_ALLOCATE, &args![POOL, 0x40u32]).u32();
        if cinfo != 0 {
            e.mem.set_u32(cinfo, 0);
            let radius = x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS));
            e.mem.set_u32(cinfo + 4, radius);
            for offset in (0x10..0x40).step_by(4) {
                e.mem.set_u32(cinfo + offset, 0);
            }
        }
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            bhk_triangle_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00ccae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the cinfo (unnamed in the engine map), `(this, flag)` with only the low byte of
/// `flag` used: in the Havok allocation scope, when that byte is not 0 gives the cinfo at
/// `+0xC` back to the pool at `011f6238` (`00aa4060`) and clears `+0xC`.
pub fn fn_00ccae90(e: &mut Engine, this: Ptr, flag: u32) {
    let old = enter_scope(e);
    if flag as u8 != 0 {
        let cinfo = e.mem.u32(this.addr() + 0xc);
        e.call(POOL_FREE, &args![POOL, cinfo]);
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    leave_scope(e, old);
}

// Translated from 00ccaee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccaee0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape::Init` (Xbox PDB): in the Havok allocation scope allocates the 0x60-byte
/// `hkpTriangleShape` (vtable `010c749c`) and builds it in place from the cinfo: size word
/// at `+4`, the float at `011b153c` (through the x87 stack) at `+0x10` and then the cinfo's
/// radius (`+4`, likewise) over it, the three vertices from the cinfo's `+0x10` to `+0x3F`
/// at `+0x20`, the words `+0x14` and `+0x16`/`+0x17` cleared and set to 6, `+6` to 1, `+8` to
/// 0, `+0xC` to 3, and 16 zero bytes at `+0x50`. Hands it to virtual slot `0x8C`, releases it
/// (`00c905b0`) and calls virtual slot `0xBC` with the cinfo.
pub fn bhk_triangle_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let shape = allocate_havok(e, 0x60);
    let default_radius = x87_quiet_bits(e.mem.u32(HK_DEFAULT_RADIUS));
    e.mem.set_u32(shape + 0x10, default_radius);
    copy_bytes(e, shape + 0x20, cinfo + 0x10, 0x30);
    e.mem.set_u16(shape + 0x14, 0);
    e.mem.set_u8(shape + 0x17, 0);
    e.mem.set_u16(shape + 6, 1);
    e.mem.set_u32(shape + 8, 0);
    e.mem.set_u32(shape + 0xc, 3);
    e.mem.set_u32(shape, VTABLE_HK_TRIANGLE_SHAPE);
    e.mem.set_u8(shape + 0x16, 6);
    e.mem.write(shape + 0x50, &[0; 16]);
    let radius = x87_quiet_bits(e.mem.u32(cinfo + 4));
    e.mem.set_u32(shape + 0x10, radius);
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, old);
}

// Translated from 00ccb080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccb080 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkTriangleShape::GetViewerStrings` (Xbox PDB): in the Havok allocation scope runs
/// `bhkConvexShape::GetViewerStrings` and appends the viewer string of the text in the
/// global at `01268664`.
pub fn bhk_triangle_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(CONVEX_SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

// Translated from 00cca9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cca8d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_triangle_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_a8d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00cca940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTriangleShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8664.
pub fn bhk_triangle_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8664
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cca8d0, bhk_triangle_shape_destructor(Ptr)),
        entry!(0x00cca940, bhk_triangle_shape_get_r_t_t_i() -> u32),
        entry!(0x00cca950, fn_00cca950(Ptr) -> Ptr),
        entry!(0x00cca9a0, bhk_triangle_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cca9c0, fn_00cca9c0(Ptr, f32)),
        entry!(0x00ccaa30, bhk_triangle_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00ccaa80, fn_00ccaa80() -> Ptr),
        entry!(0x00ccaaf0, bhk_triangle_shape_copy_members(Ptr, u32, u32)),
        entry!(0x00ccab60, bhk_triangle_shape_load_binary(Ptr, u32)),
        entry!(0x00ccabf0, bhk_triangle_shape_save_binary(Ptr, u32)),
        entry!(0x00ccacf0, bhk_triangle_shape_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00ccadb0, bhk_triangle_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00ccae90, fn_00ccae90(Ptr, u32)),
        entry!(0x00ccaee0, bhk_triangle_shape_init(Ptr, u32)),
        entry!(0x00ccb080, bhk_triangle_shape_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_triangle_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_a8d0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cca9a0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cca9a0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_a8d0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_triangle_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_a940, &args![]).u32(), 0x0126_8664);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the allocation scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// A shape (slot `0xB4` answers its `+0xC`, slot `0xC4` records) with a Havok shape
    /// block. Returns `(this, havok shape)`.
    fn shape(e: &mut Engine) -> (u32, u32) {
        let this = object(e, &[(0xb4, 0x0900_0100), (0xc4, 0x0900_0101)]);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0101]);
        let havok = e.mem.alloc(0x80);
        e.mem.set_u32(this + 8, havok);
        (this, havok)
    }

    fn cinfo_with_vertices(e: &mut Engine, this: u32) -> u32 {
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, cinfo);
        for i in 0..12 {
            e.mem.set_u32(cinfo + 0x10 + 4 * i, 0x0102_0300 + i);
        }
        cinfo
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_TRIANGLE_SHAPE, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00cc_a8d0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_TRIANGLE_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_TRIANGLE_SHAPE), 8);
        assert_eq!(
            order(&e),
            vec![0x00cc_a8d0, SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn constructor_counts_every_class_on_the_way_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x10, 6);
        assert_eq!(e.call(0x00cc_a950, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_TRIANGLE_SHAPE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        for counter in [
            COUNT_SHAPE,
            COUNT_SPHERE_REP_SHAPE,
            COUNT_CONVEX_SHAPE,
            COUNT_TRIANGLE_SHAPE,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    #[test]
    fn scale_multiplies_the_twelve_vertex_floats() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x60);
        for i in 0..12u32 {
            e.mem.set_f32(cinfo + 0x10 + 4 * i, (i + 1) as f32);
        }
        e.mem.set_f32(cinfo + 0x40, 5.0);
        e.call(0x00cc_a9c0, &args![cinfo, 0.5f32.to_bits()]);
        for i in 0..12u32 {
            assert_eq!(e.mem.f32(cinfo + 0x10 + 4 * i), (i + 1) as f32 * 0.5);
        }
        // Beyond the vertices nothing changes.
        assert_eq!(e.mem.f32(cinfo + 0x40), 5.0);
        // A signalling NaN scale is quieted and wins every lane against a number.
        e.mem.set_u32(cinfo + 0x10, 0x7f80_0002);
        e.call(0x00cc_a9c0, &args![cinfo, 0x7f80_0001u32]);
        // The x87 rule: of two NaNs the one with the larger significand wins.
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x7fc0_0002);
        for i in 1..12u32 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x7fc0_0001);
        }
        e.mem.set_u32(cinfo + 0x10, 0x7f80_0002);
        e.mem.set_f32(cinfo + 0x14, 2.0);
        e.call(0x00cc_a9c0, &args![cinfo, 3.0f32.to_bits()]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x7fc0_0002);
        assert_eq!(e.mem.f32(cinfo + 0x14), 6.0);
    }

    #[test]
    fn copy_cinfo_copies_the_three_vertices() {
        let mut e = Engine::new();
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO]);
        e.register(COPY_VECTOR, |e, a| {
            let row = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &row);
            Ret::default()
        });
        let (this, havok) = shape(&mut e);
        for i in 0..12u32 {
            e.mem.set_u32(havok + 0x20 + 4 * i, 0x100 + i);
        }
        let cinfo = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00cc_aa30, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        assert_eq!(
            calls_to(&e, COPY_VECTOR),
            vec![
                vec![cinfo + 0x10, havok + 0x20],
                vec![cinfo + 0x20, havok + 0x30],
                vec![cinfo + 0x30, havok + 0x40]
            ]
        );
        for i in 0..12u32 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x100 + i);
        }
        // No Havok shape, or no shape at all: only the base runs.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cc_aa30, &args![this, cinfo]);
        e.call(0x00cc_aa30, &args![0u32, cinfo]);
        assert!(calls_to(&e, COPY_VECTOR).is_empty());
        assert_eq!(calls_to(&e, CONVEX_SHAPE_COPY_CINFO).len(), 2);
    }

    #[test]
    fn allocate_and_construct_makes_a_0x14_byte_shape() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00cc_aa80, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(e.mem.u32(made), VTABLE_TRIANGLE_SHAPE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_aa80, &args![]).u32(), 0);
    }

    #[test]
    fn copy_members_scales_the_cinfo_unless_the_scale_is_one() {
        let mut e = Engine::new();
        quiet(&mut e, &[SHAPE_COPY_MEMBERS]);
        let (this, _) = shape(&mut e);
        let cinfo = cinfo_with_vertices(&mut e, this);
        for i in 0..12u32 {
            e.mem.set_f32(cinfo + 0x10 + 4 * i, 2.0);
        }
        let process = e.mem.alloc(0x40);
        e.mem.set_f32(process + 0x10, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00cc_aaf0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.f32(cinfo + 0x10), 2.0);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x9999, process]]
        );
        e.mem.set_f32(process + 0x10, 3.0);
        e.call(0x00cc_aaf0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.f32(cinfo + 0x10), 6.0);
        assert_eq!(e.mem.f32(cinfo + 0x3c), 6.0);
        // A NaN scale is not 1.0: it is applied (quieted).
        e.mem.set_u32(process + 0x10, 0x7f80_0001);
        e.call(0x00cc_aaf0, &args![this, 0x9999u32, process]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x7fc0_0001);
        assert_eq!(calls_to(&e, SHAPE_COPY_MEMBERS).len(), 3);
    }

    #[test]
    fn load_binary_swaps_the_vertices_unless_the_stream_byte_is_one() {
        let mut e = Engine::new();
        quiet(&mut e, &[CONVEX_SHAPE_LOAD_BINARY]);
        let (this, _) = shape(&mut e);
        let cinfo = cinfo_with_vertices(&mut e, this);
        e.mem.set_u32(cinfo + 0x40, 0x55);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cc_ab60, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0300);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc_ab60, &args![this, stream]);
        for i in 0..12 {
            assert_eq!(
                e.mem.u32(cinfo + 0x10 + 4 * i),
                (0x0102_0300 + i).swap_bytes()
            );
        }
        // Only the vertices are touched.
        assert_eq!(e.mem.u32(cinfo + 0x40), 0x55);
    }

    #[test]
    fn save_binary_swaps_around_the_base_save() {
        let mut e = Engine::new();
        // The base save sees the swapped words and records the first.
        e.register(CONVEX_SHAPE_SAVE_BINARY, |e, a| {
            let cinfo = e.mem.u32(a[0] + 0xc);
            let seen = e.mem.u32(cinfo + 0x10);
            e.mem.set_u32(a[0] + 0x14, seen);
            Ret::default()
        });
        let (this, _) = shape(&mut e);
        let cinfo = cinfo_with_vertices(&mut e, this);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cc_abf0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x14), 0x0102_0300);
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00cc_abf0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x14), 0x0003_0201);
        // Swapped back afterwards.
        for i in 0..12 {
            assert_eq!(e.mem.u32(cinfo + 0x10 + 4 * i), 0x0102_0300 + i);
        }
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_SAVE_BINARY),
            vec![vec![this, stream]; 2]
        );
    }

    #[test]
    fn create_clone_brackets_the_new_shape_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let (this, _) = shape(&mut e);
        quiet(&mut e, &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00cc_acf0, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_TRIANGLE_SHAPE);
        assert_eq!(
            order(&e),
            vec![
                0x00cc_acf0,
                CLONE_LOCK,
                OPERATOR_NEW,
                BASE_CONSTRUCT,
                0x0900_0101,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, clone, 0x4242]]);
        assert_eq!(scope(&mut e), 0x77);
        // A failed allocation clones into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc_acf0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
    }

    #[test]
    fn create_cinfo_builds_a_zero_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO, COPY_VECTOR]);
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x7f80_0001);
        e.register(POOL_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for offset in 0..a[1] {
                e.mem.set_u8(block + offset, 0xee);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this + 8, 0);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00cc_adb0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x40]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7fc0_0001);
        // The words at +8 and +0xC stay as the pool gave them.
        assert_eq!(e.mem.u32(cinfo + 8), 0xeeee_eeee);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0xeeee_eeee);
        for offset in (0x10..0x40).step_by(4) {
            assert_eq!(e.mem.u32(cinfo + offset), 0, "offset {offset:x}");
        }
        assert!(calls_to(&e, CONVEX_SHAPE_COPY_CINFO).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc_adb0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        // With a Havok shape, CopyCinfo (the shape's own) fills it in.
        let (other, _) = shape(&mut e);
        e.call_log = Some(vec![]);
        let filled = e.call(0x00cc_adb0, &args![other, created]).u32();
        assert_eq!(e.mem.u32(other + 0xc), filled);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_COPY_CINFO),
            vec![vec![other, filled]]
        );
        // A failed allocation stores null.
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        let (third, _) = shape(&mut e);
        e.mem.set_u32(third + 8, 0);
        assert_eq!(e.call(0x00cc_adb0, &args![third, created]).u32(), 0);
    }

    #[test]
    fn free_cinfo_gives_it_back_only_when_asked() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[POOL_FREE]);
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this + 0xc, 0xc1f0);
        e.call_log = Some(vec![]);
        e.call(0x00cc_ae90, &args![this, 0x100u32]);
        assert!(calls_to(&e, POOL_FREE).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0xc1f0);
        assert_eq!(scope(&mut e), 0x77);
        e.call(0x00cc_ae90, &args![this, 0x101u32]);
        assert_eq!(calls_to(&e, POOL_FREE), vec![vec![POOL, 0xc1f0]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn init_builds_the_havok_triangle_from_the_cinfo() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        // The memory router and its allocator.
        let allocator = object(&mut e, &[(4, 0x0900_0200)]);
        e.register(0x0900_0200, |e, a| {
            let block = e.mem.alloc(a[1]);
            for offset in 0..a[1] {
                e.mem.set_u8(block + offset, 0xee);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(0x00c8_5750, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        e.map(HK_DEFAULT_RADIUS, 4);
        e.mem.set_u32(HK_DEFAULT_RADIUS, 0x7f80_0003);
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let this = object(&mut e, &[(0x8c, 0x0900_0103), (0xbc, 0x0900_0104)]);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_f32(cinfo + 4, 0.25);
        for i in 0..12u32 {
            e.mem.set_u32(cinfo + 0x10 + 4 * i, 0x300 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cc_aee0, &args![this, cinfo]);
        let attached = calls_to(&e, 0x0900_0103);
        assert_eq!(attached.len(), 1);
        let shape = attached[0][1];
        assert_eq!(attached, vec![vec![this, shape]]);
        assert_eq!(e.mem.u32(shape), VTABLE_HK_TRIANGLE_SHAPE);
        assert_eq!(e.mem.u16(shape + 4), 0x60);
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.u32(shape + 8), 0);
        assert_eq!(e.mem.u32(shape + 0xc), 3);
        assert_eq!(e.mem.f32(shape + 0x10), 0.25);
        assert_eq!(e.mem.u16(shape + 0x14), 0);
        assert_eq!(e.mem.u8(shape + 0x16), 6);
        assert_eq!(e.mem.u8(shape + 0x17), 0);
        for i in 0..12u32 {
            assert_eq!(e.mem.u32(shape + 0x20 + 4 * i), 0x300 + i);
        }
        for offset in (0x50..0x60).step_by(4) {
            assert_eq!(e.mem.u32(shape + offset), 0);
        }
        // The words the code never writes keep what the allocator gave.
        assert_eq!(e.mem.u32(shape + 0x18), 0xeeee_eeee);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![shape]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(
            order(&e)
                .into_iter()
                .filter(|a| [0x0900_0103, REMOVE_REFERENCE, 0x0900_0104].contains(a))
                .collect::<Vec<_>>(),
            vec![0x0900_0103, REMOVE_REFERENCE, 0x0900_0104]
        );
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        quiet(&mut e, &[CONVEX_SHAPE_GET_VIEWER_STRINGS, 0x0096_ad30]);
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // The list stores each added text at its index.
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00cc_b080, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x1111]]);
        assert!(calls_to(&e, 0x0096_ad30).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // A full list grows first, by its grow-by plus its count.
        e.mem.set_u16(list + 8, 1);
        e.mem.set_u16(list + 0xa, 1);
        e.mem.set_u16(list + 0xe, 4);
        e.call(0x00cc_b080, &args![this, list]);
        assert_eq!(calls_to(&e, 0x0096_ad30), vec![vec![list, 5]]);
    }
}
