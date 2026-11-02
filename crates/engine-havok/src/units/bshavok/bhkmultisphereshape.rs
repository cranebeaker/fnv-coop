//! `BSHavok/bhkmultisphereshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::bhklimitedhingeconstraint::{
    allocate_sized, allocator, enter_scope, increment, push_viewer_string, ALLOCATE, POOL_ALLOCATE,
    POOL_OBJECT, REMOVE_REFERENCE,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkMultiSphereShape` (0x14 bytes, vtable `01066c5c`; Xbox PDB name, RTTI-confirmed): a
/// `bhkSphereRepShape` (Havok shape at `+8`, `pInfo` at `+0xC`, `+0x10` cleared by the
/// constructor). `pInfo` is a 16-byte cinfo from the pool: `+4` the array of spheres
/// (`x, y, z, radius` `float`s, 16 bytes each), `+8` their count and `+0xC` the array's
/// capacity word (bit 31 set: the array does not own its data). The Havok shape keeps the
/// count at `+0x10` and the spheres from `+0x20`.
///
/// The functions that allocate or tear down Havok objects raise the per-thread allocation
/// word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
/// exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
/// functions marked `Translation incomplete` are not modeled.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_MULTI_SPHERE_SHAPE: u32 = 0x0106_6c5c;
/// Instance counters of `bhkShape`, `bhkSphereRepShape` and `bhkMultiSphereShape`, and the
/// class's `NiRTTI` global (whose viewer string heads the list).
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_MULTI_SPHERE_SHAPE: u32 = 0x0126_83dc;
const MULTI_SPHERE_RTTI: u32 = 0x0126_83e0;
/// `bhkSphereRepShape` base methods: `~` (`00ca0540`) and `GetViewerStrings` (`00ca05b0`);
/// `bhkShape` methods: `LoadBinary` (`00c84d00`), `SaveBinary` (`00c84d50`), `CopyMembers`
/// (`00c84df0`, `(clone, process_info)`), `CopyCinfo` (`00c84e20`) and `IsEqual`
/// (`00c84dc0`); `bhkSerializable::Kill` (`00c876a0`), `CloneCritLock` (`00c876f0`),
/// `CloneCritUnlock` (`00c87700`) and the `bhkRefObject` constructor (`00c87c30`).
const SPHERE_REP_DESTRUCT: u32 = 0x00ca_0540;
const SPHERE_REP_GET_VIEWER_STRINGS: u32 = 0x00ca_05b0;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_IS_EQUAL: u32 = 0x00c8_4dc0;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// The stream's array routines (cdecl): `bhkStream::SwapHKArray` (`00cf0930`, `(info,
/// size, array)`), `bhkStream::LoadBinary` (`00cf0f40`, `(stream, array)`), `SaveBinary`
/// (`00cf0cf0`, `(stream, array)`), and the comparison of two arrays (`00cf08c0`, answers
/// whether they are equal in `AL`). The array reserve (`00c908c0`, cdecl `(allocator,
/// array, capacity, element size)`).
const SWAP_HK_ARRAY: u32 = 0x00cf_0930;
const STREAM_LOAD_ARRAY: u32 = 0x00cf_0f40;
const STREAM_SAVE_ARRAY: u32 = 0x00cf_0cf0;
const ARRAYS_EQUAL: u32 = 0x00cf_08c0;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkpMultiSphereShape::hkpMultiSphereShape` (`00d2a760`, `ECX` = the block, two words:
/// the spheres and their count).
const HK_MULTI_SPHERE_CONSTRUCT: u32 = 0x00d2_a760;
/// Pool free (`00aa4060`, thiscall on the pool object, one word: the block).
const POOL_FREE: u32 = 0x00aa_4060;
/// `NiGetViewerString` overloads for an `NiRTTI` (`00a7d330`, cdecl, one word), for an int
/// (`00a7d4d0`, cdecl `(name, value)`) and for a `float` (`00a7d550`, cdecl `(name, bits)`),
/// `NiPoint3::GetViewerString` (`00a59af0`, `ECX` = the point, one word: the name) and the
/// formatter (`00406d00`, cdecl `(buffer, size, format, ...)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const FORMAT: u32 = 0x0040_6d00;
/// "Num Spheres" (`010c8900`), "Pos %d" (`010c88f8`), "Radius %d" (`010c88ec`) and the
/// `float` (`011b1cd4`) the sphere positions are multiplied by before they are shown.
const NAME_NUM_SPHERES: u32 = 0x010c_8900;
const FORMAT_POSITION: u32 = 0x010c_88f8;
const FORMAT_RADIUS: u32 = 0x010c_88ec;
const POSITION_SCALE: u32 = 0x011b_1cd4;
/// The capacity word of an array that owns nothing, and the mask of its capacity.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00cb78b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb78b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::~bhkMultiSphereShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the counter `012683dc` and runs the
/// `bhkSphereRepShape` destructor (`00ca0540`).
pub fn bhk_multi_sphere_shape_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MULTI_SPHERE_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_MULTI_SPHERE_SHAPE);
    e.mem
        .set_u32(COUNT_MULTI_SPHERE_SHAPE, count.wrapping_sub(1));
    e.call(SPHERE_REP_DESTRUCT, &args![this]);
}

/// Constructs a `bhkMultiSphereShape` in `object`: the `bhkRefObject` constructor
/// (`00c87c30`), then the `bhkShape`, `bhkSphereRepShape` and `bhkMultiSphereShape`
/// vtables in turn with their counters up by one, and `+0xC` and `+0x10` cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_SHAPE);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    increment(e, COUNT_SHAPE);
    e.mem.set_u32(object, VTABLE_SPHERE_REP_SHAPE);
    increment(e, COUNT_SPHERE_REP_SHAPE);
    e.mem.set_u32(object, VTABLE_MULTI_SPHERE_SHAPE);
    increment(e, COUNT_MULTI_SPHERE_SHAPE);
}

// Translated from 00cb7920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7920 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates 0x14 bytes (`00aa13e0`) and, when that succeeded, constructs a
/// `bhkMultiSphereShape` there; answers the object, or 0. Takes no arguments (map name
/// unknown).
pub fn fn_00cb7920(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x14u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00cb79d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb79d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`) allocates 0x14 bytes (`00aa13e0`) and constructs a
/// `bhkMultiSphereShape` there when that succeeded, then has this shape copy its members
/// into it through slot `0xC4` (`CopyMembers`) with `process_info`. Answers the clone (0
/// when the allocation failed).
pub fn bhk_multi_sphere_shape_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let clone = if block != 0 {
        construct(e, block);
        block
    } else {
        0
    };
    e.vcall(this.addr(), 0xc4, &args![clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    clone
}

// Translated from 00cb7a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMultiSphereShape::CopyMembers` (Xbox PDB), vtable slot `0xC4`: takes the cinfo from
/// `CreateCinfo` (slot `0xB4`) and, when the scale at `process_info + 0x10` is not exactly
/// 1.0 (a NaN counts as not equal), multiplies all four lanes of each sphere in it by the
/// scale, from the last sphere to the first; then runs `bhkShape::CopyMembers`
/// (`00c84df0`, `(this, clone, process_info)`).
pub fn bhk_multi_sphere_shape_copy_members(
    e: &mut Engine,
    this: Ptr,
    clone: u32,
    process_info: u32,
) {
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(process_info + 0x10)));
    if scale != 1.0 {
        let spheres = e.mem.u32(info + 4);
        for index in (0..e.mem.u32(info + 8)).rev() {
            for lane in 0..4 {
                let address = spheres.wrapping_add(index * 16 + lane * 4);
                let value = e.mem.f32(address);
                e.mem.set_f32(address, scale * value);
            }
        }
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process_info]);
}

// Translated from 00cb7b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMultiSphereShape::LoadBinary` (Xbox PDB): the `bhkShape` load (`00c84d00`), then the
/// cinfo from `CreateCinfo` (slot `0xB4`); for a stream not in PC byte order
/// (`stream + 0x1E5` not 1) the array at `info + 4` is swapped (`00cf0930`, size from slot
/// `0xA8`) before the spheres are read into it (`00cf0f40`).
pub fn bhk_multi_sphere_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    if e.mem.u8(stream + 0x1e5) != 1 {
        let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, info + 4]);
    }
    e.call(STREAM_LOAD_ARRAY, &args![stream, info + 4]);
}

// Translated from 00cb7b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7b80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::Init` (Xbox PDB), vtable slot `0xB0`: with the allocation-scope
/// word at 0x10, allocates 0xA0 bytes through the router (size word at `+4`; the result is
/// not checked) and constructs the `hkpMultiSphereShape` there (`00d2a760`) from the
/// cinfo's spheres (`+4`) and count (`+8`). The new Havok shape goes to slot `0x8C`, is
/// released (`00c905b0`), and slot `0xBC` is called with the cinfo.
pub fn bhk_multi_sphere_shape_init(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let block = allocate_sized(e, 0xa0);
    let spheres = e.mem.u32(info + 4);
    let count = e.mem.u32(info + 8);
    let shape = e
        .call(HK_MULTI_SPHERE_CONSTRUCT, &args![block, spheres, count])
        .u32();
    e.vcall(me, 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(me, 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

/// Makes the sphere array at `array` hold at least `wanted` elements (`00c908c0` with the
/// larger of the wanted count and twice the capacity, element size 16) when it holds fewer.
fn reserve_spheres(e: &mut Engine, array: u32, wanted: i32) {
    let heap = allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![heap, array, new_capacity as u32, 0x10u32],
        );
    }
}

// Translated from 00cb7c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7c60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::CopyCinfo` (Xbox PDB; called by `CreateCinfo`, `00cb8270`, and
/// `IsEqual`), with the cinfo `info`: `bhkShape::CopyCinfo` (`00c84e20`) runs, then the
/// array at `info + 4` is emptied, reserved for the Havok shape's sphere count (`+0x10`,
/// 0 without a Havok shape; `00c908c0`, signed compare) and filled with the 16-byte
/// spheres from the Havok shape's `+0x20`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_multi_sphere_shape_copy_cinfo(e: &mut Engine, this: Ptr, info: u32) {
    let (word, saved) = enter_scope(e);
    e.call(SHAPE_COPY_CINFO, &args![this, info]);
    let havok = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let count = if havok != 0 {
        e.mem.u32(havok + 0x10) as i32
    } else {
        0
    };
    e.mem.set_u32(info + 8, 0);
    reserve_spheres(e, info + 4, count);
    e.mem.set_u32(info + 8, count as u32);
    let source = if havok != 0 { havok + 0x20 } else { 0 };
    for index in 0..count.max(0) as u32 {
        let target = e.mem.u32(info + 4);
        for lane in 0..4 {
            let value = e.mem.u32(source.wrapping_add(index * 16 + lane * 4));
            e.mem
                .set_u32(target.wrapping_add(index * 16 + lane * 4), value);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cb7d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7d80 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMultiSphereShape::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: with the
/// allocation-scope word at 0x10, the `bhkSphereRepShape` strings (`00ca05b0`), the viewer
/// string of the class `NiRTTI` (the global at `012683e0`) and "Num Spheres" with the
/// Havok shape's sphere count (`+0x10`, 0 without one; `00a7d4d0`). Then, for each sphere
/// of the Havok shape (stopping when there is none): the position, `x`, `y` and `z`
/// multiplied by the `float` at `011b1cd4` (x87 products stored as `float`s), named "Pos
/// %d" and shown through `NiPoint3::GetViewerString` (`00a59af0`), and the radius, named
/// "Radius %d" and added as a `float`.
pub fn bhk_multi_sphere_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(SPHERE_REP_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(MULTI_SPHERE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let havok = if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    };
    let count = if havok != 0 {
        e.mem.u32(havok + 0x10) as i32
    } else {
        0
    };
    let text = e
        .call(VIEWER_STRING_INT, &args![NAME_NUM_SPHERES, count as u32])
        .u32();
    push_viewer_string(e, strings, text);
    if havok != 0 && havok.wrapping_add(0x20) != 0 {
        let scale = f64::from(e.global::<f32>(POSITION_SCALE));
        let mut sphere = havok + 0x20;
        let mut index = 0i32;
        while index < count {
            let x = f64::from(e.mem.f32(sphere));
            let y = f64::from(e.mem.f32(sphere + 4));
            let z = f64::from(e.mem.f32(sphere + 8));
            let radius = x87_quiet_bits(e.mem.u32(sphere + 0xc));
            let point = [(x * scale) as f32, (y * scale) as f32, (z * scale) as f32];
            let name = e.with_stack(0x20, |e, buffer| {
                e.call(
                    FORMAT,
                    &args![buffer, 0x20u32, FORMAT_POSITION, index as u32],
                );
                e.with_stack(12, |e, local| {
                    for (lane, value) in point.into_iter().enumerate() {
                        e.mem.set_f32(local.addr() + 4 * lane as u32, value);
                    }
                    e.call(POINT_GET_VIEWER_STRING, &args![local, buffer]).u32()
                })
            });
            push_viewer_string(e, strings, name);
            let text = e.with_stack(0x20, |e, buffer| {
                e.call(FORMAT, &args![buffer, 0x20u32, FORMAT_RADIUS, index as u32]);
                e.call(VIEWER_STRING_FLOAT, &args![buffer, radius]).u32()
            });
            push_viewer_string(e, strings, text);
            sphere = sphere.wrapping_add(0x10);
            index += 1;
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cb8010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb8010 - the FS:[0] exception-unwinding frame is not modeled, and slot 0xA4 receives a stack word whose upper three bytes the exe leaves uninitialised (modeled as 0).
/// `bhkMultiSphereShape::SaveBinary` (Xbox PDB), vtable slot `0x58`: takes the cinfo from
/// `CreateCinfo` (slot `0xB4`; `created` is the byte it sets) and moves its sphere array
/// (`info + 4`) out into a local, leaving it empty (data 0, size 0, capacity word
/// `0x80000000`). When the stream's byte at `+0x1e4` is not 1 the (now empty) array is
/// swapped (`00cf0930`, size from slot `0xA8`) before and, unless the cinfo was created
/// just now, after. In between `bhkShape::SaveBinary` (`00c84d50`) and the local array
/// (`00cf0cf0`) are written, and slot `0xA4` (`KillCinfo`) is given the `created` word.
/// The local array is then released through the allocator (16 bytes per element) unless
/// its capacity word flags it as not owning its data.
pub fn bhk_multi_sphere_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    // The exe's frame (exception record, `SUB ESP,0x10`, four pushes and the cookie): 0x30
    // bytes, the created byte at +0x14 (00cb8041) and the local array at +0x18 (00cb80b6).
    e.with_stack(0x30, |e, frame| {
        save_binary_in_frame(e, this, stream, frame.addr())
    });
}

/// [`bhk_multi_sphere_shape_save_binary`] in the exe's frame (`frame` is its lowest address).
fn save_binary_in_frame(e: &mut Engine, this: Ptr, stream: u32, frame: u32) {
    let me = this.addr();
    let info = e.vcall(me, 0xb4, &args![frame + 0x14]).u32();
    let created = e.mem.u8(frame + 0x14);
    let array = info + 4;
    let local = frame + 0x18;
    let data = e.mem.u32(info + 4);
    e.mem.set_u32(local, data);
    let size = e.mem.u32(info + 8);
    e.mem.set_u32(local + 4, size);
    let capacity = e.mem.u32(info + 0xc);
    e.mem.set_u32(local + 8, capacity);
    e.mem.set_u32(info + 4, 0);
    e.mem.set_u32(info + 8, 0);
    e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let size = e.vcall(me, 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, array]);
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    e.call(STREAM_SAVE_ARRAY, &args![stream, local]);
    e.vcall(me, 0xa4, &args![u32::from(created)]);
    if created == 0 && e.mem.u8(stream + 0x1e4) != 1 {
        let size = e.vcall(me, 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, array]);
    }
    let heap = allocator(e);
    e.mem.set_u32(local + 4, 0);
    let capacity = e.mem.u32(local + 8);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(local);
        e.vcall(heap, 0x10, &args![data, (capacity & CAPACITY_MASK) << 4]);
    }
}

// Translated from 00cb8150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb8150 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::IsEqual` (Xbox PDB), vtable slot `0x5C`: answers 0 when
/// `bhkShape::IsEqual` (`00c84dc0`) does. Otherwise builds a local cinfo (`+0`, size
/// cleared, empty capacity word) for this shape and one for `other` through `CopyCinfo`
/// (`00cb7c60`), compares their sphere arrays (`00cf08c0`), releases the second then the
/// first array through the allocator (16 bytes per element) unless flagged as not owning
/// its data, and answers the comparison.
pub fn bhk_multi_sphere_shape_is_equal(e: &mut Engine, this: Ptr, other: u32) -> u8 {
    let base = e.call(SHAPE_IS_EQUAL, &args![this, other]).u32() as u8;
    if base == 0 {
        return 0;
    }
    e.with_stack(0x20, |e, locals| {
        let first = locals.addr();
        let second = first + 0x10;
        for info in [first, second] {
            e.mem.set_u32(info, 0);
            e.mem.set_u32(info + 4, 0);
            e.mem.set_u32(info + 8, 0);
            e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
        }
        bhk_multi_sphere_shape_copy_cinfo(e, this, first);
        bhk_multi_sphere_shape_copy_cinfo(e, Ptr::new(other), second);
        let equal = e.call(ARRAYS_EQUAL, &args![first + 4, second + 4]).u32() as u8;
        for (info, reset) in [(second, true), (first, false)] {
            let heap = allocator(e);
            e.mem.set_u32(info + 8, 0);
            let capacity = e.mem.u32(info + 0xc);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(info + 4);
                e.vcall(heap, 0x10, &args![data, (capacity & CAPACITY_MASK) << 4]);
            }
            if reset {
                e.mem.set_u32(info + 4, 0);
                e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
            }
        }
        equal
    })
}

// Translated from 00cb8270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb8270 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a byte)
/// is set to whether `pInfo` was empty. If so, a 16-byte block is taken from the pool
/// (`00aa3e40`) and, when that succeeded, cleared with the empty capacity word at `+0xC`;
/// it becomes `pInfo` (0 when the allocation failed) and, when the Havok shape at `+8`
/// exists, the cinfo is filled from it (`CopyCinfo`, `00cb7c60`, also given a null cinfo).
/// Answers `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_multi_sphere_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 0x10u32]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block + 8, 0);
            e.mem.set_u32(block + 0xc, EMPTY_CAPACITY);
        }
        let has_shape = e.mem.u32(me + 8) != 0;
        e.mem.set_u32(me + 0xc, block);
        if has_shape {
            bhk_multi_sphere_shape_copy_cinfo(e, this, block);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cb8330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb8330 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMultiSphereShape::KillCinfo` (Xbox PDB), vtable slot `0xA4`: when the low byte of
/// `flag` is not 0 and `pInfo` (`+0xC`) exists, empties its array (size 0) and releases
/// the data through the allocator (16 bytes per element) unless the capacity word flags
/// it as not owning it, clears the data and sets the empty capacity word, gives the block
/// back to the pool (`00aa4060` on the pool object `011f6238`); `+0xC` is cleared. The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_multi_sphere_shape_kill_cinfo(e: &mut Engine, this: Ptr, flag: u32) {
    let (word, saved) = enter_scope(e);
    if flag as u8 != 0 {
        let info = e.mem.u32(this.addr() + 0xc);
        if info != 0 {
            let heap = allocator(e);
            let capacity = e.mem.u32(info + 0xc);
            e.mem.set_u32(info + 8, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(info + 4);
                e.vcall(heap, 0x10, &args![data, (capacity & CAPACITY_MASK) << 4]);
            }
            e.mem.set_u32(info + 4, 0);
            e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
            e.call(POOL_FREE, &args![POOL_OBJECT, info]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_78b0, bhk_multi_sphere_shape_destruct(Ptr)),
        entry!(0x00cb_7920, fn_00cb7920() -> u32),
        entry!(0x00cb_79d0, bhk_multi_sphere_shape_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00cb_7a90,
            bhk_multi_sphere_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(0x00cb_7b20, bhk_multi_sphere_shape_load_binary(Ptr, u32)),
        entry!(0x00cb_7b80, bhk_multi_sphere_shape_init(Ptr, u32)),
        entry!(0x00cb_7c60, bhk_multi_sphere_shape_copy_cinfo(Ptr, u32)),
        entry!(
            0x00cb_7d80,
            bhk_multi_sphere_shape_get_viewer_strings(Ptr, u32)
        ),
        entry!(0x00cb_8010, bhk_multi_sphere_shape_save_binary(Ptr, u32)),
        entry!(0x00cb_8150, bhk_multi_sphere_shape_is_equal(Ptr, u32) -> u8),
        entry!(0x00cb_8270, bhk_multi_sphere_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cb_8330, bhk_multi_sphere_shape_kill_cinfo(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A shape wrapper whose Havok shape (`+8`) holds `spheres` (count at `+0x10`, the
    /// spheres from `+0x20`); answers the wrapper and the Havok shape.
    fn wrapper(e: &mut Engine, spheres: &[[f32; 4]], slots: &[(u32, u32)]) -> (u32, u32) {
        let this = vobject(e, 0x14, slots);
        let havok = e.mem.alloc(0x30 + 0x10 * spheres.len() as u32);
        e.mem.set_u32(havok + 0x10, spheres.len() as u32);
        for (index, sphere) in spheres.iter().enumerate() {
            for (lane, value) in sphere.iter().enumerate() {
                let at = havok + 0x20 + 0x10 * index as u32 + 4 * lane as u32;
                e.mem.set_f32(at, *value);
            }
        }
        e.mem.set_u32(this + 8, havok);
        (this, havok)
    }

    /// The reserve double: gives the array a fresh buffer and the wanted capacity, and logs.
    fn reserve_double(e: &mut Engine, log: &Calls, buffers: &Rc<RefCell<Vec<u32>>>) {
        let log = Rc::clone(log);
        let buffers = Rc::clone(buffers);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let buffer = e.mem.alloc(0x100);
            buffers.borrow_mut().push(buffer);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = engine();
        e.set_global(COUNT_MULTI_SPHERE_SHAPE, 2u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, SPHERE_REP_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x14);
        e.call(0x00cb_78b0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_MULTI_SPHERE_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_MULTI_SPHERE_SHAPE), 1);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, SPHERE_REP_DESTRUCT]);
        assert_eq!(calls_to(&log, SERIALIZABLE_KILL), [[this]]);
        assert_eq!(calls_to(&log, SPHERE_REP_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00cb7920_builds_the_object_and_counts_it() {
        let mut e = engine();
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0xc, 7);
        e.mem.set_u32(object + 0x10, 8);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00cb_7920, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x14]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_MULTI_SPHERE_SHAPE);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (0, 0));
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_SPHERE_REP_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_MULTI_SPHERE_SHAPE), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cb_7920, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_constructs_and_copies_inside_the_clone_lock() {
        let mut e = engine();
        let clone = e.mem.alloc(0x14);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, 0x0900_0090, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0xc4, 0x0900_0090)]);
        assert_eq!(e.call(0x00cb_79d0, &args![this, 0x99u32]).u32(), clone);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_0090,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(calls_to(&log, ALLOCATE), [[0x14]]);
        assert_eq!(calls_to(&log, 0x0900_0090), [[this, clone, 0x99]]);
        assert_eq!(e.mem.u32(clone), VTABLE_MULTI_SPHERE_SHAPE);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cb_79d0, &args![this, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, 0x0900_0090)[1], [this, 0, 0x98]);
    }

    #[test]
    fn copy_members_scales_every_sphere_unless_the_scale_is_exactly_one() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_COPY_MEMBERS, 0, &log);
        let info = e.mem.alloc(0x20);
        let spheres = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, spheres);
        e.mem.set_u32(info + 8, 2);
        let reset = |e: &mut Engine| {
            for lane in 0..8 {
                e.mem.set_f32(spheres + 4 * lane, 1.0 + lane as f32);
            }
        };
        reset(&mut e);
        e.register_double(0x0900_0048, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0048)]);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 2.0);
        e.call(0x00cb_7a90, &args![this, 0x7000u32, process]);
        let lanes: Vec<f32> = (0..8).map(|lane| e.mem.f32(spheres + 4 * lane)).collect();
        assert_eq!(lanes, [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
        assert_eq!(
            calls_to(&log, SHAPE_COPY_MEMBERS),
            [[this, 0x7000, process]]
        );
        // Exactly 1.0 leaves them alone.
        reset(&mut e);
        e.mem.set_f32(process + 0x10, 1.0);
        e.call(0x00cb_7a90, &args![this, 0x7000u32, process]);
        assert_eq!(e.mem.f32(spheres + 4), 2.0);
        // A NaN scale is "not equal": every lane becomes NaN.
        e.mem.set_f32(process + 0x10, f32::NAN);
        e.call(0x00cb_7a90, &args![this, 0x7000u32, process]);
        assert!((0..8).all(|lane| e.mem.f32(spheres + 4 * lane).is_nan()));
        // No spheres: nothing to scale.
        reset(&mut e);
        e.mem.set_u32(info + 8, 0);
        e.mem.set_f32(process + 0x10, 3.0);
        e.call(0x00cb_7a90, &args![this, 0x7000u32, process]);
        assert_eq!(e.mem.f32(spheres), 1.0);
        assert_eq!(calls_to(&log, SHAPE_COPY_MEMBERS).len(), 4);
    }

    #[test]
    fn load_binary_swaps_the_array_for_a_foreign_stream_before_reading_it() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_LOAD_BINARY, 0, &log);
        record(&mut e, SWAP_HK_ARRAY, 0, &log);
        record(&mut e, STREAM_LOAD_ARRAY, 0, &log);
        record(&mut e, 0x0900_004c, 0x44, &log);
        let info = e.mem.alloc(0x20);
        e.register_double(0x0900_0048, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xa8, 0x0900_004c), (0xb4, 0x0900_0048)]);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cb_7b20, &args![this, stream]);
        assert!(calls_to(&log, SWAP_HK_ARRAY).is_empty());
        assert_eq!(calls_to(&log, STREAM_LOAD_ARRAY), [[stream, info + 4]]);
        assert_eq!(calls_to(&log, SHAPE_LOAD_BINARY), [[this, stream]]);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cb_7b20, &args![this, stream]);
        assert_eq!(calls_to(&log, SWAP_HK_ARRAY), [[info, 0x44, info + 4]]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order[2..],
            [
                SHAPE_LOAD_BINARY,
                0x0900_004c,
                SWAP_HK_ARRAY,
                STREAM_LOAD_ARRAY
            ]
        );
    }

    #[test]
    fn init_builds_the_havok_shape_from_the_spheres_and_releases_it() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, HK_MULTI_SPHERE_CONSTRUCT, 0x8800, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0070, 0, &log);
        record(&mut e, 0x0900_0074, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0x8c, 0x0900_0070), (0xbc, 0x0900_0074)]);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x6000);
        e.mem.set_u32(info + 8, 5);
        e.call(0x00cb_7b80, &args![this, info]);
        assert_eq!(e.mem.u16(block + 4), 0xa0);
        assert_eq!(
            calls_to(&log, HK_MULTI_SPHERE_CONSTRUCT),
            [[block, 0x6000, 5]]
        );
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                HK_MULTI_SPHERE_CONSTRUCT,
                0x0900_0070,
                REMOVE_REFERENCE,
                0x0900_0074
            ]
        );
        assert_eq!(calls_to(&log, 0x0900_0070), [[this, 0x8800]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x8800]]);
        assert_eq!(calls_to(&log, 0x0900_0074), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn copy_cinfo_reserves_and_copies_the_spheres() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let buffers: Rc<RefCell<Vec<u32>>> = Rc::default();
        record(&mut e, SHAPE_COPY_CINFO, 0, &log);
        reserve_double(&mut e, &log, &buffers);
        let (this, _) = wrapper(
            &mut e,
            &[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
            ],
            &[],
        );
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 8, 99);
        e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
        let heap = allocator(&mut e);
        e.call(0x00cb_7c60, &args![this, info]);
        assert_eq!(calls_to(&log, SHAPE_COPY_CINFO), [[this, info]]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE), [[heap, info + 4, 3, 0x10]]);
        assert_eq!(e.mem.u32(info + 8), 3);
        let data = e.mem.u32(info + 4);
        assert_eq!(data, buffers.borrow()[0]);
        let lanes: Vec<f32> = (0..12).map(|lane| e.mem.f32(data + 4 * lane)).collect();
        let expected: Vec<f32> = (1..=12).map(|lane| lane as f32).collect();
        assert_eq!(lanes, expected);
        assert_eq!(e.mem.u32(word), 0x77);
        // Room enough (capacity 4 for 3): no reserve; the spheres are copied again.
        e.mem.set_u32(info + 0xc, 4);
        e.mem.set_f32(data, 0.0);
        e.call(0x00cb_7c60, &args![this, info]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE).len(), 1);
        assert_eq!(e.mem.f32(data), 1.0);
        // Too small: twice the capacity when that is more than wanted.
        e.mem.set_u32(info + 0xc, 2);
        e.call(0x00cb_7c60, &args![this, info]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[1][2..], [4, 0x10]);
        // No Havok shape: an empty array, nothing reserved.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cb_7c60, &args![this, info]);
        assert_eq!(e.mem.u32(info + 8), 0);
        assert_eq!(calls_to(&log, ARRAY_RESERVE).len(), 2);
    }

    #[test]
    fn get_viewer_strings_lists_the_count_and_each_position_and_radius() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(MULTI_SPHERE_RTTI, 0x1234u32);
        e.set_global(POSITION_SCALE, 2.0f32);
        let log: Calls = Rc::default();
        record(&mut e, SPHERE_REP_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, NI_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, VIEWER_STRING_INT, 0x3001, &log);
        record(&mut e, STRINGS_GROW, 0, &log);
        let formats: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let seen = Rc::clone(&formats);
        e.register_double(FORMAT, move |_, a| {
            assert_eq!(a[1], 0x20);
            seen.borrow_mut().push((a[2], a[3]));
            Ret::default()
        });
        let points: Rc<RefCell<Vec<[f32; 3]>>> = Rc::default();
        let named = Rc::clone(&points);
        e.register_double(POINT_GET_VIEWER_STRING, move |e, a| {
            named
                .borrow_mut()
                .push([0, 4, 8].map(|offset| e.mem.f32(a[0] + offset)));
            ret(0x4000 + named.borrow().len() as u32)
        });
        let radii: Rc<RefCell<Vec<u32>>> = Rc::default();
        let radius_log = Rc::clone(&radii);
        e.register_double(VIEWER_STRING_FLOAT, move |_, a| {
            radius_log.borrow_mut().push(a[1]);
            ret(0x5000 + radius_log.borrow().len() as u32)
        });
        let pushed: Rc<RefCell<Vec<u32>>> = Rc::default();
        let stored = Rc::clone(&pushed);
        e.register_double(STRINGS_SET_AT, move |e, a| {
            let text = e.mem.u32(a[2]);
            assert_eq!(a[1], stored.borrow().len() as u32);
            stored.borrow_mut().push(text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let (this, havok) = wrapper(&mut e, &[[1.0, 2.0, 3.0, 0.5], [4.0, 5.0, 6.0, 0.0]], &[]);
        // A signalling NaN radius comes out quiet through FLD / FSTP.
        e.mem.set_u32(havok + 0x20 + 0x10 + 0xc, 0x7fa0_0000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 32);
        e.call(0x00cb_7d80, &args![this, strings]);
        assert_eq!(
            calls_to(&log, SPHERE_REP_GET_VIEWER_STRINGS),
            [[this, strings]]
        );
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), [[0x1234]]);
        assert_eq!(calls_to(&log, VIEWER_STRING_INT), [[NAME_NUM_SPHERES, 2]]);
        assert_eq!(
            *formats.borrow(),
            [
                (FORMAT_POSITION, 0),
                (FORMAT_RADIUS, 0),
                (FORMAT_POSITION, 1),
                (FORMAT_RADIUS, 1)
            ]
        );
        assert_eq!(*points.borrow(), [[2.0, 4.0, 6.0], [8.0, 10.0, 12.0]]);
        assert_eq!(*radii.borrow(), [0.5f32.to_bits(), 0x7fe0_0000]);
        assert_eq!(
            *pushed.borrow(),
            [0x3000, 0x3001, 0x4001, 0x5001, 0x4002, 0x5002]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // Without a Havok shape only the count of 0 is listed.
        e.mem.set_u32(this + 8, 0);
        pushed.borrow_mut().clear();
        e.mem.set_u16(strings + 0xa, 0);
        e.call(0x00cb_7d80, &args![this, strings]);
        assert_eq!(calls_to(&log, VIEWER_STRING_INT)[1], [NAME_NUM_SPHERES, 0]);
        assert_eq!(*pushed.borrow(), [0x3000, 0x3001]);
    }

    #[test]
    fn save_binary_moves_the_array_out_and_writes_it() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_SAVE_BINARY, 0, &log);
        record(&mut e, SWAP_HK_ARRAY, 0, &log);
        record(&mut e, FREE_TARGET, 0, &log);
        record(&mut e, 0x0900_004c, 0x44, &log);
        record(&mut e, 0x0900_0050, 0, &log);
        let local: Rc<Cell<(u32, u32, u32)>> = Rc::default();
        let seen = Rc::clone(&local);
        let seen_log = Rc::clone(&log);
        e.register_double(STREAM_SAVE_ARRAY, move |e, a| {
            seen_log.borrow_mut().push((STREAM_SAVE_ARRAY, a.to_vec()));
            seen.set((e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)));
            Ret::default()
        });
        let info = e.mem.alloc(0x10);
        let created = Rc::new(Cell::new(1u8));
        let flag = Rc::clone(&created);
        e.register_double(0x0900_0048, move |e, a| {
            e.mem.set_u8(a[1], flag.get());
            ret(info)
        });
        let this = vobject(
            &mut e,
            0x14,
            &[
                (0xa8, 0x0900_004c),
                (0xb4, 0x0900_0048),
                (0xa4, 0x0900_0050),
            ],
        );
        let stream = e.mem.alloc(0x300);
        let data = e.mem.alloc(0x40);
        let fill = |e: &mut Engine, capacity: u32| {
            e.mem.set_u32(info + 4, data);
            e.mem.set_u32(info + 8, 2);
            e.mem.set_u32(info + 0xc, capacity);
        };
        // PC stream, new cinfo: no swaps; the owned data (4 elements) is released.
        fill(&mut e, 4);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00cb_8010, &args![this, stream]);
        assert_eq!(local.get(), (data, 2, 4));
        assert_eq!(
            [
                e.mem.u32(info + 4),
                e.mem.u32(info + 8),
                e.mem.u32(info + 0xc)
            ],
            [0, 0, EMPTY_CAPACITY]
        );
        assert_eq!(calls_to(&log, SHAPE_SAVE_BINARY), [[this, stream]]);
        assert_eq!(calls_to(&log, STREAM_SAVE_ARRAY).len(), 1);
        assert_eq!(calls_to(&log, 0x0900_0050), [[this, 1]]);
        assert!(calls_to(&log, SWAP_HK_ARRAY).is_empty());
        assert_eq!(calls_to(&log, FREE_TARGET).len(), 1);
        assert_eq!(calls_to(&log, FREE_TARGET)[0][1..], [data, 0x40]);
        // Foreign stream, existing cinfo: swapped before and after; not owned data is kept.
        created.set(0);
        fill(&mut e, EMPTY_CAPACITY | 4);
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00cb_8010, &args![this, stream]);
        assert_eq!(
            calls_to(&log, SWAP_HK_ARRAY),
            [[info, 0x44, info + 4], [info, 0x44, info + 4]]
        );
        assert_eq!(calls_to(&log, 0x0900_0050)[1], [this, 0]);
        assert_eq!(calls_to(&log, FREE_TARGET).len(), 1);
        // Foreign stream, new cinfo: swapped only before.
        created.set(1);
        fill(&mut e, 4);
        e.call(0x00cb_8010, &args![this, stream]);
        assert_eq!(calls_to(&log, SWAP_HK_ARRAY).len(), 3);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn save_binary_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_SAVE_BINARY, 0, &log);
        record(&mut e, STREAM_SAVE_ARRAY, 0, &log);
        record(&mut e, 0x0900_0050, 0, &log);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
        record(&mut e, 0x0900_0048, info, &log);
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0048), (0xa4, 0x0900_0050)]);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        let frame = e.next_stack_local(0x30);
        e.call(0x00cb_8010, &args![this, stream]);
        // The created byte at +0x14, the local array at +0x18.
        assert_eq!(calls_to(&log, 0x0900_0048), [[this, frame + 0x14]]);
        assert_eq!(calls_to(&log, STREAM_SAVE_ARRAY), [[stream, frame + 0x18]]);
    }

    #[test]
    fn is_equal_compares_the_cinfo_arrays_and_releases_them() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let buffers: Rc<RefCell<Vec<u32>>> = Rc::default();
        let base = Rc::new(Cell::new(1u32));
        let answer = Rc::clone(&base);
        let base_log = Rc::clone(&log);
        e.register_double(SHAPE_IS_EQUAL, move |_, a| {
            base_log.borrow_mut().push((SHAPE_IS_EQUAL, a.to_vec()));
            ret(answer.get())
        });
        record(&mut e, SHAPE_COPY_CINFO, 0, &log);
        record(&mut e, ARRAYS_EQUAL, 1, &log);
        record(&mut e, FREE_TARGET, 0, &log);
        reserve_double(&mut e, &log, &buffers);
        let (this, _) = wrapper(&mut e, &[[1.0; 4], [2.0; 4]], &[]);
        let (other, _) = wrapper(&mut e, &[[1.0; 4], [2.0; 4]], &[]);
        assert_eq!(e.call(0x00cb_8150, &args![this, other]).u8(), 1);
        assert_eq!(calls_to(&log, SHAPE_IS_EQUAL), [[this, other]]);
        let copied = calls_to(&log, SHAPE_COPY_CINFO);
        assert_eq!((copied[0][0], copied[1][0]), (this, other));
        let compared = calls_to(&log, ARRAYS_EQUAL);
        assert_eq!(compared.len(), 1);
        assert_eq!(compared[0][0], copied[0][1] + 4);
        assert_eq!(compared[0][1], copied[1][1] + 4);
        // The second cinfo's data is released first, then the first's (2 elements of 16).
        let frees = calls_to(&log, FREE_TARGET);
        assert_eq!(frees.len(), 2);
        let buffers = buffers.borrow().clone();
        assert_eq!(frees[0][1..], [buffers[1], 0x20]);
        assert_eq!(frees[1][1..], [buffers[0], 0x20]);
        assert_eq!(e.mem.u32(word), 0x77);
        // The base comparison refusing ends it there.
        base.set(0);
        assert_eq!(e.call(0x00cb_8150, &args![this, other]).u8(), 0);
        assert_eq!(calls_to(&log, SHAPE_COPY_CINFO).len(), 2);
        // A different answer of the array comparison is passed on.
        base.set(1);
        record(&mut e, ARRAYS_EQUAL, 0, &log);
        assert_eq!(e.call(0x00cb_8150, &args![this, other]).u8(), 0);
    }

    #[test]
    fn create_cinfo_makes_the_block_once_and_fills_it_from_the_shape() {
        let (mut e, word, _) = engine_with_allocator();
        let block = e.mem.alloc(0x10);
        for offset in 0..4 {
            e.mem.set_u32(block + 4 * offset, 0xaaaa_aaaa);
        }
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, SHAPE_COPY_CINFO, 0, &log);
        let this = e.mem.alloc(0x14);
        let created = e.mem.alloc(4);
        assert_eq!(e.call(0x00cb_8270, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x10]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(words, [0, 0, 0, EMPTY_CAPACITY]);
        assert_eq!(e.mem.u32(this + 0xc), block);
        assert!(calls_to(&log, SHAPE_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time it is kept.
        assert_eq!(e.call(0x00cb_8270, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // With a Havok shape the cinfo is filled from it (here with no spheres).
        let (shaped, _) = wrapper(&mut e, &[], &[]);
        assert_eq!(e.call(0x00cb_8270, &args![shaped, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, SHAPE_COPY_CINFO), [[shaped, block]]);
        // A failed allocation leaves nothing behind.
        record(&mut e, POOL_ALLOCATE, 0, &log);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00cb_8270, &args![this, created]).u32(), 0);
        assert_eq!(e.mem.u8(created), 1);
    }

    #[test]
    fn kill_cinfo_empties_the_array_and_gives_the_block_back_only_when_asked() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, FREE_TARGET, 0, &log);
        record(&mut e, POOL_FREE, 0, &log);
        let this = e.mem.alloc(0x14);
        let info = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 8, 2);
        e.mem.set_u32(info + 0xc, 4);
        e.mem.set_u32(this + 0xc, info);
        e.call(0x00cb_8330, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert!(calls_to(&log, POOL_FREE).is_empty());
        e.call(0x00cb_8330, &args![this, 0x0101u32]);
        assert_eq!(calls_to(&log, FREE_TARGET)[0][1..], [data, 0x40]);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_OBJECT, info]]);
        assert_eq!(
            [
                e.mem.u32(info + 4),
                e.mem.u32(info + 8),
                e.mem.u32(info + 0xc)
            ],
            [0, 0, EMPTY_CAPACITY]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Data the array does not own is not released.
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 0xc, EMPTY_CAPACITY | 4);
        e.mem.set_u32(this + 0xc, info);
        e.call(0x00cb_8330, &args![this, 1u32]);
        assert_eq!(calls_to(&log, FREE_TARGET).len(), 1);
        assert_eq!(calls_to(&log, POOL_FREE).len(), 2);
        // Nothing to free: only +0xC is cleared again.
        e.call(0x00cb_8330, &args![this, 1u32]);
        assert_eq!(calls_to(&log, POOL_FREE).len(), 2);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
