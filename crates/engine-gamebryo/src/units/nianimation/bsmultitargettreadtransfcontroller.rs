//! `NiAnimation/bsmultitargettreadtransfcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSMultiTargetTreadTransfController::ms_RTTI` (the class's `NiRTTI` record).
const BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI: u32 = 0x011f_36e4;

// Translated from 00a35950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_multi_target_tread_transf_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI)
}

/// `00a2f910`: the jump target of `00a35840`.
const JUMP_TARGET_00A35840: u32 = 0x00a2_f910;

/// `00c27120`: the jump target of `00a35870`.
const JUMP_TARGET_00A35870: u32 = 0x00c2_7120;

/// `00c283f0`: the jump target of `00a35880`.
const JUMP_TARGET_00A35880: u32 = 0x00c2_83f0;

// Translated from 00a35840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a2f910` (engine map: `NiInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSMultiTargetTreadTransfController@@UAAXAAVNiStream@@@Z`.
pub fn bs_multi_target_tread_transf_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35840, &args![this, stream]);
}

// Translated from 00a35870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::SaveBinary` (Xbox PDB): a 5-byte jump to `00c27120` (engine map: `NiPSysResetOnLoopCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSMultiTargetTreadTransfController@@UAAXAAVNiStream@@@Z`.
pub fn bs_multi_target_tread_transf_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35870, &args![this, stream]);
}

// Translated from 00a35880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::IsEqual` (Xbox PDB): a 5-byte jump to `00c283f0` (engine map: `NiPSysResetOnLoopCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSMultiTargetTreadTransfController@@UAA_NPAVNiObject@@@Z`.
pub fn bs_multi_target_tread_transf_controller_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A35880, &args![this, other]).bool()
}

/// `BSMultiTargetTreadTransfController`'s vtable (`01096694`) and size.
const MULTI_TARGET_VTABLE: u32 = 0x0109_6694;
const MULTI_TARGET_SIZE: u32 = 0x44;
/// `BSBlendTreadTransfInterpolator`'s constructor (`00a42f60`) and
/// destructor (`00a42f80`), both `thiscall`; its size.
const BLEND_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a4_2f60;
const BLEND_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a4_2f80;
const BLEND_INTERPOLATOR_SIZE: u32 = 0x34;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free
/// (`00aa4060`), both `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The allocation helpers this unit calls, all `cdecl`: `00aa1420`
/// (`(size)`, the array allocation), `00aa1070` (`(size)`), `00aa10f0`
/// (`(block)`), the sized delete `00aa1460` and the array's sized delete
/// `00aa1490` (`(block, size)`).
const ARRAY_ALLOCATE: u32 = 0x00aa_1420;
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const ARRAY_SIZED_DELETE: u32 = 0x00aa_1490;
/// `memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// The compiler's vector constructor iterator (`00401050`, `stdcall (array,
/// element size, count, constructor)`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `InterlockedIncrement` (import slot, one word: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The `NiTimeController` destructor this class's destructor ends with
/// (`00a6d290`, `thiscall`).
const TIME_CONTROLLER_DESTRUCTOR: u32 = 0x00a6_d290;
/// The 16-bit "no link" index answered by `00a35910` (`01096320`).
const INVALID_LINK_INDEX: u32 = 0x0109_6320;
/// A fresh `NiQuatTransform`: translation (3 words, `011a8400`), rotation
/// (4 words, `011f3704`), scale `float` (`01096b8c`).
const FRESH_TRANSLATE: u32 = 0x011a_8400;
const FRESH_ROTATE: u32 = 0x011f_3704;
const FRESH_SCALE: u32 = 0x0109_6b8c;
/// `NiQuaternion::Slerp` (`00a6e330`, `thiscall (this, t, &p, &q)`, the
/// result in `this`) and `ToRotation` (`004f0180`, `thiscall (this,
/// &matrix)`).
const QUATERNION_SLERP: u32 = 0x00a6_e330;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;

/// `BSMultiTargetTreadTransfController` fields (Xbox PDB, same offsets on
/// PC): `m_uFlags` +8 (`NiTimeController`), `BlendInterpsA` +0x34
/// (`BSBlendTreadTransfInterpolator` array, the count in the word before),
/// `usNumInterps` +0x38, `pLinksA` +0x3C (`LinkAVObj*` array),
/// `usNumLinks` +0x40.
const FLAGS: u32 = 0x8;
const BLEND_INTERPS: u32 = 0x34;
const NUM_INTERPS: u32 = 0x38;
const LINKS: u32 = 0x3c;
const NUM_LINKS: u32 = 0x40;
/// `LinkAVObj` (Xbox PDB), 0xC bytes: `usBlendIdx` +0, `pTarget` +4
/// (`NiAVObject*`), `pLink` +8 (`BSTreadTransfInterpolator::LinkInfo*`).
const LINK_BLEND_INDEX: u32 = 0x0;
const LINK_TARGET: u32 = 0x4;
const LINK_INFO: u32 = 0x8;
/// `NiAVObject` (PC): flags +0x30, the local rotation matrix +0x34 and
/// translation +0x58.
const AV_FLAGS: u32 = 0x30;
const AV_LOCAL_ROTATE: u32 = 0x34;
const AV_LOCAL_TRANSLATE: u32 = 0x58;
/// `BSBlendTreadTransfInterpolator::fCurrLerp` (Xbox PDB) +0x30; the
/// interpolators' transform `Update` (vtable +0x8C, `(time, object,
/// NiQuatTransform*)`).
const BLEND_CURR_LERP: u32 = 0x30;
const UPDATE_TRANSFORM_SLOT: u32 = 0x8c;

/// The byte count of a counted `new[]` of `count` elements of `size`
/// bytes: `count * size` (all ones on overflow) plus the count word (all
/// ones on a carry).
fn array_bytes(count: u32, size: u32) -> u32 {
    let product = count.checked_mul(size).unwrap_or(u32::MAX);
    product.checked_add(4).unwrap_or(u32::MAX)
}

fn link(e: &Engine, this: u32, index: u32) -> u32 {
    let links = e.mem.u32(this + LINKS);
    e.mem.u32(links.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a358a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetInterpolator` (Xbox PDB): the
/// address of blend interpolator `index` (0x34 bytes each).
pub fn bs_multi_target_tread_transf_controller_get_interpolator(
    e: &mut Engine,
    this: Ptr,
    index: u16,
) -> Ptr {
    let interps = e.mem.u32(this.addr() + BLEND_INTERPS);
    Ptr::new(interps.wrapping_add(u32::from(index) * BLEND_INTERPOLATOR_SIZE))
}

// Translated from 00a35910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of the first link whose target is `target` (the engine map
/// has no name); the 16-bit "no link" value at `01096320` when none is.
pub fn fn_00a35910(e: &mut Engine, this: Ptr, target: Ptr) -> u16 {
    let object = this.addr();
    let count = e.mem.u16(object + NUM_LINKS);
    let mut index = 0u16;
    while index < count {
        let item = link(e, object, u32::from(index));
        if e.mem.u32(item.wrapping_add(LINK_TARGET)) == target.addr() {
            return index;
        }
        index = index.wrapping_add(1);
    }
    e.global::<u16>(INVALID_LINK_INDEX)
}

// Translated from 00a35960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::~BSMultiTargetTreadTransfController`
/// (body; the engine map has no name): the class's vtable; the blend
/// interpolator array destroyed (its first element's vector deleting
/// destructor, vtable +0, with 3 when it counts any elements, else its
/// block freed); each link freed; the link array freed (`00aa10f0`); then
/// the `NiTimeController` destructor (a tail jump).
pub fn fn_00a35960(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, MULTI_TARGET_VTABLE);
    let interps = e.mem.u32(object + BLEND_INTERPS);
    if interps != 0 {
        if e.mem.u32(interps.wrapping_sub(4)) != 0 {
            e.vcall(interps, 0, &args![3u32]);
        } else {
            e.call(
                MEMORY_MANAGER_FREE,
                &args![MEMORY_MANAGER, interps.wrapping_sub(4)],
            );
        }
    }
    let mut index = 0u16;
    while index < e.mem.u16(object + NUM_LINKS) {
        let item = link(e, object, u32::from(index));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, item]);
        index = index.wrapping_add(1);
    }
    let links = e.mem.u32(object + LINKS);
    e.call(FREE, &args![links]);
    e.call(TIME_CONTROLLER_DESTRUCTOR, &args![object]);
}

// Translated from 00a35a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The controller's array set-up (the engine map has no name): stores
/// both counts; with no interpolators nothing else. Otherwise a counted
/// array of `interps` blend interpolators (each built by `00a42f60` and
/// counted once), a zeroed array of `links` link pointers, and each link
/// a new zeroed 0xC-byte `LinkAVObj` (null when its allocation fails).
pub fn fn_00a35a80(e: &mut Engine, this: Ptr, interps: u16, links: u16) {
    let object = this.addr();
    e.mem.set_u16(object + NUM_INTERPS, interps);
    e.mem.set_u16(object + NUM_LINKS, links);
    if interps == 0 {
        return;
    }
    let count = u32::from(interps);
    // `count` is never 0 here; the compiler's `new[]` still keeps its
    // zero-size branch (a 4-byte block from the memory manager).
    let block = if count == 0 {
        e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 4u32])
            .u32()
    } else {
        e.call(
            ARRAY_ALLOCATE,
            &args![array_bytes(count, BLEND_INTERPOLATOR_SIZE)],
        )
        .u32()
    };
    let array = if block != 0 {
        e.mem.set_u32(block, count);
        let array = block.wrapping_add(4);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                array,
                BLEND_INTERPOLATOR_SIZE,
                count,
                BLEND_INTERPOLATOR_CONSTRUCTOR
            ],
        );
        array
    } else {
        0
    };
    e.mem.set_u32(object + BLEND_INTERPS, array);
    for i in 0..count {
        let interps = e.mem.u32(object + BLEND_INTERPS);
        let reference_count = interps.wrapping_add(i * BLEND_INTERPOLATOR_SIZE + 4);
        e.call(INTERLOCKED_INCREMENT, &args![reference_count]);
    }
    let link_count = u32::from(links);
    let bytes = link_count * 4;
    let array = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(object + LINKS, array);
    e.call(MEMSET, &args![array, 0u32, bytes]);
    for i in 0..link_count {
        let item = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
            .u32();
        if item != 0 {
            e.mem.set_u32(item, 0);
            e.mem.set_u32(item + 4, 0);
            e.mem.set_u32(item + 8, 0);
        }
        let links = e.mem.u32(object + LINKS);
        e.mem.set_u32(links.wrapping_add(4 * i), item);
    }
}

// Translated from 00a35b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::vector deleting destructor` (Xbox PDB,
/// emitted in this unit): with bit 1 of `flags`, every element of the
/// counted array at `this` destroyed (last first) and, with bit 0, the
/// block (from its count word) given to the array's sized delete with
/// 0x34, answering the block; otherwise the one object destroyed and, with
/// bit 0, deleted (0x34 bytes), answering `this`.
pub fn bs_blend_tread_transf_interpolator_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let object = this.addr();
    if flags & 2 != 0 {
        let block = object.wrapping_sub(4);
        let count = e.mem.u32(block);
        let mut element = count
            .wrapping_mul(BLEND_INTERPOLATOR_SIZE)
            .wrapping_add(object);
        let mut remaining = count.wrapping_sub(1);
        while remaining as i32 >= 0 {
            element = element.wrapping_sub(BLEND_INTERPOLATOR_SIZE);
            e.call(BLEND_INTERPOLATOR_DESTRUCTOR, &args![element]);
            remaining = remaining.wrapping_sub(1);
        }
        if flags & 1 != 0 {
            e.call(ARRAY_SIZED_DELETE, &args![block, BLEND_INTERPOLATOR_SIZE]);
        }
        return Ptr::new(block);
    }
    e.call(BLEND_INTERPOLATOR_DESTRUCTOR, &args![object]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![object, BLEND_INTERPOLATOR_SIZE]);
    }
    this
}

// Translated from 00a35c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::Update` (Xbox PDB; `thiscall
/// (NiUpdateData&)`, `RET 4`): only when bit 3 of the flags byte is set.
/// Each blend interpolator is updated (vtable +0x8C with the data's time,
/// no object and one transform local, set fresh once before the loop);
/// for each that answers true, every link with that interpolator's index,
/// a target, and bit 1 of the target's flags set moves its target: the
/// local translation becomes `Start * (1 - lerp) + End * lerp` and the
/// local rotation the matrix of `Slerp(1 - lerp, End, Start)` of the
/// link's two rotations, with `lerp` the interpolator's `fCurrLerp`. Every
/// product, difference and sum is an x87 operation stored as a `float`, in
/// the exe's order.
pub fn bs_multi_target_tread_transf_controller_update(e: &mut Engine, this: Ptr, data: Ptr) {
    let object = this.addr();
    if (e.mem.u8(object + FLAGS) >> 3) & 1 == 0 {
        return;
    }
    // The frame mirrors the exe's stack (from `ESP` after its first push):
    // the transform local +0x70, the start rotation +0x50, the end
    // rotation +0x60, the slerped rotation +0x90.
    e.with_stack(0xa0, |e, frame| {
        let frame = frame.addr();
        let transform = frame + 0x70;
        let start_rotate = frame + 0x50;
        let end_rotate = frame + 0x60;
        let slerped = frame + 0x90;
        let scale = x87_quiet_bits(e.mem.u32(FRESH_SCALE));
        e.mem.set_u32(transform + 0x1c, scale);
        for i in 0..3 {
            let value = e.mem.u32(FRESH_TRANSLATE + 4 * i);
            e.mem.set_u32(transform + 4 * i, value);
        }
        for i in 0..4 {
            let value = e.mem.u32(FRESH_ROTATE + 4 * i);
            e.mem.set_u32(transform + 0xc + 4 * i, value);
        }
        if e.mem.u16(object + NUM_INTERPS) == 0 {
            return;
        }
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let interp = e.mem.u32(object + BLEND_INTERPS).wrapping_add(offset);
            let time = x87_quiet_bits(e.mem.u32(data.addr()));
            let updated = e
                .vcall(interp, UPDATE_TRANSFORM_SLOT, &args![time, 0u32, transform])
                .bool();
            if updated {
                let interp = e.mem.u32(object + BLEND_INTERPS).wrapping_add(offset);
                let lerp_bits = x87_quiet_bits(e.mem.u32(interp + BLEND_CURR_LERP));
                let mut link_index = 0u16;
                while link_index < e.mem.u16(object + NUM_LINKS) {
                    move_link(
                        e,
                        object,
                        u32::from(link_index),
                        index,
                        lerp_bits,
                        [start_rotate, end_rotate, slerped],
                    );
                    link_index = link_index.wrapping_add(1);
                }
            }
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(BLEND_INTERPOLATOR_SIZE);
            if index as i32 >= i32::from(e.mem.u16(object + NUM_INTERPS)) {
                break;
            }
        }
    });
}

/// One link of `Update`'s inner loop (see there): `locals` are the start
/// rotation, end rotation and slerp result in the frame.
fn move_link(
    e: &mut Engine,
    object: u32,
    link_index: u32,
    interp_index: u32,
    lerp_bits: u32,
    locals: [u32; 3],
) {
    let [start_rotate, end_rotate, slerped] = locals;
    let item = link(e, object, link_index);
    if interp_index != u32::from(e.mem.u16(item + LINK_BLEND_INDEX)) {
        return;
    }
    let target = e.mem.u32(item + LINK_TARGET);
    if target == 0 || (e.mem.u32(target + AV_FLAGS) >> 1) & 1 == 0 {
        return;
    }
    let info = e.mem.u32(item + LINK_INFO);
    let lerp = x87_load32(lerp_bits);
    let start: [u32; 3] = std::array::from_fn(|i| e.mem.u32(info + 4 * i as u32));
    let end: [u32; 3] = std::array::from_fn(|i| e.mem.u32(info + 0x20 + 4 * i as u32));
    let end_scaled = end.map(|value| x87_store32(x87_mul(x87_load32(value), lerp)));
    for i in 0..4 {
        let value = e.mem.u32(info + 0xc + 4 * i);
        e.mem.set_u32(start_rotate + 4 * i, value);
    }
    let item = link(e, object, link_index);
    let info = e.mem.u32(item + LINK_INFO);
    for i in 0..4 {
        let value = e.mem.u32(info + 0x2c + 4 * i);
        e.mem.set_u32(end_rotate + 4 * i, value);
    }
    let rest_bits = x87_store32(x87_sub(1.0, lerp));
    let rest = x87_load32(rest_bits);
    let start_x = x87_store32(x87_mul(x87_load32(start[0]), rest));
    let start_y = x87_store32(x87_mul(x87_load32(start[1]), rest));
    let start_z = x87_store32(x87_mul(rest, x87_m32(start[2])));
    let sum = [
        x87_store32(x87_add(x87_load32(start_x), x87_m32(end_scaled[0]))),
        x87_store32(x87_add(x87_load32(start_y), x87_m32(end_scaled[1]))),
        x87_store32(x87_add(x87_load32(start_z), x87_m32(end_scaled[2]))),
    ];
    e.call(
        QUATERNION_SLERP,
        &args![slerped, rest_bits, end_rotate, start_rotate],
    );
    let item = link(e, object, link_index);
    let target = e.mem.u32(item + LINK_TARGET);
    for (i, value) in sum.iter().enumerate() {
        e.mem
            .set_u32(target + AV_LOCAL_TRANSLATE + 4 * i as u32, *value);
    }
    let item = link(e, object, link_index);
    let target = e.mem.u32(item + LINK_TARGET);
    e.call(
        QUATERNION_TO_ROTATION,
        &args![slerped, target + AV_LOCAL_ROTATE],
    );
}

// Translated from 00a35f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets link `index` (the engine map has no name): its target, link info
/// and blend index; a non-null target gets bit 7 (0x80) of its flags set.
pub fn fn_00a35f70(
    e: &mut Engine,
    this: Ptr,
    index: u16,
    target: Ptr,
    blend_index: u16,
    info: Ptr,
) {
    let object = this.addr();
    let index = u32::from(index);
    let item = link(e, object, index);
    e.mem.set_u32(item + LINK_TARGET, target.addr());
    let item = link(e, object, index);
    e.mem.set_u32(item + LINK_INFO, info.addr());
    let item = link(e, object, index);
    e.mem.set_u16(item + LINK_BLEND_INDEX, blend_index);
    if !target.is_null() {
        let flags = e.mem.u32(target.addr() + AV_FLAGS) | 0x80;
        e.mem.set_u32(target.addr() + AV_FLAGS, flags);
    }
}

// Translated from 00a36050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::scalar deleting destructor` (Xbox
/// PDB): the destructor (`00a35960`), then the sized delete of 0x44 bytes
/// when bit 0 of `flags` is set. Answers `this`.
pub fn bs_multi_target_tread_transf_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a35960(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, MULTI_TARGET_SIZE]);
    }
    this
}

/// `NiInterpolator::IsTransformValueSupported` (vtable +0xB8, Xbox PDB slot
/// order) and `GuaranteeTimeRange` (+0xC4).
const SLOT_IS_TRANSFORM_VALUE_SUPPORTED: u32 = 0xb8;
const SLOT_GUARANTEE_TIME_RANGE: u32 = 0xc4;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor `CreateObject` and `CreateClone` run (`00a2f8c0`,
/// `thiscall`) and the base member copy `CreateClone` runs (`00a2f8a0`,
/// `thiscall (this, clone, cloning)`).
const BASE_CONSTRUCTOR: u32 = 0x00a2_f8c0;
const BASE_COPY_MEMBERS: u32 = 0x00a2_f8a0;
/// `NiObjectNET::m_kName` (+8 on PC): the target's name.
const TARGET_NAME: u32 = 0x8;

// Translated from 00a2fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::InterpolatorIsCorrectType` (Xbox
/// PDB; `thiscall`, two stack words, `RET 8`): the interpolator's
/// `IsTransformValueSupported` (vtable +0xB8); `this` and the index are not
/// read.
pub fn bs_multi_target_tread_transf_controller_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) -> bool {
    e.vcall(
        interpolator.addr(),
        SLOT_IS_TRANSFORM_VALUE_SUPPORTED,
        &args![],
    )
    .bool()
}

// Translated from 00a35890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetInterpolatorCount` (Xbox PDB):
/// `usNumInterps`.
pub fn bs_multi_target_tread_transf_controller_get_interpolator_count(
    e: &mut Engine,
    this: Ptr,
) -> u16 {
    e.mem.u16(this.addr() + NUM_INTERPS)
}

// Translated from 00a358b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GuaranteeTimeRange` (Xbox PDB): both
/// times (each through `FLD`/`FSTP`) passed to `GuaranteeTimeRange` (vtable
/// +0xC4) of every blend interpolator.
pub fn bs_multi_target_tread_transf_controller_guarantee_time_range(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) {
    let object = this.addr();
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    let mut index = 0u16;
    while index < e.mem.u16(object + NUM_INTERPS) {
        let interps = e.mem.u32(object + BLEND_INTERPS);
        let interp = interps.wrapping_add(u32::from(index) * BLEND_INTERPOLATOR_SIZE);
        e.vcall(interp, SLOT_GUARANTEE_TIME_RANGE, &args![start, end]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00a35900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::CreateBlendInterpolator` (Xbox PDB;
/// five stack words, `RET 0x14`): the address of blend interpolator `index`;
/// the other four words are not read.
pub fn bs_multi_target_tread_transf_controller_create_blend_interpolator(
    e: &mut Engine,
    this: Ptr,
    index: u16,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> Ptr {
    let interps = e.mem.u32(this.addr() + BLEND_INTERPS);
    Ptr::new((u32::from(index) * BLEND_INTERPOLATOR_SIZE).wrapping_add(interps))
}

/// The fields a new controller gets after the base constructor (in the
/// exe's order for `CreateObject`): no links, no interpolators.
fn init_controller(e: &mut Engine, object: u32) {
    e.mem.set_u16(object + NUM_INTERPS, 0);
    e.mem.set_u32(object + LINKS, 0);
    e.mem.set_u32(object, MULTI_TARGET_VTABLE);
    e.mem.set_u32(object + BLEND_INTERPS, 0);
    e.mem.set_u16(object + NUM_LINKS, 0);
}

// Translated from 00a359d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): 0x44 bytes from the Ni
/// allocator, the base constructor (`00a2f8c0`), then no interpolators, no
/// links and the class's vtable. Null when the allocation fails.
pub fn fn_00a359d0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![MULTI_TARGET_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCTOR, &args![object]);
    init_controller(e, object);
    Ptr::new(object)
}

// Translated from 00a35a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::CreateClone` (Xbox PDB): a new
/// object as the factory makes it, then the base member copy (`00a2f8a0`)
/// into it; when the allocation fails the copy still runs, into address 0,
/// and 0 is answered.
pub fn bs_multi_target_tread_transf_controller_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![MULTI_TARGET_SIZE]).u32();
    if object == 0 {
        e.call(BASE_COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCTOR, &args![object]);
    e.mem.set_u16(object + NUM_LINKS, 0);
    e.mem.set_u32(object, MULTI_TARGET_VTABLE);
    e.mem.set_u32(object + BLEND_INTERPS, 0);
    e.mem.set_u16(object + NUM_INTERPS, 0);
    e.mem.set_u32(object + LINKS, 0);
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a35e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetInterpolatorID` (Xbox PDB): the
/// name (`m_kName` +8) of link `index`'s target (the exe reads through a
/// null target).
pub fn bs_multi_target_tread_transf_controller_get_interpolator_id(
    e: &mut Engine,
    this: Ptr,
    index: u16,
) -> u32 {
    let item = link(e, this.addr(), u32::from(index));
    let target = e.mem.u32(item.wrapping_add(LINK_TARGET));
    e.mem.u32(target.wrapping_add(TARGET_NAME))
}

/// The inline `strcmp` the exe uses: whether the strings at `a` and `b` are
/// equal (it compares two bytes a step, stopping at the first difference or
/// at the end of `a`).
fn strings_equal(e: &Engine, a: u32, b: u32) -> bool {
    let mut offset = 0u32;
    loop {
        let byte = e.mem.u8(a.wrapping_add(offset));
        if byte != e.mem.u8(b.wrapping_add(offset)) {
            return false;
        }
        if byte == 0 {
            return true;
        }
        let byte = e.mem.u8(a.wrapping_add(offset + 1));
        if byte != e.mem.u8(b.wrapping_add(offset + 1)) {
            return false;
        }
        offset = offset.wrapping_add(2);
        if byte == 0 {
            return true;
        }
    }
}

// Translated from 00a35eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetInterpolatorIndex` (Xbox PDB,
/// the name form): the index of the first link whose target's name equals
/// `name` (the link count and array read once); the "no link" value at
/// `01096320` for a null name or when none does.
pub fn bs_multi_target_tread_transf_controller_get_interpolator_index(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
) -> u16 {
    if name.is_null() {
        return e.global::<u16>(INVALID_LINK_INDEX);
    }
    let object = this.addr();
    let count = e.mem.u16(object + NUM_LINKS);
    let mut index = 0u16;
    if index < count {
        let links = e.mem.u32(object + LINKS);
        loop {
            let item = e.mem.u32(links.wrapping_add(u32::from(index) * 4));
            let target = e.mem.u32(item.wrapping_add(LINK_TARGET));
            if target != 0 {
                let target_name = e.mem.u32(target.wrapping_add(TARGET_NAME));
                if strings_equal(e, target_name, name.addr()) {
                    return index;
                }
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    e.global::<u16>(INVALID_LINK_INDEX)
}

// Translated from 00a35f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetInterpolatorIndexFx` (Xbox PDB,
/// the `NiFixedString` form): the index of the first link whose target's
/// name handle equals `*name`; the loop runs over the interpolator count
/// (`usNumInterps`, read once) though it reads the links. The "no link"
/// value at `01096320` when none does.
pub fn bs_multi_target_tread_transf_controller_get_interpolator_index_fx(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
) -> u16 {
    let object = this.addr();
    let count = e.mem.u16(object + NUM_INTERPS);
    let mut index = 0u16;
    if index < count {
        let links = e.mem.u32(object + LINKS);
        loop {
            let item = e.mem.u32(links.wrapping_add(u32::from(index) * 4));
            let target = e.mem.u32(item.wrapping_add(LINK_TARGET));
            if target != 0 && e.mem.u32(target.wrapping_add(TARGET_NAME)) == e.mem.u32(name.addr())
            {
                return index;
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    e.global::<u16>(INVALID_LINK_INDEX)
}

// Translated from 00a35fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets link `index`'s target; a non-null target
/// gets bit 7 (0x80) of its flags set.
pub fn fn_00a35fc0(e: &mut Engine, this: Ptr, index: u16, target: Ptr) {
    let item = link(e, this.addr(), u32::from(index));
    e.mem.set_u32(item.wrapping_add(LINK_TARGET), target.addr());
    if !target.is_null() {
        let flags = e.mem.u32(target.addr() + AV_FLAGS) | 0x80;
        e.mem.set_u32(target.addr() + AV_FLAGS, flags);
    }
}

// Translated from 00a35fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills the first link without a target (the
/// link count and array read once for the search) with `info`, `blend_index`
/// and `target`, sets bit 7 (0x80) of the target's flags (the exe does not
/// test it for null) and answers the link's index; the "no link" value at
/// `01096320` when every link has a target.
pub fn fn_00a35fe0(e: &mut Engine, this: Ptr, target: Ptr, blend_index: u16, info: Ptr) -> u16 {
    let object = this.addr();
    let count = e.mem.u16(object + NUM_LINKS);
    let mut index = 0u16;
    if index < count {
        let links = e.mem.u32(object + LINKS);
        loop {
            let offset = u32::from(index) * 4;
            let item = e.mem.u32(links.wrapping_add(offset));
            if e.mem.u32(item.wrapping_add(LINK_TARGET)) == 0 {
                e.mem.set_u32(item.wrapping_add(LINK_INFO), info.addr());
                let item = e.mem.u32(e.mem.u32(object + LINKS).wrapping_add(offset));
                e.mem
                    .set_u16(item.wrapping_add(LINK_BLEND_INDEX), blend_index);
                let item = e.mem.u32(e.mem.u32(object + LINKS).wrapping_add(offset));
                e.mem.set_u32(item.wrapping_add(LINK_TARGET), target.addr());
                let flags = e.mem.u32(target.addr().wrapping_add(AV_FLAGS)) | 0x80;
                e.mem.set_u32(target.addr().wrapping_add(AV_FLAGS), flags);
                return index;
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    e.global::<u16>(INVALID_LINK_INDEX)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a35950, bs_multi_target_tread_transf_controller_get_rtti() -> Ptr),
        entry!(
            0x00a3_5840,
            bs_multi_target_tread_transf_controller_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a3_5870,
            bs_multi_target_tread_transf_controller_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a3_5880, bs_multi_target_tread_transf_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a3_58a0, bs_multi_target_tread_transf_controller_get_interpolator(Ptr, u16) -> Ptr),
        entry!(0x00a3_5910, fn_00a35910(Ptr, Ptr) -> u16),
        entry!(0x00a3_5960, fn_00a35960(Ptr)),
        entry!(0x00a3_5a80, fn_00a35a80(Ptr, u16, u16)),
        entry!(0x00a3_5b90, bs_blend_tread_transf_interpolator_vector_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a3_5c00,
            bs_multi_target_tread_transf_controller_update(Ptr, Ptr)
        ),
        entry!(0x00a3_5f70, fn_00a35f70(Ptr, u16, Ptr, u16, Ptr)),
        entry!(0x00a3_6050, bs_multi_target_tread_transf_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a2_fac0,
            bs_multi_target_tread_transf_controller_interpolator_is_correct_type(Ptr, Ptr, u32) -> bool
        ),
        entry!(
            0x00a3_5890,
            bs_multi_target_tread_transf_controller_get_interpolator_count(Ptr) -> u16
        ),
        entry!(
            0x00a3_58b0,
            bs_multi_target_tread_transf_controller_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(
            0x00a3_5900,
            bs_multi_target_tread_transf_controller_create_blend_interpolator(Ptr, u16, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a3_59d0, fn_00a359d0() -> Ptr),
        entry!(
            0x00a3_5a10,
            bs_multi_target_tread_transf_controller_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a3_5e90,
            bs_multi_target_tread_transf_controller_get_interpolator_id(Ptr, u16) -> u32
        ),
        entry!(
            0x00a3_5eb0,
            bs_multi_target_tread_transf_controller_get_interpolator_index(Ptr, Ptr) -> u16
        ),
        entry!(
            0x00a3_5f30,
            bs_multi_target_tread_transf_controller_get_interpolator_index_fx(Ptr, Ptr) -> u16
        ),
        entry!(0x00a3_5fc0, fn_00a35fc0(Ptr, u16, Ptr)),
        entry!(0x00a3_5fe0, fn_00a35fe0(Ptr, Ptr, u16, Ptr) -> u16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_multi_target_tread_transf_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a35950, &args![]).u32(),
            BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a35840_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5840;
        const TARGET: u32 = 0x00a2_f910;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a35870_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5870;
        const TARGET: u32 = 0x00c2_7120;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a35880_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a3_5880;
        const TARGET: u32 = 0x00c2_83f0;
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

    fn mt_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn mt_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &[a[0] + i * a[1]]);
            }
            Ret::default()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for addr in [
            MEMORY_MANAGER_FREE,
            FREE,
            SIZED_DELETE,
            ARRAY_SIZED_DELETE,
            BLEND_INTERPOLATOR_CONSTRUCTOR,
            BLEND_INTERPOLATOR_DESTRUCTOR,
            TIME_CONTROLLER_DESTRUCTOR,
            QUATERNION_TO_ROTATION,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A controller with `count` links, each a zeroed 0xC-byte block.
    fn mt_with_links(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0x44);
        let links = e.mem.alloc(4 * count.max(1));
        for i in 0..count {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(links + 4 * i, item);
        }
        e.mem.set_u32(this + LINKS, links);
        e.mem.set_u16(this + NUM_LINKS, count as u16);
        this
    }

    #[test]
    fn mt_get_interpolator_indexes_the_blend_array() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + BLEND_INTERPS, 0x1000);
        assert_eq!(
            e.call(0x00a3_58a0, &args![this, 0x0003_0002u32]).u32(),
            0x1068
        );
    }

    #[test]
    fn mt_find_link_answers_the_index_or_the_invalid_value() {
        let mut e = Engine::new();
        e.map(INVALID_LINK_INDEX, 4);
        e.set_global(INVALID_LINK_INDEX, 0xffffu16);
        let this = mt_with_links(&mut e, 3);
        let second = link(&e, this, 1);
        e.mem.set_u32(second + LINK_TARGET, 0x5000);
        assert_eq!(e.call(0x00a3_5910, &args![this, 0x5000u32]).u16(), 1);
        assert_eq!(e.call(0x00a3_5910, &args![this, 0x6000u32]).u16(), 0xffff);
    }

    #[test]
    fn mt_destructor_frees_the_arrays_then_runs_the_base() {
        const VECTOR_DELETE: u32 = 0x0300_0000;
        let mut e = mt_engine();
        e.register(VECTOR_DELETE, |_, _| Ret::default());
        let this = mt_with_links(&mut e, 2);
        let block = e.mem.alloc(4 + 0x34);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, VECTOR_DELETE);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 4, vtable);
        e.mem.set_u32(this + BLEND_INTERPS, block + 4);
        let links = e.mem.u32(this + LINKS);
        let (first, second) = (link(&e, this, 0), link(&e, this, 1));
        e.call(0x00a3_5960, &args![this]);
        assert_eq!(e.mem.u32(this), MULTI_TARGET_VTABLE);
        assert_eq!(mt_calls_to(&e, VECTOR_DELETE), vec![vec![block + 4, 3]]);
        assert_eq!(
            mt_calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, first], vec![MEMORY_MANAGER, second]]
        );
        assert_eq!(mt_calls_to(&e, FREE), vec![vec![links]]);
        assert_eq!(
            mt_calls_to(&e, TIME_CONTROLLER_DESTRUCTOR),
            vec![vec![this]]
        );
        // An empty counted array is freed from its count word.
        e.mem.set_u32(block, 0);
        e.mem.set_u16(this + NUM_LINKS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_5960, &args![this]);
        assert_eq!(
            mt_calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, block]]
        );
    }

    #[test]
    fn mt_set_up_builds_counted_interpolators_and_zeroed_links() {
        let mut e = mt_engine();
        let this = e.mem.alloc(0x44);
        e.call(0x00a3_5a80, &args![this, 2u32, 3u32]);
        assert_eq!(e.mem.u16(this + NUM_INTERPS), 2);
        assert_eq!(e.mem.u16(this + NUM_LINKS), 3);
        assert_eq!(mt_calls_to(&e, ARRAY_ALLOCATE), vec![vec![4 + 2 * 0x34]]);
        let interps = e.mem.u32(this + BLEND_INTERPS);
        assert_eq!(e.mem.u32(interps - 4), 2);
        assert_eq!(
            mt_calls_to(&e, BLEND_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![interps], vec![interps + 0x34]]
        );
        assert_eq!(e.mem.u32(interps + 4), 1);
        assert_eq!(e.mem.u32(interps + 0x34 + 4), 1);
        assert_eq!(mt_calls_to(&e, ALLOCATE), vec![vec![12]]);
        assert_eq!(mt_calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 3);
        for i in 0..3 {
            let item = link(&e, this, i);
            assert_ne!(item, 0);
            assert_eq!(e.mem.u32(item + LINK_TARGET), 0);
        }
        // No interpolators: only the counts.
        let other = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        e.call(0x00a3_5a80, &args![other, 0u32, 3u32]);
        assert_eq!(e.mem.u16(other + NUM_LINKS), 3);
        assert!(mt_calls_to(&e, ALLOCATE).is_empty());
        // A failed link allocation leaves a null link.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.call(0x00a3_5a80, &args![other, 1u32, 1u32]);
        assert_eq!(link(&e, other, 0), 0);
    }

    #[test]
    fn mt_vector_deleting_destructor_handles_arrays_and_single_objects() {
        let mut e = mt_engine();
        let block = e.mem.alloc(4 + 2 * 0x34);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        assert_eq!(e.call(0x00a3_5b90, &args![array, 3u32]).u32(), block);
        assert_eq!(
            mt_calls_to(&e, BLEND_INTERPOLATOR_DESTRUCTOR),
            vec![vec![array + 0x34], vec![array]]
        );
        assert_eq!(mt_calls_to(&e, ARRAY_SIZED_DELETE), vec![vec![block, 0x34]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_5b90, &args![array, 1u32]).u32(), array);
        assert_eq!(
            mt_calls_to(&e, BLEND_INTERPOLATOR_DESTRUCTOR),
            vec![vec![array]]
        );
        assert_eq!(mt_calls_to(&e, SIZED_DELETE), vec![vec![array, 0x34]]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_5b90, &args![array, 0u32]);
        assert!(mt_calls_to(&e, SIZED_DELETE).is_empty());
    }

    #[test]
    fn mt_set_link_stores_the_link_and_flags_the_target() {
        let mut e = Engine::new();
        let this = mt_with_links(&mut e, 2);
        let target = e.mem.alloc(0x9c);
        e.mem.set_u32(target + AV_FLAGS, 0x3);
        e.call(0x00a3_5f70, &args![this, 1u32, target, 7u32, 0x8000u32]);
        let item = link(&e, this, 1);
        assert_eq!(e.mem.u32(item + LINK_TARGET), target);
        assert_eq!(e.mem.u32(item + LINK_INFO), 0x8000);
        assert_eq!(e.mem.u16(item + LINK_BLEND_INDEX), 7);
        assert_eq!(e.mem.u32(target + AV_FLAGS), 0x83);
        e.call(0x00a3_5f70, &args![this, 0u32, 0u32, 1u32, 0u32]);
        assert_eq!(e.mem.u16(link(&e, this, 0) + LINK_BLEND_INDEX), 1);
    }

    #[test]
    fn mt_scalar_deleting_destructor_deletes_only_with_bit_0() {
        let mut e = mt_engine();
        let this = mt_with_links(&mut e, 0);
        assert_eq!(e.call(0x00a3_6050, &args![this, 0u32]).u32(), this);
        assert_eq!(
            mt_calls_to(&e, TIME_CONTROLLER_DESTRUCTOR),
            vec![vec![this]]
        );
        assert!(mt_calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a3_6050, &args![this, 1u32]);
        assert_eq!(mt_calls_to(&e, SIZED_DELETE), vec![vec![this, 0x44]]);
    }

    #[test]
    fn mt_update_moves_the_matching_flagged_targets() {
        const UPDATE: u32 = 0x0300_0000;
        let mut e = mt_engine();
        e.map(FRESH_TRANSLATE, 0xc);
        e.map(FRESH_ROTATE, 0x10);
        e.map(FRESH_SCALE, 4);
        e.set_global(FRESH_SCALE, 1.0f32);
        // Interpolator 0 answers true, interpolator 1 false.
        e.register(UPDATE, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(QUATERNION_SLERP, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = mt_with_links(&mut e, 3);
        e.mem.set_u8(this + FLAGS, 0x8);
        let vtable = e.mem.alloc(0x90);
        e.mem.set_u32(vtable + UPDATE_TRANSFORM_SLOT, UPDATE);
        let interps = e.mem.alloc(2 * 0x34);
        for i in 0..2 {
            e.mem.set_u32(interps + i * 0x34, vtable);
            e.mem.set_u32(interps + i * 0x34 + 8, i);
            e.mem.set_f32(interps + i * 0x34 + BLEND_CURR_LERP, 0.25);
        }
        e.mem.set_u32(this + BLEND_INTERPS, interps);
        e.mem.set_u16(this + NUM_INTERPS, 2);
        let info = e.mem.alloc(0x44);
        for i in 0..3 {
            e.mem.set_f32(info + 4 * i, 4.0);
            e.mem.set_f32(info + 0x20 + 4 * i, 8.0);
        }
        // Link 0: interpolator 0, flagged target (moves); link 1: interpolator
        // 0 without the flag; link 2: interpolator 1.
        let mut targets = vec![];
        for (i, (blend, flags)) in [(0u16, 2u32), (0, 0), (1, 2)].iter().enumerate() {
            let target = e.mem.alloc(0x9c);
            e.mem.set_u32(target + AV_FLAGS, *flags);
            let item = link(&e, this, i as u32);
            e.mem.set_u16(item + LINK_BLEND_INDEX, *blend);
            e.mem.set_u32(item + LINK_TARGET, target);
            e.mem.set_u32(item + LINK_INFO, info);
            targets.push(target);
        }
        let data = e.mem.alloc(0xc);
        e.mem.set_f32(data, 0.5);
        e.call(0x00a3_5c00, &args![this, data]);
        let updates = mt_calls_to(&e, UPDATE);
        assert_eq!(updates.len(), 2);
        assert_eq!((updates[0][1], updates[0][2]), (0.5f32.to_bits(), 0));
        assert_eq!(e.mem.f32(updates[0][3] + 0x1c), 1.0);
        // 4 * 0.75 + 8 * 0.25 = 5.
        for i in 0..3 {
            assert_eq!(e.mem.f32(targets[0] + AV_LOCAL_TRANSLATE + 4 * i), 5.0);
            assert_eq!(e.mem.f32(targets[1] + AV_LOCAL_TRANSLATE + 4 * i), 0.0);
            assert_eq!(e.mem.f32(targets[2] + AV_LOCAL_TRANSLATE + 4 * i), 0.0);
        }
        let slerps = mt_calls_to(&e, QUATERNION_SLERP);
        assert_eq!(slerps.len(), 1);
        assert_eq!(slerps[0][1], 0.75f32.to_bits());
        assert_eq!(
            mt_calls_to(&e, QUATERNION_TO_ROTATION),
            vec![vec![slerps[0][0], targets[0] + AV_LOCAL_ROTATE]]
        );
        // Without bit 3 nothing is updated.
        e.mem.set_u8(this + FLAGS, 0x7);
        e.call_log = Some(vec![]);
        e.call(0x00a3_5c00, &args![this, data]);
        assert!(mt_calls_to(&e, UPDATE).is_empty());
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::calls;

    const VTABLE: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    /// An engine with a test vtable (every slot a double answering 1) and
    /// the "no link" value 0xFFFF.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x40 {
            e.register(slot(4 * i), |_, _| 1u32.into_ret());
        }
        e.map(0x0109_6000, 0x1000);
        e.mem.set_u16(INVALID_LINK_INDEX, 0xffff);
        e
    }

    /// A controller with `links` empty links (each a zeroed `LinkAVObj`).
    fn controller(e: &mut Engine, links: u16) -> Ptr {
        let this = e.mem.alloc(MULTI_TARGET_SIZE);
        let array = e.mem.alloc(4 * u32::from(links).max(1));
        for i in 0..u32::from(links) {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(array + 4 * i, item);
        }
        e.mem.set_u32(this + LINKS, array);
        e.mem.set_u16(this + NUM_LINKS, links);
        Ptr::new(this)
    }

    /// A target object named by the string at `name`.
    fn target(e: &mut Engine, name: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + TARGET_NAME, name);
        object
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        for (i, b) in s.bytes().enumerate() {
            e.mem.set_u8(at + i as u32, b);
        }
        at
    }

    #[test]
    fn interpolator_is_correct_type_asks_the_interpolator() {
        let mut e = engine();
        let interp = e.mem.alloc(8);
        e.mem.set_u32(interp, VTABLE);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00a2_fac0, &args![0x10u32, interp, 3u32]).bool();
        assert!(answer);
        assert_eq!(
            calls(&e, slot(SLOT_IS_TRANSFORM_VALUE_SUPPORTED)),
            vec![vec![interp]]
        );
        e.register(slot(SLOT_IS_TRANSFORM_VALUE_SUPPORTED), |_, _| {
            0u32.into_ret()
        });
        assert!(!e.call(0x00a2_fac0, &args![0x10u32, interp, 3u32]).bool());
    }

    #[test]
    fn get_interpolator_count_reads_the_count() {
        let mut e = engine();
        let this = controller(&mut e, 0);
        e.mem.set_u16(this.addr() + NUM_INTERPS, 5);
        assert_eq!(e.call(0x00a3_5890, &args![this]).u16(), 5);
    }

    #[test]
    fn guarantee_time_range_reaches_every_blend_interpolator() {
        let mut e = engine();
        let this = controller(&mut e, 0);
        let interps = e.mem.alloc(2 * BLEND_INTERPOLATOR_SIZE);
        for i in 0..2 {
            e.mem.set_u32(interps + i * BLEND_INTERPOLATOR_SIZE, VTABLE);
        }
        e.mem.set_u32(this.addr() + BLEND_INTERPS, interps);
        e.mem.set_u16(this.addr() + NUM_INTERPS, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a3_58b0, &args![this, 0x7f80_0001u32, 2.0f32]);
        let second = interps + BLEND_INTERPOLATOR_SIZE;
        let two = 2.0f32.to_bits();
        assert_eq!(
            calls(&e, slot(SLOT_GUARANTEE_TIME_RANGE)),
            vec![
                vec![interps, 0x7fc0_0001, two],
                vec![second, 0x7fc0_0001, two]
            ]
        );
    }

    #[test]
    fn create_blend_interpolator_answers_the_element() {
        let mut e = engine();
        let this = controller(&mut e, 0);
        e.mem.set_u32(this.addr() + BLEND_INTERPS, 0x1000);
        let words = args![this, 3u32, 9u32, 9u32, 9u32, 9u32];
        assert_eq!(e.call(0x00a3_5900, &words).u32(), 0x1000 + 3 * 0x34);
    }

    #[test]
    fn create_object_builds_an_empty_controller() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR, |e, a| {
            for i in 0..0x11 {
                e.mem.set_u32(a[0] + 4 * i, 0xcdcd_cdcd);
            }
            Ret::default()
        });
        let object = e.call(0x00a3_59d0, &args![]).u32();
        assert_eq!(e.mem.u32(object), MULTI_TARGET_VTABLE);
        assert_eq!(e.mem.u32(object + BLEND_INTERPS), 0);
        assert_eq!(e.mem.u16(object + NUM_INTERPS), 0);
        assert_eq!(e.mem.u32(object + LINKS), 0);
        assert_eq!(e.mem.u16(object + NUM_LINKS), 0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a3_59d0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_members_even_without_an_object() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a3_5a10, &args![0x500u32, 0x600u32]).u32();
        assert_eq!(e.mem.u32(clone), MULTI_TARGET_VTABLE);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![0x500, clone, 0x600]]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_5a10, &args![0x500u32, 0x600u32]).u32(), 0);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![0x500, 0, 0x600]]);
        assert!(calls(&e, BASE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn get_interpolator_id_is_the_target_name() {
        let mut e = engine();
        let this = controller(&mut e, 2);
        let object = target(&mut e, 0x4321);
        e.call(0x00a3_5fc0, &args![this, 1u32, object]);
        assert_eq!(e.call(0x00a3_5e90, &args![this, 1u32]).u32(), 0x4321);
    }

    #[test]
    fn get_interpolator_index_compares_names() {
        let mut e = engine();
        let this = controller(&mut e, 3);
        let (bip, spine, query) = (
            text(&mut e, "Bip"),
            text(&mut e, "Spine1"),
            text(&mut e, "Spine1"),
        );
        let first = target(&mut e, bip);
        let second = target(&mut e, spine);
        e.call(0x00a3_5fc0, &args![this, 0u32, first]);
        e.call(0x00a3_5fc0, &args![this, 2u32, second]);
        assert_eq!(e.call(0x00a3_5eb0, &args![this, query]).u16(), 2);
        let other = text(&mut e, "Spine2");
        assert_eq!(e.call(0x00a3_5eb0, &args![this, other]).u16(), 0xffff);
        assert_eq!(e.call(0x00a3_5eb0, &args![this, 0u32]).u16(), 0xffff);
    }

    #[test]
    fn get_interpolator_index_fx_compares_handles_over_the_interp_count() {
        let mut e = engine();
        let this = controller(&mut e, 3);
        let object = target(&mut e, 0x777);
        e.call(0x00a3_5fc0, &args![this, 2u32, object]);
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, 0x777);
        e.mem.set_u16(this.addr() + NUM_INTERPS, 2);
        assert_eq!(e.call(0x00a3_5f30, &args![this, handle]).u16(), 0xffff);
        e.mem.set_u16(this.addr() + NUM_INTERPS, 3);
        assert_eq!(e.call(0x00a3_5f30, &args![this, handle]).u16(), 2);
    }

    #[test]
    fn set_target_marks_a_target() {
        let mut e = engine();
        let this = controller(&mut e, 1);
        let object = target(&mut e, 0);
        e.mem.set_u32(object + AV_FLAGS, 1);
        e.call(0x00a3_5fc0, &args![this, 0u32, object]);
        assert_eq!(e.mem.u32(object + AV_FLAGS), 0x81);
        e.call(0x00a3_5fc0, &args![this, 0u32, 0u32]);
        let item = link(&e, this.addr(), 0);
        assert_eq!(e.mem.u32(item + LINK_TARGET), 0);
    }

    #[test]
    fn add_link_fills_the_first_free_link() {
        let mut e = engine();
        let this = controller(&mut e, 2);
        let busy = target(&mut e, 0);
        e.call(0x00a3_5fc0, &args![this, 0u32, busy]);
        let object = target(&mut e, 0);
        let index = e
            .call(0x00a3_5fe0, &args![this, object, 5u32, 0x900u32])
            .u16();
        assert_eq!(index, 1);
        let item = link(&e, this.addr(), 1);
        assert_eq!(e.mem.u32(item + LINK_TARGET), object);
        assert_eq!(e.mem.u32(item + LINK_INFO), 0x900);
        assert_eq!(e.mem.u16(item + LINK_BLEND_INDEX), 5);
        assert_eq!(e.mem.u32(object + AV_FLAGS), 0x80);
        let more = target(&mut e, 0);
        assert_eq!(
            e.call(0x00a3_5fe0, &args![this, more, 1u32, 0u32]).u16(),
            0xffff
        );
    }
}
