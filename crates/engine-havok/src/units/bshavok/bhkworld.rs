//! `BSHavok/bhkworld.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhknicollisionobject::BhkNiCollisionObject;

const HEAP_ALLOC: u32 = 0x00aa_3e40;
const HEAP_FREE: u32 = 0x00aa_4060;
const HEAP_SINGLETON: u32 = 0x011f_6238;
const MEMORY_ALLOCATOR_DESTRUCTOR: u32 = 0x00cb_f1a0;
const HAVOK_WORLD_DESTRUCTOR: u32 = 0x00c9_3e50;
const GLOBAL_WORLD_MODE: u32 = 0x011a_fe5c;
const GLOBAL_PICK_COUNT_OVERRIDE: u32 = 0x0126_77a3;
const GLOBAL_WORLD_FLOAT: u32 = 0x0126_77c4;
const GLOBAL_WORLD_INTEGER: u32 = 0x0126_77c8;
const ALLOCATOR_VTABLE: u32 = 0x010c_3af8;
const HAVOK_WORLD_VTABLE: u32 = 0x010c_3bc4;

layout! {
    /// `bhkWorld` (Xbox PDB), 0xA0 bytes; offsets used by this unit agree with the PC image.
    pub struct BhkWorld: 0xa0 {
        /// `pInfo` (Xbox PDB), the `hkSerializableCinfo*` stored by the world.
        0x0c pInfo: Ptr,
        /// `pDebugger` (Xbox PDB).
        0x10 pDebugger: Ptr,
        /// `bAdded` (Xbox PDB).
        0x14 bAdded: bool,
        /// `bEnabled` (Xbox PDB).
        0x15 bEnabled: bool,
        /// `bDebugDisplay` (Xbox PDB).
        0x16 bDebugDisplay: bool,
        /// `iBatchAdd` (Xbox PDB).
        0x18 iBatchAdd: u32,
        /// `iBatchRemove` (Xbox PDB).
        0x1c iBatchRemove: u32,
        /// `pEntityBatch` (Xbox PDB), the entities waiting to be added (12000-byte array).
        0x2c pEntityBatch: Ptr,
        /// `iEntityBatchCount` (Xbox PDB).
        0x30 iEntityBatchCount: u32,
        /// `pPhantomBatch` (Xbox PDB), the phantoms waiting to be added or removed (800-byte array).
        0x34 pPhantomBatch: Ptr,
        /// `iPhantomBatchCount` (Xbox PDB).
        0x38 iPhantomBatchCount: u32,
        /// `pActionBatch` (Xbox PDB), the actions waiting to be removed (400-byte array).
        0x3c pActionBatch: Ptr,
        /// `iActionBatchCount` (Xbox PDB).
        0x40 iActionBatchCount: u32,
        /// `pConstraintInstanceBatch` (Xbox PDB), constraints waiting to be added or removed (800-byte array).
        0x44 pConstraintInstanceBatch: Ptr,
        /// `iConstraintInstanceBatchCount` (Xbox PDB).
        0x48 iConstraintInstanceBatchCount: u32,
        /// `pEntityRemoveBatch` (Xbox PDB), the entities waiting to be removed (12000-byte array).
        0x4c pEntityRemoveBatch: Ptr,
        /// `iEntityRemoveBatchCount` (Xbox PDB).
        0x50 iEntityRemoveBatchCount: u32,
        /// `Origin` (Xbox PDB), a four-float `hkVector4`.
        0x60 origin: [f32; 4],
        /// `pEntityListenerA.data` (Xbox PDB), the `hkArray` pointer.
        0x70 entityListeners: Ptr,
        /// `pEntityListenerA.size` (Xbox PDB).
        0x74 entityListenerCount: i32,
        /// `DebugColorA.data` (Xbox PDB), the `hkArray` of eight-byte `ColorPair`s.
        0x7c debugColorData: Ptr,
        /// `DebugColorA.size` (Xbox PDB).
        0x80 debugColorCount: u32,
        /// `fOriginalTau` (Xbox PDB).
        0x88 fOriginalTau: f32,
        /// `fOriginalDamping` (Xbox PDB).
        0x8c fOriginalDamping: f32,
    }

    /// `hkQuaternion` (Xbox PDB), a four-float `hkVector4`.
    pub struct HkQuaternion: 0x10 {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
        0x0c w: f32,
    }

    /// The `m_rootCollidable` field read by the two `bhkPickData` helpers (Xbox PDB).
    pub struct BhkPickData: 0xb0 {
        0x80 rootCollidable: Ptr,
    }
}

// Translated from 00c66170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator::blockAlloc` (Xbox PDB): allocates `size + 16` bytes
/// and returns the first 16-byte-aligned address after reserving a trailing
/// byte for the alignment distance.
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn bhk_memory_allocator_block_alloc(e: &mut Engine, _this: Ptr, size: u32) -> Ptr {
    let raw = e
        .call(HEAP_ALLOC, &args![HEAP_SINGLETON, size.wrapping_add(0x10)])
        .u32();
    let padding = 0x10 - (raw & 0xf);
    let aligned = raw.wrapping_add(padding);
    e.mem.set_u8(aligned - 1, padding as u8);
    Ptr::new(aligned)
}

// Translated from 00c661a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator::blockFree` (Xbox PDB): recovers the raw allocation
/// from the byte immediately before `block`; the second stack word is unused.
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn bhk_memory_allocator_block_free(e: &mut Engine, _this: Ptr, block: Ptr, _unused_1: u32) {
    if !block.is_null() {
        let padding = e.mem.u8(block.addr() - 1) as u32;
        e.call(
            HEAP_FREE,
            &args![HEAP_SINGLETON, block.addr().wrapping_sub(padding)],
        );
    }
}

// Translated from 00c661c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the second of its two stack arguments; `RET 8` confirms both words
/// are part of the function's ABI even though the first is not read.
/// `this` (`ECX`) is not read, but it sits in vtables and is called with `ECX` set.
pub fn bhk_world_return_second_argument(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    value: u32,
) -> u32 {
    value
}

// Translated from 00c661d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the physics update mode global to 1 for argument 0, 3 for argument
/// 4, and 2 for every other value.
pub fn bhk_world_set_update_mode(e: &mut Engine, mode: u32) {
    let value = match mode {
        0 => 1,
        4 => 3,
        _ => 2,
    };
    e.set_global(GLOBAL_WORLD_MODE, value);
}

// Translated from 00c66210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the world's integer and float step accumulators.
pub fn bhk_world_reset_step_accumulators(e: &mut Engine) {
    e.set_global(GLOBAL_WORLD_INTEGER, 0u32);
    e.set_global(GLOBAL_WORLD_FLOAT, 0.0f32);
}

// Translated from 00c66230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetPickCountOverride` (Xbox PDB): writes the override byte.
pub fn bhk_world_set_pick_count_override(e: &mut Engine, enabled: u8) {
    e.set_global(GLOBAL_PICK_COUNT_OVERRIDE, enabled);
}

// Translated from 00c66240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SethkObject` (Xbox PDB): a virtual method (`this` in `ECX`, one stack word, the
/// Havok object) that is a tail `JMP` to `bhkRefObject::SethkObject` (`00c87c50`), so the callee
/// gets `this` and the same stack word.
pub fn bhk_world_set_hk_object(e: &mut Engine, this: Ptr<BhkWorld>, hk_object: u32) {
    e.call(0x00c8_7c50, &args![this, hk_object]);
}

// Translated from 00c66300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the counter at `this + 0x18`.
pub fn bhk_world_increment_batch_add(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x18, count.wrapping_add(1));
}

// Translated from 00c66310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the counter at `this + 0x1C`.
pub fn bhk_world_increment_batch_remove(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0x1c);
    e.mem.set_u32(this.addr() + 0x1c, count.wrapping_add(1));
}

// Translated from 00c66320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::setMul` (Xbox PDB): stores the product of `left` and
/// `right` in `this`, using the PC build's SSE single-precision operations.
// Translation incomplete: 00c66320 - the /GS security-cookie check (00ec408c) is not translated.
pub fn hk_quaternion_set_mul(
    e: &mut Engine,
    this: Ptr<HkQuaternion>,
    left: Ptr<HkQuaternion>,
    right: Ptr<HkQuaternion>,
) {
    let [lx, ly, lz, lw] = [
        e.get(left, HkQuaternion::x),
        e.get(left, HkQuaternion::y),
        e.get(left, HkQuaternion::z),
        e.get(left, HkQuaternion::w),
    ];
    let [rx, ry, rz, rw] = [
        e.get(right, HkQuaternion::x),
        e.get(right, HkQuaternion::y),
        e.get(right, HkQuaternion::z),
        e.get(right, HkQuaternion::w),
    ];
    let result = [
        rw * lx + lw * rx + (rz * ly - ry * lz),
        rw * ly + lw * ry + (rx * lz - rz * lx),
        rw * lz + lw * rz + (ry * lx - rx * ly),
        rw * lw - (rz * lz + ry * ly + rx * lx),
    ];
    e.set(this, HkQuaternion::x, result[0]);
    e.set(this, HkQuaternion::y, result[1]);
    e.set(this, HkQuaternion::z, result[2]);
    e.set(this, HkQuaternion::w, result[3]);
}

// Translated from 00c663d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a referenced-object pointer: decrements the old object's count,
/// deletes it at zero, then copies and increments the new object's count.
/// The source word is read once for the compare and again after the release,
/// so the stored and retained pointer is the one the source holds by then.
pub fn hk_ref_counted_pointer_assign(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let old = e.mem.u32(this.addr());
    // 00c663db: `CMP EDI,[EBX]`, the source word is only compared here.
    if old != e.mem.u32(source.addr()) {
        if old != 0 {
            let count = e.call(0x00fd_f1c4, &args![old + 4]).i32();
            if count == 0 {
                e.vcall(old, 4, &[]);
            }
        }
        // 00c663fa: `MOV EAX,[EBX]` reads the source word again, after the release.
        let new = e.mem.u32(source.addr());
        e.mem.set_u32(this.addr(), new);
        if new != 0 {
            e.call(0x00fd_f1c0, &args![new + 4]);
        }
    }
    this
}

// Translated from 00c66420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidableCollidableFilter` deleting destructor (Xbox PDB).
pub fn hkp_collidable_collidable_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b80);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRayCollidableFilter` deleting destructor (Xbox PDB).
pub fn hkp_ray_collidable_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b8c);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionFilter` deleting destructor (Xbox PDB).
pub fn hkp_shape_collection_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3b98);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c664b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRayShapeCollectionFilter` deleting destructor (Xbox PDB).
pub fn hkp_ray_shape_collection_filter_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3bac);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c664e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhaseCastCollector` deleting destructor (Xbox PDB).
pub fn hkp_broad_phase_cast_collector_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_3bb8);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ahkpWorld` deleting destructor (Xbox PDB): installs its vtable,
/// destroys the `hkpWorld` base, and conditionally releases `this` through
/// the Havok memory router's vtable slot `+8` with the object's 16-bit size.
pub fn ahkp_world_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), HAVOK_WORLD_VTABLE);
    e.call(HAVOK_WORLD_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(0x00c8_5750, &[]).u32();
        let router_allocator = e.mem.u32(router + 0x10);
        let size = e.mem.u16(this.addr() + 4) as u32;
        e.vcall(router_allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c66540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMemoryAllocator` deleting destructor (Xbox PDB): writes its vtable,
/// calls the base allocator cleanup, and returns `this`; the stack flag is
/// unused by this body.
pub fn bhk_memory_allocator_delete(e: &mut Engine, this: Ptr, _unused_0: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ALLOCATOR_VTABLE);
    e.call(MEMORY_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00c66560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator` deleting destructor (Xbox PDB): calls the base
/// allocator cleanup and returns `this`; its stack flag is unused.
pub fn hk_memory_allocator_delete(e: &mut Engine, this: Ptr, _unused_0: u32) -> Ptr {
    e.call(MEMORY_ALLOCATOR_DESTRUCTOR, &args![this]);
    this
}

// Translated from 00c66570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Performs the thread-local Havok startup callback at `01268450` with
/// argument 1, temporarily setting the TLS Havok memory state to 0x10.
/// Structured exception unwinding is not modeled.
// Translation incomplete: 00c66570 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn bhk_on_startup_callback_invoke(e: &mut Engine) {
    let tls = e.tls();
    let previous = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    let callback = e.global::<Ptr>(0x0126_8450);
    if !callback.is_null() {
        e.vcall(callback.addr(), 8, &args![1u32]);
    }
    e.mem.set_u32(tls + 0x2b4, previous);
}

// Translated from 00c66600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOnStartupCallback` deleting destructor (Xbox PDB).
pub fn bhk_on_startup_callback_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010a_47b8);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00c66630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 8 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_8(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 8, flags]).ptr()
}

// Translated from 00c66640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 12 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_12(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0xc, flags]).ptr()
}

// Translated from 00c66650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 16 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_16(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0x10, flags]).ptr()
}

// Translated from 00c66660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjustor thunk: subtracts 20 from the `this` subobject and tail-calls
/// `00dff470`; the byte flag is forwarded unchanged.
pub fn bhk_on_startup_callback_delete_adjustor_20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f470, &args![this.addr() - 0x14, flags]).ptr()
}

// Translated from 00c66670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears down the Havok globals in the order used by the executable.
/// Structured exception unwinding is not modeled.
// Translation incomplete: 00c66670 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn bhk_world_shutdown(e: &mut Engine) {
    bhk_on_startup_callback_invoke(e);
    e.call(0x00c6_d340, &[]);
    e.call(0x00c6_d3c0, &[]);
    let collision_filter = e.global::<Ptr>(0x0126_77a8);
    if !collision_filter.is_null() {
        e.call(0x00c9_05b0, &args![collision_filter]);
        e.set_global(0x0126_77a8, 0u32);
    }
    let data = e.global::<Ptr>(0x0126_77d0);
    if !data.is_null() {
        e.call(0x00cb_f430, &args![data]);
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, data]);
    }
    e.call(0x00cb_bcb0, &[]);
    let callback = e.global::<Ptr>(0x0126_77d4);
    e.vcall(callback.addr(), 8, &args![3u32]);
    e.vcall(callback.addr(), 0, &args![0u32]);
    e.set_global(0x0126_77d4, 0u32);
    e.call(0x00cb_bfb0, &args![0u32]);
}

/// The step accumulator's globals: the frame delta as given (`011afe68`), the delta this step
/// uses (`011afe64`), what is carried to the next frame (`012677b0`), the step count
/// (`012677b4`), the step size (`012677ac`), the update mode (`012677b8`), the forced-step
/// flag (`01267b4c`), the largest delta (`010c3c28`), the skip threshold (`010c3c24`), the
/// fixed step (`01267b38`) and its scale (`011ac3a0`), the smoothed step and its rate
/// (`011afe6c`, `011afe70`), the constants 0.5 (`01011588`, a double) and 2^32 (`01099378`, the
/// unsigned-conversion fix-up), the setting the carried delta is scaled by (`01267b34`, its
/// value through `00403e20`, `thiscall`) and the scaling (`0040ebd0`, `cdecl`: two floats,
/// the answer in ST0).
const STEP_FRAME_DELTA: u32 = 0x011a_fe68;
const STEP_DELTA: u32 = 0x011a_fe64;
const STEP_CARRY: u32 = 0x0126_77b0;
const STEP_COUNT: u32 = 0x0126_77b4;
const STEP_SIZE: u32 = 0x0126_77ac;
const STEP_MODE: u32 = 0x0126_77b8;
const STEP_FORCE: u32 = 0x0126_7b4c;
const STEP_MAX_DELTA: u32 = 0x010c_3c28;
const STEP_SKIP_THRESHOLD: u32 = 0x010c_3c24;
const STEP_FIXED: u32 = 0x0126_7b38;
const STEP_FIXED_SCALE: u32 = 0x011a_c3a0;
const STEP_SMOOTHED: u32 = 0x011a_fe6c;
const STEP_SMOOTH_RATE: u32 = 0x011a_fe70;
const STEP_HALF: u32 = 0x0101_1588;
const STEP_TWO_TO_32: u32 = 0x0109_9378;
const STEP_CARRY_SETTING: u32 = 0x0126_7b34;
const SETTING_VALUE: u32 = 0x0040_3e20;
const STEP_CARRY_SCALE: u32 = 0x0040_ebd0;

/// `FCOM`/`FCOMP` then `FNSTSW AX; TEST AH,0x41`: whether C0 or C3 is set, that is `st0` below
/// or equal to `src` or the two unordered (a NaN).
fn fcom_below_or_equal(st0: f64, src: f64) -> bool {
    !(st0 > src)
}

/// `FILD dword` of an unsigned count, then (when its sign bit is set, `JGE` on the earlier
/// `TEST EAX,EAX`) `FADD dword [01099378]` (2^32).
fn fild_unsigned(e: &Engine, count: u32) -> f64 {
    let value = f64::from(count as i32);
    if (count as i32) < 0 {
        x87_add(value, x87_m32(e.global::<u32>(STEP_TWO_TO_32)))
    } else {
        value
    }
}

/// The compiler's truncating `FISTP qword` (`FNSTCW`, `OR 0xC00`, `FLDCW`, `FISTP`, `FLDCW`):
/// the low word of the 64-bit result (the integer indefinite's low word is 0).
fn fistp_chop_low(value: f64) -> u32 {
    x87_fistp64_chop(X87Ext::from(value)) as u32
}

// Translated from 00c66760 (disassembled, FalloutNV.exe 1.4.0.525)
/// The Havok step accumulator (no label; `cdecl`: the frame delta (a float), a single-step
/// byte and a forced byte), x87 instruction by instruction: each operation through
/// `crate::x87` at the running thread's precision (the main thread runs at 24-bit precision
/// here, once Direct3D has made the device), every float store rounded and every compare's
/// flags tested as the exe tests them (a NaN is unordered: `TEST AH,0x41` then sees C0 and C3).
///
/// The delta is stored to `011afe68`; the carried delta plus the delta is stored to `011afe64`
/// and read back. Above the largest delta (`010c3c28`), the largest delta is stored there and
/// read back; otherwise a value equal to or below 0 sets the count and step size to 0 and ends
/// (a NaN goes on). At most 3 steps (1 with the single-step byte); the mode (`012677b8`) picks:
/// - 0 (fixed steps): the step size is fixed step * scale (stored, read back); the count is
///   delta / step size truncated (its 64-bit `FISTP`'s low word), capped. A count: the carry
///   becomes delta - size * count (stored), the step's delta delta - the stored carry, then
///   the carry is scaled (`0040ebd0` with the carry and the value of setting `01267b34`). No
///   step: unless forced, a delta below half the size (0.5 * size compared with it) is carried
///   whole and the step's delta and size set to 0; otherwise one step of the whole delta.
/// - 2: one step of the frame delta as given.
/// - 10 (smoothed): the smoothed step moves toward the delta by its rate ((delta - smoothed)
///   * rate + smoothed, stored, read back) and is the one step's size; the carry 0.
/// - any other: unless forced, a delta below the skip threshold (`010c3c24`) is carried whole
///   (count, step delta and size 0); otherwise the carry is 0 and the count delta / fixed step
///   truncated, capped; no count gives one step of the whole delta, a count steps of delta /
///   count.
pub fn bhk_world_accumulate_step(e: &mut Engine, delta: u32, single_step: u8, forced: u8) {
    let d = x87_load32(delta);
    e.set_global(STEP_FRAME_DELTA, x87_store32(d));
    let force_step = forced != 0 || e.global::<u8>(STEP_FORCE) != 0;
    let carry = x87_load32(e.global::<u32>(STEP_CARRY));
    e.set_global(STEP_DELTA, x87_store32(x87_add(carry, d)));
    let max_delta = x87_load32(e.global::<u32>(STEP_MAX_DELTA));
    let summed = x87_load32(e.global::<u32>(STEP_DELTA));
    let accumulated = if fcom_below_or_equal(summed, max_delta) {
        // 00c667e3: FCOM with 0, `JP` taken for "above" (no flag) and "unordered" (both).
        if summed > 0.0 || summed.is_nan() {
            summed
        } else {
            e.set_global(STEP_COUNT, 0u32);
            e.set_global(STEP_SIZE, x87_store32(0.0));
            return;
        }
    } else {
        e.set_global(STEP_DELTA, x87_store32(max_delta));
        x87_load32(e.global::<u32>(STEP_DELTA))
    };
    let max_steps: u32 = if single_step == 0 { 3 } else { 1 };
    // The jump table (00c669d8, by the byte map at 00c669ec): 0, 2 and 10 have their own
    // cases; 1, 3..=9 and anything above 10 share the last one.
    match e.global::<u32>(STEP_MODE) {
        0 => {
            let fixed = x87_load32(e.global::<u32>(STEP_FIXED));
            let size = x87_mul(fixed, x87_m32(e.global::<u32>(STEP_FIXED_SCALE)));
            e.set_global(STEP_SIZE, x87_store32(size));
            let size = x87_load32(e.global::<u32>(STEP_SIZE));
            let quotient = x87_div(accumulated, size);
            let mut count = fistp_chop_low(quotient);
            e.set_global(STEP_COUNT, count);
            if count > max_steps {
                count = max_steps;
                e.set_global(STEP_COUNT, count);
            }
            if count != 0 {
                let product = x87_mul(size, fild_unsigned(e, count));
                let carried = x87_sub(accumulated, product);
                e.set_global(STEP_CARRY, x87_store32(carried));
                let used = x87_sub(accumulated, x87_m32(e.global::<u32>(STEP_CARRY)));
                e.set_global(STEP_DELTA, x87_store32(used));
                let setting = e.call(SETTING_VALUE, &args![STEP_CARRY_SETTING]).u32();
                let value = x87_store32(x87_load32(e.mem.u32(setting)));
                let carried = x87_store32(x87_load32(e.global::<u32>(STEP_CARRY)));
                let scaled = e.call(STEP_CARRY_SCALE, &args![carried, value]).f64();
                e.set_global(STEP_CARRY, x87_store32(scaled));
                return;
            }
            if !force_step {
                let half = x87_mul(size, e.global::<f64>(STEP_HALF));
                if !fcom_below_or_equal(half, accumulated) {
                    e.set_global(STEP_CARRY, x87_store32(accumulated));
                    e.set_global(STEP_DELTA, x87_store32(0.0));
                    e.set_global(STEP_SIZE, x87_store32(0.0));
                    return;
                }
            }
            // 00c6691d, then 00c66828.
            e.set_global(STEP_SIZE, x87_store32(accumulated));
            e.set_global(STEP_COUNT, 1u32);
            e.set_global(STEP_CARRY, x87_store32(0.0));
        }
        2 => {
            e.set_global(STEP_COUNT, 1u32);
            e.set_global(STEP_SIZE, x87_store32(d));
        }
        10 => {
            let smoothed = x87_load32(e.global::<u32>(STEP_SMOOTHED));
            let difference = x87_sub(accumulated, smoothed);
            let rate = x87_load32(e.global::<u32>(STEP_SMOOTH_RATE));
            let moved = x87_add(x87_mul(difference, rate), smoothed);
            e.set_global(STEP_SMOOTHED, x87_store32(moved));
            let moved = x87_load32(e.global::<u32>(STEP_SMOOTHED));
            e.set_global(STEP_SIZE, x87_store32(moved));
            e.set_global(STEP_COUNT, 1u32);
            e.set_global(STEP_CARRY, x87_store32(0.0));
        }
        _ => {
            if !force_step {
                let threshold = x87_load32(e.global::<u32>(STEP_SKIP_THRESHOLD));
                if !fcom_below_or_equal(threshold, accumulated) {
                    e.set_global(STEP_COUNT, 0u32);
                    e.set_global(STEP_CARRY, x87_store32(accumulated));
                    e.set_global(STEP_DELTA, x87_store32(0.0));
                    e.set_global(STEP_SIZE, x87_store32(0.0));
                    return;
                }
            }
            e.set_global(STEP_CARRY, x87_store32(0.0));
            let quotient = x87_div(accumulated, x87_m32(e.global::<u32>(STEP_FIXED)));
            let mut count = fistp_chop_low(quotient);
            e.set_global(STEP_COUNT, count);
            if count > max_steps {
                count = max_steps;
                e.set_global(STEP_COUNT, count);
            }
            if count == 0 {
                e.set_global(STEP_COUNT, 1u32);
                e.set_global(STEP_SIZE, x87_store32(accumulated));
                return;
            }
            let size = x87_div(accumulated, fild_unsigned(e, count));
            e.set_global(STEP_SIZE, x87_store32(size));
        }
    }
}

// Translated from 00c66a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the active solver's tau and damping according to the current
/// update mode, then updates the step count and reciprocal.
pub fn bhk_world_scale_solver_info(e: &mut Engine, this: Ptr) {
    const MODE: u32 = 0x0126_77b8;
    const STEP_SIZE: u32 = 0x0126_77ac;
    const FREQUENCY: u32 = 0x0126_7b38;
    const MINIMUM: u32 = 0x0126_7b14;
    const MAXIMUM: u32 = 0x0126_7b20;
    const ORIGINAL_TAU: u32 = 0x011a_fe60;
    const TAU: u32 = 0x0126_7b50;
    let world = if this.is_null() {
        0
    } else {
        e.vcall(this.addr(), 0x94, &[]).u32()
    };
    if e.global::<u32>(MODE) != 2 {
        // 00c66a28..00c66a38: `FLD step; FDIV frequency; FSTP [ratio]`, then `FLD [ratio]` stays
        // on the x87 stack to the end of the branch (`ftol` pops only its own argument): it is
        // the `t` of the tau blend below. The decompiler shows it as the value left in ST0 by
        // the `ftol` call, which is the step over the frequency, not the clamped step count's
        // float.
        let ratio_bits = x87_store32(x87_div(
            x87_load32(e.global::<u32>(STEP_SIZE)),
            x87_m32(e.global::<u32>(FREQUENCY)),
        ));
        let ratio = x87_load32(ratio_bits);
        // 00c66a3c..00c66a5b: `FLD 4.0; FMUL ST1` (D8), the product stored and reloaded, kept
        // unless it is below the double 2.0 (`FCOM`, `TEST AH,5; JP`), which the float 2.0 at
        // `010162c0` then replaces.
        let fourfold = x87_load32(x87_store32(x87_mul(ratio, e.global::<f64>(0x0101_db80))));
        let wanted = if fourfold < e.global::<f64>(0x0101_1590) {
            x87_load32(e.global::<u32>(0x0101_62c0))
        } else {
            fourfold
        };
        let wanted = x87_load32(x87_store32(wanted));
        let step_count = e.call(0x00ec_62c0, &args![wanted]).i32();
        e.mem.set_i32(world + 0x2f4, step_count);
        e.mem.set_u32(
            world + 0x300,
            x87_store32(x87_div(1.0, f64::from(step_count))),
        );
        let original_tau = x87_store32(x87_load32(e.global::<u32>(ORIGINAL_TAU)));
        e.set_global(TAU, original_tau);
        // 00c66a97..00c66aaf: `FLD [tau]; FLD ST0; FSUBP` leaves `1 - g`; `FLD 0.6; FMUL ST2` is
        // `DC CA`: `(1 - g) * 0.6` goes to ST2 and 0.6 stays on top; `FMULP ST3` makes `t * g` and
        // `FMULP ST2` multiplies it by 0.6; `FADDP` sums: `(1 - g) * 0.6 + (t * g) * 0.6`.
        let g = x87_load32(e.global::<u32>(TAU));
        let k = e.global::<f64>(0x0102_90b8);
        let blended = x87_add(x87_mul(x87_sub(1.0, g), k), x87_mul(x87_mul(ratio, g), k));
        let tau_bits = x87_store32(blended);
        e.mem.set_u32(world + 0x1e4, tau_bits);
        // `FLD [tau]` and `FDIV ST0,ST1` with the damping: `damping / tau`; then
        // `FDIV float [damping]`: `tau / damping`.
        let tau = x87_load32(tau_bits);
        let damping_bits = e.mem.u32(world + 0x1e8);
        e.mem.set_u32(
            world + 0x230,
            x87_store32(x87_div(x87_load32(damping_bits), tau)),
        );
        e.mem.set_u32(
            world + 0x234,
            x87_store32(x87_div(tau, x87_m32(damping_bits))),
        );
        return;
    }

    let ratio = e.global::<f32>(0x011a_fe68) / e.global::<f32>(MINIMUM);
    let rounded = e.call(0x00ec_9e10, &args![ratio as f64]).f32();
    let mut tau = e.global::<f32>(MINIMUM);
    let mut scale = rounded;
    if scale < e.global::<f32>(MAXIMUM) {
        tau = e.global::<f32>(0x011a_fe68) / e.global::<f32>(MAXIMUM);
        scale = e.global::<f32>(MAXIMUM);
    }
    let tau_field = e.mem.f32(world + 0x1e4);
    e.mem.set_f32(this.addr() + 0x88, tau_field);
    let damping_field = e.mem.f32(world + 0x1e8);
    e.mem.set_f32(this.addr() + 0x8c, damping_field);
    let ratio = e.global::<f32>(0x011a_fe68) / (tau * scale);
    let tau_new = ratio - 1.0 + tau_field;
    let damping_new = (damping_field as f64 + ratio as f64 - 1.0) as f32;
    e.mem.set_f32(world + 0x1e4, tau_new);
    e.mem.set_f32(world + 0x1e8, damping_new);
    let step_count = e.call(0x00cb_fef0, &args![tau_new]).i32();
    e.mem.set_i32(world + 0x2f4, step_count);
    e.mem.set_f32(world + 0x300, 1.0 / step_count as f32);
}

// Translated from 00c66be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the vector at `input` into this world's `Origin` vector by calling
/// `fn_00458620` (`hkVector4` scale helper).
pub fn bhk_world_scale_origin(e: &mut Engine, this: Ptr<BhkWorld>, input: Ptr) {
    e.call(0x0045_8620, &args![input, this.addr() + 0x60]);
}

// Translated from 00c66c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shifts the Havok broad phase to the requested origin and stores the
/// resulting origin on the `bhkWorld`.
// Translation incomplete: 00c66c00 - the /GS security-cookie check (00ec408c) is not translated.
pub fn bhk_world_shift_broad_phase(e: &mut Engine, this: Ptr<BhkWorld>, requested: Ptr) {
    let target = e.mem.bytes(requested.addr(), 16);
    let origin = e.mem.bytes(this.addr() + 0x60, 16);
    let mut delta = [0.0f32; 4];
    for i in 0..4 {
        let a = f32::from_le_bytes(target[i * 4..i * 4 + 4].try_into().unwrap());
        let b = f32::from_le_bytes(origin[i * 4..i * 4 + 4].try_into().unwrap());
        delta[i] = a - b;
    }
    e.with_stack(32, |e, vectors| {
        let delta_bytes: Vec<u8> = delta.into_iter().flat_map(f32::to_le_bytes).collect();
        e.mem.write(vectors.addr(), &delta_bytes);
        let output = vectors.addr() + 16;
        let world = e.vcall(this.addr(), 0x94, &[]).u32();
        e.call(0x00c9_6a20, &args![world, vectors.addr(), output, 0u32]);
        let shifted = e.mem.bytes(output, 16);
        e.vcall(this.addr(), 0x94, &[]);
        let mut new_origin = [0u8; 16];
        for i in 0..4 {
            let prior = f32::from_le_bytes(origin[i * 4..i * 4 + 4].try_into().unwrap());
            let moved = f32::from_le_bytes(shifted[i * 4..i * 4 + 4].try_into().unwrap());
            new_origin[i * 4..i * 4 + 4].copy_from_slice(&(prior + moved).to_le_bytes());
        }
        e.mem.write(this.addr() + 0x60, &new_origin);
    });
}

// Translated from 00c66c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Init` (Xbox PDB): allocates and constructs the 0x360-byte
/// Havok world, installs its game vtable, attaches the optional collision
/// filter, then invokes the world's setup virtuals. The trailing stack word
/// is the construction-info pointer. Structured exception unwinding is not modeled.
// Translation incomplete: 00c66c90 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn bhk_world_init(e: &mut Engine, this: Ptr<BhkWorld>, cinfo: Ptr) {
    let tls = e.tls();
    let saved_tls = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    let router = e.call(0x00c8_5750, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let havok_world = e.vcall(allocator, 4, &args![0x360u32]).u32();
    e.mem.set_u16(havok_world + 4, 0x360);
    let cinfo_base: Ptr = if cinfo.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(cinfo.addr() - 0xf0)
    };
    e.call(0x00c9_5a80, &args![havok_world, cinfo_base, 0x111d4u32]);
    e.mem.set_u32(havok_world, HAVOK_WORLD_VTABLE);
    e.mem.set_u32(havok_world + 0x350, 0);
    let filter = e.global::<u32>(0x0126_77a8);
    if filter != 0 {
        e.call(0x00c9_77c0, &args![havok_world, filter, 1u32, 0u32, 1u32]);
    }
    e.vcall(this.addr(), 0x8c, &args![havok_world]);
    e.call(0x00c9_05b0, &args![havok_world]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    let world = e.vcall(this.addr(), 0x94, &[]).u32();
    if world != 0 {
        e.call(0x00cb_fd20, &args![e.mem.u32(world + 0x80)]);
    }
    e.mem.set_u32(tls + 0x2b4, saved_tls);
}

// Translated from 00c66de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Link` (Xbox PDB): after linking the base reference object,
/// stores `this` in the linked Havok world's `+0x350` field when bit 0 is set.
pub fn bhk_world_link(e: &mut Engine, this: Ptr<BhkWorld>, linked: u32) {
    if !this.is_null() {
        let world = e.vcall(this.addr(), 0x94, &[]).u32();
        if world != 0 {
            e.mem.set_u32(
                world + 0x350,
                if linked & 0xff != 0 { this.addr() } else { 0 },
            );
        }
    }
    e.call(0x00c8_7c90, &args![this, linked]);
}

// Translated from 00c66e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot `+0x18` on every non-null entry in `pEntityListenerA` when the
/// world's Havok object exists and `value` is nonzero.
pub fn bhk_world_set_solver_value(e: &mut Engine, this: Ptr<BhkWorld>, value: f32) {
    if e.mem.u32(this.addr() + 8) == 0 || value == 0.0 {
        return;
    }
    let items = e.get(this, BhkWorld::entityListeners).addr();
    let count = e.get(this, BhkWorld::entityListenerCount);
    for i in 0..count.max(0) as u32 {
        let listener = e.mem.u32(items + i * 4);
        if listener != 0 {
            e.vcall(listener, 0x18, &args![value]);
        }
    }
}

// Translated from 00c66e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::KillCinfo` (Xbox PDB): when requested, destroys the
/// `hkSerializableCinfo` base at `pInfo - 0xF0` and clears `pInfo`.
/// Structured exception unwinding is not modeled.
// Translation incomplete: 00c66e70 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn bhk_world_kill_cinfo(e: &mut Engine, this: Ptr<BhkWorld>, destroy: u8) {
    let tls = e.tls();
    let saved_tls = e.mem.u32(tls + 0x2b4);
    e.mem.set_u32(tls + 0x2b4, 0x10);
    if destroy != 0 {
        let info = e.get(this, BhkWorld::pInfo);
        if !info.is_null() {
            let base = info.addr() - 0xf0;
            e.vcall(base, 0, &args![1u32]);
        }
        e.set(this, BhkWorld::pInfo, Ptr::NULL);
    }
    e.mem.set_u32(tls + 0x2b4, saved_tls);
}

// Translated from 00c66f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a collision filter's 16-bit group when empty. The group is taken
/// from the collidable's filter-info table, or from the wrapping global
/// counter (which skips zero and resumes at 10); existing groups are reused.
pub fn bhk_world_assign_collision_group(e: &mut Engine, collidable: Ptr, filter: Ptr) -> bool {
    let world = e.mem.u32(collidable.addr() + 8);
    let filter_info = if world == 0 || world == 0xffff_fff0 {
        0
    } else {
        e.mem.u32(world + 0x2c)
    };
    let encoded_group = filter_info >> 16;
    let existing = e.mem.u32(filter.addr() + 0x10);
    // 00c66f82 / 00c66f72: `00c85ce0` (RET 4) is a thiscall on the collidable (ECX is
    // the first argument, loaded at 00c66f10 and not changed).
    if existing != 0 {
        e.call(0x00c8_5ce0, &args![collidable, existing]);
        return true;
    }
    let group = if encoded_group != 0 {
        encoded_group
    } else {
        let index = filter_info & 0x7f;
        let configured = e.mem.u32(0x0126_7e70 + index * 4);
        if configured != 0 {
            configured
        } else {
            let next = e.global::<u32>(0x011b_0808).wrapping_add(1) & 0xffff;
            let next = if next == 0 { 10 } else { next };
            e.set_global(0x011b_0808, next);
            next
        }
    };
    e.mem.set_u32(filter.addr() + 0x10, group);
    if encoded_group == 0 {
        e.call(0x00c8_5ce0, &args![collidable, group]);
    }
    // 00c66f78: AL is BL, set only on the paths that call 00c85ce0.
    encoded_group == 0
}

// Translated from 00c66f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::HasActiveItems` (Xbox PDB): true when the `hkpWorld` returned
/// by slot `+0x94` has a nonzero active-island count at `+0x2C`.
pub fn bhk_world_has_active_items(e: &mut Engine, this: Ptr<BhkWorld>) -> bool {
    let world = e.vcall(this.addr(), 0x94, &[]).u32();
    world != 0 && e.mem.u32(world + 0x2c) != 0
}

// Translated from 00c66fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData::GetNiAVObject` (Xbox PDB): resolves the root collidable
/// through `bhkUtilFunctions::GetNiAVObject` (`00c7fa90`), or returns null.
pub fn bhk_pick_data_get_ni_av_object(e: &mut Engine, this: Ptr<BhkPickData>) -> Ptr {
    let collidable = e.get(this, BhkPickData::rootCollidable);
    if collidable.is_null() {
        Ptr::NULL
    } else {
        e.call(0x00c7_fa90, &args![collidable]).ptr::<()>()
    }
}

// Translated from 00c66fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData::GethkRigidBody` (Xbox PDB): returns the body subobject
/// inside `m_rootCollidable` when its byte at `+0x18` equals 1.
pub fn bhk_pick_data_get_hk_rigid_body(e: &mut Engine, this: Ptr<BhkPickData>) -> Ptr {
    let collidable = e.get(this, BhkPickData::rootCollidable);
    if collidable.is_null() || e.mem.u8(collidable.addr() + 0x18) != 1 {
        return Ptr::NULL;
    }
    Ptr::new(
        collidable
            .addr()
            .wrapping_add_signed(e.mem.i8(collidable.addr() + 0x10) as i32),
    )
}

// Translated from 00c66ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the Havok collision properties held by `filter` to the collidable
/// held by `agent`, including collision flags, world links, zeroed contact
/// data, and the activation call. All virtual calls retain their PC slots.
pub fn bhk_collision_agent_update(e: &mut Engine, agent: Ptr, filter: Ptr) {
    let collidable = e.mem.u32(agent.addr() + 0x10);
    if collidable != 0 {
        let property = e.vcall(agent.addr(), 0x3c, &[]).u32();
        if property != 0 {
            // 00c67016: CMP dword ptr [EDI+0x14],0 (the whole word).
            let flags = e.mem.u32(filter.addr() + 0x14);
            let current = e.mem.u16(property + 0xc);
            e.mem.set_u16(
                property + 0xc,
                if flags == 0 {
                    current | 0x100
                } else {
                    current & 0xfeff
                },
            );
        }
        let flags = e.mem.u16(agent.addr() + 0xc);
        if e.mem.u8(filter.addr() + 0xc) & 1 != 0 {
            e.mem.set_u16(agent.addr() + 0xc, flags | 0x0c);
        } else {
            e.mem.set_u16(agent.addr() + 0xc, flags & 0xfffb);
        }
        if e.global::<u8>(0x011a_fe58) != 0 {
            bhk_world_assign_collision_group(e, Ptr::new(collidable), filter);
        }
        let filter_world = e.vcall(e.mem.u32(filter.addr()), 0x94, &[]).u32();
        let object_world = e.vcall(collidable, 0x94, &[]).u32();
        if object_world != filter_world {
            let current = e.mem.u16(agent.addr() + 0xc);
            if e.mem.u32(filter.addr() + 0x18) == 0 {
                e.mem.set_u16(agent.addr() + 0xc, current & 0xffbf);
            } else {
                e.mem.set_u16(agent.addr() + 0xc, current | 0x40);
            }
            e.vcall(collidable, 0x9c, &args![e.mem.u32(filter.addr())]);
        }
        e.with_stack(12, |e, contacts| {
            e.mem.set_f32(contacts.addr(), 0.0);
            e.mem.write(contacts.addr() + 4, &[0; 5]);
            let data_owner = e.mem.u32(agent.addr() + 8);
            e.vcall(data_owner, 0xb8, &args![contacts.addr()]);
        });
        let filter_object = e.mem.u32(filter.addr());
        if e.mem.u8(filter_object + 0x16) != 0 {
            e.vcall(agent.addr(), 0xc0, &args![1u32]);
        }
    }
    let filter_flags = e.mem.u32(filter.addr() + 0xc);
    e.mem.set_u32(filter.addr() + 0xc, filter_flags & !1);
    let count = e.mem.u32(filter.addr() + 0x14);
    e.mem.set_u32(filter.addr() + 0x14, count.wrapping_add(1));
}

// Translated from 00c67320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates a Havok shape's transform from the four-float vector at
/// `transform + 0xC`; scales it by an optional property and skips a redundant
/// near-zero offset before calling shape slots `+0xD4` and `+0xDC`.
// Translation incomplete: 00c67320 - the /GS security-cookie check (00ec408c) is not translated.
pub fn bhk_collision_agent_update_shape_transform(e: &mut Engine, agent: Ptr, transform: Ptr) {
    let material = e.vcall(agent.addr(), 0x3c, &[]).u32();
    let shape_owner = e.mem.u32(agent.addr() + 0x10);
    if shape_owner == 0 {
        return;
    }
    let shape = e.vcall(shape_owner, 0x40, &[]).u32();
    if shape == 0 {
        return;
    }
    let vector = e.mem.u32(transform.addr() + 0xc);
    if vector == 0 {
        return;
    }
    let mut offset = [0.0f32; 4];
    for (i, value) in offset.iter_mut().enumerate() {
        *value = e.mem.f32(vector + i as u32 * 4);
    }
    if material != 0 {
        let scale = e.mem.f32(material + 0x18);
        if scale < 1.0 {
            if scale == 0.0 {
                return;
            }
            let multiplier = e.mem.f32(material + 0x1c);
            for component in &mut offset {
                *component *= multiplier as f64 as f32;
            }
            let zero_vector = [
                e.global::<f32>(0x0126_7e30),
                e.global::<f32>(0x0126_7e34),
                e.global::<f32>(0x0126_7e38),
                e.global::<f32>(0x0126_7e3c),
            ];
            let epsilon = e.global::<f32>(0x0101_7d00);
            if (0..3).all(|i| (offset[i] - zero_vector[i]).abs() <= epsilon) {
                return;
            }
        }
    }
    e.with_stack(32, |e, vectors| {
        e.vcall(shape, 0xd4, &args![vectors.addr() + 16]);
        let mut result = [0u8; 16];
        for i in 0..4 {
            let base = e.mem.f32(vectors.addr() + 16 + i as u32 * 4);
            result[i * 4..i * 4 + 4].copy_from_slice(&(base + offset[i]).to_le_bytes());
        }
        e.mem.write(vectors.addr() + 16, &result);
        e.vcall(shape, 0xdc, &args![vectors.addr() + 16]);
    });
}

// ---------------------------------------------------------------------------
// Batched world operations (`bhkWorld` batch arrays) and the functions around them.

const SCOPE_WORD: u32 = 0x2b4;
const HAVOK_ADD_REFERENCE: u32 = 0x00c9_0510;
const HAVOK_RELEASE: u32 = 0x00c9_05b0;
const MEMSET: u32 = 0x00ec_61c0;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Import slots `00fdf054` / `00fdf058`: Ghidra's decompilation of `0044fae0` / `0044fb00`
/// (one call each) names them `InitializeCriticalSection` / `DeleteCriticalSection`.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT: u32 = 0x011a_fe7c;
const GLOBAL_ISLAND_SPLIT_FLAG: u32 = 0x0126_7b44;
const GLOBAL_ZERO_VECTOR: u32 = 0x0126_7e30;
const GLOBAL_TOLERANCE: u32 = 0x0101_7d00;
const GLOBAL_ABSOLUTE_MASK: u32 = 0x0101_7d10;
const GLOBAL_WORLD_COUNT: u32 = 0x0126_77a4;
const GLOBAL_COLLISION_FILTER: u32 = 0x0126_77a8;
const GLOBAL_MULTITHREADED_SYSTEM: u32 = 0x0126_77d0;
const GLOBAL_MEMORY_SYSTEM: u32 = 0x0126_77d4;
const SLOT_GET_HK_WORLD: u32 = 0x94;

/// Sets the per-thread word at `TLS + 0x2b4` to 0x10 as every Havok-allocating
/// function of this unit does on entry; returns what to restore on exit.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + SCOPE_WORD);
    e.mem.set_u32(tls + SCOPE_WORD, 0x10);
    (tls, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// `GethkWorld` (slot `+0x94`): the `hkpWorld` of the `bhkWorld` at `this`.
fn hk_world(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), SLOT_GET_HK_WORLD, &[]).u32()
}

/// The `hkpWorld` memset the batch flushers end with: clears `count` pointers.
fn clear_pointers(e: &mut Engine, array: u32, count: u32) {
    e.call(MEMSET, &args![array, 0u32, count.wrapping_mul(4)]);
}

/// First three lanes of the vector at `vector` are each within the tolerance
/// of the lanes of the zero vector (`ABS(a - b) <= tolerance`, as SSE
/// `SUBPS`/`ANDPS`/`CMPLEPS` and the `MOVMSKPS & 7` test compute it).
fn near_zero_vector(e: &mut Engine, vector: u32, tolerance: f32) -> bool {
    (0..3u32).all(|lane| {
        let value = e.mem.f32(vector + lane * 4);
        let zero = e.mem.f32(GLOBAL_ZERO_VECTOR + lane * 4);
        let mask = e.mem.u32(GLOBAL_ABSOLUTE_MASK + lane * 4);
        f32::from_bits((value - zero).to_bits() & mask) <= tolerance
    })
}

/// The entity's `hkpRigidBody` collidable owner, resolved as the callers of
/// these functions do: the byte at `+0x28` must be 1 and the signed byte at
/// `+0x20` is a displacement from `entity + 0x10`.
fn entity_owner_target(e: &mut Engine, entity: u32) -> u32 {
    if e.mem.u8(entity + 0x28) != 1 {
        return 0;
    }
    let base = entity.wrapping_add(0x10);
    base.wrapping_add_signed(e.mem.i8(base + 0x10) as i32)
}

// Translated from 00c67430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a stream object from `name` (`00cc04d0`), saves the world's
/// `hkpWorld` into it through `hkpHavokSnapshot::save` (`00cc0d60`) and
/// returns the save routine's result byte. The result byte lives in the
/// function's own argument slot, so it starts out as the low byte of `name`.
// Translation incomplete: 00c67430 - SEH exception unwinding and the stack cookie check are unsupported.
pub fn fn_00c67430(e: &mut Engine, this: Ptr, name: u32) -> bool {
    e.with_stack(0x10, |e, stream| {
        e.call(0x00cc_04d0, &args![stream, name]);
        let stream_field = e.mem.u32(stream.addr() + 8);
        let world = hk_world(e, this);
        e.with_stack(4, |e, result| {
            e.mem.set_u32(result.addr(), name);
            // 00c67493: the cdecl `hkpHavokSnapshot::save` takes six words (ADD ESP,0x18):
            // the three pushed last and, pushed first, 0, 0 and a word whose low byte is set
            // to 0 (its upper bytes are the leftover ECX; the save reads only the low byte,
            // `fn_00cc0b50(.., flag as u8, ..)`).
            e.call(
                0x00cc_0d60,
                &args![result, world, stream_field, 0u32, 0u32, 0u32],
            );
            let saved = e.mem.u8(result.addr()) != 0;
            e.call(0x00cc_00e0, &args![stream]);
            saved
        })
    })
}

// Translated from 00c674d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` entities to the `hkpWorld` as one batch, then, for each:
/// clears bit 4 of the owner's flags (or removes the entity again when it has
/// no owner), splits the simulation island once, activates owners whose
/// `+0x1B0`/`+0x1C0` vectors are not both near zero, and releases the entity.
/// Finally zeroes the pointer array.
// Translation incomplete: 00c674d0 - SEH exception unwinding and the stack cookie check are unsupported.
pub fn fn_00c674d0(e: &mut Engine, this: Ptr, entities: Ptr, count: u32) {
    let scope = enter_scope(e);
    let world = hk_world(e, this);
    if world == 0 {
        leave_scope(e, scope);
        return;
    }
    e.call(0x00c9_4bd0, &args![world, entities, count, 0u32]);
    let mut island_split = false;
    let tolerance = if count != 0 {
        e.global::<f32>(GLOBAL_TOLERANCE)
    } else {
        0.0
    };
    for index in 0..count {
        let entity = e.mem.u32(entities.addr() + index * 4);
        let target = entity_owner_target(e, entity);
        let mut owner = 0;
        if target != 0 {
            owner = e.mem.u32(target + 0xc);
            if owner != 0 {
                let flags = e.mem.u32(owner + 0x10);
                e.mem.set_u32(owner + 0x10, flags & !0x10);
            }
        }
        if owner == 0 {
            e.with_stack(4, |e, out| {
                e.call(0x00c9_1620, &args![world, out, entity]);
            });
        }
        if !island_split {
            let entity = e.mem.u32(entities.addr() + index * 4);
            let island = e.mem.u32(entity + 0xcc);
            if island != 0 && e.mem.u16(island + 0x20) != 0xffff {
                if e.global::<u8>(GLOBAL_ISLAND_SPLIT_FLAG) != 0 {
                    let first = e.mem.u32(island + 0x48);
                    let second = e.mem.u32(island + 0x4c);
                    e.call(0x00c9_2a80, &args![world, first, second]);
                }
                e.call(0x00cb_6e70, &args![world, island]);
                island_split = true;
            }
        }
        let owner_object = if owner != 0 { e.mem.u32(owner + 8) } else { 0 };
        let first_vector = if owner_object != 0 {
            owner_object + 0x1b0
        } else {
            GLOBAL_ZERO_VECTOR
        };
        let mut activate = !near_zero_vector(e, first_vector, tolerance);
        if !activate {
            let second_vector = if owner_object != 0 {
                owner_object + 0x1c0
            } else {
                GLOBAL_ZERO_VECTOR
            };
            activate = !near_zero_vector(e, second_vector, tolerance);
        }
        if activate && owner_object != 0 {
            e.call(0x00c9_c1d0, &args![owner_object]);
        }
        let entity = e.mem.u32(entities.addr() + index * 4);
        e.call(HAVOK_RELEASE, &args![entity]);
    }
    clear_pointers(e, entities.addr(), count);
    leave_scope(e, scope);
}

// Translated from 00c676e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched phantoms (`pPhantomBatch`, at most 200) into the
/// `hkpWorld`, clears each phantom's pending bit 4, resets the property found
/// under key `01267b70` and releases the phantom; then empties the batch.
// Translation incomplete: 00c676e0 - SEH exception unwinding is unsupported.
pub fn fn_00c676e0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iPhantomBatchCount);
    if count != 0 {
        if count >= 200 {
            e.set(this, BhkWorld::iPhantomBatchCount, 200);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let array = e.get(this, BhkWorld::pPhantomBatch).addr();
            let count = e.get(this, BhkWorld::iPhantomBatchCount);
            e.call(0x00c9_54e0, &args![world, array, count]);
            let mut index = 0;
            while index < e.get(this, BhkWorld::iPhantomBatchCount) {
                let array = e.get(this, BhkWorld::pPhantomBatch).addr();
                let phantom = e.mem.u32(array + index * 4);
                let body = if phantom != 0 {
                    e.mem.u32(phantom + 0xc)
                } else {
                    0
                };
                if body != 0 {
                    let flags = e.mem.u8(body + 0x14);
                    e.mem.set_u8(body + 0x14, flags & 0xef);
                } else {
                    e.call(0x00c9_2c70, &args![world, phantom]);
                }
                let array = e.get(this, BhkWorld::pPhantomBatch).addr();
                let phantom = e.mem.u32(array + index * 4);
                let property = e.with_stack(8, |e, slot| {
                    let found = e.call(0x00c8_5ef0, &args![slot, phantom, 0x0126_7b70u32]);
                    e.mem.u32(found.u32())
                });
                if property != 0 {
                    e.with_stack(12, |e, request| {
                        e.mem.write(request.addr(), &[0; 12]);
                        e.vcall(property, 0x90, &args![request]);
                    });
                }
                let array = e.get(this, BhkWorld::pPhantomBatch).addr();
                let phantom = e.mem.u32(array + index * 4);
                e.call(HAVOK_RELEASE, &args![phantom]);
                index += 1;
            }
            let array = e.get(this, BhkWorld::pPhantomBatch).addr();
            let count = e.get(this, BhkWorld::iPhantomBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iPhantomBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched constraints (`pConstraintInstanceBatch`, at most 200)
/// into the `hkpWorld` once no entity batch is pending: each constraint with
/// no world yet whose two entities (when present) already have an island is
/// added, then released; afterwards the batch is emptied.
// Translation incomplete: 00c67850 - SEH exception unwinding is unsupported.
pub fn fn_00c67850(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
    if count != 0 && e.get(this, BhkWorld::iEntityBatchCount) == 0 {
        if count >= 200 {
            e.set(this, BhkWorld::iConstraintInstanceBatchCount, 200);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let mut index = 0;
            while index < e.get(this, BhkWorld::iConstraintInstanceBatchCount) {
                let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
                let constraint = e.mem.u32(array + index * 4);
                if constraint != 0 && e.mem.u32(constraint + 8) == 0 {
                    let first = e.mem.u32(constraint + 0x14);
                    let second = e.mem.u32(constraint + 0x18);
                    if (first == 0 || e.mem.u32(first + 0xcc) != 0)
                        && (second == 0 || e.mem.u32(second + 0xcc) != 0)
                    {
                        e.call(0x00c9_7d30, &args![world, constraint]);
                    }
                }
                let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
                let constraint = e.mem.u32(array + index * 4);
                if constraint != 0 {
                    e.call(HAVOK_RELEASE, &args![constraint]);
                }
                index += 1;
            }
            let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
            let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iConstraintInstanceBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

/// Clears bit 5 of the owner's flags for a removed entity (the same
/// displacement walk as [`entity_owner_target`]).
fn clear_entity_removal_flag(e: &mut Engine, entity: u32) {
    let target = entity_owner_target(e, entity);
    if target != 0 {
        let owner = e.mem.u32(target + 0xc);
        if owner != 0 {
            let flags = e.mem.u32(owner + 0x10);
            e.mem.set_u32(owner + 0x10, flags & !0x20);
        }
    }
}

// Translated from 00c679a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched entity removals (`pEntityRemoveBatch`, at most 3000)
/// out of the `hkpWorld`, clears bit 5 of each owner's flags, releases each
/// entity and empties the batch.
// Translation incomplete: 00c679a0 - SEH exception unwinding is unsupported.
pub fn fn_00c679a0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
    if count != 0 {
        if count >= 3000 {
            e.set(this, BhkWorld::iEntityRemoveBatchCount, 3000);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
            let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
            e.call(0x00c9_51b0, &args![world, array, count]);
            let mut index = 0;
            while index < e.get(this, BhkWorld::iEntityRemoveBatchCount) {
                let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
                let entity = e.mem.u32(array + index * 4);
                clear_entity_removal_flag(e, entity);
                let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
                let entity = e.mem.u32(array + index * 4);
                e.call(HAVOK_RELEASE, &args![entity]);
                index += 1;
            }
            let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
            let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iEntityRemoveBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::RemoveBatchedEntitiesPartial` (Xbox PDB): like the full removal
/// flush but only takes the last `min(count, limit)` entries of the batch
/// (the limit is the global at `011afe7c`); the removed slots are zeroed one
/// by one and the batch count shrinks to the entries left.
// Translation incomplete: 00c67aa0 - SEH exception unwinding is unsupported.
pub fn bhk_world_remove_batched_entities_partial(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
    if count != 0 {
        if count >= 3000 {
            e.set(this, BhkWorld::iEntityRemoveBatchCount, 3000);
        }
        let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
        let taken = e
            .global::<u32>(GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT)
            .min(count);
        let first = count - taken;
        let world = hk_world(e, this.cast());
        if world != 0 {
            let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
            e.call(0x00c9_51b0, &args![world, array + first * 4, taken]);
            let mut index = first;
            while index < e.get(this, BhkWorld::iEntityRemoveBatchCount) {
                let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
                let entity = e.mem.u32(array + index * 4);
                clear_entity_removal_flag(e, entity);
                let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
                let entity = e.mem.u32(array + index * 4);
                e.call(HAVOK_RELEASE, &args![entity]);
                let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
                e.mem.set_u32(array + index * 4, 0);
                index += 1;
            }
            e.set(this, BhkWorld::iEntityRemoveBatchCount, first);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched phantom removals (`pPhantomBatch`, at most 200) out of
/// the `hkpWorld`, clears bit 5 of each phantom's pending flags, releases
/// each phantom and empties the batch.
// Translation incomplete: 00c67bc0 - SEH exception unwinding is unsupported.
pub fn fn_00c67bc0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iPhantomBatchCount);
    if count != 0 {
        if count >= 200 {
            e.set(this, BhkWorld::iPhantomBatchCount, 200);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let array = e.get(this, BhkWorld::pPhantomBatch).addr();
            let count = e.get(this, BhkWorld::iPhantomBatchCount);
            e.call(0x00c9_5800, &args![world, array, count]);
            let mut index = 0;
            while index < e.get(this, BhkWorld::iPhantomBatchCount) {
                let array = e.get(this, BhkWorld::pPhantomBatch).addr();
                let phantom = e.mem.u32(array + index * 4);
                if phantom != 0 {
                    let body = e.mem.u32(phantom + 0xc);
                    if body != 0 {
                        let flags = e.mem.u8(body + 0x14);
                        e.mem.set_u8(body + 0x14, flags & 0xdf);
                    }
                }
                let array = e.get(this, BhkWorld::pPhantomBatch).addr();
                let phantom = e.mem.u32(array + index * 4);
                e.call(HAVOK_RELEASE, &args![phantom]);
                index += 1;
            }
            let array = e.get(this, BhkWorld::pPhantomBatch).addr();
            let count = e.get(this, BhkWorld::iPhantomBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iPhantomBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched action removals (`pActionBatch`, at most 100): every
/// action that has a world (`+8`) is removed from the `hkpWorld`, then
/// released; afterwards the batch is emptied.
// Translation incomplete: 00c67cd0 - SEH exception unwinding is unsupported.
pub fn fn_00c67cd0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iActionBatchCount);
    if count != 0 {
        if count >= 100 {
            e.set(this, BhkWorld::iActionBatchCount, 100);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let mut index = 0;
            while index < e.get(this, BhkWorld::iActionBatchCount) {
                let array = e.get(this, BhkWorld::pActionBatch).addr();
                let action = e.mem.u32(array + index * 4);
                if action != 0 && e.mem.u32(action + 8) != 0 {
                    e.call(0x00c9_7f50, &args![world, action]);
                }
                let array = e.get(this, BhkWorld::pActionBatch).addr();
                let action = e.mem.u32(array + index * 4);
                if action != 0 {
                    e.call(HAVOK_RELEASE, &args![action]);
                }
                index += 1;
            }
            let array = e.get(this, BhkWorld::pActionBatch).addr();
            let count = e.get(this, BhkWorld::iActionBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iActionBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched constraint removals (`pConstraintInstanceBatch`, at
/// most 200): every constraint that has a world (`+8`) is removed from the
/// `hkpWorld`, then released; afterwards the batch is emptied.
// Translation incomplete: 00c67df0 - SEH exception unwinding is unsupported.
pub fn fn_00c67df0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
    if count != 0 {
        if count >= 200 {
            e.set(this, BhkWorld::iConstraintInstanceBatchCount, 200);
        }
        let world = hk_world(e, this.cast());
        if world != 0 {
            let mut index = 0;
            while index < e.get(this, BhkWorld::iConstraintInstanceBatchCount) {
                let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
                let constraint = e.mem.u32(array + index * 4);
                if constraint != 0 && e.mem.u32(constraint + 8) != 0 {
                    e.with_stack(4, |e, out| {
                        e.call(0x00c9_7e80, &args![world, out, constraint]);
                    });
                }
                let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
                let constraint = e.mem.u32(array + index * 4);
                if constraint != 0 {
                    e.call(HAVOK_RELEASE, &args![constraint]);
                }
                index += 1;
            }
            let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
            let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
            clear_pointers(e, array, count);
            e.set(this, BhkWorld::iConstraintInstanceBatchCount, 0);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c67f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a `BSSimpleList<NiPointer<bhkAction>>`-shaped list at `this` (head
/// item at `+0`, next node at `+4`): destroys and frees every following node
/// (`fn_00c68ae0`), then releases the head item.
// Translation incomplete: 00c67f20 - SEH exception unwinding is unsupported.
pub fn fn_00c67f20(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 4) != 0 {
        loop {
            let node = e.mem.u32(this.addr() + 4);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(node + 4, 0);
            let node = e.mem.u32(this.addr() + 4);
            if node != 0 {
                fn_00c68ae0(e, Ptr::new(node));
                e.call(HEAP_FREE, &args![HEAP_SINGLETON, node]);
            }
            e.mem.set_u32(this.addr() + 4, next);
            if next == 0 {
                break;
            }
        }
    }
    let head = e.mem.u32(this.addr());
    if head != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![head + 4]).i32();
        if count == 0 {
            e.vcall(head, 4, &[]);
        }
        e.mem.set_u32(this.addr(), 0);
    }
}

// Translated from 00c67fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of one 0x2C-byte element of the tasklet thread-state array:
/// clears the first word and constructs the embedded object at `+4`
/// (`00c85770`); returns `this`.
pub fn fn_00c67fc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.call(0x00c8_5770, &args![this.addr() + 4]);
    this
}

// Translated from 00c67fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the tasklet thread-state table: initializes the critical
/// section at `this`, constructs the 16 elements of 0x2C bytes at `+0x18`
/// (element constructor `00c67fc0`, destructor `00a29680`) and clears the
/// count at `+0x2D8`; returns `this`.
// Translation incomplete: 00c67fe0 - SEH exception unwinding is unsupported.
pub fn fn_00c67fe0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this]);
    e.call(
        0x00ec_782f,
        &args![
            this.addr() + 0x18,
            0x2cu32,
            0x10u32,
            0x00c6_7fc0u32,
            0x00a2_9680u32
        ],
    );
    e.mem.set_u32(this.addr() + 0x2d8, 0);
    this
}

// Translated from 00c68050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTaskletManagerCallback::bhkThreadState::CreateThreadMemory` (Xbox
/// PDB): under the table's critical section, when fewer than 16 threads have
/// a slot, names and sets up the next element's memory router (`+4`), takes a
/// 0x5000-byte block from its allocator, records the block and the current
/// thread id, and counts the slot.
// Translation incomplete: 00c68050 - SEH exception unwinding is unsupported.
pub fn bhk_tasklet_manager_callback_bhk_thread_state_create_thread_memory(
    e: &mut Engine,
    this: Ptr,
) {
    e.call(ENTER_CRITICAL_SECTION, &args![this]);
    let used = e.mem.i32(this.addr() + 0x2d8);
    if used < 0x10 {
        let scope = enter_scope(e);
        let element = this
            .addr()
            .wrapping_add((used as u32).wrapping_mul(0x2c))
            .wrapping_add(0x18);
        let memory_system = e.call(0x00cb_bfc0, &[]).u32();
        // 00c680cb: the third word is a by-value flags struct (PUSH ECX; MOV [ESP],1), pushed
        // first: threadInit's flags = 1.
        e.vcall(
            memory_system,
            0xc,
            &args![element + 4, 0x010c_3c2cu32, 1u32],
        );
        e.call(0x00cb_b930, &args![element + 4]);
        let allocator = e.mem.u32(element + 0x14);
        let block = e.vcall(allocator, 4, &args![0x5000u32]).u32();
        e.mem.set_u32(element + 0x28, block);
        e.mem.set_u32(element + 8, block);
        e.mem.set_u32(element + 4, block);
        e.mem.set_u32(element + 0xc, block.wrapping_add(0x5000));
        let thread = e.call(GET_CURRENT_THREAD_ID, &[]).u32();
        e.mem.set_u32(element, thread);
        let used = e.mem.i32(this.addr() + 0x2d8);
        e.mem.set_i32(this.addr() + 0x2d8, used.wrapping_add(1));
        leave_scope(e, scope);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![this]);
}

// Translated from 00c68150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the tasklet thread-state table: destroys the 16 elements of
/// 0x2C bytes at `+0x18` (element destructor `00a29680`) and deletes the
/// critical section at `this`.
// Translation incomplete: 00c68150 - SEH exception unwinding is unsupported.
pub fn fn_00c68150(e: &mut Engine, this: Ptr) {
    e.call(
        0x00ec_5fce,
        &args![this.addr() + 0x18, 0x2cu32, 0x10u32, 0x00a2_9680u32],
    );
    e.call(DELETE_CRITICAL_SECTION, &args![this]);
}

// Translated from 00c681b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkOnStartupCallback::operator()` (Xbox PDB): runs `CreateThreadMemory`
/// on the global thread-state table at `01267830`.
pub fn bhk_on_startup_callback_operator(e: &mut Engine) {
    bhk_tasklet_manager_callback_bhk_thread_state_create_thread_memory(e, Ptr::new(0x0126_7830));
}

// Translated from 00c681c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0xF0-byte world construction-info object (vtable
/// `010c3c44`, `bhkWorldCinfo`): runs the base constructor (`00c90b80`), then
/// sets its default vector, scalars and flags from the executable's
/// constants and the debug global at `011afe5c`; returns `this`.
pub fn fn_00c681c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x00c9_0b80, &args![this]);
    let unit = e.mem.u32(0x0101_1d78);
    let second = e.mem.u32(0x011a_fed4);
    e.mem.set_u32(base, 0x010c_3c44);
    e.mem.set_u8(base + 0x29, 0);
    let scalar = e.mem.u32(0x0101_e2bc);
    e.mem.set_u32(base + 0x54, scalar);
    let scalar = e.mem.u32(0x0104_da48);
    e.mem.set_u32(base + 0x24, scalar);
    for (lane, word) in [unit, second, unit, unit].into_iter().enumerate() {
        e.mem.set_u32(base + 0x10 + lane as u32 * 4, word);
    }
    let flag = e.mem.u8(0x011a_fe5c);
    e.mem.set_u8(base + 0xd1, flag);
    e.mem.set_u8(base + 0xa7, 0);
    e.mem.set_u8(base + 0x28, 3);
    let scalar = e.mem.u32(0x0126_7b38);
    e.mem.set_u32(base + 0x68, scalar);
    this
}

/// `NiStream::RegisterLoader` arguments of [`bhk_world_init_system`]:
/// (name string, creator function), in registration order.
const BHK_STREAM_LOADERS: [(u32, u32); 54] = [
    (0x010c_40a0, 0x00c9_fe30), // "bhkBoxShape"
    (0x0109_bf34, 0x00cc_f890), // "bhkConvexListShape"
    (0x010c_408c, 0x00cc_f5d0), // "bhkConvexSweepShape"
    (0x010c_4074, 0x00cc_ea70), // "bhkConvexTransformShape"
    (0x010c_405c, 0x00cc_dd00), // "bhkConvexVerticesShape"
    (0x010c_4048, 0x00cc_d250), // "bhkCylinderShape"
    (0x010c_402c, 0x00cc_b9a0), // "bhkExtendedMeshShapeData"
    (0x010c_4014, 0x00cc_bd70), // "bhkExtendedMeshShape"
    (0x010c_4000, 0x00cb_7920), // "bhkMultiSphereShape"
    (0x010c_3ff0, 0x00ca_6670), // "bhkpMeshShape"
    (0x010c_3fdc, 0x00ca_6670), // "bhkNiTriStripsShape"
    (0x010c_3fc0, 0x00ca_53b0), // "bhkPackedNiTriStripsShape"
    (0x010c_3fa8, 0x00ca_34c0), // "hkPackedNiTriStripsData"
    (0x010c_3f98, 0x00cc_b200), // "bhkPlaneShape"
    (0x010c_3f88, 0x00ca_06d0), // "bhkSphereShape"
    (0x010c_3f74, 0x00cc_aa80), // "bhkTriangleShape"
    (0x010c_3f60, 0x00cc_9e60), // "bhkMoppBvTreeShape"
    (0x010c_3f4c, 0x00c8_c090), // "bhkTransformShape"
    (0x010c_3f3c, 0x00ca_ae30), // "bhkCapsuleShape"
    (0x010c_3f2c, 0x00c8_ae60), // "bhkListShape"
    (0x010c_3f10, 0x00cc_9700), // "bhkBallAndSocketConstraint"
    (0x010c_3ef0, 0x00cc_8dc0), // "bhkBallSocketConstraintChain"
    (0x010c_3edc, 0x00cb_dae0), // "bhkHingeConstraint"
    (0x010c_3ec0, 0x00cc_8520), // "bhkHingeLimitsConstraint"
    (0x010c_3eac, 0x00cc_7e70), // "bhkFixedConstraint"
    (0x010c_3e90, 0x00ca_a1c0), // "bhkLimitedHingeConstraint"
    (0x010c_3e78, 0x00cc_7500), // "bhkPrismaticConstraint"
    (0x010c_3e60, 0x00cc_67a0), // "bhkRagdollConstraint"
    (0x010c_3e44, 0x00cc_60b0), // "bhkRagdollLimitsConstraint"
    (0x010c_3e28, 0x00cc_5930), // "bhkStiffSpringConstraint"
    (0x010c_3e14, 0x00cc_5130), // "bhkWheelConstraint"
    (0x010c_3dfc, 0x00cc_4900), // "bhkBreakableConstraint"
    (0x010c_3de4, 0x00ca_99f0), // "bhkMalleableConstraint"
    (0x010c_3dcc, 0x00cc_42a0), // "bhkAngularDashpotAction"
    (0x010c_3db8, 0x00cc_3b30), // "bhkDashpotAction"
    (0x010c_3da8, 0x00cb_c5e0), // "bhkLiquidAction"
    (0x010c_3d90, 0x00c8_6de0), // "bhkMouseSpringAction"
    (0x010c_3d80, 0x00cc_33a0), // "bhkMotorAction"
    (0x010c_3d64, 0x00cb_84f0), // "bhkOrientHingedBodyAction"
    (0x010c_3d54, 0x00cc_2bc0), // "bhkSpringAction"
    (0x010c_3d44, 0x00c9_f5a0), // "bhkAabbPhantom"
    (0x010c_3a38, 0x00c6_5830), // "bhkCollisionObject"
    (0x010c_3d30, 0x00c8_7fc0), // "bhkPCollisionObject"
    (0x010c_3d18, 0x00cb_7640), // "bhkSPCollisionObject"
    (0x010c_3d00, 0x00c8_1730), // "bhkBlendCollisionObject"
    (0x010c_3cec, 0x00c9_b020), // "bhkBlendController"
    (0x010c_3cdc, 0x00c8_ed70), // "bhkRigidBody"
    (0x010c_3ccc, 0x00cb_39d0), // "bhkRigidBodyT"
    (0x010c_3cb4, 0x00c9_ef30), // "bhkSimpleShapePhantom"
    (0x010c_3c9c, 0x00cc_2410), // "bhkCachingShapePhantom"
    (0x010c_3c8c, 0x00cc_1ef0), // "bhkExtraData"
    (0x010c_3c78, 0x00cb_b0c0), // "bhkRagdollTemplate"
    (0x010c_3c60, 0x00cb_a2d0), // "bhkRagdollTemplateData"
    (0x010c_3c50, 0x00cc_1290), // "bhkPoseArray"
];

// Translated from 00c68230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::InitSystem` (Xbox PDB): once (while the Havok base system flag
/// at `01268414` is still clear) constructs the static allocator and the
/// free-list memory system, hands the memory system a 0x600000-byte frame
/// description, initialises the Havok base system over the static buffer at
/// `012077a0`, builds the collision filter, registers every `bhk` stream
/// loader and creates the multithreaded system if none exists.
// Translation incomplete: 00c68230 - SEH exception unwinding is unsupported.
pub fn bhk_world_init_system(e: &mut Engine) {
    const ALLOCATOR_INITIALIZED: u32 = 0x0126_7b58;
    const ALLOCATOR_STORAGE: u32 = 0x0126_7b54;
    const MEMORY_SYSTEM_STORAGE: u32 = 0x0120_4410;
    const BUFFER: u32 = 0x0120_77a0;
    let scope = enter_scope(e);
    if e.global::<u8>(0x0126_8414) == 0 {
        if e.global::<u32>(ALLOCATOR_INITIALIZED) & 1 == 0 {
            let flags = e.global::<u32>(ALLOCATOR_INITIALIZED);
            e.set_global(ALLOCATOR_INITIALIZED, flags | 1);
            e.set_global(ALLOCATOR_STORAGE, ALLOCATOR_VTABLE);
            e.call(0x00ec_658f, &args![0x00fd_dd40u32]);
        }
        e.with_stack(8, |e, frame| {
            e.call(0x00cb_bfa0, &args![frame]);
            e.mem.set_u32(frame.addr(), 0);
            let system = e.call(0x00cd_0a60, &args![MEMORY_SYSTEM_STORAGE]).u32();
            e.call(0x00cd_06c0, &args![system, ALLOCATOR_STORAGE]);
            e.set_global(GLOBAL_MEMORY_SYSTEM, system);
            e.call(0x00cb_bfb0, &args![system]);
            let system = e.global::<u32>(GLOBAL_MEMORY_SYSTEM);
            let router = e.vcall(system, 4, &args![frame, 1u32]).u32();
            let middle = e.mem.u32(router + 0x10);
            e.mem.set_u32(router + 0xc, middle);
            e.mem.set_u32(router + 0x18, middle);
            e.mem.set_u32(router + 4, BUFFER);
            e.mem.set_u32(router, BUFFER);
            e.mem.set_u32(router + 8, BUFFER + 0x60000);
            e.call(0x00cb_bb10, &args![router, 0x0084_cbd0u32, 0u32]);
        });
        e.call(0x00c8_28f0, &[]);
        let memory = e.call(MEMORY_ROUTER, &[]).u32();
        let allocator = e.mem.u32(memory + 0x10);
        let filter = e.vcall(allocator, 4, &args![0x30u32]).u32();
        e.mem.set_u16(filter + 4, 0x30);
        e.call(0x00cb_fba0, &args![filter]);
        e.mem.set_u32(filter, 0x010c_3c18);
        e.mem.set_u32(filter + 8, 0x010c_3c0c);
        e.mem.set_u32(filter + 0xc, 0x010c_3bf8);
        e.mem.set_u32(filter + 0x10, 0x010c_3bec);
        e.mem.set_u32(filter + 0x14, 0x010c_3be0);
        e.set_global(GLOBAL_COLLISION_FILTER, filter);
        for (name, creator) in BHK_STREAM_LOADERS {
            e.call(0x00a6_4900, &args![name, creator]);
        }
        if e.global::<u32>(GLOBAL_MULTITHREADED_SYSTEM) == 0 {
            let memory = e.call(HEAP_ALLOC, &args![HEAP_SINGLETON, 0x1cu32]).u32();
            let system = if memory != 0 {
                e.call(0x00cb_f400, &args![memory]).u32()
            } else {
                0
            };
            e.set_global(GLOBAL_MULTITHREADED_SYSTEM, system);
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c68750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the three floats at `input` by the global at `012677d8` into a
/// four-float vector (`w = 0`) and moves the broad phase there
/// (`bhk_world_shift_broad_phase`).
// Translation incomplete: 00c68750 - the stack cookie check is unsupported.
pub fn fn_00c68750(e: &mut Engine, this: Ptr<BhkWorld>, input: Ptr) {
    let scale = e.global::<f32>(0x0126_77d8);
    let x = e.mem.f32(input.addr()) * scale;
    let y = e.mem.f32(input.addr() + 4) * scale;
    let z = scale * e.mem.f32(input.addr() + 8);
    e.with_stack(16, |e, vector| {
        e.mem.set_f32(vector.addr(), x);
        e.mem.set_f32(vector.addr() + 4, y);
        e.mem.set_f32(vector.addr() + 8, z);
        e.mem.set_f32(vector.addr() + 12, 0.0);
        bhk_world_shift_broad_phase(e, this, vector);
    });
}

// Translated from 00c687b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::CreateCinfo` (Xbox PDB): when the world has no `pInfo`, builds a
/// 0xF0-byte construction-info object (`fn_00c681c0`), stores its address
/// `+ 0xF0` as `pInfo` and, if the world already has a Havok object, hands it
/// to the `hkpWorld` (`00c91c50`). `created` receives whether `pInfo` was
/// missing; the result is the current `pInfo`.
// Translation incomplete: 00c687b0 - SEH exception unwinding is unsupported.
pub fn bhk_world_create_cinfo(e: &mut Engine, this: Ptr<BhkWorld>, created: Ptr) -> Ptr {
    let scope = enter_scope(e);
    let missing = e.get(this, BhkWorld::pInfo).is_null();
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let memory = e.call(MEMORY_ROUTER, &[]).u32();
        let allocator = e.mem.u32(memory + 0x10);
        let object = e.vcall(allocator, 4, &args![0xf0u32]).u32();
        e.mem.set_u16(object + 4, 0xf0);
        let built = fn_00c681c0(e, Ptr::new(object)).addr();
        let info = if built != 0 { built + 0xf0 } else { 0 };
        let has_object = e.mem.u32(this.addr() + 8) != 0;
        e.set(this, BhkWorld::pInfo, Ptr::new(info));
        if has_object {
            let base = if info != 0 { info - 0xf0 } else { 0 };
            let world = hk_world(e, this.cast());
            if world != 0 {
                e.call(0x00c9_1c50, &args![world, base]);
            }
        }
    }
    let info = e.get(this, BhkWorld::pInfo);
    leave_scope(e, scope);
    info
}

// Translated from 00c688b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::UpdateDebugColor` (Xbox PDB): with a debugger attached, pops the
/// `DebugColorA` array from the back, passing each pair (second word, first
/// word, 0) to the debugger helper (`00c984a0`, on the object in `012681e0`);
/// then empties the array.
pub fn bhk_world_update_debug_color(e: &mut Engine, this: Ptr<BhkWorld>) {
    if !e.get(this, BhkWorld::pDebugger).is_null() {
        let mut remaining = e.get(this, BhkWorld::debugColorCount);
        if remaining != 0 {
            loop {
                remaining -= 1;
                let array = e.get(this, BhkWorld::debugColorData).addr();
                let first = e.mem.u32(array + remaining * 8);
                let second = e.mem.u32(array + remaining * 8 + 4);
                let target = e.global::<u32>(0x0126_81e0);
                e.call(0x00c9_84a0, &args![target, second, first, 0u32]);
                if remaining == 0 {
                    break;
                }
            }
        }
    }
    e.set(this, BhkWorld::debugColorCount, 0);
}

// Translated from 00c68900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits a scene-graph object: asks the object at `+0x1C` for its `slot
/// +0x38` object, sets bits 1 and 2 of the flags at `+0x30` (clearing bit 4)
/// unless the request's `+8` is non-zero, calls `callback(found, request)`
/// when an object was found, and, if the request's byte `+4` is set, recurses
/// over the children (`+0xA0`, count `u16` at `+0xA6`) of the node the
/// object returns from slot `+0x0C`. `cdecl`.
pub fn fn_00c68900(e: &mut Engine, object: Ptr, request: Ptr, callback: u32) {
    if object.is_null() {
        return;
    }
    let holder = e.mem.u32(object.addr() + 0x1c);
    let found = if holder != 0 {
        e.vcall(holder, 0x38, &[]).u32()
    } else {
        0
    };
    if e.mem.u32(request.addr() + 8) == 0 {
        let flags = e.mem.u32(object.addr() + 0x30);
        e.mem.set_u32(object.addr() + 0x30, (flags & !0x10) | 6);
    }
    if found != 0 {
        e.call(callback, &args![found, request]);
    }
    if e.mem.u8(request.addr() + 4) != 0 {
        let node = e.vcall(object.addr(), 0xc, &[]).u32();
        if node != 0 {
            let mut index = 0;
            while index < e.mem.u16(node + 0xa6) as u32 {
                let children = e.mem.u32(node + 0xa0);
                let child = e.mem.u32(children + index * 4);
                fn_00c68900(e, Ptr::new(child), request, callback);
                index += 1;
            }
        }
    }
}

// Translated from 00c68a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the pending batched removals in the order constraints, actions,
/// phantoms, then entities (the partial flush when `partial` is non-zero,
/// the full one otherwise).
pub fn fn_00c68a00(e: &mut Engine, this: Ptr<BhkWorld>, partial: u8) {
    fn_00c67df0(e, this);
    fn_00c67cd0(e, this);
    fn_00c67bc0(e, this);
    if partial != 0 {
        bhk_world_remove_batched_entities_partial(e, this);
    } else {
        fn_00c679a0(e, this);
    }
}

/// The four flushes callers run when a removal batch is full.
fn flush_full_removal_batches(e: &mut Engine, this: Ptr<BhkWorld>) {
    fn_00c68a00(e, this, 1);
}

// Translated from 00c68a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched entity additions (`pEntityBatch`, at most 3000) with
/// `fn_00c674d0` and clears the count.
// Translation incomplete: 00c68a40 - SEH exception unwinding is unsupported.
pub fn fn_00c68a40(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iEntityBatchCount);
    if count != 0 {
        if count >= 3000 {
            e.set(this, BhkWorld::iEntityBatchCount, 3000);
        }
        let array = e.get(this, BhkWorld::pEntityBatch);
        let count = e.get(this, BhkWorld::iEntityBatchCount);
        fn_00c674d0(e, this.cast(), array, count);
        e.set(this, BhkWorld::iEntityBatchCount, 0);
    }
    leave_scope(e, scope);
}

// Translated from 00c68ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the action list: empties it (`fn_00c67f20`), then
/// releases the head item if the list still holds one.
// Translation incomplete: 00c68ae0 - SEH exception unwinding is unsupported.
pub fn fn_00c68ae0(e: &mut Engine, this: Ptr) {
    fn_00c67f20(e, this);
    let head = e.mem.u32(this.addr());
    if head != 0 {
        let count = e.call(INTERLOCKED_DECREMENT, &args![head + 4]).i32();
        if count == 0 {
            e.vcall(head, 4, &[]);
        }
    }
}

// Translated from 00c68b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the item that `item` points to at the head of the action list at
/// `this`. An empty list just takes the item; otherwise a new 8-byte node is
/// allocated that receives the old head item (with an extra reference) and
/// the old second-node pointer, becomes the second node, and the head is
/// assigned the new item. A null item is ignored.
pub fn fn_00c68b50(e: &mut Engine, this: Ptr, item: Ptr) {
    if e.mem.u32(item.addr()) == 0 {
        return;
    }
    if e.mem.u32(this.addr()) == 0 {
        hk_ref_counted_pointer_assign(e, this, item);
        return;
    }
    let node = e.call(HEAP_ALLOC, &args![HEAP_SINGLETON, 8u32]).u32();
    if node != 0 {
        let head = e.mem.u32(this.addr());
        e.mem.set_u32(node, head);
        if head != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![head + 4]);
        }
        e.mem.set_u32(node + 4, 0);
    }
    let second = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(node + 4, second);
    e.mem.set_u32(this.addr() + 4, node);
    hk_ref_counted_pointer_assign(e, this, item);
}

// Translated from 00c68bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldCinfo` deleting destructor (Xbox PDB): runs the base destructor
/// (`00553540`) and, when bit 0 of `flags` is set, returns the object to the
/// memory router with its 16-bit size from `+4`.
pub fn bhk_world_cinfo_delete(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x0055_3540, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = e.mem.u16(this.addr() + 4) as u32;
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c68bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Remove` (Xbox PDB), entity form: with a world and an object,
/// either queues the object's body on the entity-removal batch (batch
/// removal counter non-zero; bit 5 marks it queued, a full batch is flushed
/// first) or removes it from the `hkpWorld` at once (`00c91620`). Returns
/// true when queued or when the object has no body to remove, otherwise the
/// result byte of the immediate removal.
pub fn bhk_world_remove(e: &mut Engine, this: Ptr<BhkWorld>, object: Ptr) -> bool {
    let world = hk_world(e, this.cast());
    if world == 0 || object.is_null() {
        return false;
    }
    let body = e.mem.u32(object.addr() + 0xc);
    if e.get(this, BhkWorld::iBatchRemove) != 0 {
        if body == 0 {
            return true;
        }
        let flags = e.mem.u32(body + 0x10);
        if flags & 0x30 != 0 {
            return true;
        }
        e.mem.set_u32(body + 0x10, flags | 0x20);
        if e.get(this, BhkWorld::iEntityRemoveBatchCount) >= 0xbb8 {
            flush_full_removal_batches(e, this);
        }
        e.call(HAVOK_ADD_REFERENCE, &args![object]);
        let count = e.get(this, BhkWorld::iEntityRemoveBatchCount);
        let array = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
        e.mem.set_u32(array + count * 4, object.addr());
        e.set(
            this,
            BhkWorld::iEntityRemoveBatchCount,
            count.wrapping_add(1),
        );
        return true;
    }
    if body == 0 || e.mem.u8(body + 0x10) & 0x30 != 0 {
        return true;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), object.addr());
        let result = e.call(0x00c9_1620, &args![world, out, object]).u32();
        e.mem.u8(result) != 0
    })
}

// Translated from 00c68cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Remove` (Xbox PDB), phantom form: with a world and an object,
/// queues the object's body on the phantom batch (batch removal counter
/// non-zero; byte flag bit 5 marks it queued, a full batch is flushed first)
/// or removes it from the `hkpWorld` at once (`00c92c70`). True whenever
/// both the world and the object exist.
pub fn bhk_world_remove_ov2(e: &mut Engine, this: Ptr<BhkWorld>, object: Ptr) -> bool {
    let world = hk_world(e, this.cast());
    if world == 0 || object.is_null() {
        return false;
    }
    let body = e.mem.u32(object.addr() + 0xc);
    if e.get(this, BhkWorld::iBatchRemove) != 0 {
        if body == 0 {
            return true;
        }
        let flags = e.mem.u8(body + 0x14);
        if flags & 0x30 != 0 {
            return true;
        }
        e.mem.set_u8(body + 0x14, flags | 0x20);
        if e.get(this, BhkWorld::iPhantomBatchCount) >= 0xc8 {
            flush_full_removal_batches(e, this);
        }
        e.call(HAVOK_ADD_REFERENCE, &args![object]);
        let count = e.get(this, BhkWorld::iPhantomBatchCount);
        let array = e.get(this, BhkWorld::pPhantomBatch).addr();
        e.mem.set_u32(array + count * 4, object.addr());
        e.set(this, BhkWorld::iPhantomBatchCount, count.wrapping_add(1));
        return true;
    }
    if body != 0 && e.mem.u8(body + 0x14) & 0x30 == 0 {
        e.call(0x00c9_2c70, &args![world, object]);
    }
    true
}

// Translated from 00c68d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Remove` (Xbox PDB), constraint form: with a world and an
/// object, queues the object on the constraint batch (batch removal counter
/// non-zero; a full batch is flushed first) or removes it from the
/// `hkpWorld` at once (`00c97e80`). True whenever both exist.
pub fn bhk_world_remove_ov3(e: &mut Engine, this: Ptr<BhkWorld>, object: Ptr) -> bool {
    let world = hk_world(e, this.cast());
    if world == 0 || object.is_null() {
        return false;
    }
    if e.get(this, BhkWorld::iBatchRemove) != 0 {
        if e.get(this, BhkWorld::iConstraintInstanceBatchCount) >= 0xc8 {
            flush_full_removal_batches(e, this);
        }
        e.call(HAVOK_ADD_REFERENCE, &args![object]);
        let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
        let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
        e.mem.set_u32(array + count * 4, object.addr());
        e.set(
            this,
            BhkWorld::iConstraintInstanceBatchCount,
            count.wrapping_add(1),
        );
    } else {
        e.with_stack(4, |e, out| {
            e.mem.set_u32(out.addr(), object.addr());
            e.call(0x00c9_7e80, &args![world, out, object]);
        });
    }
    true
}

// Translated from 00c68de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::UpdateDebugger` (Xbox PDB): with a debugger attached, calls its
/// slot `+0x0C` with 0.0, resets the calling thread's monitor stream
/// (`TlsGetValue` of the slot in `0126835c`, then `00cb0da0`), and calls slot
/// `+0x24` of the multithreaded system's object at `+0x18` if there is one;
/// then runs `UpdateDebugColor`.
pub fn bhk_world_update_debugger(e: &mut Engine, this: Ptr<BhkWorld>) {
    let debugger = e.get(this, BhkWorld::pDebugger);
    if !debugger.is_null() {
        e.vcall(debugger.addr(), 0xc, &args![0.0f32]);
        let slot = e.global::<u32>(0x0126_835c);
        let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
        e.call(0x00cb_0da0, &args![stream]);
        let system = e.global::<u32>(GLOBAL_MULTITHREADED_SYSTEM);
        let monitor = e.mem.u32(system + 0x18);
        if monitor != 0 {
            e.vcall(monitor, 0x24, &[]);
        }
    }
    bhk_world_update_debug_color(e, this);
}

// Translated from 00c68e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetDebugObjectColor` (Xbox PDB): appends the pair (the Havok
/// object at `object + 0x10` when `object + 8` is set, else null; `color`) to
/// the `DebugColorA` array, growing it first when full (`00c90940` with the
/// router's allocator and an element size of 8).
pub fn bhk_world_set_debug_object_color(
    e: &mut Engine,
    this: Ptr<BhkWorld>,
    object: Ptr,
    color: u32,
) {
    let wrapper = e.mem.u32(object.addr() + 8);
    let reference = if wrapper != 0 { wrapper + 0x10 } else { 0 };
    let array = this.addr() + 0x7c;
    let memory = e.call(MEMORY_ROUTER, &[]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(memory + 0x10);
        e.call(0x00c9_0940, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    if slot != 0 {
        e.mem.set_u32(slot, reference);
        e.mem.set_u32(slot + 4, color);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00c68e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the callback for the request kind (`request + 8`, index into the
/// table at `011afe88`) and, if there is one, visits `object` with it
/// (`fn_00c68900`). `cdecl`.
pub fn fn_00c68e90(e: &mut Engine, object: Ptr, request: Ptr) {
    let kind = e.mem.u32(request.addr() + 8);
    let callback = e.mem.u32(kind.wrapping_mul(4).wrapping_add(0x011a_fe88));
    if callback != 0 {
        fn_00c68900(e, object, request, callback);
    }
}

// Translated from 00c68ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits `object` with `callback` (`fn_00c68900`) when it is non-null and
/// the record `00c43490` returns for it has bit 1 set in its word at `+0xC`;
/// true when it did. `cdecl`.
pub fn fn_00c68ec0(e: &mut Engine, object: Ptr, request: Ptr, callback: u32) -> bool {
    if object.is_null() {
        return false;
    }
    let record = e.call(0x00c4_3490, &args![object]).u32();
    if record == 0 || e.mem.u32(record + 0xc) & 2 == 0 {
        return false;
    }
    fn_00c68900(e, object, request, callback);
    true
}

// Translated from 00c68f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends one batch-removal level: when `iBatchRemove` is non-zero it is
/// decremented and, on reaching zero, the pending removals are flushed
/// (`fn_00c68a00` order, partial entity flush when `partial` is non-zero);
/// when it was already zero, only a pending entity removal batch is flushed,
/// and only for `partial == 0`.
pub fn fn_00c68f00(e: &mut Engine, this: Ptr<BhkWorld>, partial: u8) {
    let level = e.get(this, BhkWorld::iBatchRemove);
    if level != 0 {
        e.set(this, BhkWorld::iBatchRemove, level - 1);
        if level - 1 == 0 {
            fn_00c68a00(e, this, partial);
        }
    } else if e.get(this, BhkWorld::iEntityRemoveBatchCount) != 0 && partial == 0 {
        fn_00c679a0(e, this);
    }
}

// Translated from 00c68f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<NiPointer<bhkAction>>::Remove` (Xbox PDB): removes the node
/// whose item equals the item `item` points to. Removing the first node
/// moves the second node's item into the head and frees that node; with no
/// second node the head item is released and cleared; any other node is
/// unlinked, destroyed (`fn_00c68ae0`) and freed. A null item or an empty list
/// does nothing.
pub fn bs_simple_list_ni_pointer_bhk_action_remove(e: &mut Engine, this: Ptr, item: Ptr) {
    let target = e.mem.u32(item.addr());
    if target == 0 {
        return;
    }
    let second = e.mem.u32(this.addr() + 4);
    if second == 0 && e.mem.u32(this.addr()) == 0 {
        return;
    }
    let mut previous = this.addr();
    let mut node = this.addr();
    loop {
        if e.mem.u32(node) == target {
            if node == this.addr() {
                if second != 0 {
                    let after = e.mem.u32(second + 4);
                    e.mem.set_u32(this.addr() + 4, after);
                    hk_ref_counted_pointer_assign(e, this, Ptr::new(second));
                    e.mem.set_u32(second + 4, 0);
                    fn_00c68ae0(e, Ptr::new(second));
                    e.call(HEAP_FREE, &args![HEAP_SINGLETON, second]);
                } else {
                    let head = e.mem.u32(this.addr());
                    if head != 0 {
                        let count = e.call(INTERLOCKED_DECREMENT, &args![head + 4]).i32();
                        if count == 0 {
                            e.vcall(head, 4, &[]);
                        }
                        e.mem.set_u32(this.addr(), 0);
                    }
                }
            } else {
                let after = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, after);
                e.mem.set_u32(node + 4, 0);
                fn_00c68ae0(e, Ptr::new(node));
                e.call(HEAP_FREE, &args![HEAP_SINGLETON, node]);
            }
            return;
        }
        previous = node;
        node = e.mem.u32(node + 4);
        if node == 0 {
            return;
        }
    }
}

// Translated from 00c69010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld` constructor: runs the base constructor (`00c87c30`), installs
/// the vtable `010c40b4`, clears the fields, copies the zero vector into
/// `Origin`, counts the world, allocates and clears the five batch arrays
/// (entities 12000 bytes, phantoms 800, actions 400, constraints 800,
/// entity removals 12000), clamps the global at `01267b38` to at most 1.0,
/// empties the action list, runs `Init` with `cinfo + 0xF0` (null stays
/// null), and records the `hkpWorld`'s tau and damping as the original ones.
// Translation incomplete: 00c69010 - SEH exception unwinding is unsupported.
pub fn fn_00c69010(e: &mut Engine, this: Ptr<BhkWorld>, cinfo: Ptr) -> Ptr<BhkWorld> {
    let base = this.addr();
    e.call(0x00c8_7c30, &args![this]);
    e.set(this, BhkWorld::pInfo, Ptr::NULL);
    e.mem.set_u32(base, 0x010c_40b4);
    e.set(this, BhkWorld::pDebugger, Ptr::NULL);
    e.set(this, BhkWorld::bAdded, false);
    e.set(this, BhkWorld::bEnabled, false);
    e.set(this, BhkWorld::bDebugDisplay, false);
    for offset in [0x18, 0x1c, 0x20, 0x24, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50] {
        e.mem.set_u32(base + offset, 0);
    }
    let zero_vector = e.mem.bytes(GLOBAL_ZERO_VECTOR, 16);
    e.mem.write(base + 0x60, &zero_vector);
    e.mem.set_u32(base + 0x70, 0);
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u32(base + 0x78, 0x8000_0000);
    e.mem.set_u32(base + 0x7c, 0);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0x8000_0000);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0);
    let scope = enter_scope(e);
    let worlds = e.global::<u32>(GLOBAL_WORLD_COUNT);
    e.set_global(GLOBAL_WORLD_COUNT, worlds.wrapping_add(1));
    for (field, size) in [
        (0x2cu32, 12000u32),
        (0x34, 800),
        (0x3c, 400),
        (0x44, 800),
        (0x4c, 12000),
    ] {
        let block = e.call(HEAP_ALLOC, &args![HEAP_SINGLETON, size]).u32();
        e.mem.set_u32(base + field, block);
        e.call(MEMSET, &args![block, 0u32, size]);
    }
    let tau_ratio = e.global::<f32>(0x0126_7b38);
    e.set_global(0x0126_7b38, if tau_ratio < 1.0 { tau_ratio } else { 1.0 });
    fn_00c67f20(e, Ptr::new(base + 0x90));
    let cinfo_base = if cinfo.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(cinfo.addr() + 0xf0)
    };
    bhk_world_init(e, this, cinfo_base);
    let world = hk_world(e, this.cast());
    let tau = e.mem.f32(world + 0x1e4);
    e.set(this, BhkWorld::fOriginalTau, tau);
    let world = hk_world(e, this.cast());
    let damping = e.mem.f32(world + 0x1e8);
    e.set(this, BhkWorld::fOriginalDamping, damping);
    leave_scope(e, scope);
    this
}

// Translated from 00c69220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`.
pub fn bhk_world_get_rtti(_e: &mut Engine, _this: Ptr<BhkWorld>) -> u32 {
    0x0126_7824
}

// Translated from 00c69230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::QCinfoSize` (Xbox PDB): the size of the construction-info
/// object, 0xF0 bytes.
pub fn bhk_world_q_cinfo_size(_e: &mut Engine, _this: Ptr<BhkWorld>) -> u32 {
    0xf0
}

/// Appends the viewer string whose pointer is in the word at `slot` to the
/// `NiTArray` of strings at `strings` (`u16` capacity at `+8`, count at
/// `+0xA`, grow-by at `+0xE`), growing it first when full.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: Ptr) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow = e.mem.u16(strings.addr() + 0xe) as u32 + count;
        e.call(0x0096_ad30, &args![strings, grow]);
    }
    e.call(0x0096_ae90, &args![strings, count, slot]);
}

// Translated from 00c69240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::GetViewerStrings` (Xbox PDB): after the base class's strings
/// (`00c878c0`), appends the RTTI name, the `Enabled` flag, whether a debugger
/// is attached (`Vis Debug`), and the global object counters (worlds,
/// actions, constraints, entities, rigid bodies, shapes, phantoms, Mopp
/// trees). The flag strings are built from a word whose upper three bytes
/// still hold the previous string pointer, as the executable passes it.
// Translation incomplete: 00c69240 - SEH exception unwinding is unsupported.
pub fn bhk_world_get_viewer_strings(e: &mut Engine, this: Ptr<BhkWorld>, strings: Ptr) {
    const COUNTERS: [(u32, u32); 8] = [
        (0x010c_41e4, 0x0126_77a4),
        (0x010c_41dc, 0x0126_8154),
        (0x010c_41d0, 0x0126_8188),
        (0x010c_41c4, 0x0126_81fc),
        (0x010c_41b8, 0x0126_81bc),
        (0x010c_41b0, 0x0126_80f8),
        (0x010c_41a4, 0x0126_8238),
        (0x010c_4190, 0x0126_8654),
    ];
    let scope = enter_scope(e);
    e.call(0x00c8_78c0, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(0x0126_7824);
        let text = e.call(0x00a7_d330, &args![name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot);

        let enabled = e.get(this, BhkWorld::bEnabled);
        e.mem.set_u8(slot.addr(), enabled as u8);
        let word = e.mem.u32(slot.addr());
        let text = e.call(0x00a7_d370, &args![0x0109_c7d8u32, word]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot);

        let attached = !e.get(this, BhkWorld::pDebugger).is_null();
        e.mem.set_u8(slot.addr(), attached as u8);
        let word = e.mem.u32(slot.addr());
        let text = e.call(0x00a7_d370, &args![0x010c_41ecu32, word]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot);

        for (label, counter) in COUNTERS {
            let value = e.mem.u32(counter);
            let text = e.call(0x00a7_d510, &args![label, value]).u32();
            e.mem.set_u32(slot.addr(), text);
            append_viewer_string(e, strings, slot);
        }
    });
    leave_scope(e, scope);
}

// Translated from 00c69580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes an action from the world: takes `action + 0x10` (an
/// `NiPointer<bhkAction>`-style reference, counted up and down around the
/// call) out of the action list at `this + 0x90`, then either queues the
/// action on the action batch (batch removal counter non-zero; a full batch of
/// 100 is flushed first) or removes it from the `hkpWorld` at once
/// (`00c97f50`). False without a world or action, otherwise true.
// Translation incomplete: 00c69580 - SEH exception unwinding is unsupported.
pub fn fn_00c69580(e: &mut Engine, this: Ptr<BhkWorld>, action: Ptr) -> bool {
    let scope = enter_scope(e);
    let world = hk_world(e, this.cast());
    if world == 0 || action.is_null() {
        leave_scope(e, scope);
        return false;
    }
    let reference = e.mem.u32(action.addr() + 0x10);
    if reference != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), reference);
            e.call(INTERLOCKED_INCREMENT, &args![reference + 4]);
            bs_simple_list_ni_pointer_bhk_action_remove(e, Ptr::new(this.addr() + 0x90), slot);
            let count = e.call(INTERLOCKED_DECREMENT, &args![reference + 4]).i32();
            if count == 0 {
                e.vcall(reference, 4, &[]);
            }
        });
    }
    if e.get(this, BhkWorld::iBatchRemove) != 0 {
        if e.get(this, BhkWorld::iActionBatchCount) >= 0x64 {
            flush_full_removal_batches(e, this);
        }
        e.call(HAVOK_ADD_REFERENCE, &args![action]);
        let count = e.get(this, BhkWorld::iActionBatchCount);
        let array = e.get(this, BhkWorld::pActionBatch).addr();
        e.mem.set_u32(array + count * 4, action.addr());
        e.set(this, BhkWorld::iActionBatchCount, count.wrapping_add(1));
    } else {
        e.call(0x00c9_7f50, &args![world, action]);
    }
    leave_scope(e, scope);
    true
}

// ---------------------------------------------------------------------------
// Picking, adding objects to the world and the notification requests.
//
// Import slots used below (named by Ghidra's decompilation of the callers):
// `00fdf044` is `TlsGetValue` (`00c696d0`), `00fdf1c0` / `00fdf1c4` are
// `InterlockedIncrement` / `InterlockedDecrement` (`00c6a540`, `00c6b3c0`).
// `RDTSC` has no model in the engine: `time_stamp_low` stands in with the
// host clock; only the profiling records written next to it use it.

const OBJECT_RECORD: u32 = 0x00c4_3490;
const PICK_CALC_TO: u32 = 0x0045_8840;
const CAST_RAY_WITH_FILTER: u32 = 0x00cb_f860;
const CAST_RAY: u32 = 0x00c9_1f70;
const CAST_RAY_OV2: u32 = 0x00c9_2040;
const RAY_COLLECTOR_VTABLE: u32 = 0x010c_3bd4;
const HIT_COPY_FROM_INPUT: u32 = 0x009c_1a70;
const HIT_COPY_FROM_OUTPUT: u32 = 0x009c_1aa0;
const SORT_HITS: u32 = 0x00c8_55f0;
const GLOBAL_PROFILE_SLOT: u32 = 0x0126_835c;
const GLOBAL_PICK_LIMIT_NORMAL: u32 = 0x011a_fe74;
const GLOBAL_PICK_LIMIT_ALTERNATE: u32 = 0x011a_fe78;
const GLOBAL_PICK_LIMIT_SELECTOR: u32 = 0x0126_77cc;
/// The string `TtPickObject` (the profiling record's begin label).
const PROFILE_BEGIN_PICK_OBJECT: u32 = 0x010c_41fc;
/// The string `Et` (the profiling record's end label).
const PROFILE_END: u32 = 0x010c_41f8;
/// Byte flag the add functions test before sending their first request.
const GLOBAL_ADD_PRELUDE_FLAG: u32 = 0x0126_77a1;
/// Table of request callbacks, indexed by the request kind at `request + 8`.
const REQUEST_CALLBACK_TABLE: u32 = 0x011a_fe88;
const GLOBAL_ENTITY_FLUSH_LIMIT: u32 = 0x011a_fe80;
const GLOBAL_PARTIAL_FLUSH_DONE: u32 = 0x0126_77a2;
const GLOBAL_UPDATING: u32 = 0x0126_77a0;
const GLOBAL_SCHEDULED_DEBUGGER: u32 = 0x0126_77cd;
const GLOBAL_STEP_TIME: u32 = 0x011a_fe64;
const GLOBAL_STEP_LENGTH: u32 = 0x011a_fe68;
const GLOBAL_STEP_SCALE: u32 = 0x0126_7b38;
const GLOBAL_STEP_FACTOR: u32 = 0x011a_c3a0;
const GLOBAL_STEP_DELTA: u32 = 0x0126_77ac;
const GLOBAL_ACQUIRE_FLAG: u32 = 0x0126_7b2c;
const GLOBAL_ADD_ENTITY_CHECK: u32 = 0x011b_0c44;
const ACQUIRE_MT_RESOURCES: u32 = 0x00cb_f6b0;
const WORLD_GET_CURRENT_TIME: u32 = 0x00c9_1020;
const WORLD_GET_CURRENT_PSI_TIME: u32 = 0x00c9_1030;
const WORLD_SET_CURRENT_TIME: u32 = 0x00c9_1040;
const WORLD_STEP_DONE: u32 = 0x00c9_1060;
const WORLD_PREPARE_STEP: u32 = 0x00c9_1bd0;
const WORLD_STEP_DELTA_TIME: u32 = 0x00c9_1b10;
const WORLD_STEP_MULTITHREADED: u32 = 0x00c9_3ae0;
const WORLD_BEFORE_STEP_A: u32 = 0x00c9_1120;
const WORLD_BEFORE_STEP_B: u32 = 0x00c9_1140;
const WORLD_ADD_ENTITY: u32 = 0x00c9_14d0;
const WORLD_ADD_PHANTOM: u32 = 0x00c9_36a0;
const WORLD_ADD_CONSTRAINT: u32 = 0x00c9_7d30;
const WORLD_ADD_ACTION: u32 = 0x00c9_7370;
const CLEANUP_DIRTY_ISLANDS: u32 = 0x00cb_55d0;
const MARK_ISLAND_INACTIVE: u32 = 0x00cb_5420;
const POINTER_ASSIGN: u32 = 0x0063_3c90;
const POINTER_RELEASE: u32 = 0x0045_cec0;
const NI_TO_HK_TRANSFORM: u32 = 0x0056_df80;
const TRANSFORM_POSITION: u32 = 0x00c7_f640;
const QUATERNION_FROM_TRANSFORM: u32 = 0x00cb_26e0;
const BODY_CREATE_T: u32 = 0x00cb_38c0;
const HAVOK_ALIGNED_ALLOC: u32 = 0x00c7_4ea0;
const SET_WORLD_OBJECT: u32 = 0x00c6_b980;
const SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
const BODY_ACCEPT_CHECK: u32 = 0x0043_b300;
const ENTITY_OWNER_RECORD: u32 = 0x0043_b4f0;
const OWNER_POSITION_VECTOR: u32 = 0x0056_0d80;
const OWNER_ROTATION_VECTOR: u32 = 0x0056_0e10;
const PHYSICS_REGISTER_PROCESSES: u32 = 0x00cd_1ed0;
const PHYSICS_CONTEXT_CREATE: u32 = 0x00cd_25d0;
const PHYSICS_CONTEXT_ADD_WORLD: u32 = 0x00cd_2530;
const VISUAL_DEBUGGER_CREATE: u32 = 0x00cd_1780;
const VISUAL_DEBUGGER_SERVE: u32 = 0x00cd_0d00;
const ARRAY_GROW: u32 = 0x00c9_0940;
const GLOBAL_BODY_CHECK_TABLE: u32 = 0x0126_83c8;
const GLOBAL_IDENTITY_MATRIX: u32 = 0x011a_9448;
const GLOBAL_IDENTITY_TRANSLATION: u32 = 0x011f_426c;
const GLOBAL_QUATERNION_BIAS: u32 = 0x010c_71e0;
const GLOBAL_BODY_ZERO_ARGUMENT: u32 = 0x0126_83b0;
const WORLD_DESTRUCTOR_VTABLE: u32 = 0x010c_40b4;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_DESTRUCTOR: u32 = 0x00c8_77b0;
const REFERENCE_OWNER_NOTIFY: u32 = 0x0055_33f0;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the calling thread's
/// record stream (`TlsGetValue` of the slot in `0126835c`) while the cursor at
/// `+4` is below the limit at `+0xC`; each record takes 12 bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot = e.global::<u32>(GLOBAL_PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Casts the ray described by `pick` with the filtered caster: builds the
/// caster object on the stack (vtable `010c3bd4`, words `+0x40` and `+0x44`
/// cleared) and calls `hkpWorldRayCaster::castRay` (`00cbf860`) with the
/// `hkpWorld`'s words `+0x58` and `+0x7C`, the filter and the output.
fn cast_ray_with_filter(e: &mut Engine, world: u32, pick: Ptr, filter: u32, output: u32) {
    e.with_stack(0x4c, |e, caster| {
        e.mem.set_u32(caster.addr(), RAY_COLLECTOR_VTABLE);
        e.mem.set_u32(caster.addr() + 0x40, 0);
        e.mem.set_u32(caster.addr() + 0x44, 0);
        let first = e.mem.u32(world + 0x58);
        let second = e.mem.u32(world + 0x7c);
        e.call(
            CAST_RAY_WITH_FILTER,
            &args![caster, first, pick, second, filter, output],
        );
    });
}

// Translated from 00c696d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::PickObject` (Xbox PDB): casts the ray of the `bhkPickData` at
/// `pick` through the world and says whether it hit something. When the pick
/// count override byte is clear and the limit (`011afe74`, or `011afe78` when
/// the byte at `012677cc` is set) is below the world float at `012677c4`, the
/// pick is skipped: the result fields are reset and false is returned. Otherwise
/// (between two profiling records, `TtPickObject` and `Et`) it asks
/// `bhkPickData::CalcTo` (`00458840`) to prepare the pick and casts: with an
/// output at `pick + 0xA8` through the world's `castRay` (or the filtered
/// caster when `pick + 0xA0` is set), else with an input at `pick + 0xA4` the
/// same way, else with the second `castRay` overload into `pick + 0x30`. A hit
/// is copied into the pick data (`009c1a70` / `009c1aa0`).
// Translation incomplete: 00c696d0 - SEH exception unwinding and the stack cookie check are unsupported.
pub fn bhk_world_pick_object(e: &mut Engine, this: Ptr<BhkWorld>, pick: Ptr) -> bool {
    let base = pick.addr();
    if e.global::<u8>(GLOBAL_PICK_COUNT_OVERRIDE) == 0 {
        let limit = if e.global::<u8>(GLOBAL_PICK_LIMIT_SELECTOR) != 0 {
            e.global::<f32>(GLOBAL_PICK_LIMIT_ALTERNATE)
        } else {
            e.global::<f32>(GLOBAL_PICK_LIMIT_NORMAL)
        };
        if limit < e.global::<f32>(GLOBAL_WORLD_FLOAT) {
            e.mem.set_u8(base + 0xac, 1);
            e.mem.set_f32(base + 0x40, 1.0);
            e.mem.set_u32(base + 0x44, u32::MAX);
            e.mem.set_u32(base + 0x50, u32::MAX);
            e.mem.set_u32(base + 0x80, 0);
            e.mem.set_u32(base + 0x70, 0);
            return false;
        }
    }
    profile_record(e, PROFILE_BEGIN_PICK_OBJECT);
    let mut hit = false;
    let world = hk_world(e, this.cast());
    if world != 0 {
        e.call(PICK_CALC_TO, &args![pick]);
        let result = base + 0x30;
        e.mem.set_f32(base + 0x40, 1.0);
        e.mem.set_u32(base + 0x80, 0);
        e.mem.set_u32(base + 0x44, u32::MAX);
        e.mem.set_u32(base + 0x70, 0);
        e.mem.set_u32(base + 0x50, u32::MAX);
        let output = e.mem.u32(base + 0xa8);
        let filter = e.mem.u32(base + 0xa0);
        if output != 0 {
            if filter == 0 {
                e.call(CAST_RAY, &args![world, pick, output]);
            } else {
                cast_ray_with_filter(e, world, pick, filter, output);
            }
            if e.mem.u32(output + 0x14) != 0 {
                e.call(SORT_HITS, &args![output]);
                let best = e.mem.u32(output + 0x10);
                e.call(HIT_COPY_FROM_OUTPUT, &args![result, best]);
                let value = e.mem.u32(best + 0x50);
                e.mem.set_u32(result + 0x50, value);
                hit = true;
            }
        } else {
            let input = e.mem.u32(base + 0xa4);
            if input != 0 {
                e.mem.set_f32(input + 0x20, 1.0);
                e.mem.set_u32(input + 0x60, 0);
                e.mem.set_u32(input + 0x24, u32::MAX);
                e.mem.set_u32(input + 0x50, 0);
                e.mem.set_u32(input + 0x30, u32::MAX);
                e.mem.set_f32(input + 4, 1.0);
                if filter == 0 {
                    e.call(CAST_RAY, &args![world, pick, input]);
                } else {
                    cast_ray_with_filter(e, world, pick, filter, input);
                }
                if e.mem.u32(input + 0x60) != 0 {
                    e.call(HIT_COPY_FROM_INPUT, &args![result, input + 0x10]);
                    hit = true;
                }
            } else {
                e.call(CAST_RAY_OV2, &args![world, pick, result]);
                hit = e.mem.u32(base + 0x80) != 0;
            }
        }
    }
    profile_record(e, PROFILE_END);
    hit
}

/// The record `00c43490` returns for `object` has bit 1 set in its word at
/// `+0xC`; false when it returns none.
fn object_is_registered(e: &mut Engine, object: Ptr) -> bool {
    let record = e.call(OBJECT_RECORD, &args![object]).u32();
    record != 0 && e.mem.u32(record + 0xc) & 2 != 0
}

/// Sends the request of kind `kind` for `object` as the small `bhkWorld`
/// notification functions do. Unless `skip_check` is non-zero the object must
/// pass `object_is_registered`; `None` is returned (and nothing is sent) for a
/// null object or a failed check. The 0x1C-byte request record holds a null
/// word at `+0`, `flag` at `+4`, the kind at `+8` and the two words `+0xC`
/// and `+0x10`; the callback of the kind (`011afe88 + 4 * kind`), if any, is
/// run on the object through `fn_00c68900`. The request's word at `+0xC`
/// after the callback is returned.
fn send_object_request(
    e: &mut Engine,
    object: Ptr,
    skip_check: u8,
    kind: u32,
    flag: u8,
    word_c: u32,
    word_10: u32,
) -> Option<u32> {
    if object.is_null() {
        return None;
    }
    if skip_check == 0 && !object_is_registered(e, object) {
        return None;
    }
    Some(e.with_stack(0x1c, |e, request| {
        let base = request.addr();
        e.mem.set_u32(base, 0);
        e.mem.set_u8(base + 4, flag);
        e.mem.set_u32(base + 8, kind);
        e.mem.set_u32(base + 0xc, word_c);
        e.mem.set_u32(base + 0x10, word_10);
        let callback = e.mem.u32(REQUEST_CALLBACK_TABLE + kind * 4);
        if callback != 0 {
            fn_00c68900(e, object, request, callback);
        }
        e.mem.u32(base + 0xc)
    }))
}

/// Runs the flushes the add functions run when a batch is full: the phantoms
/// (`fn_00c676e0`), the actions (`fn_00c6aac0`) and the constraints
/// (`fn_00c67850`).
fn flush_pending_batches(e: &mut Engine, this: Ptr<BhkWorld>) {
    fn_00c676e0(e, this);
    fn_00c6aac0(e, this);
    fn_00c67850(e, this);
}

/// Flushes the tail of the entity batch (`pEntityBatch`): the count is
/// clamped to 3000, at most `011afe80` entries from the end are added with
/// `fn_00c674d0` and the rest stay queued; marks the partial flush as done
/// (`012677a2`).
fn flush_entity_batch_partially(e: &mut Engine, this: Ptr<BhkWorld>) {
    if e.get(this, BhkWorld::iEntityBatchCount) >= 3000 {
        e.set(this, BhkWorld::iEntityBatchCount, 3000);
    }
    let count = e.get(this, BhkWorld::iEntityBatchCount);
    let limit = e.global::<u32>(GLOBAL_ENTITY_FLUSH_LIMIT);
    let take = count.min(limit);
    let remaining = count - take;
    let entities = e.get(this, BhkWorld::pEntityBatch).addr() + remaining * 4;
    fn_00c674d0(e, this.cast(), Ptr::new(entities), take);
    e.set(this, BhkWorld::iEntityBatchCount, remaining);
    e.set_global(GLOBAL_PARTIAL_FLUSH_DONE, 1u8);
}

/// SSE rotation of the vector `v` by the quaternion `q` as the executable's
/// `MULPS` / `SHUFPS` / `ADDPS` sequence computes it, every lane in single
/// precision: `2 * (cross * q.w + ((q.w * q.w + bias) * v + dot * q))`, where
/// `dot` is `v.z * q.z + (v.y * q.y + v.x * q.x)` and `cross` is
/// `v.zxy * q.yzx - v.yzx * q.zxy`, taken the other way round when
/// `negate_cross` is set.
fn rotate_by_quaternion(q: [f32; 4], v: [f32; 4], bias: [f32; 4], negate_cross: bool) -> [f32; 4] {
    let product = [v[0] * q[0], v[1] * q[1], v[2] * q[2], v[3] * q[3]];
    let dot = product[2] + (product[1] + product[0]);
    let first = [v[1] * q[2], v[2] * q[0], v[0] * q[1], v[3] * q[3]];
    let second = [v[2] * q[1], v[0] * q[2], v[1] * q[0], v[3] * q[3]];
    let mut out = [0.0f32; 4];
    for lane in 0..4 {
        let cross = if negate_cross {
            first[lane] - second[lane]
        } else {
            second[lane] - first[lane]
        };
        let scaled = cross * q[3];
        let factor = (q[3] * q[3] + bias[lane]) * v[lane];
        let sum = factor + dot * q[lane];
        let total = scaled + sum;
        out[lane] = total + total;
    }
    out
}

fn read_vector(e: &mut Engine, address: u32) -> [f32; 4] {
    [
        e.mem.f32(address),
        e.mem.f32(address + 4),
        e.mem.f32(address + 8),
        e.mem.f32(address + 12),
    ]
}

fn write_vector(e: &mut Engine, address: u32, vector: [f32; 4]) {
    for (lane, value) in vector.into_iter().enumerate() {
        e.mem.set_f32(address + lane as u32 * 4, value);
    }
}

/// Prepares the body that `node`'s transform is composed with: `body` itself
/// when `0043b300` accepts it, otherwise a copy made through `body`'s slot
/// `+0x40`, wrapped in a new 0x40-byte `bhkRigidBodyT` (`00cb38c0`) whose
/// position (`+0x30`) and rotation (`+0x20`) come from the copy's slots
/// `+0xD4` / `+0xD8` and which replaces the collision object's world object.
/// Returns the body to use (null when there is none).
fn body_for_move(e: &mut Engine, collision: u32, body: u32) -> u32 {
    if e.call(BODY_ACCEPT_CHECK, &args![GLOBAL_BODY_CHECK_TABLE, body])
        .bool()
    {
        return body;
    }
    let cloned = e.vcall(body, 0x40, &[]).u32();
    e.with_stack(4, |e, slot| {
        e.call(POINTER_ASSIGN, &args![slot, cloned]);
        let copy = e.mem.u32(slot.addr());
        let mut result = 0;
        if copy != 0 {
            e.call(SET_WORLD_OBJECT, &args![collision, 0u32]);
            let memory = e.call(HAVOK_ALIGNED_ALLOC, &args![0x40u32]).u32();
            let created = if memory != 0 {
                e.call(BODY_CREATE_T, &args![memory, copy]).u32()
            } else {
                0
            };
            e.with_stack(0x20, |e, vectors| {
                let position = vectors.addr();
                let rotation = vectors.addr() + 0x10;
                e.vcall(copy, 0xd4, &args![position]);
                e.vcall(copy, 0xd8, &args![rotation]);
                let position = e.mem.bytes(position, 16);
                let rotation = e.mem.bytes(rotation, 16);
                e.mem.write(created + 0x30, &position);
                e.mem.write(created + 0x20, &rotation);
            });
            e.vcall(created, 0xdc, &args![GLOBAL_ZERO_VECTOR]);
            e.vcall(created, 0xe0, &args![GLOBAL_BODY_ZERO_ARGUMENT]);
            e.call(SET_WORLD_OBJECT, &args![collision, created]);
            let count = e.call(INTERLOCKED_DECREMENT, &args![copy + 4]).i32();
            if count == 0 {
                e.vcall(copy, 4, &[]);
            }
            result = memory;
            e.mem.set_u32(slot.addr(), 0);
        }
        e.call(POINTER_RELEASE, &args![slot]);
        result
    })
}

/// Composes the body's position and rotation with the transform of `node`
/// (`NI2HK` of `node + 0x34`), hands the collision object to `other` and
/// clears `node`'s collision object; the flags word of `node` at `+0x30` then
/// has bit 4 cleared and bits 1 and 2 set.
fn move_body_to_other(e: &mut Engine, node: Ptr, other: Ptr, collision: u32, body: u32) {
    e.with_stack(0x80, |e, frame| {
        let transform = frame.addr();
        let transformed = frame.addr() + 0x40;
        let vector = frame.addr() + 0x50;
        let quaternion = frame.addr() + 0x60;
        let product = frame.addr() + 0x70;
        e.call(NI_TO_HK_TRANSFORM, &args![transform, node.addr() + 0x34]);
        let bias = read_vector(e, GLOBAL_QUATERNION_BIAS);
        let rotation = read_vector(e, body + 0x20);
        let position = read_vector(e, body + 0x30);
        let rotated = rotate_by_quaternion(rotation, position, bias, false);
        write_vector(e, vector, rotated);
        e.call(TRANSFORM_POSITION, &args![transformed, transform, vector]);
        e.call(QUATERNION_FROM_TRANSFORM, &args![quaternion, transform]);
        hk_quaternion_set_mul(
            e,
            Ptr::new(product),
            Ptr::new(body + 0x20),
            Ptr::new(quaternion),
        );
        let rotation = read_vector(e, body + 0x20);
        let moved = read_vector(e, transformed);
        let result = rotate_by_quaternion(rotation, moved, bias, true);
        write_vector(e, body + 0x30, result);
        let product_bytes = e.mem.bytes(product, 16);
        e.mem.write(body + 0x20, &product_bytes);
        e.call(SET_COLLISION_OBJECT, &args![other, collision]);
        e.call(SET_COLLISION_OBJECT, &args![node, 0u32]);
        let flags = e.mem.u32(node.addr() + 0x30);
        e.mem.set_u32(node.addr() + 0x30, (flags & !0x10) | 6);
    });
}

// Translated from 00c69960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::AddObjectsAndMoveController` (Xbox PDB): when both `node` and
/// `other` are non-null and `other` is registered (`object_is_registered`),
/// asks `node`'s object at `+0x1C` (slot `+0x38`) for its collision object. If
/// that has a body and its slot `+0xB4` answers false the world is marked
/// added (`bAdded`) and, unless `other` is `node`, the collision object moves
/// to `other`: with a body to use (`body_for_move`) its transform is
/// composed with `node`'s (`move_body_to_other`), without one `node` gets the
/// identity transform (nine words from `011a9448`, three from `011f426c`) and
/// the same flag changes. Afterwards the add requests (kind 0xA when the byte
/// `012677a1` is set, then kind 0) are sent for `other`. True when the world
/// was marked added.
// Translation incomplete: 00c69960 - SEH exception unwinding and the stack cookie check are unsupported.
pub fn bhk_world_add_objects_and_move_controller(
    e: &mut Engine,
    this: Ptr<BhkWorld>,
    node: Ptr,
    other: Ptr,
) -> bool {
    let scope = enter_scope(e);
    let mut added = false;
    if !node.is_null() && !other.is_null() && object_is_registered(e, other) {
        let mut found = 0u32;
        let holder = e.mem.u32(node.addr() + 0x1c);
        if holder != 0 {
            let collision = e.vcall(holder, 0x38, &[]).u32();
            if collision != 0 {
                let body = e.mem.u32(collision + 0x10);
                found = 1;
                if body != 0 && !e.vcall(collision, 0xb4, &[]).bool() {
                    e.set(this, BhkWorld::bAdded, true);
                    added = true;
                    if other != node {
                        let target = body_for_move(e, collision, body);
                        if target != 0 {
                            move_body_to_other(e, node, other, collision, target);
                        } else {
                            e.call(SET_COLLISION_OBJECT, &args![other, collision]);
                            e.call(SET_COLLISION_OBJECT, &args![node, 0u32]);
                            let identity = e.mem.bytes(GLOBAL_IDENTITY_MATRIX, 36);
                            e.mem.write(node.addr() + 0x34, &identity);
                            for offset in [0u32, 4, 8] {
                                let value = e.mem.u32(GLOBAL_IDENTITY_TRANSLATION + offset);
                                e.mem.set_u32(node.addr() + 0x58 + offset, value);
                            }
                            let flags = e.mem.u32(node.addr() + 0x30);
                            e.mem.set_u32(node.addr() + 0x30, (flags & !0x10) | 6);
                        }
                    }
                }
            }
        }
        e.with_stack(0x1c, |e, request| {
            let base = request.addr();
            e.mem.set_u32(base, this.addr());
            e.mem.set_u8(base + 4, 1);
            if e.global::<u8>(GLOBAL_ADD_PRELUDE_FLAG) != 0 {
                e.mem.set_u32(base + 8, 0xa);
                fn_00c68e90(e, other, request);
            }
            e.mem.set_u32(base + 8, 0);
            e.mem.set_u32(base + 0xc, found);
            e.mem.set_u32(base + 0x10, 0);
            e.mem.set_u32(base + 0x14, 0);
            e.mem.set_u32(base + 0x18, 1);
            let callback = e.mem.u32(REQUEST_CALLBACK_TABLE);
            if callback != 0 {
                fn_00c68900(e, other, request, callback);
            }
        });
    }
    leave_scope(e, scope);
    added
}

// Translated from 00c69e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::AddObjects` (Xbox PDB): unless `skip_check` is set, `object`
/// must be non-null and registered (`object_is_registered`). Marks the world
/// added, sends the kind 0xA request (callback `011afeb0`) when the byte
/// `012677a1` is set, then the kind 0 request with `flag_b` (cleared when the
/// object has no word at `+0x1C`) and `word`. False for a rejected object.
pub fn bhk_world_add_objects(
    e: &mut Engine,
    this: Ptr<BhkWorld>,
    object: Ptr,
    flag_a: u8,
    flag_b: u8,
    word: u32,
    skip_check: u8,
) -> bool {
    if object.is_null() {
        return false;
    }
    if skip_check == 0 && !object_is_registered(e, object) {
        return false;
    }
    e.set(this, BhkWorld::bAdded, true);
    e.with_stack(0x1c, |e, request| {
        let base = request.addr();
        e.mem.set_u32(base, this.addr());
        e.mem.set_u8(base + 4, flag_a);
        if e.global::<u8>(GLOBAL_ADD_PRELUDE_FLAG) != 0 {
            e.mem.set_u32(base + 8, 0xa);
            let callback = e.mem.u32(REQUEST_CALLBACK_TABLE + 0xa * 4);
            if callback != 0 {
                fn_00c68900(e, object, request, callback);
            }
        }
        let attach = flag_b != 0 && e.mem.u32(object.addr() + 0x1c) != 0;
        e.mem.set_u32(base + 8, 0);
        e.mem.set_u32(base + 0xc, attach as u32);
        e.mem.set_u32(base + 0x10, word);
        e.mem.set_u32(base + 0x14, 0);
        e.mem.set_u32(base + 0x18, 1);
        let callback = e.mem.u32(REQUEST_CALLBACK_TABLE);
        if callback != 0 {
            fn_00c68900(e, object, request, callback);
        }
    });
    true
}

// Translated from 00c69ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::RemoveObjects` (Xbox PDB), `cdecl` without `this`: sends the
/// kind 1 request (callback `011afe8c`) for `object` with `flag` unless the
/// object is null or (without `skip_check`) not registered. Returns the
/// `skip_check` byte itself when the request was sent (so zero when the check
/// is what let it through), zero otherwise.
pub fn bhk_world_remove_objects(e: &mut Engine, object: Ptr, flag: u8, skip_check: u8) -> u8 {
    match send_object_request(e, object, skip_check, 1, flag, 0, 0) {
        Some(_) => skip_check,
        None => 0,
    }
}

// Translated from 00c69f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::UpdatePosition` (Xbox PDB), `cdecl` without `this`: sends the
/// kind 2 request (callback `011afe90`) for `object` with `flag`; true when it
/// was sent.
pub fn bhk_world_update_position(e: &mut Engine, object: Ptr, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 2, flag, 0, 0).is_some()
}

// Translated from 00c69fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::AdjustForceAddCount` (Xbox PDB), `cdecl` without `this`: sends
/// the kind 0xD request (callback `011afebc`) with `value != 0` in the word at
/// `+0xC`; true when it was sent.
pub fn bhk_world_adjust_force_add_count(
    e: &mut Engine,
    object: Ptr,
    value: u8,
    flag: u8,
    skip_check: u8,
) -> bool {
    send_object_request(e, object, skip_check, 0xd, flag, (value != 0) as u32, 0).is_some()
}

// Translated from 00c6a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 3 (callback `011afe94`) for `object` with the byte `value` in
/// the word at `+0xC`; `cdecl` without `this`, true when it was sent. The
/// executable's name for it is unknown.
pub fn fn_00c6a040(e: &mut Engine, object: Ptr, value: u8, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 3, flag, value as u32, 0).is_some()
}

// Translated from 00c6a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 0xE (callback `011afec0`) for `object` with the byte `value`
/// in the word at `+0xC`; `cdecl` without `this`, true when it was sent. The
/// executable's name for it is unknown.
pub fn fn_00c6a0b0(e: &mut Engine, object: Ptr, value: u8, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 0xe, flag, value as u32, 0).is_some()
}

// Translated from 00c6a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 0xF (callback `011afec4`) for `object` with the float `value`
/// in the word at `+0xC`; `cdecl` without `this`, true when it was sent. The
/// executable's name for it is unknown.
pub fn fn_00c6a120(e: &mut Engine, object: Ptr, value: f32, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 0xf, flag, value.to_bits(), 0).is_some()
}

// Translated from 00c6a190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 0x10 (callback `011afec8`) for `object` with the float
/// `value` in the word at `+0xC`; `cdecl` without `this`, true when it was
/// sent. The executable's name for it is unknown.
pub fn fn_00c6a190(e: &mut Engine, object: Ptr, value: f32, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 0x10, flag, value.to_bits(), 0).is_some()
}

// Translated from 00c6a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetDisabled` (Xbox PDB), `cdecl` without `this`: request kind
/// 0x11 (callback `011afecc`) with the byte `value` in the word at `+0xC`;
/// true when it was sent.
pub fn bhk_world_set_disabled(
    e: &mut Engine,
    object: Ptr,
    value: u8,
    flag: u8,
    skip_check: u8,
) -> bool {
    send_object_request(e, object, skip_check, 0x11, flag, value as u32, 0).is_some()
}

// Translated from 00c6a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Activate` (Xbox PDB), `cdecl` without `this`: request kind 0xC
/// (callback `011afeb8`) with the byte `value` in the word at `+0xC`; true
/// when it was sent.
pub fn bhk_world_activate(
    e: &mut Engine,
    object: Ptr,
    value: u8,
    flag: u8,
    skip_check: u8,
) -> bool {
    send_object_request(e, object, skip_check, 0xc, flag, value as u32, 0).is_some()
}

// Translated from 00c6a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::KillHavok` (Xbox PDB), `cdecl` without `this`: request kind 4
/// (callback `011afe98`) for `object` with `flag`; true when it was sent.
pub fn bhk_world_kill_havok(e: &mut Engine, object: Ptr, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 4, flag, 0, 0).is_some()
}

// Translated from 00c6a350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetMotion` (Xbox PDB), `cdecl` without `this`: request kind 5
/// (callback `011afe9c`) with `motion` in the word at `+0xC` and the byte
/// `extra` in the word at `+0x10`; true when it was sent.
pub fn bhk_world_set_motion(
    e: &mut Engine,
    object: Ptr,
    motion: u32,
    flag: u8,
    skip_check: u8,
    extra: u8,
) -> bool {
    send_object_request(e, object, skip_check, 5, flag, motion, extra as u32).is_some()
}

// Translated from 00c6a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 7 (callback `011afea4`) with `value` in the word at `+0xC`;
/// `cdecl` without `this`, true when it was sent. The executable's name for it
/// is unknown.
pub fn fn_00c6a3d0(e: &mut Engine, object: Ptr, value: u32, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 7, flag, value, 0).is_some()
}

// Translated from 00c6a440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 8 (callback `011afea8`) for `object` with `flag`; `cdecl`
/// without `this`. Returns whether the callback left a non-zero word in the
/// request's word at `+0xC` (false when the request was not sent). The
/// executable's name for it is unknown.
pub fn fn_00c6a440(e: &mut Engine, object: Ptr, flag: u8, skip_check: u8) -> bool {
    send_object_request(e, object, skip_check, 8, flag, 0, 0).is_some_and(|word| word != 0)
}

// Translated from 00c6a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Request kind 9 (callback `011afeac`) with `first` in the word at `+0x10`
/// and `second` in the word at `+0xC`; `cdecl` without `this`, true when it was
/// sent. The executable's name for it is unknown.
pub fn fn_00c6a4c0(
    e: &mut Engine,
    object: Ptr,
    first: u32,
    second: u32,
    flag: u8,
    skip_check: u8,
) -> bool {
    send_object_request(e, object, skip_check, 9, flag, second, first).is_some()
}

// Translated from 00c6a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::UpdateActions` (Xbox PDB): walks the action list at `this +
/// 0x90`. An item with no `hkpWorld` (its slot `+0x94`) is taken out of the
/// list (`BSSimpleList<NiPointer<bhkAction>>::Remove`) holding a temporary
/// reference; any other item gets its slot `+0xC4` called with `delta`.
// Translation incomplete: 00c6a540 - SEH exception unwinding is unsupported.
pub fn bhk_world_update_actions(e: &mut Engine, this: Ptr<BhkWorld>, delta: f32) {
    let head = this.addr() + 0x90;
    let mut node = head;
    while node != 0 {
        let next = e.mem.u32(node + 4);
        let item = e.mem.u32(node);
        if next == 0 && item == 0 {
            break;
        }
        let mut following = next;
        if item != 0 {
            if e.vcall(item, 0x94, &[]).u32() == 0 {
                if node == head {
                    following = node;
                }
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), item);
                    e.call(INTERLOCKED_INCREMENT, &args![item + 4]);
                    bs_simple_list_ni_pointer_bhk_action_remove(e, Ptr::new(head), slot);
                    let count = e.call(INTERLOCKED_DECREMENT, &args![item + 4]).i32();
                    if count == 0 {
                        e.vcall(item, 4, &[]);
                    }
                });
            } else {
                e.vcall(item, 0xc4, &args![delta]);
            }
        }
        node = following;
    }
}

// Translated from 00c6a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `005533f0` on `this` with `value` and then stores `value` in the
/// word at `+0x24`. The class and the executable's name for it are unknown.
pub fn fn_00c6a620(e: &mut Engine, this: Ptr, value: u32) {
    e.call(REFERENCE_OWNER_NOTIFY, &args![this, value]);
    e.mem.set_u32(this.addr() + 0x24, value);
}

/// Releases an `hkArray`-style buffer through the allocator at `allocator`
/// (slot `+0x10`, `bytes` long) unless the capacity word marks it as not
/// owned (sign bit set).
fn free_array_buffer(e: &mut Engine, allocator: u32, data: u32, capacity_word: u32, bytes: u32) {
    if capacity_word & 0x8000_0000 == 0 {
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00c6a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetVisualDebugger` (Xbox PDB): with `enable` clear, releases the
/// visual debugger at `+0x10` (`00c905b0`) if there is one. With `enable`
/// set and no debugger yet, registers all physics processes (`00cd1ed0`),
/// builds a 0x90-byte physics context (`00cd25d0`) holding the world
/// (`00cd2530`), creates the 0x90-byte visual debugger over a one-element
/// array of contexts (`00cd1780`), stores it at `+0x10`, starts serving on
/// port 0x61A9 (`00cd0d00`) and frees the temporary array.
// Translation incomplete: 00c6a640 - SEH exception unwinding is unsupported.
pub fn bhk_world_set_visual_debugger(e: &mut Engine, this: Ptr<BhkWorld>, enable: u8) {
    let scope = enter_scope(e);
    let debugger = e.get(this, BhkWorld::pDebugger);
    if enable == 0 {
        if !debugger.is_null() {
            hk_world(e, this.cast());
            e.call(HAVOK_RELEASE, &args![debugger]);
            e.set(this, BhkWorld::pDebugger, Ptr::NULL);
        }
    } else if debugger.is_null() {
        e.with_stack(12, |e, contexts| {
            let base = contexts.addr();
            e.mem.set_u32(base, 0);
            e.mem.set_u32(base + 4, 0);
            e.mem.set_u32(base + 8, 0x8000_0000);
            e.call(PHYSICS_REGISTER_PROCESSES, &[]);
            let router = e.call(MEMORY_ROUTER, &[]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let memory = e.vcall(allocator, 4, &args![0x90u32]).u32();
            e.mem.set_u16(memory + 4, 0x90);
            let context = e.call(PHYSICS_CONTEXT_CREATE, &args![memory]).u32();
            let world = hk_world(e, this.cast());
            e.call(PHYSICS_CONTEXT_ADD_WORLD, &args![context, world]);
            let entry = if context != 0 { context + 8 } else { 0 };
            let router = e.call(MEMORY_ROUTER, &[]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = e.mem.u32(base + 8) & 0x3fff_ffff;
            if e.mem.u32(base + 4) == capacity {
                e.call(ARRAY_GROW, &args![allocator, contexts, 4u32]);
            }
            let size = e.mem.u32(base + 4);
            let data = e.mem.u32(base);
            e.mem.set_u32(data + size * 4, entry);
            e.mem.set_u32(base + 4, size + 1);
            let router = e.call(MEMORY_ROUTER, &[]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let memory = e.vcall(allocator, 4, &args![0x90u32]).u32();
            e.mem.set_u16(memory + 4, 0x90);
            let created = e
                .call(VISUAL_DEBUGGER_CREATE, &args![memory, contexts, 0u32])
                .u32();
            e.set(this, BhkWorld::pDebugger, Ptr::new(created));
            e.call(VISUAL_DEBUGGER_SERVE, &args![created, 0x61a9u32]);
            let router = e.call(MEMORY_ROUTER, &[]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity_word = e.mem.u32(base + 8);
            let data = e.mem.u32(base);
            e.mem.set_u32(base + 4, 0);
            free_array_buffer(
                e,
                allocator,
                data,
                capacity_word,
                (capacity_word & 0x3fff_ffff) * 4,
            );
        });
    }
    leave_scope(e, scope);
}

// Translated from 00c6a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::SetDebugDisplay` (Xbox PDB): stores `enabled` in `bDebugDisplay`
/// and, for a non-null `object`, sends the kind 6 request (callback
/// `011afea0`) with the new flag in the word at `+0xC`.
pub fn bhk_world_set_debug_display(e: &mut Engine, this: Ptr<BhkWorld>, object: Ptr, enabled: u8) {
    e.mem.set_u8(this.addr() + 0x16, enabled);
    if !object.is_null() {
        let flag = e.get(this, BhkWorld::bDebugDisplay);
        e.with_stack(0x1c, |e, request| {
            let base = request.addr();
            e.mem.set_u32(base, 0);
            e.mem.set_u8(base + 4, 1);
            e.mem.set_u32(base + 8, 6);
            e.mem.set_u32(base + 0xc, flag as u32);
            let callback = e.mem.u32(REQUEST_CALLBACK_TABLE + 6 * 4);
            if callback != 0 {
                fn_00c68900(e, object, request, callback);
            }
        });
    }
}

// Translated from 00c6a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::DeactivateAllIslands` (Xbox PDB): with an `hkpWorld`, cleans its
/// dirty islands (`00cb55d0`), copies the island pointer array (`+0x28`,
/// count `+0x2C`) into a temporary buffer and marks every island inactive
/// (`00cb5420`) whose bodies all have the vectors at `+0x1B0` and `+0x1C0`
/// within the tolerance `01017d00` of the zero vector (first three lanes); an
/// island with no bodies counts. Cleans the dirty islands again and frees the
/// buffer. The calls of the world getter whose result the executable ignores
/// are kept.
// Translation incomplete: 00c6a870 - SEH exception unwinding is unsupported.
pub fn bhk_world_deactivate_all_islands(e: &mut Engine, this: Ptr) {
    let world = hk_world(e, this);
    if world == 0 {
        return;
    }
    hk_world(e, this);
    e.call(CLEANUP_DIRTY_ISLANDS, &args![world]);
    hk_world(e, this);
    let mut data = 0u32;
    let mut capacity_word = 0x8000_0000u32;
    hk_world(e, this);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let count = e.mem.i32(world + 0x2c);
    if count > 0 {
        let allocator = e.mem.u32(router + 0x10);
        let (block, granted) = e.with_stack(4, |e, size| {
            e.mem.set_i32(size.addr(), count.wrapping_mul(4));
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        data = block;
        capacity_word = (granted.wrapping_add((granted >> 31) & 3) >> 2) as u32;
    }
    let source = e.mem.u32(world + 0x28);
    for index in 0..count.max(0) as u32 {
        let island = e.mem.u32(source + index * 4);
        e.mem.set_u32(data + index * 4, island);
    }
    hk_world(e, this);
    let tolerance = e.global::<f32>(GLOBAL_TOLERANCE);
    for index in 0..count.max(0) as u32 {
        let island = e.mem.u32(data + index * 4);
        let bodies = e.mem.i32(island + 0x4c);
        let mut moving = false;
        let mut body_index = 0;
        while body_index < bodies && !moving {
            let bodies_array = e.mem.u32(island + 0x48);
            let body = e.mem.u32(bodies_array + body_index as u32 * 4);
            if !near_zero_vector(e, body + 0x1b0, tolerance)
                || !near_zero_vector(e, body + 0x1c0, tolerance)
            {
                moving = true;
            }
            body_index += 1;
        }
        if !moving {
            hk_world(e, this);
            e.call(MARK_ISLAND_INACTIVE, &args![world, island]);
            hk_world(e, this);
        }
    }
    hk_world(e, this);
    e.call(CLEANUP_DIRTY_ISLANDS, &args![world]);
    hk_world(e, this);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    free_array_buffer(
        e,
        allocator,
        data,
        capacity_word,
        (capacity_word & 0x3fff_ffff) * 4,
    );
}

// Translated from 00c6aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flushes the batched action additions (`pActionBatch`, at most 100): each
/// action without a world (word `+8`) is added to the `hkpWorld` (`00c97370`)
/// unless one of the entries its slot `+0x10` lists has a different world
/// (word `+8`); each action is then released and the batch emptied. Does
/// nothing without an `hkpWorld`.
// Translation incomplete: 00c6aac0 - SEH exception unwinding is unsupported.
pub fn fn_00c6aac0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let scope = enter_scope(e);
    let count = e.get(this, BhkWorld::iActionBatchCount);
    if count == 0 {
        leave_scope(e, scope);
        return;
    }
    if count >= 100 {
        e.set(this, BhkWorld::iActionBatchCount, 100);
    }
    let world = hk_world(e, this.cast());
    if world == 0 {
        leave_scope(e, scope);
        return;
    }
    let mut index = 0;
    while index < e.get(this, BhkWorld::iActionBatchCount) {
        let array = e.get(this, BhkWorld::pActionBatch).addr();
        let action = e.mem.u32(array + index * 4);
        if action != 0 && e.mem.u32(action + 8) == 0 {
            e.with_stack(12, |e, entities| {
                let base = entities.addr();
                e.mem.set_u32(base, 0);
                e.mem.set_u32(base + 4, 0);
                e.mem.set_u32(base + 8, 0x8000_0000);
                e.vcall(action, 0x10, &args![entities]);
                let listed = e.mem.i32(base + 4);
                let data = e.mem.u32(base);
                let mut all_in_world = true;
                let mut entry = 0;
                while entry < listed && all_in_world {
                    let item = e.mem.u32(data + entry as u32 * 4);
                    all_in_world = e.mem.u32(item + 8) == world;
                    entry += 1;
                }
                if listed <= 0 || all_in_world {
                    let array = e.get(this, BhkWorld::pActionBatch).addr();
                    let action = e.mem.u32(array + index * 4);
                    e.call(WORLD_ADD_ACTION, &args![world, action]);
                }
                let router = e.call(MEMORY_ROUTER, &[]).u32();
                let allocator = e.mem.u32(router + 0x10);
                let capacity_word = e.mem.u32(base + 8);
                e.mem.set_u32(base + 4, 0);
                free_array_buffer(
                    e,
                    allocator,
                    data,
                    capacity_word,
                    (capacity_word & 0x3fff_ffff) * 4,
                );
            });
        }
        let array = e.get(this, BhkWorld::pActionBatch).addr();
        let action = e.mem.u32(array + index * 4);
        if action != 0 {
            e.call(HAVOK_RELEASE, &args![action]);
        }
        index += 1;
    }
    let array = e.get(this, BhkWorld::pActionBatch).addr();
    let count = e.get(this, BhkWorld::iActionBatchCount);
    clear_pointers(e, array, count);
    e.set(this, BhkWorld::iActionBatchCount, 0);
    leave_scope(e, scope);
}

// Translated from 00c6aca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld` destructor body: installs the vtable `010c40b4`, counts the
/// world out, releases the visual debugger (`00c905b0`), kills the
/// serializable base object (`00c876a0`), frees the five batch arrays, deletes
/// every entity listener (slot 0 with 1) and empties the listener count,
/// empties and destroys the action list at `+0x90` (`fn_00c67f20`,
/// `fn_00c68ae0`), frees the `DebugColorA` and `pEntityListenerA` buffers
/// and runs the base destructor (`00c877b0`).
// Translation incomplete: 00c6aca0 - SEH exception unwinding is unsupported.
pub fn fn_00c6aca0(e: &mut Engine, this: Ptr<BhkWorld>) {
    let base = this.addr();
    e.mem.set_u32(base, WORLD_DESTRUCTOR_VTABLE);
    let scope = enter_scope(e);
    let worlds = e.global::<u32>(GLOBAL_WORLD_COUNT);
    e.set_global(GLOBAL_WORLD_COUNT, worlds.wrapping_sub(1));
    let debugger = e.get(this, BhkWorld::pDebugger);
    if !debugger.is_null() {
        hk_world(e, this.cast());
        e.call(HAVOK_RELEASE, &args![debugger]);
        e.set(this, BhkWorld::pDebugger, Ptr::NULL);
    }
    e.mem.set_u32(scope.0 + SCOPE_WORD, 0x10);
    hk_world(e, this.cast());
    e.call(SERIALIZABLE_KILL, &args![this]);
    for field in [0x2cu32, 0x34, 0x3c, 0x44, 0x4c] {
        let block = e.mem.u32(base + field);
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, block]);
    }
    let mut index = 0;
    while (index as i32) < e.get(this, BhkWorld::entityListenerCount) {
        let listeners = e.get(this, BhkWorld::entityListeners).addr();
        let listener = e.mem.u32(listeners + index * 4);
        if listener != 0 {
            e.vcall(listener, 0, &args![1u32]);
        }
        index += 1;
    }
    e.set(this, BhkWorld::entityListenerCount, 0);
    fn_00c67f20(e, Ptr::new(base + 0x90));
    leave_scope(e, scope);
    fn_00c68ae0(e, Ptr::new(base + 0x90));
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.set(this, BhkWorld::debugColorCount, 0);
    let capacity_word = e.mem.u32(base + 0x84);
    let data = e.get(this, BhkWorld::debugColorData).addr();
    free_array_buffer(
        e,
        allocator,
        data,
        capacity_word,
        (capacity_word & 0x3fff_ffff) * 8,
    );
    e.set(this, BhkWorld::debugColorData, Ptr::NULL);
    e.mem.set_u32(base + 0x84, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(base + 0x74, 0);
    let capacity_word = e.mem.u32(base + 0x78);
    let data = e.get(this, BhkWorld::entityListeners).addr();
    free_array_buffer(
        e,
        allocator,
        data,
        capacity_word,
        (capacity_word & 0x3fff_ffff) * 4,
    );
    e.set(this, BhkWorld::entityListeners, Ptr::NULL);
    e.mem.set_u32(base + 0x78, 0x8000_0000);
    e.call(SERIALIZABLE_DESTRUCTOR, &args![this]);
}

// Translated from 00c6ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Update` (Xbox PDB): resets the step accumulators and raises the
/// updating byte (`012677a0`). A pending entity batch is added (the tail of at
/// most `011afe80` entries, once until `012677a2` is cleared elsewhere) and the
/// constraint and action batches are flushed. An enabled world with a
/// positive step length (`011afe68`) then runs the solver: acquires the
/// multithreaded resources when the `hkpWorld` is in mode 3, updates the
/// actions, scales the solver info, advances the world's time and steps it
/// until its step-done query (`00c91060`) says so (single or multithreaded),
/// then updates the solver value and the debugger; the result is true when it
/// stepped. Finally the remaining entity removals are flushed, a scheduled
/// visual debugger is created (`012677cd`) and the updating byte is cleared.
pub fn bhk_world_update(e: &mut Engine, this: Ptr<BhkWorld>) -> bool {
    e.set_global(GLOBAL_WORLD_FLOAT, 0.0f32);
    e.set_global(GLOBAL_UPDATING, 1u8);
    e.set_global(GLOBAL_WORLD_INTEGER, 0u32);
    let mut stepped = false;
    if e.get(this, BhkWorld::iEntityBatchCount) != 0 {
        if e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE) == 0 {
            flush_entity_batch_partially(e, this);
        }
        fn_00c67850(e, this);
        fn_00c6aac0(e, this);
    }
    if e.get(this, BhkWorld::bEnabled) && 0.0 < e.global::<f32>(GLOBAL_STEP_LENGTH) {
        let world = hk_world(e, this.cast());
        let multithreaded = e.mem.u32(world + 0xf0) == 3;
        if multithreaded {
            let system = e.global::<u32>(GLOBAL_MULTITHREADED_SYSTEM);
            let flag = e.global::<u8>(GLOBAL_ACQUIRE_FLAG) as u32;
            e.call(ACQUIRE_MT_RESOURCES, &args![system, flag]);
        }
        let scale = e.global::<f32>(GLOBAL_STEP_SCALE) as f64;
        let factor = e.global::<f32>(GLOBAL_STEP_FACTOR) as f64;
        e.set_global(GLOBAL_STEP_DELTA, (scale * factor) as f32);
        let step_time = e.global::<f32>(GLOBAL_STEP_TIME);
        bhk_world_update_actions(e, this, step_time);
        bhk_world_scale_solver_info(e, this.cast());
        hk_world(e, this.cast());
        if e.mem.u32(world + 0x94) == 0 && e.mem.u8(world + 0x9c) == 0 {
            if e.mem.u32(world + 0x8c) != 0 {
                e.call(WORLD_BEFORE_STEP_A, &args![world]);
            }
            if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
                e.call(WORLD_BEFORE_STEP_B, &args![world]);
            }
        }
        hk_world(e, this.cast());
        e.call(WORLD_GET_CURRENT_TIME, &args![world]);
        let step_time = e.global::<f32>(GLOBAL_STEP_TIME);
        e.call(WORLD_SET_CURRENT_TIME, &args![world, step_time]);
        e.call(WORLD_PREPARE_STEP, &args![world]);
        while !e.call(WORLD_STEP_DONE, &args![world]).bool() {
            e.call(WORLD_GET_CURRENT_PSI_TIME, &args![world]);
            let delta = e.global::<f32>(GLOBAL_STEP_DELTA);
            if multithreaded {
                let system = e.global::<u32>(GLOBAL_MULTITHREADED_SYSTEM);
                let first = e.mem.u32(system + 0x14);
                let second = e.mem.u32(system + 0x18);
                e.call(
                    WORLD_STEP_MULTITHREADED,
                    &args![world, first, second, delta],
                );
            } else {
                e.call(WORLD_STEP_DELTA_TIME, &args![world, delta]);
            }
        }
        let length = e.global::<f32>(GLOBAL_STEP_LENGTH);
        bhk_world_set_solver_value(e, this, length);
        bhk_world_update_debugger(e, this);
        stepped = true;
    }
    if e.get(this, BhkWorld::iEntityRemoveBatchCount) != 0 {
        bhk_world_remove_batched_entities_partial(e, this);
    }
    if e.global::<u8>(GLOBAL_SCHEDULED_DEBUGGER) != 0 {
        bhk_world_set_visual_debugger(e, this, 1);
        e.set_global(GLOBAL_SCHEDULED_DEBUGGER, 0u8);
    }
    e.set_global(GLOBAL_UPDATING, 0u8);
    stepped
}

// Translated from 00c6b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld` deleting destructor (Xbox PDB `_scalar_deleting_destructor_`):
/// runs the destructor body (`fn_00c6aca0`) and, with bit 0 of `flags` set,
/// frees the block through the alignment byte before `this`.
pub fn bhk_world_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkWorld>,
    flags: u8,
) -> Ptr<BhkWorld> {
    fn_00c6aca0(e, this);
    if flags & 1 != 0 && !this.is_null() {
        let padding = e.mem.u8(this.addr() - 1) as u32;
        e.call(
            HEAP_FREE,
            &args![HEAP_SINGLETON, this.addr().wrapping_sub(padding)],
        );
    }
    this
}

/// The owner word the add functions test: for an object whose kind byte
/// (`+0x28`) is `kind`, the word at `+0xC` of the target
/// `object + 0x10 + (signed byte at +0x20)`; `None` when there is no such
/// target or owner.
fn owner_of_kind(e: &mut Engine, object: u32, kind: u8) -> Option<u32> {
    if e.mem.u8(object + 0x28) != kind {
        return None;
    }
    let base = object.wrapping_add(0x10);
    let target = base.wrapping_add_signed(e.mem.i8(base + 0x10) as i32);
    if target == 0 {
        return None;
    }
    let owner = e.mem.u32(target + 0xc);
    (owner != 0).then_some(owner)
}

/// `bhkWorld::Add` with an entity added at once: asks `0043b4f0` about the
/// owner (given a stack slot holding the object) and, when `011b0c44` is
/// set, the record's low seven bits are not 8 and the owner's position and
/// rotation vectors (`00560d80`, `00560e10`) are both within the tolerance
/// `01017d00` of zero, the flag passed to `addEntity` is 0, otherwise 1.
fn entity_add_flag(e: &mut Engine, owner: u32, object: Ptr) -> u32 {
    if e.global::<u8>(GLOBAL_ADD_ENTITY_CHECK) == 0 {
        return 1;
    }
    let kind = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object.addr());
        let record = e.call(ENTITY_OWNER_RECORD, &args![owner, slot]).u32();
        e.mem.u8(record) & 0x7f
    });
    if kind == 8 {
        return 1;
    }
    let tolerance = e.global::<f32>(GLOBAL_TOLERANCE);
    let position = e.call(OWNER_POSITION_VECTOR, &args![owner]).u32();
    if !near_zero_vector(e, position, tolerance) {
        return 1;
    }
    let rotation = e.call(OWNER_ROTATION_VECTOR, &args![owner]).u32();
    if near_zero_vector(e, rotation, tolerance) {
        0
    } else {
        1
    }
}

// Translated from 00c6b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Add` (Xbox PDB): adds an entity (kind byte `+0x28` equal to 1)
/// or a phantom (kind 2) to the world. Needs `this` and its `hkpWorld`, and
/// an owner (word `+0xC`, resolved as the entity or phantom owner) whose flags
/// have neither bit 4 nor bit 5 set. Without batching (`iBatchAdd` zero) an
/// entity is added at once (`00c914d0`, flag from `entity_add_flag`) and a
/// phantom at once (`00c936a0`). With batching the owner is marked (flag bit
/// 4) and the object is referenced and queued: entities in `pEntityBatch` (a
/// full batch of 3000 flushes the entity tail and the other batches first),
/// phantoms in `pPhantomBatch` (a full batch of 200 flushes the other batches
/// first, and the entity tail when entities are pending and no partial flush
/// ran yet).
pub fn bhk_world_add(e: &mut Engine, this: Ptr<BhkWorld>, object: Ptr) {
    if this.is_null() {
        return;
    }
    let world = hk_world(e, this.cast());
    if world == 0 {
        return;
    }
    let batching = e.get(this, BhkWorld::iBatchAdd) != 0;
    if e.mem.u8(object.addr() + 0x28) == 1 {
        let owner = e.mem.u32(object.addr() + 0xc);
        if owner == 0 || e.mem.u8(owner + 0x10) & 0x30 != 0 {
            return;
        }
        if !batching {
            let flag = entity_add_flag(e, owner, object);
            e.call(WORLD_ADD_ENTITY, &args![world, object, flag]);
        } else {
            if e.get(this, BhkWorld::iEntityBatchCount) >= 3000 {
                flush_entity_batch_partially(e, this);
                flush_pending_batches(e, this);
            }
            let flags = e.mem.u32(owner + 0x10);
            e.mem.set_u32(owner + 0x10, flags | 0x10);
            e.call(HAVOK_ADD_REFERENCE, &args![object]);
            let count = e.get(this, BhkWorld::iEntityBatchCount);
            let array = e.get(this, BhkWorld::pEntityBatch).addr();
            e.mem.set_u32(array + count * 4, object.addr());
            e.set(this, BhkWorld::iEntityBatchCount, count.wrapping_add(1));
        }
        return;
    }
    let Some(owner) = owner_of_kind(e, object.addr(), 2) else {
        return;
    };
    if e.mem.u8(owner + 0x14) & 0x30 != 0 {
        return;
    }
    if !batching {
        e.call(WORLD_ADD_PHANTOM, &args![world, object]);
        return;
    }
    if e.get(this, BhkWorld::iPhantomBatchCount) >= 200 {
        if e.get(this, BhkWorld::iEntityBatchCount) != 0
            && e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE) == 0
        {
            flush_entity_batch_partially(e, this);
        }
        flush_pending_batches(e, this);
    }
    let flags = e.mem.u8(owner + 0x14);
    e.mem.set_u8(owner + 0x14, flags | 0x10);
    e.call(HAVOK_ADD_REFERENCE, &args![object]);
    let count = e.get(this, BhkWorld::iPhantomBatchCount);
    let array = e.get(this, BhkWorld::pPhantomBatch).addr();
    e.mem.set_u32(array + count * 4, object.addr());
    e.set(this, BhkWorld::iPhantomBatchCount, count.wrapping_add(1));
}

// Translated from 00c6b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Add_ov2` (Xbox PDB): adds a constraint instance. Needs `this`
/// and its `hkpWorld`. Without batching the constraint is added at once
/// (`00c97d30`) if it has no world yet (word `+8`) and both of its entities
/// (words `+0x14`, `+0x18`), when present, have an island (word `+0xCC`).
/// With batching it is referenced and queued in `pConstraintInstanceBatch`;
/// a full batch of 200 first flushes the entity batch (`fn_00c68a40`) and the
/// other pending batches.
pub fn bhk_world_add_ov2(e: &mut Engine, this: Ptr<BhkWorld>, constraint: Ptr) {
    if this.is_null() {
        return;
    }
    let world = hk_world(e, this.cast());
    if world == 0 {
        return;
    }
    if e.get(this, BhkWorld::iBatchAdd) == 0 {
        if constraint.is_null() || e.mem.u32(constraint.addr() + 8) != 0 {
            return;
        }
        for offset in [0x14u32, 0x18] {
            let entity = e.mem.u32(constraint.addr() + offset);
            if entity != 0 && e.mem.u32(entity + 0xcc) == 0 {
                return;
            }
        }
        e.call(WORLD_ADD_CONSTRAINT, &args![world, constraint]);
    } else {
        if e.get(this, BhkWorld::iConstraintInstanceBatchCount) >= 200 {
            fn_00c68a40(e, this);
            flush_pending_batches(e, this);
        }
        e.call(HAVOK_ADD_REFERENCE, &args![constraint]);
        let count = e.get(this, BhkWorld::iConstraintInstanceBatchCount);
        let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
        e.mem.set_u32(array + count * 4, constraint.addr());
        e.set(
            this,
            BhkWorld::iConstraintInstanceBatchCount,
            count.wrapping_add(1),
        );
    }
}

// Translated from 00c6b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorld::Add_ov3` (Xbox PDB): adds an action. Needs `this` and its
/// `hkpWorld`. Without batching the action is added at once (`00c97370`);
/// with batching it is referenced and queued in `pActionBatch`, after a full
/// batch of 100 flushed the entity tail (when entities are pending and no
/// partial flush ran yet) and the other pending batches. Then the action's
/// object at `+0x10`, if any, is put into the action list at `+0x90`
/// (`fn_00c68b50`) under a temporary reference.
// Translation incomplete: 00c6b3c0 - SEH exception unwinding is unsupported.
pub fn bhk_world_add_ov3(e: &mut Engine, this: Ptr<BhkWorld>, action: Ptr) {
    let scope = enter_scope(e);
    if !this.is_null() {
        let world = hk_world(e, this.cast());
        if world != 0 {
            if e.get(this, BhkWorld::iBatchAdd) == 0 {
                e.call(WORLD_ADD_ACTION, &args![world, action]);
            } else {
                if e.get(this, BhkWorld::iActionBatchCount) >= 100 {
                    if e.get(this, BhkWorld::iEntityBatchCount) != 0
                        && e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE) == 0
                    {
                        flush_entity_batch_partially(e, this);
                    }
                    flush_pending_batches(e, this);
                }
                e.call(HAVOK_ADD_REFERENCE, &args![action]);
                let count = e.get(this, BhkWorld::iActionBatchCount);
                let array = e.get(this, BhkWorld::pActionBatch).addr();
                e.mem.set_u32(array + count * 4, action.addr());
                e.set(this, BhkWorld::iActionBatchCount, count.wrapping_add(1));
            }
            let inner = if action.is_null() {
                0
            } else {
                e.mem.u32(action.addr() + 0x10)
            };
            if inner != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), inner);
                    e.call(INTERLOCKED_INCREMENT, &args![inner + 4]);
                    fn_00c68b50(e, Ptr::new(this.addr() + 0x90), slot);
                    let count = e.call(INTERLOCKED_DECREMENT, &args![inner + 4]).i32();
                    if count == 0 {
                        e.vcall(inner, 4, &[]);
                    }
                });
            }
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c66250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afe90). An entry of
/// the callback table at `011afe88`; `00c68900` calls such entries as `callback(collision_object,
/// request)` (cdecl, two stack words) on the `bhkNiCollisionObject` that a scene node's `+0x1c`
/// object answers to virtual slot `0x38`. This one reads only the first word: sets bit 6 (`0x40`,
/// the `bReset` flag of `m_uFlags`) of the collision object.
pub fn fn_00c66250(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, _unused_1: u32) {
    let flags = e.get(collision_object, BhkNiCollisionObject::m_uFlags);
    e.set(
        collision_object,
        BhkNiCollisionObject::m_uFlags,
        flags | 0x40,
    );
}

// Translated from 00c66260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afe94). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Sets
/// bit 0 (the `bActive` flag of `m_uFlags`) when the word at `request + 0xc` is not 0, clears it
/// otherwise; the other flag bits stay.
pub fn fn_00c66260(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let value = e.mem.u32(request.addr().wrapping_add(0xc));
    let flags = e.get(collision_object, BhkNiCollisionObject::m_uFlags);
    if value != 0 {
        e.set(
            collision_object,
            BhkNiCollisionObject::m_uFlags,
            flags | 0x1,
        );
    } else {
        e.set(
            collision_object,
            BhkNiCollisionObject::m_uFlags,
            flags & 0xfffe,
        );
    }
}

// Translated from 00c66280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afe9c). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Calls
/// the collision object's virtual slot `0xb0` with `(request[0xc], 0, request[0x10] != 0)` and
/// answers the result. The Xbox PDB lists slot `0xb0` of `bhkNiCollisionObject` as
/// `SetMotionType`.
pub fn fn_00c66280(
    e: &mut Engine,
    collision_object: Ptr<BhkNiCollisionObject>,
    request: Ptr,
) -> u32 {
    let flag = u32::from(e.mem.u32(request.addr().wrapping_add(0x10)) != 0);
    let value = e.mem.u32(request.addr().wrapping_add(0xc));
    e.vcall(collision_object.addr(), 0xb0, &args![value, 0u32, flag])
        .u32()
}

// Translated from 00c662b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afea0). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Calls
/// the collision object's virtual slot `0xc0` with `request[0xc] != 0` and answers the result. The
/// Xbox PDB lists slot `0xc0` of `bhkNiCollisionObject` as `SetDebugDisplay`.
pub fn fn_00c662b0(
    e: &mut Engine,
    collision_object: Ptr<BhkNiCollisionObject>,
    request: Ptr,
) -> u32 {
    let flag = u32::from(e.mem.u32(request.addr().wrapping_add(0xc)) != 0);
    e.vcall(collision_object.addr(), 0xc0, &args![flag]).u32()
}

// Translated from 00c662d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afebc). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`).
/// Does nothing for a null object. Otherwise asks the object's virtual slot `0x3c` (the Xbox PDB
/// lists `IsBhkBlendCollisionObject` there) for a second object and, when that is not null,
/// tail-jumps to `00c81b40` (when the word at `request + 0xc` is 0) or to `00c81ae0` (otherwise),
/// both thiscall on that second object with no stack words.
pub fn fn_00c662d0(e: &mut Engine, collision_object: Ptr, request: Ptr) {
    if collision_object.is_null() {
        return;
    }
    let blend = e.vcall(collision_object.addr(), 0x3c, &args![]).u32();
    if blend == 0 {
        return;
    }
    if e.mem.u32(request.addr().wrapping_add(0xc)) == 0 {
        e.call(0x00c8_1b40, &args![blend]);
    } else {
        e.call(0x00c8_1ae0, &args![blend]);
    }
}

// Translated from 00c67110 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afe8c). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`).
/// Reads only the first word: when the world object (`spWorldObject`, `+0x10`) is not null,
/// tail-jumps to its virtual slot `0xa0` (the Xbox PDB lists `Remove` there for `bhkWorldObject`).
pub fn fn_00c67110(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, _unused_1: u32) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    e.vcall(world_object.addr(), 0xa0, &args![]);
}

// Translated from 00c67130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afec0). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). When the
/// world object (`spWorldObject`, `+0x10`) is not null, calls `00c85d20` (thiscall on it, `RET 4`)
/// with `request[0xc] != 0`.
pub fn fn_00c67130(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let flag = u32::from(e.mem.u32(request.addr().wrapping_add(0xc)) != 0);
    e.call(0x00c8_5d20, &args![world_object, flag]);
}

// Translated from 00c67150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afec4). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Asks
/// the world object (`spWorldObject`, `+0x10`, when not null) for its object through virtual slot
/// `0x40` (the Xbox PDB lists `IsBhkRigidBody` there) and, when that is not null, copies the float
/// at `request + 0xc` to `+0x94` of the Havok object at `+8` of the answer (a `FLD`/`FSTP` copy,
/// which quiets a signalling NaN).
pub fn fn_00c67150(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let havok_object = e.mem.u32(body.wrapping_add(8));
    let value = x87_quiet_bits(e.mem.u32(request.addr().wrapping_add(0xc)));
    e.mem.set_u32(havok_object.wrapping_add(0x94), value);
}

// Translated from 00c67180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afec8). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Asks
/// the world object (`spWorldObject`, `+0x10`, when not null) for its object through virtual slot
/// `0x40` (the Xbox PDB lists `IsBhkRigidBody` there) and, when that is not null, copies the float
/// at `request + 0xc` to `+0x90` of the Havok object at `+8` of the answer (a `FLD`/`FSTP` copy,
/// which quiets a signalling NaN).
pub fn fn_00c67180(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let havok_object = e.mem.u32(body.wrapping_add(8));
    let value = x87_quiet_bits(e.mem.u32(request.addr().wrapping_add(0xc)));
    e.mem.set_u32(havok_object.wrapping_add(0x90), value);
}

// Translated from 00c671b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afeb8). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Does
/// nothing for a null collision object. Otherwise asks the world object (`spWorldObject`, `+0x10`,
/// when not null) for its object through virtual slot `0x40` (the Xbox PDB lists `IsBhkRigidBody`
/// there) and, when that is not null, calls `00561580` (thiscall on it, `RET 4`) with
/// `request[0xc] != 0`.
pub fn fn_00c671b0(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    if collision_object.is_null() {
        return;
    }
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let flag = u32::from(e.mem.u32(request.addr().wrapping_add(0xc)) != 0);
    e.call(0x0056_1580, &args![body, flag]);
}

// Translated from 00c671f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afe98; Ghidra's
/// listing of this range is misaligned, the bytes were decoded by hand). An entry of the callback
/// table at `011afe88` (`callback(collision_object, request)`, see `00c66250`); it reads only the
/// first word. Takes the scene object (`m_pkSceneObject`, `+8`) and does nothing when it is null.
/// Otherwise the object held at `+0x1c` of the scene object (when not null) is released
/// (`InterlockedDecrement` of its count at `+4`; virtual slot `4` when the count reaches 0) and
/// the field is set to null. The exe then reads the field again and, when it is not null and that
/// object's `+8` is not the scene object, calls its virtual slot `0x8c` with the scene object;
/// the field was just null, so that last step is never taken.
pub fn fn_00c671f0(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, _unused_1: u32) {
    let scene_object = e.get(collision_object, BhkNiCollisionObject::m_pkSceneObject);
    if scene_object.is_null() {
        return;
    }
    let me = scene_object.addr();
    let held = e.mem.u32(me.wrapping_add(0x1c));
    if held != 0 {
        if e.call(0x00fd_f1c4, &args![held.wrapping_add(4)]).u32() == 0 {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(me.wrapping_add(0x1c), 0);
    }
    let again = e.mem.u32(me.wrapping_add(0x1c));
    if again != 0 && e.mem.u32(again.wrapping_add(8)) != me {
        e.vcall(again, 0x8c, &args![me]);
    }
}

// Translated from 00c67240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afea4). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). When the
/// world object (`spWorldObject`, `+0x10`) is not null, calls `00c85ce0` (thiscall on it, `RET 4`)
/// with the word at `request + 0xc`.
pub fn fn_00c67240(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let value = e.mem.u32(request.addr().wrapping_add(0xc));
    e.call(0x00c8_5ce0, &args![world_object, value]);
}

// Translated from 00c67260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afea8). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Asks
/// the world object (`spWorldObject`, `+0x10`, when not null) for its object through virtual slot
/// `0x40` (the Xbox PDB lists `IsBhkRigidBody` there). When that is not null: if its Havok object
/// (`+8`) is not null, calls `00c9bff0` (thiscall on the Havok object, `RET 4`) with the address of
/// the stack word that holds `collision_object` (the first argument slot itself) and, when the byte
/// it answers the address of is not 0, increments the word at `request + 0xc`; in every case it
/// then sets the byte at `request + 4` to 0.
pub fn fn_00c67260(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let havok_object = e.mem.u32(body.wrapping_add(8));
    if havok_object != 0 {
        // The out byte is the frame's argument slot (`ESP+8`): `00c9bff0` writes the state
        // there and answers its address, and the byte is tested through that address, so the
        // test is made while the slot is there. (Scoped to `00c9bff0` alone, the byte was read
        // from a freed block.)
        let active = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), collision_object.addr());
            let found = e.call(0x00c9_bff0, &args![havok_object, slot]).u32();
            e.mem.u8(found) != 0
        });
        if active {
            let count = e.mem.u32(request.addr().wrapping_add(0xc));
            e.mem
                .set_u32(request.addr().wrapping_add(0xc), count.wrapping_add(1));
        }
    }
    e.mem.set_u8(request.addr().wrapping_add(4), 0);
}

// Translated from 00c672a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afeac). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`).
/// Changes the low seven bits of the word at `+0x2c` of the world object's Havok object (`+8` of
/// `spWorldObject`; read as 0 when there is none). With `new = request[0x10]` and `expected =
/// request[0xc]`: does nothing when `new` equals the current low seven bits, or when `expected`
/// is not 0 and differs from them. Otherwise, with a Havok object, it calls `00a29680` twice on the
/// world object (thiscall, no stack words) and stores the word with its low seven bits replaced by
/// `new`'s; in every case it ends by tail-jumping to the world object's virtual slot `0xc4` (the
/// Xbox PDB lists `UpdateCFilter` there for `bhkWorldObject`).
pub fn fn_00c672a0(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e
        .get(collision_object, BhkNiCollisionObject::spWorldObject)
        .addr();
    let new_bits = e.mem.u32(request.addr().wrapping_add(0x10));
    let expected = e.mem.u32(request.addr().wrapping_add(0xc));
    let havok_object = e.mem.u32(world_object.wrapping_add(8));
    let mut word = 0;
    if havok_object != 0 {
        let member = havok_object.wrapping_add(0x10);
        if member != 0 {
            word = e.mem.u32(member.wrapping_add(0x1c));
        }
    }
    let current = word & 0x7f;
    if new_bits == current {
        return;
    }
    if expected != 0 && expected != current {
        return;
    }
    let updated = ((word ^ new_bits) & 0x7f) ^ word;
    if havok_object != 0 {
        e.call(0x00a2_9680, &args![world_object]);
        let member = havok_object.wrapping_add(0x10);
        e.call(0x00a2_9680, &args![world_object]);
        if member != 0 {
            e.mem.set_u32(member.wrapping_add(0x1c), updated);
        }
    }
    e.vcall(world_object, 0xc4, &args![]);
}

// Translated from 00c68990 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afecc). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`). Asks
/// the world object (`spWorldObject`, `+0x10`, when not null) for its object through virtual slot
/// `0x40` (the Xbox PDB lists `IsBhkRigidBody` there) and, when that is not null, sets bit 6
/// (`0x40`) of the word at its `+0x10` when the word at `request + 0xc` is not 0, clears it
/// otherwise.
pub fn fn_00c68990(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, request: Ptr) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let flags = e.mem.u32(body.wrapping_add(0x10));
    if e.mem.u32(request.addr().wrapping_add(0xc)) != 0 {
        e.mem.set_u32(body.wrapping_add(0x10), flags | 0x40);
    } else {
        e.mem.set_u32(body.wrapping_add(0x10), flags & 0xffff_ffbf);
    }
}

// Translated from 00c689c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011afeb0). An entry of
/// the callback table at `011afe88` (`callback(collision_object, request)`, see `00c66250`); it
/// reads only the first word. Asks the world object (`spWorldObject`, `+0x10`, when not null) for
/// its object through virtual slot `0x40` (the Xbox PDB lists `IsBhkRigidBody` there). When that is
/// not null and its Havok object (`+8`) is null or has the byte `5` at `+0xe8`, calls the answer's
/// virtual slot `0xe4` with the argument `4`.
pub fn fn_00c689c0(e: &mut Engine, collision_object: Ptr<BhkNiCollisionObject>, _unused_1: u32) {
    let world_object = e.get(collision_object, BhkNiCollisionObject::spWorldObject);
    if world_object.is_null() {
        return;
    }
    let body = e.vcall(world_object.addr(), 0x40, &args![]).u32();
    if body == 0 {
        return;
    }
    let havok_object = e.mem.u32(body.wrapping_add(8));
    if havok_object != 0 && e.mem.u8(havok_object.wrapping_add(0xe8)) != 5 {
        return;
    }
    e.vcall(body, 0xe4, &args![4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_6170, bhk_memory_allocator_block_alloc(Ptr, u32) -> Ptr),
        entry!(0x00c6_61a0, bhk_memory_allocator_block_free(Ptr, Ptr, u32)),
        entry!(0x00c6_61c0, bhk_world_return_second_argument(Ptr, u32, u32) -> u32),
        entry!(0x00c6_61d0, bhk_world_set_update_mode(u32)),
        entry!(0x00c6_6210, bhk_world_reset_step_accumulators()),
        entry!(0x00c6_6230, bhk_world_set_pick_count_override(u8)),
        entry!(0x00c6_6240, bhk_world_set_hk_object(Ptr<BhkWorld>, u32)),
        entry!(0x00c6_6300, bhk_world_increment_batch_add(Ptr)),
        entry!(0x00c6_6310, bhk_world_increment_batch_remove(Ptr)),
        entry!(
            0x00c6_6320,
            hk_quaternion_set_mul(Ptr<HkQuaternion>, Ptr<HkQuaternion>, Ptr<HkQuaternion>)
        ),
        entry!(0x00c6_63d0, hk_ref_counted_pointer_assign(Ptr, Ptr) -> Ptr),
        entry!(0x00c6_6420, hkp_collidable_collidable_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6450, hkp_ray_collidable_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6480, hkp_shape_collection_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_64b0, hkp_ray_shape_collection_filter_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_64e0, hkp_broad_phase_cast_collector_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6510, ahkp_world_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6540, bhk_memory_allocator_delete(Ptr, u32) -> Ptr),
        entry!(0x00c6_6560, hk_memory_allocator_delete(Ptr, u32) -> Ptr),
        entry!(0x00c6_6570, bhk_on_startup_callback_invoke()),
        entry!(0x00c6_6600, bhk_on_startup_callback_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_6630, bhk_on_startup_callback_delete_adjustor_8(Ptr, u8) -> Ptr),
        entry!(0x00c6_6640, bhk_on_startup_callback_delete_adjustor_12(Ptr, u8) -> Ptr),
        entry!(0x00c6_6650, bhk_on_startup_callback_delete_adjustor_16(Ptr, u8) -> Ptr),
        entry!(0x00c6_6660, bhk_on_startup_callback_delete_adjustor_20(Ptr, u8) -> Ptr),
        entry!(0x00c6_6670, bhk_world_shutdown()),
        entry!(0x00c6_6760, bhk_world_accumulate_step(u32, u8, u8)),
        entry!(0x00c6_6a00, bhk_world_scale_solver_info(Ptr)),
        entry!(0x00c6_6be0, bhk_world_scale_origin(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6c00, bhk_world_shift_broad_phase(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6c90, bhk_world_init(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6de0, bhk_world_link(Ptr<BhkWorld>, u32)),
        entry!(0x00c6_6e20, bhk_world_set_solver_value(Ptr<BhkWorld>, f32)),
        entry!(0x00c6_6e70, bhk_world_kill_cinfo(Ptr<BhkWorld>, u8)),
        entry!(0x00c6_6f10, bhk_world_assign_collision_group(Ptr, Ptr) -> bool),
        entry!(
            0x00c6_6f90,
            bhk_world_has_active_items(Ptr<BhkWorld>) -> bool
        ),
        entry!(
            0x00c6_6fb0,
            bhk_pick_data_get_ni_av_object(Ptr<BhkPickData>) -> Ptr
        ),
        entry!(
            0x00c6_6fd0,
            bhk_pick_data_get_hk_rigid_body(Ptr<BhkPickData>) -> Ptr
        ),
        entry!(0x00c6_6ff0, bhk_collision_agent_update(Ptr, Ptr)),
        entry!(
            0x00c6_7320,
            bhk_collision_agent_update_shape_transform(Ptr, Ptr)
        ),
        entry!(0x00c6_7430, fn_00c67430(Ptr, u32) -> bool),
        entry!(0x00c6_74d0, fn_00c674d0(Ptr, Ptr, u32)),
        entry!(0x00c6_76e0, fn_00c676e0(Ptr<BhkWorld>)),
        entry!(0x00c6_7850, fn_00c67850(Ptr<BhkWorld>)),
        entry!(0x00c6_79a0, fn_00c679a0(Ptr<BhkWorld>)),
        entry!(
            0x00c6_7aa0,
            bhk_world_remove_batched_entities_partial(Ptr<BhkWorld>)
        ),
        entry!(0x00c6_7bc0, fn_00c67bc0(Ptr<BhkWorld>)),
        entry!(0x00c6_7cd0, fn_00c67cd0(Ptr<BhkWorld>)),
        entry!(0x00c6_7df0, fn_00c67df0(Ptr<BhkWorld>)),
        entry!(0x00c6_7f20, fn_00c67f20(Ptr)),
        entry!(0x00c6_7fc0, fn_00c67fc0(Ptr) -> Ptr),
        entry!(0x00c6_7fe0, fn_00c67fe0(Ptr) -> Ptr),
        entry!(
            0x00c6_8050,
            bhk_tasklet_manager_callback_bhk_thread_state_create_thread_memory(Ptr)
        ),
        entry!(0x00c6_8150, fn_00c68150(Ptr)),
        entry!(0x00c6_81b0, bhk_on_startup_callback_operator()),
        entry!(0x00c6_81c0, fn_00c681c0(Ptr) -> Ptr),
        entry!(0x00c6_8230, bhk_world_init_system()),
        entry!(0x00c6_8750, fn_00c68750(Ptr<BhkWorld>, Ptr)),
        entry!(
            0x00c6_87b0,
            bhk_world_create_cinfo(Ptr<BhkWorld>, Ptr) -> Ptr
        ),
        entry!(0x00c6_88b0, bhk_world_update_debug_color(Ptr<BhkWorld>)),
        entry!(0x00c6_8900, fn_00c68900(Ptr, Ptr, u32)),
        entry!(0x00c6_8a00, fn_00c68a00(Ptr<BhkWorld>, u8)),
        entry!(0x00c6_8a40, fn_00c68a40(Ptr<BhkWorld>)),
        entry!(0x00c6_8ae0, fn_00c68ae0(Ptr)),
        entry!(0x00c6_8b50, fn_00c68b50(Ptr, Ptr)),
        entry!(0x00c6_8bc0, bhk_world_cinfo_delete(Ptr, u8) -> Ptr),
        entry!(0x00c6_8bf0, bhk_world_remove(Ptr<BhkWorld>, Ptr) -> bool),
        entry!(
            0x00c6_8cb0,
            bhk_world_remove_ov2(Ptr<BhkWorld>, Ptr) -> bool
        ),
        entry!(
            0x00c6_8d50,
            bhk_world_remove_ov3(Ptr<BhkWorld>, Ptr) -> bool
        ),
        entry!(0x00c6_8de0, bhk_world_update_debugger(Ptr<BhkWorld>)),
        entry!(
            0x00c6_8e30,
            bhk_world_set_debug_object_color(Ptr<BhkWorld>, Ptr, u32)
        ),
        entry!(0x00c6_8e90, fn_00c68e90(Ptr, Ptr)),
        entry!(0x00c6_8ec0, fn_00c68ec0(Ptr, Ptr, u32) -> bool),
        entry!(0x00c6_8f00, fn_00c68f00(Ptr<BhkWorld>, u8)),
        entry!(
            0x00c6_8f50,
            bs_simple_list_ni_pointer_bhk_action_remove(Ptr, Ptr)
        ),
        entry!(
            0x00c6_9010,
            fn_00c69010(Ptr<BhkWorld>, Ptr) -> Ptr<BhkWorld>
        ),
        entry!(0x00c6_9220, bhk_world_get_rtti(Ptr<BhkWorld>) -> u32),
        entry!(0x00c6_9230, bhk_world_q_cinfo_size(Ptr<BhkWorld>) -> u32),
        entry!(
            0x00c6_9240,
            bhk_world_get_viewer_strings(Ptr<BhkWorld>, Ptr)
        ),
        entry!(0x00c6_9580, fn_00c69580(Ptr<BhkWorld>, Ptr) -> bool),
        entry!(
            0x00c6_96d0,
            bhk_world_pick_object(Ptr<BhkWorld>, Ptr) -> bool
        ),
        entry!(
            0x00c6_9960,
            bhk_world_add_objects_and_move_controller(Ptr<BhkWorld>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c6_9e10,
            bhk_world_add_objects(Ptr<BhkWorld>, Ptr, u8, u8, u32, u8) -> bool
        ),
        entry!(0x00c6_9ee0, bhk_world_remove_objects(Ptr, u8, u8) -> u8),
        entry!(0x00c6_9f50, bhk_world_update_position(Ptr, u8, u8) -> bool),
        entry!(
            0x00c6_9fc0,
            bhk_world_adjust_force_add_count(Ptr, u8, u8, u8) -> bool
        ),
        entry!(0x00c6_a040, fn_00c6a040(Ptr, u8, u8, u8) -> bool),
        entry!(0x00c6_a0b0, fn_00c6a0b0(Ptr, u8, u8, u8) -> bool),
        entry!(0x00c6_a120, fn_00c6a120(Ptr, f32, u8, u8) -> bool),
        entry!(0x00c6_a190, fn_00c6a190(Ptr, f32, u8, u8) -> bool),
        entry!(0x00c6_a200, bhk_world_set_disabled(Ptr, u8, u8, u8) -> bool),
        entry!(0x00c6_a270, bhk_world_activate(Ptr, u8, u8, u8) -> bool),
        entry!(0x00c6_a2e0, bhk_world_kill_havok(Ptr, u8, u8) -> bool),
        entry!(
            0x00c6_a350,
            bhk_world_set_motion(Ptr, u32, u8, u8, u8) -> bool
        ),
        entry!(0x00c6_a3d0, fn_00c6a3d0(Ptr, u32, u8, u8) -> bool),
        entry!(0x00c6_a440, fn_00c6a440(Ptr, u8, u8) -> bool),
        entry!(0x00c6_a4c0, fn_00c6a4c0(Ptr, u32, u32, u8, u8) -> bool),
        entry!(0x00c6_a540, bhk_world_update_actions(Ptr<BhkWorld>, f32)),
        entry!(0x00c6_a620, fn_00c6a620(Ptr, u32)),
        entry!(
            0x00c6_a640,
            bhk_world_set_visual_debugger(Ptr<BhkWorld>, u8)
        ),
        entry!(
            0x00c6_a810,
            bhk_world_set_debug_display(Ptr<BhkWorld>, Ptr, u8)
        ),
        entry!(0x00c6_a870, bhk_world_deactivate_all_islands(Ptr)),
        entry!(0x00c6_aac0, fn_00c6aac0(Ptr<BhkWorld>)),
        entry!(0x00c6_aca0, fn_00c6aca0(Ptr<BhkWorld>)),
        entry!(0x00c6_ae70, bhk_world_update(Ptr<BhkWorld>) -> bool),
        entry!(
            0x00c6_b070,
            bhk_world_scalar_deleting_destructor(Ptr<BhkWorld>, u8) -> Ptr<BhkWorld>
        ),
        entry!(0x00c6_b0a0, bhk_world_add(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_b320, bhk_world_add_ov2(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_b3c0, bhk_world_add_ov3(Ptr<BhkWorld>, Ptr)),
        entry!(0x00c6_6250, fn_00c66250(Ptr<BhkNiCollisionObject>, u32)),
        entry!(0x00c6_6260, fn_00c66260(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(
            0x00c6_6280,
            fn_00c66280(Ptr<BhkNiCollisionObject>, Ptr) -> u32
        ),
        entry!(
            0x00c6_62b0,
            fn_00c662b0(Ptr<BhkNiCollisionObject>, Ptr) -> u32
        ),
        entry!(0x00c6_62d0, fn_00c662d0(Ptr, Ptr)),
        entry!(0x00c6_7110, fn_00c67110(Ptr<BhkNiCollisionObject>, u32)),
        entry!(0x00c6_7130, fn_00c67130(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_7150, fn_00c67150(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_7180, fn_00c67180(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_71b0, fn_00c671b0(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_71f0, fn_00c671f0(Ptr<BhkNiCollisionObject>, u32)),
        entry!(0x00c6_7240, fn_00c67240(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_7260, fn_00c67260(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_72a0, fn_00c672a0(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_8990, fn_00c68990(Ptr<BhkNiCollisionObject>, Ptr)),
        entry!(0x00c6_89c0, fn_00c689c0(Ptr<BhkNiCollisionObject>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn block_alloc_adds_slack_and_stores_the_alignment_distance() {
        let mut e = engine();
        let raw = 0x0300_0003;
        e.map(0x0300_0000, 0x1000);
        e.register_double(HEAP_ALLOC, move |_, _| ret(raw));
        e.call_log = Some(vec![]);
        let block = e.call(0x00c6_6170, &args![0x5555u32, 0x20u32]).ptr::<()>();
        assert_eq!(block.addr(), raw + 0xd);
        assert_eq!(e.mem.u8(block.addr() - 1), 0xd);
        let call = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == HEAP_ALLOC)
            .unwrap();
        assert_eq!(call.1, [HEAP_SINGLETON, 0x30]);
    }

    #[test]
    fn block_free_recovers_the_raw_address_and_skips_null() {
        let mut e = engine();
        e.register_double(HEAP_FREE, move |_, _| Ret::default());
        let block = 0x0300_1010;
        e.map(0x0300_1000, 0x1000);
        e.mem.set_u8(block - 1, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c6_61a0, &args![0x5555u32, block, 0xfeedu32]);
        e.call(0x00c6_61a0, &args![0x5555u32, 0u32, 0xfeedu32]);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == HEAP_FREE)
            .collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, [HEAP_SINGLETON, block - 8]);
    }

    #[test]
    fn second_argument_helper_returns_the_second_word() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00c6_61c0, &args![0x5555u32, 17u32, 29u32]).u32(),
            29
        );
    }

    #[test]
    fn world_mode_mapping_matches_the_three_executable_cases() {
        let mut e = engine();
        for (input, expected) in [(0, 1), (1, 2), (4, 3), (2, 2), (u32::MAX, 2)] {
            e.call(0x00c6_61d0, &args![input]);
            assert_eq!(e.global::<u32>(GLOBAL_WORLD_MODE), expected);
        }
    }

    #[test]
    fn reset_world_step_clears_both_accumulators() {
        let mut e = engine();
        e.set_global(GLOBAL_WORLD_INTEGER, 17u32);
        e.set_global(GLOBAL_WORLD_FLOAT, 1.25f32);
        e.call(0x00c6_6210, &[]);
        assert_eq!(e.global::<u32>(GLOBAL_WORLD_INTEGER), 0);
        assert_eq!(e.global::<f32>(GLOBAL_WORLD_FLOAT), 0.0);
    }

    #[test]
    fn pick_count_override_stores_the_low_byte() {
        let mut e = engine();
        e.call(0x00c6_6230, &args![0x1ffu32]);
        assert_eq!(e.global::<u8>(GLOBAL_PICK_COUNT_OVERRIDE), 0xff);
    }

    #[test]
    fn set_hk_object_forwards_this_to_the_base_method() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        e.register_double(0x00c8_7c50, move |_, a| {
            assert_eq!(a, [this.addr(), 0x5555_0001]);
            Ret::default()
        });
        e.call(0x00c6_6240, &args![this, 0x5555_0001u32]);
    }

    #[test]
    fn first_batch_counter_wraps() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, u32::MAX);
        e.call(0x00c6_6300, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0);
    }

    #[test]
    fn second_batch_counter_increments_at_its_own_offset() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, 6);
        e.mem.set_u32(this.addr() + 0x1c, 9);
        e.call(0x00c6_6310, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 6);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 10);
    }

    #[test]
    fn quaternion_product_writes_the_sse_product() {
        let mut e = engine();
        let out: Ptr<HkQuaternion> = e.new_object();
        let left: Ptr<HkQuaternion> = e.new_object();
        let right: Ptr<HkQuaternion> = e.new_object();
        e.set(left, HkQuaternion::x, 1.0);
        e.set(left, HkQuaternion::y, 2.0);
        e.set(left, HkQuaternion::z, 3.0);
        e.set(left, HkQuaternion::w, 4.0);
        e.set(right, HkQuaternion::x, 5.0);
        e.set(right, HkQuaternion::y, 6.0);
        e.set(right, HkQuaternion::z, 7.0);
        e.set(right, HkQuaternion::w, 8.0);
        e.call(0x00c6_6320, &args![out, left, right]);
        assert_eq!(e.get(out, HkQuaternion::x), 24.0);
        assert_eq!(e.get(out, HkQuaternion::y), 48.0);
        assert_eq!(e.get(out, HkQuaternion::z), 48.0);
        assert_eq!(e.get(out, HkQuaternion::w), -6.0);
    }

    #[test]
    fn shared_pointer_assignment_releases_old_and_retains_new() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        let old = e.mem.alloc(8);
        let new = e.mem.alloc(8);
        e.map(0x0400_0000, 0x1000);
        e.mem.set_u32(this, old);
        e.mem.set_u32(source, new);
        e.mem.set_u32(old, 0x0400_0000);
        e.mem.set_i32(old + 4, 1);
        e.mem.set_i32(new + 4, 4);
        e.put_vtable(0x0400_0000, &[0, 0x0400_0100]);
        e.register_double(0x00fd_f1c4, move |e, a| {
            let n = e.mem.i32(a[0]) - 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x00fd_f1c0, move |e, a| {
            let n = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], old);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_63d0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), new);
        assert_eq!(e.mem.i32(old + 4), 0);
        assert_eq!(e.mem.i32(new + 4), 5);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0400_0100));
    }

    #[test]
    fn shared_pointer_assignment_rereads_the_source_after_the_release() {
        // 00c663fa: the source word is read again after the delete, so a
        // delete that rewrites it changes what is stored and retained.
        let mut e = engine();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        let old = e.mem.alloc(8);
        let new = e.mem.alloc(8);
        let replacement = e.mem.alloc(8);
        e.map(0x0400_0000, 0x1000);
        e.mem.set_u32(this, old);
        e.mem.set_u32(source, new);
        e.mem.set_u32(old, 0x0400_0000);
        e.mem.set_i32(old + 4, 1);
        e.mem.set_i32(new + 4, 4);
        e.mem.set_i32(replacement + 4, 2);
        e.put_vtable(0x0400_0000, &[0, 0x0400_0100]);
        e.register_double(0x00fd_f1c4, move |e, a| {
            let n = e.mem.i32(a[0]) - 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x00fd_f1c0, move |e, a| {
            let n = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], n);
            ret(n as u32)
        });
        e.register_double(0x0400_0100, move |e, a| {
            assert_eq!(a[0], old);
            e.mem.set_u32(source, replacement);
            Ret::default()
        });
        assert_eq!(e.call(0x00c6_63d0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), replacement);
        assert_eq!(e.mem.i32(old + 4), 0);
        assert_eq!(e.mem.i32(replacement + 4), 3);
        assert_eq!(e.mem.i32(new + 4), 4);
    }

    fn assert_filter_destructor(address: u32, expected_vtable: u32) {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.register_double(HEAP_FREE, move |_, a| {
            assert_eq!(a, [HEAP_SINGLETON, this]);
            Ret::default()
        });
        e.call(address, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), expected_vtable);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == HEAP_FREE)
                .count(),
            1
        );
    }

    #[test]
    fn collidable_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6420, 0x010c_3b80);
    }

    #[test]
    fn ray_collidable_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6450, 0x010c_3b8c);
    }

    #[test]
    fn shape_collection_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_6480, 0x010c_3b98);
    }

    #[test]
    fn ray_shape_filter_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_64b0, 0x010c_3bac);
    }

    #[test]
    fn broad_phase_collector_destructor_sets_vtable_and_frees_when_flagged() {
        assert_filter_destructor(0x00c6_64e0, 0x010c_3bb8);
    }

    #[test]
    fn ahkp_world_destructor_destroys_base_and_uses_router_for_delete() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let router = e.mem.alloc(0x14);
        let allocator = e.mem.alloc(4);
        e.map(0x0400_0000, 0x1000);
        e.mem.set_u16(this + 4, 0x360);
        e.mem.set_u32(router + 0x10, allocator);
        e.put_vtable(0x0400_0000, &[0, 0, 0x0400_0100]);
        e.mem.set_u32(allocator, 0x0400_0000);
        e.register_double(HAVOK_WORLD_DESTRUCTOR, move |_, a| {
            assert_eq!(a, [this]);
            Ret::default()
        });
        e.register_double(0x00c8_5750, move |_, _| ret(router));
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [allocator, this, 0x360]);
            Ret::default()
        });
        e.call(0x00c6_6510, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), HAVOK_WORLD_VTABLE);
    }

    #[test]
    fn bhk_memory_allocator_destructor_sets_vtable_and_calls_base_cleanup() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.register_double(MEMORY_ALLOCATOR_DESTRUCTOR, move |_, a| {
            assert_eq!(a[0], this);
            Ret::default()
        });
        e.call(0x00c6_6540, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), ALLOCATOR_VTABLE);
    }

    #[test]
    fn hk_memory_allocator_destructor_calls_base_without_changing_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234);
        e.register_double(MEMORY_ALLOCATOR_DESTRUCTOR, move |_, a| {
            assert_eq!(a[0], this);
            Ret::default()
        });
        e.call(0x00c6_6560, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
    }

    #[test]
    fn startup_callback_uses_vtable_slot_eight_and_restores_tls() {
        let mut e = engine();
        let callback = e.mem.alloc(4);
        let table = 0x0400_0000;
        e.put_vtable(table, &[0, 0, 0x0400_0100]);
        e.mem.set_u32(callback, table);
        e.set_global(0x0126_8450, callback);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 7);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [callback, 1]);
            Ret::default()
        });
        e.call(0x00c6_6570, &[]);
        assert_eq!(e.mem.u32(tls + 0x2b4), 7);
    }

    #[test]
    fn startup_callback_destructor_installs_vtable_and_obeys_free_flag() {
        assert_filter_destructor(0x00c6_6600, 0x010a_47b8);
    }

    fn assert_adjustor(address: u32, adjustment: u32) {
        let mut e = engine();
        let object = 0x0500_1000;
        e.register_double(0x00df_f470, move |_, a| {
            assert_eq!(a[0], object - adjustment);
            assert_eq!(a[1], 1);
            ret(a[0])
        });
        assert_eq!(
            e.call(address, &args![object, 1u32]).u32(),
            object - adjustment
        );
    }

    #[test]
    fn adjustor_thunk_subtracts_eight() {
        assert_adjustor(0x00c6_6630, 8);
    }

    #[test]
    fn adjustor_thunk_subtracts_twelve() {
        assert_adjustor(0x00c6_6640, 12);
    }

    #[test]
    fn adjustor_thunk_subtracts_sixteen() {
        assert_adjustor(0x00c6_6650, 16);
    }

    #[test]
    fn adjustor_thunk_subtracts_twenty() {
        assert_adjustor(0x00c6_6660, 20);
    }

    #[test]
    fn teardown_runs_callbacks_and_clears_owned_globals_in_order() {
        let mut e = engine();
        let startup = e.mem.alloc(4);
        let callback = e.mem.alloc(4);
        e.put_vtable(0x0400_0000, &[0x0400_0100, 0, 0x0400_0108]);
        e.mem.set_u32(startup, 0x0400_0000);
        e.mem.set_u32(callback, 0x0400_0000);
        e.set_global(0x0126_8450, startup);
        e.set_global(0x0126_77d4, callback);
        e.set_global(0x0126_77a8, 0u32);
        e.set_global(0x0126_77d0, 0u32);
        for address in [
            0x00c6_d340,
            0x00c6_d3c0,
            0x00cb_bcb0,
            0x00cb_bfb0,
            0x0400_0100,
            0x0400_0108,
        ] {
            e.register_double(address, move |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c6_6670, &[]);
        let addresses: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .collect();
        let expected = [
            0x00c6_6670,
            0x0400_0108,
            0x00c6_d340,
            0x00c6_d3c0,
            0x00cb_bcb0,
            0x0400_0108,
            0x0400_0100,
            0x00cb_bfb0,
        ];
        assert_eq!(addresses, expected);
        assert_eq!(e.global::<u32>(0x0126_77d4), 0);
    }

    /// An engine at the point the game steps Havok: the main thread at 24-bit precision (the
    /// renderer's device made), the largest delta 166.66667 and the skip threshold 1/120.
    fn stepping() -> Engine {
        let mut e = engine();
        engine_core::units::platform::direct3d_device_created(&mut e, 0);
        e.set_global(STEP_MAX_DELTA, 0x4326_aaabu32);
        e.set_global(STEP_SKIP_THRESHOLD, 0x3c08_8889u32);
        e.set_global(STEP_FIXED, 0x3c88_8889u32); // 1/60
        e.set_global(STEP_FIXED_SCALE, 1.0f32);
        e.set_global(STEP_HALF, 0.5f64);
        e.set_global(STEP_TWO_TO_32, 0x4f80_0000u32);
        e.set_global(STEP_CARRY, 0u32);
        e
    }

    fn step(e: &mut Engine, delta: u32, single: u32, forced: u32) {
        e.call(0x00c6_6760, &args![delta, single, forced]);
    }

    /// Mode 0, 0.05 s with 1/60 steps, worked out from the instructions at 24-bit precision:
    /// 0.05 / (1/60) is 3 - 1.1e-7, which rounds to 3.0 at 24 bits (53-bit precision would keep
    /// 2.9999999 and give two steps); 3 * (1/60) is 26843547 * 2^-29, rounded (a tie, to even)
    /// to 13421774 * 2^-28, one unit above 0.05 (13421773 * 2^-28), so the carry is -2^-28
    /// (`b1800000`) and the step delta 0.05 + 2^-28 (`3d4cccce`); then the carry is scaled
    /// through `0040ebd0` with the setting's value.
    #[test]
    fn fixed_steps_carry_the_rest_and_scale_it() {
        let mut e = stepping();
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 0x4000_0000);
        e.register_double(SETTING_VALUE, move |_, a| {
            assert_eq!(a, [STEP_CARRY_SETTING]);
            ret(setting)
        });
        e.register_double(STEP_CARRY_SCALE, |_, a| {
            assert_eq!(a, [0xb180_0000, 0x4000_0000]);
            Ret {
                st0: 0.25,
                ..Ret::default()
            }
        });
        step(&mut e, 0x3d4c_cccd, 0, 0);
        assert_eq!(e.global::<u32>(STEP_FRAME_DELTA), 0x3d4c_cccd);
        assert_eq!(e.global::<u32>(STEP_COUNT), 3);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0x3c88_8889);
        assert_eq!(e.global::<u32>(STEP_DELTA), 0x3d4c_ccce);
        assert_eq!(e.global::<f32>(STEP_CARRY), 0.25);
        // Single-stepping caps the count at 1: the carry 0.05 - 1/60 (25 bits exactly, so rounded
        // to a float as the 24-bit subtraction rounds it) and the step delta 0.05 less that.
        let mut e = stepping();
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 0x4000_0000);
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        e.register_double(STEP_CARRY_SCALE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        step(&mut e, 0x3d4c_cccd, 1, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        let carried = 0.05f32 - f32::from_bits(0x3c88_8889);
        assert_eq!(e.global::<f32>(STEP_CARRY), carried);
        assert_eq!(
            e.global::<f32>(STEP_DELTA),
            0.05f32 - f32::from_bits(carried.to_bits())
        );
        // Before the device (53-bit precision) the quotient stays below 3: two steps.
        let mut e = stepping();
        engine_core::units::crt::set_x87_control_word(&mut e, 0x027f);
        let setting = e.mem.alloc(4);
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        e.register_double(STEP_CARRY_SCALE, |_, _| Ret::default());
        step(&mut e, 0x3d4c_cccd, 0, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 2);
    }

    /// Below one step: carried whole unless half a step is reached or the step is forced.
    #[test]
    fn a_short_delta_is_carried_unless_half_a_step_or_forced() {
        let mut e = stepping();
        step(&mut e, 0x3c00_0000, 0, 0); // 1/128 < 1/120
        assert_eq!(e.global::<u32>(STEP_COUNT), 0);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0x3c00_0000);
        assert_eq!(e.global::<u32>(STEP_DELTA), 0);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0);
        // Carry 1/128 + 1/256 = 3/256 >= 1/120: one step of it, nothing carried.
        step(&mut e, 0x3b80_0000, 0, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        assert_eq!(e.global::<f32>(STEP_SIZE), 3.0 / 256.0);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0);
        // Forced: one step at once.
        let mut e = stepping();
        step(&mut e, 0x3c00_0000, 0, 1);
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0x3c00_0000);
    }

    /// Zero (and -0) end with no step; a NaN is unordered everywhere: it goes on, its count
    /// is the 64-bit integer indefinite's low word 0, half a step does not exceed it, so one
    /// step of the NaN; above the largest delta the largest is used.
    #[test]
    fn zero_nan_and_the_largest_delta() {
        let mut e = stepping();
        e.set_global(STEP_COUNT, 7u32);
        step(&mut e, 0x8000_0000, 0, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 0);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0);
        assert_eq!(e.global::<u32>(STEP_DELTA), 0);
        step(&mut e, 0x7fc1_2345, 0, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0x7fc1_2345);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0);
        // A signalling NaN delta is stored quieted.
        let mut e = stepping();
        e.set_global(STEP_MODE, 2u32);
        step(&mut e, 0x7f80_0001, 0, 0);
        assert_eq!(e.global::<u32>(STEP_FRAME_DELTA), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0x7fc0_0001);
        // 200 s: the largest delta, 166.66667 / (1/60) steps capped at 3, each 1/60.
        let mut e = stepping();
        e.register_double(SETTING_VALUE, |e, _| ret(e.mem.alloc(4)));
        e.register_double(STEP_CARRY_SCALE, |_, _| Ret::default());
        step(&mut e, 200.0f32.to_bits(), 0, 0);
        assert_eq!(e.global::<u32>(STEP_COUNT), 3);
        let size = f32::from_bits(0x3c88_8889);
        let carried = f32::from_bits(0x4326_aaab) - size * 3.0;
        assert_eq!(
            e.global::<f32>(STEP_DELTA),
            f32::from_bits(0x4326_aaab) - carried
        );
    }

    /// Mode 10 at 24-bit precision: each operation rounds to a float, (delta - smoothed) *
    /// rate + smoothed done as three float operations, which here differs from rounding the
    /// 53-bit result once.
    #[test]
    fn smoothing_rounds_each_operation_to_24_bits() {
        let (delta, smoothed, rate) = (0.09f32, 0.02f32, 0.15f32);
        let three_floats = (delta - smoothed) * rate + smoothed;
        let once = ((f64::from(delta) - f64::from(smoothed)) * f64::from(rate)
            + f64::from(smoothed)) as f32;
        let mut e = stepping();
        e.set_global(STEP_MODE, 10u32);
        e.set_global(STEP_SMOOTHED, smoothed);
        e.set_global(STEP_SMOOTH_RATE, rate);
        step(&mut e, delta.to_bits(), 0, 0);
        assert_eq!(e.global::<f32>(STEP_SMOOTHED), three_floats);
        assert_eq!(e.global::<f32>(STEP_SIZE), three_floats);
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0);
        // At 53-bit precision (before the device) the same instructions give the other value.
        let mut e = stepping();
        engine_core::units::crt::set_x87_control_word(&mut e, 0x027f);
        e.set_global(STEP_MODE, 10u32);
        e.set_global(STEP_SMOOTHED, smoothed);
        e.set_global(STEP_SMOOTH_RATE, rate);
        step(&mut e, delta.to_bits(), 0, 0);
        assert_eq!(e.global::<f32>(STEP_SMOOTHED), once);
        assert_ne!(three_floats, once, "pick values where the precision shows");
    }

    /// The other modes: skip below the threshold, else steps of delta / count.
    #[test]
    fn other_modes_split_the_delta_evenly() {
        let mut e = stepping();
        e.set_global(STEP_MODE, 5u32);
        step(&mut e, 0x3c00_0000, 0, 0); // 1/128 < 1/120: skipped and carried
        assert_eq!(e.global::<u32>(STEP_COUNT), 0);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0x3c00_0000);
        let mut e = stepping();
        e.set_global(STEP_MODE, 1u32);
        step(&mut e, 0x3d4c_cccd, 0, 0); // 0.05: 3.0 at 24 bits, three steps of 0.05 / 3
        assert_eq!(e.global::<u32>(STEP_COUNT), 3);
        assert_eq!(e.global::<f32>(STEP_SIZE), 0.05f32 / 3.0);
        assert_eq!(e.global::<u32>(STEP_CARRY), 0);
        let mut e = stepping();
        e.set_global(STEP_MODE, 11u32);
        step(&mut e, 0x3c10_0000, 0, 0); // 0.0087890625 >= 1/120, below 1/60: one step
        assert_eq!(e.global::<u32>(STEP_COUNT), 1);
        assert_eq!(e.global::<u32>(STEP_SIZE), 0x3c10_0000);
    }

    #[test]
    fn step_accumulator_handles_negative_and_manual_modes() {
        let mut e = engine();
        e.set_global(0x010c_3c28, 166.66667f32);
        e.set_global(0x010c_3c24, 0.008333334f32);
        e.set_global(0x0126_77b8, 2u32);
        e.set_global(0x0126_77b0, 0.0f32);
        e.call(0x00c6_6760, &args![-1.0f32, 0u32, 0u32]);
        assert_eq!(e.global::<u32>(0x0126_77b4), 0);
        assert_eq!(e.global::<f32>(0x0126_77ac), 0.0);
        e.call(0x00c6_6760, &args![0.25f32, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(0x0126_77b4), 1);
        assert_eq!(e.global::<f32>(0x0126_77ac), 0.25);
    }

    #[test]
    fn scale_solver_info_uses_mode_one_and_writes_solver_fields() {
        let mut e = engine();
        let bhk: Ptr = Ptr::new(e.mem.alloc(4));
        let world = e.mem.alloc(0x304);
        let table = 0x0400_0000;
        let mut bhk_slots = vec![0; 0x26];
        bhk_slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(table, &bhk_slots);
        e.mem.set_u32(bhk.addr(), table);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.set_global(0x0126_77b8, 0u32);
        e.set_global(0x0126_77ac, 0.5f32);
        e.set_global(0x0126_7b38, 1.0f32);
        e.set_global(0x011a_fe60, 0.5f32);
        e.set_global(0x0101_db80, 4.0f64);
        e.set_global(0x0101_1590, 2.0f64);
        e.set_global(0x0101_62c0, 2.0f32);
        // The double 0.6 the blend multiplies by (`010290b8`): the float 0.6 widened.
        e.set_global(0x0102_90b8, f64::from(0.6f32));
        // `ftol` is handed the clamped `4 * step / frequency`: 2.0 here.
        e.register_double(0x00ec_62c0, move |_, a| {
            assert_eq!(f64::from_bits(u64::from(a[1]) << 32 | u64::from(a[0])), 2.0);
            ret(2)
        });
        e.mem.set_f32(world + 0x1e8, 0.75);
        e.call(0x00c6_6a00, &args![bhk]);
        assert_eq!(e.mem.i32(world + 0x2f4), 2);
        assert_eq!(e.mem.f32(world + 0x300), 0.5);
        assert_eq!(e.global::<f32>(0x0126_7b50), 0.5);
        // The exe (00c66a97..00c66aaf, `DC CA` at 00c66aa7) blends `(1 - g) * 0.6 + (t * g) * 0.6`
        // with `g` = 0.5 and `t` = the step over the frequency = 0.5, left on the x87 stack by
        // the first half of the function: 0.75 * 0.6f, which is exactly the float
        // 0.4500000179. (Reading `t` as the clamped step count's float 2.0, as the decompiler's
        // `ftol` result suggests, gave 0.9.)
        let expected = (0.75 * f64::from(0.6f32)) as f32;
        assert_eq!(expected, 0.450_000_02f32);
        assert_eq!(e.mem.f32(world + 0x1e4), expected);
        // Damping over tau and tau over damping (`FDIV ST0,ST1`, `FDIV float [damping]`).
        assert!((e.mem.f32(world + 0x230) - 0.75 / expected).abs() < 1e-6);
        assert!((e.mem.f32(world + 0x234) - expected / 0.75).abs() < 1e-6);
    }

    #[test]
    fn origin_scale_helper_passes_source_then_destination() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let input = e.mem.alloc(16);
        e.register_double(0x0045_8620, move |_, a| {
            assert_eq!(a, [input, this.addr() + 0x60]);
            Ret::default()
        });
        e.call(0x00c6_6be0, &args![this, input]);
    }

    #[test]
    fn broad_phase_shift_updates_origin_from_the_shifted_delta() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let requested = e.mem.alloc(16);
        let world = e.mem.alloc(4);
        let world_table = 0x0400_0000;
        let bhk_table = 0x0400_1000;
        e.put_vtable(world_table, &[0; 0x26]);
        let mut bhk_slots = vec![0; 0x26];
        bhk_slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(bhk_table, &bhk_slots);
        e.mem.set_u32(this.addr(), bhk_table);
        for i in 0..4 {
            e.mem.set_f32(this.addr() + 0x60 + i * 4, i as f32);
            e.mem.set_f32(requested + i * 4, (i + 10) as f32);
        }
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00c9_6a20, move |e, a| {
            assert_eq!(a[0], world);
            assert_eq!(
                (0..4).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                [10.0, 10.0, 10.0, 10.0]
            );
            for i in 0..4 {
                e.mem.set_f32(a[2] + i * 4, 2.0);
            }
            assert_eq!(a[3], 0);
            Ret::default()
        });
        e.call(0x00c6_6c00, &args![this, requested]);
        for i in 0..4 {
            assert_eq!(e.mem.f32(this.addr() + 0x60 + i * 4), i as f32 + 2.0);
        }
    }

    #[test]
    fn bhk_world_init_builds_and_attaches_havok_world_in_call_order() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x360);
        let allocator = e.mem.alloc(4);
        let router = e.mem.alloc(0x14);
        let bhk_table = 0x0400_0000;
        let allocator_table = 0x0400_1000;
        let mut bhk_slots = vec![0; 0x30];
        bhk_slots[0x8c / 4] = 0x0400_008c;
        bhk_slots[0x94 / 4] = 0x0400_0094;
        bhk_slots[0xbc / 4] = 0x0400_00bc;
        e.put_vtable(bhk_table, &bhk_slots);
        e.put_vtable(allocator_table, &[0, 0x0400_0100]);
        e.mem.set_u32(this.addr(), bhk_table);
        e.mem.set_u32(allocator, allocator_table);
        e.mem.set_u32(router + 0x10, allocator);
        e.set(this, BhkWorld::pInfo, Ptr::new(0x0700_0000));
        e.set_global(0x0126_77a8, 0x0710_0000u32);
        e.register_double(0x00c8_5750, move |_, _| ret(router));
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [allocator, 0x360]);
            ret(world)
        });
        e.register_double(0x00c9_5a80, move |_, a| {
            assert_eq!(a, [world, 0x0700_0000, 0x111d4]);
            Ret::default()
        });
        e.register_double(0x00c9_77c0, move |_, a| {
            assert_eq!(a, [world, 0x0710_0000, 1, 0, 1]);
            Ret::default()
        });
        e.register_double(0x0400_008c, move |_, a| {
            assert_eq!(a, [this.addr(), world]);
            Ret::default()
        });
        e.register_double(0x00c9_05b0, move |_, a| {
            assert_eq!(a, [world]);
            Ret::default()
        });
        e.register_double(0x0400_00bc, move |_, a| {
            assert_eq!(a, [this.addr(), 0x0700_00f0]);
            Ret::default()
        });
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00cb_fd20, move |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_6c90, &args![this, 0x0700_00f0u32]);
        let calls = e.call_log.as_ref().unwrap();
        let addresses: Vec<_> = calls.iter().map(|(address, _)| *address).collect();
        let constructor = addresses.iter().position(|a| *a == 0x00c9_5a80).unwrap();
        let filter = addresses.iter().position(|a| *a == 0x00c9_77c0).unwrap();
        let attach = addresses.iter().position(|a| *a == 0x0400_008c).unwrap();
        let setup = addresses.iter().position(|a| *a == 0x00c9_05b0).unwrap();
        let info = addresses.iter().position(|a| *a == 0x0400_00bc).unwrap();
        assert!(constructor < filter && filter < attach && attach < setup && setup < info);
        assert_eq!(e.mem.u32(world), HAVOK_WORLD_VTABLE);
        assert_eq!(e.mem.u32(world + 0x350), 0);
        assert_eq!(e.mem.u16(world + 4), 0x360);
    }

    #[test]
    fn link_sets_world_pointer_only_when_link_flag_is_set() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x354);
        let table = 0x0400_0000;
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(table, &slots);
        e.mem.set_u32(this.addr(), table);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        e.register_double(0x00c8_7c90, move |_, _| Ret::default());
        e.call(0x00c6_6de0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(world + 0x350), this.addr());
        e.call(0x00c6_6de0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(world + 0x350), 0);
    }

    #[test]
    fn entity_listener_update_skips_zero_and_calls_each_non_null_entry() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let listeners = e.mem.alloc(12);
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 8, 1);
        e.set(this, BhkWorld::entityListeners, Ptr::new(listeners));
        e.set(this, BhkWorld::entityListenerCount, 3);
        e.mem.set_u32(listeners, a);
        e.mem.set_u32(listeners + 4, 0);
        e.mem.set_u32(listeners + 8, b);
        let mut listener_slots = vec![0; 7];
        listener_slots[0x18 / 4] = 0x0400_0100;
        e.put_vtable(0x0400_0000, &listener_slots);
        e.mem.set_u32(a, 0x0400_0000);
        e.mem.set_u32(b, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[1], 2.5f32.to_bits());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_6e20, &args![this, 2.5f32]);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x0400_0100)
                .count(),
            2
        );
    }

    #[test]
    fn kill_cinfo_clears_pointer_and_calls_deleting_destructor_when_requested() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let base = e.mem.alloc(4);
        e.set(this, BhkWorld::pInfo, Ptr::new(base + 0xf0));
        e.put_vtable(0x0400_0000, &[0x0400_0100]);
        e.mem.set_u32(base, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a, [base, 1]);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 3);
        e.call(0x00c6_6e70, &args![this, 1u32]);
        assert!(e.get(this, BhkWorld::pInfo).is_null());
        assert_eq!(e.mem.u32(tls + 0x2b4), 3);
    }

    #[test]
    fn collision_filter_reuses_existing_group_and_assigns_encoded_group() {
        let mut e = engine();
        let collidable = e.mem.alloc(0x30);
        let filter = e.mem.alloc(0x14);
        let world = e.mem.alloc(0x30);
        e.mem.set_u32(collidable + 8, world);
        e.mem.set_u32(world + 0x2c, 0x1234_0000);
        e.mem.set_u32(filter + 0x10, 77);
        e.register_double(0x00c8_5ce0, move |_, a| {
            assert!(a == [collidable, 77] || a == [collidable, 0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c6_6f10, &args![collidable, filter]).bool());
        assert_eq!(e.mem.u32(filter + 0x10), 77);
        e.mem.set_u32(filter + 0x10, 0);
        // An encoded group is stored without the call: AL (BL) stays 0.
        assert!(!e.call(0x00c6_6f10, &args![collidable, filter]).bool());
        assert_eq!(e.mem.u32(filter + 0x10), 0x1234);
        let groups: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00c8_5ce0)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(groups, [vec![collidable, 77]]);
    }

    #[test]
    fn has_active_items_reads_the_havok_world_count() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let world = e.mem.alloc(0x30);
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0094;
        e.put_vtable(0x0400_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0400_0000);
        e.mem.set_u32(world + 0x2c, 3);
        e.register_double(0x0400_0094, move |_, _| ret(world));
        assert!(e.call(0x00c6_6f90, &args![this]).bool());
        e.mem.set_u32(world + 0x2c, 0);
        assert!(!e.call(0x00c6_6f90, &args![this]).bool());
    }

    #[test]
    fn pick_data_resolves_ni_object_only_for_a_root_collidable() {
        let mut e = engine();
        let this: Ptr<BhkPickData> = e.new_object();
        let collidable = e.mem.alloc(4);
        e.set(this, BhkPickData::rootCollidable, Ptr::new(collidable));
        e.register_double(0x00c7_fa90, move |_, a| {
            assert_eq!(a[0], collidable);
            ret(0x0700_1234)
        });
        assert_eq!(e.call(0x00c6_6fb0, &args![this]).u32(), 0x0700_1234);
        e.set(this, BhkPickData::rootCollidable, Ptr::NULL);
        assert_eq!(e.call(0x00c6_6fb0, &args![this]).u32(), 0);
    }

    #[test]
    fn pick_data_rigid_body_uses_signed_embedded_offset_and_type_byte() {
        let mut e = engine();
        let this: Ptr<BhkPickData> = e.new_object();
        let collidable = e.mem.alloc(0x20);
        e.set(this, BhkPickData::rootCollidable, Ptr::new(collidable));
        e.mem.set_u8(collidable + 0x18, 1);
        e.mem.set_i8(collidable + 0x10, 4);
        assert_eq!(e.call(0x00c6_6fd0, &args![this]).u32(), collidable + 4);
        e.mem.set_u8(collidable + 0x18, 0);
        assert_eq!(e.call(0x00c6_6fd0, &args![this]).u32(), 0);
    }

    #[test]
    fn collision_agent_empty_branch_clears_filter_active_bit_and_counts_contact() {
        let mut e = engine();
        let agent = e.mem.alloc(0x20);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(agent + 0x10, 0);
        e.mem.set_u32(filter + 0xc, 5);
        e.mem.set_u32(filter + 0x14, 8);
        e.call(0x00c6_6ff0, &args![agent, filter]);
        assert_eq!(e.mem.u32(filter + 0xc), 4);
        assert_eq!(e.mem.u32(filter + 0x14), 9);
    }

    #[test]
    fn collision_agent_applies_world_flags_then_forwards_contact_and_activation() {
        let mut e = engine();
        let agent = e.mem.alloc(0x20);
        let filter = e.mem.alloc(0x20);
        let collidable = e.mem.alloc(0x20);
        let property = e.mem.alloc(0x10);
        let filter_object = e.mem.alloc(0x20);
        let data_owner = e.mem.alloc(4);
        let filter_world = 0x0700_1000;
        let collidable_world = 0x0700_2000;
        let agent_table = 0x0400_0000;
        let filter_table = 0x0400_1000;
        let collidable_table = 0x0400_2000;
        let data_table = 0x0400_3000;
        let mut slots = vec![0; 0x31];
        slots[0x3c / 4] = 0x0400_0100;
        slots[0xc0 / 4] = 0x0400_0114;
        e.put_vtable(agent_table, &slots);
        let mut slots = vec![0; 0x26];
        slots[0x94 / 4] = 0x0400_0104;
        e.put_vtable(filter_table, &slots);
        let mut slots = vec![0; 0x28];
        slots[0x94 / 4] = 0x0400_0108;
        slots[0x9c / 4] = 0x0400_010c;
        e.put_vtable(collidable_table, &slots);
        let mut slots = vec![0; 0x2f];
        slots[0xb8 / 4] = 0x0400_0110;
        e.put_vtable(data_table, &slots);
        e.mem.set_u32(agent, agent_table);
        e.mem.set_u32(agent + 8, data_owner);
        e.mem.set_u32(agent + 0x10, collidable);
        e.mem.set_u32(filter, filter_object);
        e.mem.set_u32(filter + 0xc, 1);
        e.mem.set_u32(filter + 0x14, 0);
        e.mem.set_u32(filter + 0x18, 1);
        e.mem.set_u32(collidable, collidable_table);
        e.mem.set_u32(filter_object, filter_table);
        e.mem.set_u32(data_owner, data_table);
        e.mem.set_u8(filter_object + 0x16, 1);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], agent);
            ret(property)
        });
        e.register_double(0x0400_0104, move |_, a| {
            assert_eq!(a[0], filter_object);
            ret(filter_world)
        });
        e.register_double(0x0400_0108, move |_, a| {
            assert_eq!(a[0], collidable);
            ret(collidable_world)
        });
        e.register_double(0x0400_010c, move |_, a| {
            assert_eq!(a, [collidable, filter_object]);
            Ret::default()
        });
        e.register_double(0x0400_0110, move |e, a| {
            assert_eq!(a[0], data_owner);
            assert_eq!(e.mem.f32(a[1]), 0.0);
            assert_eq!(e.mem.bytes(a[1] + 4, 5), [0; 5]);
            Ret::default()
        });
        e.register_double(0x0400_0114, move |_, a| {
            assert_eq!(a, [agent, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_6ff0, &args![agent, filter]);
        assert_eq!(e.mem.u16(property + 0xc), 0x100);
        assert_eq!(e.mem.u16(agent + 0xc), 0x4c);
        assert_eq!(e.mem.u32(filter + 0xc), 0);
        assert_eq!(e.mem.u32(filter + 0x14), 1);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(address, _)| address)
            .collect();
        assert_eq!(
            calls,
            [
                0x00c6_6ff0,
                0x0400_0100,
                0x0400_0104,
                0x0400_0108,
                0x0400_010c,
                0x0400_0110,
                0x0400_0114
            ]
        );
    }

    #[test]
    fn shape_transform_early_return_when_no_shape_owner_exists() {
        let mut e = engine();
        let agent = e.mem.alloc(0x14);
        let transform = e.mem.alloc(0x10);
        let mut slots = vec![0; 0x10];
        slots[0x3c / 4] = 0x0400_0100;
        e.put_vtable(0x0400_0000, &slots);
        e.mem.set_u32(agent, 0x0400_0000);
        e.register_double(0x0400_0100, move |_, _| ret(0));
        e.call(0x00c6_7320, &args![agent, transform]);
        assert_eq!(e.mem.u32(agent + 0x10), 0);
    }

    #[test]
    fn shape_transform_scales_offset_and_calls_shape_get_then_set_in_order() {
        let mut e = engine();
        let agent = e.mem.alloc(0x14);
        let owner = e.mem.alloc(4);
        let transform = e.mem.alloc(0x10);
        let material = e.mem.alloc(0x20);
        let vector = e.mem.alloc(16);
        let shape = e.mem.alloc(4);
        let agent_table = 0x0400_0000;
        let owner_table = 0x0400_1000;
        let shape_table = 0x0400_2000;
        let mut slots = vec![0; 0x10];
        slots[0x3c / 4] = 0x0400_0100;
        e.put_vtable(agent_table, &slots);
        let mut slots = vec![0; 0x11];
        slots[0x40 / 4] = 0x0400_0104;
        e.put_vtable(owner_table, &slots);
        let mut slots = vec![0; 0x38];
        slots[0xd4 / 4] = 0x0400_0108;
        slots[0xdc / 4] = 0x0400_010c;
        e.put_vtable(shape_table, &slots);
        e.mem.set_u32(agent, agent_table);
        e.mem.set_u32(agent + 0x10, owner);
        e.mem.set_u32(owner, owner_table);
        e.mem.set_u32(shape, shape_table);
        e.mem.set_u32(transform + 0xc, vector);
        e.mem.set_f32(material + 0x18, 0.5);
        e.mem.set_f32(material + 0x1c, 2.0);
        e.mem.set_f32(vector, 1.0);
        e.mem.set_f32(vector + 4, 2.0);
        e.mem.set_f32(vector + 8, 3.0);
        e.mem.set_f32(vector + 12, 4.0);
        e.register_double(0x0400_0100, move |_, a| {
            assert_eq!(a[0], agent);
            ret(material)
        });
        e.register_double(0x0400_0104, move |_, a| {
            assert_eq!(a[0], owner);
            ret(shape)
        });
        e.register_double(0x0400_0108, move |e, a| {
            assert_eq!(a[0], shape);
            for (i, value) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + i as u32 * 4, value);
            }
            Ret::default()
        });
        e.register_double(0x0400_010c, move |e, a| {
            assert_eq!(a[0], shape);
            assert_eq!(
                (0..4).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                [12.0, 24.0, 36.0, 48.0]
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_7320, &args![agent, transform]);
        let calls: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(address, _)| address)
            .collect();
        assert_eq!(
            calls,
            [
                0x00c6_7320,
                0x0400_0100,
                0x0400_0104,
                0x0400_0108,
                0x0400_010c
            ]
        );
    }

    // ----- batch operations and the functions around them -----

    const GETTER: u32 = 0x0400_0094;

    /// A `bhkWorld` object whose slot `+0x94` returns `hk_world` (kept in the
    /// spare word at `+0x98` of the test object).
    fn world_with(e: &mut Engine, hk_world: u32) -> Ptr<BhkWorld> {
        let this: Ptr<BhkWorld> = e.new_object();
        let mut slots = vec![0; 0x30];
        slots[0x94 / 4] = GETTER;
        e.put_vtable(0x0400_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0400_0000);
        e.mem.set_u32(this.addr() + 0x98, hk_world);
        e.register_double(GETTER, |e, a| ret(e.mem.u32(a[0] + 0x98)));
        this
    }

    /// `memset` stand-in that really clears the memory.
    fn memset_double(e: &mut Engine) {
        e.register_double(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![0; a[2] as usize]);
            Ret::default()
        });
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register_double(address, |_, _| Ret::default());
        }
    }

    /// Marks the allocation-scope word so a test can see it restored.
    fn scope_marker(e: &mut Engine) -> u32 {
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 7);
        tls
    }

    fn assert_scope_restored(e: &mut Engine, tls: u32) {
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 7);
    }

    fn log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    fn addresses(calls: &[(u32, Vec<u32>)]) -> Vec<u32> {
        calls.iter().map(|(address, _)| *address).collect()
    }

    fn calls_to(calls: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn pointer_array(e: &mut Engine, items: &[u32]) -> u32 {
        let array = e.mem.alloc(items.len() as u32 * 4 + 16);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(array + i as u32 * 4, *item);
        }
        array
    }

    /// Interlocked increment and decrement stand-ins that really count.
    fn interlocked_doubles(e: &mut Engine) {
        e.register_double(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.register_double(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
    }

    /// A referenced object (count 1 at `+4`) whose `DeleteThis` slot `+4` is
    /// the double at `delete_address`.
    fn counted_object(e: &mut Engine, table: u32, delete_address: u32) -> u32 {
        e.put_vtable(table, &[0, delete_address]);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 4, 1);
        object
    }

    #[test]
    fn snapshot_save_returns_the_result_byte_and_closes_the_stream() {
        let mut e = engine();
        let this = world_with(&mut e, 0x0500_0000);
        e.register_double(0x00cc_04d0, |e, a| {
            assert_eq!(a[1] >> 8, 0x12);
            e.mem.set_u32(a[0] + 8, 0x77);
            Ret::default()
        });
        e.register_double(0x00cc_0d60, |e, a| {
            assert_eq!(a[1..], [0x0500_0000, 0x77, 0, 0, 0]);
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        quiet(&mut e, &[0x00cc_00e0]);
        log(&mut e);
        assert!(e.call(0x00c6_7430, &args![this, 0x1201u32]).bool());
        assert_eq!(
            addresses(&calls(&mut e)),
            [0x00c6_7430, 0x00cc_04d0, GETTER, 0x00cc_0d60, 0x00cc_00e0]
        );
        // A save that leaves the byte alone returns the low byte of the argument.
        quiet(&mut e, &[0x00cc_0d60]);
        assert!(e.call(0x00c6_7430, &args![this, 0x1201u32]).bool());
        assert!(!e.call(0x00c6_7430, &args![this, 0x1200u32]).bool());
    }

    fn entity_batch_fixture(e: &mut Engine) -> (u32, u32, u32, u32, u32) {
        for lane in 0..4 {
            e.mem.set_u32(GLOBAL_ABSOLUTE_MASK + lane * 4, 0x7fff_ffff);
        }
        e.set_global(GLOBAL_TOLERANCE, 0.001f32);
        e.set_global(GLOBAL_ISLAND_SPLIT_FLAG, 1u8);
        let rigid = e.mem.alloc(0x200);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 8, rigid);
        e.mem.set_u32(owner + 0x10, 0x1f);
        let island = e.mem.alloc(0x60);
        e.mem.set_u16(island + 0x20, 5);
        e.mem.set_u32(island + 0x48, 11);
        e.mem.set_u32(island + 0x4c, 22);
        // Entity A: byte +0x28 is 1, displacement +0x20 = 0x20 reaches the owner holder.
        let entity_a = e.mem.alloc(0x100);
        e.mem.set_u8(entity_a + 0x28, 1);
        e.mem.set_u8(entity_a + 0x20, 0x20);
        e.mem.set_u32(entity_a + 0x30 + 0xc, owner);
        e.mem.set_u32(entity_a + 0xcc, island);
        let entity_b = e.mem.alloc(0x100);
        e.mem.set_u32(entity_b + 0xcc, island);
        (rigid, owner, island, entity_a, entity_b)
    }

    #[test]
    fn add_entity_batch_updates_owners_removes_ownerless_and_splits_once() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let (rigid, owner, island, entity_a, entity_b) = entity_batch_fixture(&mut e);
        e.mem.set_f32(rigid + 0x1b0, 1.0);
        let entities = pointer_array(&mut e, &[entity_a, entity_b]);
        quiet(
            &mut e,
            &[
                0x00c9_4bd0,
                0x00c9_1620,
                0x00c9_2a80,
                0x00cb_6e70,
                0x00c9_c1d0,
                HAVOK_RELEASE,
            ],
        );
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_74d0, &args![this, entities, 2u32]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_4bd0), [[world, entities, 2, 0]]);
        assert_eq!(e.mem.u32(owner + 0x10), 0x0f);
        assert_eq!(calls_to(&seen, 0x00c9_1620).len(), 1);
        assert_eq!(calls_to(&seen, 0x00c9_1620)[0][0], world);
        assert_eq!(calls_to(&seen, 0x00c9_1620)[0][2], entity_b);
        assert_eq!(calls_to(&seen, 0x00c9_2a80), [[world, 11, 22]]);
        assert_eq!(calls_to(&seen, 0x00cb_6e70), [[world, island]]);
        assert_eq!(calls_to(&seen, 0x00c9_c1d0), [[rigid]]);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[entity_a], [entity_b]]);
        assert_eq!(calls_to(&seen, MEMSET), [[entities, 0, 8]]);
        assert_eq!(e.mem.u32(entities), 0);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn add_entity_batch_leaves_quiet_owners_asleep_and_needs_a_world() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let this = world_with(&mut e, 0x0500_0000);
        let (rigid, _, _, entity_a, _) = entity_batch_fixture(&mut e);
        e.mem.set_f32(rigid + 0x1c0, 0.5);
        let entities = pointer_array(&mut e, &[entity_a]);
        quiet(
            &mut e,
            &[
                0x00c9_4bd0,
                0x00c9_2a80,
                0x00cb_6e70,
                0x00c9_c1d0,
                HAVOK_RELEASE,
            ],
        );
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_74d0, &args![this, entities, 1u32]);
        // The second vector is not near zero, so the owner is activated.
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_c1d0), [[rigid]]);
        e.mem.set_f32(rigid + 0x1c0, 0.0);
        let entities = pointer_array(&mut e, &[entity_a]);
        log(&mut e);
        e.call(0x00c6_74d0, &args![this, entities, 1u32]);
        assert!(calls_to(&calls(&mut e), 0x00c9_c1d0).is_empty());
        // No hkpWorld: nothing is called.
        let bare = world_with(&mut e, 0);
        log(&mut e);
        e.call(0x00c6_74d0, &args![bare, entities, 1u32]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_74d0, GETTER]);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn add_phantom_batch_clears_pending_bits_and_resets_properties() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let body = e.mem.alloc(0x20);
        e.mem.set_u8(body + 0x14, 0xff);
        let with_body = e.mem.alloc(0x20);
        e.mem.set_u32(with_body + 0xc, body);
        let without_body = e.mem.alloc(0x20);
        let array = pointer_array(&mut e, &[with_body, without_body]);
        e.set(this, BhkWorld::pPhantomBatch, Ptr::new(array));
        e.set(this, BhkWorld::iPhantomBatchCount, 2);
        let property = e.mem.alloc(0x20);
        e.put_vtable(0x0400_1000, &[0; 0x25]);
        e.mem.set_u32(property, 0x0400_1000);
        e.mem.set_u32(0x0400_1000 + 0x90, 0x0400_1090);
        e.register_double(0x0400_1090, move |e, a| {
            assert_eq!(a[0], property);
            assert_eq!(e.mem.bytes(a[1], 12), [0; 12]);
            Ret::default()
        });
        e.register_double(0x00c8_5ef0, move |e, a| {
            assert_eq!(a[2], 0x0126_7b70);
            e.mem
                .set_u32(a[0], if a[1] == with_body { property } else { 0 });
            ret(a[0])
        });
        quiet(&mut e, &[0x00c9_54e0, 0x00c9_2c70, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_76e0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_54e0), [[world, array, 2]]);
        assert_eq!(e.mem.u8(body + 0x14), 0xef);
        assert_eq!(calls_to(&seen, 0x00c9_2c70), [[world, without_body]]);
        assert_eq!(calls_to(&seen, 0x0400_1090).len(), 1);
        assert_eq!(
            calls_to(&seen, HAVOK_RELEASE),
            [[with_body], [without_body]]
        );
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 0);
        assert_eq!(e.mem.u32(array), 0);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn add_phantom_batch_clamps_the_count_even_without_a_world() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let this = world_with(&mut e, 0);
        e.set(this, BhkWorld::iPhantomBatchCount, 250);
        e.call(0x00c6_76e0, &args![this]);
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 200);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn add_constraint_batch_waits_for_entities_and_checks_islands() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let placed = e.mem.alloc(0xd0);
        e.mem.set_u32(placed + 0xcc, 1);
        let unplaced = e.mem.alloc(0xd0);
        let ok = e.mem.alloc(0x20);
        e.mem.set_u32(ok + 0x14, placed);
        let already_added = e.mem.alloc(0x20);
        e.mem.set_u32(already_added + 8, 1);
        let blocked = e.mem.alloc(0x20);
        e.mem.set_u32(blocked + 0x18, unplaced);
        let array = pointer_array(&mut e, &[ok, already_added, blocked]);
        e.set(this, BhkWorld::pConstraintInstanceBatch, Ptr::new(array));
        e.set(this, BhkWorld::iConstraintInstanceBatchCount, 3);
        e.set(this, BhkWorld::iEntityBatchCount, 1);
        quiet(&mut e, &[0x00c9_7d30, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_7850, &args![this]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_7850]);
        e.set(this, BhkWorld::iEntityBatchCount, 0);
        log(&mut e);
        e.call(0x00c6_7850, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_7d30), [[world, ok]]);
        assert_eq!(
            calls_to(&seen, HAVOK_RELEASE),
            [[ok], [already_added], [blocked]]
        );
        assert_eq!(e.get(this, BhkWorld::iConstraintInstanceBatchCount), 0);
        assert_scope_restored(&mut e, tls);
    }

    fn removal_entity(e: &mut Engine, flags: u32) -> (u32, u32) {
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x10, flags);
        let entity = e.mem.alloc(0x100);
        e.mem.set_u8(entity + 0x28, 1);
        e.mem.set_u8(entity + 0x20, 0x20);
        e.mem.set_u32(entity + 0x30 + 0xc, owner);
        (entity, owner)
    }

    #[test]
    fn entity_removal_flush_clears_flag_five_releases_and_empties() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let (marked, owner) = removal_entity(&mut e, 0xff);
        let plain = e.mem.alloc(0x100);
        let array = pointer_array(&mut e, &[marked, plain]);
        e.set(this, BhkWorld::pEntityRemoveBatch, Ptr::new(array));
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 2);
        quiet(&mut e, &[0x00c9_51b0, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_79a0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_51b0), [[world, array, 2]]);
        assert_eq!(e.mem.u32(owner + 0x10), 0xdf);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[marked], [plain]]);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 0);
        assert_eq!(e.mem.u32(array), 0);
        assert_scope_restored(&mut e, tls);
        // The count is clamped to 3000 even when there is no world.
        let bare = world_with(&mut e, 0);
        e.set(bare, BhkWorld::iEntityRemoveBatchCount, 5000);
        e.call(0x00c6_79a0, &args![bare]);
        assert_eq!(e.get(bare, BhkWorld::iEntityRemoveBatchCount), 3000);
    }

    #[test]
    fn partial_entity_removal_takes_the_last_entries_up_to_the_limit() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        e.set_global(GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT, 2u32);
        let (marked, owner) = removal_entity(&mut e, 0xff);
        let plain: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x100)).collect();
        let array = pointer_array(&mut e, &[plain[0], plain[1], plain[2], marked, plain[3]]);
        e.set(this, BhkWorld::pEntityRemoveBatch, Ptr::new(array));
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 5);
        quiet(&mut e, &[0x00c9_51b0, HAVOK_RELEASE]);
        log(&mut e);
        e.call(0x00c6_7aa0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_51b0), [[world, array + 12, 2]]);
        assert_eq!(e.mem.u32(owner + 0x10), 0xdf);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[marked], [plain[3]]]);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 3);
        assert_eq!(
            (0..5).map(|i| e.mem.u32(array + i * 4)).collect::<Vec<_>>(),
            [plain[0], plain[1], plain[2], 0, 0]
        );
        // A limit above the count takes everything.
        e.set_global(GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT, 10u32);
        log(&mut e);
        e.call(0x00c6_7aa0, &args![this]);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_51b0), [[world, array, 3]]);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 0);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn phantom_removal_flush_clears_flag_five() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let body = e.mem.alloc(0x20);
        e.mem.set_u8(body + 0x14, 0xff);
        let phantom = e.mem.alloc(0x20);
        e.mem.set_u32(phantom + 0xc, body);
        let bare = e.mem.alloc(0x20);
        let array = pointer_array(&mut e, &[phantom, 0, bare]);
        e.set(this, BhkWorld::pPhantomBatch, Ptr::new(array));
        e.set(this, BhkWorld::iPhantomBatchCount, 3);
        quiet(&mut e, &[0x00c9_5800, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_7bc0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_5800), [[world, array, 3]]);
        assert_eq!(e.mem.u8(body + 0x14), 0xdf);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[phantom], [0], [bare]]);
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 0);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn action_removal_flush_removes_only_actions_with_a_world() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let live = e.mem.alloc(0x20);
        e.mem.set_u32(live + 8, 1);
        let dead = e.mem.alloc(0x20);
        let array = pointer_array(&mut e, &[live, dead, 0]);
        e.set(this, BhkWorld::pActionBatch, Ptr::new(array));
        e.set(this, BhkWorld::iActionBatchCount, 3);
        quiet(&mut e, &[0x00c9_7f50, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_7cd0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_7f50), [[world, live]]);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[live], [dead]]);
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 0);
        // 100 or more entries are clamped to 100.
        let bare = world_with(&mut e, 0);
        e.set(bare, BhkWorld::iActionBatchCount, 150);
        e.call(0x00c6_7cd0, &args![bare]);
        assert_eq!(e.get(bare, BhkWorld::iActionBatchCount), 100);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn constraint_removal_flush_removes_only_constraints_with_a_world() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let live = e.mem.alloc(0x20);
        e.mem.set_u32(live + 8, 1);
        let dead = e.mem.alloc(0x20);
        let array = pointer_array(&mut e, &[live, dead]);
        e.set(this, BhkWorld::pConstraintInstanceBatch, Ptr::new(array));
        e.set(this, BhkWorld::iConstraintInstanceBatchCount, 2);
        quiet(&mut e, &[0x00c9_7e80, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_7df0, &args![this]);
        let seen = calls(&mut e);
        let removals = calls_to(&seen, 0x00c9_7e80);
        assert_eq!(removals.len(), 1);
        assert_eq!((removals[0][0], removals[0][2]), (world, live));
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[live], [dead]]);
        assert_eq!(e.get(this, BhkWorld::iConstraintInstanceBatchCount), 0);
        assert_scope_restored(&mut e, tls);
    }

    /// A list `head -> first -> second`, every item a counted object.
    fn action_list(e: &mut Engine) -> (u32, [u32; 3], [u32; 2]) {
        let items = [
            counted_object(e, 0x0400_2000, 0x0400_2100),
            counted_object(e, 0x0400_2200, 0x0400_2300),
            counted_object(e, 0x0400_2400, 0x0400_2500),
        ];
        let list = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, first);
        e.mem.set_u32(first, items[1]);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, items[2]);
        (list, items, [first, second])
    }

    #[test]
    fn list_clear_destroys_following_nodes_then_releases_the_head() {
        let mut e = engine();
        interlocked_doubles(&mut e);
        let (list, items, nodes) = action_list(&mut e);
        quiet(&mut e, &[0x0400_2100, 0x0400_2300, 0x0400_2500, HEAP_FREE]);
        log(&mut e);
        e.call(0x00c6_7f20, &args![list]);
        let seen = calls(&mut e);
        // Each following node is cut loose, destroyed and freed in turn, then the head goes.
        assert_eq!(
            calls_to(&seen, HEAP_FREE),
            [[HEAP_SINGLETON, nodes[0]], [HEAP_SINGLETON, nodes[1]]]
        );
        let deletes: Vec<u32> = addresses(&seen)
            .into_iter()
            .filter(|a| (0x0400_2000..0x0400_3000).contains(a))
            .collect();
        assert_eq!(deletes, [0x0400_2300, 0x0400_2500, 0x0400_2100]);
        assert_eq!(calls_to(&seen, 0x0400_2100), [[items[0]]]);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        // An empty list does nothing.
        log(&mut e);
        e.call(0x00c6_7f20, &args![list]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_7f20]);
    }

    #[test]
    fn list_destructor_body_clears_the_list_once() {
        let mut e = engine();
        interlocked_doubles(&mut e);
        let (list, items, _) = action_list(&mut e);
        quiet(&mut e, &[0x0400_2100, 0x0400_2300, 0x0400_2500, HEAP_FREE]);
        log(&mut e);
        e.call(0x00c6_8ae0, &args![list]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x0400_2100), [[items[0]]]);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(items[0] + 4), 0);
    }

    #[test]
    fn thread_state_element_and_table_constructors() {
        let mut e = engine();
        quiet(
            &mut e,
            &[0x00c8_5770, INITIALIZE_CRITICAL_SECTION, 0x00ec_782f],
        );
        let element = e.mem.alloc(0x2c);
        e.mem.set_u32(element, 9);
        log(&mut e);
        assert_eq!(e.call(0x00c6_7fc0, &args![element]).u32(), element);
        assert_eq!(e.mem.u32(element), 0);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_7fc0, 0x00c8_5770]);
        let table = e.mem.alloc(0x2e0);
        e.mem.set_u32(table + 0x2d8, 5);
        log(&mut e);
        assert_eq!(e.call(0x00c6_7fe0, &args![table]).u32(), table);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, INITIALIZE_CRITICAL_SECTION), [[table]]);
        assert_eq!(
            calls_to(&seen, 0x00ec_782f),
            [[table + 0x18, 0x2c, 0x10, 0x00c6_7fc0, 0x00a2_9680]]
        );
        assert_eq!(e.mem.u32(table + 0x2d8), 0);
        quiet(&mut e, &[0x00ec_5fce, DELETE_CRITICAL_SECTION]);
        log(&mut e);
        e.call(0x00c6_8150, &args![table]);
        let seen = calls(&mut e);
        assert_eq!(
            calls_to(&seen, 0x00ec_5fce),
            [[table + 0x18, 0x2c, 0x10, 0x00a2_9680]]
        );
        assert_eq!(calls_to(&seen, DELETE_CRITICAL_SECTION), [[table]]);
    }

    #[test]
    fn create_thread_memory_fills_the_next_element_under_the_lock() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let table = e.mem.alloc(0x2e0);
        e.mem.set_u32(table + 0x2d8, 2);
        let element = table + 0x18 + 2 * 0x2c;
        let block = e.mem.alloc(0x40);
        let allocator = counted_object(&mut e, 0x0400_3000, 0x0400_3100);
        let manager = e.mem.alloc(8);
        e.put_vtable(0x0400_3200, &[0, 0, 0, 0x0400_3300]);
        e.mem.set_u32(manager, 0x0400_3200);
        e.register_double(0x0400_3100, move |_, a| {
            assert_eq!(a[1], 0x5000);
            ret(block)
        });
        e.register_double(0x0400_3300, move |_, a| {
            assert_eq!(a[1..], [element + 4, 0x010c_3c2c, 1]);
            Ret::default()
        });
        e.register_double(0x00cb_bfc0, move |_, _| ret(manager));
        e.register_double(0x00cb_b930, move |e, a| {
            assert_eq!(a[0], element + 4);
            e.mem.set_u32(element + 0x14, allocator);
            Ret::default()
        });
        e.register_double(GET_CURRENT_THREAD_ID, |_, _| ret(0x1234));
        quiet(&mut e, &[ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]);
        log(&mut e);
        e.call(0x00c6_8050, &args![table]);
        let seen = calls(&mut e);
        assert_eq!(
            addresses(&seen),
            [
                0x00c6_8050,
                ENTER_CRITICAL_SECTION,
                0x00cb_bfc0,
                0x0400_3300,
                0x00cb_b930,
                0x0400_3100,
                GET_CURRENT_THREAD_ID,
                LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(e.mem.u32(element), 0x1234);
        assert_eq!(
            [4, 8, 12, 0x28].map(|o| e.mem.u32(element + o)),
            [block, block, block + 0x5000, block]
        );
        assert_eq!(e.mem.u32(table + 0x2d8), 3);
        assert_scope_restored(&mut e, tls);
        // A full table only takes and drops the lock.
        e.mem.set_u32(table + 0x2d8, 16);
        log(&mut e);
        e.call(0x00c6_8050, &args![table]);
        assert_eq!(
            addresses(&calls(&mut e)),
            [0x00c6_8050, ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]
        );
    }

    #[test]
    fn startup_operator_runs_create_thread_memory_on_the_global_table() {
        let mut e = engine();
        e.mem.set_u32(0x0126_7830 + 0x2d8, 16);
        quiet(&mut e, &[ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]);
        log(&mut e);
        e.call(0x00c6_81b0, &[]);
        assert_eq!(
            calls_to(&calls(&mut e), ENTER_CRITICAL_SECTION),
            [[0x0126_7830]]
        );
    }

    #[test]
    fn world_cinfo_constructor_sets_defaults_from_the_executable_constants() {
        let mut e = engine();
        e.set_global(0x0101_1d78, 1.5f32);
        e.set_global(0x011a_fed4, 2.5f32);
        e.set_global(0x0101_e2bc, 3.5f32);
        e.set_global(0x0104_da48, 4.5f32);
        e.set_global(0x011a_fe5c, 1u8);
        e.set_global(0x0126_7b38, 5.5f32);
        let this = e.mem.alloc(0xf0);
        e.register_double(0x00c9_0b80, move |_, a| {
            assert_eq!(a, [this]);
            Ret::default()
        });
        assert_eq!(e.call(0x00c6_81c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_3c44);
        assert_eq!(
            [0x10, 0x14, 0x18, 0x1c].map(|o| e.mem.f32(this + o)),
            [1.5, 2.5, 1.5, 1.5]
        );
        assert_eq!(e.mem.f32(this + 0x24), 4.5);
        assert_eq!(e.mem.f32(this + 0x54), 3.5);
        assert_eq!(e.mem.f32(this + 0x68), 5.5);
        assert_eq!(
            [0x28, 0x29, 0xa7, 0xd1].map(|o| e.mem.u8(this + o)),
            [3, 0, 0, 1]
        );
    }

    #[test]
    fn init_system_builds_the_havok_system_once() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        e.set_global(0x0126_8414, 0u8);
        e.set_global(0x0126_7b58, 0u32);
        let system = counted_object(&mut e, 0x0400_4000, 0x0400_4100);
        e.register_double(0x00cb_bfa0, |e, a| {
            e.mem.set_u32(a[0], 0x32000);
            e.mem.set_u32(a[0] + 4, 0x0060_0000);
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x0099_0000);
        e.register_double(0x0400_4100, move |e, a| {
            assert_eq!(a[0], system);
            assert_eq!(e.mem.u32(a[1]), 0);
            assert_eq!(e.mem.u32(a[1] + 4), 0x0060_0000);
            assert_eq!(a[2], 1);
            ret(router)
        });
        e.register_double(0x00cd_0a60, move |_, a| {
            assert_eq!(a[0], 0x0120_4410);
            ret(system)
        });
        let filter = e.mem.alloc(0x30);
        let allocator_object = e.mem.alloc(8);
        let allocator_table = 0x0400_4200;
        e.put_vtable(allocator_table, &[0, 0x0400_4300]);
        e.mem.set_u32(allocator_object, allocator_table);
        e.register_double(0x0400_4300, move |_, a| {
            assert_eq!(a[1], 0x30);
            ret(filter)
        });
        let memory_router = e.mem.alloc(0x20);
        e.mem.set_u32(memory_router + 0x10, allocator_object);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(memory_router));
        e.register_double(HEAP_ALLOC, |e, a| {
            assert_eq!(a[1], 0x1c);
            ret(e.mem.alloc(0x20))
        });
        e.register_double(0x00cb_f400, |_, a| ret(a[0]));
        quiet(
            &mut e,
            &[
                0x00ec_658f,
                0x00cd_06c0,
                0x00cb_bfb0,
                0x00cb_bb10,
                0x00c8_28f0,
                0x00cb_fba0,
                0x00a6_4900,
            ],
        );
        log(&mut e);
        e.call(0x00c6_8230, &[]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00ec_658f), [[0x00fd_dd40]]);
        assert_eq!(e.global::<u32>(0x0126_7b54), ALLOCATOR_VTABLE);
        assert_eq!(e.global::<u32>(0x0126_7b58) & 1, 1);
        assert_eq!(calls_to(&seen, 0x00cd_06c0), [[system, 0x0126_7b54]]);
        assert_eq!(calls_to(&seen, 0x00cb_bfb0), [[system]]);
        assert_eq!(e.global::<u32>(GLOBAL_MEMORY_SYSTEM), system);
        assert_eq!(calls_to(&seen, 0x00cb_bb10), [[router, 0x0084_cbd0, 0]]);
        assert_eq!(
            [0, 4, 8, 0xc, 0x18].map(|o| e.mem.u32(router + o)),
            [
                0x0120_77a0,
                0x0120_77a0,
                0x0126_77a0,
                0x0099_0000,
                0x0099_0000
            ]
        );
        assert_eq!(e.global::<u32>(GLOBAL_COLLISION_FILTER), filter);
        assert_eq!(e.mem.u16(filter + 4), 0x30);
        assert_eq!(
            [0, 8, 0xc, 0x10, 0x14].map(|o| e.mem.u32(filter + o)),
            [
                0x010c_3c18,
                0x010c_3c0c,
                0x010c_3bf8,
                0x010c_3bec,
                0x010c_3be0
            ]
        );
        let loaders = calls_to(&seen, 0x00a6_4900);
        assert_eq!(loaders.len(), 54);
        assert_eq!(loaders[0], [0x010c_40a0, 0x00c9_fe30]);
        assert_eq!(loaders[53], [0x010c_3c50, 0x00cc_1290]);
        assert_ne!(e.global::<u32>(GLOBAL_MULTITHREADED_SYSTEM), 0);
        assert_scope_restored(&mut e, tls);
        // Once the base system is up, nothing happens.
        e.set_global(0x0126_8414, 1u8);
        log(&mut e);
        e.call(0x00c6_8230, &[]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8230]);
    }

    #[test]
    fn scaled_broad_phase_shift_scales_the_input_with_a_zero_w() {
        let mut e = engine();
        let this = world_with(&mut e, 0x0500_0000);
        e.set_global(0x0126_77d8, 2.0f32);
        let input = e.mem.alloc(16);
        for (i, value) in [1.0f32, 2.5, -3.0].into_iter().enumerate() {
            e.mem.set_f32(input + i as u32 * 4, value);
        }
        e.register_double(0x00c9_6a20, |e, a| {
            assert_eq!(
                (0..4).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                [2.0, 5.0, -6.0, 0.0]
            );
            Ret::default()
        });
        log(&mut e);
        e.call(0x00c6_8750, &args![this, input]);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_6a20).len(), 1);
    }

    #[test]
    fn create_cinfo_builds_the_construction_info_once() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let hk_world = 0x0500_0000;
        let this = world_with(&mut e, hk_world);
        let created = e.mem.alloc(4);
        let object = e.mem.alloc(0x100);
        let allocator = counted_object(&mut e, 0x0400_5000, 0x0400_5100);
        e.register_double(0x0400_5100, move |_, a| {
            assert_eq!(a[1], 0xf0);
            ret(object)
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        quiet(&mut e, &[0x00c9_0b80, 0x00c9_1c50]);
        // No Havok object yet: the world is not told about the cinfo.
        log(&mut e);
        let info = e.call(0x00c6_87b0, &args![this, created]).u32();
        assert_eq!(info, object + 0xf0);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u16(object + 4), 0xf0);
        assert_eq!(e.mem.u32(object), 0x010c_3c44);
        assert_eq!(e.get(this, BhkWorld::pInfo).addr(), object + 0xf0);
        assert!(calls_to(&calls(&mut e), 0x00c9_1c50).is_empty());
        // Existing cinfo: nothing is built.
        log(&mut e);
        let again = e.call(0x00c6_87b0, &args![this, created]).u32();
        assert_eq!(again, object + 0xf0);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&calls(&mut e), 0x00c9_0b80).is_empty());
        // With a Havok object the cinfo base is handed to the hkpWorld.
        e.set(this, BhkWorld::pInfo, Ptr::NULL);
        e.mem.set_u32(this.addr() + 8, 0x0600_0000);
        log(&mut e);
        e.call(0x00c6_87b0, &args![this, created]);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_1c50), [[hk_world, object]]);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn debug_color_update_pops_pairs_from_the_back() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        let pairs = pointer_array(&mut e, &[1, 2, 3, 4, 5, 6]);
        e.set(this, BhkWorld::debugColorData, Ptr::new(pairs));
        e.set(this, BhkWorld::debugColorCount, 3);
        e.set_global(0x0126_81e0, 0x0777_0000u32);
        quiet(&mut e, &[0x00c9_84a0]);
        // Without a debugger only the count is reset.
        log(&mut e);
        e.call(0x00c6_88b0, &args![this]);
        assert!(calls_to(&calls(&mut e), 0x00c9_84a0).is_empty());
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 0);
        e.set(this, BhkWorld::debugColorCount, 3);
        e.set(this, BhkWorld::pDebugger, Ptr::new(0x0700_0000));
        log(&mut e);
        e.call(0x00c6_88b0, &args![this]);
        assert_eq!(
            calls_to(&calls(&mut e), 0x00c9_84a0),
            [
                [0x0777_0000, 6, 5, 0],
                [0x0777_0000, 4, 3, 0],
                [0x0777_0000, 2, 1, 0]
            ]
        );
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 0);
    }

    #[test]
    fn scene_visit_marks_flags_calls_back_and_recurses_into_children() {
        let mut e = engine();
        let request = e.mem.alloc(0x10);
        e.mem.set_u8(request + 4, 1);
        // Leaf: object with a holder whose slot +0x38 returns a found object.
        let found = 0x0888_0000u32;
        e.put_vtable(0x0400_6000, &[0; 0x20]);
        e.mem.set_u32(0x0400_6000 + 0x38, 0x0400_6038);
        e.register_double(0x0400_6038, move |_, _| ret(found));
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0x0400_6000);
        // Node slot +0x0C returns the node itself (as `IsNode`).
        e.put_vtable(0x0400_6100, &[0, 0, 0, 0x0400_610c]);
        let root = e.mem.alloc(0xb0);
        e.mem.set_u32(root, 0x0400_6100);
        e.register_double(0x0400_610c, move |_, a| ret(a[0]));
        let child = e.mem.alloc(0xb0);
        e.put_vtable(0x0400_6200, &[0, 0, 0, 0x0400_620c]);
        e.register_double(0x0400_620c, |_, _| Ret::default());
        e.mem.set_u32(child, 0x0400_6200);
        e.mem.set_u32(child + 0x1c, holder);
        e.mem.set_u32(child + 0x30, 0x1f);
        let children = pointer_array(&mut e, &[child, 0]);
        e.mem.set_u32(root + 0xa0, children);
        e.mem.set_u16(root + 0xa6, 2);
        e.register_double(0x0400_6300, |_, _| Ret::default());
        log(&mut e);
        e.call(0x00c6_8900, &args![root, request, 0x0400_6300u32]);
        assert_eq!(calls_to(&calls(&mut e), 0x0400_6300), [[found, request]]);
        // The request's +8 is zero, so the child's flags get bits 1 and 2 and lose bit 4.
        assert_eq!(e.mem.u32(child + 0x30), 0x0f);
        assert_eq!(e.mem.u32(root + 0x30), 6);
        // With +8 set the flags stay.
        e.mem.set_u32(request + 8, 1);
        e.mem.set_u32(child + 0x30, 0x10);
        e.call(0x00c6_8900, &args![child, request, 0x0400_6300u32]);
        assert_eq!(e.mem.u32(child + 0x30), 0x10);
        // A null object is ignored.
        e.call(0x00c6_8900, &args![0u32, request, 0x0400_6300u32]);
    }

    #[test]
    fn removal_flush_order_depends_on_the_partial_flag() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        // One queued item in every batch.
        let entity = e.mem.alloc(0x100);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 8, 1);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 8, 1);
        let phantom = e.mem.alloc(0x20);
        for (array_field, count_field, item) in [
            (
                BhkWorld::pConstraintInstanceBatch,
                BhkWorld::iConstraintInstanceBatchCount,
                constraint,
            ),
            (BhkWorld::pActionBatch, BhkWorld::iActionBatchCount, action),
            (
                BhkWorld::pPhantomBatch,
                BhkWorld::iPhantomBatchCount,
                phantom,
            ),
            (
                BhkWorld::pEntityRemoveBatch,
                BhkWorld::iEntityRemoveBatchCount,
                entity,
            ),
        ] {
            let array = pointer_array(&mut e, &[item]);
            e.set(this, array_field, Ptr::new(array));
            e.set(this, count_field, 1);
        }
        let entities = e.get(this, BhkWorld::pEntityRemoveBatch).addr();
        quiet(
            &mut e,
            &[
                0x00c9_7e80,
                0x00c9_7f50,
                0x00c9_5800,
                0x00c9_51b0,
                HAVOK_RELEASE,
            ],
        );
        memset_double(&mut e);
        e.set_global(GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT, 1u32);
        let flushes = [0x00c9_7e80, 0x00c9_7f50, 0x00c9_5800, 0x00c9_51b0];
        for partial in [0u8, 1] {
            log(&mut e);
            e.call(0x00c6_8a00, &args![this, partial]);
            let seen = calls(&mut e);
            let order: Vec<u32> = addresses(&seen)
                .into_iter()
                .filter(|a| flushes.contains(a))
                .collect();
            assert_eq!(order, flushes);
            if partial == 1 {
                assert_eq!(calls_to(&seen, 0x00c9_51b0), [[world, entities, 1]]);
            }
            // Refill for the next round.
            e.mem.set_u32(entities, entity);
            let constraints = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
            e.mem.set_u32(constraints, constraint);
            let actions = e.get(this, BhkWorld::pActionBatch).addr();
            e.mem.set_u32(actions, action);
            let phantoms = e.get(this, BhkWorld::pPhantomBatch).addr();
            e.mem.set_u32(phantoms, phantom);
            e.set(this, BhkWorld::iConstraintInstanceBatchCount, 1);
            e.set(this, BhkWorld::iActionBatchCount, 1);
            e.set(this, BhkWorld::iPhantomBatchCount, 1);
            e.set(this, BhkWorld::iEntityRemoveBatchCount, 1);
        }
    }

    #[test]
    fn flush_of_entity_additions_clamps_and_empties_the_batch() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let entity = e.mem.alloc(0x100);
        let array = pointer_array(&mut e, &vec![entity; 3000]);
        e.set(this, BhkWorld::pEntityBatch, Ptr::new(array));
        e.set(this, BhkWorld::iEntityBatchCount, 3500);
        quiet(
            &mut e,
            &[
                0x00c9_4bd0,
                0x00c9_1620,
                0x00c9_2a80,
                0x00cb_6e70,
                HAVOK_RELEASE,
            ],
        );
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_8a40, &args![this]);
        assert_eq!(
            calls_to(&calls(&mut e), 0x00c9_4bd0),
            [[world, array, 3000, 0]]
        );
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 0);
        assert_scope_restored(&mut e, tls);
        // An empty batch does nothing.
        log(&mut e);
        e.call(0x00c6_8a40, &args![this]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8a40]);
    }

    #[test]
    fn list_add_head_pushes_the_old_head_into_a_new_second_node() {
        let mut e = engine();
        interlocked_doubles(&mut e);
        let item = counted_object(&mut e, 0x0400_7000, 0x0400_7100);
        let old = counted_object(&mut e, 0x0400_7200, 0x0400_7300);
        let list = e.mem.alloc(8);
        let slot = e.mem.alloc(4);
        // A null item is ignored.
        e.call(0x00c6_8b50, &args![list, slot]);
        assert_eq!(e.mem.u32(list), 0);
        // An empty list just takes the item.
        e.mem.set_u32(slot, item);
        e.call(0x00c6_8b50, &args![list, slot]);
        assert_eq!(e.mem.u32(list), item);
        assert_eq!(e.mem.u32(item + 4), 2);
        // A non-empty list gets a node for the old head.
        let node = e.mem.alloc(8);
        e.register_double(HEAP_ALLOC, move |_, a| {
            assert_eq!(a, [HEAP_SINGLETON, 8]);
            ret(node)
        });
        e.mem.set_u32(list, old);
        e.mem.set_u32(list + 4, 0x0999_0000);
        e.call(0x00c6_8b50, &args![list, slot]);
        assert_eq!(e.mem.u32(list), item);
        assert_eq!(e.mem.u32(list + 4), node);
        assert_eq!(e.mem.u32(node), old);
        assert_eq!(e.mem.u32(node + 4), 0x0999_0000);
        // The new node took a reference and the head dropped its own.
        assert_eq!(e.mem.u32(old + 4), 1);
    }

    #[test]
    fn world_cinfo_delete_runs_the_base_destructor_and_frees_on_request() {
        let mut e = engine();
        let this = e.mem.alloc(0xf0);
        e.mem.set_u16(this + 4, 0xf0);
        let allocator = counted_object(&mut e, 0x0400_8000, 0x0400_8100);
        e.put_vtable(0x0400_8000, &[0, 0, 0x0400_8100]);
        e.register_double(0x0400_8100, move |_, a| {
            assert_eq!(a[1..], [this, 0xf0]);
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        quiet(&mut e, &[0x0055_3540]);
        log(&mut e);
        assert_eq!(e.call(0x00c6_8bc0, &args![this, 0u8]).u32(), this);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8bc0, 0x0055_3540]);
        log(&mut e);
        assert_eq!(e.call(0x00c6_8bc0, &args![this, 1u8]).u32(), this);
        assert!(addresses(&calls(&mut e)).contains(&0x0400_8100));
    }

    fn removal_object(e: &mut Engine, flags_at: u32, flags: u32) -> (u32, u32) {
        let body = e.mem.alloc(0x20);
        match flags_at {
            0x10 => e.mem.set_u32(body + 0x10, flags),
            _ => e.mem.set_u8(body + flags_at, flags as u8),
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0xc, body);
        (object, body)
    }

    #[test]
    fn remove_entity_queues_or_removes_at_once() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let (object, body) = removal_object(&mut e, 0x10, 0x01);
        let array = e.mem.alloc(0x40);
        e.set(this, BhkWorld::pEntityRemoveBatch, Ptr::new(array));
        quiet(&mut e, &[HAVOK_ADD_REFERENCE]);
        // Immediate removal reports the result byte.
        e.register_double(0x00c9_1620, |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(a[1])
        });
        assert!(e.call(0x00c6_8bf0, &args![this, object]).bool());
        e.register_double(0x00c9_1620, |e, a| {
            e.mem.set_u8(a[1], 0);
            ret(a[1])
        });
        assert!(!e.call(0x00c6_8bf0, &args![this, object]).bool());
        // Already queued or removing bodies, and bodiless objects, are reported as done.
        e.mem.set_u32(body + 0x10, 0x10);
        assert!(e.call(0x00c6_8bf0, &args![this, object]).bool());
        // Batch mode queues the object and marks the body.
        e.mem.set_u32(body + 0x10, 0x01);
        e.set(this, BhkWorld::iBatchRemove, 1);
        log(&mut e);
        assert!(e.call(0x00c6_8bf0, &args![this, object]).bool());
        assert_eq!(e.mem.u32(body + 0x10), 0x21);
        assert_eq!(e.mem.u32(array), object);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 1);
        assert_eq!(calls_to(&calls(&mut e), HAVOK_ADD_REFERENCE), [[object]]);
        // Already marked: nothing more is queued.
        assert!(e.call(0x00c6_8bf0, &args![this, object]).bool());
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 1);
        // Missing world or object.
        assert!(!e.call(0x00c6_8bf0, &args![this, 0u32]).bool());
        let bare = world_with(&mut e, 0);
        assert!(!e.call(0x00c6_8bf0, &args![bare, object]).bool());
    }

    #[test]
    fn remove_entity_flushes_a_full_batch_first() {
        let mut e = engine();
        let this = world_with(&mut e, 0x0500_0000);
        let (object, _) = removal_object(&mut e, 0x10, 0x01);
        let array = e.mem.alloc(0x4000);
        e.set(this, BhkWorld::pEntityRemoveBatch, Ptr::new(array));
        e.set(this, BhkWorld::iBatchRemove, 1);
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 0xbb8);
        quiet(&mut e, &[HAVOK_ADD_REFERENCE, 0x00c9_51b0, HAVOK_RELEASE]);
        memset_double(&mut e);
        log(&mut e);
        e.call(0x00c6_8bf0, &args![this, object]);
        // The partial flush takes none of the entries (limit 0), then the object is queued.
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_51b0).len(), 1);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 0xbb9);
        assert_eq!(e.mem.u32(array + 0xbb8 * 4), object);
    }

    #[test]
    fn remove_phantom_queues_or_removes_at_once() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let (object, body) = removal_object(&mut e, 0x14, 0x01);
        let array = e.mem.alloc(0x40);
        e.set(this, BhkWorld::pPhantomBatch, Ptr::new(array));
        quiet(&mut e, &[HAVOK_ADD_REFERENCE, 0x00c9_2c70]);
        log(&mut e);
        assert!(e.call(0x00c6_8cb0, &args![this, object]).bool());
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_2c70), [[world, object]]);
        // A body that is already marked is left alone.
        e.mem.set_u8(body + 0x14, 0x10);
        log(&mut e);
        assert!(e.call(0x00c6_8cb0, &args![this, object]).bool());
        assert!(calls_to(&calls(&mut e), 0x00c9_2c70).is_empty());
        // Batch mode queues and marks.
        e.mem.set_u8(body + 0x14, 0x01);
        e.set(this, BhkWorld::iBatchRemove, 1);
        assert!(e.call(0x00c6_8cb0, &args![this, object]).bool());
        assert_eq!(e.mem.u8(body + 0x14), 0x21);
        assert_eq!(e.mem.u32(array), object);
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 1);
        // A bodiless object counts as done; a missing world or object does not.
        let bodiless = e.mem.alloc(0x20);
        assert!(e.call(0x00c6_8cb0, &args![this, bodiless]).bool());
        assert!(!e.call(0x00c6_8cb0, &args![this, 0u32]).bool());
        let bare = world_with(&mut e, 0);
        assert!(!e.call(0x00c6_8cb0, &args![bare, object]).bool());
    }

    #[test]
    fn remove_constraint_queues_or_removes_at_once() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let object = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x40);
        e.set(this, BhkWorld::pConstraintInstanceBatch, Ptr::new(array));
        quiet(&mut e, &[HAVOK_ADD_REFERENCE, 0x00c9_7e80]);
        log(&mut e);
        assert!(e.call(0x00c6_8d50, &args![this, object]).bool());
        let removals = calls_to(&calls(&mut e), 0x00c9_7e80);
        assert_eq!((removals[0][0], removals[0][2]), (world, object));
        e.set(this, BhkWorld::iBatchRemove, 2);
        log(&mut e);
        assert!(e.call(0x00c6_8d50, &args![this, object]).bool());
        assert_eq!(e.mem.u32(array), object);
        assert_eq!(e.get(this, BhkWorld::iConstraintInstanceBatchCount), 1);
        assert_eq!(calls_to(&calls(&mut e), HAVOK_ADD_REFERENCE), [[object]]);
        assert!(!e.call(0x00c6_8d50, &args![this, 0u32]).bool());
        let bare = world_with(&mut e, 0);
        assert!(!e.call(0x00c6_8d50, &args![bare, object]).bool());
    }

    #[test]
    fn update_debugger_resets_the_monitor_stream_then_the_colors() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        // Without a debugger only the color update runs (it clears the count).
        e.set(this, BhkWorld::debugColorCount, 4);
        e.call(0x00c6_8de0, &args![this]);
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 0);
        let debugger = e.mem.alloc(8);
        e.put_vtable(0x0400_9000, &[0, 0, 0, 0x0400_900c]);
        e.mem.set_u32(debugger, 0x0400_9000);
        e.set(this, BhkWorld::pDebugger, Ptr::new(debugger));
        e.register_double(0x0400_900c, move |_, a| {
            assert_eq!(a, [debugger, 0]);
            Ret::default()
        });
        e.set_global(0x0126_835c, 3u32);
        e.register_double(TLS_GET_VALUE, |_, a| {
            assert_eq!(a, [3]);
            ret(0x0abc_0000)
        });
        let system = e.mem.alloc(0x20);
        let monitor = e.mem.alloc(8);
        e.put_vtable(0x0400_9100, &[0; 10]);
        e.mem.set_u32(0x0400_9100 + 0x24, 0x0400_9124);
        e.mem.set_u32(monitor, 0x0400_9100);
        e.mem.set_u32(system + 0x18, monitor);
        e.set_global(GLOBAL_MULTITHREADED_SYSTEM, system);
        e.register_double(0x0400_9124, move |_, a| {
            assert_eq!(a, [monitor]);
            Ret::default()
        });
        quiet(&mut e, &[0x00cb_0da0]);
        log(&mut e);
        e.call(0x00c6_8de0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(
            addresses(&seen),
            [
                0x00c6_8de0,
                0x0400_900c,
                TLS_GET_VALUE,
                0x00cb_0da0,
                0x0400_9124
            ]
        );
        assert_eq!(calls_to(&seen, 0x00cb_0da0), [[0x0abc_0000]]);
        // Without the system's monitor object the last call is skipped.
        e.mem.set_u32(system + 0x18, 0);
        log(&mut e);
        e.call(0x00c6_8de0, &args![this]);
        assert!(calls_to(&calls(&mut e), 0x0400_9124).is_empty());
    }
    #[test]
    fn debug_object_color_appends_a_pair_and_grows_a_full_array() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let array = this.addr() + 0x7c;
        let wrapper = e.mem.alloc(0x40);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 8, wrapper);
        let allocator = 0x0766_0000;
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 0x8000_0002);
        e.call(0x00c6_8e30, &args![this, object, 0xff00_00ffu32]);
        assert_eq!(e.mem.u32(data), wrapper + 0x10);
        assert_eq!(e.mem.u32(data + 4), 0xff00_00ff);
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 1);
        // The count equals the capacity (flag bits masked off): the array grows first.
        let grown = e.mem.alloc(0x80);
        e.mem.set_u32(array + 8, 0x8000_0001);
        e.register_double(0x00c9_0940, move |e, a| {
            assert_eq!(a, [allocator, array, 8]);
            let old = e.mem.bytes(data, 8);
            e.mem.write(grown, &old);
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 0x8000_0004);
            Ret::default()
        });
        let null_object = e.mem.alloc(0x20);
        e.call(0x00c6_8e30, &args![this, null_object, 5u32]);
        assert_eq!(e.mem.u32(grown), wrapper + 0x10);
        assert_eq!(e.mem.u32(grown + 8), 0);
        assert_eq!(e.mem.u32(grown + 12), 5);
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 2);
    }

    #[test]
    fn request_callback_table_dispatches_by_kind() {
        let mut e = engine();
        let request = e.mem.alloc(0x10);
        e.mem.set_u32(request + 8, 2);
        e.set_global(0x011a_fe88 + 8, 0x0400_a000u32);
        let holder = e.mem.alloc(8);
        e.put_vtable(0x0400_a100, &[0; 0x20]);
        e.mem.set_u32(0x0400_a100 + 0x38, 0x0400_a138);
        e.mem.set_u32(holder, 0x0400_a100);
        e.register_double(0x0400_a138, |_, _| ret(0x0aaa_0000));
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x1c, holder);
        e.register_double(0x0400_a000, |_, _| Ret::default());
        log(&mut e);
        e.call(0x00c6_8e90, &args![object, request]);
        assert_eq!(
            calls_to(&calls(&mut e), 0x0400_a000),
            [[0x0aaa_0000, request]]
        );
        // Kind 3 has no entry: the object is not visited.
        e.mem.set_u32(request + 8, 3);
        log(&mut e);
        e.call(0x00c6_8e90, &args![object, request]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8e90]);
    }

    #[test]
    fn conditional_visit_needs_a_record_with_bit_one() {
        let mut e = engine();
        let request = e.mem.alloc(0x10);
        e.mem.set_u32(request + 8, 1);
        let object = e.mem.alloc(0x40);
        let record = e.mem.alloc(0x20);
        e.register_double(0x00c4_3490, move |_, a| {
            assert_eq!(a, [object]);
            ret(record)
        });
        assert!(!e
            .call(0x00c6_8ec0, &args![0u32, request, 0x0400_b000u32])
            .bool());
        assert!(!e
            .call(0x00c6_8ec0, &args![object, request, 0x0400_b000u32])
            .bool());
        e.mem.set_u32(record + 0xc, 1);
        assert!(!e
            .call(0x00c6_8ec0, &args![object, request, 0x0400_b000u32])
            .bool());
        e.mem.set_u32(record + 0xc, 3);
        let holder = e.mem.alloc(8);
        e.put_vtable(0x0400_b100, &[0; 0x20]);
        e.mem.set_u32(0x0400_b100 + 0x38, 0x0400_b138);
        e.mem.set_u32(holder, 0x0400_b100);
        e.mem.set_u32(object + 0x1c, holder);
        e.register_double(0x0400_b138, |_, _| ret(0x0aaa_0000));
        e.register_double(0x0400_b000, |_, a| {
            assert_eq!(a[0], 0x0aaa_0000);
            Ret::default()
        });
        assert!(e
            .call(0x00c6_8ec0, &args![object, request, 0x0400_b000u32])
            .bool());
    }

    #[test]
    fn end_of_batch_removal_flushes_when_the_level_reaches_zero() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        quiet(&mut e, &[0x00c9_51b0, HAVOK_RELEASE]);
        let entity = e.mem.alloc(0x100);
        let array = pointer_array(&mut e, &[entity]);
        e.set(this, BhkWorld::pEntityRemoveBatch, Ptr::new(array));
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 1);
        memset_double(&mut e);
        // Level 2 -> 1: nothing is flushed.
        e.set(this, BhkWorld::iBatchRemove, 2);
        log(&mut e);
        e.call(0x00c6_8f00, &args![this, 0u8]);
        assert_eq!(e.get(this, BhkWorld::iBatchRemove), 1);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8f00]);
        // Level 1 -> 0: everything is flushed (the full entity flush for 0).
        e.mem.set_u32(array, entity);
        log(&mut e);
        e.call(0x00c6_8f00, &args![this, 0u8]);
        assert_eq!(e.get(this, BhkWorld::iBatchRemove), 0);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_51b0), [[world, array, 1]]);
        // Level 0: a pending batch is flushed only for the full form.
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 1);
        e.mem.set_u32(array, entity);
        log(&mut e);
        e.call(0x00c6_8f00, &args![this, 1u8]);
        assert!(calls_to(&calls(&mut e), 0x00c9_51b0).is_empty());
        log(&mut e);
        e.call(0x00c6_8f00, &args![this, 0u8]);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_51b0).len(), 1);
        // Level 1 -> 0 with the partial form uses the partial entity flush.
        e.set(this, BhkWorld::iBatchRemove, 1);
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 1);
        e.set_global(GLOBAL_ENTITY_REMOVE_PARTIAL_LIMIT, 1u32);
        e.mem.set_u32(array, entity);
        log(&mut e);
        e.call(0x00c6_8f00, &args![this, 1u8]);
        assert_eq!(e.get(this, BhkWorld::iEntityRemoveBatchCount), 0);
        assert_eq!(calls_to(&calls(&mut e), 0x00c9_51b0), [[world, array, 1]]);
    }

    #[test]
    fn list_remove_handles_head_middle_and_missing_items() {
        let mut e = engine();
        interlocked_doubles(&mut e);
        quiet(&mut e, &[0x0400_2100, 0x0400_2300, 0x0400_2500, HEAP_FREE]);
        // Missing item: nothing changes.
        let (list, items, nodes) = action_list(&mut e);
        let stranger = counted_object(&mut e, 0x0400_c000, 0x0400_c100);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, stranger);
        log(&mut e);
        e.call(0x00c6_8f50, &args![list, slot]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_8f50]);
        // A middle node is unlinked, destroyed and freed.
        e.mem.set_u32(slot, items[1]);
        log(&mut e);
        e.call(0x00c6_8f50, &args![list, slot]);
        let seen = calls(&mut e);
        assert_eq!(e.mem.u32(list + 4), nodes[1]);
        assert_eq!(calls_to(&seen, HEAP_FREE), [[HEAP_SINGLETON, nodes[0]]]);
        assert_eq!(calls_to(&seen, 0x0400_2300), [[items[1]]]);
        // The head: the following node's item moves into the head.
        e.mem.set_u32(slot, items[0]);
        log(&mut e);
        e.call(0x00c6_8f50, &args![list, slot]);
        let seen = calls(&mut e);
        assert_eq!(e.mem.u32(list), items[2]);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(calls_to(&seen, HEAP_FREE), [[HEAP_SINGLETON, nodes[1]]]);
        // A single remaining head item is released and cleared.
        e.mem.set_u32(slot, items[2]);
        log(&mut e);
        e.call(0x00c6_8f50, &args![list, slot]);
        assert_eq!(e.mem.u32(list), 0);
        // An empty list and a null item do nothing.
        e.call(0x00c6_8f50, &args![list, slot]);
        e.mem.set_u32(slot, 0);
        e.call(0x00c6_8f50, &args![list, slot]);
    }

    /// Doubles for everything `bhkWorld::Init` (this file) calls while the
    /// constructor runs; returns the 0x360-byte Havok world it creates.
    fn constructor_doubles(e: &mut Engine, hk_world: u32, expected_cinfo: u32) -> u32 {
        let created = e.mem.alloc(0x360);
        let allocator = counted_object(e, 0x0400_d100, 0x0400_d200);
        e.register_double(0x0400_d200, move |_, a| {
            assert_eq!(a[1], 0x360);
            ret(created)
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        let mut slots = vec![0; 0x30];
        slots[0x8c / 4] = 0x0400_d28c;
        slots[0x94 / 4] = GETTER;
        slots[0xbc / 4] = 0x0400_d2bc;
        e.put_vtable(0x010c_40b4, &slots);
        e.register_double(GETTER, move |_, _| ret(hk_world));
        e.register_double(0x00c8_7c30, |e, a| {
            e.mem.set_u32(a[0], 0x010c_40b4);
            Ret::default()
        });
        e.register_double(0x0400_d2bc, move |_, a| {
            assert_eq!(a[1], expected_cinfo);
            Ret::default()
        });
        quiet(
            e,
            &[0x0400_d28c, 0x00c9_5a80, HAVOK_RELEASE, 0x00cb_fd20, MEMSET],
        );
        created
    }

    #[test]
    fn world_constructor_initialises_fields_and_arrays() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let this: Ptr<BhkWorld> = e.new_object();
        let hk_world = e.mem.alloc(0x304);
        e.mem.set_f32(hk_world + 0x1e4, 0.25);
        e.mem.set_f32(hk_world + 0x1e8, 0.75);
        constructor_doubles(&mut e, hk_world, 0x0900_00f0);
        e.set_global(GLOBAL_WORLD_COUNT, 4u32);
        e.set_global(0x0126_7b38, 3.0f32);
        for lane in 0..4 {
            e.mem
                .set_f32(GLOBAL_ZERO_VECTOR + lane * 4, lane as f32 + 1.0);
        }
        let blocks: Vec<u32> = (0..5).map(|_| e.mem.alloc(12000)).collect();
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen_sizes = sizes.clone();
        let mut next = 0;
        let handed = blocks.clone();
        e.register_double(HEAP_ALLOC, move |_, a| {
            seen_sizes.borrow_mut().push(a[1]);
            next += 1;
            ret(handed[next - 1])
        });
        let cinfo = Ptr::<()>::new(0x0900_0000);
        let result = e.call(0x00c6_9010, &args![this, cinfo]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(*sizes.borrow(), [12000, 800, 400, 800, 12000]);
        assert_eq!(
            [0x2c, 0x34, 0x3c, 0x44, 0x4c].map(|o| e.mem.u32(this.addr() + o)),
            [blocks[0], blocks[1], blocks[2], blocks[3], blocks[4]]
        );
        assert_eq!(e.global::<u32>(GLOBAL_WORLD_COUNT), 5);
        assert_eq!(e.global::<f32>(0x0126_7b38), 1.0);
        assert_eq!(e.mem.u32(this.addr()), 0x010c_40b4);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.f32(this.addr() + 0x60 + i * 4))
                .collect::<Vec<_>>(),
            [1.0, 2.0, 3.0, 4.0]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x78), 0x8000_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x84), 0x8000_0000);
        assert_eq!(e.get(this, BhkWorld::fOriginalTau), 0.25);
        assert_eq!(e.get(this, BhkWorld::fOriginalDamping), 0.75);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn world_constructor_keeps_a_smaller_tau_ratio_and_a_null_cinfo() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let hk_world = e.mem.alloc(0x304);
        constructor_doubles(&mut e, hk_world, 0);
        e.set_global(0x0126_7b38, 0.5f32);
        e.register_double(HEAP_ALLOC, |e, _| ret(e.mem.alloc(12000)));
        e.call(0x00c6_9010, &args![this, 0u32]);
        assert_eq!(e.global::<f32>(0x0126_7b38), 0.5);
    }

    #[test]
    fn class_constants_are_the_rtti_address_and_the_cinfo_size() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        assert_eq!(e.call(0x00c6_9220, &args![this]).u32(), 0x0126_7824);
        assert_eq!(e.call(0x00c6_9230, &args![this]).u32(), 0xf0);
    }

    #[test]
    fn viewer_strings_append_the_flags_and_counters_in_order() {
        let mut e = engine();
        let tls = scope_marker(&mut e);
        let this: Ptr<BhkWorld> = e.new_object();
        e.set(this, BhkWorld::bEnabled, true);
        e.set(this, BhkWorld::pDebugger, Ptr::new(0x0700_0000));
        e.mem.set_u32(0x0126_7824, 0x0abc_0001);
        let counters = [
            0x0126_77a4u32,
            0x0126_8154,
            0x0126_8188,
            0x0126_81fc,
            0x0126_81bc,
            0x0126_80f8,
            0x0126_8238,
            0x0126_8654,
        ];
        for (i, counter) in counters.into_iter().enumerate() {
            e.mem.set_u32(counter, 100 + i as u32);
        }
        // The string array: capacity 2, count 0, grow-by 4.
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 0);
        e.mem.set_u16(strings + 0xe, 4);
        quiet(&mut e, &[0x00c8_78c0]);
        e.register_double(0x00a7_d330, |_, a| {
            assert_eq!(a, [0x0abc_0001]);
            ret(0x1000)
        });
        e.register_double(0x00a7_d370, |_, a| ret(a[0] ^ (a[1] & 0xff)));
        e.register_double(0x00a7_d510, |_, a| ret(a[0] + a[1]));
        let pushed = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen_pushed = pushed.clone();
        e.register_double(0x0096_ae90, move |e, a| {
            seen_pushed.borrow_mut().push((a[1], e.mem.u32(a[2])));
            let count = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, count);
            Ret::default()
        });
        let grown = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen_grown = grown.clone();
        e.register_double(0x0096_ad30, move |e, a| {
            seen_grown.borrow_mut().push(a[1]);
            let capacity = e.mem.u16(a[0] + 8) + 4;
            e.mem.set_u16(a[0] + 8, capacity);
            Ret::default()
        });
        log(&mut e);
        e.call(0x00c6_9240, &args![this, strings]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c8_78c0), [[this.addr(), strings]]);
        let pushed = pushed.borrow();
        assert_eq!(pushed.len(), 11);
        assert_eq!(
            pushed.iter().map(|p| p.0).collect::<Vec<_>>(),
            (0..11).collect::<Vec<_>>()
        );
        assert_eq!(pushed[0].1, 0x1000);
        assert_eq!(pushed[1].1, 0x0109_c7d8 ^ 1);
        assert_eq!(pushed[2].1, 0x010c_41ec ^ 1);
        assert_eq!(pushed[3].1, 0x010c_41e4 + 100);
        assert_eq!(pushed[10].1, 0x010c_4190 + 107);
        // Grown whenever the count reached the capacity: at 2, 6 and 10 entries.
        assert_eq!(*grown.borrow(), [6, 10, 14]);
        // The flag strings receive the previous string pointer in the upper bytes.
        let flag_calls = calls_to(&seen, 0x00a7_d370);
        assert_eq!(flag_calls[0], [0x0109_c7d8, 0x1001]);
        assert_eq!(flag_calls[1][1], (0x0109_c7d8 ^ 1) & !0xff | 1);
        assert_scope_restored(&mut e, tls);
    }

    #[test]
    fn remove_action_updates_the_list_then_queues_or_removes() {
        let mut e = engine();
        interlocked_doubles(&mut e);
        let tls = scope_marker(&mut e);
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let reference = counted_object(&mut e, 0x0400_e000, 0x0400_e100);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0x10, reference);
        let array = e.mem.alloc(0x400);
        e.set(this, BhkWorld::pActionBatch, Ptr::new(array));
        // The action list at +0x90 holds only the action's reference.
        e.mem.set_u32(this.addr() + 0x90, reference);
        e.mem.set_u32(reference + 4, 1);
        quiet(
            &mut e,
            &[0x0400_e100, HAVOK_ADD_REFERENCE, 0x00c9_7f50, HEAP_FREE],
        );
        log(&mut e);
        assert!(e.call(0x00c6_9580, &args![this, action]).bool());
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x00c9_7f50), [[world, action]]);
        assert_eq!(e.mem.u32(this.addr() + 0x90), 0);
        // The temporary reference went up and down again; the list's own count was dropped.
        assert_eq!(e.mem.u32(reference + 4), 0);
        // Batch mode queues the action instead.
        e.set(this, BhkWorld::iBatchRemove, 1);
        e.mem.set_u32(action + 0x10, 0);
        log(&mut e);
        assert!(e.call(0x00c6_9580, &args![this, action]).bool());
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, HAVOK_ADD_REFERENCE), [[action]]);
        assert_eq!(e.mem.u32(array), action);
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 1);
        assert!(calls_to(&seen, 0x00c9_7f50).is_empty());
        // Missing world or action.
        assert!(!e.call(0x00c6_9580, &args![this, 0u32]).bool());
        let bare = world_with(&mut e, 0);
        assert!(!e.call(0x00c6_9580, &args![bare, action]).bool());
        assert_scope_restored(&mut e, tls);
    }

    // ----- picking, adding objects and the notification requests -----

    /// Writes a 0x80-slot vtable at `address` with the given (byte offset, target) slots.
    fn vtable_at(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        let mut table = vec![0u32; 0x80];
        for &(offset, target) in slots {
            table[(offset / 4) as usize] = target;
        }
        e.put_vtable(address, &table);
    }

    /// The world getter answers `world` once and no world afterwards, so the
    /// flushers that run in between find nothing to flush.
    fn once_world(e: &mut Engine, this: Ptr<BhkWorld>, world: u32) {
        e.mem.set_u32(this.addr() + 0x9c, 0);
        e.register_double(GETTER, move |e, a| {
            let calls = e.mem.u32(a[0] + 0x9c);
            e.mem.set_u32(a[0] + 0x9c, calls + 1);
            ret(if calls == 0 { world } else { 0 })
        });
    }

    /// Request callback double for `kind`: copies the 0x1C-byte request into
    /// the returned sink (`+0x20`: the `found` object it was called with,
    /// `+0x24`: call count) and sets the request's word `+0xC` to `reply`
    /// when that is non-zero.
    fn record_requests(e: &mut Engine, kind: u32, reply: u32) -> u32 {
        let sink = e.mem.alloc(0x40);
        let double = 0x0400_8000 + kind * 4;
        e.mem.set_u32(REQUEST_CALLBACK_TABLE + kind * 4, double);
        e.register_double(double, move |e, a| {
            let bytes = e.mem.bytes(a[1], 0x1c);
            e.mem.write(sink, &bytes);
            e.mem.set_u32(sink + 0x20, a[0]);
            let count = e.mem.u32(sink + 0x24);
            e.mem.set_u32(sink + 0x24, count + 1);
            if reply != 0 {
                e.mem.set_u32(a[1] + 0xc, reply);
            }
            Ret::default()
        });
        sink
    }

    const FOUND: u32 = 0x0abc_0001;

    /// An object `fn_00c68900` visits: its holder (word `+0x1C`) answers
    /// `found` from slot `+0x38`, and slot `+0x0C` of the object finds no
    /// node. `id` keeps the tables of several objects apart.
    fn visitable_object_with(e: &mut Engine, id: u32, found: u32) -> u32 {
        let base = 0x0400_c000 + id * 0x1000;
        vtable_at(e, base, &[(0xc, base + 0x800)]);
        vtable_at(e, base + 0x400, &[(0x38, base + 0x804)]);
        e.register_double(base + 0x800, |_, _| Ret::default());
        e.register_double(base + 0x804, move |_, _| ret(found));
        let object = e.mem.alloc(0xb0);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(object, base);
        e.mem.set_u32(object + 0x1c, holder);
        e.mem.set_u32(holder, base + 0x400);
        object
    }

    fn visitable_object(e: &mut Engine) -> u32 {
        visitable_object_with(e, 0, FOUND)
    }

    /// `00c43490` answers a record whose word `+0xC` is `bits`.
    fn registered(e: &mut Engine, bits: u32) {
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0xc, bits);
        e.register_double(OBJECT_RECORD, move |_, _| ret(record));
    }

    struct Sent {
        result: Ret,
        words: [u32; 7],
        count: u32,
        found: u32,
    }

    /// Runs the request function at `address` on a registered, visitable
    /// object with the arguments `extra` after the object.
    fn run_request(address: u32, kind: u32, reply: u32, extra: &[u32]) -> Sent {
        let mut e = engine();
        let object = visitable_object(&mut e);
        registered(&mut e, 2);
        let sink = record_requests(&mut e, kind, reply);
        let mut words = vec![object];
        words.extend_from_slice(extra);
        let result = e.call(address, &words);
        let mut seen = [0; 7];
        for (i, word) in seen.iter_mut().enumerate() {
            *word = e.mem.u32(sink + i as u32 * 4);
        }
        Sent {
            result,
            words: seen,
            count: e.mem.u32(sink + 0x24),
            found: e.mem.u32(sink + 0x20),
        }
    }

    /// The request function sends nothing for an unregistered object, an
    /// object without a record and a null object, but does once the skip flag
    /// (`extra[skip_at]`) is set.
    fn check_rejection(address: u32, kind: u32, extra: &[u32], skip_at: usize) {
        let mut e = engine();
        let object = visitable_object(&mut e);
        let sink = record_requests(&mut e, kind, 0);
        let mut words = vec![object];
        words.extend_from_slice(extra);
        words[1 + skip_at] = 0;
        registered(&mut e, 0);
        assert_eq!(e.call(address, &words).u8(), 0);
        e.register_double(OBJECT_RECORD, |_, _| Ret::default());
        assert_eq!(e.call(address, &words).u8(), 0);
        let mut null = words.clone();
        null[0] = 0;
        null[1 + skip_at] = 1;
        assert_eq!(e.call(address, &null).u8(), 0);
        assert_eq!(e.mem.u32(sink + 0x24), 0);
        words[1 + skip_at] = 1;
        e.call(address, &words);
        assert_eq!(e.mem.u32(sink + 0x24), 1);
    }

    fn flag_byte(sent: &Sent) -> u32 {
        sent.words[1] & 0xff
    }

    #[test]
    fn update_position_sends_the_kind_2_request() {
        let sent = run_request(0x00c6_9f50, 2, 0, &[1, 0]);
        assert!(sent.result.bool());
        assert_eq!((sent.words[0], flag_byte(&sent), sent.words[2]), (0, 1, 2));
        assert_eq!((sent.count, sent.found), (1, FOUND));
        check_rejection(0x00c6_9f50, 2, &[1, 0], 1);
    }

    #[test]
    fn remove_objects_sends_the_kind_1_request_and_returns_the_skip_byte() {
        let sent = run_request(0x00c6_9ee0, 1, 0, &[1, 0]);
        // Passing the check (skip clear) returns the skip byte itself: zero.
        assert_eq!(sent.result.u8(), 0);
        assert_eq!((flag_byte(&sent), sent.words[2], sent.count), (1, 1, 1));
        let forced = run_request(0x00c6_9ee0, 1, 0, &[0, 1]);
        assert_eq!(forced.result.u8(), 1);
        assert_eq!((flag_byte(&forced), forced.count), (0, 1));
        check_rejection(0x00c6_9ee0, 1, &[1, 0], 1);
    }

    #[test]
    fn adjust_force_add_count_sends_kind_13_with_a_truth_value() {
        let sent = run_request(0x00c6_9fc0, 0xd, 0, &[5, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (1, 0xd, 1)
        );
        let zero = run_request(0x00c6_9fc0, 0xd, 0, &[0, 0, 0]);
        assert_eq!((flag_byte(&zero), zero.words[3]), (0, 0));
        check_rejection(0x00c6_9fc0, 0xd, &[5, 1, 0], 2);
    }

    #[test]
    fn unnamed_kind_3_request_carries_a_byte_value() {
        let sent = run_request(0x00c6_a040, 3, 0, &[0x1ff, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (
                flag_byte(&sent),
                sent.words[2],
                sent.words[3],
                sent.words[4]
            ),
            (1, 3, 0xff, 0)
        );
        check_rejection(0x00c6_a040, 3, &[7, 1, 0], 2);
    }

    #[test]
    fn unnamed_kind_14_request_carries_a_byte_value() {
        let sent = run_request(0x00c6_a0b0, 0xe, 0, &[0x2a, 0, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (0, 0xe, 0x2a)
        );
        check_rejection(0x00c6_a0b0, 0xe, &[7, 1, 0], 2);
    }

    #[test]
    fn unnamed_kind_15_request_carries_a_float() {
        let sent = run_request(0x00c6_a120, 0xf, 0, &[1.5f32.to_bits(), 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (1, 0xf, 1.5f32.to_bits())
        );
        check_rejection(0x00c6_a120, 0xf, &[1.5f32.to_bits(), 1, 0], 2);
    }

    #[test]
    fn unnamed_kind_16_request_carries_a_float() {
        let sent = run_request(0x00c6_a190, 0x10, 0, &[(-2.25f32).to_bits(), 0, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (0, 0x10, (-2.25f32).to_bits())
        );
        check_rejection(0x00c6_a190, 0x10, &[1.0f32.to_bits(), 1, 0], 2);
    }

    #[test]
    fn set_disabled_sends_kind_17_with_a_byte() {
        let sent = run_request(0x00c6_a200, 0x11, 0, &[1, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (1, 0x11, 1)
        );
        check_rejection(0x00c6_a200, 0x11, &[1, 1, 0], 2);
    }

    #[test]
    fn activate_sends_kind_12_with_a_byte() {
        let sent = run_request(0x00c6_a270, 0xc, 0, &[0x101, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (1, 0xc, 1)
        );
        check_rejection(0x00c6_a270, 0xc, &[1, 1, 0], 2);
    }

    #[test]
    fn kill_havok_sends_the_kind_4_request() {
        let sent = run_request(0x00c6_a2e0, 4, 0, &[1, 0]);
        assert!(sent.result.bool());
        assert_eq!((flag_byte(&sent), sent.words[2]), (1, 4));
        check_rejection(0x00c6_a2e0, 4, &[1, 0], 1);
    }

    #[test]
    fn set_motion_sends_kind_5_with_a_word_and_a_byte() {
        let sent = run_request(0x00c6_a350, 5, 0, &[0x1234_5678, 1, 0, 0x1ff]);
        assert!(sent.result.bool());
        assert_eq!(
            (
                flag_byte(&sent),
                sent.words[2],
                sent.words[3],
                sent.words[4]
            ),
            (1, 5, 0x1234_5678, 0xff)
        );
        check_rejection(0x00c6_a350, 5, &[3, 1, 0, 2], 2);
    }

    #[test]
    fn unnamed_kind_7_request_carries_a_word() {
        let sent = run_request(0x00c6_a3d0, 7, 0, &[0xdead_beef, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (flag_byte(&sent), sent.words[2], sent.words[3]),
            (1, 7, 0xdead_beef)
        );
        check_rejection(0x00c6_a3d0, 7, &[3, 1, 0], 2);
    }

    #[test]
    fn unnamed_kind_8_request_answers_with_the_callbacks_reply() {
        let sent = run_request(0x00c6_a440, 8, 9, &[1, 0]);
        assert!(sent.result.bool());
        assert_eq!((flag_byte(&sent), sent.words[2], sent.words[3]), (1, 8, 0));
        // A callback that leaves the word clear makes the answer false.
        let silent = run_request(0x00c6_a440, 8, 0, &[1, 0]);
        assert!(!silent.result.bool());
        assert_eq!(silent.count, 1);
        check_rejection(0x00c6_a440, 8, &[1, 0], 1);
    }

    #[test]
    fn unnamed_kind_9_request_carries_two_words() {
        let sent = run_request(0x00c6_a4c0, 9, 0, &[0x11, 0x22, 1, 0]);
        assert!(sent.result.bool());
        assert_eq!(
            (
                flag_byte(&sent),
                sent.words[2],
                sent.words[3],
                sent.words[4]
            ),
            (1, 9, 0x22, 0x11)
        );
        check_rejection(0x00c6_a4c0, 9, &[0x11, 0x22, 1, 0], 3);
    }

    #[test]
    fn add_objects_sends_the_prelude_request_and_then_kind_0() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        let object = visitable_object(&mut e);
        registered(&mut e, 2);
        let prelude = record_requests(&mut e, 0xa, 0);
        let main = record_requests(&mut e, 0, 0);
        // The prelude byte is clear: only the kind 0 request goes out.
        let call_args = |flag_b: u32, skip: u32| args![this, object, 1u32, flag_b, 0x77u32, skip];
        assert!(e.call(0x00c6_9e10, &call_args(1, 0)).bool());
        assert!(e.get(this, BhkWorld::bAdded));
        assert_eq!(e.mem.u32(prelude + 0x24), 0);
        assert_eq!(e.mem.u32(main + 0x24), 1);
        assert_eq!(e.mem.u32(main), this.addr());
        assert_eq!(e.mem.u32(main + 4) & 0xff, 1);
        assert_eq!(e.mem.u32(main + 8), 0);
        assert_eq!(e.mem.u32(main + 0xc), 1);
        assert_eq!(e.mem.u32(main + 0x10), 0x77);
        assert_eq!(e.mem.u32(main + 0x14), 0);
        assert_eq!(e.mem.u32(main + 0x18), 1);
        assert_eq!(e.mem.u32(main + 0x20), FOUND);
        // The second flag clears the attach word.
        e.call(0x00c6_9e10, &call_args(0, 0));
        assert_eq!(e.mem.u32(main + 0xc), 0);
        // With the prelude byte set the kind 0xA request goes first.
        e.set_global(GLOBAL_ADD_PRELUDE_FLAG, 1u8);
        e.call(0x00c6_9e10, &call_args(1, 0));
        assert_eq!(e.mem.u32(prelude + 0x24), 1);
        assert_eq!(e.mem.u32(prelude + 8), 0xa);
        assert_eq!(e.mem.u32(prelude), this.addr());
        // The object's flags: the kind 0 visit sets bits 1 and 2.
        assert_eq!(e.mem.u32(object + 0x30) & 6, 6);
    }

    #[test]
    fn add_objects_rejects_null_and_unregistered_objects_unless_skipped() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        let object = visitable_object(&mut e);
        registered(&mut e, 0);
        let main = record_requests(&mut e, 0, 0);
        assert!(!e
            .call(0x00c6_9e10, &args![this, 0u32, 1u32, 1u32, 0u32, 1u32])
            .bool());
        assert!(!e
            .call(0x00c6_9e10, &args![this, object, 1u32, 1u32, 0u32, 0u32])
            .bool());
        assert!(!e.get(this, BhkWorld::bAdded));
        assert_eq!(e.mem.u32(main + 0x24), 0);
        assert!(e
            .call(0x00c6_9e10, &args![this, object, 1u32, 1u32, 0u32, 1u32])
            .bool());
        assert!(e.get(this, BhkWorld::bAdded));
        assert_eq!(e.mem.u32(main + 0x24), 1);
    }

    #[test]
    fn update_actions_updates_live_actions_and_drops_dead_ones() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        interlocked_doubles(&mut e);
        quiet(&mut e, &[HEAP_FREE]);
        e.register_double(0x0400_d0f0, |_, _| ret(0x0500_0000));
        e.register_double(0x0400_d0f4, |_, _| Ret::default());
        e.register_double(0x0400_d0f8, |_, _| ret(0));
        let live = e.mem.alloc(0x20);
        let dead = e.mem.alloc(0x20);
        // Slot 0x94 (the world), slot 0xC4 (the update) and slot 4 (delete).
        let delete = e.mem.alloc(8);
        e.mem.set_u32(delete, 0);
        vtable_at(
            &mut e,
            0x0400_d000,
            &[(0x94, 0x0400_d0f0), (0xc4, 0x0400_d0f4), (4, 0x0400_d0fc)],
        );
        vtable_at(
            &mut e,
            0x0400_d400,
            &[(0x94, 0x0400_d0f8), (0xc4, 0x0400_d0f4), (4, 0x0400_d0fc)],
        );
        e.register_double(0x0400_d0fc, move |e, a| {
            let n = e.mem.u32(delete);
            e.mem.set_u32(delete, n + 1);
            assert_eq!(a[0], dead);
            Ret::default()
        });
        e.mem.set_u32(live, 0x0400_d000);
        e.mem.set_u32(live + 4, 1);
        e.mem.set_u32(dead, 0x0400_d400);
        e.mem.set_u32(dead + 4, 1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, dead);
        e.mem.set_u32(this.addr() + 0x90, live);
        e.mem.set_u32(this.addr() + 0x94, second);
        log(&mut e);
        e.call(0x00c6_a540, &args![this, 0.25f32]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, 0x0400_d0f4), [[live, 0.25f32.to_bits()]]);
        // The dead action left the list and its last reference was dropped.
        assert_eq!(e.mem.u32(this.addr() + 0x90), live);
        assert_eq!(e.mem.u32(this.addr() + 0x94), 0);
        assert_eq!(e.mem.u32(delete), 1);
        assert_eq!(e.mem.u32(dead + 4), 0);
        // A dead head item is removed and the walk ends on the empty head.
        let head_dead = e.mem.alloc(0x20);
        e.mem.set_u32(head_dead, 0x0400_d400);
        e.mem.set_u32(head_dead + 4, 1);
        e.mem.set_u32(this.addr() + 0x90, head_dead);
        e.mem.set_u32(this.addr() + 0x94, 0);
        e.register_double(0x0400_d0fc, |_, _| Ret::default());
        e.call(0x00c6_a540, &args![this, 1.0f32]);
        assert_eq!(e.mem.u32(this.addr() + 0x90), 0);
        assert_eq!(e.mem.u32(head_dead + 4), 0);
    }

    #[test]
    fn fn_00c6a620_notifies_and_stores_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.register_double(REFERENCE_OWNER_NOTIFY, move |_, a| {
            assert_eq!(a, [this, 0x1234]);
            Ret::default()
        });
        log(&mut e);
        e.call(0x00c6_a620, &args![this, 0x1234u32]);
        assert_eq!(
            addresses(&calls(&mut e)),
            [0x00c6_a620, REFERENCE_OWNER_NOTIFY]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0x1234);
    }

    #[test]
    fn set_debug_display_stores_the_flag_and_notifies_an_object() {
        let mut e = engine();
        let this: Ptr<BhkWorld> = e.new_object();
        let object = visitable_object(&mut e);
        let sink = record_requests(&mut e, 6, 0);
        e.call(0x00c6_a810, &args![this, 0u32, 1u32]);
        assert!(e.get(this, BhkWorld::bDebugDisplay));
        assert_eq!(e.mem.u32(sink + 0x24), 0);
        e.call(0x00c6_a810, &args![this, object, 0x100u32]);
        // The byte is stored as given (low byte zero), the request says so.
        assert!(!e.get(this, BhkWorld::bDebugDisplay));
        assert_eq!(e.mem.u32(sink + 0x24), 1);
        assert_eq!(e.mem.u32(sink), 0);
        assert_eq!(e.mem.u32(sink + 4) & 0xff, 1);
        assert_eq!(e.mem.u32(sink + 8), 6);
        assert_eq!(e.mem.u32(sink + 0xc), 0);
        e.call(0x00c6_a810, &args![this, object, 1u32]);
        assert_eq!(e.mem.u32(sink + 0xc), 1);
        assert_eq!(e.mem.u32(sink + 0x24), 2);
    }

    /// An allocator object whose slot 4 allocates `block` bytes, slot 0xC
    /// allocates the requested size and slot 0x10 frees (recording
    /// `(address, size)` pairs into the returned log: count at `+0`).
    fn allocator_with_log(e: &mut Engine) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let record = e.mem.alloc(0x40);
        vtable_at(
            e,
            0x0400_e000,
            &[(4, 0x0400_e0f0), (0xc, 0x0400_e0f4), (0x10, 0x0400_e0f8)],
        );
        e.mem.set_u32(allocator, 0x0400_e000);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(0x0400_e0f0, |e, a| ret(e.mem.alloc(a[1].max(0x100))));
        e.register_double(0x0400_e0f4, |e, a| {
            ret(e.mem.alloc(e.mem.u32(a[1]).max(16)))
        });
        e.register_double(0x0400_e0f8, move |e, a| {
            let count = e.mem.u32(record);
            e.mem.set_u32(record + 4 + count * 8, a[1]);
            e.mem.set_u32(record + 8 + count * 8, a[2]);
            e.mem.set_u32(record, count + 1);
            Ret::default()
        });
        (allocator, record)
    }

    #[test]
    fn set_visual_debugger_creates_and_releases_the_debugger() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let scope = scope_marker(&mut e);
        let (_, record) = allocator_with_log(&mut e);
        quiet(
            &mut e,
            &[
                PHYSICS_REGISTER_PROCESSES,
                VISUAL_DEBUGGER_SERVE,
                HAVOK_RELEASE,
            ],
        );
        e.register_double(PHYSICS_CONTEXT_CREATE, |_, a| ret(a[0]));
        // The context is remembered so the create double can compare it.
        e.register_double(PHYSICS_CONTEXT_ADD_WORLD, move |e, a| {
            assert_eq!(a[1], world);
            assert_eq!(e.mem.u16(a[0] + 4), 0x90);
            e.mem.set_u32(0x0126_9000, a[0]);
            Ret::default()
        });
        e.register_double(ARRAY_GROW, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 4), 0);
            assert_eq!(a[2], 4);
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.register_double(VISUAL_DEBUGGER_CREATE, |e, a| {
            // One context was queued: the entry is `context + 8`.
            assert_eq!(e.mem.u32(a[1] + 4), 1);
            let entry = e.mem.u32(e.mem.u32(a[1]));
            assert_eq!(entry, e.mem.u32(0x0126_9000) + 8);
            assert_eq!(a[2], 0);
            ret(a[0])
        });
        log(&mut e);
        e.call(0x00c6_a640, &args![this, 1u32]);
        let seen = calls(&mut e);
        let debugger = e.get(this, BhkWorld::pDebugger);
        assert!(!debugger.is_null());
        assert_eq!(e.mem.u16(debugger.addr() + 4), 0x90);
        assert_eq!(
            calls_to(&seen, VISUAL_DEBUGGER_SERVE),
            [[debugger.addr(), 0x61a9]]
        );
        // The temporary array of contexts was freed with its 4-element capacity.
        assert_eq!(e.mem.u32(record), 1);
        assert_eq!(e.mem.u32(record + 8), 16);
        assert_scope_restored(&mut e, scope);
        // Already created: nothing happens.
        log(&mut e);
        e.call(0x00c6_a640, &args![this, 1u32]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_a640]);
        // Disabling releases it through the world getter first.
        log(&mut e);
        e.call(0x00c6_a640, &args![this, 0u32]);
        let seen = calls(&mut e);
        assert_eq!(addresses(&seen), [0x00c6_a640, GETTER, HAVOK_RELEASE]);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[debugger.addr()]]);
        assert!(e.get(this, BhkWorld::pDebugger).is_null());
        // Disabling without one does nothing.
        log(&mut e);
        e.call(0x00c6_a640, &args![this, 0u32]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_a640]);
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn deactivate_all_islands_marks_only_the_still_islands() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        let this = world_with(&mut e, world);
        let (_, record) = allocator_with_log(&mut e);
        e.set_global(GLOBAL_TOLERANCE, 0.001f32);
        for lane in 0..4 {
            e.mem.set_u32(GLOBAL_ABSOLUTE_MASK + lane * 4, 0x7fff_ffff);
            e.mem.set_u32(GLOBAL_ZERO_VECTOR + lane * 4, 0);
        }
        let still_body = e.mem.alloc(0x200);
        let moving_body = e.mem.alloc(0x200);
        let spinning_body = e.mem.alloc(0x200);
        e.mem.set_f32(moving_body + 0x1b0, 1.0);
        e.mem.set_f32(spinning_body + 0x1c0 + 8, -2.0);
        let island = |e: &mut Engine, bodies: &[u32]| {
            let island = e.mem.alloc(0x60);
            let array = pointer_array(e, bodies);
            e.mem.set_u32(island + 0x48, array);
            e.mem.set_u32(island + 0x4c, bodies.len() as u32);
            island
        };
        let quiet_island = island(&mut e, &[still_body, still_body]);
        let moving_island = island(&mut e, &[still_body, moving_body]);
        let spinning_island = island(&mut e, &[spinning_body]);
        let empty_island = island(&mut e, &[]);
        let islands = pointer_array(
            &mut e,
            &[quiet_island, moving_island, spinning_island, empty_island],
        );
        e.mem.set_u32(world + 0x28, islands);
        e.mem.set_u32(world + 0x2c, 4);
        quiet(&mut e, &[CLEANUP_DIRTY_ISLANDS, MARK_ISLAND_INACTIVE]);
        log(&mut e);
        e.call(0x00c6_a870, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(
            calls_to(&seen, MARK_ISLAND_INACTIVE),
            [[world, quiet_island], [world, empty_island]]
        );
        assert_eq!(calls_to(&seen, CLEANUP_DIRTY_ISLANDS), [[world], [world]]);
        // The temporary copy (4 pointers) was freed through slot 0x10.
        assert_eq!(e.mem.u32(record), 1);
        assert_eq!(e.mem.u32(record + 8), 16);
        // Without an hkpWorld nothing happens.
        let bare = world_with(&mut e, 0);
        log(&mut e);
        e.call(0x00c6_a870, &args![bare]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_a870, GETTER]);
    }

    #[test]
    fn deactivate_all_islands_with_no_islands_allocates_nothing() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        let this = world_with(&mut e, world);
        let (_, record) = allocator_with_log(&mut e);
        quiet(&mut e, &[CLEANUP_DIRTY_ISLANDS, MARK_ISLAND_INACTIVE]);
        e.call(0x00c6_a870, &args![this]);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn action_batch_flush_adds_actions_whose_entities_are_in_this_world() {
        let mut e = engine();
        let world = 0x0500_0000;
        let this = world_with(&mut e, world);
        let scope = scope_marker(&mut e);
        let (_, record) = allocator_with_log(&mut e);
        memset_double(&mut e);
        quiet(&mut e, &[WORLD_ADD_ACTION, HAVOK_RELEASE]);
        // Slot 0x10 lists the entities of an action: `entities[n]` per action.
        let entity_here = e.mem.alloc(0x20);
        let entity_elsewhere = e.mem.alloc(0x20);
        e.mem.set_u32(entity_here + 8, world);
        e.mem.set_u32(entity_elsewhere + 8, 0x0600_0000);
        let list_a = pointer_array(&mut e, &[entity_here]);
        let list_b = pointer_array(&mut e, &[entity_here, entity_elsewhere]);
        vtable_at(&mut e, 0x0400_f000, &[(0x10, 0x0400_f0f0)]);
        e.register_double(0x0400_f0f0, move |e, a| {
            let list = e.mem.u32(a[0] + 0x60);
            let (data, size) = match list {
                1 => (list_a, 1),
                2 => (list_b, 2),
                _ => (0, 0),
            };
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, size);
            // Not owned: the executable's own empty buffer marks capacity this way.
            e.mem.set_u32(a[1] + 8, 0x8000_0000);
            Ret::default()
        });
        let make_action = |e: &mut Engine, list: u32, owner_world: u32| {
            let action = e.mem.alloc(0x80);
            e.mem.set_u32(action, 0x0400_f000);
            e.mem.set_u32(action + 8, owner_world);
            e.mem.set_u32(action + 0x60, list);
            action
        };
        let plain = make_action(&mut e, 0, 0);
        let here = make_action(&mut e, 1, 0);
        let mixed = make_action(&mut e, 2, 0);
        let owned = make_action(&mut e, 0, 0x0700_0000);
        let batch = pointer_array(&mut e, &[plain, here, mixed, owned, 0]);
        e.set(this, BhkWorld::pActionBatch, Ptr::new(batch));
        e.set(this, BhkWorld::iActionBatchCount, 5);
        log(&mut e);
        e.call(0x00c6_aac0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(
            calls_to(&seen, WORLD_ADD_ACTION),
            [[world, plain], [world, here]]
        );
        assert_eq!(
            calls_to(&seen, HAVOK_RELEASE),
            [[plain], [here], [mixed], [owned]]
        );
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 0);
        assert_eq!(e.mem.u32(batch), 0);
        assert_eq!(e.mem.u32(record), 0);
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn action_batch_flush_clamps_and_skips_without_a_world() {
        let mut e = engine();
        let scope = scope_marker(&mut e);
        let this = world_with(&mut e, 0);
        memset_double(&mut e);
        // Nothing queued.
        log(&mut e);
        e.call(0x00c6_aac0, &args![this]);
        assert_eq!(addresses(&calls(&mut e)), [0x00c6_aac0]);
        // No world: the count is clamped to 100 and the batch is left alone.
        e.set(this, BhkWorld::iActionBatchCount, 150);
        e.call(0x00c6_aac0, &args![this]);
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 100);
        assert_scope_restored(&mut e, scope);
    }

    /// A world laid out for the destructor tests, with every callee doubled.
    fn dying_world(e: &mut Engine) -> (Ptr<BhkWorld>, u32) {
        // The block starts 16 bytes before `this` so the byte before it can be written.
        world_with(e, 0);
        let this: Ptr<BhkWorld> = Ptr::new(e.mem.alloc(0xb0) + 0x10);
        e.mem.set_u32(this.addr(), 0x0400_0000);
        // The destructor installs its own vtable: it must still answer the world getter.
        vtable_at(e, WORLD_DESTRUCTOR_VTABLE, &[(0x94, GETTER)]);
        let (_, record) = allocator_with_log(e);
        quiet(
            e,
            &[
                HEAP_FREE,
                SERIALIZABLE_KILL,
                SERIALIZABLE_DESTRUCTOR,
                HAVOK_RELEASE,
            ],
        );
        interlocked_doubles(e);
        for (i, field) in [0x2cu32, 0x34, 0x3c, 0x44, 0x4c].into_iter().enumerate() {
            e.mem.set_u32(this.addr() + field, 0x0800_0000 + i as u32);
        }
        (this, record)
    }

    #[test]
    fn destructor_body_tears_the_world_down_in_order() {
        let mut e = engine();
        let (this, record) = dying_world(&mut e);
        let scope = scope_marker(&mut e);
        e.set_global(GLOBAL_WORLD_COUNT, 5u32);
        let debugger = e.mem.alloc(0x10);
        e.set(this, BhkWorld::pDebugger, Ptr::new(debugger));
        // Two listeners (one null): slot 0 of the first is called with 1.
        vtable_at(&mut e, 0x0400_9400, &[(0, 0x0400_94f0)]);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener, 0x0400_9400);
        e.register_double(0x0400_94f0, move |_, a| {
            assert_eq!(a, [listener, 1]);
            Ret::default()
        });
        let listeners = pointer_array(&mut e, &[listener, 0]);
        e.set(this, BhkWorld::entityListeners, Ptr::new(listeners));
        e.set(this, BhkWorld::entityListenerCount, 2);
        e.mem.set_u32(this.addr() + 0x78, 4);
        // Owned debug color buffer of capacity 3 (24 bytes).
        let colors = e.mem.alloc(0x20);
        e.set(this, BhkWorld::debugColorData, Ptr::new(colors));
        e.set(this, BhkWorld::debugColorCount, 2);
        e.mem.set_u32(this.addr() + 0x84, 3);
        log(&mut e);
        e.call(0x00c6_aca0, &args![this]);
        let seen = calls(&mut e);
        assert_eq!(e.mem.u32(this.addr()), WORLD_DESTRUCTOR_VTABLE);
        assert_eq!(e.global::<u32>(GLOBAL_WORLD_COUNT), 4);
        assert_eq!(calls_to(&seen, HAVOK_RELEASE), [[debugger]]);
        assert!(e.get(this, BhkWorld::pDebugger).is_null());
        assert_eq!(calls_to(&seen, SERIALIZABLE_KILL), [[this.addr()]]);
        assert_eq!(calls_to(&seen, SERIALIZABLE_DESTRUCTOR), [[this.addr()]]);
        let frees: Vec<u32> = calls_to(&seen, HEAP_FREE).iter().map(|a| a[1]).collect();
        assert_eq!(
            frees,
            [
                0x0800_0000,
                0x0800_0001,
                0x0800_0002,
                0x0800_0003,
                0x0800_0004
            ]
        );
        assert_eq!(e.get(this, BhkWorld::entityListenerCount), 0);
        assert!(e.get(this, BhkWorld::entityListeners).is_null());
        assert_eq!(e.mem.u32(this.addr() + 0x78), 0x8000_0000);
        assert!(e.get(this, BhkWorld::debugColorData).is_null());
        assert_eq!(e.get(this, BhkWorld::debugColorCount), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x84), 0x8000_0000);
        // Both owned buffers went back to the allocator: colors 3 * 8, listeners 4 * 4.
        assert_eq!(e.mem.u32(record), 2);
        assert_eq!((e.mem.u32(record + 4), e.mem.u32(record + 8)), (colors, 24));
        assert_eq!(
            (e.mem.u32(record + 12), e.mem.u32(record + 16)),
            (listeners, 16)
        );
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn destructor_body_does_not_free_borrowed_buffers() {
        let mut e = engine();
        let (this, record) = dying_world(&mut e);
        e.mem.set_u32(this.addr() + 0x78, 0x8000_0000);
        e.mem.set_u32(this.addr() + 0x84, 0x8000_0000);
        log(&mut e);
        e.call(0x00c6_aca0, &args![this]);
        let seen = calls(&mut e);
        assert!(calls_to(&seen, HAVOK_RELEASE).is_empty());
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let (this, _) = dying_world(&mut e);
        // The block starts 5 bytes before `this`: the byte before it says so.
        e.mem.set_u8(this.addr() - 1, 5);
        log(&mut e);
        assert_eq!(e.call(0x00c6_b070, &args![this, 0u32]).u32(), this.addr());
        let seen = calls(&mut e);
        assert!(!calls_to(&seen, SERIALIZABLE_DESTRUCTOR).is_empty());
        assert_eq!(calls_to(&seen, HEAP_FREE).len(), 5);
        log(&mut e);
        e.call(0x00c6_b070, &args![this, 1u32]);
        let seen = calls(&mut e);
        let frees = calls_to(&seen, HEAP_FREE);
        assert_eq!(frees.len(), 6);
        assert_eq!(frees[5], [HEAP_SINGLETON, this.addr() - 5]);
    }

    /// Doubles for everything `Update` calls on the Havok side.
    fn stepping_world(e: &mut Engine, mode: u32) -> (Ptr<BhkWorld>, u32) {
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0xf0, mode);
        let this = world_with(e, world);
        e.set(this, BhkWorld::bEnabled, true);
        e.set_global(GLOBAL_STEP_LENGTH, 0.5f32);
        e.set_global(GLOBAL_STEP_TIME, 0.25f32);
        e.set_global(GLOBAL_STEP_SCALE, 60.0f32);
        e.set_global(GLOBAL_STEP_FACTOR, 0.5f32);
        quiet(
            e,
            &[
                WORLD_BEFORE_STEP_A,
                WORLD_BEFORE_STEP_B,
                WORLD_GET_CURRENT_TIME,
                WORLD_SET_CURRENT_TIME,
                WORLD_PREPARE_STEP,
                WORLD_GET_CURRENT_PSI_TIME,
                WORLD_STEP_DELTA_TIME,
                WORLD_STEP_MULTITHREADED,
                ACQUIRE_MT_RESOURCES,
                0x00ec_62c0,
            ],
        );
        // Two steps, then the step-done query says so.
        let steps = e.mem.alloc(8);
        e.register_double(WORLD_STEP_DONE, move |e, _| {
            let n = e.mem.u32(steps);
            e.mem.set_u32(steps, n + 1);
            ret((n >= 2) as u32)
        });
        (this, world)
    }

    #[test]
    fn update_steps_a_single_threaded_world() {
        let mut e = engine();
        let (this, world) = stepping_world(&mut e, 0);
        e.set_global(GLOBAL_WORLD_FLOAT, 9.0f32);
        e.set_global(GLOBAL_WORLD_INTEGER, 9u32);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 1);
        log(&mut e);
        assert!(e.call(0x00c6_ae70, &args![this]).bool());
        let seen = calls(&mut e);
        let of = [
            WORLD_BEFORE_STEP_A,
            WORLD_BEFORE_STEP_B,
            WORLD_GET_CURRENT_TIME,
            WORLD_SET_CURRENT_TIME,
            WORLD_PREPARE_STEP,
            WORLD_STEP_DONE,
            WORLD_GET_CURRENT_PSI_TIME,
            WORLD_STEP_DELTA_TIME,
            ACQUIRE_MT_RESOURCES,
        ];
        let order: Vec<u32> = addresses(&seen)
            .into_iter()
            .filter(|a| of.contains(a))
            .collect();
        assert_eq!(
            order,
            [
                WORLD_BEFORE_STEP_A,
                WORLD_BEFORE_STEP_B,
                WORLD_GET_CURRENT_TIME,
                WORLD_SET_CURRENT_TIME,
                WORLD_PREPARE_STEP,
                WORLD_STEP_DONE,
                WORLD_GET_CURRENT_PSI_TIME,
                WORLD_STEP_DELTA_TIME,
                WORLD_STEP_DONE,
                WORLD_GET_CURRENT_PSI_TIME,
                WORLD_STEP_DELTA_TIME,
                WORLD_STEP_DONE,
            ]
        );
        assert_eq!(
            calls_to(&seen, WORLD_SET_CURRENT_TIME),
            [[world, 0.25f32.to_bits()]]
        );
        // 60 * 0.5 computed in double precision and rounded to float.
        assert_eq!(e.global::<f32>(GLOBAL_STEP_DELTA), 30.0);
        assert_eq!(
            calls_to(&seen, WORLD_STEP_DELTA_TIME),
            [[world, 30.0f32.to_bits()], [world, 30.0f32.to_bits()]]
        );
        assert_eq!(e.global::<u8>(GLOBAL_UPDATING), 0);
        // The accumulators were cleared first (and the solver scaling ran).
        assert_eq!(e.global::<f32>(GLOBAL_WORLD_FLOAT), 0.0);
        assert_eq!(e.global::<u32>(GLOBAL_WORLD_INTEGER), 0);
    }

    #[test]
    fn update_steps_a_multithreaded_world_through_the_system() {
        let mut e = engine();
        let (this, world) = stepping_world(&mut e, 3);
        let system = e.mem.alloc(0x40);
        e.mem.set_u32(system + 0x14, 0x1111);
        e.mem.set_u32(system + 0x18, 0x2222);
        e.set_global(GLOBAL_MULTITHREADED_SYSTEM, system);
        e.set_global(GLOBAL_ACQUIRE_FLAG, 1u8);
        log(&mut e);
        assert!(e.call(0x00c6_ae70, &args![this]).bool());
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, ACQUIRE_MT_RESOURCES), [[system, 1]]);
        let delta = 30.0f32.to_bits();
        assert_eq!(
            calls_to(&seen, WORLD_STEP_MULTITHREADED),
            [
                [world, 0x1111, 0x2222, delta],
                [world, 0x1111, 0x2222, delta]
            ]
        );
        assert!(calls_to(&seen, WORLD_STEP_DELTA_TIME).is_empty());
        // The single-threaded hooks did not run: the world has none pending.
        assert!(calls_to(&seen, WORLD_BEFORE_STEP_A).is_empty());
    }

    #[test]
    fn update_skips_the_hooks_when_the_world_is_busy() {
        let mut e = engine();
        let (this, world) = stepping_world(&mut e, 0);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0x94, 1);
        log(&mut e);
        assert!(e.call(0x00c6_ae70, &args![this]).bool());
        let seen = calls(&mut e);
        assert!(calls_to(&seen, WORLD_BEFORE_STEP_A).is_empty());
        assert!(calls_to(&seen, WORLD_BEFORE_STEP_B).is_empty());
    }

    #[test]
    fn update_does_not_step_a_disabled_world_or_a_zero_length_step() {
        let mut e = engine();
        let (this, _) = stepping_world(&mut e, 0);
        e.set(this, BhkWorld::bEnabled, false);
        log(&mut e);
        assert!(!e.call(0x00c6_ae70, &args![this]).bool());
        assert!(calls_to(&calls(&mut e), WORLD_STEP_DONE).is_empty());
        e.set(this, BhkWorld::bEnabled, true);
        e.set_global(GLOBAL_STEP_LENGTH, 0.0f32);
        log(&mut e);
        assert!(!e.call(0x00c6_ae70, &args![this]).bool());
        assert!(calls_to(&calls(&mut e), WORLD_STEP_DONE).is_empty());
        assert_eq!(e.global::<u8>(GLOBAL_UPDATING), 0);
    }

    #[test]
    fn update_flushes_an_entity_tail_once_and_runs_a_scheduled_debugger() {
        let mut e = engine();
        let this = world_with(&mut e, 0);
        let array = e.mem.alloc(0x100);
        e.set(this, BhkWorld::pEntityBatch, Ptr::new(array));
        e.set(this, BhkWorld::iEntityBatchCount, 5);
        e.set_global(GLOBAL_ENTITY_FLUSH_LIMIT, 2u32);
        // An existing debugger keeps the scheduled creation a no-op.
        let debugger = e.mem.alloc(0x10);
        e.set(this, BhkWorld::pDebugger, Ptr::new(debugger));
        e.set_global(GLOBAL_SCHEDULED_DEBUGGER, 1u8);
        // A pending entity removal runs the partial removal.
        e.set(this, BhkWorld::iEntityRemoveBatchCount, 1);
        assert!(!e.call(0x00c6_ae70, &args![this]).bool());
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 3);
        assert_eq!(e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE), 1);
        assert_eq!(e.global::<u8>(GLOBAL_SCHEDULED_DEBUGGER), 0);
        // The partial flush is not repeated until the flag is cleared.
        assert!(!e.call(0x00c6_ae70, &args![this]).bool());
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 3);
        // A batch over 3000 is clamped first.
        e.set_global(GLOBAL_PARTIAL_FLUSH_DONE, 0u8);
        e.set(this, BhkWorld::iEntityBatchCount, 4000);
        e.set_global(GLOBAL_ENTITY_FLUSH_LIMIT, 100u32);
        e.call(0x00c6_ae70, &args![this]);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 2900);
    }

    /// A world with an `hkpWorld` for the add functions, every callee doubled.
    fn adding_world(e: &mut Engine, batching: bool) -> (Ptr<BhkWorld>, u32) {
        let world = 0x0500_0000;
        let this = world_with(e, world);
        if batching {
            e.set(this, BhkWorld::iBatchAdd, 1);
        }
        let entities = e.mem.alloc(0x3000 * 4);
        let phantoms = e.mem.alloc(0x400 * 4);
        let constraints = e.mem.alloc(0x400 * 4);
        let actions = e.mem.alloc(0x200 * 4);
        e.set(this, BhkWorld::pEntityBatch, Ptr::new(entities));
        e.set(this, BhkWorld::pPhantomBatch, Ptr::new(phantoms));
        e.set(
            this,
            BhkWorld::pConstraintInstanceBatch,
            Ptr::new(constraints),
        );
        e.set(this, BhkWorld::pActionBatch, Ptr::new(actions));
        quiet(
            e,
            &[
                HAVOK_ADD_REFERENCE,
                WORLD_ADD_ENTITY,
                WORLD_ADD_PHANTOM,
                WORLD_ADD_CONSTRAINT,
                WORLD_ADD_ACTION,
            ],
        );
        (this, world)
    }

    /// An entity (kind byte 1) whose owner flags are `flags`.
    fn entity_with_owner(e: &mut Engine, flags: u32) -> (u32, u32) {
        let entity = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(entity + 0x28, 1);
        e.mem.set_u32(entity + 0xc, owner);
        e.mem.set_u32(owner + 0x10, flags);
        (entity, owner)
    }

    /// A phantom (kind byte 2) whose owner sits at `phantom + 0x10` itself.
    fn phantom_with_owner(e: &mut Engine, flags: u8) -> (u32, u32) {
        let phantom = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(phantom + 0x28, 2);
        e.mem.set_i8(phantom + 0x20, 0);
        e.mem.set_u32(phantom + 0x10 + 0xc, owner);
        e.mem.set_u8(owner + 0x14, flags);
        (phantom, owner)
    }

    #[test]
    fn add_puts_an_entity_straight_into_the_world() {
        let mut e = engine();
        let (this, world) = adding_world(&mut e, false);
        let (entity, owner) = entity_with_owner(&mut e, 0);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, entity]);
        let seen = calls(&mut e);
        // The check byte is clear: the flag argument is 1.
        assert_eq!(calls_to(&seen, WORLD_ADD_ENTITY), [[world, entity, 1]]);
        // Owner flags with bit 4 or 5 stop it.
        e.mem.set_u32(owner + 0x10, 0x20);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, entity]);
        assert!(calls_to(&calls(&mut e), WORLD_ADD_ENTITY).is_empty());
        // So do a missing owner, an unknown kind, a null world object and no world.
        e.mem.set_u32(entity + 0xc, 0);
        e.call(0x00c6_b0a0, &args![this, entity]);
        e.mem.set_u8(entity + 0x28, 3);
        e.call(0x00c6_b0a0, &args![this, entity]);
        e.call(0x00c6_b0a0, &args![0u32, entity]);
        let bare = world_with(&mut e, 0);
        e.call(0x00c6_b0a0, &args![bare, entity]);
    }

    #[test]
    fn add_clears_the_entity_flag_only_for_still_unremarkable_owners() {
        let mut e = engine();
        let (this, world) = adding_world(&mut e, false);
        let (entity, owner) = entity_with_owner(&mut e, 0);
        e.set_global(GLOBAL_ADD_ENTITY_CHECK, 1u8);
        e.set_global(GLOBAL_TOLERANCE, 0.001f32);
        for lane in 0..4 {
            e.mem.set_u32(GLOBAL_ABSOLUTE_MASK + lane * 4, 0x7fff_ffff);
            e.mem.set_u32(GLOBAL_ZERO_VECTOR + lane * 4, 0);
        }
        let record = e.mem.alloc(8);
        let position = e.mem.alloc(0x10);
        let rotation = e.mem.alloc(0x10);
        e.register_double(ENTITY_OWNER_RECORD, move |e, a| {
            // The object travels in a stack slot; the record is that slot.
            assert_eq!(a[0], owner);
            assert_eq!(e.mem.u32(a[1]), entity);
            ret(record)
        });
        e.register_double(OWNER_POSITION_VECTOR, move |_, _| ret(position));
        e.register_double(OWNER_ROTATION_VECTOR, move |_, _| ret(rotation));
        let flag_after = |e: &mut Engine| {
            log(e);
            e.call(0x00c6_b0a0, &args![this, entity]);
            calls_to(&calls(e), WORLD_ADD_ENTITY)[0][2]
        };
        // Kind byte 8 (low seven bits): the flag stays 1.
        e.mem.set_u8(record, 0x88);
        assert_eq!(flag_after(&mut e), 1);
        // Other kind with zero vectors: the flag is cleared.
        e.mem.set_u8(record, 0x05);
        assert_eq!(flag_after(&mut e), 0);
        // A moving position or rotation keeps it.
        e.mem.set_f32(position + 4, 1.0);
        assert_eq!(flag_after(&mut e), 1);
        e.mem.set_f32(position + 4, 0.0);
        e.mem.set_f32(rotation + 8, 0.5);
        assert_eq!(flag_after(&mut e), 1);
        // Without the check byte the flag is always 1.
        e.mem.set_f32(rotation + 8, 0.0);
        e.set_global(GLOBAL_ADD_ENTITY_CHECK, 0u8);
        assert_eq!(flag_after(&mut e), 1);
        let _ = (world, owner);
    }

    #[test]
    fn add_queues_entities_when_batching_and_flushes_a_full_batch() {
        let mut e = engine();
        let (this, _) = adding_world(&mut e, true);
        let scope = scope_marker(&mut e);
        let (entity, owner) = entity_with_owner(&mut e, 0x01);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, entity]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, HAVOK_ADD_REFERENCE), [[entity]]);
        assert!(calls_to(&seen, WORLD_ADD_ENTITY).is_empty());
        assert_eq!(e.mem.u32(owner + 0x10), 0x11);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 1);
        let array = e.get(this, BhkWorld::pEntityBatch).addr();
        assert_eq!(e.mem.u32(array), entity);
        once_world(&mut e, this, 0x0500_0000);
        // A full batch (3000) flushes its tail (the world getter has no
        // hkpWorld for the flushers here) and then the other batches, then
        // queues the entity.
        e.mem.set_u32(owner + 0x10, 0x01);
        e.set(this, BhkWorld::iEntityBatchCount, 3000);
        e.set_global(GLOBAL_ENTITY_FLUSH_LIMIT, 1000u32);
        e.call(0x00c6_b0a0, &args![this, entity]);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 2001);
        assert_eq!(e.mem.u32(array + 2000 * 4), entity);
        assert_eq!(e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE), 1);
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn add_handles_phantoms_directly_and_in_batches() {
        let mut e = engine();
        let (this, world) = adding_world(&mut e, false);
        let (phantom, owner) = phantom_with_owner(&mut e, 0);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        assert_eq!(
            calls_to(&calls(&mut e), WORLD_ADD_PHANTOM),
            [[world, phantom]]
        );
        // Blocked by owner flags, a missing owner or the wrong kind.
        e.mem.set_u8(owner + 0x14, 0x10);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        assert!(calls_to(&calls(&mut e), WORLD_ADD_PHANTOM).is_empty());
        e.mem.set_u8(owner + 0x14, 0);
        e.mem.set_u8(phantom + 0x28, 3);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        assert!(calls_to(&calls(&mut e), WORLD_ADD_PHANTOM).is_empty());
        e.mem.set_u8(phantom + 0x28, 2);
        // Batching: the owner is marked and the phantom queued.
        e.set(this, BhkWorld::iBatchAdd, 1);
        log(&mut e);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, HAVOK_ADD_REFERENCE), [[phantom]]);
        assert_eq!(e.mem.u8(owner + 0x14), 0x10);
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 1);
        let array = e.get(this, BhkWorld::pPhantomBatch).addr();
        assert_eq!(e.mem.u32(array), phantom);
    }

    #[test]
    fn add_flushes_before_queueing_a_phantom_into_a_full_batch() {
        let mut e = engine();
        let (this, _) = adding_world(&mut e, true);
        let (phantom, _) = phantom_with_owner(&mut e, 0);
        once_world(&mut e, this, 0x0500_0000);
        e.set(this, BhkWorld::iPhantomBatchCount, 200);
        // Entities are pending and no partial flush has run: it runs first.
        e.set(this, BhkWorld::iEntityBatchCount, 7);
        e.set_global(GLOBAL_ENTITY_FLUSH_LIMIT, 3u32);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 4);
        assert_eq!(e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE), 1);
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 201);
        // With the partial flush already done it is not repeated.
        let owner_word = e.mem.u32(phantom + 0x1c);
        e.mem.set_u8(owner_word + 0x14, 0);
        e.mem.set_u32(this.addr() + 0x9c, 0);
        e.call(0x00c6_b0a0, &args![this, phantom]);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 4);
        // The phantom flush clamped the full batch to 200 before queueing again.
        assert_eq!(e.get(this, BhkWorld::iPhantomBatchCount), 201);
    }

    #[test]
    fn add_ov2_adds_a_constraint_whose_entities_have_islands() {
        let mut e = engine();
        let (this, world) = adding_world(&mut e, false);
        let constraint = e.mem.alloc(0x40);
        let first = e.mem.alloc(0xd0);
        let second = e.mem.alloc(0xd0);
        e.mem.set_u32(constraint + 0x14, first);
        e.mem.set_u32(constraint + 0x18, second);
        e.mem.set_u32(first + 0xcc, 1);
        e.mem.set_u32(second + 0xcc, 1);
        log(&mut e);
        e.call(0x00c6_b320, &args![this, constraint]);
        assert_eq!(
            calls_to(&calls(&mut e), WORLD_ADD_CONSTRAINT),
            [[world, constraint]]
        );
        // An entity without an island, an existing world, a null constraint stop it.
        e.mem.set_u32(second + 0xcc, 0);
        log(&mut e);
        e.call(0x00c6_b320, &args![this, constraint]);
        e.mem.set_u32(constraint + 0x18, 0);
        e.call(0x00c6_b320, &args![this, constraint]);
        assert_eq!(calls_to(&calls(&mut e), WORLD_ADD_CONSTRAINT).len(), 1);
        e.mem.set_u32(constraint + 8, 5);
        log(&mut e);
        e.call(0x00c6_b320, &args![this, constraint]);
        e.call(0x00c6_b320, &args![this, 0u32]);
        e.call(0x00c6_b320, &args![0u32, constraint]);
        assert!(calls_to(&calls(&mut e), WORLD_ADD_CONSTRAINT).is_empty());
    }

    #[test]
    fn add_ov2_queues_constraints_when_batching() {
        let mut e = engine();
        let (this, _) = adding_world(&mut e, true);
        let constraint = e.mem.alloc(0x40);
        log(&mut e);
        e.call(0x00c6_b320, &args![this, constraint]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, HAVOK_ADD_REFERENCE), [[constraint]]);
        assert_eq!(e.get(this, BhkWorld::iConstraintInstanceBatchCount), 1);
        let array = e.get(this, BhkWorld::pConstraintInstanceBatch).addr();
        assert_eq!(e.mem.u32(array), constraint);
        // A full batch (200) first flushes the entity batch (its count is
        // clamped and cleared by `fn_00c68a40`, which has no world to use).
        once_world(&mut e, this, 0x0500_0000);
        e.set(this, BhkWorld::iConstraintInstanceBatchCount, 200);
        e.set(this, BhkWorld::iEntityBatchCount, 4000);
        e.call(0x00c6_b320, &args![this, constraint]);
        assert_eq!(e.get(this, BhkWorld::iConstraintInstanceBatchCount), 201);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 0);
    }

    #[test]
    fn add_ov3_adds_an_action_and_registers_its_object() {
        let mut e = engine();
        let (this, world) = adding_world(&mut e, false);
        let scope = scope_marker(&mut e);
        interlocked_doubles(&mut e);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner + 4, 1);
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + 0x10, inner);
        log(&mut e);
        e.call(0x00c6_b3c0, &args![this, action]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, WORLD_ADD_ACTION), [[world, action]]);
        // The object went into the action list at +0x90 and the temporary
        // reference was dropped again (the list holds the count of 2).
        assert_eq!(e.mem.u32(this.addr() + 0x90), inner);
        assert_eq!(e.mem.u32(inner + 4), 2);
        assert_scope_restored(&mut e, scope);
        // Without an hkpWorld or `this` nothing is added.
        let bare = world_with(&mut e, 0);
        log(&mut e);
        e.call(0x00c6_b3c0, &args![bare, action]);
        e.call(0x00c6_b3c0, &args![0u32, action]);
        assert!(calls_to(&calls(&mut e), WORLD_ADD_ACTION).is_empty());
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn add_ov3_queues_actions_when_batching() {
        let mut e = engine();
        let (this, _) = adding_world(&mut e, true);
        let action = e.mem.alloc(0x40);
        log(&mut e);
        e.call(0x00c6_b3c0, &args![this, action]);
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, HAVOK_ADD_REFERENCE), [[action]]);
        assert!(calls_to(&seen, WORLD_ADD_ACTION).is_empty());
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 1);
        let array = e.get(this, BhkWorld::pActionBatch).addr();
        assert_eq!(e.mem.u32(array), action);
        // A full batch (100) flushes the entity tail once, then queues.
        once_world(&mut e, this, 0x0500_0000);
        e.set(this, BhkWorld::iActionBatchCount, 100);
        e.set(this, BhkWorld::iEntityBatchCount, 9);
        e.set_global(GLOBAL_ENTITY_FLUSH_LIMIT, 4u32);
        e.call(0x00c6_b3c0, &args![this, action]);
        assert_eq!(e.get(this, BhkWorld::iEntityBatchCount), 5);
        assert_eq!(e.global::<u8>(GLOBAL_PARTIAL_FLUSH_DONE), 1);
        assert_eq!(e.get(this, BhkWorld::iActionBatchCount), 101);
    }

    // ----- picking -----

    /// A world and pick data for `PickObject` with the thread's profiling
    /// record stream (room for two records) and `CalcTo` doubled.
    fn pick_fixture(e: &mut Engine, world: u32) -> (Ptr<BhkWorld>, u32, u32) {
        let this = world_with(e, world);
        let pick = e.mem.alloc(0xb0);
        e.set_global(GLOBAL_PICK_COUNT_OVERRIDE, 0u8);
        e.set_global(GLOBAL_PICK_LIMIT_SELECTOR, 0u8);
        e.set_global(GLOBAL_PICK_LIMIT_NORMAL, 1.0f32);
        e.set_global(GLOBAL_WORLD_FLOAT, 0.5f32);
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(GLOBAL_PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [3]);
            ret(stream)
        });
        quiet(e, &[PICK_CALC_TO]);
        (this, pick, stream)
    }

    #[test]
    fn pick_object_skips_the_cast_when_the_limit_is_below_the_world_float() {
        let mut e = engine();
        let (this, pick, stream) = pick_fixture(&mut e, 0);
        e.set_global(GLOBAL_WORLD_FLOAT, 2.0f32);
        e.set_global(GLOBAL_PICK_LIMIT_ALTERNATE, 5.0f32);
        for offset in [0x70, 0x80] {
            e.mem.set_u32(pick + offset, 5);
        }
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
        assert_eq!(e.mem.u8(pick + 0xac), 1);
        assert_eq!(e.mem.f32(pick + 0x40), 1.0);
        assert_eq!(e.mem.u32(pick + 0x44), u32::MAX);
        assert_eq!(e.mem.u32(pick + 0x50), u32::MAX);
        assert_eq!(e.mem.u32(pick + 0x70), 0);
        assert_eq!(e.mem.u32(pick + 0x80), 0);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20);
        // The selector byte picks the larger alternate limit: not skipped.
        e.set_global(GLOBAL_PICK_LIMIT_SELECTOR, 1u8);
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_PICK_OBJECT);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        // The override byte forces the pick as well.
        e.set_global(GLOBAL_PICK_LIMIT_SELECTOR, 0u8);
        e.set_global(GLOBAL_PICK_COUNT_OVERRIDE, 1u8);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.call(0x00c6_96d0, &args![this, pick]);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
        // A full stream takes no record.
        e.mem.set_u32(stream + 4, stream + 0x38);
        e.mem.set_u32(stream + 0x20, 0);
        e.call(0x00c6_96d0, &args![this, pick]);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
        assert_eq!(e.mem.u32(stream + 0x20), 0);
    }

    #[test]
    fn pick_object_casts_into_the_output_and_copies_the_best_hit() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x58, 0x58);
        e.mem.set_u32(world + 0x7c, 0x7c);
        let (this, pick, _) = pick_fixture(&mut e, world);
        let output = e.mem.alloc(0x80);
        let best = e.mem.alloc(0x80);
        e.mem.set_u32(pick + 0xa8, output);
        e.mem.set_u32(output + 0x14, 2);
        e.mem.set_u32(output + 0x10, best);
        e.mem.set_u32(best + 0x50, 0x77);
        quiet(&mut e, &[CAST_RAY, SORT_HITS, HIT_COPY_FROM_OUTPUT]);
        log(&mut e);
        assert!(e.call(0x00c6_96d0, &args![this, pick]).bool());
        let seen = calls(&mut e);
        let of = [PICK_CALC_TO, CAST_RAY, SORT_HITS, HIT_COPY_FROM_OUTPUT];
        let order: Vec<u32> = addresses(&seen)
            .into_iter()
            .filter(|a| of.contains(a))
            .collect();
        assert_eq!(order, of);
        assert_eq!(calls_to(&seen, PICK_CALC_TO), [[pick]]);
        assert_eq!(calls_to(&seen, CAST_RAY), [[world, pick, output]]);
        assert_eq!(calls_to(&seen, SORT_HITS), [[output]]);
        assert_eq!(calls_to(&seen, HIT_COPY_FROM_OUTPUT), [[pick + 0x30, best]]);
        assert_eq!(e.mem.u32(pick + 0x80), 0x77);
        assert_eq!(e.mem.f32(pick + 0x40), 1.0);
        assert_eq!(e.mem.u32(pick + 0x44), u32::MAX);
        assert_eq!(e.mem.u32(pick + 0x50), u32::MAX);
        assert_eq!(e.mem.u32(pick + 0x70), 0);
        // No hits: false and nothing copied.
        e.mem.set_u32(output + 0x14, 0);
        log(&mut e);
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
        let seen = calls(&mut e);
        assert!(calls_to(&seen, SORT_HITS).is_empty());
        assert!(calls_to(&seen, HIT_COPY_FROM_OUTPUT).is_empty());
    }

    #[test]
    fn pick_object_uses_the_filtered_caster_when_a_filter_is_set() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x58, 0x58);
        e.mem.set_u32(world + 0x7c, 0x7c);
        let (this, pick, _) = pick_fixture(&mut e, world);
        let output = e.mem.alloc(0x80);
        e.mem.set_u32(pick + 0xa8, output);
        e.mem.set_u32(pick + 0xa0, 0x0f17);
        e.register_double(CAST_RAY_WITH_FILTER, move |e, a| {
            assert_eq!(a[1..], [0x58, pick, 0x7c, 0x0f17, output]);
            assert_eq!(e.mem.u32(a[0]), RAY_COLLECTOR_VTABLE);
            assert_eq!(e.mem.u32(a[0] + 0x40), 0);
            assert_eq!(e.mem.u32(a[0] + 0x44), 0);
            Ret::default()
        });
        quiet(&mut e, &[CAST_RAY]);
        log(&mut e);
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
        let seen = calls(&mut e);
        assert_eq!(calls_to(&seen, CAST_RAY_WITH_FILTER).len(), 1);
        assert!(calls_to(&seen, CAST_RAY).is_empty());
    }

    #[test]
    fn pick_object_casts_into_the_input_and_copies_its_hit() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        let (this, pick, _) = pick_fixture(&mut e, world);
        let input = e.mem.alloc(0x80);
        e.mem.set_u32(pick + 0xa4, input);
        e.register_double(CAST_RAY, move |e, a| {
            assert_eq!(a, [world, pick, input]);
            assert_eq!(e.mem.f32(input + 0x20), 1.0);
            assert_eq!(e.mem.u32(input + 0x24), u32::MAX);
            assert_eq!(e.mem.u32(input + 0x30), u32::MAX);
            assert_eq!(e.mem.u32(input + 0x50), 0);
            assert_eq!(e.mem.f32(input + 4), 1.0);
            e.mem.set_u32(input + 0x60, 1);
            Ret::default()
        });
        quiet(&mut e, &[HIT_COPY_FROM_INPUT]);
        log(&mut e);
        assert!(e.call(0x00c6_96d0, &args![this, pick]).bool());
        assert_eq!(
            calls_to(&calls(&mut e), HIT_COPY_FROM_INPUT),
            [[pick + 0x30, input + 0x10]]
        );
        // The filtered caster casts into the input as well; no hit, no copy.
        e.mem.set_u32(pick + 0xa0, 0x0f17);
        e.register_double(CAST_RAY_WITH_FILTER, move |_, a| {
            assert_eq!(a[1..], [0, pick, 0, 0x0f17, input]);
            Ret::default()
        });
        log(&mut e);
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
        assert!(calls_to(&calls(&mut e), HIT_COPY_FROM_INPUT).is_empty());
    }

    #[test]
    fn pick_object_falls_back_to_the_second_cast_overload() {
        let mut e = engine();
        let world = e.mem.alloc(0x100);
        let (this, pick, _) = pick_fixture(&mut e, world);
        e.register_double(CAST_RAY_OV2, move |e, a| {
            assert_eq!(a, [world, pick, pick + 0x30]);
            e.mem.set_u32(pick + 0x80, 3);
            Ret::default()
        });
        assert!(e.call(0x00c6_96d0, &args![this, pick]).bool());
        e.register_double(CAST_RAY_OV2, |_, _| Ret::default());
        assert!(!e.call(0x00c6_96d0, &args![this, pick]).bool());
    }

    // ----- AddObjectsAndMoveController -----

    struct MoveFixture {
        this: Ptr<BhkWorld>,
        node: u32,
        other: u32,
        collision: u32,
        body: u32,
        sink: u32,
        prelude: u32,
    }

    /// `node` has a collision object with a body; `other` is a registered,
    /// visitable object. Slot 0xB4 of the collision object answers
    /// `blocked`.
    fn move_fixture(e: &mut Engine, blocked: bool) -> MoveFixture {
        let this = world_with(e, 0);
        registered(e, 2);
        interlocked_doubles(e);
        let collision = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x80);
        let node = visitable_object_with(e, 1, collision);
        let other = visitable_object_with(e, 2, FOUND);
        e.mem.set_u32(collision + 0x10, body);
        vtable_at(e, 0x0400_b400, &[(0xb4, 0x0400_b4f0)]);
        e.mem.set_u32(collision, 0x0400_b400);
        e.register_double(0x0400_b4f0, move |_, _| ret(blocked as u32));
        for lane in 0..4 {
            e.mem.set_f32(GLOBAL_QUATERNION_BIAS + lane * 4, -0.5);
        }
        let sink = record_requests(e, 0, 0);
        let prelude = record_requests(e, 0xa, 0);
        quiet(e, &[SET_COLLISION_OBJECT]);
        MoveFixture {
            this,
            node,
            other,
            collision,
            body,
            sink,
            prelude,
        }
    }

    const MOVE: u32 = 0x00c6_9960;

    #[test]
    fn move_controller_needs_both_objects_and_a_registered_other() {
        let mut e = engine();
        let f = move_fixture(&mut e, false);
        let scope = scope_marker(&mut e);
        let this = f.this;
        assert!(!e.call(MOVE, &args![this, 0u32, f.other]).bool());
        assert!(!e.call(MOVE, &args![this, f.node, 0u32]).bool());
        registered(&mut e, 0);
        assert!(!e.call(MOVE, &args![this, f.node, f.other]).bool());
        assert!(!e.get(this, BhkWorld::bAdded));
        assert_eq!(e.mem.u32(f.sink + 0x24), 0);
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn move_controller_sends_the_add_requests_for_the_same_object() {
        let mut e = engine();
        let f = move_fixture(&mut e, false);
        let scope = scope_marker(&mut e);
        e.set_global(GLOBAL_ADD_PRELUDE_FLAG, 1u8);
        e.mem.set_u32(f.node + 0x30, 0x10);
        // The node is its own target: it is marked added, nothing moves.
        assert!(e.call(MOVE, &args![f.this, f.node, f.node]).bool());
        assert!(e.get(f.this, BhkWorld::bAdded));
        let sink = f.sink;
        assert_eq!(e.mem.u32(sink + 0x24), 1);
        assert_eq!(e.mem.u32(sink), f.this.addr());
        assert_eq!(e.mem.u32(sink + 4) & 0xff, 1);
        assert_eq!(e.mem.u32(sink + 8), 0);
        // `found` is the collision object (the node's holder answers it).
        assert_eq!(e.mem.u32(sink + 0xc), 1);
        assert_eq!(e.mem.u32(sink + 0x10), 0);
        assert_eq!(e.mem.u32(sink + 0x14), 0);
        assert_eq!(e.mem.u32(sink + 0x18), 1);
        assert_eq!(e.mem.u32(sink + 0x20), f.collision);
        assert_eq!(e.mem.u32(f.prelude + 0x24), 1);
        assert_eq!(e.mem.u32(f.prelude + 8), 0xa);
        // The kind 0 visit set bits 1 and 2 of the flags and cleared bit 4.
        assert_eq!(e.mem.u32(f.node + 0x30), 6);
        assert_scope_restored(&mut e, scope);
    }

    #[test]
    fn move_controller_reports_a_blocked_collision_object_as_not_added() {
        let mut e = engine();
        let f = move_fixture(&mut e, true);
        assert!(!e.call(MOVE, &args![f.this, f.node, f.other]).bool());
        assert!(!e.get(f.this, BhkWorld::bAdded));
        // The requests still go out with the found flag set.
        assert_eq!(e.mem.u32(f.sink + 0x24), 1);
        assert_eq!(e.mem.u32(f.sink + 0xc), 1);
        // A node without a holder sends them with the flag clear.
        e.mem.set_u32(f.node + 0x1c, 0);
        assert!(!e.call(MOVE, &args![f.this, f.node, f.other]).bool());
        assert_eq!(e.mem.u32(f.sink + 0x24), 2);
        assert_eq!(e.mem.u32(f.sink + 0xc), 0);
    }

    #[test]
    fn move_controller_composes_the_bodys_transform_with_the_nodes() {
        let mut e = engine();
        let f = move_fixture(&mut e, false);
        let (node, other, collision, body) = (f.node, f.other, f.collision, f.body);
        write_vector(&mut e, body + 0x20, [0.0, 0.0, 0.0, 1.0]);
        write_vector(&mut e, body + 0x30, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(node + 0x30, 0x11);
        e.register_double(BODY_ACCEPT_CHECK, move |_, a| {
            assert_eq!(a, [GLOBAL_BODY_CHECK_TABLE, body]);
            ret(1)
        });
        e.register_double(NI_TO_HK_TRANSFORM, move |e, a| {
            assert_eq!(a[1], node + 0x34);
            e.mem.set_u32(a[0], 0xcafe);
            Ret::default()
        });
        e.register_double(TRANSFORM_POSITION, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0xcafe);
            let vector = read_vector(e, a[2]);
            // The rotation by the identity quaternion returned the position.
            assert_eq!(vector, [1.0, 2.0, 3.0, 4.0]);
            write_vector(e, a[0], [2.0, 4.0, 6.0, 4.0]);
            Ret::default()
        });
        e.register_double(QUATERNION_FROM_TRANSFORM, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0xcafe);
            write_vector(e, a[0], [0.1, 0.2, 0.3, 0.4]);
            Ret::default()
        });
        log(&mut e);
        assert!(e.call(MOVE, &args![f.this, node, other]).bool());
        let seen = calls(&mut e);
        assert_eq!(read_vector(&mut e, body + 0x30), [2.0, 4.0, 6.0, 4.0]);
        assert_eq!(read_vector(&mut e, body + 0x20), [0.1, 0.2, 0.3, 0.4]);
        assert_eq!(
            calls_to(&seen, SET_COLLISION_OBJECT),
            [[other, collision], [node, 0]]
        );
        // Bit 4 cleared, bits 1 and 2 set.
        assert_eq!(e.mem.u32(node + 0x30), 7);
    }

    #[test]
    fn move_controller_clones_a_body_the_check_does_not_accept() {
        let mut e = engine();
        let f = move_fixture(&mut e, false);
        let (node, other, collision, body) = (f.node, f.other, f.collision, f.body);
        e.register_double(BODY_ACCEPT_CHECK, |_, _| ret(0));
        // Slot 0x40 of the body answers the clone, a counted object whose
        // slots 0xD4 / 0xD8 give position and rotation.
        let clone = e.mem.alloc(0x40);
        e.mem.set_u32(clone + 4, 1);
        vtable_at(
            &mut e,
            0x0400_b800,
            &[(4, 0x0400_b8f0), (0xd4, 0x0400_b8f4), (0xd8, 0x0400_b8f8)],
        );
        e.mem.set_u32(clone, 0x0400_b800);
        vtable_at(&mut e, 0x0400_bc00, &[(0x40, 0x0400_bcf0)]);
        e.mem.set_u32(body, 0x0400_bc00);
        e.register_double(0x0400_bcf0, move |_, a| {
            assert_eq!(a, [body]);
            ret(clone)
        });
        e.register_double(0x0400_b8f0, move |_, a| {
            assert_eq!(a, [clone]);
            Ret::default()
        });
        e.register_double(0x0400_b8f4, |e, a| {
            write_vector(e, a[1], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register_double(0x0400_b8f8, |e, a| {
            write_vector(e, a[1], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        // The pointer wrapper stores the clone; the new body is built in
        // aligned memory.
        e.register_double(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(POINTER_RELEASE, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0);
            Ret::default()
        });
        let memory = e.mem.alloc(0x40);
        e.register_double(HAVOK_ALIGNED_ALLOC, move |_, a| {
            assert_eq!(a, [0x40]);
            ret(memory)
        });
        e.register_double(BODY_CREATE_T, move |_, a| {
            assert_eq!(a, [memory, clone]);
            ret(memory)
        });
        vtable_at(
            &mut e,
            0x0400_b000,
            &[(0xdc, 0x0400_b0f0), (0xe0, 0x0400_b0f4)],
        );
        e.mem.set_u32(memory, 0x0400_b000);
        e.register_double(0x0400_b0f0, move |_, a| {
            assert_eq!(a, [memory, GLOBAL_ZERO_VECTOR]);
            Ret::default()
        });
        e.register_double(0x0400_b0f4, move |_, a| {
            assert_eq!(a, [memory, GLOBAL_BODY_ZERO_ARGUMENT]);
            Ret::default()
        });
        quiet(
            &mut e,
            &[SET_WORLD_OBJECT, NI_TO_HK_TRANSFORM, TRANSFORM_POSITION],
        );
        e.register_double(QUATERNION_FROM_TRANSFORM, |e, a| {
            write_vector(e, a[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register_double(TRANSFORM_POSITION, |e, a| {
            write_vector(e, a[0], [5.0, 6.0, 7.0, 8.0]);
            Ret::default()
        });
        log(&mut e);
        assert!(e.call(MOVE, &args![f.this, node, other]).bool());
        let seen = calls(&mut e);
        assert_eq!(
            calls_to(&seen, SET_WORLD_OBJECT),
            [[collision, 0], [collision, memory]]
        );
        // The copy lost its last reference and was deleted through slot 4.
        assert_eq!(e.mem.u32(clone + 4), 0);
        assert_eq!(calls_to(&seen, 0x0400_b8f0).len(), 1);
        // The new body took the clone's position and rotation, then the
        // composition with the node's transform.
        assert_eq!(read_vector(&mut e, memory + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(read_vector(&mut e, memory + 0x20), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            calls_to(&seen, SET_COLLISION_OBJECT),
            [[other, collision], [node, 0]]
        );
    }

    #[test]
    fn move_controller_gives_the_node_an_identity_transform_without_a_body() {
        let mut e = engine();
        let f = move_fixture(&mut e, false);
        let (node, other, collision, body) = (f.node, f.other, f.collision, f.body);
        e.register_double(BODY_ACCEPT_CHECK, |_, _| ret(0));
        vtable_at(&mut e, 0x0400_bc00, &[(0x40, 0x0400_bcf0)]);
        e.mem.set_u32(body, 0x0400_bc00);
        // The body's clone is null: the pointer wrapper stores null.
        e.register_double(0x0400_bcf0, |_, _| ret(0));
        e.register_double(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet(&mut e, &[POINTER_RELEASE]);
        for word in 0..9 {
            e.mem
                .set_u32(GLOBAL_IDENTITY_MATRIX + word * 4, 0x100 + word);
        }
        for word in 0..3 {
            e.mem
                .set_u32(GLOBAL_IDENTITY_TRANSLATION + word * 4, 0x200 + word);
        }
        e.mem.set_u32(node + 0x30, 0x11);
        log(&mut e);
        assert!(e.call(MOVE, &args![f.this, node, other]).bool());
        let seen = calls(&mut e);
        assert_eq!(
            calls_to(&seen, SET_COLLISION_OBJECT),
            [[other, collision], [node, 0]]
        );
        for word in 0..9 {
            assert_eq!(e.mem.u32(node + 0x34 + word * 4), 0x100 + word);
        }
        for word in 0..3 {
            assert_eq!(e.mem.u32(node + 0x58 + word * 4), 0x200 + word);
        }
        assert_eq!(e.mem.u32(node + 0x30), 7);
        assert!(calls_to(&seen, SET_WORLD_OBJECT).is_empty());
    }

    #[test]
    fn rotation_helper_rotates_a_vector_about_an_axis() {
        let bias = [-0.5; 4];
        // 0.6 / 0.8 are sin and cos of half the angle: cos = 0.28, sin = 0.96.
        let q = [0.0, 0.0, 0.6, 0.8];
        let rotated = rotate_by_quaternion(q, [1.0, 0.0, 0.0, 0.0], bias, false);
        for (value, expected) in rotated.into_iter().zip([0.28, 0.96, 0.0, 0.0]) {
            assert!((value - expected).abs() < 1e-6, "{rotated:?}");
        }
        // Negating the cross product rotates the other way.
        let back = rotate_by_quaternion(q, [1.0, 0.0, 0.0, 0.0], bias, true);
        for (value, expected) in back.into_iter().zip([0.28, -0.96, 0.0, 0.0]) {
            assert!((value - expected).abs() < 1e-6, "{back:?}");
        }
    }

    #[test]
    fn profile_record_appends_a_record_while_there_is_room() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x2c);
        e.set_global(GLOBAL_PROFILE_SLOT, 1u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        profile_record(&mut e, 0x1234);
        assert_eq!(e.mem.u32(stream + 0x20), 0x1234);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x2c);
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double at `address` that records the argument words of each call and answers `answer`.
    fn recording_double(e: &mut Engine, address: u32, answer: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// An object whose vtable (zeroed, 0x100 bytes) holds `0x7000_0000 + slot` in each of `slots`.
    fn object_with_slots(e: &mut Engine, slots: &[u32]) -> u32 {
        let table = e.mem.alloc(0x100);
        for slot in slots {
            e.mem.set_u32(table + slot, 0x7000_0000 + slot);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    /// A `bhkNiCollisionObject` (flags at `+0xc`, world object at `+0x10`).
    fn new_collision_object(e: &mut Engine, flags: u16, world_object: u32) -> u32 {
        let object = e.mem.alloc(0x14);
        e.mem.set_u16(object + 0xc, flags);
        e.mem.set_u32(object + 0x10, world_object);
        object
    }

    /// A callback request record of five words.
    fn new_request(e: &mut Engine, words: [u32; 5]) -> u32 {
        let request = e.mem.alloc(0x14);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(request + 4 * index as u32, *word);
        }
        request
    }

    /// `(collision_object, body, havok_object)`: a collision object whose world object answers
    /// `body` to slot `0x40`, and `body` holds `havok_object` at `+8`.
    fn body_in_world(e: &mut Engine) -> (u32, u32, u32) {
        let havok_object = e.mem.alloc(0x100);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 8, havok_object);
        let world_object = object_with_slots(e, &[0x40]);
        e.register_double(0x7000_0040, move |_, _| Ret {
            eax: body,
            ..Ret::default()
        });
        let collision_object = new_collision_object(e, 0, world_object);
        (collision_object, body, havok_object)
    }

    #[test]
    fn fn_00c66250_sets_the_reset_flag() {
        let mut e = Engine::new();
        let collision_object = new_collision_object(&mut e, 0x8001, 0);
        e.call(0x00c6_6250, &args![collision_object, 0u32]);
        assert_eq!(e.mem.u16(collision_object + 0xc), 0x8041);
    }

    #[test]
    fn fn_00c66260_sets_or_clears_the_active_flag() {
        let mut e = Engine::new();
        let collision_object = new_collision_object(&mut e, 0x8040, 0);
        let request = new_request(&mut e, [0, 0, 0, 5, 0]);
        e.call(0x00c6_6260, &args![collision_object, request]);
        assert_eq!(e.mem.u16(collision_object + 0xc), 0x8041);
        e.mem.set_u32(request + 0xc, 0);
        e.call(0x00c6_6260, &args![collision_object, request]);
        assert_eq!(e.mem.u16(collision_object + 0xc), 0x8040);
    }

    #[test]
    fn fn_00c66280_calls_slot_b0_with_the_value_zero_and_the_flag() {
        let mut e = Engine::new();
        let collision_object = object_with_slots(&mut e, &[0xb0]);
        let log = recording_double(&mut e, 0x7000_00b0, 0x99);
        let request = new_request(&mut e, [0, 0, 0, 7, 9]);
        assert_eq!(
            e.call(0x00c6_6280, &args![collision_object, request]).u32(),
            0x99
        );
        e.mem.set_u32(request + 0x10, 0);
        e.call(0x00c6_6280, &args![collision_object, request]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![collision_object, 7, 0, 1],
                vec![collision_object, 7, 0, 0]
            ]
        );
    }

    #[test]
    fn fn_00c662b0_calls_slot_c0_with_the_flag() {
        let mut e = Engine::new();
        let collision_object = object_with_slots(&mut e, &[0xc0]);
        let log = recording_double(&mut e, 0x7000_00c0, 0x98);
        let request = new_request(&mut e, [0, 0, 0, 3, 0]);
        assert_eq!(
            e.call(0x00c6_62b0, &args![collision_object, request]).u32(),
            0x98
        );
        e.mem.set_u32(request + 0xc, 0);
        e.call(0x00c6_62b0, &args![collision_object, request]);
        assert_eq!(
            *log.borrow(),
            vec![vec![collision_object, 1], vec![collision_object, 0]]
        );
    }

    #[test]
    fn fn_00c662d0_runs_the_off_or_on_step_on_the_blend_object() {
        let mut e = Engine::new();
        let collision_object = object_with_slots(&mut e, &[0x3c]);
        recording_double(&mut e, 0x7000_003c, 0x8000);
        let off = recording_double(&mut e, 0x00c8_1b40, 0);
        let on = recording_double(&mut e, 0x00c8_1ae0, 0);
        let request = new_request(&mut e, [0, 0, 0, 0, 0]);
        e.call(0x00c6_62d0, &args![collision_object, request]);
        assert_eq!((off.borrow().len(), on.borrow().len()), (1, 0));
        assert_eq!(*off.borrow(), vec![vec![0x8000]]);
        e.mem.set_u32(request + 0xc, 1);
        e.call(0x00c6_62d0, &args![collision_object, request]);
        assert_eq!(*on.borrow(), vec![vec![0x8000]]);
        // A null object does nothing.
        e.call(0x00c6_62d0, &args![0u32, request]);
        assert_eq!((off.borrow().len(), on.borrow().len()), (1, 1));
    }

    #[test]
    fn fn_00c662d0_does_nothing_without_a_blend_object() {
        let mut e = Engine::new();
        let collision_object = object_with_slots(&mut e, &[0x3c]);
        recording_double(&mut e, 0x7000_003c, 0);
        let off = recording_double(&mut e, 0x00c8_1b40, 0);
        let on = recording_double(&mut e, 0x00c8_1ae0, 0);
        let request = new_request(&mut e, [0, 0, 0, 0, 0]);
        e.call(0x00c6_62d0, &args![collision_object, request]);
        assert!(off.borrow().is_empty() && on.borrow().is_empty());
    }

    #[test]
    fn fn_00c67110_calls_slot_a0_of_the_world_object() {
        let mut e = Engine::new();
        let world_object = object_with_slots(&mut e, &[0xa0]);
        let log = recording_double(&mut e, 0x7000_00a0, 0);
        let with_world = new_collision_object(&mut e, 0, world_object);
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_7110, &args![without_world, 0u32]);
        assert!(log.borrow().is_empty());
        e.call(0x00c6_7110, &args![with_world, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![world_object]]);
    }

    #[test]
    fn fn_00c67130_hands_the_flag_to_the_world_object() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00c8_5d20, 0);
        let with_world = new_collision_object(&mut e, 0, 0x6000);
        let without_world = new_collision_object(&mut e, 0, 0);
        let request = new_request(&mut e, [0, 0, 0, 4, 0]);
        e.call(0x00c6_7130, &args![without_world, request]);
        assert!(log.borrow().is_empty());
        e.call(0x00c6_7130, &args![with_world, request]);
        e.mem.set_u32(request + 0xc, 0);
        e.call(0x00c6_7130, &args![with_world, request]);
        assert_eq!(*log.borrow(), vec![vec![0x6000, 1], vec![0x6000, 0]]);
    }

    #[test]
    fn fn_00c67150_copies_the_float_to_offset_94_of_the_havok_object() {
        let mut e = Engine::new();
        let (collision_object, _, havok_object) = body_in_world(&mut e);
        let request = new_request(&mut e, [0, 0, 0, 1.5f32.to_bits(), 0]);
        e.call(0x00c6_7150, &args![collision_object, request]);
        assert_eq!(e.mem.u32(havok_object + 0x94), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(havok_object + 0x90), 0);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(request + 0xc, 0x7f80_0001);
        e.call(0x00c6_7150, &args![collision_object, request]);
        assert_eq!(e.mem.u32(havok_object + 0x94), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c67150_needs_a_world_object_and_a_body() {
        let mut e = Engine::new();
        let request = new_request(&mut e, [0, 0, 0, 1.5f32.to_bits(), 0]);
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_7150, &args![without_world, request]);
        let world_object = object_with_slots(&mut e, &[0x40]);
        recording_double(&mut e, 0x7000_0040, 0);
        let without_body = new_collision_object(&mut e, 0, world_object);
        e.call(0x00c6_7150, &args![without_body, request]);
    }

    #[test]
    fn fn_00c67180_copies_the_float_to_offset_90_of_the_havok_object() {
        let mut e = Engine::new();
        let (collision_object, _, havok_object) = body_in_world(&mut e);
        let request = new_request(&mut e, [0, 0, 0, 2.5f32.to_bits(), 0]);
        e.call(0x00c6_7180, &args![collision_object, request]);
        assert_eq!(e.mem.u32(havok_object + 0x90), 2.5f32.to_bits());
        assert_eq!(e.mem.u32(havok_object + 0x94), 0);
        e.mem.set_u32(request + 0xc, 0x7f80_0001);
        e.call(0x00c6_7180, &args![collision_object, request]);
        assert_eq!(e.mem.u32(havok_object + 0x90), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c67180_needs_a_world_object_and_a_body() {
        let mut e = Engine::new();
        let request = new_request(&mut e, [0, 0, 0, 1.5f32.to_bits(), 0]);
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_7180, &args![without_world, request]);
        let world_object = object_with_slots(&mut e, &[0x40]);
        recording_double(&mut e, 0x7000_0040, 0);
        let without_body = new_collision_object(&mut e, 0, world_object);
        e.call(0x00c6_7180, &args![without_body, request]);
    }

    #[test]
    fn fn_00c671b0_hands_the_flag_to_the_body() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x0056_1580, 0);
        let (collision_object, _, _) = body_in_world(&mut e);
        let request = new_request(&mut e, [0, 0, 0, 6, 0]);
        e.call(0x00c6_71b0, &args![collision_object, request]);
        e.mem.set_u32(request + 0xc, 0);
        e.call(0x00c6_71b0, &args![collision_object, request]);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][1..], [1]);
        assert_eq!(calls[1][1..], [0]);
        assert_eq!(calls[0][0], calls[1][0]);
    }

    #[test]
    fn fn_00c671b0_does_nothing_without_a_collision_object_world_object_or_body() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x0056_1580, 0);
        let request = new_request(&mut e, [0, 0, 0, 6, 0]);
        e.call(0x00c6_71b0, &args![0u32, request]);
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_71b0, &args![without_world, request]);
        let world_object = object_with_slots(&mut e, &[0x40]);
        recording_double(&mut e, 0x7000_0040, 0);
        let without_body = new_collision_object(&mut e, 0, world_object);
        e.call(0x00c6_71b0, &args![without_body, request]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00c671f0_releases_the_object_held_by_the_scene_object() {
        for (count, deleted) in [(1u32, true), (2, false)] {
            let mut e = Engine::new();
            e.register(0x00fd_f1c4, |e, a| {
                let value = e.mem.u32(a[0]).wrapping_sub(1);
                e.mem.set_u32(a[0], value);
                Ret {
                    eax: value,
                    ..Ret::default()
                }
            });
            let held = object_with_slots(&mut e, &[4]);
            e.mem.set_u32(held + 4, count);
            let log = recording_double(&mut e, 0x7000_0004, 0);
            let scene_object = e.mem.alloc(0x40);
            e.mem.set_u32(scene_object + 0x1c, held);
            let collision_object = new_collision_object(&mut e, 0, 0);
            e.mem.set_u32(collision_object + 8, scene_object);
            e.call(0x00c6_71f0, &args![collision_object, 0u32]);
            assert_eq!(e.mem.u32(scene_object + 0x1c), 0);
            assert_eq!(e.mem.u32(held + 4), count - 1);
            assert_eq!(
                *log.borrow(),
                if deleted { vec![vec![held]] } else { vec![] }
            );
        }
    }

    #[test]
    fn fn_00c671f0_leaves_an_empty_scene_object_and_a_null_scene_object_alone() {
        let mut e = Engine::new();
        let scene_object = e.mem.alloc(0x40);
        let collision_object = new_collision_object(&mut e, 0, 0);
        e.mem.set_u32(collision_object + 8, scene_object);
        e.call(0x00c6_71f0, &args![collision_object, 0u32]);
        assert_eq!(e.mem.u32(scene_object + 0x1c), 0);
        e.mem.set_u32(collision_object + 8, 0);
        e.call(0x00c6_71f0, &args![collision_object, 0u32]);
    }

    #[test]
    fn fn_00c67240_hands_the_value_to_the_world_object() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00c8_5ce0, 0);
        let with_world = new_collision_object(&mut e, 0, 0x6000);
        let without_world = new_collision_object(&mut e, 0, 0);
        let request = new_request(&mut e, [0, 0, 0, 0x1234, 0]);
        e.call(0x00c6_7240, &args![without_world, request]);
        assert!(log.borrow().is_empty());
        e.call(0x00c6_7240, &args![with_world, request]);
        assert_eq!(*log.borrow(), vec![vec![0x6000, 0x1234]]);
    }

    #[test]
    fn fn_00c67260_counts_when_the_lookup_answers_a_set_byte() {
        for (byte, counted) in [(1u8, 1u32), (0, 0)] {
            let mut e = Engine::new();
            let (collision_object, _, havok_object) = body_in_world(&mut e);
            let answer = e.mem.alloc(4);
            e.mem.set_u8(answer, byte);
            let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00c9_bff0, move |e, a| {
                log.borrow_mut().push((a[0], e.mem.u32(a[1])));
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
            let request = new_request(&mut e, [0, 0x0101, 0, 5, 0]);
            e.call(0x00c6_7260, &args![collision_object, request]);
            assert_eq!(*seen.borrow(), vec![(havok_object, collision_object)]);
            assert_eq!(e.mem.u32(request + 0xc), 5 + counted);
            assert_eq!(e.mem.u8(request + 4), 0);
            // The byte next to it is not touched.
            assert_eq!(e.mem.u8(request + 5), 1);
        }
    }

    /// `00c9bff0` writes the state into the argument slot and answers the slot, as the exe's
    /// does: the byte is tested while the slot is still there. With the freed-block check a slot
    /// freed before the test would read as the fill word's low byte, counting by chance.
    #[test]
    fn fn_00c67260_tests_the_byte_in_a_live_slot() {
        for (byte, counted) in [(1u8, 1u32), (0, 0)] {
            let mut e = Engine::new();
            e.mem.enable_freed_check(true);
            let (collision_object, _, _) = body_in_world(&mut e);
            e.register_double(0x00c9_bff0, move |e, a| {
                e.mem.set_u8(a[1], byte);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            let request = new_request(&mut e, [0, 0x0101, 0, 5, 0]);
            e.call(0x00c6_7260, &args![collision_object, request]);
            assert_eq!(e.mem.u32(request + 0xc), 5 + counted, "byte {byte}");
            assert_eq!(e.mem.u8(request + 4), 0);
        }
    }

    #[test]
    fn fn_00c67260_without_a_havok_object_only_clears_the_request_byte() {
        let mut e = Engine::new();
        let (collision_object, body, _) = body_in_world(&mut e);
        e.mem.set_u32(body + 8, 0);
        let log = recording_double(&mut e, 0x00c9_bff0, 0);
        let request = new_request(&mut e, [0, 1, 0, 5, 0]);
        e.call(0x00c6_7260, &args![collision_object, request]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(request + 0xc), 5);
        assert_eq!(e.mem.u8(request + 4), 0);
    }

    #[test]
    fn fn_00c672a0_replaces_the_low_seven_bits_and_updates_the_filter() {
        for (expected, new_bits, changes) in [
            (0u32, 0x2au32, true),
            (0x15, 0x2a, true),
            (0x20, 0x2a, false),
            (0, 0x15, false),
        ] {
            let mut e = Engine::new();
            let world_object = object_with_slots(&mut e, &[0xc4]);
            let update = recording_double(&mut e, 0x7000_00c4, 0);
            let lock = recording_double(&mut e, 0x00a2_9680, 0);
            let havok_object = e.mem.alloc(0x40);
            e.mem.set_u32(havok_object + 0x2c, 0x1234_5695);
            e.mem.set_u32(world_object + 8, havok_object);
            let collision_object = new_collision_object(&mut e, 0, world_object);
            let request = new_request(&mut e, [0, 0, 0, expected, new_bits]);
            e.call(0x00c6_72a0, &args![collision_object, request]);
            if changes {
                assert_eq!(e.mem.u32(havok_object + 0x2c), 0x1234_56aa);
                assert_eq!(*lock.borrow(), vec![vec![world_object], vec![world_object]]);
                assert_eq!(*update.borrow(), vec![vec![world_object]]);
            } else {
                assert_eq!(e.mem.u32(havok_object + 0x2c), 0x1234_5695);
                assert!(lock.borrow().is_empty());
                assert!(update.borrow().is_empty());
            }
        }
    }

    #[test]
    fn fn_00c672a0_without_a_havok_object_only_updates_the_filter() {
        let mut e = Engine::new();
        let world_object = object_with_slots(&mut e, &[0xc4]);
        let update = recording_double(&mut e, 0x7000_00c4, 0);
        let lock = recording_double(&mut e, 0x00a2_9680, 0);
        let collision_object = new_collision_object(&mut e, 0, world_object);
        let request = new_request(&mut e, [0, 0, 0, 0, 5]);
        e.call(0x00c6_72a0, &args![collision_object, request]);
        assert!(lock.borrow().is_empty());
        assert_eq!(*update.borrow(), vec![vec![world_object]]);
        // The current bits read as 0, so a request for 0 changes nothing.
        e.mem.set_u32(request + 0x10, 0);
        e.call(0x00c6_72a0, &args![collision_object, request]);
        assert_eq!(update.borrow().len(), 1);
    }

    #[test]
    fn fn_00c68990_sets_or_clears_bit_6_of_the_body_flags() {
        let mut e = Engine::new();
        let (collision_object, body, _) = body_in_world(&mut e);
        e.mem.set_u32(body + 0x10, 0x8000_0001);
        let request = new_request(&mut e, [0, 0, 0, 1, 0]);
        e.call(0x00c6_8990, &args![collision_object, request]);
        assert_eq!(e.mem.u32(body + 0x10), 0x8000_0041);
        e.mem.set_u32(request + 0xc, 0);
        e.call(0x00c6_8990, &args![collision_object, request]);
        assert_eq!(e.mem.u32(body + 0x10), 0x8000_0001);
    }

    #[test]
    fn fn_00c68990_needs_a_world_object_and_a_body() {
        let mut e = Engine::new();
        let request = new_request(&mut e, [0, 0, 0, 1, 0]);
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_8990, &args![without_world, request]);
        let world_object = object_with_slots(&mut e, &[0x40]);
        recording_double(&mut e, 0x7000_0040, 0);
        let without_body = new_collision_object(&mut e, 0, world_object);
        e.call(0x00c6_8990, &args![without_body, request]);
    }

    #[test]
    fn fn_00c689c0_calls_slot_e4_for_a_missing_havok_object_or_type_5() {
        for (havok_type, called) in [(Some(5u8), true), (Some(3), false), (None, true)] {
            let mut e = Engine::new();
            let body = object_with_slots(&mut e, &[0xe4]);
            let log = recording_double(&mut e, 0x7000_00e4, 0);
            let world_object = object_with_slots(&mut e, &[0x40]);
            recording_double(&mut e, 0x7000_0040, body);
            if let Some(byte) = havok_type {
                let havok_object = e.mem.alloc(0x100);
                e.mem.set_u8(havok_object + 0xe8, byte);
                e.mem.set_u32(body + 8, havok_object);
            }
            let collision_object = new_collision_object(&mut e, 0, world_object);
            e.call(0x00c6_89c0, &args![collision_object, 0u32]);
            assert_eq!(
                *log.borrow(),
                if called { vec![vec![body, 4]] } else { vec![] }
            );
        }
    }

    #[test]
    fn fn_00c689c0_needs_a_world_object_and_a_body() {
        let mut e = Engine::new();
        let without_world = new_collision_object(&mut e, 0, 0);
        e.call(0x00c6_89c0, &args![without_world, 0u32]);
        let world_object = object_with_slots(&mut e, &[0x40]);
        recording_double(&mut e, 0x7000_0040, 0);
        let without_body = new_collision_object(&mut e, 0, world_object);
        e.call(0x00c6_89c0, &args![without_body, 0u32]);
    }
}
