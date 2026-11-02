//! `NiAnimation/niuvcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_decrement};
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};
use crate::units::nimain::nitimecontroller::NiTimeController;

/// `NiUVController::ms_RTTI` (the class's `NiRTTI` record).
const NI_UV_CONTROLLER_RTTI: u32 = 0x011f_3f18;

// Translated from 00a45b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_uv_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_UV_CONTROLLER_RTTI)
}

// Translated from 00a46000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a45b60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x50 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_uv_controller_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_5b60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x50u32]);
    }
    this
}

layout! {
    /// `NiUVController` (Xbox PDB), 0x50 bytes on PC as on the Xbox (offsets checked against
    /// the PC code): an `NiTimeController` (`crate::units::nimain::nitimecontroller`), the
    /// last key index of each UV list, the texture set and the data.
    pub struct NiUVController: 0x50 {
        /// `m_uiLastUOffsetIdx` (Xbox PDB).
        0x34 m_uiLastUOffsetIdx: u32,
        /// `m_uiLastUTilingIdx` (Xbox PDB).
        0x38 m_uiLastUTilingIdx: u32,
        /// `m_uiLastVOffsetIdx` (Xbox PDB).
        0x3C m_uiLastVOffsetIdx: u32,
        /// `m_uiLastVTilingIdx` (Xbox PDB).
        0x40 m_uiLastVTilingIdx: u32,
        /// `m_usTextureSet` (Xbox PDB).
        0x44 m_usTextureSet: u16,
        /// `m_spUVData` (Xbox PDB): `NiPointer<NiUVData>`.
        0x48 m_spUVData: u32,
        /// `m_bNeedPreDisplay` (Xbox PDB).
        0x4C m_bNeedPreDisplay: u8,
    }
}

/// `NiUVController`'s vtable (`01098064`).
const NI_UV_CONTROLLER_VTABLE: u32 = 0x0109_8064;
/// `NiTimeController`'s constructor (`00a6d220`, `thiscall`), destructor body (`00a6d290`),
/// `LinkObject` (`00a6d500`), `RegisterStreamables` (`00a6d0f0`), `IsEqual` (`00a6cc80`),
/// `SetTarget` (`00a6d450`), `LoadBinary` (`00a6d5e0`), `SaveBinary` (`00a6d730`),
/// `GetViewerStrings` (`00a6d830`), each with one stack word but the constructor and
/// destructor, and its member copy (`00a6cfe0`, (clone, cloning process)).
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `NiStream::GetObjectFromLinkID` (`00a63430`) and the link-id read (`00a645b0`), both
/// `thiscall` on the stream with no stack words.
const STREAM_GET_OBJECT_FROM_LINK: u32 = 0x00a6_3430;
const STREAM_READ_LINK: u32 = 0x00a6_45b0;
/// The game's `operator new` (`00aa13e0`, `cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiFloatKey::GenInterp` (`cdecl`, (time, keys, type, count, &last index, size); answers in
/// `ST0`).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiGeometryData::GetTextureSet` (`00a66960`, `thiscall`, the set; answers its UV pairs) and
/// `NiGeometryData::MarkAsChanged` (`00a67090`, `thiscall`, the flags).
const GEOMETRY_GET_TEXTURE_SET: u32 = 0x00a6_6960;
const GEOMETRY_MARK_AS_CHANGED: u32 = 0x00a6_7090;
/// The target's geometry data (`NiGeometry::m_spModelData`, +0xB8 in the PC code), the data's
/// vertex count (`NiGeometryData::m_usVertices`, +8) and flags word (`m_usDataFlags`, +0xC).
const TARGET_MODEL_DATA: u32 = 0xb8;
const DATA_VERTICES: u32 = 8;
const DATA_FLAGS: u32 = 0xc;
/// `0.5` (`double`, `01011588`).
const HALF: u32 = 0x0101_1588;
/// The key-time bounds: the floats a range starts from (`FLT_MAX`, `01016970`; `-FLT_MAX`,
/// `01015f5c`) and the doubles the first list is compared with (`FLT_MAX`, `010231b0`;
/// `-FLT_MAX`, `010241b0`).
const LO_START: u32 = 0x0101_6970;
const HI_START: u32 = 0x0101_5f5c;
const LO_LIMIT: u32 = 0x0102_31b0;
const HI_LIMIT: u32 = 0x0102_41b0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the text) and the viewer-string array's grow
/// (`0096ad30`, `thiscall`, the new size) and set (`0096ae90`, `thiscall`, (index, &text)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The UV data's key list `channel` (U offset, V offset, U tiling, V tiling): its count (+8 +
/// 0xc * channel), keys and type after it and its size byte (+0x48 + channel).
fn data_channel(channel: u32) -> (u32, u32, u32, u32) {
    let count = 8 + 0xc * channel;
    (count, count + 4, count + 8, 0x48 + channel)
}

/// The shape of the four key getters: with data, the list's count, type and size byte through
/// the three pointers and its keys as the answer; without, zeros and null.
fn channel_keys(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    channel: u32,
    count: Ptr,
    kind: Ptr,
    size: Ptr,
) -> u32 {
    let data = e.get(this, NiUVController::m_spUVData);
    if data == 0 {
        e.mem.set_u32(count.addr(), 0);
        e.mem.set_u32(kind.addr(), 0);
        e.mem.set_u8(size.addr(), 0);
        return 0;
    }
    let (count_at, keys_at, type_at, size_at) = data_channel(channel);
    let word = e.mem.u32(data + count_at);
    e.mem.set_u32(count.addr(), word);
    let word = e.mem.u32(data + type_at);
    e.mem.set_u32(kind.addr(), word);
    let byte = e.mem.u8(data + size_at);
    e.mem.set_u8(size.addr(), byte);
    e.mem.u32(data + keys_at)
}

/// The construction `CreateObject` and `CreateClone` share on a new block: the
/// `NiTimeController` constructor, the class's vtable and every member of its own empty.
fn construct(e: &mut Engine, block: u32) {
    let this = Ptr::<NiUVController>::new(block);
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_UV_CONTROLLER_VTABLE);
    e.set(this, NiUVController::m_spUVData, 0);
    e.set(this, NiUVController::m_usTextureSet, 0);
    e.set(this, NiUVController::m_uiLastUOffsetIdx, 0);
    e.set(this, NiUVController::m_uiLastVOffsetIdx, 0);
    e.set(this, NiUVController::m_uiLastUTilingIdx, 0);
    e.set(this, NiUVController::m_uiLastVTilingIdx, 0);
    e.set(this, NiUVController::m_bNeedPreDisplay, 0);
}

/// Releases the UV data when there is one (`InterlockedDecrement`; slot +4 at zero).
fn release_data(e: &mut Engine, this: Ptr<NiUVController>) {
    let data = e.get(this, NiUVController::m_spUVData);
    if data != 0 && interlocked_decrement(e, data + 4) == 0 {
        e.vcall(data, 4, &args![]);
    }
}

// Translated from 00a45910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::LinkObject` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::LinkObject` (`00a6d500`), then the UV data takes the stream's next
/// linked object (`NiStream::GetObjectFromLinkID`, `00a63430`; the counted `NiPointer`
/// assignment).
pub fn ni_uv_controller_link_object(e: &mut Engine, this: Ptr<NiUVController>, stream: u32) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(STREAM_GET_OBJECT_FROM_LINK, &args![stream]).u32();
    assign_ref_counted(e, this.addr() + 0x48, object);
}

// Translated from 00a45970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false when
/// `NiTimeController::RegisterStreamables` (`00a6d0f0`) is; otherwise the data's own (its slot
/// +0x54) when there is data, and true.
pub fn ni_uv_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    stream: u32,
) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.get(this, NiUVController::m_spUVData);
    if data != 0 {
        e.vcall(data, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a459a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::IsEqual` (Xbox PDB; `thiscall`, the other object): equal when
/// `NiTimeController::IsEqual` (`00a6cc80`) is, the texture sets are equal, both or neither
/// have data, and with data the data is equal (its slot +0x5c).
pub fn ni_uv_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    other: Ptr<NiUVController>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiUVController::m_usTextureSet) != e.get(other, NiUVController::m_usTextureSet) {
        return false;
    }
    let theirs = e.get(other, NiUVController::m_spUVData);
    let mine = e.get(this, NiUVController::m_spUVData);
    if (mine == 0) != (theirs == 0) {
        return false;
    }
    mine == 0 || e.vcall(mine, 0x5c, &args![theirs]).bool()
}

// Translated from 00a459f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (&count, &type, &size)): the data's U offset keys
/// (count +8, type +0x10, size byte +0x48 through the pointers; the keys +0xc as the answer),
/// or zeros and null without data.
pub fn fn_00a459f0(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    count: Ptr,
    kind: Ptr,
    size: Ptr,
) -> u32 {
    channel_keys(e, this, 0, count, kind, size)
}

// Translated from 00a45a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (&count, &type, &size)): as `00a459f0` for the V
/// offset keys (+0x14, +0x1c, +0x49; keys +0x18).
pub fn fn_00a45a40(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    count: Ptr,
    kind: Ptr,
    size: Ptr,
) -> u32 {
    channel_keys(e, this, 1, count, kind, size)
}

// Translated from 00a45a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (&count, &type, &size)): as `00a459f0` for the U
/// tiling keys (+0x20, +0x28, +0x4a; keys +0x24).
pub fn fn_00a45a90(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    count: Ptr,
    kind: Ptr,
    size: Ptr,
) -> u32 {
    channel_keys(e, this, 2, count, kind, size)
}

// Translated from 00a45ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (&count, &type, &size)): as `00a459f0` for the V
/// tiling keys (+0x2c, +0x34, +0x4b; keys +0x30).
pub fn fn_00a45ae0(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    count: Ptr,
    kind: Ptr,
    size: Ptr,
) -> u32 {
    channel_keys(e, this, 3, count, kind, size)
}

// Translated from 00a45b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::SetTarget` (Xbox PDB; `thiscall`, the target): the data's last values
/// (+0x38..+0x44; the data is not tested for null) reset to offsets 0 and tilings 1, then a
/// tail jump to `NiTimeController::SetTarget` (`00a6d450`) with the target.
pub fn ni_uv_controller_set_target(e: &mut Engine, this: Ptr<NiUVController>, target: u32) {
    let data = e.get(this, NiUVController::m_spUVData);
    let one = x87_store32(1.0);
    e.mem.set_u32(data + 0x38, 0);
    e.mem.set_u32(data + 0x3c, 0);
    e.mem.set_u32(data + 0x40, one);
    e.mem.set_u32(data + 0x44, one);
    e.call(TIME_CONTROLLER_SET_TARGET, &args![this, target]);
}

// Translated from 00a45b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its body the `NiUVController`
/// destructor body): the class's vtable, the data released and cleared, the (now null) data
/// pointer's release run again as the member destructor, then a tail jump to the
/// `NiTimeController` destructor body (`00a6d290`).
pub fn fn_00a45b60(e: &mut Engine, this: Ptr<NiUVController>) {
    e.mem.set_u32(this.addr(), NI_UV_CONTROLLER_VTABLE);
    if e.get(this, NiUVController::m_spUVData) != 0 {
        release_data(e, this);
        e.set(this, NiUVController::m_spUVData, 0);
    }
    release_data(e, this);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00a45bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its body the key time range):
/// the low and high key times start at `FLT_MAX` and `-FLT_MAX` (`01016970`, `01015f5c`);
/// with data, the first U offset key's time becomes the low time when below the double
/// `FLT_MAX` and the last's the high time when above the double `-FLT_MAX`; for the other
/// three lists, a first key time below the low time and a last key time above the high time
/// replace them. Lists without keys are skipped; the last key is at `size * (count - 1)`.
pub fn fn_00a45bc0(e: &mut Engine, this: Ptr<NiUVController>) {
    let controller = Ptr::<NiTimeController>::new(this.addr());
    let data = e.get(this, NiUVController::m_spUVData);
    let lo = x87_quiet_bits(e.mem.u32(LO_START));
    e.set(controller, NiTimeController::m_fLoKeyTime, lo);
    let hi = x87_quiet_bits(e.mem.u32(HI_START));
    e.set(controller, NiTimeController::m_fHiKeyTime, hi);
    for channel in 0..4 {
        if data == 0 {
            return;
        }
        let (count_at, keys_at, _, size_at) = data_channel(channel);
        let keys = e.mem.u32(data + keys_at);
        let count = e.mem.u32(data + count_at);
        let size = u32::from(e.mem.u8(data + size_at));
        if keys == 0 {
            continue;
        }
        let first = x87_load32(e.mem.u32(keys));
        let take_first = if channel == 0 {
            first < x87_load64(e.mem.u64(LO_LIMIT))
        } else {
            x87_load32(e.get(controller, NiTimeController::m_fLoKeyTime)) > first
        };
        if take_first {
            let word = x87_quiet_bits(e.mem.u32(keys));
            e.set(controller, NiTimeController::m_fLoKeyTime, word);
        }
        let last = keys.wrapping_add(size.wrapping_mul(count.wrapping_sub(1)));
        let last_time = x87_load32(e.mem.u32(last));
        let take_last = if channel == 0 {
            last_time > x87_load64(e.mem.u64(HI_LIMIT))
        } else {
            x87_load32(e.get(controller, NiTimeController::m_fHiKeyTime)) < last_time
        };
        if take_last {
            let word = x87_quiet_bits(e.mem.u32(last));
            e.set(controller, NiTimeController::m_fHiKeyTime, word);
        }
    }
}

// Translated from 00a45cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::Update` (Xbox PDB; `thiscall`, a pointer to the time): unless
/// `NiTimeController::DontDoUpdate` (`00a36250`, the time through the x87 stack) says so, the
/// controller asks for a pre-display pass (`m_bNeedPreDisplay` = 1).
pub fn ni_uv_controller_update(e: &mut Engine, this: Ptr<NiUVController>, time: Ptr) {
    let t = x87_quiet_bits(e.mem.u32(time.addr()));
    if !e.call(DONT_DO_UPDATE, &args![this, t]).bool() {
        e.set(this, NiUVController::m_bNeedPreDisplay, 1);
    }
}

// Translated from 00a45d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::OnPreDisplay` (Xbox PDB; `thiscall`, no stack words): when a pass is
/// pending and the texture set is below the geometry data's set count (its flags word +0xC,
/// bit 0), the four UV values at the scaled time (+0x28) are interpolated
/// (`NiFloatKey::GenInterp` on each list from `00a459f0`..`00a45ae0`, with the controller's
/// last index; offsets default to 0 and tilings to 1 without keys). With the data's last
/// values (+0x38..+0x44), `a = u tiling / last u tiling` and `b = v tiling / last v tiling`,
/// each texture coordinate of the set (`GetTextureSet`, one pair per vertex) becomes `u * a +
/// ((1 - a) * 0.5 - (u offset - last u offset * a))` and `v * b + ((1 - b) * 0.5 + (v offset
/// - last v offset * b))`; the new values become the data's last values, the geometry data is
/// marked changed (`MarkAsChanged(8)`) and the pass done. Every x87 operation and float
/// temporary as the exe does them.
pub fn ni_uv_controller_on_pre_display(e: &mut Engine, this: Ptr<NiUVController>) {
    if e.get(this, NiUVController::m_bNeedPreDisplay) == 0 {
        return;
    }
    let target = e.get(
        Ptr::<NiTimeController>::new(this.addr()),
        NiTimeController::m_pkTarget,
    );
    let geometry = e.mem.u32(target + TARGET_MODEL_DATA);
    let sets = e.mem.u16(geometry + DATA_FLAGS) & 1;
    if e.get(this, NiUVController::m_usTextureSet) >= sets {
        return;
    }
    let one = x87_store32(1.0);
    // The stack locals from ESP-0x24 up (offsets into the block): the key size, count and
    // type words the getters fill (0, 4, 8), the V offset (0xc), U tiling (0x10), V tiling
    // (0x14) and U offset (0x18), then two temporaries (0x1c, 0x20).
    e.with_stack(0x24, |e, locals| {
        let l = locals.addr();
        let (size_word, count_word, type_word) = (l, l + 4, l + 8);
        let (v_offset, u_tiling, v_tiling, u_offset) = (l + 0xc, l + 0x10, l + 0x14, l + 0x18);
        let (v_translate, v_scale) = (l + 0x1c, l + 0x20);
        type Getter = fn(&mut Engine, Ptr<NiUVController>, Ptr, Ptr, Ptr) -> u32;
        let channels: [(Getter, u32, u32, u32); 4] = [
            (fn_00a459f0, u_offset, 0x34, 0),
            (fn_00a45a40, v_offset, 0x3c, 0),
            (fn_00a45a90, u_tiling, 0x38, one),
            (fn_00a45ae0, v_tiling, 0x40, one),
        ];
        for (getter, value, last_index, default) in channels {
            e.mem.set_u32(value, default);
            let keys = getter(
                e,
                this,
                Ptr::new(count_word),
                Ptr::new(type_word),
                Ptr::new(size_word),
            );
            if keys != 0 {
                let time = x87_quiet_bits(e.mem.u32(this.addr() + 0x28));
                let words = args![
                    time,
                    keys,
                    e.mem.u32(type_word),
                    e.mem.u32(count_word),
                    this.addr() + last_index,
                    e.mem.u32(size_word)
                ];
                let result = e.call(FLOAT_KEY_GEN_INTERP, &words).f64();
                e.mem.set_u32(value, x87_store32(result));
            }
        }
        let data = e.get(this, NiUVController::m_spUVData);
        let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
        let m32 = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
        let a = x87_store32(x87_div(ld(e, u_tiling), m32(e, data + 0x40)));
        e.mem.set_u32(type_word, a);
        let word = x87_quiet_bits(e.mem.u32(data + 0x38));
        e.mem.set_u32(count_word, word);
        let b = x87_store32(x87_div(ld(e, v_tiling), m32(e, data + 0x44)));
        e.mem.set_u32(v_scale, b);
        let word = x87_quiet_bits(e.mem.u32(data + 0x3c));
        e.mem.set_u32(size_word, word);
        let set = u32::from(e.get(this, NiUVController::m_usTextureSet));
        let mut pair = e
            .call(GEOMETRY_GET_TEXTURE_SET, &args![geometry, set])
            .u32();
        let a = ld(e, type_word);
        let vertices = e.mem.u16(geometry + DATA_VERTICES);
        let half = x87_load64(e.mem.u64(HALF));
        let scaled_half_a = x87_mul(x87_sub(1.0, a), half);
        let u = ld(e, u_offset);
        let shifted = x87_store32(x87_sub(u, x87_mul(ld(e, count_word), a)));
        e.mem.set_u32(u_offset, shifted);
        let u_translate = x87_store32(x87_sub(scaled_half_a, ld(e, u_offset)));
        e.mem.set_u32(u_offset, u_translate);
        let b = ld(e, v_scale);
        let shifted = x87_store32(x87_sub(m32(e, v_offset), x87_mul(ld(e, size_word), b)));
        e.mem.set_u32(v_scale, shifted);
        let translate = x87_add(x87_mul(half, x87_sub(1.0, b)), m32(e, v_scale));
        e.mem.set_u32(v_translate, x87_store32(translate));
        if vertices != 0 {
            let u_add = ld(e, u_offset);
            let v_add = ld(e, v_translate);
            for _ in 0..vertices {
                let s = x87_add(x87_mul(ld(e, pair), a), u_add);
                e.mem.set_u32(pair, x87_store32(s));
                let t = x87_add(x87_mul(ld(e, pair + 4), b), v_add);
                e.mem.set_u32(pair + 4, x87_store32(t));
                pair = pair.wrapping_add(8);
            }
        }
        let data = e.get(this, NiUVController::m_spUVData);
        e.mem.set_u32(data + 0x38, x87_store32(u));
        let word = x87_quiet_bits(e.mem.u32(v_offset));
        e.mem.set_u32(data + 0x3c, word);
        let word = x87_quiet_bits(e.mem.u32(u_tiling));
        e.mem.set_u32(data + 0x40, word);
        let word = x87_quiet_bits(e.mem.u32(v_tiling));
        e.mem.set_u32(data + 0x44, word);
        e.call(GEOMETRY_MARK_AS_CHANGED, &args![geometry, 8u32]);
        e.set(this, NiUVController::m_bNeedPreDisplay, 0);
    });
}

// Translated from 00a45f50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (no arguments; by its body the creation function of
/// `NiUVController`): 0x50 bytes from `operator new`; when that succeeds, the
/// `NiTimeController` constructor, the class's vtable and its own members empty. Answers
/// the object, or null.
pub fn fn_00a45f50(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    construct(e, block);
    block
}

// Translated from 00a45fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the data; by its body the data setter): the data
/// pointer takes `data` (the counted `NiPointer` assignment), the four last key indices are
/// reset and the key time range is recomputed (`00a45bc0`).
pub fn fn_00a45fa0(e: &mut Engine, this: Ptr<NiUVController>, data: u32) {
    assign_ref_counted(e, this.addr() + 0x48, data);
    e.set(this, NiUVController::m_uiLastUOffsetIdx, 0);
    e.set(this, NiUVController::m_uiLastVOffsetIdx, 0);
    e.set(this, NiUVController::m_uiLastUTilingIdx, 0);
    e.set(this, NiUVController::m_uiLastVTilingIdx, 0);
    fn_00a45bc0(e, this);
}

// Translated from 00a46020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::LoadBinary` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::LoadBinary` (`00a6d5e0`), the 2-byte texture set read in place (the
/// stream argument's stack word is the byte-count word, set to 2) and the data's link id
/// (`00a645b0`).
pub fn ni_uv_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    stream: Ptr<NiStream>,
) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, word| {
        let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        e.mem.set_u32(word.addr(), 2);
        let done = e
            .call(
                function,
                &args![input, this.addr() + 0x44, 2u32, word, 1u32],
            )
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
    e.call(STREAM_READ_LINK, &args![stream]);
}

// Translated from 00a46070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::SaveBinary` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::SaveBinary` (`00a6d730`), the 2-byte texture set written (the stream
/// argument's stack word is the byte-count word, set to 2) and the data's link (the stream's
/// slot +0x30 with the data).
pub fn ni_uv_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    stream: Ptr<NiStream>,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, word| {
        let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
        let function = e.get(output, NiBinaryStream::m_pfnWrite);
        e.mem.set_u32(word.addr(), 2);
        let done = e
            .call(
                function,
                &args![output, this.addr() + 0x44, 2u32, word, 1u32],
            )
            .u32();
        let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            output,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
    let data = e.get(this, NiUVController::m_spUVData);
    e.vcall(stream.addr(), 0x30, &args![data]);
}

// Translated from 00a460c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::CreateClone` (Xbox PDB; `thiscall`, the cloning process): a new
/// controller (0x50 bytes from `operator new`, built as `00a45f50` builds one; null when that
/// fails) given the `NiTimeController` members (`00a6cfe0`, also run on null), this one's last
/// key indices and texture set, and this one's data through the data setter (`00a45fa0`) when
/// there is data. Answers the clone.
pub fn ni_uv_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    process: u32,
) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block != 0 {
        construct(e, block);
    }
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, block, process]);
    let clone = Ptr::<NiUVController>::new(block);
    for field in [
        NiUVController::m_uiLastUOffsetIdx,
        NiUVController::m_uiLastVOffsetIdx,
        NiUVController::m_uiLastUTilingIdx,
        NiUVController::m_uiLastVTilingIdx,
    ] {
        let word = e.get(this, field);
        e.set(clone, field, word);
    }
    let set = e.get(this, NiUVController::m_usTextureSet);
    e.set(clone, NiUVController::m_usTextureSet, set);
    let data = e.get(this, NiUVController::m_spUVData);
    if data != 0 {
        fn_00a45fa0(e, clone, data);
    }
    block
}

// Translated from 00a46150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVController::GetViewerStrings` (Xbox PDB; `thiscall`, the strings):
/// `NiTimeController::GetViewerStrings` (`00a6d830`), then the class name (the first word of
/// the `NiRTTI` record at `011f3f18`, through `NiGetViewerString`) appended (growing the array
/// to the step +0xE plus the size when the 16-bit size +0xA has reached the capacity +0x8; the
/// text goes in through the strings argument's stack word).
pub fn ni_uv_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiUVController>,
    strings: u32,
) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_UV_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a45b30, ni_uv_controller_get_rtti() -> Ptr),
        entry!(0x00a46000, ni_uv_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_5910,
            ni_uv_controller_link_object(Ptr<NiUVController>, u32)
        ),
        entry!(
            0x00a4_5970,
            ni_uv_controller_register_streamables(Ptr<NiUVController>, u32) -> bool
        ),
        entry!(
            0x00a4_59a0,
            ni_uv_controller_is_equal(Ptr<NiUVController>, Ptr<NiUVController>) -> bool
        ),
        entry!(
            0x00a4_59f0,
            fn_00a459f0(Ptr<NiUVController>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a4_5a40,
            fn_00a45a40(Ptr<NiUVController>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a4_5a90,
            fn_00a45a90(Ptr<NiUVController>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a4_5ae0,
            fn_00a45ae0(Ptr<NiUVController>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a4_5b40,
            ni_uv_controller_set_target(Ptr<NiUVController>, u32)
        ),
        entry!(0x00a4_5b60, fn_00a45b60(Ptr<NiUVController>)),
        entry!(0x00a4_5bc0, fn_00a45bc0(Ptr<NiUVController>)),
        entry!(
            0x00a4_5cf0,
            ni_uv_controller_update(Ptr<NiUVController>, Ptr)
        ),
        entry!(
            0x00a4_5d10,
            ni_uv_controller_on_pre_display(Ptr<NiUVController>)
        ),
        entry!(0x00a4_5f50, fn_00a45f50() -> u32),
        entry!(0x00a4_5fa0, fn_00a45fa0(Ptr<NiUVController>, u32)),
        entry!(
            0x00a4_6020,
            ni_uv_controller_load_binary(Ptr<NiUVController>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_6070,
            ni_uv_controller_save_binary(Ptr<NiUVController>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_60c0,
            ni_uv_controller_create_clone(Ptr<NiUVController>, u32) -> u32
        ),
        entry!(
            0x00a4_6150,
            ni_uv_controller_get_viewer_strings(Ptr<NiUVController>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_uv_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a45b30, &args![]).u32(), NI_UV_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_uv_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_5b60, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a46000, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a46000, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x50]]);
        assert_eq!(calls_to(&e, 0x00a4_5b60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    const LOG: u32 = 0x0300_0500;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Counting stand-ins for the imported `InterlockedIncrement` and `InterlockedDecrement`.
    fn counting(e: &mut Engine) {
        e.register(0x00fd_f1c0, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    /// A counted object (count 1) whose slot `slot` is the double `f`, deleting through slot
    /// +4 into the log.
    fn counted_object(e: &mut Engine, slot: u32, f: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, LOG);
        e.mem.set_u32(vtable + slot, f);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn link_object_takes_the_linked_data() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(LOG, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        let new = counted_object(&mut e, 0, 0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK, move |_, _| new.into_ret());
        let this = e.new_object::<NiUVController>();
        let old = counted_object(&mut e, 0, 0);
        e.set(this, NiUVController::m_spUVData, old);
        e.call_log = Some(vec![]);
        e.call(0x00a4_5910, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_LINK_OBJECT),
            vec![vec![this.addr(), 0x6000]]
        );
        assert_eq!(
            calls_to(&e, STREAM_GET_OBJECT_FROM_LINK),
            vec![vec![0x6000]]
        );
        assert_eq!(e.get(this, NiUVController::m_spUVData), new);
        assert_eq!((e.mem.u32(old + 4), e.mem.u32(new + 4)), (0, 2));
        assert_eq!(calls_to(&e, LOG), vec![vec![old]]);
    }

    #[test]
    fn register_streamables_reaches_the_data() {
        let mut e = Engine::new();
        e.register(LOG, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, a| {
            (a[1] == 7).into_ret()
        });
        let this = e.new_object::<NiUVController>();
        let data = counted_object(&mut e, 0x54, LOG);
        e.set(this, NiUVController::m_spUVData, data);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_5970, &args![this, 6u32]).bool());
        assert!(calls_to(&e, LOG).is_empty());
        assert!(e.call(0x00a4_5970, &args![this, 7u32]).bool());
        assert_eq!(calls_to(&e, LOG), vec![vec![data, 7]]);
    }

    #[test]
    fn is_equal_compares_the_set_and_the_data() {
        const DATA_EQUAL: u32 = 0x0300_0510;
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(DATA_EQUAL, |_, a| (a[1] != 0x4444).into_ret());
        let a = e.new_object::<NiUVController>();
        let b = e.new_object::<NiUVController>();
        let equal = |e: &mut Engine| e.call(0x00a4_59a0, &args![a, b]).bool();
        assert!(equal(&mut e));
        e.set(b, NiUVController::m_usTextureSet, 1);
        assert!(!equal(&mut e));
        e.set(b, NiUVController::m_usTextureSet, 0);
        let data = counted_object(&mut e, 0x5c, DATA_EQUAL);
        e.set(a, NiUVController::m_spUVData, data);
        assert!(!equal(&mut e));
        e.set(b, NiUVController::m_spUVData, 0x5555);
        assert!(equal(&mut e));
        e.set(b, NiUVController::m_spUVData, 0x4444);
        assert!(!equal(&mut e));
    }

    #[test]
    fn the_channel_getters_answer_each_list() {
        let mut e = Engine::new();
        let this = e.new_object::<NiUVController>();
        let data = e.mem.alloc(0x4c);
        for channel in 0..4 {
            let (count_at, keys_at, type_at, size_at) = data_channel(channel);
            e.mem.set_u32(data + count_at, 1 + channel);
            e.mem.set_u32(data + keys_at, 0x7000 + channel);
            e.mem.set_u32(data + type_at, 0x10 + channel);
            e.mem.set_u8(data + size_at, 0x20 + channel as u8);
        }
        e.set(this, NiUVController::m_spUVData, data);
        let out = e.mem.alloc(0xc);
        for (channel, addr) in [0x00a4_59f0u32, 0x00a4_5a40, 0x00a4_5a90, 0x00a4_5ae0]
            .iter()
            .enumerate()
        {
            let channel = channel as u32;
            let keys = e.call(*addr, &args![this, out, out + 4, out + 8]).u32();
            assert_eq!(keys, 0x7000 + channel);
            assert_eq!(
                (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u8(out + 8)),
                (1 + channel, 0x10 + channel, 0x20 + channel as u8)
            );
        }
        e.set(this, NiUVController::m_spUVData, 0);
        e.mem.set_u32(out + 8, 0xffff_ffff);
        assert_eq!(
            e.call(0x00a4_5ae0, &args![this, out, out + 4, out + 8])
                .u32(),
            0
        );
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
        // Only the size byte is written.
        assert_eq!(e.mem.u32(out + 8), 0xffff_ff00);
    }

    #[test]
    fn set_target_resets_the_last_values() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_SET_TARGET, |_, _| Ret::default());
        let this = e.new_object::<NiUVController>();
        let data = e.mem.alloc(0x4c);
        floats(&mut e, data + 0x38, &[5.0, 6.0, 7.0, 8.0]);
        e.set(this, NiUVController::m_spUVData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a4_5b40, &args![this, 0x9000u32]);
        assert_eq!(read(&e, data + 0x38, 4), vec![0.0, 0.0, 1.0, 1.0]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SET_TARGET),
            vec![vec![this.addr(), 0x9000]]
        );
    }

    #[test]
    fn fn_00a45b60_releases_the_data_once() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(LOG, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let this = e.new_object::<NiUVController>();
        let data = counted_object(&mut e, 0, 0);
        e.set(this, NiUVController::m_spUVData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a4_5b60, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_UV_CONTROLLER_VTABLE);
        assert_eq!(e.get(this, NiUVController::m_spUVData), 0);
        assert_eq!(e.mem.u32(data + 4), 0);
        assert_eq!(calls_to(&e, LOG), vec![vec![data]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_DESTRUCT),
            vec![vec![this.addr()]]
        );
    }

    fn range_constants(e: &mut Engine) {
        for at in [LO_START, HI_START] {
            e.map(at, 4);
        }
        for at in [LO_LIMIT, HI_LIMIT] {
            e.map(at, 8);
        }
        e.mem.set_f32(LO_START, f32::MAX);
        e.mem.set_f32(HI_START, f32::MIN);
        e.mem.set_u64(LO_LIMIT, f64::from(f32::MAX).to_bits());
        e.mem.set_u64(HI_LIMIT, f64::from(f32::MIN).to_bits());
    }

    #[test]
    fn fn_00a45bc0_spans_the_key_times() {
        let mut e = Engine::new();
        range_constants(&mut e);
        let this = e.new_object::<NiUVController>();
        let controller = Ptr::<NiTimeController>::new(this.addr());
        // Without data: the start values.
        e.call(0x00a4_5bc0, &args![this]);
        assert_eq!(
            e.get(controller, NiTimeController::m_fLoKeyTime),
            f32::MAX.to_bits()
        );
        assert_eq!(
            e.get(controller, NiTimeController::m_fHiKeyTime),
            f32::MIN.to_bits()
        );
        let data = e.mem.alloc(0x4c);
        e.set(this, NiUVController::m_spUVData, data);
        // U offset keys at 1 and 3 (8 bytes each); U tiling keys at 0.5 and 2; V tiling at
        // 2 .. 4; V offset none.
        let lists: [(u32, &[f32]); 3] = [
            (0, &[1.0, 0.0, 3.0, 0.0]),
            (2, &[0.5, 0.0, 2.0, 0.0]),
            (3, &[2.0, 0.0, 4.0, 0.0]),
        ];
        for (channel, keys) in lists {
            let (count_at, keys_at, _, size_at) = data_channel(channel);
            let at = e.mem.alloc(0x10);
            floats(&mut e, at, keys);
            e.mem.set_u32(data + count_at, 2);
            e.mem.set_u32(data + keys_at, at);
            e.mem.set_u8(data + size_at, 8);
        }
        e.call(0x00a4_5bc0, &args![this]);
        assert_eq!(
            e.get(controller, NiTimeController::m_fLoKeyTime),
            0.5f32.to_bits()
        );
        assert_eq!(
            e.get(controller, NiTimeController::m_fHiKeyTime),
            4.0f32.to_bits()
        );
    }

    #[test]
    fn update_asks_for_a_pass_unless_told_not_to() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, a| {
            (f32::from_bits(a[1]) > 1.0).into_ret()
        });
        let this = e.new_object::<NiUVController>();
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 2.0);
        e.call(0x00a4_5cf0, &args![this, time]);
        assert_eq!(e.get(this, NiUVController::m_bNeedPreDisplay), 0);
        e.mem.set_f32(time, 0.5);
        e.call(0x00a4_5cf0, &args![this, time]);
        assert_eq!(e.get(this, NiUVController::m_bNeedPreDisplay), 1);
    }

    /// A pending controller on a geometry with two vertices, its data's last values (0.25,
    /// 0.5, 2, 4) and the interpolations answering per key list (by its keys address).
    fn pre_display_engine() -> (Engine, Ptr<NiUVController>, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(HALF, 8);
        e.mem.set_u64(HALF, 0.5f64.to_bits());
        let this = e.new_object::<NiUVController>();
        e.set(this, NiUVController::m_bNeedPreDisplay, 1);
        e.mem.set_f32(this.addr() + 0x28, 1.5);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u16(geometry + DATA_VERTICES, 2);
        e.mem.set_u16(geometry + DATA_FLAGS, 1);
        let target = e.mem.alloc(0xc0);
        e.mem.set_u32(target + TARGET_MODEL_DATA, geometry);
        e.mem.set_u32(this.addr() + 0x2c, target);
        let data = e.mem.alloc(0x4c);
        floats(&mut e, data + 0x38, &[0.25, 0.5, 2.0, 4.0]);
        e.set(this, NiUVController::m_spUVData, data);
        let uv = e.mem.alloc(0x10);
        floats(&mut e, uv, &[1.0, 2.0, 3.0, 4.0]);
        e.register_double(GEOMETRY_GET_TEXTURE_SET, move |_, _| uv.into_ret());
        e.register(GEOMETRY_MARK_AS_CHANGED, |_, _| Ret::default());
        e.register(FLOAT_KEY_GEN_INTERP, |_, a| Ret {
            st0: f64::from(a[1] - 0x6ff0),
            ..Ret::default()
        });
        (e, this, data, uv, geometry)
    }

    #[test]
    fn on_pre_display_moves_the_coordinates() {
        let (mut e, this, data, uv, geometry) = pre_display_engine();
        // Keys for every list: U offset 0x7000 (16), V offset 0x7001 (17), U tiling 0x7002
        // (18), V tiling 0x7003 (19), so the values interpolate to 16, 17, 18 and 19.
        for channel in 0..4 {
            let (count_at, keys_at, type_at, size_at) = data_channel(channel);
            e.mem.set_u32(data + count_at, 1);
            e.mem.set_u32(data + keys_at, 0x7000 + channel);
            e.mem.set_u32(data + type_at, 2);
            e.mem.set_u8(data + size_at, 8);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a4_5d10, &args![this]);
        let interp = calls_to(&e, FLOAT_KEY_GEN_INTERP);
        let indices: Vec<u32> = interp.iter().map(|c| c[4] - this.addr()).collect();
        assert_eq!(indices, vec![0x34, 0x3c, 0x38, 0x40]);
        assert_eq!(interp[0][..4], [1.5f32.to_bits(), 0x7000, 2, 1]);
        assert_eq!(interp[0][5], 8);
        // a = 18 / 2 = 9, b = 19 / 4 = 4.75; u' = u * 9 + ((1 - 9) * 0.5 - (16 - 0.25 * 9)),
        // v' = v * 4.75 + ((1 - 4.75) * 0.5 + (17 - 0.5 * 4.75)).
        let a = 9.0f32;
        let b = 4.75f32;
        let ut = (1.0 - a) * 0.5 - (16.0 - 0.25 * a);
        let vt = (1.0 - b) * 0.5 + (17.0 - 0.5 * b);
        assert_eq!(
            read(&e, uv, 4),
            vec![a + ut, 2.0 * b + vt, 3.0 * a + ut, 4.0 * b + vt]
        );
        assert_eq!(read(&e, data + 0x38, 4), vec![16.0, 17.0, 18.0, 19.0]);
        assert_eq!(
            calls_to(&e, GEOMETRY_MARK_AS_CHANGED),
            vec![vec![geometry, 8]]
        );
        assert_eq!(e.get(this, NiUVController::m_bNeedPreDisplay), 0);
    }

    #[test]
    fn on_pre_display_defaults_and_skips() {
        let (mut e, this, data, uv, geometry) = pre_display_engine();
        // No keys at all: offsets 0, tilings 1.
        e.call_log = Some(vec![]);
        e.call(0x00a4_5d10, &args![this]);
        assert!(calls_to(&e, FLOAT_KEY_GEN_INTERP).is_empty());
        assert_eq!(read(&e, data + 0x38, 4), vec![0.0, 0.0, 1.0, 1.0]);
        let a = 0.5f32;
        let b = 0.25f32;
        let ut = (1.0 - a) * 0.5 - (0.0 - 0.25 * a);
        let vt = (1.0 - b) * 0.5 + (0.0 - 0.5 * b);
        assert_eq!(
            read(&e, uv, 4),
            vec![a + ut, 2.0 * b + vt, 3.0 * a + ut, 4.0 * b + vt]
        );
        // Not pending, or a texture set the data does not have: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a4_5d10, &args![this]);
        assert!(calls_to(&e, GEOMETRY_GET_TEXTURE_SET).is_empty());
        e.set(this, NiUVController::m_bNeedPreDisplay, 1);
        e.mem.set_u16(geometry + DATA_FLAGS, 0);
        e.call(0x00a4_5d10, &args![this]);
        assert!(calls_to(&e, GEOMETRY_GET_TEXTURE_SET).is_empty());
        assert_eq!(e.get(this, NiUVController::m_bNeedPreDisplay), 1);
    }

    #[test]
    fn fn_00a45f50_makes_an_empty_controller() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            for i in 0..a[0] / 4 {
                e.mem.set_u32(block + 4 * i, 0xdddd_dddd);
            }
            block.into_ret()
        });
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let this = Ptr::<NiUVController>::new(e.call(0x00a4_5f50, &args![]).u32());
        assert_eq!(e.mem.u32(this.addr()), NI_UV_CONTROLLER_VTABLE);
        for at in [0x34u32, 0x38, 0x3c, 0x40, 0x48] {
            assert_eq!(e.mem.u32(this.addr() + at), 0);
        }
        assert_eq!(e.get(this, NiUVController::m_usTextureSet), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x4c), 0);
        // Unchanged after the members.
        assert_eq!(e.mem.u8(this.addr() + 0x4d), 0xdd);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_5f50, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00a45fa0_takes_the_data_and_resets() {
        let mut e = Engine::new();
        counting(&mut e);
        range_constants(&mut e);
        let this = e.new_object::<NiUVController>();
        for at in [0x34u32, 0x38, 0x3c, 0x40] {
            e.mem.set_u32(this.addr() + at, 9);
        }
        let data = e.mem.alloc(0x4c);
        e.mem.set_u32(data + 4, 1);
        e.call(0x00a4_5fa0, &args![this, data]);
        assert_eq!(e.get(this, NiUVController::m_spUVData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        for at in [0x34u32, 0x38, 0x3c, 0x40] {
            assert_eq!(e.mem.u32(this.addr() + at), 0);
        }
        // The key time range was recomputed (no keys: the start values).
        assert_eq!(e.mem.f32(this.addr() + 0x14), f32::MAX);
    }

    #[test]
    fn load_and_save_binary_move_the_texture_set() {
        const READ: u32 = 0x0300_0520;
        const WRITE: u32 = 0x0300_0530;
        let mut e = Engine::new();
        for f in [
            TIME_CONTROLLER_LOAD_BINARY,
            TIME_CONTROLLER_SAVE_BINARY,
            STREAM_READ_LINK,
            LOG,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            e.mem.set_u16(a[1], 3);
            2u32.into_ret()
        });
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            let set = u32::from(e.mem.u16(a[1]));
            e.call(LOG, &args![set]);
            2u32.into_ret()
        });
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        e.set(output, NiBinaryStream::m_pfnWrite, WRITE);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, LOG);
        e.mem.set_u32(stream.addr(), vtable);
        let this = e.new_object::<NiUVController>();
        e.set(this, NiUVController::m_spUVData, 0x8000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6020, &args![this, stream]);
        assert_eq!(e.get(this, NiUVController::m_usTextureSet), 3);
        assert_eq!(e.mem.u32(input.addr() + 4), 2);
        assert_eq!(calls_to(&e, STREAM_READ_LINK), vec![vec![stream.addr()]]);
        e.call(0x00a4_6070, &args![this, stream]);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![3], vec![stream.addr(), 0x8000]]
        );
        assert_eq!(e.mem.u32(output.addr() + 4), 2);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SAVE_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
    }

    #[test]
    fn create_clone_copies_the_members_and_the_data() {
        let mut e = Engine::new();
        counting(&mut e);
        range_constants(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiUVController>();
        for (i, at) in [0x34u32, 0x38, 0x3c, 0x40].iter().enumerate() {
            e.mem.set_u32(this.addr() + at, 1 + i as u32);
        }
        e.set(this, NiUVController::m_usTextureSet, 7);
        let data = e.mem.alloc(0x4c);
        e.mem.set_u32(data + 4, 1);
        e.set(this, NiUVController::m_spUVData, data);
        e.call_log = Some(vec![]);
        let clone = Ptr::<NiUVController>::new(e.call(0x00a4_60c0, &args![this, 0x9000u32]).u32());
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x9000]]
        );
        assert_eq!(e.mem.u32(clone.addr()), NI_UV_CONTROLLER_VTABLE);
        assert_eq!(e.get(clone, NiUVController::m_usTextureSet), 7);
        assert_eq!(e.get(clone, NiUVController::m_spUVData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        // The data setter resets the indices it copied.
        assert_eq!(e.get(clone, NiUVController::m_uiLastUOffsetIdx), 0);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_UV_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_UV_CONTROLLER_RTTI, 0x0108_0000);
        for f in [TIME_CONTROLLER_GET_VIEWER_STRINGS, STRINGS_GROW] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0108_0001);
            Ret::default()
        });
        let this = e.new_object::<NiUVController>();
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6150, &args![this, strings]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
    }
}
