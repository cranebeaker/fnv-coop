//! `BSHavok/bhkcachingshapephantom.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkCachingShapePhantom` (0x18 bytes, vtable `010c4694`) wraps the Havok
//! `hkpCachingShapePhantom` (0x170 bytes) that keeps, at `+0x160` / `+0x164`, the array of
//! collision details (agent and collidable, 8 bytes each) the ray cast walks.
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie check (`00ec408c`) of the
//! functions marked `Translation incomplete` are not modeled; the `RDTSC` time stamps of the
//! profiling records of the ray cast come from the host clock.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSerializable::Kill` (engine map name, `00c876a0`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// `~bhkShapePhantom` (engine map name, `00c9eb30`) and its `GetViewerStrings` (`00c9ed30`).
const SHAPE_PHANTOM_DESTRUCT: u32 = 0x00c9_eb30;
const SHAPE_PHANTOM_GET_VIEWER_STRINGS: u32 = 0x00c9_ed30;
/// `bhkWorldObject::CopyMembers` (engine map name, `00c85900`).
const WORLD_OBJECT_COPY_MEMBERS: u32 = 0x00c8_5900;
/// The base constructor of the 0x10-byte serializable part (`00c87c30`).
const OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::CloneCritLock` / `CloneCritUnlock` (engine map names).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `operator new` (`00aa13e0`, cdecl size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Constructor of the Havok phantom (`00d4ce10`, thiscall on the new block, three stack words:
/// the Cinfo's `+4`, the address of the 64-byte transform copy, the Cinfo's first word).
const HAVOK_PHANTOM_CONSTRUCT: u32 = 0x00d4_ce10;
/// Reference count decrement of a Havok object (`00c905b0`; the engine map does not name it).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `bhkCollisionFilter::IsCollisionEnabled` (engine map name, `00c84740`, cdecl: the two filter
/// infos; answers a bool).
const IS_COLLISION_ENABLED: u32 = 0x00c8_4740;
/// `hkVector4::setRotatedDir` (engine map name, `00c7f700`, thiscall on the result vector: the
/// transform, the vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `kernel32` `TlsGetValue` import slot, and the global with the TLS index of the profiling
/// record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The labels of the two profiling records of the ray cast: `"TtrcCachingPhantom"` (begin) and
/// `"Et"` (end).
const RAY_BEGIN_LABEL: u32 = 0x010c_9544;
const RAY_END_LABEL: u32 = 0x010c_41f8;
/// Vtables of the chain of base classes the constructor steps through, ending in
/// `bhkCachingShapePhantom`'s, and the object counters it raises.
const WORLD_OBJECT_VTABLE_A: u32 = 0x0103_0394;
const WORLD_OBJECT_VTABLE_B: u32 = 0x0103_078c;
const WORLD_OBJECT_VTABLE_C: u32 = 0x0103_06b4;
const CACHING_SHAPE_PHANTOM_VTABLE: u32 = 0x010c_4694;
const COUNT_A: u32 = 0x0126_810c;
const COUNT_B: u32 = 0x0126_8238;
const COUNT_C: u32 = 0x0126_8208;
const CACHING_SHAPE_PHANTOM_COUNT: u32 = 0x0126_85a0;
/// `bhkCachingShapePhantom::ms_RTTI` (its first word is the class name).
const CACHING_SHAPE_PHANTOM_RTTI: u32 = 0x0126_85a4;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkCachingShapePhantom` (Xbox PDB, 0x18 bytes).
    pub struct BhkCachingShapePhantom: 0x18 {
        /// `phkObject` (Xbox PDB): the `hkpCachingShapePhantom`.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB).
        0x0C pInfo: Ptr,
        /// `iFlags` (Xbox PDB, `bhkWorldObject`).
        0x10 iFlags: u32,
        /// `cFlags` (Xbox PDB, `bhkPhantom`).
        0x14 cFlags: u8,
    }
}

layout! {
    /// `hkpCachingShapePhantom` (Xbox PDB, 0x170 bytes) as far as the ray cast uses it.
    pub struct HkpCachingShapePhantom: 0x170 {
        /// `m_world` (Xbox PDB).
        0x008 m_world: Ptr,
        /// `m_collisionDetails.m_data` (Xbox PDB): `hkpCollisionDetail` entries of 8 bytes
        /// (`m_agent`, `m_collidable`).
        0x160 collisionDetailsData: Ptr,
        /// `m_collisionDetails.m_size` (Xbox PDB).
        0x164 collisionDetailsSize: i32,
    }
}

layout! {
    /// `hkpWorld` (Xbox PDB) as far as the ray cast uses it.
    pub struct HkpWorld: 0x350 {
        /// `m_collisionFilter` (Xbox PDB).
        0x07C m_collisionFilter: Ptr,
    }
}

layout! {
    /// `hkpCollidable` (Xbox PDB) as far as the ray cast uses it.
    pub struct HkpCollidable: 0x50 {
        /// `m_shape` (Xbox PDB).
        0x00 m_shape: Ptr,
        /// `m_motion` (Xbox PDB): here the transform (three rotation rows and the
        /// translation, 16 bytes each).
        0x08 m_motion: Ptr,
        /// `m_collisionFilterInfo` (Xbox PDB, in the broad-phase handle).
        0x1C m_collisionFilterInfo: u32,
    }
}

layout! {
    /// `hkpWorldRayCastInput` (Xbox PDB, 0x30 bytes): the two vectors `m_from` (`+0`) and
    /// `m_to` (`+0x10`) are read directly.
    pub struct HkpWorldRayCastInput: 0x30 {
        /// `m_enableShapeCollectionFilter` (Xbox PDB).
        0x20 m_enableShapeCollectionFilter: u8,
        /// `m_filterInfo` (Xbox PDB).
        0x24 m_filterInfo: u32,
    }
}

layout! {
    /// `hkpWorldRayCastOutput` (Xbox PDB, 0x60 bytes): the normal vector is at `+0`.
    pub struct HkpWorldRayCastOutput: 0x60 {
        /// `m_hitFraction` (Xbox PDB).
        0x10 m_hitFraction: f32,
        /// `m_rootCollidable` (Xbox PDB).
        0x50 m_rootCollidable: Ptr,
    }
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the
/// host clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream
/// (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`) is below its limit
/// (`+0xC`); a record is 12 bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

fn load_f32(e: &Engine, address: u32) -> f32 {
    f32::from_bits(e.mem.u32(address))
}

/// The ray point `point` (a 16-byte vector in memory) taken into the space of `transform` as
/// the inlined SSE code does: `v = point - translation` (the vector at `transform + 0x30`),
/// then per lane `(vy * column1 + vx * column0) + vz * column2` where the columns are read
/// from the three rows of the transform (`a`, `b`, `c` at `+0`, `+0x10`, `+0x20`) as
/// `(a1, b1, c1, c3)`, `(a0, b0, c0, c1)` and `(a2, b2, c2, c3)`.
fn into_local_space(e: &Engine, point: u32, transform: u32) -> [f32; 4] {
    let vx = load_f32(e, point) - load_f32(e, transform + 0x30);
    let vy = load_f32(e, point + 4) - load_f32(e, transform + 0x34);
    let vz = load_f32(e, point + 8) - load_f32(e, transform + 0x38);
    let row =
        |base: u32| -> [f32; 4] { std::array::from_fn(|lane| load_f32(e, base + 4 * lane as u32)) };
    let (a, b, c) = (row(transform), row(transform + 0x10), row(transform + 0x20));
    let y_columns = [a[1], b[1], c[1], c[3]];
    let x_columns = [a[0], b[0], c[0], c[1]];
    let z_columns = [a[2], b[2], c[2], c[3]];
    std::array::from_fn(|lane| (vy * y_columns[lane] + vx * x_columns[lane]) + vz * z_columns[lane])
}

fn store_vector(e: &mut Engine, address: u32, lanes: [f32; 4]) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

// Translated from 00cc2350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2350 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCachingShapePhantom::~bhkCachingShapePhantom` (Xbox PDB): sets the vtable, runs `Kill`
/// (`00c876a0`), lowers the object counter (`012685a0`) and runs `~bhkShapePhantom`
/// (`00c9eb30`).
pub fn bhk_caching_shape_phantom_destructor(e: &mut Engine, this: Ptr<BhkCachingShapePhantom>) {
    e.mem.set_u32(this.addr(), CACHING_SHAPE_PHANTOM_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(CACHING_SHAPE_PHANTOM_COUNT);
    e.mem
        .set_u32(CACHING_SHAPE_PHANTOM_COUNT, count.wrapping_sub(1));
    e.call(SHAPE_PHANTOM_DESTRUCT, &args![this]);
}

// Translated from 00cc23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a `bhkCachingShapePhantom` in place (unconfirmed name): the base constructor
/// (`00c87c30`), then each base class's step: its vtable stored and its object counter raised
/// (`0126810c`, `01268238`, `01268208`, `012685a0`), `+0xc` and `+0x10` cleared and the byte
/// at `+0x14` cleared. Answers `this`.
pub fn fn_00cc23c0(e: &mut Engine, this: Ptr<BhkCachingShapePhantom>) -> Ptr {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), WORLD_OBJECT_VTABLE_A);
    e.set(this, BhkCachingShapePhantom::pInfo, Ptr::new(0));
    e.set(this, BhkCachingShapePhantom::iFlags, 0);
    let count = e.mem.u32(COUNT_A);
    e.mem.set_u32(COUNT_A, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), WORLD_OBJECT_VTABLE_B);
    let count = e.mem.u32(COUNT_B);
    e.mem.set_u32(COUNT_B, count.wrapping_add(1));
    e.mem.set_u32(this.addr(), WORLD_OBJECT_VTABLE_C);
    let count = e.mem.u32(COUNT_C);
    e.mem.set_u32(COUNT_C, count.wrapping_add(1));
    e.set(this, BhkCachingShapePhantom::cFlags, 0);
    e.mem.set_u32(this.addr(), CACHING_SHAPE_PHANTOM_VTABLE);
    let count = e.mem.u32(CACHING_SHAPE_PHANTOM_COUNT);
    e.mem
        .set_u32(CACHING_SHAPE_PHANTOM_COUNT, count.wrapping_add(1));
    e.set(this, BhkCachingShapePhantom::cFlags, 0);
    this.cast()
}

// Translated from 00cc2410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2410 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates (0x18 bytes, `operator new`) and constructs a `bhkCachingShapePhantom`
/// (unconfirmed name; the Xbox PDB has a static `CreateObject`). Answers the object, or 0 when
/// the allocation failed.
pub fn fn_00cc2410(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00cc23c0(e, Ptr::new(block))
}

// Translated from 00cc2480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2480 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCachingShapePhantom::CreateClone` (Xbox PDB), `(process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope and the clone lock (`00c876f0`, `00c87700`), constructs a new
/// phantom as `00cc2410` does (0 when the allocation fails) and runs
/// `bhkWorldObject::CopyMembers` (`00c85900`, `(this, clone, process)`) on it. Answers the clone.
pub fn bhk_caching_shape_phantom_create_clone(
    e: &mut Engine,
    this: Ptr<BhkCachingShapePhantom>,
    process: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    let clone = if block != 0 {
        fn_00cc23c0(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    e.call(WORLD_OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, scope);
    clone
}

// Translated from 00cc2530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2530 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkCachingShapePhantom::Init` (Xbox PDB), `(cinfo)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, allocates the 0x170-byte Havok phantom from the memory router's allocator (virtual
/// slot 4) with its 16-bit size stored at `+4`, copies the 64 bytes at `cinfo + 0x20` (the
/// transform; the exe moves them through SSE registers without arithmetic) to a local block
/// and constructs the phantom with `00d4ce10` on `(cinfo + 4, block, cinfo's first word)`.
/// Hands the result to `SethkObject` (virtual `+0x8c`), drops the construction's reference
/// (`00c905b0`) and runs `PostInit` (virtual `+0xbc`) with the Cinfo.
pub fn bhk_caching_shape_phantom_init(
    e: &mut Engine,
    this: Ptr<BhkCachingShapePhantom>,
    cinfo: Ptr,
) {
    let scope = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x170u32]).u32();
    e.mem.set_u16(block + 4, 0x170);
    let created = e.with_stack(0x40, |e, matrix| {
        for offset in (0..0x40).step_by(4) {
            let word = e.mem.u32(cinfo.addr() + 0x20 + offset);
            e.mem.set_u32(matrix.addr() + offset, word);
        }
        let first = e.mem.u32(cinfo.addr());
        let second = e.mem.u32(cinfo.addr() + 4);
        e.call(
            HAVOK_PHANTOM_CONSTRUCT,
            &args![block, second, matrix, first],
        )
        .u32()
    });
    e.vcall(this.addr(), 0x8c, &args![created]);
    e.call(REMOVE_REFERENCE, &args![created]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, scope);
}

// Translated from 00cc26c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc26c0 - the /GS cookie check is not modeled; the RDTSC time stamps of its profiling records come from the host clock.
/// `hkpCachingShapePhantomRay::castRay` (Xbox PDB), `(input, output)` on the Havok phantom:
/// writes a profiling record, then walks the collision details from the last to the first. For
/// each whose collidable has a shape and for which `IsCollisionEnabled(input filter info,
/// collidable filter info)` (`00c84740`) answers true, the ray's two points are taken into
/// the collidable's transform space (the SSE code of [`into_local_space`]) and cast at the
/// shape (virtual `+0x20`, with a result byte, the shape ray cast input and its output; the
/// input's filter info and shape-collection filter, `world->m_collisionFilter + 0x10` when the
/// input enables it and the filter exists, stay the same for every shape, while the output's
/// hit fraction, extra info, first shape key and shape key index are reset to 1, -1, -1 and 0
/// before each cast). A hit replaces the output's normal, hit fraction and root collidable
/// when it is the first hit or its fraction is below the output's (the exe's `FCOMP` skips
/// ties and NaN). The root collidable's transform finally rotates the output normal
/// (`00c7f700`), and a second profiling record is written.
pub fn hkp_caching_shape_phantom_ray_cast_ray(
    e: &mut Engine,
    this: Ptr<HkpCachingShapePhantom>,
    input: Ptr<HkpWorldRayCastInput>,
    output: Ptr<HkpWorldRayCastOutput>,
) {
    profile_record(e, RAY_BEGIN_LABEL);
    // The stack frame of the exe from the result byte (`+7`) to the end of the shape ray cast
    // output: the shape ray cast input at `+0x10` (from, to, filter info, shape-collection
    // filter) and its output at `+0x40` (normal, hit fraction at `+0x50`, extra info at
    // `+0x54`, first shape key at `+0x60`, shape key index at `+0x80`).
    const RESULT: u32 = 7;
    const SHAPE_INPUT: u32 = 0x10;
    const SHAPE_OUTPUT: u32 = 0x40;
    const HIT_FRACTION: u32 = 0x50;
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0xa4`, three pushes) is 0xb0 bytes from a
    // 16-byte boundary; `frame` here is its +0x10 (the result byte at +0x17, 00cc289d; the shape
    // input at +0x20, 00cc2878; its output at +0x50, 00cc2862).
    e.with_stack(0xb0 + 0x10, |e, raw| {
        let frame = ((raw.addr() + 0xf) & !0xf) + 0x10;
        e.mem.set_u32(frame + HIT_FRACTION, 1.0f32.to_bits());
        e.mem.set_u32(frame + 0x54, 0xffff_ffff);
        e.mem.set_u32(frame + 0x60, 0xffff_ffff);
        e.mem.set_u32(frame + 0x80, 0);
        e.mem.set_u32(frame + 0x34, 0);
        let filter_info = e.get(input, HkpWorldRayCastInput::m_filterInfo);
        e.mem.set_u32(frame + 0x30, filter_info);
        if e.get(input, HkpWorldRayCastInput::m_enableShapeCollectionFilter) != 0 {
            let world = e.get(this, HkpCachingShapePhantom::m_world);
            let filter = e
                .get(world.cast::<HkpWorld>(), HkpWorld::m_collisionFilter)
                .addr();
            let collection_filter = if filter != 0 { filter + 0x10 } else { 0 };
            e.mem.set_u32(frame + 0x34, collection_filter);
        } else {
            e.mem.set_u32(frame + 0x34, 0);
        }
        let mut index = e
            .get(this, HkpCachingShapePhantom::collisionDetailsSize)
            .wrapping_sub(1);
        let mut have_hit = false;
        while index >= 0 {
            let details = e
                .get(this, HkpCachingShapePhantom::collisionDetailsData)
                .addr();
            let collidable = e.mem.u32(
                details
                    .wrapping_add((index as u32).wrapping_mul(8))
                    .wrapping_add(4),
            );
            let shape = e.mem.u32(collidable);
            let collidable = Ptr::<HkpCollidable>::new(collidable);
            let ray_filter = e.mem.u32(input.addr() + 0x24);
            let collidable_filter = e.get(collidable, HkpCollidable::m_collisionFilterInfo);
            if shape != 0
                && e.call(IS_COLLISION_ENABLED, &args![ray_filter, collidable_filter])
                    .bool()
            {
                let transform = e.get(collidable, HkpCollidable::m_motion).addr();
                let from = into_local_space(e, input.addr(), transform);
                store_vector(e, frame + SHAPE_INPUT, from);
                let to = into_local_space(e, input.addr() + 0x10, transform);
                store_vector(e, frame + SHAPE_INPUT + 0x10, to);
                e.mem.set_u32(frame + HIT_FRACTION, 1.0f32.to_bits());
                // After the two pushes (00cc2866, 00cc287f) ESP is 8 bytes lower: 00cc2885
                // [ESP+0x6c], 00cc2889 [ESP+0x98] and 00cc2894 [ESP+0x78] are the same extra info,
                // shape key index and first shape key the reset before the loop writes.
                e.mem.set_u32(frame + 0x54, 0xffff_ffff);
                e.mem.set_u32(frame + 0x80, 0);
                e.mem.set_u32(frame + 0x60, 0xffff_ffff);
                e.vcall(
                    shape,
                    0x20,
                    &args![frame + RESULT, frame + SHAPE_INPUT, frame + SHAPE_OUTPUT],
                );
                if e.mem.u8(frame + RESULT) != 0 {
                    let new_fraction = load_f32(e, frame + HIT_FRACTION);
                    let best = load_f32(e, output.addr() + 0x10);
                    let better = best > new_fraction;
                    if !have_hit || better {
                        let normal: Vec<u32> = (0..4)
                            .map(|lane| e.mem.u32(frame + SHAPE_OUTPUT + 4 * lane))
                            .collect();
                        e.mem
                            .set_u32(output.addr() + 0x10, x87_quiet_bits(new_fraction.to_bits()));
                        have_hit = true;
                        for (lane, word) in normal.into_iter().enumerate() {
                            e.mem.set_u32(output.addr() + 4 * lane as u32, word);
                        }
                        e.set(
                            output,
                            HkpWorldRayCastOutput::m_rootCollidable,
                            collidable.cast(),
                        );
                    }
                }
            }
            index = index.wrapping_sub(1);
        }
    });
    let root = e.get(output, HkpWorldRayCastOutput::m_rootCollidable);
    if root.addr() != 0 {
        let transform = e.get(root.cast::<HkpCollidable>(), HkpCollidable::m_motion);
        e.call(SET_ROTATED_DIR, &args![output, transform, output]);
    }
    profile_record(e, RAY_END_LABEL);
}

// Translated from 00cc2950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2950 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCachingShapePhantom::GetViewerStrings` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10`
/// scope, the base strings (`00c9ed30`) and the class name (`NiGetViewerString` of the first
/// word of the RTTI at `012685a4`) stored in the string array.
pub fn bhk_caching_shape_phantom_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkCachingShapePhantom>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(SHAPE_PHANTOM_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CACHING_SHAPE_PHANTOM_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cc2350,
            bhk_caching_shape_phantom_destructor(Ptr<BhkCachingShapePhantom>)
        ),
        entry!(0x00cc23c0, fn_00cc23c0(Ptr<BhkCachingShapePhantom>) -> Ptr),
        entry!(0x00cc2410, fn_00cc2410() -> Ptr),
        entry!(
            0x00cc2480,
            bhk_caching_shape_phantom_create_clone(Ptr<BhkCachingShapePhantom>, Ptr) -> Ptr
        ),
        entry!(
            0x00cc2530,
            bhk_caching_shape_phantom_init(Ptr<BhkCachingShapePhantom>, Ptr)
        ),
        entry!(
            0x00cc26c0,
            hkp_caching_shape_phantom_ray_cast_ray(
                Ptr<HkpCachingShapePhantom>,
                Ptr<HkpWorldRayCastInput>,
                Ptr<HkpWorldRayCastOutput>,
            )
        ),
        entry!(
            0x00cc2950,
            bhk_caching_shape_phantom_get_viewer_strings(Ptr<BhkCachingShapePhantom>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHAPE_SLOT: u32 = 0x7000_0020;
    const HAVOK_SLOT_8C: u32 = 0x7000_008c;
    const HAVOK_SLOT_BC: u32 = 0x7000_00bc;
    const ALLOC_SLOT: u32 = 0x7000_0004;
    const ROUTER_OBJECT: u32 = 0x7100_0000;
    const PROFILE_STREAM: u32 = 0x7200_0000;
    const INSPECTED: u32 = 0x7200_0040;
    const WORLD: u32 = 0x7300_0000;
    const FILTER: u32 = 0x7400_0000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// Writes a vtable at `address` whose slots (byte offsets) point at the targets.
    fn put_vtable_at(e: &mut Engine, address: u32, entries: &[(u32, u32)]) {
        let words = entries
            .iter()
            .map(|(offset, _)| offset / 4 + 1)
            .max()
            .unwrap();
        let mut table = vec![0u32; words as usize];
        for (offset, target) in entries {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &table);
    }

    #[test]
    fn bhk_caching_shape_phantom_destructor_runs_kill_then_the_phantom_destructor() {
        let mut e = logged_engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SHAPE_PHANTOM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(CACHING_SHAPE_PHANTOM_COUNT, 3);
        e.call(0x00cc_2350, &args![this]);
        assert_eq!(e.mem.u32(this), CACHING_SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(CACHING_SHAPE_PHANTOM_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SHAPE_PHANTOM_DESTRUCT), vec![vec![this]]);
        assert_eq!(
            order_of(&e, &[SERIALIZABLE_KILL, SHAPE_PHANTOM_DESTRUCT]),
            vec![SERIALIZABLE_KILL, SHAPE_PHANTOM_DESTRUCT]
        );
    }

    #[test]
    fn fn_00cc23c0_constructs_the_phantom_through_its_bases() {
        let mut e = logged_engine();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        for offset in [0xc, 0x10, 0x14] {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        for (address, value) in [
            (COUNT_A, 1),
            (COUNT_B, 2),
            (COUNT_C, 3),
            (CACHING_SHAPE_PHANTOM_COUNT, 4),
        ] {
            e.mem.set_u32(address, value);
        }
        assert_eq!(e.call(0x00cc_23c0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), CACHING_SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.mem.u32(COUNT_A), 2);
        assert_eq!(e.mem.u32(COUNT_B), 3);
        assert_eq!(e.mem.u32(COUNT_C), 4);
        assert_eq!(e.mem.u32(CACHING_SHAPE_PHANTOM_COUNT), 5);
    }

    #[test]
    fn fn_00cc2410_allocates_0x18_bytes_and_constructs() {
        let mut e = logged_engine();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00cc_2410, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x18]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), CACHING_SHAPE_PHANTOM_VTABLE);
        // A failed allocation answers 0.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_2410, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn bhk_caching_shape_phantom_create_clone_copies_the_members_under_the_clone_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(WORLD_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00cc_2480, &args![this, 0x3000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), CACHING_SHAPE_PHANTOM_VTABLE);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 0x3000]]
        );
        let expected = vec![
            CLONE_LOCK,
            OPERATOR_NEW,
            OBJECT_CONSTRUCT,
            WORLD_OBJECT_COPY_MEMBERS,
            CLONE_UNLOCK,
        ];
        assert_eq!(order_of(&e, &expected), expected);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A failed allocation still copies, onto a null clone.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_2480, &args![this, 0x3000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_COPY_MEMBERS)[1],
            vec![this, 0, 0x3000]
        );
    }

    #[test]
    fn bhk_caching_shape_phantom_init_builds_the_havok_phantom() {
        let mut e = logged_engine();
        put_vtable_at(&mut e, 0x7001_1000, &[(4, ALLOC_SLOT)]);
        e.map(ROUTER_OBJECT, 0x40);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ROUTER_OBJECT + 0x20);
        e.mem.set_u32(ROUTER_OBJECT + 0x20, 0x7001_1000);
        e.register(MEMORY_ROUTER, |_, _| ROUTER_OBJECT.into_ret());
        e.register(ALLOC_SLOT, |e, a| e.mem.alloc(a[1]).into_ret());
        // The double keeps the transform copy, which is freed afterwards, inside the block.
        e.register(HAVOK_PHANTOM_CONSTRUCT, |e, a| {
            let bytes = e.mem.bytes(a[2], 0x40);
            e.mem.write(a[0] + 0x10, &bytes);
            a[0].into_ret()
        });
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(HAVOK_SLOT_8C, |_, _| Ret::default());
        e.register(HAVOK_SLOT_BC, |_, _| Ret::default());
        put_vtable_at(
            &mut e,
            0x7001_2000,
            &[(0x8c, HAVOK_SLOT_8C), (0xbc, HAVOK_SLOT_BC)],
        );
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, 0x7001_2000);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo, 0x1111);
        e.mem.set_u32(cinfo + 4, 0x2222);
        for index in 0..16u32 {
            e.mem.set_u32(cinfo + 0x20 + 4 * index, 0x100 + index);
        }
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cc_2530, &args![this, cinfo]);
        let constructs = calls_to(&e, HAVOK_PHANTOM_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let block = constructs[0][0];
        assert_eq!(constructs[0][1], 0x2222);
        assert_eq!(constructs[0][3], 0x1111);
        assert_eq!(e.mem.u16(block + 4), 0x170);
        assert_eq!(
            calls_to(&e, ALLOC_SLOT),
            vec![vec![ROUTER_OBJECT + 0x20, 0x170]]
        );
        let copied: Vec<u32> = (0..16)
            .map(|index| e.mem.u32(block + 0x10 + 4 * index))
            .collect();
        let expected: Vec<u32> = (0..16).map(|index| 0x100 + index).collect();
        assert_eq!(copied, expected);
        assert_eq!(calls_to(&e, HAVOK_SLOT_8C), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, HAVOK_SLOT_BC), vec![vec![this, cinfo]]);
        let order = vec![
            HAVOK_PHANTOM_CONSTRUCT,
            HAVOK_SLOT_8C,
            REMOVE_REFERENCE,
            HAVOK_SLOT_BC,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    struct Scene {
        this: u32,
        collidables: Vec<u32>,
        input: u32,
        output: u32,
    }

    /// The ray cast scene: a phantom with `shapes` collision details (collidable `k` has the
    /// transform with rows (1,2,3,4), (5,6,7,8), (9,10,11,12) and no translation, the filter
    /// info `0x100 + k` and a shape whose virtual `+0x20` is `SHAPE_SLOT`; the shape's `+4`
    /// holds `k`), a world with a collision filter, the profile stream, and a ray from
    /// (1,1,1,1) to (2,2,2,2) with filter info `0xabcd`.
    fn scene(e: &mut Engine, shapes: usize) -> Scene {
        e.map(PROFILE_STREAM, 0x100);
        e.mem.set_u32(PROFILE_STREAM + 4, PROFILE_STREAM + 0x20);
        e.mem.set_u32(PROFILE_STREAM + 0xc, PROFILE_STREAM + 0x80);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register(TLS_GET_VALUE, |_, _| PROFILE_STREAM.into_ret());
        e.register(IS_COLLISION_ENABLED, |_, _| 1u32.into_ret());
        e.register(SET_ROTATED_DIR, |_, _| Ret::default());
        e.register(SHAPE_SLOT, |_, _| Ret::default());
        put_vtable_at(e, 0x7001_3000, &[(0x20, SHAPE_SLOT)]);
        e.map(WORLD, 0x100);
        e.mem.set_u32(WORLD + 0x7c, FILTER);
        let this = e.mem.alloc(0x170);
        e.mem.set_u32(this + 8, WORLD);
        let details = e.mem.alloc(8 * shapes as u32);
        e.mem.set_u32(this + 0x160, details);
        e.mem.set_u32(this + 0x164, shapes as u32);
        let mut collidables = vec![];
        for index in 0..shapes as u32 {
            let collidable = e.mem.alloc(0x50);
            let transform = e.mem.alloc(0x40);
            let shape = e.mem.alloc(0x10);
            e.mem.set_u32(shape, 0x7001_3000);
            e.mem.set_u32(shape + 4, index);
            e.mem.set_u32(collidable, shape);
            e.mem.set_u32(collidable + 8, transform);
            e.mem.set_u32(collidable + 0x1c, 0x100 + index);
            e.mem.set_u32(details + 8 * index + 4, collidable);
            for (row, base) in [(0, 1.0f32), (1, 5.0), (2, 9.0)] {
                for lane in 0..4u32 {
                    e.mem
                        .set_f32(transform + 0x10 * row + 4 * lane, base + lane as f32);
                }
            }
            collidables.push(collidable);
        }
        let input = e.mem.alloc(0x30);
        for lane in 0..4u32 {
            e.mem.set_f32(input + 4 * lane, 1.0);
            e.mem.set_f32(input + 0x10 + 4 * lane, 2.0);
        }
        e.mem.set_u32(input + 0x24, 0xabcd);
        let output = e.mem.alloc(0x60);
        e.mem.set_f32(output + 0x10, 1.0);
        Scene {
            this,
            collidables,
            input,
            output,
        }
    }

    #[test]
    fn hkp_caching_shape_phantom_ray_cast_ray_keeps_the_nearest_hit_from_the_last_detail_up() {
        let mut e = logged_engine();
        let scene = scene(&mut e, 3);
        // Shape k hits at fraction [0.75, 0.25, 0.5][k] with a normal of (k, k, k).
        e.register(SHAPE_SLOT, |e, a| {
            let (shape, result, output) = (a[0], a[1], a[3]);
            let k = e.mem.u32(shape + 4);
            e.mem.set_u8(result, 1);
            e.mem
                .set_f32(output + 0x10, [0.75f32, 0.25, 0.5][k as usize]);
            for lane in 0..3 {
                e.mem.set_f32(output + 4 * lane, k as f32);
            }
            Ret::default()
        });
        let shapes: Vec<u32> = scene.collidables.iter().map(|c| e.mem.u32(*c)).collect();
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        let casts = calls_to(&e, SHAPE_SLOT);
        assert_eq!(
            casts.iter().map(|c| c[0]).collect::<Vec<u32>>(),
            vec![shapes[2], shapes[1], shapes[0]]
        );
        // Shape 2 hits at 0.5, shape 1 at 0.25 is nearer, shape 0 at 0.75 is not.
        assert_eq!(e.mem.f32(scene.output + 0x10), 0.25);
        assert_eq!(e.mem.u32(scene.output + 0x50), scene.collidables[1]);
        assert_eq!(e.mem.f32(scene.output), 1.0);
        assert_eq!(e.mem.f32(scene.output + 8), 1.0);
        let transform = e.mem.u32(scene.collidables[1] + 8);
        assert_eq!(
            calls_to(&e, SET_ROTATED_DIR),
            vec![vec![scene.output, transform, scene.output]]
        );
        // Two profiling records: the begin and the end labels.
        assert_eq!(e.mem.u32(PROFILE_STREAM + 4), PROFILE_STREAM + 0x20 + 24);
        assert_eq!(e.mem.u32(PROFILE_STREAM + 0x20), RAY_BEGIN_LABEL);
        assert_eq!(e.mem.u32(PROFILE_STREAM + 0x2c), RAY_END_LABEL);
    }

    #[test]
    fn hkp_caching_shape_phantom_ray_cast_ray_keeps_its_locals_in_the_aligned_exe_frame() {
        let mut e = logged_engine();
        let scene = scene(&mut e, 2);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        let casts = calls_to(&e, SHAPE_SLOT);
        assert_eq!(casts.len(), 2);
        for cast in &casts {
            // Frame +0x17 (from a 16-byte boundary), +0x20 and +0x50.
            assert_eq!(cast[1] & 0xf, 7);
            assert_eq!(cast[2], cast[1] + 9);
            assert_eq!(cast[3], cast[1] + 0x39);
            assert_eq!(cast[1], casts[0][1]);
        }
    }

    #[test]
    fn hkp_caching_shape_phantom_ray_cast_ray_resets_the_shape_output_before_each_cast() {
        let mut e = logged_engine();
        let scene = scene(&mut e, 2);
        // The double records, for shape k, the shape ray cast output's extra info (+0x14), first
        // shape key (+0x20), shape key index (+0x40) and the words +0x1c, +0x28, +0x48 as it is
        // given them, then overwrites all six and reports no hit.
        e.register(SHAPE_SLOT, |e, a| {
            let (shape, result, output) = (a[0], a[1], a[3]);
            let record = INSPECTED + 0x18 * e.mem.u32(shape + 4);
            for (slot, offset) in [0x14u32, 0x20, 0x40, 0x1c, 0x28, 0x48]
                .into_iter()
                .enumerate()
            {
                let word = e.mem.u32(output + offset);
                e.mem.set_u32(record + 4 * slot as u32, word);
                e.mem.set_u32(output + offset, 0x5000 + offset);
            }
            e.mem.set_u8(result, 0);
            Ret::default()
        });
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 2);
        let seen = |e: &Engine, k: u32| -> Vec<u32> {
            (0..6)
                .map(|slot| e.mem.u32(INSPECTED + 0x18 * k + 4 * slot))
                .collect()
        };
        // The first cast (detail 1) gets the reset before the loop.
        assert_eq!(seen(&e, 1)[..3].to_vec(), vec![0xffff_ffff, 0xffff_ffff, 0]);
        // The second (detail 0) gets -1, -1 and 0 again (00cc2885, 00cc2894, 00cc2889 after the
        // two pushes), while the words the first cast left at +0x1c, +0x28, +0x48 are kept.
        assert_eq!(
            seen(&e, 0),
            vec![0xffff_ffff, 0xffff_ffff, 0, 0x501c, 0x5028, 0x5048]
        );
    }

    #[test]
    fn hkp_caching_shape_phantom_ray_cast_ray_passes_the_ray_in_each_shape_space() {
        let mut e = logged_engine();
        let scene = scene(&mut e, 1);
        let transform = e.mem.u32(scene.collidables[0] + 8);
        for lane in 0..3 {
            e.mem.set_f32(transform + 0x30 + 4 * lane, 0.5);
        }
        // The double copies the shape ray cast input it was given and reports no hit.
        e.register(SHAPE_SLOT, |e, a| {
            let bytes = e.mem.bytes(a[2], 0x30);
            e.mem.write(INSPECTED, &bytes);
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.mem.set_u8(scene.input + 0x20, 1);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        let from: Vec<f32> = (0..4).map(|lane| e.mem.f32(INSPECTED + 4 * lane)).collect();
        // v = (0.5, 0.5, 0.5): lane 0 = (vy*a1 + vx*a0) + vz*a2 = (1 + 0.5) + 1.5, lane 3 =
        // (vy*c3 + vx*c1) + vz*c3.
        assert_eq!(from, vec![3.0, 9.0, 15.0, 17.0]);
        let to: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(INSPECTED + 0x10 + 4 * lane))
            .collect();
        assert_eq!(to, vec![9.0, 27.0, 45.0, 51.0]);
        // The filter info of the input and the world's collision filter plus 0x10.
        assert_eq!(e.mem.u32(INSPECTED + 0x20), 0xabcd);
        assert_eq!(e.mem.u32(INSPECTED + 0x24), FILTER + 0x10);
        // No hit: the normal is not rotated.
        assert!(calls_to(&e, SET_ROTATED_DIR).is_empty());
        // Without the enable byte the collection filter is null.
        e.mem.set_u8(scene.input + 0x20, 0);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        assert_eq!(e.mem.u32(INSPECTED + 0x24), 0);
    }

    #[test]
    fn hkp_caching_shape_phantom_ray_cast_ray_skips_filtered_missing_and_tied_hits() {
        let mut e = logged_engine();
        let scene = scene(&mut e, 3);
        // Collidable 2 is rejected by the filter, collidable 1 has no shape.
        e.register(IS_COLLISION_ENABLED, |_, a| (a[1] != 0x102).into_ret());
        e.register(SHAPE_SLOT, |e, a| {
            e.mem.set_u8(a[1], 1);
            e.mem.set_f32(a[3] + 0x10, 0.5);
            Ret::default()
        });
        e.mem.set_u32(scene.collidables[1], 0);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), 1);
        assert_eq!(
            calls_to(&e, IS_COLLISION_ENABLED),
            vec![vec![0xabcd, 0x102], vec![0xabcd, 0x100]]
        );
        assert_eq!(e.mem.f32(scene.output + 0x10), 0.5);
        assert_eq!(e.mem.u32(scene.output + 0x50), scene.collidables[0]);
        // Equal fractions: the first hit found (the last detail) stays.
        let shape = e.mem.u32(scene.collidables[0]);
        e.mem.set_u32(scene.collidables[1], shape);
        e.register(IS_COLLISION_ENABLED, |_, _| 1u32.into_ret());
        e.mem.set_u32(scene.output + 0x50, 0);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        assert_eq!(e.mem.u32(scene.output + 0x50), scene.collidables[2]);
        // No details: nothing is cast and the normal is not rotated.
        let casts = calls_to(&e, SHAPE_SLOT).len();
        e.mem.set_u32(scene.this + 0x164, 0);
        e.mem.set_u32(scene.output + 0x50, 0);
        e.call(0x00cc_26c0, &args![scene.this, scene.input, scene.output]);
        assert_eq!(calls_to(&e, SHAPE_SLOT).len(), casts);
        assert_eq!(calls_to(&e, SET_ROTATED_DIR).len(), 2);
        assert_eq!(calls_to(&e, TLS_GET_VALUE).len(), 6);
    }

    #[test]
    fn bhk_caching_shape_phantom_get_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(SHAPE_PHANTOM_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(CACHING_SHAPE_PHANTOM_RTTI, 0x4444);
        let this = e.mem.alloc(0x18);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cc_2950, &args![this, array]);
        assert_eq!(
            calls_to(&e, SHAPE_PHANTOM_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
