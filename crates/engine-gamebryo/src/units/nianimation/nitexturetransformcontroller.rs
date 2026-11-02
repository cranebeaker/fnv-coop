//! `NiAnimation/nitexturetransformcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTextureTransformController::ms_RTTI` (the class's `NiRTTI` record).
const NI_TEXTURE_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_3f20;

// Translated from 00a46480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_texture_transform_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TEXTURE_TRANSFORM_CONTROLLER_RTTI)
}

// Translated from 00a46ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a576c0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3f20`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_texture_transform_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_76c0;
    const CLASS_NAME_POINTER: u32 = 0x011f_3f20;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

layout! {
    /// `NiTextureTransformController` (Xbox PDB), 0x50 bytes; the fields this
    /// unit uses (the same offsets on PC).
    pub struct NiTextureTransformController: 0x50 {
        /// `NiTimeController::m_uFlags` (Xbox PDB).
        0x08 m_uFlags: u16,
        /// `NiTimeController::m_fScaledTime` (Xbox PDB).
        0x28 m_fScaledTime: u32,
        /// `NiTimeController::m_pkTarget` (Xbox PDB): the
        /// `NiTexturingProperty`.
        0x2C m_pkTarget: u32,
        /// `NiSingleInterpController::m_spInterpolator` (Xbox PDB).
        0x34 m_spInterpolator: u32,
        /// `m_uiLastIdx` (Xbox PDB).
        0x38 m_uiLastIdx: u32,
        /// `m_pkMap` (Xbox PDB): `NiTexturingProperty::Map*`.
        0x3C m_pkMap: u32,
        /// `m_bShaderMap` (Xbox PDB).
        0x40 m_bShaderMap: u8,
        /// `m_uiMapIndex` (Xbox PDB).
        0x44 m_uiMapIndex: u32,
        /// `m_eMember` (Xbox PDB): `TransformMember` (0 translate U,
        /// 1 translate V, 2 rotate, 3 scale U, 4 scale V, by the format
        /// strings `GetCtlrID` picks).
        0x48 m_eMember: u32,
        /// `m_pcCtlrID` (Xbox PDB): `char*`.
        0x4C m_pcCtlrID: u32,
    }
}

/// The class's vtable.
const VTABLE: u32 = 0x0109_8184;
/// `NiNew`-style allocation (`00aa13e0`, `cdecl`, size), the matching frees
/// (`00aa10f0`, `cdecl`, pointer; `00aa1460`, `cdecl`, pointer and size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE: u32 = 0x00aa_10f0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The `char` allocation `GetCtlrID` uses (`00aa1070`, `cdecl`, size).
const NI_ALLOCATE_CHARS: u32 = 0x00aa_1070;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`,
/// size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The `NiTextureTransform` constructor (`00a88eb0`, `thiscall`, `RET 0x14`:
/// translate pointer, rotation float, scale pointer, centre pointer, method).
const TEXTURE_TRANSFORM_CONSTRUCTOR: u32 = 0x00a8_8eb0;
/// The float a new transform's centre gets in both coordinates (`01016248`).
const TRANSFORM_CENTRE: u32 = 0x0101_6248;
/// The float stored as the scaled time while flag bit 5 is set (`01096324`).
const DISABLED_TIME: u32 = 0x0109_6324;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`, one float).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The base class constructor (`00a57640`, `thiscall`, no stack words;
/// unnamed in the engine map).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7640;
/// The base class's member copy (`00a57620`, `thiscall`, clone and cloning
/// process; a jump to `00a38ef0`).
const BASE_COPY_MEMBERS: u32 = 0x00a5_7620;
/// `NiTimeController::SetTarget` (`00a6d450`, `thiscall`, one word).
const SET_TARGET: u32 = 0x00a6_d450;
/// The `NiTimeController` destructor body (`00a6d290`, `thiscall`).
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// `NiFloatInterpController::LoadBinary` (`00a57c70`, `thiscall`, stream).
const BASE_LOAD_BINARY: u32 = 0x00a5_7c70;
/// `00a645b0` (`thiscall` on the stream; `NiMain/nistream.cpp`), run for
/// files older than version `0x0A010068`.
const STREAM_00A645B0: u32 = 0x00a6_45b0;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, ...).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The `GetCtlrID` formats, by `m_eMember`: `"%d-%d-TT_TRANSLATE_U"`,
/// `"..._V"`, `"%d-%d-TT_ROTATE"`, `"%d-%d-TT_SCALE_U"`, `"..._V"`.
const CTLR_ID_FORMATS: [u32; 5] = [
    0x0109_8168,
    0x0109_8150,
    0x0109_8140,
    0x0109_812c,
    0x0109_8118,
];
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C; the input stream's
/// `m_uiAbsoluteCurrentPos` (+4) and `m_pfnRead` (+8, `cdecl` (stream,
/// buffer, size, &component size, component count), returns the bytes
/// read); `NiStream`'s file version (+0xD8).
const STREAM_INPUT: u32 = 0x24c;
const STREAM_FILE_VERSION: u32 = 0xd8;

/// One `m_pfnRead` call of the stream's input, as the code inlines
/// `NiStreamLoadBinary`: the component size word is set to `size`, and the
/// bytes it answers are added to the input's position.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, size_word: u32) {
    let input = e.mem.u32(stream + STREAM_INPUT);
    let read = e.mem.u32(input + 8);
    e.mem.set_u32(size_word, size);
    let count = e
        .call(read, &args![input, buffer, size, size_word, 1u32])
        .u32();
    let position = e.mem.u32(input + 4);
    e.mem.set_u32(input + 4, position.wrapping_add(count));
}

// Translated from 00a46220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `NiTextureTransform`'s translation
/// setter (fields read at their offsets: `m_kTranslate` +0, `m_bMatrixDirty`
/// +0x1C, Xbox PDB). The dirty flag is set unless it was clear and both
/// coordinates of `point` equal the current ones (`FUCOMPP`: a NaN is never
/// equal); the two words are copied either way.
pub fn fn_00a46220(e: &mut Engine, this: Ptr, point: Ptr) {
    let this = this.addr();
    let point = point.addr();
    let unchanged = e.mem.u8(this + 0x1c) == 0
        && x87_load32(e.mem.u32(point)) == x87_load32(e.mem.u32(this))
        && x87_load32(e.mem.u32(point + 4)) == x87_load32(e.mem.u32(this + 4));
    e.mem.set_u8(this + 0x1c, u8::from(!unchanged));
    let x = e.mem.u32(point);
    e.mem.set_u32(this, x);
    let y = e.mem.u32(point + 4);
    e.mem.set_u32(this + 4, y);
}

// Translated from 00a46270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `NiTextureTransform`'s rotation
/// setter (`m_fRotate` +8, `m_bMatrixDirty` +0x1C, Xbox PDB): the dirty flag
/// is set unless it was clear and `rotation` equals the current value
/// (`FUCOMPP`); the value is stored through the x87 stack (a signalling NaN
/// comes out quiet).
pub fn fn_00a46270(e: &mut Engine, this: Ptr, rotation: f32) {
    let this = this.addr();
    let value = x87_load32(rotation.to_bits());
    let unchanged = e.mem.u8(this + 0x1c) == 0 && x87_load32(e.mem.u32(this + 8)) == value;
    e.mem.set_u32(this + 8, x87_store32(value));
    e.mem.set_u8(this + 0x1c, u8::from(!unchanged));
}

// Translated from 00a462b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `NiTextureTransform`'s scale
/// setter (`m_kScale` +0xC, `m_bMatrixDirty` +0x1C, Xbox PDB), the same
/// test as the translation setter (`00a46220`) on the scale.
pub fn fn_00a462b0(e: &mut Engine, this: Ptr, point: Ptr) {
    let this = this.addr();
    let point = point.addr();
    let unchanged = e.mem.u8(this + 0x1c) == 0
        && x87_load32(e.mem.u32(point)) == x87_load32(e.mem.u32(this + 0xc))
        && x87_load32(e.mem.u32(point + 4)) == x87_load32(e.mem.u32(this + 0x10));
    e.mem.set_u8(this + 0x1c, u8::from(!unchanged));
    let x = e.mem.u32(point);
    e.mem.set_u32(this + 0xc, x);
    let y = e.mem.u32(point + 4);
    e.mem.set_u32(this + 0x10, y);
}

// Translated from 00a46300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::GetCtlrID` (Xbox PDB): the cached ID
/// string, or a new 0x32-byte one (`00aa1070`) formatted with `NiSprintf`
/// as `"<shader map flag>-<map index>-TT_<member>"`. A member above 4 leaves
/// the new buffer unformatted.
pub fn ni_texture_transform_controller_get_ctlr_id(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
) -> u32 {
    let id = e.get(this, NiTextureTransformController::m_pcCtlrID);
    if id != 0 {
        return id;
    }
    let buffer = e.call(NI_ALLOCATE_CHARS, &args![0x32u32]).u32();
    e.set(this, NiTextureTransformController::m_pcCtlrID, buffer);
    let member = e.get(this, NiTextureTransformController::m_eMember);
    if let Some(&format) = CTLR_ID_FORMATS.get(member as usize) {
        let index = e.get(this, NiTextureTransformController::m_uiMapIndex);
        let shader = u32::from(e.get(this, NiTextureTransformController::m_bShaderMap));
        e.call(NI_SPRINTF, &args![buffer, 0x32u32, format, shader, index]);
    }
    e.get(this, NiTextureTransformController::m_pcCtlrID)
}

// Translated from 00a46410 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTexturingProperty`): the
/// size (`u16` at +0xA) of the shader map array at +0x2C, or 0 without one.
pub fn fn_00a46410(e: &mut Engine, this: Ptr) -> u32 {
    let shader_maps = e.mem.u32(this.addr() + 0x2c);
    if shader_maps == 0 {
        return 0;
    }
    u32::from(e.mem.u16(shader_maps + 0xa))
}

// Translated from 00a46420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::SetInternalMap` (Xbox PDB): true when a
/// map is already set; otherwise, with a target `NiTexturingProperty`, takes
/// the map at `m_uiMapIndex` from its shader maps (+0x2C, an array with the
/// pointers at +4 and the size at +0xA; null when missing or out of range)
/// or from its maps (the array at +0x20, not range-checked) and answers
/// whether it is non-null. Without a target: false.
pub fn ni_texture_transform_controller_set_internal_map(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
) -> bool {
    if e.get(this, NiTextureTransformController::m_pkMap) != 0 {
        return true;
    }
    let target = e.get(this, NiTextureTransformController::m_pkTarget);
    if target == 0 {
        return false;
    }
    let index = e.get(this, NiTextureTransformController::m_uiMapIndex);
    let map = if e.get(this, NiTextureTransformController::m_bShaderMap) != 0 {
        let shader_maps = e.mem.u32(target + 0x2c);
        if shader_maps != 0 && index < u32::from(e.mem.u16(shader_maps + 0xa)) {
            let array = e.mem.u32(shader_maps + 4);
            e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
        } else {
            0
        }
    } else {
        let array = e.mem.u32(target + 0x20);
        e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
    };
    e.set(this, NiTextureTransformController::m_pkMap, map);
    map != 0
}

// Translated from 00a46490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::Update` (Xbox PDB; `thiscall`, one word:
/// the update data, whose first float is the time; `RET 4`). With flag bit 5
/// set the scaled time becomes the float at `01096324`; otherwise, when
/// `DontDoUpdate(time)` answers true, it goes on only with an interpolator
/// whose vtable slot +0xD4 answers true. The interpolator's float update
/// (slot +0x9C: scaled time, target, the address of the argument's own
/// stack slot, which receives the value) must succeed and the map must be
/// found (`SetInternalMap`); a map without a transform (+0xC) gets a new one
/// (0x48 bytes from the memory manager; translate (0, 0), rotation 0, scale
/// (1, 1), centre from `01016248`, method 2), or null when the allocation
/// fails. The value then goes into the member `m_eMember` names, through the
/// transform's setters (`00a46220`, `00a46270`, `00a462b0`); a member above
/// 4 sets nothing.
pub fn ni_texture_transform_controller_update(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    update_data: u32,
) {
    let flags = e.get(this, NiTextureTransformController::m_uFlags) as u8;
    if (flags >> 5) & 1 != 0 {
        let time = x87_quiet_bits(e.mem.u32(DISABLED_TIME));
        e.set(this, NiTextureTransformController::m_fScaledTime, time);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update_data));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.get(this, NiTextureTransformController::m_spInterpolator);
            if interpolator == 0 || !e.vcall(interpolator, 0xd4, &[]).bool() {
                return;
            }
        }
    }
    let interpolator = e.get(this, NiTextureTransformController::m_spInterpolator);
    if interpolator == 0 {
        return;
    }
    // The argument's stack slot holds the update data pointer until the
    // interpolator writes the value into it.
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, update_data);
        let time = x87_quiet_bits(e.get(this, NiTextureTransformController::m_fScaledTime));
        let target = e.get(this, NiTextureTransformController::m_pkTarget);
        if !e
            .vcall(interpolator, 0x9c, &args![time, target, slot])
            .bool()
        {
            return;
        }
        if !ni_texture_transform_controller_set_internal_map(e, this) {
            return;
        }
        let map = e.get(this, NiTextureTransformController::m_pkMap);
        let mut transform = e.mem.u32(map + 0xc);
        if transform == 0 {
            let block = e
                .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x48u32])
                .u32();
            transform = if block == 0 {
                0
            } else {
                e.with_stack(0x18, |e, points| {
                    let points = points.addr();
                    let centre = x87_quiet_bits(e.mem.u32(TRANSFORM_CENTRE));
                    let one = 1.0f32.to_bits();
                    let zero = 0.0f32.to_bits();
                    // translate (0, 0), scale (1, 1), centre (c, c)
                    for (offset, word) in [zero, zero, one, one, centre, centre]
                        .into_iter()
                        .enumerate()
                    {
                        e.mem.set_u32(points + 4 * offset as u32, word);
                    }
                    e.call(
                        TEXTURE_TRANSFORM_CONSTRUCTOR,
                        &args![block, points, zero, points + 8, points + 0x10, 2u32],
                    )
                    .u32()
                })
            };
            let map = e.get(this, NiTextureTransformController::m_pkMap);
            e.mem.set_u32(map + 0xc, transform);
        }
        // FLD/FSTP of the value the interpolator left in the slot.
        let value = x87_quiet_bits(e.mem.u32(slot));
        let member = e.get(this, NiTextureTransformController::m_eMember);
        match member {
            0 | 1 | 3 | 4 => {
                // NiTextureTransform::m_kTranslate +0 or m_kScale +0xC.
                let base = if member < 2 { 0 } else { 0xc };
                let x = e.mem.u32(transform + base);
                let y = e.mem.u32(transform + base + 4);
                let point = if member % 3 == 0 {
                    [value, y]
                } else {
                    [x, value]
                };
                e.with_stack(8, |e, at| {
                    e.mem.set_u32(at.addr(), point[0]);
                    e.mem.set_u32(at.addr() + 4, point[1]);
                    if member < 2 {
                        fn_00a46220(e, Ptr::new(transform), at);
                    } else {
                        fn_00a462b0(e, Ptr::new(transform), at);
                    }
                });
            }
            2 => fn_00a46270(e, Ptr::new(transform), f32::from_bits(value)),
            _ => {}
        }
    });
}

// Translated from 00a46740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body the `NiTextureTransformController`
/// constructor (`thiscall`: target `NiTexturingProperty`, map, member;
/// `RET 0xC`). Runs the base constructor (`00a57640`), sets the vtable, the
/// member, and clears the ID, last index, shader flag and map index; then
/// sets the target (`NiTimeController::SetTarget`, clearing the map first
/// when the target changes) and the map. Without a map the ID is freed.
/// Otherwise, with a target, finds the map in the target's maps (+0x20,
/// `u16` count at +0x26): found, it records index and flag 0, frees the ID
/// and runs `SetInternalMap`. Else it searches the shader maps (count from
/// `00a46410`): found, it records the index and flag 1, frees the ID, and
/// (inlined `SetInternalMap`, which does nothing here since the map is set)
/// fills a missing map from the target.
pub fn fn_00a46740(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    target: u32,
    map: u32,
    member: u32,
) -> Ptr<NiTextureTransformController> {
    type C = NiTextureTransformController;
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.set(this, C::m_eMember, member);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, C::m_pcCtlrID, 0);
    e.set(this, C::m_uiLastIdx, 0);
    e.set(this, C::m_bShaderMap, 0);
    e.set(this, C::m_uiMapIndex, 0);
    if e.get(this, C::m_pkTarget) != target {
        e.set(this, C::m_pkMap, 0);
    }
    e.call(SET_TARGET, &args![this, target]);
    e.set(this, C::m_pkMap, map);
    if map == 0 {
        let id = e.get(this, C::m_pcCtlrID);
        e.call(NI_FREE, &args![id]);
        e.set(this, C::m_pcCtlrID, 0);
        return this;
    }
    let property = e.get(this, C::m_pkTarget);
    if property == 0 {
        return this;
    }
    let count = u32::from(e.mem.u16(property + 0x26));
    let maps = e.mem.u32(property + 0x20);
    for index in 0..count {
        if e.mem.u32(maps + index * 4) == map {
            e.set(this, C::m_bShaderMap, 0);
            e.set(this, C::m_uiMapIndex, index);
            let id = e.get(this, C::m_pcCtlrID);
            e.call(NI_FREE, &args![id]);
            e.set(this, C::m_pcCtlrID, 0);
            ni_texture_transform_controller_set_internal_map(e, this);
            return this;
        }
    }
    let shader_count = e.call(0x00a4_6410, &args![property]).u32();
    if shader_count == 0 {
        return this;
    }
    let shader_maps = e.mem.u32(property + 0x2c);
    let mut index = 0u32;
    loop {
        let entry = if shader_maps != 0 && index < u32::from(e.mem.u16(shader_maps + 0xa)) {
            let array = e.mem.u32(shader_maps + 4);
            e.mem.u32(array + index * 4)
        } else {
            0
        };
        if entry == map {
            break;
        }
        index += 1;
        if index >= shader_count {
            return this;
        }
    }
    e.set(this, C::m_bShaderMap, 1);
    e.set(this, C::m_uiMapIndex, index);
    let id = e.get(this, C::m_pcCtlrID);
    e.call(NI_FREE, &args![id]);
    e.set(this, C::m_pcCtlrID, 0);
    // The inlined SetInternalMap: m_pkMap is `map` (non-null) here, so this
    // never runs; translated as the exe has it.
    let target = e.get(this, C::m_pkTarget);
    if e.get(this, C::m_pkMap) == 0 && target != 0 {
        let index = e.get(this, C::m_uiMapIndex);
        let found = if e.get(this, C::m_bShaderMap) != 0 {
            let shader_maps = e.mem.u32(target + 0x2c);
            if shader_maps != 0 && index < u32::from(e.mem.u16(shader_maps + 0xa)) {
                let array = e.mem.u32(shader_maps + 4);
                e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
            } else {
                0
            }
        } else {
            let array = e.mem.u32(target + 0x20);
            e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
        };
        e.set(this, C::m_pkMap, found);
    }
    this
}

// Translated from 00a46890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::scalar deleting destructor` (Xbox PDB):
/// sets the vtable, frees the ID (`00aa10f0`) and clears it, releases the
/// interpolator (`InterlockedDecrement` of its count at +4; `DeleteThis`,
/// slot +4, at zero; the pointer is left as it is), runs the
/// `NiTimeController` destructor body (`00a6d290`) and, with bit 0 of
/// `flags`, frees the 0x50 bytes (`00aa1460`). Returns `this`.
pub fn ni_texture_transform_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    flags: u32,
) -> Ptr<NiTextureTransformController> {
    e.mem.set_u32(this.addr(), VTABLE);
    let id = e.get(this, NiTextureTransformController::m_pcCtlrID);
    e.call(NI_FREE, &args![id]);
    e.set(this, NiTextureTransformController::m_pcCtlrID, 0);
    let interpolator = e.get(this, NiTextureTransformController::m_spInterpolator);
    if interpolator != 0
        && e.call(INTERLOCKED_DECREMENT, &args![interpolator + 4])
            .u32()
            == 0
    {
        e.vcall(interpolator, 0x4, &[]);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0x50u32]);
    }
    this
}

// Translated from 00a468f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::CreateClone` (Xbox PDB): a new controller
/// (0x50 bytes, `00aa13e0`; constructed with no target, map or member, or
/// null when the allocation fails), the base members copied (`00a57620`)
/// and the last index, shader flag, map index and member copied. With a
/// failed allocation the copies write through the null pointer as in the
/// exe.
pub fn ni_texture_transform_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    cloning: u32,
) -> Ptr<NiTextureTransformController> {
    type C = NiTextureTransformController;
    let block = e.call(NI_ALLOCATE, &args![0x50u32]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00a46740(e, Ptr::new(block), 0, 0, 0)
    };
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let last = e.get(this, C::m_uiLastIdx);
    e.set(clone, C::m_uiLastIdx, last);
    let shader = e.get(this, C::m_bShaderMap);
    e.set(clone, C::m_bShaderMap, shader);
    let index = e.get(this, C::m_uiMapIndex);
    e.set(clone, C::m_uiMapIndex, index);
    let member = e.get(this, C::m_eMember);
    e.set(clone, C::m_eMember, member);
    clone
}

// Translated from 00a46950 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::CreateObject` (by its body; Ghidra has no
/// function here): a new controller (0x50 bytes, `00aa13e0`) constructed
/// with no target, map or member, or null when the allocation fails.
pub fn ni_texture_transform_controller_create_object(
    e: &mut Engine,
) -> Ptr<NiTextureTransformController> {
    let block = e.call(NI_ALLOCATE, &args![0x50u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00a46740(e, Ptr::new(block), 0, 0, 0)
}

// Translated from 00a46970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary` (`00a57c70`), then one byte (read into the argument's own
/// stack slot, which holds the stream pointer before) whose non-zero value
/// is the shader map flag, the map index (4 bytes, straight into +0x44) and
/// the member (4 bytes). Files older than `0x0A010068` then run `00a645b0`
/// on the stream.
pub fn ni_texture_transform_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    stream: u32,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, locals| {
        let size_word = locals.addr();
        let member = locals.addr() + 4;
        let slot = locals.addr() + 8;
        e.mem.set_u32(slot, stream);
        stream_read(e, stream, slot, 1, size_word);
        let shader = e.mem.u8(slot) != 0;
        e.set(
            this,
            NiTextureTransformController::m_bShaderMap,
            u8::from(shader),
        );
        stream_read(e, stream, this.addr() + 0x44, 4, size_word);
        stream_read(e, stream, member, 4, size_word);
        let member = e.mem.u32(member);
        e.set(this, NiTextureTransformController::m_eMember, member);
    });
    if e.mem.u32(stream + STREAM_FILE_VERSION) < 0x0a01_0068 {
        e.call(STREAM_00A645B0, &args![stream]);
    }
}

/// The base `IsEqual` (`00a57c10`) and `SaveBinary` (`00a57c00`), both `thiscall` with one
/// word: the engine map names them `NiBoolInterpController`'s; each is a jump to
/// `NiSingleInterpController`'s (`00a39030`, `00a39010`).
const BASE_IS_EQUAL: u32 = 0x00a5_7c10;
const BASE_SAVE_BINARY: u32 = 0x00a5_7c00;
/// `NiStream::m_pkOstr` (Xbox PDB) +0x250; the output stream's `m_pfnWrite` (+0xC, the same
/// arguments as the read).
const STREAM_OUTPUT: u32 = 0x250;

/// One `m_pfnWrite` call of the stream's output: the component size word at `size_word` is
/// set to `size`, and the bytes it answers are added to the output's position.
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32, size_word: u32) {
    let output = e.mem.u32(stream + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    e.mem.set_u32(size_word, size);
    let count = e
        .call(write, &args![output, buffer, size, size_word, 1u32])
        .u32();
    let position = e.mem.u32(output + 4);
    e.mem.set_u32(output + 4, position.wrapping_add(count));
}

// Translated from 00a461a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::SetTarget` (Xbox PDB; `thiscall`, the target): a new target
/// forgets the map (`m_pkMap` cleared); then `NiTimeController::SetTarget` (`00a6d450`, a
/// tail jump; its `EAX` is answered).
pub fn ni_texture_transform_controller_set_target(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    target: u32,
) -> u32 {
    if e.get(this, NiTextureTransformController::m_pkTarget) != target {
        e.set(this, NiTextureTransformController::m_pkMap, 0);
    }
    e.call(SET_TARGET, &args![this, target]).u32()
}

// Translated from 00a461e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::IsEqual` (Xbox PDB; `thiscall`, the other object): the
/// base `IsEqual` (`00a57c10`), then `m_uiLastIdx`, `m_bShaderMap`, `m_uiMapIndex` and
/// `m_eMember` equal.
pub fn ni_texture_transform_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    other: Ptr<NiTextureTransformController>,
) -> bool {
    type C = NiTextureTransformController;
    e.call(BASE_IS_EQUAL, &args![this, other]).bool()
        && e.get(other, C::m_uiLastIdx) == e.get(this, C::m_uiLastIdx)
        && e.get(other, C::m_bShaderMap) == e.get(this, C::m_bShaderMap)
        && e.get(other, C::m_uiMapIndex) == e.get(this, C::m_uiMapIndex)
        && e.get(other, C::m_eMember) == e.get(this, C::m_eMember)
}

// Translated from 00a463e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTexturingProperty`, an index): the shader
/// map at `index` of the array at +0x2C (the pointers at +4, the size at +0xA), or 0 without
/// the array or past its size.
pub fn fn_00a463e0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let shader_maps = e.mem.u32(this.addr() + 0x2c);
    if shader_maps == 0 || index >= u32::from(e.mem.u16(shader_maps + 0xa)) {
        return 0;
    }
    let array = e.mem.u32(shader_maps + 4);
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a46660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::GetTargetFloatValue` (Xbox PDB; `thiscall`, the address
/// of the float): when the map is found (`SetInternalMap`), its transform (+0xC of the map)
/// gives the member `m_eMember` names (0 translate U +0, 1 translate V +4, 2 rotation +8, 3
/// scale U +0xC, 4 scale V +0x10, through the x87 stack); a map without a transform gives
/// 0; a member above 4 leaves the float as it is, and so does a missing map.
pub fn ni_texture_transform_controller_get_target_float_value(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    value: Ptr,
) {
    if !ni_texture_transform_controller_set_internal_map(e, this) {
        return;
    }
    let map = e.get(this, NiTextureTransformController::m_pkMap);
    let transform = e.mem.u32(map + 0xc);
    if transform == 0 {
        e.mem.set_u32(value.addr(), x87_store32(0.0));
        return;
    }
    // The compiler's jump table (`00a4672c`): one case per member.
    let offset = match e.get(this, NiTextureTransformController::m_eMember) {
        0 => 0,
        1 => 4,
        2 => 8,
        3 => 0xc,
        4 => 0x10,
        _ => return,
    };
    let word = x87_quiet_bits(e.mem.u32(transform + offset));
    e.mem.set_u32(value.addr(), word);
}

// Translated from 00a46a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureTransformController::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base
/// `SaveBinary` (`00a57c00`), then `m_bShaderMap` (1 byte, copied into the low byte of the
/// stream argument's own stack slot first), `m_uiMapIndex` and `m_eMember` (4 bytes each),
/// the component size word being the function's one local.
pub fn ni_texture_transform_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiTextureTransformController>,
    stream: u32,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let shader = e.get(this, NiTextureTransformController::m_bShaderMap);
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let size_word = argument + 4;
        e.mem.set_u32(argument, stream);
        e.mem.set_u8(argument, shader);
        stream_write(e, stream, argument, 1, size_word);
        stream_write(e, stream, this.addr() + 0x44, 4, size_word);
        stream_write(e, stream, this.addr() + 0x48, 4, size_word);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type C = NiTextureTransformController;
    vec![
        entry!(
            0x00a4_61a0,
            ni_texture_transform_controller_set_target(Ptr<C>, u32) -> u32
        ),
        entry!(
            0x00a4_61e0,
            ni_texture_transform_controller_is_equal(Ptr<C>, Ptr<C>) -> bool
        ),
        entry!(0x00a4_63e0, fn_00a463e0(Ptr, u32) -> u32),
        entry!(
            0x00a4_6660,
            ni_texture_transform_controller_get_target_float_value(Ptr<C>, Ptr)
        ),
        entry!(
            0x00a4_6a30,
            ni_texture_transform_controller_save_binary(Ptr<C>, u32)
        ),
        entry!(0x00a46220, fn_00a46220(Ptr, Ptr)),
        entry!(0x00a46270, fn_00a46270(Ptr, f32)),
        entry!(0x00a462b0, fn_00a462b0(Ptr, Ptr)),
        entry!(
            0x00a46300,
            ni_texture_transform_controller_get_ctlr_id(Ptr<C>) -> u32
        ),
        entry!(0x00a46410, fn_00a46410(Ptr) -> u32),
        entry!(
            0x00a46420,
            ni_texture_transform_controller_set_internal_map(Ptr<C>) -> bool
        ),
        entry!(
            0x00a46490,
            ni_texture_transform_controller_update(Ptr<C>, u32)
        ),
        entry!(0x00a46740, fn_00a46740(Ptr<C>, u32, u32, u32) -> Ptr<C>),
        entry!(
            0x00a46890,
            ni_texture_transform_controller_scalar_deleting_destructor(Ptr<C>, u32) -> Ptr<C>
        ),
        entry!(
            0x00a468f0,
            ni_texture_transform_controller_create_clone(Ptr<C>, u32) -> Ptr<C>
        ),
        entry!(0x00a46950, ni_texture_transform_controller_create_object() -> Ptr<C>),
        entry!(
            0x00a46970,
            ni_texture_transform_controller_load_binary(Ptr<C>, u32)
        ),
        entry!(0x00a46480, ni_texture_transform_controller_get_rtti() -> Ptr),
        entry!(
            0x00a46ac0,
            ni_texture_transform_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_texture_transform_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a46480, &args![]).u32(),
            NI_TEXTURE_TRANSFORM_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_texture_transform_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00a5_76c0;
        const NAME_POINTER: u32 = 0x011f_3f20;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_texture_transform_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    type C = NiTextureTransformController;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> Ptr {
        let at = e.mem.alloc(8);
        e.mem.set_f32(at, x);
        e.mem.set_f32(at + 4, y);
        Ptr::new(at)
    }

    #[test]
    fn transform_setters_mark_the_matrix_dirty_on_a_change() {
        let mut e = Engine::new();
        let transform = e.mem.alloc(0x48);
        e.mem.set_f32(transform, 1.0);
        e.mem.set_f32(transform + 4, 2.0);
        let same = point(&mut e, 1.0, 2.0);
        e.call(0x00a4_6220, &args![transform, same]);
        assert_eq!(e.mem.u8(transform + 0x1c), 0);
        let other = point(&mut e, 1.0, 3.0);
        e.call(0x00a4_6220, &args![transform, other]);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);
        assert_eq!(e.mem.f32(transform + 4), 3.0);
        // Once dirty it stays dirty, even for the same value.
        e.call(0x00a4_6220, &args![transform, other]);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);

        e.mem.set_u8(transform + 0x1c, 0);
        e.call(0x00a4_6270, &args![transform, 0.0f32]);
        assert_eq!(e.mem.u8(transform + 0x1c), 0);
        e.call(0x00a4_6270, &args![transform, 0.5f32]);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);
        assert_eq!(e.mem.f32(transform + 8), 0.5);
        // A signalling NaN is stored quiet, and is never equal.
        e.mem.set_u8(transform + 0x1c, 0);
        e.mem.set_u32(transform + 8, 0x7f80_0001);
        e.call(0x00a4_6270, &args![transform, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(transform + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);

        e.mem.set_u8(transform + 0x1c, 0);
        let scale = point(&mut e, 0.0, 0.0);
        e.call(0x00a4_62b0, &args![transform, scale]);
        assert_eq!(e.mem.u8(transform + 0x1c), 0);
        let scale = point(&mut e, 2.0, 0.0);
        e.call(0x00a4_62b0, &args![transform, scale]);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);
        assert_eq!(e.mem.f32(transform + 0xc), 2.0);
    }

    #[test]
    fn ctlr_id_is_formatted_once_by_member() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE_CHARS, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let this: Ptr<C> = e.new_object();
        e.set(this, C::m_eMember, 2);
        e.set(this, C::m_bShaderMap, 1);
        e.set(this, C::m_uiMapIndex, 3);
        e.call_log = Some(vec![]);
        let id = e.call(0x00a4_6300, &args![this]).u32();
        assert_ne!(id, 0);
        assert_eq!(e.get(this, C::m_pcCtlrID), id);
        assert_eq!(
            calls_to(&e, NI_SPRINTF),
            vec![vec![id, 0x32, 0x0109_8140, 1, 3]]
        );
        // Cached: no second allocation.
        assert_eq!(e.call(0x00a4_6300, &args![this]).u32(), id);
        assert_eq!(calls_to(&e, NI_ALLOCATE_CHARS).len(), 1);
        // A member above 4 is not formatted.
        let other: Ptr<C> = e.new_object();
        e.set(other, C::m_eMember, 5);
        e.call(0x00a4_6300, &args![other]);
        assert_eq!(calls_to(&e, NI_SPRINTF).len(), 1);
    }

    /// A texturing property with two maps and a shader map array of
    /// `shader_count` entries holding `shader_maps`.
    fn property(e: &mut Engine, maps: [u32; 2], shader_maps: &[u32]) -> u32 {
        let property = e.mem.alloc(0x40);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, maps[0]);
        e.mem.set_u32(array + 4, maps[1]);
        e.mem.set_u32(property + 0x20, array);
        e.mem.set_u16(property + 0x26, 2);
        if !shader_maps.is_empty() {
            let holder = e.mem.alloc(0x10);
            let entries = e.mem.alloc(4 * shader_maps.len() as u32);
            for (i, map) in shader_maps.iter().enumerate() {
                e.mem.set_u32(entries + 4 * i as u32, *map);
            }
            e.mem.set_u32(holder + 4, entries);
            e.mem.set_u16(holder + 0xa, shader_maps.len() as u16);
            e.mem.set_u32(property + 0x2c, holder);
        }
        property
    }

    #[test]
    fn shader_map_count_and_internal_map() {
        let mut e = Engine::new();
        let plain = property(&mut e, [0x100, 0x200], &[]);
        assert_eq!(e.call(0x00a4_6410, &args![plain]).u32(), 0);
        let shaded = property(&mut e, [0x100, 0x200], &[0x300, 0x400, 0x500]);
        assert_eq!(e.call(0x00a4_6410, &args![shaded]).u32(), 3);

        let this: Ptr<C> = e.new_object();
        assert!(!e.call(0x00a4_6420, &args![this]).bool());
        e.set(this, C::m_pkTarget, shaded);
        e.set(this, C::m_uiMapIndex, 1);
        assert!(e.call(0x00a4_6420, &args![this]).bool());
        assert_eq!(e.get(this, C::m_pkMap), 0x200);
        // Already set: true without a lookup.
        e.set(this, C::m_uiMapIndex, 0);
        assert!(e.call(0x00a4_6420, &args![this]).bool());
        assert_eq!(e.get(this, C::m_pkMap), 0x200);
        e.set(this, C::m_pkMap, 0);
        e.set(this, C::m_bShaderMap, 1);
        e.set(this, C::m_uiMapIndex, 2);
        assert!(e.call(0x00a4_6420, &args![this]).bool());
        assert_eq!(e.get(this, C::m_pkMap), 0x500);
        e.set(this, C::m_pkMap, 0);
        e.set(this, C::m_uiMapIndex, 3);
        assert!(!e.call(0x00a4_6420, &args![this]).bool());
        assert_eq!(e.get(this, C::m_pkMap), 0);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(SET_TARGET, |e, a| {
            e.mem.set_u32(a[0] + 0x2c, a[1]);
            Ret::default()
        });
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e
    }

    #[test]
    fn constructor_finds_the_map_in_the_target() {
        let mut e = constructor_engine();
        let shaded = property(&mut e, [0x100, 0x200], &[0x300, 0x400]);
        e.call_log = Some(vec![]);
        let this: Ptr<C> = e.new_object();
        e.call(0x00a4_6740, &args![this, shaded, 0x200u32, 3u32]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, C::m_eMember), 3);
        assert_eq!(e.get(this, C::m_pkMap), 0x200);
        assert_eq!(e.get(this, C::m_bShaderMap), 0);
        assert_eq!(e.get(this, C::m_uiMapIndex), 1);
        assert_eq!(calls_to(&e, SET_TARGET), vec![vec![this.addr(), shaded]]);

        let this: Ptr<C> = e.new_object();
        e.call(0x00a4_6740, &args![this, shaded, 0x400u32, 0u32]);
        assert_eq!(e.get(this, C::m_bShaderMap), 1);
        assert_eq!(e.get(this, C::m_uiMapIndex), 1);
        assert_eq!(e.get(this, C::m_pkMap), 0x400);

        // A map the target does not hold: index and flag stay clear.
        let this: Ptr<C> = e.new_object();
        e.call(0x00a4_6740, &args![this, shaded, 0x999u32, 0u32]);
        assert_eq!(e.get(this, C::m_bShaderMap), 0);
        assert_eq!(e.get(this, C::m_uiMapIndex), 0);
        assert_eq!(e.get(this, C::m_pkMap), 0x999);

        // No map: the ID is freed and the search skipped.
        let before = calls_to(&e, NI_FREE).len();
        let object = e.call(0x00a4_6950, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(calls_to(&e, NI_FREE).len(), before + 1);
    }

    #[test]
    fn clone_copies_the_members() {
        let mut e = constructor_engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let this: Ptr<C> = e.new_object();
        e.set(this, C::m_uiLastIdx, 7);
        e.set(this, C::m_bShaderMap, 1);
        e.set(this, C::m_uiMapIndex, 2);
        e.set(this, C::m_eMember, 4);
        e.call_log = Some(vec![]);
        let clone: Ptr<C> = e.call(0x00a4_68f0, &args![this, 0x55u32]).ptr();
        assert_eq!(e.get(clone, C::m_uiLastIdx), 7);
        assert_eq!(e.get(clone, C::m_bShaderMap), 1);
        assert_eq!(e.get(clone, C::m_uiMapIndex), 2);
        assert_eq!(e.get(clone, C::m_eMember), 4);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x55]]
        );
    }

    #[test]
    fn destructor_releases_and_frees_on_flag() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0012_3450, |_, _| Ret::default());
        e.put_vtable(0x0012_0000, &[0, 0x0012_3450]);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, 0x0012_0000);
        e.mem.set_u32(interpolator + 4, 1);
        let this: Ptr<C> = e.new_object();
        e.set(this, C::m_spInterpolator, interpolator);
        e.set(this, C::m_pcCtlrID, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6890, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, C::m_pcCtlrID), 0);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, 0x0012_3450), vec![vec![interpolator]]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this.addr(), 0x50]]);
        e.call_log = Some(vec![]);
        e.mem.set_u32(interpolator + 4, 2);
        e.call(0x00a4_6890, &args![this, 0u32]);
        assert!(calls_to(&e, 0x0012_3450).is_empty());
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
    }

    /// An interpolator whose float update (slot +0x9C) writes 0.75 into its
    /// out slot and answers true, and whose slot +0xD4 answers `active`.
    fn interpolator(e: &mut Engine, active: u32) -> u32 {
        e.register(0x0013_0000, |e, a| {
            e.mem.set_f32(a[3], 0.75);
            1u32.into_ret()
        });
        let active_fn = if active != 0 {
            0x0013_0010
        } else {
            0x0013_0020
        };
        e.register(0x0013_0010, |_, _| 1u32.into_ret());
        e.register(0x0013_0020, |_, _| 0u32.into_ret());
        let mut slots = vec![0u32; 0x36];
        slots[0x9c / 4] = 0x0013_0000;
        slots[0xd4 / 4] = active_fn;
        let vtable = e.mem.alloc(0xd8);
        for (i, s) in slots.iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *s);
        }
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn update_writes_the_value_into_the_member() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.set_global(TRANSFORM_CENTRE, 0.5f32);
        e.map(0x0109_6000, 0x1000);
        e.set_global(DISABLED_TIME, -1.0f32);
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(TEXTURE_TRANSFORM_CONSTRUCTOR, |e, a| {
            assert_eq!(f32::from_bits(a[2]), 0.0);
            assert_eq!(e.mem.f32(a[3]), 1.0);
            assert_eq!(e.mem.f32(a[4] + 4), 0.5);
            assert_eq!(a[5], 2);
            let this = a[0];
            for i in 0..2 {
                let t = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(this + 4 * i, t);
                let s = e.mem.u32(a[3] + 4 * i);
                e.mem.set_u32(this + 0xc + 4 * i, s);
            }
            this.into_ret()
        });
        let map = e.mem.alloc(0x10);
        let target = property(&mut e, [map, 0], &[]);
        let this: Ptr<C> = e.new_object();
        let interp = interpolator(&mut e, 1);
        e.set(this, C::m_spInterpolator, interp);
        e.set(this, C::m_pkTarget, target);
        e.set(this, C::m_fScaledTime, 2.0f32.to_bits());
        e.set(this, C::m_eMember, 4);
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6490, &args![this, data]);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this.addr(), 3.0f32.to_bits()]]
        );
        let update = calls_to(&e, 0x0013_0000);
        assert_eq!(&update[0][..3], [interp, 2.0f32.to_bits(), target]);
        let transform = e.mem.u32(map + 0xc);
        assert_ne!(transform, 0);
        // Scale V takes the value; scale U keeps 1.
        assert_eq!(e.mem.f32(transform + 0xc), 1.0);
        assert_eq!(e.mem.f32(transform + 0x10), 0.75);
        assert_eq!(e.mem.u8(transform + 0x1c), 1);

        // Translate U, with flag bit 5: the disabled time is used.
        e.set(this, C::m_eMember, 0);
        e.set(this, C::m_uFlags, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6490, &args![this, data]);
        assert!(calls_to(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(e.get(this, C::m_fScaledTime), (-1.0f32).to_bits());
        assert_eq!(e.mem.f32(transform), 0.75);
        assert_eq!(e.mem.f32(transform + 4), 0.0);

        // Rotation.
        e.set(this, C::m_eMember, 2);
        e.call(0x00a4_6490, &args![this, data]);
        assert_eq!(e.mem.f32(transform + 8), 0.75);

        // DontDoUpdate true with an inactive interpolator: nothing runs.
        e.set(this, C::m_uFlags, 0);
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        let idle = interpolator(&mut e, 0);
        e.set(this, C::m_spInterpolator, idle);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6490, &args![this, data]);
        assert!(calls_to(&e, 0x0013_0000).is_empty());
    }

    #[test]
    fn load_binary_reads_flag_index_and_member() {
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_00A645B0, |_, _| Ret::default());
        // The read function hands out 1, then 6, then 4 (as little-endian
        // words of the requested size).
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x0014_0000);
        e.mem.set_u32(input + 0xc, 0);
        e.register(0x0014_0000, |e, a| {
            let n = e.mem.u32(a[0] + 0xc);
            let value = [1u32, 6, 4][n as usize];
            if a[2] == 1 {
                e.mem.set_u8(a[1], value as u8);
            } else {
                e.mem.set_u32(a[1], value);
            }
            e.mem.set_u32(a[0] + 0xc, n + 1);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, 0x0a01_0067);
        let this: Ptr<C> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00a4_6970, &args![this, stream]);
        assert_eq!(e.get(this, C::m_bShaderMap), 1);
        assert_eq!(e.get(this, C::m_uiMapIndex), 6);
        assert_eq!(e.get(this, C::m_eMember), 4);
        assert_eq!(e.mem.u32(input + 4), 9);
        assert_eq!(calls_to(&e, STREAM_00A645B0), vec![vec![stream]]);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, 0x0a01_0068);
        e.mem.set_u32(input + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6970, &args![this, stream]);
        assert!(calls_to(&e, STREAM_00A645B0).is_empty());
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    type C = NiTextureTransformController;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn set_target_forgets_the_map_for_a_new_target() {
        let mut e = Engine::new();
        e.register(SET_TARGET, |_, a| a[1].into_ret());
        let this = e.new_object::<C>();
        e.set(this, C::m_pkTarget, 0x1000);
        e.set(this, C::m_pkMap, 0x2000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_61a0, &[this.addr(), 0x1000]).u32(), 0x1000);
        assert_eq!(e.get(this, C::m_pkMap), 0x2000);
        e.call(0x00a4_61a0, &[this.addr(), 0x3000]);
        assert_eq!(e.get(this, C::m_pkMap), 0);
        assert_eq!(
            calls_to(&e, SET_TARGET),
            vec![vec![this.addr(), 0x1000], vec![this.addr(), 0x3000]]
        );
    }

    #[test]
    fn is_equal_compares_the_map_settings() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |e, a| (e.mem.u32(a[1] + 4) == 0).into_ret());
        let this = e.new_object::<C>();
        let other = e.new_object::<C>();
        for object in [this, other] {
            e.set(object, C::m_uiLastIdx, 3);
            e.set(object, C::m_bShaderMap, 1);
            e.set(object, C::m_uiMapIndex, 2);
            e.set(object, C::m_eMember, 4);
        }
        let equal = |e: &mut Engine| e.call(0x00a4_61e0, &[this.addr(), other.addr()]).bool();
        assert!(equal(&mut e));
        e.mem.set_u32(other.addr() + 4, 1);
        assert!(!equal(&mut e));
        e.mem.set_u32(other.addr() + 4, 0);
        e.set(other, C::m_uiLastIdx, 4);
        assert!(!equal(&mut e));
        e.set(other, C::m_uiLastIdx, 3);
        e.set(other, C::m_bShaderMap, 0);
        assert!(!equal(&mut e));
        e.set(other, C::m_bShaderMap, 1);
        e.set(other, C::m_uiMapIndex, 0);
        assert!(!equal(&mut e));
        e.set(other, C::m_uiMapIndex, 2);
        e.set(other, C::m_eMember, 1);
        assert!(!equal(&mut e));
    }

    #[test]
    fn shader_map_00a463e0_is_range_checked() {
        let mut e = Engine::new();
        let property = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00a4_63e0, &[property, 0]).u32(), 0);
        let maps = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x5555);
        e.mem.set_u32(maps + 4, array);
        e.mem.set_u16(maps + 0xa, 2);
        e.mem.set_u32(property + 0x2c, maps);
        assert_eq!(e.call(0x00a4_63e0, &[property, 1]).u32(), 0x5555);
        assert_eq!(e.call(0x00a4_63e0, &[property, 2]).u32(), 0);
    }

    #[test]
    fn target_float_value_reads_the_member_of_the_transform() {
        let mut e = Engine::new();
        let this = e.new_object::<C>();
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x1234);
        // No target: SetInternalMap fails and the value stays.
        e.call(0x00a4_6660, &[this.addr(), value]);
        assert_eq!(e.mem.u32(value), 0x1234);
        let map = e.mem.alloc(0x10);
        e.set(this, C::m_pkMap, map);
        // A map without a transform: 0.
        e.call(0x00a4_6660, &[this.addr(), value]);
        assert_eq!(e.mem.u32(value), 0);
        let transform = e.mem.alloc(0x20);
        for i in 0..5u32 {
            e.mem.set_f32(transform + 4 * i, i as f32 + 0.5);
        }
        e.mem.set_u32(map + 0xc, transform);
        for member in 0..5u32 {
            e.set(this, C::m_eMember, member);
            e.call(0x00a4_6660, &[this.addr(), value]);
            assert_eq!(e.mem.f32(value), member as f32 + 0.5);
        }
        // A signalling NaN comes out quiet; a member above 4 leaves the value.
        e.mem.set_u32(transform + 8, 0x7f80_0001);
        e.set(this, C::m_eMember, 2);
        e.call(0x00a4_6660, &[this.addr(), value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0001);
        e.set(this, C::m_eMember, 5);
        e.call(0x00a4_6660, &[this.addr(), value]);
        assert_eq!(e.mem.u32(value), 0x7fc0_0001);
    }

    #[test]
    fn save_binary_writes_the_map_settings() {
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            if a[2] == 1 {
                assert_eq!(e.mem.u8(a[1]), 1);
            }
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.new_object::<C>();
        e.set(this, C::m_bShaderMap, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6a30, &[this.addr(), stream]);
        let writes: Vec<(u32, u32)> = calls_to(&e, 0x0300_0000)
            .iter()
            .map(|a| (a[1], a[2]))
            .collect();
        assert_eq!(
            writes[1..],
            [(this.addr() + 0x44, 4), (this.addr() + 0x48, 4)]
        );
        assert_eq!(writes[0].1, 1);
        assert_eq!(e.mem.u32(output + 4), 9);
        assert_eq!(
            calls_to(&e, BASE_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
    }
}
