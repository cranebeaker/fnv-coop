//! `fallout shared/bsanimgroupsequence.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a344f0` (named `NiControllerSequence::CreateClone` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const CREATE_CLONE_TARGET_00A344F0: u32 = 0x00a3_44f0;

// Translated from 004eec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::CreateClone` (Xbox PDB
/// `?CreateClone@BSAnimGroupSequence@@UAAPAVNiObject@@AAVNiCloningProcess@@@Z`, vtable word
/// 0101de84): forwards `this` and the `NiCloningProcess` reference to the base body `00a344f0`
/// and returns its `NiObject *`. Return type from the decorated name and from the callee,
/// whose last instructions are `MOV EAX,ESI` with the new clone in ESI.
pub fn bs_anim_group_sequence_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    e.call(CREATE_CLONE_TARGET_00A344F0, &args![this, cloning_process])
        .ptr()
}

/// `00a34540` (named `NiControllerSequence::GetViewerStrings` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const VIEWER_STRINGS_TARGET_00A34540: u32 = 0x00a3_4540;

// Translated from 004eec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::GetViewerStrings` (Xbox PDB
/// `?GetViewerStrings@BSAnimGroupSequence@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z`, vtable word
/// 0101de9c): forwards `this` and the `NiTPrimitiveArray<char *>` pointer to the base body
/// `00a34540`. Returns nothing: the decorated name is `X` (void), and the callee ends with its
/// stack cookie check, not a result.
pub fn bs_anim_group_sequence_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VIEWER_STRINGS_TARGET_00A34540, &args![this, strings]);
}

/// `0045cec0` (unnamed in the engine map): `thiscall`, no stack words; the `NiPointer` release
/// the game runs on a smart-pointer member or temporary.
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// `0066b0d0` (unnamed in the engine map): `thiscall`, one word (`RET 4`); the `NiPointer`
/// assignment (`*this = value`).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00537bd0` (unnamed in the engine map): `thiscall`, no stack words; on the
/// `NiControllerManager` (`m_pkOwner`), returns the object the sequence animates.
const OWNER_TARGET: u32 = 0x0053_7bd0;
/// `006815c0` (unnamed in the engine map): `thiscall`, no stack words; on an `IDTag`, returns
/// the address of its name (an `NiFixedString` the caller passes on).
const ID_TAG_NAME: u32 = 0x0068_15c0;
/// `0043b1b0` (unnamed in the engine map): `thiscall`, no stack words; on an `NiFixedString`,
/// returns its characters (null for an empty handle).
const FIXED_STRING_CHARS: u32 = 0x0043_b1b0;
/// The byte stored into `InterpArrayItem::m_ucBlendIdx` when an interpolator is dropped
/// (read from memory as the exe does).
const INVALID_BLEND_INDEX_01096968: u32 = 0x0109_6968;
/// `BSAnimGroupSequence`'s vtable (`004eeb00` stores it).
const BS_ANIM_GROUP_SEQUENCE_VTABLE: u32 = 0x0101_de3c;
/// The global word `004ef990` returns.
const GLOBAL_011C61C8: u32 = 0x011c_61c8;

/// `NiControllerSequence` fields (Xbox PDB, same offsets on PC; the layout lives in
/// `engine-gamebryo`'s `nicontrollersequence.rs`, read here at the offsets).
const SEQUENCE_ARRAY_SIZE: u32 = 0x0c;
const SEQUENCE_INTERP_ARRAY: u32 = 0x14;
const SEQUENCE_ID_TAG_ARRAY: u32 = 0x18;
const SEQUENCE_LAST_TIME: u32 = 0x34;
const SEQUENCE_WEIGHTED_LAST_TIME: u32 = 0x38;
const SEQUENCE_OWNER: u32 = 0x40;
const SEQUENCE_STATE: u32 = 0x44;
const SEQUENCE_OFFSET: u32 = 0x48;
const SEQUENCE_DEST_FRAME: u32 = 0x54;
const SEQUENCE_ACCUM_ROOT_NAME: u32 = 0x5c;
const SEQUENCE_ACCUM_ROOT: u32 = 0x60;
/// `BSAnimGroupSequence::spAnimGroup` (Xbox PDB) +0x74.
const SEQUENCE_ANIM_GROUP: u32 = 0x74;

/// Drops interpolator `index` of the sequence: `m_spInterpCtlr` (`+4` of the 0x10-byte
/// `InterpArrayItem`) is set to null through `0066b0d0`, `m_pkBlendInterp` (`+8`) cleared and
/// `m_ucBlendIdx` (`+0xc`) set to the byte at `01096968`.
fn drop_interpolator(e: &mut Engine, this: Ptr, index: u32) {
    let items = e.mem.u32(this.addr() + SEQUENCE_INTERP_ARRAY);
    e.call(
        POINTER_ASSIGN,
        &args![items.wrapping_add(index << 4).wrapping_add(4), 0u32],
    );
    let items = e.mem.u32(this.addr() + SEQUENCE_INTERP_ARRAY);
    e.mem
        .set_u32(items.wrapping_add(index << 4).wrapping_add(8), 0);
    let items = e.mem.u32(this.addr() + SEQUENCE_INTERP_ARRAY);
    let invalid = e.mem.u8(INVALID_BLEND_INDEX_01096968);
    e.mem
        .set_u8(items.wrapping_add(index << 4).wrapping_add(0xc), invalid);
}

/// The address of `IDTag` `index` (0x14 bytes each) of the sequence.
fn id_tag(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem
        .u32(this.addr() + SEQUENCE_ID_TAG_ARRAY)
        .wrapping_add(index.wrapping_mul(0x14))
}

// Translated from 004eeb00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004eeb00 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `BSAnimGroupSequence::~BSAnimGroupSequence` (Xbox PDB): stores the class's vtable
/// `0101de3c`, releases `spAnimGroup` (`+0x74`, `0045cec0`) and runs the
/// `NiControllerSequence` destructor `00a35640`.
pub fn bs_anim_group_sequence_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_ANIM_GROUP_SEQUENCE_VTABLE);
    e.call(
        POINTER_RELEASE,
        &args![this.addr().wrapping_add(SEQUENCE_ANIM_GROUP)],
    );
    e.call(0x00a3_5640, &args![this]);
}

// Translated from 004eeb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004eeb60 - the C++ exception-unwinding frame (FS:[0]) that releases the temporary and frees the block on a throw is not translated
/// Unnamed in the engine map (`cdecl`, three words): makes a `BSAnimGroupSequence` from
/// `first` and `second`. A temporary `NiPointer` (one word on the stack, built by
/// `00633c90(temp, 0)`) is filled by `00a35700(first, second, &temp)`; 0x78 bytes are taken
/// from `00aa13e0` and, when that succeeded, constructed by `004ee9f0(block, 0,
/// 00559450(&temp))` (the pointer's object); the result (0 for a failed allocation) is
/// assigned to the `NiPointer` at `result` (`0066b0d0`) and the temporary is released
/// (`0045cec0`).
pub fn fn_004eeb60(e: &mut Engine, first: u32, second: u32, result: Ptr) {
    e.with_stack(4, |e, temp| {
        e.call(0x0063_3c90, &args![temp, 0u32]);
        e.call(0x00a3_5700, &args![first, second, temp]);
        let block = e.call(0x00aa_13e0, &args![0x78u32]).u32();
        let sequence = if block != 0 {
            let object = e.call(0x0055_9450, &args![temp]).u32();
            e.call(0x004e_e9f0, &args![block, 0u32, object]).u32()
        } else {
            0
        };
        e.call(POINTER_ASSIGN, &args![result, sequence]);
        e.call(POINTER_RELEASE, &args![temp]);
    });
}

// Translated from 004eec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::GetScaledTime` (Xbox PDB): 0.0 unless `m_eState` (`+0x44`, signed)
/// is 1 to 3; otherwise `time + m_fOffset` (`+0x48`, stored as a `float`) goes through
/// `NiControllerSequence::ComputeScaledTime` (`00a30970`, with 1) and its `ST0` is the result.
pub fn bs_anim_group_sequence_get_scaled_time(e: &mut Engine, this: Ptr, time: f32) -> f64 {
    let state = e.mem.u32(this.addr() + SEQUENCE_STATE) as i32;
    if state <= 0 || state > 3 {
        return 0.0;
    }
    let offset = e.mem.u32(this.addr() + SEQUENCE_OFFSET);
    let sum = x87_store32(x87_add(x87_load32(time.to_bits()), x87_m32(offset)));
    e.call(0x00a3_0970, &args![this, sum, 1u32]).f64()
}

// Translated from 004eecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: rebinds the sequence to its owner's target. Without an owner
/// (`m_pkOwner`, `+0x40`) nothing happens. The target is `00537bd0(owner)`; an accumulation
/// root (`+0x60`) is looked up again by its own name (`00413f40(root)`) through the target's
/// vtable slot `0x8c`. Then for each `IDTag` (`m_uiArraySize`, `+0xc`): the object of the
/// target's vtable slot `0x98` is asked through its slot `0x9c` about the tag's name
/// (`006815c0`); when it answers 0, the target's slot `0x90` is called with the name and 0 and
/// the interpolator is dropped (controller pointer nulled, blend interpolator cleared, blend
/// index set to the byte at `01096968`).
pub fn fn_004eecb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + SEQUENCE_OWNER) == 0 {
        return;
    }
    let owner = e.mem.u32(base + SEQUENCE_OWNER);
    let target = e.call(OWNER_TARGET, &args![owner]).u32();
    let accum_root = e.mem.u32(base + SEQUENCE_ACCUM_ROOT);
    if accum_root != 0 {
        let name = e.call(0x0041_3f40, &args![accum_root]).u32();
        let found = e.vcall(target, 0x8c, &args![name]).u32();
        e.mem.set_u32(base + SEQUENCE_ACCUM_ROOT, found);
    }
    let mut index = 0u32;
    while index < e.mem.u32(base + SEQUENCE_ARRAY_SIZE) {
        let tag = id_tag(e, this, index);
        let holder = e.vcall(target, 0x98, &args![]).u32();
        let name = e.call(ID_TAG_NAME, &args![tag]).u32();
        if e.vcall(holder, 0x9c, &args![name]).u32() == 0 {
            let name = e.call(ID_TAG_NAME, &args![tag]).u32();
            e.vcall(target, 0x90, &args![name, 0u32]);
            drop_interpolator(e, this, index);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 004eedd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): returns `00a31910(this, first, target, second)`, the
/// target being `00537bd0` on the owner (`m_pkOwner`, `+0x40`). The exe pushes `second` before
/// calling `00537bd0` (which takes no stack word), so it is `00a31910`'s third word. The engine
/// map names `00a31910` `NiControllerSequence::IDTag::IsTreadController` (`RET 0xc`); its `EAX`
/// is passed back.
pub fn fn_004eedd0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let owner = e.mem.u32(this.addr() + SEQUENCE_OWNER);
    let target = e.call(OWNER_TARGET, &args![owner]).u32();
    e.call(0x00a3_1910, &args![this, first, target, second])
        .u32()
}

// Translated from 004eee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: rebinds the sequence to its owner's target by name. The target
/// is `00537bd0(m_pkOwner)` (the owner is not checked); an accumulation root (`+0x60`) is looked
/// up again through the target's vtable slot `0x8c` with `m_kAccumRootName` (`+0x5c`, by
/// address). Then each interpolator whose `IDTag` name has no characters (`0043b1b0` of
/// `006815c0(tag)` is null) or is not found through the target's slot `0x8c` is dropped
/// (controller pointer nulled, blend interpolator cleared, blend index set to the byte at
/// `01096968`).
pub fn fn_004eee00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let owner = e.mem.u32(base + SEQUENCE_OWNER);
    let target = e.call(OWNER_TARGET, &args![owner]).u32();
    if e.mem.u32(base + SEQUENCE_ACCUM_ROOT) != 0 {
        let found = e
            .vcall(target, 0x8c, &args![base + SEQUENCE_ACCUM_ROOT_NAME])
            .u32();
        e.mem.set_u32(base + SEQUENCE_ACCUM_ROOT, found);
    }
    let mut index = 0u32;
    while index < e.mem.u32(base + SEQUENCE_ARRAY_SIZE) {
        let tag = id_tag(e, this, index);
        let name = e.call(ID_TAG_NAME, &args![tag]).u32();
        let chars = e.call(FIXED_STRING_CHARS, &args![name]).u32();
        let keep = chars != 0 && {
            let name = e.call(ID_TAG_NAME, &args![tag]).u32();
            e.vcall(target, 0x8c, &args![name]).u32() != 0
        };
        if !keep {
            drop_interpolator(e, this, index);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 004ef970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): `0043b370(this, flag, 0x20000)` with the low byte of
/// `flag` zero-extended.
pub fn fn_004ef970(e: &mut Engine, this: Ptr, flag: u32) {
    e.call(0x0043_b370, &args![this, u32::from(flag as u8), 0x20000u32]);
}

// Translated from 004ef990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns the word at `011c61c8` (no arguments are read).
pub fn fn_004ef990(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_011C61C8)
}

// Translated from 004ef9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::GetPriority` (Xbox PDB), `RET 8`: finds the interpolator whose
/// `IDTag` name (`0043b1b0(006815c0(tag))`) compares equal to `name` through `00404dc0(chars,
/// name)` (0 = equal). A tag without characters is reported through `005b5e40(010238b0,
/// chars of the sequence's name)` (`00413f40` on the sequence, then `0043b1b0`) and skipped.
/// On a match the index is stored at `index_out` when that is not null and the item's
/// priority byte (`InterpArrayItem` `+0xd`) is returned; without one, 0.
pub fn bs_anim_group_sequence_get_priority(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    index_out: Ptr,
) -> u8 {
    let base = this.addr();
    let mut index = 0u32;
    while index < e.mem.u32(base + SEQUENCE_ARRAY_SIZE) {
        let tag = id_tag(e, this, index);
        let tag_name = e.call(ID_TAG_NAME, &args![tag]).u32();
        let chars = e.call(FIXED_STRING_CHARS, &args![tag_name]).u32();
        if chars == 0 {
            let own_name = e.call(0x0041_3f40, &args![this]).u32();
            let own_chars = e.call(FIXED_STRING_CHARS, &args![own_name]).u32();
            e.call(0x005b_5e40, &args![0x0102_38b0u32, own_chars]);
        }
        if chars != 0 && e.call(0x0040_4dc0, &args![chars, name]).u32() == 0 {
            if !index_out.is_null() {
                e.mem.set_u32(index_out.addr(), index);
            }
            let items = e.mem.u32(base + SEQUENCE_INTERP_ARRAY);
            return e.mem.u8(items.wrapping_add(index << 4).wrapping_add(0xd));
        }
        index = index.wrapping_add(1);
    }
    0
}

// Translated from 004efc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::SaveGame` (Xbox PDB), `RET 8`: writes through `00865e50(buffer, data,
/// size, 0)` the `float` `m_fOffset + time` (`+0x48`), the low byte of `m_eState` (`+0x44`),
/// `m_fLastTime` (`+0x34`), `m_fWeightedLastTime` (`+0x38`) and `m_fDestFrame` (`+0x54`).
pub fn bs_anim_group_sequence_save_game(e: &mut Engine, this: Ptr, buffer: Ptr, time: f32) {
    let base = this.addr();
    let offset = e.mem.u32(base + SEQUENCE_OFFSET);
    let sum = x87_store32(x87_add(x87_load32(offset), x87_m32(time.to_bits())));
    e.with_stack(8, |e, locals| {
        let time_slot = locals.addr();
        let state_slot = time_slot + 4;
        e.mem.set_u32(time_slot, sum);
        e.call(0x0086_5e50, &args![buffer, time_slot, 4u32, 0u32]);
        let state = e.mem.u8(base + SEQUENCE_STATE);
        e.mem.set_u8(state_slot, state);
        e.call(0x0086_5e50, &args![buffer, state_slot, 1u32, 0u32]);
    });
    for field in [
        SEQUENCE_LAST_TIME,
        SEQUENCE_WEIGHTED_LAST_TIME,
        SEQUENCE_DEST_FRAME,
    ] {
        e.call(0x0086_5e50, &args![buffer, base + field, 4u32, 0u32]);
    }
}

// Translated from 004efdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::Deactivate` (Xbox PDB), `RET 8`. With an accumulation root (`+0x60`)
/// and an animation group (`spAnimGroup`, `+0x74`, through `00559450`) whose `005f2420` value
/// is 0x17 to 0x19 (signed): the root's controller of RTTI `012031b8`
/// (`NiObjectNET::GetController`, `00a5c570`), when there is one and `005f36f0` on it is
/// non-zero, leads to the controller of RTTI `011f36bc` of `009611e0(root)`; when that exists,
/// `004f0380(controller, m_fLastTime, 1)` runs (`+0x34`, copied as a `float`). Then
/// `NiControllerSequence::Deactivate(this, ease_out, transition)` (`00a35030`, the low byte of
/// `transition` zero-extended), whose answer is this one's: the exe ends on that call and keeps
/// its `AL` (the PDB's `bool`).
pub fn bs_anim_group_sequence_deactivate(
    e: &mut Engine,
    this: Ptr,
    ease_out: f32,
    transition: u32,
) -> bool {
    let base = this.addr();
    if e.mem.u32(base + SEQUENCE_ACCUM_ROOT) != 0 {
        let group = e
            .call(0x0055_9450, &args![base + SEQUENCE_ANIM_GROUP])
            .u32();
        let kind = e.call(0x005f_2420, &args![group]).i32();
        if (0x17..=0x19).contains(&kind) {
            let root = e.mem.u32(base + SEQUENCE_ACCUM_ROOT);
            let first = e.call(0x00a5_c570, &args![root, 0x0120_31b8u32]).u32();
            if first != 0 && e.call(0x005f_36f0, &args![first]).u32() != 0 {
                let root = e.mem.u32(base + SEQUENCE_ACCUM_ROOT);
                let object = e.call(0x0096_11e0, &args![root]).u32();
                let controller = e.call(0x00a5_c570, &args![object, 0x011f_36bcu32]).u32();
                if controller != 0 {
                    let last_time = x87_quiet_bits(e.mem.u32(base + SEQUENCE_LAST_TIME));
                    e.call(0x004f_0380, &args![controller, last_time, 1u32]);
                }
            }
        }
    }
    let ease_out = x87_quiet_bits(ease_out.to_bits());
    e.call(
        0x00a3_5030,
        &args![this, ease_out, u32::from(transition as u8)],
    )
    .bool()
}
/// The x87 condition bits `FCOM`/`FUCOM` leave in `AH` after `FNSTSW AX` for `st0` compared
/// with `st1`: C0 (0x01) for less, C3 (0x40) for equal, C3|C2|C0 (0x45) for unordered, none for
/// greater.
fn fpu_compare(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 < st1 {
        0x01
    } else if st0 == st1 {
        0x40
    } else {
        0
    }
}

/// The x86 parity flag of `TEST AH,mask`: set when the masked byte has an even number of bits.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// `"Sound: "`.
const SOUND_PREFIX_010238A8: u32 = 0x0102_38a8;
/// `"Enum: StopSounds "`.
const STOP_SOUNDS_PREFIX_01023894: u32 = 0x0102_3894;
/// `"Enum: HitShader"`.
const HIT_SHADER_PREFIX_01023884: u32 = 0x0102_3884;
/// `FLT_MAX` (`float`); the code negates it after loading it.
const FLOAT_MAX_01023850: u32 = 0x0102_3850;
/// The player character (`PlayerCharacter *`).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// `TESDataHandler *`.
const DATA_HANDLER_011C3F2C: u32 = 0x011c_3f2c;
/// Three words (a zero point) copied as the sound's default position.
const DEFAULT_POSITION_011F426C: u32 = 0x011f_426c;
/// `_strnicmp` (`cdecl`: first, second, count).
const STRNICMP: u32 = 0x00ec_7ec0;
/// `strchr` (`cdecl`: string, character); the engine map places it in `modelloader.cpp`.
const STRCHR: u32 = 0x0043_b9d0;
/// A string copy into a 32-byte buffer (`cdecl`: destination, source).
const COPY_STRING: u32 = 0x0040_46f0;
/// `BSSoundHandle` destructor (`thiscall`).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;

/// The size of `PlaySounds`'s stack frame below `EBP` modelled here: every local whose address
/// the code passes on lives at its exe offset (`frame + FRAME - offset`).
const PLAY_SOUNDS_FRAME: u32 = 0x1c0;

// Translated from 004eef00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004eef00 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handles and fixed strings on a throw, and the /GS cookie check (00ec408c), are not translated
/// `BSAnimGroupSequence::PlaySounds` (Xbox PDB), `cdecl` (`sequence`, `actor`), result in `AL`.
///
/// Only for a `sequence` whose `008041a0` value is 1. The window is `[begin, end)` with
/// `begin = 00507b00(sequence)` and `end = 00598040(sequence)` (both stored as `float`s); it is
/// skipped when either is `-FLT_MAX` or they are equal, and wraps when `begin > end`. Each text
/// key of the list `00495f30(00700300(sequence), &count)` (8 bytes each; the list is only asked
/// for when `00700300` is non-zero) whose text (`0043b1b0(00717e50(key))`) is not null and whose
/// time (`006a7f50(key)`, as a `float`) is in the window is handled:
/// - `"Sound: <EDID> [<node>]"`: the text is cut at its `'\r'` and at the space after the
///   name (the node name is copied out), the sound is `TESDataHandler::GetSound_ov2(data
///   handler, name)`, and both cuts are restored (the space after the key is done).
/// - `"Enum: StopSounds [<node>]"`: the node (`actor` vtable `0x1d0`, then its slot `0x9c` with
///   the name as a fixed string, or with `004ef990()` without one; the `0x1d0` object itself
///   when that fails) has its moving sounds stopped (`BSAudio::StopMovingSounds(audio, node,
///   1.0, 2.0)`).
/// - `"Enum: HitShader"`: `PlayerCharacter::PlayImpactBasedHitShader(player, actor)`.
///
/// A sound plays through a `BSSoundHandle` made by `BSAudio::GetSoundHandleByNumericID` with
/// the sound's form id and flags built from the sound's data (`004e75d0`, `005e39b0`), 0x200000
/// for the player, and 0x40002002 / 0x40002101 / 0x40002102 for an actor or 0x40000022 /
/// 0x40000121 / 0x40000122 without one; with an actor the sound follows the node and starts at
/// its position (`0045bb80`), and an actor's sound that is neither 2D nor flagged must pass
/// `0082eca0(sound, position, 1.0)` first. Afterwards, when `0059bb30(sequence)` is non-zero, the
/// result is 1 unless `00639aa0(sequence)` is `-FLT_MAX` or `00598040(sequence) <
/// 00508100(sequence)`; otherwise 0.
pub fn bs_anim_group_sequence_play_sounds(e: &mut Engine, sequence: Ptr, actor: Ptr) -> bool {
    if sequence.is_null() || e.call(0x0080_41a0, &args![sequence]).u32() != 1 {
        return false;
    }
    e.with_stack(PLAY_SOUNDS_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + PLAY_SOUNDS_FRAME - offset;
        play_sounds_keys(e, sequence, actor, &at);
    });
    if e.call(0x0059_bb30, &args![sequence]).u32() == 0 {
        return false;
    }
    let limit = e.call(0x0063_9aa0, &args![sequence]).f64();
    let negative_max = -x87_load32(e.mem.u32(FLOAT_MAX_01023850));
    if !parity_even(fpu_compare(negative_max, limit) & 0x44) {
        return false;
    }
    let end = e.call(0x0059_8040, &args![sequence]).f64();
    let other = e.call(0x0050_8100, &args![sequence]).f64();
    fpu_compare(other, end) & 0x41 != 0
}

/// The text-key loop of `PlaySounds` (`004eef00`), with the frame's address-taken locals at
/// `at(offset below EBP)`.
fn play_sounds_keys(e: &mut Engine, sequence: Ptr, actor: Ptr, at: &dyn Fn(u32) -> u32) {
    let begin_bits = {
        let value = e.call(0x0050_7b00, &args![sequence]).f64();
        x87_store32(value)
    };
    let end_bits = {
        let value = e.call(0x0059_8040, &args![sequence]).f64();
        x87_store32(value)
    };
    let begin = x87_load32(begin_bits);
    let end = x87_load32(end_bits);
    let max = -x87_load32(e.mem.u32(FLOAT_MAX_01023850));
    if !parity_even(fpu_compare(begin, max) & 0x44)
        || !parity_even(fpu_compare(end, max) & 0x44)
        || !parity_even(fpu_compare(end, begin) & 0x44)
    {
        return;
    }
    let wrapped = fpu_compare(begin, end) & 0x41 == 0;
    let count_slot = at(0x20);
    e.mem.set_u32(count_slot, 0);
    let mut keys = 0u32;
    if e.call(0x0070_0300, &args![sequence]).u32() != 0 {
        let holder = e.call(0x0070_0300, &args![sequence]).u32();
        keys = e.call(0x0049_5f30, &args![holder, count_slot]).u32();
    }
    let mut index = 0u32;
    while index < e.mem.u32(count_slot) {
        let key = keys.wrapping_add(index.wrapping_mul(8));
        let text_handle = e.call(0x0071_7e50, &args![key]).u32();
        let text = e.call(FIXED_STRING_CHARS, &args![text_handle]).u32();
        if text != 0 {
            play_sounds_key(e, actor, key, text, begin, end, wrapped, at);
        }
        index = index.wrapping_add(1);
    }
}

/// The node of `actor` named by the 32-byte string at `name` (`actor` vtable `0x1d0`, then that
/// object's slot `0x9c` with the name as an `NiFixedString` built at `fixed` by `00438170` and
/// released by `004381b0`), or with `004ef990()` when the name is empty.
fn play_sounds_find_node(e: &mut Engine, actor: Ptr, name: u32, fixed: u32) -> u32 {
    if e.mem.u8(name) != 0 {
        let object = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
        let handle = e.call(0x0043_8170, &args![fixed, name]).u32();
        let node = e.vcall(object, 0x9c, &args![handle]).u32();
        e.call(0x0043_81b0, &args![fixed]);
        node
    } else {
        let object = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
        let fallback = fn_004ef990(e);
        e.vcall(object, 0x9c, &args![fallback]).u32()
    }
}

/// One text key of `PlaySounds` (`004eef00`), from the window test to the restored space.
#[allow(clippy::too_many_arguments)]
fn play_sounds_key(
    e: &mut Engine,
    actor: Ptr,
    key: u32,
    text: u32,
    begin: f64,
    end: f64,
    wrapped: bool,
    at: &dyn Fn(u32) -> u32,
) {
    let time = x87_load32(x87_store32(e.call(0x006a_7f50, &args![key]).f64()));
    let mut play = false;
    let mut node = 0u32;
    let normal_window = |play: &mut bool| {
        if parity_even(fpu_compare(begin, time) & 0x41) {
            return;
        }
        if fpu_compare(end, time) & 0x41 != 0 {
            return;
        }
        *play = true;
    };
    if wrapped {
        if !parity_even(fpu_compare(begin, time) & 0x05) {
            play = true;
        } else if fpu_compare(end, time) & 0x01 != 0 {
            normal_window(&mut play);
        } else {
            play = true;
        }
    } else {
        normal_window(&mut play);
    }
    if play
        && e.call(STRNICMP, &args![SOUND_PREFIX_010238A8, text, 7u32])
            .u32()
            != 0
    {
        play = false;
        if e.call(STRNICMP, &args![STOP_SOUNDS_PREFIX_01023894, text, 0x11u32])
            .u32()
            == 0
        {
            let buffer = at(0x84);
            e.mem.set_u8(buffer, 0);
            let carriage_return = e.call(STRCHR, &args![text, 0xdu32]).u32();
            if carriage_return != 0 {
                e.mem.set_u8(carriage_return, 0);
            }
            let rest = text.wrapping_add(0x10);
            if e.mem.u8(rest) != 0 && e.mem.u8(rest.wrapping_add(1)) != 0 {
                e.call(COPY_STRING, &args![buffer, rest.wrapping_add(1)]);
            }
            if carriage_return != 0 {
                e.mem.set_u8(carriage_return, 0xd);
            }
            if e.vcall(actor.addr(), 0x1d0, &args![]).u32() != 0 {
                node = play_sounds_find_node(e, actor, buffer, at(0xb8));
                if node == 0 {
                    node = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
                }
            }
            if node != 0 {
                let audio = e.call(0x0045_3a70, &args![]).u32();
                e.call(
                    0x00ad_8570,
                    &args![audio, node, 1.0f32.to_bits(), 0x4000_0000u32],
                );
            }
        } else if e
            .call(STRNICMP, &args![HIT_SHADER_PREFIX_01023884, text, 0xfu32])
            .u32()
            == 0
        {
            let player = e.mem.u32(PLAYER_011DEA3C);
            e.call(0x0096_2290, &args![player, actor]);
        }
    }
    let mut sound = 0u32;
    let mut space = 0u32;
    let name = at(0x4c);
    e.mem.set_u8(name, 0);
    if play {
        let carriage_return = e.call(STRCHR, &args![text, 0xdu32]).u32();
        if carriage_return != 0 {
            e.mem.set_u8(carriage_return, 0);
        }
        space = e.call(STRCHR, &args![text.wrapping_add(7), 0x20u32]).u32();
        if space != 0 {
            e.call(COPY_STRING, &args![name, space.wrapping_add(1)]);
            e.mem.set_u8(space, 0);
        }
        let data_handler = e.mem.u32(DATA_HANDLER_011C3F2C);
        sound = e
            .call(0x0046_16c0, &args![data_handler, text.wrapping_add(7)])
            .u32();
        if carriage_return != 0 {
            e.mem.set_u8(carriage_return, 0xd);
        }
    }
    if sound != 0 {
        play_sounds_sound(e, actor, sound, node, at);
    }
    if space != 0 {
        e.mem.set_u8(space, 0x20);
    }
}

/// The sound's data word at `+4` of what `004e75d0(sound, out)` returns (`out` is a 0x24-byte
/// local).
fn sound_data_flags(e: &mut Engine, sound: u32, out: u32) -> u32 {
    let data = e.call(0x004e_75d0, &args![sound, out]).u32();
    e.mem.u32(data.wrapping_add(4))
}

/// Makes the sound handle at `handle` from `sound` with `flags`: `BSAudio::GetSoundHandleByNumericID(
/// audio, temp, form id, flags)` into the temporary handle at `temp`, assigned to `handle`
/// (`00418900`) and the temporary destroyed.
fn make_sound_handle(e: &mut Engine, handle: u32, sound: u32, flags: u32, temp: u32) {
    let form_id = e.call(0x0084_e3a0, &args![sound]).u32();
    let audio = e.call(0x0045_3a70, &args![]).u32();
    let made = e
        .call(0x00ad_73b0, &args![audio, temp, form_id, flags])
        .u32();
    e.call(0x0041_8900, &args![handle, made]);
    e.call(SOUND_HANDLE_DESTROY, &args![temp]);
}

/// The playing part of `PlaySounds` (`004eef00`) for a found sound.
fn play_sounds_sound(e: &mut Engine, actor: Ptr, sound: u32, node: u32, at: &dyn Fn(u32) -> u32) {
    let mut node = node;
    let handle = at(0xb4);
    e.call(0x0041_a250, &args![handle]);
    let mut position = [
        e.mem.u32(DEFAULT_POSITION_011F426C),
        e.mem.u32(DEFAULT_POSITION_011F426C + 4),
        e.mem.u32(DEFAULT_POSITION_011F426C + 8),
    ];
    let data = sound_data_flags(e, sound, at(0xdc));
    let mut flags = e.call(0x005e_39b0, &args![data]).u32();
    let two_d = sound_data_flags(e, sound, at(0x100)) & 0x40 != 0
        && sound_data_flags(e, sound, at(0x124)) & 0x800 == 0;
    let flagged = flags & 0x10 != 0;
    if actor.addr() == e.mem.u32(PLAYER_011DEA3C) {
        flags |= 0x0020_0000;
    }
    if !actor.is_null() && e.vcall(actor.addr(), 0x1d0, &args![]).u32() != 0 {
        node = play_sounds_find_node(e, actor, at(0x4c), at(0x128));
        if flagged && node == 0 {
            node = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
        }
        if node != 0 {
            fn_004ef970(e, Ptr::new(node), 1);
        }
        let object = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
        fn_004ef970(e, Ptr::new(object), 1);
        let source = if node != 0 {
            e.call(0x0045_bb80, &args![node]).u32()
        } else {
            let object = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
            e.call(0x0045_bb80, &args![object]).u32()
        };
        for (slot, word) in position.iter_mut().enumerate() {
            *word = e.mem.u32(source + 4 * slot as u32);
        }
    }
    let audible = actor.is_null()
        || flagged
        || two_d
        || e.call(
            0x0082_eca0,
            &args![sound, position[0], position[1], position[2], 1.0f32],
        )
        .u8()
            != 0;
    if !actor.is_null() && audible {
        if flagged {
            make_sound_handle(e, handle, sound, flags | 0x4000_2002, at(0x134));
        } else if sound_data_flags(e, sound, at(0x158)) & 0x40 != 0 {
            make_sound_handle(e, handle, sound, flags | 0x4000_2101, at(0x164));
        } else {
            make_sound_handle(e, handle, sound, flags | 0x4000_2102, at(0x170));
        }
    } else if actor.is_null() {
        if flagged {
            make_sound_handle(e, handle, sound, flags | 0x4000_0022, at(0x17c));
        } else if sound_data_flags(e, sound, at(0x1a0)) & 0x40 != 0 {
            make_sound_handle(e, handle, sound, flags | 0x4000_0121, at(0x1ac));
        } else {
            make_sound_handle(e, handle, sound, flags | 0x4000_0122, at(0x1b8));
        }
    }
    if node != 0 {
        e.call(0x00ad_8f20, &args![handle, node]);
    }
    e.call(
        0x00ad_8b60,
        &args![
            handle,
            x87_quiet_bits(position[0]),
            x87_quiet_bits(position[1]),
            x87_quiet_bits(position[2])
        ],
    );
    e.call(0x00ad_8830, &args![handle, u32::from(flagged)]);
    e.call(SOUND_HANDLE_DESTROY, &args![handle]);
}

/// The object at `011de45c` whose methods (`tessaveloadgame.cpp` in the
/// engine map) write (`008579b0`) and read (`008579e0`) bytes (`RET 8`:
/// address, size) and answer a version byte (`008df040`).
const SAVE_LOAD_OBJECT_011DE45C: u32 = 0x011d_e45c;
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
const SAVE_LOAD_VERSION: u32 = 0x008d_f040;
/// Reads bytes from a load buffer (`00864980`, `this` = buffer; `RET 8`:
/// address, size).
const LOAD_BUFFER_READ: u32 = 0x0086_4980;
/// `m_fOffset`'s neighbours that the loads reset (`+0x4C`, `+0x50`).
const SEQUENCE_AFTER_OFFSET_1: u32 = 0x4c;
const SEQUENCE_AFTER_OFFSET_2: u32 = 0x50;

/// The version byte of the object at `011de45c`.
fn save_load_version(e: &mut Engine) -> u8 {
    let object = e.mem.u32(SAVE_LOAD_OBJECT_011DE45C);
    e.call(SAVE_LOAD_VERSION, &args![object]).u8()
}

/// Writes (`008579b0`) or reads (`008579e0`) `size` bytes at `at` through
/// the object at `011de45c`.
fn save_load_bytes(e: &mut Engine, function: u32, at: u32, size: u32) {
    let object = e.mem.u32(SAVE_LOAD_OBJECT_011DE45C);
    e.call(function, &args![object, at, size]);
}

/// The tail both loads share: `+0x4C` and `+0x50` set to `-FLT_MAX` (the
/// constant at `01023850` loaded and negated), then
/// `BSAnimGroupSequence::GetScaledTime(time)` (passed through `FLD`/`FSTP`,
/// its `ST0` dropped).
fn reset_after_load(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    for field in [SEQUENCE_AFTER_OFFSET_1, SEQUENCE_AFTER_OFFSET_2] {
        let max = e.mem.u32(FLOAT_MAX_01023850);
        e.mem
            .set_u32(base + field, x87_quiet_bits(max) ^ 0x8000_0000);
    }
    bs_anim_group_sequence_get_scaled_time(e, this, x87_quiet(time));
}

// Translated from 004efa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +8 of each interpolator item (`m_pkInterpArray`,
/// `+0x14`, 0x10 bytes each) for the item count `0084e3a0(this)` answers
/// (the word at +0xC, `m_uiArraySize`), asked again before each item.
pub fn fn_004efa50(e: &mut Engine, this: Ptr) {
    let mut index = 0u32;
    while index < e.call(0x0084_e3a0, &args![this]).u32() {
        let items = e.mem.u32(this.addr() + SEQUENCE_INTERP_ARRAY);
        e.mem
            .set_u32(items.wrapping_add(index << 4).wrapping_add(8), 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 004efaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of a sequence's save data: 20 bytes, 24 when the version byte
/// of the object at `011de45c` is below 0x71 (no argument is read).
pub fn fn_004efaa0(e: &mut Engine) -> u16 {
    let mut size: u16 = 8;
    if save_load_version(e) < 0x71 {
        size += 4;
    }
    size + 12
}

// Translated from 004efb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `fn_004efaa0` (`this` unread) and passes its `AX` on.
pub fn fn_004efb10(e: &mut Engine, _this: Ptr) -> u16 {
    fn_004efaa0(e)
}

// Translated from 004efb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the sequence through the object at `011de45c` (`RET 4`): the
/// `float` `m_fOffset + time` (`+0x48`), the word `m_eState` (`+0x44`), the
/// offset sum again when the version byte is below 0x71, `m_fLastTime`
/// (`+0x34`), `m_fWeightedLastTime` (`+0x38`) and `m_fDestFrame` (`+0x54`),
/// four bytes each.
pub fn fn_004efb20(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let offset = e.mem.u32(base + SEQUENCE_OFFSET);
    let sum = x87_store32(x87_add(x87_load32(offset), x87_m32(time.to_bits())));
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), sum);
        save_load_bytes(e, SAVE_LOAD_WRITE, local.addr(), 4);
        save_load_bytes(e, SAVE_LOAD_WRITE, base + SEQUENCE_STATE, 4);
        if save_load_version(e) < 0x71 {
            save_load_bytes(e, SAVE_LOAD_WRITE, local.addr(), 4);
        }
    });
    for field in [
        SEQUENCE_LAST_TIME,
        SEQUENCE_WEIGHTED_LAST_TIME,
        SEQUENCE_DEST_FRAME,
    ] {
        save_load_bytes(e, SAVE_LOAD_WRITE, base + field, 4);
    }
}

// Translated from 004efbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the sequence through the object at `011de45c` (`RET 4`), the
/// reverse of `fn_004efb20`: the saved offset sum into a stack `float`,
/// `m_eState` (`+0x44`, a word), the sum again when the version byte is
/// below 0x71; `m_fOffset` (`+0x48`) becomes the sum minus `time`; then
/// `m_fLastTime`, `m_fWeightedLastTime` and `m_fDestFrame`, the fields at
/// `+0x4C` and `+0x50` set to `-FLT_MAX`, and `GetScaledTime(time)` runs.
pub fn fn_004efbc0(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let sum = e.with_stack(4, |e, local| {
        save_load_bytes(e, SAVE_LOAD_READ, local.addr(), 4);
        save_load_bytes(e, SAVE_LOAD_READ, base + SEQUENCE_STATE, 4);
        if save_load_version(e) < 0x71 {
            save_load_bytes(e, SAVE_LOAD_READ, local.addr(), 4);
        }
        e.mem.u32(local.addr())
    });
    let offset = x87_store32(x87_sub(x87_load32(sum), x87_m32(time.to_bits())));
    e.mem.set_u32(base + SEQUENCE_OFFSET, offset);
    for field in [
        SEQUENCE_LAST_TIME,
        SEQUENCE_WEIGHTED_LAST_TIME,
        SEQUENCE_DEST_FRAME,
    ] {
        save_load_bytes(e, SAVE_LOAD_READ, base + field, 4);
    }
    reset_after_load(e, this, time);
}

// Translated from 004efd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the sequence from `buffer` (`RET 8`; `00864980` reads), the
/// reverse of `BSAnimGroupSequence::SaveGame`: a `float` (0.0 before the
/// read) less `time` becomes `m_fOffset` (`+0x48`), a byte zero-extended
/// becomes `m_eState` (`+0x44`), then `m_fLastTime`, `m_fWeightedLastTime`
/// and `m_fDestFrame`; the fields at `+0x4C` and `+0x50` are set to
/// `-FLT_MAX` and `GetScaledTime(time)` runs.
pub fn fn_004efd10(e: &mut Engine, this: Ptr, buffer: Ptr, time: f32) {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let value = locals.addr();
        let state = value + 7;
        e.mem.set_u32(value, 0.0f32.to_bits());
        e.call(LOAD_BUFFER_READ, &args![buffer, value, 4u32]);
        let read = e.mem.u32(value);
        let offset = x87_store32(x87_sub(x87_load32(read), x87_m32(time.to_bits())));
        e.mem.set_u32(base + SEQUENCE_OFFSET, offset);
        e.mem.set_u8(state, 0);
        e.call(LOAD_BUFFER_READ, &args![buffer, state, 1u32]);
        let byte = e.mem.u8(state);
        e.mem.set_u32(base + SEQUENCE_STATE, u32::from(byte));
    });
    for field in [
        SEQUENCE_LAST_TIME,
        SEQUENCE_WEIGHTED_LAST_TIME,
        SEQUENCE_DEST_FRAME,
    ] {
        e.call(LOAD_BUFFER_READ, &args![buffer, base + field, 4u32]);
    }
    reset_after_load(e, this, time);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_ec20, bs_anim_group_sequence_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x004e_ec40,
            bs_anim_group_sequence_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x004e_eb00, bs_anim_group_sequence_destructor(Ptr)),
        entry!(0x004e_eb60, fn_004eeb60(u32, u32, Ptr)),
        entry!(0x004e_ec60, bs_anim_group_sequence_get_scaled_time(Ptr, f32) -> f64),
        entry!(0x004e_ecb0, fn_004eecb0(Ptr)),
        entry!(0x004e_edd0, fn_004eedd0(Ptr, u32, u32) -> u32),
        entry!(0x004e_ee00, fn_004eee00(Ptr)),
        entry!(0x004e_ef00, bs_anim_group_sequence_play_sounds(Ptr, Ptr) -> bool),
        entry!(0x004e_f970, fn_004ef970(Ptr, u32)),
        entry!(0x004e_f990, fn_004ef990() -> u32),
        entry!(0x004e_f9a0, bs_anim_group_sequence_get_priority(Ptr, u32, Ptr) -> u8),
        entry!(0x004e_fc90, bs_anim_group_sequence_save_game(Ptr, Ptr, f32)),
        entry!(
            0x004e_fdc0,
            bs_anim_group_sequence_deactivate(Ptr, f32, u32) -> bool
        ),
        entry!(0x004e_fa50, fn_004efa50(Ptr)),
        entry!(0x004e_faa0, fn_004efaa0() -> u16),
        entry!(0x004e_fb10, fn_004efb10(Ptr) -> u16),
        entry!(0x004e_fb20, fn_004efb20(Ptr, f32)),
        entry!(0x004e_fbc0, fn_004efbc0(Ptr, f32)),
        entry!(0x004e_fd10, fn_004efd10(Ptr, Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_anim_group_sequence_create_clone_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(CREATE_CLONE_TARGET_00A344F0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x004e_ec20, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CREATE_CLONE_TARGET_00A344F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }

    #[test]
    fn forwarder_bs_anim_group_sequence_get_viewer_strings_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET_00A34540, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004e_ec40, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VIEWER_STRINGS_TARGET_00A34540)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A sequence with `tags` ID tags and as many interpolator items.
    fn sequence(e: &mut Engine, tags: u32) -> u32 {
        let this = e.mem.alloc(0x78);
        e.mem.set_u32(this + SEQUENCE_ARRAY_SIZE, tags);
        let items = e.mem.alloc(0x10 * tags.max(1));
        for offset in (0..0x10 * tags.max(1)).step_by(4) {
            e.mem.set_u32(items + offset, 0xaaaa_aaaa);
        }
        e.mem.set_u32(this + SEQUENCE_INTERP_ARRAY, items);
        let id_tags = e.mem.alloc(0x14 * tags.max(1));
        e.mem.set_u32(this + SEQUENCE_ID_TAG_ARRAY, id_tags);
        this
    }

    fn invalid_index(e: &mut Engine) {
        e.map(INVALID_BLEND_INDEX_01096968 & !0xfff, 0x1000);
        e.mem.set_u8(INVALID_BLEND_INDEX_01096968, 0xff);
    }

    #[test]
    fn destructor_stores_the_vtable_and_runs_the_bases() {
        let mut e = Engine::new();
        quiet(&mut e, &[POINTER_RELEASE, 0x00a3_5640]);
        let this = e.mem.alloc(0x78);
        e.call_log = Some(vec![]);
        e.call(0x004e_eb00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), BS_ANIM_GROUP_SEQUENCE_VTABLE);
        assert_eq!(
            log[1..],
            [
                (POINTER_RELEASE, vec![this + 0x74]),
                (0x00a3_5640, vec![this])
            ]
        );
    }

    #[test]
    fn fn_004eeb60_builds_a_sequence_into_the_pointer() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[0x0063_3c90, 0x00a3_5700, POINTER_ASSIGN, POINTER_RELEASE],
        );
        e.register(0x00aa_13e0, |_, _| 0x5000u32.into_ret());
        e.register(0x0055_9450, |_, _| 0x6000u32.into_ret());
        e.register(0x004e_e9f0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_eb60, &args![0x11u32, 0x22u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let temp = calls(&log, 0x0063_3c90)[0][0];
        assert_eq!(calls(&log, 0x0063_3c90), vec![vec![temp, 0]]);
        assert_eq!(calls(&log, 0x00a3_5700), vec![vec![0x11, 0x22, temp]]);
        assert_eq!(calls(&log, 0x00aa_13e0), vec![vec![0x78]]);
        assert_eq!(calls(&log, 0x004e_e9f0), vec![vec![0x5000, 0, 0x6000]]);
        assert_eq!(calls(&log, POINTER_ASSIGN), vec![vec![0x7000, 0x5000]]);
        assert_eq!(calls(&log, POINTER_RELEASE), vec![vec![temp]]);
        // A failed allocation assigns null and constructs nothing.
        e.register(0x00aa_13e0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_eb60, &args![0x11u32, 0x22u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x004e_e9f0).is_empty());
        assert_eq!(calls(&log, POINTER_ASSIGN), vec![vec![0x7000, 0]]);
    }

    #[test]
    fn get_scaled_time_needs_an_active_state() {
        let mut e = Engine::new();
        e.register(0x00a3_0970, |_, a| {
            float_ret(f64::from(f32::from_bits(a[1])) * 2.0 + f64::from(a[2]))
        });
        let this = sequence(&mut e, 0);
        e.mem.set_u32(this + SEQUENCE_OFFSET, 0.5f32.to_bits());
        for (state, expected) in [
            (0u32, 0.0),
            (4, 0.0),
            (0xffff_ffff, 0.0),
            (1, 6.0),
            (3, 6.0),
        ] {
            e.mem.set_u32(this + SEQUENCE_STATE, state);
            let result = e.call(0x004e_ec60, &args![this, 2.0f32]).f64();
            assert_eq!(result, expected, "state {state}");
        }
    }

    #[test]
    fn fn_004eecb0_drops_the_tags_the_target_does_not_know() {
        let mut e = Engine::new();
        invalid_index(&mut e);
        let this = sequence(&mut e, 2);
        // Without an owner nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x004e_ecb0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + SEQUENCE_OWNER, 0x100);
        e.mem.set_u32(this + SEQUENCE_ACCUM_ROOT, 0x200);
        let target = e.mem.alloc(4);
        let target_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(target, target_vtable);
        e.mem.set_u32(target_vtable + 0x8c, 0x7000_008c);
        e.mem.set_u32(target_vtable + 0x90, 0x7000_0090);
        e.mem.set_u32(target_vtable + 0x98, 0x7000_0098);
        let holder = e.mem.alloc(4);
        let holder_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(holder, holder_vtable);
        e.mem.set_u32(holder_vtable + 0x9c, 0x7000_009c);
        e.register_double(OWNER_TARGET, move |_, _| target.into_ret());
        e.register(0x0041_3f40, |_, _| 0x300u32.into_ret());
        e.register(0x7000_008c, |_, _| 0x400u32.into_ret());
        e.register_double(0x7000_0098, move |_, _| holder.into_ret());
        // The first tag is known, the second is not.
        e.register(ID_TAG_NAME, |_, a| (a[0] + 1).into_ret());
        let first_tag = e.mem.u32(this + SEQUENCE_ID_TAG_ARRAY);
        e.register_double(0x7000_009c, move |_, a| {
            u32::from(a[1] == first_tag + 1).into_ret()
        });
        quiet(&mut e, &[0x7000_0090, POINTER_ASSIGN]);
        e.call_log = Some(vec![]);
        e.call(0x004e_ecb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + SEQUENCE_ACCUM_ROOT), 0x400);
        assert_eq!(calls(&log, 0x7000_008c), vec![vec![target, 0x300]]);
        let second_tag = first_tag + 0x14;
        assert_eq!(
            calls(&log, 0x7000_0090),
            vec![vec![target, second_tag + 1, 0]]
        );
        let items = e.mem.u32(this + SEQUENCE_INTERP_ARRAY);
        assert_eq!(calls(&log, POINTER_ASSIGN), vec![vec![items + 0x14, 0]]);
        assert_eq!(e.mem.u32(items + 8), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(items + 0x18), 0);
        assert_eq!(e.mem.u8(items + 0x1c), 0xff);
        assert_eq!(e.mem.u8(items + 0xc), 0xaa);
    }

    #[test]
    fn fn_004eedd0_passes_the_owner_target_and_the_second_word() {
        let mut e = Engine::new();
        let this = sequence(&mut e, 0);
        e.mem.set_u32(this + SEQUENCE_OWNER, 0x100);
        e.register(OWNER_TARGET, |_, a| (a[0] + 1).into_ret());
        e.register(0x00a3_1910, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x004e_edd0, &args![this, 5u32, 6u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 0x77);
        assert_eq!(calls(&log, OWNER_TARGET), vec![vec![0x100]]);
        assert_eq!(calls(&log, 0x00a3_1910), vec![vec![this, 5, 0x101, 6]]);
    }

    #[test]
    fn fn_004eee00_drops_tags_without_a_name_or_a_target() {
        let mut e = Engine::new();
        invalid_index(&mut e);
        let this = sequence(&mut e, 3);
        e.mem.set_u32(this + SEQUENCE_ACCUM_ROOT, 0x200);
        let target = e.mem.alloc(4);
        let target_vtable = e.mem.alloc(0x90);
        e.mem.set_u32(target, target_vtable);
        e.mem.set_u32(target_vtable + 0x8c, 0x7000_008c);
        e.register_double(OWNER_TARGET, move |_, _| target.into_ret());
        let tags = e.mem.u32(this + SEQUENCE_ID_TAG_ARRAY);
        e.register(ID_TAG_NAME, |_, a| a[0].into_ret());
        // Tag 0: no characters; tag 1: found; tag 2: not found.
        e.register_double(FIXED_STRING_CHARS, move |_, a| {
            u32::from(a[0] != tags).into_ret()
        });
        e.register_double(0x7000_008c, move |_, a| {
            let found = a[1] == this + SEQUENCE_ACCUM_ROOT_NAME || a[1] == tags + 0x14;
            u32::from(found).into_ret()
        });
        quiet(&mut e, &[POINTER_ASSIGN]);
        e.call_log = Some(vec![]);
        e.call(0x004e_ee00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + SEQUENCE_ACCUM_ROOT), 1);
        let items = e.mem.u32(this + SEQUENCE_INTERP_ARRAY);
        assert_eq!(
            calls(&log, POINTER_ASSIGN),
            vec![vec![items + 4, 0], vec![items + 0x24, 0]]
        );
        assert_eq!(e.mem.u8(items + 0xc), 0xff);
        assert_eq!(e.mem.u8(items + 0x1c), 0xaa);
        assert_eq!(e.mem.u8(items + 0x2c), 0xff);
        // The nameless tag is not looked up.
        assert_eq!(calls(&log, 0x7000_008c).len(), 3);
    }

    #[test]
    fn fn_004ef970_passes_the_flag_byte_and_the_mask() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0043_b370]);
        e.call_log = Some(vec![]);
        e.call(0x004e_f970, &args![0x10u32, 0x1234_5601u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0043_b370), vec![vec![0x10, 1, 0x20000]]);
    }

    #[test]
    fn fn_004ef990_reads_the_global() {
        let mut e = Engine::new();
        e.map(GLOBAL_011C61C8 & !0xfff, 0x1000);
        e.mem.set_u32(GLOBAL_011C61C8, 0xabcd);
        assert_eq!(e.call(0x004e_f990, &[]).u32(), 0xabcd);
    }

    #[test]
    fn get_priority_finds_the_named_interpolator() {
        let mut e = Engine::new();
        let this = sequence(&mut e, 3);
        let items = e.mem.u32(this + SEQUENCE_INTERP_ARRAY);
        e.mem.set_u8(items + 0x2d, 7);
        let tags = e.mem.u32(this + SEQUENCE_ID_TAG_ARRAY);
        e.register(ID_TAG_NAME, |_, a| a[0].into_ret());
        // Tag 0 has no characters, tag 1 another name, tag 2 the one asked for.
        e.register_double(FIXED_STRING_CHARS, move |_, a| {
            if a[0] == tags {
                0u32.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        e.register(0x0041_3f40, |_, _| 0x900u32.into_ret());
        e.register_double(0x0040_4dc0, move |_, a| {
            u32::from(a[0] != tags + 0x28).into_ret()
        });
        quiet(&mut e, &[0x005b_5e40]);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let priority = e.call(0x004e_f9a0, &args![this, 0x55u32, out]).u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(priority, 7);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(calls(&log, 0x005b_5e40), vec![vec![0x0102_38b0, 0x900]]);
        assert_eq!(
            calls(&log, 0x0040_4dc0),
            vec![vec![tags + 0x14, 0x55], vec![tags + 0x28, 0x55]]
        );
        // Without a match: 0, and a null out pointer is not written.
        e.register(0x0040_4dc0, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x004e_f9a0, &args![this, 0x55u32, 0u32]).u8(), 0);
    }

    #[test]
    fn save_game_writes_the_five_fields() {
        let mut e = Engine::new();
        let this = sequence(&mut e, 0);
        e.mem.set_u32(this + SEQUENCE_OFFSET, 1.5f32.to_bits());
        e.mem.set_u32(this + SEQUENCE_STATE, 0x0102);
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = written.clone();
        e.register_double(0x0086_5e50, move |e, a| {
            assert_eq!(a[0], 0x900);
            assert_eq!(a[3], 0);
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            record.borrow_mut().push((a[1], a[2], value));
            Ret::default()
        });
        e.call(0x004e_fc90, &args![this, 0x900u32, 2.0f32]);
        let written = written.borrow();
        assert_eq!(written.len(), 5);
        assert_eq!((written[0].1, written[0].2), (4, 3.5f32.to_bits()));
        assert_eq!((written[1].1, written[1].2), (1, 2));
        assert_eq!(written[2].0, this + 0x34);
        assert_eq!(written[3].0, this + 0x38);
        assert_eq!(written[4].0, this + 0x54);
    }

    #[test]
    fn deactivate_updates_the_transition_controller_for_the_listed_groups() {
        let mut e = Engine::new();
        let this = sequence(&mut e, 0);
        e.mem.set_u32(this + SEQUENCE_LAST_TIME, 0x7f80_0001);
        e.register(0x0055_9450, |_, _| 0x100u32.into_ret());
        e.register(0x00a5_c570, |_, a| (a[0] + 1).into_ret());
        e.register(0x005f_36f0, |_, _| 1u32.into_ret());
        e.register(0x0096_11e0, |_, _| 0x500u32.into_ret());
        quiet(&mut e, &[0x004f_0380, 0x00a3_5030]);
        for (root, kind, updates) in [
            (0u32, 0x18, false),
            (0x200, 0x16, false),
            (0x200, 0x1a, false),
            (0x200, 0x17, true),
            (0x200, 0x19, true),
        ] {
            e.mem.set_u32(this + SEQUENCE_ACCUM_ROOT, root);
            e.register_double(0x005f_2420, move |_, _| (kind as u32).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x004e_fdc0, &args![this, 0x7f80_0002u32, 0x101u32]);
            let log = e.call_log.take().unwrap();
            let expected = if updates {
                vec![vec![0x501, 0x7fc0_0001, 1]]
            } else {
                vec![]
            };
            assert_eq!(calls(&log, 0x004f_0380), expected, "kind {kind:#x}");
            assert_eq!(calls(&log, 0x00a3_5030), vec![vec![this, 0x7fc0_0002, 1]]);
        }
        // No first controller: nothing is updated.
        e.mem.set_u32(this + SEQUENCE_ACCUM_ROOT, 0x200);
        e.register(0x00a5_c570, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_fdc0, &args![this, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x00a5_c570), vec![vec![0x200, 0x0120_31b8]]);
        assert!(calls(&log, 0x0096_11e0).is_empty());
    }

    #[test]
    fn deactivate_answers_the_sequence_deactivate() {
        let mut e = Engine::new();
        let this = sequence(&mut e, 0);
        e.mem.set_u32(this + SEQUENCE_ACCUM_ROOT, 0);
        e.register(0x00a3_5030, |_, _| 1u32.into_ret());
        assert!(e.call(0x004e_fdc0, &args![this, 0u32, 0u32]).bool());
        e.register(0x00a3_5030, |_, _| 0u32.into_ret());
        assert!(!e.call(0x004e_fdc0, &args![this, 0u32, 0u32]).bool());
    }

    struct Keys {
        texts: Vec<(f32, &'static str)>,
    }

    /// Sets up a sequence whose text keys are `keys`, playing window `[begin, end)`.
    fn play_world(e: &mut Engine, keys: &Keys, begin: f32, end: f32) -> u32 {
        e.map(FLOAT_MAX_01023850 & !0xfff, 0x1000);
        e.mem.set_u32(FLOAT_MAX_01023850, f32::MAX.to_bits());
        for (address, text) in [
            (SOUND_PREFIX_010238A8, "Sound: "),
            (STOP_SOUNDS_PREFIX_01023894, "Enum: StopSounds "),
            (HIT_SHADER_PREFIX_01023884, "Enum: HitShader"),
        ] {
            e.mem.write(address, text.as_bytes());
            e.mem.set_u8(address + text.len() as u32, 0);
        }
        for page in [
            PLAYER_011DEA3C,
            DATA_HANDLER_011C3F2C,
            DEFAULT_POSITION_011F426C,
        ] {
            e.map(page & !0xfff, 0x1000);
        }
        e.mem.set_u32(DATA_HANDLER_011C3F2C, 0xd00);
        let sequence = e.mem.alloc(0x78);
        e.register(0x0080_41a0, |_, _| 1u32.into_ret());
        e.register_double(0x0050_7b00, move |_, _| float_ret(f64::from(begin)));
        e.register_double(0x0059_8040, move |_, _| float_ret(f64::from(end)));
        e.register(0x0070_0300, |_, _| 0x600u32.into_ret());
        let list = e.mem.alloc(8 * keys.texts.len() as u32 + 8);
        for (index, (time, text)) in keys.texts.iter().enumerate() {
            let string = e.mem.alloc(text.len() as u32 + 1);
            e.mem.write(string, text.as_bytes());
            e.mem.set_u8(string + text.len() as u32, 0);
            e.mem.set_u32(list + 8 * index as u32, string);
            e.mem.set_u32(list + 8 * index as u32 + 4, time.to_bits());
        }
        let count = keys.texts.len() as u32;
        e.register_double(0x0049_5f30, move |e, a| {
            e.mem.set_u32(a[1], count);
            list.into_ret()
        });
        e.register(0x0071_7e50, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_CHARS, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x006a_7f50, |e, a| {
            float_ret(f64::from(f32::from_bits(e.mem.u32(a[0] + 4))))
        });
        e.register(STRNICMP, |e, a| {
            let mut differ = 0u32;
            for offset in 0..a[2] {
                let left = e.mem.u8(a[0] + offset).to_ascii_lowercase();
                let right = e.mem.u8(a[1] + offset).to_ascii_lowercase();
                if left != right {
                    differ = 1;
                    break;
                }
                if left == 0 {
                    break;
                }
            }
            differ.into_ret()
        });
        e.register(STRCHR, |e, a| {
            let mut at = a[0];
            loop {
                let byte = e.mem.u8(at);
                if u32::from(byte) == a[1] & 0xff {
                    return at.into_ret();
                }
                if byte == 0 {
                    return 0u32.into_ret();
                }
                at += 1;
            }
        });
        e.register(COPY_STRING, |e, a| {
            let mut offset = 0;
            loop {
                let byte = e.mem.u8(a[1] + offset);
                e.mem.set_u8(a[0] + offset, byte);
                if byte == 0 {
                    break;
                }
                offset += 1;
            }
            Ret::default()
        });
        e.register(0x0059_bb30, |_, _| 0u32.into_ret());
        sequence
    }

    type Names = std::rc::Rc<std::cell::RefCell<Vec<String>>>;

    /// Doubles `TESDataHandler::GetSound_ov2` with `sound` and records the names it is asked
    /// for (read during the call, while the text is cut).
    fn record_sounds(e: &mut Engine, sound: u32) -> Names {
        let names: Names = Default::default();
        let record = names.clone();
        e.register_double(0x0046_16c0, move |e, a| {
            let mut text = String::new();
            let mut at = a[1];
            while e.mem.u8(at) != 0 {
                text.push(e.mem.u8(at) as char);
                at += 1;
            }
            record.borrow_mut().push(text);
            sound.into_ret()
        });
        names
    }

    /// Doubles `004e75d0` with a zeroed block (the sound's data words read 0).
    fn sound_data(e: &mut Engine) {
        let data = e.mem.alloc(0x24);
        e.register_double(0x004e_75d0, move |_, _| data.into_ret());
    }

    #[test]
    fn play_sounds_plays_the_keys_in_the_window_without_an_actor() {
        let mut e = Engine::new();
        let keys = Keys {
            texts: vec![
                (0.5, "Sound: NPCHumanFootstep LFoot\r\n"),
                (0.1, "Sound: TooEarly"),
                (0.9, "Sound: TooLate"),
                (0.6, "Other text"),
            ],
        };
        let sequence = play_world(&mut e, &keys, 0.2, 0.8);
        let names = record_sounds(&mut e, 0xe00);
        sound_data(&mut e);
        e.register(0x005e_39b0, |_, _| 0x10u32.into_ret());
        // Not the player: no 0x200000.
        e.mem.set_u32(PLAYER_011DEA3C, 0x1000);
        e.register(0x0084_e3a0, |_, _| 0x1234u32.into_ret());
        e.register(0x0045_3a70, |_, _| 0xa00u32.into_ret());
        e.register(0x00ad_73b0, |_, a| a[1].into_ret());
        quiet(
            &mut e,
            &[
                0x0041_a250,
                0x0041_8900,
                SOUND_HANDLE_DESTROY,
                0x00ad_8f20,
                0x00ad_8b60,
                0x00ad_8830,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x004e_ef00, &args![sequence, 0u32]).u8();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 0);
        assert_eq!(*names.borrow(), vec!["NPCHumanFootstep"]);
        let data_handler_calls = calls(&log, 0x0046_16c0);
        assert_eq!(data_handler_calls[0][0], 0xd00);
        // The cuts are restored after the call.
        let text = data_handler_calls[0][1] - 7;
        assert_eq!(e.mem.u8(text + 23), b' ');
        assert_eq!(e.mem.u8(text + 29), b'\r');
        // Flag 0x10 of the sound data: the flagged, actorless handle.
        let made = calls(&log, 0x00ad_73b0);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][0], 0xa00);
        assert_eq!(made[0][2..], [0x1234, 0x10 | 0x4000_0022]);
        assert!(calls(&log, 0x00ad_8f20).is_empty());
        assert_eq!(calls(&log, 0x00ad_8830)[0][1], 1);
        assert_eq!(calls(&log, 0x00ad_8b60)[0][1..], [0, 0, 0]);
    }

    #[test]
    fn play_sounds_wraps_the_window_and_checks_the_end() {
        let mut e = Engine::new();
        let keys = Keys {
            texts: vec![(0.9, "Sound: A"), (0.1, "Sound: B"), (0.5, "Sound: C")],
        };
        let sequence = play_world(&mut e, &keys, 0.8, 0.2);
        let names = record_sounds(&mut e, 0);
        e.register(0x0059_bb30, |_, _| 1u32.into_ret());
        e.register(0x0063_9aa0, |_, _| float_ret(1.0));
        e.register(0x0050_8100, |_, _| float_ret(0.1));
        e.call_log = Some(vec![]);
        let result = e.call(0x004e_ef00, &args![sequence, 0u32]).u8();
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0041_a250).is_empty());
        assert_eq!(*names.borrow(), vec!["A", "B"]);
        // 0.2 (end) is not below 0.1: the result stays 1.
        assert_eq!(result, 1);
        e.register(0x0050_8100, |_, _| float_ret(0.3));
        assert_eq!(e.call(0x004e_ef00, &args![sequence, 0u32]).u8(), 0);
        e.register(0x0063_9aa0, |_, _| float_ret(-f64::from(f32::MAX)));
        e.register(0x0050_8100, |_, _| float_ret(0.1));
        assert_eq!(e.call(0x004e_ef00, &args![sequence, 0u32]).u8(), 0);
    }

    #[test]
    fn play_sounds_skips_an_empty_or_unbounded_window() {
        let mut e = Engine::new();
        let keys = Keys {
            texts: vec![(0.5, "Sound: A")],
        };
        for (begin, end) in [(0.5f32, 0.5f32), (-f32::MAX, 1.0), (0.0, -f32::MAX)] {
            let sequence = play_world(&mut e, &keys, begin, end);
            e.call_log = Some(vec![]);
            e.call(0x004e_ef00, &args![sequence, 0u32]);
            let log = e.call_log.take().unwrap();
            assert!(calls(&log, 0x0070_0300).is_empty());
        }
        // A null sequence or another generic location does nothing at all.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004e_ef00, &args![0u32, 0u32]).u8(), 0);
        e.register(0x0080_41a0, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x004e_ef00, &args![0x10u32, 0u32]).u8(), 0);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0059_bb30).is_empty());
    }

    #[test]
    fn play_sounds_stops_sounds_and_plays_on_the_actor_node() {
        let mut e = Engine::new();
        let keys = Keys {
            texts: vec![
                (0.3, "Enum: StopSounds Bip01\r"),
                (0.4, "Sound: Clank Bip02"),
                (0.5, "Enum: HitShader"),
            ],
        };
        let sequence = play_world(&mut e, &keys, 0.2, 0.8);
        // The actor: vtable slot 0x1d0 gives its root, whose slot 0x9c finds nodes.
        let actor = e.mem.alloc(4);
        let actor_vtable = e.mem.alloc(0x1d4);
        e.mem.set_u32(actor, actor_vtable);
        e.mem.set_u32(actor_vtable + 0x1d0, 0x7000_01d0);
        let root = e.mem.alloc(4);
        let root_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(root, root_vtable);
        e.mem.set_u32(root_vtable + 0x9c, 0x7000_009c);
        e.register_double(0x7000_01d0, move |_, _| root.into_ret());
        e.register(0x0043_8170, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        // The node lookup answers with the fixed string's slot plus one.
        e.register(0x7000_009c, |_, a| (a[1] + 1).into_ret());
        quiet(
            &mut e,
            &[0x0043_81b0, 0x00ad_8570, 0x0096_2290, 0x0043_b370],
        );
        e.register(0x0045_3a70, |_, _| 0xa00u32.into_ret());
        e.mem.set_u32(PLAYER_011DEA3C, actor);
        let names = record_sounds(&mut e, 0xe00);
        sound_data(&mut e);
        e.register(0x005e_39b0, |_, _| 0u32.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x1234u32.into_ret());
        e.register(0x00ad_73b0, |_, a| a[1].into_ret());
        let position = e.mem.alloc(0xc);
        e.mem.set_u32(position, 1.0f32.to_bits());
        e.mem.set_u32(position + 4, 2.0f32.to_bits());
        e.mem.set_u32(position + 8, 0x7f80_0001);
        e.register_double(0x0045_bb80, move |_, _| position.into_ret());
        e.register(0x0082_eca0, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                0x0041_a250,
                0x0041_8900,
                SOUND_HANDLE_DESTROY,
                0x00ad_8f20,
                0x00ad_8b60,
                0x00ad_8830,
            ],
        );
        // The sound data words read zero: neither flagged nor 2D.
        e.call_log = Some(vec![]);
        e.call(0x004e_ef00, &args![sequence, actor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(names.borrow()[0], "Clank");
        let stops = calls(&log, 0x00ad_8570);
        assert_eq!(stops.len(), 1);
        assert_eq!(stops[0][0], 0xa00);
        assert_eq!(stops[0][2..], [1.0f32.to_bits(), 0x4000_0000]);
        assert_eq!(calls(&log, 0x0096_2290), vec![vec![actor, actor]]);
        let made = calls(&log, 0x00ad_73b0);
        assert_eq!(made[0][3], 0x0020_0000 | 0x4000_2102);
        let follow = calls(&log, 0x00ad_8f20);
        assert_eq!(follow.len(), 1);
        assert_eq!(
            calls(&log, 0x00ad_8b60)[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 0x7fc0_0001]
        );
        let audible = calls(&log, 0x0082_eca0);
        assert_eq!(
            audible,
            vec![vec![
                0xe00,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                0x7f80_0001,
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(calls(&log, 0x0043_b370).len(), 2);
        // An inaudible actor sound makes no handle but still plays the empty one.
        e.register(0x0082_eca0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_ef00, &args![sequence, actor]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x00ad_73b0).is_empty());
        assert_eq!(calls(&log, 0x00ad_8830).len(), 1);
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

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const OBJECT: u32 = 0x0900_0000;

    /// The object at `011de45c` with version byte `version`; reads fill each
    /// byte with 0x40 plus the count of reads so far.
    fn engine(version: u32) -> Engine {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_OBJECT_011DE45C, 4);
        e.mem.set_u32(SAVE_LOAD_OBJECT_011DE45C, OBJECT);
        e.register_double(SAVE_LOAD_VERSION, move |_, _| ret(version));
        e.register(SAVE_LOAD_WRITE, |_, _| Ret::default());
        let mut reads = 0u8;
        e.register_double(SAVE_LOAD_READ, move |e, a| {
            reads += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0x40 + reads);
            }
            Ret::default()
        });
        let mut reads = 0u8;
        e.register_double(LOAD_BUFFER_READ, move |e, a| {
            reads += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0x40 + reads);
            }
            Ret::default()
        });
        e.register(0x00a3_0970, |_, _| Ret::default());
        e.map(FLOAT_MAX_01023850, 4);
        e.mem.set_u32(FLOAT_MAX_01023850, f32::MAX.to_bits());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn clears_each_interpolator_word() {
        let mut e = engine(0x80);
        let this = e.mem.alloc(0x80);
        let items = e.mem.alloc(0x30);
        e.mem.write(items, &[0xcd; 0x30]);
        e.mem.set_u32(this + SEQUENCE_INTERP_ARRAY, items);
        e.register(0x0084_e3a0, |_, _| ret(2));
        e.call(0x004e_fa50, &[this]);
        assert_eq!(e.mem.u32(items + 8), 0);
        assert_eq!(e.mem.u32(items + 0x18), 0);
        assert_eq!(e.mem.u32(items + 0x28), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(items + 4), 0xcdcd_cdcd);
        assert_eq!(calls_to(&e, 0x0084_e3a0).len(), 3);
    }

    #[test]
    fn save_size_depends_on_the_version() {
        let mut e = engine(0x71);
        assert_eq!(e.call(0x004e_faa0, &[]).u32(), 20);
        assert_eq!(e.call(0x004e_fb10, &[0x1234]).u32(), 20);
        let mut e = engine(0x70);
        assert_eq!(e.call(0x004e_faa0, &[]).u32(), 24);
        assert_eq!(calls_to(&e, SAVE_LOAD_VERSION), vec![vec![OBJECT]]);
    }

    #[test]
    fn save_writes_the_offset_sum_and_the_fields() {
        for (version, writes) in [(0x70u32, 6usize), (0x71, 5)] {
            let mut e = engine(version);
            let this = e.mem.alloc(0x80);
            e.mem.set_u32(this + SEQUENCE_OFFSET, 1.5f32.to_bits());
            let sums = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let seen = sums.clone();
            e.register_double(SAVE_LOAD_WRITE, move |e, a| {
                seen.borrow_mut().push(e.mem.u32(a[1]));
                Ret::default()
            });
            e.call(0x004e_fb20, &args![this, 2.0f32]);
            let writes_seen = calls_to(&e, SAVE_LOAD_WRITE);
            assert_eq!(writes_seen.len(), writes);
            assert_eq!(writes_seen[1], vec![OBJECT, this + SEQUENCE_STATE, 4]);
            assert_eq!(
                writes_seen[writes - 1],
                vec![OBJECT, this + SEQUENCE_DEST_FRAME, 4]
            );
            assert_eq!(sums.borrow()[0], 3.5f32.to_bits());
            if writes == 6 {
                assert_eq!(sums.borrow()[2], 3.5f32.to_bits());
            }
        }
    }

    #[test]
    fn load_reads_back_and_resets() {
        let mut e = engine(0x70);
        let this = e.mem.alloc(0x80);
        e.call(0x004e_fbc0, &args![this, 0.0f32]);
        // The third read (0x43 bytes) overwrote the first.
        let sum = f32::from_bits(0x4343_4343);
        assert_eq!(e.mem.u32(this + SEQUENCE_OFFSET), sum.to_bits());
        assert_eq!(e.mem.u32(this + SEQUENCE_STATE), 0x4242_4242);
        assert_eq!(e.mem.u32(this + SEQUENCE_DEST_FRAME), 0x4646_4646);
        assert_eq!(e.mem.u32(this + 0x4c), (-f32::MAX).to_bits());
        assert_eq!(e.mem.u32(this + 0x50), (-f32::MAX).to_bits());
        let mut e = engine(0x71);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + SEQUENCE_STATE, 2);
        // State 0x42424242 after the read: GetScaledTime does not compute.
        e.call(0x004e_fbc0, &args![this, 1.0f32]);
        assert_eq!(calls_to(&e, SAVE_LOAD_READ).len(), 5);
        assert!(calls_to(&e, 0x00a3_0970).is_empty());
        let sum = f32::from_bits(0x4141_4141);
        assert_eq!(
            e.mem.u32(this + SEQUENCE_OFFSET),
            ((sum as f64 - 1.0) as f32).to_bits()
        );
    }

    #[test]
    fn load_from_buffer_reads_a_float_a_byte_and_three_fields() {
        let mut e = engine(0x71);
        let this = e.mem.alloc(0x80);
        e.register_double(LOAD_BUFFER_READ, |e, a| {
            match a[2] {
                1 => e.mem.set_u8(a[1], 2),
                _ => e.mem.set_u32(a[1], 5.0f32.to_bits()),
            }
            Ret::default()
        });
        e.register(0x00a3_0970, |_, _| Ret::default());
        e.call(0x004e_fd10, &args![this, 0x5000u32, 1.5f32]);
        assert_eq!(e.mem.f32(this + SEQUENCE_OFFSET), 3.5);
        assert_eq!(e.mem.u32(this + SEQUENCE_STATE), 2);
        assert_eq!(e.mem.f32(this + SEQUENCE_LAST_TIME), 5.0);
        assert_eq!(e.mem.u32(this + 0x50), (-f32::MAX).to_bits());
        let reads = calls_to(&e, LOAD_BUFFER_READ);
        assert_eq!(reads.len(), 5);
        assert_eq!(reads[1][2], 1);
        // State 2: GetScaledTime runs with offset + time.
        let scaled = calls_to(&e, 0x00a3_0970);
        assert_eq!(scaled, vec![vec![this, 5.0f32.to_bits(), 1]]);
    }
}
